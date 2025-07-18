use ::libc;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    pub type exclude;
    pub type __dirstream;
    pub type cycle_check_state;
    pub type hash_table;
    fn __xstat(
        __ver: libc::c_int,
        __filename: *const libc::c_char,
        __stat_buf: *mut stat,
    ) -> libc::c_int;
    fn __fxstat(
        __ver: libc::c_int,
        __fildes: libc::c_int,
        __stat_buf: *mut stat,
    ) -> libc::c_int;
    fn __fxstatat(
        __ver: libc::c_int,
        __fildes: libc::c_int,
        __filename: *const libc::c_char,
        __stat_buf: *mut stat,
        __flag: libc::c_int,
    ) -> libc::c_int;
    fn wcrtomb(__s: *mut libc::c_char, __wc: wchar_t, __ps: *mut mbstate_t) -> size_t;
    fn rpl_free(_: *mut libc::c_void);
    fn rpl_mbrtowc(
        pwc: *mut wchar_t,
        s: *const libc::c_char,
        n: size_t,
        ps: *mut mbstate_t,
    ) -> size_t;
    fn rpl_mbrlen(s: *const libc::c_char, n: size_t, ps: *mut mbstate_t) -> size_t;
    fn fclose(__stream: *mut FILE) -> libc::c_int;
    fn fflush_unlocked(__stream: *mut FILE) -> libc::c_int;
    static mut stdin: *mut FILE;
    static mut stdout: *mut FILE;
    static mut stderr: *mut FILE;
    fn rpl_fopen(
        filename_0: *const libc::c_char,
        mode: *const libc::c_char,
    ) -> *mut FILE;
    fn puts(__s: *const libc::c_char) -> libc::c_int;
    fn fputs_unlocked(__s: *const libc::c_char, __stream: *mut FILE) -> libc::c_int;
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
    fn fprintf(_: *mut FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
    fn vfprintf(
        _: *mut FILE,
        _: *const libc::c_char,
        _: ::std::ffi::VaList,
    ) -> libc::c_int;
    fn clearerr_unlocked(__stream: *mut FILE);
    fn __uflow(_: *mut FILE) -> libc::c_int;
    fn __overflow(_: *mut FILE, _: libc::c_int) -> libc::c_int;
    fn abort() -> !;
    fn atexit(__func: Option::<unsafe extern "C" fn() -> ()>) -> libc::c_int;
    fn strip_trailing_slashes(file: *mut libc::c_char) -> bool;
    fn __errno_location() -> *mut libc::c_int;
    static mut optarg: *mut libc::c_char;
    static mut optind: libc::c_int;
    fn isatty(__fd: libc::c_int) -> libc::c_int;
    fn sysconf(__name: libc::c_int) -> libc::c_long;
    fn close(__fd: libc::c_int) -> libc::c_int;
    fn lseek(__fd: libc::c_int, __offset: __off_t, __whence: libc::c_int) -> __off_t;
    fn exit(_: libc::c_int) -> !;
    fn getenv(__name: *const libc::c_char) -> *mut libc::c_char;
    fn getprogname() -> *const libc::c_char;
    fn memmove(
        _: *mut libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn memset(
        _: *mut libc::c_void,
        _: libc::c_int,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn memchr(
        _: *const libc::c_void,
        _: libc::c_int,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn rawmemchr(__s: *const libc::c_void, __c: libc::c_int) -> *mut libc::c_void;
    fn memrchr(
        __s: *const libc::c_void,
        __c: libc::c_int,
        __n: size_t,
    ) -> *mut libc::c_void;
    fn strcpy(_: *mut libc::c_char, _: *const libc::c_char) -> *mut libc::c_char;
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    fn mempcpy(
        _: *mut libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    fn __ctype_toupper_loc() -> *mut *const __int32_t;
    fn dcgettext(
        __domainname: *const libc::c_char,
        __msgid: *const libc::c_char,
        __category: libc::c_int,
    ) -> *mut libc::c_char;
    fn textdomain(__domainname: *const libc::c_char) -> *mut libc::c_char;
    fn bindtextdomain(
        __domainname: *const libc::c_char,
        __dirname: *const libc::c_char,
    ) -> *mut libc::c_char;
    fn setlocale(
        __category: libc::c_int,
        __locale: *const libc::c_char,
    ) -> *mut libc::c_char;
    fn splice(
        __fdin: libc::c_int,
        __offin: *mut __off64_t,
        __fdout: libc::c_int,
        __offout: *mut __off64_t,
        __len: size_t,
        __flags: libc::c_uint,
    ) -> __ssize_t;
    static mut argmatch_die: argmatch_exit_fn;
    fn __xargmatch_internal(
        context: *const libc::c_char,
        arg: *const libc::c_char,
        arglist: *const *const libc::c_char,
        vallist: *const libc::c_void,
        valsize: size_t,
        exit_fn: argmatch_exit_fn,
        allow_abbreviation: bool,
    ) -> ptrdiff_t;
    fn c_stack_action(
        _: Option::<unsafe extern "C" fn(libc::c_int) -> ()>,
    ) -> libc::c_int;
    fn close_stdout();
    fn should_colorize() -> libc::c_int;
    fn init_colorize();
    fn print_start_colorize(
        sgr_start_0: *const libc::c_char,
        sgr_seq: *const libc::c_char,
    );
    fn print_end_colorize(sgr_end_0: *const libc::c_char);
    fn error(
        __status: libc::c_int,
        __errnum: libc::c_int,
        __format: *const libc::c_char,
        _: ...
    );
    fn new_exclude() -> *mut exclude;
    fn add_exclude(_: *mut exclude, _: *const libc::c_char, _: libc::c_int);
    fn add_exclude_file(
        _: Option::<
            unsafe extern "C" fn(*mut exclude, *const libc::c_char, libc::c_int) -> (),
        >,
        _: *mut exclude,
        _: *const libc::c_char,
        _: libc::c_int,
        _: libc::c_char,
    ) -> libc::c_int;
    fn excluded_file_name(_: *const exclude, _: *const libc::c_char) -> bool;
    static mut exit_failure: libc::c_int;
    fn openat_safer(
        _: libc::c_int,
        _: *const libc::c_char,
        _: libc::c_int,
        _: ...
    ) -> libc::c_int;
    fn rpl_fts_close(_: *mut FTS) -> libc::c_int;
    fn rpl_fts_open(
        _: *const *mut libc::c_char,
        _: libc::c_int,
        _: Option::<
            unsafe extern "C" fn(*mut *const FTSENT, *mut *const FTSENT) -> libc::c_int,
        >,
    ) -> *mut FTS;
    fn rpl_fts_read(_: *mut FTS) -> *mut FTSENT;
    fn rpl_fts_set(_: *mut FTS, _: *mut FTSENT, _: libc::c_int) -> libc::c_int;
    fn getopt_long(
        ___argc: libc::c_int,
        ___argv: *const *mut libc::c_char,
        __shortopts: *const libc::c_char,
        __longopts: *const option,
        __longind: *mut libc::c_int,
    ) -> libc::c_int;
    fn hash_free(table: *mut Hash_table);
    fn hash_initialize(
        candidate: size_t,
        tuning: *const Hash_tuning,
        hasher: Hash_hasher,
        comparator: Hash_comparator,
        data_freer: Hash_data_freer,
    ) -> *mut Hash_table;
    fn hash_insert_if_absent(
        table: *mut Hash_table,
        entry: *const libc::c_void,
        matched_ent: *mut *const libc::c_void,
    ) -> libc::c_int;
    fn safe_read(fd: libc::c_int, buf: *mut libc::c_void, count: size_t) -> size_t;
    fn ximalloc(s: idx_t) -> *mut libc::c_void;
    fn xpalloc(
        pa: *mut libc::c_void,
        pn: *mut idx_t,
        n_incr_min: idx_t,
        n_max: ptrdiff_t,
        s: idx_t,
    ) -> *mut libc::c_void;
    fn xstrdup(str: *const libc::c_char) -> *mut libc::c_char;
    fn xnmalloc(n: size_t, s: size_t) -> *mut libc::c_void;
    fn init_localeinfo(_: *mut localeinfo);
    fn case_folded_counterparts(_: wint_t, _: *mut wchar_t) -> libc::c_int;
    fn wordinit();
    fn GEAcompile(
        _: *mut libc::c_char,
        _: idx_t,
        _: reg_syntax_t,
        _: bool,
    ) -> *mut libc::c_void;
    fn EGexecute(
        _: *mut libc::c_void,
        _: *const libc::c_char,
        _: idx_t,
        _: *mut idx_t,
        _: *const libc::c_char,
    ) -> ptrdiff_t;
    fn Fcompile(
        _: *mut libc::c_char,
        _: idx_t,
        _: reg_syntax_t,
        _: bool,
    ) -> *mut libc::c_void;
    fn Fexecute(
        _: *mut libc::c_void,
        _: *const libc::c_char,
        _: idx_t,
        _: *mut idx_t,
        _: *const libc::c_char,
    ) -> ptrdiff_t;
    fn xalloc_die();
    fn c_strcasecmp(s1: *const libc::c_char, s2: *const libc::c_char) -> libc::c_int;
    fn version_etc(
        stream: *mut FILE,
        command_name: *const libc::c_char,
        package: *const libc::c_char,
        version: *const libc::c_char,
        _: ...
    );
    fn emit_bug_reporting_address();
    fn xstrtoimax(
        _: *const libc::c_char,
        _: *mut *mut libc::c_char,
        _: libc::c_int,
        _: *mut intmax_t,
        _: *const libc::c_char,
    ) -> strtol_error;
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
pub type __int32_t = libc::c_int;
pub type __intmax_t = libc::c_long;
pub type __uintmax_t = libc::c_ulong;
pub type __dev_t = libc::c_ulong;
pub type __uid_t = libc::c_uint;
pub type __gid_t = libc::c_uint;
pub type __ino_t = libc::c_ulong;
pub type __mode_t = libc::c_uint;
pub type __nlink_t = libc::c_ulong;
pub type __off_t = libc::c_long;
pub type __off64_t = libc::c_long;
pub type __time_t = libc::c_long;
pub type __blksize_t = libc::c_long;
pub type __blkcnt_t = libc::c_long;
pub type __ssize_t = libc::c_long;
pub type __syscall_slong_t = libc::c_long;
pub type dev_t = __dev_t;
pub type mode_t = __mode_t;
pub type off_t = __off_t;
pub type ssize_t = __ssize_t;
pub type size_t = libc::c_ulong;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct timespec {
    pub tv_sec: __time_t,
    pub tv_nsec: __syscall_slong_t,
}
pub type ptrdiff_t = libc::c_long;
pub type wchar_t = libc::c_int;
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
pub type va_list = __builtin_va_list;
pub type wint_t = libc::c_uint;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct mbstate_t {
    pub __count: libc::c_int,
    pub __value: C2RustUnnamed,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed {
    pub __wch: libc::c_uint,
    pub __wchb: [libc::c_char; 4],
}
pub type __mbstate_t = mbstate_t;
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
pub type uint_fast64_t = libc::c_ulong;
pub type intptr_t = libc::c_long;
pub type uintptr_t = libc::c_ulong;
pub type intmax_t = __intmax_t;
pub type uintmax_t = __uintmax_t;
pub type C2RustUnnamed_0 = libc::c_uint;
pub const _SC_THREAD_ROBUST_PRIO_PROTECT: C2RustUnnamed_0 = 248;
pub const _SC_THREAD_ROBUST_PRIO_INHERIT: C2RustUnnamed_0 = 247;
pub const _SC_XOPEN_STREAMS: C2RustUnnamed_0 = 246;
pub const _SC_TRACE_USER_EVENT_MAX: C2RustUnnamed_0 = 245;
pub const _SC_TRACE_SYS_MAX: C2RustUnnamed_0 = 244;
pub const _SC_TRACE_NAME_MAX: C2RustUnnamed_0 = 243;
pub const _SC_TRACE_EVENT_NAME_MAX: C2RustUnnamed_0 = 242;
pub const _SC_SS_REPL_MAX: C2RustUnnamed_0 = 241;
pub const _SC_V7_LPBIG_OFFBIG: C2RustUnnamed_0 = 240;
pub const _SC_V7_LP64_OFF64: C2RustUnnamed_0 = 239;
pub const _SC_V7_ILP32_OFFBIG: C2RustUnnamed_0 = 238;
pub const _SC_V7_ILP32_OFF32: C2RustUnnamed_0 = 237;
pub const _SC_RAW_SOCKETS: C2RustUnnamed_0 = 236;
pub const _SC_IPV6: C2RustUnnamed_0 = 235;
pub const _SC_LEVEL4_CACHE_LINESIZE: C2RustUnnamed_0 = 199;
pub const _SC_LEVEL4_CACHE_ASSOC: C2RustUnnamed_0 = 198;
pub const _SC_LEVEL4_CACHE_SIZE: C2RustUnnamed_0 = 197;
pub const _SC_LEVEL3_CACHE_LINESIZE: C2RustUnnamed_0 = 196;
pub const _SC_LEVEL3_CACHE_ASSOC: C2RustUnnamed_0 = 195;
pub const _SC_LEVEL3_CACHE_SIZE: C2RustUnnamed_0 = 194;
pub const _SC_LEVEL2_CACHE_LINESIZE: C2RustUnnamed_0 = 193;
pub const _SC_LEVEL2_CACHE_ASSOC: C2RustUnnamed_0 = 192;
pub const _SC_LEVEL2_CACHE_SIZE: C2RustUnnamed_0 = 191;
pub const _SC_LEVEL1_DCACHE_LINESIZE: C2RustUnnamed_0 = 190;
pub const _SC_LEVEL1_DCACHE_ASSOC: C2RustUnnamed_0 = 189;
pub const _SC_LEVEL1_DCACHE_SIZE: C2RustUnnamed_0 = 188;
pub const _SC_LEVEL1_ICACHE_LINESIZE: C2RustUnnamed_0 = 187;
pub const _SC_LEVEL1_ICACHE_ASSOC: C2RustUnnamed_0 = 186;
pub const _SC_LEVEL1_ICACHE_SIZE: C2RustUnnamed_0 = 185;
pub const _SC_TRACE_LOG: C2RustUnnamed_0 = 184;
pub const _SC_TRACE_INHERIT: C2RustUnnamed_0 = 183;
pub const _SC_TRACE_EVENT_FILTER: C2RustUnnamed_0 = 182;
pub const _SC_TRACE: C2RustUnnamed_0 = 181;
pub const _SC_HOST_NAME_MAX: C2RustUnnamed_0 = 180;
pub const _SC_V6_LPBIG_OFFBIG: C2RustUnnamed_0 = 179;
pub const _SC_V6_LP64_OFF64: C2RustUnnamed_0 = 178;
pub const _SC_V6_ILP32_OFFBIG: C2RustUnnamed_0 = 177;
pub const _SC_V6_ILP32_OFF32: C2RustUnnamed_0 = 176;
pub const _SC_2_PBS_CHECKPOINT: C2RustUnnamed_0 = 175;
pub const _SC_STREAMS: C2RustUnnamed_0 = 174;
pub const _SC_SYMLOOP_MAX: C2RustUnnamed_0 = 173;
pub const _SC_2_PBS_TRACK: C2RustUnnamed_0 = 172;
pub const _SC_2_PBS_MESSAGE: C2RustUnnamed_0 = 171;
pub const _SC_2_PBS_LOCATE: C2RustUnnamed_0 = 170;
pub const _SC_2_PBS_ACCOUNTING: C2RustUnnamed_0 = 169;
pub const _SC_2_PBS: C2RustUnnamed_0 = 168;
pub const _SC_USER_GROUPS_R: C2RustUnnamed_0 = 167;
pub const _SC_USER_GROUPS: C2RustUnnamed_0 = 166;
pub const _SC_TYPED_MEMORY_OBJECTS: C2RustUnnamed_0 = 165;
pub const _SC_TIMEOUTS: C2RustUnnamed_0 = 164;
pub const _SC_SYSTEM_DATABASE_R: C2RustUnnamed_0 = 163;
pub const _SC_SYSTEM_DATABASE: C2RustUnnamed_0 = 162;
pub const _SC_THREAD_SPORADIC_SERVER: C2RustUnnamed_0 = 161;
pub const _SC_SPORADIC_SERVER: C2RustUnnamed_0 = 160;
pub const _SC_SPAWN: C2RustUnnamed_0 = 159;
pub const _SC_SIGNALS: C2RustUnnamed_0 = 158;
pub const _SC_SHELL: C2RustUnnamed_0 = 157;
pub const _SC_REGEX_VERSION: C2RustUnnamed_0 = 156;
pub const _SC_REGEXP: C2RustUnnamed_0 = 155;
pub const _SC_SPIN_LOCKS: C2RustUnnamed_0 = 154;
pub const _SC_READER_WRITER_LOCKS: C2RustUnnamed_0 = 153;
pub const _SC_NETWORKING: C2RustUnnamed_0 = 152;
pub const _SC_SINGLE_PROCESS: C2RustUnnamed_0 = 151;
pub const _SC_MULTI_PROCESS: C2RustUnnamed_0 = 150;
pub const _SC_MONOTONIC_CLOCK: C2RustUnnamed_0 = 149;
pub const _SC_FILE_SYSTEM: C2RustUnnamed_0 = 148;
pub const _SC_FILE_LOCKING: C2RustUnnamed_0 = 147;
pub const _SC_FILE_ATTRIBUTES: C2RustUnnamed_0 = 146;
pub const _SC_PIPE: C2RustUnnamed_0 = 145;
pub const _SC_FIFO: C2RustUnnamed_0 = 144;
pub const _SC_FD_MGMT: C2RustUnnamed_0 = 143;
pub const _SC_DEVICE_SPECIFIC_R: C2RustUnnamed_0 = 142;
pub const _SC_DEVICE_SPECIFIC: C2RustUnnamed_0 = 141;
pub const _SC_DEVICE_IO: C2RustUnnamed_0 = 140;
pub const _SC_THREAD_CPUTIME: C2RustUnnamed_0 = 139;
pub const _SC_CPUTIME: C2RustUnnamed_0 = 138;
pub const _SC_CLOCK_SELECTION: C2RustUnnamed_0 = 137;
pub const _SC_C_LANG_SUPPORT_R: C2RustUnnamed_0 = 136;
pub const _SC_C_LANG_SUPPORT: C2RustUnnamed_0 = 135;
pub const _SC_BASE: C2RustUnnamed_0 = 134;
pub const _SC_BARRIERS: C2RustUnnamed_0 = 133;
pub const _SC_ADVISORY_INFO: C2RustUnnamed_0 = 132;
pub const _SC_XOPEN_REALTIME_THREADS: C2RustUnnamed_0 = 131;
pub const _SC_XOPEN_REALTIME: C2RustUnnamed_0 = 130;
pub const _SC_XOPEN_LEGACY: C2RustUnnamed_0 = 129;
pub const _SC_XBS5_LPBIG_OFFBIG: C2RustUnnamed_0 = 128;
pub const _SC_XBS5_LP64_OFF64: C2RustUnnamed_0 = 127;
pub const _SC_XBS5_ILP32_OFFBIG: C2RustUnnamed_0 = 126;
pub const _SC_XBS5_ILP32_OFF32: C2RustUnnamed_0 = 125;
pub const _SC_NL_TEXTMAX: C2RustUnnamed_0 = 124;
pub const _SC_NL_SETMAX: C2RustUnnamed_0 = 123;
pub const _SC_NL_NMAX: C2RustUnnamed_0 = 122;
pub const _SC_NL_MSGMAX: C2RustUnnamed_0 = 121;
pub const _SC_NL_LANGMAX: C2RustUnnamed_0 = 120;
pub const _SC_NL_ARGMAX: C2RustUnnamed_0 = 119;
pub const _SC_USHRT_MAX: C2RustUnnamed_0 = 118;
pub const _SC_ULONG_MAX: C2RustUnnamed_0 = 117;
pub const _SC_UINT_MAX: C2RustUnnamed_0 = 116;
pub const _SC_UCHAR_MAX: C2RustUnnamed_0 = 115;
pub const _SC_SHRT_MIN: C2RustUnnamed_0 = 114;
pub const _SC_SHRT_MAX: C2RustUnnamed_0 = 113;
pub const _SC_SCHAR_MIN: C2RustUnnamed_0 = 112;
pub const _SC_SCHAR_MAX: C2RustUnnamed_0 = 111;
pub const _SC_SSIZE_MAX: C2RustUnnamed_0 = 110;
pub const _SC_NZERO: C2RustUnnamed_0 = 109;
pub const _SC_MB_LEN_MAX: C2RustUnnamed_0 = 108;
pub const _SC_WORD_BIT: C2RustUnnamed_0 = 107;
pub const _SC_LONG_BIT: C2RustUnnamed_0 = 106;
pub const _SC_INT_MIN: C2RustUnnamed_0 = 105;
pub const _SC_INT_MAX: C2RustUnnamed_0 = 104;
pub const _SC_CHAR_MIN: C2RustUnnamed_0 = 103;
pub const _SC_CHAR_MAX: C2RustUnnamed_0 = 102;
pub const _SC_CHAR_BIT: C2RustUnnamed_0 = 101;
pub const _SC_XOPEN_XPG4: C2RustUnnamed_0 = 100;
pub const _SC_XOPEN_XPG3: C2RustUnnamed_0 = 99;
pub const _SC_XOPEN_XPG2: C2RustUnnamed_0 = 98;
pub const _SC_2_UPE: C2RustUnnamed_0 = 97;
pub const _SC_2_C_VERSION: C2RustUnnamed_0 = 96;
pub const _SC_2_CHAR_TERM: C2RustUnnamed_0 = 95;
pub const _SC_XOPEN_SHM: C2RustUnnamed_0 = 94;
pub const _SC_XOPEN_ENH_I18N: C2RustUnnamed_0 = 93;
pub const _SC_XOPEN_CRYPT: C2RustUnnamed_0 = 92;
pub const _SC_XOPEN_UNIX: C2RustUnnamed_0 = 91;
pub const _SC_XOPEN_XCU_VERSION: C2RustUnnamed_0 = 90;
pub const _SC_XOPEN_VERSION: C2RustUnnamed_0 = 89;
pub const _SC_PASS_MAX: C2RustUnnamed_0 = 88;
pub const _SC_ATEXIT_MAX: C2RustUnnamed_0 = 87;
pub const _SC_AVPHYS_PAGES: C2RustUnnamed_0 = 86;
pub const _SC_PHYS_PAGES: C2RustUnnamed_0 = 85;
pub const _SC_NPROCESSORS_ONLN: C2RustUnnamed_0 = 84;
pub const _SC_NPROCESSORS_CONF: C2RustUnnamed_0 = 83;
pub const _SC_THREAD_PROCESS_SHARED: C2RustUnnamed_0 = 82;
pub const _SC_THREAD_PRIO_PROTECT: C2RustUnnamed_0 = 81;
pub const _SC_THREAD_PRIO_INHERIT: C2RustUnnamed_0 = 80;
pub const _SC_THREAD_PRIORITY_SCHEDULING: C2RustUnnamed_0 = 79;
pub const _SC_THREAD_ATTR_STACKSIZE: C2RustUnnamed_0 = 78;
pub const _SC_THREAD_ATTR_STACKADDR: C2RustUnnamed_0 = 77;
pub const _SC_THREAD_THREADS_MAX: C2RustUnnamed_0 = 76;
pub const _SC_THREAD_STACK_MIN: C2RustUnnamed_0 = 75;
pub const _SC_THREAD_KEYS_MAX: C2RustUnnamed_0 = 74;
pub const _SC_THREAD_DESTRUCTOR_ITERATIONS: C2RustUnnamed_0 = 73;
pub const _SC_TTY_NAME_MAX: C2RustUnnamed_0 = 72;
pub const _SC_LOGIN_NAME_MAX: C2RustUnnamed_0 = 71;
pub const _SC_GETPW_R_SIZE_MAX: C2RustUnnamed_0 = 70;
pub const _SC_GETGR_R_SIZE_MAX: C2RustUnnamed_0 = 69;
pub const _SC_THREAD_SAFE_FUNCTIONS: C2RustUnnamed_0 = 68;
pub const _SC_THREADS: C2RustUnnamed_0 = 67;
pub const _SC_T_IOV_MAX: C2RustUnnamed_0 = 66;
pub const _SC_PII_OSI_M: C2RustUnnamed_0 = 65;
pub const _SC_PII_OSI_CLTS: C2RustUnnamed_0 = 64;
pub const _SC_PII_OSI_COTS: C2RustUnnamed_0 = 63;
pub const _SC_PII_INTERNET_DGRAM: C2RustUnnamed_0 = 62;
pub const _SC_PII_INTERNET_STREAM: C2RustUnnamed_0 = 61;
pub const _SC_IOV_MAX: C2RustUnnamed_0 = 60;
pub const _SC_UIO_MAXIOV: C2RustUnnamed_0 = 60;
pub const _SC_SELECT: C2RustUnnamed_0 = 59;
pub const _SC_POLL: C2RustUnnamed_0 = 58;
pub const _SC_PII_OSI: C2RustUnnamed_0 = 57;
pub const _SC_PII_INTERNET: C2RustUnnamed_0 = 56;
pub const _SC_PII_SOCKET: C2RustUnnamed_0 = 55;
pub const _SC_PII_XTI: C2RustUnnamed_0 = 54;
pub const _SC_PII: C2RustUnnamed_0 = 53;
pub const _SC_2_LOCALEDEF: C2RustUnnamed_0 = 52;
pub const _SC_2_SW_DEV: C2RustUnnamed_0 = 51;
pub const _SC_2_FORT_RUN: C2RustUnnamed_0 = 50;
pub const _SC_2_FORT_DEV: C2RustUnnamed_0 = 49;
pub const _SC_2_C_DEV: C2RustUnnamed_0 = 48;
pub const _SC_2_C_BIND: C2RustUnnamed_0 = 47;
pub const _SC_2_VERSION: C2RustUnnamed_0 = 46;
pub const _SC_CHARCLASS_NAME_MAX: C2RustUnnamed_0 = 45;
pub const _SC_RE_DUP_MAX: C2RustUnnamed_0 = 44;
pub const _SC_LINE_MAX: C2RustUnnamed_0 = 43;
pub const _SC_EXPR_NEST_MAX: C2RustUnnamed_0 = 42;
pub const _SC_EQUIV_CLASS_MAX: C2RustUnnamed_0 = 41;
pub const _SC_COLL_WEIGHTS_MAX: C2RustUnnamed_0 = 40;
pub const _SC_BC_STRING_MAX: C2RustUnnamed_0 = 39;
pub const _SC_BC_SCALE_MAX: C2RustUnnamed_0 = 38;
pub const _SC_BC_DIM_MAX: C2RustUnnamed_0 = 37;
pub const _SC_BC_BASE_MAX: C2RustUnnamed_0 = 36;
pub const _SC_TIMER_MAX: C2RustUnnamed_0 = 35;
pub const _SC_SIGQUEUE_MAX: C2RustUnnamed_0 = 34;
pub const _SC_SEM_VALUE_MAX: C2RustUnnamed_0 = 33;
pub const _SC_SEM_NSEMS_MAX: C2RustUnnamed_0 = 32;
pub const _SC_RTSIG_MAX: C2RustUnnamed_0 = 31;
pub const _SC_PAGESIZE: C2RustUnnamed_0 = 30;
pub const _SC_VERSION: C2RustUnnamed_0 = 29;
pub const _SC_MQ_PRIO_MAX: C2RustUnnamed_0 = 28;
pub const _SC_MQ_OPEN_MAX: C2RustUnnamed_0 = 27;
pub const _SC_DELAYTIMER_MAX: C2RustUnnamed_0 = 26;
pub const _SC_AIO_PRIO_DELTA_MAX: C2RustUnnamed_0 = 25;
pub const _SC_AIO_MAX: C2RustUnnamed_0 = 24;
pub const _SC_AIO_LISTIO_MAX: C2RustUnnamed_0 = 23;
pub const _SC_SHARED_MEMORY_OBJECTS: C2RustUnnamed_0 = 22;
pub const _SC_SEMAPHORES: C2RustUnnamed_0 = 21;
pub const _SC_MESSAGE_PASSING: C2RustUnnamed_0 = 20;
pub const _SC_MEMORY_PROTECTION: C2RustUnnamed_0 = 19;
pub const _SC_MEMLOCK_RANGE: C2RustUnnamed_0 = 18;
pub const _SC_MEMLOCK: C2RustUnnamed_0 = 17;
pub const _SC_MAPPED_FILES: C2RustUnnamed_0 = 16;
pub const _SC_FSYNC: C2RustUnnamed_0 = 15;
pub const _SC_SYNCHRONIZED_IO: C2RustUnnamed_0 = 14;
pub const _SC_PRIORITIZED_IO: C2RustUnnamed_0 = 13;
pub const _SC_ASYNCHRONOUS_IO: C2RustUnnamed_0 = 12;
pub const _SC_TIMERS: C2RustUnnamed_0 = 11;
pub const _SC_PRIORITY_SCHEDULING: C2RustUnnamed_0 = 10;
pub const _SC_REALTIME_SIGNALS: C2RustUnnamed_0 = 9;
pub const _SC_SAVED_IDS: C2RustUnnamed_0 = 8;
pub const _SC_JOB_CONTROL: C2RustUnnamed_0 = 7;
pub const _SC_TZNAME_MAX: C2RustUnnamed_0 = 6;
pub const _SC_STREAM_MAX: C2RustUnnamed_0 = 5;
pub const _SC_OPEN_MAX: C2RustUnnamed_0 = 4;
pub const _SC_NGROUPS_MAX: C2RustUnnamed_0 = 3;
pub const _SC_CLK_TCK: C2RustUnnamed_0 = 2;
pub const _SC_CHILD_MAX: C2RustUnnamed_0 = 1;
pub const _SC_ARG_MAX: C2RustUnnamed_0 = 0;
pub type C2RustUnnamed_1 = libc::c_uint;
pub const EXIT_TROUBLE: C2RustUnnamed_1 = 2;
pub type C2RustUnnamed_2 = libc::c_uint;
pub const NCHAR: C2RustUnnamed_2 = 256;
pub type argmatch_exit_fn = Option::<unsafe extern "C" fn() -> ()>;
pub type DIR = __dirstream;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct I_ring {
    pub ir_data: [libc::c_int; 4],
    pub ir_default_val: libc::c_int,
    pub ir_front: libc::c_uint,
    pub ir_back: libc::c_uint,
    pub ir_empty: bool,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct FTS {
    pub fts_cur: *mut _ftsent,
    pub fts_child: *mut _ftsent,
    pub fts_array: *mut *mut _ftsent,
    pub fts_dev: dev_t,
    pub fts_path: *mut libc::c_char,
    pub fts_rfd: libc::c_int,
    pub fts_cwd_fd: libc::c_int,
    pub fts_pathlen: size_t,
    pub fts_nitems: size_t,
    pub fts_compar: Option::<
        unsafe extern "C" fn(*mut *const _ftsent, *mut *const _ftsent) -> libc::c_int,
    >,
    pub fts_options: libc::c_int,
    pub fts_leaf_optimization_works_ht: *mut hash_table,
    pub fts_cycle: C2RustUnnamed_3,
    pub fts_fd_ring: I_ring,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_3 {
    pub ht: *mut hash_table,
    pub state: *mut cycle_check_state,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _ftsent {
    pub fts_cycle: *mut _ftsent,
    pub fts_parent: *mut _ftsent,
    pub fts_link: *mut _ftsent,
    pub fts_dirp: *mut DIR,
    pub fts_number: libc::c_long,
    pub fts_pointer: *mut libc::c_void,
    pub fts_accpath: *mut libc::c_char,
    pub fts_path: *mut libc::c_char,
    pub fts_errno: libc::c_int,
    pub fts_symfd: libc::c_int,
    pub fts_pathlen: size_t,
    pub fts_fts: *mut FTS,
    pub fts_level: ptrdiff_t,
    pub fts_namelen: size_t,
    pub fts_info: libc::c_ushort,
    pub fts_flags: libc::c_ushort,
    pub fts_instr: libc::c_ushort,
    pub fts_statp: [stat; 1],
    pub fts_name: [libc::c_char; 0],
}
pub type FTSENT = _ftsent;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct option {
    pub name: *const libc::c_char,
    pub has_arg: libc::c_int,
    pub flag: *mut libc::c_int,
    pub val: libc::c_int,
}
pub type idx_t = ptrdiff_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct patloc {
    pub lineno: idx_t,
    pub filename: *const libc::c_char,
    pub fileline: idx_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct hash_tuning {
    pub shrink_threshold: libc::c_float,
    pub shrink_factor: libc::c_float,
    pub growth_threshold: libc::c_float,
    pub growth_factor: libc::c_float,
    pub is_n_buckets: bool,
}
pub type Hash_tuning = hash_tuning;
pub type Hash_table = hash_table;
pub type Hash_hasher = Option::<
    unsafe extern "C" fn(*const libc::c_void, size_t) -> size_t,
>;
pub type Hash_comparator = Option::<
    unsafe extern "C" fn(*const libc::c_void, *const libc::c_void) -> bool,
>;
pub type Hash_data_freer = Option::<unsafe extern "C" fn(*mut libc::c_void) -> ()>;
pub type reg_syntax_t = libc::c_ulong;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct localeinfo {
    pub multibyte: bool,
    pub simple: bool,
    pub using_utf8: bool,
    pub sbclen: [libc::c_schar; 256],
    pub sbctowc: [wint_t; 256],
}
pub type strtol_error = libc::c_uint;
pub const LONGINT_INVALID: strtol_error = 4;
pub const LONGINT_INVALID_SUFFIX_CHAR_WITH_OVERFLOW: strtol_error = 3;
pub const LONGINT_INVALID_SUFFIX_CHAR: strtol_error = 2;
pub const LONGINT_OVERFLOW: strtol_error = 1;
pub const LONGINT_OK: strtol_error = 0;
pub type C2RustUnnamed_4 = libc::c_uint;
pub const SEP_CHAR_SELECTED: C2RustUnnamed_4 = 58;
pub type C2RustUnnamed_5 = libc::c_uint;
pub const SEP_CHAR_REJECTED: C2RustUnnamed_5 = 45;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct color_cap {
    pub name: *const libc::c_char,
    pub var: *mut *const libc::c_char,
    pub fct: Option::<unsafe extern "C" fn() -> ()>,
}
pub type C2RustUnnamed_6 = libc::c_uint;
pub const NO_IGNORE_CASE_OPTION: C2RustUnnamed_6 = 137;
pub const LABEL_OPTION: C2RustUnnamed_6 = 136;
pub const LINE_BUFFERED_OPTION: C2RustUnnamed_6 = 135;
pub const INCLUDE_OPTION: C2RustUnnamed_6 = 134;
pub const GROUP_SEPARATOR_OPTION: C2RustUnnamed_6 = 133;
pub const EXCLUDE_FROM_OPTION: C2RustUnnamed_6 = 132;
pub const EXCLUDE_OPTION: C2RustUnnamed_6 = 131;
pub const EXCLUDE_DIRECTORY_OPTION: C2RustUnnamed_6 = 130;
pub const COLOR_OPTION: C2RustUnnamed_6 = 129;
pub const BINARY_FILES_OPTION: C2RustUnnamed_6 = 128;
pub type directories_type = libc::c_uint;
pub const SKIP_DIRECTORIES: directories_type = 4;
pub const RECURSE_DIRECTORIES: directories_type = 3;
pub const READ_DIRECTORIES: directories_type = 2;
pub type C2RustUnnamed_7 = libc::c_uint;
pub const basic_fts_options: C2RustUnnamed_7 = 776;
pub type C2RustUnnamed_8 = libc::c_uint;
pub const SKIP_DEVICES: C2RustUnnamed_8 = 2;
pub const READ_DEVICES: C2RustUnnamed_8 = 1;
pub const READ_COMMAND_LINE_DEVICES: C2RustUnnamed_8 = 0;
pub const LISTFILES_MATCHING: C2RustUnnamed_9 = 1;
pub const LISTFILES_NONMATCHING: C2RustUnnamed_9 = 2;
pub type C2RustUnnamed_9 = libc::c_uint;
pub const LISTFILES_NONE: C2RustUnnamed_9 = 0;
pub const INITIAL_BUFSIZE: C2RustUnnamed_15 = 98304;
pub const BINARY_BINARY_FILES: C2RustUnnamed_10 = 0;
pub type C2RustUnnamed_10 = libc::c_uint;
pub const WITHOUT_MATCH_BINARY_FILES: C2RustUnnamed_10 = 2;
pub const TEXT_BINARY_FILES: C2RustUnnamed_10 = 1;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_11 {
    pub _gl_dummy: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_12 {
    pub _gl_dummy: libc::c_int,
}
pub type uword = uintmax_t;
pub const uword_size: C2RustUnnamed_13 = 8;
pub type execute_fp_t = Option::<
    unsafe extern "C" fn(
        *mut libc::c_void,
        *const libc::c_char,
        idx_t,
        *mut idx_t,
        *const libc::c_char,
    ) -> ptrdiff_t,
>;
pub type compile_fp_t = Option::<
    unsafe extern "C" fn(
        *mut libc::c_char,
        idx_t,
        reg_syntax_t,
        bool,
    ) -> *mut libc::c_void,
>;
pub type C2RustUnnamed_13 = libc::c_uint;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_14 {
    pub _gl_dummy: libc::c_int,
}
pub type C2RustUnnamed_15 = libc::c_uint;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_16 {
    pub name: [libc::c_char; 12],
    pub syntax: libc::c_int,
    pub compile: compile_fp_t,
    pub execute: execute_fp_t,
}
pub type C2RustUnnamed_17 = libc::c_uint;
pub const G_MATCHER_INDEX: C2RustUnnamed_17 = 0;
pub const F_MATCHER_INDEX: C2RustUnnamed_17 = 2;
pub const E_MATCHER_INDEX: C2RustUnnamed_17 = 1;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_18 {
    pub _gl_dummy: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_19 {
    pub _gl_dummy: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_20 {
    pub _gl_dummy: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_21 {
    pub _gl_dummy: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_22 {
    pub _gl_dummy: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_23 {
    pub _gl_dummy: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_24 {
    pub _gl_dummy: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_25 {
    pub _gl_dummy: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_26 {
    pub _gl_dummy: libc::c_int,
}
#[inline]
unsafe extern "C" fn stat(
    mut __path: *const libc::c_char,
    mut __statbuf: *mut stat,
) -> libc::c_int {
    return __xstat(1 as libc::c_int, __path, __statbuf);
}
#[inline]
unsafe extern "C" fn fstat(
    mut __fd: libc::c_int,
    mut __statbuf: *mut stat,
) -> libc::c_int {
    return __fxstat(1 as libc::c_int, __fd, __statbuf);
}
#[inline]
unsafe extern "C" fn fstatat(
    mut __fd: libc::c_int,
    mut __filename: *const libc::c_char,
    mut __statbuf: *mut stat,
    mut __flag: libc::c_int,
) -> libc::c_int {
    return __fxstatat(1 as libc::c_int, __fd, __filename, __statbuf, __flag);
}
#[inline]
unsafe extern "C" fn ferror_unlocked(mut __stream: *mut FILE) -> libc::c_int {
    return ((*__stream)._flags & 0x20 as libc::c_int != 0 as libc::c_int) as libc::c_int;
}
#[inline]
unsafe extern "C" fn putchar_unlocked(mut __c: libc::c_int) -> libc::c_int {
    return if ((*stdout)._IO_write_ptr >= (*stdout)._IO_write_end) as libc::c_int
        as libc::c_long != 0
    {
        __overflow(stdout, __c as libc::c_uchar as libc::c_int)
    } else {
        let fresh0 = (*stdout)._IO_write_ptr;
        (*stdout)._IO_write_ptr = ((*stdout)._IO_write_ptr).offset(1);
        *fresh0 = __c as libc::c_char;
        *fresh0 as libc::c_uchar as libc::c_int
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
unsafe extern "C" fn getc_unlocked(mut __fp: *mut FILE) -> libc::c_int {
    return if ((*__fp)._IO_read_ptr >= (*__fp)._IO_read_end) as libc::c_int
        as libc::c_long != 0
    {
        __uflow(__fp)
    } else {
        let fresh2 = (*__fp)._IO_read_ptr;
        (*__fp)._IO_read_ptr = ((*__fp)._IO_read_ptr).offset(1);
        *(fresh2 as *mut libc::c_uchar) as libc::c_int
    };
}
#[inline]
unsafe extern "C" fn toupper(mut __c: libc::c_int) -> libc::c_int {
    return if __c >= -(128 as libc::c_int) && __c < 256 as libc::c_int {
        *(*__ctype_toupper_loc()).offset(__c as isize)
    } else {
        __c
    };
}
#[inline]
unsafe extern "C" fn to_uchar(mut ch: libc::c_char) -> libc::c_uchar {
    return ch as libc::c_uchar;
}
#[inline]
unsafe extern "C" fn c_isdigit(mut c: libc::c_int) -> bool {
    match c {
        48 | 49 | 50 | 51 | 52 | 53 | 54 | 55 | 56 | 57 => return 1 as libc::c_int != 0,
        _ => return 0 as libc::c_int != 0,
    };
}
#[inline]
unsafe extern "C" fn imbrlen(
    mut s: *const libc::c_char,
    mut n: idx_t,
    mut mbs: *mut mbstate_t,
) -> ptrdiff_t {
    let mut len: size_t = rpl_mbrlen(s, n as size_t, mbs);
    if len <= 16 as libc::c_int as libc::c_ulong {
        return len as ptrdiff_t;
    }
    let mut neglen: ptrdiff_t = len.wrapping_neg() as ptrdiff_t;
    return -neglen;
}
#[inline]
unsafe extern "C" fn mb_clen(
    mut s: *const libc::c_char,
    mut n: idx_t,
    mut mbs: *mut mbstate_t,
) -> ptrdiff_t {
    let mut len: libc::c_schar = localeinfo.sbclen[to_uchar(*s) as usize];
    return if len as libc::c_int == -(2 as libc::c_int) {
        imbrlen(s, n, mbs)
    } else {
        len as libc::c_long
    };
}
#[inline]
unsafe extern "C" fn xset_binary_mode_error() {}
#[inline]
unsafe extern "C" fn xset_binary_mode(mut fd: libc::c_int, mut mode: libc::c_int) {
    if set_binary_mode(fd, mode) < 0 as libc::c_int {
        xset_binary_mode_error();
    }
}
#[inline]
unsafe extern "C" fn __gl_setmode(
    mut fd: libc::c_int,
    mut mode: libc::c_int,
) -> libc::c_int {
    return 0 as libc::c_int;
}
#[inline]
unsafe extern "C" fn set_binary_mode(
    mut fd: libc::c_int,
    mut mode: libc::c_int,
) -> libc::c_int {
    return __gl_setmode(fd, mode);
}
static mut SEP_STR_GROUP: [libc::c_char; 3] = unsafe {
    *::std::mem::transmute::<&[u8; 3], &[libc::c_char; 3]>(b"--\0")
};
static mut out_stat: stat = stat {
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
static mut show_help: libc::c_int = 0;
static mut show_version: bool = false;
static mut suppress_errors: bool = false;
static mut color_option: libc::c_int = 0;
static mut only_matching: bool = false;
static mut align_tabs: bool = false;
static mut offset_width: libc::c_int = 0;
static mut patloc: *mut patloc = 0 as *const patloc as *mut patloc;
static mut patlocs_allocated: idx_t = 0;
static mut patlocs_used: idx_t = 0;
static mut pattern_array: *mut libc::c_char = 0 as *const libc::c_char
    as *mut libc::c_char;
static mut n_patterns: idx_t = 0;
static mut pattern_table: *mut Hash_table = 0 as *const Hash_table as *mut Hash_table;
unsafe extern "C" fn hash_pattern(
    mut pat: *const libc::c_void,
    mut n_buckets: size_t,
) -> size_t {
    let mut h15: uint_fast64_t = 5381 as libc::c_int as uint_fast64_t;
    let mut h32: uint_fast64_t = 3657500101 as libc::c_long as uint_fast64_t;
    let mut h64: uint_fast64_t = 4123221751654370051 as libc::c_long as uint_fast64_t;
    let mut h: size_t = if h64 <= 18446744073709551615 as libc::c_ulong {
        h64
    } else if h32 <= 18446744073709551615 as libc::c_ulong {
        h32
    } else {
        h15
    };
    let mut pat_offset: intptr_t = pat as intptr_t - 1 as libc::c_int as libc::c_long;
    let mut s: *const libc::c_uchar = (pattern_array as *const libc::c_uchar)
        .offset(pat_offset as isize);
    while *s as libc::c_int != '\n' as i32 {
        h = h.wrapping_mul(33 as libc::c_int as libc::c_ulong) ^ *s as libc::c_ulong;
        s = s.offset(1);
        s;
    }
    return h.wrapping_rem(n_buckets);
}
unsafe extern "C" fn compare_patterns(
    mut a: *const libc::c_void,
    mut b: *const libc::c_void,
) -> bool {
    let mut a_offset: intptr_t = a as intptr_t - 1 as libc::c_int as libc::c_long;
    let mut b_offset: intptr_t = b as intptr_t - 1 as libc::c_int as libc::c_long;
    let mut p: *const libc::c_char = pattern_array.offset(a_offset as isize);
    let mut q: *const libc::c_char = pattern_array.offset(b_offset as isize);
    while *p as libc::c_int == *q as libc::c_int {
        if *p as libc::c_int == '\n' as i32 {
            return 1 as libc::c_int != 0;
        }
        p = p.offset(1);
        p;
        q = q.offset(1);
        q;
    }
    return 0 as libc::c_int != 0;
}
unsafe extern "C" fn update_patterns(
    mut keys: *mut libc::c_char,
    mut dupfree_size: idx_t,
    mut size: idx_t,
    mut filename_0: *const libc::c_char,
) -> idx_t {
    let mut dst: *mut libc::c_char = keys.offset(dupfree_size as isize);
    let mut fileline: idx_t = 1 as libc::c_int as idx_t;
    let mut prev_inserted: libc::c_int = 0 as libc::c_int;
    let mut srclim: *const libc::c_char = keys.offset(size as isize);
    let mut patsize: idx_t = 0;
    let mut src: *const libc::c_char = keys.offset(dupfree_size as isize);
    while src < srclim {
        let mut patend: *const libc::c_char = rawmemchr(
            src as *const libc::c_void,
            '\n' as i32,
        ) as *const libc::c_char;
        patsize = patend.offset(1 as libc::c_int as isize).offset_from(src)
            as libc::c_long;
        memmove(
            dst as *mut libc::c_void,
            src as *const libc::c_void,
            patsize as libc::c_ulong,
        );
        let mut dst_offset_1: intptr_t = dst.offset_from(keys) as libc::c_long
            + 1 as libc::c_int as libc::c_long;
        let mut inserted: libc::c_int = hash_insert_if_absent(
            pattern_table,
            dst_offset_1 as *mut libc::c_void,
            0 as *mut *const libc::c_void,
        );
        if inserted != 0 {
            if inserted < 0 as libc::c_int {
                xalloc_die();
            }
            dst = dst.offset(patsize as isize);
            if prev_inserted == 0 {
                if patlocs_used == patlocs_allocated {
                    patloc = xpalloc(
                        patloc as *mut libc::c_void,
                        &mut patlocs_allocated,
                        1 as libc::c_int as idx_t,
                        -(1 as libc::c_int) as ptrdiff_t,
                        ::std::mem::size_of::<patloc>() as libc::c_ulong as idx_t,
                    ) as *mut patloc;
                }
                let fresh3 = patlocs_used;
                patlocs_used = patlocs_used + 1;
                *patloc
                    .offset(
                        fresh3 as isize,
                    ) = {
                    let mut init = patloc {
                        lineno: n_patterns,
                        filename: filename_0,
                        fileline: fileline,
                    };
                    init
                };
            }
            n_patterns += 1;
            n_patterns;
        }
        prev_inserted = inserted;
        fileline += 1;
        fileline;
        src = src.offset(patsize as isize);
    }
    return dst.offset_from(keys) as libc::c_long;
}
pub unsafe extern "C" fn pattern_file_name(
    mut lineno: idx_t,
    mut new_lineno: *mut idx_t,
) -> *const libc::c_char {
    let mut i: idx_t = 0;
    i = 1 as libc::c_int as idx_t;
    while i < patlocs_used {
        if lineno < (*patloc.offset(i as isize)).lineno {
            break;
        }
        i += 1;
        i;
    }
    *new_lineno = lineno
        - (*patloc.offset((i - 1 as libc::c_int as libc::c_long) as isize)).lineno
        + (*patloc.offset((i - 1 as libc::c_int as libc::c_long) as isize)).fileline;
    return (*patloc.offset((i - 1 as libc::c_int as libc::c_long) as isize)).filename;
}
unsafe extern "C" fn clear_asan_poison() {}
unsafe extern "C" fn asan_poison(mut addr: *const libc::c_void, mut size: idx_t) {}
static mut group_separator: *const libc::c_char = unsafe { SEP_STR_GROUP.as_ptr() };
static mut selected_match_color: *const libc::c_char = b"01;31\0" as *const u8
    as *const libc::c_char;
static mut context_match_color: *const libc::c_char = b"01;31\0" as *const u8
    as *const libc::c_char;
static mut filename_color: *const libc::c_char = b"35\0" as *const u8
    as *const libc::c_char;
static mut line_num_color: *const libc::c_char = b"32\0" as *const u8
    as *const libc::c_char;
static mut byte_num_color: *const libc::c_char = b"32\0" as *const u8
    as *const libc::c_char;
static mut sep_color: *const libc::c_char = b"36\0" as *const u8 as *const libc::c_char;
static mut selected_line_color: *const libc::c_char = b"\0" as *const u8
    as *const libc::c_char;
static mut context_line_color: *const libc::c_char = b"\0" as *const u8
    as *const libc::c_char;
static mut sgr_start: *const libc::c_char = b"\x1B[%sm\x1B[K\0" as *const u8
    as *const libc::c_char;
static mut sgr_end: *const libc::c_char = b"\x1B[m\x1B[K\0" as *const u8
    as *const libc::c_char;
unsafe extern "C" fn pr_sgr_start(mut s: *const libc::c_char) {
    if *s != 0 {
        print_start_colorize(sgr_start, s);
    }
}
unsafe extern "C" fn pr_sgr_end(mut s: *const libc::c_char) {
    if *s != 0 {
        print_end_colorize(sgr_end);
    }
}
unsafe extern "C" fn pr_sgr_start_if(mut s: *const libc::c_char) {
    if color_option != 0 {
        pr_sgr_start(s);
    }
}
unsafe extern "C" fn pr_sgr_end_if(mut s: *const libc::c_char) {
    if color_option != 0 {
        pr_sgr_end(s);
    }
}
unsafe extern "C" fn color_cap_mt_fct() {
    context_match_color = selected_match_color;
}
unsafe extern "C" fn color_cap_rv_fct() {
    color_option = -(1 as libc::c_int);
}
unsafe extern "C" fn color_cap_ne_fct() {
    sgr_start = b"\x1B[%sm\0" as *const u8 as *const libc::c_char;
    sgr_end = b"\x1B[m\0" as *const u8 as *const libc::c_char;
}
static mut color_dict: [color_cap; 12] = unsafe {
    [
        {
            let mut init = color_cap {
                name: b"mt\0" as *const u8 as *const libc::c_char,
                var: &selected_match_color as *const *const libc::c_char
                    as *mut *const libc::c_char,
                fct: Some(color_cap_mt_fct as unsafe extern "C" fn() -> ()),
            };
            init
        },
        {
            let mut init = color_cap {
                name: b"ms\0" as *const u8 as *const libc::c_char,
                var: &selected_match_color as *const *const libc::c_char
                    as *mut *const libc::c_char,
                fct: None,
            };
            init
        },
        {
            let mut init = color_cap {
                name: b"mc\0" as *const u8 as *const libc::c_char,
                var: &context_match_color as *const *const libc::c_char
                    as *mut *const libc::c_char,
                fct: None,
            };
            init
        },
        {
            let mut init = color_cap {
                name: b"fn\0" as *const u8 as *const libc::c_char,
                var: &filename_color as *const *const libc::c_char
                    as *mut *const libc::c_char,
                fct: None,
            };
            init
        },
        {
            let mut init = color_cap {
                name: b"ln\0" as *const u8 as *const libc::c_char,
                var: &line_num_color as *const *const libc::c_char
                    as *mut *const libc::c_char,
                fct: None,
            };
            init
        },
        {
            let mut init = color_cap {
                name: b"bn\0" as *const u8 as *const libc::c_char,
                var: &byte_num_color as *const *const libc::c_char
                    as *mut *const libc::c_char,
                fct: None,
            };
            init
        },
        {
            let mut init = color_cap {
                name: b"se\0" as *const u8 as *const libc::c_char,
                var: &sep_color as *const *const libc::c_char
                    as *mut *const libc::c_char,
                fct: None,
            };
            init
        },
        {
            let mut init = color_cap {
                name: b"sl\0" as *const u8 as *const libc::c_char,
                var: &selected_line_color as *const *const libc::c_char
                    as *mut *const libc::c_char,
                fct: None,
            };
            init
        },
        {
            let mut init = color_cap {
                name: b"cx\0" as *const u8 as *const libc::c_char,
                var: &context_line_color as *const *const libc::c_char
                    as *mut *const libc::c_char,
                fct: None,
            };
            init
        },
        {
            let mut init = color_cap {
                name: b"rv\0" as *const u8 as *const libc::c_char,
                var: 0 as *const *const libc::c_char as *mut *const libc::c_char,
                fct: Some(color_cap_rv_fct as unsafe extern "C" fn() -> ()),
            };
            init
        },
        {
            let mut init = color_cap {
                name: b"ne\0" as *const u8 as *const libc::c_char,
                var: 0 as *const *const libc::c_char as *mut *const libc::c_char,
                fct: Some(color_cap_ne_fct as unsafe extern "C" fn() -> ()),
            };
            init
        },
        {
            let mut init = color_cap {
                name: 0 as *const libc::c_char,
                var: 0 as *const *const libc::c_char as *mut *const libc::c_char,
                fct: None,
            };
            init
        },
    ]
};
static mut stdout_errno: libc::c_int = 0;
unsafe extern "C" fn putchar_errno(mut c: libc::c_int) {
    if putchar_unlocked(c) < 0 as libc::c_int {
        stdout_errno = *__errno_location();
    }
}
unsafe extern "C" fn fputs_errno(mut s: *const libc::c_char) {
    if fputs_unlocked(s, stdout) < 0 as libc::c_int {
        stdout_errno = *__errno_location();
    }
}
unsafe extern "C" fn printf_errno(mut format: *const libc::c_char, mut args: ...) {
    let mut ap: ::std::ffi::VaListImpl;
    ap = args.clone();
    if vfprintf(stdout, format, ap.as_va_list()) < 0 as libc::c_int {
        stdout_errno = *__errno_location();
    }
}
unsafe extern "C" fn fwrite_errno(
    mut ptr: *const libc::c_void,
    mut size: idx_t,
    mut nmemb: idx_t,
) {
    if (if 0 != 0 && 0 != 0
        && (size as size_t).wrapping_mul(nmemb as size_t)
            <= 8 as libc::c_int as libc::c_ulong
        && size as size_t != 0 as libc::c_int as libc::c_ulong
    {
        {
            let mut __ptr: *const libc::c_char = ptr as *const libc::c_char;
            let mut __stream: *mut FILE = stdout;
            let mut __cnt: size_t = 0;
            __cnt = (size as size_t).wrapping_mul(nmemb as size_t);
            while __cnt > 0 as libc::c_int as libc::c_ulong {
                let fresh4 = __ptr;
                __ptr = __ptr.offset(1);
                if putc_unlocked(*fresh4 as libc::c_int, __stream) == -(1 as libc::c_int)
                {
                    break;
                }
                __cnt = __cnt.wrapping_sub(1);
                __cnt;
            }
            (size as size_t)
                .wrapping_mul(nmemb as size_t)
                .wrapping_sub(__cnt)
                .wrapping_div(size as size_t)
        }
    } else {
        if 0 != 0 && size as size_t == 0 as libc::c_int as libc::c_ulong
            || 0 != 0 && nmemb as size_t == 0 as libc::c_int as libc::c_ulong
        {
            0 as libc::c_int as size_t
        } else {
            fwrite_unlocked(ptr, size as size_t, nmemb as size_t, stdout)
        }
    }) != nmemb as libc::c_ulong
    {
        stdout_errno = *__errno_location();
    }
}
unsafe extern "C" fn fflush_errno() {
    if fflush_unlocked(stdout) != 0 as libc::c_int {
        stdout_errno = *__errno_location();
    }
}
static mut excluded_patterns: [*mut exclude; 2] = [0 as *const exclude
    as *mut exclude; 2];
static mut excluded_directory_patterns: [*mut exclude; 2] = [0 as *const exclude
    as *mut exclude; 2];
static mut short_options: [libc::c_char; 58] = unsafe {
    *::std::mem::transmute::<
        &[u8; 58],
        &[libc::c_char; 58],
    >(b"0123456789A:B:C:D:EFGHIPTUVX:abcd:e:f:hiLlm:noqRrsuvwxyZz\0")
};
static mut long_options: [option; 51] = unsafe {
    [
        {
            let mut init = option {
                name: b"basic-regexp\0" as *const u8 as *const libc::c_char,
                has_arg: 0 as libc::c_int,
                flag: 0 as *const libc::c_int as *mut libc::c_int,
                val: 'G' as i32,
            };
            init
        },
        {
            let mut init = option {
                name: b"extended-regexp\0" as *const u8 as *const libc::c_char,
                has_arg: 0 as libc::c_int,
                flag: 0 as *const libc::c_int as *mut libc::c_int,
                val: 'E' as i32,
            };
            init
        },
        {
            let mut init = option {
                name: b"fixed-regexp\0" as *const u8 as *const libc::c_char,
                has_arg: 0 as libc::c_int,
                flag: 0 as *const libc::c_int as *mut libc::c_int,
                val: 'F' as i32,
            };
            init
        },
        {
            let mut init = option {
                name: b"fixed-strings\0" as *const u8 as *const libc::c_char,
                has_arg: 0 as libc::c_int,
                flag: 0 as *const libc::c_int as *mut libc::c_int,
                val: 'F' as i32,
            };
            init
        },
        {
            let mut init = option {
                name: b"perl-regexp\0" as *const u8 as *const libc::c_char,
                has_arg: 0 as libc::c_int,
                flag: 0 as *const libc::c_int as *mut libc::c_int,
                val: 'P' as i32,
            };
            init
        },
        {
            let mut init = option {
                name: b"after-context\0" as *const u8 as *const libc::c_char,
                has_arg: 1 as libc::c_int,
                flag: 0 as *const libc::c_int as *mut libc::c_int,
                val: 'A' as i32,
            };
            init
        },
        {
            let mut init = option {
                name: b"before-context\0" as *const u8 as *const libc::c_char,
                has_arg: 1 as libc::c_int,
                flag: 0 as *const libc::c_int as *mut libc::c_int,
                val: 'B' as i32,
            };
            init
        },
        {
            let mut init = option {
                name: b"binary-files\0" as *const u8 as *const libc::c_char,
                has_arg: 1 as libc::c_int,
                flag: 0 as *const libc::c_int as *mut libc::c_int,
                val: BINARY_FILES_OPTION as libc::c_int,
            };
            init
        },
        {
            let mut init = option {
                name: b"byte-offset\0" as *const u8 as *const libc::c_char,
                has_arg: 0 as libc::c_int,
                flag: 0 as *const libc::c_int as *mut libc::c_int,
                val: 'b' as i32,
            };
            init
        },
        {
            let mut init = option {
                name: b"context\0" as *const u8 as *const libc::c_char,
                has_arg: 1 as libc::c_int,
                flag: 0 as *const libc::c_int as *mut libc::c_int,
                val: 'C' as i32,
            };
            init
        },
        {
            let mut init = option {
                name: b"color\0" as *const u8 as *const libc::c_char,
                has_arg: 2 as libc::c_int,
                flag: 0 as *const libc::c_int as *mut libc::c_int,
                val: COLOR_OPTION as libc::c_int,
            };
            init
        },
        {
            let mut init = option {
                name: b"colour\0" as *const u8 as *const libc::c_char,
                has_arg: 2 as libc::c_int,
                flag: 0 as *const libc::c_int as *mut libc::c_int,
                val: COLOR_OPTION as libc::c_int,
            };
            init
        },
        {
            let mut init = option {
                name: b"count\0" as *const u8 as *const libc::c_char,
                has_arg: 0 as libc::c_int,
                flag: 0 as *const libc::c_int as *mut libc::c_int,
                val: 'c' as i32,
            };
            init
        },
        {
            let mut init = option {
                name: b"devices\0" as *const u8 as *const libc::c_char,
                has_arg: 1 as libc::c_int,
                flag: 0 as *const libc::c_int as *mut libc::c_int,
                val: 'D' as i32,
            };
            init
        },
        {
            let mut init = option {
                name: b"directories\0" as *const u8 as *const libc::c_char,
                has_arg: 1 as libc::c_int,
                flag: 0 as *const libc::c_int as *mut libc::c_int,
                val: 'd' as i32,
            };
            init
        },
        {
            let mut init = option {
                name: b"exclude\0" as *const u8 as *const libc::c_char,
                has_arg: 1 as libc::c_int,
                flag: 0 as *const libc::c_int as *mut libc::c_int,
                val: EXCLUDE_OPTION as libc::c_int,
            };
            init
        },
        {
            let mut init = option {
                name: b"exclude-from\0" as *const u8 as *const libc::c_char,
                has_arg: 1 as libc::c_int,
                flag: 0 as *const libc::c_int as *mut libc::c_int,
                val: EXCLUDE_FROM_OPTION as libc::c_int,
            };
            init
        },
        {
            let mut init = option {
                name: b"exclude-dir\0" as *const u8 as *const libc::c_char,
                has_arg: 1 as libc::c_int,
                flag: 0 as *const libc::c_int as *mut libc::c_int,
                val: EXCLUDE_DIRECTORY_OPTION as libc::c_int,
            };
            init
        },
        {
            let mut init = option {
                name: b"file\0" as *const u8 as *const libc::c_char,
                has_arg: 1 as libc::c_int,
                flag: 0 as *const libc::c_int as *mut libc::c_int,
                val: 'f' as i32,
            };
            init
        },
        {
            let mut init = option {
                name: b"files-with-matches\0" as *const u8 as *const libc::c_char,
                has_arg: 0 as libc::c_int,
                flag: 0 as *const libc::c_int as *mut libc::c_int,
                val: 'l' as i32,
            };
            init
        },
        {
            let mut init = option {
                name: b"files-without-match\0" as *const u8 as *const libc::c_char,
                has_arg: 0 as libc::c_int,
                flag: 0 as *const libc::c_int as *mut libc::c_int,
                val: 'L' as i32,
            };
            init
        },
        {
            let mut init = option {
                name: b"group-separator\0" as *const u8 as *const libc::c_char,
                has_arg: 1 as libc::c_int,
                flag: 0 as *const libc::c_int as *mut libc::c_int,
                val: GROUP_SEPARATOR_OPTION as libc::c_int,
            };
            init
        },
        {
            let mut init = option {
                name: b"help\0" as *const u8 as *const libc::c_char,
                has_arg: 0 as libc::c_int,
                flag: &show_help as *const libc::c_int as *mut libc::c_int,
                val: 1 as libc::c_int,
            };
            init
        },
        {
            let mut init = option {
                name: b"include\0" as *const u8 as *const libc::c_char,
                has_arg: 1 as libc::c_int,
                flag: 0 as *const libc::c_int as *mut libc::c_int,
                val: INCLUDE_OPTION as libc::c_int,
            };
            init
        },
        {
            let mut init = option {
                name: b"ignore-case\0" as *const u8 as *const libc::c_char,
                has_arg: 0 as libc::c_int,
                flag: 0 as *const libc::c_int as *mut libc::c_int,
                val: 'i' as i32,
            };
            init
        },
        {
            let mut init = option {
                name: b"no-ignore-case\0" as *const u8 as *const libc::c_char,
                has_arg: 0 as libc::c_int,
                flag: 0 as *const libc::c_int as *mut libc::c_int,
                val: NO_IGNORE_CASE_OPTION as libc::c_int,
            };
            init
        },
        {
            let mut init = option {
                name: b"initial-tab\0" as *const u8 as *const libc::c_char,
                has_arg: 0 as libc::c_int,
                flag: 0 as *const libc::c_int as *mut libc::c_int,
                val: 'T' as i32,
            };
            init
        },
        {
            let mut init = option {
                name: b"label\0" as *const u8 as *const libc::c_char,
                has_arg: 1 as libc::c_int,
                flag: 0 as *const libc::c_int as *mut libc::c_int,
                val: LABEL_OPTION as libc::c_int,
            };
            init
        },
        {
            let mut init = option {
                name: b"line-buffered\0" as *const u8 as *const libc::c_char,
                has_arg: 0 as libc::c_int,
                flag: 0 as *const libc::c_int as *mut libc::c_int,
                val: LINE_BUFFERED_OPTION as libc::c_int,
            };
            init
        },
        {
            let mut init = option {
                name: b"line-number\0" as *const u8 as *const libc::c_char,
                has_arg: 0 as libc::c_int,
                flag: 0 as *const libc::c_int as *mut libc::c_int,
                val: 'n' as i32,
            };
            init
        },
        {
            let mut init = option {
                name: b"line-regexp\0" as *const u8 as *const libc::c_char,
                has_arg: 0 as libc::c_int,
                flag: 0 as *const libc::c_int as *mut libc::c_int,
                val: 'x' as i32,
            };
            init
        },
        {
            let mut init = option {
                name: b"max-count\0" as *const u8 as *const libc::c_char,
                has_arg: 1 as libc::c_int,
                flag: 0 as *const libc::c_int as *mut libc::c_int,
                val: 'm' as i32,
            };
            init
        },
        {
            let mut init = option {
                name: b"no-filename\0" as *const u8 as *const libc::c_char,
                has_arg: 0 as libc::c_int,
                flag: 0 as *const libc::c_int as *mut libc::c_int,
                val: 'h' as i32,
            };
            init
        },
        {
            let mut init = option {
                name: b"no-group-separator\0" as *const u8 as *const libc::c_char,
                has_arg: 0 as libc::c_int,
                flag: 0 as *const libc::c_int as *mut libc::c_int,
                val: GROUP_SEPARATOR_OPTION as libc::c_int,
            };
            init
        },
        {
            let mut init = option {
                name: b"no-messages\0" as *const u8 as *const libc::c_char,
                has_arg: 0 as libc::c_int,
                flag: 0 as *const libc::c_int as *mut libc::c_int,
                val: 's' as i32,
            };
            init
        },
        {
            let mut init = option {
                name: b"null\0" as *const u8 as *const libc::c_char,
                has_arg: 0 as libc::c_int,
                flag: 0 as *const libc::c_int as *mut libc::c_int,
                val: 'Z' as i32,
            };
            init
        },
        {
            let mut init = option {
                name: b"null-data\0" as *const u8 as *const libc::c_char,
                has_arg: 0 as libc::c_int,
                flag: 0 as *const libc::c_int as *mut libc::c_int,
                val: 'z' as i32,
            };
            init
        },
        {
            let mut init = option {
                name: b"only-matching\0" as *const u8 as *const libc::c_char,
                has_arg: 0 as libc::c_int,
                flag: 0 as *const libc::c_int as *mut libc::c_int,
                val: 'o' as i32,
            };
            init
        },
        {
            let mut init = option {
                name: b"quiet\0" as *const u8 as *const libc::c_char,
                has_arg: 0 as libc::c_int,
                flag: 0 as *const libc::c_int as *mut libc::c_int,
                val: 'q' as i32,
            };
            init
        },
        {
            let mut init = option {
                name: b"recursive\0" as *const u8 as *const libc::c_char,
                has_arg: 0 as libc::c_int,
                flag: 0 as *const libc::c_int as *mut libc::c_int,
                val: 'r' as i32,
            };
            init
        },
        {
            let mut init = option {
                name: b"dereference-recursive\0" as *const u8 as *const libc::c_char,
                has_arg: 0 as libc::c_int,
                flag: 0 as *const libc::c_int as *mut libc::c_int,
                val: 'R' as i32,
            };
            init
        },
        {
            let mut init = option {
                name: b"regexp\0" as *const u8 as *const libc::c_char,
                has_arg: 1 as libc::c_int,
                flag: 0 as *const libc::c_int as *mut libc::c_int,
                val: 'e' as i32,
            };
            init
        },
        {
            let mut init = option {
                name: b"invert-match\0" as *const u8 as *const libc::c_char,
                has_arg: 0 as libc::c_int,
                flag: 0 as *const libc::c_int as *mut libc::c_int,
                val: 'v' as i32,
            };
            init
        },
        {
            let mut init = option {
                name: b"silent\0" as *const u8 as *const libc::c_char,
                has_arg: 0 as libc::c_int,
                flag: 0 as *const libc::c_int as *mut libc::c_int,
                val: 'q' as i32,
            };
            init
        },
        {
            let mut init = option {
                name: b"text\0" as *const u8 as *const libc::c_char,
                has_arg: 0 as libc::c_int,
                flag: 0 as *const libc::c_int as *mut libc::c_int,
                val: 'a' as i32,
            };
            init
        },
        {
            let mut init = option {
                name: b"binary\0" as *const u8 as *const libc::c_char,
                has_arg: 0 as libc::c_int,
                flag: 0 as *const libc::c_int as *mut libc::c_int,
                val: 'U' as i32,
            };
            init
        },
        {
            let mut init = option {
                name: b"unix-byte-offsets\0" as *const u8 as *const libc::c_char,
                has_arg: 0 as libc::c_int,
                flag: 0 as *const libc::c_int as *mut libc::c_int,
                val: 'u' as i32,
            };
            init
        },
        {
            let mut init = option {
                name: b"version\0" as *const u8 as *const libc::c_char,
                has_arg: 0 as libc::c_int,
                flag: 0 as *const libc::c_int as *mut libc::c_int,
                val: 'V' as i32,
            };
            init
        },
        {
            let mut init = option {
                name: b"with-filename\0" as *const u8 as *const libc::c_char,
                has_arg: 0 as libc::c_int,
                flag: 0 as *const libc::c_int as *mut libc::c_int,
                val: 'H' as i32,
            };
            init
        },
        {
            let mut init = option {
                name: b"word-regexp\0" as *const u8 as *const libc::c_char,
                has_arg: 0 as libc::c_int,
                flag: 0 as *const libc::c_int as *mut libc::c_int,
                val: 'w' as i32,
            };
            init
        },
        {
            let mut init = option {
                name: 0 as *const libc::c_char,
                has_arg: 0 as libc::c_int,
                flag: 0 as *const libc::c_int as *mut libc::c_int,
                val: 0 as libc::c_int,
            };
            init
        },
    ]
};
pub static mut match_icase: bool = false;
pub static mut match_words: bool = false;
pub static mut match_lines: bool = false;
pub static mut eolbyte: libc::c_char = 0;
static mut filename: *const libc::c_char = 0 as *const libc::c_char;
static mut omit_dot_slash: bool = false;
static mut errseen: bool = false;
static mut encoding_error_output: bool = false;
static mut directories_args: [*const libc::c_char; 4] = [
    b"read\0" as *const u8 as *const libc::c_char,
    b"recurse\0" as *const u8 as *const libc::c_char,
    b"skip\0" as *const u8 as *const libc::c_char,
    0 as *const libc::c_char,
];
static mut directories_types: [directories_type; 3] = [
    READ_DIRECTORIES,
    RECURSE_DIRECTORIES,
    SKIP_DIRECTORIES,
];
static mut directories: directories_type = READ_DIRECTORIES;
static mut fts_options: libc::c_int = basic_fts_options as libc::c_int
    | 0x1 as libc::c_int | 0x10 as libc::c_int;
static mut devices: C2RustUnnamed_8 = READ_COMMAND_LINE_DEVICES;
unsafe extern "C" fn is_device_mode(mut m: mode_t) -> bool {
    return m & 0o170000 as libc::c_int as libc::c_uint
        == 0o20000 as libc::c_int as libc::c_uint
        || m & 0o170000 as libc::c_int as libc::c_uint
            == 0o60000 as libc::c_int as libc::c_uint
        || m & 0o170000 as libc::c_int as libc::c_uint
            == 0o140000 as libc::c_int as libc::c_uint
        || m & 0o170000 as libc::c_int as libc::c_uint
            == 0o10000 as libc::c_int as libc::c_uint;
}
unsafe extern "C" fn skip_devices(mut command_line: bool) -> bool {
    return devices as libc::c_uint == SKIP_DEVICES as libc::c_int as libc::c_uint
        || (devices as libc::c_uint
            == READ_COMMAND_LINE_DEVICES as libc::c_int as libc::c_uint) as libc::c_int
            & !command_line as libc::c_int != 0;
}
unsafe extern "C" fn usable_st_size(mut st: *const stat) -> bool {
    return (*st).st_mode & 0o170000 as libc::c_int as libc::c_uint
        == 0o100000 as libc::c_int as libc::c_uint
        || ((*st).st_mode).wrapping_sub((*st).st_mode) != 0 || 0 as libc::c_int != 0;
}
static mut seek_failed: bool = false;
static mut seek_data_failed: bool = false;
static mut execute: execute_fp_t = None;
static mut compiled_pattern: *mut libc::c_void = 0 as *const libc::c_void
    as *mut libc::c_void;
pub unsafe extern "C" fn input_filename() -> *const libc::c_char {
    if filename.is_null() {
        filename = dcgettext(
            0 as *const libc::c_char,
            b"(standard input)\0" as *const u8 as *const libc::c_char,
            5 as libc::c_int,
        );
    }
    return filename;
}
unsafe extern "C" fn suppressible_error(mut errnum: libc::c_int) {
    if !suppress_errors {
        error(
            0 as libc::c_int,
            errnum,
            b"%s\0" as *const u8 as *const libc::c_char,
            input_filename(),
        );
    }
    errseen = 1 as libc::c_int != 0;
}
unsafe extern "C" fn clean_up_stdout() {
    if stdout_errno == 0 {
        close_stdout();
    }
}
static mut uword_max: uword = 18446744073709551615 as libc::c_ulong;
pub static mut localeinfo: localeinfo = localeinfo {
    multibyte: false,
    simple: false,
    using_utf8: false,
    sbclen: [0; 256],
    sbctowc: [0; 256],
};
static mut unibyte_mask: uword = 0;
unsafe extern "C" fn initialize_unibyte_mask() {
    let mut mask: libc::c_uchar = 0 as libc::c_int as libc::c_uchar;
    let mut ms1b: libc::c_int = 1 as libc::c_int;
    let mut i: libc::c_int = 1 as libc::c_int;
    while i <= 127 as libc::c_int * 2 as libc::c_int + 1 as libc::c_int {
        if (localeinfo.sbclen[i as usize] as libc::c_int != 1 as libc::c_int)
            as libc::c_int & (mask as libc::c_int & i == 0) as libc::c_int != 0
        {
            while ms1b * 2 as libc::c_int <= i {
                ms1b *= 2 as libc::c_int;
            }
            mask = (mask as libc::c_int | ms1b) as libc::c_uchar;
        }
        i += 1;
        i;
    }
    unibyte_mask = uword_max
        .wrapping_div(
            (127 as libc::c_int * 2 as libc::c_int + 1 as libc::c_int) as libc::c_ulong,
        )
        .wrapping_mul(mask as libc::c_ulong);
}
unsafe extern "C" fn skip_easy_bytes(
    mut buf: *const libc::c_char,
) -> *const libc::c_char {
    let mut p: *const libc::c_char = 0 as *const libc::c_char;
    let mut s: *const uword = 0 as *const uword;
    p = buf;
    while (p as uintptr_t).wrapping_rem(uword_size as libc::c_int as libc::c_ulong)
        != 0 as libc::c_int as libc::c_ulong
    {
        if to_uchar(*p) as libc::c_ulong & unibyte_mask != 0 {
            return p;
        }
        p = p.offset(1);
        p;
    }
    s = p as *const uword;
    while *s & unibyte_mask == 0 {
        s = s.offset(1);
        s;
    }
    p = s as *const libc::c_char;
    while to_uchar(*p) as libc::c_ulong & unibyte_mask == 0 {
        p = p.offset(1);
        p;
    }
    return p;
}
unsafe extern "C" fn buf_has_encoding_errors(
    mut buf: *mut libc::c_char,
    mut size: idx_t,
) -> bool {
    if unibyte_mask == 0 {
        return 0 as libc::c_int != 0;
    }
    let mut mbs: mbstate_t = {
        let mut init = __mbstate_t {
            __count: 0 as libc::c_int,
            __value: C2RustUnnamed { __wch: 0 },
        };
        init
    };
    let mut clen: ptrdiff_t = 0;
    *buf.offset(size as isize) = -(1 as libc::c_int) as libc::c_char;
    let mut p: *const libc::c_char = buf;
    loop {
        p = skip_easy_bytes(p);
        if !(p < buf.offset(size as isize) as *const libc::c_char) {
            break;
        }
        clen = imbrlen(
            p,
            buf.offset(size as isize).offset_from(p) as libc::c_long,
            &mut mbs,
        );
        if clen < 0 as libc::c_int as libc::c_long {
            return 1 as libc::c_int != 0;
        }
        p = p.offset(clen as isize);
    }
    return 0 as libc::c_int != 0;
}
unsafe extern "C" fn buf_has_nulls(mut buf: *mut libc::c_char, mut size: idx_t) -> bool {
    *buf.offset(size as isize) = 0 as libc::c_int as libc::c_char;
    return strlen(buf) != size as libc::c_ulong;
}
unsafe extern "C" fn file_must_have_nulls(
    mut size: idx_t,
    mut fd: libc::c_int,
    mut st: *const stat,
) -> bool {
    if 4 as libc::c_int != 0 as libc::c_int && !seek_failed
        && usable_st_size(st) as libc::c_int != 0 && size < (*st).st_size
    {
        let mut cur: off_t = size;
        if 0 as libc::c_int != 0 || fd == 0 as libc::c_int {
            cur = lseek(fd, 0 as libc::c_int as __off_t, 1 as libc::c_int);
            if cur < 0 as libc::c_int as libc::c_long {
                return 0 as libc::c_int != 0;
            }
        }
        let mut hole_start: off_t = lseek(fd, cur, 4 as libc::c_int);
        if 0 as libc::c_int as libc::c_long <= hole_start {
            if lseek(fd, cur, 0 as libc::c_int) < 0 as libc::c_int as libc::c_long {
                suppressible_error(*__errno_location());
            }
            if hole_start < (*st).st_size {
                return 1 as libc::c_int != 0;
            }
        }
    }
    return 0 as libc::c_int != 0;
}
unsafe extern "C" fn context_length_arg(
    mut str: *const libc::c_char,
    mut out: *mut intmax_t,
) {
    let mut current_block_1: u64;
    match xstrtoimax(
        str,
        0 as *mut *mut libc::c_char,
        10 as libc::c_int,
        out,
        b"\0" as *const u8 as *const libc::c_char,
    ) as libc::c_uint
    {
        0 | 1 => {
            if 0 as libc::c_int as libc::c_long <= *out {
                current_block_1 = 715039052867723359;
            } else {
                current_block_1 = 13399684147782614896;
            }
        }
        _ => {
            current_block_1 = 13399684147782614896;
        }
    }
    match current_block_1 {
        13399684147782614896 => {
            if ::std::mem::size_of::<C2RustUnnamed_14>() as libc::c_ulong != 0 {
                error(
                    EXIT_TROUBLE as libc::c_int,
                    0 as libc::c_int,
                    b"%s: %s\0" as *const u8 as *const libc::c_char,
                    str,
                    dcgettext(
                        0 as *const libc::c_char,
                        b"invalid context length argument\0" as *const u8
                            as *const libc::c_char,
                        5 as libc::c_int,
                    ),
                );
                if 0 as libc::c_int != 0 {} else {
                    unreachable!();
                };
            } else {
                error(
                    EXIT_TROUBLE as libc::c_int,
                    0 as libc::c_int,
                    b"%s: %s\0" as *const u8 as *const libc::c_char,
                    str,
                    dcgettext(
                        0 as *const libc::c_char,
                        b"invalid context length argument\0" as *const u8
                            as *const libc::c_char,
                        5 as libc::c_int,
                    ),
                );
                if 0 as libc::c_int != 0 {} else {
                    unreachable!();
                };
            };
        }
        _ => {}
    };
}
unsafe extern "C" fn exclude_options(mut command_line: bool) -> libc::c_int {
    return (1 as libc::c_int) << 28 as libc::c_int
        | (if command_line as libc::c_int != 0 {
            0 as libc::c_int
        } else {
            (1 as libc::c_int) << 30 as libc::c_int
        });
}
unsafe extern "C" fn skipped_file(
    mut name: *const libc::c_char,
    mut command_line: bool,
    mut is_dir: bool,
) -> bool {
    let mut pats: *mut *mut exclude = 0 as *mut *mut exclude;
    if !is_dir {
        pats = excluded_patterns.as_mut_ptr();
    } else if directories as libc::c_uint
        == SKIP_DIRECTORIES as libc::c_int as libc::c_uint
    {
        return 1 as libc::c_int != 0
    } else if command_line as libc::c_int != 0 && omit_dot_slash as libc::c_int != 0 {
        return 0 as libc::c_int != 0
    } else {
        pats = excluded_directory_patterns.as_mut_ptr();
    }
    return !(*pats.offset(command_line as isize)).is_null()
        && excluded_file_name(*pats.offset(command_line as isize), name) as libc::c_int
            != 0;
}
static mut buffer: *mut libc::c_char = 0 as *const libc::c_char as *mut libc::c_char;
static mut bufalloc: idx_t = 0;
static mut bufdesc: libc::c_int = 0;
static mut bufbeg: *mut libc::c_char = 0 as *const libc::c_char as *mut libc::c_char;
static mut buflim: *mut libc::c_char = 0 as *const libc::c_char as *mut libc::c_char;
static mut pagesize: idx_t = 0;
static mut bufoffset: off_t = 0;
static mut after_last_match: off_t = 0;
static mut skip_nuls: bool = false;
static mut skip_empty_lines: bool = false;
static mut totalnl: intmax_t = 0;
unsafe extern "C" fn add_count(mut a: intmax_t, mut b: idx_t) -> intmax_t {
    let mut sum: intmax_t = 0;
    let (fresh5, fresh6) = a.overflowing_add(b);
    *(&mut sum as *mut intmax_t) = fresh5;
    if fresh6 {
        if ::std::mem::size_of::<C2RustUnnamed_12>() as libc::c_ulong != 0 {
            error(
                EXIT_TROUBLE as libc::c_int,
                0 as libc::c_int,
                dcgettext(
                    0 as *const libc::c_char,
                    b"input is too large to count\0" as *const u8 as *const libc::c_char,
                    5 as libc::c_int,
                ),
            );
            if 0 as libc::c_int != 0 {} else {
                unreachable!();
            };
        } else {
            error(
                EXIT_TROUBLE as libc::c_int,
                0 as libc::c_int,
                dcgettext(
                    0 as *const libc::c_char,
                    b"input is too large to count\0" as *const u8 as *const libc::c_char,
                    5 as libc::c_int,
                ),
            );
            if 0 as libc::c_int != 0 {} else {
                unreachable!();
            };
        };
    }
    return sum;
}
unsafe extern "C" fn all_zeros(mut buf: *const libc::c_char, mut size: idx_t) -> bool {
    let mut p: *const libc::c_char = buf;
    while p < buf.offset(size as isize) {
        if *p != 0 {
            return 0 as libc::c_int != 0;
        }
        p = p.offset(1);
        p;
    }
    return 1 as libc::c_int != 0;
}
unsafe extern "C" fn reset(mut fd: libc::c_int, mut st: *const stat) -> bool {
    buflim = if (buffer.offset(1 as libc::c_int as isize) as uintptr_t)
        .wrapping_rem(pagesize as libc::c_ulong) == 0 as libc::c_int as libc::c_ulong
    {
        buffer.offset(1 as libc::c_int as isize)
    } else {
        buffer
            .offset(1 as libc::c_int as isize)
            .offset(
                (pagesize as libc::c_ulong)
                    .wrapping_sub(
                        (buffer.offset(1 as libc::c_int as isize) as uintptr_t)
                            .wrapping_rem(pagesize as libc::c_ulong),
                    ) as isize,
            )
    };
    bufbeg = buflim;
    *bufbeg.offset(-(1 as libc::c_int) as isize) = eolbyte;
    bufdesc = fd;
    bufoffset = if fd == 0 as libc::c_int {
        lseek(fd, 0 as libc::c_int as __off_t, 1 as libc::c_int)
    } else {
        0 as libc::c_int as libc::c_long
    };
    seek_failed = bufoffset < 0 as libc::c_int as libc::c_long;
    seek_data_failed = seek_failed;
    if seek_failed {
        if *__errno_location() != 29 as libc::c_int {
            suppressible_error(*__errno_location());
            return 0 as libc::c_int != 0;
        }
        bufoffset = 0 as libc::c_int as off_t;
    }
    return 1 as libc::c_int != 0;
}
unsafe extern "C" fn fillbuf(mut save: idx_t, mut st: *const stat) -> bool {
    let mut readbuf: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut min_after_buflim: idx_t = pagesize
        + uword_size as libc::c_int as libc::c_long;
    if min_after_buflim
        <= buffer.offset(bufalloc as isize).offset_from(buflim) as libc::c_long
    {
        readbuf = buflim;
    } else {
        let mut newbuf: *mut libc::c_char = 0 as *mut libc::c_char;
        let mut minsize: idx_t = save + pagesize;
        let mut incr_min: ptrdiff_t = minsize - bufalloc + min_after_buflim;
        if incr_min <= 0 as libc::c_int as libc::c_long {
            newbuf = buffer;
        } else {
            let mut alloc_max: ptrdiff_t = -(1 as libc::c_int) as ptrdiff_t;
            if usable_st_size(st) {
                let mut to_be_read: off_t = (*st).st_size - bufoffset;
                let mut a: ptrdiff_t = 0;
                if 0 as libc::c_int as libc::c_long <= to_be_read
                    && {
                        let (fresh7, fresh8) = to_be_read
                            .overflowing_add(save + min_after_buflim);
                        *(&mut a as *mut ptrdiff_t) = fresh7;
                        !fresh8
                    }
                {
                    alloc_max = if a > bufalloc + incr_min {
                        a
                    } else {
                        bufalloc + incr_min
                    };
                }
            }
            newbuf = xpalloc(
                0 as *mut libc::c_void,
                &mut bufalloc,
                incr_min,
                alloc_max,
                1 as libc::c_int as idx_t,
            ) as *mut libc::c_char;
        }
        readbuf = if (newbuf.offset(1 as libc::c_int as isize).offset(save as isize)
            as uintptr_t)
            .wrapping_rem(pagesize as libc::c_ulong) == 0 as libc::c_int as libc::c_ulong
        {
            newbuf.offset(1 as libc::c_int as isize).offset(save as isize)
        } else {
            newbuf
                .offset(1 as libc::c_int as isize)
                .offset(save as isize)
                .offset(
                    (pagesize as libc::c_ulong)
                        .wrapping_sub(
                            (newbuf
                                .offset(1 as libc::c_int as isize)
                                .offset(save as isize) as uintptr_t)
                                .wrapping_rem(pagesize as libc::c_ulong),
                        ) as isize,
                )
        };
        let mut moved: idx_t = save + 1 as libc::c_int as libc::c_long;
        memmove(
            readbuf.offset(-(moved as isize)) as *mut libc::c_void,
            buflim.offset(-(moved as isize)) as *const libc::c_void,
            moved as libc::c_ulong,
        );
        if (0 as libc::c_int as libc::c_long) < incr_min {
            rpl_free(buffer as *mut libc::c_void);
            buffer = newbuf;
        }
    }
    bufbeg = readbuf.offset(-(save as isize));
    clear_asan_poison();
    let mut readsize: idx_t = buffer
        .offset(bufalloc as isize)
        .offset(-(uword_size as libc::c_int as isize))
        .offset_from(readbuf) as libc::c_long;
    readsize -= readsize % pagesize;
    let mut fillsize: idx_t = 0;
    let mut cc: bool = 1 as libc::c_int != 0;
    loop {
        fillsize = safe_read(bufdesc, readbuf as *mut libc::c_void, readsize as size_t)
            as idx_t;
        if fillsize as libc::c_ulong == -(1 as libc::c_int) as size_t {
            fillsize = 0 as libc::c_int as idx_t;
            cc = 0 as libc::c_int != 0;
        }
        bufoffset += fillsize;
        if (fillsize == 0 as libc::c_int as libc::c_long) as libc::c_int
            | !skip_nuls as libc::c_int != 0 || !all_zeros(readbuf, fillsize)
        {
            break;
        }
        totalnl = add_count(totalnl, fillsize);
        if 3 as libc::c_int != 0 as libc::c_int && !seek_data_failed {
            let mut data_start: off_t = lseek(bufdesc, bufoffset, 3 as libc::c_int);
            if data_start < 0 as libc::c_int as libc::c_long
                && *__errno_location() == 6 as libc::c_int
                && usable_st_size(st) as libc::c_int != 0 && bufoffset < (*st).st_size
            {
                data_start = lseek(
                    bufdesc,
                    0 as libc::c_int as __off_t,
                    2 as libc::c_int,
                );
            }
            if data_start < 0 as libc::c_int as libc::c_long {
                seek_data_failed = 1 as libc::c_int != 0;
            } else {
                totalnl = add_count(totalnl, data_start - bufoffset);
                bufoffset = data_start;
            }
        }
    }
    buflim = readbuf.offset(fillsize as isize);
    memset(
        buflim as *mut libc::c_void,
        0 as libc::c_int,
        uword_size as libc::c_int as libc::c_ulong,
    );
    asan_poison(
        buflim.offset(uword_size as libc::c_int as isize) as *const libc::c_void,
        bufalloc - buflim.offset_from(buffer) as libc::c_long
            - uword_size as libc::c_int as libc::c_long,
    );
    return cc;
}
static mut binary_files: C2RustUnnamed_10 = BINARY_BINARY_FILES;
static mut list_files: C2RustUnnamed_9 = LISTFILES_NONE;
static mut out_file: libc::c_int = 0;
static mut filename_mask: libc::c_int = 0;
static mut out_quiet: bool = false;
static mut out_invert: bool = false;
static mut out_line: bool = false;
static mut out_byte: bool = false;
static mut out_before: intmax_t = 0;
static mut out_after: intmax_t = 0;
static mut count_matches: bool = false;
static mut max_count: intmax_t = 0;
static mut line_buffered: bool = false;
static mut label: *mut libc::c_char = 0 as *const libc::c_char as *mut libc::c_char;
static mut totalcc: intmax_t = 0;
static mut lastnl: *const libc::c_char = 0 as *const libc::c_char;
static mut lastout: *mut libc::c_char = 0 as *const libc::c_char as *mut libc::c_char;
static mut outleft: intmax_t = 0;
static mut pending: intmax_t = 0;
static mut done_on_match: bool = false;
static mut exit_on_match: bool = false;
static mut dev_null_output: bool = false;
static mut binary: bool = false;
unsafe extern "C" fn nlscan(mut lim: *const libc::c_char) {
    let mut newlines: idx_t = 0 as libc::c_int as idx_t;
    let mut beg: *const libc::c_char = lastnl;
    while beg < lim {
        beg = memchr(
            beg as *const libc::c_void,
            eolbyte as libc::c_int,
            lim.offset_from(beg) as libc::c_long as libc::c_ulong,
        ) as *const libc::c_char;
        if beg.is_null() {
            break;
        }
        newlines += 1;
        newlines;
        beg = beg.offset(1);
        beg;
    }
    totalnl = add_count(totalnl, newlines);
    lastnl = lim;
}
unsafe extern "C" fn print_filename() {
    pr_sgr_start_if(filename_color);
    fputs_errno(input_filename());
    pr_sgr_end_if(filename_color);
}
unsafe extern "C" fn print_sep(mut sep: libc::c_char) {
    pr_sgr_start_if(sep_color);
    putchar_errno(sep as libc::c_int);
    pr_sgr_end_if(sep_color);
}
unsafe extern "C" fn print_offset(mut pos: intmax_t, mut color: *const libc::c_char) {
    pr_sgr_start_if(color);
    printf_errno(b"%*ld\0" as *const u8 as *const libc::c_char, offset_width, pos);
    pr_sgr_end_if(color);
}
unsafe extern "C" fn print_line_head(
    mut beg: *mut libc::c_char,
    mut len: idx_t,
    mut lim: *const libc::c_char,
    mut sep: libc::c_char,
) -> bool {
    if binary_files as libc::c_uint != TEXT_BINARY_FILES as libc::c_int as libc::c_uint {
        let mut ch: libc::c_char = *beg.offset(len as isize);
        let mut encoding_errors: bool = buf_has_encoding_errors(beg, len);
        *beg.offset(len as isize) = ch;
        if encoding_errors {
            encoding_error_output = 1 as libc::c_int != 0;
            return 0 as libc::c_int != 0;
        }
    }
    if out_file != 0 {
        print_filename();
        if filename_mask != 0 {
            print_sep(sep);
        } else {
            putchar_errno(0 as libc::c_int);
        }
    }
    if out_line {
        if lastnl < lim {
            nlscan(beg);
            totalnl = add_count(totalnl, 1 as libc::c_int as idx_t);
            lastnl = lim;
        }
        print_offset(totalnl, line_num_color);
        print_sep(sep);
    }
    if out_byte {
        let mut pos: intmax_t = add_count(
            totalcc,
            beg.offset_from(bufbeg) as libc::c_long,
        );
        print_offset(pos, byte_num_color);
        print_sep(sep);
    }
    if align_tabs as libc::c_int != 0
        && out_file | out_line as libc::c_int | out_byte as libc::c_int != 0
        && len != 0 as libc::c_int as libc::c_long
    {
        putchar_errno('\t' as i32);
    }
    return 1 as libc::c_int != 0;
}
unsafe extern "C" fn print_line_middle(
    mut beg: *mut libc::c_char,
    mut lim: *mut libc::c_char,
    mut line_color: *const libc::c_char,
    mut match_color: *const libc::c_char,
) -> *mut libc::c_char {
    let mut match_size: idx_t = 0;
    let mut match_offset: ptrdiff_t = 0;
    let mut cur: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut mid: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut b: *mut libc::c_char = 0 as *mut libc::c_char;
    cur = beg;
    while cur < lim
        && {
            match_offset = execute
                .unwrap()(
                compiled_pattern,
                beg,
                lim.offset_from(beg) as libc::c_long,
                &mut match_size,
                cur,
            );
            0 as libc::c_int as libc::c_long <= match_offset
        }
    {
        b = beg.offset(match_offset as isize);
        if b == lim {
            break;
        }
        if match_size == 0 as libc::c_int as libc::c_long {
            match_size = 1 as libc::c_int as idx_t;
            if mid.is_null() {
                mid = cur;
            }
        } else {
            if only_matching {
                let mut sep: libc::c_char = (if out_invert as libc::c_int != 0 {
                    SEP_CHAR_REJECTED as libc::c_int
                } else {
                    SEP_CHAR_SELECTED as libc::c_int
                }) as libc::c_char;
                if !print_line_head(b, match_size, lim, sep) {
                    return 0 as *mut libc::c_char;
                }
            } else {
                pr_sgr_start(line_color);
                if !mid.is_null() {
                    cur = mid;
                    mid = 0 as *mut libc::c_char;
                }
                fwrite_errno(
                    cur as *const libc::c_void,
                    1 as libc::c_int as idx_t,
                    b.offset_from(cur) as libc::c_long,
                );
            }
            pr_sgr_start_if(match_color);
            fwrite_errno(
                b as *const libc::c_void,
                1 as libc::c_int as idx_t,
                match_size,
            );
            pr_sgr_end_if(match_color);
            if only_matching {
                putchar_errno(eolbyte as libc::c_int);
            }
        }
        cur = b.offset(match_size as isize);
    }
    if only_matching {
        cur = lim;
    } else if !mid.is_null() {
        cur = mid;
    }
    return cur;
}
unsafe extern "C" fn print_line_tail(
    mut beg: *mut libc::c_char,
    mut lim: *const libc::c_char,
    mut line_color: *const libc::c_char,
) -> *mut libc::c_char {
    let mut eol_size: idx_t = 0;
    let mut tail_size: idx_t = 0;
    eol_size = (lim > beg as *const libc::c_char
        && *lim.offset(-(1 as libc::c_int) as isize) as libc::c_int
            == eolbyte as libc::c_int) as libc::c_int as idx_t;
    eol_size
        += (lim.offset(-(eol_size as isize)) > beg as *const libc::c_char
            && *lim.offset(-(1 as libc::c_int as libc::c_long + eol_size) as isize)
                as libc::c_int == '\r' as i32) as libc::c_int as libc::c_long;
    tail_size = lim.offset(-(eol_size as isize)).offset_from(beg) as libc::c_long;
    if tail_size > 0 as libc::c_int as libc::c_long {
        pr_sgr_start(line_color);
        fwrite_errno(beg as *const libc::c_void, 1 as libc::c_int as idx_t, tail_size);
        beg = beg.offset(tail_size as isize);
        pr_sgr_end(line_color);
    }
    return beg;
}
unsafe extern "C" fn prline(
    mut beg: *mut libc::c_char,
    mut lim: *mut libc::c_char,
    mut sep: libc::c_char,
) {
    let mut matching: bool = false;
    let mut line_color: *const libc::c_char = 0 as *const libc::c_char;
    let mut match_color: *const libc::c_char = 0 as *const libc::c_char;
    if !only_matching {
        if !print_line_head(
            beg,
            lim.offset_from(beg) as libc::c_long - 1 as libc::c_int as libc::c_long,
            lim,
            sep,
        ) {
            return;
        }
    }
    matching = (sep as libc::c_int == SEP_CHAR_SELECTED as libc::c_int) as libc::c_int
        ^ out_invert as libc::c_int != 0;
    if color_option != 0 {
        line_color = if (sep as libc::c_int == SEP_CHAR_SELECTED as libc::c_int)
            as libc::c_int
            ^ (out_invert as libc::c_int != 0 && color_option < 0 as libc::c_int)
                as libc::c_int != 0
        {
            selected_line_color
        } else {
            context_line_color
        };
        match_color = if sep as libc::c_int == SEP_CHAR_SELECTED as libc::c_int {
            selected_match_color
        } else {
            context_match_color
        };
    } else {
        match_color = 0 as *const libc::c_char;
        line_color = match_color;
    }
    if only_matching as libc::c_int != 0 && matching as libc::c_int != 0
        || color_option != 0
            && (*line_color as libc::c_int != 0 || *match_color as libc::c_int != 0)
    {
        if matching as libc::c_int != 0
            && (only_matching as libc::c_int != 0 || *match_color as libc::c_int != 0)
        {
            beg = print_line_middle(beg, lim, line_color, match_color);
            if beg.is_null() {
                return;
            }
        }
        if !only_matching && *line_color as libc::c_int != 0 {
            beg = print_line_tail(beg, lim, line_color);
        }
    }
    if !only_matching && lim > beg {
        fwrite_errno(
            beg as *const libc::c_void,
            1 as libc::c_int as idx_t,
            lim.offset_from(beg) as libc::c_long,
        );
    }
    if line_buffered {
        fflush_errno();
    }
    if stdout_errno != 0 {
        if ::std::mem::size_of::<C2RustUnnamed_11>() as libc::c_ulong != 0 {
            error(
                EXIT_TROUBLE as libc::c_int,
                stdout_errno,
                dcgettext(
                    0 as *const libc::c_char,
                    b"write error\0" as *const u8 as *const libc::c_char,
                    5 as libc::c_int,
                ),
            );
            if 0 as libc::c_int != 0 {} else {
                unreachable!();
            };
        } else {
            error(
                EXIT_TROUBLE as libc::c_int,
                stdout_errno,
                dcgettext(
                    0 as *const libc::c_char,
                    b"write error\0" as *const u8 as *const libc::c_char,
                    5 as libc::c_int,
                ),
            );
            if 0 as libc::c_int != 0 {} else {
                unreachable!();
            };
        };
    }
    lastout = lim;
}
unsafe extern "C" fn prpending(mut lim: *const libc::c_char) {
    if lastout.is_null() {
        lastout = bufbeg;
    }
    while (0 as libc::c_int as libc::c_long) < pending
        && lastout < lim as *mut libc::c_char
    {
        let mut nl: *mut libc::c_char = rawmemchr(
            lastout as *const libc::c_void,
            eolbyte as libc::c_int,
        ) as *mut libc::c_char;
        prline(
            lastout,
            nl.offset(1 as libc::c_int as isize),
            SEP_CHAR_REJECTED as libc::c_int as libc::c_char,
        );
        pending -= 1;
        pending;
    }
}
unsafe extern "C" fn prtext(mut beg: *mut libc::c_char, mut lim: *mut libc::c_char) {
    static mut used: bool = false;
    let mut eol: libc::c_char = eolbyte;
    if !out_quiet && pending > 0 as libc::c_int as libc::c_long {
        prpending(beg);
    }
    let mut p: *mut libc::c_char = beg;
    if !out_quiet {
        let mut bp: *const libc::c_char = if !lastout.is_null() {
            lastout
        } else {
            bufbeg
        };
        let mut i: intmax_t = 0;
        i = 0 as libc::c_int as intmax_t;
        while i < out_before {
            if p > bp as *mut libc::c_char {
                loop {
                    p = p.offset(-1);
                    p;
                    if !(*p.offset(-(1 as libc::c_int) as isize) as libc::c_int
                        != eol as libc::c_int)
                    {
                        break;
                    }
                }
            }
            i += 1;
            i;
        }
        if (0 as libc::c_int as libc::c_long <= out_before
            || 0 as libc::c_int as libc::c_long <= out_after) && used as libc::c_int != 0
            && p != lastout && !group_separator.is_null()
        {
            pr_sgr_start_if(sep_color);
            fputs_errno(group_separator);
            pr_sgr_end_if(sep_color);
            putchar_errno('\n' as i32);
        }
        while p < beg {
            let mut nl: *mut libc::c_char = rawmemchr(
                p as *const libc::c_void,
                eol as libc::c_int,
            ) as *mut libc::c_char;
            nl = nl.offset(1);
            nl;
            prline(p, nl, SEP_CHAR_REJECTED as libc::c_int as libc::c_char);
            p = nl;
        }
    }
    let mut n: intmax_t = 0;
    if out_invert {
        n = 0 as libc::c_int as intmax_t;
        while p < lim && n < outleft {
            let mut nl_0: *mut libc::c_char = rawmemchr(
                p as *const libc::c_void,
                eol as libc::c_int,
            ) as *mut libc::c_char;
            nl_0 = nl_0.offset(1);
            nl_0;
            if !out_quiet {
                prline(p, nl_0, SEP_CHAR_SELECTED as libc::c_int as libc::c_char);
            }
            p = nl_0;
            n += 1;
            n;
        }
    } else {
        if !out_quiet {
            prline(beg, lim, SEP_CHAR_SELECTED as libc::c_int as libc::c_char);
        }
        n = 1 as libc::c_int as intmax_t;
        p = lim;
    }
    after_last_match = bufoffset - buflim.offset_from(p) as libc::c_long;
    pending = if out_quiet as libc::c_int != 0 {
        0 as libc::c_int as libc::c_long
    } else if 0 as libc::c_int as libc::c_long > out_after {
        0 as libc::c_int as libc::c_long
    } else {
        out_after
    };
    used = 1 as libc::c_int != 0;
    outleft -= n;
}
unsafe extern "C" fn zap_nuls(
    mut p: *mut libc::c_char,
    mut lim: *mut libc::c_char,
    mut eol: libc::c_char,
) {
    if eol != 0 {
        loop {
            *lim = '\0' as i32 as libc::c_char;
            p = p.offset(strlen(p) as isize);
            *lim = eol;
            if p == lim {
                break;
            }
            loop {
                let fresh9 = p;
                p = p.offset(1);
                *fresh9 = eol;
                if !(*p == 0) {
                    break;
                }
            }
        }
    }
}
unsafe extern "C" fn grepbuf(
    mut beg: *mut libc::c_char,
    mut lim: *const libc::c_char,
) -> intmax_t {
    let mut outleft0: intmax_t = outleft;
    let mut endp: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut p: *mut libc::c_char = beg;
    while p < lim as *mut libc::c_char {
        let mut match_size: idx_t = 0;
        let mut match_offset: ptrdiff_t = execute
            .unwrap()(
            compiled_pattern,
            p,
            lim.offset_from(p) as libc::c_long,
            &mut match_size,
            0 as *const libc::c_char,
        );
        if match_offset < 0 as libc::c_int as libc::c_long {
            if !out_invert {
                break;
            }
            match_offset = lim.offset_from(p) as libc::c_long;
            match_size = 0 as libc::c_int as idx_t;
        }
        let mut b: *mut libc::c_char = p.offset(match_offset as isize);
        endp = b.offset(match_size as isize);
        if !out_invert && b == lim as *mut libc::c_char {
            break;
        }
        if !out_invert || p < b {
            let mut prbeg: *mut libc::c_char = if out_invert as libc::c_int != 0 {
                p
            } else {
                b
            };
            let mut prend: *mut libc::c_char = if out_invert as libc::c_int != 0 {
                b
            } else {
                endp
            };
            prtext(prbeg, prend);
            if outleft == 0 || done_on_match as libc::c_int != 0 {
                if exit_on_match {
                    exit(
                        if errseen as libc::c_int != 0 {
                            exit_failure
                        } else {
                            0 as libc::c_int
                        },
                    );
                }
                break;
            }
        }
        p = endp;
    }
    return outleft0 - outleft;
}
unsafe extern "C" fn grep(
    mut fd: libc::c_int,
    mut st: *const stat,
    mut ineof: *mut bool,
) -> intmax_t {
    let mut current_block: u64;
    let mut nlines: intmax_t = 0;
    let mut i: intmax_t = 0;
    let mut residue: idx_t = 0;
    let mut save: idx_t = 0;
    let mut oldc: libc::c_char = 0;
    let mut beg: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut lim: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut eol: libc::c_char = eolbyte;
    let mut nul_zapper: libc::c_char = '\0' as i32 as libc::c_char;
    let mut done_on_match_0: bool = done_on_match;
    let mut out_quiet_0: bool = out_quiet;
    let mut nlines_first_null: intmax_t = -(1 as libc::c_int) as intmax_t;
    if !reset(fd, st) {
        return 0 as libc::c_int as intmax_t;
    }
    totalcc = 0 as libc::c_int as intmax_t;
    lastout = 0 as *mut libc::c_char;
    totalnl = 0 as libc::c_int as intmax_t;
    outleft = max_count;
    after_last_match = 0 as libc::c_int as off_t;
    pending = 0 as libc::c_int as intmax_t;
    skip_nuls = skip_empty_lines as libc::c_int != 0 && eol == 0;
    encoding_error_output = 0 as libc::c_int != 0;
    nlines = 0 as libc::c_int as intmax_t;
    residue = 0 as libc::c_int as idx_t;
    save = 0 as libc::c_int as idx_t;
    if !fillbuf(save, st) {
        suppressible_error(*__errno_location());
        return 0 as libc::c_int as intmax_t;
    }
    offset_width = 0 as libc::c_int;
    if align_tabs {
        let mut num: intmax_t = if usable_st_size(st) as libc::c_int != 0 {
            (*st).st_size
        } else {
            9223372036854775807 as libc::c_long
        };
        num
            += (out_line as libc::c_int != 0
                && num < 9223372036854775807 as libc::c_long) as libc::c_int
                as libc::c_long;
        loop {
            offset_width += 1;
            offset_width;
            num /= 10 as libc::c_int as libc::c_long;
            if !(num != 0 as libc::c_int as libc::c_long) {
                break;
            }
        }
    }
    let mut firsttime: bool = 1 as libc::c_int != 0;
    loop {
        if nlines_first_null < 0 as libc::c_int as libc::c_long
            && eol as libc::c_int != 0
            && binary_files as libc::c_uint
                != TEXT_BINARY_FILES as libc::c_int as libc::c_uint
            && (buf_has_nulls(bufbeg, buflim.offset_from(bufbeg) as libc::c_long)
                as libc::c_int != 0
                || firsttime as libc::c_int != 0
                    && file_must_have_nulls(
                        buflim.offset_from(bufbeg) as libc::c_long,
                        fd,
                        st,
                    ) as libc::c_int != 0)
        {
            if binary_files as libc::c_uint
                == WITHOUT_MATCH_BINARY_FILES as libc::c_int as libc::c_uint
            {
                return 0 as libc::c_int as intmax_t;
            }
            if !count_matches {
                out_quiet = 1 as libc::c_int != 0;
                done_on_match = out_quiet;
            }
            nlines_first_null = nlines;
            nul_zapper = eol;
            skip_nuls = skip_empty_lines;
        }
        lastnl = bufbeg;
        if !lastout.is_null() {
            lastout = bufbeg;
        }
        beg = bufbeg.offset(save as isize);
        if beg == buflim {
            *ineof = 1 as libc::c_int != 0;
            current_block = 6560072651652764009;
            break;
        } else {
            zap_nuls(beg, buflim, nul_zapper);
            oldc = *beg.offset(-(1 as libc::c_int) as isize);
            *beg.offset(-(1 as libc::c_int) as isize) = eol;
            lim = memrchr(
                beg.offset(-(1 as libc::c_int as isize)) as *const libc::c_void,
                eol as libc::c_int,
                (buflim.offset_from(beg) as libc::c_long
                    + 1 as libc::c_int as libc::c_long) as size_t,
            ) as *mut libc::c_char;
            lim = lim.offset(1);
            lim;
            *beg.offset(-(1 as libc::c_int) as isize) = oldc;
            if lim == beg {
                lim = beg.offset(-(residue as isize));
            }
            beg = beg.offset(-(residue as isize));
            residue = buflim.offset_from(lim) as libc::c_long;
            if beg < lim {
                if outleft != 0 {
                    nlines += grepbuf(beg, lim);
                }
                if pending != 0 {
                    prpending(lim);
                }
                if outleft == 0 && pending == 0
                    || done_on_match as libc::c_int != 0
                        && (if 0 as libc::c_int as libc::c_long > nlines_first_null {
                            0 as libc::c_int as libc::c_long
                        } else {
                            nlines_first_null
                        }) < nlines
                {
                    current_block = 8573782648970308714;
                    break;
                }
            }
            i = 0 as libc::c_int as intmax_t;
            beg = lim;
            while i < out_before && beg > bufbeg && beg != lastout {
                i += 1;
                i;
                loop {
                    beg = beg.offset(-1);
                    beg;
                    if !(*beg.offset(-(1 as libc::c_int) as isize) as libc::c_int
                        != eol as libc::c_int)
                    {
                        break;
                    }
                }
            }
            if beg != lastout {
                lastout = 0 as *mut libc::c_char;
            }
            save = lim.offset(residue as isize).offset_from(beg) as libc::c_long;
            if out_byte {
                totalcc = add_count(
                    totalcc,
                    buflim.offset_from(bufbeg) as libc::c_long - save,
                );
            }
            if out_line {
                nlscan(beg);
            }
            if !fillbuf(save, st) {
                suppressible_error(*__errno_location());
                current_block = 8573782648970308714;
                break;
            } else {
                firsttime = 0 as libc::c_int != 0;
            }
        }
    }
    match current_block {
        6560072651652764009 => {
            if residue != 0 {
                let fresh10 = buflim;
                buflim = buflim.offset(1);
                *fresh10 = eol;
                if outleft != 0 {
                    nlines
                        += grepbuf(
                            bufbeg.offset(save as isize).offset(-(residue as isize)),
                            buflim,
                        );
                }
                if pending != 0 {
                    prpending(buflim);
                }
            }
        }
        _ => {}
    }
    done_on_match = done_on_match_0;
    out_quiet = out_quiet_0;
    if binary_files as libc::c_uint == BINARY_BINARY_FILES as libc::c_int as libc::c_uint
        && !out_quiet
        && (encoding_error_output as libc::c_int != 0
            || 0 as libc::c_int as libc::c_long <= nlines_first_null
                && nlines_first_null < nlines)
    {
        error(
            0 as libc::c_int,
            0 as libc::c_int,
            dcgettext(
                0 as *const libc::c_char,
                b"%s: binary file matches\0" as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ),
            input_filename(),
        );
    }
    return nlines;
}
unsafe extern "C" fn grepdirent(
    mut fts: *mut FTS,
    mut ent: *mut FTSENT,
    mut command_line: bool,
) -> bool {
    let mut follow: bool = false;
    command_line = (command_line as libc::c_int
        & ((*ent).fts_level == 0 as libc::c_int as libc::c_long) as libc::c_int) != 0;
    if (*ent).fts_info as libc::c_int == 6 as libc::c_int {
        return 1 as libc::c_int != 0;
    }
    if !command_line
        && skipped_file(
            ((*ent).fts_name).as_mut_ptr(),
            0 as libc::c_int != 0,
            (*ent).fts_info as libc::c_int == 1 as libc::c_int
                || (*ent).fts_info as libc::c_int == 2 as libc::c_int
                || (*ent).fts_info as libc::c_int == 4 as libc::c_int,
        ) as libc::c_int != 0
    {
        rpl_fts_set(fts, ent, 4 as libc::c_int);
        return 1 as libc::c_int != 0;
    }
    filename = (*ent).fts_path;
    if omit_dot_slash as libc::c_int != 0
        && *filename.offset(1 as libc::c_int as isize) as libc::c_int != 0
    {
        filename = filename.offset(2 as libc::c_int as isize);
    }
    follow = (*fts).fts_options & 0x2 as libc::c_int != 0
        || (*fts).fts_options & 0x1 as libc::c_int != 0
            && command_line as libc::c_int != 0;
    match (*ent).fts_info as libc::c_int {
        1 => {
            if directories as libc::c_uint
                == RECURSE_DIRECTORIES as libc::c_int as libc::c_uint
            {
                return 1 as libc::c_int != 0;
            }
            rpl_fts_set(fts, ent, 4 as libc::c_int);
        }
        2 => {
            if !suppress_errors {
                error(
                    0 as libc::c_int,
                    0 as libc::c_int,
                    dcgettext(
                        0 as *const libc::c_char,
                        b"%s: warning: recursive directory loop\0" as *const u8
                            as *const libc::c_char,
                        5 as libc::c_int,
                    ),
                    filename,
                );
            }
            return 1 as libc::c_int != 0;
        }
        4 | 7 | 10 => {
            suppressible_error((*ent).fts_errno);
            return 1 as libc::c_int != 0;
        }
        3 | 11 => {
            if skip_devices(command_line) {
                let mut st: *mut stat = ((*ent).fts_statp).as_mut_ptr();
                let mut st1: stat = stat {
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
                if (*st).st_mode == 0 {
                    let mut flag: libc::c_int = if follow as libc::c_int != 0 {
                        0 as libc::c_int
                    } else {
                        0x100 as libc::c_int
                    };
                    if fstatat((*fts).fts_cwd_fd, (*ent).fts_accpath, &mut st1, flag)
                        != 0 as libc::c_int
                    {
                        suppressible_error(*__errno_location());
                        return 1 as libc::c_int != 0;
                    }
                    st = &mut st1;
                }
                if is_device_mode((*st).st_mode) {
                    return 1 as libc::c_int != 0;
                }
            }
        }
        8 | 13 => {}
        12 | 14 => return 1 as libc::c_int != 0,
        _ => {
            abort();
        }
    }
    return grepfile((*fts).fts_cwd_fd, (*ent).fts_accpath, follow, command_line);
}
unsafe extern "C" fn open_symlink_nofollow_error(mut err: libc::c_int) -> bool {
    if err == 40 as libc::c_int || err == 31 as libc::c_int {
        return 1 as libc::c_int != 0;
    }
    return 0 as libc::c_int != 0;
}
unsafe extern "C" fn grepfile(
    mut dirdesc: libc::c_int,
    mut name: *const libc::c_char,
    mut follow: bool,
    mut command_line: bool,
) -> bool {
    let mut oflag: libc::c_int = 0 as libc::c_int | 0o400 as libc::c_int
        | (if binary as libc::c_int != 0 { 0 as libc::c_int } else { 0 as libc::c_int })
        | (if follow as libc::c_int != 0 {
            0 as libc::c_int
        } else {
            0o400000 as libc::c_int
        })
        | (if skip_devices(command_line) as libc::c_int != 0 {
            0o4000 as libc::c_int
        } else {
            0 as libc::c_int
        });
    let mut desc: libc::c_int = openat_safer(dirdesc, name, oflag);
    if desc < 0 as libc::c_int {
        if follow as libc::c_int != 0
            || !open_symlink_nofollow_error(*__errno_location())
        {
            suppressible_error(*__errno_location());
        }
        return 1 as libc::c_int != 0;
    }
    return grepdesc(desc, command_line);
}
unsafe extern "C" fn drain_input(mut fd: libc::c_int, mut st: *const stat) -> bool {
    let mut nbytes: ssize_t = 0;
    if (*st).st_mode & 0o170000 as libc::c_int as libc::c_uint
        == 0o10000 as libc::c_int as libc::c_uint && dev_null_output as libc::c_int != 0
    {
        nbytes = splice(
            fd,
            0 as *mut __off64_t,
            1 as libc::c_int,
            0 as *mut __off64_t,
            INITIAL_BUFSIZE as libc::c_int as size_t,
            1 as libc::c_int as libc::c_uint,
        );
        if 0 as libc::c_int as libc::c_long <= nbytes
            || *__errno_location() != 22 as libc::c_int
        {
            while (0 as libc::c_int as libc::c_long) < nbytes {
                nbytes = splice(
                    fd,
                    0 as *mut __off64_t,
                    1 as libc::c_int,
                    0 as *mut __off64_t,
                    INITIAL_BUFSIZE as libc::c_int as size_t,
                    1 as libc::c_int as libc::c_uint,
                );
            }
            return nbytes == 0 as libc::c_int as libc::c_long;
        }
    }
    loop {
        nbytes = safe_read(fd, buffer as *mut libc::c_void, bufalloc as size_t)
            as ssize_t;
        if !(nbytes != 0) {
            break;
        }
        if nbytes as libc::c_ulong == -(1 as libc::c_int) as size_t {
            return 0 as libc::c_int != 0;
        }
    }
    return 1 as libc::c_int != 0;
}
unsafe extern "C" fn finalize_input(
    mut fd: libc::c_int,
    mut st: *const stat,
    mut ineof: bool,
) {
    if fd == 0 as libc::c_int
        && (if outleft != 0 {
            (!ineof
                && (seek_failed as libc::c_int != 0
                    || lseek(fd, 0 as libc::c_int as __off_t, 2 as libc::c_int)
                        < 0 as libc::c_int as libc::c_long
                        && *__errno_location() != 22 as libc::c_int)
                && !drain_input(fd, st)) as libc::c_int
        } else {
            (bufoffset != after_last_match && !seek_failed
                && lseek(fd, after_last_match, 0 as libc::c_int)
                    < 0 as libc::c_int as libc::c_long) as libc::c_int
        }) != 0
    {
        suppressible_error(*__errno_location());
    }
}
unsafe extern "C" fn grepdesc(mut desc: libc::c_int, mut command_line: bool) -> bool {
    let mut count: intmax_t = 0;
    let mut status: bool = 1 as libc::c_int != 0;
    let mut ineof: bool = 0 as libc::c_int != 0;
    let mut st: stat = stat {
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
    if fstat(desc, &mut st) != 0 as libc::c_int {
        suppressible_error(*__errno_location());
    } else if !(desc != 0 as libc::c_int
        && skip_devices(command_line) as libc::c_int != 0
        && is_device_mode(st.st_mode) as libc::c_int != 0)
    {
        if !(desc != 0 as libc::c_int && command_line as libc::c_int != 0
            && skipped_file(
                filename,
                1 as libc::c_int != 0,
                (st.st_mode & 0o170000 as libc::c_int as libc::c_uint
                    == 0o40000 as libc::c_int as libc::c_uint) as libc::c_int
                    != 0 as libc::c_int,
            ) as libc::c_int != 0)
        {
            if out_file < 0 as libc::c_int {
                out_file = (st.st_mode & 0o170000 as libc::c_int as libc::c_uint
                    == 0o40000 as libc::c_int as libc::c_uint) as libc::c_int;
            }
            if desc != 0 as libc::c_int
                && directories as libc::c_uint
                    == RECURSE_DIRECTORIES as libc::c_int as libc::c_uint
                && st.st_mode & 0o170000 as libc::c_int as libc::c_uint
                    == 0o40000 as libc::c_int as libc::c_uint
            {
                let mut fts: *mut FTS = 0 as *mut FTS;
                let mut ent: *mut FTSENT = 0 as *mut FTSENT;
                let mut opts: libc::c_int = fts_options
                    & !(if command_line as libc::c_int != 0 {
                        0 as libc::c_int
                    } else {
                        0x1 as libc::c_int
                    });
                let mut fts_arg: [*mut libc::c_char; 2] = [0 as *mut libc::c_char; 2];
                if close(desc) != 0 as libc::c_int {
                    suppressible_error(*__errno_location());
                }
                fts_arg[0 as libc::c_int as usize] = filename as *mut libc::c_char;
                fts_arg[1 as libc::c_int as usize] = 0 as *mut libc::c_char;
                fts = rpl_fts_open(fts_arg.as_mut_ptr(), opts, None);
                if fts.is_null() {
                    xalloc_die();
                }
                loop {
                    ent = rpl_fts_read(fts);
                    if ent.is_null() {
                        break;
                    }
                    status = (status as libc::c_int
                        & grepdirent(fts, ent, command_line) as libc::c_int) != 0;
                }
                if *__errno_location() != 0 {
                    suppressible_error(*__errno_location());
                }
                if rpl_fts_close(fts) != 0 as libc::c_int {
                    suppressible_error(*__errno_location());
                }
                return status;
            }
            if !(desc != 0 as libc::c_int
                && (directories as libc::c_uint
                    == SKIP_DIRECTORIES as libc::c_int as libc::c_uint
                    && st.st_mode & 0o170000 as libc::c_int as libc::c_uint
                        == 0o40000 as libc::c_int as libc::c_uint
                    || (devices as libc::c_uint
                        == SKIP_DEVICES as libc::c_int as libc::c_uint
                        || devices as libc::c_uint
                            == READ_COMMAND_LINE_DEVICES as libc::c_int as libc::c_uint
                            && !command_line)
                        && is_device_mode(st.st_mode) as libc::c_int != 0))
            {
                if !out_quiet
                    && list_files as libc::c_uint
                        == LISTFILES_NONE as libc::c_int as libc::c_uint
                    && (1 as libc::c_int as libc::c_long) < max_count
                    && st.st_mode & 0o170000 as libc::c_int as libc::c_uint
                        == 0o100000 as libc::c_int as libc::c_uint
                    && (st.st_ino == out_stat.st_ino && st.st_dev == out_stat.st_dev)
                {
                    if !suppress_errors {
                        error(
                            0 as libc::c_int,
                            0 as libc::c_int,
                            dcgettext(
                                0 as *const libc::c_char,
                                b"%s: input file is also the output\0" as *const u8
                                    as *const libc::c_char,
                                5 as libc::c_int,
                            ),
                            input_filename(),
                        );
                    }
                    errseen = 1 as libc::c_int != 0;
                } else {
                    count = grep(desc, &mut st, &mut ineof);
                    if count_matches {
                        if out_file != 0 {
                            print_filename();
                            if filename_mask != 0 {
                                print_sep(SEP_CHAR_SELECTED as libc::c_int as libc::c_char);
                            } else {
                                putchar_errno(0 as libc::c_int);
                            }
                        }
                        printf_errno(
                            b"%ld\n\0" as *const u8 as *const libc::c_char,
                            count,
                        );
                        if line_buffered {
                            fflush_errno();
                        }
                    }
                    status = count == 0;
                    if list_files as libc::c_uint
                        == LISTFILES_NONE as libc::c_int as libc::c_uint
                    {
                        finalize_input(desc, &mut st, ineof);
                    } else if list_files as libc::c_uint
                        == (if status as libc::c_int != 0 {
                            LISTFILES_NONMATCHING as libc::c_int
                        } else {
                            LISTFILES_MATCHING as libc::c_int
                        }) as libc::c_uint
                    {
                        print_filename();
                        putchar_errno('\n' as i32 & filename_mask);
                        if line_buffered {
                            fflush_errno();
                        }
                    }
                }
            }
        }
    }
    if desc != 0 as libc::c_int && close(desc) != 0 as libc::c_int {
        suppressible_error(*__errno_location());
    }
    return status;
}
unsafe extern "C" fn grep_command_line_arg(mut arg: *const libc::c_char) -> bool {
    if strcmp(arg, b"-\0" as *const u8 as *const libc::c_char) == 0 as libc::c_int {
        filename = label;
        if binary {
            xset_binary_mode(0 as libc::c_int, 0 as libc::c_int);
        }
        return grepdesc(0 as libc::c_int, 1 as libc::c_int != 0);
    } else {
        filename = arg;
        return grepfile(
            -(100 as libc::c_int),
            arg,
            1 as libc::c_int != 0,
            1 as libc::c_int != 0,
        );
    };
}
pub unsafe extern "C" fn usage(mut status: libc::c_int) {
    if status != 0 as libc::c_int {
        fprintf(
            stderr,
            dcgettext(
                0 as *const libc::c_char,
                b"Usage: %s [OPTION]... PATTERNS [FILE]...\n\0" as *const u8
                    as *const libc::c_char,
                5 as libc::c_int,
            ),
            getprogname(),
        );
        fprintf(
            stderr,
            dcgettext(
                0 as *const libc::c_char,
                b"Try '%s --help' for more information.\n\0" as *const u8
                    as *const libc::c_char,
                5 as libc::c_int,
            ),
            getprogname(),
        );
    } else {
        printf(
            dcgettext(
                0 as *const libc::c_char,
                b"Usage: %s [OPTION]... PATTERNS [FILE]...\n\0" as *const u8
                    as *const libc::c_char,
                5 as libc::c_int,
            ),
            getprogname(),
        );
        printf(
            dcgettext(
                0 as *const libc::c_char,
                b"Search for PATTERNS in each FILE.\n\0" as *const u8
                    as *const libc::c_char,
                5 as libc::c_int,
            ),
        );
        printf(
            dcgettext(
                0 as *const libc::c_char,
                b"Example: %s -i 'hello world' menu.h main.c\nPATTERNS can contain multiple patterns separated by newlines.\n\nPattern selection and interpretation:\n\0"
                    as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ),
            getprogname(),
        );
        printf(
            dcgettext(
                0 as *const libc::c_char,
                b"  -E, --extended-regexp     PATTERNS are extended regular expressions\n  -F, --fixed-strings       PATTERNS are strings\n  -G, --basic-regexp        PATTERNS are basic regular expressions\n  -P, --perl-regexp         PATTERNS are Perl regular expressions\n\0"
                    as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ),
        );
        printf(
            dcgettext(
                0 as *const libc::c_char,
                b"  -e, --regexp=PATTERNS     use PATTERNS for matching\n  -f, --file=FILE           take PATTERNS from FILE\n  -i, --ignore-case         ignore case distinctions in patterns and data\n      --no-ignore-case      do not ignore case distinctions (default)\n  -w, --word-regexp         match only whole words\n  -x, --line-regexp         match only whole lines\n  -z, --null-data           a data line ends in 0 byte, not newline\n\0"
                    as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ),
        );
        printf(
            dcgettext(
                0 as *const libc::c_char,
                b"\nMiscellaneous:\n  -s, --no-messages         suppress error messages\n  -v, --invert-match        select non-matching lines\n  -V, --version             display version information and exit\n      --help                display this help text and exit\n\0"
                    as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ),
        );
        printf(
            dcgettext(
                0 as *const libc::c_char,
                b"\nOutput control:\n  -m, --max-count=NUM       stop after NUM selected lines\n  -b, --byte-offset         print the byte offset with output lines\n  -n, --line-number         print line number with output lines\n      --line-buffered       flush output on every line\n  -H, --with-filename       print file name with output lines\n  -h, --no-filename         suppress the file name prefix on output\n      --label=LABEL         use LABEL as the standard input file name prefix\n\0"
                    as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ),
        );
        printf(
            dcgettext(
                0 as *const libc::c_char,
                b"  -o, --only-matching       show only nonempty parts of lines that match\n  -q, --quiet, --silent     suppress all normal output\n      --binary-files=TYPE   assume that binary files are TYPE;\n                            TYPE is 'binary', 'text', or 'without-match'\n  -a, --text                equivalent to --binary-files=text\n\0"
                    as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ),
        );
        printf(
            dcgettext(
                0 as *const libc::c_char,
                b"  -I                        equivalent to --binary-files=without-match\n  -d, --directories=ACTION  how to handle directories;\n                            ACTION is 'read', 'recurse', or 'skip'\n  -D, --devices=ACTION      how to handle devices, FIFOs and sockets;\n                            ACTION is 'read' or 'skip'\n  -r, --recursive           like --directories=recurse\n  -R, --dereference-recursive  likewise, but follow all symlinks\n\0"
                    as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ),
        );
        printf(
            dcgettext(
                0 as *const libc::c_char,
                b"      --include=GLOB        search only files that match GLOB (a file pattern)\n      --exclude=GLOB        skip files that match GLOB\n      --exclude-from=FILE   skip files that match any file pattern from FILE\n      --exclude-dir=GLOB    skip directories that match GLOB\n\0"
                    as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ),
        );
        printf(
            dcgettext(
                0 as *const libc::c_char,
                b"  -L, --files-without-match  print only names of FILEs with no selected lines\n  -l, --files-with-matches  print only names of FILEs with selected lines\n  -c, --count               print only a count of selected lines per FILE\n  -T, --initial-tab         make tabs line up (if needed)\n  -Z, --null                print 0 byte after FILE name\n\0"
                    as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ),
        );
        printf(
            dcgettext(
                0 as *const libc::c_char,
                b"\nContext control:\n  -B, --before-context=NUM  print NUM lines of leading context\n  -A, --after-context=NUM   print NUM lines of trailing context\n  -C, --context=NUM         print NUM lines of output context\n\0"
                    as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ),
        );
        printf(
            dcgettext(
                0 as *const libc::c_char,
                b"  -NUM                      same as --context=NUM\n      --group-separator=SEP  print SEP on line between matches with context\n      --no-group-separator  do not print separator for matches with context\n      --color[=WHEN],\n      --colour[=WHEN]       use markers to highlight the matching strings;\n                            WHEN is 'always', 'never', or 'auto'\n  -U, --binary              do not strip CR characters at EOL (MSDOS/Windows)\n\n\0"
                    as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ),
        );
        printf(
            dcgettext(
                0 as *const libc::c_char,
                b"When FILE is '-', read standard input.  With no FILE, read '.' if\nrecursive, '-' otherwise.  With fewer than two FILEs, assume -h.\nExit status is 0 if any line is selected, 1 otherwise;\nif any error occurs and -q is not given, the exit status is 2.\n\0"
                    as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ),
        );
        emit_bug_reporting_address();
    }
    exit(status);
}
static mut matchers: [C2RustUnnamed_16; 6] = unsafe {
    [
        {
            let mut init = C2RustUnnamed_16 {
                name: *::std::mem::transmute::<
                    &[u8; 12],
                    &mut [libc::c_char; 12],
                >(b"grep\0\0\0\0\0\0\0\0"),
                syntax: ((((1 as libc::c_int as libc::c_ulong) << 1 as libc::c_int)
                    << 1 as libc::c_int
                    | ((((((1 as libc::c_int as libc::c_ulong) << 1 as libc::c_int)
                        << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
                        << 1 as libc::c_int) << 1 as libc::c_int
                    | (((((((1 as libc::c_int as libc::c_ulong) << 1 as libc::c_int)
                        << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
                        << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int
                    | (((((((((1 as libc::c_int as libc::c_ulong) << 1 as libc::c_int)
                        << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
                        << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
                        << 1 as libc::c_int) << 1 as libc::c_int
                    | ((((((((((((((((1 as libc::c_int as libc::c_ulong)
                        << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
                        << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
                        << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
                        << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
                        << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
                        << 1 as libc::c_int
                    | (1 as libc::c_int as libc::c_ulong) << 1 as libc::c_int
                    | ((((((((((((((((((((((((1 as libc::c_int as libc::c_ulong)
                        << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
                        << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
                        << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
                        << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
                        << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
                        << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
                        << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
                        << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int
                    | (((((((((((1 as libc::c_int as libc::c_ulong) << 1 as libc::c_int)
                        << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
                        << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
                        << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
                        << 1 as libc::c_int)
                    & !(((((((((((((((((((((((((1 as libc::c_int as libc::c_ulong)
                        << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
                        << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
                        << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
                        << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
                        << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
                        << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
                        << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
                        << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int
                        | (((((((1 as libc::c_int as libc::c_ulong) << 1 as libc::c_int)
                            << 1 as libc::c_int) << 1 as libc::c_int)
                            << 1 as libc::c_int) << 1 as libc::c_int)
                            << 1 as libc::c_int) << 1 as libc::c_int)) as libc::c_int,
                compile: Some(
                    GEAcompile
                        as unsafe extern "C" fn(
                            *mut libc::c_char,
                            idx_t,
                            reg_syntax_t,
                            bool,
                        ) -> *mut libc::c_void,
                ),
                execute: Some(
                    EGexecute
                        as unsafe extern "C" fn(
                            *mut libc::c_void,
                            *const libc::c_char,
                            idx_t,
                            *mut idx_t,
                            *const libc::c_char,
                        ) -> ptrdiff_t,
                ),
            };
            init
        },
        {
            let mut init = C2RustUnnamed_16 {
                name: *::std::mem::transmute::<
                    &[u8; 12],
                    &mut [libc::c_char; 12],
                >(b"egrep\0\0\0\0\0\0\0"),
                syntax: ((((1 as libc::c_int as libc::c_ulong) << 1 as libc::c_int)
                    << 1 as libc::c_int
                    | ((((((1 as libc::c_int as libc::c_ulong) << 1 as libc::c_int)
                        << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
                        << 1 as libc::c_int) << 1 as libc::c_int
                    | (((((((1 as libc::c_int as libc::c_ulong) << 1 as libc::c_int)
                        << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
                        << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int
                    | (((((((((1 as libc::c_int as libc::c_ulong) << 1 as libc::c_int)
                        << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
                        << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
                        << 1 as libc::c_int) << 1 as libc::c_int
                    | ((((((((((((((((1 as libc::c_int as libc::c_ulong)
                        << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
                        << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
                        << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
                        << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
                        << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
                        << 1 as libc::c_int
                    | (((1 as libc::c_int as libc::c_ulong) << 1 as libc::c_int)
                        << 1 as libc::c_int) << 1 as libc::c_int
                    | ((((1 as libc::c_int as libc::c_ulong) << 1 as libc::c_int)
                        << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int
                    | ((((((((((((1 as libc::c_int as libc::c_ulong) << 1 as libc::c_int)
                        << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
                        << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
                        << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
                        << 1 as libc::c_int) << 1 as libc::c_int
                    | (((((((((((((1 as libc::c_int as libc::c_ulong)
                        << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
                        << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
                        << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
                        << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
                        << 1 as libc::c_int
                    | (((((((((((((((1 as libc::c_int as libc::c_ulong)
                        << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
                        << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
                        << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
                        << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
                        << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int
                    | (((((1 as libc::c_int as libc::c_ulong) << 1 as libc::c_int)
                        << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
                        << 1 as libc::c_int
                    | (((((((((((((((((1 as libc::c_int as libc::c_ulong)
                        << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
                        << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
                        << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
                        << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
                        << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
                        << 1 as libc::c_int) << 1 as libc::c_int
                    | (((((((((((((((((((((1 as libc::c_int as libc::c_ulong)
                        << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
                        << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
                        << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
                        << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
                        << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
                        << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
                        << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int
                    | (((((((((((1 as libc::c_int as libc::c_ulong) << 1 as libc::c_int)
                        << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
                        << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
                        << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
                        << 1 as libc::c_int)
                    & !((((((1 as libc::c_int as libc::c_ulong) << 1 as libc::c_int)
                        << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
                        << 1 as libc::c_int
                        | (((((((1 as libc::c_int as libc::c_ulong) << 1 as libc::c_int)
                            << 1 as libc::c_int) << 1 as libc::c_int)
                            << 1 as libc::c_int) << 1 as libc::c_int)
                            << 1 as libc::c_int) << 1 as libc::c_int)) as libc::c_int,
                compile: Some(
                    GEAcompile
                        as unsafe extern "C" fn(
                            *mut libc::c_char,
                            idx_t,
                            reg_syntax_t,
                            bool,
                        ) -> *mut libc::c_void,
                ),
                execute: Some(
                    EGexecute
                        as unsafe extern "C" fn(
                            *mut libc::c_void,
                            *const libc::c_char,
                            idx_t,
                            *mut idx_t,
                            *const libc::c_char,
                        ) -> ptrdiff_t,
                ),
            };
            init
        },
        {
            let mut init = C2RustUnnamed_16 {
                name: *::std::mem::transmute::<
                    &[u8; 12],
                    &mut [libc::c_char; 12],
                >(b"fgrep\0\0\0\0\0\0\0"),
                syntax: 0 as libc::c_int,
                compile: Some(
                    Fcompile
                        as unsafe extern "C" fn(
                            *mut libc::c_char,
                            idx_t,
                            reg_syntax_t,
                            bool,
                        ) -> *mut libc::c_void,
                ),
                execute: Some(
                    Fexecute
                        as unsafe extern "C" fn(
                            *mut libc::c_void,
                            *const libc::c_char,
                            idx_t,
                            *mut idx_t,
                            *const libc::c_char,
                        ) -> ptrdiff_t,
                ),
            };
            init
        },
        {
            let mut init = C2RustUnnamed_16 {
                name: *::std::mem::transmute::<
                    &[u8; 12],
                    &mut [libc::c_char; 12],
                >(b"awk\0\0\0\0\0\0\0\0\0"),
                syntax: (1 as libc::c_int as libc::c_ulong
                    | (((((((1 as libc::c_int as libc::c_ulong) << 1 as libc::c_int)
                        << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
                        << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int
                    | (((((((((((((1 as libc::c_int as libc::c_ulong)
                        << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
                        << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
                        << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
                        << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
                        << 1 as libc::c_int
                    | ((((((((((((((1 as libc::c_int as libc::c_ulong)
                        << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
                        << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
                        << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
                        << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
                        << 1 as libc::c_int) << 1 as libc::c_int
                    | (((((((((((((((1 as libc::c_int as libc::c_ulong)
                        << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
                        << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
                        << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
                        << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
                        << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int
                    | ((((((((((((((((1 as libc::c_int as libc::c_ulong)
                        << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
                        << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
                        << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
                        << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
                        << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
                        << 1 as libc::c_int
                    | ((((((1 as libc::c_int as libc::c_ulong) << 1 as libc::c_int)
                        << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
                        << 1 as libc::c_int) << 1 as libc::c_int
                    | (((1 as libc::c_int as libc::c_ulong) << 1 as libc::c_int)
                        << 1 as libc::c_int) << 1 as libc::c_int
                    | ((1 as libc::c_int as libc::c_ulong) << 1 as libc::c_int)
                        << 1 as libc::c_int
                    | (((((((((((((((((1 as libc::c_int as libc::c_ulong)
                        << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
                        << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
                        << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
                        << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
                        << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
                        << 1 as libc::c_int) << 1 as libc::c_int
                    | (((((((((((((((((((1 as libc::c_int as libc::c_ulong)
                        << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
                        << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
                        << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
                        << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
                        << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
                        << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
                        << 1 as libc::c_int) as libc::c_int,
                compile: Some(
                    GEAcompile
                        as unsafe extern "C" fn(
                            *mut libc::c_char,
                            idx_t,
                            reg_syntax_t,
                            bool,
                        ) -> *mut libc::c_void,
                ),
                execute: Some(
                    EGexecute
                        as unsafe extern "C" fn(
                            *mut libc::c_void,
                            *const libc::c_char,
                            idx_t,
                            *mut idx_t,
                            *const libc::c_char,
                        ) -> ptrdiff_t,
                ),
            };
            init
        },
        {
            let mut init = C2RustUnnamed_16 {
                name: *::std::mem::transmute::<
                    &[u8; 12],
                    &mut [libc::c_char; 12],
                >(b"gawk\0\0\0\0\0\0\0\0"),
                syntax: ((((1 as libc::c_int as libc::c_ulong) << 1 as libc::c_int)
                    << 1 as libc::c_int
                    | ((((((1 as libc::c_int as libc::c_ulong) << 1 as libc::c_int)
                        << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
                        << 1 as libc::c_int) << 1 as libc::c_int
                    | (((((((1 as libc::c_int as libc::c_ulong) << 1 as libc::c_int)
                        << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
                        << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int
                    | (((((((((1 as libc::c_int as libc::c_ulong) << 1 as libc::c_int)
                        << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
                        << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
                        << 1 as libc::c_int) << 1 as libc::c_int
                    | ((((((((((((((((1 as libc::c_int as libc::c_ulong)
                        << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
                        << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
                        << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
                        << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
                        << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
                        << 1 as libc::c_int
                    | (((1 as libc::c_int as libc::c_ulong) << 1 as libc::c_int)
                        << 1 as libc::c_int) << 1 as libc::c_int
                    | ((((1 as libc::c_int as libc::c_ulong) << 1 as libc::c_int)
                        << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int
                    | ((((((((((((1 as libc::c_int as libc::c_ulong) << 1 as libc::c_int)
                        << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
                        << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
                        << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
                        << 1 as libc::c_int) << 1 as libc::c_int
                    | (((((((((((((1 as libc::c_int as libc::c_ulong)
                        << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
                        << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
                        << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
                        << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
                        << 1 as libc::c_int
                    | (((((((((((((((1 as libc::c_int as libc::c_ulong)
                        << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
                        << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
                        << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
                        << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
                        << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int
                    | (((((1 as libc::c_int as libc::c_ulong) << 1 as libc::c_int)
                        << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
                        << 1 as libc::c_int
                    | (((((((((((((((((1 as libc::c_int as libc::c_ulong)
                        << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
                        << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
                        << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
                        << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
                        << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
                        << 1 as libc::c_int) << 1 as libc::c_int
                    | 1 as libc::c_int as libc::c_ulong
                    | (((((((((((((((((((((1 as libc::c_int as libc::c_ulong)
                        << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
                        << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
                        << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
                        << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
                        << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
                        << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
                        << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
                    & !((((((((1 as libc::c_int as libc::c_ulong) << 1 as libc::c_int)
                        << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
                        << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int
                        | ((((1 as libc::c_int as libc::c_ulong) << 1 as libc::c_int)
                            << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int
                        | (((((1 as libc::c_int as libc::c_ulong) << 1 as libc::c_int)
                            << 1 as libc::c_int) << 1 as libc::c_int)
                            << 1 as libc::c_int) << 1 as libc::c_int)) as libc::c_int,
                compile: Some(
                    GEAcompile
                        as unsafe extern "C" fn(
                            *mut libc::c_char,
                            idx_t,
                            reg_syntax_t,
                            bool,
                        ) -> *mut libc::c_void,
                ),
                execute: Some(
                    EGexecute
                        as unsafe extern "C" fn(
                            *mut libc::c_void,
                            *const libc::c_char,
                            idx_t,
                            *mut idx_t,
                            *const libc::c_char,
                        ) -> ptrdiff_t,
                ),
            };
            init
        },
        {
            let mut init = C2RustUnnamed_16 {
                name: *::std::mem::transmute::<
                    &[u8; 12],
                    &mut [libc::c_char; 12],
                >(b"posixawk\0\0\0\0"),
                syntax: (((1 as libc::c_int as libc::c_ulong) << 1 as libc::c_int)
                    << 1 as libc::c_int
                    | ((((((1 as libc::c_int as libc::c_ulong) << 1 as libc::c_int)
                        << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
                        << 1 as libc::c_int) << 1 as libc::c_int
                    | (((((((1 as libc::c_int as libc::c_ulong) << 1 as libc::c_int)
                        << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
                        << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int
                    | (((((((((1 as libc::c_int as libc::c_ulong) << 1 as libc::c_int)
                        << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
                        << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
                        << 1 as libc::c_int) << 1 as libc::c_int
                    | ((((((((((((((((1 as libc::c_int as libc::c_ulong)
                        << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
                        << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
                        << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
                        << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
                        << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
                        << 1 as libc::c_int
                    | (((1 as libc::c_int as libc::c_ulong) << 1 as libc::c_int)
                        << 1 as libc::c_int) << 1 as libc::c_int
                    | ((((1 as libc::c_int as libc::c_ulong) << 1 as libc::c_int)
                        << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int
                    | ((((((((((((1 as libc::c_int as libc::c_ulong) << 1 as libc::c_int)
                        << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
                        << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
                        << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
                        << 1 as libc::c_int) << 1 as libc::c_int
                    | (((((((((((((1 as libc::c_int as libc::c_ulong)
                        << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
                        << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
                        << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
                        << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
                        << 1 as libc::c_int
                    | (((((((((((((((1 as libc::c_int as libc::c_ulong)
                        << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
                        << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
                        << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
                        << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
                        << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int
                    | (((((1 as libc::c_int as libc::c_ulong) << 1 as libc::c_int)
                        << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
                        << 1 as libc::c_int
                    | (((((((((((((((((1 as libc::c_int as libc::c_ulong)
                        << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
                        << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
                        << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
                        << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
                        << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
                        << 1 as libc::c_int) << 1 as libc::c_int
                    | 1 as libc::c_int as libc::c_ulong
                    | (((((((((1 as libc::c_int as libc::c_ulong) << 1 as libc::c_int)
                        << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
                        << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
                        << 1 as libc::c_int) << 1 as libc::c_int
                    | (((((((((((((((((((1 as libc::c_int as libc::c_ulong)
                        << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
                        << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
                        << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
                        << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
                        << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
                        << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
                        << 1 as libc::c_int
                    | (((((((((((((((((((((1 as libc::c_int as libc::c_ulong)
                        << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
                        << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
                        << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
                        << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
                        << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
                        << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
                        << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
                    as libc::c_int,
                compile: Some(
                    GEAcompile
                        as unsafe extern "C" fn(
                            *mut libc::c_char,
                            idx_t,
                            reg_syntax_t,
                            bool,
                        ) -> *mut libc::c_void,
                ),
                execute: Some(
                    EGexecute
                        as unsafe extern "C" fn(
                            *mut libc::c_void,
                            *const libc::c_char,
                            idx_t,
                            *mut idx_t,
                            *const libc::c_char,
                        ) -> ptrdiff_t,
                ),
            };
            init
        },
    ]
};
unsafe extern "C" fn setmatcher(
    mut m: *const libc::c_char,
    mut matcher: libc::c_int,
) -> libc::c_int {
    let mut i: libc::c_int = 0 as libc::c_int;
    while (i as libc::c_ulong)
        < (::std::mem::size_of::<[C2RustUnnamed_16; 6]>() as libc::c_ulong)
            .wrapping_div(::std::mem::size_of::<C2RustUnnamed_16>() as libc::c_ulong)
    {
        if strcmp(m, (matchers[i as usize].name).as_ptr()) == 0 as libc::c_int {
            if 0 as libc::c_int <= matcher && matcher != i {
                if ::std::mem::size_of::<C2RustUnnamed_20>() as libc::c_ulong != 0 {
                    error(
                        EXIT_TROUBLE as libc::c_int,
                        0 as libc::c_int,
                        dcgettext(
                            0 as *const libc::c_char,
                            b"conflicting matchers specified\0" as *const u8
                                as *const libc::c_char,
                            5 as libc::c_int,
                        ),
                    );
                    if 0 as libc::c_int != 0 {} else {
                        unreachable!();
                    };
                } else {
                    error(
                        EXIT_TROUBLE as libc::c_int,
                        0 as libc::c_int,
                        dcgettext(
                            0 as *const libc::c_char,
                            b"conflicting matchers specified\0" as *const u8
                                as *const libc::c_char,
                            5 as libc::c_int,
                        ),
                    );
                    if 0 as libc::c_int != 0 {} else {
                        unreachable!();
                    };
                };
            }
            return i;
        }
        i += 1;
        i;
    }
    if strcmp(m, b"perl\0" as *const u8 as *const libc::c_char) == 0 as libc::c_int {
        if ::std::mem::size_of::<C2RustUnnamed_19>() as libc::c_ulong != 0 {
            error(
                EXIT_TROUBLE as libc::c_int,
                0 as libc::c_int,
                dcgettext(
                    0 as *const libc::c_char,
                    b"Perl matching not supported in a --disable-perl-regexp build\0"
                        as *const u8 as *const libc::c_char,
                    5 as libc::c_int,
                ),
            );
            if 0 as libc::c_int != 0 {} else {
                unreachable!();
            };
        } else {
            error(
                EXIT_TROUBLE as libc::c_int,
                0 as libc::c_int,
                dcgettext(
                    0 as *const libc::c_char,
                    b"Perl matching not supported in a --disable-perl-regexp build\0"
                        as *const u8 as *const libc::c_char,
                    5 as libc::c_int,
                ),
            );
            if 0 as libc::c_int != 0 {} else {
                unreachable!();
            };
        };
    }
    if ::std::mem::size_of::<C2RustUnnamed_18>() as libc::c_ulong != 0 {
        error(
            EXIT_TROUBLE as libc::c_int,
            0 as libc::c_int,
            dcgettext(
                0 as *const libc::c_char,
                b"invalid matcher %s\0" as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ),
            m,
        );
        if 0 as libc::c_int != 0 {} else {
            unreachable!();
        };
    } else {
        error(
            EXIT_TROUBLE as libc::c_int,
            0 as libc::c_int,
            dcgettext(
                0 as *const libc::c_char,
                b"invalid matcher %s\0" as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ),
            m,
        );
        if 0 as libc::c_int != 0 {} else {
            unreachable!();
        };
    };
    panic!();
}
unsafe extern "C" fn get_nondigit_option(
    mut argc: libc::c_int,
    mut argv: *const *mut libc::c_char,
    mut default_context: *mut intmax_t,
) -> libc::c_int {
    static mut prev_digit_optind: libc::c_int = -(1 as libc::c_int);
    let mut this_digit_optind: libc::c_int = 0;
    let mut was_digit: bool = false;
    let mut buf: [libc::c_char; 25] = [0; 25];
    let mut p: *mut libc::c_char = buf.as_mut_ptr();
    let mut opt: libc::c_int = 0;
    was_digit = 0 as libc::c_int != 0;
    this_digit_optind = optind;
    loop {
        opt = getopt_long(
            argc,
            argv as *mut *mut libc::c_char,
            short_options.as_ptr(),
            long_options.as_ptr(),
            0 as *mut libc::c_int,
        );
        if !c_isdigit(opt) {
            break;
        }
        if prev_digit_optind != this_digit_optind || !was_digit {
            p = buf.as_mut_ptr();
        } else {
            p = p
                .offset(
                    -((buf[0 as libc::c_int as usize] as libc::c_int == '0' as i32)
                        as libc::c_int as isize),
                );
        }
        if p
            == buf
                .as_mut_ptr()
                .offset(
                    ::std::mem::size_of::<[libc::c_char; 25]>() as libc::c_ulong as isize,
                )
                .offset(-(4 as libc::c_int as isize))
        {
            strcpy(p, b"...\0" as *const u8 as *const libc::c_char);
            p = p.offset(3 as libc::c_int as isize);
            break;
        } else {
            let fresh11 = p;
            p = p.offset(1);
            *fresh11 = opt as libc::c_char;
            was_digit = 1 as libc::c_int != 0;
            prev_digit_optind = this_digit_optind;
            this_digit_optind = optind;
        }
    }
    if p != buf.as_mut_ptr() {
        *p = '\0' as i32 as libc::c_char;
        context_length_arg(buf.as_mut_ptr(), default_context);
    }
    return opt;
}
unsafe extern "C" fn parse_grep_colors() {
    let mut p: *const libc::c_char = 0 as *const libc::c_char;
    let mut q: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut name: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut val: *mut libc::c_char = 0 as *mut libc::c_char;
    p = getenv(b"GREP_COLORS\0" as *const u8 as *const libc::c_char);
    if p.is_null() || *p as libc::c_int == '\0' as i32 {
        return;
    }
    q = xstrdup(p);
    name = q;
    val = 0 as *mut libc::c_char;
    loop {
        if *q as libc::c_int == ':' as i32 || *q as libc::c_int == '\0' as i32 {
            let mut c: libc::c_char = *q;
            let mut cap: *const color_cap = 0 as *const color_cap;
            let fresh12 = q;
            q = q.offset(1);
            *fresh12 = '\0' as i32 as libc::c_char;
            cap = color_dict.as_ptr();
            while !((*cap).name).is_null() {
                if strcmp((*cap).name, name) == 0 as libc::c_int {
                    break;
                }
                cap = cap.offset(1);
                cap;
            }
            if !((*cap).var).is_null() && !val.is_null() {
                *(*cap).var = val;
            }
            if ((*cap).fct).is_some() {
                ((*cap).fct).unwrap()();
            }
            if c as libc::c_int == '\0' as i32 {
                return;
            }
            name = q;
            val = 0 as *mut libc::c_char;
        } else if *q as libc::c_int == '=' as i32 {
            if q == name || !val.is_null() {
                return;
            }
            let fresh13 = q;
            q = q.offset(1);
            *fresh13 = '\0' as i32 as libc::c_char;
            val = q;
        } else if val.is_null() {
            q = q.offset(1);
            q;
        } else if *q as libc::c_int == ';' as i32
            || c_isdigit(*q as libc::c_int) as libc::c_int != 0
        {
            q = q.offset(1);
            q;
        } else {
            return
        }
    };
}
unsafe extern "C" fn contains_encoding_error(
    mut pat: *const libc::c_char,
    mut patlen: idx_t,
) -> bool {
    let mut mbs: mbstate_t = {
        let mut init = __mbstate_t {
            __count: 0 as libc::c_int,
            __value: C2RustUnnamed { __wch: 0 },
        };
        init
    };
    let mut charlen: ptrdiff_t = 0;
    let mut i: idx_t = 0 as libc::c_int as idx_t;
    while i < patlen {
        charlen = mb_clen(pat.offset(i as isize), patlen - i, &mut mbs);
        if charlen < 0 as libc::c_int as libc::c_long {
            return 1 as libc::c_int != 0;
        }
        i += charlen;
    }
    return 0 as libc::c_int != 0;
}
static mut ok_fold: [libc::c_schar; 256] = [0; 256];
unsafe extern "C" fn setup_ok_fold() {
    let mut i: libc::c_int = 0 as libc::c_int;
    while i < NCHAR as libc::c_int {
        let mut wi: wint_t = localeinfo.sbctowc[i as usize];
        if !(wi == 0xffffffff as libc::c_uint) {
            let mut ok: libc::c_int = 1 as libc::c_int;
            let mut folded: [wchar_t; 32] = [0; 32];
            let mut n: libc::c_int = case_folded_counterparts(wi, folded.as_mut_ptr());
            loop {
                n -= 1;
                if !(0 as libc::c_int <= n) {
                    break;
                }
                let mut buf: [libc::c_char; 16] = [0; 16];
                let mut s: mbstate_t = {
                    let mut init = __mbstate_t {
                        __count: 0 as libc::c_int,
                        __value: C2RustUnnamed { __wch: 0 },
                    };
                    init
                };
                if !(wcrtomb(buf.as_mut_ptr(), folded[n as usize], &mut s)
                    != 1 as libc::c_int as libc::c_ulong)
                {
                    continue;
                }
                ok = -(1 as libc::c_int);
                break;
            }
            ok_fold[i as usize] = ok as libc::c_schar;
        }
        i += 1;
        i;
    }
}
unsafe extern "C" fn fgrep_icase_charlen(
    mut pat: *const libc::c_char,
    mut patlen: idx_t,
    mut mbs: *mut mbstate_t,
) -> ptrdiff_t {
    let mut pat0: libc::c_uchar = *pat.offset(0 as libc::c_int as isize)
        as libc::c_uchar;
    if localeinfo.sbctowc[pat0 as usize] != 0xffffffff as libc::c_uint {
        return ok_fold[pat0 as usize] as ptrdiff_t;
    }
    let mut wc: wchar_t = 0;
    let mut wn: size_t = rpl_mbrtowc(&mut wc, pat, patlen as size_t, mbs);
    if (16 as libc::c_int as libc::c_ulong) < wn {
        return -(1 as libc::c_int) as ptrdiff_t;
    }
    let mut folded: [wchar_t; 32] = [0; 32];
    if case_folded_counterparts(wc as wint_t, folded.as_mut_ptr()) != 0 {
        return -(1 as libc::c_int) as ptrdiff_t;
    }
    let mut i: idx_t = wn as idx_t;
    loop {
        i -= 1;
        if !((0 as libc::c_int as libc::c_long) < i) {
            break;
        }
        let mut c: libc::c_uchar = *pat.offset(i as isize) as libc::c_uchar;
        if ({
            let mut __res: libc::c_int = 0;
            if ::std::mem::size_of::<libc::c_uchar>() as libc::c_ulong
                > 1 as libc::c_int as libc::c_ulong
            {
                if 0 != 0 {
                    let mut __c: libc::c_int = c as libc::c_int;
                    __res = if __c < -(128 as libc::c_int) || __c > 255 as libc::c_int {
                        __c
                    } else {
                        *(*__ctype_toupper_loc()).offset(__c as isize)
                    };
                } else {
                    __res = toupper(c as libc::c_int);
                }
            } else {
                __res = *(*__ctype_toupper_loc()).offset(c as libc::c_int as isize);
            }
            __res
        }) != c as libc::c_int
        {
            return -(1 as libc::c_int) as ptrdiff_t;
        }
    }
    return wn as ptrdiff_t;
}
unsafe extern "C" fn fgrep_icase_available(
    mut pat: *const libc::c_char,
    mut patlen: idx_t,
) -> bool {
    let mut mbs: mbstate_t = {
        let mut init = __mbstate_t {
            __count: 0 as libc::c_int,
            __value: C2RustUnnamed { __wch: 0 },
        };
        init
    };
    let mut i: idx_t = 0 as libc::c_int as idx_t;
    while i < patlen {
        let mut n: libc::c_int = fgrep_icase_charlen(
            pat.offset(i as isize),
            patlen - i,
            &mut mbs,
        ) as libc::c_int;
        if n < 0 as libc::c_int {
            return 0 as libc::c_int != 0;
        }
        i += n as libc::c_long;
    }
    return 1 as libc::c_int != 0;
}
pub unsafe extern "C" fn fgrep_to_grep_pattern(
    mut keys_p: *mut *mut libc::c_char,
    mut len_p: *mut idx_t,
) {
    let mut len: idx_t = *len_p;
    let mut keys: *mut libc::c_char = *keys_p;
    let mut mb_state: mbstate_t = {
        let mut init = __mbstate_t {
            __count: 0 as libc::c_int,
            __value: C2RustUnnamed { __wch: 0 },
        };
        init
    };
    let mut new_keys: *mut libc::c_char = xnmalloc(
        (len + 1 as libc::c_int as libc::c_long) as size_t,
        2 as libc::c_int as size_t,
    ) as *mut libc::c_char;
    let mut p: *mut libc::c_char = new_keys;
    let mut n: ptrdiff_t = 0;
    while len != 0 {
        n = mb_clen(keys, len, &mut mb_state);
        let mut current_block_9: u64;
        match n {
            -2 => {
                n = len;
                current_block_9 = 10049178306403832508;
            }
            -1 => {
                memset(
                    &mut mb_state as *mut mbstate_t as *mut libc::c_void,
                    0 as libc::c_int,
                    ::std::mem::size_of::<mbstate_t>() as libc::c_ulong,
                );
                n = 1 as libc::c_int as ptrdiff_t;
                current_block_9 = 10592720037715695683;
            }
            1 => {
                current_block_9 = 10592720037715695683;
            }
            _ => {
                current_block_9 = 10049178306403832508;
            }
        }
        match current_block_9 {
            10049178306403832508 => {
                p = mempcpy(
                    p as *mut libc::c_void,
                    keys as *const libc::c_void,
                    n as libc::c_ulong,
                ) as *mut libc::c_char;
            }
            _ => {
                match *keys as libc::c_int {
                    36 | 42 | 46 | 91 | 92 | 94 => {
                        let fresh14 = p;
                        p = p.offset(1);
                        *fresh14 = '\\' as i32 as libc::c_char;
                    }
                    _ => {}
                }
                let fresh15 = p;
                p = p.offset(1);
                *fresh15 = *keys;
            }
        }
        keys = keys.offset(n as isize);
        len -= n;
    }
    *p = '\n' as i32 as libc::c_char;
    rpl_free(*keys_p as *mut libc::c_void);
    *keys_p = new_keys;
    *len_p = p.offset_from(new_keys) as libc::c_long;
}
unsafe extern "C" fn try_fgrep_pattern(
    mut matcher: libc::c_int,
    mut keys: *mut libc::c_char,
    mut len_p: *mut idx_t,
) -> libc::c_int {
    let mut current_block: u64;
    let mut result: libc::c_int = matcher;
    let mut len: idx_t = *len_p;
    let mut new_keys: *mut libc::c_char = ximalloc(
        len + 1 as libc::c_int as libc::c_long,
    ) as *mut libc::c_char;
    let mut p: *mut libc::c_char = new_keys;
    let mut q: *const libc::c_char = keys;
    let mut mb_state: mbstate_t = {
        let mut init = __mbstate_t {
            __count: 0 as libc::c_int,
            __value: C2RustUnnamed { __wch: 0 },
        };
        init
    };
    loop {
        if !(len != 0 as libc::c_int as libc::c_long) {
            current_block = 9828876828309294594;
            break;
        }
        match *q as libc::c_int {
            36 | 42 | 46 | 91 | 94 => {
                current_block = 17735046344643924837;
                break;
            }
            40 | 43 | 63 | 123 | 124 => {
                if matcher != G_MATCHER_INDEX as libc::c_int {
                    current_block = 17735046344643924837;
                    break;
                }
            }
            92 => {
                if (1 as libc::c_int as libc::c_long) < len {
                    match *q.offset(1 as libc::c_int as isize) as libc::c_int {
                        10 | 66 | 83 | 87 | 39 | 60 | 98 | 115 | 119 | 96 | 62 | 49 | 50
                        | 51 | 52 | 53 | 54 | 55 | 56 | 57 => {
                            current_block = 17735046344643924837;
                            break;
                        }
                        40 | 43 | 63 | 123 | 124 | 41 => {
                            if matcher == G_MATCHER_INDEX as libc::c_int {
                                current_block = 17735046344643924837;
                                break;
                            }
                        }
                        _ => {}
                    }
                    q = q.offset(1);
                    q;
                    len -= 1;
                    len;
                }
            }
            _ => {}
        }
        let mut clen: ptrdiff_t = if match_icase as libc::c_int != 0 {
            fgrep_icase_charlen(q, len, &mut mb_state)
        } else {
            mb_clen(q, len, &mut mb_state)
        };
        if clen < 0 as libc::c_int as libc::c_long {
            current_block = 17735046344643924837;
            break;
        }
        p = mempcpy(
            p as *mut libc::c_void,
            q as *const libc::c_void,
            clen as libc::c_ulong,
        ) as *mut libc::c_char;
        q = q.offset(clen as isize);
        len -= clen;
    }
    match current_block {
        9828876828309294594 => {
            if *len_p != p.offset_from(new_keys) as libc::c_long {
                *len_p = p.offset_from(new_keys) as libc::c_long;
                let mut keys_end: *mut libc::c_char = mempcpy(
                    keys as *mut libc::c_void,
                    new_keys as *const libc::c_void,
                    p.offset_from(new_keys) as libc::c_long as libc::c_ulong,
                ) as *mut libc::c_char;
                *keys_end = '\n' as i32 as libc::c_char;
            }
            result = F_MATCHER_INDEX as libc::c_int;
        }
        _ => {}
    }
    rpl_free(new_keys as *mut libc::c_void);
    return result;
}
unsafe fn main_0(
    mut argc: libc::c_int,
    mut argv: *mut *mut libc::c_char,
) -> libc::c_int {
    let mut keys: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut keycc: idx_t = 0 as libc::c_int as idx_t;
    let mut keyalloc: idx_t = 0 as libc::c_int as idx_t;
    let mut matcher: libc::c_int = -(1 as libc::c_int);
    let mut opt: libc::c_int = 0;
    let mut prev_optind: libc::c_int = 0;
    let mut last_recursive: libc::c_int = 0;
    let mut default_context: intmax_t = 0;
    let mut fp: *mut FILE = 0 as *mut FILE;
    ::std::ptr::write_volatile(
        &mut exit_failure as *mut libc::c_int,
        EXIT_TROUBLE as libc::c_int,
    );
    let mut filename_option: libc::c_int = 0 as libc::c_int;
    eolbyte = '\n' as i32 as libc::c_char;
    filename_mask = !(0 as libc::c_int);
    max_count = 9223372036854775807 as libc::c_long;
    out_before = -(1 as libc::c_int) as intmax_t;
    out_after = out_before;
    default_context = -(1 as libc::c_int) as intmax_t;
    only_matching = 0 as libc::c_int != 0;
    setlocale(6 as libc::c_int, b"\0" as *const u8 as *const libc::c_char);
    bindtextdomain(
        b"grep\0" as *const u8 as *const libc::c_char,
        b"/usr/local/share/locale\0" as *const u8 as *const libc::c_char,
    );
    textdomain(b"grep\0" as *const u8 as *const libc::c_char);
    init_localeinfo(&mut localeinfo);
    atexit(Some(clean_up_stdout as unsafe extern "C" fn() -> ()));
    c_stack_action(None);
    last_recursive = 0 as libc::c_int;
    pattern_table = hash_initialize(
        0 as libc::c_int as size_t,
        0 as *const Hash_tuning,
        Some(
            hash_pattern as unsafe extern "C" fn(*const libc::c_void, size_t) -> size_t,
        ),
        Some(
            compare_patterns
                as unsafe extern "C" fn(*const libc::c_void, *const libc::c_void) -> bool,
        ),
        None,
    );
    if pattern_table.is_null() {
        xalloc_die();
    }
    loop {
        prev_optind = optind;
        opt = get_nondigit_option(argc, argv, &mut default_context);
        if !(opt != -(1 as libc::c_int)) {
            break;
        }
        let mut current_block_134: u64;
        match opt {
            65 => {
                context_length_arg(optarg, &mut out_after);
                current_block_134 = 4534765400774009001;
            }
            66 => {
                context_length_arg(optarg, &mut out_before);
                current_block_134 = 4534765400774009001;
            }
            67 => {
                context_length_arg(optarg, &mut default_context);
                current_block_134 = 4534765400774009001;
            }
            68 => {
                if strcmp(optarg, b"read\0" as *const u8 as *const libc::c_char)
                    == 0 as libc::c_int
                {
                    devices = READ_DEVICES;
                } else if strcmp(optarg, b"skip\0" as *const u8 as *const libc::c_char)
                    == 0 as libc::c_int
                {
                    devices = SKIP_DEVICES;
                } else {
                    if ::std::mem::size_of::<C2RustUnnamed_26>() as libc::c_ulong != 0 {
                        error(
                            EXIT_TROUBLE as libc::c_int,
                            0 as libc::c_int,
                            dcgettext(
                                0 as *const libc::c_char,
                                b"unknown devices method\0" as *const u8
                                    as *const libc::c_char,
                                5 as libc::c_int,
                            ),
                        );
                        if 0 as libc::c_int != 0 {} else {
                            unreachable!();
                        };
                    } else {
                        error(
                            EXIT_TROUBLE as libc::c_int,
                            0 as libc::c_int,
                            dcgettext(
                                0 as *const libc::c_char,
                                b"unknown devices method\0" as *const u8
                                    as *const libc::c_char,
                                5 as libc::c_int,
                            ),
                        );
                        if 0 as libc::c_int != 0 {} else {
                            unreachable!();
                        };
                    };
                }
                current_block_134 = 4534765400774009001;
            }
            69 => {
                matcher = setmatcher(
                    b"egrep\0" as *const u8 as *const libc::c_char,
                    matcher,
                );
                current_block_134 = 4534765400774009001;
            }
            70 => {
                matcher = setmatcher(
                    b"fgrep\0" as *const u8 as *const libc::c_char,
                    matcher,
                );
                current_block_134 = 4534765400774009001;
            }
            80 => {
                matcher = setmatcher(
                    b"perl\0" as *const u8 as *const libc::c_char,
                    matcher,
                );
                current_block_134 = 4534765400774009001;
            }
            71 => {
                matcher = setmatcher(
                    b"grep\0" as *const u8 as *const libc::c_char,
                    matcher,
                );
                current_block_134 = 4534765400774009001;
            }
            88 => {
                matcher = setmatcher(optarg, matcher);
                current_block_134 = 4534765400774009001;
            }
            72 => {
                filename_option = 1 as libc::c_int;
                current_block_134 = 4534765400774009001;
            }
            73 => {
                binary_files = WITHOUT_MATCH_BINARY_FILES;
                current_block_134 = 4534765400774009001;
            }
            84 => {
                align_tabs = 1 as libc::c_int != 0;
                current_block_134 = 4534765400774009001;
            }
            117 => {
                error(
                    0 as libc::c_int,
                    0 as libc::c_int,
                    dcgettext(
                        0 as *const libc::c_char,
                        b"warning: --unix-byte-offsets (-u) is obsolete\0" as *const u8
                            as *const libc::c_char,
                        5 as libc::c_int,
                    ),
                );
                current_block_134 = 4534765400774009001;
            }
            86 => {
                show_version = 1 as libc::c_int != 0;
                current_block_134 = 4534765400774009001;
            }
            97 => {
                binary_files = TEXT_BINARY_FILES;
                current_block_134 = 4534765400774009001;
            }
            98 => {
                out_byte = 1 as libc::c_int != 0;
                current_block_134 = 4534765400774009001;
            }
            99 => {
                count_matches = 1 as libc::c_int != 0;
                current_block_134 = 4534765400774009001;
            }
            100 => {
                directories = directories_types[__xargmatch_internal(
                    b"--directories\0" as *const u8 as *const libc::c_char,
                    optarg,
                    directories_args.as_ptr(),
                    directories_types.as_ptr() as *const libc::c_void,
                    ::std::mem::size_of::<directories_type>() as libc::c_ulong,
                    argmatch_die,
                    1 as libc::c_int != 0,
                ) as usize];
                if directories as libc::c_uint
                    == RECURSE_DIRECTORIES as libc::c_int as libc::c_uint
                {
                    last_recursive = prev_optind;
                }
                current_block_134 = 4534765400774009001;
            }
            101 => {
                let mut cc: idx_t = strlen(optarg) as idx_t;
                let mut shortage: ptrdiff_t = keycc - keyalloc + cc
                    + 1 as libc::c_int as libc::c_long;
                if (0 as libc::c_int as libc::c_long) < shortage {
                    keys = xpalloc(
                        keys as *mut libc::c_void,
                        &mut keyalloc,
                        shortage,
                        -(1 as libc::c_int) as ptrdiff_t,
                        1 as libc::c_int as idx_t,
                    ) as *mut libc::c_char;
                    pattern_array = keys;
                }
                let mut keyend: *mut libc::c_char = mempcpy(
                    keys.offset(keycc as isize) as *mut libc::c_void,
                    optarg as *const libc::c_void,
                    cc as libc::c_ulong,
                ) as *mut libc::c_char;
                *keyend = '\n' as i32 as libc::c_char;
                keycc = update_patterns(
                    keys,
                    keycc,
                    keycc + cc + 1 as libc::c_int as libc::c_long,
                    b"\0" as *const u8 as *const libc::c_char,
                );
                current_block_134 = 4534765400774009001;
            }
            102 => {
                if strcmp(optarg, b"-\0" as *const u8 as *const libc::c_char)
                    == 0 as libc::c_int
                {
                    if binary {
                        xset_binary_mode(0 as libc::c_int, 0 as libc::c_int);
                    }
                    fp = stdin;
                } else {
                    fp = rpl_fopen(
                        optarg,
                        if binary as libc::c_int != 0 {
                            b"rb\0" as *const u8 as *const libc::c_char
                        } else {
                            b"r\0" as *const u8 as *const libc::c_char
                        },
                    );
                    if fp.is_null() {
                        if ::std::mem::size_of::<C2RustUnnamed_25>() as libc::c_ulong
                            != 0
                        {
                            error(
                                EXIT_TROUBLE as libc::c_int,
                                *__errno_location(),
                                b"%s\0" as *const u8 as *const libc::c_char,
                                optarg,
                            );
                            if 0 as libc::c_int != 0 {} else {
                                unreachable!();
                            };
                        } else {
                            error(
                                EXIT_TROUBLE as libc::c_int,
                                *__errno_location(),
                                b"%s\0" as *const u8 as *const libc::c_char,
                                optarg,
                            );
                            if 0 as libc::c_int != 0 {} else {
                                unreachable!();
                            };
                        };
                    }
                }
                let mut newkeycc: idx_t = keycc;
                let mut cc_0: idx_t = 0;
                loop {
                    let mut shortage_0: ptrdiff_t = newkeycc - keyalloc
                        + 2 as libc::c_int as libc::c_long;
                    if (0 as libc::c_int as libc::c_long) < shortage_0 {
                        keys = xpalloc(
                            keys as *mut libc::c_void,
                            &mut keyalloc,
                            shortage_0,
                            -(1 as libc::c_int) as ptrdiff_t,
                            1 as libc::c_int as idx_t,
                        ) as *mut libc::c_char;
                        pattern_array = keys;
                    }
                    cc_0 = (if 0 != 0 && 0 != 0
                        && (1 as libc::c_int as size_t)
                            .wrapping_mul(
                                (keyalloc - (newkeycc + 1 as libc::c_int as libc::c_long))
                                    as size_t,
                            ) <= 8 as libc::c_int as libc::c_ulong
                        && 1 as libc::c_int as size_t
                            != 0 as libc::c_int as libc::c_ulong
                    {
                        {
                            let mut __ptr: *mut libc::c_char = keys
                                .offset(newkeycc as isize);
                            let mut __stream: *mut FILE = fp;
                            let mut __cnt: size_t = 0;
                            __cnt = (1 as libc::c_int as size_t)
                                .wrapping_mul(
                                    (keyalloc - (newkeycc + 1 as libc::c_int as libc::c_long))
                                        as size_t,
                                );
                            while __cnt > 0 as libc::c_int as libc::c_ulong {
                                let mut __c: libc::c_int = getc_unlocked(__stream);
                                if __c == -(1 as libc::c_int) {
                                    break;
                                }
                                let fresh16 = __ptr;
                                __ptr = __ptr.offset(1);
                                *fresh16 = __c as libc::c_char;
                                __cnt = __cnt.wrapping_sub(1);
                                __cnt;
                            }
                            (1 as libc::c_int as size_t)
                                .wrapping_mul(
                                    (keyalloc - (newkeycc + 1 as libc::c_int as libc::c_long))
                                        as size_t,
                                )
                                .wrapping_sub(__cnt)
                                .wrapping_div(1 as libc::c_int as size_t)
                        }
                    } else if 0 != 0
                        && 1 as libc::c_int as size_t
                            == 0 as libc::c_int as libc::c_ulong
                        || 0 != 0
                            && (keyalloc - (newkeycc + 1 as libc::c_int as libc::c_long))
                                as size_t == 0 as libc::c_int as libc::c_ulong
                    {
                        0 as libc::c_int as size_t
                    } else {
                        fread_unlocked(
                            keys.offset(newkeycc as isize) as *mut libc::c_void,
                            1 as libc::c_int as size_t,
                            (keyalloc - (newkeycc + 1 as libc::c_int as libc::c_long))
                                as size_t,
                            fp,
                        )
                    }) as idx_t;
                    if cc_0 == 0 as libc::c_int as libc::c_long {
                        break;
                    }
                    newkeycc += cc_0;
                }
                let mut err: libc::c_int = *__errno_location();
                if ferror_unlocked(fp) == 0 {
                    err = 0 as libc::c_int;
                    if fp == stdin {
                        clearerr_unlocked(fp);
                    } else if fclose(fp) != 0 as libc::c_int {
                        err = *__errno_location();
                    }
                }
                if err != 0 {
                    if ::std::mem::size_of::<C2RustUnnamed_24>() as libc::c_ulong != 0 {
                        error(
                            EXIT_TROUBLE as libc::c_int,
                            err,
                            b"%s\0" as *const u8 as *const libc::c_char,
                            optarg,
                        );
                        if 0 as libc::c_int != 0 {} else {
                            unreachable!();
                        };
                    } else {
                        error(
                            EXIT_TROUBLE as libc::c_int,
                            err,
                            b"%s\0" as *const u8 as *const libc::c_char,
                            optarg,
                        );
                        if 0 as libc::c_int != 0 {} else {
                            unreachable!();
                        };
                    };
                }
                if newkeycc != keycc
                    && *keys
                        .offset((newkeycc - 1 as libc::c_int as libc::c_long) as isize)
                        as libc::c_int != '\n' as i32
                {
                    let fresh17 = newkeycc;
                    newkeycc = newkeycc + 1;
                    *keys.offset(fresh17 as isize) = '\n' as i32 as libc::c_char;
                }
                keycc = update_patterns(keys, keycc, newkeycc, optarg);
                current_block_134 = 4534765400774009001;
            }
            104 => {
                filename_option = -(1 as libc::c_int);
                current_block_134 = 4534765400774009001;
            }
            105 | 121 => {
                match_icase = 1 as libc::c_int != 0;
                current_block_134 = 4534765400774009001;
            }
            137 => {
                match_icase = 0 as libc::c_int != 0;
                current_block_134 = 4534765400774009001;
            }
            76 => {
                list_files = LISTFILES_NONMATCHING;
                current_block_134 = 4534765400774009001;
            }
            108 => {
                list_files = LISTFILES_MATCHING;
                current_block_134 = 4534765400774009001;
            }
            109 => {
                match xstrtoimax(
                    optarg,
                    0 as *mut *mut libc::c_char,
                    10 as libc::c_int,
                    &mut max_count,
                    b"\0" as *const u8 as *const libc::c_char,
                ) as libc::c_uint
                {
                    0 | 1 => {}
                    _ => {
                        if ::std::mem::size_of::<C2RustUnnamed_23>() as libc::c_ulong
                            != 0
                        {
                            error(
                                EXIT_TROUBLE as libc::c_int,
                                0 as libc::c_int,
                                dcgettext(
                                    0 as *const libc::c_char,
                                    b"invalid max count\0" as *const u8 as *const libc::c_char,
                                    5 as libc::c_int,
                                ),
                            );
                            if 0 as libc::c_int != 0 {} else {
                                unreachable!();
                            };
                        } else {
                            error(
                                EXIT_TROUBLE as libc::c_int,
                                0 as libc::c_int,
                                dcgettext(
                                    0 as *const libc::c_char,
                                    b"invalid max count\0" as *const u8 as *const libc::c_char,
                                    5 as libc::c_int,
                                ),
                            );
                            if 0 as libc::c_int != 0 {} else {
                                unreachable!();
                            };
                        };
                    }
                }
                current_block_134 = 4534765400774009001;
            }
            110 => {
                out_line = 1 as libc::c_int != 0;
                current_block_134 = 4534765400774009001;
            }
            111 => {
                only_matching = 1 as libc::c_int != 0;
                current_block_134 = 4534765400774009001;
            }
            113 => {
                exit_on_match = 1 as libc::c_int != 0;
                ::std::ptr::write_volatile(
                    &mut exit_failure as *mut libc::c_int,
                    0 as libc::c_int,
                );
                current_block_134 = 4534765400774009001;
            }
            82 => {
                fts_options = basic_fts_options as libc::c_int | 0x2 as libc::c_int;
                current_block_134 = 17041416599325743607;
            }
            114 => {
                current_block_134 = 17041416599325743607;
            }
            115 => {
                suppress_errors = 1 as libc::c_int != 0;
                current_block_134 = 4534765400774009001;
            }
            118 => {
                out_invert = 1 as libc::c_int != 0;
                current_block_134 = 4534765400774009001;
            }
            119 => {
                wordinit();
                match_words = 1 as libc::c_int != 0;
                current_block_134 = 4534765400774009001;
            }
            120 => {
                match_lines = 1 as libc::c_int != 0;
                current_block_134 = 4534765400774009001;
            }
            90 => {
                filename_mask = 0 as libc::c_int;
                current_block_134 = 4534765400774009001;
            }
            122 => {
                eolbyte = '\0' as i32 as libc::c_char;
                current_block_134 = 4534765400774009001;
            }
            128 => {
                if strcmp(optarg, b"binary\0" as *const u8 as *const libc::c_char)
                    == 0 as libc::c_int
                {
                    binary_files = BINARY_BINARY_FILES;
                } else if strcmp(optarg, b"text\0" as *const u8 as *const libc::c_char)
                    == 0 as libc::c_int
                {
                    binary_files = TEXT_BINARY_FILES;
                } else if strcmp(
                    optarg,
                    b"without-match\0" as *const u8 as *const libc::c_char,
                ) == 0 as libc::c_int
                {
                    binary_files = WITHOUT_MATCH_BINARY_FILES;
                } else {
                    if ::std::mem::size_of::<C2RustUnnamed_22>() as libc::c_ulong != 0 {
                        error(
                            EXIT_TROUBLE as libc::c_int,
                            0 as libc::c_int,
                            dcgettext(
                                0 as *const libc::c_char,
                                b"unknown binary-files type\0" as *const u8
                                    as *const libc::c_char,
                                5 as libc::c_int,
                            ),
                        );
                        if 0 as libc::c_int != 0 {} else {
                            unreachable!();
                        };
                    } else {
                        error(
                            EXIT_TROUBLE as libc::c_int,
                            0 as libc::c_int,
                            dcgettext(
                                0 as *const libc::c_char,
                                b"unknown binary-files type\0" as *const u8
                                    as *const libc::c_char,
                                5 as libc::c_int,
                            ),
                        );
                        if 0 as libc::c_int != 0 {} else {
                            unreachable!();
                        };
                    };
                }
                current_block_134 = 4534765400774009001;
            }
            129 => {
                if !optarg.is_null() {
                    if c_strcasecmp(
                        optarg,
                        b"always\0" as *const u8 as *const libc::c_char,
                    ) == 0
                        || c_strcasecmp(
                            optarg,
                            b"yes\0" as *const u8 as *const libc::c_char,
                        ) == 0
                        || c_strcasecmp(
                            optarg,
                            b"force\0" as *const u8 as *const libc::c_char,
                        ) == 0
                    {
                        color_option = 1 as libc::c_int;
                    } else if c_strcasecmp(
                        optarg,
                        b"never\0" as *const u8 as *const libc::c_char,
                    ) == 0
                        || c_strcasecmp(
                            optarg,
                            b"no\0" as *const u8 as *const libc::c_char,
                        ) == 0
                        || c_strcasecmp(
                            optarg,
                            b"none\0" as *const u8 as *const libc::c_char,
                        ) == 0
                    {
                        color_option = 0 as libc::c_int;
                    } else if c_strcasecmp(
                        optarg,
                        b"auto\0" as *const u8 as *const libc::c_char,
                    ) == 0
                        || c_strcasecmp(
                            optarg,
                            b"tty\0" as *const u8 as *const libc::c_char,
                        ) == 0
                        || c_strcasecmp(
                            optarg,
                            b"if-tty\0" as *const u8 as *const libc::c_char,
                        ) == 0
                    {
                        color_option = 2 as libc::c_int;
                    } else {
                        show_help = 1 as libc::c_int;
                    }
                } else {
                    color_option = 2 as libc::c_int;
                }
                current_block_134 = 4534765400774009001;
            }
            131 | 134 => {
                let mut cmd: libc::c_int = 0 as libc::c_int;
                while cmd < 2 as libc::c_int {
                    if (excluded_patterns[cmd as usize]).is_null() {
                        excluded_patterns[cmd as usize] = new_exclude();
                    }
                    add_exclude(
                        excluded_patterns[cmd as usize],
                        optarg,
                        (if opt == INCLUDE_OPTION as libc::c_int {
                            (1 as libc::c_int) << 29 as libc::c_int
                        } else {
                            0 as libc::c_int
                        }) | exclude_options(cmd != 0),
                    );
                    cmd += 1;
                    cmd;
                }
                current_block_134 = 4534765400774009001;
            }
            132 => {
                let mut cmd_0: libc::c_int = 0 as libc::c_int;
                while cmd_0 < 2 as libc::c_int {
                    if (excluded_patterns[cmd_0 as usize]).is_null() {
                        excluded_patterns[cmd_0 as usize] = new_exclude();
                    }
                    if add_exclude_file(
                        Some(
                            add_exclude
                                as unsafe extern "C" fn(
                                    *mut exclude,
                                    *const libc::c_char,
                                    libc::c_int,
                                ) -> (),
                        ),
                        excluded_patterns[cmd_0 as usize],
                        optarg,
                        exclude_options(cmd_0 != 0),
                        '\n' as i32 as libc::c_char,
                    ) != 0 as libc::c_int
                    {
                        if ::std::mem::size_of::<C2RustUnnamed_21>() as libc::c_ulong
                            != 0
                        {
                            error(
                                EXIT_TROUBLE as libc::c_int,
                                *__errno_location(),
                                b"%s\0" as *const u8 as *const libc::c_char,
                                optarg,
                            );
                            if 0 as libc::c_int != 0 {} else {
                                unreachable!();
                            };
                        } else {
                            error(
                                EXIT_TROUBLE as libc::c_int,
                                *__errno_location(),
                                b"%s\0" as *const u8 as *const libc::c_char,
                                optarg,
                            );
                            if 0 as libc::c_int != 0 {} else {
                                unreachable!();
                            };
                        };
                    }
                    cmd_0 += 1;
                    cmd_0;
                }
                current_block_134 = 4534765400774009001;
            }
            130 => {
                strip_trailing_slashes(optarg);
                let mut cmd_1: libc::c_int = 0 as libc::c_int;
                while cmd_1 < 2 as libc::c_int {
                    if (excluded_directory_patterns[cmd_1 as usize]).is_null() {
                        excluded_directory_patterns[cmd_1 as usize] = new_exclude();
                    }
                    add_exclude(
                        excluded_directory_patterns[cmd_1 as usize],
                        optarg,
                        exclude_options(cmd_1 != 0),
                    );
                    cmd_1 += 1;
                    cmd_1;
                }
                current_block_134 = 4534765400774009001;
            }
            133 => {
                group_separator = optarg;
                current_block_134 = 4534765400774009001;
            }
            135 => {
                line_buffered = 1 as libc::c_int != 0;
                current_block_134 = 4534765400774009001;
            }
            136 => {
                label = optarg;
                current_block_134 = 4534765400774009001;
            }
            85 | 0 => {
                current_block_134 = 4534765400774009001;
            }
            _ => {
                usage(EXIT_TROUBLE as libc::c_int);
                current_block_134 = 4534765400774009001;
            }
        }
        match current_block_134 {
            17041416599325743607 => {
                directories = RECURSE_DIRECTORIES;
                last_recursive = prev_optind;
            }
            _ => {}
        }
    }
    if show_version {
        version_etc(
            stdout,
            getprogname(),
            b"GNU grep\0" as *const u8 as *const libc::c_char,
            b"3.11\0" as *const u8 as *const libc::c_char,
            0 as *mut libc::c_void as *mut libc::c_char,
        );
        puts(
            dcgettext(
                0 as *const libc::c_char,
                b"Written by Mike Haertel and others; see\n<https://git.savannah.gnu.org/cgit/grep.git/tree/AUTHORS>.\0"
                    as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ),
        );
        return 0 as libc::c_int;
    }
    if show_help != 0 {
        usage(0 as libc::c_int);
    }
    if !keys.is_null() {
        if keycc == 0 as libc::c_int as libc::c_long {
            out_invert = (out_invert as libc::c_int ^ 1 as libc::c_int) != 0;
            match_words = 0 as libc::c_int != 0;
            match_lines = match_words;
            let fresh18 = keycc;
            keycc = keycc + 1;
            *keys.offset(fresh18 as isize) = '\n' as i32 as libc::c_char;
        }
    } else if optind < argc {
        let fresh19 = optind;
        optind = optind + 1;
        let mut pat: *const libc::c_char = *argv.offset(fresh19 as isize);
        let mut skip_bs: bool = matcher != F_MATCHER_INDEX as libc::c_int
            && *pat.offset(0 as libc::c_int as isize) as libc::c_int == '\\' as i32
            && *pat.offset(1 as libc::c_int as isize) as libc::c_int == '-' as i32;
        keys = xstrdup(pat.offset(skip_bs as libc::c_int as isize));
        pattern_array = keys;
        let mut patlen: idx_t = strlen(keys) as idx_t;
        *keys.offset(patlen as isize) = '\n' as i32 as libc::c_char;
        keycc = update_patterns(
            keys,
            0 as libc::c_int as idx_t,
            patlen + 1 as libc::c_int as libc::c_long,
            b"\0" as *const u8 as *const libc::c_char,
        );
    } else {
        usage(EXIT_TROUBLE as libc::c_int);
    }
    keycc -= 1;
    keycc;
    hash_free(pattern_table);
    let mut possibly_tty: bool = 0 as libc::c_int != 0;
    let mut tmp_stat: stat = stat {
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
    if !exit_on_match && fstat(1 as libc::c_int, &mut tmp_stat) == 0 as libc::c_int {
        if tmp_stat.st_mode & 0o170000 as libc::c_int as libc::c_uint
            == 0o100000 as libc::c_int as libc::c_uint
        {
            out_stat = tmp_stat;
        } else if tmp_stat.st_mode & 0o170000 as libc::c_int as libc::c_uint
            == 0o20000 as libc::c_int as libc::c_uint
        {
            let mut null_stat: stat = stat {
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
            if stat(b"/dev/null\0" as *const u8 as *const libc::c_char, &mut null_stat)
                == 0 as libc::c_int
                && (tmp_stat.st_ino == null_stat.st_ino
                    && tmp_stat.st_dev == null_stat.st_dev)
            {
                dev_null_output = 1 as libc::c_int != 0;
            } else {
                possibly_tty = 1 as libc::c_int != 0;
            }
        }
    }
    if exit_on_match as libc::c_int | dev_null_output as libc::c_int != 0 {
        list_files = LISTFILES_NONE;
    }
    if exit_on_match as libc::c_int | dev_null_output as libc::c_int != 0
        || list_files as libc::c_uint != LISTFILES_NONE as libc::c_int as libc::c_uint
    {
        count_matches = 0 as libc::c_int != 0;
        done_on_match = 1 as libc::c_int != 0;
    }
    out_quiet = count_matches as libc::c_int | done_on_match as libc::c_int != 0;
    if out_after < 0 as libc::c_int as libc::c_long {
        out_after = default_context;
    }
    if out_before < 0 as libc::c_int as libc::c_long {
        out_before = default_context;
    }
    if (max_count == 0 as libc::c_int as libc::c_long
        || keycc == 0 as libc::c_int as libc::c_long && out_invert as libc::c_int != 0
            && !match_lines && !match_words)
        && list_files as libc::c_uint
            != LISTFILES_NONMATCHING as libc::c_int as libc::c_uint
    {
        return 1 as libc::c_int;
    }
    if color_option == 2 as libc::c_int {
        color_option = (possibly_tty as libc::c_int != 0 && should_colorize() != 0
            && isatty(1 as libc::c_int) != 0) as libc::c_int;
    }
    init_colorize();
    if color_option != 0 {
        let mut userval: *mut libc::c_char = getenv(
            b"GREP_COLOR\0" as *const u8 as *const libc::c_char,
        );
        if !userval.is_null() && *userval as libc::c_int != '\0' as i32 {
            let mut q: *mut libc::c_char = userval;
            while *q as libc::c_int == ';' as i32
                || c_isdigit(*q as libc::c_int) as libc::c_int != 0
            {
                if *q.offset(1 as libc::c_int as isize) == 0 {
                    context_match_color = userval;
                    selected_match_color = context_match_color;
                    break;
                } else {
                    q = q.offset(1);
                    q;
                }
            }
        }
        parse_grep_colors();
        if selected_match_color == userval as *const libc::c_char
            || context_match_color == userval as *const libc::c_char
        {
            error(
                0 as libc::c_int,
                0 as libc::c_int,
                dcgettext(
                    0 as *const libc::c_char,
                    b"warning: GREP_COLOR='%s' is deprecated; use GREP_COLORS='mt=%s'\0"
                        as *const u8 as *const libc::c_char,
                    5 as libc::c_int,
                ),
                userval,
                userval,
            );
        }
    }
    initialize_unibyte_mask();
    if matcher < 0 as libc::c_int {
        matcher = G_MATCHER_INDEX as libc::c_int;
    }
    if matcher == F_MATCHER_INDEX as libc::c_int
        || matcher == E_MATCHER_INDEX as libc::c_int
        || matcher == G_MATCHER_INDEX as libc::c_int
    {
        if match_icase {
            setup_ok_fold();
        }
        if matcher == F_MATCHER_INDEX as libc::c_int {
            if if !localeinfo.multibyte {
                (n_patterns == 1 as libc::c_int as libc::c_long
                    && match_words as libc::c_int != 0) as libc::c_int
            } else {
                (contains_encoding_error(keys, keycc) as libc::c_int != 0
                    || match_icase as libc::c_int != 0
                        && !fgrep_icase_available(keys, keycc)) as libc::c_int
            } != 0
            {
                fgrep_to_grep_pattern(&mut pattern_array, &mut keycc);
                keys = pattern_array;
                matcher = G_MATCHER_INDEX as libc::c_int;
            }
        } else if (1 as libc::c_int as libc::c_long) < n_patterns {
            matcher = try_fgrep_pattern(matcher, keys, &mut keycc);
        }
    }
    execute = matchers[matcher as usize].execute;
    compiled_pattern = (matchers[matcher as usize].compile)
        .unwrap()(
        keys,
        keycc,
        matchers[matcher as usize].syntax as reg_syntax_t,
        only_matching as libc::c_int | color_option != 0,
    );
    let mut eolbytes: [libc::c_char; 3] = [
        0 as libc::c_int as libc::c_char,
        eolbyte,
        0 as libc::c_int as libc::c_char,
    ];
    let mut match_size: idx_t = 0;
    skip_empty_lines = (execute
        .unwrap()(
        compiled_pattern,
        eolbytes.as_mut_ptr().offset(1 as libc::c_int as isize),
        1 as libc::c_int as idx_t,
        &mut match_size,
        0 as *const libc::c_char,
    ) == 0 as libc::c_int as libc::c_long) as libc::c_int == out_invert as libc::c_int;
    let mut num_operands: libc::c_int = argc - optind;
    out_file = if filename_option == 0 as libc::c_int && num_operands <= 1 as libc::c_int
    {
        -((directories as libc::c_uint
            == RECURSE_DIRECTORIES as libc::c_int as libc::c_uint) as libc::c_int)
    } else {
        (0 as libc::c_int <= filename_option) as libc::c_int
    };
    if binary {
        xset_binary_mode(1 as libc::c_int, 0 as libc::c_int);
    }
    let mut psize: libc::c_long = sysconf(_SC_PAGESIZE as libc::c_int);
    if !((0 as libc::c_int as libc::c_long) < psize
        && psize
            <= (9223372036854775807 as libc::c_long
                - uword_size as libc::c_int as libc::c_long)
                / 2 as libc::c_int as libc::c_long)
    {
        abort();
    }
    pagesize = psize;
    bufalloc = (if (INITIAL_BUFSIZE as libc::c_int as uintptr_t)
        .wrapping_rem(pagesize as libc::c_ulong) == 0 as libc::c_int as libc::c_ulong
    {
        INITIAL_BUFSIZE as libc::c_int as libc::c_ulong
    } else {
        (INITIAL_BUFSIZE as libc::c_int as libc::c_ulong)
            .wrapping_add(
                (pagesize as libc::c_ulong)
                    .wrapping_sub(
                        (INITIAL_BUFSIZE as libc::c_int as uintptr_t)
                            .wrapping_rem(pagesize as libc::c_ulong),
                    ),
            )
    })
        .wrapping_add(pagesize as libc::c_ulong)
        .wrapping_add(uword_size as libc::c_int as libc::c_ulong) as idx_t;
    buffer = ximalloc(bufalloc) as *mut libc::c_char;
    if fts_options & 0x2 as libc::c_int != 0
        && devices as libc::c_uint
            == READ_COMMAND_LINE_DEVICES as libc::c_int as libc::c_uint
    {
        devices = READ_DEVICES;
    }
    let mut files: *const *mut libc::c_char = 0 as *const *mut libc::c_char;
    if (0 as libc::c_int) < num_operands {
        files = argv.offset(optind as isize);
    } else if directories as libc::c_uint
        == RECURSE_DIRECTORIES as libc::c_int as libc::c_uint
        && (0 as libc::c_int) < last_recursive
    {
        static mut cwd_only: [*mut libc::c_char; 2] = [
            b".\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            0 as *const libc::c_char as *mut libc::c_char,
        ];
        files = cwd_only.as_ptr();
        omit_dot_slash = 1 as libc::c_int != 0;
    } else {
        static mut stdin_only: [*mut libc::c_char; 2] = [
            b"-\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            0 as *const libc::c_char as *mut libc::c_char,
        ];
        files = stdin_only.as_ptr();
    }
    let mut status: bool = 1 as libc::c_int != 0;
    loop {
        let fresh20 = files;
        files = files.offset(1);
        status = (status as libc::c_int & grep_command_line_arg(*fresh20) as libc::c_int) != 0;
        if (*files).is_null() {
            break;
        }
    }
    return if errseen as libc::c_int != 0 {
        EXIT_TROUBLE as libc::c_int
    } else {
        status as libc::c_int
    };
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
