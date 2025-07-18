use ::libc;
use ::c2rust_asm_casts;
use c2rust_asm_casts::AsmCastTrait;
use std::arch::asm;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    pub type sockaddr_x25;
    pub type sockaddr_ns;
    pub type sockaddr_iso;
    pub type sockaddr_ipx;
    pub type sockaddr_inarp;
    pub type sockaddr_eon;
    pub type sockaddr_dl;
    pub type sockaddr_ax25;
    pub type sockaddr_at;
    static mut stdin: *mut FILE;
    static mut stdout: *mut FILE;
    static mut stderr: *mut FILE;
    fn fclose(__stream: *mut FILE) -> libc::c_int;
    fn fflush(__stream: *mut FILE) -> libc::c_int;
    fn fopen(_: *const libc::c_char, _: *const libc::c_char) -> *mut FILE;
    fn fprintf(_: *mut FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn sprintf(_: *mut libc::c_char, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn vfprintf(
        _: *mut FILE,
        _: *const libc::c_char,
        _: ::std::ffi::VaList,
    ) -> libc::c_int;
    fn vsnprintf(
        _: *mut libc::c_char,
        _: libc::c_ulong,
        _: *const libc::c_char,
        _: ::std::ffi::VaList,
    ) -> libc::c_int;
    fn sscanf(_: *const libc::c_char, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn fgets(
        __s: *mut libc::c_char,
        __n: libc::c_int,
        __stream: *mut FILE,
    ) -> *mut libc::c_char;
    fn fread(
        _: *mut libc::c_void,
        _: libc::c_ulong,
        _: libc::c_ulong,
        _: *mut FILE,
    ) -> libc::c_ulong;
    fn fwrite(
        _: *const libc::c_void,
        _: libc::c_ulong,
        _: libc::c_ulong,
        _: *mut FILE,
    ) -> libc::c_ulong;
    fn feof(__stream: *mut FILE) -> libc::c_int;
    fn fileno(__stream: *mut FILE) -> libc::c_int;
    fn memcpy(
        _: *mut libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn memmove(
        _: *mut libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn strcpy(_: *mut libc::c_char, _: *const libc::c_char) -> *mut libc::c_char;
    fn strncpy(
        _: *mut libc::c_char,
        _: *const libc::c_char,
        _: libc::c_ulong,
    ) -> *mut libc::c_char;
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    fn strdup(_: *const libc::c_char) -> *mut libc::c_char;
    fn strchr(_: *const libc::c_char, _: libc::c_int) -> *mut libc::c_char;
    fn strrchr(_: *const libc::c_char, _: libc::c_int) -> *mut libc::c_char;
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    fn select(
        __nfds: libc::c_int,
        __readfds: *mut fd_set,
        __writefds: *mut fd_set,
        __exceptfds: *mut fd_set,
        __timeout: *mut timeval,
    ) -> libc::c_int;
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    fn realloc(_: *mut libc::c_void, _: libc::c_ulong) -> *mut libc::c_void;
    fn free(__ptr: *mut libc::c_void);
    fn abort() -> !;
    fn exit(_: libc::c_int) -> !;
    fn getenv(__name: *const libc::c_char) -> *mut libc::c_char;
    fn setenv(
        __name: *const libc::c_char,
        __value: *const libc::c_char,
        __replace: libc::c_int,
    ) -> libc::c_int;
    fn getloadavg(__loadavg: *mut libc::c_double, __nelem: libc::c_int) -> libc::c_int;
    fn close(__fd: libc::c_int) -> libc::c_int;
    fn read(__fd: libc::c_int, __buf: *mut libc::c_void, __nbytes: size_t) -> ssize_t;
    fn write(__fd: libc::c_int, __buf: *const libc::c_void, __n: size_t) -> ssize_t;
    fn pipe(__pipedes: *mut libc::c_int) -> libc::c_int;
    fn chdir(__path: *const libc::c_char) -> libc::c_int;
    fn dup2(__fd: libc::c_int, __fd2: libc::c_int) -> libc::c_int;
    fn execv(
        __path: *const libc::c_char,
        __argv: *const *mut libc::c_char,
    ) -> libc::c_int;
    fn execvp(
        __file: *const libc::c_char,
        __argv: *const *mut libc::c_char,
    ) -> libc::c_int;
    fn sysconf(__name: libc::c_int) -> libc::c_long;
    fn getpid() -> __pid_t;
    fn fork() -> __pid_t;
    fn isatty(__fd: libc::c_int) -> libc::c_int;
    fn unlink(__name: *const libc::c_char) -> libc::c_int;
    fn getpagesize() -> libc::c_int;
    fn open(__file: *const libc::c_char, __oflag: libc::c_int, _: ...) -> libc::c_int;
    fn time(__timer: *mut time_t) -> time_t;
    fn __xstat(
        __ver: libc::c_int,
        __filename: *const libc::c_char,
        __stat_buf: *mut stat,
    ) -> libc::c_int;
    fn waitpid(
        __pid: __pid_t,
        __stat_loc: *mut libc::c_int,
        __options: libc::c_int,
    ) -> __pid_t;
    fn sigemptyset(__set: *mut sigset_t) -> libc::c_int;
    fn sigaction(
        __sig: libc::c_int,
        __act: *const sigaction,
        __oact: *mut sigaction,
    ) -> libc::c_int;
    fn signal(__sig: libc::c_int, __handler: __sighandler_t) -> __sighandler_t;
    fn kill(__pid: __pid_t, __sig: libc::c_int) -> libc::c_int;
    fn setpriority(
        __which: __priority_which_t,
        __who: id_t,
        __prio: libc::c_int,
    ) -> libc::c_int;
    fn socket(
        __domain: libc::c_int,
        __type: libc::c_int,
        __protocol: libc::c_int,
    ) -> libc::c_int;
    fn bind(
        __fd: libc::c_int,
        __addr: __CONST_SOCKADDR_ARG,
        __len: socklen_t,
    ) -> libc::c_int;
    fn connect(
        __fd: libc::c_int,
        __addr: __CONST_SOCKADDR_ARG,
        __len: socklen_t,
    ) -> libc::c_int;
    fn listen(__fd: libc::c_int, __n: libc::c_int) -> libc::c_int;
    fn accept(
        __fd: libc::c_int,
        __addr: __SOCKADDR_ARG,
        __addr_len: *mut socklen_t,
    ) -> libc::c_int;
    fn gethostbyname(__name: *const libc::c_char) -> *mut hostent;
    fn __ctype_b_loc() -> *mut *const libc::c_ushort;
    fn __errno_location() -> *mut libc::c_int;
    fn dcgettext(
        __domainname: *const libc::c_char,
        __msgid: *const libc::c_char,
        __category: libc::c_int,
    ) -> *mut libc::c_char;
    fn remove_newlines_and_comments(buff: *mut libc::c_char);
    fn tokenize_spaces_dyn(buff: *mut libc::c_char) -> *mut *mut libc::c_char;
    fn tokenize_char_dyn(
        buff: *mut libc::c_char,
        tchar: libc::c_int,
    ) -> *mut *mut libc::c_char;
    fn freadline(fr: *mut FILE) -> *mut libc::c_char;
    fn numhash_init(
        nt: *mut numhashtable,
        bitsize: libc::c_int,
        depth: libc::c_int,
    ) -> libc::c_int;
    fn numhash_add(
        nt: *mut numhashtable,
        key: libc::c_int,
        data: *mut libc::c_void,
    ) -> libc::c_int;
    fn numhash_remove(nt: *mut numhashtable, key: libc::c_int) -> libc::c_int;
    fn numhash_get_smallest_free(nt: *mut numhashtable) -> libc::c_int;
    fn numhash_free(nt: *mut numhashtable) -> libc::c_int;
    fn format_check_if_formatted(
        format: *mut libc::c_char,
        fchars: *mut libc::c_char,
    ) -> libc::c_int;
    fn format_replace(
        format: *mut libc::c_char,
        is_escape: libc::c_int,
        _: ...
    ) -> *mut libc::c_char;
    fn strappendf(
        string: *mut *mut libc::c_char,
        format: *mut libc::c_char,
        _: ...
    ) -> libc::c_int;
    fn vstrappendf(
        string: *mut *mut libc::c_char,
        format: *mut libc::c_char,
        ap: ::std::ffi::VaList,
    ) -> libc::c_int;
    fn linebuffer_reset(lb: *mut linebuffer) -> libc::c_int;
    fn linebuffer_free(lb: *mut linebuffer) -> libc::c_int;
    fn linebuffer_concatenate(
        lb: *mut linebuffer,
        buff: *mut libc::c_char,
        size: size_t,
    ) -> libc::c_int;
    fn linebuffer_read_line(
        fd: libc::c_int,
        lb: *mut linebuffer,
        timeout: libc::c_int,
    ) -> *mut libc::c_char;
    fn linebuffer_fetch(lb: *mut linebuffer) -> *mut libc::c_char;
    fn linebuffer_flush(lb: *mut linebuffer) -> *mut libc::c_char;
    fn longhelp_fprint(
        fw: *mut FILE,
        entries: *mut longhelp_entry,
        flags: libc::c_int,
        width: libc::c_int,
    ) -> libc::c_int;
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
pub type __uint8_t = libc::c_uchar;
pub type __uint16_t = libc::c_ushort;
pub type __uint32_t = libc::c_uint;
pub type __dev_t = libc::c_ulong;
pub type __uid_t = libc::c_uint;
pub type __gid_t = libc::c_uint;
pub type __ino_t = libc::c_ulong;
pub type __mode_t = libc::c_uint;
pub type __nlink_t = libc::c_ulong;
pub type __off_t = libc::c_long;
pub type __off64_t = libc::c_long;
pub type __pid_t = libc::c_int;
pub type __clock_t = libc::c_long;
pub type __id_t = libc::c_uint;
pub type __time_t = libc::c_long;
pub type __suseconds_t = libc::c_long;
pub type __blksize_t = libc::c_long;
pub type __blkcnt_t = libc::c_long;
pub type __ssize_t = libc::c_long;
pub type __syscall_slong_t = libc::c_long;
pub type __socklen_t = libc::c_uint;
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
pub type id_t = __id_t;
pub type time_t = __time_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __sigset_t {
    pub __val: [libc::c_ulong; 16],
}
pub type sigset_t = __sigset_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct timeval {
    pub tv_sec: __time_t,
    pub tv_usec: __suseconds_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct timespec {
    pub tv_sec: __time_t,
    pub tv_nsec: __syscall_slong_t,
}
pub type __fd_mask = libc::c_long;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct fd_set {
    pub fds_bits: [__fd_mask; 16],
}
pub type socklen_t = __socklen_t;
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
pub type __sigval_t = sigval;
#[derive(Copy, Clone)]
#[repr(C)]
pub union sigval {
    pub sival_int: libc::c_int,
    pub sival_ptr: *mut libc::c_void,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct siginfo_t {
    pub si_signo: libc::c_int,
    pub si_errno: libc::c_int,
    pub si_code: libc::c_int,
    pub __pad0: libc::c_int,
    pub _sifields: C2RustUnnamed_0,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_0 {
    pub _pad: [libc::c_int; 28],
    pub _kill: C2RustUnnamed_9,
    pub _timer: C2RustUnnamed_8,
    pub _rt: C2RustUnnamed_7,
    pub _sigchld: C2RustUnnamed_6,
    pub _sigfault: C2RustUnnamed_3,
    pub _sigpoll: C2RustUnnamed_2,
    pub _sigsys: C2RustUnnamed_1,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_1 {
    pub _call_addr: *mut libc::c_void,
    pub _syscall: libc::c_int,
    pub _arch: libc::c_uint,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_2 {
    pub si_band: libc::c_long,
    pub si_fd: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_3 {
    pub si_addr: *mut libc::c_void,
    pub si_addr_lsb: libc::c_short,
    pub _bounds: C2RustUnnamed_4,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_4 {
    pub _addr_bnd: C2RustUnnamed_5,
    pub _pkey: __uint32_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_5 {
    pub _lower: *mut libc::c_void,
    pub _upper: *mut libc::c_void,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_6 {
    pub si_pid: __pid_t,
    pub si_uid: __uid_t,
    pub si_status: libc::c_int,
    pub si_utime: __clock_t,
    pub si_stime: __clock_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_7 {
    pub si_pid: __pid_t,
    pub si_uid: __uid_t,
    pub si_sigval: __sigval_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_8 {
    pub si_tid: libc::c_int,
    pub si_overrun: libc::c_int,
    pub si_sigval: __sigval_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_9 {
    pub si_pid: __pid_t,
    pub si_uid: __uid_t,
}
pub type __sighandler_t = Option::<unsafe extern "C" fn(libc::c_int) -> ()>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sigaction {
    pub __sigaction_handler: C2RustUnnamed_10,
    pub sa_mask: __sigset_t,
    pub sa_flags: libc::c_int,
    pub sa_restorer: Option::<unsafe extern "C" fn() -> ()>,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_10 {
    pub sa_handler: __sighandler_t,
    pub sa_sigaction: Option::<
        unsafe extern "C" fn(libc::c_int, *mut siginfo_t, *mut libc::c_void) -> (),
    >,
}
pub type __priority_which = libc::c_uint;
pub const PRIO_USER: __priority_which = 2;
pub const PRIO_PGRP: __priority_which = 1;
pub const PRIO_PROCESS: __priority_which = 0;
pub type __priority_which_t = __priority_which;
pub type __socket_type = libc::c_uint;
pub const SOCK_NONBLOCK: __socket_type = 2048;
pub const SOCK_CLOEXEC: __socket_type = 524288;
pub const SOCK_PACKET: __socket_type = 10;
pub const SOCK_DCCP: __socket_type = 6;
pub const SOCK_SEQPACKET: __socket_type = 5;
pub const SOCK_RDM: __socket_type = 4;
pub const SOCK_RAW: __socket_type = 3;
pub const SOCK_DGRAM: __socket_type = 2;
pub const SOCK_STREAM: __socket_type = 1;
pub type sa_family_t = libc::c_ushort;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sockaddr {
    pub sa_family: sa_family_t,
    pub sa_data: [libc::c_char; 14],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union __SOCKADDR_ARG {
    pub __sockaddr__: *mut sockaddr,
    pub __sockaddr_at__: *mut sockaddr_at,
    pub __sockaddr_ax25__: *mut sockaddr_ax25,
    pub __sockaddr_dl__: *mut sockaddr_dl,
    pub __sockaddr_eon__: *mut sockaddr_eon,
    pub __sockaddr_in__: *mut sockaddr_in,
    pub __sockaddr_in6__: *mut sockaddr_in6,
    pub __sockaddr_inarp__: *mut sockaddr_inarp,
    pub __sockaddr_ipx__: *mut sockaddr_ipx,
    pub __sockaddr_iso__: *mut sockaddr_iso,
    pub __sockaddr_ns__: *mut sockaddr_ns,
    pub __sockaddr_un__: *mut sockaddr_un,
    pub __sockaddr_x25__: *mut sockaddr_x25,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sockaddr_un {
    pub sun_family: sa_family_t,
    pub sun_path: [libc::c_char; 108],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sockaddr_in6 {
    pub sin6_family: sa_family_t,
    pub sin6_port: in_port_t,
    pub sin6_flowinfo: uint32_t,
    pub sin6_addr: in6_addr,
    pub sin6_scope_id: uint32_t,
}
pub type uint32_t = __uint32_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct in6_addr {
    pub __in6_u: C2RustUnnamed_11,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_11 {
    pub __u6_addr8: [uint8_t; 16],
    pub __u6_addr16: [uint16_t; 8],
    pub __u6_addr32: [uint32_t; 4],
}
pub type uint16_t = __uint16_t;
pub type uint8_t = __uint8_t;
pub type in_port_t = uint16_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sockaddr_in {
    pub sin_family: sa_family_t,
    pub sin_port: in_port_t,
    pub sin_addr: in_addr,
    pub sin_zero: [libc::c_uchar; 8],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct in_addr {
    pub s_addr: in_addr_t,
}
pub type in_addr_t = uint32_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub union __CONST_SOCKADDR_ARG {
    pub __sockaddr__: *const sockaddr,
    pub __sockaddr_at__: *const sockaddr_at,
    pub __sockaddr_ax25__: *const sockaddr_ax25,
    pub __sockaddr_dl__: *const sockaddr_dl,
    pub __sockaddr_eon__: *const sockaddr_eon,
    pub __sockaddr_in__: *const sockaddr_in,
    pub __sockaddr_in6__: *const sockaddr_in6,
    pub __sockaddr_inarp__: *const sockaddr_inarp,
    pub __sockaddr_ipx__: *const sockaddr_ipx,
    pub __sockaddr_iso__: *const sockaddr_iso,
    pub __sockaddr_ns__: *const sockaddr_ns,
    pub __sockaddr_un__: *const sockaddr_un,
    pub __sockaddr_x25__: *const sockaddr_x25,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct hostent {
    pub h_name: *mut libc::c_char,
    pub h_aliases: *mut *mut libc::c_char,
    pub h_addrtype: libc::c_int,
    pub h_length: libc::c_int,
    pub h_addr_list: *mut *mut libc::c_char,
}
pub type C2RustUnnamed_12 = libc::c_uint;
pub const _ISalnum: C2RustUnnamed_12 = 8;
pub const _ISpunct: C2RustUnnamed_12 = 4;
pub const _IScntrl: C2RustUnnamed_12 = 2;
pub const _ISblank: C2RustUnnamed_12 = 1;
pub const _ISgraph: C2RustUnnamed_12 = 32768;
pub const _ISprint: C2RustUnnamed_12 = 16384;
pub const _ISspace: C2RustUnnamed_12 = 8192;
pub const _ISxdigit: C2RustUnnamed_12 = 4096;
pub const _ISdigit: C2RustUnnamed_12 = 2048;
pub const _ISalpha: C2RustUnnamed_12 = 1024;
pub const _ISlower: C2RustUnnamed_12 = 512;
pub const _ISupper: C2RustUnnamed_12 = 256;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct numhashnode {
    pub node: C2RustUnnamed_13,
    pub nchild: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_13 {
    pub leaves: *mut numhashnode,
    pub data: *mut libc::c_void,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct numhashtable {
    pub table: numhashnode,
    pub depth: libc::c_int,
    pub bitsize: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct linebuffer {
    pub buffer: *mut libc::c_char,
    pub length: size_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct longhelp_entry {
    pub options: *mut libc::c_char,
    pub description: *mut libc::c_char,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct child {
    pub prev: *mut child,
    pub next: *mut child,
    pub id: libc::c_int,
    pub pid: libc::c_int,
    pub rs: *mut remoteshell,
    pub fdstdout: libc::c_int,
    pub lout: linebuffer,
    pub fdstderr: libc::c_int,
    pub lerr: linebuffer,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct remoteshell {
    pub pid: libc::c_int,
    pub fhsend: libc::c_int,
    pub fhrecv: libc::c_int,
    pub num_processes: libc::c_int,
    pub achild: libc::c_int,
    pub estatus: libc::c_int,
    pub lrsh: linebuffer,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct command {
    pub is_shell: libc::c_int,
    pub argv: *mut *mut libc::c_char,
    pub argc: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct remotehost {
    pub hostspec: *mut libc::c_char,
    pub num_processes: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct parameter {
    pub name: *mut libc::c_char,
    pub no_touch_std: libc::c_int,
    pub c: command,
    pub id: libc::c_int,
    pub status: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct client {
    pub prev: *mut client,
    pub next: *mut client,
    pub peer: libc::c_int,
    pub lcli: linebuffer,
    pub status: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct pendingclient {
    pub qid: libc::c_int,
    pub data: *mut libc::c_void,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct imutex {
    pub prev: *mut imutex,
    pub next: *mut imutex,
    pub name: *mut libc::c_char,
    pub state: libc::c_int,
    pub pclients: *mut pendingclient,
    pub npclient: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct dqueue {
    pub prev: *mut dqueue,
    pub next: *mut dqueue,
    pub qclient: *mut client,
    pub id: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct logdata {
    pub fwlog: *mut FILE,
    pub loglevel: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct paralleldata {
    pub in_0: *mut libc::c_char,
    pub out: *mut libc::c_char,
    pub err: *mut libc::c_char,
    pub fwout: *mut FILE,
    pub formatout: *mut libc::c_char,
    pub fwerr: *mut FILE,
    pub formaterr: *mut libc::c_char,
    pub omit_newlines: libc::c_int,
    pub envvarname: *mut libc::c_char,
    pub shell: *mut libc::c_char,
    pub rsh: *mut libc::c_char,
    pub rshcmd: *mut libc::c_char,
    pub rshargs: *mut *mut libc::c_char,
    pub fallback_to_die: libc::c_int,
    pub log: *mut logdata,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct parallelstatus {
    pub childlist: *mut child,
    pub achild: libc::c_int,
    pub sock: libc::c_int,
    pub hsck: libc::c_int,
    pub clientlist: *mut client,
    pub imutexlist: *mut imutex,
    pub dqueuelist: *mut dqueue,
    pub iqueue: libc::c_int,
    pub nfinished: libc::c_int,
    pub npending: libc::c_int,
    pub nparam: libc::c_int,
    pub t0: time_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct request {
    pub cl: *mut client,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct hypervisorstatus {
    pub clientlist: *mut client,
    pub num_processes: libc::c_int,
    pub use_load: libc::c_int,
    pub use_fifo: libc::c_int,
    pub requests: *mut request,
    pub nrequest: libc::c_int,
    pub nrunning: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct signalinfo {
    pub signal: libc::c_int,
    pub pid: libc::c_int,
    pub exitstatus: libc::c_int,
    pub exitsignal: libc::c_int,
}
#[inline]
unsafe extern "C" fn __bswap_16(mut __bsx: __uint16_t) -> __uint16_t {
    return (__bsx as libc::c_int >> 8 as libc::c_int & 0xff as libc::c_int
        | (__bsx as libc::c_int & 0xff as libc::c_int) << 8 as libc::c_int)
        as __uint16_t;
}
#[inline]
unsafe extern "C" fn __bswap_32(mut __bsx: __uint32_t) -> __uint32_t {
    return (__bsx & 0xff000000 as libc::c_uint) >> 24 as libc::c_int
        | (__bsx & 0xff0000 as libc::c_uint) >> 8 as libc::c_int
        | (__bsx & 0xff00 as libc::c_uint) << 8 as libc::c_int
        | (__bsx & 0xff as libc::c_uint) << 24 as libc::c_int;
}
#[inline]
unsafe extern "C" fn stat(
    mut __path: *const libc::c_char,
    mut __statbuf: *mut stat,
) -> libc::c_int {
    return __xstat(1 as libc::c_int, __path, __statbuf);
}
unsafe extern "C" fn xmalloc(mut size: size_t) -> *mut libc::c_void {
    let mut ret: *mut libc::c_void = 0 as *mut libc::c_void;
    ret = malloc(size);
    if ret.is_null() && 1 as libc::c_int > 0 as libc::c_int {
        fprintf(
            stderr,
            b"pexec.c: %s.\n\0" as *const u8 as *const libc::c_char,
            dcgettext(
                0 as *const libc::c_char,
                b"memory exhausted\0" as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ),
        );
        abort();
    }
    return ret;
}
unsafe extern "C" fn xrealloc(
    mut ptr: *mut libc::c_void,
    mut size: size_t,
) -> *mut libc::c_void {
    let mut ret: *mut libc::c_void = 0 as *mut libc::c_void;
    ret = realloc(ptr, size);
    if ret.is_null() && size > 0 as libc::c_int as libc::c_ulong {
        fprintf(
            stderr,
            b"pexec.c: %s.\n\0" as *const u8 as *const libc::c_char,
            dcgettext(
                0 as *const libc::c_char,
                b"memory exhausted\0" as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ),
        );
        abort();
    }
    return ret;
}
unsafe extern "C" fn xstrdup(mut s: *const libc::c_char) -> *mut libc::c_char {
    let mut ret: *mut libc::c_char = 0 as *mut libc::c_char;
    ret = strdup(s);
    if ret.is_null() && 1 as libc::c_int > 0 as libc::c_int {
        fprintf(
            stderr,
            b"pexec.c: %s.\n\0" as *const u8 as *const libc::c_char,
            dcgettext(
                0 as *const libc::c_char,
                b"memory exhausted\0" as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ),
        );
        abort();
    }
    return ret;
}
pub static mut progbasename: *mut libc::c_char = 0 as *const libc::c_char
    as *mut libc::c_char;
pub static mut sig_pipe: [libc::c_int; 2] = [0; 2];
pub static mut logmsg_submit_task: [*mut libc::c_char; 9] = [
    0 as *const libc::c_char as *mut libc::c_char,
    b"unable to open /dev/null special file for reading\0" as *const u8
        as *const libc::c_char as *mut libc::c_char,
    b"unable to redirect from input file: file cannot be opened\0" as *const u8
        as *const libc::c_char as *mut libc::c_char,
    b"unable to open /dev/null special file for writing\0" as *const u8
        as *const libc::c_char as *mut libc::c_char,
    b"unable to create internal pipes\0" as *const u8 as *const libc::c_char
        as *mut libc::c_char,
    b"unable to redirect to output file: file cannot be created\0" as *const u8
        as *const libc::c_char as *mut libc::c_char,
    b"unable to redirect to error file: file cannot be created\0" as *const u8
        as *const libc::c_char as *mut libc::c_char,
    b"unknown error\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
];
pub unsafe extern "C" fn log_message(
    mut log: *mut logdata,
    mut loglevel: libc::c_int,
    mut p: *mut parameter,
    mut msg: *mut libc::c_char,
    mut args: ...
) -> libc::c_int {
    let mut ap: ::std::ffi::VaListImpl;
    if ((*log).fwlog).is_null() || (*log).loglevel < loglevel || msg.is_null() {
        return 0 as libc::c_int;
    }
    if !p.is_null() {
        fprintf((*log).fwlog, b"[%s] \0" as *const u8 as *const libc::c_char, (*p).name);
    }
    ap = args.clone();
    vfprintf((*log).fwlog, msg, ap.as_va_list());
    fprintf((*log).fwlog, b"\n\0" as *const u8 as *const libc::c_char);
    fflush((*log).fwlog);
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn background(
    mut nochdir: libc::c_int,
    mut noclose: libc::c_int,
) -> libc::c_int {
    let mut child_pid: libc::c_int = 0;
    let mut nulldev_fd: libc::c_int = 0;
    child_pid = fork();
    if child_pid < 0 as libc::c_int {
        return -(1 as libc::c_int)
    } else if child_pid == 0 as libc::c_int {
        if nochdir == 0 {
            chdir(b"/\0" as *const u8 as *const libc::c_char);
        }
        if noclose == 0 {
            nulldev_fd = open(
                b"/dev/null\0" as *const u8 as *const libc::c_char,
                0 as libc::c_int,
            );
            if nulldev_fd >= 0 as libc::c_int {
                if isatty(0 as libc::c_int) != 0 {
                    close(0 as libc::c_int);
                    dup2(nulldev_fd, 0 as libc::c_int);
                }
                close(nulldev_fd);
            }
            nulldev_fd = open(
                b"/dev/null\0" as *const u8 as *const libc::c_char,
                0o1 as libc::c_int,
            );
            if nulldev_fd >= 0 as libc::c_int {
                if isatty(1 as libc::c_int) != 0 {
                    close(1 as libc::c_int);
                    dup2(nulldev_fd, 1 as libc::c_int);
                }
                if isatty(2 as libc::c_int) != 0 {
                    close(2 as libc::c_int);
                    dup2(nulldev_fd, 2 as libc::c_int);
                }
                close(nulldev_fd);
            }
        }
    } else {
        exit(0 as libc::c_int);
    }
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn sig_act_child(mut signum: libc::c_int) {
    let mut pid: libc::c_int = 0;
    let mut ret: libc::c_int = 0;
    let mut status: libc::c_int = 0;
    let mut sci: signalinfo = signalinfo {
        signal: 0,
        pid: 0,
        exitstatus: 0,
        exitsignal: 0,
    };
    if sig_pipe[1 as libc::c_int as usize] < 0 as libc::c_int {
        return;
    }
    loop {
        pid = waitpid(-(1 as libc::c_int), &mut status, 1 as libc::c_int);
        if !(pid > 0 as libc::c_int) {
            break;
        }
        sci.signal = 17 as libc::c_int;
        sci.pid = pid;
        sci.exitstatus = (status & 0xff00 as libc::c_int) >> 8 as libc::c_int;
        if ((status & 0x7f as libc::c_int) + 1 as libc::c_int) as libc::c_schar
            as libc::c_int >> 1 as libc::c_int > 0 as libc::c_int
        {
            sci.exitsignal = status & 0x7f as libc::c_int;
        } else {
            sci.exitsignal = -(1 as libc::c_int);
        }
        ret = write(
            sig_pipe[1 as libc::c_int as usize],
            &mut sci as *mut signalinfo as *const libc::c_void,
            ::std::mem::size_of::<signalinfo>() as libc::c_ulong,
        ) as libc::c_int;
    };
}
pub unsafe extern "C" fn sig_act_interrupt(mut signum: libc::c_int) {
    let mut sci: signalinfo = signalinfo {
        signal: 0,
        pid: 0,
        exitstatus: 0,
        exitsignal: 0,
    };
    let mut ret: libc::c_int = 0;
    if sig_pipe[1 as libc::c_int as usize] < 0 as libc::c_int {
        return;
    }
    sci.signal = signum;
    sci.pid = 0 as libc::c_int;
    sci.exitstatus = 0 as libc::c_int;
    sci.exitsignal = (0 as libc::c_int == 0) as libc::c_int;
    ret = write(
        sig_pipe[1 as libc::c_int as usize],
        &mut sci as *mut signalinfo as *const libc::c_void,
        ::std::mem::size_of::<signalinfo>() as libc::c_ulong,
    ) as libc::c_int;
}
pub unsafe extern "C" fn fdwait(
    mut fd: libc::c_int,
    mut timeout: libc::c_int,
) -> libc::c_int {
    let mut set: fd_set = fd_set { fds_bits: [0; 16] };
    let mut tv: timeval = timeval { tv_sec: 0, tv_usec: 0 };
    let mut ret: libc::c_int = 0;
    let mut __d0: libc::c_int = 0;
    let mut __d1: libc::c_int = 0;
    let fresh0 = &mut __d0;
    let fresh1;
    let fresh2 = (::std::mem::size_of::<fd_set>() as libc::c_ulong)
        .wrapping_div(::std::mem::size_of::<__fd_mask>() as libc::c_ulong);
    let fresh3 = &mut __d1;
    let fresh4;
    let fresh5 = &mut *(set.fds_bits).as_mut_ptr().offset(0 as libc::c_int as isize)
        as *mut __fd_mask;
    asm!(
        "cld; rep; stosq", inlateout("cx") c2rust_asm_casts::AsmCast::cast_in(fresh0,
        fresh2) => fresh1, inlateout("di") c2rust_asm_casts::AsmCast::cast_in(fresh3,
        fresh5) => fresh4, inlateout("ax") 0 as libc::c_int => _,
        options(preserves_flags, att_syntax)
    );
    c2rust_asm_casts::AsmCast::cast_out(fresh0, fresh2, fresh1);
    c2rust_asm_casts::AsmCast::cast_out(fresh3, fresh5, fresh4);
    set
        .fds_bits[(fd
        / (8 as libc::c_int
            * ::std::mem::size_of::<__fd_mask>() as libc::c_ulong as libc::c_int))
        as usize]
        |= ((1 as libc::c_ulong)
            << fd
                % (8 as libc::c_int
                    * ::std::mem::size_of::<__fd_mask>() as libc::c_ulong
                        as libc::c_int)) as __fd_mask;
    if timeout > 0 as libc::c_int {
        tv.tv_sec = timeout as __time_t;
        tv.tv_usec = 0 as libc::c_int as __suseconds_t;
        ret = select(
            fd + 1 as libc::c_int,
            &mut set,
            0 as *mut fd_set,
            0 as *mut fd_set,
            &mut tv,
        );
    } else {
        ret = select(
            fd + 1 as libc::c_int,
            &mut set,
            0 as *mut fd_set,
            0 as *mut fd_set,
            0 as *mut timeval,
        );
    }
    return ret;
}
pub unsafe extern "C" fn env_export(
    mut name: *mut libc::c_char,
    mut value: *mut libc::c_char,
) -> libc::c_int {
    let mut ret: libc::c_int = 0;
    ret = setenv(name, value, (0 as libc::c_int == 0) as libc::c_int);
    return ret;
}
pub unsafe extern "C" fn is_nasty_char(mut c: libc::c_int) -> libc::c_int {
    if *(*__ctype_b_loc()).offset(c as isize) as libc::c_int
        & _ISalnum as libc::c_int as libc::c_ushort as libc::c_int != 0
    {
        return 0 as libc::c_int
    } else if c == '-' as i32 || c == '+' as i32 || c == '@' as i32 || c == '^' as i32 {
        return 0 as libc::c_int
    } else if c == '_' as i32 || c == '=' as i32 || c == ':' as i32 || c == '/' as i32 {
        return 0 as libc::c_int
    } else if c == ',' as i32 || c == '.' as i32 {
        return 0 as libc::c_int
    } else {
        return 1 as libc::c_int
    };
}
pub unsafe extern "C" fn get_nasty_char_number(
    mut arg: *mut libc::c_char,
) -> libc::c_int {
    let mut n: libc::c_int = 0;
    n = 0 as libc::c_int;
    while *arg != 0 {
        if is_nasty_char(*arg as libc::c_int) != 0 {
            n += 1;
            n;
        }
        arg = arg.offset(1);
        arg;
    }
    return n;
}
pub unsafe extern "C" fn concatenate_arguments(
    mut argc: libc::c_int,
    mut argv: *mut *mut libc::c_char,
) -> *mut libc::c_char {
    let mut command: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut len: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    let mut l: libc::c_int = 0;
    len = 0 as libc::c_int;
    command = 0 as *mut libc::c_char;
    i = 0 as libc::c_int;
    while i < argc {
        l = strlen(*argv.offset(i as isize)) as libc::c_int;
        if command.is_null() {
            command = xmalloc((l + 1 as libc::c_int) as size_t) as *mut libc::c_char;
        } else {
            command = xrealloc(
                command as *mut libc::c_void,
                (len + 1 as libc::c_int + l + 1 as libc::c_int) as size_t,
            ) as *mut libc::c_char;
            strcpy(
                command.offset(len as isize),
                b" \0" as *const u8 as *const libc::c_char,
            );
            len += 1;
            len;
        }
        strcpy(command.offset(len as isize), *argv.offset(i as isize));
        len += l;
        i += 1;
        i;
    }
    if command.is_null() {
        command = xmalloc(16 as libc::c_int as size_t) as *mut libc::c_char;
    }
    *command.offset(len as isize) = 0 as libc::c_int as libc::c_char;
    return command;
}
unsafe extern "C" fn daemon_commandtoken_is_nasty(mut c: libc::c_int) -> libc::c_int {
    if c <= 32 as libc::c_int || c >= 127 as libc::c_int || c == '"' as i32
        || c == '\'' as i32 || c == '\\' as i32
    {
        return 1 as libc::c_int
    } else {
        return 0 as libc::c_int
    };
}
unsafe extern "C" fn hex_digit(mut c: libc::c_int) -> libc::c_int {
    if '0' as i32 <= c && c <= '9' as i32 {
        return c - '0' as i32
    } else if 'a' as i32 <= c && c <= 'f' as i32 {
        return c - 'a' as i32 + 10 as libc::c_int
    } else if 'A' as i32 <= c && c <= 'F' as i32 {
        return c - 'A' as i32 + 10 as libc::c_int
    } else {
        return -(1 as libc::c_int)
    };
}
pub unsafe extern "C" fn daemon_commandtoken_escape(
    mut buff: *mut libc::c_char,
    mut size: libc::c_int,
) -> *mut libc::c_char {
    let mut l: libc::c_int = 0;
    let mut s: libc::c_int = 0;
    let mut c: libc::c_int = 0;
    let mut p: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut q: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut out: *mut libc::c_char = 0 as *mut libc::c_char;
    if buff.is_null() {
        return 0 as *mut libc::c_char;
    }
    l = 0 as libc::c_int;
    p = buff;
    s = size;
    while s > 0 as libc::c_int {
        if daemon_commandtoken_is_nasty(*p as libc::c_int) != 0 {
            l += 3 as libc::c_int;
        } else {
            l += 1;
            l;
        }
        p = p.offset(1);
        p;
        s -= 1;
        s;
    }
    out = xmalloc((l + 1 as libc::c_int) as size_t) as *mut libc::c_char;
    p = buff;
    q = out;
    s = size;
    while s > 0 as libc::c_int {
        if daemon_commandtoken_is_nasty(*p as libc::c_int) != 0 {
            c = *(p as *mut libc::c_uchar) as libc::c_int;
            sprintf(q, b"\\%.2X\0" as *const u8 as *const libc::c_char, c);
            q = q.offset(3 as libc::c_int as isize);
        } else {
            *q = *p;
            q = q.offset(1);
            q;
        }
        p = p.offset(1);
        p;
        s -= 1;
        s;
    }
    *q = 0 as libc::c_int as libc::c_char;
    return out;
}
pub unsafe extern "C" fn daemon_commandtoken_escape_string(
    mut buff: *mut libc::c_char,
) -> *mut libc::c_char {
    let mut len: libc::c_int = 0;
    if buff.is_null()
        || {
            len = strlen(buff) as libc::c_int;
            len <= 0 as libc::c_int
        }
    {
        return 0 as *mut libc::c_char
    } else {
        return daemon_commandtoken_escape(buff, len)
    };
}
pub unsafe extern "C" fn daemon_commandtoken_unescape(
    mut buff: *mut libc::c_char,
) -> libc::c_int {
    let mut out: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut h1: libc::c_int = 0;
    let mut h2: libc::c_int = 0;
    let mut ret: libc::c_int = 0;
    ret = 0 as libc::c_int;
    out = buff;
    while *buff != 0 {
        if *buff as libc::c_int == '\\' as i32
            && {
                h1 = hex_digit(*buff.offset(1 as libc::c_int as isize) as libc::c_int);
                h1 >= 0 as libc::c_int
            }
            && {
                h2 = hex_digit(*buff.offset(2 as libc::c_int as isize) as libc::c_int);
                h2 >= 0 as libc::c_int
            }
        {
            *out = (h1 * 16 as libc::c_int + h2) as libc::c_char;
            buff = buff.offset(3 as libc::c_int as isize);
            out = out.offset(1);
            out;
        } else {
            *out = *buff;
            buff = buff.offset(1);
            buff;
            out = out.offset(1);
            out;
        }
        ret += 1;
        ret;
    }
    *out = 0 as libc::c_int as libc::c_char;
    return ret;
}
pub unsafe extern "C" fn hprintf(
    mut handle: libc::c_int,
    mut msg: *mut libc::c_char,
    mut args: ...
) -> libc::c_int {
    let mut buff: [libc::c_char; 256] = [0; 256];
    let mut tbuff: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut ap: ::std::ffi::VaListImpl;
    let mut n: libc::c_int = 0;
    ap = args.clone();
    n = vsnprintf(
        buff.as_mut_ptr(),
        256 as libc::c_int as libc::c_ulong,
        msg,
        ap.as_va_list(),
    );
    if n < 256 as libc::c_int {
        write(handle, buff.as_mut_ptr() as *const libc::c_void, n as size_t);
        return n;
    } else {
        tbuff = 0 as *mut libc::c_char;
        ap = args.clone();
        vstrappendf(&mut tbuff, msg, ap.as_va_list());
        if !tbuff.is_null() {
            n = strlen(tbuff) as libc::c_int;
            write(handle, tbuff as *const libc::c_void, n as size_t);
            free(tbuff as *mut libc::c_void);
        } else {
            n = 0 as libc::c_int;
        }
        return n;
    };
}
pub unsafe extern "C" fn get_number_of_cpus_proccpuinfo() -> libc::c_int {
    let mut fr: *mut FILE = 0 as *mut FILE;
    let mut buff: [libc::c_char; 1024] = [0; 1024];
    let mut ncpu: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    fr = fopen(
        b"/proc/cpuinfo\0" as *const u8 as *const libc::c_char,
        b"rb\0" as *const u8 as *const libc::c_char,
    );
    if !fr.is_null() {
        ncpu = 0 as libc::c_int;
        while feof(fr) == 0 {
            if (fgets(buff.as_mut_ptr(), 1024 as libc::c_int - 1 as libc::c_int, fr))
                .is_null()
            {
                break;
            }
            i = 0 as libc::c_int;
            while (i as libc::c_ulong) < strlen(buff.as_mut_ptr()) {
                if buff[i as usize] as libc::c_int == 10 as libc::c_int
                    || buff[i as usize] as libc::c_int == 9 as libc::c_int
                    || buff[i as usize] as libc::c_int == 13 as libc::c_int
                    || buff[i as usize] as libc::c_int == 32 as libc::c_int
                {
                    memmove(
                        buff.as_mut_ptr().offset(i as isize) as *mut libc::c_void,
                        buff
                            .as_mut_ptr()
                            .offset(i as isize)
                            .offset(1 as libc::c_int as isize) as *const libc::c_void,
                        strlen(buff.as_mut_ptr().offset(i as isize)),
                    );
                } else {
                    i += 1;
                    i;
                }
            }
            j = 0 as libc::c_int;
            while (j as libc::c_ulong) < strlen(buff.as_mut_ptr()) {
                if buff[j as usize] as libc::c_int == ':' as i32 {
                    break;
                }
                j += 1;
                j;
            }
            if j as libc::c_ulong == strlen(buff.as_mut_ptr()) {
                continue;
            }
            buff[j as usize] = 0 as libc::c_int as libc::c_char;
            if strcmp(
                buff.as_mut_ptr(),
                b"processor\0" as *const u8 as *const libc::c_char,
            ) == 0 as libc::c_int
            {
                ncpu += 1;
                ncpu;
            }
        }
        fclose(fr);
    } else {
        ncpu = 0 as libc::c_int;
    }
    return ncpu;
}
pub unsafe extern "C" fn get_number_of_cpus_sysconf() -> libc::c_int {
    let mut ncpu: libc::c_int = 0;
    ncpu = sysconf(_SC_NPROCESSORS_ONLN as libc::c_int) as libc::c_int;
    return ncpu;
}
pub unsafe extern "C" fn get_number_of_cpus() -> libc::c_int {
    let mut ncpu: libc::c_int = 0;
    let mut w: libc::c_int = 0;
    ncpu = get_number_of_cpus_proccpuinfo();
    w = get_number_of_cpus_sysconf();
    if w > 0 as libc::c_int && ncpu > 0 as libc::c_int {
        ncpu = if w < ncpu { w } else { ncpu };
    }
    if ncpu <= 0 as libc::c_int {
        ncpu = 1 as libc::c_int;
    }
    return ncpu;
}
pub unsafe extern "C" fn get_bit_size(mut n: libc::c_int) -> libc::c_int {
    let mut r: libc::c_int = 0;
    r = 0 as libc::c_int;
    while n > 0 as libc::c_int {
        r += 1;
        r;
        n /= 2 as libc::c_int;
    }
    return r;
}
pub unsafe extern "C" fn get_child_by_pid(
    mut cc: *mut child,
    mut pid: libc::c_int,
) -> *mut child {
    while !cc.is_null() {
        if (*cc).pid == pid {
            return cc;
        }
        cc = (*cc).next;
    }
    return 0 as *mut child;
}
pub unsafe extern "C" fn get_child_by_id(
    mut cc: *mut child,
    mut id: libc::c_int,
) -> *mut child {
    while !cc.is_null() {
        if (*cc).id == id {
            return cc;
        }
        cc = (*cc).next;
    }
    return 0 as *mut child;
}
pub unsafe extern "C" fn get_imutex_by_name(
    mut mx: *mut imutex,
    mut name: *mut libc::c_char,
) -> *mut imutex {
    while !mx.is_null() {
        if strcmp((*mx).name, name) == 0 as libc::c_int {
            return mx;
        }
        mx = (*mx).next;
    }
    return 0 as *mut imutex;
}
pub unsafe extern "C" fn fd_avail(mut fd: libc::c_int) -> libc::c_int {
    let mut set: fd_set = fd_set { fds_bits: [0; 16] };
    let mut tv: timeval = timeval { tv_sec: 0, tv_usec: 0 };
    let mut __d0: libc::c_int = 0;
    let mut __d1: libc::c_int = 0;
    let fresh6 = &mut __d0;
    let fresh7;
    let fresh8 = (::std::mem::size_of::<fd_set>() as libc::c_ulong)
        .wrapping_div(::std::mem::size_of::<__fd_mask>() as libc::c_ulong);
    let fresh9 = &mut __d1;
    let fresh10;
    let fresh11 = &mut *(set.fds_bits).as_mut_ptr().offset(0 as libc::c_int as isize)
        as *mut __fd_mask;
    asm!(
        "cld; rep; stosq", inlateout("cx") c2rust_asm_casts::AsmCast::cast_in(fresh6,
        fresh8) => fresh7, inlateout("di") c2rust_asm_casts::AsmCast::cast_in(fresh9,
        fresh11) => fresh10, inlateout("ax") 0 as libc::c_int => _,
        options(preserves_flags, att_syntax)
    );
    c2rust_asm_casts::AsmCast::cast_out(fresh6, fresh8, fresh7);
    c2rust_asm_casts::AsmCast::cast_out(fresh9, fresh11, fresh10);
    set
        .fds_bits[(fd
        / (8 as libc::c_int
            * ::std::mem::size_of::<__fd_mask>() as libc::c_ulong as libc::c_int))
        as usize]
        |= ((1 as libc::c_ulong)
            << fd
                % (8 as libc::c_int
                    * ::std::mem::size_of::<__fd_mask>() as libc::c_ulong
                        as libc::c_int)) as __fd_mask;
    tv.tv_sec = 0 as libc::c_int as __time_t;
    tv.tv_usec = 0 as libc::c_int as __suseconds_t;
    select(fd + 1 as libc::c_int, &mut set, 0 as *mut fd_set, 0 as *mut fd_set, &mut tv);
    if set
        .fds_bits[(fd
        / (8 as libc::c_int
            * ::std::mem::size_of::<__fd_mask>() as libc::c_ulong as libc::c_int))
        as usize]
        & ((1 as libc::c_ulong)
            << fd
                % (8 as libc::c_int
                    * ::std::mem::size_of::<__fd_mask>() as libc::c_ulong
                        as libc::c_int)) as __fd_mask != 0 as libc::c_int as libc::c_long
    {
        return 1 as libc::c_int
    } else {
        return 0 as libc::c_int
    };
}
pub unsafe extern "C" fn fileformat_replace(
    mut format: *mut libc::c_char,
    mut par: *mut parameter,
) -> *mut libc::c_char {
    let mut name: *mut libc::c_char = 0 as *mut libc::c_char;
    name = format_replace(
        format,
        0 as libc::c_int,
        's' as i32,
        2 as libc::c_int,
        (*par).name,
        'k' as i32,
        2 as libc::c_int,
        (*par).id,
        'd' as i32,
        2 as libc::c_int,
        (*par).id + 1 as libc::c_int,
        0 as libc::c_int,
    );
    return name;
}
pub unsafe extern "C" fn submit_task(
    mut p: *mut paralleldata,
    mut par: *mut parameter,
    mut c: *mut child,
    mut no_format_replace: libc::c_int,
    mut ps: *mut parallelstatus,
) -> libc::c_int {
    let mut stdfd: [libc::c_int; 3] = [0; 3];
    let mut pipeout: [libc::c_int; 2] = [0; 2];
    let mut pipeerr: [libc::c_int; 2] = [0; 2];
    let mut pid: libc::c_int = 0;
    if (*par).no_touch_std != 0 {
        stdfd[0 as libc::c_int as usize] = -(1 as libc::c_int);
        stdfd[1 as libc::c_int as usize] = -(1 as libc::c_int);
        stdfd[2 as libc::c_int as usize] = -(1 as libc::c_int);
        pipeout[0 as libc::c_int as usize] = -(1 as libc::c_int);
        pipeout[1 as libc::c_int as usize] = -(1 as libc::c_int);
        pipeerr[0 as libc::c_int as usize] = -(1 as libc::c_int);
        pipeerr[1 as libc::c_int as usize] = -(1 as libc::c_int);
    } else {
        if ((*p).in_0).is_null() {
            stdfd[0 as libc::c_int
                as usize] = open(
                b"/dev/null\0" as *const u8 as *const libc::c_char,
                0 as libc::c_int,
            );
            if stdfd[0 as libc::c_int as usize] < 0 as libc::c_int {
                return -(1 as libc::c_int);
            }
        } else {
            let mut inname: *mut libc::c_char = 0 as *mut libc::c_char;
            if no_format_replace != 0 {
                stdfd[0 as libc::c_int as usize] = open((*p).in_0, 0 as libc::c_int);
            } else {
                inname = fileformat_replace((*p).in_0, par);
                stdfd[0 as libc::c_int as usize] = open(inname, 0 as libc::c_int);
                free(inname as *mut libc::c_void);
            }
            if stdfd[0 as libc::c_int as usize] < 0 as libc::c_int {
                return -(2 as libc::c_int);
            }
        }
        if ((*p).out).is_null() {
            stdfd[1 as libc::c_int
                as usize] = open(
                b"/dev/null\0" as *const u8 as *const libc::c_char,
                0o1 as libc::c_int,
            );
            if stdfd[1 as libc::c_int as usize] < 0 as libc::c_int {
                return -(3 as libc::c_int);
            }
            pipeout[0 as libc::c_int as usize] = -(1 as libc::c_int);
            pipeout[1 as libc::c_int as usize] = -(1 as libc::c_int);
        } else if !((*p).fwout).is_null() {
            if pipe(pipeout.as_mut_ptr()) != 0 {
                return -(4 as libc::c_int);
            }
            stdfd[1 as libc::c_int as usize] = pipeout[1 as libc::c_int as usize];
        } else {
            let mut outname: *mut libc::c_char = 0 as *mut libc::c_char;
            if no_format_replace != 0 {
                stdfd[1 as libc::c_int
                    as usize] = open(
                    (*p).out,
                    0o1 as libc::c_int | 0o100 as libc::c_int | 0o1000 as libc::c_int,
                    0o644 as libc::c_int,
                );
            } else {
                outname = fileformat_replace((*p).out, par);
                stdfd[1 as libc::c_int
                    as usize] = open(
                    outname,
                    0o1 as libc::c_int | 0o100 as libc::c_int | 0o1000 as libc::c_int,
                    0o644 as libc::c_int,
                );
                free(outname as *mut libc::c_void);
            }
            if stdfd[1 as libc::c_int as usize] < 0 as libc::c_int {
                return -(5 as libc::c_int);
            }
            pipeout[0 as libc::c_int as usize] = -(1 as libc::c_int);
            pipeout[1 as libc::c_int as usize] = -(1 as libc::c_int);
        }
        if ((*p).err).is_null() {
            stdfd[2 as libc::c_int
                as usize] = open(
                b"/dev/null\0" as *const u8 as *const libc::c_char,
                0o1 as libc::c_int,
            );
            if stdfd[2 as libc::c_int as usize] < 0 as libc::c_int {
                return -(3 as libc::c_int);
            }
            pipeerr[0 as libc::c_int as usize] = -(1 as libc::c_int);
            pipeerr[1 as libc::c_int as usize] = -(1 as libc::c_int);
        } else if !((*p).fwerr).is_null() {
            if pipe(pipeerr.as_mut_ptr()) != 0 {
                return -(4 as libc::c_int);
            }
            stdfd[2 as libc::c_int as usize] = pipeerr[1 as libc::c_int as usize];
        } else {
            let mut errname: *mut libc::c_char = 0 as *mut libc::c_char;
            if no_format_replace != 0 {
                stdfd[2 as libc::c_int
                    as usize] = open(
                    (*p).err,
                    0o1 as libc::c_int | 0o100 as libc::c_int | 0o1000 as libc::c_int,
                    0o644 as libc::c_int,
                );
            } else {
                errname = fileformat_replace((*p).err, par);
                stdfd[2 as libc::c_int
                    as usize] = open(
                    errname,
                    0o1 as libc::c_int | 0o100 as libc::c_int | 0o1000 as libc::c_int,
                    0o644 as libc::c_int,
                );
                free(errname as *mut libc::c_void);
            }
            if stdfd[2 as libc::c_int as usize] < 0 as libc::c_int {
                return -(6 as libc::c_int);
            }
            pipeerr[0 as libc::c_int as usize] = -(1 as libc::c_int);
            pipeerr[1 as libc::c_int as usize] = -(1 as libc::c_int);
        }
    }
    pid = fork();
    if pid < 0 as libc::c_int {
        return -(4 as libc::c_int)
    } else if pid > 0 as libc::c_int {
        if pipeout[0 as libc::c_int as usize] >= 0 as libc::c_int {
            (*c).fdstdout = pipeout[0 as libc::c_int as usize];
            close(pipeout[1 as libc::c_int as usize]);
        } else {
            (*c).fdstdout = -(1 as libc::c_int);
            if stdfd[1 as libc::c_int as usize] >= 0 as libc::c_int {
                close(stdfd[1 as libc::c_int as usize]);
            }
        }
        if pipeerr[0 as libc::c_int as usize] >= 0 as libc::c_int {
            (*c).fdstderr = pipeerr[0 as libc::c_int as usize];
            close(pipeerr[1 as libc::c_int as usize]);
        } else {
            (*c).fdstderr = -(1 as libc::c_int);
            if stdfd[2 as libc::c_int as usize] >= 0 as libc::c_int {
                close(stdfd[2 as libc::c_int as usize]);
            }
        }
        if stdfd[0 as libc::c_int as usize] >= 0 as libc::c_int {
            close(stdfd[0 as libc::c_int as usize]);
        }
        return pid;
    } else {
        if pipeout[0 as libc::c_int as usize] >= 0 as libc::c_int {
            close(pipeout[0 as libc::c_int as usize]);
        }
        if pipeerr[0 as libc::c_int as usize] >= 0 as libc::c_int {
            close(pipeerr[0 as libc::c_int as usize]);
        }
        if !ps.is_null() {
            let mut cl: *mut client = 0 as *mut client;
            let mut cc: *mut child = 0 as *mut child;
            cl = (*ps).clientlist;
            while !cl.is_null() {
                if (*cl).peer >= 0 as libc::c_int {
                    close((*cl).peer);
                }
                cl = (*cl).next;
            }
            if (*ps).sock >= 0 as libc::c_int {
                close((*ps).sock);
            }
            if (*ps).hsck >= 0 as libc::c_int {
                close((*ps).hsck);
            }
            cc = (*ps).childlist;
            while !cc.is_null() {
                if (*cc).fdstdout >= 0 as libc::c_int {
                    close((*cc).fdstdout);
                }
                if (*cc).fdstderr >= 0 as libc::c_int {
                    close((*cc).fdstderr);
                }
                cc = (*cc).next;
            }
        }
        if stdfd[0 as libc::c_int as usize] >= 0 as libc::c_int {
            close(0 as libc::c_int);
            dup2(stdfd[0 as libc::c_int as usize], 0 as libc::c_int);
            close(stdfd[0 as libc::c_int as usize]);
        }
        if stdfd[1 as libc::c_int as usize] >= 0 as libc::c_int {
            close(1 as libc::c_int);
            dup2(stdfd[1 as libc::c_int as usize], 1 as libc::c_int);
            close(stdfd[1 as libc::c_int as usize]);
        }
        if stdfd[2 as libc::c_int as usize] >= 0 as libc::c_int {
            close(2 as libc::c_int);
            dup2(stdfd[2 as libc::c_int as usize], 2 as libc::c_int);
            close(stdfd[2 as libc::c_int as usize]);
        }
        close(sig_pipe[0 as libc::c_int as usize]);
        close(sig_pipe[1 as libc::c_int as usize]);
        if !((*p).envvarname).is_null() && !((*par).name).is_null() {
            env_export((*p).envvarname, (*par).name);
        }
        if (*par).c.is_shell > 0 as libc::c_int {
            let mut argv: [*mut libc::c_char; 4] = [0 as *mut libc::c_char; 4];
            argv[0 as libc::c_int as usize] = (*p).shell;
            argv[1 as libc::c_int
                as usize] = b"-c\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char;
            argv[2 as libc::c_int
                as usize] = concatenate_arguments((*par).c.argc, (*par).c.argv);
            argv[3 as libc::c_int as usize] = 0 as *mut libc::c_char;
            execv((*p).shell, argv.as_mut_ptr() as *const *mut libc::c_char);
            fprintf(
                stderr,
                dcgettext(
                    0 as *const libc::c_char,
                    b"%s: unable to execute the shell '%s'.\n\0" as *const u8
                        as *const libc::c_char,
                    5 as libc::c_int,
                ),
                progbasename,
                (*p).shell,
            );
            exit(2 as libc::c_int);
        } else if (*par).c.is_shell < 0 as libc::c_int {
            let mut argv_0: *mut *mut libc::c_char = 0 as *mut *mut libc::c_char;
            let mut i: libc::c_int = 0;
            let mut argc: libc::c_int = 0;
            let mut n: libc::c_int = 0;
            n = 0 as libc::c_int;
            n = 0 as libc::c_int;
            while !(*((*p).rshargs).offset(n as isize)).is_null() {
                n += 1;
                n;
            }
            argc = n + 1 as libc::c_int + (*par).c.argc + 1 as libc::c_int;
            argv_0 = xmalloc(
                (::std::mem::size_of::<*mut libc::c_char>() as libc::c_ulong)
                    .wrapping_mul((argc + 1 as libc::c_int) as libc::c_ulong),
            ) as *mut *mut libc::c_char;
            argc = 0 as libc::c_int;
            i = 0 as libc::c_int;
            while i < n {
                let ref mut fresh12 = *argv_0.offset(argc as isize);
                *fresh12 = *((*p).rshargs).offset(i as isize);
                argc += 1;
                argc;
                i += 1;
                i;
            }
            if !((*par).name).is_null() {
                let ref mut fresh13 = *argv_0.offset(argc as isize);
                *fresh13 = (*par).name;
                argc += 1;
                argc;
            } else {
                let ref mut fresh14 = *argv_0.offset(argc as isize);
                *fresh14 = b"localhost\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char;
                argc += 1;
                argc;
            }
            i = 0 as libc::c_int;
            while i < (*par).c.argc {
                let ref mut fresh15 = *argv_0.offset(argc as isize);
                *fresh15 = *((*par).c.argv).offset(i as isize);
                argc += 1;
                argc;
                i += 1;
                i;
            }
            let ref mut fresh16 = *argv_0.offset(argc as isize);
            *fresh16 = 0 as *mut libc::c_char;
            execvp(
                *argv_0.offset(0 as libc::c_int as isize),
                argv_0 as *const *mut libc::c_char,
            );
            fprintf(
                stderr,
                dcgettext(
                    0 as *const libc::c_char,
                    b"%s: unable to execute the remote shell command '%s'.\n\0"
                        as *const u8 as *const libc::c_char,
                    5 as libc::c_int,
                ),
                progbasename,
                *argv_0.offset(0 as libc::c_int as isize),
            );
            exit(2 as libc::c_int);
        } else {
            let mut argv_1: *mut *mut libc::c_char = 0 as *mut *mut libc::c_char;
            let mut i_0: libc::c_int = 0;
            let mut argc_0: libc::c_int = 0;
            argc_0 = (*par).c.argc;
            argv_1 = xmalloc(
                (::std::mem::size_of::<*mut libc::c_char>() as libc::c_ulong)
                    .wrapping_mul((argc_0 + 1 as libc::c_int) as libc::c_ulong),
            ) as *mut *mut libc::c_char;
            i_0 = 0 as libc::c_int;
            while i_0 < argc_0 {
                let ref mut fresh17 = *argv_1.offset(i_0 as isize);
                *fresh17 = *((*par).c.argv).offset(i_0 as isize);
                i_0 += 1;
                i_0;
            }
            let ref mut fresh18 = *argv_1.offset(argc_0 as isize);
            *fresh18 = 0 as *mut libc::c_char;
            execvp(
                *argv_1.offset(0 as libc::c_int as isize),
                argv_1 as *const *mut libc::c_char,
            );
            fprintf(
                stderr,
                dcgettext(
                    0 as *const libc::c_char,
                    b"%s: unable to execute the command '%s'.\n\0" as *const u8
                        as *const libc::c_char,
                    5 as libc::c_int,
                ),
                progbasename,
                *argv_1.offset(0 as libc::c_int as isize),
            );
            exit(2 as libc::c_int);
        }
    };
}
pub unsafe extern "C" fn send_task(
    mut p: *mut paralleldata,
    mut par: *mut parameter,
    mut cc: *mut child,
) -> libc::c_int {
    let mut args: *mut *mut libc::c_char = 0 as *mut *mut libc::c_char;
    let mut token: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut a: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    args = xmalloc(
        (::std::mem::size_of::<*mut libc::c_char>() as libc::c_ulong)
            .wrapping_mul(16 as libc::c_int as libc::c_ulong),
    ) as *mut *mut libc::c_char;
    a = 0 as libc::c_int;
    while a < 16 as libc::c_int {
        let ref mut fresh19 = *args.offset(a as isize);
        *fresh19 = 0 as *mut libc::c_char;
        a += 1;
        a;
    }
    a = 0 as libc::c_int;
    strappendf(
        &mut *args.offset(a as isize) as *mut *mut libc::c_char,
        b"execute\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    a += 1;
    a;
    strappendf(
        &mut *args.offset(a as isize) as *mut *mut libc::c_char,
        b"identifier=%d\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        (*par).id,
    );
    a += 1;
    a;
    if !((*p).envvarname).is_null() && !((*par).name).is_null() {
        strappendf(
            &mut *args.offset(a as isize) as *mut *mut libc::c_char,
            b"envname=%s\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            (*p).envvarname,
        );
        a += 1;
        a;
        strappendf(
            &mut *args.offset(a as isize) as *mut *mut libc::c_char,
            b"envvalue=%s\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            (*par).name,
        );
        a += 1;
        a;
    }
    if !((*p).in_0).is_null() {
        let mut inname: *mut libc::c_char = 0 as *mut libc::c_char;
        inname = fileformat_replace((*p).in_0, par);
        strappendf(
            &mut *args.offset(a as isize) as *mut *mut libc::c_char,
            b"in=%s\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            inname,
        );
        a += 1;
        a;
        free(inname as *mut libc::c_void);
    }
    if !((*p).out).is_null() && ((*p).fwout).is_null() {
        let mut outname: *mut libc::c_char = 0 as *mut libc::c_char;
        outname = fileformat_replace((*p).out, par);
        strappendf(
            &mut *args.offset(a as isize) as *mut *mut libc::c_char,
            b"out=%s\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            outname,
        );
        a += 1;
        a;
        free(outname as *mut libc::c_void);
    } else if !((*p).fwout).is_null() {
        strappendf(
            &mut *args.offset(a as isize) as *mut *mut libc::c_char,
            b"out=-\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        );
        a += 1;
        a;
    }
    if !((*p).err).is_null() && ((*p).fwerr).is_null() {
        let mut errname: *mut libc::c_char = 0 as *mut libc::c_char;
        errname = fileformat_replace((*p).err, par);
        strappendf(
            &mut *args.offset(a as isize) as *mut *mut libc::c_char,
            b"err=%s\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            errname,
        );
        a += 1;
        a;
        free(errname as *mut libc::c_void);
    } else if !((*p).fwerr).is_null() {
        strappendf(
            &mut *args.offset(a as isize) as *mut *mut libc::c_char,
            b"err=-\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        );
        a += 1;
        a;
    }
    if (*par).c.is_shell != 0 {
        strappendf(
            &mut *args.offset(a as isize) as *mut *mut libc::c_char,
            b"shell=%s\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            (*p).shell,
        );
        a += 1;
        a;
        strappendf(
            &mut *args.offset(a as isize) as *mut *mut libc::c_char,
            b"-\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        );
        a += 1;
        a;
        let ref mut fresh20 = *args.offset(a as isize);
        *fresh20 = concatenate_arguments((*par).c.argc, (*par).c.argv);
        a += 1;
        a;
    } else {
        strappendf(
            &mut *args.offset(a as isize) as *mut *mut libc::c_char,
            b"-\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        );
        a += 1;
        a;
        args = xrealloc(
            args as *mut libc::c_void,
            (::std::mem::size_of::<*mut libc::c_char>() as libc::c_ulong)
                .wrapping_mul((16 as libc::c_int + (*par).c.argc) as libc::c_ulong),
        ) as *mut *mut libc::c_char;
        i = 0 as libc::c_int;
        while i < (*par).c.argc {
            let ref mut fresh21 = *args.offset(a as isize);
            *fresh21 = xstrdup(*((*par).c.argv).offset(i as isize));
            a += 1;
            a;
            i += 1;
            i;
        }
    }
    let ref mut fresh22 = *args.offset(a as isize);
    *fresh22 = 0 as *mut libc::c_char;
    i = 0 as libc::c_int;
    while i < a {
        token = daemon_commandtoken_escape_string(*args.offset(i as isize));
        if i < a - 1 as libc::c_int {
            hprintf(
                (*(*cc).rs).fhsend,
                b"%s \0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                token,
            );
        } else {
            hprintf(
                (*(*cc).rs).fhsend,
                b"%s\n\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                token,
            );
        }
        free(token as *mut libc::c_void);
        free(*args.offset(i as isize) as *mut libc::c_void);
        i += 1;
        i;
    }
    free(args as *mut libc::c_void);
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn read_fd_block(
    mut fd: libc::c_int,
    mut data: *mut libc::c_void,
    mut size: size_t,
) -> size_t {
    let mut cdata: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut rd: ssize_t = 0;
    let mut tr: size_t = 0;
    cdata = data as *mut libc::c_char;
    tr = 0 as libc::c_int as size_t;
    while size > 0 as libc::c_int as libc::c_ulong {
        rd = read(fd, cdata as *mut libc::c_void, size);
        if rd < 0 as libc::c_int as libc::c_long
            && *__errno_location() == 4 as libc::c_int
        {
            continue;
        }
        if rd <= 0 as libc::c_int as libc::c_long {
            return 0 as libc::c_int as size_t
        } else {
            size = (size as libc::c_ulong).wrapping_sub(rd as libc::c_ulong) as size_t
                as size_t;
            cdata = cdata.offset(rd as isize);
            tr = (tr as libc::c_ulong).wrapping_add(rd as libc::c_ulong) as size_t
                as size_t;
        }
    }
    return tr;
}
pub unsafe extern "C" fn read_signalinfo(
    mut fd: libc::c_int,
    mut sci: *mut signalinfo,
) -> libc::c_int {
    return read_fd_block(
        fd,
        sci as *mut libc::c_void,
        ::std::mem::size_of::<signalinfo>() as libc::c_ulong,
    ) as libc::c_int;
}
pub unsafe extern "C" fn write_out_line(
    mut fw: *mut FILE,
    mut format: *mut libc::c_char,
    mut lbuffer: *mut libc::c_char,
    mut p: *mut parameter,
    mut omit_newlines: libc::c_int,
) -> libc::c_int {
    let mut outline: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut len: size_t = 0;
    outline = format_replace(
        format,
        1 as libc::c_int,
        'k' as i32,
        1 as libc::c_int,
        (*p).id,
        'd' as i32,
        1 as libc::c_int,
        (*p).id + 1 as libc::c_int,
        's' as i32,
        2 as libc::c_int,
        (*p).name,
        'l' as i32,
        2 as libc::c_int,
        lbuffer,
        0 as *mut libc::c_void,
    );
    if !outline.is_null() {
        len = strlen(outline);
        if omit_newlines != 0 {
            fwrite(
                outline as *const libc::c_void,
                1 as libc::c_int as libc::c_ulong,
                len,
                fw,
            );
        } else {
            fprintf(fw, b"%s\n\0" as *const u8 as *const libc::c_char, outline);
        }
        fflush(fw);
        free(outline as *mut libc::c_void);
    }
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn write_output(
    mut buff: *mut libc::c_char,
    mut n: size_t,
    mut fw: *mut FILE,
    mut format: *mut libc::c_char,
    mut omit_newlines: libc::c_int,
    mut lb: *mut linebuffer,
    mut p: *mut parameter,
) -> libc::c_int {
    let mut line: *mut libc::c_char = 0 as *mut libc::c_char;
    if fw.is_null() {
        return 0 as libc::c_int;
    }
    if buff.is_null() {
        n = 0 as libc::c_int as size_t;
    }
    if format.is_null() || lb.is_null() {
        if n > 0 as libc::c_int as libc::c_ulong && !buff.is_null() {
            fwrite(
                buff as *const libc::c_void,
                1 as libc::c_int as libc::c_ulong,
                n,
                fw,
            );
            fflush(fw);
        }
        return 0 as libc::c_int;
    } else if !format.is_null() && !lb.is_null() {
        linebuffer_concatenate(lb, buff, n);
        loop {
            line = linebuffer_fetch(lb);
            if !(!line.is_null()
                || buff.is_null()
                    && {
                        line = linebuffer_flush(lb);
                        !line.is_null()
                    })
            {
                break;
            }
            write_out_line(fw, format, line, p, omit_newlines);
            free(line as *mut libc::c_void);
        }
        return 0 as libc::c_int;
    } else {
        return -(1 as libc::c_int)
    };
}
pub unsafe extern "C" fn imutex_cleanup(
    mut mx: *mut imutex,
    mut qid: libc::c_int,
    mut data: *mut libc::c_void,
) -> libc::c_int {
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut r: libc::c_int = 0;
    while !mx.is_null() {
        if !(((*mx).pclients).is_null() || (*mx).npclient <= 0 as libc::c_int) {
            r = 0 as libc::c_int;
            i = 0 as libc::c_int;
            while i < (*mx).npclient {
                if (*((*mx).pclients).offset(i as isize)).qid == qid
                    && (*((*mx).pclients).offset(i as isize)).data == data
                {
                    (*((*mx).pclients).offset(i as isize)).qid = -(1 as libc::c_int);
                    let ref mut fresh23 = (*((*mx).pclients).offset(i as isize)).data;
                    *fresh23 = 0 as *mut libc::c_void;
                    r += 1;
                    r;
                }
                i += 1;
                i;
            }
            if r == (*mx).npclient {
                free((*mx).pclients as *mut libc::c_void);
                (*mx).pclients = 0 as *mut pendingclient;
                (*mx).npclient = 0 as libc::c_int;
            } else if r != 0 {
                i = 0 as libc::c_int;
                j = 0 as libc::c_int;
                while i < (*mx).npclient {
                    if !((*((*mx).pclients).offset(i as isize)).qid < 0 as libc::c_int
                        && ((*((*mx).pclients).offset(i as isize)).data).is_null())
                    {
                        *((*mx).pclients)
                            .offset(j as isize) = *((*mx).pclients).offset(i as isize);
                        j += 1;
                        j;
                    }
                    i += 1;
                    i;
                }
                (*mx).npclient = j;
            }
        }
        mx = (*mx).next;
    }
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn imutex_lock_get_params(
    mut cmd: *mut *mut libc::c_char,
    mut rname: *mut *mut libc::c_char,
    mut rmaxnum: *mut libc::c_int,
) -> libc::c_int {
    let mut name: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut i: libc::c_int = 0;
    let mut maxnum: libc::c_int = 0;
    name = 0 as *mut libc::c_char;
    maxnum = 1 as libc::c_int;
    i = 1 as libc::c_int;
    while !(*cmd.offset(i as isize)).is_null() {
        if (strcmp(*cmd.offset(i as isize), b"-m\0" as *const u8 as *const libc::c_char)
            == 0 as libc::c_int
            || strcmp(
                *cmd.offset(i as isize),
                b"--maximum\0" as *const u8 as *const libc::c_char,
            ) == 0 as libc::c_int)
            && !(*cmd.offset((i + 1 as libc::c_int) as isize)).is_null()
        {
            i += 1;
            i;
            sscanf(
                *cmd.offset(i as isize),
                b"%d\0" as *const u8 as *const libc::c_char,
                &mut maxnum as *mut libc::c_int,
            );
        } else if *(*cmd.offset(i as isize)).offset(0 as libc::c_int as isize)
            as libc::c_int == '-' as i32
        {
            return 1 as libc::c_int
        } else if name.is_null() {
            name = *cmd.offset(i as isize);
        } else {
            return 1 as libc::c_int
        }
        i += 1;
        i;
    }
    *rname = name;
    *rmaxnum = maxnum;
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn imutex_unlock_get_params(
    mut cmd: *mut *mut libc::c_char,
    mut rname: *mut *mut libc::c_char,
) -> libc::c_int {
    let mut name: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut i: libc::c_int = 0;
    name = 0 as *mut libc::c_char;
    i = 1 as libc::c_int;
    while !(*cmd.offset(i as isize)).is_null() {
        if *(*cmd.offset(i as isize)).offset(0 as libc::c_int as isize) as libc::c_int
            == '-' as i32
        {
            return 1 as libc::c_int
        } else if name.is_null() {
            name = *cmd.offset(i as isize);
        } else {
            return 1 as libc::c_int
        }
        i += 1;
        i;
    }
    *rname = name;
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn imutex_mutex_get_params(
    mut cmd: *mut *mut libc::c_char,
    mut rname: *mut *mut libc::c_char,
) -> libc::c_int {
    let mut name: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut i: libc::c_int = 0;
    name = 0 as *mut libc::c_char;
    i = 1 as libc::c_int;
    while !(*cmd.offset(i as isize)).is_null() {
        if *(*cmd.offset(i as isize)).offset(0 as libc::c_int as isize) as libc::c_int
            == '-' as i32
        {
            return 1 as libc::c_int
        } else if name.is_null() {
            name = *cmd.offset(i as isize);
        } else {
            return 1 as libc::c_int
        }
        i += 1;
        i;
    }
    *rname = name;
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn remote_client_master_parse_command_tokens(
    mut cmd: *mut *mut libc::c_char,
    mut ps: *mut parallelstatus,
    mut qid: libc::c_int,
    mut dd: *mut libc::c_void,
) -> libc::c_int {
    let mut tc: time_t = 0;
    let mut i: libc::c_int = 0;
    let mut ret: libc::c_int = 0;
    let mut rs: *mut remoteshell = 0 as *mut remoteshell;
    let mut cl: *mut client = 0 as *mut client;
    if qid >= 0 as libc::c_int {
        rs = dd as *mut remoteshell;
        cl = 0 as *mut client;
    } else {
        rs = 0 as *mut remoteshell;
        cl = dd as *mut client;
    }
    if cmd.is_null() || (*cmd.offset(0 as libc::c_int as isize)).is_null() {
        return 0 as libc::c_int
    } else if strcmp(
        *cmd.offset(0 as libc::c_int as isize),
        b"status\0" as *const u8 as *const libc::c_char,
    ) == 0 as libc::c_int
    {
        time(&mut tc);
        if qid >= 0 as libc::c_int && !rs.is_null() {
            hprintf(
                (*rs).fhsend,
                b"remote %d status %d/%d/%d %d %d\n\0" as *const u8
                    as *const libc::c_char as *mut libc::c_char,
                qid,
                (*ps).nfinished,
                (*ps).npending,
                (*ps).nparam,
                (*ps).t0 as libc::c_int,
                tc as libc::c_int,
            );
        } else if !cl.is_null() {
            hprintf(
                (*cl).peer,
                b"status %d/%d/%d %d %d\n\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                (*ps).nfinished,
                (*ps).npending,
                (*ps).nparam,
                (*ps).t0 as libc::c_int,
                tc as libc::c_int,
            );
        }
        ret = 0 as libc::c_int;
    } else if strcmp(
        *cmd.offset(0 as libc::c_int as isize),
        b"lock\0" as *const u8 as *const libc::c_char,
    ) == 0 as libc::c_int
    {
        let mut name: *mut libc::c_char = 0 as *mut libc::c_char;
        let mut maxnum: libc::c_int = 0;
        let mut mx: *mut imutex = 0 as *mut imutex;
        name = 0 as *mut libc::c_char;
        maxnum = 0 as libc::c_int;
        i = imutex_lock_get_params(cmd, &mut name, &mut maxnum);
        if i != 0 || name.is_null() {
            hprintf(
                (*cl).peer,
                b"error unexpected arguments for '%s'\n\0" as *const u8
                    as *const libc::c_char as *mut libc::c_char,
                *cmd.offset(0 as libc::c_int as isize),
            );
        } else {
            mx = get_imutex_by_name((*ps).imutexlist, name);
            if !mx.is_null() {
                (*mx)
                    .pclients = xrealloc(
                    (*mx).pclients as *mut libc::c_void,
                    (::std::mem::size_of::<pendingclient>() as libc::c_ulong)
                        .wrapping_mul(
                            ((*mx).npclient + 1 as libc::c_int) as libc::c_ulong,
                        ),
                ) as *mut pendingclient;
                (*((*mx).pclients).offset((*mx).npclient as isize)).qid = qid;
                let ref mut fresh24 = (*((*mx).pclients).offset((*mx).npclient as isize))
                    .data;
                *fresh24 = dd;
                (*mx).npclient += 1;
                (*mx).npclient;
                (*mx).state += 1;
                (*mx).state;
            } else {
                mx = xmalloc(::std::mem::size_of::<imutex>() as libc::c_ulong)
                    as *mut imutex;
                (*mx).next = (*ps).imutexlist;
                (*mx).prev = 0 as *mut imutex;
                if !((*ps).imutexlist).is_null() {
                    (*(*ps).imutexlist).prev = mx;
                }
                (*ps).imutexlist = mx;
                (*mx).name = xstrdup(name);
                (*mx).state = 1 as libc::c_int;
                (*mx).pclients = 0 as *mut pendingclient;
                (*mx).npclient = 0 as libc::c_int;
                if qid >= 0 as libc::c_int && !rs.is_null() {
                    hprintf(
                        (*rs).fhsend,
                        b"remote %d locked \"%s\"\n\0" as *const u8
                            as *const libc::c_char as *mut libc::c_char,
                        qid,
                        (*mx).name,
                    );
                } else if !cl.is_null() {
                    hprintf(
                        (*cl).peer,
                        b"locked \"%s\"\n\0" as *const u8 as *const libc::c_char
                            as *mut libc::c_char,
                        (*mx).name,
                    );
                }
            }
        }
        ret = 0 as libc::c_int;
    } else if strcmp(
        *cmd.offset(0 as libc::c_int as isize),
        b"unlock\0" as *const u8 as *const libc::c_char,
    ) == 0 as libc::c_int
    {
        let mut name_0: *mut libc::c_char = 0 as *mut libc::c_char;
        let mut mx_0: *mut imutex = 0 as *mut imutex;
        let mut pcw: pendingclient = pendingclient {
            qid: 0,
            data: 0 as *mut libc::c_void,
        };
        let mut is_pcw_set: libc::c_int = 0;
        name_0 = 0 as *mut libc::c_char;
        i = imutex_unlock_get_params(cmd, &mut name_0);
        if i == 0 && !name_0.is_null()
            && {
                mx_0 = get_imutex_by_name((*ps).imutexlist, name_0);
                !mx_0.is_null()
            }
        {
            (*mx_0).state -= 1;
            (*mx_0).state;
            if !((*mx_0).pclients).is_null() && (*mx_0).npclient > 0 as libc::c_int {
                memcpy(
                    &mut pcw as *mut pendingclient as *mut libc::c_void,
                    &mut *((*mx_0).pclients).offset(0 as libc::c_int as isize)
                        as *mut pendingclient as *const libc::c_void,
                    ::std::mem::size_of::<pendingclient>() as libc::c_ulong,
                );
                is_pcw_set = 1 as libc::c_int;
                if (*mx_0).npclient > 1 as libc::c_int {
                    memmove(
                        (*mx_0).pclients as *mut libc::c_void,
                        ((*mx_0).pclients).offset(1 as libc::c_int as isize)
                            as *const libc::c_void,
                        (::std::mem::size_of::<pendingclient>() as libc::c_ulong)
                            .wrapping_mul(
                                ((*mx_0).npclient - 1 as libc::c_int) as libc::c_ulong,
                            ),
                    );
                } else {
                    free((*mx_0).pclients as *mut libc::c_void);
                    (*mx_0).pclients = 0 as *mut pendingclient;
                }
                (*mx_0).npclient -= 1;
                (*mx_0).npclient;
            } else {
                is_pcw_set = 0 as libc::c_int;
            }
            if is_pcw_set != 0 {
                if pcw.qid >= 0 as libc::c_int && !(pcw.data).is_null() {
                    hprintf(
                        (*(pcw.data as *mut remoteshell)).fhsend,
                        b"remote %d locked \"%s\"\n\0" as *const u8
                            as *const libc::c_char as *mut libc::c_char,
                        pcw.qid,
                        (*mx_0).name,
                    );
                } else if !(pcw.data).is_null() {
                    hprintf(
                        (*(pcw.data as *mut client)).peer,
                        b"locked \"%s\"\n\0" as *const u8 as *const libc::c_char
                            as *mut libc::c_char,
                        (*mx_0).name,
                    );
                }
            }
            if (*mx_0).state <= 0 as libc::c_int {
                free((*mx_0).name as *mut libc::c_void);
                if !((*mx_0).next).is_null() {
                    (*(*mx_0).next).prev = (*mx_0).prev;
                }
                if !((*mx_0).prev).is_null() {
                    (*(*mx_0).prev).next = (*mx_0).next;
                } else {
                    (*ps).imutexlist = (*mx_0).next;
                }
                free(mx_0 as *mut libc::c_void);
            }
        }
        ret = 0 as libc::c_int;
    } else if strcmp(
        *cmd.offset(0 as libc::c_int as isize),
        b"mutex\0" as *const u8 as *const libc::c_char,
    ) == 0 as libc::c_int
    {
        let mut mx_1: *mut imutex = 0 as *mut imutex;
        let mut name_1: *mut libc::c_char = 0 as *mut libc::c_char;
        let mut lcnt: libc::c_int = 0;
        name_1 = 0 as *mut libc::c_char;
        i = imutex_mutex_get_params(cmd, &mut name_1);
        if i != 0 || name_1.is_null() {
            hprintf(
                (*cl).peer,
                b"error unexpected arguments for '%s'\n\0" as *const u8
                    as *const libc::c_char as *mut libc::c_char,
                *cmd.offset(0 as libc::c_int as isize),
            );
        } else {
            mx_1 = get_imutex_by_name((*ps).imutexlist, name_1);
            if !mx_1.is_null() {
                lcnt = (*mx_1).npclient;
            } else {
                lcnt = 0 as libc::c_int;
            }
            if qid >= 0 as libc::c_int && !rs.is_null() {
                hprintf(
                    (*rs).fhsend,
                    b"remote %d mutex \"%s\" %d\n\0" as *const u8 as *const libc::c_char
                        as *mut libc::c_char,
                    qid,
                    name_1,
                    lcnt,
                );
            } else if !cl.is_null() {
                hprintf(
                    (*cl).peer,
                    b"mutex \"%s\" %d\n\0" as *const u8 as *const libc::c_char
                        as *mut libc::c_char,
                    name_1,
                    lcnt,
                );
            }
        }
        ret = 0 as libc::c_int;
    } else if strcmp(
        *cmd.offset(0 as libc::c_int as isize),
        b"exit\0" as *const u8 as *const libc::c_char,
    ) == 0 as libc::c_int
    {
        hprintf(
            (*cl).peer,
            b"bye\n\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        );
        ret = 1 as libc::c_int;
    } else {
        hprintf(
            (*cl).peer,
            b"error unexpected command '%s'\n\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            *cmd.offset(0 as libc::c_int as isize),
        );
        ret = 0 as libc::c_int;
    }
    return ret;
}
pub unsafe extern "C" fn remote_client_master_parse_command(
    mut line: *mut libc::c_char,
    mut ps: *mut parallelstatus,
    mut qid: libc::c_int,
    mut dd: *mut libc::c_void,
) -> libc::c_int {
    let mut ret: libc::c_int = 0;
    let mut cmd: *mut *mut libc::c_char = 0 as *mut *mut libc::c_char;
    cmd = tokenize_spaces_dyn(line);
    if cmd.is_null() {
        return 0 as libc::c_int
    } else if (*cmd.offset(0 as libc::c_int as isize)).is_null() {
        free(cmd as *mut libc::c_void);
        return 0 as libc::c_int;
    }
    ret = remote_client_master_parse_command_tokens(cmd, ps, qid, dd);
    free(cmd as *mut libc::c_void);
    return ret;
}
pub unsafe extern "C" fn pexec_submit(
    mut p: *mut paralleldata,
    mut ps: *mut parallelstatus,
    mut ntp: *mut numhashtable,
    mut ntf: *mut numhashtable,
    mut rs: *mut remoteshell,
    mut cp: *mut parameter,
    mut n: libc::c_int,
) -> libc::c_int {
    let mut cc: *mut child = 0 as *mut child;
    cc = xmalloc(::std::mem::size_of::<child>() as libc::c_ulong) as *mut child;
    (*cc).fdstdout = -(1 as libc::c_int);
    (*cc).fdstderr = -(1 as libc::c_int);
    (*cc).next = (*ps).childlist;
    (*cc).prev = 0 as *mut child;
    if !((*ps).childlist).is_null() {
        (*(*ps).childlist).prev = cc;
    }
    (*ps).childlist = cc;
    (*ps).achild += 1;
    (*ps).achild;
    (*rs).achild += 1;
    (*rs).achild;
    (*cc).rs = rs;
    if (*rs).num_processes == 0 {
        (*rs).estatus = 0 as libc::c_int;
    }
    linebuffer_reset(&mut (*cc).lout);
    linebuffer_reset(&mut (*cc).lerr);
    if (*rs).pid >= 0 as libc::c_int {
        (*cc).id = n;
        (*cc).pid = -(1 as libc::c_int);
        send_task(p, cp, cc);
        (*ps).npending += 1;
        (*ps).npending;
        numhash_add(ntp, n, 0 as *mut libc::c_void);
    } else {
        (*cc).id = n;
        (*cc).pid = submit_task(p, cp, cc, 0 as libc::c_int, ps);
        (*ps).npending += 1;
        (*ps).npending;
        numhash_add(ntp, n, 0 as *mut libc::c_void);
        if (*cc).pid < 0 as libc::c_int {
            (*ps).nfinished += 1;
            (*ps).nfinished;
            numhash_add(ntf, n, 0 as *mut libc::c_void);
            log_message(
                (*p).log,
                1 as libc::c_int,
                cp,
                b"error while invoking task: %s\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                logmsg_submit_task[-(*cc).pid as usize],
            );
            if !((*cc).next).is_null() {
                (*(*cc).next).prev = (*cc).prev;
            }
            if !((*cc).prev).is_null() {
                (*(*cc).prev).next = (*cc).next;
            } else {
                (*ps).childlist = (*cc).next;
            }
            free(cc as *mut libc::c_void);
            (*ps).achild -= 1;
            (*ps).achild;
            (*rs).achild -= 1;
            (*rs).achild;
        }
    }
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn pexec_get_free_remote_shell(
    mut rshells: *mut remoteshell,
    mut nrshell: libc::c_int,
) -> *mut remoteshell {
    let mut rs: *mut remoteshell = 0 as *mut remoteshell;
    let mut r: libc::c_int = 0;
    r = 0 as libc::c_int;
    rs = rshells;
    while r < nrshell {
        if (0 as libc::c_int) < (*rs).num_processes && (*rs).achild < (*rs).num_processes
            || (*rs).estatus >= 2 as libc::c_int
        {
            return rs;
        }
        r += 1;
        r;
        rs = rs.offset(1);
        rs;
    }
    return 0 as *mut remoteshell;
}
pub unsafe extern "C" fn pexec_do_parallelized_execution(
    mut p: *mut paralleldata,
    mut params: *mut parameter,
    mut nparam: libc::c_int,
    mut rshells: *mut remoteshell,
    mut nrshell: libc::c_int,
    mut sock: libc::c_int,
    mut hsck: libc::c_int,
) -> libc::c_int {
    let mut cc: *mut child = 0 as *mut child;
    let mut chldact: sigaction = sigaction {
        __sigaction_handler: C2RustUnnamed_10 {
            sa_handler: None,
        },
        sa_mask: __sigset_t { __val: [0; 16] },
        sa_flags: 0,
        sa_restorer: None,
    };
    let mut sci: signalinfo = signalinfo {
        signal: 0,
        pid: 0,
        exitstatus: 0,
        exitsignal: 0,
    };
    let mut ntp: numhashtable = numhashtable {
        table: numhashnode {
            node: C2RustUnnamed_13 {
                leaves: 0 as *mut numhashnode,
            },
            nchild: 0,
        },
        depth: 0,
        bitsize: 0,
    };
    let mut ntf: numhashtable = numhashtable {
        table: numhashnode {
            node: C2RustUnnamed_13 {
                leaves: 0 as *mut numhashnode,
            },
            nchild: 0,
        },
        depth: 0,
        bitsize: 0,
    };
    let mut i: libc::c_int = 0;
    let mut n: libc::c_int = 0;
    let mut r: libc::c_int = 0;
    let mut cp: *mut parameter = 0 as *mut parameter;
    let mut set: fd_set = fd_set { fds_bits: [0; 16] };
    let mut spipe: libc::c_int = 0;
    let mut max: libc::c_int = 0;
    let mut status: libc::c_int = 0;
    let mut buff: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut buffsize: libc::c_int = 0;
    let mut rs: *mut remoteshell = 0 as *mut remoteshell;
    let mut cl: *mut client = 0 as *mut client;
    let mut ps: parallelstatus = parallelstatus {
        childlist: 0 as *mut child,
        achild: 0,
        sock: 0,
        hsck: 0,
        clientlist: 0 as *mut client,
        imutexlist: 0 as *mut imutex,
        dqueuelist: 0 as *mut dqueue,
        iqueue: 0,
        nfinished: 0,
        npending: 0,
        nparam: 0,
        t0: 0,
    };
    let mut lhyp: linebuffer = linebuffer {
        buffer: 0 as *mut libc::c_char,
        length: 0,
    };
    if pipe(sig_pipe.as_mut_ptr()) < 0 as libc::c_int {
        return -(1 as libc::c_int);
    }
    spipe = sig_pipe[0 as libc::c_int as usize];
    chldact
        .__sigaction_handler
        .sa_handler = Some(sig_act_child as unsafe extern "C" fn(libc::c_int) -> ());
    sigemptyset(&mut chldact.sa_mask);
    chldact.sa_flags = 1 as libc::c_int | 0x10000000 as libc::c_int;
    sigaction(17 as libc::c_int, &mut chldact, 0 as *mut sigaction);
    ps.childlist = 0 as *mut child;
    ps.achild = 0 as libc::c_int;
    r = 0 as libc::c_int;
    rs = rshells;
    while r < nrshell {
        (*rs).achild = 0 as libc::c_int;
        (*rs).estatus = 0 as libc::c_int;
        r += 1;
        r;
        rs = rs.offset(1);
        rs;
    }
    ps.nfinished = 0 as libc::c_int;
    numhash_init(&mut ntf, get_bit_size(nparam), 4 as libc::c_int);
    ps.npending = 0 as libc::c_int;
    numhash_init(&mut ntp, get_bit_size(nparam), 4 as libc::c_int);
    ps.nparam = nparam;
    buffsize = getpagesize();
    buff = xmalloc(buffsize as size_t) as *mut libc::c_char;
    ps.sock = sock;
    ps.hsck = hsck;
    ps.clientlist = 0 as *mut client;
    ps.imutexlist = 0 as *mut imutex;
    linebuffer_reset(&mut lhyp);
    time(&mut ps.t0);
    while ps.nfinished < ps.nparam {
        loop {
            n = numhash_get_smallest_free(&mut ntp);
            if n < 0 as libc::c_int || n >= nparam {
                break;
            }
            rs = pexec_get_free_remote_shell(rshells, nrshell);
            if rs.is_null() {
                break;
            }
            cp = &mut *params.offset(n as isize) as *mut parameter;
            pexec_submit(p, &mut ps, &mut ntp, &mut ntf, rs, cp, n);
        }
        rs = rshells;
        r = 0 as libc::c_int;
        while r < nrshell {
            if (*rs).num_processes == 0 && (*rs).estatus == 0 {
                (*rs).estatus = 1 as libc::c_int;
                if (*rs).pid >= 0 as libc::c_int {
                    hprintf(
                        (*rs).fhsend,
                        b"request\n\0" as *const u8 as *const libc::c_char
                            as *mut libc::c_char,
                    );
                } else if hsck >= 0 as libc::c_int {
                    hprintf(
                        hsck,
                        b"request\n\0" as *const u8 as *const libc::c_char
                            as *mut libc::c_char,
                    );
                }
            }
            r += 1;
            r;
            rs = rs.offset(1);
            rs;
        }
        if ps.nfinished >= nparam {
            break;
        }
        let mut __d0: libc::c_int = 0;
        let mut __d1: libc::c_int = 0;
        let fresh25 = &mut __d0;
        let fresh26;
        let fresh27 = (::std::mem::size_of::<fd_set>() as libc::c_ulong)
            .wrapping_div(::std::mem::size_of::<__fd_mask>() as libc::c_ulong);
        let fresh28 = &mut __d1;
        let fresh29;
        let fresh30 = &mut *(set.fds_bits).as_mut_ptr().offset(0 as libc::c_int as isize)
            as *mut __fd_mask;
        asm!(
            "cld; rep; stosq", inlateout("cx")
            c2rust_asm_casts::AsmCast::cast_in(fresh25, fresh27) => fresh26,
            inlateout("di") c2rust_asm_casts::AsmCast::cast_in(fresh28, fresh30) =>
            fresh29, inlateout("ax") 0 as libc::c_int => _, options(preserves_flags,
            att_syntax)
        );
        c2rust_asm_casts::AsmCast::cast_out(fresh25, fresh27, fresh26);
        c2rust_asm_casts::AsmCast::cast_out(fresh28, fresh30, fresh29);
        set
            .fds_bits[(spipe
            / (8 as libc::c_int
                * ::std::mem::size_of::<__fd_mask>() as libc::c_ulong as libc::c_int))
            as usize]
            |= ((1 as libc::c_ulong)
                << spipe
                    % (8 as libc::c_int
                        * ::std::mem::size_of::<__fd_mask>() as libc::c_ulong
                            as libc::c_int)) as __fd_mask;
        max = spipe;
        cc = ps.childlist;
        while !cc.is_null() {
            if !((*cc).id < 0 as libc::c_int || (*cc).pid < 0 as libc::c_int) {
                if (*cc).fdstdout >= 0 as libc::c_int {
                    set
                        .fds_bits[((*cc).fdstdout
                        / (8 as libc::c_int
                            * ::std::mem::size_of::<__fd_mask>() as libc::c_ulong
                                as libc::c_int)) as usize]
                        |= ((1 as libc::c_ulong)
                            << (*cc).fdstdout
                                % (8 as libc::c_int
                                    * ::std::mem::size_of::<__fd_mask>() as libc::c_ulong
                                        as libc::c_int)) as __fd_mask;
                    if max < (*cc).fdstdout {
                        max = (*cc).fdstdout;
                    }
                }
                if (*cc).fdstderr >= 0 as libc::c_int {
                    set
                        .fds_bits[((*cc).fdstderr
                        / (8 as libc::c_int
                            * ::std::mem::size_of::<__fd_mask>() as libc::c_ulong
                                as libc::c_int)) as usize]
                        |= ((1 as libc::c_ulong)
                            << (*cc).fdstderr
                                % (8 as libc::c_int
                                    * ::std::mem::size_of::<__fd_mask>() as libc::c_ulong
                                        as libc::c_int)) as __fd_mask;
                    if max < (*cc).fdstderr {
                        max = (*cc).fdstderr;
                    }
                }
            }
            cc = (*cc).next;
        }
        rs = rshells;
        r = 0 as libc::c_int;
        while r < nrshell {
            if !((*rs).pid < 0 as libc::c_int || (*rs).fhsend < 0 as libc::c_int
                || (*rs).fhrecv < 0 as libc::c_int)
            {
                set
                    .fds_bits[((*rs).fhrecv
                    / (8 as libc::c_int
                        * ::std::mem::size_of::<__fd_mask>() as libc::c_ulong
                            as libc::c_int)) as usize]
                    |= ((1 as libc::c_ulong)
                        << (*rs).fhrecv
                            % (8 as libc::c_int
                                * ::std::mem::size_of::<__fd_mask>() as libc::c_ulong
                                    as libc::c_int)) as __fd_mask;
                if max < (*rs).fhrecv {
                    max = (*rs).fhrecv;
                }
            }
            r += 1;
            r;
            rs = rs.offset(1);
            rs;
        }
        if sock >= 0 as libc::c_int {
            set
                .fds_bits[(sock
                / (8 as libc::c_int
                    * ::std::mem::size_of::<__fd_mask>() as libc::c_ulong
                        as libc::c_int)) as usize]
                |= ((1 as libc::c_ulong)
                    << sock
                        % (8 as libc::c_int
                            * ::std::mem::size_of::<__fd_mask>() as libc::c_ulong
                                as libc::c_int)) as __fd_mask;
            if max < sock {
                max = sock;
            }
        }
        if hsck >= 0 as libc::c_int {
            set
                .fds_bits[(hsck
                / (8 as libc::c_int
                    * ::std::mem::size_of::<__fd_mask>() as libc::c_ulong
                        as libc::c_int)) as usize]
                |= ((1 as libc::c_ulong)
                    << hsck
                        % (8 as libc::c_int
                            * ::std::mem::size_of::<__fd_mask>() as libc::c_ulong
                                as libc::c_int)) as __fd_mask;
            if max < hsck {
                max = hsck;
            }
        }
        cl = ps.clientlist;
        while !cl.is_null() {
            set
                .fds_bits[((*cl).peer
                / (8 as libc::c_int
                    * ::std::mem::size_of::<__fd_mask>() as libc::c_ulong
                        as libc::c_int)) as usize]
                |= ((1 as libc::c_ulong)
                    << (*cl).peer
                        % (8 as libc::c_int
                            * ::std::mem::size_of::<__fd_mask>() as libc::c_ulong
                                as libc::c_int)) as __fd_mask;
            if max < (*cl).peer {
                max = (*cl).peer;
            }
            cl = (*cl).next;
        }
        i = select(
            max + 1 as libc::c_int,
            &mut set,
            0 as *mut fd_set,
            0 as *mut fd_set,
            0 as *mut timeval,
        );
        if i < 0 as libc::c_int && *__errno_location() == 4 as libc::c_int {
            continue;
        }
        if set
            .fds_bits[(spipe
            / (8 as libc::c_int
                * ::std::mem::size_of::<__fd_mask>() as libc::c_ulong as libc::c_int))
            as usize]
            & ((1 as libc::c_ulong)
                << spipe
                    % (8 as libc::c_int
                        * ::std::mem::size_of::<__fd_mask>() as libc::c_ulong
                            as libc::c_int)) as __fd_mask
            != 0 as libc::c_int as libc::c_long
        {
            if !(read_signalinfo(spipe, &mut sci) > 0 as libc::c_int) {
                fprintf(
                    stderr,
                    dcgettext(
                        0 as *const libc::c_char,
                        b"%s: fatal error: read_signalinfo() failed.\n\0" as *const u8
                            as *const libc::c_char,
                        5 as libc::c_int,
                    ),
                    progbasename,
                );
                exit(1 as libc::c_int);
            }
            if sci.exitsignal < 0 as libc::c_int {
                cc = get_child_by_pid(ps.childlist, sci.pid);
                n = (*cc).id;
                (*cc).id = -(1 as libc::c_int);
                (*cc).pid = -(1 as libc::c_int);
                cp = &mut *params.offset(n as isize) as *mut parameter;
                numhash_add(&mut ntf, n, 0 as *mut libc::c_void);
                ps.nfinished += 1;
                ps.nfinished;
                if (*cc).fdstdout >= 0 as libc::c_int && !((*p).fwout).is_null() {
                    while fd_avail((*cc).fdstdout) != 0 {
                        n = read(
                            (*cc).fdstdout,
                            buff as *mut libc::c_void,
                            buffsize as size_t,
                        ) as libc::c_int;
                        if n > 0 as libc::c_int {
                            write_output(
                                buff,
                                n as size_t,
                                (*p).fwout,
                                (*p).formatout,
                                (*p).omit_newlines,
                                &mut (*cc).lout,
                                cp,
                            );
                        } else if !(n < 0 as libc::c_int
                            && *__errno_location() == 4 as libc::c_int)
                        {
                            break;
                        }
                    }
                    write_output(
                        0 as *mut libc::c_char,
                        0 as libc::c_int as size_t,
                        (*p).fwout,
                        (*p).formatout,
                        (*p).omit_newlines,
                        &mut (*cc).lout,
                        cp,
                    );
                    close((*cc).fdstdout);
                    (*cc).fdstdout = -(1 as libc::c_int);
                }
                if (*cc).fdstderr >= 0 as libc::c_int && !((*p).fwerr).is_null() {
                    while fd_avail((*cc).fdstderr) != 0 {
                        n = read(
                            (*cc).fdstderr,
                            buff as *mut libc::c_void,
                            buffsize as size_t,
                        ) as libc::c_int;
                        if n > 0 as libc::c_int {
                            write_output(
                                buff,
                                n as size_t,
                                (*p).fwerr,
                                (*p).formaterr,
                                (*p).omit_newlines,
                                &mut (*cc).lerr,
                                cp,
                            );
                        } else if !(n < 0 as libc::c_int
                            && *__errno_location() == 4 as libc::c_int)
                        {
                            break;
                        }
                    }
                    write_output(
                        0 as *mut libc::c_char,
                        0 as libc::c_int as size_t,
                        (*p).fwerr,
                        (*p).formaterr,
                        (*p).omit_newlines,
                        &mut (*cc).lerr,
                        cp,
                    );
                    close((*cc).fdstderr);
                    (*cc).fdstderr = -(1 as libc::c_int);
                }
                if !((*cc).next).is_null() {
                    (*(*cc).next).prev = (*cc).prev;
                }
                if !((*cc).prev).is_null() {
                    (*(*cc).prev).next = (*cc).next;
                } else {
                    ps.childlist = (*cc).next;
                }
                (*(*cc).rs).achild -= 1;
                (*(*cc).rs).achild;
                ps.achild -= 1;
                ps.achild;
                if (*(*cc).rs).num_processes == 0 && hsck >= 0 as libc::c_int {
                    hprintf(
                        hsck,
                        b"ready\n\0" as *const u8 as *const libc::c_char
                            as *mut libc::c_char,
                    );
                }
                free(cc as *mut libc::c_void);
            } else if (*p).fallback_to_die != 0 {
                close(sig_pipe[0 as libc::c_int as usize]);
                sig_pipe[0 as libc::c_int as usize] = -(1 as libc::c_int);
                close(sig_pipe[1 as libc::c_int as usize]);
                sig_pipe[1 as libc::c_int as usize] = -(1 as libc::c_int);
                cc = ps.childlist;
                while !cc.is_null() {
                    if !((*cc).pid < 0 as libc::c_int) {
                        kill((*cc).pid, 9 as libc::c_int);
                        waitpid((*cc).pid, &mut status, 0 as libc::c_int);
                        free(cc as *mut libc::c_void);
                    }
                    cc = (*cc).next;
                }
                ps.childlist = 0 as *mut child;
                ps.achild = 0 as libc::c_int;
                return 1 as libc::c_int;
            } else {
                cc = get_child_by_pid(ps.childlist, sci.pid);
                numhash_remove(&mut ntp, n);
                ps.npending -= 1;
                ps.npending;
                if !((*cc).next).is_null() {
                    (*(*cc).next).prev = (*cc).prev;
                }
                if !((*cc).prev).is_null() {
                    (*(*cc).prev).next = (*cc).next;
                } else {
                    ps.childlist = (*cc).next;
                }
                (*(*cc).rs).achild -= 1;
                (*(*cc).rs).achild;
                ps.achild -= 1;
                ps.achild;
                free(cc as *mut libc::c_void);
            }
        }
        cc = ps.childlist;
        while !cc.is_null() {
            if !((*cc).id < 0 as libc::c_int || (*cc).pid < 0 as libc::c_int) {
                cp = &mut *params.offset((*cc).id as isize) as *mut parameter;
                if (*cc).fdstdout >= 0 as libc::c_int && !((*p).fwout).is_null()
                    && set
                        .fds_bits[((*cc).fdstdout
                        / (8 as libc::c_int
                            * ::std::mem::size_of::<__fd_mask>() as libc::c_ulong
                                as libc::c_int)) as usize]
                        & ((1 as libc::c_ulong)
                            << (*cc).fdstdout
                                % (8 as libc::c_int
                                    * ::std::mem::size_of::<__fd_mask>() as libc::c_ulong
                                        as libc::c_int)) as __fd_mask
                        != 0 as libc::c_int as libc::c_long
                {
                    n = read(
                        (*cc).fdstdout,
                        buff as *mut libc::c_void,
                        buffsize as size_t,
                    ) as libc::c_int;
                    if n > 0 as libc::c_int {
                        write_output(
                            buff,
                            n as size_t,
                            (*p).fwout,
                            (*p).formatout,
                            (*p).omit_newlines,
                            &mut (*cc).lout,
                            cp,
                        );
                    } else if !(n < 0 as libc::c_int
                        && *__errno_location() == 4 as libc::c_int)
                    {
                        close((*cc).fdstdout);
                        (*cc).fdstdout = -(1 as libc::c_int);
                    }
                }
                if (*cc).fdstderr >= 0 as libc::c_int && !((*p).fwerr).is_null()
                    && set
                        .fds_bits[((*cc).fdstderr
                        / (8 as libc::c_int
                            * ::std::mem::size_of::<__fd_mask>() as libc::c_ulong
                                as libc::c_int)) as usize]
                        & ((1 as libc::c_ulong)
                            << (*cc).fdstderr
                                % (8 as libc::c_int
                                    * ::std::mem::size_of::<__fd_mask>() as libc::c_ulong
                                        as libc::c_int)) as __fd_mask
                        != 0 as libc::c_int as libc::c_long
                {
                    n = read(
                        (*cc).fdstderr,
                        buff as *mut libc::c_void,
                        buffsize as size_t,
                    ) as libc::c_int;
                    if n > 0 as libc::c_int {
                        write_output(
                            buff,
                            n as size_t,
                            (*p).fwerr,
                            (*p).formaterr,
                            (*p).omit_newlines,
                            &mut (*cc).lerr,
                            cp,
                        );
                    } else if !(n < 0 as libc::c_int
                        && *__errno_location() == 4 as libc::c_int)
                    {
                        close((*cc).fdstderr);
                        (*cc).fdstderr = -(1 as libc::c_int);
                    }
                }
            }
            cc = (*cc).next;
        }
        rs = rshells;
        r = 0 as libc::c_int;
        while r < nrshell {
            let mut len: libc::c_int = 0;
            let mut line: *mut libc::c_char = 0 as *mut libc::c_char;
            let mut cmd: *mut *mut libc::c_char = 0 as *mut *mut libc::c_char;
            if !((*rs).pid < 0 as libc::c_int || (*rs).fhsend < 0 as libc::c_int
                || (*rs).fhrecv < 0 as libc::c_int)
            {
                if set
                    .fds_bits[((*rs).fhrecv
                    / (8 as libc::c_int
                        * ::std::mem::size_of::<__fd_mask>() as libc::c_ulong
                            as libc::c_int)) as usize]
                    & ((1 as libc::c_ulong)
                        << (*rs).fhrecv
                            % (8 as libc::c_int
                                * ::std::mem::size_of::<__fd_mask>() as libc::c_ulong
                                    as libc::c_int)) as __fd_mask
                    != 0 as libc::c_int as libc::c_long
                {
                    n = read((*rs).fhrecv, buff as *mut libc::c_void, buffsize as size_t)
                        as libc::c_int;
                    if n == 0 as libc::c_int {
                        break;
                    }
                    if !(n < 0 as libc::c_int && *__errno_location() == 4 as libc::c_int)
                    {
                        linebuffer_concatenate(&mut (*rs).lrsh, buff, n as size_t);
                        loop {
                            line = linebuffer_fetch(&mut (*rs).lrsh);
                            if line.is_null() {
                                break;
                            }
                            cmd = tokenize_spaces_dyn(line);
                            if !cmd.is_null()
                                && !(*cmd.offset(0 as libc::c_int as isize)).is_null()
                            {
                                if strcmp(
                                    *cmd.offset(0 as libc::c_int as isize),
                                    b"output\0" as *const u8 as *const libc::c_char,
                                ) == 0 as libc::c_int
                                    && !(*cmd.offset(1 as libc::c_int as isize)).is_null()
                                    && !(*cmd.offset(2 as libc::c_int as isize)).is_null()
                                {
                                    sscanf(
                                        *cmd.offset(1 as libc::c_int as isize),
                                        b"%d\0" as *const u8 as *const libc::c_char,
                                        &mut n as *mut libc::c_int,
                                    );
                                    cc = get_child_by_id(ps.childlist, n);
                                    cp = &mut *params.offset(n as isize) as *mut parameter;
                                    len = daemon_commandtoken_unescape(
                                        *cmd.offset(2 as libc::c_int as isize),
                                    );
                                    write_output(
                                        *cmd.offset(2 as libc::c_int as isize),
                                        len as size_t,
                                        (*p).fwout,
                                        (*p).formatout,
                                        (*p).omit_newlines,
                                        &mut (*cc).lout,
                                        cp,
                                    );
                                } else if strcmp(
                                    *cmd.offset(0 as libc::c_int as isize),
                                    b"error\0" as *const u8 as *const libc::c_char,
                                ) == 0 as libc::c_int
                                    && !(*cmd.offset(1 as libc::c_int as isize)).is_null()
                                    && !(*cmd.offset(2 as libc::c_int as isize)).is_null()
                                {
                                    sscanf(
                                        *cmd.offset(1 as libc::c_int as isize),
                                        b"%d\0" as *const u8 as *const libc::c_char,
                                        &mut n as *mut libc::c_int,
                                    );
                                    cc = get_child_by_id(ps.childlist, n);
                                    cp = &mut *params.offset(n as isize) as *mut parameter;
                                    len = daemon_commandtoken_unescape(
                                        *cmd.offset(2 as libc::c_int as isize),
                                    );
                                    write_output(
                                        *cmd.offset(2 as libc::c_int as isize),
                                        len as size_t,
                                        (*p).fwerr,
                                        (*p).formaterr,
                                        (*p).omit_newlines,
                                        &mut (*cc).lerr,
                                        cp,
                                    );
                                } else if strcmp(
                                    *cmd.offset(0 as libc::c_int as isize),
                                    b"finish\0" as *const u8 as *const libc::c_char,
                                ) == 0 as libc::c_int
                                    && !(*cmd.offset(1 as libc::c_int as isize)).is_null()
                                {
                                    sscanf(
                                        *cmd.offset(1 as libc::c_int as isize),
                                        b"%d\0" as *const u8 as *const libc::c_char,
                                        &mut n as *mut libc::c_int,
                                    );
                                    cc = get_child_by_id(ps.childlist, n);
                                    numhash_add(&mut ntf, n, 0 as *mut libc::c_void);
                                    ps.nfinished += 1;
                                    ps.nfinished;
                                    if !((*cc).next).is_null() {
                                        (*(*cc).next).prev = (*cc).prev;
                                    }
                                    if !((*cc).prev).is_null() {
                                        (*(*cc).prev).next = (*cc).next;
                                    } else {
                                        ps.childlist = (*cc).next;
                                    }
                                    (*(*cc).rs).achild -= 1;
                                    (*(*cc).rs).achild;
                                    ps.achild -= 1;
                                    ps.achild;
                                    if (*(*cc).rs).num_processes == 0 {
                                        hprintf(
                                            (*(*cc).rs).fhsend,
                                            b"ready\n\0" as *const u8 as *const libc::c_char
                                                as *mut libc::c_char,
                                        );
                                    }
                                    free(cc as *mut libc::c_void);
                                } else if strcmp(
                                    *cmd.offset(0 as libc::c_int as isize),
                                    b"remote\0" as *const u8 as *const libc::c_char,
                                ) == 0 as libc::c_int
                                    && !(*cmd.offset(1 as libc::c_int as isize)).is_null()
                                    && !(*cmd.offset(2 as libc::c_int as isize)).is_null()
                                {
                                    let mut qid: libc::c_int = 0;
                                    let mut dd: *mut libc::c_void = 0 as *mut libc::c_void;
                                    if sscanf(
                                        *cmd.offset(1 as libc::c_int as isize),
                                        b"%d\0" as *const u8 as *const libc::c_char,
                                        &mut qid as *mut libc::c_int,
                                    ) == 1 as libc::c_int
                                    {
                                        if qid >= 0 as libc::c_int {
                                            dd = rs as *mut libc::c_void;
                                        } else {
                                            dd = 0 as *mut libc::c_void;
                                        }
                                        remote_client_master_parse_command_tokens(
                                            cmd.offset(2 as libc::c_int as isize),
                                            &mut ps,
                                            qid,
                                            dd,
                                        );
                                    }
                                } else if strcmp(
                                    *cmd.offset(0 as libc::c_int as isize),
                                    b"acknowledged\0" as *const u8 as *const libc::c_char,
                                ) == 0 as libc::c_int
                                {
                                    (*rs).estatus = 2 as libc::c_int;
                                }
                            }
                            if !cmd.is_null() {
                                free(cmd as *mut libc::c_void);
                            }
                            free(line as *mut libc::c_void);
                        }
                    }
                }
            }
            r += 1;
            r;
            rs = rs.offset(1);
            rs;
        }
        cl = ps.clientlist;
        while !cl.is_null() {
            let mut line_0: *mut libc::c_char = 0 as *mut libc::c_char;
            let mut closepeer: libc::c_int = 0;
            if set
                .fds_bits[((*cl).peer
                / (8 as libc::c_int
                    * ::std::mem::size_of::<__fd_mask>() as libc::c_ulong
                        as libc::c_int)) as usize]
                & ((1 as libc::c_ulong)
                    << (*cl).peer
                        % (8 as libc::c_int
                            * ::std::mem::size_of::<__fd_mask>() as libc::c_ulong
                                as libc::c_int)) as __fd_mask
                != 0 as libc::c_int as libc::c_long
            {
                n = read((*cl).peer, buff as *mut libc::c_void, buffsize as size_t)
                    as libc::c_int;
                if n == 0 as libc::c_int {
                    close((*cl).peer);
                    linebuffer_free(&mut (*cl).lcli);
                    imutex_cleanup(
                        ps.imutexlist,
                        -(1 as libc::c_int),
                        cl as *mut libc::c_void,
                    );
                    if !((*cl).next).is_null() {
                        (*(*cl).next).prev = (*cl).prev;
                    }
                    if !((*cl).prev).is_null() {
                        (*(*cl).prev).next = (*cl).next;
                    } else {
                        ps.clientlist = (*cl).next;
                    }
                    free(cl as *mut libc::c_void);
                    break;
                } else if !(n < 0 as libc::c_int
                    && *__errno_location() == 4 as libc::c_int)
                {
                    linebuffer_concatenate(&mut (*cl).lcli, buff, n as size_t);
                    closepeer = 0 as libc::c_int;
                    loop {
                        line_0 = linebuffer_fetch(&mut (*cl).lcli);
                        if line_0.is_null() {
                            break;
                        }
                        closepeer = remote_client_master_parse_command(
                            line_0,
                            &mut ps,
                            -(1 as libc::c_int),
                            cl as *mut libc::c_void,
                        );
                        free(line_0 as *mut libc::c_void);
                        if closepeer != 0 {
                            break;
                        }
                    }
                    if closepeer != 0 {
                        close((*cl).peer);
                        linebuffer_free(&mut (*cl).lcli);
                        imutex_cleanup(
                            ps.imutexlist,
                            -(1 as libc::c_int),
                            cl as *mut libc::c_void,
                        );
                        if !((*cl).next).is_null() {
                            (*(*cl).next).prev = (*cl).prev;
                        }
                        if !((*cl).prev).is_null() {
                            (*(*cl).prev).next = (*cl).next;
                        } else {
                            ps.clientlist = (*cl).next;
                        }
                        free(cl as *mut libc::c_void);
                        break;
                    }
                }
            }
            cl = (*cl).next;
        }
        if hsck >= 0 as libc::c_int
            && set
                .fds_bits[(hsck
                / (8 as libc::c_int
                    * ::std::mem::size_of::<__fd_mask>() as libc::c_ulong
                        as libc::c_int)) as usize]
                & ((1 as libc::c_ulong)
                    << hsck
                        % (8 as libc::c_int
                            * ::std::mem::size_of::<__fd_mask>() as libc::c_ulong
                                as libc::c_int)) as __fd_mask
                != 0 as libc::c_int as libc::c_long
        {
            let mut line_1: *mut libc::c_char = 0 as *mut libc::c_char;
            let mut cmd_0: *mut *mut libc::c_char = 0 as *mut *mut libc::c_char;
            n = read(hsck, buff as *mut libc::c_void, buffsize as size_t) as libc::c_int;
            if n > 0 as libc::c_int {
                linebuffer_concatenate(&mut lhyp, buff, n as size_t);
            }
            loop {
                line_1 = linebuffer_fetch(&mut lhyp);
                if line_1.is_null() {
                    break;
                }
                cmd_0 = tokenize_spaces_dyn(line_1);
                if !cmd_0.is_null() {
                    if strcmp(
                        *cmd_0.offset(0 as libc::c_int as isize),
                        b"acknowledged\0" as *const u8 as *const libc::c_char,
                    ) == 0 as libc::c_int
                    {
                        rs = rshells;
                        r = 0 as libc::c_int;
                        while r < nrshell {
                            if (*rs).num_processes == 0 {
                                (*rs).estatus = 2 as libc::c_int;
                            }
                            r += 1;
                            r;
                            rs = rs.offset(1);
                            rs;
                        }
                    }
                    free(cmd_0 as *mut libc::c_void);
                }
                free(line_1 as *mut libc::c_void);
            }
        }
        if sock >= 0 as libc::c_int
            && set
                .fds_bits[(sock
                / (8 as libc::c_int
                    * ::std::mem::size_of::<__fd_mask>() as libc::c_ulong
                        as libc::c_int)) as usize]
                & ((1 as libc::c_ulong)
                    << sock
                        % (8 as libc::c_int
                            * ::std::mem::size_of::<__fd_mask>() as libc::c_ulong
                                as libc::c_int)) as __fd_mask
                != 0 as libc::c_int as libc::c_long
        {
            cl = xmalloc(::std::mem::size_of::<client>() as libc::c_ulong)
                as *mut client;
            (*cl)
                .peer = accept(
                sock,
                __SOCKADDR_ARG {
                    __sockaddr__: 0 as *mut libc::c_void as *mut sockaddr,
                },
                0 as *mut socklen_t,
            );
            linebuffer_reset(&mut (*cl).lcli);
            (*cl).next = ps.clientlist;
            (*cl).prev = 0 as *mut client;
            if !(ps.clientlist).is_null() {
                (*ps.clientlist).prev = cl;
            }
            ps.clientlist = cl;
            (*cl).status = 0 as libc::c_int;
        }
    }
    rs = rshells;
    r = 0 as libc::c_int;
    while r < nrshell {
        if (*rs).num_processes == 0 {
            if (*rs).pid >= 0 as libc::c_int {
                hprintf(
                    (*rs).fhsend,
                    b"completed\n\0" as *const u8 as *const libc::c_char
                        as *mut libc::c_char,
                );
            } else if hsck >= 0 as libc::c_int {
                hprintf(
                    hsck,
                    b"completed\n\0" as *const u8 as *const libc::c_char
                        as *mut libc::c_char,
                );
            }
        }
        r += 1;
        r;
        rs = rs.offset(1);
        rs;
    }
    free(buff as *mut libc::c_void);
    numhash_free(&mut ntp);
    numhash_free(&mut ntf);
    signal(17 as libc::c_int, None);
    close(sig_pipe[0 as libc::c_int as usize]);
    close(sig_pipe[1 as libc::c_int as usize]);
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn remote_client_daemon_parse_command(
    mut line: *mut libc::c_char,
    mut cl: *mut client,
    mut ps: *mut parallelstatus,
    mut fhsend: libc::c_int,
) -> libc::c_int {
    let mut i: libc::c_int = 0;
    let mut ret: libc::c_int = 0;
    let mut cmd: *mut *mut libc::c_char = 0 as *mut *mut libc::c_char;
    cmd = tokenize_spaces_dyn(line);
    if cmd.is_null() {
        return 0 as libc::c_int
    } else if (*cmd.offset(0 as libc::c_int as isize)).is_null() {
        free(cmd as *mut libc::c_void);
        return 0 as libc::c_int;
    } else if strcmp(
        *cmd.offset(0 as libc::c_int as isize),
        b"status\0" as *const u8 as *const libc::c_char,
    ) == 0 as libc::c_int
    {
        let mut dq: *mut dqueue = 0 as *mut dqueue;
        dq = xmalloc(::std::mem::size_of::<dqueue>() as libc::c_ulong) as *mut dqueue;
        (*dq).id = (*ps).iqueue;
        (*dq).qclient = cl;
        (*dq).next = (*ps).dqueuelist;
        (*dq).prev = 0 as *mut dqueue;
        if !((*ps).dqueuelist).is_null() {
            (*(*ps).dqueuelist).prev = dq;
        }
        (*ps).dqueuelist = dq;
        hprintf(
            fhsend,
            b"remote %d status\n\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            (*dq).id,
        );
        (*ps).iqueue += 1;
        (*ps).iqueue;
        ret = 0 as libc::c_int;
    } else if strcmp(
        *cmd.offset(0 as libc::c_int as isize),
        b"lock\0" as *const u8 as *const libc::c_char,
    ) == 0 as libc::c_int
    {
        let mut name: *mut libc::c_char = 0 as *mut libc::c_char;
        let mut maxnum: libc::c_int = 0;
        let mut dq_0: *mut dqueue = 0 as *mut dqueue;
        name = 0 as *mut libc::c_char;
        maxnum = 0 as libc::c_int;
        i = imutex_lock_get_params(cmd, &mut name, &mut maxnum);
        if i != 0 || name.is_null() {
            hprintf(
                (*cl).peer,
                b"error unexpected arguments for '%s'\n\0" as *const u8
                    as *const libc::c_char as *mut libc::c_char,
                *cmd.offset(0 as libc::c_int as isize),
            );
        } else {
            dq_0 = xmalloc(::std::mem::size_of::<dqueue>() as libc::c_ulong)
                as *mut dqueue;
            (*dq_0).id = (*ps).iqueue;
            (*dq_0).qclient = cl;
            (*dq_0).next = (*ps).dqueuelist;
            (*dq_0).prev = 0 as *mut dqueue;
            if !((*ps).dqueuelist).is_null() {
                (*(*ps).dqueuelist).prev = dq_0;
            }
            (*ps).dqueuelist = dq_0;
            hprintf(
                fhsend,
                b"remote %d lock \"%s\"\n\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                (*dq_0).id,
                name,
            );
            (*ps).iqueue += 1;
            (*ps).iqueue;
        }
        ret = 0 as libc::c_int;
    } else if strcmp(
        *cmd.offset(0 as libc::c_int as isize),
        b"unlock\0" as *const u8 as *const libc::c_char,
    ) == 0 as libc::c_int
    {
        let mut name_0: *mut libc::c_char = 0 as *mut libc::c_char;
        name_0 = 0 as *mut libc::c_char;
        i = imutex_unlock_get_params(cmd, &mut name_0);
        if !(i != 0 || name_0.is_null()) {
            hprintf(
                fhsend,
                b"remote -1 unlock \"%s\"\n\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                name_0,
            );
        }
        ret = 0 as libc::c_int;
    } else if strcmp(
        *cmd.offset(0 as libc::c_int as isize),
        b"mutex\0" as *const u8 as *const libc::c_char,
    ) == 0 as libc::c_int
    {
        let mut name_1: *mut libc::c_char = 0 as *mut libc::c_char;
        let mut dq_1: *mut dqueue = 0 as *mut dqueue;
        name_1 = 0 as *mut libc::c_char;
        i = imutex_mutex_get_params(cmd, &mut name_1);
        if i != 0 || name_1.is_null() {
            hprintf(
                (*cl).peer,
                b"error unexpected arguments for '%s'\n\0" as *const u8
                    as *const libc::c_char as *mut libc::c_char,
                *cmd.offset(0 as libc::c_int as isize),
            );
        } else {
            dq_1 = xmalloc(::std::mem::size_of::<dqueue>() as libc::c_ulong)
                as *mut dqueue;
            (*dq_1).id = (*ps).iqueue;
            (*dq_1).qclient = cl;
            (*dq_1).next = (*ps).dqueuelist;
            (*dq_1).prev = 0 as *mut dqueue;
            if !((*ps).dqueuelist).is_null() {
                (*(*ps).dqueuelist).prev = dq_1;
            }
            (*ps).dqueuelist = dq_1;
            hprintf(
                fhsend,
                b"remote %d mutex \"%s\"\n\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                (*dq_1).id,
                name_1,
            );
            (*ps).iqueue += 1;
            (*ps).iqueue;
        }
        ret = 0 as libc::c_int;
    } else if strcmp(
        *cmd.offset(0 as libc::c_int as isize),
        b"exit\0" as *const u8 as *const libc::c_char,
    ) == 0 as libc::c_int
    {
        hprintf(
            (*cl).peer,
            b"bye\n\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        );
        ret = 1 as libc::c_int;
    } else {
        hprintf(
            (*cl).peer,
            b"error unexpected command '%s'\n\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            *cmd.offset(0 as libc::c_int as isize),
        );
        ret = 0 as libc::c_int;
    }
    free(cmd as *mut libc::c_void);
    return ret;
}
pub unsafe extern "C" fn dqueue_get_queue_by_id(
    mut ps: *mut parallelstatus,
    mut id: libc::c_int,
) -> *mut dqueue {
    let mut dq: *mut dqueue = 0 as *mut dqueue;
    dq = (*ps).dqueuelist;
    while !dq.is_null() {
        if (*dq).id == id {
            return dq;
        }
        dq = (*dq).next;
    }
    return 0 as *mut dqueue;
}
pub unsafe extern "C" fn daemon_process_command(
    mut ps: *mut parallelstatus,
    mut fhsend: libc::c_int,
    mut cmd: *mut *mut libc::c_char,
) -> libc::c_int {
    let mut i: libc::c_int = 0;
    let mut n: libc::c_int = 0;
    let mut id: libc::c_int = 0;
    let mut zeroarg: libc::c_int = 0;
    let mut shell: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut in_0: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut out: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut err: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut envname: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut envvalue: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut eoc: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut p: paralleldata = paralleldata {
        in_0: 0 as *mut libc::c_char,
        out: 0 as *mut libc::c_char,
        err: 0 as *mut libc::c_char,
        fwout: 0 as *mut FILE,
        formatout: 0 as *mut libc::c_char,
        fwerr: 0 as *mut FILE,
        formaterr: 0 as *mut libc::c_char,
        omit_newlines: 0,
        envvarname: 0 as *mut libc::c_char,
        shell: 0 as *mut libc::c_char,
        rsh: 0 as *mut libc::c_char,
        rshcmd: 0 as *mut libc::c_char,
        rshargs: 0 as *mut *mut libc::c_char,
        fallback_to_die: 0,
        log: 0 as *mut logdata,
    };
    let mut par: parameter = parameter {
        name: 0 as *mut libc::c_char,
        no_touch_std: 0,
        c: command {
            is_shell: 0,
            argv: 0 as *mut *mut libc::c_char,
            argc: 0,
        },
        id: 0,
        status: 0,
    };
    let mut cc: *mut child = 0 as *mut child;
    let mut dq: *mut dqueue = 0 as *mut dqueue;
    if cmd.is_null() || (*cmd.offset(0 as libc::c_int as isize)).is_null() {
        return 0 as libc::c_int;
    }
    n = 0 as libc::c_int;
    while !(*cmd.offset(n as isize)).is_null() {
        n += 1;
        n;
    }
    if strcmp(
        *cmd.offset(0 as libc::c_int as isize),
        b"exit\0" as *const u8 as *const libc::c_char,
    ) == 0 as libc::c_int
    {
        return 1 as libc::c_int
    } else if strcmp(
        *cmd.offset(0 as libc::c_int as isize),
        b"remote\0" as *const u8 as *const libc::c_char,
    ) == 0 as libc::c_int && n >= 3 as libc::c_int
    {
        let mut qid: libc::c_int = 0;
        let mut j: libc::c_int = 0;
        let mut buff: *mut libc::c_char = 0 as *mut libc::c_char;
        if sscanf(
            *cmd.offset(1 as libc::c_int as isize),
            b"%d\0" as *const u8 as *const libc::c_char,
            &mut qid as *mut libc::c_int,
        ) < 1 as libc::c_int
        {
            qid = -(1 as libc::c_int);
        }
        dq = dqueue_get_queue_by_id(ps, qid);
        if !dq.is_null() {
            buff = 0 as *mut libc::c_char;
            strappendf(
                &mut buff as *mut *mut libc::c_char,
                b"%s\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                *cmd.offset(2 as libc::c_int as isize),
            );
            if strcmp(
                *cmd.offset(2 as libc::c_int as isize),
                b"locked\0" as *const u8 as *const libc::c_char,
            ) == 0 as libc::c_int
                || strcmp(
                    *cmd.offset(2 as libc::c_int as isize),
                    b"mutex\0" as *const u8 as *const libc::c_char,
                ) == 0 as libc::c_int
            {
                strappendf(
                    &mut buff as *mut *mut libc::c_char,
                    b" \"%s\"\0" as *const u8 as *const libc::c_char
                        as *mut libc::c_char,
                    if !(*cmd.offset(3 as libc::c_int as isize)).is_null() {
                        *cmd.offset(3 as libc::c_int as isize) as *const libc::c_char
                    } else {
                        b"-\0" as *const u8 as *const libc::c_char
                    },
                );
                if !(*cmd.offset(3 as libc::c_int as isize)).is_null() {
                    j = 4 as libc::c_int;
                } else {
                    j = 3 as libc::c_int;
                }
            } else {
                j = 3 as libc::c_int;
            }
            while !(*cmd.offset(j as isize)).is_null() {
                strappendf(
                    &mut buff as *mut *mut libc::c_char,
                    b" %s\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                    *cmd.offset(j as isize),
                );
                j += 1;
                j;
            }
            hprintf(
                (*(*dq).qclient).peer,
                b"%s\n\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                buff,
            );
            if !buff.is_null() {
                free(buff as *mut libc::c_void);
            }
            if !((*dq).next).is_null() {
                (*(*dq).next).prev = (*dq).prev;
            }
            if !((*dq).prev).is_null() {
                (*(*dq).prev).next = (*dq).next;
            } else {
                (*ps).dqueuelist = (*dq).next;
            }
            free(dq as *mut libc::c_void);
        }
        return 0 as libc::c_int;
    } else if strcmp(
        *cmd.offset(0 as libc::c_int as isize),
        b"execute\0" as *const u8 as *const libc::c_char,
    ) == 0 as libc::c_int
    {
        shell = 0 as *mut libc::c_char;
        in_0 = 0 as *mut libc::c_char;
        out = 0 as *mut libc::c_char;
        err = 0 as *mut libc::c_char;
        envname = 0 as *mut libc::c_char;
        envvalue = 0 as *mut libc::c_char;
        id = -(1 as libc::c_int);
        zeroarg = -(1 as libc::c_int);
        i = 1 as libc::c_int;
        while i < n {
            if *(*cmd.offset(i as isize)).offset(0 as libc::c_int as isize)
                as libc::c_int == '-' as i32
            {
                zeroarg = i + 1 as libc::c_int;
                break;
            } else {
                eoc = strchr(*cmd.offset(i as isize), '=' as i32);
                if eoc.is_null() {
                    hprintf(
                        fhsend,
                        b"message \"invalid argument '%s' for '%s'\"\n\0" as *const u8
                            as *const libc::c_char as *mut libc::c_char,
                        *cmd.offset(i as isize),
                        *cmd.offset(0 as libc::c_int as isize),
                    );
                    return 0 as libc::c_int;
                }
                *eoc = 0 as libc::c_int as libc::c_char;
                eoc = eoc.offset(1);
                eoc;
                if strcmp(
                    *cmd.offset(i as isize),
                    b"id\0" as *const u8 as *const libc::c_char,
                ) == 0 as libc::c_int
                    || strcmp(
                        *cmd.offset(i as isize),
                        b"identifier\0" as *const u8 as *const libc::c_char,
                    ) == 0 as libc::c_int
                {
                    sscanf(
                        eoc,
                        b"%d\0" as *const u8 as *const libc::c_char,
                        &mut id as *mut libc::c_int,
                    );
                } else if strcmp(
                    *cmd.offset(i as isize),
                    b"shell\0" as *const u8 as *const libc::c_char,
                ) == 0 as libc::c_int
                {
                    shell = eoc;
                } else if strcmp(
                    *cmd.offset(i as isize),
                    b"in\0" as *const u8 as *const libc::c_char,
                ) == 0 as libc::c_int
                {
                    in_0 = eoc;
                } else if strcmp(
                    *cmd.offset(i as isize),
                    b"out\0" as *const u8 as *const libc::c_char,
                ) == 0 as libc::c_int
                {
                    out = eoc;
                } else if strcmp(
                    *cmd.offset(i as isize),
                    b"err\0" as *const u8 as *const libc::c_char,
                ) == 0 as libc::c_int
                {
                    err = eoc;
                } else if strcmp(
                    *cmd.offset(i as isize),
                    b"envname\0" as *const u8 as *const libc::c_char,
                ) == 0 as libc::c_int
                {
                    envname = eoc;
                } else if strcmp(
                    *cmd.offset(i as isize),
                    b"envvalue\0" as *const u8 as *const libc::c_char,
                ) == 0 as libc::c_int
                {
                    envvalue = eoc;
                } else {
                    hprintf(
                        fhsend,
                        b"message \"invalid argument '%s' for '%s'\"\n\0" as *const u8
                            as *const libc::c_char as *mut libc::c_char,
                        *cmd.offset(i as isize),
                        *cmd.offset(0 as libc::c_int as isize),
                    );
                    return 0 as libc::c_int;
                }
                i += 1;
                i;
            }
        }
        if zeroarg < 0 as libc::c_int || id < 0 as libc::c_int {
            hprintf(
                fhsend,
                b"message \"invalid syntax near '%s'\"\n\0" as *const u8
                    as *const libc::c_char as *mut libc::c_char,
                *cmd.offset(0 as libc::c_int as isize),
            );
            return 0 as libc::c_int;
        } else if zeroarg >= n {
            hprintf(
                fhsend,
                b"message \"command specification is missing near '%s'\"\n\0"
                    as *const u8 as *const libc::c_char as *mut libc::c_char,
                *cmd.offset(0 as libc::c_int as isize),
            );
            return 0 as libc::c_int;
        }
        if in_0.is_null() {
            p.in_0 = 0 as *mut libc::c_char;
        } else {
            p.in_0 = in_0;
        }
        if out.is_null() {
            p.out = 0 as *mut libc::c_char;
            p.fwout = 0 as *mut FILE;
        } else if strcmp(out, b"-\0" as *const u8 as *const libc::c_char)
            == 0 as libc::c_int
        {
            p.out = out;
            p.fwout = stdout;
        } else {
            p.out = out;
            p.fwout = 0 as *mut FILE;
        }
        if err.is_null() {
            p.err = 0 as *mut libc::c_char;
            p.fwerr = 0 as *mut FILE;
        } else if strcmp(err, b"-\0" as *const u8 as *const libc::c_char)
            == 0 as libc::c_int
        {
            p.err = err;
            p.fwerr = stderr;
        } else {
            p.err = err;
            p.fwerr = 0 as *mut FILE;
        }
        if !envname.is_null() && !envvalue.is_null() {
            p.envvarname = envname;
            par.name = envvalue;
        } else {
            p.envvarname = 0 as *mut libc::c_char;
            par.name = 0 as *mut libc::c_char;
        }
        if !shell.is_null() {
            p.shell = shell;
            par.c.is_shell = 1 as libc::c_int;
            par.c.argc = n - zeroarg;
            par.c.argv = cmd.offset(zeroarg as isize);
        } else {
            p.shell = 0 as *mut libc::c_char;
            par.c.is_shell = 0 as libc::c_int;
            par.c.argc = n - zeroarg;
            par.c.argv = cmd.offset(zeroarg as isize);
        }
        par.no_touch_std = 0 as libc::c_int;
        cc = xmalloc(::std::mem::size_of::<child>() as libc::c_ulong) as *mut child;
        (*cc).fdstdout = -(1 as libc::c_int);
        (*cc).fdstderr = -(1 as libc::c_int);
        (*cc).id = id;
        (*cc)
            .pid = submit_task(
            &mut p,
            &mut par,
            cc,
            (0 as libc::c_int == 0) as libc::c_int,
            ps,
        );
        if (*cc).pid < 0 as libc::c_int {
            hprintf(
                fhsend,
                b"message \"unable to execute, reason code: %d\"\n\0" as *const u8
                    as *const libc::c_char as *mut libc::c_char,
                (*cc).pid,
            );
            free(cc as *mut libc::c_void);
            return 0 as libc::c_int;
        } else {
            (*cc).next = (*ps).childlist;
            (*cc).prev = 0 as *mut child;
            if !((*ps).childlist).is_null() {
                (*(*ps).childlist).prev = cc;
            }
            (*ps).childlist = cc;
            return 0 as libc::c_int;
        }
    } else if strcmp(
        *cmd.offset(0 as libc::c_int as isize),
        b"request\0" as *const u8 as *const libc::c_char,
    ) == 0 as libc::c_int
    {
        if (*ps).hsck < 0 as libc::c_int {
            hprintf(
                fhsend,
                b"message \"unexpected 'request': hypervisor has not been connected to daemon\"\n\0"
                    as *const u8 as *const libc::c_char as *mut libc::c_char,
            );
        } else {
            hprintf(
                (*ps).hsck,
                b"request\n\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            );
        }
        return 0 as libc::c_int;
    } else if strcmp(
        *cmd.offset(0 as libc::c_int as isize),
        b"ready\0" as *const u8 as *const libc::c_char,
    ) == 0 as libc::c_int
    {
        if (*ps).hsck < 0 as libc::c_int {
            hprintf(
                fhsend,
                b"message \"unexpected 'ready': hypervisor has not been connected to daemon\"\n\0"
                    as *const u8 as *const libc::c_char as *mut libc::c_char,
            );
        } else {
            hprintf(
                (*ps).hsck,
                b"ready\n\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            );
        }
        return 0 as libc::c_int;
    } else if strcmp(
        *cmd.offset(0 as libc::c_int as isize),
        b"completed\0" as *const u8 as *const libc::c_char,
    ) == 0 as libc::c_int
    {
        if (*ps).hsck < 0 as libc::c_int {
            hprintf(
                fhsend,
                b"message \"unexpected 'completed': hypervisor has not been connected to daemon\"\n\0"
                    as *const u8 as *const libc::c_char as *mut libc::c_char,
            );
        } else {
            hprintf(
                (*ps).hsck,
                b"completed\n\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            );
        }
        return 0 as libc::c_int;
    } else {
        hprintf(
            fhsend,
            b"message \"invalid command: %s\"\n\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            *cmd.offset(0 as libc::c_int as isize),
        );
        return 0 as libc::c_int;
    };
}
pub unsafe extern "C" fn daemon_send_data(
    mut fhsend: libc::c_int,
    mut streamname: *mut libc::c_char,
    mut id: libc::c_int,
    mut buff: *mut libc::c_char,
    mut size: libc::c_int,
) -> libc::c_int {
    let mut out: *mut libc::c_char = 0 as *mut libc::c_char;
    if buff.is_null() {
        return 0 as libc::c_int;
    }
    out = daemon_commandtoken_escape(buff, size);
    hprintf(
        fhsend,
        b"%s %d \0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        streamname,
        id,
    );
    write(fhsend, out as *const libc::c_void, strlen(out));
    free(out as *mut libc::c_void);
    hprintf(fhsend, b"\n\0" as *const u8 as *const libc::c_char as *mut libc::c_char);
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn daemon_send_output(
    mut fhsend: libc::c_int,
    mut id: libc::c_int,
    mut buff: *mut libc::c_char,
    mut size: libc::c_int,
) -> libc::c_int {
    return daemon_send_data(
        fhsend,
        b"output\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        id,
        buff,
        size,
    );
}
pub unsafe extern "C" fn daemon_send_error(
    mut fhsend: libc::c_int,
    mut id: libc::c_int,
    mut buff: *mut libc::c_char,
    mut size: libc::c_int,
) -> libc::c_int {
    return daemon_send_data(
        fhsend,
        b"error\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        id,
        buff,
        size,
    );
}
pub unsafe extern "C" fn pexec_daemon_main_loop(
    mut fhrecv: libc::c_int,
    mut fhsend: libc::c_int,
    mut num_processes: libc::c_int,
    mut sock: libc::c_int,
    mut hsck: libc::c_int,
) -> libc::c_int {
    let mut set: fd_set = fd_set { fds_bits: [0; 16] };
    let mut ps: parallelstatus = parallelstatus {
        childlist: 0 as *mut child,
        achild: 0,
        sock: 0,
        hsck: 0,
        clientlist: 0 as *mut client,
        imutexlist: 0 as *mut imutex,
        dqueuelist: 0 as *mut dqueue,
        iqueue: 0,
        nfinished: 0,
        npending: 0,
        nparam: 0,
        t0: 0,
    };
    let mut cc: *mut child = 0 as *mut child;
    let mut chldact: sigaction = sigaction {
        __sigaction_handler: C2RustUnnamed_10 {
            sa_handler: None,
        },
        sa_mask: __sigset_t { __val: [0; 16] },
        sa_flags: 0,
        sa_restorer: None,
    };
    let mut sci: signalinfo = signalinfo {
        signal: 0,
        pid: 0,
        exitstatus: 0,
        exitsignal: 0,
    };
    let mut spipe: libc::c_int = 0;
    let mut max: libc::c_int = 0;
    let mut status: libc::c_int = 0;
    let mut buffsize: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    let mut n: libc::c_int = 0;
    let mut buff: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut cmd: *mut *mut libc::c_char = 0 as *mut *mut libc::c_char;
    let mut line: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut is_in_loop: libc::c_int = 0;
    let mut ret: libc::c_int = 0;
    let mut lcmd: linebuffer = linebuffer {
        buffer: 0 as *mut libc::c_char,
        length: 0,
    };
    let mut lhyp: linebuffer = linebuffer {
        buffer: 0 as *mut libc::c_char,
        length: 0,
    };
    let mut cl: *mut client = 0 as *mut client;
    hprintf(
        fhsend,
        b"initialization num_processes=%d\n\0" as *const u8 as *const libc::c_char
            as *mut libc::c_char,
        num_processes,
    );
    buffsize = getpagesize();
    buff = xmalloc(buffsize as size_t) as *mut libc::c_char;
    if pipe(sig_pipe.as_mut_ptr()) < 0 as libc::c_int {
        return -(1 as libc::c_int);
    }
    spipe = sig_pipe[0 as libc::c_int as usize];
    chldact
        .__sigaction_handler
        .sa_handler = Some(sig_act_child as unsafe extern "C" fn(libc::c_int) -> ());
    sigemptyset(&mut chldact.sa_mask);
    chldact.sa_flags = 1 as libc::c_int | 0x10000000 as libc::c_int;
    sigaction(17 as libc::c_int, &mut chldact, 0 as *mut sigaction);
    ps.childlist = 0 as *mut child;
    ps.achild = 0 as libc::c_int;
    ps.clientlist = 0 as *mut client;
    ps.imutexlist = 0 as *mut imutex;
    ps.dqueuelist = 0 as *mut dqueue;
    ps.iqueue = 0 as libc::c_int;
    ps.sock = sock;
    ps.hsck = hsck;
    linebuffer_reset(&mut lcmd);
    linebuffer_reset(&mut lhyp);
    is_in_loop = 1 as libc::c_int;
    while is_in_loop != 0 {
        let mut __d0: libc::c_int = 0;
        let mut __d1: libc::c_int = 0;
        let fresh31 = &mut __d0;
        let fresh32;
        let fresh33 = (::std::mem::size_of::<fd_set>() as libc::c_ulong)
            .wrapping_div(::std::mem::size_of::<__fd_mask>() as libc::c_ulong);
        let fresh34 = &mut __d1;
        let fresh35;
        let fresh36 = &mut *(set.fds_bits).as_mut_ptr().offset(0 as libc::c_int as isize)
            as *mut __fd_mask;
        asm!(
            "cld; rep; stosq", inlateout("cx")
            c2rust_asm_casts::AsmCast::cast_in(fresh31, fresh33) => fresh32,
            inlateout("di") c2rust_asm_casts::AsmCast::cast_in(fresh34, fresh36) =>
            fresh35, inlateout("ax") 0 as libc::c_int => _, options(preserves_flags,
            att_syntax)
        );
        c2rust_asm_casts::AsmCast::cast_out(fresh31, fresh33, fresh32);
        c2rust_asm_casts::AsmCast::cast_out(fresh34, fresh36, fresh35);
        set
            .fds_bits[(spipe
            / (8 as libc::c_int
                * ::std::mem::size_of::<__fd_mask>() as libc::c_ulong as libc::c_int))
            as usize]
            |= ((1 as libc::c_ulong)
                << spipe
                    % (8 as libc::c_int
                        * ::std::mem::size_of::<__fd_mask>() as libc::c_ulong
                            as libc::c_int)) as __fd_mask;
        max = spipe;
        set
            .fds_bits[(fhrecv
            / (8 as libc::c_int
                * ::std::mem::size_of::<__fd_mask>() as libc::c_ulong as libc::c_int))
            as usize]
            |= ((1 as libc::c_ulong)
                << fhrecv
                    % (8 as libc::c_int
                        * ::std::mem::size_of::<__fd_mask>() as libc::c_ulong
                            as libc::c_int)) as __fd_mask;
        if max < fhrecv {
            max = fhrecv;
        }
        cc = ps.childlist;
        while !cc.is_null() {
            if !((*cc).id < 0 as libc::c_int || (*cc).pid < 0 as libc::c_int) {
                if (*cc).fdstdout >= 0 as libc::c_int {
                    set
                        .fds_bits[((*cc).fdstdout
                        / (8 as libc::c_int
                            * ::std::mem::size_of::<__fd_mask>() as libc::c_ulong
                                as libc::c_int)) as usize]
                        |= ((1 as libc::c_ulong)
                            << (*cc).fdstdout
                                % (8 as libc::c_int
                                    * ::std::mem::size_of::<__fd_mask>() as libc::c_ulong
                                        as libc::c_int)) as __fd_mask;
                    if max < (*cc).fdstdout {
                        max = (*cc).fdstdout;
                    }
                }
                if (*cc).fdstderr >= 0 as libc::c_int {
                    set
                        .fds_bits[((*cc).fdstderr
                        / (8 as libc::c_int
                            * ::std::mem::size_of::<__fd_mask>() as libc::c_ulong
                                as libc::c_int)) as usize]
                        |= ((1 as libc::c_ulong)
                            << (*cc).fdstderr
                                % (8 as libc::c_int
                                    * ::std::mem::size_of::<__fd_mask>() as libc::c_ulong
                                        as libc::c_int)) as __fd_mask;
                    if max < (*cc).fdstderr {
                        max = (*cc).fdstderr;
                    }
                }
            }
            cc = (*cc).next;
        }
        if sock >= 0 as libc::c_int {
            set
                .fds_bits[(sock
                / (8 as libc::c_int
                    * ::std::mem::size_of::<__fd_mask>() as libc::c_ulong
                        as libc::c_int)) as usize]
                |= ((1 as libc::c_ulong)
                    << sock
                        % (8 as libc::c_int
                            * ::std::mem::size_of::<__fd_mask>() as libc::c_ulong
                                as libc::c_int)) as __fd_mask;
            if max < sock {
                max = sock;
            }
        }
        if hsck >= 0 as libc::c_int {
            set
                .fds_bits[(hsck
                / (8 as libc::c_int
                    * ::std::mem::size_of::<__fd_mask>() as libc::c_ulong
                        as libc::c_int)) as usize]
                |= ((1 as libc::c_ulong)
                    << hsck
                        % (8 as libc::c_int
                            * ::std::mem::size_of::<__fd_mask>() as libc::c_ulong
                                as libc::c_int)) as __fd_mask;
            if max < hsck {
                max = hsck;
            }
        }
        cl = ps.clientlist;
        while !cl.is_null() {
            set
                .fds_bits[((*cl).peer
                / (8 as libc::c_int
                    * ::std::mem::size_of::<__fd_mask>() as libc::c_ulong
                        as libc::c_int)) as usize]
                |= ((1 as libc::c_ulong)
                    << (*cl).peer
                        % (8 as libc::c_int
                            * ::std::mem::size_of::<__fd_mask>() as libc::c_ulong
                                as libc::c_int)) as __fd_mask;
            if max < (*cl).peer {
                max = (*cl).peer;
            }
            cl = (*cl).next;
        }
        i = select(
            max + 1 as libc::c_int,
            &mut set,
            0 as *mut fd_set,
            0 as *mut fd_set,
            0 as *mut timeval,
        );
        if set
            .fds_bits[(spipe
            / (8 as libc::c_int
                * ::std::mem::size_of::<__fd_mask>() as libc::c_ulong as libc::c_int))
            as usize]
            & ((1 as libc::c_ulong)
                << spipe
                    % (8 as libc::c_int
                        * ::std::mem::size_of::<__fd_mask>() as libc::c_ulong
                            as libc::c_int)) as __fd_mask
            != 0 as libc::c_int as libc::c_long
        {
            if !(read_signalinfo(spipe, &mut sci) > 0 as libc::c_int) {
                fprintf(
                    stderr,
                    dcgettext(
                        0 as *const libc::c_char,
                        b"%s: fatal error: read_signalinfo() failed.\n\0" as *const u8
                            as *const libc::c_char,
                        5 as libc::c_int,
                    ),
                    progbasename,
                );
                exit(1 as libc::c_int);
            }
            if sci.exitsignal < 0 as libc::c_int {
                cc = get_child_by_pid(ps.childlist, sci.pid);
                if (*cc).fdstdout >= 0 as libc::c_int {
                    while fd_avail((*cc).fdstdout) != 0 {
                        n = read(
                            (*cc).fdstdout,
                            buff as *mut libc::c_void,
                            buffsize as size_t,
                        ) as libc::c_int;
                        if n > 0 as libc::c_int {
                            daemon_send_output(fhsend, (*cc).id, buff, n);
                        } else if !(n < 0 as libc::c_int
                            && *__errno_location() == 4 as libc::c_int)
                        {
                            break;
                        }
                    }
                    close((*cc).fdstdout);
                    (*cc).fdstdout = -(1 as libc::c_int);
                }
                if (*cc).fdstderr >= 0 as libc::c_int {
                    while fd_avail((*cc).fdstderr) != 0 {
                        n = read(
                            (*cc).fdstderr,
                            buff as *mut libc::c_void,
                            buffsize as size_t,
                        ) as libc::c_int;
                        if n > 0 as libc::c_int {
                            daemon_send_error(fhsend, (*cc).id, buff, n);
                        } else if !(n < 0 as libc::c_int
                            && *__errno_location() == 4 as libc::c_int)
                        {
                            break;
                        }
                    }
                    close((*cc).fdstderr);
                    (*cc).fdstderr = -(1 as libc::c_int);
                }
            }
            hprintf(
                fhsend,
                b"finish %d signal=%d status=%d\n\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                (*cc).id,
                sci.exitsignal,
                sci.exitstatus,
            );
            if !((*cc).next).is_null() {
                (*(*cc).next).prev = (*cc).prev;
            }
            if !((*cc).prev).is_null() {
                (*(*cc).prev).next = (*cc).next;
            } else {
                ps.childlist = (*cc).next;
            }
            free(cc as *mut libc::c_void);
            ps.achild -= 1;
            ps.achild;
        } else if set
            .fds_bits[(fhrecv
            / (8 as libc::c_int
                * ::std::mem::size_of::<__fd_mask>() as libc::c_ulong as libc::c_int))
            as usize]
            & ((1 as libc::c_ulong)
                << fhrecv
                    % (8 as libc::c_int
                        * ::std::mem::size_of::<__fd_mask>() as libc::c_ulong
                            as libc::c_int)) as __fd_mask
            != 0 as libc::c_int as libc::c_long
        {
            n = read(fhrecv, buff as *mut libc::c_void, buffsize as size_t)
                as libc::c_int;
            if n == 0 as libc::c_int {
                is_in_loop = 0 as libc::c_int;
                break;
            } else {
                if n < 0 as libc::c_int && *__errno_location() == 4 as libc::c_int {
                    continue;
                }
                linebuffer_concatenate(&mut lcmd, buff, n as size_t);
                loop {
                    line = linebuffer_fetch(&mut lcmd);
                    if line.is_null() {
                        break;
                    }
                    cmd = tokenize_spaces_dyn(line);
                    if !cmd.is_null() {
                        i = 0 as libc::c_int;
                        while !(*cmd.offset(i as isize)).is_null() {
                            daemon_commandtoken_unescape(*cmd.offset(i as isize));
                            i += 1;
                            i;
                        }
                        ret = daemon_process_command(&mut ps, fhsend, cmd);
                        free(cmd as *mut libc::c_void);
                        free(line as *mut libc::c_void);
                        if !(ret > 0 as libc::c_int) {
                            continue;
                        }
                        is_in_loop = 0 as libc::c_int;
                        break;
                    } else {
                        free(line as *mut libc::c_void);
                    }
                }
            }
        } else {
            cc = ps.childlist;
            while !cc.is_null() {
                if !((*cc).id < 0 as libc::c_int || (*cc).pid < 0 as libc::c_int) {
                    if (*cc).fdstdout >= 0 as libc::c_int
                        && set
                            .fds_bits[((*cc).fdstdout
                            / (8 as libc::c_int
                                * ::std::mem::size_of::<__fd_mask>() as libc::c_ulong
                                    as libc::c_int)) as usize]
                            & ((1 as libc::c_ulong)
                                << (*cc).fdstdout
                                    % (8 as libc::c_int
                                        * ::std::mem::size_of::<__fd_mask>() as libc::c_ulong
                                            as libc::c_int)) as __fd_mask
                            != 0 as libc::c_int as libc::c_long
                    {
                        n = read(
                            (*cc).fdstdout,
                            buff as *mut libc::c_void,
                            buffsize as size_t,
                        ) as libc::c_int;
                        if n > 0 as libc::c_int {
                            daemon_send_output(fhsend, (*cc).id, buff, n);
                        } else if !(n < 0 as libc::c_int
                            && *__errno_location() == 4 as libc::c_int)
                        {
                            close((*cc).fdstdout);
                            (*cc).fdstdout = -(1 as libc::c_int);
                        }
                    }
                    if (*cc).fdstderr >= 0 as libc::c_int
                        && set
                            .fds_bits[((*cc).fdstderr
                            / (8 as libc::c_int
                                * ::std::mem::size_of::<__fd_mask>() as libc::c_ulong
                                    as libc::c_int)) as usize]
                            & ((1 as libc::c_ulong)
                                << (*cc).fdstderr
                                    % (8 as libc::c_int
                                        * ::std::mem::size_of::<__fd_mask>() as libc::c_ulong
                                            as libc::c_int)) as __fd_mask
                            != 0 as libc::c_int as libc::c_long
                    {
                        n = read(
                            (*cc).fdstderr,
                            buff as *mut libc::c_void,
                            buffsize as size_t,
                        ) as libc::c_int;
                        if n > 0 as libc::c_int {
                            daemon_send_error(fhsend, (*cc).id, buff, n);
                        } else if !(n < 0 as libc::c_int
                            && *__errno_location() == 4 as libc::c_int)
                        {
                            close((*cc).fdstderr);
                            (*cc).fdstderr = -(1 as libc::c_int);
                        }
                    }
                }
                cc = (*cc).next;
            }
            cl = ps.clientlist;
            while !cl.is_null() {
                let mut line_0: *mut libc::c_char = 0 as *mut libc::c_char;
                let mut closepeer: libc::c_int = 0;
                if set
                    .fds_bits[((*cl).peer
                    / (8 as libc::c_int
                        * ::std::mem::size_of::<__fd_mask>() as libc::c_ulong
                            as libc::c_int)) as usize]
                    & ((1 as libc::c_ulong)
                        << (*cl).peer
                            % (8 as libc::c_int
                                * ::std::mem::size_of::<__fd_mask>() as libc::c_ulong
                                    as libc::c_int)) as __fd_mask
                    != 0 as libc::c_int as libc::c_long
                {
                    n = read((*cl).peer, buff as *mut libc::c_void, buffsize as size_t)
                        as libc::c_int;
                    if n == 0 as libc::c_int {
                        close((*cl).peer);
                        linebuffer_free(&mut (*cl).lcli);
                        if !((*cl).next).is_null() {
                            (*(*cl).next).prev = (*cl).prev;
                        }
                        if !((*cl).prev).is_null() {
                            (*(*cl).prev).next = (*cl).next;
                        } else {
                            ps.clientlist = (*cl).next;
                        }
                        free(cl as *mut libc::c_void);
                        break;
                    } else if !(n < 0 as libc::c_int
                        && *__errno_location() == 4 as libc::c_int)
                    {
                        linebuffer_concatenate(&mut (*cl).lcli, buff, n as size_t);
                        closepeer = 0 as libc::c_int;
                        loop {
                            line_0 = linebuffer_fetch(&mut (*cl).lcli);
                            if line_0.is_null() {
                                break;
                            }
                            closepeer = remote_client_daemon_parse_command(
                                line_0,
                                cl,
                                &mut ps,
                                fhsend,
                            );
                            free(line_0 as *mut libc::c_void);
                            if closepeer != 0 {
                                break;
                            }
                        }
                        if closepeer != 0 {
                            close((*cl).peer);
                            linebuffer_free(&mut (*cl).lcli);
                            if !((*cl).next).is_null() {
                                (*(*cl).next).prev = (*cl).prev;
                            }
                            if !((*cl).prev).is_null() {
                                (*(*cl).prev).next = (*cl).next;
                            } else {
                                ps.clientlist = (*cl).next;
                            }
                            free(cl as *mut libc::c_void);
                            break;
                        }
                    }
                }
                cl = (*cl).next;
            }
            if hsck >= 0 as libc::c_int
                && set
                    .fds_bits[(hsck
                    / (8 as libc::c_int
                        * ::std::mem::size_of::<__fd_mask>() as libc::c_ulong
                            as libc::c_int)) as usize]
                    & ((1 as libc::c_ulong)
                        << hsck
                            % (8 as libc::c_int
                                * ::std::mem::size_of::<__fd_mask>() as libc::c_ulong
                                    as libc::c_int)) as __fd_mask
                    != 0 as libc::c_int as libc::c_long
            {
                n = read(hsck, buff as *mut libc::c_void, buffsize as size_t)
                    as libc::c_int;
                if n > 0 as libc::c_int {
                    linebuffer_concatenate(&mut lhyp, buff, n as size_t);
                }
                loop {
                    line = linebuffer_fetch(&mut lhyp);
                    if line.is_null() {
                        break;
                    }
                    cmd = tokenize_spaces_dyn(line);
                    if !cmd.is_null() {
                        if strcmp(
                            *cmd.offset(0 as libc::c_int as isize),
                            b"acknowledged\0" as *const u8 as *const libc::c_char,
                        ) == 0 as libc::c_int
                        {
                            hprintf(
                                fhsend,
                                b"acknowledged\n\0" as *const u8 as *const libc::c_char
                                    as *mut libc::c_char,
                            );
                        }
                        free(cmd as *mut libc::c_void);
                    }
                    free(line as *mut libc::c_void);
                }
            }
            if sock >= 0 as libc::c_int
                && set
                    .fds_bits[(sock
                    / (8 as libc::c_int
                        * ::std::mem::size_of::<__fd_mask>() as libc::c_ulong
                            as libc::c_int)) as usize]
                    & ((1 as libc::c_ulong)
                        << sock
                            % (8 as libc::c_int
                                * ::std::mem::size_of::<__fd_mask>() as libc::c_ulong
                                    as libc::c_int)) as __fd_mask
                    != 0 as libc::c_int as libc::c_long
            {
                cl = xmalloc(::std::mem::size_of::<client>() as libc::c_ulong)
                    as *mut client;
                (*cl)
                    .peer = accept(
                    sock,
                    __SOCKADDR_ARG {
                        __sockaddr__: 0 as *mut libc::c_void as *mut sockaddr,
                    },
                    0 as *mut socklen_t,
                );
                linebuffer_reset(&mut (*cl).lcli);
                (*cl).next = ps.clientlist;
                (*cl).prev = 0 as *mut client;
                if !(ps.clientlist).is_null() {
                    (*ps.clientlist).prev = cl;
                }
                ps.clientlist = cl;
                (*cl).status = 0 as libc::c_int;
            }
        }
    }
    linebuffer_free(&mut lcmd);
    free(buff as *mut libc::c_void);
    signal(17 as libc::c_int, None);
    close(sig_pipe[0 as libc::c_int as usize]);
    sig_pipe[0 as libc::c_int as usize] = -(1 as libc::c_int);
    close(sig_pipe[1 as libc::c_int as usize]);
    sig_pipe[1 as libc::c_int as usize] = -(1 as libc::c_int);
    cc = ps.childlist;
    while !cc.is_null() {
        if !((*cc).pid < 0 as libc::c_int) {
            kill((*cc).pid, 9 as libc::c_int);
            waitpid((*cc).pid, &mut status, 0 as libc::c_int);
            free(cc as *mut libc::c_void);
        }
        cc = (*cc).next;
    }
    ps.childlist = 0 as *mut child;
    ps.achild = 0 as libc::c_int;
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn parse_host_data(
    mut arg: *mut libc::c_char,
    mut rrhosts: *mut *mut remotehost,
    mut rnrhost: *mut libc::c_int,
) -> libc::c_int {
    let mut hostlist: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut hosts: *mut *mut libc::c_char = 0 as *mut *mut libc::c_char;
    let mut eoc: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut j: libc::c_int = 0;
    let mut n: libc::c_int = 0;
    let mut r: libc::c_int = 0;
    let mut rhosts: *mut remotehost = 0 as *mut remotehost;
    let mut nrhost: libc::c_int = 0;
    if arg.is_null() || *arg as libc::c_int == 0 as libc::c_int {
        return -(1 as libc::c_int);
    }
    hostlist = xstrdup(arg);
    hosts = tokenize_char_dyn(hostlist, ',' as i32);
    rhosts = 0 as *mut remotehost;
    nrhost = 0 as libc::c_int;
    j = 0 as libc::c_int;
    while !hosts.is_null() && !(*hosts.offset(j as isize)).is_null() {
        eoc = strchr(*hosts.offset(j as isize), ':' as i32);
        rhosts = xrealloc(
            rhosts as *mut libc::c_void,
            (::std::mem::size_of::<remotehost>() as libc::c_ulong)
                .wrapping_mul((nrhost + 1 as libc::c_int) as libc::c_ulong),
        ) as *mut remotehost;
        if eoc.is_null() {
            if strcmp(
                *hosts.offset(j as isize),
                b"auto\0" as *const u8 as *const libc::c_char,
            ) == 0 as libc::c_int
            {
                let ref mut fresh37 = (*rhosts.offset(nrhost as isize)).hostspec;
                *fresh37 = 0 as *mut libc::c_char;
                (*rhosts.offset(nrhost as isize)).num_processes = -(1 as libc::c_int);
            } else if strcmp(
                *hosts.offset(j as isize),
                b"managed\0" as *const u8 as *const libc::c_char,
            ) == 0 as libc::c_int
            {
                let ref mut fresh38 = (*rhosts.offset(nrhost as isize)).hostspec;
                *fresh38 = 0 as *mut libc::c_char;
                (*rhosts.offset(nrhost as isize)).num_processes = -(2 as libc::c_int);
            } else if strcmp(
                *hosts.offset(j as isize),
                b"ncpu\0" as *const u8 as *const libc::c_char,
            ) == 0 as libc::c_int
            {
                let ref mut fresh39 = (*rhosts.offset(nrhost as isize)).hostspec;
                *fresh39 = 0 as *mut libc::c_char;
                (*rhosts.offset(nrhost as isize)).num_processes = -(3 as libc::c_int);
            } else if sscanf(
                *hosts.offset(j as isize),
                b"%d\0" as *const u8 as *const libc::c_char,
                &mut n as *mut libc::c_int,
            ) == 1 as libc::c_int && n > 0 as libc::c_int
            {
                let ref mut fresh40 = (*rhosts.offset(nrhost as isize)).hostspec;
                *fresh40 = 0 as *mut libc::c_char;
                (*rhosts.offset(nrhost as isize)).num_processes = n;
            } else {
                let ref mut fresh41 = (*rhosts.offset(nrhost as isize)).hostspec;
                *fresh41 = xstrdup(*hosts.offset(j as isize));
                (*rhosts.offset(nrhost as isize)).num_processes = -(1 as libc::c_int);
            }
        } else if eoc == *hosts.offset(j as isize) {
            free(rhosts as *mut libc::c_void);
            return 1 as libc::c_int;
        } else {
            *eoc = 0 as libc::c_int as libc::c_char;
            if strcmp(
                eoc.offset(1 as libc::c_int as isize),
                b"auto\0" as *const u8 as *const libc::c_char,
            ) == 0 as libc::c_int
            {
                n = -(1 as libc::c_int);
            } else if strcmp(
                eoc.offset(1 as libc::c_int as isize),
                b"managed\0" as *const u8 as *const libc::c_char,
            ) == 0 as libc::c_int
            {
                n = -(2 as libc::c_int);
            } else if strcmp(
                eoc.offset(1 as libc::c_int as isize),
                b"ncpu\0" as *const u8 as *const libc::c_char,
            ) == 0 as libc::c_int
            {
                n = -(3 as libc::c_int);
            } else {
                r = sscanf(
                    eoc.offset(1 as libc::c_int as isize),
                    b"%d\0" as *const u8 as *const libc::c_char,
                    &mut n as *mut libc::c_int,
                );
                if r == 0 as libc::c_int
                    || r == 1 as libc::c_int && n <= 0 as libc::c_int
                {
                    free(rhosts as *mut libc::c_void);
                    free(hostlist as *mut libc::c_void);
                    return 1 as libc::c_int;
                }
            }
            let ref mut fresh42 = (*rhosts.offset(nrhost as isize)).hostspec;
            *fresh42 = xstrdup(*hosts.offset(j as isize));
            (*rhosts.offset(nrhost as isize)).num_processes = n;
        }
        nrhost += 1;
        nrhost;
        j += 1;
        j;
    }
    free(hosts as *mut libc::c_void);
    free(hostlist as *mut libc::c_void);
    *rrhosts = rhosts;
    *rnrhost = nrhost;
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn remote_shell_init(
    mut rsh: *mut libc::c_char,
    mut rshargs: *mut *mut libc::c_char,
    mut pexec_self: *mut libc::c_char,
    mut ctrlport: *mut libc::c_char,
    mut timeout: libc::c_int,
    mut rh: *mut remotehost,
    mut rs: *mut remoteshell,
    mut prio: libc::c_int,
) -> libc::c_int {
    let mut pipesend: [libc::c_int; 2] = [0; 2];
    let mut piperecv: [libc::c_int; 2] = [0; 2];
    let mut pid: libc::c_int = 0;
    if ((*rh).hostspec).is_null() {
        (*rs).pid = -(1 as libc::c_int);
        (*rs).fhrecv = -(1 as libc::c_int);
        (*rs).fhsend = (*rs).fhrecv;
        (*rs).num_processes = (*rh).num_processes;
        (*rs).achild = 0 as libc::c_int;
        linebuffer_reset(&mut (*rs).lrsh);
        return 0 as libc::c_int;
    }
    if pipe(pipesend.as_mut_ptr()) < 0 as libc::c_int {
        return -(1 as libc::c_int);
    }
    if pipe(piperecv.as_mut_ptr()) < 0 as libc::c_int {
        return -(1 as libc::c_int);
    }
    pid = fork();
    if pid < 0 as libc::c_int {
        return -(2 as libc::c_int)
    } else if pid > 0 as libc::c_int {
        let mut line: *mut libc::c_char = 0 as *mut libc::c_char;
        let mut tokens: *mut *mut libc::c_char = 0 as *mut *mut libc::c_char;
        (*rs).pid = pid;
        (*rs).fhsend = pipesend[1 as libc::c_int as usize];
        close(pipesend[0 as libc::c_int as usize]);
        (*rs).fhrecv = piperecv[0 as libc::c_int as usize];
        close(piperecv[1 as libc::c_int as usize]);
        (*rs).achild = 0 as libc::c_int;
        linebuffer_reset(&mut (*rs).lrsh);
        line = linebuffer_read_line((*rs).fhrecv, &mut (*rs).lrsh, timeout);
        if line.is_null() {
            return -(3 as libc::c_int);
        }
        tokens = tokenize_spaces_dyn(line);
        if !tokens.is_null() && !(*tokens.offset(0 as libc::c_int as isize)).is_null()
            && strcmp(
                *tokens.offset(0 as libc::c_int as isize),
                b"initialization\0" as *const u8 as *const libc::c_char,
            ) == 0 as libc::c_int
            && !(*tokens.offset(1 as libc::c_int as isize)).is_null()
            && sscanf(
                *tokens.offset(1 as libc::c_int as isize),
                b"num_processes=%d\0" as *const u8 as *const libc::c_char,
                &mut (*rs).num_processes as *mut libc::c_int,
            ) == 1 as libc::c_int && (*rs).num_processes >= 0 as libc::c_int
        {
            free(tokens as *mut libc::c_void);
            free(line as *mut libc::c_void);
            return 0 as libc::c_int;
        } else {
            if !tokens.is_null() {
                free(tokens as *mut libc::c_void);
            }
            free(line as *mut libc::c_void);
            return -(4 as libc::c_int);
        }
    } else {
        let mut i: libc::c_int = 0;
        let mut n: libc::c_int = 0;
        let mut argv: *mut *mut libc::c_char = 0 as *mut *mut libc::c_char;
        let mut num_buff: [libc::c_char; 32] = [0; 32];
        let mut pri_buff: [libc::c_char; 32] = [0; 32];
        close(pipesend[1 as libc::c_int as usize]);
        close(piperecv[0 as libc::c_int as usize]);
        close(0 as libc::c_int);
        dup2(pipesend[0 as libc::c_int as usize], 0 as libc::c_int);
        close(pipesend[0 as libc::c_int as usize]);
        close(1 as libc::c_int);
        dup2(piperecv[1 as libc::c_int as usize], 1 as libc::c_int);
        close(piperecv[1 as libc::c_int as usize]);
        close(2 as libc::c_int);
        dup2(1 as libc::c_int, 2 as libc::c_int);
        n = 0 as libc::c_int;
        while !(*rshargs.offset(n as isize)).is_null() {
            n += 1;
            n;
        }
        argv = xmalloc(
            (::std::mem::size_of::<*mut libc::c_char>() as libc::c_ulong)
                .wrapping_mul((n + 32 as libc::c_int) as libc::c_ulong),
        ) as *mut *mut libc::c_char;
        i = 0 as libc::c_int;
        while i < n {
            let ref mut fresh43 = *argv.offset(i as isize);
            *fresh43 = *rshargs.offset(i as isize);
            i += 1;
            i;
        }
        let fresh44 = i;
        i = i + 1;
        let ref mut fresh45 = *argv.offset(fresh44 as isize);
        *fresh45 = (*rh).hostspec;
        let fresh46 = i;
        i = i + 1;
        let ref mut fresh47 = *argv.offset(fresh46 as isize);
        *fresh47 = pexec_self;
        let fresh48 = i;
        i = i + 1;
        let ref mut fresh49 = *argv.offset(fresh48 as isize);
        *fresh49 = b"--tunnel\0" as *const u8 as *const libc::c_char
            as *mut libc::c_char;
        if (*rh).num_processes > 0 as libc::c_int {
            let fresh50 = i;
            i = i + 1;
            let ref mut fresh51 = *argv.offset(fresh50 as isize);
            *fresh51 = b"--number\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char;
            sprintf(
                num_buff.as_mut_ptr(),
                b"%d\0" as *const u8 as *const libc::c_char,
                (*rh).num_processes,
            );
            let fresh52 = i;
            i = i + 1;
            let ref mut fresh53 = *argv.offset(fresh52 as isize);
            *fresh53 = num_buff.as_mut_ptr();
        } else if (*rh).num_processes == -(1 as libc::c_int) {
            let fresh54 = i;
            i = i + 1;
            let ref mut fresh55 = *argv.offset(fresh54 as isize);
            *fresh55 = b"--number\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char;
            let fresh56 = i;
            i = i + 1;
            let ref mut fresh57 = *argv.offset(fresh56 as isize);
            *fresh57 = b"auto\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char;
        } else if (*rh).num_processes == -(2 as libc::c_int) {
            let fresh58 = i;
            i = i + 1;
            let ref mut fresh59 = *argv.offset(fresh58 as isize);
            *fresh59 = b"--number\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char;
            let fresh60 = i;
            i = i + 1;
            let ref mut fresh61 = *argv.offset(fresh60 as isize);
            *fresh61 = b"managed\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char;
        } else if (*rh).num_processes == -(3 as libc::c_int) {
            let fresh62 = i;
            i = i + 1;
            let ref mut fresh63 = *argv.offset(fresh62 as isize);
            *fresh63 = b"--number\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char;
            let fresh64 = i;
            i = i + 1;
            let ref mut fresh65 = *argv.offset(fresh64 as isize);
            *fresh65 = b"ncpu\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char;
        }
        if prio > 0 as libc::c_int {
            let fresh66 = i;
            i = i + 1;
            let ref mut fresh67 = *argv.offset(fresh66 as isize);
            *fresh67 = b"--nice\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char;
            sprintf(
                pri_buff.as_mut_ptr(),
                b"%d\0" as *const u8 as *const libc::c_char,
                prio - 128 as libc::c_int,
            );
            let fresh68 = i;
            i = i + 1;
            let ref mut fresh69 = *argv.offset(fresh68 as isize);
            *fresh69 = pri_buff.as_mut_ptr();
        }
        if !ctrlport.is_null() {
            let fresh70 = i;
            i = i + 1;
            let ref mut fresh71 = *argv.offset(fresh70 as isize);
            *fresh71 = b"--bind\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char;
            let fresh72 = i;
            i = i + 1;
            let ref mut fresh73 = *argv.offset(fresh72 as isize);
            *fresh73 = ctrlport;
        }
        let fresh74 = i;
        i = i + 1;
        let ref mut fresh75 = *argv.offset(fresh74 as isize);
        *fresh75 = 0 as *mut libc::c_char;
        execvp(rsh, argv as *const *mut libc::c_char);
        fprintf(
            stdout,
            b"initialization execution=failed\n\0" as *const u8 as *const libc::c_char,
        );
        exit(1 as libc::c_int);
    };
}
pub unsafe extern "C" fn is_unix_socket_name(
    mut name: *mut libc::c_char,
) -> libc::c_int {
    if !(strchr(name, '/' as i32)).is_null() {
        return 1 as libc::c_int
    } else {
        return 0 as libc::c_int
    };
}
pub unsafe extern "C" fn is_inet_socket_name(
    mut name: *mut libc::c_char,
) -> libc::c_int {
    let mut port: libc::c_int = 0;
    let mut colon: *mut libc::c_char = 0 as *mut libc::c_char;
    if sscanf(
        name,
        b"%d\0" as *const u8 as *const libc::c_char,
        &mut port as *mut libc::c_int,
    ) == 1 as libc::c_int
    {
        return 1 as libc::c_int
    } else if sscanf(
        name,
        b"*:%d\0" as *const u8 as *const libc::c_char,
        &mut port as *mut libc::c_int,
    ) == 1 as libc::c_int
    {
        return 1 as libc::c_int
    } else {
        colon = strchr(name, ':' as i32);
        if !colon.is_null()
            && sscanf(
                colon.offset(1 as libc::c_int as isize),
                b"%d\0" as *const u8 as *const libc::c_char,
                &mut port as *mut libc::c_int,
            ) == 1 as libc::c_int
        {
            return 2 as libc::c_int
        } else {
            return 0 as libc::c_int
        }
    };
}
pub unsafe extern "C" fn remote_control_port_bind(
    mut ctrlport: *mut libc::c_char,
    mut rctrlport: *mut *mut libc::c_char,
    mut allow_auto: libc::c_int,
    mut fail_on_existing: libc::c_int,
) -> libc::c_int {
    let mut sock: libc::c_int = 0;
    let mut port: libc::c_int = 0;
    let mut ret: libc::c_int = 0;
    let mut pid: libc::c_int = 0;
    let mut buff: [libc::c_char; 256] = [0; 256];
    let mut colon: *mut libc::c_char = 0 as *mut libc::c_char;
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
    if ctrlport.is_null() {
        return -(1 as libc::c_int)
    } else if allow_auto != 0
        && strcmp(ctrlport, b"inet\0" as *const u8 as *const libc::c_char)
            == 0 as libc::c_int
    {
        let mut inaddr: sockaddr_in = sockaddr_in {
            sin_family: 0,
            sin_port: 0,
            sin_addr: in_addr { s_addr: 0 },
            sin_zero: [0; 8],
        };
        sock = socket(2 as libc::c_int, SOCK_STREAM as libc::c_int, 0 as libc::c_int);
        if sock < 0 as libc::c_int {
            return -(1 as libc::c_int);
        }
        port = 11228 as libc::c_int;
        ret = -(1 as libc::c_int);
        while port < 16384 as libc::c_int {
            inaddr.sin_family = 2 as libc::c_int as sa_family_t;
            inaddr.sin_port = __bswap_16(port as __uint16_t);
            inaddr.sin_addr.s_addr = 0 as libc::c_int as in_addr_t;
            ret = bind(
                sock,
                __CONST_SOCKADDR_ARG {
                    __sockaddr__: &mut inaddr as *mut sockaddr_in as *mut sockaddr,
                },
                ::std::mem::size_of::<sockaddr_in>() as libc::c_ulong as socklen_t,
            );
            if ret == 0 {
                break;
            }
            port += 1;
            port;
        }
        if ret < 0 as libc::c_int {
            close(sock);
            return -(1 as libc::c_int);
        }
        sprintf(buff.as_mut_ptr(), b"%d\0" as *const u8 as *const libc::c_char, port);
        if !rctrlport.is_null() {
            *rctrlport = xstrdup(buff.as_mut_ptr());
        }
        return sock;
    } else if sscanf(
        ctrlport,
        b"%d\0" as *const u8 as *const libc::c_char,
        &mut port as *mut libc::c_int,
    ) == 1 as libc::c_int
        || sscanf(
            ctrlport,
            b"*:%d\0" as *const u8 as *const libc::c_char,
            &mut port as *mut libc::c_int,
        ) == 1 as libc::c_int
    {
        let mut inaddr_0: sockaddr_in = sockaddr_in {
            sin_family: 0,
            sin_port: 0,
            sin_addr: in_addr { s_addr: 0 },
            sin_zero: [0; 8],
        };
        sock = socket(2 as libc::c_int, SOCK_STREAM as libc::c_int, 0 as libc::c_int);
        if sock < 0 as libc::c_int {
            return -(1 as libc::c_int);
        }
        inaddr_0.sin_family = 2 as libc::c_int as sa_family_t;
        inaddr_0.sin_port = __bswap_16(port as __uint16_t);
        inaddr_0.sin_addr.s_addr = 0 as libc::c_int as in_addr_t;
        ret = bind(
            sock,
            __CONST_SOCKADDR_ARG {
                __sockaddr__: &mut inaddr_0 as *mut sockaddr_in as *mut sockaddr,
            },
            ::std::mem::size_of::<sockaddr_in>() as libc::c_ulong as socklen_t,
        );
        if ret < 0 as libc::c_int {
            close(sock);
            return -(1 as libc::c_int);
        }
        sprintf(buff.as_mut_ptr(), b"%d\0" as *const u8 as *const libc::c_char, port);
        if !rctrlport.is_null() {
            *rctrlport = xstrdup(buff.as_mut_ptr());
        }
        return sock;
    } else {
        colon = strchr(ctrlport, ':' as i32);
        if !colon.is_null()
            && sscanf(
                colon.offset(1 as libc::c_int as isize),
                b"%d\0" as *const u8 as *const libc::c_char,
                &mut port as *mut libc::c_int,
            ) == 1 as libc::c_int
        {
            let mut inaddr_1: sockaddr_in = sockaddr_in {
                sin_family: 0,
                sin_port: 0,
                sin_addr: in_addr { s_addr: 0 },
                sin_zero: [0; 8],
            };
            let mut hostname: *mut libc::c_char = 0 as *mut libc::c_char;
            let mut peer: *mut hostent = 0 as *mut hostent;
            hostname = xmalloc(
                (colon.offset_from(ctrlport) as libc::c_long as size_t)
                    .wrapping_add(1 as libc::c_int as libc::c_ulong),
            ) as *mut libc::c_char;
            memcpy(
                hostname as *mut libc::c_void,
                ctrlport as *const libc::c_void,
                colon.offset_from(ctrlport) as libc::c_long as libc::c_ulong,
            );
            *hostname
                .offset(
                    colon.offset_from(ctrlport) as libc::c_long as isize,
                ) = 0 as libc::c_int as libc::c_char;
            peer = gethostbyname(hostname);
            free(hostname as *mut libc::c_void);
            if peer.is_null() || (*peer).h_addrtype != 2 as libc::c_int {
                return -(1 as libc::c_int);
            }
            sock = socket(
                2 as libc::c_int,
                SOCK_STREAM as libc::c_int,
                0 as libc::c_int,
            );
            if sock < 0 as libc::c_int {
                return -(1 as libc::c_int);
            }
            inaddr_1.sin_family = 2 as libc::c_int as sa_family_t;
            inaddr_1.sin_port = __bswap_16(port as __uint16_t);
            memcpy(
                &mut inaddr_1.sin_addr.s_addr as *mut in_addr_t as *mut libc::c_void,
                *((*peer).h_addr_list).offset(0 as libc::c_int as isize)
                    as *const libc::c_void,
                (*peer).h_length as libc::c_ulong,
            );
            ret = bind(
                sock,
                __CONST_SOCKADDR_ARG {
                    __sockaddr__: &mut inaddr_1 as *mut sockaddr_in as *mut sockaddr,
                },
                ::std::mem::size_of::<sockaddr_in>() as libc::c_ulong as socklen_t,
            );
            if ret < 0 as libc::c_int {
                close(sock);
                return -(1 as libc::c_int);
            }
            if !rctrlport.is_null() {
                *rctrlport = xstrdup(ctrlport);
            }
            return sock;
        } else if allow_auto != 0
            && strcmp(ctrlport, b"unix\0" as *const u8 as *const libc::c_char)
                == 0 as libc::c_int
        {
            let mut unaddr: sockaddr_un = sockaddr_un {
                sun_family: 0,
                sun_path: [0; 108],
            };
            sock = socket(
                1 as libc::c_int,
                SOCK_STREAM as libc::c_int,
                0 as libc::c_int,
            );
            if sock < 0 as libc::c_int {
                return -(1 as libc::c_int);
            }
            unaddr.sun_family = 1 as libc::c_int as sa_family_t;
            pid = getpid();
            sprintf(
                (unaddr.sun_path).as_mut_ptr(),
                b"/tmp/pexec.%d.sock\0" as *const u8 as *const libc::c_char,
                pid,
            );
            if stat((unaddr.sun_path).as_mut_ptr(), &mut st) == 0
                && st.st_mode & 0o170000 as libc::c_int as libc::c_uint
                    == 0o140000 as libc::c_int as libc::c_uint
            {
                unlink((unaddr.sun_path).as_mut_ptr());
            }
            ret = bind(
                sock,
                __CONST_SOCKADDR_ARG {
                    __sockaddr__: &mut unaddr as *mut sockaddr_un as *mut sockaddr,
                },
                ::std::mem::size_of::<sockaddr_un>() as libc::c_ulong as socklen_t,
            );
            if ret < 0 as libc::c_int {
                close(sock);
                return -(1 as libc::c_int);
            }
            if !rctrlport.is_null() {
                *rctrlport = xstrdup((unaddr.sun_path).as_mut_ptr());
            }
            return sock;
        } else if is_unix_socket_name(ctrlport) != 0 {
            let mut unaddr_0: sockaddr_un = sockaddr_un {
                sun_family: 0,
                sun_path: [0; 108],
            };
            sock = socket(
                1 as libc::c_int,
                SOCK_STREAM as libc::c_int,
                0 as libc::c_int,
            );
            if sock < 0 as libc::c_int {
                return -(1 as libc::c_int);
            }
            unaddr_0.sun_family = 1 as libc::c_int as sa_family_t;
            strncpy(
                (unaddr_0.sun_path).as_mut_ptr(),
                ctrlport,
                (108 as libc::c_int - 1 as libc::c_int) as libc::c_ulong,
            );
            unaddr_0
                .sun_path[(108 as libc::c_int - 1 as libc::c_int)
                as usize] = 0 as libc::c_int as libc::c_char;
            if fail_on_existing != 0
                && stat((unaddr_0.sun_path).as_mut_ptr(), &mut st) == 0
            {
                close(sock);
                return -(1 as libc::c_int);
            } else if stat((unaddr_0.sun_path).as_mut_ptr(), &mut st) == 0
                && st.st_mode & 0o170000 as libc::c_int as libc::c_uint
                    == 0o140000 as libc::c_int as libc::c_uint
            {
                unlink((unaddr_0.sun_path).as_mut_ptr());
            }
            ret = bind(
                sock,
                __CONST_SOCKADDR_ARG {
                    __sockaddr__: &mut unaddr_0 as *mut sockaddr_un as *mut sockaddr,
                },
                ::std::mem::size_of::<sockaddr_un>() as libc::c_ulong as socklen_t,
            );
            if ret < 0 as libc::c_int {
                close(sock);
                return -(1 as libc::c_int);
            }
            if !rctrlport.is_null() {
                *rctrlport = xstrdup((unaddr_0.sun_path).as_mut_ptr());
            }
            return sock;
        } else {
            return -(1 as libc::c_int)
        }
    };
}
pub unsafe extern "C" fn bind_variable_export(
    mut envvar: *mut libc::c_char,
    mut ctrlport: *mut libc::c_char,
) -> libc::c_int {
    if ctrlport.is_null() {
        return -(1 as libc::c_int);
    }
    env_export(envvar, ctrlport);
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn remote_control_port_connect(
    mut ctrlport: *mut libc::c_char,
) -> libc::c_int {
    let mut sock: libc::c_int = 0;
    let mut port: libc::c_int = 0;
    let mut ret: libc::c_int = 0;
    let mut inaddr: sockaddr_in = sockaddr_in {
        sin_family: 0,
        sin_port: 0,
        sin_addr: in_addr { s_addr: 0 },
        sin_zero: [0; 8],
    };
    let mut unaddr: sockaddr_un = sockaddr_un {
        sun_family: 0,
        sun_path: [0; 108],
    };
    let mut sc: *mut libc::c_char = 0 as *mut libc::c_char;
    if ctrlport.is_null() || *ctrlport.offset(0 as libc::c_int as isize) == 0 {
        return -(1 as libc::c_int)
    } else if is_unix_socket_name(ctrlport) != 0 {
        sock = socket(1 as libc::c_int, SOCK_STREAM as libc::c_int, 0 as libc::c_int);
        if sock < 0 as libc::c_int {
            return -(1 as libc::c_int);
        }
        unaddr.sun_family = 1 as libc::c_int as sa_family_t;
        strncpy(
            (unaddr.sun_path).as_mut_ptr(),
            ctrlport,
            (108 as libc::c_int - 1 as libc::c_int) as libc::c_ulong,
        );
        unaddr
            .sun_path[(108 as libc::c_int - 1 as libc::c_int)
            as usize] = 0 as libc::c_int as libc::c_char;
        ret = connect(
            sock,
            __CONST_SOCKADDR_ARG {
                __sockaddr__: &mut unaddr as *mut sockaddr_un as *mut sockaddr,
            },
            ::std::mem::size_of::<sockaddr_un>() as libc::c_ulong as socklen_t,
        );
        if ret < 0 as libc::c_int {
            close(sock);
            return -(1 as libc::c_int);
        }
        return sock;
    } else {
        inaddr.sin_family = 2 as libc::c_int as sa_family_t;
        sc = strchr(ctrlport, ':' as i32);
        if sc.is_null() {
            if sscanf(
                ctrlport,
                b"%d\0" as *const u8 as *const libc::c_char,
                &mut port as *mut libc::c_int,
            ) < 1 as libc::c_int
            {
                return -(1 as libc::c_int);
            }
            inaddr.sin_addr.s_addr = __bswap_32(0x7f000001 as libc::c_int as in_addr_t);
            inaddr.sin_port = __bswap_16(port as __uint16_t);
        } else {
            let mut hostname: *mut libc::c_char = 0 as *mut libc::c_char;
            let mut len: libc::c_int = 0;
            let mut peer: *mut hostent = 0 as *mut hostent;
            if sscanf(
                sc.offset(1 as libc::c_int as isize),
                b"%d\0" as *const u8 as *const libc::c_char,
                &mut port as *mut libc::c_int,
            ) < 1 as libc::c_int
            {
                return -(1 as libc::c_int);
            }
            len = sc.offset_from(ctrlport) as libc::c_long as libc::c_int;
            hostname = xmalloc((len + 1 as libc::c_int) as size_t) as *mut libc::c_char;
            memcpy(
                hostname as *mut libc::c_void,
                ctrlport as *const libc::c_void,
                len as libc::c_ulong,
            );
            *hostname.offset(len as isize) = 0 as libc::c_int as libc::c_char;
            peer = gethostbyname(hostname);
            free(hostname as *mut libc::c_void);
            if peer.is_null() {
                return -(1 as libc::c_int);
            }
            memcpy(
                &mut inaddr.sin_addr.s_addr as *mut in_addr_t as *mut libc::c_void,
                *((*peer).h_addr_list).offset(0 as libc::c_int as isize)
                    as *const libc::c_void,
                (*peer).h_length as libc::c_ulong,
            );
            inaddr.sin_port = __bswap_16(port as __uint16_t);
        }
        sock = socket(2 as libc::c_int, SOCK_STREAM as libc::c_int, 0 as libc::c_int);
        if sock < 0 as libc::c_int {
            return -(1 as libc::c_int);
        }
        ret = connect(
            sock,
            __CONST_SOCKADDR_ARG {
                __sockaddr__: &mut inaddr as *mut sockaddr_in as *mut sockaddr,
            },
            ::std::mem::size_of::<sockaddr_in>() as libc::c_ulong as socklen_t,
        );
        if ret < 0 as libc::c_int {
            close(sock);
            return -(1 as libc::c_int);
        }
        return sock;
    };
}
pub unsafe extern "C" fn remote_status(
    mut sock: libc::c_int,
    mut fw: *mut FILE,
) -> libc::c_int {
    let mut lrcv: linebuffer = linebuffer {
        buffer: 0 as *mut libc::c_char,
        length: 0,
    };
    let mut line: *mut libc::c_char = 0 as *mut libc::c_char;
    hprintf(
        sock,
        b"status --all\n\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    linebuffer_reset(&mut lrcv);
    line = linebuffer_read_line(sock, &mut lrcv, 0 as libc::c_int);
    if !line.is_null() {
        remove_newlines_and_comments(line);
        fprintf(fw, b"%s\n\0" as *const u8 as *const libc::c_char, line);
        free(line as *mut libc::c_void);
    }
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn remote_lock(
    mut sock: libc::c_int,
    mut fw: *mut FILE,
    mut name: *mut libc::c_char,
) -> libc::c_int {
    let mut lrcv: linebuffer = linebuffer {
        buffer: 0 as *mut libc::c_char,
        length: 0,
    };
    let mut line: *mut libc::c_char = 0 as *mut libc::c_char;
    hprintf(
        sock,
        b"lock \"%s\"\n\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        name,
    );
    linebuffer_reset(&mut lrcv);
    line = linebuffer_read_line(sock, &mut lrcv, 0 as libc::c_int);
    if !line.is_null() {
        remove_newlines_and_comments(line);
        if !fw.is_null() {
            fprintf(fw, b"%s\n\0" as *const u8 as *const libc::c_char, line);
        }
        free(line as *mut libc::c_void);
    }
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn remote_unlock(
    mut sock: libc::c_int,
    mut fw: *mut FILE,
    mut name: *mut libc::c_char,
) -> libc::c_int {
    hprintf(
        sock,
        b"unlock \"%s\"\n\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        name,
    );
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn remote_copy(
    mut sock: libc::c_int,
    mut name: *mut libc::c_char,
    mut frin: *mut FILE,
    mut fwout: *mut FILE,
) -> libc::c_int {
    let mut buff: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut blksize: libc::c_int = 0;
    let mut r: libc::c_int = 0;
    let mut fd: libc::c_int = 0;
    let mut set: fd_set = fd_set { fds_bits: [0; 16] };
    let mut __d0: libc::c_int = 0;
    let mut __d1: libc::c_int = 0;
    let fresh76 = &mut __d0;
    let fresh77;
    let fresh78 = (::std::mem::size_of::<fd_set>() as libc::c_ulong)
        .wrapping_div(::std::mem::size_of::<__fd_mask>() as libc::c_ulong);
    let fresh79 = &mut __d1;
    let fresh80;
    let fresh81 = &mut *(set.fds_bits).as_mut_ptr().offset(0 as libc::c_int as isize)
        as *mut __fd_mask;
    asm!(
        "cld; rep; stosq", inlateout("cx") c2rust_asm_casts::AsmCast::cast_in(fresh76,
        fresh78) => fresh77, inlateout("di") c2rust_asm_casts::AsmCast::cast_in(fresh79,
        fresh81) => fresh80, inlateout("ax") 0 as libc::c_int => _,
        options(preserves_flags, att_syntax)
    );
    c2rust_asm_casts::AsmCast::cast_out(fresh76, fresh78, fresh77);
    c2rust_asm_casts::AsmCast::cast_out(fresh79, fresh81, fresh80);
    fd = fileno(frin);
    set
        .fds_bits[(fd
        / (8 as libc::c_int
            * ::std::mem::size_of::<__fd_mask>() as libc::c_ulong as libc::c_int))
        as usize]
        |= ((1 as libc::c_ulong)
            << fd
                % (8 as libc::c_int
                    * ::std::mem::size_of::<__fd_mask>() as libc::c_ulong
                        as libc::c_int)) as __fd_mask;
    select(
        fd + 1 as libc::c_int,
        &mut set,
        0 as *mut fd_set,
        0 as *mut fd_set,
        0 as *mut timeval,
    );
    if sock >= 0 as libc::c_int && !name.is_null() {
        remote_lock(sock, 0 as *mut FILE, name);
    }
    blksize = getpagesize();
    buff = xmalloc(blksize as size_t) as *mut libc::c_char;
    while feof(frin) == 0 {
        r = fread(
            buff as *mut libc::c_void,
            1 as libc::c_int as libc::c_ulong,
            blksize as libc::c_ulong,
            frin,
        ) as libc::c_int;
        if r <= 0 as libc::c_int {
            break;
        }
        fwrite(
            buff as *const libc::c_void,
            1 as libc::c_int as libc::c_ulong,
            r as libc::c_ulong,
            fwout,
        );
    }
    free(buff as *mut libc::c_void);
    if sock >= 0 as libc::c_int && !name.is_null() {
        remote_unlock(sock, 0 as *mut FILE, name);
    }
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn remote_atomic_execute(
    mut is_shell_commands: libc::c_int,
    mut shell: *mut libc::c_char,
    mut argc: libc::c_int,
    mut argv: *mut *mut libc::c_char,
    mut sock: libc::c_int,
) -> libc::c_int {
    let mut pid: libc::c_int = 0;
    let mut status: libc::c_int = 0;
    pid = fork();
    if pid > 0 as libc::c_int {
        waitpid(pid, &mut status, 0 as libc::c_int);
        return (status & 0xff00 as libc::c_int) >> 8 as libc::c_int;
    } else {
        if sock >= 0 as libc::c_int {
            close(sock);
        }
        if is_shell_commands != 0 {
            let mut largv: [*mut libc::c_char; 4] = [0 as *mut libc::c_char; 4];
            largv[0 as libc::c_int as usize] = shell;
            largv[1 as libc::c_int
                as usize] = b"-c\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char;
            largv[2 as libc::c_int as usize] = *argv.offset(0 as libc::c_int as isize);
            largv[3 as libc::c_int as usize] = 0 as *mut libc::c_char;
            execv(shell, largv.as_mut_ptr() as *const *mut libc::c_char);
            fprintf(
                stderr,
                dcgettext(
                    0 as *const libc::c_char,
                    b"%s: unable to execute the script '%s'.\n\0" as *const u8
                        as *const libc::c_char,
                    5 as libc::c_int,
                ),
                shell,
                progbasename,
            );
            exit(2 as libc::c_int);
        } else {
            let mut largv_0: *mut *mut libc::c_char = 0 as *mut *mut libc::c_char;
            let mut i: libc::c_int = 0;
            let mut largc: libc::c_int = 0;
            largc = argc;
            largv_0 = xmalloc(
                (::std::mem::size_of::<*mut libc::c_char>() as libc::c_ulong)
                    .wrapping_mul((argc + 1 as libc::c_int) as libc::c_ulong),
            ) as *mut *mut libc::c_char;
            i = 0 as libc::c_int;
            while i < argc {
                let ref mut fresh82 = *largv_0.offset(i as isize);
                *fresh82 = *argv.offset(i as isize);
                i += 1;
                i;
            }
            let ref mut fresh83 = *largv_0.offset(argc as isize);
            *fresh83 = 0 as *mut libc::c_char;
            execvp(
                *largv_0.offset(0 as libc::c_int as isize),
                largv_0 as *const *mut libc::c_char,
            );
            fprintf(
                stderr,
                dcgettext(
                    0 as *const libc::c_char,
                    b"%s: unable to execute the command '%s'.\n\0" as *const u8
                        as *const libc::c_char,
                    5 as libc::c_int,
                ),
                progbasename,
                *argv.offset(0 as libc::c_int as isize),
            );
            exit(2 as libc::c_int);
        }
    };
}
pub unsafe extern "C" fn remote_disconnect(mut sock: libc::c_int) -> libc::c_int {
    let mut buff: [libc::c_char; 16] = [0; 16];
    let mut n: libc::c_int = 0;
    hprintf(sock, b"exit\n\0" as *const u8 as *const libc::c_char as *mut libc::c_char);
    n = read(sock, buff.as_mut_ptr() as *mut libc::c_void, 16 as libc::c_int as size_t)
        as libc::c_int;
    return n;
}
pub unsafe extern "C" fn pexec_hypervisor_check_load(
    mut loadtype: libc::c_int,
) -> libc::c_int {
    let mut loadavg: [libc::c_double; 3] = [0.; 3];
    if !(0 as libc::c_int <= loadtype && loadtype < 3 as libc::c_int) {
        return 0 as libc::c_int;
    }
    getloadavg(loadavg.as_mut_ptr(), 3 as libc::c_int);
    return loadavg[loadtype as usize] as libc::c_int;
}
pub unsafe extern "C" fn pexec_hypervisor_request_cleanup(
    mut hs: *mut hypervisorstatus,
    mut cl: *mut client,
) -> libc::c_int {
    let mut i: libc::c_int = 0;
    i = 0 as libc::c_int;
    while i < (*hs).nrequest {
        if (*((*hs).requests).offset(i as isize)).cl == cl {
            if i < (*hs).nrequest - 1 as libc::c_int {
                memmove(
                    ((*hs).requests).offset(i as isize) as *mut libc::c_void,
                    ((*hs).requests).offset(i as isize).offset(1 as libc::c_int as isize)
                        as *const libc::c_void,
                    (::std::mem::size_of::<request>() as libc::c_ulong)
                        .wrapping_mul(
                            ((*hs).nrequest - i - 1 as libc::c_int) as libc::c_ulong,
                        ),
                );
            }
            (*hs).nrequest -= 1;
            (*hs).nrequest;
        } else {
            i += 1;
            i;
        }
    }
    if (*hs).nrequest <= 0 as libc::c_int {
        free((*hs).requests as *mut libc::c_void);
        (*hs).requests = 0 as *mut request;
        (*hs).nrequest = 0 as libc::c_int;
    }
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn pexec_hypervisor_acknowledge_pending(
    mut hs: *mut hypervisorstatus,
) -> libc::c_int {
    let mut rw: *mut request = 0 as *mut request;
    while (*hs).nrequest > 0 as libc::c_int && (*hs).nrunning < (*hs).num_processes
        && pexec_hypervisor_check_load((*hs).use_load) < (*hs).num_processes
    {
        if (*hs).use_fifo != 0 {
            rw = &mut *((*hs).requests).offset(0 as libc::c_int as isize)
                as *mut request;
        } else {
            rw = &mut *((*hs).requests)
                .offset(((*hs).nrequest - 1 as libc::c_int) as isize) as *mut request;
        }
        hprintf(
            (*(*rw).cl).peer,
            b"acknowledged\n\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        );
        (*hs).nrunning += 1;
        (*hs).nrunning;
        (*(*rw).cl).status += 1;
        (*(*rw).cl).status;
        if (*hs).nrequest > 1 as libc::c_int && (*hs).use_fifo != 0 {
            memmove(
                (*hs).requests as *mut libc::c_void,
                ((*hs).requests).offset(1 as libc::c_int as isize)
                    as *const libc::c_void,
                (::std::mem::size_of::<request>() as libc::c_ulong)
                    .wrapping_mul(((*hs).nrequest - 1 as libc::c_int) as libc::c_ulong),
            );
        }
        (*hs).nrequest -= 1;
        (*hs).nrequest;
        if (*hs).nrequest <= 0 as libc::c_int {
            free((*hs).requests as *mut libc::c_void);
            (*hs).requests = 0 as *mut request;
            (*hs).nrequest = 0 as libc::c_int;
        }
    }
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn pexec_hypervisor_client_parse_command(
    mut line: *mut libc::c_char,
    mut hs: *mut hypervisorstatus,
    mut cl: *mut client,
) -> libc::c_int {
    let mut cmd: *mut *mut libc::c_char = 0 as *mut *mut libc::c_char;
    let mut rw: *mut request = 0 as *mut request;
    let mut ret: libc::c_int = 0;
    cmd = tokenize_spaces_dyn(line);
    if cmd.is_null() {
        return 0 as libc::c_int
    } else if (*cmd.offset(0 as libc::c_int as isize)).is_null() {
        free(cmd as *mut libc::c_void);
        return 0 as libc::c_int;
    }
    if strcmp(
        *cmd.offset(0 as libc::c_int as isize),
        b"request\0" as *const u8 as *const libc::c_char,
    ) == 0 as libc::c_int
    {
        (*hs)
            .requests = xrealloc(
            (*hs).requests as *mut libc::c_void,
            (::std::mem::size_of::<request>() as libc::c_ulong)
                .wrapping_mul(((*hs).nrequest + 1 as libc::c_int) as libc::c_ulong),
        ) as *mut request;
        rw = &mut *((*hs).requests).offset((*hs).nrequest as isize) as *mut request;
        (*rw).cl = cl;
        (*hs).nrequest += 1;
        (*hs).nrequest;
        pexec_hypervisor_acknowledge_pending(hs);
        ret = 0 as libc::c_int;
    } else if strcmp(
        *cmd.offset(0 as libc::c_int as isize),
        b"ready\0" as *const u8 as *const libc::c_char,
    ) == 0 as libc::c_int
    {
        (*hs).nrunning -= 1;
        (*hs).nrunning;
        (*cl).status -= 1;
        (*cl).status;
        pexec_hypervisor_acknowledge_pending(hs);
        ret = 0 as libc::c_int;
    } else if strcmp(
        *cmd.offset(0 as libc::c_int as isize),
        b"completed\0" as *const u8 as *const libc::c_char,
    ) == 0 as libc::c_int
    {
        ret = 0 as libc::c_int;
    } else if strcmp(
        *cmd.offset(0 as libc::c_int as isize),
        b"status\0" as *const u8 as *const libc::c_char,
    ) == 0 as libc::c_int
    {
        let mut loadavg: [libc::c_double; 3] = [0.; 3];
        getloadavg(loadavg.as_mut_ptr(), 3 as libc::c_int);
        hprintf(
            (*cl).peer,
            b"status num_processes=%d use_load=%d use_fifo=%d nrunning=%d nrequest=%d load=%.2f,%.2f,%.2f\n\0"
                as *const u8 as *const libc::c_char as *mut libc::c_char,
            (*hs).num_processes,
            (*hs).use_load,
            (*hs).use_fifo,
            (*hs).nrunning,
            (*hs).nrequest,
            loadavg[0 as libc::c_int as usize],
            loadavg[1 as libc::c_int as usize],
            loadavg[2 as libc::c_int as usize],
        );
        ret = 0 as libc::c_int;
    } else if strcmp(
        *cmd.offset(0 as libc::c_int as isize),
        b"set\0" as *const u8 as *const libc::c_char,
    ) == 0 as libc::c_int
    {
        let mut i: libc::c_int = 0;
        let mut w: libc::c_int = 0;
        let mut invvar: *mut libc::c_char = 0 as *mut libc::c_char;
        invvar = 0 as *mut libc::c_char;
        i = 1 as libc::c_int;
        while !(*cmd.offset(i as isize)).is_null() {
            if sscanf(
                *cmd.offset(i as isize),
                b"num_processes=%d\0" as *const u8 as *const libc::c_char,
                &mut w as *mut libc::c_int,
            ) == 1 as libc::c_int
            {
                if w < 0 as libc::c_int {
                    w = 0 as libc::c_int;
                }
                (*hs).num_processes = w;
            } else if sscanf(
                *cmd.offset(i as isize),
                b"use_load=%d\0" as *const u8 as *const libc::c_char,
                &mut w as *mut libc::c_int,
            ) == 1 as libc::c_int
            {
                if w < 0 as libc::c_int {
                    w = -(1 as libc::c_int);
                } else if w > 2 as libc::c_int {
                    w = 2 as libc::c_int;
                }
                (*hs).use_load = w;
            } else if sscanf(
                *cmd.offset(i as isize),
                b"use_fifo=%d\0" as *const u8 as *const libc::c_char,
                &mut w as *mut libc::c_int,
            ) == 1 as libc::c_int
            {
                if w == 0 {
                    w = 0 as libc::c_int;
                } else {
                    w = 1 as libc::c_int;
                }
                (*hs).use_fifo = w;
            } else {
                invvar = *cmd.offset(i as isize);
            }
            i += 1;
            i;
        }
        if !invvar.is_null() {
            hprintf(
                (*cl).peer,
                b"message \"invalid variable alternation '%s'\"\n\0" as *const u8
                    as *const libc::c_char as *mut libc::c_char,
                invvar,
            );
        }
        ret = 0 as libc::c_int;
    } else if strcmp(
        *cmd.offset(0 as libc::c_int as isize),
        b"close\0" as *const u8 as *const libc::c_char,
    ) == 0 as libc::c_int
    {
        ret = 1 as libc::c_int;
    } else if strcmp(
        *cmd.offset(0 as libc::c_int as isize),
        b"terminate\0" as *const u8 as *const libc::c_char,
    ) == 0 as libc::c_int
    {
        ret = -(1 as libc::c_int);
    } else {
        hprintf(
            (*cl).peer,
            b"message \"invalid command '%s'\"\n\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            *cmd.offset(0 as libc::c_int as isize),
        );
        ret = 0 as libc::c_int;
    }
    free(cmd as *mut libc::c_void);
    return ret;
}
pub unsafe extern "C" fn pexec_hypervisor_main_loop(
    mut sock: libc::c_int,
    mut num_processes: libc::c_int,
    mut use_load: libc::c_int,
    mut use_fifo: libc::c_int,
) -> libc::c_int {
    let mut cl: *mut client = 0 as *mut client;
    let mut cnext: *mut client = 0 as *mut client;
    let mut hs: hypervisorstatus = hypervisorstatus {
        clientlist: 0 as *mut client,
        num_processes: 0,
        use_load: 0,
        use_fifo: 0,
        requests: 0 as *mut request,
        nrequest: 0,
        nrunning: 0,
    };
    let mut set: fd_set = fd_set { fds_bits: [0; 16] };
    let mut i: libc::c_int = 0;
    let mut max: libc::c_int = 0;
    let mut buffsize: libc::c_int = 0;
    let mut n: libc::c_int = 0;
    let mut buff: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut is_in_loop: libc::c_int = 0;
    let mut intact: sigaction = sigaction {
        __sigaction_handler: C2RustUnnamed_10 {
            sa_handler: None,
        },
        sa_mask: __sigset_t { __val: [0; 16] },
        sa_flags: 0,
        sa_restorer: None,
    };
    let mut sci: signalinfo = signalinfo {
        signal: 0,
        pid: 0,
        exitstatus: 0,
        exitsignal: 0,
    };
    let mut spipe: libc::c_int = 0;
    let mut tv: timeval = timeval { tv_sec: 0, tv_usec: 0 };
    if sock < 0 as libc::c_int {
        return -(1 as libc::c_int);
    }
    hs.clientlist = 0 as *mut client;
    hs.num_processes = num_processes;
    hs.use_load = use_load;
    hs.use_fifo = use_fifo;
    hs.requests = 0 as *mut request;
    hs.nrequest = 0 as libc::c_int;
    hs.nrunning = 0 as libc::c_int;
    buffsize = getpagesize();
    buff = xmalloc(buffsize as size_t) as *mut libc::c_char;
    is_in_loop = 1 as libc::c_int;
    if pipe(sig_pipe.as_mut_ptr()) < 0 as libc::c_int {
        return -(1 as libc::c_int);
    }
    intact
        .__sigaction_handler
        .sa_handler = Some(sig_act_interrupt as unsafe extern "C" fn(libc::c_int) -> ());
    sigemptyset(&mut intact.sa_mask);
    intact.sa_flags = 1 as libc::c_int | 0x10000000 as libc::c_int;
    sigaction(2 as libc::c_int, &mut intact, 0 as *mut sigaction);
    spipe = sig_pipe[0 as libc::c_int as usize];
    while is_in_loop != 0 {
        let mut __d0: libc::c_int = 0;
        let mut __d1: libc::c_int = 0;
        let fresh84 = &mut __d0;
        let fresh85;
        let fresh86 = (::std::mem::size_of::<fd_set>() as libc::c_ulong)
            .wrapping_div(::std::mem::size_of::<__fd_mask>() as libc::c_ulong);
        let fresh87 = &mut __d1;
        let fresh88;
        let fresh89 = &mut *(set.fds_bits).as_mut_ptr().offset(0 as libc::c_int as isize)
            as *mut __fd_mask;
        asm!(
            "cld; rep; stosq", inlateout("cx")
            c2rust_asm_casts::AsmCast::cast_in(fresh84, fresh86) => fresh85,
            inlateout("di") c2rust_asm_casts::AsmCast::cast_in(fresh87, fresh89) =>
            fresh88, inlateout("ax") 0 as libc::c_int => _, options(preserves_flags,
            att_syntax)
        );
        c2rust_asm_casts::AsmCast::cast_out(fresh84, fresh86, fresh85);
        c2rust_asm_casts::AsmCast::cast_out(fresh87, fresh89, fresh88);
        set
            .fds_bits[(sock
            / (8 as libc::c_int
                * ::std::mem::size_of::<__fd_mask>() as libc::c_ulong as libc::c_int))
            as usize]
            |= ((1 as libc::c_ulong)
                << sock
                    % (8 as libc::c_int
                        * ::std::mem::size_of::<__fd_mask>() as libc::c_ulong
                            as libc::c_int)) as __fd_mask;
        max = sock;
        cl = hs.clientlist;
        while !cl.is_null() {
            set
                .fds_bits[((*cl).peer
                / (8 as libc::c_int
                    * ::std::mem::size_of::<__fd_mask>() as libc::c_ulong
                        as libc::c_int)) as usize]
                |= ((1 as libc::c_ulong)
                    << (*cl).peer
                        % (8 as libc::c_int
                            * ::std::mem::size_of::<__fd_mask>() as libc::c_ulong
                                as libc::c_int)) as __fd_mask;
            if max < (*cl).peer {
                max = (*cl).peer;
            }
            cl = (*cl).next;
        }
        if spipe >= 0 as libc::c_int {
            set
                .fds_bits[(spipe
                / (8 as libc::c_int
                    * ::std::mem::size_of::<__fd_mask>() as libc::c_ulong
                        as libc::c_int)) as usize]
                |= ((1 as libc::c_ulong)
                    << spipe
                        % (8 as libc::c_int
                            * ::std::mem::size_of::<__fd_mask>() as libc::c_ulong
                                as libc::c_int)) as __fd_mask;
            if max < spipe {
                max = spipe;
            }
        }
        if use_load >= 0 as libc::c_int {
            tv.tv_sec = 2 as libc::c_int as __time_t;
            tv.tv_usec = 0 as libc::c_int as __suseconds_t;
            i = select(
                max + 1 as libc::c_int,
                &mut set,
                0 as *mut fd_set,
                0 as *mut fd_set,
                &mut tv,
            );
        } else {
            i = select(
                max + 1 as libc::c_int,
                &mut set,
                0 as *mut fd_set,
                0 as *mut fd_set,
                0 as *mut timeval,
            );
        }
        if i < 0 as libc::c_int && *__errno_location() == 4 as libc::c_int {
            continue;
        }
        pexec_hypervisor_acknowledge_pending(&mut hs);
        cl = hs.clientlist;
        while !cl.is_null() {
            let mut line: *mut libc::c_char = 0 as *mut libc::c_char;
            let mut closepeer: libc::c_int = 0;
            cnext = (*cl).next;
            if set
                .fds_bits[((*cl).peer
                / (8 as libc::c_int
                    * ::std::mem::size_of::<__fd_mask>() as libc::c_ulong
                        as libc::c_int)) as usize]
                & ((1 as libc::c_ulong)
                    << (*cl).peer
                        % (8 as libc::c_int
                            * ::std::mem::size_of::<__fd_mask>() as libc::c_ulong
                                as libc::c_int)) as __fd_mask
                != 0 as libc::c_int as libc::c_long
            {
                n = read((*cl).peer, buff as *mut libc::c_void, buffsize as size_t)
                    as libc::c_int;
                if n <= 0 as libc::c_int {
                    closepeer = 1 as libc::c_int;
                } else {
                    closepeer = 0 as libc::c_int;
                }
                if n > 0 as libc::c_int && closepeer == 0 {
                    linebuffer_concatenate(&mut (*cl).lcli, buff, n as size_t);
                }
                loop {
                    line = linebuffer_fetch(&mut (*cl).lcli);
                    if !(!line.is_null() && closepeer == 0) {
                        break;
                    }
                    closepeer = pexec_hypervisor_client_parse_command(line, &mut hs, cl);
                    free(line as *mut libc::c_void);
                    if closepeer != 0 {
                        break;
                    }
                }
                if closepeer != 0 {
                    close((*cl).peer);
                    if (*cl).status > 0 as libc::c_int {
                        hs.nrunning -= (*cl).status;
                    }
                    linebuffer_free(&mut (*cl).lcli);
                    pexec_hypervisor_request_cleanup(&mut hs, cl);
                    if !((*cl).next).is_null() {
                        (*(*cl).next).prev = (*cl).prev;
                    }
                    if !((*cl).prev).is_null() {
                        (*(*cl).prev).next = (*cl).next;
                    } else {
                        hs.clientlist = (*cl).next;
                    }
                    free(cl as *mut libc::c_void);
                    pexec_hypervisor_acknowledge_pending(&mut hs);
                }
                if closepeer < 0 as libc::c_int {
                    is_in_loop = 0 as libc::c_int;
                }
            }
            cl = cnext;
        }
        if set
            .fds_bits[(spipe
            / (8 as libc::c_int
                * ::std::mem::size_of::<__fd_mask>() as libc::c_ulong as libc::c_int))
            as usize]
            & ((1 as libc::c_ulong)
                << spipe
                    % (8 as libc::c_int
                        * ::std::mem::size_of::<__fd_mask>() as libc::c_ulong
                            as libc::c_int)) as __fd_mask
            != 0 as libc::c_int as libc::c_long
        {
            if !(read_signalinfo(spipe, &mut sci) > 0 as libc::c_int) {
                fprintf(
                    stderr,
                    dcgettext(
                        0 as *const libc::c_char,
                        b"%s: fatal error: read_signalinfo() failed.\n\0" as *const u8
                            as *const libc::c_char,
                        5 as libc::c_int,
                    ),
                    progbasename,
                );
                exit(1 as libc::c_int);
            }
            if sci.signal == 2 as libc::c_int && sci.exitsignal != 0 {
                is_in_loop = 0 as libc::c_int;
            }
        }
        if set
            .fds_bits[(sock
            / (8 as libc::c_int
                * ::std::mem::size_of::<__fd_mask>() as libc::c_ulong as libc::c_int))
            as usize]
            & ((1 as libc::c_ulong)
                << sock
                    % (8 as libc::c_int
                        * ::std::mem::size_of::<__fd_mask>() as libc::c_ulong
                            as libc::c_int)) as __fd_mask
            != 0 as libc::c_int as libc::c_long
        {
            cl = xmalloc(::std::mem::size_of::<client>() as libc::c_ulong)
                as *mut client;
            (*cl)
                .peer = accept(
                sock,
                __SOCKADDR_ARG {
                    __sockaddr__: 0 as *mut libc::c_void as *mut sockaddr,
                },
                0 as *mut socklen_t,
            );
            linebuffer_reset(&mut (*cl).lcli);
            (*cl).next = hs.clientlist;
            (*cl).prev = 0 as *mut client;
            if !(hs.clientlist).is_null() {
                (*hs.clientlist).prev = cl;
            }
            hs.clientlist = cl;
            (*cl).status = 0 as libc::c_int;
        }
    }
    signal(2 as libc::c_int, None);
    close(sig_pipe[0 as libc::c_int as usize]);
    sig_pipe[0 as libc::c_int as usize] = -(1 as libc::c_int);
    close(sig_pipe[1 as libc::c_int as usize]);
    sig_pipe[1 as libc::c_int as usize] = -(1 as libc::c_int);
    free(buff as *mut libc::c_void);
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn pexec_hypervisor_stop(mut sock: libc::c_int) -> libc::c_int {
    hprintf(
        sock,
        b"terminate\n\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn fprint_parameters(
    mut fw: *mut FILE,
    mut params: *mut parameter,
    mut nparam: libc::c_int,
) -> libc::c_int {
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    i = 0 as libc::c_int;
    while i < nparam {
        fprintf(
            fw,
            b"name='%s' is_shell=%d \0" as *const u8 as *const libc::c_char,
            (*params.offset(i as isize)).name,
            (*params.offset(i as isize)).c.is_shell,
        );
        fprintf(fw, b"args:\0" as *const u8 as *const libc::c_char);
        j = 0 as libc::c_int;
        while j < (*params.offset(i as isize)).c.argc {
            fprintf(
                fw,
                b" '%s'\0" as *const u8 as *const libc::c_char,
                *((*params.offset(i as isize)).c.argv).offset(j as isize),
            );
            j += 1;
            j;
        }
        fprintf(fw, b"\n\0" as *const u8 as *const libc::c_char);
        i += 1;
        i;
    }
    return 0 as libc::c_int;
}
pub static mut pexec_long_help: [longhelp_entry; 53] = [
    {
        let mut init = longhelp_entry {
            options: b"Options:\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            description: 0 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = longhelp_entry {
            options: b"General options:\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            description: 0 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = longhelp_entry {
            options: b"-h, --help\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            description: b"Gives general summary about the command line options.\0"
                as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = longhelp_entry {
            options: b"--long-help\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            description: b"Gives a detailed list of command line options.\0" as *const u8
                as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = longhelp_entry {
            options: b"--version\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            description: b"Gives some version information about the program.\0"
                as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = longhelp_entry {
            options: b"-s, --shell <shell>\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            description: b"Full path (e.g. /bin/sh) of the shell or interpreter to be used for script execution.\0"
                as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = longhelp_entry {
            options: b"-c, --shell-command\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            description: b"Use the specified shell to interpret the command(s) instead of direct execution.\0"
                as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = longhelp_entry {
            options: b"-m, --multiple-command\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            description: b"Allow multiple individual shell command scripts to be executed in parallel with the variation of the parameters.\0"
                as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = longhelp_entry {
            options: b"-e, --environment <variable>\0" as *const u8
                as *const libc::c_char as *mut libc::c_char,
            description: b"Name of an environmental variable which is set to the respective parameter before each execution.\0"
                as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = longhelp_entry {
            options: b"-n, --number <number>\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            description: b"The maximal number of processes running simultaneously. The <number> itself can even be a complex specification of remote hosts (see documentation for more details).\0"
                as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = longhelp_entry {
            options: b"-C, --control <port>\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            description: b"The control port of a hypervisor daemon (full path of a UNIX socket or an INET host specification).\0"
                as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = longhelp_entry {
            options: b"-p, --list <list>\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            description: b"The single-argument form of main parameter list.\0"
                as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = longhelp_entry {
            options: b"-r, --parameters <list>\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            description: b"The multiple-argument form of the main parameter list.\0"
                as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = longhelp_entry {
            options: b"-f, --listfile <file>\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            description: b"The main parameter list file.\0" as *const u8
                as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = longhelp_entry {
            options: b"-w, --column <index>\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            description: b"The column index from where the parameters should be taken if they are read from a parameter file.\0"
                as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = longhelp_entry {
            options: b"-t, --complete\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            description: b"Threat the whole line as a single parameter if the parameters are read from a file.\0"
                as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = longhelp_entry {
            options: b"-z, --nice\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            description: b"Sets the scheduling priority of pexec and all children (executed processes) to the priority defined by this nice value.\0"
                as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = longhelp_entry {
            options: b"--\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            description: b"A marker after which the command to execute begins.\0"
                as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = longhelp_entry {
            options: b"Redirecting standard input, output and error:\0" as *const u8
                as *const libc::c_char as *mut libc::c_char,
            description: 0 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = longhelp_entry {
            options: b"-i, --input <input>\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            description: b"The (optionally formatted) name of the input file which is used for redirecting the standard input.\0"
                as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = longhelp_entry {
            options: b"-o, --output <output>\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            description: b"The (optionally formatted) name of the output file which is used for redirecting the standard output.\0"
                as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = longhelp_entry {
            options: b"-u, --error <output>\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            description: b"The (optionally formatted) name of the output error file, which is used for redirecting the standard error.\0"
                as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = longhelp_entry {
            options: b"-R, --normal-redirection\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            description: b"Equivalent to specifying --output -, --error - and --input /dev/null.\0"
                as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = longhelp_entry {
            options: b"-a, --output-format <format>\0" as *const u8
                as *const libc::c_char as *mut libc::c_char,
            description: b"The format of the final standard output redirection if the output of all of the processes are gathered into the same file.\0"
                as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = longhelp_entry {
            options: b"-b, --error-format <format>\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            description: b"The same final redirection format for the standard error.\0"
                as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = longhelp_entry {
            options: b"-x, --omit-newlines\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            description: b"Disable automatic newlines after the output and error formats.\0"
                as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = longhelp_entry {
            options: b"Execution using remote hosts:\0" as *const u8
                as *const libc::c_char as *mut libc::c_char,
            description: 0 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = longhelp_entry {
            options: b"-g, --remote-shell <remote_shell>\0" as *const u8
                as *const libc::c_char as *mut libc::c_char,
            description: b"The name or full path of the remote shell to be used for building the tunnel between the local and the peer host(s). Default: ``/usr/bin/ssh''.\0"
                as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = longhelp_entry {
            options: b"-P, --pexec <pexec>\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            description: b"The full path of the pexec program on the remote hosts. If this option is omitted, pexec tries to figure out from the invoking syntax and/or the current path.\0"
                as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = longhelp_entry {
            options: b"-T, --tunnel\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            description: b"Internal use only (pexec will start in tunnel daemon mode).\0"
                as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = longhelp_entry {
            options: b"Remote control, mutual exclusions and atomic command execution:\0"
                as *const u8 as *const libc::c_char as *mut libc::c_char,
            description: 0 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = longhelp_entry {
            options: b"-y, --bind <port>\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            description: b"This option lets pexec to be remote controlled via INET or UNIX domain sockets.\0"
                as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = longhelp_entry {
            options: b"-E, --pexec-connection-variable <env>\0" as *const u8
                as *const libc::c_char as *mut libc::c_char,
            description: b"This option overrides the default environment name PEXEC_REMOTE_PORT to the specified value, which is used by the ``-p|--connect auto'' combination to determine the control socket with which the running pexec instance can be controlled.\0"
                as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = longhelp_entry {
            options: b"-j, --remote\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            description: b"Used to remote control and/or poll the status of other running instances of pexec.\0"
                as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = longhelp_entry {
            options: b"-p, --connect <port>\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            description: b"Remote control port to connect to.\0" as *const u8
                as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = longhelp_entry {
            options: b"-t, --status\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            description: b"Prints the actual status of the running jobs in a human-readable form.\0"
                as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = longhelp_entry {
            options: b"-l, --lock <mutex>\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            description: b"Locks the specified mutex (if the mutex is not locked by someone else, otherwise it will block until the mutex is released).\0"
                as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = longhelp_entry {
            options: b"-u, --unlock <mutex>\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            description: b"Unlocks the specified mutex.\0" as *const u8
                as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = longhelp_entry {
            options: b"-m, --mutex <mutex>\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            description: b"Name of the mutex.\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = longhelp_entry {
            options: b"-d, --dump <filename>\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            description: b"Dump the content of the given file to standard output, if ``-m|--mutex'' is given, this will be atomic.\0"
                as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = longhelp_entry {
            options: b"-s, --save <filename>\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            description: b"Save the content of standard input to the given file, if ``-m|--mutex'' is given, this will be atomic.\0"
                as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = longhelp_entry {
            options: b"-a, --atomic <command>\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            description: b"Execute the given command. If ``-m|--mutex'' is given, the exectution is going to be atomic with respect to that mutex.\0"
                as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = longhelp_entry {
            options: b"Hypervisor mode:\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            description: 0 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = longhelp_entry {
            options: b"-H, --hypervisor\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            description: b"Starts pexec in hypervisor mode.\0" as *const u8
                as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = longhelp_entry {
            options: b"-C, --control <port>\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            description: b"The control port used by the hypervisor.\0" as *const u8
                as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = longhelp_entry {
            options: b"-l, --load <window>\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            description: b"Use load also to limit the number of simultaneous processes with the specified load average interval (0, 1 or 2, or 1min, 5min or 15min, respectively).\0"
                as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = longhelp_entry {
            options: b"-f, --fifo\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            description: b"First in first out queue processing.\0" as *const u8
                as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = longhelp_entry {
            options: b"-s, --lifo\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            description: b"Last in first out (stack) queue processing (default).\0"
                as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = longhelp_entry {
            options: b"Logging:\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            description: 0 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = longhelp_entry {
            options: b"-L, --log <file>\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            description: b"The name of the log file.\0" as *const u8
                as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = longhelp_entry {
            options: b"-W, --log-level <level>\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            description: b"The logging level.\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = longhelp_entry {
            options: b"-V, --verbose\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            description: b"Increase the log level by one.\0" as *const u8
                as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = longhelp_entry {
            options: 0 as *const libc::c_char as *mut libc::c_char,
            description: 0 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
];
pub unsafe extern "C" fn fprint_pexec_long_help(mut fw: *mut FILE) -> libc::c_int {
    fprintf(
        fw,
        b"Usage:\tpexec [options] [-c|-m] [--] command [arguments] | 'compound command'\nExecute commands or shell scripts in parallel on a single host or\non remote hosts using a remote shell.\n\n\0"
            as *const u8 as *const libc::c_char,
    );
    longhelp_fprint(
        fw,
        pexec_long_help.as_mut_ptr(),
        0 as libc::c_int,
        -(1 as libc::c_int),
    );
    fprintf(fw, b"\n\0" as *const u8 as *const libc::c_char);
    fprintf(
        fw,
        b"Report bugs to <apal@szofi.elte.hu>\n\0" as *const u8 as *const libc::c_char,
    );
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn fprint_pexec_usage(mut fw: *mut FILE) -> libc::c_int {
    fprintf(
        fw,
        b"Usage:\tpexec\t[options] [--] command [arguments]\n\t\t[options] -c [--] 'compound command'\n\t\t[options] -m [--] 'compound command 1' 'compound command 2' ...\n\0"
            as *const u8 as *const libc::c_char,
    );
    fprintf(
        fw,
        b"General Options:\n\t[-h|--help|--long-help] [--version]\n\t[-c|--shell-command] [-m|--multiple-command] [-s|--shell <shell>]\n\t[-e|--environment <environmental_variable_name>]\n\0"
            as *const u8 as *const libc::c_char,
    );
    fprintf(
        fw,
        b"\t[-p|--list <list_of_parametes> [-p ...] [-p ...]]\n\t[-r|--parameters <list_of_parametes> {--|-<option} [-r ...] [-r ...]]\n\t[-f|--listfile <parameter_file> [-w|--column <column>|-t|--complete]]\n\t[-n|--number auto|<num>|managed|ncpu [-C|--control {<port>|</path>}]]\n\t[-l|--load|--use-load <load>] [-z|--nice <nice>]\n\0"
            as *const u8 as *const libc::c_char,
    );
    fprintf(
        fw,
        b"Redirecting input and output:\n\t[-i|--input <format_for_standard_input_file>]\n\t[-o|--output <format_for_standard_output_file>]\n\t[-u|--error <format_for_standard_error_file>]\n\t[-a|--output-format <format_for_stdout_redirection> [-x]]\n\t[-b|--error-format <format_for_stderr_redirection> [-x|--omit-newlines]]\n\t[-R|--normal-redirection]\n\0"
            as *const u8 as *const libc::c_char,
    );
    fprintf(
        fw,
        b"Parallelization using remote hosts:\n\t[-g|--remote-shell \"<remote_shell [options]>\"] [-q|--timeout <sec>]\n\t-n|--number [<host>:]{auto|<num>|managed|ncpu}[,...],[auto|<num>|...]\n\t[-P|--pexec <full_pexec_path_on_the_remote_host(s)>] [-k|--local-files]\n\0"
            as *const u8 as *const libc::c_char,
    );
    fprintf(
        fw,
        b"Running as a tunnel daemon (only for internal use, see also the manual):\n\t-T|--tunnel [-z|--nice <nice>]\n\t[-n|--number auto|<num>|managed|ncpu [-C|--control {<port>|</path>}]]\n\0"
            as *const u8 as *const libc::c_char,
    );
    fprintf(
        fw,
        b"Remote control, mutual exclusions and atomic command execution:\n\t[-y|--bind inet|unix|<port>|/<path>]\n\t-j|--remote [-p|--connect auto|/<path>|[host:]<port>] [-t|--status]\n\t[-E|--pexec-connection-variable <env_variable_name>]\n\t[{-l|--lock|--mutex-lock | -u|--unlock|--mutex-unlock} <name>]\n\t[-m|--mutex <name> {-d|--dump | -s|--save} <filename> ]\n\t[-m|--mutex <name> -a|--atomic [-c|--shell-command] [--] command [...]]\n\0"
            as *const u8 as *const libc::c_char,
    );
    fprintf(
        fw,
        b"Hypervisor mode and operations:\n\t-H|--hypervisor [-C|--control {<port>|</path>}] [start|stop]\n\t[-n|--number auto|<num>] [-l|--load|--use-load <load>]\n\t[-f|--fifo | -s|--lifo|--stack]\n\t(default hypervisor socket: %s)\n\0"
            as *const u8 as *const libc::c_char,
        b"/tmp/pexec.sock\0" as *const u8 as *const libc::c_char,
    );
    fprintf(
        fw,
        b"Logging (for normal and hypervisor modes):\n\t[-L|--log <log_file>] [-W|--log-level <log_level> | -V|--verbose [...]]\nCommand specifications:\n\t[--] { command [args] | 'compound' | 'compound 1' ['compound 2'...] }\n\0"
            as *const u8 as *const libc::c_char,
    );
    fprintf(
        fw,
        b"For more information, see --long-help or the full texinfo documentation.\nExamples can be found in the ``Examples'' section of the documentation.\n\0"
            as *const u8 as *const libc::c_char,
    );
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn fprint_version(mut fw: *mut FILE) -> libc::c_int {
    fprintf(
        fw,
        b"%s %s (%s)\n\0" as *const u8 as *const libc::c_char,
        progbasename,
        b"1.0rc8\0" as *const u8 as *const libc::c_char,
        b"2009.07.02\0" as *const u8 as *const libc::c_char,
    );
    fprintf(
        fw,
        b"Copyright (C) 2007, 2008-2009; Pal, Andras <apal@szofi.elte.hu>\n\n\0"
            as *const u8 as *const libc::c_char,
    );
    fprintf(
        fw,
        b"This is free software. You may redistribute copies of it under the terms of\nthe GNU General Public License <http://www.gnu.org/licenses/gpl.html>.\nThere is NO WARRANTY, to the extent permitted by law. \n\n\0"
            as *const u8 as *const libc::c_char,
    );
    fprintf(
        fw,
        b"This software was written by Andras Pal. The core part was written \nwhile working for the Hungarian-made Automated Telescope (HAT) project \nto make the data processing more easier and therefore find many-many \nextrasolar planets. See more information about this project: \nhttp://hatnet.hu. Another internal libraries (e.g. numhash.[ch]) were \nprimarily written for other projects.\n\0"
            as *const u8 as *const libc::c_char,
    );
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn fprint_err_invarg0(mut arg: *mut libc::c_char) -> libc::c_int {
    fprintf(
        stderr,
        dcgettext(
            0 as *const libc::c_char,
            b"%s: error: invalid command line argument '%s'.\n\0" as *const u8
                as *const libc::c_char,
            5 as libc::c_int,
        ),
        progbasename,
        arg,
    );
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn fprint_err_invarg1(mut arg: *mut libc::c_char) -> libc::c_int {
    fprintf(
        stderr,
        dcgettext(
            0 as *const libc::c_char,
            b"%s: error: invalid or missing argument near '%s'.\n\0" as *const u8
                as *const libc::c_char,
            5 as libc::c_int,
        ),
        progbasename,
        arg,
    );
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn fprint_err_invarg2(mut arg: *mut libc::c_char) -> libc::c_int {
    fprintf(
        stderr,
        dcgettext(
            0 as *const libc::c_char,
            b"%s: error: special command line argument '%s' must be the first in the list.\n\0"
                as *const u8 as *const libc::c_char,
            5 as libc::c_int,
        ),
        progbasename,
        arg,
    );
    return 0 as libc::c_int;
}
unsafe fn main_0(
    mut argc: libc::c_int,
    mut argv: *mut *mut libc::c_char,
) -> libc::c_int {
    let mut is_shell_commands: libc::c_int = 0;
    let mut is_multi_commands: libc::c_int = 0;
    let mut zeroarg: libc::c_int = 0;
    let mut rhosts: *mut remotehost = 0 as *mut remotehost;
    let mut nrhost: libc::c_int = 0;
    let mut list: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut listfile: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut logfile: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut listcolumn: libc::c_int = 0;
    let mut p: paralleldata = paralleldata {
        in_0: 0 as *mut libc::c_char,
        out: 0 as *mut libc::c_char,
        err: 0 as *mut libc::c_char,
        fwout: 0 as *mut FILE,
        formatout: 0 as *mut libc::c_char,
        fwerr: 0 as *mut FILE,
        formaterr: 0 as *mut libc::c_char,
        omit_newlines: 0,
        envvarname: 0 as *mut libc::c_char,
        shell: 0 as *mut libc::c_char,
        rsh: 0 as *mut libc::c_char,
        rshcmd: 0 as *mut libc::c_char,
        rshargs: 0 as *mut *mut libc::c_char,
        fallback_to_die: 0,
        log: 0 as *mut logdata,
    };
    let mut log: logdata = logdata {
        fwlog: 0 as *mut FILE,
        loglevel: 0,
    };
    let mut ncmd: libc::c_int = 0;
    let mut llen: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    let mut ret: libc::c_int = 0;
    let mut prio: libc::c_int = 0;
    let mut params: *mut parameter = 0 as *mut parameter;
    let mut nparam: libc::c_int = 0;
    let mut pnames: *mut *mut libc::c_char = 0 as *mut *mut libc::c_char;
    let mut pexec_self: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut ctrlport: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut ctrlenv: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut hypcport: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut timeout: libc::c_int = 0;
    let mut use_load: libc::c_int = 0;
    let mut mode: libc::c_int = 0;
    is_shell_commands = 0 as libc::c_int;
    is_multi_commands = 0 as libc::c_int;
    zeroarg = -(1 as libc::c_int);
    list = 0 as *mut libc::c_char;
    pnames = 0 as *mut *mut libc::c_char;
    logfile = 0 as *mut libc::c_char;
    listfile = logfile;
    llen = 0 as libc::c_int;
    rhosts = 0 as *mut remotehost;
    nrhost = -(1 as libc::c_int);
    p.envvarname = 0 as *mut libc::c_char;
    p.in_0 = 0 as *mut libc::c_char;
    p.err = p.in_0;
    p.out = p.err;
    p.shell = b"/bin/sh\0" as *const u8 as *const libc::c_char as *mut libc::c_char;
    p.rshcmd = 0 as *mut libc::c_char;
    prio = 0 as libc::c_int;
    p.formatout = 0 as *mut libc::c_char;
    p.formaterr = 0 as *mut libc::c_char;
    p.omit_newlines = 0 as libc::c_int;
    log.loglevel = -(1 as libc::c_int);
    log.fwlog = 0 as *mut FILE;
    listcolumn = 0 as libc::c_int;
    progbasename = strrchr(*argv.offset(0 as libc::c_int as isize), '/' as i32);
    if !progbasename.is_null() {
        progbasename = progbasename.offset(1);
        progbasename;
    } else {
        progbasename = *argv.offset(0 as libc::c_int as isize);
    }
    mode = 0 as libc::c_int;
    if !(strchr(*argv.offset(0 as libc::c_int as isize), '/' as i32)).is_null() {
        pexec_self = *argv.offset(0 as libc::c_int as isize);
    } else {
        pexec_self = progbasename;
    }
    timeout = 60 as libc::c_int;
    ctrlport = 0 as *mut libc::c_char;
    ctrlenv = b"PEXEC_REMOTE_PORT\0" as *const u8 as *const libc::c_char
        as *mut libc::c_char;
    hypcport = 0 as *mut libc::c_char;
    use_load = -(1 as libc::c_int);
    i = 1 as libc::c_int;
    while i < argc {
        if strcmp(*argv.offset(i as isize), b"-h\0" as *const u8 as *const libc::c_char)
            == 0 as libc::c_int
            || strcmp(
                *argv.offset(i as isize),
                b"--short-help\0" as *const u8 as *const libc::c_char,
            ) == 0 as libc::c_int
            || strcmp(
                *argv.offset(i as isize),
                b"--help\0" as *const u8 as *const libc::c_char,
            ) == 0 as libc::c_int
            || strcmp(
                *argv.offset(i as isize),
                b"--help-short\0" as *const u8 as *const libc::c_char,
            ) == 0 as libc::c_int
        {
            fprint_pexec_usage(stdout);
            return 0 as libc::c_int;
        }
        if strcmp(
            *argv.offset(i as isize),
            b"--long-help\0" as *const u8 as *const libc::c_char,
        ) == 0 as libc::c_int
            || strcmp(
                *argv.offset(i as isize),
                b"--help-long\0" as *const u8 as *const libc::c_char,
            ) == 0 as libc::c_int
        {
            fprint_pexec_long_help(stdout);
            return 0 as libc::c_int;
        } else if strcmp(
            *argv.offset(i as isize),
            b"--version\0" as *const u8 as *const libc::c_char,
        ) == 0 as libc::c_int
        {
            fprint_version(stdout);
            return 0 as libc::c_int;
        } else {
            if strcmp(
                *argv.offset(i as isize),
                b"-c\0" as *const u8 as *const libc::c_char,
            ) == 0 as libc::c_int
                || strcmp(
                    *argv.offset(i as isize),
                    b"--shell-command\0" as *const u8 as *const libc::c_char,
                ) == 0 as libc::c_int
            {
                is_shell_commands = 1 as libc::c_int;
            } else if strcmp(
                *argv.offset(i as isize),
                b"-m\0" as *const u8 as *const libc::c_char,
            ) == 0 as libc::c_int
                || strcmp(
                    *argv.offset(i as isize),
                    b"--multiple-command\0" as *const u8 as *const libc::c_char,
                ) == 0 as libc::c_int
            {
                is_multi_commands = 1 as libc::c_int;
            } else if strcmp(
                *argv.offset(i as isize),
                b"-e\0" as *const u8 as *const libc::c_char,
            ) == 0 as libc::c_int
                || strcmp(
                    *argv.offset(i as isize),
                    b"--environment\0" as *const u8 as *const libc::c_char,
                ) == 0 as libc::c_int
                || strcmp(
                    *argv.offset(i as isize),
                    b"--setenv\0" as *const u8 as *const libc::c_char,
                ) == 0 as libc::c_int
            {
                i += 1;
                if i >= argc {
                    fprint_err_invarg1(*argv.offset((i - 1 as libc::c_int) as isize));
                    return 1 as libc::c_int;
                }
                p.envvarname = *argv.offset(i as isize);
            } else if strcmp(
                *argv.offset(i as isize),
                b"-s\0" as *const u8 as *const libc::c_char,
            ) == 0 as libc::c_int
                || strcmp(
                    *argv.offset(i as isize),
                    b"--shell\0" as *const u8 as *const libc::c_char,
                ) == 0 as libc::c_int
            {
                i += 1;
                if i >= argc {
                    fprint_err_invarg1(*argv.offset((i - 1 as libc::c_int) as isize));
                    return 1 as libc::c_int;
                }
                p.shell = *argv.offset(i as isize);
            } else if strcmp(
                *argv.offset(i as isize),
                b"-g\0" as *const u8 as *const libc::c_char,
            ) == 0 as libc::c_int
                || strcmp(
                    *argv.offset(i as isize),
                    b"--remote-shell\0" as *const u8 as *const libc::c_char,
                ) == 0 as libc::c_int
            {
                let fresh90 = i;
                i = i + 1;
                if fresh90 >= argc {
                    fprint_err_invarg1(*argv.offset((i - 1 as libc::c_int) as isize));
                    return 1 as libc::c_int;
                }
                p.rshcmd = xstrdup(*argv.offset(i as isize));
            } else if strcmp(
                *argv.offset(i as isize),
                b"-p\0" as *const u8 as *const libc::c_char,
            ) == 0 as libc::c_int
                || strcmp(
                    *argv.offset(i as isize),
                    b"--list\0" as *const u8 as *const libc::c_char,
                ) == 0 as libc::c_int
            {
                i += 1;
                if i >= argc {
                    fprint_err_invarg1(*argv.offset((i - 1 as libc::c_int) as isize));
                    return 1 as libc::c_int;
                } else if list.is_null() {
                    list = xstrdup(*argv.offset(i as isize));
                    llen = strlen(list) as libc::c_int;
                } else {
                    let mut ilen: libc::c_int = 0;
                    ilen = strlen(*argv.offset(i as isize)) as libc::c_int;
                    list = xrealloc(
                        list as *mut libc::c_void,
                        (llen + ilen + 2 as libc::c_int) as size_t,
                    ) as *mut libc::c_char;
                    strcpy(
                        list.offset(llen as isize),
                        b" \0" as *const u8 as *const libc::c_char,
                    );
                    llen += 1;
                    llen;
                    strcpy(list.offset(llen as isize), *argv.offset(i as isize));
                    llen += ilen;
                }
            } else if strcmp(
                *argv.offset(i as isize),
                b"-r\0" as *const u8 as *const libc::c_char,
            ) == 0 as libc::c_int
                || strcmp(
                    *argv.offset(i as isize),
                    b"--parameters\0" as *const u8 as *const libc::c_char,
                ) == 0 as libc::c_int
            {
                i += 1;
                i;
                while i < argc
                    && *(*argv.offset(i as isize)).offset(0 as libc::c_int as isize)
                        as libc::c_int != '-' as i32
                {
                    if list.is_null() {
                        list = xstrdup(*argv.offset(i as isize));
                        llen = strlen(list) as libc::c_int;
                    } else {
                        let mut ilen_0: libc::c_int = 0;
                        ilen_0 = strlen(*argv.offset(i as isize)) as libc::c_int;
                        list = xrealloc(
                            list as *mut libc::c_void,
                            (llen + ilen_0 + 2 as libc::c_int) as size_t,
                        ) as *mut libc::c_char;
                        strcpy(
                            list.offset(llen as isize),
                            b" \0" as *const u8 as *const libc::c_char,
                        );
                        llen += 1;
                        llen;
                        strcpy(list.offset(llen as isize), *argv.offset(i as isize));
                        llen += ilen_0;
                    }
                    i += 1;
                    i;
                }
                i -= 1;
                i;
            } else if strcmp(
                *argv.offset(i as isize),
                b"-f\0" as *const u8 as *const libc::c_char,
            ) == 0 as libc::c_int
                || strcmp(
                    *argv.offset(i as isize),
                    b"--listfile\0" as *const u8 as *const libc::c_char,
                ) == 0 as libc::c_int
            {
                i += 1;
                if i >= argc {
                    fprint_err_invarg1(*argv.offset((i - 1 as libc::c_int) as isize));
                    return 1 as libc::c_int;
                }
                listfile = *argv.offset(i as isize);
            } else if strcmp(
                *argv.offset(i as isize),
                b"-w\0" as *const u8 as *const libc::c_char,
            ) == 0 as libc::c_int
                || strcmp(
                    *argv.offset(i as isize),
                    b"--column\0" as *const u8 as *const libc::c_char,
                ) == 0 as libc::c_int
            {
                i += 1;
                if i >= argc {
                    fprint_err_invarg1(*argv.offset((i - 1 as libc::c_int) as isize));
                    return 1 as libc::c_int;
                } else if sscanf(
                    *argv.offset(i as isize),
                    b"%d\0" as *const u8 as *const libc::c_char,
                    &mut listcolumn as *mut libc::c_int,
                ) < 1 as libc::c_int || listcolumn <= 0 as libc::c_int
                {
                    fprint_err_invarg1(*argv.offset((i - 1 as libc::c_int) as isize));
                    return 1 as libc::c_int;
                }
                listcolumn -= 1;
                listcolumn;
            } else if strcmp(
                *argv.offset(i as isize),
                b"-t\0" as *const u8 as *const libc::c_char,
            ) == 0 as libc::c_int
                || strcmp(
                    *argv.offset(i as isize),
                    b"--complete\0" as *const u8 as *const libc::c_char,
                ) == 0 as libc::c_int
                || strcmp(
                    *argv.offset(i as isize),
                    b"--complete-line\0" as *const u8 as *const libc::c_char,
                ) == 0 as libc::c_int
            {
                listcolumn = -(1 as libc::c_int);
            } else if strcmp(
                *argv.offset(i as isize),
                b"-n\0" as *const u8 as *const libc::c_char,
            ) == 0 as libc::c_int
                || strcmp(
                    *argv.offset(i as isize),
                    b"--number\0" as *const u8 as *const libc::c_char,
                ) == 0 as libc::c_int
            {
                i += 1;
                if i >= argc {
                    fprint_err_invarg1(*argv.offset((i - 1 as libc::c_int) as isize));
                    return 1 as libc::c_int;
                } else if strcmp(
                    *argv.offset(i as isize),
                    b"auto\0" as *const u8 as *const libc::c_char,
                ) == 0 as libc::c_int
                {
                    rhosts = 0 as *mut remotehost;
                    nrhost = -(1 as libc::c_int);
                } else if strcmp(
                    *argv.offset(i as isize),
                    b"managed\0" as *const u8 as *const libc::c_char,
                ) == 0 as libc::c_int
                {
                    rhosts = 0 as *mut remotehost;
                    nrhost = -(2 as libc::c_int);
                } else if strcmp(
                    *argv.offset(i as isize),
                    b"ncpu\0" as *const u8 as *const libc::c_char,
                ) == 0 as libc::c_int
                {
                    rhosts = 0 as *mut remotehost;
                    nrhost = -(3 as libc::c_int);
                } else if parse_host_data(
                    *argv.offset(i as isize),
                    &mut rhosts,
                    &mut nrhost,
                ) != 0
                {
                    fprint_err_invarg1(*argv.offset((i - 1 as libc::c_int) as isize));
                    return 1 as libc::c_int;
                }
            } else if strcmp(
                *argv.offset(i as isize),
                b"-M\0" as *const u8 as *const libc::c_char,
            ) == 0 as libc::c_int
                || strcmp(
                    *argv.offset(i as isize),
                    b"--managed\0" as *const u8 as *const libc::c_char,
                ) == 0 as libc::c_int
            {
                rhosts = 0 as *mut remotehost;
                nrhost = -(2 as libc::c_int);
            } else if strcmp(
                *argv.offset(i as isize),
                b"-l\0" as *const u8 as *const libc::c_char,
            ) == 0 as libc::c_int
                || strcmp(
                    *argv.offset(i as isize),
                    b"--load\0" as *const u8 as *const libc::c_char,
                ) == 0 as libc::c_int
                || strcmp(
                    *argv.offset(i as isize),
                    b"--use-load\0" as *const u8 as *const libc::c_char,
                ) == 0 as libc::c_int
            {
                i += 1;
                if i >= argc {
                    fprint_err_invarg1(*argv.offset((i - 1 as libc::c_int) as isize));
                    return 1 as libc::c_int;
                } else if strcmp(
                    *argv.offset(i as isize),
                    b"0\0" as *const u8 as *const libc::c_char,
                ) == 0 as libc::c_int
                    || strcmp(
                        *argv.offset(i as isize),
                        b"1m\0" as *const u8 as *const libc::c_char,
                    ) == 0 as libc::c_int
                    || strcmp(
                        *argv.offset(i as isize),
                        b"1min\0" as *const u8 as *const libc::c_char,
                    ) == 0 as libc::c_int
                {
                    use_load = 0 as libc::c_int;
                } else if strcmp(
                    *argv.offset(i as isize),
                    b"1\0" as *const u8 as *const libc::c_char,
                ) == 0 as libc::c_int
                    || strcmp(
                        *argv.offset(i as isize),
                        b"5m\0" as *const u8 as *const libc::c_char,
                    ) == 0 as libc::c_int
                    || strcmp(
                        *argv.offset(i as isize),
                        b"5min\0" as *const u8 as *const libc::c_char,
                    ) == 0 as libc::c_int
                {
                    use_load = 1 as libc::c_int;
                } else if strcmp(
                    *argv.offset(i as isize),
                    b"2\0" as *const u8 as *const libc::c_char,
                ) == 0 as libc::c_int
                    || strcmp(
                        *argv.offset(i as isize),
                        b"15m\0" as *const u8 as *const libc::c_char,
                    ) == 0 as libc::c_int
                    || strcmp(
                        *argv.offset(i as isize),
                        b"15min\0" as *const u8 as *const libc::c_char,
                    ) == 0 as libc::c_int
                {
                    use_load = 2 as libc::c_int;
                } else {
                    fprint_err_invarg1(*argv.offset((i - 1 as libc::c_int) as isize));
                    return 1 as libc::c_int;
                }
            } else if strcmp(
                *argv.offset(i as isize),
                b"-z\0" as *const u8 as *const libc::c_char,
            ) == 0 as libc::c_int
                || strcmp(
                    *argv.offset(i as isize),
                    b"--nice\0" as *const u8 as *const libc::c_char,
                ) == 0 as libc::c_int
            {
                i += 1;
                if i >= argc {
                    fprint_err_invarg1(*argv.offset((i - 1 as libc::c_int) as isize));
                    return 1 as libc::c_int;
                } else if sscanf(
                    *argv.offset(i as isize),
                    b"%d\0" as *const u8 as *const libc::c_char,
                    &mut prio as *mut libc::c_int,
                ) < 1 as libc::c_int
                {
                    fprint_err_invarg1(*argv.offset((i - 1 as libc::c_int) as isize));
                    return 1 as libc::c_int;
                }
                prio += 128 as libc::c_int;
            } else if strcmp(
                *argv.offset(i as isize),
                b"-q\0" as *const u8 as *const libc::c_char,
            ) == 0 as libc::c_int
                || strcmp(
                    *argv.offset(i as isize),
                    b"--timeout\0" as *const u8 as *const libc::c_char,
                ) == 0 as libc::c_int
            {
                i += 1;
                if i >= argc {
                    fprint_err_invarg1(*argv.offset((i - 1 as libc::c_int) as isize));
                    return 1 as libc::c_int;
                } else if sscanf(
                    *argv.offset(i as isize),
                    b"%d\0" as *const u8 as *const libc::c_char,
                    &mut timeout as *mut libc::c_int,
                ) < 1 as libc::c_int
                {
                    fprint_err_invarg1(*argv.offset((i - 1 as libc::c_int) as isize));
                    return 1 as libc::c_int;
                }
                if timeout < 0 as libc::c_int {
                    timeout = 0 as libc::c_int;
                }
            } else if strcmp(
                *argv.offset(i as isize),
                b"-P\0" as *const u8 as *const libc::c_char,
            ) == 0 as libc::c_int
                || strcmp(
                    *argv.offset(i as isize),
                    b"--pexec\0" as *const u8 as *const libc::c_char,
                ) == 0 as libc::c_int
            {
                i += 1;
                if i >= argc {
                    fprint_err_invarg1(*argv.offset((i - 1 as libc::c_int) as isize));
                    return 1 as libc::c_int;
                }
                pexec_self = *argv.offset(i as isize);
            } else if strcmp(
                *argv.offset(i as isize),
                b"-E\0" as *const u8 as *const libc::c_char,
            ) == 0 as libc::c_int
                || strcmp(
                    *argv.offset(i as isize),
                    b"--pexec-connection-variable\0" as *const u8 as *const libc::c_char,
                ) == 0 as libc::c_int
            {
                i += 1;
                if i >= argc {
                    fprint_err_invarg1(*argv.offset((i - 1 as libc::c_int) as isize));
                    return 1 as libc::c_int;
                }
                ctrlenv = *argv.offset(i as isize);
            } else if strcmp(
                *argv.offset(i as isize),
                b"-i\0" as *const u8 as *const libc::c_char,
            ) == 0 as libc::c_int
                || strcmp(
                    *argv.offset(i as isize),
                    b"--input\0" as *const u8 as *const libc::c_char,
                ) == 0 as libc::c_int
            {
                i += 1;
                if i >= argc {
                    fprint_err_invarg1(*argv.offset((i - 1 as libc::c_int) as isize));
                    return 1 as libc::c_int;
                }
                p.in_0 = *argv.offset(i as isize);
            } else if strcmp(
                *argv.offset(i as isize),
                b"-o\0" as *const u8 as *const libc::c_char,
            ) == 0 as libc::c_int
                || strcmp(
                    *argv.offset(i as isize),
                    b"--output\0" as *const u8 as *const libc::c_char,
                ) == 0 as libc::c_int
            {
                i += 1;
                if i >= argc {
                    fprint_err_invarg1(*argv.offset((i - 1 as libc::c_int) as isize));
                    return 1 as libc::c_int;
                }
                p.out = *argv.offset(i as isize);
            } else if strcmp(
                *argv.offset(i as isize),
                b"-u\0" as *const u8 as *const libc::c_char,
            ) == 0 as libc::c_int
                || strcmp(
                    *argv.offset(i as isize),
                    b"--error\0" as *const u8 as *const libc::c_char,
                ) == 0 as libc::c_int
                || strcmp(
                    *argv.offset(i as isize),
                    b"--output-error\0" as *const u8 as *const libc::c_char,
                ) == 0 as libc::c_int
            {
                i += 1;
                if i >= argc {
                    fprint_err_invarg1(*argv.offset((i - 1 as libc::c_int) as isize));
                    return 1 as libc::c_int;
                }
                p.err = *argv.offset(i as isize);
            } else if strcmp(
                *argv.offset(i as isize),
                b"-R\0" as *const u8 as *const libc::c_char,
            ) == 0 as libc::c_int
                || strcmp(
                    *argv.offset(i as isize),
                    b"--normal-redirection\0" as *const u8 as *const libc::c_char,
                ) == 0 as libc::c_int
            {
                p.in_0 = 0 as *mut libc::c_char;
                p.out = b"-\0" as *const u8 as *const libc::c_char as *mut libc::c_char;
                p.err = b"-\0" as *const u8 as *const libc::c_char as *mut libc::c_char;
            } else if strcmp(
                *argv.offset(i as isize),
                b"-a\0" as *const u8 as *const libc::c_char,
            ) == 0 as libc::c_int
                || strcmp(
                    *argv.offset(i as isize),
                    b"--output-format\0" as *const u8 as *const libc::c_char,
                ) == 0 as libc::c_int
            {
                i += 1;
                if i >= argc {
                    fprint_err_invarg1(*argv.offset((i - 1 as libc::c_int) as isize));
                    return 1 as libc::c_int;
                }
                p.formatout = *argv.offset(i as isize);
            } else if strcmp(
                *argv.offset(i as isize),
                b"-b\0" as *const u8 as *const libc::c_char,
            ) == 0 as libc::c_int
                || strcmp(
                    *argv.offset(i as isize),
                    b"--error-format\0" as *const u8 as *const libc::c_char,
                ) == 0 as libc::c_int
            {
                i += 1;
                if i >= argc {
                    fprint_err_invarg1(*argv.offset((i - 1 as libc::c_int) as isize));
                    return 1 as libc::c_int;
                }
                p.formaterr = *argv.offset(i as isize);
            } else if strcmp(
                *argv.offset(i as isize),
                b"-x\0" as *const u8 as *const libc::c_char,
            ) == 0 as libc::c_int
                || strcmp(
                    *argv.offset(i as isize),
                    b"--omit-newlines\0" as *const u8 as *const libc::c_char,
                ) == 0 as libc::c_int
            {
                p.omit_newlines = 1 as libc::c_int;
            } else if strcmp(
                *argv.offset(i as isize),
                b"-y\0" as *const u8 as *const libc::c_char,
            ) == 0 as libc::c_int
                || strcmp(
                    *argv.offset(i as isize),
                    b"--bind\0" as *const u8 as *const libc::c_char,
                ) == 0 as libc::c_int
            {
                i += 1;
                if i >= argc {
                    fprint_err_invarg1(*argv.offset((i - 1 as libc::c_int) as isize));
                    return 1 as libc::c_int;
                }
                ctrlport = *argv.offset(i as isize);
            } else if strcmp(
                *argv.offset(i as isize),
                b"-C\0" as *const u8 as *const libc::c_char,
            ) == 0 as libc::c_int
                || strcmp(
                    *argv.offset(i as isize),
                    b"--control\0" as *const u8 as *const libc::c_char,
                ) == 0 as libc::c_int
            {
                i += 1;
                if i >= argc {
                    fprint_err_invarg1(*argv.offset((i - 1 as libc::c_int) as isize));
                    return 1 as libc::c_int;
                }
                hypcport = *argv.offset(i as isize);
            } else if strcmp(
                *argv.offset(i as isize),
                b"-L\0" as *const u8 as *const libc::c_char,
            ) == 0 as libc::c_int
                || strcmp(
                    *argv.offset(i as isize),
                    b"--log\0" as *const u8 as *const libc::c_char,
                ) == 0 as libc::c_int
            {
                i += 1;
                if i >= argc {
                    fprint_err_invarg1(*argv.offset((i - 1 as libc::c_int) as isize));
                    return 1 as libc::c_int;
                }
                logfile = *argv.offset(i as isize);
            } else if strcmp(
                *argv.offset(i as isize),
                b"-V\0" as *const u8 as *const libc::c_char,
            ) == 0 as libc::c_int
                || strcmp(
                    *argv.offset(i as isize),
                    b"--verbose\0" as *const u8 as *const libc::c_char,
                ) == 0 as libc::c_int
            {
                log.loglevel += 1;
                log.loglevel;
            } else if strcmp(
                *argv.offset(i as isize),
                b"-W\0" as *const u8 as *const libc::c_char,
            ) == 0 as libc::c_int
                || strcmp(
                    *argv.offset(i as isize),
                    b"--log-level\0" as *const u8 as *const libc::c_char,
                ) == 0 as libc::c_int
            {
                i += 1;
                if i >= argc {
                    fprint_err_invarg1(*argv.offset((i - 1 as libc::c_int) as isize));
                    return 1 as libc::c_int;
                } else if sscanf(
                    *argv.offset(i as isize),
                    b"%d\0" as *const u8 as *const libc::c_char,
                    &mut log.loglevel as *mut libc::c_int,
                ) < 1 as libc::c_int || log.loglevel < 0 as libc::c_int
                {
                    fprint_err_invarg1(*argv.offset((i - 1 as libc::c_int) as isize));
                    return 1 as libc::c_int;
                }
            } else if strcmp(
                *argv.offset(i as isize),
                b"-T\0" as *const u8 as *const libc::c_char,
            ) == 0 as libc::c_int
                || strcmp(
                    *argv.offset(i as isize),
                    b"--tunnel\0" as *const u8 as *const libc::c_char,
                ) == 0 as libc::c_int
            {
                if i > 1 as libc::c_int {
                    fprint_err_invarg2(*argv.offset(i as isize));
                    return 1 as libc::c_int;
                }
                zeroarg = i + 1 as libc::c_int;
                mode = 1 as libc::c_int;
                break;
            } else if strcmp(
                *argv.offset(i as isize),
                b"-j\0" as *const u8 as *const libc::c_char,
            ) == 0 as libc::c_int
                || strcmp(
                    *argv.offset(i as isize),
                    b"--remote\0" as *const u8 as *const libc::c_char,
                ) == 0 as libc::c_int
            {
                if i > 1 as libc::c_int {
                    fprint_err_invarg2(*argv.offset(i as isize));
                    return 1 as libc::c_int;
                }
                zeroarg = i + 1 as libc::c_int;
                mode = 2 as libc::c_int;
                break;
            } else if strcmp(
                *argv.offset(i as isize),
                b"-H\0" as *const u8 as *const libc::c_char,
            ) == 0 as libc::c_int
                || strcmp(
                    *argv.offset(i as isize),
                    b"--hypervisor\0" as *const u8 as *const libc::c_char,
                ) == 0 as libc::c_int
            {
                if i > 1 as libc::c_int {
                    fprint_err_invarg2(*argv.offset(i as isize));
                    return 1 as libc::c_int;
                }
                zeroarg = i + 1 as libc::c_int;
                mode = 3 as libc::c_int;
                break;
            } else if strcmp(
                *argv.offset(i as isize),
                b"--\0" as *const u8 as *const libc::c_char,
            ) == 0 as libc::c_int
            {
                zeroarg = i + 1 as libc::c_int;
                if zeroarg >= argc {
                    zeroarg = -(1 as libc::c_int);
                }
                break;
            } else if *(*argv.offset(i as isize)).offset(0 as libc::c_int as isize)
                as libc::c_int == '-' as i32
            {
                fprint_err_invarg0(*argv.offset(i as isize));
                return 1 as libc::c_int;
            } else {
                zeroarg = i;
                break;
            }
            i += 1;
            i;
        }
    }
    if zeroarg < 0 as libc::c_int {
        return 0 as libc::c_int;
    }
    if mode == 0 as libc::c_int {
        let mut rshells: *mut remoteshell = 0 as *mut remoteshell;
        let mut rs: *mut remoteshell = 0 as *mut remoteshell;
        let mut nrshell: libc::c_int = 0;
        let mut r: libc::c_int = 0;
        let mut status: libc::c_int = 0;
        let mut sock: libc::c_int = 0;
        let mut hsck: libc::c_int = 0;
        let mut pctrlport: *mut libc::c_char = 0 as *mut libc::c_char;
        let mut is_rhosts_defined: libc::c_int = 0;
        let mut no_touch_std: libc::c_int = 0;
        if rhosts.is_null() {
            rhosts = xmalloc(
                (::std::mem::size_of::<remotehost>() as libc::c_ulong)
                    .wrapping_mul(1 as libc::c_int as libc::c_ulong),
            ) as *mut remotehost;
            let ref mut fresh91 = (*rhosts.offset(0 as libc::c_int as isize)).hostspec;
            *fresh91 = 0 as *mut libc::c_char;
            (*rhosts.offset(0 as libc::c_int as isize)).num_processes = nrhost;
            nrhost = 1 as libc::c_int;
            is_rhosts_defined = 0 as libc::c_int;
        } else {
            is_rhosts_defined = (0 as libc::c_int == 0) as libc::c_int;
        }
        if (p.rshcmd).is_null() {
            p.rshcmd = xstrdup(b"/usr/bin/ssh\0" as *const u8 as *const libc::c_char);
        }
        p.rshargs = tokenize_spaces_dyn(p.rshcmd);
        p.rsh = *(p.rshargs).offset(0 as libc::c_int as isize);
        if !list.is_null() && !listfile.is_null() {
            fprintf(
                stderr,
                dcgettext(
                    0 as *const libc::c_char,
                    b"%s: error: both parameter list and list file are defined.\n\0"
                        as *const u8 as *const libc::c_char,
                    5 as libc::c_int,
                ),
                progbasename,
            );
            return 1 as libc::c_int;
        } else if !list.is_null() {
            pnames = tokenize_spaces_dyn(list);
            nparam = 0 as libc::c_int;
            while !pnames.is_null() && !(*pnames.offset(nparam as isize)).is_null() {
                nparam += 1;
                nparam;
            }
            params = xmalloc(
                (::std::mem::size_of::<parameter>() as libc::c_ulong)
                    .wrapping_mul(nparam as libc::c_ulong),
            ) as *mut parameter;
            i = 0 as libc::c_int;
            while i < nparam {
                let ref mut fresh92 = (*params.offset(i as isize)).name;
                *fresh92 = *pnames.offset(i as isize);
                i += 1;
                i;
            }
        } else if !listfile.is_null() {
            let mut fr: *mut FILE = 0 as *mut FILE;
            let mut line: *mut libc::c_char = 0 as *mut libc::c_char;
            let mut tokens: *mut *mut libc::c_char = 0 as *mut *mut libc::c_char;
            let mut ntoken: libc::c_int = 0;
            if strcmp(listfile, b"-\0" as *const u8 as *const libc::c_char)
                == 0 as libc::c_int
            {
                fr = stdin;
            } else {
                fr = fopen(listfile, b"rb\0" as *const u8 as *const libc::c_char);
            }
            if fr.is_null() {
                fprintf(
                    stderr,
                    dcgettext(
                        0 as *const libc::c_char,
                        b"%s: error: unable to open list file.\n\0" as *const u8
                            as *const libc::c_char,
                        5 as libc::c_int,
                    ),
                    progbasename,
                );
                return 1 as libc::c_int;
            }
            params = 0 as *mut parameter;
            nparam = 0 as libc::c_int;
            while feof(fr) == 0 {
                line = freadline(fr);
                if line.is_null() {
                    break;
                }
                remove_newlines_and_comments(line);
                if !(strlen(line) > 0 as libc::c_int as libc::c_ulong) {
                    free(line as *mut libc::c_void);
                } else if listcolumn < 0 as libc::c_int {
                    params = xrealloc(
                        params as *mut libc::c_void,
                        (::std::mem::size_of::<parameter>() as libc::c_ulong)
                            .wrapping_mul((nparam + 1 as libc::c_int) as libc::c_ulong),
                    ) as *mut parameter;
                    let ref mut fresh93 = (*params.offset(nparam as isize)).name;
                    *fresh93 = xstrdup(line);
                    free(line as *mut libc::c_void);
                    nparam += 1;
                    nparam;
                } else {
                    tokens = tokenize_spaces_dyn(line);
                    if tokens.is_null() {
                        free(line as *mut libc::c_void);
                    } else {
                        ntoken = 0 as libc::c_int;
                        while !(*tokens.offset(ntoken as isize)).is_null() {
                            ntoken += 1;
                            ntoken;
                        }
                        if listcolumn >= ntoken {
                            free(tokens as *mut libc::c_void);
                            free(line as *mut libc::c_void);
                        } else {
                            params = xrealloc(
                                params as *mut libc::c_void,
                                (::std::mem::size_of::<parameter>() as libc::c_ulong)
                                    .wrapping_mul((nparam + 1 as libc::c_int) as libc::c_ulong),
                            ) as *mut parameter;
                            let ref mut fresh94 = (*params.offset(nparam as isize)).name;
                            *fresh94 = xstrdup(*tokens.offset(listcolumn as isize));
                            free(tokens as *mut libc::c_void);
                            free(line as *mut libc::c_void);
                            nparam += 1;
                            nparam;
                        }
                    }
                }
            }
            if fileno(fr) != fileno(stdin) {
                fclose(fr);
            }
        } else {
            params = 0 as *mut parameter;
            nparam = 0 as libc::c_int;
        }
        i = 0 as libc::c_int;
        while i < nparam {
            (*params.offset(i as isize)).c.is_shell = 0 as libc::c_int;
            let ref mut fresh95 = (*params.offset(i as isize)).c.argv;
            *fresh95 = 0 as *mut *mut libc::c_char;
            (*params.offset(i as isize)).c.argc = 0 as libc::c_int;
            i += 1;
            i;
        }
        if is_multi_commands != 0 {
            ncmd = argc - zeroarg;
        } else {
            ncmd = 1 as libc::c_int;
        }
        no_touch_std = 0 as libc::c_int;
        if nparam > 0 as libc::c_int && is_multi_commands != 0 && nparam != ncmd {
            fprintf(
                stderr,
                dcgettext(
                    0 as *const libc::c_char,
                    b"%s: error: number of parameters and commands mismatch.\n\0"
                        as *const u8 as *const libc::c_char,
                    5 as libc::c_int,
                ),
                progbasename,
            );
            return 1 as libc::c_int;
        } else if is_multi_commands != 0 && nparam > 0 as libc::c_int {
            i = 0 as libc::c_int;
            while i < nparam {
                (*params.offset(i as isize)).c.is_shell = 1 as libc::c_int;
                (*params.offset(i as isize)).c.argc = 1 as libc::c_int;
                let ref mut fresh96 = (*params.offset(i as isize)).c.argv;
                *fresh96 = argv.offset(zeroarg as isize).offset(i as isize);
                i += 1;
                i;
            }
        } else if is_multi_commands != 0 {
            nparam = ncmd;
            params = xmalloc(
                (::std::mem::size_of::<parameter>() as libc::c_ulong)
                    .wrapping_mul(nparam as libc::c_ulong),
            ) as *mut parameter;
            i = 0 as libc::c_int;
            while i < ncmd {
                let ref mut fresh97 = (*params.offset(i as isize)).name;
                *fresh97 = 0 as *mut libc::c_char;
                (*params.offset(i as isize)).c.is_shell = 1 as libc::c_int;
                (*params.offset(i as isize)).c.argc = 1 as libc::c_int;
                let ref mut fresh98 = (*params.offset(i as isize)).c.argv;
                *fresh98 = argv.offset(zeroarg as isize).offset(i as isize);
                i += 1;
                i;
            }
        } else if nparam > 0 as libc::c_int && is_shell_commands != 0 {
            i = 0 as libc::c_int;
            while i < nparam {
                (*params.offset(i as isize)).c.is_shell = 1 as libc::c_int;
                (*params.offset(i as isize)).c.argc = argc - zeroarg;
                let ref mut fresh99 = (*params.offset(i as isize)).c.argv;
                *fresh99 = argv.offset(zeroarg as isize);
                i += 1;
                i;
            }
        } else if nparam > 0 as libc::c_int {
            i = 0 as libc::c_int;
            while i < nparam {
                (*params.offset(i as isize)).c.is_shell = 0 as libc::c_int;
                (*params.offset(i as isize)).c.argc = argc - zeroarg;
                let ref mut fresh100 = (*params.offset(i as isize)).c.argv;
                *fresh100 = argv.offset(zeroarg as isize);
                i += 1;
                i;
            }
        } else if is_rhosts_defined == 0 {
            nparam = 1 as libc::c_int;
            params = xmalloc(
                (::std::mem::size_of::<parameter>() as libc::c_ulong)
                    .wrapping_mul(nparam as libc::c_ulong),
            ) as *mut parameter;
            let ref mut fresh101 = (*params.offset(0 as libc::c_int as isize)).name;
            *fresh101 = 0 as *mut libc::c_char;
            (*params.offset(0 as libc::c_int as isize)).c.is_shell = is_shell_commands;
            let ref mut fresh102 = (*params.offset(0 as libc::c_int as isize)).c.argv;
            *fresh102 = argv.offset(zeroarg as isize);
            (*params.offset(0 as libc::c_int as isize)).c.argc = argc - zeroarg;
            no_touch_std = (0 as libc::c_int == 0) as libc::c_int;
        } else {
            nparam = nrhost;
            params = xmalloc(
                (::std::mem::size_of::<parameter>() as libc::c_ulong)
                    .wrapping_mul(nparam as libc::c_ulong),
            ) as *mut parameter;
            i = 0 as libc::c_int;
            while i < nparam {
                let ref mut fresh103 = (*params.offset(i as isize)).name;
                *fresh103 = (*rhosts.offset(i as isize)).hostspec;
                (*params.offset(i as isize)).c.is_shell = -(1 as libc::c_int);
                let ref mut fresh104 = (*params.offset(i as isize)).c.argv;
                *fresh104 = argv.offset(zeroarg as isize);
                (*params.offset(i as isize)).c.argc = argc - zeroarg;
                i += 1;
                i;
            }
            let ref mut fresh105 = (*rhosts.offset(0 as libc::c_int as isize)).hostspec;
            *fresh105 = 0 as *mut libc::c_char;
            (*rhosts.offset(0 as libc::c_int as isize)).num_processes = nparam;
            nrhost = 1 as libc::c_int;
            no_touch_std = (0 as libc::c_int == 0) as libc::c_int;
        }
        i = 0 as libc::c_int;
        while i < nparam {
            (*params.offset(i as isize)).status = 0 as libc::c_int;
            (*params.offset(i as isize)).no_touch_std = 0 as libc::c_int;
            (*params.offset(i as isize)).id = i;
            i += 1;
            i;
        }
        if (no_touch_std != 0 || nparam <= 1 as libc::c_int) && (p.in_0).is_null()
            && (p.out).is_null() && (p.err).is_null()
        {
            i = 0 as libc::c_int;
            while i < nparam {
                (*params.offset(i as isize)).no_touch_std = 1 as libc::c_int;
                i += 1;
                i;
            }
        }
        if !(p.out).is_null()
            && format_check_if_formatted(
                p.out,
                b"skd\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            ) == 0
        {
            if strcmp(p.out, b"-\0" as *const u8 as *const libc::c_char)
                == 0 as libc::c_int
                || strcmp(p.out, b"-1\0" as *const u8 as *const libc::c_char)
                    == 0 as libc::c_int
            {
                p.fwout = stdout;
            } else if strcmp(p.out, b"-2\0" as *const u8 as *const libc::c_char)
                == 0 as libc::c_int
            {
                p.fwout = stderr;
            } else {
                p.fwout = fopen(p.out, b"wb\0" as *const u8 as *const libc::c_char);
            }
            if (p.fwout).is_null() {
                fprintf(
                    stderr,
                    dcgettext(
                        0 as *const libc::c_char,
                        b"%s: error: unable to create collective output file '%s'.\n\0"
                            as *const u8 as *const libc::c_char,
                        5 as libc::c_int,
                    ),
                    progbasename,
                    p.out,
                );
                return 1 as libc::c_int;
            }
        } else {
            p.fwout = 0 as *mut FILE;
        }
        if !(p.err).is_null()
            && format_check_if_formatted(
                p.err,
                b"skd\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            ) == 0
        {
            if strcmp(p.err, b"-\0" as *const u8 as *const libc::c_char)
                == 0 as libc::c_int
                || strcmp(p.err, b"-2\0" as *const u8 as *const libc::c_char)
                    == 0 as libc::c_int
            {
                p.fwerr = stderr;
            } else if strcmp(p.err, b"-1\0" as *const u8 as *const libc::c_char)
                == 0 as libc::c_int
            {
                p.fwerr = stdout;
            } else {
                p.fwerr = fopen(p.err, b"wb\0" as *const u8 as *const libc::c_char);
            }
            if (p.fwerr).is_null() {
                fprintf(
                    stderr,
                    dcgettext(
                        0 as *const libc::c_char,
                        b"%s: error: unable to create collective error file '%s'.\n\0"
                            as *const u8 as *const libc::c_char,
                        5 as libc::c_int,
                    ),
                    progbasename,
                    p.err,
                );
                return 1 as libc::c_int;
            }
        } else {
            p.fwerr = 0 as *mut FILE;
        }
        if !logfile.is_null() && log.loglevel != 0 as libc::c_int {
            if strcmp(logfile, b"-\0" as *const u8 as *const libc::c_char)
                == 0 as libc::c_int
                || strcmp(logfile, b"-2\0" as *const u8 as *const libc::c_char)
                    == 0 as libc::c_int
            {
                log.fwlog = stderr;
            } else if strcmp(logfile, b"-1\0" as *const u8 as *const libc::c_char)
                == 0 as libc::c_int
            {
                log.fwlog = stdout;
            } else {
                log.fwlog = fopen(logfile, b"wb\0" as *const u8 as *const libc::c_char);
            }
            if (log.fwlog).is_null() {
                fprintf(
                    stderr,
                    dcgettext(
                        0 as *const libc::c_char,
                        b"%s: error: unable to create log file '%s'.\n\0" as *const u8
                            as *const libc::c_char,
                        5 as libc::c_int,
                    ),
                    progbasename,
                    logfile,
                );
                return 1 as libc::c_int;
            }
            if log.loglevel < 0 as libc::c_int {
                log.loglevel = 1 as libc::c_int;
            }
        } else if log.loglevel > 0 as libc::c_int {
            log.fwlog = stderr;
        } else {
            log.fwlog = 0 as *mut FILE;
            log.loglevel = 0 as libc::c_int;
        }
        if prio > 0 as libc::c_int {
            i = setpriority(
                PRIO_PROCESS,
                0 as libc::c_int as id_t,
                prio - 128 as libc::c_int,
            );
            if i < 0 as libc::c_int {
                fprintf(
                    stderr,
                    dcgettext(
                        0 as *const libc::c_char,
                        b"%s: warning: unable to set scheduling priority to %d.\n\0"
                            as *const u8 as *const libc::c_char,
                        5 as libc::c_int,
                    ),
                    progbasename,
                    prio - 128 as libc::c_int,
                );
            }
        }
        if !ctrlport.is_null() {
            sock = remote_control_port_bind(
                ctrlport,
                &mut pctrlport,
                1 as libc::c_int,
                1 as libc::c_int,
            );
            if sock < 0 as libc::c_int {
                fprintf(
                    stderr,
                    dcgettext(
                        0 as *const libc::c_char,
                        b"%s: error: unable to create or bind control socket to %s.\n\0"
                            as *const u8 as *const libc::c_char,
                        5 as libc::c_int,
                    ),
                    progbasename,
                    ctrlport,
                );
                return 1 as libc::c_int;
            }
        } else {
            sock = -(1 as libc::c_int);
            pctrlport = 0 as *mut libc::c_char;
        }
        p.fallback_to_die = 1 as libc::c_int;
        nrshell = nrhost;
        rshells = xmalloc(
            (::std::mem::size_of::<remoteshell>() as libc::c_ulong)
                .wrapping_mul(nrshell as libc::c_ulong),
        ) as *mut remoteshell;
        hsck = -(1 as libc::c_int);
        i = 0 as libc::c_int;
        while i < nrhost {
            if ((*rhosts.offset(i as isize)).hostspec).is_null()
                && ((*rhosts.offset(i as isize)).num_processes == -(1 as libc::c_int)
                    || (*rhosts.offset(i as isize)).num_processes == -(2 as libc::c_int))
                && hsck < 0 as libc::c_int
            {
                if hypcport.is_null() {
                    hypcport = b"/tmp/pexec.sock\0" as *const u8 as *const libc::c_char
                        as *mut libc::c_char;
                }
                hsck = remote_control_port_connect(hypcport);
                if hsck < 0 as libc::c_int
                    && (*rhosts.offset(i as isize)).num_processes == -(2 as libc::c_int)
                {
                    fprintf(
                        stderr,
                        dcgettext(
                            0 as *const libc::c_char,
                            b"%s: error: unable to connect hypervisor socket '%s'.\n\0"
                                as *const u8 as *const libc::c_char,
                            5 as libc::c_int,
                        ),
                        progbasename,
                        hypcport,
                    );
                    return 1 as libc::c_int;
                } else if hsck < 0 as libc::c_int {
                    (*rhosts.offset(i as isize)).num_processes = -(3 as libc::c_int);
                    hsck = -(1 as libc::c_int);
                } else {
                    (*rhosts.offset(i as isize)).num_processes = 0 as libc::c_int;
                }
                if (*rhosts.offset(i as isize)).num_processes < 0 as libc::c_int {
                    (*rhosts.offset(i as isize)).num_processes = get_number_of_cpus();
                }
            }
            r = remote_shell_init(
                p.rsh,
                p.rshargs,
                pexec_self,
                ctrlport,
                timeout,
                &mut *rhosts.offset(i as isize),
                &mut *rshells.offset(i as isize),
                prio,
            );
            if r != 0 {
                fprintf(
                    stderr,
                    dcgettext(
                        0 as *const libc::c_char,
                        b"%s: error: unable to connect or initialize remote shell '%s' and/or pexec daemon '%s' to the host '%s' (reason code=%d).\n\0"
                            as *const u8 as *const libc::c_char,
                        5 as libc::c_int,
                    ),
                    progbasename,
                    p.rsh,
                    pexec_self,
                    (*rhosts.offset(i as isize)).hostspec,
                    r,
                );
                return 1 as libc::c_int;
            }
            i += 1;
            i;
        }
        if !pctrlport.is_null() {
            bind_variable_export(ctrlenv, pctrlport);
        }
        if sock >= 0 as libc::c_int {
            listen(sock, 256 as libc::c_int);
        }
        p.log = &mut log;
        signal(
            13 as libc::c_int,
            ::std::mem::transmute::<
                libc::intptr_t,
                __sighandler_t,
            >(1 as libc::c_int as libc::intptr_t),
        );
        ret = pexec_do_parallelized_execution(
            &mut p,
            params,
            nparam,
            rshells,
            nrshell,
            sock,
            hsck,
        );
        i = 0 as libc::c_int;
        rs = rshells;
        while i < nrshell {
            if !((*rs).pid < 0 as libc::c_int) {
                hprintf(
                    (*rs).fhsend,
                    b"exit\n\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                );
                waitpid((*rs).pid, &mut status, 0 as libc::c_int);
            }
            i += 1;
            i;
            rs = rs.offset(1);
            rs;
        }
        if !rshells.is_null() {
            free(rshells as *mut libc::c_void);
        }
        if !(log.fwlog).is_null() && fileno(log.fwlog) != 1 as libc::c_int
            && fileno(log.fwlog) != 2 as libc::c_int
        {
            fclose(log.fwlog);
        }
        if !(p.fwout).is_null() && fileno(p.fwout) != 1 as libc::c_int
            && fileno(p.fwout) != 2 as libc::c_int
        {
            fclose(p.fwout);
        }
        if !(p.fwerr).is_null() && fileno(p.fwerr) != 1 as libc::c_int
            && fileno(p.fwerr) != 2 as libc::c_int
        {
            fclose(p.fwerr);
        }
        if sock >= 0 as libc::c_int {
            close(sock);
            if !pctrlport.is_null() && is_unix_socket_name(pctrlport) != 0 {
                unlink(pctrlport);
            }
        }
        if hsck >= 0 as libc::c_int {
            hprintf(
                hsck,
                b"close\n\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            );
            fdwait(hsck, 0 as libc::c_int);
            i = close(hsck);
        }
        if !pnames.is_null() {
            free(pnames as *mut libc::c_void);
        }
        if !list.is_null() {
            free(list as *mut libc::c_void);
        }
        if !params.is_null() {
            free(params as *mut libc::c_void);
        }
        if !pctrlport.is_null() {
            free(pctrlport as *mut libc::c_void);
        }
        free(p.rshcmd as *mut libc::c_void);
        free(p.rshargs as *mut libc::c_void);
        i = 0 as libc::c_int;
        while i < nrhost {
            if !((*rhosts.offset(i as isize)).hostspec).is_null() {
                free((*rhosts.offset(i as isize)).hostspec as *mut libc::c_void);
            }
            i += 1;
            i;
        }
        free(rhosts as *mut libc::c_void);
        return ret;
    } else if mode == 1 as libc::c_int {
        let mut fhrecv: libc::c_int = 0;
        let mut fhsend: libc::c_int = 0;
        let mut num_processes: libc::c_int = 0;
        let mut sock_0: libc::c_int = 0;
        let mut hsck_0: libc::c_int = 0;
        let mut pctrlport_0: *mut libc::c_char = 0 as *mut libc::c_char;
        num_processes = -(1 as libc::c_int);
        ctrlport = 0 as *mut libc::c_char;
        prio = 0 as libc::c_int;
        i = zeroarg;
        while i < argc {
            if strcmp(
                *argv.offset(i as isize),
                b"-n\0" as *const u8 as *const libc::c_char,
            ) == 0 as libc::c_int
                || strcmp(
                    *argv.offset(i as isize),
                    b"--number\0" as *const u8 as *const libc::c_char,
                ) == 0 as libc::c_int
            {
                i += 1;
                if i >= argc {
                    fprint_err_invarg1(*argv.offset((i - 1 as libc::c_int) as isize));
                    return 1 as libc::c_int;
                } else if strcmp(
                    *argv.offset(i as isize),
                    b"auto\0" as *const u8 as *const libc::c_char,
                ) == 0 as libc::c_int
                {
                    num_processes = -(1 as libc::c_int);
                } else if strcmp(
                    *argv.offset(i as isize),
                    b"managed\0" as *const u8 as *const libc::c_char,
                ) == 0 as libc::c_int
                {
                    num_processes = -(2 as libc::c_int);
                } else if strcmp(
                    *argv.offset(i as isize),
                    b"ncpu\0" as *const u8 as *const libc::c_char,
                ) == 0 as libc::c_int
                {
                    num_processes = -(3 as libc::c_int);
                } else if sscanf(
                    *argv.offset(i as isize),
                    b"%d\0" as *const u8 as *const libc::c_char,
                    &mut num_processes as *mut libc::c_int,
                ) < 1 as libc::c_int || num_processes <= 0 as libc::c_int
                {
                    fprint_err_invarg1(*argv.offset((i - 1 as libc::c_int) as isize));
                    return 1 as libc::c_int;
                }
            } else if strcmp(
                *argv.offset(i as isize),
                b"-y\0" as *const u8 as *const libc::c_char,
            ) == 0 as libc::c_int
                || strcmp(
                    *argv.offset(i as isize),
                    b"--bind\0" as *const u8 as *const libc::c_char,
                ) == 0 as libc::c_int
            {
                i += 1;
                if i >= argc {
                    fprint_err_invarg1(*argv.offset((i - 1 as libc::c_int) as isize));
                    return 1 as libc::c_int;
                }
                ctrlport = *argv.offset(i as isize);
            } else if strcmp(
                *argv.offset(i as isize),
                b"-C\0" as *const u8 as *const libc::c_char,
            ) == 0 as libc::c_int
                || strcmp(
                    *argv.offset(i as isize),
                    b"--control\0" as *const u8 as *const libc::c_char,
                ) == 0 as libc::c_int
            {
                i += 1;
                if i >= argc {
                    fprint_err_invarg1(*argv.offset((i - 1 as libc::c_int) as isize));
                    return 1 as libc::c_int;
                }
                hypcport = *argv.offset(i as isize);
            } else if strcmp(
                *argv.offset(i as isize),
                b"-z\0" as *const u8 as *const libc::c_char,
            ) == 0 as libc::c_int
                || strcmp(
                    *argv.offset(i as isize),
                    b"--nice\0" as *const u8 as *const libc::c_char,
                ) == 0 as libc::c_int
            {
                i += 1;
                if i >= argc {
                    fprint_err_invarg1(*argv.offset((i - 1 as libc::c_int) as isize));
                    return 1 as libc::c_int;
                } else if sscanf(
                    *argv.offset(i as isize),
                    b"%d\0" as *const u8 as *const libc::c_char,
                    &mut prio as *mut libc::c_int,
                ) < 1 as libc::c_int
                {
                    fprint_err_invarg1(*argv.offset((i - 1 as libc::c_int) as isize));
                    return 1 as libc::c_int;
                }
                prio += 128 as libc::c_int;
            } else {
                fprint_err_invarg0(*argv.offset(i as isize));
                return 1 as libc::c_int;
            }
            i += 1;
            i;
        }
        if num_processes == -(1 as libc::c_int) || num_processes == -(2 as libc::c_int) {
            if hypcport.is_null() {
                hypcport = b"/tmp/pexec.sock\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char;
            }
            hsck_0 = remote_control_port_connect(hypcport);
            if hsck_0 < 0 as libc::c_int && num_processes == -(2 as libc::c_int) {
                fprintf(
                    stderr,
                    dcgettext(
                        0 as *const libc::c_char,
                        b"%s: error: unable to connect hypervisor socket '%s'.\n\0"
                            as *const u8 as *const libc::c_char,
                        5 as libc::c_int,
                    ),
                    progbasename,
                    hypcport,
                );
                return 1 as libc::c_int;
            } else if hsck_0 < 0 as libc::c_int {
                num_processes = -(3 as libc::c_int);
                hsck_0 = -(1 as libc::c_int);
            } else {
                num_processes = 0 as libc::c_int;
            }
        } else {
            hsck_0 = -(1 as libc::c_int);
        }
        if num_processes < 0 as libc::c_int {
            num_processes = get_number_of_cpus();
        }
        if !ctrlport.is_null() {
            sock_0 = remote_control_port_bind(
                ctrlport,
                &mut pctrlport_0,
                1 as libc::c_int,
                1 as libc::c_int,
            );
            if sock_0 < 0 as libc::c_int {
                fprintf(
                    stderr,
                    dcgettext(
                        0 as *const libc::c_char,
                        b"%s: error: unable to create or bind control socket to %s.\n\0"
                            as *const u8 as *const libc::c_char,
                        5 as libc::c_int,
                    ),
                    progbasename,
                    ctrlport,
                );
                return 1 as libc::c_int;
            }
        } else {
            sock_0 = -(1 as libc::c_int);
            pctrlport_0 = 0 as *mut libc::c_char;
        }
        fhrecv = fileno(stdin);
        fhsend = fileno(stdout);
        if !pctrlport_0.is_null() {
            bind_variable_export(ctrlenv, pctrlport_0);
        }
        if sock_0 >= 0 as libc::c_int {
            listen(sock_0, 256 as libc::c_int);
        }
        if prio > 0 as libc::c_int {
            i = setpriority(
                PRIO_PROCESS,
                0 as libc::c_int as id_t,
                prio - 128 as libc::c_int,
            );
        }
        signal(
            13 as libc::c_int,
            ::std::mem::transmute::<
                libc::intptr_t,
                __sighandler_t,
            >(1 as libc::c_int as libc::intptr_t),
        );
        pexec_daemon_main_loop(fhrecv, fhsend, num_processes, sock_0, hsck_0);
        if sock_0 >= 0 as libc::c_int {
            close(sock_0);
            if !pctrlport_0.is_null() && is_unix_socket_name(pctrlport_0) != 0 {
                unlink(pctrlport_0);
            }
        }
        if hsck_0 >= 0 as libc::c_int {
            hprintf(
                hsck_0,
                b"close\n\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            );
            fdwait(hsck_0, 0 as libc::c_int);
            i = close(hsck_0);
        }
        if !pctrlport_0.is_null() {
            free(pctrlport_0 as *mut libc::c_void);
        }
        return 0 as libc::c_int;
    } else if mode == 2 as libc::c_int {
        let mut ctrlport_0: *mut libc::c_char = 0 as *mut libc::c_char;
        let mut sock_1: libc::c_int = 0;
        let mut task: libc::c_int = 0;
        let mut file: *mut libc::c_char = 0 as *mut libc::c_char;
        let mut name: *mut libc::c_char = 0 as *mut libc::c_char;
        let mut fr_0: *mut FILE = 0 as *mut FILE;
        let mut fw: *mut FILE = 0 as *mut FILE;
        let mut azeroarg: libc::c_int = 0;
        ctrlport_0 = 0 as *mut libc::c_char;
        task = 1 as libc::c_int;
        name = 0 as *mut libc::c_char;
        file = 0 as *mut libc::c_char;
        is_shell_commands = 0 as libc::c_int;
        azeroarg = -(1 as libc::c_int);
        i = zeroarg;
        while i < argc {
            if strcmp(
                *argv.offset(i as isize),
                b"-p\0" as *const u8 as *const libc::c_char,
            ) == 0 as libc::c_int
                || strcmp(
                    *argv.offset(i as isize),
                    b"--connect\0" as *const u8 as *const libc::c_char,
                ) == 0 as libc::c_int
            {
                i += 1;
                if i >= argc {
                    fprint_err_invarg1(*argv.offset((i - 1 as libc::c_int) as isize));
                    return 1 as libc::c_int;
                }
                ctrlport_0 = *argv.offset(i as isize);
            } else if strcmp(
                *argv.offset(i as isize),
                b"-t\0" as *const u8 as *const libc::c_char,
            ) == 0 as libc::c_int
                || strcmp(
                    *argv.offset(i as isize),
                    b"--status\0" as *const u8 as *const libc::c_char,
                ) == 0 as libc::c_int
            {
                task = 1 as libc::c_int;
            } else if strcmp(
                *argv.offset(i as isize),
                b"-l\0" as *const u8 as *const libc::c_char,
            ) == 0 as libc::c_int
                || strcmp(
                    *argv.offset(i as isize),
                    b"--lock\0" as *const u8 as *const libc::c_char,
                ) == 0 as libc::c_int
                || strcmp(
                    *argv.offset(i as isize),
                    b"--mutex-lock\0" as *const u8 as *const libc::c_char,
                ) == 0 as libc::c_int
            {
                i += 1;
                if i >= argc {
                    fprint_err_invarg1(*argv.offset((i - 1 as libc::c_int) as isize));
                    return 1 as libc::c_int;
                }
                task = 2 as libc::c_int;
                name = *argv.offset(i as isize);
            } else if strcmp(
                *argv.offset(i as isize),
                b"-u\0" as *const u8 as *const libc::c_char,
            ) == 0 as libc::c_int
                || strcmp(
                    *argv.offset(i as isize),
                    b"--unlock\0" as *const u8 as *const libc::c_char,
                ) == 0 as libc::c_int
                || strcmp(
                    *argv.offset(i as isize),
                    b"--mutex-unlock\0" as *const u8 as *const libc::c_char,
                ) == 0 as libc::c_int
            {
                i += 1;
                if i >= argc {
                    fprint_err_invarg1(*argv.offset((i - 1 as libc::c_int) as isize));
                    return 1 as libc::c_int;
                }
                task = 3 as libc::c_int;
                name = *argv.offset(i as isize);
            } else if strcmp(
                *argv.offset(i as isize),
                b"-m\0" as *const u8 as *const libc::c_char,
            ) == 0 as libc::c_int
                || strcmp(
                    *argv.offset(i as isize),
                    b"--mutex\0" as *const u8 as *const libc::c_char,
                ) == 0 as libc::c_int
            {
                i += 1;
                if i >= argc {
                    fprint_err_invarg1(*argv.offset((i - 1 as libc::c_int) as isize));
                    return 1 as libc::c_int;
                }
                name = *argv.offset(i as isize);
            } else if strcmp(
                *argv.offset(i as isize),
                b"-d\0" as *const u8 as *const libc::c_char,
            ) == 0 as libc::c_int
                || strcmp(
                    *argv.offset(i as isize),
                    b"--dump\0" as *const u8 as *const libc::c_char,
                ) == 0 as libc::c_int
            {
                i += 1;
                if i >= argc {
                    fprint_err_invarg1(*argv.offset((i - 1 as libc::c_int) as isize));
                    return 1 as libc::c_int;
                }
                task = 4 as libc::c_int;
                file = *argv.offset(i as isize);
            } else if strcmp(
                *argv.offset(i as isize),
                b"-s\0" as *const u8 as *const libc::c_char,
            ) == 0 as libc::c_int
                || strcmp(
                    *argv.offset(i as isize),
                    b"--save\0" as *const u8 as *const libc::c_char,
                ) == 0 as libc::c_int
            {
                i += 1;
                if i >= argc {
                    fprint_err_invarg1(*argv.offset((i - 1 as libc::c_int) as isize));
                    return 1 as libc::c_int;
                }
                task = 5 as libc::c_int;
                file = *argv.offset(i as isize);
            } else if strcmp(
                *argv.offset(i as isize),
                b"-c\0" as *const u8 as *const libc::c_char,
            ) == 0 as libc::c_int
                || strcmp(
                    *argv.offset(i as isize),
                    b"--shell-command\0" as *const u8 as *const libc::c_char,
                ) == 0 as libc::c_int
            {
                is_shell_commands = 1 as libc::c_int;
            } else if strcmp(
                *argv.offset(i as isize),
                b"-a\0" as *const u8 as *const libc::c_char,
            ) == 0 as libc::c_int
                || strcmp(
                    *argv.offset(i as isize),
                    b"--atomic\0" as *const u8 as *const libc::c_char,
                ) == 0 as libc::c_int
            {
                task = 6 as libc::c_int;
            } else if strcmp(
                *argv.offset(i as isize),
                b"--\0" as *const u8 as *const libc::c_char,
            ) == 0 as libc::c_int && task == 6 as libc::c_int
            {
                azeroarg = i + 1 as libc::c_int;
                break;
            } else if *(*argv.offset(i as isize)).offset(0 as libc::c_int as isize)
                as libc::c_int == '-' as i32
            {
                fprint_err_invarg0(*argv.offset(i as isize));
                return 1 as libc::c_int;
            } else if task == 6 as libc::c_int {
                azeroarg = i;
                break;
            } else {
                fprint_err_invarg0(*argv.offset(i as isize));
                return 1 as libc::c_int;
            }
            i += 1;
            i;
        }
        if ctrlport_0.is_null()
            || strcmp(ctrlport_0, b"auto\0" as *const u8 as *const libc::c_char)
                == 0 as libc::c_int
        {
            ctrlport_0 = getenv(ctrlenv);
        }
        if ctrlport_0.is_null() {
            fprintf(
                stderr,
                dcgettext(
                    0 as *const libc::c_char,
                    b"%s: error: connection port has not been defined or set in the environment.\n\0"
                        as *const u8 as *const libc::c_char,
                    5 as libc::c_int,
                ),
                progbasename,
            );
            return 1 as libc::c_int;
        }
        sock_1 = remote_control_port_connect(ctrlport_0);
        if sock_1 < 0 as libc::c_int {
            fprintf(
                stderr,
                dcgettext(
                    0 as *const libc::c_char,
                    b"%s: error: unable to connect to '%s'.\n\0" as *const u8
                        as *const libc::c_char,
                    5 as libc::c_int,
                ),
                progbasename,
                ctrlport_0,
            );
            return 1 as libc::c_int;
        }
        match task {
            1 => {
                ret = remote_status(sock_1, stdout);
            }
            2 => {
                ret = remote_lock(sock_1, 0 as *mut FILE, name);
            }
            3 => {
                ret = remote_unlock(sock_1, 0 as *mut FILE, name);
            }
            4 => {
                if !file.is_null()
                    && {
                        fr_0 = fopen(file, b"rb\0" as *const u8 as *const libc::c_char);
                        !fr_0.is_null()
                    } && !name.is_null()
                {
                    ret = remote_copy(sock_1, name, fr_0, stdout);
                    fclose(fr_0);
                } else {
                    fprintf(
                        stderr,
                        dcgettext(
                            0 as *const libc::c_char,
                            b"%s: error: invalid or unspecified input file (%s).\n\0"
                                as *const u8 as *const libc::c_char,
                            5 as libc::c_int,
                        ),
                        progbasename,
                        if file.is_null() {
                            b"-\0" as *const u8 as *const libc::c_char
                        } else {
                            file as *const libc::c_char
                        },
                    );
                    ret = 1 as libc::c_int;
                }
            }
            5 => {
                if !file.is_null()
                    && {
                        fw = fopen(file, b"wb\0" as *const u8 as *const libc::c_char);
                        !fw.is_null()
                    } && !name.is_null()
                {
                    ret = remote_copy(sock_1, name, stdin, fw);
                    fclose(fw);
                } else {
                    fprintf(
                        stderr,
                        dcgettext(
                            0 as *const libc::c_char,
                            b"%s: error: invalid or unspecified output file (%s).\n\0"
                                as *const u8 as *const libc::c_char,
                            5 as libc::c_int,
                        ),
                        progbasename,
                        if file.is_null() {
                            b"-\0" as *const u8 as *const libc::c_char
                        } else {
                            file as *const libc::c_char
                        },
                    );
                    ret = 1 as libc::c_int;
                }
            }
            6 => {
                if !name.is_null() {
                    remote_lock(sock_1, 0 as *mut FILE, name);
                }
                if azeroarg < argc {
                    ret = remote_atomic_execute(
                        is_shell_commands,
                        p.shell,
                        argc - azeroarg,
                        argv.offset(azeroarg as isize),
                        sock_1,
                    );
                } else {
                    ret = 0 as libc::c_int;
                }
                if !name.is_null() {
                    remote_unlock(sock_1, 0 as *mut FILE, name);
                }
            }
            _ => {
                ret = 1 as libc::c_int;
            }
        }
        remote_disconnect(sock_1);
        close(sock_1);
        return ret;
    } else if mode == 3 as libc::c_int {
        let mut startstop: libc::c_int = 0;
        let mut num_processes_0: libc::c_int = 0;
        let mut use_load_0: libc::c_int = 0;
        let mut use_fifo: libc::c_int = 0;
        let mut sock_2: libc::c_int = 0;
        hypcport = 0 as *mut libc::c_char;
        startstop = -(1 as libc::c_int);
        num_processes_0 = -(1 as libc::c_int);
        use_load_0 = -(1 as libc::c_int);
        use_fifo = 0 as libc::c_int;
        i = zeroarg;
        while i < argc {
            if strcmp(
                *argv.offset(i as isize),
                b"-n\0" as *const u8 as *const libc::c_char,
            ) == 0 as libc::c_int
                || strcmp(
                    *argv.offset(i as isize),
                    b"--number\0" as *const u8 as *const libc::c_char,
                ) == 0 as libc::c_int
            {
                i += 1;
                if i >= argc {
                    fprint_err_invarg1(*argv.offset((i - 1 as libc::c_int) as isize));
                    return 1 as libc::c_int;
                } else if strcmp(
                    *argv.offset(i as isize),
                    b"auto\0" as *const u8 as *const libc::c_char,
                ) == 0 as libc::c_int
                    || strcmp(
                        *argv.offset(i as isize),
                        b"ncpu\0" as *const u8 as *const libc::c_char,
                    ) == 0 as libc::c_int
                {
                    num_processes_0 = -(3 as libc::c_int);
                } else if sscanf(
                    *argv.offset(i as isize),
                    b"%d\0" as *const u8 as *const libc::c_char,
                    &mut num_processes_0 as *mut libc::c_int,
                ) < 1 as libc::c_int || num_processes_0 <= 0 as libc::c_int
                {
                    fprint_err_invarg1(*argv.offset((i - 1 as libc::c_int) as isize));
                    return 1 as libc::c_int;
                }
            } else if strcmp(
                *argv.offset(i as isize),
                b"-l\0" as *const u8 as *const libc::c_char,
            ) == 0 as libc::c_int
                || strcmp(
                    *argv.offset(i as isize),
                    b"--load\0" as *const u8 as *const libc::c_char,
                ) == 0 as libc::c_int
                || strcmp(
                    *argv.offset(i as isize),
                    b"--use-load\0" as *const u8 as *const libc::c_char,
                ) == 0 as libc::c_int
            {
                i += 1;
                if i >= argc {
                    fprint_err_invarg1(*argv.offset((i - 1 as libc::c_int) as isize));
                    return 1 as libc::c_int;
                } else if strcmp(
                    *argv.offset(i as isize),
                    b"0\0" as *const u8 as *const libc::c_char,
                ) == 0 as libc::c_int
                    || strcmp(
                        *argv.offset(i as isize),
                        b"1m\0" as *const u8 as *const libc::c_char,
                    ) == 0 as libc::c_int
                    || strcmp(
                        *argv.offset(i as isize),
                        b"1min\0" as *const u8 as *const libc::c_char,
                    ) == 0 as libc::c_int
                {
                    use_load_0 = 0 as libc::c_int;
                } else if strcmp(
                    *argv.offset(i as isize),
                    b"1\0" as *const u8 as *const libc::c_char,
                ) == 0 as libc::c_int
                    || strcmp(
                        *argv.offset(i as isize),
                        b"5m\0" as *const u8 as *const libc::c_char,
                    ) == 0 as libc::c_int
                    || strcmp(
                        *argv.offset(i as isize),
                        b"5min\0" as *const u8 as *const libc::c_char,
                    ) == 0 as libc::c_int
                {
                    use_load_0 = 1 as libc::c_int;
                } else if strcmp(
                    *argv.offset(i as isize),
                    b"2\0" as *const u8 as *const libc::c_char,
                ) == 0 as libc::c_int
                    || strcmp(
                        *argv.offset(i as isize),
                        b"15m\0" as *const u8 as *const libc::c_char,
                    ) == 0 as libc::c_int
                    || strcmp(
                        *argv.offset(i as isize),
                        b"15min\0" as *const u8 as *const libc::c_char,
                    ) == 0 as libc::c_int
                {
                    use_load_0 = 2 as libc::c_int;
                } else {
                    fprint_err_invarg1(*argv.offset((i - 1 as libc::c_int) as isize));
                    return 1 as libc::c_int;
                }
            } else if strcmp(
                *argv.offset(i as isize),
                b"-C\0" as *const u8 as *const libc::c_char,
            ) == 0 as libc::c_int
                || strcmp(
                    *argv.offset(i as isize),
                    b"--control\0" as *const u8 as *const libc::c_char,
                ) == 0 as libc::c_int
            {
                i += 1;
                if i >= argc {
                    fprint_err_invarg1(*argv.offset((i - 1 as libc::c_int) as isize));
                    return 1 as libc::c_int;
                }
                hypcport = *argv.offset(i as isize);
            } else if strcmp(
                *argv.offset(i as isize),
                b"-L\0" as *const u8 as *const libc::c_char,
            ) == 0 as libc::c_int
                || strcmp(
                    *argv.offset(i as isize),
                    b"--log\0" as *const u8 as *const libc::c_char,
                ) == 0 as libc::c_int
            {
                i += 1;
                if i >= argc {
                    fprint_err_invarg1(*argv.offset((i - 1 as libc::c_int) as isize));
                    return 1 as libc::c_int;
                }
                logfile = *argv.offset(i as isize);
            } else if strcmp(
                *argv.offset(i as isize),
                b"-V\0" as *const u8 as *const libc::c_char,
            ) == 0 as libc::c_int
                || strcmp(
                    *argv.offset(i as isize),
                    b"--verbose\0" as *const u8 as *const libc::c_char,
                ) == 0 as libc::c_int
            {
                log.loglevel += 1;
                log.loglevel;
            } else if strcmp(
                *argv.offset(i as isize),
                b"-W\0" as *const u8 as *const libc::c_char,
            ) == 0 as libc::c_int
                || strcmp(
                    *argv.offset(i as isize),
                    b"--log-level\0" as *const u8 as *const libc::c_char,
                ) == 0 as libc::c_int
            {
                i += 1;
                if i >= argc {
                    fprint_err_invarg1(*argv.offset((i - 1 as libc::c_int) as isize));
                    return 1 as libc::c_int;
                } else if sscanf(
                    *argv.offset(i as isize),
                    b"%d\0" as *const u8 as *const libc::c_char,
                    &mut log.loglevel as *mut libc::c_int,
                ) < 1 as libc::c_int || log.loglevel < 0 as libc::c_int
                {
                    fprint_err_invarg1(*argv.offset((i - 1 as libc::c_int) as isize));
                    return 1 as libc::c_int;
                }
            } else if strcmp(
                *argv.offset(i as isize),
                b"-f\0" as *const u8 as *const libc::c_char,
            ) == 0 as libc::c_int
                || strcmp(
                    *argv.offset(i as isize),
                    b"--fifo\0" as *const u8 as *const libc::c_char,
                ) == 0 as libc::c_int
            {
                use_fifo = 1 as libc::c_int;
            } else if strcmp(
                *argv.offset(i as isize),
                b"-s\0" as *const u8 as *const libc::c_char,
            ) == 0 as libc::c_int
                || strcmp(
                    *argv.offset(i as isize),
                    b"--lifo\0" as *const u8 as *const libc::c_char,
                ) == 0 as libc::c_int
                || strcmp(
                    *argv.offset(i as isize),
                    b"--stack\0" as *const u8 as *const libc::c_char,
                ) == 0 as libc::c_int
            {
                use_fifo = 0 as libc::c_int;
            } else if strcmp(
                *argv.offset(i as isize),
                b"start\0" as *const u8 as *const libc::c_char,
            ) == 0 as libc::c_int
            {
                startstop = 0 as libc::c_int;
            } else if strcmp(
                *argv.offset(i as isize),
                b"stop\0" as *const u8 as *const libc::c_char,
            ) == 0 as libc::c_int
            {
                startstop = 1 as libc::c_int;
            } else {
                fprint_err_invarg0(*argv.offset(i as isize));
                return 1 as libc::c_int;
            }
            i += 1;
            i;
        }
        if num_processes_0 <= 0 as libc::c_int {
            num_processes_0 = get_number_of_cpus();
        }
        if hypcport.is_null() {
            hypcport = b"/tmp/pexec.sock\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char;
        }
        if startstop > 0 as libc::c_int {
            sock_2 = remote_control_port_connect(hypcport);
            if sock_2 < 0 as libc::c_int {
                fprintf(
                    stderr,
                    dcgettext(
                        0 as *const libc::c_char,
                        b"%s: error: unable to connect to hypervisor control socket '%s' in order to stop the service.\n\0"
                            as *const u8 as *const libc::c_char,
                        5 as libc::c_int,
                    ),
                    progbasename,
                    hypcport,
                );
                return 1 as libc::c_int;
            }
            pexec_hypervisor_stop(sock_2);
            close(sock_2);
            return 0 as libc::c_int;
        }
        sock_2 = remote_control_port_bind(
            hypcport,
            0 as *mut *mut libc::c_char,
            0 as libc::c_int,
            1 as libc::c_int,
        );
        if sock_2 < 0 as libc::c_int {
            fprintf(
                stderr,
                dcgettext(
                    0 as *const libc::c_char,
                    b"%s: error: unable to create or bind control socket to %s.\n\0"
                        as *const u8 as *const libc::c_char,
                    5 as libc::c_int,
                ),
                progbasename,
                hypcport,
            );
            return 1 as libc::c_int;
        }
        if !(startstop < 0 as libc::c_int) {
            background(0 as libc::c_int, 0 as libc::c_int);
        }
        if sock_2 >= 0 as libc::c_int {
            listen(sock_2, 256 as libc::c_int);
        }
        signal(
            13 as libc::c_int,
            ::std::mem::transmute::<
                libc::intptr_t,
                __sighandler_t,
            >(1 as libc::c_int as libc::intptr_t),
        );
        pexec_hypervisor_main_loop(sock_2, num_processes_0, use_load_0, use_fifo);
        if is_unix_socket_name(hypcport) != 0 {
            unlink(hypcport);
        }
        return 0 as libc::c_int;
    } else {
        fprintf(
            stderr,
            dcgettext(
                0 as *const libc::c_char,
                b"%s: error: internal: invalid mode code %d.\n\0" as *const u8
                    as *const libc::c_char,
                5 as libc::c_int,
            ),
            progbasename,
            mode,
        );
        return 2 as libc::c_int;
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
