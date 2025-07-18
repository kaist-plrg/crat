use ::libc;
use ::c2rust_bitfields;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    pub type json_object;
    pub type aesrand;
    pub type iterator;
    fn strtol(
        _: *const libc::c_char,
        _: *mut *mut libc::c_char,
        _: libc::c_int,
    ) -> libc::c_long;
    fn free(__ptr: *mut libc::c_void);
    fn exit(_: libc::c_int) -> !;
    static mut stdout: *mut FILE;
    static mut stderr: *mut FILE;
    fn fclose(__stream: *mut FILE) -> libc::c_int;
    fn fopen(_: *const libc::c_char, _: *const libc::c_char) -> *mut FILE;
    fn fprintf(_: *mut FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
    fn sprintf(_: *mut libc::c_char, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn memset(
        _: *mut libc::c_void,
        _: libc::c_int,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    fn strdup(_: *const libc::c_char) -> *mut libc::c_char;
    fn strchr(_: *const libc::c_char, _: libc::c_int) -> *mut libc::c_char;
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    fn strerror(_: libc::c_int) -> *mut libc::c_char;
    fn sysconf(__name: libc::c_int) -> libc::c_long;
    fn __assert_fail(
        __assertion: *const libc::c_char,
        __file: *const libc::c_char,
        __line: libc::c_uint,
        __function: *const libc::c_char,
    ) -> !;
    fn __errno_location() -> *mut libc::c_int;
    fn strftime(
        __s: *mut libc::c_char,
        __maxsize: size_t,
        __format: *const libc::c_char,
        __tp: *const tm,
    ) -> size_t;
    fn localtime(__timer: *const time_t) -> *mut tm;
    fn time(__timer: *mut time_t) -> time_t;
    fn inet_ntoa(__in: in_addr) -> *mut libc::c_char;
    fn json_tokener_parse(str: *const libc::c_char) -> *mut json_object;
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
    fn pthread_mutex_lock(__mutex: *mut pthread_mutex_t) -> libc::c_int;
    fn pthread_mutex_unlock(__mutex: *mut pthread_mutex_t) -> libc::c_int;
    fn blocklist_init(
        allowlist: *mut libc::c_char,
        blocklist: *mut libc::c_char,
        allowlist_entries: *mut *mut libc::c_char,
        allowlist_entries_len: size_t,
        blocklist_entries: *mut *mut libc::c_char,
        blocklist_entries_len: size_t,
        ignore_invalid_hosts: libc::c_int,
    ) -> libc::c_int;
    fn blocklist_count_allowed() -> uint64_t;
    fn blocklist_count_not_allowed() -> uint64_t;
    fn log_fatal(
        loggerName: *const libc::c_char,
        logMessage: *const libc::c_char,
        _: ...
    ) -> !;
    fn log_warn(
        loggerName: *const libc::c_char,
        logMessage: *const libc::c_char,
        _: ...
    ) -> libc::c_int;
    fn log_info(
        loggerName: *const libc::c_char,
        logMessage: *const libc::c_char,
        _: ...
    ) -> libc::c_int;
    fn log_debug(
        loggerName: *const libc::c_char,
        logMessage: *const libc::c_char,
        _: ...
    ) -> libc::c_int;
    fn log_init(
        stream: *mut FILE,
        level: LogLevel,
        syslog_enabled: libc::c_int,
        syslog_app: *const libc::c_char,
    ) -> libc::c_int;
    fn random_bytes(dst: *mut libc::c_void, n: size_t) -> libc::c_int;
    fn parse_max_hosts(max_targets: *mut libc::c_char) -> uint32_t;
    fn enforce_range(
        name: *const libc::c_char,
        v: libc::c_int,
        min: libc::c_int,
        max: libc::c_int,
    );
    fn split_string(
        in_0: *const libc::c_char,
        len: *mut libc::c_int,
        results: *mut *mut *const libc::c_char,
    );
    fn fprintw(f: *mut FILE, s: *const libc::c_char, w: size_t);
    fn parse_mac(out: *mut macaddr_t, in_0: *mut libc::c_char) -> libc::c_int;
    fn file_exists(name: *mut libc::c_char) -> libc::c_int;
    fn drop_privs() -> libc::c_int;
    fn set_cpu(core: uint32_t) -> libc::c_int;
    fn xcalloc(count: size_t, size: size_t) -> *mut libc::c_void;
    fn xmalloc(size: size_t) -> *mut libc::c_void;
    fn pbm_init() -> *mut *mut uint8_t;
    fn pbm_load_from_file(b: *mut *mut uint8_t, file: *mut libc::c_char) -> uint32_t;
    fn aesrand_init_from_seed(_: uint64_t) -> *mut aesrand_t;
    fn cmdline_parser_ext(
        argc: libc::c_int,
        argv: *mut *mut libc::c_char,
        args_info: *mut gengetopt_args_info,
        params: *mut cmdline_parser_params,
    ) -> libc::c_int;
    fn cmdline_parser_print_help();
    fn cmdline_parser_print_version();
    fn cmdline_parser_params_create() -> *mut cmdline_parser_params;
    fn cmdline_parser_free(args_info: *mut gengetopt_args_info);
    fn cmdline_parser_config_file(
        filename: *const libc::c_char,
        args_info: *mut gengetopt_args_info,
        params: *mut cmdline_parser_params,
    ) -> libc::c_int;
    fn cmdline_parser_required(
        args_info: *mut gengetopt_args_info,
        prog_name: *const libc::c_char,
    ) -> libc::c_int;
    fn get_shard(it: *mut iterator_t, thread_id: uint8_t) -> *mut shard_t;
    fn get_dryrun_socket() -> sock_t;
    fn get_socket(id: uint32_t) -> sock_t;
    fn send_init() -> *mut iterator_t;
    fn send_run(_: sock_t, _: *mut shard_t) -> libc::c_int;
    fn recv_run(recv_ready_mutex_0: *mut pthread_mutex_t) -> libc::c_int;
    fn parse_filter_string(filter: *mut libc::c_char) -> libc::c_int;
    fn fds_get_index_by_name(
        fds: *mut fielddefset_t,
        name: *const libc::c_char,
    ) -> libc::c_int;
    fn gen_fielddef_set(fds: *mut fielddefset_t, fs: *mut fielddef_t, len: libc::c_int);
    fn fs_generate_fieldset_translation(
        t: *mut translation_t,
        avail: *mut fielddefset_t,
        req: *mut *const libc::c_char,
        reqlen: libc::c_int,
    );
    fn fs_generate_full_fieldset_translation(
        t: *mut translation_t,
        avail: *mut fielddefset_t,
    );
    fn validate_filter(root: *mut node_t, fields: *mut fielddefset_t) -> libc::c_int;
    static mut zrecv: state_recv;
    static mut zconf: state_conf;
    fn init_empty_global_configuration(c: *mut state_conf);
    static mut zsend: state_send;
    fn monitor_run(it: *mut iterator_t, lock: *mut pthread_mutex_t);
    fn monitor_init();
    fn get_hw_addr(
        gw_ip: *mut in_addr,
        iface: *mut libc::c_char,
        hw_mac: *mut libc::c_uchar,
    ) -> libc::c_int;
    fn get_default_gw(gw: *mut in_addr, iface: *mut libc::c_char) -> libc::c_int;
    fn get_iface_ip(iface: *mut libc::c_char, ip: *mut in_addr) -> libc::c_int;
    fn get_default_iface() -> *mut libc::c_char;
    fn json_metadata(_: *mut FILE);
    fn parse_source_ip_addresses(given_string: *mut libc::c_char);
    fn get_output_module_by_name(_: *const libc::c_char) -> *mut output_module_t;
    fn print_output_modules();
    fn get_probe_module_by_name(_: *const libc::c_char) -> *mut probe_module_t;
    fn print_probe_modules();
    static mut ip_fields_len: libc::c_int;
    static mut sys_fields_len: libc::c_int;
    static mut ip_fields: [fielddef_t; 6];
    static mut sys_fields: [fielddef_t; 5];
}
pub type size_t = libc::c_ulong;
pub type __u_char = libc::c_uchar;
pub type __uint8_t = libc::c_uchar;
pub type __uint16_t = libc::c_ushort;
pub type __uint32_t = libc::c_uint;
pub type __uint64_t = libc::c_ulong;
pub type __off_t = libc::c_long;
pub type __off64_t = libc::c_long;
pub type __time_t = libc::c_long;
pub type __syscall_slong_t = libc::c_long;
pub type u_char = __u_char;
pub type time_t = __time_t;
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
pub type pthread_t = libc::c_ulong;
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
pub struct tm {
    pub tm_sec: libc::c_int,
    pub tm_min: libc::c_int,
    pub tm_hour: libc::c_int,
    pub tm_mday: libc::c_int,
    pub tm_mon: libc::c_int,
    pub tm_year: libc::c_int,
    pub tm_wday: libc::c_int,
    pub tm_yday: libc::c_int,
    pub tm_isdst: libc::c_int,
    pub tm_gmtoff: libc::c_long,
    pub tm_zone: *const libc::c_char,
}
pub type uint8_t = __uint8_t;
pub type uint16_t = __uint16_t;
pub type uint32_t = __uint32_t;
pub type uint64_t = __uint64_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct in_addr {
    pub s_addr: in_addr_t,
}
pub type in_addr_t = uint32_t;
pub type C2RustUnnamed_0 = libc::c_uint;
pub const PTHREAD_MUTEX_FAST_NP: C2RustUnnamed_0 = 0;
pub const PTHREAD_MUTEX_DEFAULT: C2RustUnnamed_0 = 0;
pub const PTHREAD_MUTEX_ERRORCHECK: C2RustUnnamed_0 = 2;
pub const PTHREAD_MUTEX_RECURSIVE: C2RustUnnamed_0 = 1;
pub const PTHREAD_MUTEX_NORMAL: C2RustUnnamed_0 = 0;
pub const PTHREAD_MUTEX_ADAPTIVE_NP: C2RustUnnamed_0 = 3;
pub const PTHREAD_MUTEX_ERRORCHECK_NP: C2RustUnnamed_0 = 2;
pub const PTHREAD_MUTEX_RECURSIVE_NP: C2RustUnnamed_0 = 1;
pub const PTHREAD_MUTEX_TIMED_NP: C2RustUnnamed_0 = 0;
#[derive(Copy, Clone, BitfieldStruct)]
#[repr(C)]
pub struct ip {
    #[bitfield(name = "ip_hl", ty = "libc::c_uint", bits = "0..=3")]
    #[bitfield(name = "ip_v", ty = "libc::c_uint", bits = "4..=7")]
    pub ip_hl_ip_v: [u8; 1],
    pub ip_tos: uint8_t,
    pub ip_len: libc::c_ushort,
    pub ip_id: libc::c_ushort,
    pub ip_off: libc::c_ushort,
    pub ip_ttl: uint8_t,
    pub ip_p: uint8_t,
    pub ip_sum: libc::c_ushort,
    pub ip_src: in_addr,
    pub ip_dst: in_addr,
}
pub type LogLevel = libc::c_uint;
pub const ZNUM_LOGLEVELS: LogLevel = 6;
pub const ZLOG_TRACE: LogLevel = 5;
pub const ZLOG_DEBUG: LogLevel = 4;
pub const ZLOG_INFO: LogLevel = 3;
pub const ZLOG_WARN: LogLevel = 2;
pub const ZLOG_ERROR: LogLevel = 1;
pub const ZLOG_FATAL: LogLevel = 0;
pub type ipaddr_n_t = uint32_t;
pub type port_n_t = uint16_t;
pub type port_h_t = uint16_t;
pub type macaddr_t = libc::c_uchar;
pub type aesrand_t = aesrand;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct gengetopt_args_info {
    pub target_port_arg: libc::c_int,
    pub target_port_orig: *mut libc::c_char,
    pub target_port_help: *const libc::c_char,
    pub output_file_arg: *mut libc::c_char,
    pub output_file_orig: *mut libc::c_char,
    pub output_file_help: *const libc::c_char,
    pub blocklist_file_arg: *mut libc::c_char,
    pub blocklist_file_orig: *mut libc::c_char,
    pub blocklist_file_help: *const libc::c_char,
    pub allowlist_file_arg: *mut libc::c_char,
    pub allowlist_file_orig: *mut libc::c_char,
    pub allowlist_file_help: *const libc::c_char,
    pub list_of_ips_file_arg: *mut libc::c_char,
    pub list_of_ips_file_orig: *mut libc::c_char,
    pub list_of_ips_file_help: *const libc::c_char,
    pub rate_arg: libc::c_int,
    pub rate_orig: *mut libc::c_char,
    pub rate_help: *const libc::c_char,
    pub bandwidth_arg: *mut libc::c_char,
    pub bandwidth_orig: *mut libc::c_char,
    pub bandwidth_help: *const libc::c_char,
    pub batch_arg: libc::c_int,
    pub batch_orig: *mut libc::c_char,
    pub batch_help: *const libc::c_char,
    pub max_targets_arg: *mut libc::c_char,
    pub max_targets_orig: *mut libc::c_char,
    pub max_targets_help: *const libc::c_char,
    pub max_runtime_arg: libc::c_int,
    pub max_runtime_orig: *mut libc::c_char,
    pub max_runtime_help: *const libc::c_char,
    pub max_results_arg: libc::c_int,
    pub max_results_orig: *mut libc::c_char,
    pub max_results_help: *const libc::c_char,
    pub probes_arg: libc::c_int,
    pub probes_orig: *mut libc::c_char,
    pub probes_help: *const libc::c_char,
    pub cooldown_time_arg: libc::c_int,
    pub cooldown_time_orig: *mut libc::c_char,
    pub cooldown_time_help: *const libc::c_char,
    pub seed_arg: libc::c_long,
    pub seed_orig: *mut libc::c_char,
    pub seed_help: *const libc::c_char,
    pub retries_arg: libc::c_int,
    pub retries_orig: *mut libc::c_char,
    pub retries_help: *const libc::c_char,
    pub dryrun_help: *const libc::c_char,
    pub shards_arg: libc::c_int,
    pub shards_orig: *mut libc::c_char,
    pub shards_help: *const libc::c_char,
    pub shard_arg: libc::c_int,
    pub shard_orig: *mut libc::c_char,
    pub shard_help: *const libc::c_char,
    pub source_port_arg: *mut libc::c_char,
    pub source_port_orig: *mut libc::c_char,
    pub source_port_help: *const libc::c_char,
    pub source_ip_arg: *mut libc::c_char,
    pub source_ip_orig: *mut libc::c_char,
    pub source_ip_help: *const libc::c_char,
    pub gateway_mac_arg: *mut libc::c_char,
    pub gateway_mac_orig: *mut libc::c_char,
    pub gateway_mac_help: *const libc::c_char,
    pub source_mac_arg: *mut libc::c_char,
    pub source_mac_orig: *mut libc::c_char,
    pub source_mac_help: *const libc::c_char,
    pub interface_arg: *mut libc::c_char,
    pub interface_orig: *mut libc::c_char,
    pub interface_help: *const libc::c_char,
    pub iplayer_help: *const libc::c_char,
    pub probe_module_arg: *mut libc::c_char,
    pub probe_module_orig: *mut libc::c_char,
    pub probe_module_help: *const libc::c_char,
    pub probe_args_arg: *mut libc::c_char,
    pub probe_args_orig: *mut libc::c_char,
    pub probe_args_help: *const libc::c_char,
    pub probe_ttl_arg: libc::c_int,
    pub probe_ttl_orig: *mut libc::c_char,
    pub probe_ttl_help: *const libc::c_char,
    pub list_probe_modules_help: *const libc::c_char,
    pub output_fields_arg: *mut libc::c_char,
    pub output_fields_orig: *mut libc::c_char,
    pub output_fields_help: *const libc::c_char,
    pub output_module_arg: *mut libc::c_char,
    pub output_module_orig: *mut libc::c_char,
    pub output_module_help: *const libc::c_char,
    pub output_args_arg: *mut libc::c_char,
    pub output_args_orig: *mut libc::c_char,
    pub output_args_help: *const libc::c_char,
    pub output_filter_arg: *mut libc::c_char,
    pub output_filter_orig: *mut libc::c_char,
    pub output_filter_help: *const libc::c_char,
    pub list_output_modules_help: *const libc::c_char,
    pub list_output_fields_help: *const libc::c_char,
    pub no_header_row_help: *const libc::c_char,
    pub verbosity_arg: libc::c_int,
    pub verbosity_orig: *mut libc::c_char,
    pub verbosity_help: *const libc::c_char,
    pub log_file_arg: *mut libc::c_char,
    pub log_file_orig: *mut libc::c_char,
    pub log_file_help: *const libc::c_char,
    pub log_directory_arg: *mut libc::c_char,
    pub log_directory_orig: *mut libc::c_char,
    pub log_directory_help: *const libc::c_char,
    pub metadata_file_arg: *mut libc::c_char,
    pub metadata_file_orig: *mut libc::c_char,
    pub metadata_file_help: *const libc::c_char,
    pub status_updates_file_arg: *mut libc::c_char,
    pub status_updates_file_orig: *mut libc::c_char,
    pub status_updates_file_help: *const libc::c_char,
    pub quiet_help: *const libc::c_char,
    pub disable_syslog_help: *const libc::c_char,
    pub notes_arg: *mut libc::c_char,
    pub notes_orig: *mut libc::c_char,
    pub notes_help: *const libc::c_char,
    pub user_metadata_arg: *mut libc::c_char,
    pub user_metadata_orig: *mut libc::c_char,
    pub user_metadata_help: *const libc::c_char,
    pub config_arg: *mut libc::c_char,
    pub config_orig: *mut libc::c_char,
    pub config_help: *const libc::c_char,
    pub max_sendto_failures_arg: libc::c_int,
    pub max_sendto_failures_orig: *mut libc::c_char,
    pub max_sendto_failures_help: *const libc::c_char,
    pub min_hitrate_arg: libc::c_float,
    pub min_hitrate_orig: *mut libc::c_char,
    pub min_hitrate_help: *const libc::c_char,
    pub sender_threads_arg: libc::c_int,
    pub sender_threads_orig: *mut libc::c_char,
    pub sender_threads_help: *const libc::c_char,
    pub cores_arg: *mut libc::c_char,
    pub cores_orig: *mut libc::c_char,
    pub cores_help: *const libc::c_char,
    pub ignore_blocklist_errors_help: *const libc::c_char,
    pub help_help: *const libc::c_char,
    pub version_help: *const libc::c_char,
    pub target_port_given: libc::c_uint,
    pub output_file_given: libc::c_uint,
    pub blocklist_file_given: libc::c_uint,
    pub allowlist_file_given: libc::c_uint,
    pub list_of_ips_file_given: libc::c_uint,
    pub rate_given: libc::c_uint,
    pub bandwidth_given: libc::c_uint,
    pub batch_given: libc::c_uint,
    pub max_targets_given: libc::c_uint,
    pub max_runtime_given: libc::c_uint,
    pub max_results_given: libc::c_uint,
    pub probes_given: libc::c_uint,
    pub cooldown_time_given: libc::c_uint,
    pub seed_given: libc::c_uint,
    pub retries_given: libc::c_uint,
    pub dryrun_given: libc::c_uint,
    pub shards_given: libc::c_uint,
    pub shard_given: libc::c_uint,
    pub source_port_given: libc::c_uint,
    pub source_ip_given: libc::c_uint,
    pub gateway_mac_given: libc::c_uint,
    pub source_mac_given: libc::c_uint,
    pub interface_given: libc::c_uint,
    pub iplayer_given: libc::c_uint,
    pub probe_module_given: libc::c_uint,
    pub probe_args_given: libc::c_uint,
    pub probe_ttl_given: libc::c_uint,
    pub list_probe_modules_given: libc::c_uint,
    pub output_fields_given: libc::c_uint,
    pub output_module_given: libc::c_uint,
    pub output_args_given: libc::c_uint,
    pub output_filter_given: libc::c_uint,
    pub list_output_modules_given: libc::c_uint,
    pub list_output_fields_given: libc::c_uint,
    pub no_header_row_given: libc::c_uint,
    pub verbosity_given: libc::c_uint,
    pub log_file_given: libc::c_uint,
    pub log_directory_given: libc::c_uint,
    pub metadata_file_given: libc::c_uint,
    pub status_updates_file_given: libc::c_uint,
    pub quiet_given: libc::c_uint,
    pub disable_syslog_given: libc::c_uint,
    pub notes_given: libc::c_uint,
    pub user_metadata_given: libc::c_uint,
    pub config_given: libc::c_uint,
    pub max_sendto_failures_given: libc::c_uint,
    pub min_hitrate_given: libc::c_uint,
    pub sender_threads_given: libc::c_uint,
    pub cores_given: libc::c_uint,
    pub ignore_blocklist_errors_given: libc::c_uint,
    pub help_given: libc::c_uint,
    pub version_given: libc::c_uint,
    pub inputs: *mut *mut libc::c_char,
    pub inputs_num: libc::c_uint,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct cmdline_parser_params {
    pub override_0: libc::c_int,
    pub initialize: libc::c_int,
    pub check_required: libc::c_int,
    pub check_ambiguity: libc::c_int,
    pub print_errors: libc::c_int,
}
pub type shard_complete_cb = Option::<
    unsafe extern "C" fn(uint8_t, *mut libc::c_void) -> (),
>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct shard {
    pub state: shard_state,
    pub params: shard_params,
    pub current: uint64_t,
    pub iterations: uint64_t,
    pub thread_id: uint8_t,
    pub cb: shard_complete_cb,
    pub arg: *mut libc::c_void,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct shard_params {
    pub first: uint64_t,
    pub last: uint64_t,
    pub factor: uint64_t,
    pub modulus: uint64_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct shard_state {
    pub packets_sent: uint64_t,
    pub hosts_scanned: uint32_t,
    pub max_hosts: uint32_t,
    pub max_packets: uint32_t,
    pub hosts_blocklisted: uint32_t,
    pub hosts_allowlisted: uint32_t,
    pub packets_failed: uint32_t,
    pub first_scanned: uint32_t,
}
pub type shard_t = shard;
pub type iterator_t = iterator;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sock_t {
    pub sock: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct field_def {
    pub name: *const libc::c_char,
    pub type_0: *const libc::c_char,
    pub desc: *const libc::c_char,
}
pub type fielddef_t = field_def;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct fielddef_set {
    pub fielddefs: [fielddef_t; 128],
    pub len: libc::c_int,
}
pub type fielddefset_t = fielddef_set;
#[derive(Copy, Clone)]
#[repr(C)]
pub union field_val {
    pub ptr: *mut libc::c_void,
    pub num: uint64_t,
}
pub type field_val_t = field_val;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct field {
    pub name: *const libc::c_char,
    pub type_0: libc::c_int,
    pub free_: libc::c_int,
    pub len: size_t,
    pub value: field_val_t,
}
pub type field_t = field;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct fieldset {
    pub len: libc::c_int,
    pub fields: [field_t; 128],
    pub fds: *mut fielddefset_t,
    pub inner_type: libc::c_int,
    pub type_0: libc::c_int,
    pub free_: libc::c_int,
}
pub type fieldset_t = fieldset;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct translation {
    pub len: libc::c_int,
    pub translation: [libc::c_int; 128],
}
pub type translation_t = translation;
pub type operation = libc::c_uint;
pub const GT_EQ: operation = 7;
pub const LT_EQ: operation = 6;
pub const OR: operation = 5;
pub const AND: operation = 4;
pub const NEQ: operation = 3;
pub const EQ: operation = 2;
pub const LT: operation = 1;
pub const GT: operation = 0;
pub type node_type = libc::c_uint;
pub const INT: node_type = 3;
pub const STRING: node_type = 2;
pub const FIELD: node_type = 1;
pub const OP: node_type = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct field_id {
    pub index: libc::c_int,
    pub fieldname: *mut libc::c_char,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union node_value {
    pub field: field_id,
    pub string_literal: *mut libc::c_char,
    pub int_literal: uint64_t,
    pub op: operation,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct node_st {
    pub left_child: *mut node_st,
    pub right_child: *mut node_st,
    pub type_0: node_type,
    pub value: node_value,
}
pub type node_t = node_st;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct output_filter {
    pub expression: *mut node_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct probe_module {
    pub name: *const libc::c_char,
    pub max_packet_length: size_t,
    pub pcap_filter: *const libc::c_char,
    pub pcap_snaplen: size_t,
    pub port_args: uint8_t,
    pub global_initialize: probe_global_init_cb,
    pub thread_initialize: probe_thread_init_cb,
    pub make_packet: probe_make_packet_cb,
    pub print_packet: probe_print_packet_cb,
    pub validate_packet: probe_validate_packet_cb,
    pub process_packet: probe_classify_packet_cb,
    pub close: probe_close_cb,
    pub output_type: libc::c_int,
    pub fields: *mut fielddef_t,
    pub numfields: libc::c_int,
    pub helptext: *const libc::c_char,
}
pub type probe_close_cb = Option::<
    unsafe extern "C" fn(
        *mut state_conf,
        *mut state_send,
        *mut state_recv,
    ) -> libc::c_int,
>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct state_recv {
    pub success_total: uint32_t,
    pub success_unique: uint32_t,
    pub app_success_total: uint32_t,
    pub app_success_unique: uint32_t,
    pub cooldown_total: uint32_t,
    pub cooldown_unique: uint32_t,
    pub failure_total: uint32_t,
    pub filter_success: uint64_t,
    pub ip_fragments: uint32_t,
    pub validation_passed: uint32_t,
    pub validation_failed: uint32_t,
    pub complete: libc::c_int,
    pub start: libc::c_double,
    pub finish: libc::c_double,
    pub pcap_recv: uint32_t,
    pub pcap_drop: uint32_t,
    pub pcap_ifdrop: uint32_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct state_send {
    pub start: libc::c_double,
    pub finish: libc::c_double,
    pub packets_sent: uint64_t,
    pub hosts_scanned: uint64_t,
    pub blocklisted: uint64_t,
    pub allowlisted: uint64_t,
    pub warmup: libc::c_int,
    pub complete: libc::c_int,
    pub first_scanned: uint32_t,
    pub max_targets: uint32_t,
    pub sendto_failures: uint32_t,
    pub max_index: uint32_t,
    pub list_of_ips_pbm: *mut *mut uint8_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct state_conf {
    pub log_level: libc::c_int,
    pub target_port: port_h_t,
    pub source_port_first: port_h_t,
    pub source_port_last: port_h_t,
    pub max_targets: uint32_t,
    pub max_runtime: uint32_t,
    pub max_results: uint32_t,
    pub iface: *mut libc::c_char,
    pub rate: libc::c_int,
    pub bandwidth: uint64_t,
    pub cooldown_secs: libc::c_int,
    pub senders: uint8_t,
    pub batch: uint8_t,
    pub pin_cores_len: uint32_t,
    pub pin_cores: *mut uint32_t,
    pub seed_provided: libc::c_int,
    pub seed: uint64_t,
    pub aes: *mut aesrand_t,
    pub generator: uint32_t,
    pub shard_num: uint16_t,
    pub total_shards: uint16_t,
    pub packet_streams: libc::c_int,
    pub probe_module: *mut probe_module,
    pub output_module_name: *mut libc::c_char,
    pub output_module: *mut output_module,
    pub probe_args: *mut libc::c_char,
    pub probe_ttl: uint8_t,
    pub output_args: *mut libc::c_char,
    pub gw_mac: [macaddr_t; 6],
    pub hw_mac: [macaddr_t; 6],
    pub gw_ip: uint32_t,
    pub gw_mac_set: libc::c_int,
    pub hw_mac_set: libc::c_int,
    pub source_ip_addresses: [in_addr_t; 256],
    pub number_source_ips: uint32_t,
    pub send_ip_pkts: libc::c_int,
    pub output_filename: *mut libc::c_char,
    pub blocklist_filename: *mut libc::c_char,
    pub allowlist_filename: *mut libc::c_char,
    pub list_of_ips_filename: *mut libc::c_char,
    pub list_of_ips_count: uint32_t,
    pub metadata_filename: *mut libc::c_char,
    pub metadata_file: *mut FILE,
    pub notes: *mut libc::c_char,
    pub custom_metadata_str: *mut libc::c_char,
    pub destination_cidrs: *mut *mut libc::c_char,
    pub destination_cidrs_len: libc::c_int,
    pub raw_output_fields: *const libc::c_char,
    pub output_fields: *mut *const libc::c_char,
    pub filter: output_filter,
    pub output_filter_str: *mut libc::c_char,
    pub fsconf: fieldset_conf,
    pub output_fields_len: libc::c_int,
    pub log_file: *mut libc::c_char,
    pub log_directory: *mut libc::c_char,
    pub status_updates_file: *mut libc::c_char,
    pub dryrun: libc::c_int,
    pub quiet: libc::c_int,
    pub ignore_invalid_hosts: libc::c_int,
    pub syslog: libc::c_int,
    pub recv_ready: libc::c_int,
    pub num_retries: libc::c_int,
    pub total_allowed: uint64_t,
    pub total_disallowed: uint64_t,
    pub max_sendto_failures: libc::c_int,
    pub min_hitrate: libc::c_float,
    pub data_link_size: libc::c_int,
    pub default_mode: libc::c_int,
    pub no_header_row: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct fieldset_conf {
    pub defs: fielddefset_t,
    pub outdefs: fielddefset_t,
    pub translation: translation_t,
    pub success_index: libc::c_int,
    pub app_success_index: libc::c_int,
    pub classification_index: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct output_module {
    pub name: *const libc::c_char,
    pub supports_dynamic_output: libc::c_int,
    pub update_interval: libc::c_uint,
    pub init: output_init_cb,
    pub start: output_update_cb,
    pub update: output_update_cb,
    pub close: output_update_cb,
    pub process_ip: output_packet_cb,
    pub helptext: *const libc::c_char,
}
pub type output_packet_cb = Option::<
    unsafe extern "C" fn(*mut fieldset_t) -> libc::c_int,
>;
pub type output_update_cb = Option::<
    unsafe extern "C" fn(
        *mut state_conf,
        *mut state_send,
        *mut state_recv,
    ) -> libc::c_int,
>;
pub type output_init_cb = Option::<
    unsafe extern "C" fn(
        *mut state_conf,
        *mut *const libc::c_char,
        libc::c_int,
    ) -> libc::c_int,
>;
pub type probe_classify_packet_cb = Option::<
    unsafe extern "C" fn(
        *const u_char,
        uint32_t,
        *mut fieldset_t,
        *mut uint32_t,
        timespec,
    ) -> (),
>;
pub type probe_validate_packet_cb = Option::<
    unsafe extern "C" fn(
        *const ip,
        uint32_t,
        *mut uint32_t,
        *mut uint32_t,
    ) -> libc::c_int,
>;
pub type probe_print_packet_cb = Option::<
    unsafe extern "C" fn(*mut FILE, *mut libc::c_void) -> (),
>;
pub type probe_make_packet_cb = Option::<
    unsafe extern "C" fn(
        *mut libc::c_void,
        *mut size_t,
        ipaddr_n_t,
        ipaddr_n_t,
        uint8_t,
        *mut uint32_t,
        libc::c_int,
        *mut libc::c_void,
    ) -> libc::c_int,
>;
pub type probe_thread_init_cb = Option::<
    unsafe extern "C" fn(
        *mut libc::c_void,
        *mut macaddr_t,
        *mut macaddr_t,
        port_n_t,
        *mut *mut libc::c_void,
    ) -> libc::c_int,
>;
pub type probe_global_init_cb = Option::<
    unsafe extern "C" fn(*mut state_conf) -> libc::c_int,
>;
pub type output_module_t = output_module;
pub type probe_module_t = probe_module;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct send_arg {
    pub cpu: uint32_t,
    pub sock: sock_t,
    pub shard: *mut shard_t,
}
pub type send_arg_t = send_arg;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct recv_arg {
    pub cpu: uint32_t,
}
pub type recv_arg_t = recv_arg;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct mon_start_arg {
    pub cpu: uint32_t,
    pub it: *mut iterator_t,
    pub recv_ready_mutex: *mut pthread_mutex_t,
}
pub type mon_start_arg_t = mon_start_arg;
#[inline]
unsafe extern "C" fn atoi(mut __nptr: *const libc::c_char) -> libc::c_int {
    return strtol(
        __nptr,
        0 as *mut libc::c_void as *mut *mut libc::c_char,
        10 as libc::c_int,
    ) as libc::c_int;
}
pub static mut recv_ready_mutex: pthread_mutex_t = pthread_mutex_t {
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
};
pub static mut default_help_text: *const libc::c_char = b"By default, ZMap prints out unique, successful IP addresses (e.g., SYN-ACK from a TCP SYN scan) in ASCII form (e.g., 192.168.1.5) to stdout or the specified output file. Internally this is handled by the \"csv\" output module and is equivalent to running zmap --output-module=csv --output-fields=saddr --output-filter=\"success = 1 && repeat = 0\" --no-header-row.\0"
    as *const u8 as *const libc::c_char;
unsafe extern "C" fn start_send(mut arg: *mut libc::c_void) -> *mut libc::c_void {
    let mut s: *mut send_arg_t = arg as *mut send_arg_t;
    log_debug(
        b"zmap\0" as *const u8 as *const libc::c_char,
        b"Pinning a send thread to core %u\0" as *const u8 as *const libc::c_char,
        (*s).cpu,
    );
    set_cpu((*s).cpu);
    send_run((*s).sock, (*s).shard);
    free(s as *mut libc::c_void);
    return 0 as *mut libc::c_void;
}
unsafe extern "C" fn start_recv(mut arg: *mut libc::c_void) -> *mut libc::c_void {
    let mut r: *mut recv_arg_t = arg as *mut recv_arg_t;
    log_debug(
        b"zmap\0" as *const u8 as *const libc::c_char,
        b"Pinning receive thread to core %u\0" as *const u8 as *const libc::c_char,
        (*r).cpu,
    );
    set_cpu((*r).cpu);
    recv_run(&mut recv_ready_mutex);
    return 0 as *mut libc::c_void;
}
unsafe extern "C" fn start_mon(mut arg: *mut libc::c_void) -> *mut libc::c_void {
    let mut mon_arg: *mut mon_start_arg_t = arg as *mut mon_start_arg_t;
    log_debug(
        b"zmap\0" as *const u8 as *const libc::c_char,
        b"Pinning monitor thread to core %u\0" as *const u8 as *const libc::c_char,
        (*mon_arg).cpu,
    );
    set_cpu((*mon_arg).cpu);
    monitor_run((*mon_arg).it, (*mon_arg).recv_ready_mutex);
    free(mon_arg as *mut libc::c_void);
    return 0 as *mut libc::c_void;
}
unsafe extern "C" fn start_zmap() {
    if (zconf.iface).is_null() {
        zconf.iface = get_default_iface();
        if !(zconf.iface).is_null() {} else {
            __assert_fail(
                b"zconf.iface\0" as *const u8 as *const libc::c_char,
                b"zmap.c\0" as *const u8 as *const libc::c_char,
                119 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 22],
                    &[libc::c_char; 22],
                >(b"void start_zmap(void)\0"))
                    .as_ptr(),
            );
        }
        'c_9482: {
            if !(zconf.iface).is_null() {} else {
                __assert_fail(
                    b"zconf.iface\0" as *const u8 as *const libc::c_char,
                    b"zmap.c\0" as *const u8 as *const libc::c_char,
                    119 as libc::c_int as libc::c_uint,
                    (*::std::mem::transmute::<
                        &[u8; 22],
                        &[libc::c_char; 22],
                    >(b"void start_zmap(void)\0"))
                        .as_ptr(),
                );
            }
        };
        log_debug(
            b"zmap\0" as *const u8 as *const libc::c_char,
            b"no interface provided. will use default interface (%s).\0" as *const u8
                as *const libc::c_char,
            zconf.iface,
        );
    }
    if zconf.number_source_ips == 0 as libc::c_int as libc::c_uint {
        let mut default_ip: in_addr = in_addr { s_addr: 0 };
        if get_iface_ip(zconf.iface, &mut default_ip) < 0 as libc::c_int {
            log_fatal(
                b"zmap\0" as *const u8 as *const libc::c_char,
                b"could not detect default IP address for %s. Try specifying a source address (-S).\0"
                    as *const u8 as *const libc::c_char,
                zconf.iface,
            );
        }
        zconf.source_ip_addresses[0 as libc::c_int as usize] = default_ip.s_addr;
        zconf.number_source_ips = (zconf.number_source_ips).wrapping_add(1);
        zconf.number_source_ips;
        log_debug(
            b"zmap\0" as *const u8 as *const libc::c_char,
            b"no source IP address given. will use default address: %s.\0" as *const u8
                as *const libc::c_char,
            inet_ntoa(default_ip),
        );
    }
    if zconf.gw_mac_set == 0 {
        let mut gw_ip: in_addr = in_addr { s_addr: 0 };
        memset(
            &mut gw_ip as *mut in_addr as *mut libc::c_void,
            0 as libc::c_int,
            ::std::mem::size_of::<in_addr>() as libc::c_ulong,
        );
        if get_default_gw(&mut gw_ip, zconf.iface) < 0 as libc::c_int {
            log_fatal(
                b"zmap\0" as *const u8 as *const libc::c_char,
                b"could not detect default gateway address for %s. Try setting default gateway mac address (-G). If this is a newly launched machine, try completing an outgoing network connection (e.g. curl https://zmap.io), and trying again.\0"
                    as *const u8 as *const libc::c_char,
                zconf.iface,
            );
        }
        log_debug(
            b"zmap\0" as *const u8 as *const libc::c_char,
            b"found gateway IP %s on %s\0" as *const u8 as *const libc::c_char,
            inet_ntoa(gw_ip),
            zconf.iface,
        );
        zconf.gw_ip = gw_ip.s_addr;
        memset(
            &mut zconf.gw_mac as *mut [macaddr_t; 6] as *mut libc::c_void,
            0 as libc::c_int,
            6 as libc::c_int as libc::c_ulong,
        );
        if get_hw_addr(&mut gw_ip, zconf.iface, (zconf.gw_mac).as_mut_ptr()) != 0 {
            log_fatal(
                b"zmap\0" as *const u8 as *const libc::c_char,
                b"could not detect GW MAC address for %s on %s. Try setting default gateway mac address (-G), or run \"arp <gateway_ip>\" in terminal. If this is a newly launched machine, try completing an outgoing network connection (e.g. curl https://zmap.io), and trying again.\0"
                    as *const u8 as *const libc::c_char,
                inet_ntoa(gw_ip),
                zconf.iface,
            );
        }
        zconf.gw_mac_set = 1 as libc::c_int;
    }
    log_debug(
        b"send\0" as *const u8 as *const libc::c_char,
        b"gateway MAC address %02x:%02x:%02x:%02x:%02x:%02x\0" as *const u8
            as *const libc::c_char,
        zconf.gw_mac[0 as libc::c_int as usize] as libc::c_int,
        zconf.gw_mac[1 as libc::c_int as usize] as libc::c_int,
        zconf.gw_mac[2 as libc::c_int as usize] as libc::c_int,
        zconf.gw_mac[3 as libc::c_int as usize] as libc::c_int,
        zconf.gw_mac[4 as libc::c_int as usize] as libc::c_int,
        zconf.gw_mac[5 as libc::c_int as usize] as libc::c_int,
    );
    if !(zconf.output_module).is_null()
        && !(b"no output module set\0" as *const u8 as *const libc::c_char).is_null()
    {} else {
        __assert_fail(
            b"zconf.output_module && \"no output module set\"\0" as *const u8
                as *const libc::c_char,
            b"zmap.c\0" as *const u8 as *const libc::c_char,
            170 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 22],
                &[libc::c_char; 22],
            >(b"void start_zmap(void)\0"))
                .as_ptr(),
        );
    }
    'c_9190: {
        if !(zconf.output_module).is_null()
            && !(b"no output module set\0" as *const u8 as *const libc::c_char).is_null()
        {} else {
            __assert_fail(
                b"zconf.output_module && \"no output module set\"\0" as *const u8
                    as *const libc::c_char,
                b"zmap.c\0" as *const u8 as *const libc::c_char,
                170 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 22],
                    &[libc::c_char; 22],
                >(b"void start_zmap(void)\0"))
                    .as_ptr(),
            );
        }
    };
    log_debug(
        b"zmap\0" as *const u8 as *const libc::c_char,
        b"output module: %s\0" as *const u8 as *const libc::c_char,
        (*zconf.output_module).name,
    );
    if !(zconf.output_module).is_null() && ((*zconf.output_module).init).is_some() {
        if ((*zconf.output_module).init)
            .unwrap()(&mut zconf, zconf.output_fields, zconf.output_fields_len) != 0
        {
            log_fatal(
                b"zmap\0" as *const u8 as *const libc::c_char,
                b"output module did not initialize successfully.\0" as *const u8
                    as *const libc::c_char,
            );
        }
    }
    let mut it: *mut iterator_t = send_init();
    if it.is_null() {
        log_fatal(
            b"zmap\0" as *const u8 as *const libc::c_char,
            b"unable to initialize sending component\0" as *const u8
                as *const libc::c_char,
        );
    }
    if !(zconf.output_module).is_null() && ((*zconf.output_module).start).is_some() {
        ((*zconf.output_module).start).unwrap()(&mut zconf, &mut zsend, &mut zrecv);
    }
    let mut cpu: uint32_t = 0 as libc::c_int as uint32_t;
    let mut tsend: *mut pthread_t = 0 as *mut pthread_t;
    let mut trecv: pthread_t = 0;
    let mut tmon: pthread_t = 0;
    let mut r: libc::c_int = 0;
    if zconf.dryrun == 0 {
        let mut recv_arg: *mut recv_arg_t = xmalloc(
            ::std::mem::size_of::<recv_arg_t>() as libc::c_ulong,
        ) as *mut recv_arg_t;
        (*recv_arg)
            .cpu = *(zconf.pin_cores)
            .offset(cpu.wrapping_rem(zconf.pin_cores_len) as isize);
        cpu = (cpu as libc::c_uint).wrapping_add(1 as libc::c_int as libc::c_uint)
            as uint32_t as uint32_t;
        r = pthread_create(
            &mut trecv,
            0 as *const pthread_attr_t,
            Some(
                start_recv
                    as unsafe extern "C" fn(*mut libc::c_void) -> *mut libc::c_void,
            ),
            recv_arg as *mut libc::c_void,
        );
        if r != 0 as libc::c_int {
            log_fatal(
                b"zmap\0" as *const u8 as *const libc::c_char,
                b"unable to create recv thread\0" as *const u8 as *const libc::c_char,
            );
        }
        loop {
            pthread_mutex_lock(&mut recv_ready_mutex);
            if zconf.recv_ready != 0 {
                pthread_mutex_unlock(&mut recv_ready_mutex);
                break;
            } else {
                pthread_mutex_unlock(&mut recv_ready_mutex);
            }
        }
    }
    tsend = xmalloc(
        (zconf.senders as libc::c_ulong)
            .wrapping_mul(::std::mem::size_of::<pthread_t>() as libc::c_ulong),
    ) as *mut pthread_t;
    let mut i: uint8_t = 0 as libc::c_int as uint8_t;
    while (i as libc::c_int) < zconf.senders as libc::c_int {
        let mut sock: sock_t = sock_t { sock: 0 };
        if zconf.dryrun != 0 {
            sock = get_dryrun_socket();
        } else {
            sock = get_socket(i as uint32_t);
        }
        let mut arg: *mut send_arg_t = xmalloc(
            ::std::mem::size_of::<send_arg_t>() as libc::c_ulong,
        ) as *mut send_arg_t;
        (*arg).sock = sock;
        (*arg).shard = get_shard(it, i);
        (*arg)
            .cpu = *(zconf.pin_cores)
            .offset(cpu.wrapping_rem(zconf.pin_cores_len) as isize);
        cpu = (cpu as libc::c_uint).wrapping_add(1 as libc::c_int as libc::c_uint)
            as uint32_t as uint32_t;
        let mut r_0: libc::c_int = pthread_create(
            &mut *tsend.offset(i as isize),
            0 as *const pthread_attr_t,
            Some(
                start_send
                    as unsafe extern "C" fn(*mut libc::c_void) -> *mut libc::c_void,
            ),
            arg as *mut libc::c_void,
        );
        if r_0 != 0 as libc::c_int {
            log_fatal(
                b"zmap\0" as *const u8 as *const libc::c_char,
                b"unable to create send thread\0" as *const u8 as *const libc::c_char,
            );
        }
        i = i.wrapping_add(1);
        i;
    }
    log_debug(
        b"zmap\0" as *const u8 as *const libc::c_char,
        b"%d sender threads spawned\0" as *const u8 as *const libc::c_char,
        zconf.senders as libc::c_int,
    );
    if zconf.dryrun == 0 {
        monitor_init();
        let mut mon_arg: *mut mon_start_arg_t = xmalloc(
            ::std::mem::size_of::<mon_start_arg_t>() as libc::c_ulong,
        ) as *mut mon_start_arg_t;
        (*mon_arg).it = it;
        (*mon_arg).recv_ready_mutex = &mut recv_ready_mutex;
        (*mon_arg)
            .cpu = *(zconf.pin_cores)
            .offset(cpu.wrapping_rem(zconf.pin_cores_len) as isize);
        let mut r_1: libc::c_int = pthread_create(
            &mut tmon,
            0 as *const pthread_attr_t,
            Some(
                start_mon as unsafe extern "C" fn(*mut libc::c_void) -> *mut libc::c_void,
            ),
            mon_arg as *mut libc::c_void,
        );
        if r_1 != 0 as libc::c_int {
            log_fatal(
                b"zmap\0" as *const u8 as *const libc::c_char,
                b"unable to create monitor thread\0" as *const u8 as *const libc::c_char,
            );
        }
    }
    drop_privs();
    let mut i_0: uint8_t = 0 as libc::c_int as uint8_t;
    while (i_0 as libc::c_int) < zconf.senders as libc::c_int {
        let mut r_2: libc::c_int = pthread_join(
            *tsend.offset(i_0 as isize),
            0 as *mut *mut libc::c_void,
        );
        if r_2 != 0 as libc::c_int {
            log_fatal(
                b"zmap\0" as *const u8 as *const libc::c_char,
                b"unable to join send thread\0" as *const u8 as *const libc::c_char,
            );
        }
        i_0 = i_0.wrapping_add(1);
        i_0;
    }
    log_debug(
        b"zmap\0" as *const u8 as *const libc::c_char,
        b"senders finished\0" as *const u8 as *const libc::c_char,
    );
    if zconf.dryrun == 0 {
        r = pthread_join(trecv, 0 as *mut *mut libc::c_void);
        if r != 0 as libc::c_int {
            log_fatal(
                b"zmap\0" as *const u8 as *const libc::c_char,
                b"unable to join recv thread\0" as *const u8 as *const libc::c_char,
            );
        }
        if zconf.quiet == 0 || !(zconf.status_updates_file).is_null() {
            pthread_join(tmon, 0 as *mut *mut libc::c_void);
            if r != 0 as libc::c_int {
                log_fatal(
                    b"zmap\0" as *const u8 as *const libc::c_char,
                    b"unable to join monitor thread\0" as *const u8
                        as *const libc::c_char,
                );
            }
        }
    }
    if !(zconf.metadata_filename).is_null() {
        json_metadata(zconf.metadata_file);
    }
    if !(zconf.output_module).is_null() && ((*zconf.output_module).close).is_some() {
        ((*zconf.output_module).close).unwrap()(&mut zconf, &mut zsend, &mut zrecv);
    }
    if !(zconf.probe_module).is_null() && ((*zconf.probe_module).close).is_some() {
        ((*zconf.probe_module).close).unwrap()(&mut zconf, &mut zsend, &mut zrecv);
    }
    log_info(
        b"zmap\0" as *const u8 as *const libc::c_char,
        b"completed\0" as *const u8 as *const libc::c_char,
    );
}
unsafe fn main_0(
    mut argc: libc::c_int,
    mut argv: *mut *mut libc::c_char,
) -> libc::c_int {
    let mut args: gengetopt_args_info = gengetopt_args_info {
        target_port_arg: 0,
        target_port_orig: 0 as *mut libc::c_char,
        target_port_help: 0 as *const libc::c_char,
        output_file_arg: 0 as *mut libc::c_char,
        output_file_orig: 0 as *mut libc::c_char,
        output_file_help: 0 as *const libc::c_char,
        blocklist_file_arg: 0 as *mut libc::c_char,
        blocklist_file_orig: 0 as *mut libc::c_char,
        blocklist_file_help: 0 as *const libc::c_char,
        allowlist_file_arg: 0 as *mut libc::c_char,
        allowlist_file_orig: 0 as *mut libc::c_char,
        allowlist_file_help: 0 as *const libc::c_char,
        list_of_ips_file_arg: 0 as *mut libc::c_char,
        list_of_ips_file_orig: 0 as *mut libc::c_char,
        list_of_ips_file_help: 0 as *const libc::c_char,
        rate_arg: 0,
        rate_orig: 0 as *mut libc::c_char,
        rate_help: 0 as *const libc::c_char,
        bandwidth_arg: 0 as *mut libc::c_char,
        bandwidth_orig: 0 as *mut libc::c_char,
        bandwidth_help: 0 as *const libc::c_char,
        batch_arg: 0,
        batch_orig: 0 as *mut libc::c_char,
        batch_help: 0 as *const libc::c_char,
        max_targets_arg: 0 as *mut libc::c_char,
        max_targets_orig: 0 as *mut libc::c_char,
        max_targets_help: 0 as *const libc::c_char,
        max_runtime_arg: 0,
        max_runtime_orig: 0 as *mut libc::c_char,
        max_runtime_help: 0 as *const libc::c_char,
        max_results_arg: 0,
        max_results_orig: 0 as *mut libc::c_char,
        max_results_help: 0 as *const libc::c_char,
        probes_arg: 0,
        probes_orig: 0 as *mut libc::c_char,
        probes_help: 0 as *const libc::c_char,
        cooldown_time_arg: 0,
        cooldown_time_orig: 0 as *mut libc::c_char,
        cooldown_time_help: 0 as *const libc::c_char,
        seed_arg: 0,
        seed_orig: 0 as *mut libc::c_char,
        seed_help: 0 as *const libc::c_char,
        retries_arg: 0,
        retries_orig: 0 as *mut libc::c_char,
        retries_help: 0 as *const libc::c_char,
        dryrun_help: 0 as *const libc::c_char,
        shards_arg: 0,
        shards_orig: 0 as *mut libc::c_char,
        shards_help: 0 as *const libc::c_char,
        shard_arg: 0,
        shard_orig: 0 as *mut libc::c_char,
        shard_help: 0 as *const libc::c_char,
        source_port_arg: 0 as *mut libc::c_char,
        source_port_orig: 0 as *mut libc::c_char,
        source_port_help: 0 as *const libc::c_char,
        source_ip_arg: 0 as *mut libc::c_char,
        source_ip_orig: 0 as *mut libc::c_char,
        source_ip_help: 0 as *const libc::c_char,
        gateway_mac_arg: 0 as *mut libc::c_char,
        gateway_mac_orig: 0 as *mut libc::c_char,
        gateway_mac_help: 0 as *const libc::c_char,
        source_mac_arg: 0 as *mut libc::c_char,
        source_mac_orig: 0 as *mut libc::c_char,
        source_mac_help: 0 as *const libc::c_char,
        interface_arg: 0 as *mut libc::c_char,
        interface_orig: 0 as *mut libc::c_char,
        interface_help: 0 as *const libc::c_char,
        iplayer_help: 0 as *const libc::c_char,
        probe_module_arg: 0 as *mut libc::c_char,
        probe_module_orig: 0 as *mut libc::c_char,
        probe_module_help: 0 as *const libc::c_char,
        probe_args_arg: 0 as *mut libc::c_char,
        probe_args_orig: 0 as *mut libc::c_char,
        probe_args_help: 0 as *const libc::c_char,
        probe_ttl_arg: 0,
        probe_ttl_orig: 0 as *mut libc::c_char,
        probe_ttl_help: 0 as *const libc::c_char,
        list_probe_modules_help: 0 as *const libc::c_char,
        output_fields_arg: 0 as *mut libc::c_char,
        output_fields_orig: 0 as *mut libc::c_char,
        output_fields_help: 0 as *const libc::c_char,
        output_module_arg: 0 as *mut libc::c_char,
        output_module_orig: 0 as *mut libc::c_char,
        output_module_help: 0 as *const libc::c_char,
        output_args_arg: 0 as *mut libc::c_char,
        output_args_orig: 0 as *mut libc::c_char,
        output_args_help: 0 as *const libc::c_char,
        output_filter_arg: 0 as *mut libc::c_char,
        output_filter_orig: 0 as *mut libc::c_char,
        output_filter_help: 0 as *const libc::c_char,
        list_output_modules_help: 0 as *const libc::c_char,
        list_output_fields_help: 0 as *const libc::c_char,
        no_header_row_help: 0 as *const libc::c_char,
        verbosity_arg: 0,
        verbosity_orig: 0 as *mut libc::c_char,
        verbosity_help: 0 as *const libc::c_char,
        log_file_arg: 0 as *mut libc::c_char,
        log_file_orig: 0 as *mut libc::c_char,
        log_file_help: 0 as *const libc::c_char,
        log_directory_arg: 0 as *mut libc::c_char,
        log_directory_orig: 0 as *mut libc::c_char,
        log_directory_help: 0 as *const libc::c_char,
        metadata_file_arg: 0 as *mut libc::c_char,
        metadata_file_orig: 0 as *mut libc::c_char,
        metadata_file_help: 0 as *const libc::c_char,
        status_updates_file_arg: 0 as *mut libc::c_char,
        status_updates_file_orig: 0 as *mut libc::c_char,
        status_updates_file_help: 0 as *const libc::c_char,
        quiet_help: 0 as *const libc::c_char,
        disable_syslog_help: 0 as *const libc::c_char,
        notes_arg: 0 as *mut libc::c_char,
        notes_orig: 0 as *mut libc::c_char,
        notes_help: 0 as *const libc::c_char,
        user_metadata_arg: 0 as *mut libc::c_char,
        user_metadata_orig: 0 as *mut libc::c_char,
        user_metadata_help: 0 as *const libc::c_char,
        config_arg: 0 as *mut libc::c_char,
        config_orig: 0 as *mut libc::c_char,
        config_help: 0 as *const libc::c_char,
        max_sendto_failures_arg: 0,
        max_sendto_failures_orig: 0 as *mut libc::c_char,
        max_sendto_failures_help: 0 as *const libc::c_char,
        min_hitrate_arg: 0.,
        min_hitrate_orig: 0 as *mut libc::c_char,
        min_hitrate_help: 0 as *const libc::c_char,
        sender_threads_arg: 0,
        sender_threads_orig: 0 as *mut libc::c_char,
        sender_threads_help: 0 as *const libc::c_char,
        cores_arg: 0 as *mut libc::c_char,
        cores_orig: 0 as *mut libc::c_char,
        cores_help: 0 as *const libc::c_char,
        ignore_blocklist_errors_help: 0 as *const libc::c_char,
        help_help: 0 as *const libc::c_char,
        version_help: 0 as *const libc::c_char,
        target_port_given: 0,
        output_file_given: 0,
        blocklist_file_given: 0,
        allowlist_file_given: 0,
        list_of_ips_file_given: 0,
        rate_given: 0,
        bandwidth_given: 0,
        batch_given: 0,
        max_targets_given: 0,
        max_runtime_given: 0,
        max_results_given: 0,
        probes_given: 0,
        cooldown_time_given: 0,
        seed_given: 0,
        retries_given: 0,
        dryrun_given: 0,
        shards_given: 0,
        shard_given: 0,
        source_port_given: 0,
        source_ip_given: 0,
        gateway_mac_given: 0,
        source_mac_given: 0,
        interface_given: 0,
        iplayer_given: 0,
        probe_module_given: 0,
        probe_args_given: 0,
        probe_ttl_given: 0,
        list_probe_modules_given: 0,
        output_fields_given: 0,
        output_module_given: 0,
        output_args_given: 0,
        output_filter_given: 0,
        list_output_modules_given: 0,
        list_output_fields_given: 0,
        no_header_row_given: 0,
        verbosity_given: 0,
        log_file_given: 0,
        log_directory_given: 0,
        metadata_file_given: 0,
        status_updates_file_given: 0,
        quiet_given: 0,
        disable_syslog_given: 0,
        notes_given: 0,
        user_metadata_given: 0,
        config_given: 0,
        max_sendto_failures_given: 0,
        min_hitrate_given: 0,
        sender_threads_given: 0,
        cores_given: 0,
        ignore_blocklist_errors_given: 0,
        help_given: 0,
        version_given: 0,
        inputs: 0 as *mut *mut libc::c_char,
        inputs_num: 0,
    };
    let mut params: *mut cmdline_parser_params = 0 as *mut cmdline_parser_params;
    params = cmdline_parser_params_create();
    (*params).initialize = 1 as libc::c_int;
    (*params).override_0 = 0 as libc::c_int;
    (*params).check_required = 0 as libc::c_int;
    let mut config_loaded: libc::c_int = 0 as libc::c_int;
    if cmdline_parser_ext(argc, argv, &mut args, params) != 0 as libc::c_int {
        exit(0 as libc::c_int);
    }
    if args.config_given != 0 || file_exists(args.config_arg) != 0 {
        (*params).initialize = 0 as libc::c_int;
        (*params).override_0 = 0 as libc::c_int;
        if cmdline_parser_config_file(args.config_arg, &mut args, params)
            != 0 as libc::c_int
        {
            exit(1 as libc::c_int);
        }
        config_loaded = 1 as libc::c_int;
    }
    init_empty_global_configuration(&mut zconf);
    zconf.log_level = args.verbosity_arg;
    zconf.log_file = args.log_file_arg;
    zconf.log_directory = args.log_directory_arg;
    if args.disable_syslog_given != 0 {
        zconf.syslog = 0 as libc::c_int;
    } else {
        zconf.syslog = 1 as libc::c_int;
    }
    if !(zconf.log_file).is_null() && !(zconf.log_directory).is_null() {
        log_init(
            stderr,
            zconf.log_level as LogLevel,
            zconf.syslog,
            b"zmap\0" as *const u8 as *const libc::c_char,
        );
        log_fatal(
            b"zmap\0" as *const u8 as *const libc::c_char,
            b"log-file and log-directory cannot specified simultaneously.\0" as *const u8
                as *const libc::c_char,
        );
    }
    let mut log_location: *mut FILE = 0 as *mut FILE;
    if !(zconf.log_file).is_null() {
        log_location = fopen(zconf.log_file, b"w\0" as *const u8 as *const libc::c_char);
    } else if !(zconf.log_directory).is_null() {
        let mut now: time_t = 0;
        time(&mut now);
        let mut local: *mut tm = localtime(&mut now);
        let mut path: [libc::c_char; 100] = [0; 100];
        strftime(
            path.as_mut_ptr(),
            100 as libc::c_int as size_t,
            b"zmap-%Y-%m-%dT%H%M%S%z.log\0" as *const u8 as *const libc::c_char,
            local,
        );
        let mut fullpath: *mut libc::c_char = xmalloc(
            (strlen(zconf.log_directory))
                .wrapping_add(strlen(path.as_mut_ptr()))
                .wrapping_add(2 as libc::c_int as libc::c_ulong),
        ) as *mut libc::c_char;
        sprintf(
            fullpath,
            b"%s/%s\0" as *const u8 as *const libc::c_char,
            zconf.log_directory,
            path.as_mut_ptr(),
        );
        log_location = fopen(fullpath, b"w\0" as *const u8 as *const libc::c_char);
        free(fullpath as *mut libc::c_void);
    } else {
        log_location = stderr;
    }
    if log_location.is_null() {
        log_init(
            stderr,
            zconf.log_level as LogLevel,
            zconf.syslog,
            b"zmap\0" as *const u8 as *const libc::c_char,
        );
        log_fatal(
            b"zmap\0" as *const u8 as *const libc::c_char,
            b"unable to open specified log file: %s\0" as *const u8
                as *const libc::c_char,
            strerror(*__errno_location()),
        );
    }
    log_init(
        log_location,
        zconf.log_level as LogLevel,
        zconf.syslog,
        b"zmap\0" as *const u8 as *const libc::c_char,
    );
    log_debug(
        b"zmap\0" as *const u8 as *const libc::c_char,
        b"zmap main thread started\0" as *const u8 as *const libc::c_char,
    );
    if config_loaded != 0 {
        log_debug(
            b"zmap\0" as *const u8 as *const libc::c_char,
            b"Loaded configuration file %s\0" as *const u8 as *const libc::c_char,
            args.config_arg,
        );
    }
    if zconf.syslog != 0 {
        log_debug(
            b"zmap\0" as *const u8 as *const libc::c_char,
            b"syslog support enabled\0" as *const u8 as *const libc::c_char,
        );
    } else {
        log_info(
            b"zmap\0" as *const u8 as *const libc::c_char,
            b"syslog support disabled\0" as *const u8 as *const libc::c_char,
        );
    }
    log_debug(
        b"zmap\0" as *const u8 as *const libc::c_char,
        b"requested ouput-module: %s\0" as *const u8 as *const libc::c_char,
        args.output_module_arg,
    );
    zconf
        .default_mode = !(args.output_module_given != 0 || args.output_filter_given != 0
        || args.output_fields_given != 0) as libc::c_int;
    if zconf.default_mode != 0 {
        log_info(
            b"zmap\0" as *const u8 as *const libc::c_char,
            b"By default, ZMap will output the unique IP addresses of hosts that respond successfully (e.g., SYN-ACK packet). This is equivalent to running ZMap with the following flags: --output-module=csv --output-fields=saddr --output-filter='success=1 && repeat=0' --no-header-row. If you want all responses, explicitly set an output module or set --output-filter=\"\".\0"
                as *const u8 as *const libc::c_char,
        );
        zconf
            .output_module = get_output_module_by_name(
            b"csv\0" as *const u8 as *const libc::c_char,
        );
        zconf.output_module_name = strdup(b"csv\0" as *const u8 as *const libc::c_char);
        zconf.no_header_row = 1 as libc::c_int;
    } else if args.output_module_given == 0 {
        log_debug(
            b"zmap\0" as *const u8 as *const libc::c_char,
            b"No output module provided. Will use csv.\0" as *const u8
                as *const libc::c_char,
        );
        zconf
            .output_module = get_output_module_by_name(
            b"csv\0" as *const u8 as *const libc::c_char,
        );
        zconf.output_module_name = strdup(b"csv\0" as *const u8 as *const libc::c_char);
    } else {
        zconf.output_module = get_output_module_by_name(args.output_module_arg);
        if (zconf.output_module).is_null() {
            log_fatal(
                b"zmap\0" as *const u8 as *const libc::c_char,
                b"specified output module (%s) does not exist\n\0" as *const u8
                    as *const libc::c_char,
                args.output_module_arg,
            );
        }
        zconf.output_module_name = strdup(args.output_module_arg);
    }
    zconf.probe_module = get_probe_module_by_name(args.probe_module_arg);
    if (zconf.probe_module).is_null() {
        log_fatal(
            b"zmap\0" as *const u8 as *const libc::c_char,
            b"specified probe module (%s) does not exist\n\0" as *const u8
                as *const libc::c_char,
            args.probe_module_arg,
        );
    }
    if (*zconf.probe_module).output_type == 2 as libc::c_int
        && (*zconf.output_module).supports_dynamic_output == 0
    {
        log_fatal(
            b"zmap\0" as *const u8 as *const libc::c_char,
            b"specified probe module (%s) requires dynamic output support, which output module (%s) does not support. Most likely you want to use JSON output.\0"
                as *const u8 as *const libc::c_char,
            args.probe_module_arg,
            args.output_module_arg,
        );
    }
    if args.help_given != 0 {
        cmdline_parser_print_help();
        printf(
            b"\nProbe Module (%s) Help:\n\0" as *const u8 as *const libc::c_char,
            (*zconf.probe_module).name,
        );
        if !((*zconf.probe_module).helptext).is_null() {
            fprintw(stdout, (*zconf.probe_module).helptext, 80 as libc::c_int as size_t);
        } else {
            printf(b"no help text available\n\0" as *const u8 as *const libc::c_char);
        }
        if !(zconf.output_module).is_null()
            && !(b"no output module set\0" as *const u8 as *const libc::c_char).is_null()
        {} else {
            __assert_fail(
                b"zconf.output_module && \"no output module set\"\0" as *const u8
                    as *const libc::c_char,
                b"zmap.c\0" as *const u8 as *const libc::c_char,
                452 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 23],
                    &[libc::c_char; 23],
                >(b"int main(int, char **)\0"))
                    .as_ptr(),
            );
        }
        'c_11862: {
            if !(zconf.output_module).is_null()
                && !(b"no output module set\0" as *const u8 as *const libc::c_char)
                    .is_null()
            {} else {
                __assert_fail(
                    b"zconf.output_module && \"no output module set\"\0" as *const u8
                        as *const libc::c_char,
                    b"zmap.c\0" as *const u8 as *const libc::c_char,
                    452 as libc::c_int as libc::c_uint,
                    (*::std::mem::transmute::<
                        &[u8; 23],
                        &[libc::c_char; 23],
                    >(b"int main(int, char **)\0"))
                        .as_ptr(),
                );
            }
        };
        let mut module_name: *const libc::c_char = if zconf.default_mode != 0 {
            b"Default\0" as *const u8 as *const libc::c_char
        } else {
            (*zconf.output_module).name
        };
        printf(
            b"\nOutput Module (%s) Help:\n\0" as *const u8 as *const libc::c_char,
            module_name,
        );
        if zconf.default_mode != 0 {
            fprintw(stdout, default_help_text, 80 as libc::c_int as size_t);
        } else if !((*zconf.output_module).helptext).is_null() {
            fprintw(
                stdout,
                (*zconf.output_module).helptext,
                80 as libc::c_int as size_t,
            );
        } else {
            printf(b"no help text available\n\0" as *const u8 as *const libc::c_char);
        }
        exit(0 as libc::c_int);
    }
    if args.version_given != 0 {
        cmdline_parser_print_version();
        exit(0 as libc::c_int);
    }
    if args.list_output_modules_given != 0 {
        print_output_modules();
        exit(0 as libc::c_int);
    }
    if args.list_probe_modules_given != 0 {
        print_probe_modules();
        exit(0 as libc::c_int);
    }
    if args.iplayer_given != 0 {
        zconf.send_ip_pkts = 1 as libc::c_int;
        zconf.gw_mac_set = 1 as libc::c_int;
        memset(
            (zconf.gw_mac).as_mut_ptr() as *mut libc::c_void,
            0 as libc::c_int,
            6 as libc::c_int as libc::c_ulong,
        );
    }
    if cmdline_parser_required(&mut args, b"zmap\0" as *const u8 as *const libc::c_char)
        != 0 as libc::c_int
    {
        exit(1 as libc::c_int);
    }
    memset(
        &mut zconf.fsconf as *mut fieldset_conf as *mut libc::c_void,
        0 as libc::c_int,
        ::std::mem::size_of::<fieldset_conf>() as libc::c_ulong,
    );
    let mut fds: *mut fielddefset_t = &mut zconf.fsconf.defs;
    gen_fielddef_set(
        fds,
        &mut ip_fields as *mut [fielddef_t; 6] as *mut fielddef_t,
        ip_fields_len,
    );
    gen_fielddef_set(fds, (*zconf.probe_module).fields, (*zconf.probe_module).numfields);
    gen_fielddef_set(
        fds,
        &mut sys_fields as *mut [fielddef_t; 5] as *mut fielddef_t,
        sys_fields_len,
    );
    if args.list_output_fields_given != 0 {
        let mut i: libc::c_int = 0 as libc::c_int;
        while i < (*fds).len {
            printf(
                b"%-15s %6s: %s\n\0" as *const u8 as *const libc::c_char,
                (*fds).fielddefs[i as usize].name,
                (*fds).fielddefs[i as usize].type_0,
                (*fds).fielddefs[i as usize].desc,
            );
            i += 1;
            i;
        }
        exit(0 as libc::c_int);
    }
    zconf
        .fsconf
        .success_index = fds_get_index_by_name(
        fds,
        b"success\0" as *const u8 as *const libc::c_char,
    );
    if zconf.fsconf.success_index < 0 as libc::c_int {
        log_fatal(
            b"fieldset\0" as *const u8 as *const libc::c_char,
            b"probe module does not supply required success field.\0" as *const u8
                as *const libc::c_char,
        );
    }
    zconf
        .fsconf
        .app_success_index = fds_get_index_by_name(
        fds,
        b"app_success\0" as *const u8 as *const libc::c_char,
    );
    if zconf.fsconf.app_success_index < 0 as libc::c_int {
        log_debug(
            b"fieldset\0" as *const u8 as *const libc::c_char,
            b"probe module does not supply application success field.\0" as *const u8
                as *const libc::c_char,
        );
    } else {
        log_debug(
            b"fieldset\0" as *const u8 as *const libc::c_char,
            b"probe module supplies app_success output field. It will be included in monitor output\0"
                as *const u8 as *const libc::c_char,
        );
    }
    zconf
        .fsconf
        .classification_index = fds_get_index_by_name(
        fds,
        b"classification\0" as *const u8 as *const libc::c_char,
    );
    if zconf.fsconf.classification_index < 0 as libc::c_int {
        log_fatal(
            b"fieldset\0" as *const u8 as *const libc::c_char,
            b"probe module does not supply required packet classification field.\0"
                as *const u8 as *const libc::c_char,
        );
    }
    if args.output_fields_given != 0 {
        zconf.raw_output_fields = args.output_fields_arg;
    } else {
        zconf.raw_output_fields = b"saddr\0" as *const u8 as *const libc::c_char;
    }
    if strcmp(zconf.raw_output_fields, b"*\0" as *const u8 as *const libc::c_char) == 0 {
        zconf.output_fields_len = zconf.fsconf.defs.len;
        zconf
            .output_fields = xcalloc(
            zconf.fsconf.defs.len as size_t,
            ::std::mem::size_of::<*const libc::c_char>() as libc::c_ulong,
        ) as *mut *const libc::c_char;
        let mut i_0: libc::c_int = 0 as libc::c_int;
        while i_0 < zconf.fsconf.defs.len {
            let ref mut fresh0 = *(zconf.output_fields).offset(i_0 as isize);
            *fresh0 = zconf.fsconf.defs.fielddefs[i_0 as usize].name;
            i_0 += 1;
            i_0;
        }
        fs_generate_full_fieldset_translation(
            &mut zconf.fsconf.translation,
            &mut zconf.fsconf.defs,
        );
    } else {
        split_string(
            zconf.raw_output_fields,
            &mut zconf.output_fields_len,
            &mut zconf.output_fields,
        );
        let mut i_1: libc::c_int = 0 as libc::c_int;
        while i_1 < zconf.output_fields_len {
            log_debug(
                b"zmap\0" as *const u8 as *const libc::c_char,
                b"requested output field (%i): %s\0" as *const u8 as *const libc::c_char,
                i_1,
                *(zconf.output_fields).offset(i_1 as isize),
            );
            i_1 += 1;
            i_1;
        }
        fs_generate_fieldset_translation(
            &mut zconf.fsconf.translation,
            &mut zconf.fsconf.defs,
            zconf.output_fields,
            zconf.output_fields_len,
        );
    }
    if zconf.default_mode != 0 {
        log_debug(
            b"filter\0" as *const u8 as *const libc::c_char,
            b"No output filter specified. Will use default: exclude duplicates and unssuccessful\0"
                as *const u8 as *const libc::c_char,
        );
    } else if args.output_filter_given != 0
        && strcmp(args.output_filter_arg, b"\0" as *const u8 as *const libc::c_char) != 0
    {
        if parse_filter_string(args.output_filter_arg) == 0 {
            log_fatal(
                b"zmap\0" as *const u8 as *const libc::c_char,
                b"Unable to parse filter expression\0" as *const u8
                    as *const libc::c_char,
            );
        }
        if validate_filter(zconf.filter.expression, &mut zconf.fsconf.defs) == 0 {
            log_fatal(
                b"zmap\0" as *const u8 as *const libc::c_char,
                b"Invalid filter\0" as *const u8 as *const libc::c_char,
            );
        }
        zconf.output_filter_str = args.output_filter_arg;
        log_debug(
            b"filter\0" as *const u8 as *const libc::c_char,
            b"will use output filter %s\0" as *const u8 as *const libc::c_char,
            args.output_filter_arg,
        );
    } else if args.output_filter_given != 0 {
        log_debug(
            b"filter\0" as *const u8 as *const libc::c_char,
            b"Empty output filter provided. ZMap will output all results, including duplicate and non-successful responses.\0"
                as *const u8 as *const libc::c_char,
        );
    } else {
        log_info(
            b"filter\0" as *const u8 as *const libc::c_char,
            b"No output filter provided. ZMap will output all results, including duplicate and non-successful responses (e.g., RST and ICMP packets). If you want a filter similar to ZMap's default behavior, you can set an output filter similar to the following: --output-filter=\"success=1 && repeat=0\".\0"
                as *const u8 as *const libc::c_char,
        );
    }
    zconf.ignore_invalid_hosts = args.ignore_blocklist_errors_given as libc::c_int;
    if args.dryrun_given != 0 {
        zconf.dryrun = 1 as libc::c_int;
    }
    if args.quiet_given != 0 {
        zconf.quiet = 1 as libc::c_int;
    }
    if args.no_header_row_given != 0 {
        zconf.no_header_row = 1 as libc::c_int;
    }
    zconf.cooldown_secs = args.cooldown_time_arg;
    if args.output_file_given != 0 {
        zconf.output_filename = args.output_file_arg;
    }
    if args.blocklist_file_given != 0 {
        zconf.blocklist_filename = args.blocklist_file_arg;
    }
    if args.list_of_ips_file_given != 0 {
        zconf.list_of_ips_filename = args.list_of_ips_file_arg;
    }
    if args.probe_args_given != 0 {
        zconf.probe_args = args.probe_args_arg;
    }
    if args.probe_ttl_given != 0 {
        zconf.probe_ttl = args.probe_ttl_arg as uint8_t;
    }
    if args.output_args_given != 0 {
        zconf.output_args = args.output_args_arg;
    }
    if args.interface_given != 0 {
        zconf.iface = args.interface_arg;
    }
    if args.max_runtime_given != 0 {
        zconf.max_runtime = args.max_runtime_arg as uint32_t;
    }
    if args.max_results_given != 0 {
        zconf.max_results = args.max_results_arg as uint32_t;
    }
    if args.rate_given != 0 {
        zconf.rate = args.rate_arg;
    }
    if args.probes_given != 0 {
        zconf.packet_streams = args.probes_arg;
    }
    if args.status_updates_file_given != 0 {
        zconf.status_updates_file = args.status_updates_file_arg;
    }
    if args.retries_given != 0 {
        zconf.num_retries = args.retries_arg;
    }
    if args.max_sendto_failures_given != 0 {
        zconf.max_sendto_failures = args.max_sendto_failures_arg;
    }
    if args.min_hitrate_given != 0 {
        zconf.min_hitrate = args.min_hitrate_arg;
    }
    if zconf.num_retries < 0 as libc::c_int {
        log_fatal(
            b"zmap\0" as *const u8 as *const libc::c_char,
            b"Invalid retry count\0" as *const u8 as *const libc::c_char,
        );
    }
    if zconf.max_sendto_failures >= 0 as libc::c_int {
        log_debug(
            b"zmap\0" as *const u8 as *const libc::c_char,
            b"scan will abort if more than %i sendto failures occur\0" as *const u8
                as *const libc::c_char,
            zconf.max_sendto_failures,
        );
    }
    if zconf.min_hitrate as libc::c_double > 0.0f64 {
        log_debug(
            b"zmap\0" as *const u8 as *const libc::c_char,
            b"scan will abort if hitrate falls below %f\0" as *const u8
                as *const libc::c_char,
            zconf.min_hitrate as libc::c_double,
        );
    }
    if !(args.metadata_file_arg).is_null() {
        zconf.metadata_filename = args.metadata_file_arg;
        if strcmp(zconf.metadata_filename, b"-\0" as *const u8 as *const libc::c_char)
            == 0
        {
            zconf.metadata_file = stdout;
        } else {
            zconf
                .metadata_file = fopen(
                zconf.metadata_filename,
                b"w\0" as *const u8 as *const libc::c_char,
            );
        }
        if (zconf.metadata_file).is_null() {
            log_fatal(
                b"metadata\0" as *const u8 as *const libc::c_char,
                b"unable to open metadata file (%s): %s\0" as *const u8
                    as *const libc::c_char,
                zconf.metadata_filename,
                strerror(*__errno_location()),
            );
        }
        log_debug(
            b"metadata\0" as *const u8 as *const libc::c_char,
            b"metdata will be saved to %s\0" as *const u8 as *const libc::c_char,
            zconf.metadata_filename,
        );
    }
    if args.user_metadata_given != 0 {
        zconf.custom_metadata_str = args.user_metadata_arg;
        if (json_tokener_parse(zconf.custom_metadata_str)).is_null() {
            log_fatal(
                b"metadata\0" as *const u8 as *const libc::c_char,
                b"unable to parse custom user metadata\0" as *const u8
                    as *const libc::c_char,
            );
        } else {
            log_debug(
                b"metadata\0" as *const u8 as *const libc::c_char,
                b"user metadata validated successfully\0" as *const u8
                    as *const libc::c_char,
            );
        }
    }
    if args.notes_given != 0 {
        zconf.notes = args.notes_arg;
    }
    zconf.destination_cidrs = args.inputs;
    zconf.destination_cidrs_len = args.inputs_num as libc::c_int;
    if !(zconf.destination_cidrs).is_null() && !(zconf.blocklist_filename).is_null()
        && strcmp(
            zconf.blocklist_filename,
            b"/etc/zmap/blocklist.conf\0" as *const u8 as *const libc::c_char,
        ) == 0
    {
        log_warn(
            b"blocklist\0" as *const u8 as *const libc::c_char,
            b"ZMap is currently using the default blocklist located at /etc/zmap/blocklist.conf. By default, this blocklist excludes locally scoped networks (e.g. 10.0.0.0/8, 127.0.0.1/8, and 192.168.0.0/16). If you are trying to scan local networks, you can change the default blocklist by editing the default ZMap configuration at /etc/zmap/zmap.conf.\0"
                as *const u8 as *const libc::c_char,
        );
    }
    if args.allowlist_file_given != 0 {
        zconf.allowlist_filename = args.allowlist_file_arg;
    }
    if (*zconf.probe_module).port_args != 0 {
        if args.source_port_given != 0 {
            let mut dash: *mut libc::c_char = strchr(args.source_port_arg, '-' as i32);
            if !dash.is_null() {
                *dash = '\0' as i32 as libc::c_char;
                zconf.source_port_first = atoi(args.source_port_arg) as port_h_t;
                enforce_range(
                    b"starting source-port\0" as *const u8 as *const libc::c_char,
                    zconf.source_port_first as libc::c_int,
                    0 as libc::c_int,
                    0xffff as libc::c_int,
                );
                zconf
                    .source_port_last = atoi(dash.offset(1 as libc::c_int as isize))
                    as port_h_t;
                enforce_range(
                    b"ending source-port\0" as *const u8 as *const libc::c_char,
                    zconf.source_port_last as libc::c_int,
                    0 as libc::c_int,
                    0xffff as libc::c_int,
                );
                if zconf.source_port_first as libc::c_int
                    > zconf.source_port_last as libc::c_int
                {
                    fprintf(
                        stderr,
                        b"%s: invalid source port range: last port is less than first port\n\0"
                            as *const u8 as *const libc::c_char,
                        b"zmap\0" as *const u8 as *const libc::c_char,
                    );
                    exit(1 as libc::c_int);
                }
            } else {
                let mut port: libc::c_int = atoi(args.source_port_arg);
                enforce_range(
                    b"source-port\0" as *const u8 as *const libc::c_char,
                    port,
                    0 as libc::c_int,
                    0xffff as libc::c_int,
                );
                zconf.source_port_first = port as port_h_t;
                zconf.source_port_last = port as port_h_t;
            }
        }
        if args.target_port_given == 0 {
            log_fatal(
                b"zmap\0" as *const u8 as *const libc::c_char,
                b"target port (-p) is required for this type of probe\0" as *const u8
                    as *const libc::c_char,
            );
        }
        enforce_range(
            b"target-port\0" as *const u8 as *const libc::c_char,
            args.target_port_arg,
            0 as libc::c_int,
            0xffff as libc::c_int,
        );
        zconf.target_port = args.target_port_arg as port_h_t;
    }
    if args.source_ip_given != 0 {
        parse_source_ip_addresses(args.source_ip_arg);
    }
    if args.gateway_mac_given != 0 {
        if parse_mac((zconf.gw_mac).as_mut_ptr(), args.gateway_mac_arg) == 0 {
            fprintf(
                stderr,
                b"%s: invalid MAC address `%s'\n\0" as *const u8 as *const libc::c_char,
                b"zmap\0" as *const u8 as *const libc::c_char,
                args.gateway_mac_arg,
            );
            exit(1 as libc::c_int);
        }
        zconf.gw_mac_set = 1 as libc::c_int;
    }
    if args.source_mac_given != 0 {
        if parse_mac((zconf.hw_mac).as_mut_ptr(), args.source_mac_arg) == 0 {
            fprintf(
                stderr,
                b"%s: invalid MAC address `%s'\n\0" as *const u8 as *const libc::c_char,
                b"zmap\0" as *const u8 as *const libc::c_char,
                args.gateway_mac_arg,
            );
            exit(1 as libc::c_int);
        }
        log_debug(
            b"send\0" as *const u8 as *const libc::c_char,
            b"source MAC address specified on CLI: %02x:%02x:%02x:%02x:%02x:%02x\0"
                as *const u8 as *const libc::c_char,
            zconf.hw_mac[0 as libc::c_int as usize] as libc::c_int,
            zconf.hw_mac[1 as libc::c_int as usize] as libc::c_int,
            zconf.hw_mac[2 as libc::c_int as usize] as libc::c_int,
            zconf.hw_mac[3 as libc::c_int as usize] as libc::c_int,
            zconf.hw_mac[4 as libc::c_int as usize] as libc::c_int,
            zconf.hw_mac[5 as libc::c_int as usize] as libc::c_int,
        );
        zconf.hw_mac_set = 1 as libc::c_int;
    }
    if args.seed_given != 0 {
        zconf.seed = args.seed_arg as uint64_t;
        zconf.seed_provided = 1 as libc::c_int;
    } else {
        if random_bytes(
            &mut zconf.seed as *mut uint64_t as *mut libc::c_void,
            ::std::mem::size_of::<uint64_t>() as libc::c_ulong,
        ) == 0
        {
            log_fatal(
                b"zmap\0" as *const u8 as *const libc::c_char,
                b"unable to generate random bytes needed for seed\0" as *const u8
                    as *const libc::c_char,
            );
        }
        zconf.seed_provided = 0 as libc::c_int;
    }
    zconf.aes = aesrand_init_from_seed(zconf.seed);
    zconf.shard_num = 0 as libc::c_int as uint16_t;
    zconf.total_shards = 1 as libc::c_int as uint16_t;
    if (args.shard_given != 0 || args.shards_given != 0) && args.seed_given == 0 {
        log_fatal(
            b"zmap\0" as *const u8 as *const libc::c_char,
            b"Need to specify seed if sharding a scan\0" as *const u8
                as *const libc::c_char,
        );
    }
    if args.shard_given ^ args.shards_given != 0 {
        log_fatal(
            b"zmap\0" as *const u8 as *const libc::c_char,
            b"Need to specify both shard number and total number of shards\0"
                as *const u8 as *const libc::c_char,
        );
    }
    if args.shard_given != 0 {
        enforce_range(
            b"shard\0" as *const u8 as *const libc::c_char,
            args.shard_arg,
            0 as libc::c_int,
            65534 as libc::c_int,
        );
    }
    if args.shards_given != 0 {
        enforce_range(
            b"shards\0" as *const u8 as *const libc::c_char,
            args.shards_arg,
            1 as libc::c_int,
            65535 as libc::c_int,
        );
    }
    if args.shard_given != 0 {
        zconf.shard_num = args.shard_arg as uint16_t;
    }
    if args.shards_given != 0 {
        zconf.total_shards = args.shards_arg as uint16_t;
    }
    if zconf.shard_num as libc::c_int >= zconf.total_shards as libc::c_int {
        log_fatal(
            b"zmap\0" as *const u8 as *const libc::c_char,
            b"With %hhu total shards, shard number (%hhu) must be in range [0, %hhu)\0"
                as *const u8 as *const libc::c_char,
            zconf.total_shards as libc::c_int,
            zconf.shard_num as libc::c_int,
            zconf.total_shards as libc::c_int,
        );
    }
    if args.bandwidth_given != 0 {
        zconf.bandwidth = atoi(args.bandwidth_arg) as uint64_t;
        let mut suffix: *mut libc::c_char = args.bandwidth_arg;
        while *suffix as libc::c_int >= '0' as i32
            && *suffix as libc::c_int <= '9' as i32
        {
            suffix = suffix.offset(1);
            suffix;
        }
        if *suffix != 0 {
            match *suffix as libc::c_int {
                71 | 103 => {
                    zconf
                        .bandwidth = (zconf.bandwidth as libc::c_ulong)
                        .wrapping_mul(1000000000 as libc::c_int as libc::c_ulong)
                        as uint64_t as uint64_t;
                }
                77 | 109 => {
                    zconf
                        .bandwidth = (zconf.bandwidth as libc::c_ulong)
                        .wrapping_mul(1000000 as libc::c_int as libc::c_ulong)
                        as uint64_t as uint64_t;
                }
                75 | 107 => {
                    zconf
                        .bandwidth = (zconf.bandwidth as libc::c_ulong)
                        .wrapping_mul(1000 as libc::c_int as libc::c_ulong) as uint64_t
                        as uint64_t;
                }
                _ => {
                    fprintf(
                        stderr,
                        b"%s: unknown bandwidth suffix '%s' (supported suffixes are G, M and K)\n\0"
                            as *const u8 as *const libc::c_char,
                        b"zmap\0" as *const u8 as *const libc::c_char,
                        suffix,
                    );
                    exit(1 as libc::c_int);
                }
            }
        }
    }
    if args.batch_given != 0 {
        zconf.batch = args.batch_arg as uint8_t;
    }
    if args.max_targets_given != 0 {
        zconf.max_targets = parse_max_hosts(args.max_targets_arg);
    }
    if blocklist_init(
        zconf.allowlist_filename,
        zconf.blocklist_filename,
        zconf.destination_cidrs,
        zconf.destination_cidrs_len as size_t,
        0 as *mut *mut libc::c_char,
        0 as libc::c_int as size_t,
        zconf.ignore_invalid_hosts,
    ) != 0
    {
        log_fatal(
            b"zmap\0" as *const u8 as *const libc::c_char,
            b"unable to initialize blocklist / allowlist\0" as *const u8
                as *const libc::c_char,
        );
    }
    if !(zconf.list_of_ips_filename).is_null() {
        zsend.list_of_ips_pbm = pbm_init();
        zconf
            .list_of_ips_count = pbm_load_from_file(
            zsend.list_of_ips_pbm,
            zconf.list_of_ips_filename,
        );
    }
    let mut allowed: uint64_t = blocklist_count_allowed();
    zconf.total_allowed = allowed;
    zconf.total_disallowed = blocklist_count_not_allowed();
    if allowed as libc::c_ulonglong
        <= ((1 as libc::c_longlong) << 32 as libc::c_int) as libc::c_ulonglong
    {} else {
        __assert_fail(
            b"allowed <= (1LL << 32)\0" as *const u8 as *const libc::c_char,
            b"zmap.c\0" as *const u8 as *const libc::c_char,
            826 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 23],
                &[libc::c_char; 23],
            >(b"int main(int, char **)\0"))
                .as_ptr(),
        );
    }
    'c_9930: {
        if allowed as libc::c_ulonglong
            <= ((1 as libc::c_longlong) << 32 as libc::c_int) as libc::c_ulonglong
        {} else {
            __assert_fail(
                b"allowed <= (1LL << 32)\0" as *const u8 as *const libc::c_char,
                b"zmap.c\0" as *const u8 as *const libc::c_char,
                826 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 23],
                    &[libc::c_char; 23],
                >(b"int main(int, char **)\0"))
                    .as_ptr(),
            );
        }
    };
    if zconf.total_allowed == 0 {
        log_fatal(
            b"zmap\0" as *const u8 as *const libc::c_char,
            b"zero eligible addresses to scan\0" as *const u8 as *const libc::c_char,
        );
    }
    if zconf.list_of_ips_count > 0 as libc::c_int as libc::c_uint
        && (0xffffffff as libc::c_uint).wrapping_div(zconf.list_of_ips_count)
            > 100000 as libc::c_int as libc::c_uint
    {
        log_warn(
            b"zmap\0" as *const u8 as *const libc::c_char,
            b"list of IPs is small compared to address space. Performance will suffer, consider using an allowlist instead\0"
                as *const u8 as *const libc::c_char,
        );
    }
    if zconf.max_targets != 0 {
        zsend.max_targets = zconf.max_targets;
    }
    if args.sender_threads_given != 0 {
        zconf.senders = args.sender_threads_arg as uint8_t;
    } else {
        zconf.senders = 1 as libc::c_int as uint8_t;
    }
    if (2 as libc::c_int * zconf.senders as libc::c_int) as libc::c_uint
        >= zsend.max_targets
    {
        log_warn(
            b"zmap\0" as *const u8 as *const libc::c_char,
            b"too few targets relative to senders, dropping to one sender\0" as *const u8
                as *const libc::c_char,
        );
        zconf.senders = 1 as libc::c_int as uint8_t;
    }
    if args.cores_given != 0 {
        let mut core_list: *mut *const libc::c_char = 0 as *mut *const libc::c_char;
        let mut len: libc::c_int = 0 as libc::c_int;
        split_string(args.cores_arg, &mut len, &mut core_list);
        zconf.pin_cores_len = len as uint32_t;
        zconf
            .pin_cores = xcalloc(
            zconf.pin_cores_len as size_t,
            ::std::mem::size_of::<uint32_t>() as libc::c_ulong,
        ) as *mut uint32_t;
        let mut i_2: uint32_t = 0 as libc::c_int as uint32_t;
        while i_2 < zconf.pin_cores_len {
            *(zconf.pin_cores)
                .offset(
                    i_2 as isize,
                ) = atoi(*core_list.offset(i_2 as isize)) as uint32_t;
            i_2 = i_2.wrapping_add(1);
            i_2;
        }
    } else {
        let mut num_cores: libc::c_int = sysconf(_SC_NPROCESSORS_ONLN as libc::c_int)
            as libc::c_int;
        zconf.pin_cores_len = num_cores as uint32_t;
        zconf
            .pin_cores = xcalloc(
            zconf.pin_cores_len as size_t,
            ::std::mem::size_of::<uint32_t>() as libc::c_ulong,
        ) as *mut uint32_t;
        let mut i_3: uint32_t = 0 as libc::c_int as uint32_t;
        while i_3 < zconf.pin_cores_len {
            *(zconf.pin_cores).offset(i_3 as isize) = i_3;
            i_3 = i_3.wrapping_add(1);
            i_3;
        }
    }
    start_zmap();
    fclose(log_location);
    cmdline_parser_free(&mut args);
    free(params as *mut libc::c_void);
    return 0 as libc::c_int;
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
