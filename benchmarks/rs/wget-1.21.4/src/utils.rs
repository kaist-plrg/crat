use ::libc;
use ::c2rust_bitfields;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    pub type re_dfa_t;
    pub type real_pcre;
    pub type hash_table;
    static mut stdin: *mut FILE;
    static mut stdout: *mut FILE;
    static mut stderr: *mut FILE;
    fn fclose(__stream: *mut FILE) -> libc::c_int;
    fn freopen(
        __filename: *const libc::c_char,
        __modes: *const libc::c_char,
        __stream: *mut FILE,
    ) -> *mut FILE;
    fn fdopen(__fd: libc::c_int, __modes: *const libc::c_char) -> *mut FILE;
    fn fprintf(_: *mut FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
    fn sprintf(_: *mut libc::c_char, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn snprintf(
        _: *mut libc::c_char,
        _: libc::c_ulong,
        _: *const libc::c_char,
        _: ...
    ) -> libc::c_int;
    fn vasprintf(
        __ptr: *mut *mut libc::c_char,
        __f: *const libc::c_char,
        __arg: ::std::ffi::VaList,
    ) -> libc::c_int;
    fn ftello(__stream: *mut FILE) -> __off_t;
    fn time(__timer: *mut time_t) -> time_t;
    fn strftime(
        __s: *mut libc::c_char,
        __maxsize: size_t,
        __format: *const libc::c_char,
        __tp: *const tm,
    ) -> size_t;
    fn localtime(__timer: *const time_t) -> *mut tm;
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
    fn __lxstat(
        __ver: libc::c_int,
        __filename: *const libc::c_char,
        __stat_buf: *mut stat,
    ) -> libc::c_int;
    fn mkdir(__path: *const libc::c_char, __mode: __mode_t) -> libc::c_int;
    fn fnmatch(
        __pattern: *const libc::c_char,
        __name: *const libc::c_char,
        __flags: libc::c_int,
    ) -> libc::c_int;
    fn dcgettext(
        __domainname: *const libc::c_char,
        __msgid: *const libc::c_char,
        __category: libc::c_int,
    ) -> *mut libc::c_char;
    fn localeconv() -> *mut lconv;
    fn memcpy(
        _: *mut libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn memset(
        _: *mut libc::c_void,
        _: libc::c_int,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn strcpy(_: *mut libc::c_char, _: *const libc::c_char) -> *mut libc::c_char;
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    fn strdup(_: *const libc::c_char) -> *mut libc::c_char;
    fn strchr(_: *const libc::c_char, _: libc::c_int) -> *mut libc::c_char;
    fn strrchr(_: *const libc::c_char, _: libc::c_int) -> *mut libc::c_char;
    fn strpbrk(_: *const libc::c_char, _: *const libc::c_char) -> *mut libc::c_char;
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    fn strerror(_: libc::c_int) -> *mut libc::c_char;
    fn strcasecmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    fn rpl_free(_: *mut libc::c_void);
    static mut opt: options;
    fn random() -> libc::c_long;
    fn srandom(__seed: libc::c_uint);
    fn abort() -> !;
    fn exit(_: libc::c_int) -> !;
    fn xmalloc(s: size_t) -> *mut libc::c_void;
    fn xrealloc(p: *mut libc::c_void, s: size_t) -> *mut libc::c_void;
    fn xstrdup(str: *const libc::c_char) -> *mut libc::c_char;
    fn perror(__s: *const libc::c_char);
    fn fileno(__stream: *mut FILE) -> libc::c_int;
    fn rpl_fopen(filename: *const libc::c_char, mode: *const libc::c_char) -> *mut FILE;
    fn rpl_fseeko(fp: *mut FILE, offset: off_t, whence: libc::c_int) -> libc::c_int;
    fn logprintf(_: log_options, _: *const libc::c_char, _: ...);
    fn debug_logprintf(_: *const libc::c_char, _: ...);
    fn log_set_save_context(_: bool) -> bool;
    fn quote(arg: *const libc::c_char) -> *const libc::c_char;
    static mut exec_name: *const libc::c_char;
    fn close(__fd: libc::c_int) -> libc::c_int;
    fn read(__fd: libc::c_int, __buf: *mut libc::c_void, __nbytes: size_t) -> ssize_t;
    fn sleep(__seconds: libc::c_uint) -> libc::c_uint;
    fn usleep(__useconds: __useconds_t) -> libc::c_int;
    fn pathconf(__path: *const libc::c_char, __name: libc::c_int) -> libc::c_long;
    fn getpid() -> __pid_t;
    fn setsid() -> __pid_t;
    fn getuid() -> __uid_t;
    fn group_member(__gid: __gid_t) -> libc::c_int;
    fn fork() -> __pid_t;
    fn unlink(__name: *const libc::c_char) -> libc::c_int;
    fn __errno_location() -> *mut libc::c_int;
    fn open(__file: *const libc::c_char, __oflag: libc::c_int, _: ...) -> libc::c_int;
    fn utime(__file: *const libc::c_char, __file_times: *const utimbuf) -> libc::c_int;
    fn setitimer(
        __which: __itimer_which_t,
        __new: *const itimerval,
        __old: *mut itimerval,
    ) -> libc::c_int;
    fn rpl_ioctl(fd: libc::c_int, request: libc::c_int, _: ...) -> libc::c_int;
    fn signal(__sig: libc::c_int, __handler: __sighandler_t) -> __sighandler_t;
    fn __sigsetjmp(__env: *mut __jmp_buf_tag, __savemask: libc::c_int) -> libc::c_int;
    fn siglongjmp(_: *mut __jmp_buf_tag, _: libc::c_int) -> !;
    fn rpl_regcomp(
        __preg: *mut regex_t,
        __pattern: *const libc::c_char,
        __cflags: libc::c_int,
    ) -> libc::c_int;
    fn rpl_regexec(
        __preg: *const regex_t,
        __String: *const libc::c_char,
        __nmatch: size_t,
        __pmatch: *mut regmatch_t,
        __eflags: libc::c_int,
    ) -> libc::c_int;
    fn rpl_regerror(
        __errcode: libc::c_int,
        __preg: *const regex_t,
        __errbuf: *mut libc::c_char,
        __errbuf_size: size_t,
    ) -> size_t;
    fn pcre_compile(
        _: *const libc::c_char,
        _: libc::c_int,
        _: *mut *const libc::c_char,
        _: *mut libc::c_int,
        _: *const libc::c_uchar,
    ) -> *mut pcre;
    fn pcre_exec(
        _: *const pcre,
        _: *const pcre_extra,
        _: *const libc::c_char,
        _: libc::c_int,
        _: libc::c_int,
        _: libc::c_int,
        _: *mut libc::c_int,
        _: libc::c_int,
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
    fn hash_table_destroy(_: *mut hash_table);
    fn hash_table_contains(_: *const hash_table, _: *const libc::c_void) -> libc::c_int;
    fn hash_table_put(
        _: *mut hash_table,
        _: *const libc::c_void,
        _: *const libc::c_void,
    );
    fn hash_table_iterate(_: *mut hash_table, _: *mut hash_table_iterator);
    fn hash_table_iter_next(_: *mut hash_table_iterator) -> libc::c_int;
    fn c_strcasecmp(s1: *const libc::c_char, s2: *const libc::c_char) -> libc::c_int;
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
pub type __int64_t = libc::c_long;
pub type __dev_t = libc::c_ulong;
pub type __uid_t = libc::c_uint;
pub type __gid_t = libc::c_uint;
pub type __ino_t = libc::c_ulong;
pub type __mode_t = libc::c_uint;
pub type __nlink_t = libc::c_ulong;
pub type __off_t = libc::c_long;
pub type __off64_t = libc::c_long;
pub type __pid_t = libc::c_int;
pub type __time_t = libc::c_long;
pub type __useconds_t = libc::c_uint;
pub type __suseconds_t = libc::c_long;
pub type __blksize_t = libc::c_long;
pub type __blkcnt_t = libc::c_long;
pub type __ssize_t = libc::c_long;
pub type __syscall_slong_t = libc::c_long;
pub type ino_t = __ino_t;
pub type dev_t = __dev_t;
pub type mode_t = __mode_t;
pub type off_t = __off_t;
pub type pid_t = __pid_t;
pub type ssize_t = __ssize_t;
pub type time_t = __time_t;
pub type size_t = libc::c_ulong;
pub type int64_t = __int64_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __sigset_t {
    pub __val: [libc::c_ulong; 16],
}
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
pub struct lconv {
    pub decimal_point: *mut libc::c_char,
    pub thousands_sep: *mut libc::c_char,
    pub grouping: *mut libc::c_char,
    pub int_curr_symbol: *mut libc::c_char,
    pub currency_symbol: *mut libc::c_char,
    pub mon_decimal_point: *mut libc::c_char,
    pub mon_thousands_sep: *mut libc::c_char,
    pub mon_grouping: *mut libc::c_char,
    pub positive_sign: *mut libc::c_char,
    pub negative_sign: *mut libc::c_char,
    pub int_frac_digits: libc::c_char,
    pub frac_digits: libc::c_char,
    pub p_cs_precedes: libc::c_char,
    pub p_sep_by_space: libc::c_char,
    pub n_cs_precedes: libc::c_char,
    pub n_sep_by_space: libc::c_char,
    pub p_sign_posn: libc::c_char,
    pub n_sign_posn: libc::c_char,
    pub int_p_cs_precedes: libc::c_char,
    pub int_p_sep_by_space: libc::c_char,
    pub int_n_cs_precedes: libc::c_char,
    pub int_n_sep_by_space: libc::c_char,
    pub int_p_sign_posn: libc::c_char,
    pub int_n_sign_posn: libc::c_char,
}
pub type wgint = int64_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct options {
    pub verbose: libc::c_int,
    pub quiet: bool,
    pub ntry: libc::c_int,
    pub retry_connrefused: bool,
    pub retry_on_host_error: bool,
    pub retry_on_http_error: *mut libc::c_char,
    pub background: bool,
    pub ignore_length: bool,
    pub recursive: bool,
    pub spanhost: bool,
    pub max_redirect: libc::c_int,
    pub relative_only: bool,
    pub no_parent: bool,
    pub reclevel: libc::c_int,
    pub dirstruct: bool,
    pub no_dirstruct: bool,
    pub cut_dirs: libc::c_int,
    pub add_hostdir: bool,
    pub protocol_directories: bool,
    pub noclobber: bool,
    pub unlink_requested: bool,
    pub dir_prefix: *mut libc::c_char,
    pub lfilename: *mut libc::c_char,
    pub input_filename: *mut libc::c_char,
    pub choose_config: *mut libc::c_char,
    pub noconfig: bool,
    pub force_html: bool,
    pub default_page: *mut libc::c_char,
    pub spider: bool,
    pub accepts: *mut *mut libc::c_char,
    pub rejects: *mut *mut libc::c_char,
    pub excludes: *mut *const libc::c_char,
    pub includes: *mut *const libc::c_char,
    pub ignore_case: bool,
    pub acceptregex_s: *mut libc::c_char,
    pub rejectregex_s: *mut libc::c_char,
    pub acceptregex: *mut libc::c_void,
    pub rejectregex: *mut libc::c_void,
    pub regex_type: C2RustUnnamed_2,
    pub regex_compile_fun: Option::<
        unsafe extern "C" fn(*const libc::c_char) -> *mut libc::c_void,
    >,
    pub regex_match_fun: Option::<
        unsafe extern "C" fn(*const libc::c_void, *const libc::c_char) -> bool,
    >,
    pub domains: *mut *mut libc::c_char,
    pub exclude_domains: *mut *mut libc::c_char,
    pub dns_cache: bool,
    pub follow_tags: *mut *mut libc::c_char,
    pub ignore_tags: *mut *mut libc::c_char,
    pub follow_ftp: bool,
    pub retr_symlinks: bool,
    pub output_document: *mut libc::c_char,
    pub warc_filename: *mut libc::c_char,
    pub warc_tempdir: *mut libc::c_char,
    pub warc_cdx_dedup_filename: *mut libc::c_char,
    pub warc_maxsize: wgint,
    pub warc_compression_enabled: bool,
    pub warc_digests_enabled: bool,
    pub warc_cdx_enabled: bool,
    pub warc_keep_log: bool,
    pub warc_user_headers: *mut *mut libc::c_char,
    pub enable_xattr: bool,
    pub user: *mut libc::c_char,
    pub passwd: *mut libc::c_char,
    pub ask_passwd: bool,
    pub use_askpass: *mut libc::c_char,
    pub always_rest: bool,
    pub start_pos: wgint,
    pub ftp_user: *mut libc::c_char,
    pub ftp_passwd: *mut libc::c_char,
    pub netrc: bool,
    pub ftp_glob: bool,
    pub ftp_pasv: bool,
    pub http_user: *mut libc::c_char,
    pub http_passwd: *mut libc::c_char,
    pub user_headers: *mut *mut libc::c_char,
    pub http_keep_alive: bool,
    pub use_proxy: bool,
    pub allow_cache: bool,
    pub http_proxy: *mut libc::c_char,
    pub ftp_proxy: *mut libc::c_char,
    pub https_proxy: *mut libc::c_char,
    pub no_proxy: *mut *mut libc::c_char,
    pub base_href: *mut libc::c_char,
    pub progress_type: *mut libc::c_char,
    pub show_progress: libc::c_int,
    pub noscroll: bool,
    pub proxy_user: *mut libc::c_char,
    pub proxy_passwd: *mut libc::c_char,
    pub read_timeout: libc::c_double,
    pub dns_timeout: libc::c_double,
    pub connect_timeout: libc::c_double,
    pub random_wait: bool,
    pub wait: libc::c_double,
    pub waitretry: libc::c_double,
    pub use_robots: bool,
    pub limit_rate: wgint,
    pub quota: wgint,
    pub server_response: bool,
    pub save_headers: bool,
    pub content_on_error: bool,
    pub debug: bool,
    pub timestamping: bool,
    pub if_modified_since: bool,
    pub backup_converted: bool,
    pub backups: libc::c_int,
    pub useragent: *mut libc::c_char,
    pub referer: *mut libc::c_char,
    pub convert_links: bool,
    pub convert_file_only: bool,
    pub remove_listing: bool,
    pub htmlify: bool,
    pub dot_style: *mut libc::c_char,
    pub dot_bytes: wgint,
    pub dots_in_line: libc::c_int,
    pub dot_spacing: libc::c_int,
    pub delete_after: bool,
    pub adjust_extension: bool,
    pub page_requisites: bool,
    pub bind_address: *mut libc::c_char,
    pub cookies: bool,
    pub cookies_input: *mut libc::c_char,
    pub cookies_output: *mut libc::c_char,
    pub keep_badhash: bool,
    pub keep_session_cookies: bool,
    pub post_data: *mut libc::c_char,
    pub post_file_name: *mut libc::c_char,
    pub method: *mut libc::c_char,
    pub body_data: *mut libc::c_char,
    pub body_file: *mut libc::c_char,
    pub restrict_files_os: C2RustUnnamed_1,
    pub restrict_files_ctrl: bool,
    pub restrict_files_nonascii: bool,
    pub restrict_files_case: C2RustUnnamed_0,
    pub strict_comments: bool,
    pub preserve_perm: bool,
    pub ipv4_only: bool,
    pub ipv6_only: bool,
    pub prefer_family: C2RustUnnamed,
    pub content_disposition: bool,
    pub auth_without_challenge: bool,
    pub enable_iri: bool,
    pub encoding_remote: *mut libc::c_char,
    pub locale: *const libc::c_char,
    pub trustservernames: bool,
    pub useservertimestamps: bool,
    pub show_all_dns_entries: bool,
    pub report_bps: bool,
    pub compression: compression_options,
    pub rejected_log: *mut libc::c_char,
    pub homedir: *const libc::c_char,
    pub wgetrcfile: *const libc::c_char,
}
pub type compression_options = libc::c_uint;
pub const compression_none: compression_options = 2;
pub const compression_gzip: compression_options = 1;
pub const compression_auto: compression_options = 0;
pub type C2RustUnnamed = libc::c_uint;
pub const prefer_none: C2RustUnnamed = 2;
pub const prefer_ipv6: C2RustUnnamed = 1;
pub const prefer_ipv4: C2RustUnnamed = 0;
pub type C2RustUnnamed_0 = libc::c_uint;
pub const restrict_uppercase: C2RustUnnamed_0 = 2;
pub const restrict_lowercase: C2RustUnnamed_0 = 1;
pub const restrict_no_case_restriction: C2RustUnnamed_0 = 0;
pub type C2RustUnnamed_1 = libc::c_uint;
pub const restrict_windows: C2RustUnnamed_1 = 2;
pub const restrict_vms: C2RustUnnamed_1 = 1;
pub const restrict_unix: C2RustUnnamed_1 = 0;
pub type C2RustUnnamed_2 = libc::c_uint;
pub const regex_type_posix: C2RustUnnamed_2 = 1;
pub const regex_type_pcre: C2RustUnnamed_2 = 0;
pub type va_list = __builtin_va_list;
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
pub type log_options = libc::c_uint;
pub const LOG_PROGRESS: log_options = 4;
pub const LOG_ALWAYS: log_options = 3;
pub const LOG_NONVERBOSE: log_options = 2;
pub const LOG_NOTQUIET: log_options = 1;
pub const LOG_VERBOSE: log_options = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct utimbuf {
    pub actime: __time_t,
    pub modtime: __time_t,
}
pub type __itimer_which = libc::c_uint;
pub const ITIMER_PROF: __itimer_which = 2;
pub const ITIMER_VIRTUAL: __itimer_which = 1;
pub const ITIMER_REAL: __itimer_which = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct itimerval {
    pub it_interval: timeval,
    pub it_value: timeval,
}
pub type __itimer_which_t = __itimer_which;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct winsize {
    pub ws_row: libc::c_ushort,
    pub ws_col: libc::c_ushort,
    pub ws_xpixel: libc::c_ushort,
    pub ws_ypixel: libc::c_ushort,
}
pub type __sighandler_t = Option::<unsafe extern "C" fn(libc::c_int) -> ()>;
pub type __jmp_buf = [libc::c_long; 8];
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __jmp_buf_tag {
    pub __jmpbuf: __jmp_buf,
    pub __mask_was_saved: libc::c_int,
    pub __saved_mask: __sigset_t,
}
pub type sigjmp_buf = [__jmp_buf_tag; 1];
pub type __re_long_size_t = size_t;
pub type reg_syntax_t = libc::c_ulong;
pub type C2RustUnnamed_3 = libc::c_int;
pub const _REG_ERPAREN: C2RustUnnamed_3 = 16;
pub const _REG_ESIZE: C2RustUnnamed_3 = 15;
pub const _REG_EEND: C2RustUnnamed_3 = 14;
pub const _REG_BADRPT: C2RustUnnamed_3 = 13;
pub const _REG_ESPACE: C2RustUnnamed_3 = 12;
pub const _REG_ERANGE: C2RustUnnamed_3 = 11;
pub const _REG_BADBR: C2RustUnnamed_3 = 10;
pub const _REG_EBRACE: C2RustUnnamed_3 = 9;
pub const _REG_EPAREN: C2RustUnnamed_3 = 8;
pub const _REG_EBRACK: C2RustUnnamed_3 = 7;
pub const _REG_ESUBREG: C2RustUnnamed_3 = 6;
pub const _REG_EESCAPE: C2RustUnnamed_3 = 5;
pub const _REG_ECTYPE: C2RustUnnamed_3 = 4;
pub const _REG_ECOLLATE: C2RustUnnamed_3 = 3;
pub const _REG_BADPAT: C2RustUnnamed_3 = 2;
pub const _REG_NOMATCH: C2RustUnnamed_3 = 1;
pub const _REG_NOERROR: C2RustUnnamed_3 = 0;
pub const _REG_ENOSYS: C2RustUnnamed_3 = -1;
#[derive(Copy, Clone, BitfieldStruct)]
#[repr(C)]
pub struct re_pattern_buffer {
    pub buffer: *mut re_dfa_t,
    pub allocated: __re_long_size_t,
    pub used: __re_long_size_t,
    pub syntax: reg_syntax_t,
    pub fastmap: *mut libc::c_char,
    pub translate: *mut libc::c_uchar,
    pub re_nsub: size_t,
    #[bitfield(name = "can_be_null", ty = "libc::c_uint", bits = "0..=0")]
    #[bitfield(name = "regs_allocated", ty = "libc::c_uint", bits = "1..=2")]
    #[bitfield(name = "fastmap_accurate", ty = "libc::c_uint", bits = "3..=3")]
    #[bitfield(name = "no_sub", ty = "libc::c_uint", bits = "4..=4")]
    #[bitfield(name = "not_bol", ty = "libc::c_uint", bits = "5..=5")]
    #[bitfield(name = "not_eol", ty = "libc::c_uint", bits = "6..=6")]
    #[bitfield(name = "newline_anchor", ty = "libc::c_uint", bits = "7..=7")]
    pub can_be_null_regs_allocated_fastmap_accurate_no_sub_not_bol_not_eol_newline_anchor: [u8; 1],
    #[bitfield(padding)]
    pub c2rust_padding: [u8; 7],
}
pub type regex_t = re_pattern_buffer;
pub type regoff_t = ssize_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct regmatch_t {
    pub rm_so: regoff_t,
    pub rm_eo: regoff_t,
}
pub type pcre = real_pcre;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct pcre_extra {
    pub flags: libc::c_ulong,
    pub study_data: *mut libc::c_void,
    pub match_limit: libc::c_ulong,
    pub callout_data: *mut libc::c_void,
    pub tables: *const libc::c_uchar,
    pub match_limit_recursion: libc::c_ulong,
    pub mark: *mut *mut libc::c_uchar,
    pub executable_jit: *mut libc::c_void,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct file_memory {
    pub content: *mut libc::c_char,
    pub length: libc::c_long,
    pub mmap_p: libc::c_int,
}
pub const WGET_EXIT_SUCCESS: C2RustUnnamed_4 = 0;
pub const WGET_EXIT_GENERIC_ERROR: C2RustUnnamed_4 = 1;
pub type file_stats_t = file_stat_s;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct file_stat_s {
    pub access_err: libc::c_int,
    pub st_ino: ino_t,
    pub st_dev: dev_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct hash_table_iterator {
    pub key: *mut libc::c_void,
    pub value: *mut libc::c_void,
    pub pos: *mut libc::c_void,
    pub end: *mut libc::c_void,
}
pub type C2RustUnnamed_4 = libc::c_uint;
pub const WGET_EXIT_UNKNOWN: C2RustUnnamed_4 = 9;
pub const WGET_EXIT_SERVER_ERROR: C2RustUnnamed_4 = 8;
pub const WGET_EXIT_PROTOCOL_ERROR: C2RustUnnamed_4 = 7;
pub const WGET_EXIT_SERVER_AUTH_FAIL: C2RustUnnamed_4 = 6;
pub const WGET_EXIT_SSL_AUTH_FAIL: C2RustUnnamed_4 = 5;
pub const WGET_EXIT_NETWORK_FAIL: C2RustUnnamed_4 = 4;
pub const WGET_EXIT_IO_FAIL: C2RustUnnamed_4 = 3;
pub const WGET_EXIT_PARSE_ERROR: C2RustUnnamed_4 = 2;
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
unsafe extern "C" fn lstat(
    mut __path: *const libc::c_char,
    mut __statbuf: *mut stat,
) -> libc::c_int {
    return __lxstat(1 as libc::c_int, __path, __statbuf);
}
#[inline]
unsafe extern "C" fn c_isspace(mut c: libc::c_int) -> bool {
    match c {
        32 | 9 | 10 | 11 | 12 | 13 => return 1 as libc::c_int != 0,
        _ => return 0 as libc::c_int != 0,
    };
}
#[inline]
unsafe extern "C" fn c_tolower(mut c: libc::c_int) -> libc::c_int {
    match c {
        65 | 66 | 67 | 68 | 69 | 70 | 71 | 72 | 73 | 74 | 75 | 76 | 77 | 78 | 79 | 80
        | 81 | 82 | 83 | 84 | 85 | 86 | 87 | 88 | 89 | 90 => {
            return c - 'A' as i32 + 'a' as i32;
        }
        _ => return c,
    };
}
unsafe extern "C" fn memfatal(
    mut context: *const libc::c_char,
    mut attempted_size: libc::c_long,
) {
    log_set_save_context(0 as libc::c_int != 0);
    if attempted_size == -(3 as libc::c_int) as libc::c_long {
        logprintf(
            LOG_ALWAYS,
            dcgettext(
                0 as *const libc::c_char,
                b"%s: %s: Failed to allocate enough memory; memory exhausted.\n\0"
                    as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ),
            exec_name,
            context,
        );
    } else {
        logprintf(
            LOG_ALWAYS,
            dcgettext(
                0 as *const libc::c_char,
                b"%s: %s: Failed to allocate %ld bytes; memory exhausted.\n\0"
                    as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ),
            exec_name,
            context,
            attempted_size,
        );
    }
    exit(WGET_EXIT_GENERIC_ERROR as libc::c_int);
}
pub static mut char_prop: [libc::c_uchar; 256] = [
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    2 as libc::c_int as libc::c_uchar,
    1 as libc::c_int as libc::c_uchar,
    1 as libc::c_int as libc::c_uchar,
    1 as libc::c_int as libc::c_uchar,
    16 as libc::c_int as libc::c_uchar,
    1 as libc::c_int as libc::c_uchar,
    1 as libc::c_int as libc::c_uchar,
    1 as libc::c_int as libc::c_uchar,
    1 as libc::c_int as libc::c_uchar,
    1 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    1 as libc::c_int as libc::c_uchar,
    1 as libc::c_int as libc::c_uchar,
    16 as libc::c_int as libc::c_uchar,
    4 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    80 as libc::c_int as libc::c_uchar,
    80 as libc::c_int as libc::c_uchar,
    80 as libc::c_int as libc::c_uchar,
    80 as libc::c_int as libc::c_uchar,
    80 as libc::c_int as libc::c_uchar,
    80 as libc::c_int as libc::c_uchar,
    80 as libc::c_int as libc::c_uchar,
    80 as libc::c_int as libc::c_uchar,
    80 as libc::c_int as libc::c_uchar,
    80 as libc::c_int as libc::c_uchar,
    1 as libc::c_int as libc::c_uchar,
    1 as libc::c_int as libc::c_uchar,
    1 as libc::c_int as libc::c_uchar,
    1 as libc::c_int as libc::c_uchar,
    1 as libc::c_int as libc::c_uchar,
    1 as libc::c_int as libc::c_uchar,
    1 as libc::c_int as libc::c_uchar,
    80 as libc::c_int as libc::c_uchar,
    80 as libc::c_int as libc::c_uchar,
    80 as libc::c_int as libc::c_uchar,
    80 as libc::c_int as libc::c_uchar,
    80 as libc::c_int as libc::c_uchar,
    80 as libc::c_int as libc::c_uchar,
    16 as libc::c_int as libc::c_uchar,
    16 as libc::c_int as libc::c_uchar,
    16 as libc::c_int as libc::c_uchar,
    16 as libc::c_int as libc::c_uchar,
    16 as libc::c_int as libc::c_uchar,
    16 as libc::c_int as libc::c_uchar,
    16 as libc::c_int as libc::c_uchar,
    16 as libc::c_int as libc::c_uchar,
    16 as libc::c_int as libc::c_uchar,
    16 as libc::c_int as libc::c_uchar,
    16 as libc::c_int as libc::c_uchar,
    16 as libc::c_int as libc::c_uchar,
    16 as libc::c_int as libc::c_uchar,
    16 as libc::c_int as libc::c_uchar,
    16 as libc::c_int as libc::c_uchar,
    16 as libc::c_int as libc::c_uchar,
    16 as libc::c_int as libc::c_uchar,
    16 as libc::c_int as libc::c_uchar,
    16 as libc::c_int as libc::c_uchar,
    16 as libc::c_int as libc::c_uchar,
    1 as libc::c_int as libc::c_uchar,
    1 as libc::c_int as libc::c_uchar,
    1 as libc::c_int as libc::c_uchar,
    1 as libc::c_int as libc::c_uchar,
    16 as libc::c_int as libc::c_uchar,
    1 as libc::c_int as libc::c_uchar,
    96 as libc::c_int as libc::c_uchar,
    96 as libc::c_int as libc::c_uchar,
    96 as libc::c_int as libc::c_uchar,
    96 as libc::c_int as libc::c_uchar,
    96 as libc::c_int as libc::c_uchar,
    96 as libc::c_int as libc::c_uchar,
    32 as libc::c_int as libc::c_uchar,
    32 as libc::c_int as libc::c_uchar,
    32 as libc::c_int as libc::c_uchar,
    32 as libc::c_int as libc::c_uchar,
    32 as libc::c_int as libc::c_uchar,
    32 as libc::c_int as libc::c_uchar,
    32 as libc::c_int as libc::c_uchar,
    32 as libc::c_int as libc::c_uchar,
    32 as libc::c_int as libc::c_uchar,
    32 as libc::c_int as libc::c_uchar,
    32 as libc::c_int as libc::c_uchar,
    32 as libc::c_int as libc::c_uchar,
    32 as libc::c_int as libc::c_uchar,
    32 as libc::c_int as libc::c_uchar,
    32 as libc::c_int as libc::c_uchar,
    32 as libc::c_int as libc::c_uchar,
    32 as libc::c_int as libc::c_uchar,
    32 as libc::c_int as libc::c_uchar,
    32 as libc::c_int as libc::c_uchar,
    32 as libc::c_int as libc::c_uchar,
    1 as libc::c_int as libc::c_uchar,
    1 as libc::c_int as libc::c_uchar,
    1 as libc::c_int as libc::c_uchar,
    17 as libc::c_int as libc::c_uchar,
    8 as libc::c_int as libc::c_uchar,
    8 as libc::c_int as libc::c_uchar,
    8 as libc::c_int as libc::c_uchar,
    8 as libc::c_int as libc::c_uchar,
    8 as libc::c_int as libc::c_uchar,
    8 as libc::c_int as libc::c_uchar,
    8 as libc::c_int as libc::c_uchar,
    8 as libc::c_int as libc::c_uchar,
    8 as libc::c_int as libc::c_uchar,
    8 as libc::c_int as libc::c_uchar,
    8 as libc::c_int as libc::c_uchar,
    8 as libc::c_int as libc::c_uchar,
    8 as libc::c_int as libc::c_uchar,
    8 as libc::c_int as libc::c_uchar,
    8 as libc::c_int as libc::c_uchar,
    8 as libc::c_int as libc::c_uchar,
    8 as libc::c_int as libc::c_uchar,
    8 as libc::c_int as libc::c_uchar,
    8 as libc::c_int as libc::c_uchar,
    8 as libc::c_int as libc::c_uchar,
    8 as libc::c_int as libc::c_uchar,
    8 as libc::c_int as libc::c_uchar,
    8 as libc::c_int as libc::c_uchar,
    8 as libc::c_int as libc::c_uchar,
    8 as libc::c_int as libc::c_uchar,
    8 as libc::c_int as libc::c_uchar,
    8 as libc::c_int as libc::c_uchar,
    8 as libc::c_int as libc::c_uchar,
    8 as libc::c_int as libc::c_uchar,
    8 as libc::c_int as libc::c_uchar,
    8 as libc::c_int as libc::c_uchar,
    8 as libc::c_int as libc::c_uchar,
    8 as libc::c_int as libc::c_uchar,
    8 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    8 as libc::c_int as libc::c_uchar,
];
pub unsafe extern "C" fn xstrdup_lower(mut s: *const libc::c_char) -> *mut libc::c_char {
    let mut copy: *mut libc::c_char = xstrdup(s);
    let mut p: *mut libc::c_char = copy;
    while *p != 0 {
        *p = c_tolower(*p as libc::c_int) as libc::c_char;
        p = p.offset(1);
        p;
    }
    return copy;
}
pub unsafe extern "C" fn strdupdelim(
    mut beg: *const libc::c_char,
    mut end: *const libc::c_char,
) -> *mut libc::c_char {
    if !beg.is_null() && beg <= end {
        let mut res: *mut libc::c_char = xmalloc(
            (end.offset_from(beg) as libc::c_long + 1 as libc::c_int as libc::c_long)
                as size_t,
        ) as *mut libc::c_char;
        memcpy(
            res as *mut libc::c_void,
            beg as *const libc::c_void,
            end.offset_from(beg) as libc::c_long as libc::c_ulong,
        );
        *res
            .offset(
                end.offset_from(beg) as libc::c_long as isize,
            ) = '\0' as i32 as libc::c_char;
        return res;
    }
    return xstrdup(b"\0" as *const u8 as *const libc::c_char);
}
pub unsafe extern "C" fn sepstring(
    mut s: *const libc::c_char,
) -> *mut *mut libc::c_char {
    let mut res: *mut *mut libc::c_char = 0 as *mut *mut libc::c_char;
    let mut p: *const libc::c_char = 0 as *const libc::c_char;
    let mut i: libc::c_int = 0 as libc::c_int;
    if s.is_null() || *s == 0 {
        return 0 as *mut *mut libc::c_char;
    }
    res = 0 as *mut *mut libc::c_char;
    p = s;
    while *s != 0 {
        if *s as libc::c_int == ',' as i32 {
            res = xrealloc(
                res as *mut libc::c_void,
                ((i + 2 as libc::c_int) as libc::c_ulong)
                    .wrapping_mul(
                        ::std::mem::size_of::<*mut libc::c_char>() as libc::c_ulong,
                    ),
            ) as *mut *mut libc::c_char;
            let ref mut fresh0 = *res.offset(i as isize);
            *fresh0 = strdupdelim(p, s);
            i += 1;
            let ref mut fresh1 = *res.offset(i as isize);
            *fresh1 = 0 as *mut libc::c_char;
            s = s.offset(1);
            s;
            while c_isspace(*s as libc::c_int) {
                s = s.offset(1);
                s;
            }
            p = s;
        } else {
            s = s.offset(1);
            s;
        }
    }
    res = xrealloc(
        res as *mut libc::c_void,
        ((i + 2 as libc::c_int) as libc::c_ulong)
            .wrapping_mul(::std::mem::size_of::<*mut libc::c_char>() as libc::c_ulong),
    ) as *mut *mut libc::c_char;
    let ref mut fresh2 = *res.offset(i as isize);
    *fresh2 = strdupdelim(p, s);
    let ref mut fresh3 = *res.offset((i + 1 as libc::c_int) as isize);
    *fresh3 = 0 as *mut libc::c_char;
    return res;
}
pub unsafe extern "C" fn aprintf(
    mut fmt: *const libc::c_char,
    mut args: ...
) -> *mut libc::c_char {
    let mut ret: libc::c_int = 0;
    let mut args_0: ::std::ffi::VaListImpl;
    let mut str: *mut libc::c_char = 0 as *mut libc::c_char;
    args_0 = args.clone();
    ret = vasprintf(&mut str, fmt, args_0.as_va_list());
    if ret < 0 as libc::c_int && *__errno_location() == 12 as libc::c_int {
        memfatal(
            b"aprintf\0" as *const u8 as *const libc::c_char,
            -(3 as libc::c_int) as libc::c_long,
        );
    } else if ret < 0 as libc::c_int {
        return 0 as *mut libc::c_char
    }
    return str;
}
pub unsafe extern "C" fn strlcpy(
    mut dst: *mut libc::c_char,
    mut src: *const libc::c_char,
    mut size: size_t,
) -> size_t {
    let mut old: *const libc::c_char = src;
    if size != 0 {
        loop {
            size = size.wrapping_sub(1);
            if !(size != 0) {
                break;
            }
            let fresh4 = src;
            src = src.offset(1);
            let fresh5 = dst;
            dst = dst.offset(1);
            *fresh5 = *fresh4;
            if *fresh5 == 0 {
                return (src.offset_from(old) as libc::c_long
                    - 1 as libc::c_int as libc::c_long) as size_t;
            }
        }
        *dst = 0 as libc::c_int as libc::c_char;
    }
    loop {
        let fresh6 = src;
        src = src.offset(1);
        if !(*fresh6 != 0) {
            break;
        }
    }
    return (src.offset_from(old) as libc::c_long - 1 as libc::c_int as libc::c_long)
        as size_t;
}
pub unsafe extern "C" fn concat_strings(
    mut str0: *const libc::c_char,
    mut args: ...
) -> *mut libc::c_char {
    let mut args_0: ::std::ffi::VaListImpl;
    let mut arg: *const libc::c_char = 0 as *const libc::c_char;
    let mut length: size_t = 0 as libc::c_int as size_t;
    let mut pos: size_t = 0 as libc::c_int as size_t;
    let mut s: *mut libc::c_char = 0 as *mut libc::c_char;
    if str0.is_null() {
        return 0 as *mut libc::c_char;
    }
    args_0 = args.clone();
    arg = str0;
    while !arg.is_null() {
        length = (length as libc::c_ulong).wrapping_add(strlen(arg)) as size_t as size_t;
        arg = args_0.arg::<*const libc::c_char>();
    }
    s = xmalloc(length.wrapping_add(1 as libc::c_int as libc::c_ulong))
        as *mut libc::c_char;
    args_0 = args.clone();
    arg = str0;
    while !arg.is_null() {
        pos = (pos as libc::c_ulong)
            .wrapping_add(
                strlcpy(
                    s.offset(pos as isize),
                    arg,
                    length
                        .wrapping_sub(pos)
                        .wrapping_add(1 as libc::c_int as libc::c_ulong),
                ),
            ) as size_t as size_t;
        arg = args_0.arg::<*const libc::c_char>();
    }
    return s;
}
unsafe extern "C" fn fmttime(
    mut t: time_t,
    mut fmt: *const libc::c_char,
) -> *mut libc::c_char {
    static mut output: [libc::c_char; 32] = [0; 32];
    let mut tm: *mut tm = localtime(&mut t);
    if tm.is_null() {
        abort();
    }
    if strftime(
        output.as_mut_ptr(),
        ::std::mem::size_of::<[libc::c_char; 32]>() as libc::c_ulong,
        fmt,
        tm,
    ) == 0
    {
        abort();
    }
    return output.as_mut_ptr();
}
pub unsafe extern "C" fn time_str(mut t: time_t) -> *mut libc::c_char {
    return fmttime(t, b"%H:%M:%S\0" as *const u8 as *const libc::c_char);
}
pub unsafe extern "C" fn datetime_str(mut t: time_t) -> *mut libc::c_char {
    return fmttime(t, b"%Y-%m-%d %H:%M:%S\0" as *const u8 as *const libc::c_char);
}
pub unsafe extern "C" fn fork_to_background() -> bool {
    let mut pid: pid_t = 0;
    let mut logfile_changed: bool = 0 as libc::c_int != 0;
    if (opt.lfilename).is_null()
        && (!opt.quiet || opt.server_response as libc::c_int != 0)
    {
        let mut new_log_fp: *mut FILE = unique_create(
            b"wget-log\0" as *const u8 as *const libc::c_char,
            0 as libc::c_int != 0,
            &mut opt.lfilename,
        );
        if !new_log_fp.is_null() {
            logfile_changed = 1 as libc::c_int != 0;
            fclose(new_log_fp);
        }
    }
    pid = fork();
    if pid < 0 as libc::c_int {
        perror(b"fork\0" as *const u8 as *const libc::c_char);
        exit(WGET_EXIT_GENERIC_ERROR as libc::c_int);
    } else if pid != 0 as libc::c_int {
        printf(
            dcgettext(
                0 as *const libc::c_char,
                b"Continuing in background, pid %d.\n\0" as *const u8
                    as *const libc::c_char,
                5 as libc::c_int,
            ),
            pid,
        );
        if logfile_changed {
            printf(
                dcgettext(
                    0 as *const libc::c_char,
                    b"Output will be written to %s.\n\0" as *const u8
                        as *const libc::c_char,
                    5 as libc::c_int,
                ),
                quote(opt.lfilename),
            );
        }
        exit(WGET_EXIT_SUCCESS as libc::c_int);
    }
    setsid();
    if (freopen(
        b"/dev/null\0" as *const u8 as *const libc::c_char,
        b"r\0" as *const u8 as *const libc::c_char,
        stdin,
    ))
        .is_null()
    {
        if opt.debug as libc::c_long != 0 {
            debug_logprintf(
                b"Failed to redirect stdin to /dev/null.\n\0" as *const u8
                    as *const libc::c_char,
            );
        }
    }
    if (freopen(
        b"/dev/null\0" as *const u8 as *const libc::c_char,
        b"w\0" as *const u8 as *const libc::c_char,
        stdout,
    ))
        .is_null()
    {
        if opt.debug as libc::c_long != 0 {
            debug_logprintf(
                b"Failed to redirect stdout to /dev/null.\n\0" as *const u8
                    as *const libc::c_char,
            );
        }
    }
    if (freopen(
        b"/dev/null\0" as *const u8 as *const libc::c_char,
        b"w\0" as *const u8 as *const libc::c_char,
        stderr,
    ))
        .is_null()
    {
        if opt.debug as libc::c_long != 0 {
            debug_logprintf(
                b"Failed to redirect stderr to /dev/null.\n\0" as *const u8
                    as *const libc::c_char,
            );
        }
    }
    return logfile_changed;
}
pub unsafe extern "C" fn touch(mut file: *const libc::c_char, mut tm: time_t) {
    let mut times: utimbuf = utimbuf { actime: 0, modtime: 0 };
    times.modtime = tm;
    times.actime = time(0 as *mut time_t);
    if utime(file, &mut times) == -(1 as libc::c_int) {
        logprintf(
            LOG_NOTQUIET,
            b"utime(%s): %s\n\0" as *const u8 as *const libc::c_char,
            file,
            strerror(*__errno_location()),
        );
    }
}
pub unsafe extern "C" fn remove_link(mut file: *const libc::c_char) -> libc::c_int {
    let mut err: libc::c_int = 0 as libc::c_int;
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
    if lstat(file, &mut st) == 0 as libc::c_int
        && st.st_mode & 0o170000 as libc::c_int as libc::c_uint
            == 0o120000 as libc::c_int as libc::c_uint
    {
        if opt.debug as libc::c_long != 0 {
            debug_logprintf(
                b"Unlinking %s (symlink).\n\0" as *const u8 as *const libc::c_char,
                file,
            );
        }
        err = unlink(file);
        if err != 0 as libc::c_int {
            logprintf(
                LOG_VERBOSE,
                dcgettext(
                    0 as *const libc::c_char,
                    b"Failed to unlink symlink %s: %s\n\0" as *const u8
                        as *const libc::c_char,
                    5 as libc::c_int,
                ),
                quote(file),
                strerror(*__errno_location()),
            );
        }
    }
    return err;
}
pub unsafe extern "C" fn file_exists_p(
    mut filename: *const libc::c_char,
    mut fstats: *mut file_stats_t,
) -> bool {
    let mut buf: stat = stat {
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
    if filename.is_null() {
        return 0 as libc::c_int != 0;
    }
    *__errno_location() = 0 as libc::c_int;
    if stat(filename, &mut buf) == 0 as libc::c_int
        && buf.st_mode & 0o170000 as libc::c_int as libc::c_uint
            == 0o100000 as libc::c_int as libc::c_uint
        && (0o400 as libc::c_int as libc::c_uint & buf.st_mode != 0
            && getuid() == buf.st_uid
            || (0o400 as libc::c_int >> 3 as libc::c_int) as libc::c_uint & buf.st_mode
                != 0 && group_member(buf.st_gid) != 0
            || (0o400 as libc::c_int >> 3 as libc::c_int >> 3 as libc::c_int)
                as libc::c_uint & buf.st_mode != 0)
    {
        if !fstats.is_null() {
            (*fstats).access_err = 0 as libc::c_int;
            (*fstats).st_ino = buf.st_ino;
            (*fstats).st_dev = buf.st_dev;
        }
        return 1 as libc::c_int != 0;
    } else {
        if !fstats.is_null() {
            (*fstats)
                .access_err = if *__errno_location() == 0 as libc::c_int {
                13 as libc::c_int
            } else {
                *__errno_location()
            };
        }
        *__errno_location() = 0 as libc::c_int;
        return 0 as libc::c_int != 0;
    };
}
pub unsafe extern "C" fn file_non_directory_p(mut path: *const libc::c_char) -> bool {
    let mut buf: stat = stat {
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
    if lstat(path, &mut buf) != 0 as libc::c_int {
        return 0 as libc::c_int != 0;
    }
    return if buf.st_mode & 0o170000 as libc::c_int as libc::c_uint
        == 0o40000 as libc::c_int as libc::c_uint
    {
        0 as libc::c_int
    } else {
        1 as libc::c_int
    } != 0;
}
pub unsafe extern "C" fn file_size(mut filename: *const libc::c_char) -> wgint {
    let mut size: wgint = 0;
    let mut fp: *mut FILE = rpl_fopen(
        filename,
        b"rb\0" as *const u8 as *const libc::c_char,
    );
    if fp.is_null() {
        return -(1 as libc::c_int) as wgint;
    }
    rpl_fseeko(fp, 0 as libc::c_int as off_t, 2 as libc::c_int);
    size = ftello(fp);
    fclose(fp);
    return size;
}
unsafe extern "C" fn unique_name_1(
    mut prefix: *const libc::c_char,
) -> *mut libc::c_char {
    let mut count: libc::c_int = 1 as libc::c_int;
    let mut plen: libc::c_int = strlen(prefix) as libc::c_int;
    let mut template: *mut libc::c_char = xmalloc(
        (plen + 1 as libc::c_int + 24 as libc::c_int) as size_t,
    ) as *mut libc::c_char;
    let mut template_tail: *mut libc::c_char = template.offset(plen as isize);
    memcpy(
        template as *mut libc::c_void,
        prefix as *const libc::c_void,
        plen as libc::c_ulong,
    );
    let fresh7 = template_tail;
    template_tail = template_tail.offset(1);
    *fresh7 = '.' as i32 as libc::c_char;
    loop {
        let fresh8 = count;
        count = count + 1;
        number_to_string(template_tail, fresh8 as wgint);
        if !(file_exists_p(template, 0 as *mut file_stats_t) as libc::c_int != 0
            && count < 999999 as libc::c_int)
        {
            break;
        }
    }
    return template;
}
pub unsafe extern "C" fn unique_name_passthrough(
    mut file: *const libc::c_char,
) -> *mut libc::c_char {
    return if file_exists_p(file, 0 as *mut file_stats_t) as libc::c_int != 0 {
        unique_name_1(file)
    } else {
        file as *mut libc::c_char
    };
}
pub unsafe extern "C" fn unique_name(
    mut file: *const libc::c_char,
) -> *mut libc::c_char {
    return if file_exists_p(file, 0 as *mut file_stats_t) as libc::c_int != 0 {
        unique_name_1(file)
    } else {
        xstrdup(file)
    };
}
pub unsafe extern "C" fn unique_create(
    mut name: *const libc::c_char,
    mut binary: bool,
    mut opened_name: *mut *mut libc::c_char,
) -> *mut FILE {
    let mut uname: *mut libc::c_char = unique_name(name);
    let mut fp: *mut FILE = 0 as *mut FILE;
    loop {
        fp = fopen_excl(uname, binary as libc::c_int);
        if !(fp.is_null() && *__errno_location() == 17 as libc::c_int) {
            break;
        }
        rpl_free(uname as *mut libc::c_void);
        uname = 0 as *mut libc::c_char;
        uname = unique_name(name);
    }
    if !opened_name.is_null() {
        if !fp.is_null() {
            *opened_name = uname;
        } else {
            *opened_name = 0 as *mut libc::c_char;
            rpl_free(uname as *mut libc::c_void);
            uname = 0 as *mut libc::c_char;
        }
    } else {
        rpl_free(uname as *mut libc::c_void);
        uname = 0 as *mut libc::c_char;
    }
    return fp;
}
pub unsafe extern "C" fn fopen_excl(
    mut fname: *const libc::c_char,
    mut binary: libc::c_int,
) -> *mut FILE {
    let mut fd: libc::c_int = 0;
    let mut flags: libc::c_int = 0o1 as libc::c_int | 0o100 as libc::c_int
        | 0o200 as libc::c_int;
    if binary != 0 {
        flags |= 0 as libc::c_int;
    }
    fd = open(fname, flags, 0o666 as libc::c_int);
    if fd < 0 as libc::c_int {
        return 0 as *mut FILE;
    }
    return fdopen(
        fd,
        if binary != 0 {
            b"wb\0" as *const u8 as *const libc::c_char
        } else {
            b"w\0" as *const u8 as *const libc::c_char
        },
    );
}
pub unsafe extern "C" fn fopen_stat(
    mut fname: *const libc::c_char,
    mut mode: *const libc::c_char,
    mut fstats: *mut file_stats_t,
) -> *mut FILE {
    let mut fd: libc::c_int = 0;
    let mut fp: *mut FILE = 0 as *mut FILE;
    let mut fdstats: stat = stat {
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
    fp = rpl_fopen(fname, mode);
    if fp.is_null() {
        logprintf(
            LOG_NOTQUIET,
            dcgettext(
                0 as *const libc::c_char,
                b"Failed to Fopen file %s\n\0" as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ),
            fname,
        );
        return 0 as *mut FILE;
    }
    fd = fileno(fp);
    if fd < 0 as libc::c_int {
        logprintf(
            LOG_NOTQUIET,
            dcgettext(
                0 as *const libc::c_char,
                b"Failed to get FD for file %s\n\0" as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ),
            fname,
        );
        fclose(fp);
        return 0 as *mut FILE;
    }
    memset(
        &mut fdstats as *mut stat as *mut libc::c_void,
        0 as libc::c_int,
        ::std::mem::size_of::<stat>() as libc::c_ulong,
    );
    if fstat(fd, &mut fdstats) == -(1 as libc::c_int) {
        logprintf(
            LOG_NOTQUIET,
            dcgettext(
                0 as *const libc::c_char,
                b"Failed to stat file %s, (check permissions)\n\0" as *const u8
                    as *const libc::c_char,
                5 as libc::c_int,
            ),
            fname,
        );
        fclose(fp);
        return 0 as *mut FILE;
    }
    if !fstats.is_null()
        && (fdstats.st_dev != (*fstats).st_dev || fdstats.st_ino != (*fstats).st_ino)
    {
        logprintf(
            LOG_NOTQUIET,
            dcgettext(
                0 as *const libc::c_char,
                b"File %s changed since the last check. Security check failed.\n\0"
                    as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ),
            fname,
        );
        fclose(fp);
        return 0 as *mut FILE;
    }
    return fp;
}
pub unsafe extern "C" fn open_stat(
    mut fname: *const libc::c_char,
    mut flags: libc::c_int,
    mut mode: mode_t,
    mut fstats: *mut file_stats_t,
) -> libc::c_int {
    let mut fd: libc::c_int = 0;
    let mut fdstats: stat = stat {
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
    fd = open(fname, flags, mode);
    if fd < 0 as libc::c_int {
        logprintf(
            LOG_NOTQUIET,
            dcgettext(
                0 as *const libc::c_char,
                b"Failed to open file %s, reason :%s\n\0" as *const u8
                    as *const libc::c_char,
                5 as libc::c_int,
            ),
            fname,
            strerror(*__errno_location()),
        );
        return -(1 as libc::c_int);
    }
    memset(
        &mut fdstats as *mut stat as *mut libc::c_void,
        0 as libc::c_int,
        ::std::mem::size_of::<stat>() as libc::c_ulong,
    );
    if fstat(fd, &mut fdstats) == -(1 as libc::c_int) {
        logprintf(
            LOG_NOTQUIET,
            dcgettext(
                0 as *const libc::c_char,
                b"Failed to stat file %s, error: %s\n\0" as *const u8
                    as *const libc::c_char,
                5 as libc::c_int,
            ),
            fname,
            strerror(*__errno_location()),
        );
        close(fd);
        return -(1 as libc::c_int);
    }
    if !fstats.is_null()
        && (fdstats.st_dev != (*fstats).st_dev || fdstats.st_ino != (*fstats).st_ino)
    {
        logprintf(
            LOG_NOTQUIET,
            dcgettext(
                0 as *const libc::c_char,
                b"Trying to open file %s but it changed since last check. Security check failed.\n\0"
                    as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ),
            fname,
        );
        close(fd);
        return -(1 as libc::c_int);
    }
    return fd;
}
pub unsafe extern "C" fn make_directory(
    mut directory: *const libc::c_char,
) -> libc::c_int {
    let mut i: libc::c_int = 0;
    let mut ret: libc::c_int = 0;
    let mut quit: libc::c_int = 0 as libc::c_int;
    let mut buf: [libc::c_char; 1024] = [0; 1024];
    let mut dir: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut len: size_t = strlen(directory);
    if len < ::std::mem::size_of::<[libc::c_char; 1024]>() as libc::c_ulong {
        memcpy(
            buf.as_mut_ptr() as *mut libc::c_void,
            directory as *const libc::c_void,
            len.wrapping_add(1 as libc::c_int as libc::c_ulong),
        );
        dir = buf.as_mut_ptr();
    } else {
        dir = xstrdup(directory);
    }
    i = (*dir as libc::c_int == '/' as i32) as libc::c_int;
    loop {
        while *dir.offset(i as isize) as libc::c_int != 0
            && *dir.offset(i as isize) as libc::c_int != '/' as i32
        {
            i += 1;
            i;
        }
        if *dir.offset(i as isize) == 0 {
            quit = 1 as libc::c_int;
        }
        *dir.offset(i as isize) = '\0' as i32 as libc::c_char;
        if !file_exists_p(dir, 0 as *mut file_stats_t) {
            ret = mkdir(dir, 0o777 as libc::c_int as __mode_t);
        } else {
            ret = 0 as libc::c_int;
        }
        if quit != 0 {
            break;
        }
        *dir.offset(i as isize) = '/' as i32 as libc::c_char;
        i += 1;
        i;
    }
    if dir != buf.as_mut_ptr() {
        rpl_free(dir as *mut libc::c_void);
        dir = 0 as *mut libc::c_char;
    }
    return ret;
}
pub unsafe extern "C" fn file_merge(
    mut base: *const libc::c_char,
    mut file: *const libc::c_char,
) -> *mut libc::c_char {
    let mut result: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut cut: *const libc::c_char = strrchr(base, '/' as i32) as *const libc::c_char;
    if cut.is_null() {
        return xstrdup(file);
    }
    result = xmalloc(
        ((cut.offset_from(base) as libc::c_long + 1 as libc::c_int as libc::c_long)
            as libc::c_ulong)
            .wrapping_add(strlen(file))
            .wrapping_add(1 as libc::c_int as libc::c_ulong),
    ) as *mut libc::c_char;
    memcpy(
        result as *mut libc::c_void,
        base as *const libc::c_void,
        cut.offset_from(base) as libc::c_long as libc::c_ulong,
    );
    *result
        .offset(
            cut.offset_from(base) as libc::c_long as isize,
        ) = '/' as i32 as libc::c_char;
    strcpy(
        result
            .offset(cut.offset_from(base) as libc::c_long as isize)
            .offset(1 as libc::c_int as isize),
        file,
    );
    return result;
}
pub unsafe extern "C" fn fnmatch_nocase(
    mut pattern: *const libc::c_char,
    mut string: *const libc::c_char,
    mut flags: libc::c_int,
) -> libc::c_int {
    return fnmatch(pattern, string, flags | (1 as libc::c_int) << 4 as libc::c_int);
}
pub unsafe extern "C" fn acceptable(mut s: *const libc::c_char) -> bool {
    let mut p: *const libc::c_char = 0 as *const libc::c_char;
    if !(opt.output_document).is_null()
        && strcmp(s, opt.output_document) == 0 as libc::c_int
    {
        return 1 as libc::c_int != 0;
    }
    p = strrchr(s, '/' as i32);
    if !p.is_null() {
        s = p.offset(1 as libc::c_int as isize);
    }
    if !(opt.accepts).is_null() {
        if !(opt.rejects).is_null() {
            return in_acclist(
                opt.accepts as *const *const libc::c_char,
                s,
                1 as libc::c_int != 0,
            ) as libc::c_int != 0
                && !in_acclist(
                    opt.rejects as *const *const libc::c_char,
                    s,
                    1 as libc::c_int != 0,
                )
        } else {
            return in_acclist(
                opt.accepts as *const *const libc::c_char,
                s,
                1 as libc::c_int != 0,
            )
        }
    } else if !(opt.rejects).is_null() {
        return !in_acclist(
            opt.rejects as *const *const libc::c_char,
            s,
            1 as libc::c_int != 0,
        )
    }
    return 1 as libc::c_int != 0;
}
pub unsafe extern "C" fn accept_url(mut s: *const libc::c_char) -> bool {
    if !(opt.acceptregex).is_null()
        && !(opt.regex_match_fun).unwrap()(opt.acceptregex, s)
    {
        return 0 as libc::c_int != 0;
    }
    if !(opt.rejectregex).is_null()
        && (opt.regex_match_fun).unwrap()(opt.rejectregex, s) as libc::c_int != 0
    {
        return 0 as libc::c_int != 0;
    }
    return 1 as libc::c_int != 0;
}
pub unsafe extern "C" fn subdir_p(
    mut d1: *const libc::c_char,
    mut d2: *const libc::c_char,
) -> bool {
    if *d1 as libc::c_int == '\0' as i32 {
        return 1 as libc::c_int != 0;
    }
    if !opt.ignore_case {
        while *d1 as libc::c_int != 0 && *d2 as libc::c_int != 0
            && *d1 as libc::c_int == *d2 as libc::c_int
        {
            d1 = d1.offset(1);
            d1;
            d2 = d2.offset(1);
            d2;
        }
    } else {
        while *d1 as libc::c_int != 0 && *d2 as libc::c_int != 0
            && c_tolower(*d1 as libc::c_int) == c_tolower(*d2 as libc::c_int)
        {
            d1 = d1.offset(1);
            d1;
            d2 = d2.offset(1);
            d2;
        }
    }
    return *d1 as libc::c_int == '\0' as i32
        && (*d2 as libc::c_int == '\0' as i32 || *d2 as libc::c_int == '/' as i32);
}
unsafe extern "C" fn dir_matches_p(
    mut dirlist: *mut *const libc::c_char,
    mut dir: *const libc::c_char,
) -> bool {
    let mut x: *mut *const libc::c_char = 0 as *mut *const libc::c_char;
    let mut matcher: Option::<
        unsafe extern "C" fn(
            *const libc::c_char,
            *const libc::c_char,
            libc::c_int,
        ) -> libc::c_int,
    > = if opt.ignore_case as libc::c_int != 0 {
        Some(
            fnmatch_nocase
                as unsafe extern "C" fn(
                    *const libc::c_char,
                    *const libc::c_char,
                    libc::c_int,
                ) -> libc::c_int,
        )
    } else {
        Some(
            fnmatch
                as unsafe extern "C" fn(
                    *const libc::c_char,
                    *const libc::c_char,
                    libc::c_int,
                ) -> libc::c_int,
        )
    };
    x = dirlist;
    while !(*x).is_null() {
        let mut p: *const libc::c_char = (*x)
            .offset((**x as libc::c_int == '/' as i32) as libc::c_int as isize);
        if has_wildcards_p(p) {
            if matcher.unwrap()(p, dir, (1 as libc::c_int) << 0 as libc::c_int)
                == 0 as libc::c_int
            {
                break;
            }
        } else if subdir_p(p, dir) {
            break;
        }
        x = x.offset(1);
        x;
    }
    return if !(*x).is_null() { 1 as libc::c_int } else { 0 as libc::c_int } != 0;
}
pub unsafe extern "C" fn accdir(mut directory: *const libc::c_char) -> bool {
    if *directory as libc::c_int == '/' as i32 {
        directory = directory.offset(1);
        directory;
    }
    if !(opt.includes).is_null() {
        if !dir_matches_p(opt.includes, directory) {
            return 0 as libc::c_int != 0;
        }
    }
    if !(opt.excludes).is_null() {
        if dir_matches_p(opt.excludes, directory) {
            return 0 as libc::c_int != 0;
        }
    }
    return 1 as libc::c_int != 0;
}
pub unsafe extern "C" fn match_tail(
    mut string: *const libc::c_char,
    mut tail: *const libc::c_char,
    mut fold_case: bool,
) -> bool {
    let mut pos: libc::c_int = strlen(string) as libc::c_int
        - strlen(tail) as libc::c_int;
    if pos < 0 as libc::c_int {
        return 0 as libc::c_int != 0;
    }
    if !fold_case {
        return strcmp(string.offset(pos as isize), tail) == 0
    } else {
        return strcasecmp(string.offset(pos as isize), tail) == 0
    };
}
unsafe extern "C" fn in_acclist(
    mut accepts: *const *const libc::c_char,
    mut s: *const libc::c_char,
    mut backward: bool,
) -> bool {
    while !(*accepts).is_null() {
        if has_wildcards_p(*accepts) {
            let mut res: libc::c_int = if opt.ignore_case as libc::c_int != 0 {
                fnmatch_nocase(*accepts, s, 0 as libc::c_int)
            } else {
                fnmatch(*accepts, s, 0 as libc::c_int)
            };
            if res == 0 as libc::c_int {
                return 1 as libc::c_int != 0;
            }
        } else if backward {
            if match_tail(s, *accepts, opt.ignore_case) {
                return 1 as libc::c_int != 0;
            }
        } else {
            let mut cmp: libc::c_int = if opt.ignore_case as libc::c_int != 0 {
                strcasecmp(s, *accepts)
            } else {
                strcmp(s, *accepts)
            };
            if cmp == 0 as libc::c_int {
                return 1 as libc::c_int != 0;
            }
        }
        accepts = accepts.offset(1);
        accepts;
    }
    return 0 as libc::c_int != 0;
}
pub unsafe extern "C" fn suffix(mut str: *const libc::c_char) -> *mut libc::c_char {
    let mut p: *mut libc::c_char = 0 as *mut libc::c_char;
    p = strrchr(str, '.' as i32);
    if !p.is_null()
        && (strchr(p.offset(1 as libc::c_int as isize), '/' as i32)).is_null()
    {
        return p.offset(1 as libc::c_int as isize);
    }
    return 0 as *mut libc::c_char;
}
pub unsafe extern "C" fn has_wildcards_p(mut s: *const libc::c_char) -> bool {
    return !(strpbrk(s, b"*?[]\0" as *const u8 as *const libc::c_char)).is_null();
}
pub unsafe extern "C" fn has_html_suffix_p(mut fname: *const libc::c_char) -> bool {
    let mut suf: *mut libc::c_char = 0 as *mut libc::c_char;
    suf = suffix(fname);
    if suf.is_null() {
        return 0 as libc::c_int != 0;
    }
    if c_strcasecmp(suf, b"html\0" as *const u8 as *const libc::c_char) == 0 {
        return 1 as libc::c_int != 0;
    }
    if c_strcasecmp(suf, b"htm\0" as *const u8 as *const libc::c_char) == 0 {
        return 1 as libc::c_int != 0;
    }
    if *suf.offset(0 as libc::c_int as isize) as libc::c_int != 0
        && c_strcasecmp(
            suf.offset(1 as libc::c_int as isize),
            b"html\0" as *const u8 as *const libc::c_char,
        ) == 0
    {
        return 1 as libc::c_int != 0;
    }
    return 0 as libc::c_int != 0;
}
pub unsafe extern "C" fn wget_read_file(
    mut file: *const libc::c_char,
) -> *mut file_memory {
    let mut current_block: u64;
    let mut fd: libc::c_int = 0;
    let mut fm: *mut file_memory = 0 as *mut file_memory;
    let mut size: libc::c_long = 0;
    let mut inhibit_close: bool = 0 as libc::c_int != 0;
    if *file as libc::c_int == '-' as i32 && *file.offset(1 as libc::c_int as isize) == 0
    {
        fd = fileno(stdin);
        inhibit_close = 1 as libc::c_int != 0;
    } else {
        fd = open(file, 0 as libc::c_int);
    }
    if fd < 0 as libc::c_int {
        return 0 as *mut file_memory;
    }
    fm = xmalloc(::std::mem::size_of::<file_memory>() as libc::c_ulong)
        as *mut file_memory;
    let mut buf: stat = stat {
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
    if !(fstat(fd, &mut buf) < 0 as libc::c_int) {
        (*fm).length = buf.st_size;
        (*fm)
            .content = mmap(
            0 as *mut libc::c_void,
            (*fm).length as size_t,
            0x1 as libc::c_int | 0x2 as libc::c_int,
            0x2 as libc::c_int,
            fd,
            0 as libc::c_int as __off_t,
        ) as *mut libc::c_char;
        if !((*fm).content
            == -(1 as libc::c_int) as *mut libc::c_void as *mut libc::c_char)
        {
            if !inhibit_close {
                close(fd);
            }
            (*fm).mmap_p = 1 as libc::c_int;
            return fm;
        }
    }
    (*fm).length = 0 as libc::c_int as libc::c_long;
    size = 512 as libc::c_int as libc::c_long;
    (*fm).content = xmalloc(size as size_t) as *mut libc::c_char;
    loop {
        let mut nread: wgint = 0;
        if (*fm).length > size / 2 as libc::c_int as libc::c_long {
            size <<= 1 as libc::c_int;
            (*fm)
                .content = xrealloc((*fm).content as *mut libc::c_void, size as size_t)
                as *mut libc::c_char;
        }
        nread = read(
            fd,
            ((*fm).content).offset((*fm).length as isize) as *mut libc::c_void,
            (size - (*fm).length) as size_t,
        );
        if nread > 0 as libc::c_int as libc::c_long {
            (*fm).length += nread;
        } else if nread < 0 as libc::c_int as libc::c_long {
            current_block = 16811375726927433417;
            break;
        } else {
            current_block = 2719512138335094285;
            break;
        }
    }
    match current_block {
        16811375726927433417 => {
            if !inhibit_close {
                close(fd);
            }
            rpl_free((*fm).content as *mut libc::c_void);
            (*fm).content = 0 as *mut libc::c_char;
            rpl_free(fm as *mut libc::c_void);
            fm = 0 as *mut file_memory;
            return 0 as *mut file_memory;
        }
        _ => {
            if !inhibit_close {
                close(fd);
            }
            if size > (*fm).length && (*fm).length != 0 as libc::c_int as libc::c_long {
                (*fm)
                    .content = xrealloc(
                    (*fm).content as *mut libc::c_void,
                    (*fm).length as size_t,
                ) as *mut libc::c_char;
            }
            (*fm).mmap_p = 0 as libc::c_int;
            return fm;
        }
    };
}
pub unsafe extern "C" fn wget_read_file_free(mut fm: *mut file_memory) {
    if (*fm).mmap_p != 0 {
        munmap((*fm).content as *mut libc::c_void, (*fm).length as size_t);
    } else {
        rpl_free((*fm).content as *mut libc::c_void);
        (*fm).content = 0 as *mut libc::c_char;
    }
    rpl_free(fm as *mut libc::c_void);
    fm = 0 as *mut file_memory;
}
pub unsafe extern "C" fn free_vec(mut vec: *mut *mut libc::c_char) {
    if !vec.is_null() {
        let mut p: *mut *mut libc::c_char = vec;
        while !(*p).is_null() {
            rpl_free(*p as *mut libc::c_void);
            *p = 0 as *mut libc::c_char;
            p = p.offset(1);
            p;
        }
        rpl_free(vec as *mut libc::c_void);
        vec = 0 as *mut *mut libc::c_char;
    }
}
pub unsafe extern "C" fn merge_vecs(
    mut v1: *mut *mut libc::c_char,
    mut v2: *mut *mut libc::c_char,
) -> *mut *mut libc::c_char {
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    if v1.is_null() {
        return v2;
    }
    if v2.is_null() {
        return v1;
    }
    if (*v2).is_null() {
        rpl_free(v2 as *mut libc::c_void);
        v2 = 0 as *mut *mut libc::c_char;
        return v1;
    }
    i = 0 as libc::c_int;
    while !(*v1.offset(i as isize)).is_null() {
        i += 1;
        i;
    }
    j = 0 as libc::c_int;
    while !(*v2.offset(j as isize)).is_null() {
        j += 1;
        j;
    }
    v1 = xrealloc(
        v1 as *mut libc::c_void,
        ((i + j + 1 as libc::c_int) as libc::c_ulong)
            .wrapping_mul(::std::mem::size_of::<*mut libc::c_char>() as libc::c_ulong),
    ) as *mut *mut libc::c_char;
    memcpy(
        v1.offset(i as isize) as *mut libc::c_void,
        v2 as *const libc::c_void,
        ((j + 1 as libc::c_int) as libc::c_ulong)
            .wrapping_mul(::std::mem::size_of::<*mut libc::c_char>() as libc::c_ulong),
    );
    rpl_free(v2 as *mut libc::c_void);
    v2 = 0 as *mut *mut libc::c_char;
    return v1;
}
pub unsafe extern "C" fn vec_append(
    mut vec: *mut *mut libc::c_char,
    mut str: *const libc::c_char,
) -> *mut *mut libc::c_char {
    let mut cnt: libc::c_int = 0;
    if !vec.is_null() {
        cnt = 0 as libc::c_int;
        while !(*vec.offset(cnt as isize)).is_null() {
            cnt += 1;
            cnt;
        }
        cnt += 1;
        cnt;
    } else {
        cnt = 1 as libc::c_int;
    }
    vec = xrealloc(
        vec as *mut libc::c_void,
        ((cnt + 1 as libc::c_int) as libc::c_ulong)
            .wrapping_mul(::std::mem::size_of::<*mut libc::c_char>() as libc::c_ulong),
    ) as *mut *mut libc::c_char;
    let ref mut fresh9 = *vec.offset((cnt - 1 as libc::c_int) as isize);
    *fresh9 = xstrdup(str);
    let ref mut fresh10 = *vec.offset(cnt as isize);
    *fresh10 = 0 as *mut libc::c_char;
    return vec;
}
pub unsafe extern "C" fn string_set_add(
    mut ht: *mut hash_table,
    mut s: *const libc::c_char,
) {
    if hash_table_contains(ht, s as *const libc::c_void) != 0 {
        return;
    }
    hash_table_put(
        ht,
        xstrdup(s) as *const libc::c_void,
        b"1\0" as *const u8 as *const libc::c_char as *const libc::c_void,
    );
}
pub unsafe extern "C" fn string_set_contains(
    mut ht: *mut hash_table,
    mut s: *const libc::c_char,
) -> libc::c_int {
    return hash_table_contains(ht, s as *const libc::c_void);
}
pub unsafe extern "C" fn string_set_to_array(
    mut ht: *mut hash_table,
    mut array: *mut *mut libc::c_char,
) {
    let mut iter: hash_table_iterator = hash_table_iterator {
        key: 0 as *mut libc::c_void,
        value: 0 as *mut libc::c_void,
        pos: 0 as *mut libc::c_void,
        end: 0 as *mut libc::c_void,
    };
    hash_table_iterate(ht, &mut iter);
    while hash_table_iter_next(&mut iter) != 0 {
        let fresh11 = array;
        array = array.offset(1);
        *fresh11 = iter.key as *mut libc::c_char;
    }
}
pub unsafe extern "C" fn string_set_free(mut ht: *mut hash_table) {
    let mut iter: hash_table_iterator = hash_table_iterator {
        key: 0 as *mut libc::c_void,
        value: 0 as *mut libc::c_void,
        pos: 0 as *mut libc::c_void,
        end: 0 as *mut libc::c_void,
    };
    hash_table_iterate(ht, &mut iter);
    while hash_table_iter_next(&mut iter) != 0 {
        rpl_free(iter.key);
        iter.key = 0 as *mut libc::c_void;
    }
    hash_table_destroy(ht);
}
pub unsafe extern "C" fn free_keys_and_values(mut ht: *mut hash_table) {
    let mut iter: hash_table_iterator = hash_table_iterator {
        key: 0 as *mut libc::c_void,
        value: 0 as *mut libc::c_void,
        pos: 0 as *mut libc::c_void,
        end: 0 as *mut libc::c_void,
    };
    hash_table_iterate(ht, &mut iter);
    while hash_table_iter_next(&mut iter) != 0 {
        rpl_free(iter.key);
        iter.key = 0 as *mut libc::c_void;
        rpl_free(iter.value);
        iter.value = 0 as *mut libc::c_void;
    }
}
unsafe extern "C" fn get_grouping_data(
    mut sep: *mut *const libc::c_char,
    mut grouping: *mut *const libc::c_char,
) {
    static mut cached_sep: *const libc::c_char = 0 as *const libc::c_char;
    static mut cached_grouping: *const libc::c_char = 0 as *const libc::c_char;
    static mut initialized: bool = false;
    if !initialized {
        let mut lconv: *mut lconv = localeconv();
        cached_sep = (*lconv).thousands_sep;
        cached_grouping = (*lconv).grouping;
        if *cached_sep == 0 {
            if *(*lconv).decimal_point as libc::c_int != ',' as i32 {
                cached_sep = b",\0" as *const u8 as *const libc::c_char;
            } else {
                cached_sep = b".\0" as *const u8 as *const libc::c_char;
            }
            cached_grouping = b"\x03\0" as *const u8 as *const libc::c_char;
        }
        initialized = 1 as libc::c_int != 0;
    }
    *sep = cached_sep;
    *grouping = cached_grouping;
}
pub unsafe extern "C" fn with_thousand_seps(mut n: wgint) -> *const libc::c_char {
    static mut outbuf: [libc::c_char; 48] = [0; 48];
    let mut p: *mut libc::c_char = outbuf
        .as_mut_ptr()
        .offset(::std::mem::size_of::<[libc::c_char; 48]>() as libc::c_ulong as isize);
    let mut grouping: *const libc::c_char = 0 as *const libc::c_char;
    let mut sep: *const libc::c_char = 0 as *const libc::c_char;
    let mut seplen: libc::c_int = 0;
    let mut i: libc::c_int = 0 as libc::c_int;
    let mut groupsize: libc::c_int = 0;
    let mut atgroup: *const libc::c_char = 0 as *const libc::c_char;
    let mut negative: bool = n < 0 as libc::c_int as libc::c_long;
    get_grouping_data(&mut sep, &mut grouping);
    seplen = strlen(sep) as libc::c_int;
    atgroup = grouping;
    let fresh12 = atgroup;
    atgroup = atgroup.offset(1);
    groupsize = *fresh12 as libc::c_int;
    if negative {
        n = -n;
    }
    p = p.offset(-1);
    *p = '\0' as i32 as libc::c_char;
    loop {
        p = p.offset(-1);
        *p = (n % 10 as libc::c_int as libc::c_long + '0' as i32 as libc::c_long)
            as libc::c_char;
        n /= 10 as libc::c_int as libc::c_long;
        if n == 0 as libc::c_int as libc::c_long {
            break;
        }
        i += 1;
        if i == groupsize {
            if seplen == 1 as libc::c_int {
                p = p.offset(-1);
                *p = *sep;
            } else {
                p = p.offset(-(seplen as isize));
                memcpy(
                    p as *mut libc::c_void,
                    sep as *const libc::c_void,
                    seplen as libc::c_ulong,
                );
            }
            i = 0 as libc::c_int;
            if *atgroup != 0 {
                let fresh13 = atgroup;
                atgroup = atgroup.offset(1);
                groupsize = *fresh13 as libc::c_int;
            }
        }
    }
    if negative {
        p = p.offset(-1);
        *p = '-' as i32 as libc::c_char;
    }
    return p;
}
pub unsafe extern "C" fn human_readable(
    mut n: wgint,
    acc: libc::c_int,
    decimals: libc::c_int,
) -> *mut libc::c_char {
    static mut powers: [libc::c_char; 6] = [
        'K' as i32 as libc::c_char,
        'M' as i32 as libc::c_char,
        'G' as i32 as libc::c_char,
        'T' as i32 as libc::c_char,
        'P' as i32 as libc::c_char,
        'E' as i32 as libc::c_char,
    ];
    static mut buf: [libc::c_char; 8] = [0; 8];
    let mut i: size_t = 0;
    if n < 1024 as libc::c_int as libc::c_long {
        snprintf(
            buf.as_mut_ptr(),
            ::std::mem::size_of::<[libc::c_char; 8]>() as libc::c_ulong,
            b"%d\0" as *const u8 as *const libc::c_char,
            n as libc::c_int,
        );
        return buf.as_mut_ptr();
    }
    i = 0 as libc::c_int as size_t;
    while i
        < (::std::mem::size_of::<[libc::c_char; 6]>() as libc::c_ulong)
            .wrapping_div(::std::mem::size_of::<libc::c_char>() as libc::c_ulong)
    {
        if (n / 1024 as libc::c_int as libc::c_long)
            < 1024 as libc::c_int as libc::c_long
            || i
                == (::std::mem::size_of::<[libc::c_char; 6]>() as libc::c_ulong)
                    .wrapping_div(::std::mem::size_of::<libc::c_char>() as libc::c_ulong)
                    .wrapping_sub(1 as libc::c_int as libc::c_ulong)
        {
            let mut val: libc::c_double = n as libc::c_double / 1024.0f64;
            snprintf(
                buf.as_mut_ptr(),
                ::std::mem::size_of::<[libc::c_char; 8]>() as libc::c_ulong,
                b"%.*f%c\0" as *const u8 as *const libc::c_char,
                if val < acc as libc::c_double { decimals } else { 0 as libc::c_int },
                val,
                powers[i as usize] as libc::c_int,
            );
            return buf.as_mut_ptr();
        }
        n /= 1024 as libc::c_int as libc::c_long;
        i = i.wrapping_add(1);
        i;
    }
    return 0 as *mut libc::c_char;
}
pub unsafe extern "C" fn numdigit(mut number: wgint) -> libc::c_int {
    let mut cnt: libc::c_int = 1 as libc::c_int;
    if number < 0 as libc::c_int as libc::c_long {
        cnt += 1;
        cnt;
    }
    loop {
        number /= 10 as libc::c_int as libc::c_long;
        if !(number != 0 as libc::c_int as libc::c_long) {
            break;
        }
        cnt += 1;
        cnt;
    }
    return cnt;
}
pub unsafe extern "C" fn number_to_string(
    mut buffer: *mut libc::c_char,
    mut number: wgint,
) -> *mut libc::c_char {
    let mut p: *mut libc::c_char = buffer;
    let mut n: wgint = number;
    let mut last_digit_char: libc::c_int = 0 as libc::c_int;
    if n < 0 as libc::c_int as libc::c_long {
        if n < -(9223372036854775807 as libc::c_long) {
            let mut last_digit: libc::c_int = (n % 10 as libc::c_int as libc::c_long)
                as libc::c_int;
            if last_digit < 0 as libc::c_int {
                last_digit_char = '0' as i32 - last_digit;
            } else {
                last_digit_char = '0' as i32 + last_digit;
            }
            n /= 10 as libc::c_int as libc::c_long;
        }
        let fresh14 = p;
        p = p.offset(1);
        *fresh14 = '-' as i32 as libc::c_char;
        n = -n;
    }
    if n < 10 as libc::c_int as libc::c_long {
        let fresh15 = p;
        p = p.offset(1);
        *fresh15 = (n / 1 as libc::c_int as libc::c_long + '0' as i32 as libc::c_long)
            as libc::c_char;
    } else if n < 100 as libc::c_int as libc::c_long {
        let fresh16 = p;
        p = p.offset(1);
        *fresh16 = (n / 10 as libc::c_int as libc::c_long + '0' as i32 as libc::c_long)
            as libc::c_char;
        n %= 10 as libc::c_int as libc::c_long;
        let fresh17 = p;
        p = p.offset(1);
        *fresh17 = (n / (10 as libc::c_int / 10 as libc::c_int) as libc::c_long
            + '0' as i32 as libc::c_long) as libc::c_char;
    } else if n < 1000 as libc::c_int as libc::c_long {
        let fresh18 = p;
        p = p.offset(1);
        *fresh18 = (n / 100 as libc::c_int as libc::c_long + '0' as i32 as libc::c_long)
            as libc::c_char;
        n %= 100 as libc::c_int as libc::c_long;
        let fresh19 = p;
        p = p.offset(1);
        *fresh19 = (n / (100 as libc::c_int / 10 as libc::c_int) as libc::c_long
            + '0' as i32 as libc::c_long) as libc::c_char;
        n %= (100 as libc::c_int / 10 as libc::c_int) as libc::c_long;
        let fresh20 = p;
        p = p.offset(1);
        *fresh20 = (n
            / (100 as libc::c_int / 10 as libc::c_int / 10 as libc::c_int)
                as libc::c_long + '0' as i32 as libc::c_long) as libc::c_char;
    } else if n < 10000 as libc::c_int as libc::c_long {
        let fresh21 = p;
        p = p.offset(1);
        *fresh21 = (n / 1000 as libc::c_int as libc::c_long + '0' as i32 as libc::c_long)
            as libc::c_char;
        n %= 1000 as libc::c_int as libc::c_long;
        let fresh22 = p;
        p = p.offset(1);
        *fresh22 = (n / (1000 as libc::c_int / 10 as libc::c_int) as libc::c_long
            + '0' as i32 as libc::c_long) as libc::c_char;
        n %= (1000 as libc::c_int / 10 as libc::c_int) as libc::c_long;
        let fresh23 = p;
        p = p.offset(1);
        *fresh23 = (n
            / (1000 as libc::c_int / 10 as libc::c_int / 10 as libc::c_int)
                as libc::c_long + '0' as i32 as libc::c_long) as libc::c_char;
        n
            %= (1000 as libc::c_int / 10 as libc::c_int / 10 as libc::c_int)
                as libc::c_long;
        let fresh24 = p;
        p = p.offset(1);
        *fresh24 = (n
            / (1000 as libc::c_int / 10 as libc::c_int / 10 as libc::c_int
                / 10 as libc::c_int) as libc::c_long + '0' as i32 as libc::c_long)
            as libc::c_char;
    } else if n < 100000 as libc::c_int as libc::c_long {
        let fresh25 = p;
        p = p.offset(1);
        *fresh25 = (n / 10000 as libc::c_int as libc::c_long
            + '0' as i32 as libc::c_long) as libc::c_char;
        n %= 10000 as libc::c_int as libc::c_long;
        let fresh26 = p;
        p = p.offset(1);
        *fresh26 = (n / (10000 as libc::c_int / 10 as libc::c_int) as libc::c_long
            + '0' as i32 as libc::c_long) as libc::c_char;
        n %= (10000 as libc::c_int / 10 as libc::c_int) as libc::c_long;
        let fresh27 = p;
        p = p.offset(1);
        *fresh27 = (n
            / (10000 as libc::c_int / 10 as libc::c_int / 10 as libc::c_int)
                as libc::c_long + '0' as i32 as libc::c_long) as libc::c_char;
        n
            %= (10000 as libc::c_int / 10 as libc::c_int / 10 as libc::c_int)
                as libc::c_long;
        let fresh28 = p;
        p = p.offset(1);
        *fresh28 = (n
            / (10000 as libc::c_int / 10 as libc::c_int / 10 as libc::c_int
                / 10 as libc::c_int) as libc::c_long + '0' as i32 as libc::c_long)
            as libc::c_char;
        n
            %= (10000 as libc::c_int / 10 as libc::c_int / 10 as libc::c_int
                / 10 as libc::c_int) as libc::c_long;
        let fresh29 = p;
        p = p.offset(1);
        *fresh29 = (n
            / (10000 as libc::c_int / 10 as libc::c_int / 10 as libc::c_int
                / 10 as libc::c_int / 10 as libc::c_int) as libc::c_long
            + '0' as i32 as libc::c_long) as libc::c_char;
    } else if n < 1000000 as libc::c_int as libc::c_long {
        let fresh30 = p;
        p = p.offset(1);
        *fresh30 = (n / 100000 as libc::c_int as libc::c_long
            + '0' as i32 as libc::c_long) as libc::c_char;
        n %= 100000 as libc::c_int as libc::c_long;
        let fresh31 = p;
        p = p.offset(1);
        *fresh31 = (n / (100000 as libc::c_int / 10 as libc::c_int) as libc::c_long
            + '0' as i32 as libc::c_long) as libc::c_char;
        n %= (100000 as libc::c_int / 10 as libc::c_int) as libc::c_long;
        let fresh32 = p;
        p = p.offset(1);
        *fresh32 = (n
            / (100000 as libc::c_int / 10 as libc::c_int / 10 as libc::c_int)
                as libc::c_long + '0' as i32 as libc::c_long) as libc::c_char;
        n
            %= (100000 as libc::c_int / 10 as libc::c_int / 10 as libc::c_int)
                as libc::c_long;
        let fresh33 = p;
        p = p.offset(1);
        *fresh33 = (n
            / (100000 as libc::c_int / 10 as libc::c_int / 10 as libc::c_int
                / 10 as libc::c_int) as libc::c_long + '0' as i32 as libc::c_long)
            as libc::c_char;
        n
            %= (100000 as libc::c_int / 10 as libc::c_int / 10 as libc::c_int
                / 10 as libc::c_int) as libc::c_long;
        let fresh34 = p;
        p = p.offset(1);
        *fresh34 = (n
            / (100000 as libc::c_int / 10 as libc::c_int / 10 as libc::c_int
                / 10 as libc::c_int / 10 as libc::c_int) as libc::c_long
            + '0' as i32 as libc::c_long) as libc::c_char;
        n
            %= (100000 as libc::c_int / 10 as libc::c_int / 10 as libc::c_int
                / 10 as libc::c_int / 10 as libc::c_int) as libc::c_long;
        let fresh35 = p;
        p = p.offset(1);
        *fresh35 = (n
            / (100000 as libc::c_int / 10 as libc::c_int / 10 as libc::c_int
                / 10 as libc::c_int / 10 as libc::c_int / 10 as libc::c_int)
                as libc::c_long + '0' as i32 as libc::c_long) as libc::c_char;
    } else if n < 10000000 as libc::c_int as libc::c_long {
        let fresh36 = p;
        p = p.offset(1);
        *fresh36 = (n / 1000000 as libc::c_int as libc::c_long
            + '0' as i32 as libc::c_long) as libc::c_char;
        n %= 1000000 as libc::c_int as libc::c_long;
        let fresh37 = p;
        p = p.offset(1);
        *fresh37 = (n / (1000000 as libc::c_int / 10 as libc::c_int) as libc::c_long
            + '0' as i32 as libc::c_long) as libc::c_char;
        n %= (1000000 as libc::c_int / 10 as libc::c_int) as libc::c_long;
        let fresh38 = p;
        p = p.offset(1);
        *fresh38 = (n
            / (1000000 as libc::c_int / 10 as libc::c_int / 10 as libc::c_int)
                as libc::c_long + '0' as i32 as libc::c_long) as libc::c_char;
        n
            %= (1000000 as libc::c_int / 10 as libc::c_int / 10 as libc::c_int)
                as libc::c_long;
        let fresh39 = p;
        p = p.offset(1);
        *fresh39 = (n
            / (1000000 as libc::c_int / 10 as libc::c_int / 10 as libc::c_int
                / 10 as libc::c_int) as libc::c_long + '0' as i32 as libc::c_long)
            as libc::c_char;
        n
            %= (1000000 as libc::c_int / 10 as libc::c_int / 10 as libc::c_int
                / 10 as libc::c_int) as libc::c_long;
        let fresh40 = p;
        p = p.offset(1);
        *fresh40 = (n
            / (1000000 as libc::c_int / 10 as libc::c_int / 10 as libc::c_int
                / 10 as libc::c_int / 10 as libc::c_int) as libc::c_long
            + '0' as i32 as libc::c_long) as libc::c_char;
        n
            %= (1000000 as libc::c_int / 10 as libc::c_int / 10 as libc::c_int
                / 10 as libc::c_int / 10 as libc::c_int) as libc::c_long;
        let fresh41 = p;
        p = p.offset(1);
        *fresh41 = (n
            / (1000000 as libc::c_int / 10 as libc::c_int / 10 as libc::c_int
                / 10 as libc::c_int / 10 as libc::c_int / 10 as libc::c_int)
                as libc::c_long + '0' as i32 as libc::c_long) as libc::c_char;
        n
            %= (1000000 as libc::c_int / 10 as libc::c_int / 10 as libc::c_int
                / 10 as libc::c_int / 10 as libc::c_int / 10 as libc::c_int)
                as libc::c_long;
        let fresh42 = p;
        p = p.offset(1);
        *fresh42 = (n
            / (1000000 as libc::c_int / 10 as libc::c_int / 10 as libc::c_int
                / 10 as libc::c_int / 10 as libc::c_int / 10 as libc::c_int
                / 10 as libc::c_int) as libc::c_long + '0' as i32 as libc::c_long)
            as libc::c_char;
    } else if n < 100000000 as libc::c_int as libc::c_long {
        let fresh43 = p;
        p = p.offset(1);
        *fresh43 = (n / 10000000 as libc::c_int as libc::c_long
            + '0' as i32 as libc::c_long) as libc::c_char;
        n %= 10000000 as libc::c_int as libc::c_long;
        let fresh44 = p;
        p = p.offset(1);
        *fresh44 = (n / (10000000 as libc::c_int / 10 as libc::c_int) as libc::c_long
            + '0' as i32 as libc::c_long) as libc::c_char;
        n %= (10000000 as libc::c_int / 10 as libc::c_int) as libc::c_long;
        let fresh45 = p;
        p = p.offset(1);
        *fresh45 = (n
            / (10000000 as libc::c_int / 10 as libc::c_int / 10 as libc::c_int)
                as libc::c_long + '0' as i32 as libc::c_long) as libc::c_char;
        n
            %= (10000000 as libc::c_int / 10 as libc::c_int / 10 as libc::c_int)
                as libc::c_long;
        let fresh46 = p;
        p = p.offset(1);
        *fresh46 = (n
            / (10000000 as libc::c_int / 10 as libc::c_int / 10 as libc::c_int
                / 10 as libc::c_int) as libc::c_long + '0' as i32 as libc::c_long)
            as libc::c_char;
        n
            %= (10000000 as libc::c_int / 10 as libc::c_int / 10 as libc::c_int
                / 10 as libc::c_int) as libc::c_long;
        let fresh47 = p;
        p = p.offset(1);
        *fresh47 = (n
            / (10000000 as libc::c_int / 10 as libc::c_int / 10 as libc::c_int
                / 10 as libc::c_int / 10 as libc::c_int) as libc::c_long
            + '0' as i32 as libc::c_long) as libc::c_char;
        n
            %= (10000000 as libc::c_int / 10 as libc::c_int / 10 as libc::c_int
                / 10 as libc::c_int / 10 as libc::c_int) as libc::c_long;
        let fresh48 = p;
        p = p.offset(1);
        *fresh48 = (n
            / (10000000 as libc::c_int / 10 as libc::c_int / 10 as libc::c_int
                / 10 as libc::c_int / 10 as libc::c_int / 10 as libc::c_int)
                as libc::c_long + '0' as i32 as libc::c_long) as libc::c_char;
        n
            %= (10000000 as libc::c_int / 10 as libc::c_int / 10 as libc::c_int
                / 10 as libc::c_int / 10 as libc::c_int / 10 as libc::c_int)
                as libc::c_long;
        let fresh49 = p;
        p = p.offset(1);
        *fresh49 = (n
            / (10000000 as libc::c_int / 10 as libc::c_int / 10 as libc::c_int
                / 10 as libc::c_int / 10 as libc::c_int / 10 as libc::c_int
                / 10 as libc::c_int) as libc::c_long + '0' as i32 as libc::c_long)
            as libc::c_char;
        n
            %= (10000000 as libc::c_int / 10 as libc::c_int / 10 as libc::c_int
                / 10 as libc::c_int / 10 as libc::c_int / 10 as libc::c_int
                / 10 as libc::c_int) as libc::c_long;
        let fresh50 = p;
        p = p.offset(1);
        *fresh50 = (n
            / (10000000 as libc::c_int / 10 as libc::c_int / 10 as libc::c_int
                / 10 as libc::c_int / 10 as libc::c_int / 10 as libc::c_int
                / 10 as libc::c_int / 10 as libc::c_int) as libc::c_long
            + '0' as i32 as libc::c_long) as libc::c_char;
    } else if n < 1000000000 as libc::c_int as libc::c_long {
        let fresh51 = p;
        p = p.offset(1);
        *fresh51 = (n / 100000000 as libc::c_int as libc::c_long
            + '0' as i32 as libc::c_long) as libc::c_char;
        n %= 100000000 as libc::c_int as libc::c_long;
        let fresh52 = p;
        p = p.offset(1);
        *fresh52 = (n / (100000000 as libc::c_int / 10 as libc::c_int) as libc::c_long
            + '0' as i32 as libc::c_long) as libc::c_char;
        n %= (100000000 as libc::c_int / 10 as libc::c_int) as libc::c_long;
        let fresh53 = p;
        p = p.offset(1);
        *fresh53 = (n
            / (100000000 as libc::c_int / 10 as libc::c_int / 10 as libc::c_int)
                as libc::c_long + '0' as i32 as libc::c_long) as libc::c_char;
        n
            %= (100000000 as libc::c_int / 10 as libc::c_int / 10 as libc::c_int)
                as libc::c_long;
        let fresh54 = p;
        p = p.offset(1);
        *fresh54 = (n
            / (100000000 as libc::c_int / 10 as libc::c_int / 10 as libc::c_int
                / 10 as libc::c_int) as libc::c_long + '0' as i32 as libc::c_long)
            as libc::c_char;
        n
            %= (100000000 as libc::c_int / 10 as libc::c_int / 10 as libc::c_int
                / 10 as libc::c_int) as libc::c_long;
        let fresh55 = p;
        p = p.offset(1);
        *fresh55 = (n
            / (100000000 as libc::c_int / 10 as libc::c_int / 10 as libc::c_int
                / 10 as libc::c_int / 10 as libc::c_int) as libc::c_long
            + '0' as i32 as libc::c_long) as libc::c_char;
        n
            %= (100000000 as libc::c_int / 10 as libc::c_int / 10 as libc::c_int
                / 10 as libc::c_int / 10 as libc::c_int) as libc::c_long;
        let fresh56 = p;
        p = p.offset(1);
        *fresh56 = (n
            / (100000000 as libc::c_int / 10 as libc::c_int / 10 as libc::c_int
                / 10 as libc::c_int / 10 as libc::c_int / 10 as libc::c_int)
                as libc::c_long + '0' as i32 as libc::c_long) as libc::c_char;
        n
            %= (100000000 as libc::c_int / 10 as libc::c_int / 10 as libc::c_int
                / 10 as libc::c_int / 10 as libc::c_int / 10 as libc::c_int)
                as libc::c_long;
        let fresh57 = p;
        p = p.offset(1);
        *fresh57 = (n
            / (100000000 as libc::c_int / 10 as libc::c_int / 10 as libc::c_int
                / 10 as libc::c_int / 10 as libc::c_int / 10 as libc::c_int
                / 10 as libc::c_int) as libc::c_long + '0' as i32 as libc::c_long)
            as libc::c_char;
        n
            %= (100000000 as libc::c_int / 10 as libc::c_int / 10 as libc::c_int
                / 10 as libc::c_int / 10 as libc::c_int / 10 as libc::c_int
                / 10 as libc::c_int) as libc::c_long;
        let fresh58 = p;
        p = p.offset(1);
        *fresh58 = (n
            / (100000000 as libc::c_int / 10 as libc::c_int / 10 as libc::c_int
                / 10 as libc::c_int / 10 as libc::c_int / 10 as libc::c_int
                / 10 as libc::c_int / 10 as libc::c_int) as libc::c_long
            + '0' as i32 as libc::c_long) as libc::c_char;
        n
            %= (100000000 as libc::c_int / 10 as libc::c_int / 10 as libc::c_int
                / 10 as libc::c_int / 10 as libc::c_int / 10 as libc::c_int
                / 10 as libc::c_int / 10 as libc::c_int) as libc::c_long;
        let fresh59 = p;
        p = p.offset(1);
        *fresh59 = (n
            / (100000000 as libc::c_int / 10 as libc::c_int / 10 as libc::c_int
                / 10 as libc::c_int / 10 as libc::c_int / 10 as libc::c_int
                / 10 as libc::c_int / 10 as libc::c_int / 10 as libc::c_int)
                as libc::c_long + '0' as i32 as libc::c_long) as libc::c_char;
    } else if n < 10 as libc::c_int as libc::c_long * 1000000000 as libc::c_int as wgint
    {
        let fresh60 = p;
        p = p.offset(1);
        *fresh60 = (n / 1000000000 as libc::c_int as libc::c_long
            + '0' as i32 as libc::c_long) as libc::c_char;
        n %= 1000000000 as libc::c_int as libc::c_long;
        let fresh61 = p;
        p = p.offset(1);
        *fresh61 = (n / (1000000000 as libc::c_int / 10 as libc::c_int) as libc::c_long
            + '0' as i32 as libc::c_long) as libc::c_char;
        n %= (1000000000 as libc::c_int / 10 as libc::c_int) as libc::c_long;
        let fresh62 = p;
        p = p.offset(1);
        *fresh62 = (n
            / (1000000000 as libc::c_int / 10 as libc::c_int / 10 as libc::c_int)
                as libc::c_long + '0' as i32 as libc::c_long) as libc::c_char;
        n
            %= (1000000000 as libc::c_int / 10 as libc::c_int / 10 as libc::c_int)
                as libc::c_long;
        let fresh63 = p;
        p = p.offset(1);
        *fresh63 = (n
            / (1000000000 as libc::c_int / 10 as libc::c_int / 10 as libc::c_int
                / 10 as libc::c_int) as libc::c_long + '0' as i32 as libc::c_long)
            as libc::c_char;
        n
            %= (1000000000 as libc::c_int / 10 as libc::c_int / 10 as libc::c_int
                / 10 as libc::c_int) as libc::c_long;
        let fresh64 = p;
        p = p.offset(1);
        *fresh64 = (n
            / (1000000000 as libc::c_int / 10 as libc::c_int / 10 as libc::c_int
                / 10 as libc::c_int / 10 as libc::c_int) as libc::c_long
            + '0' as i32 as libc::c_long) as libc::c_char;
        n
            %= (1000000000 as libc::c_int / 10 as libc::c_int / 10 as libc::c_int
                / 10 as libc::c_int / 10 as libc::c_int) as libc::c_long;
        let fresh65 = p;
        p = p.offset(1);
        *fresh65 = (n
            / (1000000000 as libc::c_int / 10 as libc::c_int / 10 as libc::c_int
                / 10 as libc::c_int / 10 as libc::c_int / 10 as libc::c_int)
                as libc::c_long + '0' as i32 as libc::c_long) as libc::c_char;
        n
            %= (1000000000 as libc::c_int / 10 as libc::c_int / 10 as libc::c_int
                / 10 as libc::c_int / 10 as libc::c_int / 10 as libc::c_int)
                as libc::c_long;
        let fresh66 = p;
        p = p.offset(1);
        *fresh66 = (n
            / (1000000000 as libc::c_int / 10 as libc::c_int / 10 as libc::c_int
                / 10 as libc::c_int / 10 as libc::c_int / 10 as libc::c_int
                / 10 as libc::c_int) as libc::c_long + '0' as i32 as libc::c_long)
            as libc::c_char;
        n
            %= (1000000000 as libc::c_int / 10 as libc::c_int / 10 as libc::c_int
                / 10 as libc::c_int / 10 as libc::c_int / 10 as libc::c_int
                / 10 as libc::c_int) as libc::c_long;
        let fresh67 = p;
        p = p.offset(1);
        *fresh67 = (n
            / (1000000000 as libc::c_int / 10 as libc::c_int / 10 as libc::c_int
                / 10 as libc::c_int / 10 as libc::c_int / 10 as libc::c_int
                / 10 as libc::c_int / 10 as libc::c_int) as libc::c_long
            + '0' as i32 as libc::c_long) as libc::c_char;
        n
            %= (1000000000 as libc::c_int / 10 as libc::c_int / 10 as libc::c_int
                / 10 as libc::c_int / 10 as libc::c_int / 10 as libc::c_int
                / 10 as libc::c_int / 10 as libc::c_int) as libc::c_long;
        let fresh68 = p;
        p = p.offset(1);
        *fresh68 = (n
            / (1000000000 as libc::c_int / 10 as libc::c_int / 10 as libc::c_int
                / 10 as libc::c_int / 10 as libc::c_int / 10 as libc::c_int
                / 10 as libc::c_int / 10 as libc::c_int / 10 as libc::c_int)
                as libc::c_long + '0' as i32 as libc::c_long) as libc::c_char;
        n
            %= (1000000000 as libc::c_int / 10 as libc::c_int / 10 as libc::c_int
                / 10 as libc::c_int / 10 as libc::c_int / 10 as libc::c_int
                / 10 as libc::c_int / 10 as libc::c_int / 10 as libc::c_int)
                as libc::c_long;
        let fresh69 = p;
        p = p.offset(1);
        *fresh69 = (n
            / (1000000000 as libc::c_int / 10 as libc::c_int / 10 as libc::c_int
                / 10 as libc::c_int / 10 as libc::c_int / 10 as libc::c_int
                / 10 as libc::c_int / 10 as libc::c_int / 10 as libc::c_int
                / 10 as libc::c_int) as libc::c_long + '0' as i32 as libc::c_long)
            as libc::c_char;
    } else if n < 100 as libc::c_int as libc::c_long * 1000000000 as libc::c_int as wgint
    {
        let fresh70 = p;
        p = p.offset(1);
        *fresh70 = (n
            / (10 as libc::c_int as libc::c_long * 1000000000 as libc::c_int as wgint)
            + '0' as i32 as libc::c_long) as libc::c_char;
        n %= 10 as libc::c_int as libc::c_long * 1000000000 as libc::c_int as wgint;
        let fresh71 = p;
        p = p.offset(1);
        *fresh71 = (n
            / (10 as libc::c_int as libc::c_long * 1000000000 as libc::c_int as wgint
                / 10 as libc::c_int as libc::c_long) + '0' as i32 as libc::c_long)
            as libc::c_char;
        n
            %= 10 as libc::c_int as libc::c_long * 1000000000 as libc::c_int as wgint
                / 10 as libc::c_int as libc::c_long;
        let fresh72 = p;
        p = p.offset(1);
        *fresh72 = (n
            / (10 as libc::c_int as libc::c_long * 1000000000 as libc::c_int as wgint
                / 10 as libc::c_int as libc::c_long / 10 as libc::c_int as libc::c_long)
            + '0' as i32 as libc::c_long) as libc::c_char;
        n
            %= 10 as libc::c_int as libc::c_long * 1000000000 as libc::c_int as wgint
                / 10 as libc::c_int as libc::c_long / 10 as libc::c_int as libc::c_long;
        let fresh73 = p;
        p = p.offset(1);
        *fresh73 = (n
            / (10 as libc::c_int as libc::c_long * 1000000000 as libc::c_int as wgint
                / 10 as libc::c_int as libc::c_long / 10 as libc::c_int as libc::c_long
                / 10 as libc::c_int as libc::c_long) + '0' as i32 as libc::c_long)
            as libc::c_char;
        n
            %= 10 as libc::c_int as libc::c_long * 1000000000 as libc::c_int as wgint
                / 10 as libc::c_int as libc::c_long / 10 as libc::c_int as libc::c_long
                / 10 as libc::c_int as libc::c_long;
        let fresh74 = p;
        p = p.offset(1);
        *fresh74 = (n
            / (10 as libc::c_int as libc::c_long * 1000000000 as libc::c_int as wgint
                / 10 as libc::c_int as libc::c_long / 10 as libc::c_int as libc::c_long
                / 10 as libc::c_int as libc::c_long / 10 as libc::c_int as libc::c_long)
            + '0' as i32 as libc::c_long) as libc::c_char;
        n
            %= 10 as libc::c_int as libc::c_long * 1000000000 as libc::c_int as wgint
                / 10 as libc::c_int as libc::c_long / 10 as libc::c_int as libc::c_long
                / 10 as libc::c_int as libc::c_long / 10 as libc::c_int as libc::c_long;
        let fresh75 = p;
        p = p.offset(1);
        *fresh75 = (n
            / (10 as libc::c_int as libc::c_long * 1000000000 as libc::c_int as wgint
                / 10 as libc::c_int as libc::c_long / 10 as libc::c_int as libc::c_long
                / 10 as libc::c_int as libc::c_long / 10 as libc::c_int as libc::c_long
                / 10 as libc::c_int as libc::c_long) + '0' as i32 as libc::c_long)
            as libc::c_char;
        n
            %= 10 as libc::c_int as libc::c_long * 1000000000 as libc::c_int as wgint
                / 10 as libc::c_int as libc::c_long / 10 as libc::c_int as libc::c_long
                / 10 as libc::c_int as libc::c_long / 10 as libc::c_int as libc::c_long
                / 10 as libc::c_int as libc::c_long;
        let fresh76 = p;
        p = p.offset(1);
        *fresh76 = (n
            / (10 as libc::c_int as libc::c_long * 1000000000 as libc::c_int as wgint
                / 10 as libc::c_int as libc::c_long / 10 as libc::c_int as libc::c_long
                / 10 as libc::c_int as libc::c_long / 10 as libc::c_int as libc::c_long
                / 10 as libc::c_int as libc::c_long / 10 as libc::c_int as libc::c_long)
            + '0' as i32 as libc::c_long) as libc::c_char;
        n
            %= 10 as libc::c_int as libc::c_long * 1000000000 as libc::c_int as wgint
                / 10 as libc::c_int as libc::c_long / 10 as libc::c_int as libc::c_long
                / 10 as libc::c_int as libc::c_long / 10 as libc::c_int as libc::c_long
                / 10 as libc::c_int as libc::c_long / 10 as libc::c_int as libc::c_long;
        let fresh77 = p;
        p = p.offset(1);
        *fresh77 = (n
            / (10 as libc::c_int as libc::c_long * 1000000000 as libc::c_int as wgint
                / 10 as libc::c_int as libc::c_long / 10 as libc::c_int as libc::c_long
                / 10 as libc::c_int as libc::c_long / 10 as libc::c_int as libc::c_long
                / 10 as libc::c_int as libc::c_long / 10 as libc::c_int as libc::c_long
                / 10 as libc::c_int as libc::c_long) + '0' as i32 as libc::c_long)
            as libc::c_char;
        n
            %= 10 as libc::c_int as libc::c_long * 1000000000 as libc::c_int as wgint
                / 10 as libc::c_int as libc::c_long / 10 as libc::c_int as libc::c_long
                / 10 as libc::c_int as libc::c_long / 10 as libc::c_int as libc::c_long
                / 10 as libc::c_int as libc::c_long / 10 as libc::c_int as libc::c_long
                / 10 as libc::c_int as libc::c_long;
        let fresh78 = p;
        p = p.offset(1);
        *fresh78 = (n
            / (10 as libc::c_int as libc::c_long * 1000000000 as libc::c_int as wgint
                / 10 as libc::c_int as libc::c_long / 10 as libc::c_int as libc::c_long
                / 10 as libc::c_int as libc::c_long / 10 as libc::c_int as libc::c_long
                / 10 as libc::c_int as libc::c_long / 10 as libc::c_int as libc::c_long
                / 10 as libc::c_int as libc::c_long / 10 as libc::c_int as libc::c_long)
            + '0' as i32 as libc::c_long) as libc::c_char;
        n
            %= 10 as libc::c_int as libc::c_long * 1000000000 as libc::c_int as wgint
                / 10 as libc::c_int as libc::c_long / 10 as libc::c_int as libc::c_long
                / 10 as libc::c_int as libc::c_long / 10 as libc::c_int as libc::c_long
                / 10 as libc::c_int as libc::c_long / 10 as libc::c_int as libc::c_long
                / 10 as libc::c_int as libc::c_long / 10 as libc::c_int as libc::c_long;
        let fresh79 = p;
        p = p.offset(1);
        *fresh79 = (n
            / (10 as libc::c_int as libc::c_long * 1000000000 as libc::c_int as wgint
                / 10 as libc::c_int as libc::c_long / 10 as libc::c_int as libc::c_long
                / 10 as libc::c_int as libc::c_long / 10 as libc::c_int as libc::c_long
                / 10 as libc::c_int as libc::c_long / 10 as libc::c_int as libc::c_long
                / 10 as libc::c_int as libc::c_long / 10 as libc::c_int as libc::c_long
                / 10 as libc::c_int as libc::c_long) + '0' as i32 as libc::c_long)
            as libc::c_char;
        n
            %= 10 as libc::c_int as libc::c_long * 1000000000 as libc::c_int as wgint
                / 10 as libc::c_int as libc::c_long / 10 as libc::c_int as libc::c_long
                / 10 as libc::c_int as libc::c_long / 10 as libc::c_int as libc::c_long
                / 10 as libc::c_int as libc::c_long / 10 as libc::c_int as libc::c_long
                / 10 as libc::c_int as libc::c_long / 10 as libc::c_int as libc::c_long
                / 10 as libc::c_int as libc::c_long;
        let fresh80 = p;
        p = p.offset(1);
        *fresh80 = (n
            / (10 as libc::c_int as libc::c_long * 1000000000 as libc::c_int as wgint
                / 10 as libc::c_int as libc::c_long / 10 as libc::c_int as libc::c_long
                / 10 as libc::c_int as libc::c_long / 10 as libc::c_int as libc::c_long
                / 10 as libc::c_int as libc::c_long / 10 as libc::c_int as libc::c_long
                / 10 as libc::c_int as libc::c_long / 10 as libc::c_int as libc::c_long
                / 10 as libc::c_int as libc::c_long / 10 as libc::c_int as libc::c_long)
            + '0' as i32 as libc::c_long) as libc::c_char;
    } else if n
        < 1000 as libc::c_int as libc::c_long * 1000000000 as libc::c_int as wgint
    {
        let fresh81 = p;
        p = p.offset(1);
        *fresh81 = (n
            / (100 as libc::c_int as libc::c_long * 1000000000 as libc::c_int as wgint)
            + '0' as i32 as libc::c_long) as libc::c_char;
        n %= 100 as libc::c_int as libc::c_long * 1000000000 as libc::c_int as wgint;
        let fresh82 = p;
        p = p.offset(1);
        *fresh82 = (n
            / (100 as libc::c_int as libc::c_long * 1000000000 as libc::c_int as wgint
                / 10 as libc::c_int as libc::c_long) + '0' as i32 as libc::c_long)
            as libc::c_char;
        n
            %= 100 as libc::c_int as libc::c_long * 1000000000 as libc::c_int as wgint
                / 10 as libc::c_int as libc::c_long;
        let fresh83 = p;
        p = p.offset(1);
        *fresh83 = (n
            / (100 as libc::c_int as libc::c_long * 1000000000 as libc::c_int as wgint
                / 10 as libc::c_int as libc::c_long / 10 as libc::c_int as libc::c_long)
            + '0' as i32 as libc::c_long) as libc::c_char;
        n
            %= 100 as libc::c_int as libc::c_long * 1000000000 as libc::c_int as wgint
                / 10 as libc::c_int as libc::c_long / 10 as libc::c_int as libc::c_long;
        let fresh84 = p;
        p = p.offset(1);
        *fresh84 = (n
            / (100 as libc::c_int as libc::c_long * 1000000000 as libc::c_int as wgint
                / 10 as libc::c_int as libc::c_long / 10 as libc::c_int as libc::c_long
                / 10 as libc::c_int as libc::c_long) + '0' as i32 as libc::c_long)
            as libc::c_char;
        n
            %= 100 as libc::c_int as libc::c_long * 1000000000 as libc::c_int as wgint
                / 10 as libc::c_int as libc::c_long / 10 as libc::c_int as libc::c_long
                / 10 as libc::c_int as libc::c_long;
        let fresh85 = p;
        p = p.offset(1);
        *fresh85 = (n
            / (100 as libc::c_int as libc::c_long * 1000000000 as libc::c_int as wgint
                / 10 as libc::c_int as libc::c_long / 10 as libc::c_int as libc::c_long
                / 10 as libc::c_int as libc::c_long / 10 as libc::c_int as libc::c_long)
            + '0' as i32 as libc::c_long) as libc::c_char;
        n
            %= 100 as libc::c_int as libc::c_long * 1000000000 as libc::c_int as wgint
                / 10 as libc::c_int as libc::c_long / 10 as libc::c_int as libc::c_long
                / 10 as libc::c_int as libc::c_long / 10 as libc::c_int as libc::c_long;
        let fresh86 = p;
        p = p.offset(1);
        *fresh86 = (n
            / (100 as libc::c_int as libc::c_long * 1000000000 as libc::c_int as wgint
                / 10 as libc::c_int as libc::c_long / 10 as libc::c_int as libc::c_long
                / 10 as libc::c_int as libc::c_long / 10 as libc::c_int as libc::c_long
                / 10 as libc::c_int as libc::c_long) + '0' as i32 as libc::c_long)
            as libc::c_char;
        n
            %= 100 as libc::c_int as libc::c_long * 1000000000 as libc::c_int as wgint
                / 10 as libc::c_int as libc::c_long / 10 as libc::c_int as libc::c_long
                / 10 as libc::c_int as libc::c_long / 10 as libc::c_int as libc::c_long
                / 10 as libc::c_int as libc::c_long;
        let fresh87 = p;
        p = p.offset(1);
        *fresh87 = (n
            / (100 as libc::c_int as libc::c_long * 1000000000 as libc::c_int as wgint
                / 10 as libc::c_int as libc::c_long / 10 as libc::c_int as libc::c_long
                / 10 as libc::c_int as libc::c_long / 10 as libc::c_int as libc::c_long
                / 10 as libc::c_int as libc::c_long / 10 as libc::c_int as libc::c_long)
            + '0' as i32 as libc::c_long) as libc::c_char;
        n
            %= 100 as libc::c_int as libc::c_long * 1000000000 as libc::c_int as wgint
                / 10 as libc::c_int as libc::c_long / 10 as libc::c_int as libc::c_long
                / 10 as libc::c_int as libc::c_long / 10 as libc::c_int as libc::c_long
                / 10 as libc::c_int as libc::c_long / 10 as libc::c_int as libc::c_long;
        let fresh88 = p;
        p = p.offset(1);
        *fresh88 = (n
            / (100 as libc::c_int as libc::c_long * 1000000000 as libc::c_int as wgint
                / 10 as libc::c_int as libc::c_long / 10 as libc::c_int as libc::c_long
                / 10 as libc::c_int as libc::c_long / 10 as libc::c_int as libc::c_long
                / 10 as libc::c_int as libc::c_long / 10 as libc::c_int as libc::c_long
                / 10 as libc::c_int as libc::c_long) + '0' as i32 as libc::c_long)
            as libc::c_char;
        n
            %= 100 as libc::c_int as libc::c_long * 1000000000 as libc::c_int as wgint
                / 10 as libc::c_int as libc::c_long / 10 as libc::c_int as libc::c_long
                / 10 as libc::c_int as libc::c_long / 10 as libc::c_int as libc::c_long
                / 10 as libc::c_int as libc::c_long / 10 as libc::c_int as libc::c_long
                / 10 as libc::c_int as libc::c_long;
        let fresh89 = p;
        p = p.offset(1);
        *fresh89 = (n
            / (100 as libc::c_int as libc::c_long * 1000000000 as libc::c_int as wgint
                / 10 as libc::c_int as libc::c_long / 10 as libc::c_int as libc::c_long
                / 10 as libc::c_int as libc::c_long / 10 as libc::c_int as libc::c_long
                / 10 as libc::c_int as libc::c_long / 10 as libc::c_int as libc::c_long
                / 10 as libc::c_int as libc::c_long / 10 as libc::c_int as libc::c_long)
            + '0' as i32 as libc::c_long) as libc::c_char;
        n
            %= 100 as libc::c_int as libc::c_long * 1000000000 as libc::c_int as wgint
                / 10 as libc::c_int as libc::c_long / 10 as libc::c_int as libc::c_long
                / 10 as libc::c_int as libc::c_long / 10 as libc::c_int as libc::c_long
                / 10 as libc::c_int as libc::c_long / 10 as libc::c_int as libc::c_long
                / 10 as libc::c_int as libc::c_long / 10 as libc::c_int as libc::c_long;
        let fresh90 = p;
        p = p.offset(1);
        *fresh90 = (n
            / (100 as libc::c_int as libc::c_long * 1000000000 as libc::c_int as wgint
                / 10 as libc::c_int as libc::c_long / 10 as libc::c_int as libc::c_long
                / 10 as libc::c_int as libc::c_long / 10 as libc::c_int as libc::c_long
                / 10 as libc::c_int as libc::c_long / 10 as libc::c_int as libc::c_long
                / 10 as libc::c_int as libc::c_long / 10 as libc::c_int as libc::c_long
                / 10 as libc::c_int as libc::c_long) + '0' as i32 as libc::c_long)
            as libc::c_char;
        n
            %= 100 as libc::c_int as libc::c_long * 1000000000 as libc::c_int as wgint
                / 10 as libc::c_int as libc::c_long / 10 as libc::c_int as libc::c_long
                / 10 as libc::c_int as libc::c_long / 10 as libc::c_int as libc::c_long
                / 10 as libc::c_int as libc::c_long / 10 as libc::c_int as libc::c_long
                / 10 as libc::c_int as libc::c_long / 10 as libc::c_int as libc::c_long
                / 10 as libc::c_int as libc::c_long;
        let fresh91 = p;
        p = p.offset(1);
        *fresh91 = (n
            / (100 as libc::c_int as libc::c_long * 1000000000 as libc::c_int as wgint
                / 10 as libc::c_int as libc::c_long / 10 as libc::c_int as libc::c_long
                / 10 as libc::c_int as libc::c_long / 10 as libc::c_int as libc::c_long
                / 10 as libc::c_int as libc::c_long / 10 as libc::c_int as libc::c_long
                / 10 as libc::c_int as libc::c_long / 10 as libc::c_int as libc::c_long
                / 10 as libc::c_int as libc::c_long / 10 as libc::c_int as libc::c_long)
            + '0' as i32 as libc::c_long) as libc::c_char;
        n
            %= 100 as libc::c_int as libc::c_long * 1000000000 as libc::c_int as wgint
                / 10 as libc::c_int as libc::c_long / 10 as libc::c_int as libc::c_long
                / 10 as libc::c_int as libc::c_long / 10 as libc::c_int as libc::c_long
                / 10 as libc::c_int as libc::c_long / 10 as libc::c_int as libc::c_long
                / 10 as libc::c_int as libc::c_long / 10 as libc::c_int as libc::c_long
                / 10 as libc::c_int as libc::c_long / 10 as libc::c_int as libc::c_long;
        let fresh92 = p;
        p = p.offset(1);
        *fresh92 = (n
            / (100 as libc::c_int as libc::c_long * 1000000000 as libc::c_int as wgint
                / 10 as libc::c_int as libc::c_long / 10 as libc::c_int as libc::c_long
                / 10 as libc::c_int as libc::c_long / 10 as libc::c_int as libc::c_long
                / 10 as libc::c_int as libc::c_long / 10 as libc::c_int as libc::c_long
                / 10 as libc::c_int as libc::c_long / 10 as libc::c_int as libc::c_long
                / 10 as libc::c_int as libc::c_long / 10 as libc::c_int as libc::c_long
                / 10 as libc::c_int as libc::c_long) + '0' as i32 as libc::c_long)
            as libc::c_char;
    } else if n
        < 10000 as libc::c_int as libc::c_long * 1000000000 as libc::c_int as wgint
    {
        let fresh93 = p;
        p = p.offset(1);
        *fresh93 = (n
            / (1000 as libc::c_int as libc::c_long * 1000000000 as libc::c_int as wgint)
            + '0' as i32 as libc::c_long) as libc::c_char;
        n %= 1000 as libc::c_int as libc::c_long * 1000000000 as libc::c_int as wgint;
        let fresh94 = p;
        p = p.offset(1);
        *fresh94 = (n
            / (1000 as libc::c_int as libc::c_long * 1000000000 as libc::c_int as wgint
                / 10 as libc::c_int as libc::c_long) + '0' as i32 as libc::c_long)
            as libc::c_char;
        n
            %= 1000 as libc::c_int as libc::c_long * 1000000000 as libc::c_int as wgint
                / 10 as libc::c_int as libc::c_long;
        let fresh95 = p;
        p = p.offset(1);
        *fresh95 = (n
            / (1000 as libc::c_int as libc::c_long * 1000000000 as libc::c_int as wgint
                / 10 as libc::c_int as libc::c_long / 10 as libc::c_int as libc::c_long)
            + '0' as i32 as libc::c_long) as libc::c_char;
        n
            %= 1000 as libc::c_int as libc::c_long * 1000000000 as libc::c_int as wgint
                / 10 as libc::c_int as libc::c_long / 10 as libc::c_int as libc::c_long;
        let fresh96 = p;
        p = p.offset(1);
        *fresh96 = (n
            / (1000 as libc::c_int as libc::c_long * 1000000000 as libc::c_int as wgint
                / 10 as libc::c_int as libc::c_long / 10 as libc::c_int as libc::c_long
                / 10 as libc::c_int as libc::c_long) + '0' as i32 as libc::c_long)
            as libc::c_char;
        n
            %= 1000 as libc::c_int as libc::c_long * 1000000000 as libc::c_int as wgint
                / 10 as libc::c_int as libc::c_long / 10 as libc::c_int as libc::c_long
                / 10 as libc::c_int as libc::c_long;
        let fresh97 = p;
        p = p.offset(1);
        *fresh97 = (n
            / (1000 as libc::c_int as libc::c_long * 1000000000 as libc::c_int as wgint
                / 10 as libc::c_int as libc::c_long / 10 as libc::c_int as libc::c_long
                / 10 as libc::c_int as libc::c_long / 10 as libc::c_int as libc::c_long)
            + '0' as i32 as libc::c_long) as libc::c_char;
        n
            %= 1000 as libc::c_int as libc::c_long * 1000000000 as libc::c_int as wgint
                / 10 as libc::c_int as libc::c_long / 10 as libc::c_int as libc::c_long
                / 10 as libc::c_int as libc::c_long / 10 as libc::c_int as libc::c_long;
        let fresh98 = p;
        p = p.offset(1);
        *fresh98 = (n
            / (1000 as libc::c_int as libc::c_long * 1000000000 as libc::c_int as wgint
                / 10 as libc::c_int as libc::c_long / 10 as libc::c_int as libc::c_long
                / 10 as libc::c_int as libc::c_long / 10 as libc::c_int as libc::c_long
                / 10 as libc::c_int as libc::c_long) + '0' as i32 as libc::c_long)
            as libc::c_char;
        n
            %= 1000 as libc::c_int as libc::c_long * 1000000000 as libc::c_int as wgint
                / 10 as libc::c_int as libc::c_long / 10 as libc::c_int as libc::c_long
                / 10 as libc::c_int as libc::c_long / 10 as libc::c_int as libc::c_long
                / 10 as libc::c_int as libc::c_long;
        let fresh99 = p;
        p = p.offset(1);
        *fresh99 = (n
            / (1000 as libc::c_int as libc::c_long * 1000000000 as libc::c_int as wgint
                / 10 as libc::c_int as libc::c_long / 10 as libc::c_int as libc::c_long
                / 10 as libc::c_int as libc::c_long / 10 as libc::c_int as libc::c_long
                / 10 as libc::c_int as libc::c_long / 10 as libc::c_int as libc::c_long)
            + '0' as i32 as libc::c_long) as libc::c_char;
        n
            %= 1000 as libc::c_int as libc::c_long * 1000000000 as libc::c_int as wgint
                / 10 as libc::c_int as libc::c_long / 10 as libc::c_int as libc::c_long
                / 10 as libc::c_int as libc::c_long / 10 as libc::c_int as libc::c_long
                / 10 as libc::c_int as libc::c_long / 10 as libc::c_int as libc::c_long;
        let fresh100 = p;
        p = p.offset(1);
        *fresh100 = (n
            / (1000 as libc::c_int as libc::c_long * 1000000000 as libc::c_int as wgint
                / 10 as libc::c_int as libc::c_long / 10 as libc::c_int as libc::c_long
                / 10 as libc::c_int as libc::c_long / 10 as libc::c_int as libc::c_long
                / 10 as libc::c_int as libc::c_long / 10 as libc::c_int as libc::c_long
                / 10 as libc::c_int as libc::c_long) + '0' as i32 as libc::c_long)
            as libc::c_char;
        n
            %= 1000 as libc::c_int as libc::c_long * 1000000000 as libc::c_int as wgint
                / 10 as libc::c_int as libc::c_long / 10 as libc::c_int as libc::c_long
                / 10 as libc::c_int as libc::c_long / 10 as libc::c_int as libc::c_long
                / 10 as libc::c_int as libc::c_long / 10 as libc::c_int as libc::c_long
                / 10 as libc::c_int as libc::c_long;
        let fresh101 = p;
        p = p.offset(1);
        *fresh101 = (n
            / (1000 as libc::c_int as libc::c_long * 1000000000 as libc::c_int as wgint
                / 10 as libc::c_int as libc::c_long / 10 as libc::c_int as libc::c_long
                / 10 as libc::c_int as libc::c_long / 10 as libc::c_int as libc::c_long
                / 10 as libc::c_int as libc::c_long / 10 as libc::c_int as libc::c_long
                / 10 as libc::c_int as libc::c_long / 10 as libc::c_int as libc::c_long)
            + '0' as i32 as libc::c_long) as libc::c_char;
        n
            %= 1000 as libc::c_int as libc::c_long * 1000000000 as libc::c_int as wgint
                / 10 as libc::c_int as libc::c_long / 10 as libc::c_int as libc::c_long
                / 10 as libc::c_int as libc::c_long / 10 as libc::c_int as libc::c_long
                / 10 as libc::c_int as libc::c_long / 10 as libc::c_int as libc::c_long
                / 10 as libc::c_int as libc::c_long / 10 as libc::c_int as libc::c_long;
        let fresh102 = p;
        p = p.offset(1);
        *fresh102 = (n
            / (1000 as libc::c_int as libc::c_long * 1000000000 as libc::c_int as wgint
                / 10 as libc::c_int as libc::c_long / 10 as libc::c_int as libc::c_long
                / 10 as libc::c_int as libc::c_long / 10 as libc::c_int as libc::c_long
                / 10 as libc::c_int as libc::c_long / 10 as libc::c_int as libc::c_long
                / 10 as libc::c_int as libc::c_long / 10 as libc::c_int as libc::c_long
                / 10 as libc::c_int as libc::c_long) + '0' as i32 as libc::c_long)
            as libc::c_char;
        n
            %= 1000 as libc::c_int as libc::c_long * 1000000000 as libc::c_int as wgint
                / 10 as libc::c_int as libc::c_long / 10 as libc::c_int as libc::c_long
                / 10 as libc::c_int as libc::c_long / 10 as libc::c_int as libc::c_long
                / 10 as libc::c_int as libc::c_long / 10 as libc::c_int as libc::c_long
                / 10 as libc::c_int as libc::c_long / 10 as libc::c_int as libc::c_long
                / 10 as libc::c_int as libc::c_long;
        let fresh103 = p;
        p = p.offset(1);
        *fresh103 = (n
            / (1000 as libc::c_int as libc::c_long * 1000000000 as libc::c_int as wgint
                / 10 as libc::c_int as libc::c_long / 10 as libc::c_int as libc::c_long
                / 10 as libc::c_int as libc::c_long / 10 as libc::c_int as libc::c_long
                / 10 as libc::c_int as libc::c_long / 10 as libc::c_int as libc::c_long
                / 10 as libc::c_int as libc::c_long / 10 as libc::c_int as libc::c_long
                / 10 as libc::c_int as libc::c_long / 10 as libc::c_int as libc::c_long)
            + '0' as i32 as libc::c_long) as libc::c_char;
        n
            %= 1000 as libc::c_int as libc::c_long * 1000000000 as libc::c_int as wgint
                / 10 as libc::c_int as libc::c_long / 10 as libc::c_int as libc::c_long
                / 10 as libc::c_int as libc::c_long / 10 as libc::c_int as libc::c_long
                / 10 as libc::c_int as libc::c_long / 10 as libc::c_int as libc::c_long
                / 10 as libc::c_int as libc::c_long / 10 as libc::c_int as libc::c_long
                / 10 as libc::c_int as libc::c_long / 10 as libc::c_int as libc::c_long;
        let fresh104 = p;
        p = p.offset(1);
        *fresh104 = (n
            / (1000 as libc::c_int as libc::c_long * 1000000000 as libc::c_int as wgint
                / 10 as libc::c_int as libc::c_long / 10 as libc::c_int as libc::c_long
                / 10 as libc::c_int as libc::c_long / 10 as libc::c_int as libc::c_long
                / 10 as libc::c_int as libc::c_long / 10 as libc::c_int as libc::c_long
                / 10 as libc::c_int as libc::c_long / 10 as libc::c_int as libc::c_long
                / 10 as libc::c_int as libc::c_long / 10 as libc::c_int as libc::c_long
                / 10 as libc::c_int as libc::c_long) + '0' as i32 as libc::c_long)
            as libc::c_char;
        n
            %= 1000 as libc::c_int as libc::c_long * 1000000000 as libc::c_int as wgint
                / 10 as libc::c_int as libc::c_long / 10 as libc::c_int as libc::c_long
                / 10 as libc::c_int as libc::c_long / 10 as libc::c_int as libc::c_long
                / 10 as libc::c_int as libc::c_long / 10 as libc::c_int as libc::c_long
                / 10 as libc::c_int as libc::c_long / 10 as libc::c_int as libc::c_long
                / 10 as libc::c_int as libc::c_long / 10 as libc::c_int as libc::c_long
                / 10 as libc::c_int as libc::c_long;
        let fresh105 = p;
        p = p.offset(1);
        *fresh105 = (n
            / (1000 as libc::c_int as libc::c_long * 1000000000 as libc::c_int as wgint
                / 10 as libc::c_int as libc::c_long / 10 as libc::c_int as libc::c_long
                / 10 as libc::c_int as libc::c_long / 10 as libc::c_int as libc::c_long
                / 10 as libc::c_int as libc::c_long / 10 as libc::c_int as libc::c_long
                / 10 as libc::c_int as libc::c_long / 10 as libc::c_int as libc::c_long
                / 10 as libc::c_int as libc::c_long / 10 as libc::c_int as libc::c_long
                / 10 as libc::c_int as libc::c_long / 10 as libc::c_int as libc::c_long)
            + '0' as i32 as libc::c_long) as libc::c_char;
    } else if n
        < 100000 as libc::c_int as libc::c_long * 1000000000 as libc::c_int as wgint
    {
        let fresh106 = p;
        p = p.offset(1);
        *fresh106 = (n
            / (10000 as libc::c_int as libc::c_long * 1000000000 as libc::c_int as wgint)
            + '0' as i32 as libc::c_long) as libc::c_char;
        n %= 10000 as libc::c_int as libc::c_long * 1000000000 as libc::c_int as wgint;
        let fresh107 = p;
        p = p.offset(1);
        *fresh107 = (n
            / (10000 as libc::c_int as libc::c_long * 1000000000 as libc::c_int as wgint
                / 10 as libc::c_int as libc::c_long) + '0' as i32 as libc::c_long)
            as libc::c_char;
        n
            %= 10000 as libc::c_int as libc::c_long * 1000000000 as libc::c_int as wgint
                / 10 as libc::c_int as libc::c_long;
        let fresh108 = p;
        p = p.offset(1);
        *fresh108 = (n
            / (10000 as libc::c_int as libc::c_long * 1000000000 as libc::c_int as wgint
                / 10 as libc::c_int as libc::c_long / 10 as libc::c_int as libc::c_long)
            + '0' as i32 as libc::c_long) as libc::c_char;
        n
            %= 10000 as libc::c_int as libc::c_long * 1000000000 as libc::c_int as wgint
                / 10 as libc::c_int as libc::c_long / 10 as libc::c_int as libc::c_long;
        let fresh109 = p;
        p = p.offset(1);
        *fresh109 = (n
            / (10000 as libc::c_int as libc::c_long * 1000000000 as libc::c_int as wgint
                / 10 as libc::c_int as libc::c_long / 10 as libc::c_int as libc::c_long
                / 10 as libc::c_int as libc::c_long) + '0' as i32 as libc::c_long)
            as libc::c_char;
        n
            %= 10000 as libc::c_int as libc::c_long * 1000000000 as libc::c_int as wgint
                / 10 as libc::c_int as libc::c_long / 10 as libc::c_int as libc::c_long
                / 10 as libc::c_int as libc::c_long;
        let fresh110 = p;
        p = p.offset(1);
        *fresh110 = (n
            / (10000 as libc::c_int as libc::c_long * 1000000000 as libc::c_int as wgint
                / 10 as libc::c_int as libc::c_long / 10 as libc::c_int as libc::c_long
                / 10 as libc::c_int as libc::c_long / 10 as libc::c_int as libc::c_long)
            + '0' as i32 as libc::c_long) as libc::c_char;
        n
            %= 10000 as libc::c_int as libc::c_long * 1000000000 as libc::c_int as wgint
                / 10 as libc::c_int as libc::c_long / 10 as libc::c_int as libc::c_long
                / 10 as libc::c_int as libc::c_long / 10 as libc::c_int as libc::c_long;
        let fresh111 = p;
        p = p.offset(1);
        *fresh111 = (n
            / (10000 as libc::c_int as libc::c_long * 1000000000 as libc::c_int as wgint
                / 10 as libc::c_int as libc::c_long / 10 as libc::c_int as libc::c_long
                / 10 as libc::c_int as libc::c_long / 10 as libc::c_int as libc::c_long
                / 10 as libc::c_int as libc::c_long) + '0' as i32 as libc::c_long)
            as libc::c_char;
        n
            %= 10000 as libc::c_int as libc::c_long * 1000000000 as libc::c_int as wgint
                / 10 as libc::c_int as libc::c_long / 10 as libc::c_int as libc::c_long
                / 10 as libc::c_int as libc::c_long / 10 as libc::c_int as libc::c_long
                / 10 as libc::c_int as libc::c_long;
        let fresh112 = p;
        p = p.offset(1);
        *fresh112 = (n
            / (10000 as libc::c_int as libc::c_long * 1000000000 as libc::c_int as wgint
                / 10 as libc::c_int as libc::c_long / 10 as libc::c_int as libc::c_long
                / 10 as libc::c_int as libc::c_long / 10 as libc::c_int as libc::c_long
                / 10 as libc::c_int as libc::c_long / 10 as libc::c_int as libc::c_long)
            + '0' as i32 as libc::c_long) as libc::c_char;
        n
            %= 10000 as libc::c_int as libc::c_long * 1000000000 as libc::c_int as wgint
                / 10 as libc::c_int as libc::c_long / 10 as libc::c_int as libc::c_long
                / 10 as libc::c_int as libc::c_long / 10 as libc::c_int as libc::c_long
                / 10 as libc::c_int as libc::c_long / 10 as libc::c_int as libc::c_long;
        let fresh113 = p;
        p = p.offset(1);
        *fresh113 = (n
            / (10000 as libc::c_int as libc::c_long * 1000000000 as libc::c_int as wgint
                / 10 as libc::c_int as libc::c_long / 10 as libc::c_int as libc::c_long
                / 10 as libc::c_int as libc::c_long / 10 as libc::c_int as libc::c_long
                / 10 as libc::c_int as libc::c_long / 10 as libc::c_int as libc::c_long
                / 10 as libc::c_int as libc::c_long) + '0' as i32 as libc::c_long)
            as libc::c_char;
        n
            %= 10000 as libc::c_int as libc::c_long * 1000000000 as libc::c_int as wgint
                / 10 as libc::c_int as libc::c_long / 10 as libc::c_int as libc::c_long
                / 10 as libc::c_int as libc::c_long / 10 as libc::c_int as libc::c_long
                / 10 as libc::c_int as libc::c_long / 10 as libc::c_int as libc::c_long
                / 10 as libc::c_int as libc::c_long;
        let fresh114 = p;
        p = p.offset(1);
        *fresh114 = (n
            / (10000 as libc::c_int as libc::c_long * 1000000000 as libc::c_int as wgint
                / 10 as libc::c_int as libc::c_long / 10 as libc::c_int as libc::c_long
                / 10 as libc::c_int as libc::c_long / 10 as libc::c_int as libc::c_long
                / 10 as libc::c_int as libc::c_long / 10 as libc::c_int as libc::c_long
                / 10 as libc::c_int as libc::c_long / 10 as libc::c_int as libc::c_long)
            + '0' as i32 as libc::c_long) as libc::c_char;
        n
            %= 10000 as libc::c_int as libc::c_long * 1000000000 as libc::c_int as wgint
                / 10 as libc::c_int as libc::c_long / 10 as libc::c_int as libc::c_long
                / 10 as libc::c_int as libc::c_long / 10 as libc::c_int as libc::c_long
                / 10 as libc::c_int as libc::c_long / 10 as libc::c_int as libc::c_long
                / 10 as libc::c_int as libc::c_long / 10 as libc::c_int as libc::c_long;
        let fresh115 = p;
        p = p.offset(1);
        *fresh115 = (n
            / (10000 as libc::c_int as libc::c_long * 1000000000 as libc::c_int as wgint
                / 10 as libc::c_int as libc::c_long / 10 as libc::c_int as libc::c_long
                / 10 as libc::c_int as libc::c_long / 10 as libc::c_int as libc::c_long
                / 10 as libc::c_int as libc::c_long / 10 as libc::c_int as libc::c_long
                / 10 as libc::c_int as libc::c_long / 10 as libc::c_int as libc::c_long
                / 10 as libc::c_int as libc::c_long) + '0' as i32 as libc::c_long)
            as libc::c_char;
        n
            %= 10000 as libc::c_int as libc::c_long * 1000000000 as libc::c_int as wgint
                / 10 as libc::c_int as libc::c_long / 10 as libc::c_int as libc::c_long
                / 10 as libc::c_int as libc::c_long / 10 as libc::c_int as libc::c_long
                / 10 as libc::c_int as libc::c_long / 10 as libc::c_int as libc::c_long
                / 10 as libc::c_int as libc::c_long / 10 as libc::c_int as libc::c_long
                / 10 as libc::c_int as libc::c_long;
        let fresh116 = p;
        p = p.offset(1);
        *fresh116 = (n
            / (10000 as libc::c_int as libc::c_long * 1000000000 as libc::c_int as wgint
                / 10 as libc::c_int as libc::c_long / 10 as libc::c_int as libc::c_long
                / 10 as libc::c_int as libc::c_long / 10 as libc::c_int as libc::c_long
                / 10 as libc::c_int as libc::c_long / 10 as libc::c_int as libc::c_long
                / 10 as libc::c_int as libc::c_long / 10 as libc::c_int as libc::c_long
                / 10 as libc::c_int as libc::c_long / 10 as libc::c_int as libc::c_long)
            + '0' as i32 as libc::c_long) as libc::c_char;
        n
            %= 10000 as libc::c_int as libc::c_long * 1000000000 as libc::c_int as wgint
                / 10 as libc::c_int as libc::c_long / 10 as libc::c_int as libc::c_long
                / 10 as libc::c_int as libc::c_long / 10 as libc::c_int as libc::c_long
                / 10 as libc::c_int as libc::c_long / 10 as libc::c_int as libc::c_long
                / 10 as libc::c_int as libc::c_long / 10 as libc::c_int as libc::c_long
                / 10 as libc::c_int as libc::c_long / 10 as libc::c_int as libc::c_long;
        let fresh117 = p;
        p = p.offset(1);
        *fresh117 = (n
            / (10000 as libc::c_int as libc::c_long * 1000000000 as libc::c_int as wgint
                / 10 as libc::c_int as libc::c_long / 10 as libc::c_int as libc::c_long
                / 10 as libc::c_int as libc::c_long / 10 as libc::c_int as libc::c_long
                / 10 as libc::c_int as libc::c_long / 10 as libc::c_int as libc::c_long
                / 10 as libc::c_int as libc::c_long / 10 as libc::c_int as libc::c_long
                / 10 as libc::c_int as libc::c_long / 10 as libc::c_int as libc::c_long
                / 10 as libc::c_int as libc::c_long) + '0' as i32 as libc::c_long)
            as libc::c_char;
        n
            %= 10000 as libc::c_int as libc::c_long * 1000000000 as libc::c_int as wgint
                / 10 as libc::c_int as libc::c_long / 10 as libc::c_int as libc::c_long
                / 10 as libc::c_int as libc::c_long / 10 as libc::c_int as libc::c_long
                / 10 as libc::c_int as libc::c_long / 10 as libc::c_int as libc::c_long
                / 10 as libc::c_int as libc::c_long / 10 as libc::c_int as libc::c_long
                / 10 as libc::c_int as libc::c_long / 10 as libc::c_int as libc::c_long
                / 10 as libc::c_int as libc::c_long;
        let fresh118 = p;
        p = p.offset(1);
        *fresh118 = (n
            / (10000 as libc::c_int as libc::c_long * 1000000000 as libc::c_int as wgint
                / 10 as libc::c_int as libc::c_long / 10 as libc::c_int as libc::c_long
                / 10 as libc::c_int as libc::c_long / 10 as libc::c_int as libc::c_long
                / 10 as libc::c_int as libc::c_long / 10 as libc::c_int as libc::c_long
                / 10 as libc::c_int as libc::c_long / 10 as libc::c_int as libc::c_long
                / 10 as libc::c_int as libc::c_long / 10 as libc::c_int as libc::c_long
                / 10 as libc::c_int as libc::c_long / 10 as libc::c_int as libc::c_long)
            + '0' as i32 as libc::c_long) as libc::c_char;
        n
            %= 10000 as libc::c_int as libc::c_long * 1000000000 as libc::c_int as wgint
                / 10 as libc::c_int as libc::c_long / 10 as libc::c_int as libc::c_long
                / 10 as libc::c_int as libc::c_long / 10 as libc::c_int as libc::c_long
                / 10 as libc::c_int as libc::c_long / 10 as libc::c_int as libc::c_long
                / 10 as libc::c_int as libc::c_long / 10 as libc::c_int as libc::c_long
                / 10 as libc::c_int as libc::c_long / 10 as libc::c_int as libc::c_long
                / 10 as libc::c_int as libc::c_long / 10 as libc::c_int as libc::c_long;
        let fresh119 = p;
        p = p.offset(1);
        *fresh119 = (n
            / (10000 as libc::c_int as libc::c_long * 1000000000 as libc::c_int as wgint
                / 10 as libc::c_int as libc::c_long / 10 as libc::c_int as libc::c_long
                / 10 as libc::c_int as libc::c_long / 10 as libc::c_int as libc::c_long
                / 10 as libc::c_int as libc::c_long / 10 as libc::c_int as libc::c_long
                / 10 as libc::c_int as libc::c_long / 10 as libc::c_int as libc::c_long
                / 10 as libc::c_int as libc::c_long / 10 as libc::c_int as libc::c_long
                / 10 as libc::c_int as libc::c_long / 10 as libc::c_int as libc::c_long
                / 10 as libc::c_int as libc::c_long) + '0' as i32 as libc::c_long)
            as libc::c_char;
    } else if n
        < 1000000 as libc::c_int as libc::c_long * 1000000000 as libc::c_int as wgint
    {
        let fresh120 = p;
        p = p.offset(1);
        *fresh120 = (n
            / (100000 as libc::c_int as libc::c_long
                * 1000000000 as libc::c_int as wgint) + '0' as i32 as libc::c_long)
            as libc::c_char;
        n %= 100000 as libc::c_int as libc::c_long * 1000000000 as libc::c_int as wgint;
        let fresh121 = p;
        p = p.offset(1);
        *fresh121 = (n
            / (100000 as libc::c_int as libc::c_long * 1000000000 as libc::c_int as wgint
                / 10 as libc::c_int as libc::c_long) + '0' as i32 as libc::c_long)
            as libc::c_char;
        n
            %= 100000 as libc::c_int as libc::c_long * 1000000000 as libc::c_int as wgint
                / 10 as libc::c_int as libc::c_long;
        let fresh122 = p;
        p = p.offset(1);
        *fresh122 = (n
            / (100000 as libc::c_int as libc::c_long * 1000000000 as libc::c_int as wgint
                / 10 as libc::c_int as libc::c_long / 10 as libc::c_int as libc::c_long)
            + '0' as i32 as libc::c_long) as libc::c_char;
        n
            %= 100000 as libc::c_int as libc::c_long * 1000000000 as libc::c_int as wgint
                / 10 as libc::c_int as libc::c_long / 10 as libc::c_int as libc::c_long;
        let fresh123 = p;
        p = p.offset(1);
        *fresh123 = (n
            / (100000 as libc::c_int as libc::c_long * 1000000000 as libc::c_int as wgint
                / 10 as libc::c_int as libc::c_long / 10 as libc::c_int as libc::c_long
                / 10 as libc::c_int as libc::c_long) + '0' as i32 as libc::c_long)
            as libc::c_char;
        n
            %= 100000 as libc::c_int as libc::c_long * 1000000000 as libc::c_int as wgint
                / 10 as libc::c_int as libc::c_long / 10 as libc::c_int as libc::c_long
                / 10 as libc::c_int as libc::c_long;
        let fresh124 = p;
        p = p.offset(1);
        *fresh124 = (n
            / (100000 as libc::c_int as libc::c_long * 1000000000 as libc::c_int as wgint
                / 10 as libc::c_int as libc::c_long / 10 as libc::c_int as libc::c_long
                / 10 as libc::c_int as libc::c_long / 10 as libc::c_int as libc::c_long)
            + '0' as i32 as libc::c_long) as libc::c_char;
        n
            %= 100000 as libc::c_int as libc::c_long * 1000000000 as libc::c_int as wgint
                / 10 as libc::c_int as libc::c_long / 10 as libc::c_int as libc::c_long
                / 10 as libc::c_int as libc::c_long / 10 as libc::c_int as libc::c_long;
        let fresh125 = p;
        p = p.offset(1);
        *fresh125 = (n
            / (100000 as libc::c_int as libc::c_long * 1000000000 as libc::c_int as wgint
                / 10 as libc::c_int as libc::c_long / 10 as libc::c_int as libc::c_long
                / 10 as libc::c_int as libc::c_long / 10 as libc::c_int as libc::c_long
                / 10 as libc::c_int as libc::c_long) + '0' as i32 as libc::c_long)
            as libc::c_char;
        n
            %= 100000 as libc::c_int as libc::c_long * 1000000000 as libc::c_int as wgint
                / 10 as libc::c_int as libc::c_long / 10 as libc::c_int as libc::c_long
                / 10 as libc::c_int as libc::c_long / 10 as libc::c_int as libc::c_long
                / 10 as libc::c_int as libc::c_long;
        let fresh126 = p;
        p = p.offset(1);
        *fresh126 = (n
            / (100000 as libc::c_int as libc::c_long * 1000000000 as libc::c_int as wgint
                / 10 as libc::c_int as libc::c_long / 10 as libc::c_int as libc::c_long
                / 10 as libc::c_int as libc::c_long / 10 as libc::c_int as libc::c_long
                / 10 as libc::c_int as libc::c_long / 10 as libc::c_int as libc::c_long)
            + '0' as i32 as libc::c_long) as libc::c_char;
        n
            %= 100000 as libc::c_int as libc::c_long * 1000000000 as libc::c_int as wgint
                / 10 as libc::c_int as libc::c_long / 10 as libc::c_int as libc::c_long
                / 10 as libc::c_int as libc::c_long / 10 as libc::c_int as libc::c_long
                / 10 as libc::c_int as libc::c_long / 10 as libc::c_int as libc::c_long;
        let fresh127 = p;
        p = p.offset(1);
        *fresh127 = (n
            / (100000 as libc::c_int as libc::c_long * 1000000000 as libc::c_int as wgint
                / 10 as libc::c_int as libc::c_long / 10 as libc::c_int as libc::c_long
                / 10 as libc::c_int as libc::c_long / 10 as libc::c_int as libc::c_long
                / 10 as libc::c_int as libc::c_long / 10 as libc::c_int as libc::c_long
                / 10 as libc::c_int as libc::c_long) + '0' as i32 as libc::c_long)
            as libc::c_char;
        n
            %= 100000 as libc::c_int as libc::c_long * 1000000000 as libc::c_int as wgint
                / 10 as libc::c_int as libc::c_long / 10 as libc::c_int as libc::c_long
                / 10 as libc::c_int as libc::c_long / 10 as libc::c_int as libc::c_long
                / 10 as libc::c_int as libc::c_long / 10 as libc::c_int as libc::c_long
                / 10 as libc::c_int as libc::c_long;
        let fresh128 = p;
        p = p.offset(1);
        *fresh128 = (n
            / (100000 as libc::c_int as libc::c_long * 1000000000 as libc::c_int as wgint
                / 10 as libc::c_int as libc::c_long / 10 as libc::c_int as libc::c_long
                / 10 as libc::c_int as libc::c_long / 10 as libc::c_int as libc::c_long
                / 10 as libc::c_int as libc::c_long / 10 as libc::c_int as libc::c_long
                / 10 as libc::c_int as libc::c_long / 10 as libc::c_int as libc::c_long)
            + '0' as i32 as libc::c_long) as libc::c_char;
        n
            %= 100000 as libc::c_int as libc::c_long * 1000000000 as libc::c_int as wgint
                / 10 as libc::c_int as libc::c_long / 10 as libc::c_int as libc::c_long
                / 10 as libc::c_int as libc::c_long / 10 as libc::c_int as libc::c_long
                / 10 as libc::c_int as libc::c_long / 10 as libc::c_int as libc::c_long
                / 10 as libc::c_int as libc::c_long / 10 as libc::c_int as libc::c_long;
        let fresh129 = p;
        p = p.offset(1);
        *fresh129 = (n
            / (100000 as libc::c_int as libc::c_long * 1000000000 as libc::c_int as wgint
                / 10 as libc::c_int as libc::c_long / 10 as libc::c_int as libc::c_long
                / 10 as libc::c_int as libc::c_long / 10 as libc::c_int as libc::c_long
                / 10 as libc::c_int as libc::c_long / 10 as libc::c_int as libc::c_long
                / 10 as libc::c_int as libc::c_long / 10 as libc::c_int as libc::c_long
                / 10 as libc::c_int as libc::c_long) + '0' as i32 as libc::c_long)
            as libc::c_char;
        n
            %= 100000 as libc::c_int as libc::c_long * 1000000000 as libc::c_int as wgint
                / 10 as libc::c_int as libc::c_long / 10 as libc::c_int as libc::c_long
                / 10 as libc::c_int as libc::c_long / 10 as libc::c_int as libc::c_long
                / 10 as libc::c_int as libc::c_long / 10 as libc::c_int as libc::c_long
                / 10 as libc::c_int as libc::c_long / 10 as libc::c_int as libc::c_long
                / 10 as libc::c_int as libc::c_long;
        let fresh130 = p;
        p = p.offset(1);
        *fresh130 = (n
            / (100000 as libc::c_int as libc::c_long * 1000000000 as libc::c_int as wgint
                / 10 as libc::c_int as libc::c_long / 10 as libc::c_int as libc::c_long
                / 10 as libc::c_int as libc::c_long / 10 as libc::c_int as libc::c_long
                / 10 as libc::c_int as libc::c_long / 10 as libc::c_int as libc::c_long
                / 10 as libc::c_int as libc::c_long / 10 as libc::c_int as libc::c_long
                / 10 as libc::c_int as libc::c_long / 10 as libc::c_int as libc::c_long)
            + '0' as i32 as libc::c_long) as libc::c_char;
        n
            %= 100000 as libc::c_int as libc::c_long * 1000000000 as libc::c_int as wgint
                / 10 as libc::c_int as libc::c_long / 10 as libc::c_int as libc::c_long
                / 10 as libc::c_int as libc::c_long / 10 as libc::c_int as libc::c_long
                / 10 as libc::c_int as libc::c_long / 10 as libc::c_int as libc::c_long
                / 10 as libc::c_int as libc::c_long / 10 as libc::c_int as libc::c_long
                / 10 as libc::c_int as libc::c_long / 10 as libc::c_int as libc::c_long;
        let fresh131 = p;
        p = p.offset(1);
        *fresh131 = (n
            / (100000 as libc::c_int as libc::c_long * 1000000000 as libc::c_int as wgint
                / 10 as libc::c_int as libc::c_long / 10 as libc::c_int as libc::c_long
                / 10 as libc::c_int as libc::c_long / 10 as libc::c_int as libc::c_long
                / 10 as libc::c_int as libc::c_long / 10 as libc::c_int as libc::c_long
                / 10 as libc::c_int as libc::c_long / 10 as libc::c_int as libc::c_long
                / 10 as libc::c_int as libc::c_long / 10 as libc::c_int as libc::c_long
                / 10 as libc::c_int as libc::c_long) + '0' as i32 as libc::c_long)
            as libc::c_char;
        n
            %= 100000 as libc::c_int as libc::c_long * 1000000000 as libc::c_int as wgint
                / 10 as libc::c_int as libc::c_long / 10 as libc::c_int as libc::c_long
                / 10 as libc::c_int as libc::c_long / 10 as libc::c_int as libc::c_long
                / 10 as libc::c_int as libc::c_long / 10 as libc::c_int as libc::c_long
                / 10 as libc::c_int as libc::c_long / 10 as libc::c_int as libc::c_long
                / 10 as libc::c_int as libc::c_long / 10 as libc::c_int as libc::c_long
                / 10 as libc::c_int as libc::c_long;
        let fresh132 = p;
        p = p.offset(1);
        *fresh132 = (n
            / (100000 as libc::c_int as libc::c_long * 1000000000 as libc::c_int as wgint
                / 10 as libc::c_int as libc::c_long / 10 as libc::c_int as libc::c_long
                / 10 as libc::c_int as libc::c_long / 10 as libc::c_int as libc::c_long
                / 10 as libc::c_int as libc::c_long / 10 as libc::c_int as libc::c_long
                / 10 as libc::c_int as libc::c_long / 10 as libc::c_int as libc::c_long
                / 10 as libc::c_int as libc::c_long / 10 as libc::c_int as libc::c_long
                / 10 as libc::c_int as libc::c_long / 10 as libc::c_int as libc::c_long)
            + '0' as i32 as libc::c_long) as libc::c_char;
        n
            %= 100000 as libc::c_int as libc::c_long * 1000000000 as libc::c_int as wgint
                / 10 as libc::c_int as libc::c_long / 10 as libc::c_int as libc::c_long
                / 10 as libc::c_int as libc::c_long / 10 as libc::c_int as libc::c_long
                / 10 as libc::c_int as libc::c_long / 10 as libc::c_int as libc::c_long
                / 10 as libc::c_int as libc::c_long / 10 as libc::c_int as libc::c_long
                / 10 as libc::c_int as libc::c_long / 10 as libc::c_int as libc::c_long
                / 10 as libc::c_int as libc::c_long / 10 as libc::c_int as libc::c_long;
        let fresh133 = p;
        p = p.offset(1);
        *fresh133 = (n
            / (100000 as libc::c_int as libc::c_long * 1000000000 as libc::c_int as wgint
                / 10 as libc::c_int as libc::c_long / 10 as libc::c_int as libc::c_long
                / 10 as libc::c_int as libc::c_long / 10 as libc::c_int as libc::c_long
                / 10 as libc::c_int as libc::c_long / 10 as libc::c_int as libc::c_long
                / 10 as libc::c_int as libc::c_long / 10 as libc::c_int as libc::c_long
                / 10 as libc::c_int as libc::c_long / 10 as libc::c_int as libc::c_long
                / 10 as libc::c_int as libc::c_long / 10 as libc::c_int as libc::c_long
                / 10 as libc::c_int as libc::c_long) + '0' as i32 as libc::c_long)
            as libc::c_char;
        n
            %= 100000 as libc::c_int as libc::c_long * 1000000000 as libc::c_int as wgint
                / 10 as libc::c_int as libc::c_long / 10 as libc::c_int as libc::c_long
                / 10 as libc::c_int as libc::c_long / 10 as libc::c_int as libc::c_long
                / 10 as libc::c_int as libc::c_long / 10 as libc::c_int as libc::c_long
                / 10 as libc::c_int as libc::c_long / 10 as libc::c_int as libc::c_long
                / 10 as libc::c_int as libc::c_long / 10 as libc::c_int as libc::c_long
                / 10 as libc::c_int as libc::c_long / 10 as libc::c_int as libc::c_long
                / 10 as libc::c_int as libc::c_long;
        let fresh134 = p;
        p = p.offset(1);
        *fresh134 = (n
            / (100000 as libc::c_int as libc::c_long * 1000000000 as libc::c_int as wgint
                / 10 as libc::c_int as libc::c_long / 10 as libc::c_int as libc::c_long
                / 10 as libc::c_int as libc::c_long / 10 as libc::c_int as libc::c_long
                / 10 as libc::c_int as libc::c_long / 10 as libc::c_int as libc::c_long
                / 10 as libc::c_int as libc::c_long / 10 as libc::c_int as libc::c_long
                / 10 as libc::c_int as libc::c_long / 10 as libc::c_int as libc::c_long
                / 10 as libc::c_int as libc::c_long / 10 as libc::c_int as libc::c_long
                / 10 as libc::c_int as libc::c_long / 10 as libc::c_int as libc::c_long)
            + '0' as i32 as libc::c_long) as libc::c_char;
    } else if n
        < 10000000 as libc::c_int as libc::c_long * 1000000000 as libc::c_int as wgint
    {
        let fresh135 = p;
        p = p.offset(1);
        *fresh135 = (n
            / (1000000 as libc::c_int as libc::c_long
                * 1000000000 as libc::c_int as wgint) + '0' as i32 as libc::c_long)
            as libc::c_char;
        n %= 1000000 as libc::c_int as libc::c_long * 1000000000 as libc::c_int as wgint;
        let fresh136 = p;
        p = p.offset(1);
        *fresh136 = (n
            / (1000000 as libc::c_int as libc::c_long
                * 1000000000 as libc::c_int as wgint / 10 as libc::c_int as libc::c_long)
            + '0' as i32 as libc::c_long) as libc::c_char;
        n
            %= 1000000 as libc::c_int as libc::c_long
                * 1000000000 as libc::c_int as wgint / 10 as libc::c_int as libc::c_long;
        let fresh137 = p;
        p = p.offset(1);
        *fresh137 = (n
            / (1000000 as libc::c_int as libc::c_long
                * 1000000000 as libc::c_int as wgint / 10 as libc::c_int as libc::c_long
                / 10 as libc::c_int as libc::c_long) + '0' as i32 as libc::c_long)
            as libc::c_char;
        n
            %= 1000000 as libc::c_int as libc::c_long
                * 1000000000 as libc::c_int as wgint / 10 as libc::c_int as libc::c_long
                / 10 as libc::c_int as libc::c_long;
        let fresh138 = p;
        p = p.offset(1);
        *fresh138 = (n
            / (1000000 as libc::c_int as libc::c_long
                * 1000000000 as libc::c_int as wgint / 10 as libc::c_int as libc::c_long
                / 10 as libc::c_int as libc::c_long / 10 as libc::c_int as libc::c_long)
            + '0' as i32 as libc::c_long) as libc::c_char;
        n
            %= 1000000 as libc::c_int as libc::c_long
                * 1000000000 as libc::c_int as wgint / 10 as libc::c_int as libc::c_long
                / 10 as libc::c_int as libc::c_long / 10 as libc::c_int as libc::c_long;
        let fresh139 = p;
        p = p.offset(1);
        *fresh139 = (n
            / (1000000 as libc::c_int as libc::c_long
                * 1000000000 as libc::c_int as wgint / 10 as libc::c_int as libc::c_long
                / 10 as libc::c_int as libc::c_long / 10 as libc::c_int as libc::c_long
                / 10 as libc::c_int as libc::c_long) + '0' as i32 as libc::c_long)
            as libc::c_char;
        n
            %= 1000000 as libc::c_int as libc::c_long
                * 1000000000 as libc::c_int as wgint / 10 as libc::c_int as libc::c_long
                / 10 as libc::c_int as libc::c_long / 10 as libc::c_int as libc::c_long
                / 10 as libc::c_int as libc::c_long;
        let fresh140 = p;
        p = p.offset(1);
        *fresh140 = (n
            / (1000000 as libc::c_int as libc::c_long
                * 1000000000 as libc::c_int as wgint / 10 as libc::c_int as libc::c_long
                / 10 as libc::c_int as libc::c_long / 10 as libc::c_int as libc::c_long
                / 10 as libc::c_int as libc::c_long / 10 as libc::c_int as libc::c_long)
            + '0' as i32 as libc::c_long) as libc::c_char;
        n
            %= 1000000 as libc::c_int as libc::c_long
                * 1000000000 as libc::c_int as wgint / 10 as libc::c_int as libc::c_long
                / 10 as libc::c_int as libc::c_long / 10 as libc::c_int as libc::c_long
                / 10 as libc::c_int as libc::c_long / 10 as libc::c_int as libc::c_long;
        let fresh141 = p;
        p = p.offset(1);
        *fresh141 = (n
            / (1000000 as libc::c_int as libc::c_long
                * 1000000000 as libc::c_int as wgint / 10 as libc::c_int as libc::c_long
                / 10 as libc::c_int as libc::c_long / 10 as libc::c_int as libc::c_long
                / 10 as libc::c_int as libc::c_long / 10 as libc::c_int as libc::c_long
                / 10 as libc::c_int as libc::c_long) + '0' as i32 as libc::c_long)
            as libc::c_char;
        n
            %= 1000000 as libc::c_int as libc::c_long
                * 1000000000 as libc::c_int as wgint / 10 as libc::c_int as libc::c_long
                / 10 as libc::c_int as libc::c_long / 10 as libc::c_int as libc::c_long
                / 10 as libc::c_int as libc::c_long / 10 as libc::c_int as libc::c_long
                / 10 as libc::c_int as libc::c_long;
        let fresh142 = p;
        p = p.offset(1);
        *fresh142 = (n
            / (1000000 as libc::c_int as libc::c_long
                * 1000000000 as libc::c_int as wgint / 10 as libc::c_int as libc::c_long
                / 10 as libc::c_int as libc::c_long / 10 as libc::c_int as libc::c_long
                / 10 as libc::c_int as libc::c_long / 10 as libc::c_int as libc::c_long
                / 10 as libc::c_int as libc::c_long / 10 as libc::c_int as libc::c_long)
            + '0' as i32 as libc::c_long) as libc::c_char;
        n
            %= 1000000 as libc::c_int as libc::c_long
                * 1000000000 as libc::c_int as wgint / 10 as libc::c_int as libc::c_long
                / 10 as libc::c_int as libc::c_long / 10 as libc::c_int as libc::c_long
                / 10 as libc::c_int as libc::c_long / 10 as libc::c_int as libc::c_long
                / 10 as libc::c_int as libc::c_long / 10 as libc::c_int as libc::c_long;
        let fresh143 = p;
        p = p.offset(1);
        *fresh143 = (n
            / (1000000 as libc::c_int as libc::c_long
                * 1000000000 as libc::c_int as wgint / 10 as libc::c_int as libc::c_long
                / 10 as libc::c_int as libc::c_long / 10 as libc::c_int as libc::c_long
                / 10 as libc::c_int as libc::c_long / 10 as libc::c_int as libc::c_long
                / 10 as libc::c_int as libc::c_long / 10 as libc::c_int as libc::c_long
                / 10 as libc::c_int as libc::c_long) + '0' as i32 as libc::c_long)
            as libc::c_char;
        n
            %= 1000000 as libc::c_int as libc::c_long
                * 1000000000 as libc::c_int as wgint / 10 as libc::c_int as libc::c_long
                / 10 as libc::c_int as libc::c_long / 10 as libc::c_int as libc::c_long
                / 10 as libc::c_int as libc::c_long / 10 as libc::c_int as libc::c_long
                / 10 as libc::c_int as libc::c_long / 10 as libc::c_int as libc::c_long
                / 10 as libc::c_int as libc::c_long;
        let fresh144 = p;
        p = p.offset(1);
        *fresh144 = (n
            / (1000000 as libc::c_int as libc::c_long
                * 1000000000 as libc::c_int as wgint / 10 as libc::c_int as libc::c_long
                / 10 as libc::c_int as libc::c_long / 10 as libc::c_int as libc::c_long
                / 10 as libc::c_int as libc::c_long / 10 as libc::c_int as libc::c_long
                / 10 as libc::c_int as libc::c_long / 10 as libc::c_int as libc::c_long
                / 10 as libc::c_int as libc::c_long / 10 as libc::c_int as libc::c_long)
            + '0' as i32 as libc::c_long) as libc::c_char;
        n
            %= 1000000 as libc::c_int as libc::c_long
                * 1000000000 as libc::c_int as wgint / 10 as libc::c_int as libc::c_long
                / 10 as libc::c_int as libc::c_long / 10 as libc::c_int as libc::c_long
                / 10 as libc::c_int as libc::c_long / 10 as libc::c_int as libc::c_long
                / 10 as libc::c_int as libc::c_long / 10 as libc::c_int as libc::c_long
                / 10 as libc::c_int as libc::c_long / 10 as libc::c_int as libc::c_long;
        let fresh145 = p;
        p = p.offset(1);
        *fresh145 = (n
            / (1000000 as libc::c_int as libc::c_long
                * 1000000000 as libc::c_int as wgint / 10 as libc::c_int as libc::c_long
                / 10 as libc::c_int as libc::c_long / 10 as libc::c_int as libc::c_long
                / 10 as libc::c_int as libc::c_long / 10 as libc::c_int as libc::c_long
                / 10 as libc::c_int as libc::c_long / 10 as libc::c_int as libc::c_long
                / 10 as libc::c_int as libc::c_long / 10 as libc::c_int as libc::c_long
                / 10 as libc::c_int as libc::c_long) + '0' as i32 as libc::c_long)
            as libc::c_char;
        n
            %= 1000000 as libc::c_int as libc::c_long
                * 1000000000 as libc::c_int as wgint / 10 as libc::c_int as libc::c_long
                / 10 as libc::c_int as libc::c_long / 10 as libc::c_int as libc::c_long
                / 10 as libc::c_int as libc::c_long / 10 as libc::c_int as libc::c_long
                / 10 as libc::c_int as libc::c_long / 10 as libc::c_int as libc::c_long
                / 10 as libc::c_int as libc::c_long / 10 as libc::c_int as libc::c_long
                / 10 as libc::c_int as libc::c_long;
        let fresh146 = p;
        p = p.offset(1);
        *fresh146 = (n
            / (1000000 as libc::c_int as libc::c_long
                * 1000000000 as libc::c_int as wgint / 10 as libc::c_int as libc::c_long
                / 10 as libc::c_int as libc::c_long / 10 as libc::c_int as libc::c_long
                / 10 as libc::c_int as libc::c_long / 10 as libc::c_int as libc::c_long
                / 10 as libc::c_int as libc::c_long / 10 as libc::c_int as libc::c_long
                / 10 as libc::c_int as libc::c_long / 10 as libc::c_int as libc::c_long
                / 10 as libc::c_int as libc::c_long / 10 as libc::c_int as libc::c_long)
            + '0' as i32 as libc::c_long) as libc::c_char;
        n
            %= 1000000 as libc::c_int as libc::c_long
                * 1000000000 as libc::c_int as wgint / 10 as libc::c_int as libc::c_long
                / 10 as libc::c_int as libc::c_long / 10 as libc::c_int as libc::c_long
                / 10 as libc::c_int as libc::c_long / 10 as libc::c_int as libc::c_long
                / 10 as libc::c_int as libc::c_long / 10 as libc::c_int as libc::c_long
                / 10 as libc::c_int as libc::c_long / 10 as libc::c_int as libc::c_long
                / 10 as libc::c_int as libc::c_long / 10 as libc::c_int as libc::c_long;
        let fresh147 = p;
        p = p.offset(1);
        *fresh147 = (n
            / (1000000 as libc::c_int as libc::c_long
                * 1000000000 as libc::c_int as wgint / 10 as libc::c_int as libc::c_long
                / 10 as libc::c_int as libc::c_long / 10 as libc::c_int as libc::c_long
                / 10 as libc::c_int as libc::c_long / 10 as libc::c_int as libc::c_long
                / 10 as libc::c_int as libc::c_long / 10 as libc::c_int as libc::c_long
                / 10 as libc::c_int as libc::c_long / 10 as libc::c_int as libc::c_long
                / 10 as libc::c_int as libc::c_long / 10 as libc::c_int as libc::c_long
                / 10 as libc::c_int as libc::c_long) + '0' as i32 as libc::c_long)
            as libc::c_char;
        n
            %= 1000000 as libc::c_int as libc::c_long
                * 1000000000 as libc::c_int as wgint / 10 as libc::c_int as libc::c_long
                / 10 as libc::c_int as libc::c_long / 10 as libc::c_int as libc::c_long
                / 10 as libc::c_int as libc::c_long / 10 as libc::c_int as libc::c_long
                / 10 as libc::c_int as libc::c_long / 10 as libc::c_int as libc::c_long
                / 10 as libc::c_int as libc::c_long / 10 as libc::c_int as libc::c_long
                / 10 as libc::c_int as libc::c_long / 10 as libc::c_int as libc::c_long
                / 10 as libc::c_int as libc::c_long;
        let fresh148 = p;
        p = p.offset(1);
        *fresh148 = (n
            / (1000000 as libc::c_int as libc::c_long
                * 1000000000 as libc::c_int as wgint / 10 as libc::c_int as libc::c_long
                / 10 as libc::c_int as libc::c_long / 10 as libc::c_int as libc::c_long
                / 10 as libc::c_int as libc::c_long / 10 as libc::c_int as libc::c_long
                / 10 as libc::c_int as libc::c_long / 10 as libc::c_int as libc::c_long
                / 10 as libc::c_int as libc::c_long / 10 as libc::c_int as libc::c_long
                / 10 as libc::c_int as libc::c_long / 10 as libc::c_int as libc::c_long
                / 10 as libc::c_int as libc::c_long / 10 as libc::c_int as libc::c_long)
            + '0' as i32 as libc::c_long) as libc::c_char;
        n
            %= 1000000 as libc::c_int as libc::c_long
                * 1000000000 as libc::c_int as wgint / 10 as libc::c_int as libc::c_long
                / 10 as libc::c_int as libc::c_long / 10 as libc::c_int as libc::c_long
                / 10 as libc::c_int as libc::c_long / 10 as libc::c_int as libc::c_long
                / 10 as libc::c_int as libc::c_long / 10 as libc::c_int as libc::c_long
                / 10 as libc::c_int as libc::c_long / 10 as libc::c_int as libc::c_long
                / 10 as libc::c_int as libc::c_long / 10 as libc::c_int as libc::c_long
                / 10 as libc::c_int as libc::c_long / 10 as libc::c_int as libc::c_long;
        let fresh149 = p;
        p = p.offset(1);
        *fresh149 = (n
            / (1000000 as libc::c_int as libc::c_long
                * 1000000000 as libc::c_int as wgint / 10 as libc::c_int as libc::c_long
                / 10 as libc::c_int as libc::c_long / 10 as libc::c_int as libc::c_long
                / 10 as libc::c_int as libc::c_long / 10 as libc::c_int as libc::c_long
                / 10 as libc::c_int as libc::c_long / 10 as libc::c_int as libc::c_long
                / 10 as libc::c_int as libc::c_long / 10 as libc::c_int as libc::c_long
                / 10 as libc::c_int as libc::c_long / 10 as libc::c_int as libc::c_long
                / 10 as libc::c_int as libc::c_long / 10 as libc::c_int as libc::c_long
                / 10 as libc::c_int as libc::c_long) + '0' as i32 as libc::c_long)
            as libc::c_char;
        n
            %= 1000000 as libc::c_int as libc::c_long
                * 1000000000 as libc::c_int as wgint / 10 as libc::c_int as libc::c_long
                / 10 as libc::c_int as libc::c_long / 10 as libc::c_int as libc::c_long
                / 10 as libc::c_int as libc::c_long / 10 as libc::c_int as libc::c_long
                / 10 as libc::c_int as libc::c_long / 10 as libc::c_int as libc::c_long
                / 10 as libc::c_int as libc::c_long / 10 as libc::c_int as libc::c_long
                / 10 as libc::c_int as libc::c_long / 10 as libc::c_int as libc::c_long
                / 10 as libc::c_int as libc::c_long / 10 as libc::c_int as libc::c_long
                / 10 as libc::c_int as libc::c_long;
        let fresh150 = p;
        p = p.offset(1);
        *fresh150 = (n
            / (1000000 as libc::c_int as libc::c_long
                * 1000000000 as libc::c_int as wgint / 10 as libc::c_int as libc::c_long
                / 10 as libc::c_int as libc::c_long / 10 as libc::c_int as libc::c_long
                / 10 as libc::c_int as libc::c_long / 10 as libc::c_int as libc::c_long
                / 10 as libc::c_int as libc::c_long / 10 as libc::c_int as libc::c_long
                / 10 as libc::c_int as libc::c_long / 10 as libc::c_int as libc::c_long
                / 10 as libc::c_int as libc::c_long / 10 as libc::c_int as libc::c_long
                / 10 as libc::c_int as libc::c_long / 10 as libc::c_int as libc::c_long
                / 10 as libc::c_int as libc::c_long / 10 as libc::c_int as libc::c_long)
            + '0' as i32 as libc::c_long) as libc::c_char;
    } else if n
        < 100000000 as libc::c_int as libc::c_long * 1000000000 as libc::c_int as wgint
    {
        let fresh151 = p;
        p = p.offset(1);
        *fresh151 = (n
            / (10000000 as libc::c_int as libc::c_long
                * 1000000000 as libc::c_int as wgint) + '0' as i32 as libc::c_long)
            as libc::c_char;
        n
            %= 10000000 as libc::c_int as libc::c_long
                * 1000000000 as libc::c_int as wgint;
        let fresh152 = p;
        p = p.offset(1);
        *fresh152 = (n
            / (10000000 as libc::c_int as libc::c_long
                * 1000000000 as libc::c_int as wgint / 10 as libc::c_int as libc::c_long)
            + '0' as i32 as libc::c_long) as libc::c_char;
        n
            %= 10000000 as libc::c_int as libc::c_long
                * 1000000000 as libc::c_int as wgint / 10 as libc::c_int as libc::c_long;
        let fresh153 = p;
        p = p.offset(1);
        *fresh153 = (n
            / (10000000 as libc::c_int as libc::c_long
                * 1000000000 as libc::c_int as wgint / 10 as libc::c_int as libc::c_long
                / 10 as libc::c_int as libc::c_long) + '0' as i32 as libc::c_long)
            as libc::c_char;
        n
            %= 10000000 as libc::c_int as libc::c_long
                * 1000000000 as libc::c_int as wgint / 10 as libc::c_int as libc::c_long
                / 10 as libc::c_int as libc::c_long;
        let fresh154 = p;
        p = p.offset(1);
        *fresh154 = (n
            / (10000000 as libc::c_int as libc::c_long
                * 1000000000 as libc::c_int as wgint / 10 as libc::c_int as libc::c_long
                / 10 as libc::c_int as libc::c_long / 10 as libc::c_int as libc::c_long)
            + '0' as i32 as libc::c_long) as libc::c_char;
        n
            %= 10000000 as libc::c_int as libc::c_long
                * 1000000000 as libc::c_int as wgint / 10 as libc::c_int as libc::c_long
                / 10 as libc::c_int as libc::c_long / 10 as libc::c_int as libc::c_long;
        let fresh155 = p;
        p = p.offset(1);
        *fresh155 = (n
            / (10000000 as libc::c_int as libc::c_long
                * 1000000000 as libc::c_int as wgint / 10 as libc::c_int as libc::c_long
                / 10 as libc::c_int as libc::c_long / 10 as libc::c_int as libc::c_long
                / 10 as libc::c_int as libc::c_long) + '0' as i32 as libc::c_long)
            as libc::c_char;
        n
            %= 10000000 as libc::c_int as libc::c_long
                * 1000000000 as libc::c_int as wgint / 10 as libc::c_int as libc::c_long
                / 10 as libc::c_int as libc::c_long / 10 as libc::c_int as libc::c_long
                / 10 as libc::c_int as libc::c_long;
        let fresh156 = p;
        p = p.offset(1);
        *fresh156 = (n
            / (10000000 as libc::c_int as libc::c_long
                * 1000000000 as libc::c_int as wgint / 10 as libc::c_int as libc::c_long
                / 10 as libc::c_int as libc::c_long / 10 as libc::c_int as libc::c_long
                / 10 as libc::c_int as libc::c_long / 10 as libc::c_int as libc::c_long)
            + '0' as i32 as libc::c_long) as libc::c_char;
        n
            %= 10000000 as libc::c_int as libc::c_long
                * 1000000000 as libc::c_int as wgint / 10 as libc::c_int as libc::c_long
                / 10 as libc::c_int as libc::c_long / 10 as libc::c_int as libc::c_long
                / 10 as libc::c_int as libc::c_long / 10 as libc::c_int as libc::c_long;
        let fresh157 = p;
        p = p.offset(1);
        *fresh157 = (n
            / (10000000 as libc::c_int as libc::c_long
                * 1000000000 as libc::c_int as wgint / 10 as libc::c_int as libc::c_long
                / 10 as libc::c_int as libc::c_long / 10 as libc::c_int as libc::c_long
                / 10 as libc::c_int as libc::c_long / 10 as libc::c_int as libc::c_long
                / 10 as libc::c_int as libc::c_long) + '0' as i32 as libc::c_long)
            as libc::c_char;
        n
            %= 10000000 as libc::c_int as libc::c_long
                * 1000000000 as libc::c_int as wgint / 10 as libc::c_int as libc::c_long
                / 10 as libc::c_int as libc::c_long / 10 as libc::c_int as libc::c_long
                / 10 as libc::c_int as libc::c_long / 10 as libc::c_int as libc::c_long
                / 10 as libc::c_int as libc::c_long;
        let fresh158 = p;
        p = p.offset(1);
        *fresh158 = (n
            / (10000000 as libc::c_int as libc::c_long
                * 1000000000 as libc::c_int as wgint / 10 as libc::c_int as libc::c_long
                / 10 as libc::c_int as libc::c_long / 10 as libc::c_int as libc::c_long
                / 10 as libc::c_int as libc::c_long / 10 as libc::c_int as libc::c_long
                / 10 as libc::c_int as libc::c_long / 10 as libc::c_int as libc::c_long)
            + '0' as i32 as libc::c_long) as libc::c_char;
        n
            %= 10000000 as libc::c_int as libc::c_long
                * 1000000000 as libc::c_int as wgint / 10 as libc::c_int as libc::c_long
                / 10 as libc::c_int as libc::c_long / 10 as libc::c_int as libc::c_long
                / 10 as libc::c_int as libc::c_long / 10 as libc::c_int as libc::c_long
                / 10 as libc::c_int as libc::c_long / 10 as libc::c_int as libc::c_long;
        let fresh159 = p;
        p = p.offset(1);
        *fresh159 = (n
            / (10000000 as libc::c_int as libc::c_long
                * 1000000000 as libc::c_int as wgint / 10 as libc::c_int as libc::c_long
                / 10 as libc::c_int as libc::c_long / 10 as libc::c_int as libc::c_long
                / 10 as libc::c_int as libc::c_long / 10 as libc::c_int as libc::c_long
                / 10 as libc::c_int as libc::c_long / 10 as libc::c_int as libc::c_long
                / 10 as libc::c_int as libc::c_long) + '0' as i32 as libc::c_long)
            as libc::c_char;
        n
            %= 10000000 as libc::c_int as libc::c_long
                * 1000000000 as libc::c_int as wgint / 10 as libc::c_int as libc::c_long
                / 10 as libc::c_int as libc::c_long / 10 as libc::c_int as libc::c_long
                / 10 as libc::c_int as libc::c_long / 10 as libc::c_int as libc::c_long
                / 10 as libc::c_int as libc::c_long / 10 as libc::c_int as libc::c_long
                / 10 as libc::c_int as libc::c_long;
        let fresh160 = p;
        p = p.offset(1);
        *fresh160 = (n
            / (10000000 as libc::c_int as libc::c_long
                * 1000000000 as libc::c_int as wgint / 10 as libc::c_int as libc::c_long
                / 10 as libc::c_int as libc::c_long / 10 as libc::c_int as libc::c_long
                / 10 as libc::c_int as libc::c_long / 10 as libc::c_int as libc::c_long
                / 10 as libc::c_int as libc::c_long / 10 as libc::c_int as libc::c_long
                / 10 as libc::c_int as libc::c_long / 10 as libc::c_int as libc::c_long)
            + '0' as i32 as libc::c_long) as libc::c_char;
        n
            %= 10000000 as libc::c_int as libc::c_long
                * 1000000000 as libc::c_int as wgint / 10 as libc::c_int as libc::c_long
                / 10 as libc::c_int as libc::c_long / 10 as libc::c_int as libc::c_long
                / 10 as libc::c_int as libc::c_long / 10 as libc::c_int as libc::c_long
                / 10 as libc::c_int as libc::c_long / 10 as libc::c_int as libc::c_long
                / 10 as libc::c_int as libc::c_long / 10 as libc::c_int as libc::c_long;
        let fresh161 = p;
        p = p.offset(1);
        *fresh161 = (n
            / (10000000 as libc::c_int as libc::c_long
                * 1000000000 as libc::c_int as wgint / 10 as libc::c_int as libc::c_long
                / 10 as libc::c_int as libc::c_long / 10 as libc::c_int as libc::c_long
                / 10 as libc::c_int as libc::c_long / 10 as libc::c_int as libc::c_long
                / 10 as libc::c_int as libc::c_long / 10 as libc::c_int as libc::c_long
                / 10 as libc::c_int as libc::c_long / 10 as libc::c_int as libc::c_long
                / 10 as libc::c_int as libc::c_long) + '0' as i32 as libc::c_long)
            as libc::c_char;
        n
            %= 10000000 as libc::c_int as libc::c_long
                * 1000000000 as libc::c_int as wgint / 10 as libc::c_int as libc::c_long
                / 10 as libc::c_int as libc::c_long / 10 as libc::c_int as libc::c_long
                / 10 as libc::c_int as libc::c_long / 10 as libc::c_int as libc::c_long
                / 10 as libc::c_int as libc::c_long / 10 as libc::c_int as libc::c_long
                / 10 as libc::c_int as libc::c_long / 10 as libc::c_int as libc::c_long
                / 10 as libc::c_int as libc::c_long;
        let fresh162 = p;
        p = p.offset(1);
        *fresh162 = (n
            / (10000000 as libc::c_int as libc::c_long
                * 1000000000 as libc::c_int as wgint / 10 as libc::c_int as libc::c_long
                / 10 as libc::c_int as libc::c_long / 10 as libc::c_int as libc::c_long
                / 10 as libc::c_int as libc::c_long / 10 as libc::c_int as libc::c_long
                / 10 as libc::c_int as libc::c_long / 10 as libc::c_int as libc::c_long
                / 10 as libc::c_int as libc::c_long / 10 as libc::c_int as libc::c_long
                / 10 as libc::c_int as libc::c_long / 10 as libc::c_int as libc::c_long)
            + '0' as i32 as libc::c_long) as libc::c_char;
        n
            %= 10000000 as libc::c_int as libc::c_long
                * 1000000000 as libc::c_int as wgint / 10 as libc::c_int as libc::c_long
                / 10 as libc::c_int as libc::c_long / 10 as libc::c_int as libc::c_long
                / 10 as libc::c_int as libc::c_long / 10 as libc::c_int as libc::c_long
                / 10 as libc::c_int as libc::c_long / 10 as libc::c_int as libc::c_long
                / 10 as libc::c_int as libc::c_long / 10 as libc::c_int as libc::c_long
                / 10 as libc::c_int as libc::c_long / 10 as libc::c_int as libc::c_long;
        let fresh163 = p;
        p = p.offset(1);
        *fresh163 = (n
            / (10000000 as libc::c_int as libc::c_long
                * 1000000000 as libc::c_int as wgint / 10 as libc::c_int as libc::c_long
                / 10 as libc::c_int as libc::c_long / 10 as libc::c_int as libc::c_long
                / 10 as libc::c_int as libc::c_long / 10 as libc::c_int as libc::c_long
                / 10 as libc::c_int as libc::c_long / 10 as libc::c_int as libc::c_long
                / 10 as libc::c_int as libc::c_long / 10 as libc::c_int as libc::c_long
                / 10 as libc::c_int as libc::c_long / 10 as libc::c_int as libc::c_long
                / 10 as libc::c_int as libc::c_long) + '0' as i32 as libc::c_long)
            as libc::c_char;
        n
            %= 10000000 as libc::c_int as libc::c_long
                * 1000000000 as libc::c_int as wgint / 10 as libc::c_int as libc::c_long
                / 10 as libc::c_int as libc::c_long / 10 as libc::c_int as libc::c_long
                / 10 as libc::c_int as libc::c_long / 10 as libc::c_int as libc::c_long
                / 10 as libc::c_int as libc::c_long / 10 as libc::c_int as libc::c_long
                / 10 as libc::c_int as libc::c_long / 10 as libc::c_int as libc::c_long
                / 10 as libc::c_int as libc::c_long / 10 as libc::c_int as libc::c_long
                / 10 as libc::c_int as libc::c_long;
        let fresh164 = p;
        p = p.offset(1);
        *fresh164 = (n
            / (10000000 as libc::c_int as libc::c_long
                * 1000000000 as libc::c_int as wgint / 10 as libc::c_int as libc::c_long
                / 10 as libc::c_int as libc::c_long / 10 as libc::c_int as libc::c_long
                / 10 as libc::c_int as libc::c_long / 10 as libc::c_int as libc::c_long
                / 10 as libc::c_int as libc::c_long / 10 as libc::c_int as libc::c_long
                / 10 as libc::c_int as libc::c_long / 10 as libc::c_int as libc::c_long
                / 10 as libc::c_int as libc::c_long / 10 as libc::c_int as libc::c_long
                / 10 as libc::c_int as libc::c_long / 10 as libc::c_int as libc::c_long)
            + '0' as i32 as libc::c_long) as libc::c_char;
        n
            %= 10000000 as libc::c_int as libc::c_long
                * 1000000000 as libc::c_int as wgint / 10 as libc::c_int as libc::c_long
                / 10 as libc::c_int as libc::c_long / 10 as libc::c_int as libc::c_long
                / 10 as libc::c_int as libc::c_long / 10 as libc::c_int as libc::c_long
                / 10 as libc::c_int as libc::c_long / 10 as libc::c_int as libc::c_long
                / 10 as libc::c_int as libc::c_long / 10 as libc::c_int as libc::c_long
                / 10 as libc::c_int as libc::c_long / 10 as libc::c_int as libc::c_long
                / 10 as libc::c_int as libc::c_long / 10 as libc::c_int as libc::c_long;
        let fresh165 = p;
        p = p.offset(1);
        *fresh165 = (n
            / (10000000 as libc::c_int as libc::c_long
                * 1000000000 as libc::c_int as wgint / 10 as libc::c_int as libc::c_long
                / 10 as libc::c_int as libc::c_long / 10 as libc::c_int as libc::c_long
                / 10 as libc::c_int as libc::c_long / 10 as libc::c_int as libc::c_long
                / 10 as libc::c_int as libc::c_long / 10 as libc::c_int as libc::c_long
                / 10 as libc::c_int as libc::c_long / 10 as libc::c_int as libc::c_long
                / 10 as libc::c_int as libc::c_long / 10 as libc::c_int as libc::c_long
                / 10 as libc::c_int as libc::c_long / 10 as libc::c_int as libc::c_long
                / 10 as libc::c_int as libc::c_long) + '0' as i32 as libc::c_long)
            as libc::c_char;
        n
            %= 10000000 as libc::c_int as libc::c_long
                * 1000000000 as libc::c_int as wgint / 10 as libc::c_int as libc::c_long
                / 10 as libc::c_int as libc::c_long / 10 as libc::c_int as libc::c_long
                / 10 as libc::c_int as libc::c_long / 10 as libc::c_int as libc::c_long
                / 10 as libc::c_int as libc::c_long / 10 as libc::c_int as libc::c_long
                / 10 as libc::c_int as libc::c_long / 10 as libc::c_int as libc::c_long
                / 10 as libc::c_int as libc::c_long / 10 as libc::c_int as libc::c_long
                / 10 as libc::c_int as libc::c_long / 10 as libc::c_int as libc::c_long
                / 10 as libc::c_int as libc::c_long;
        let fresh166 = p;
        p = p.offset(1);
        *fresh166 = (n
            / (10000000 as libc::c_int as libc::c_long
                * 1000000000 as libc::c_int as wgint / 10 as libc::c_int as libc::c_long
                / 10 as libc::c_int as libc::c_long / 10 as libc::c_int as libc::c_long
                / 10 as libc::c_int as libc::c_long / 10 as libc::c_int as libc::c_long
                / 10 as libc::c_int as libc::c_long / 10 as libc::c_int as libc::c_long
                / 10 as libc::c_int as libc::c_long / 10 as libc::c_int as libc::c_long
                / 10 as libc::c_int as libc::c_long / 10 as libc::c_int as libc::c_long
                / 10 as libc::c_int as libc::c_long / 10 as libc::c_int as libc::c_long
                / 10 as libc::c_int as libc::c_long / 10 as libc::c_int as libc::c_long)
            + '0' as i32 as libc::c_long) as libc::c_char;
        n
            %= 10000000 as libc::c_int as libc::c_long
                * 1000000000 as libc::c_int as wgint / 10 as libc::c_int as libc::c_long
                / 10 as libc::c_int as libc::c_long / 10 as libc::c_int as libc::c_long
                / 10 as libc::c_int as libc::c_long / 10 as libc::c_int as libc::c_long
                / 10 as libc::c_int as libc::c_long / 10 as libc::c_int as libc::c_long
                / 10 as libc::c_int as libc::c_long / 10 as libc::c_int as libc::c_long
                / 10 as libc::c_int as libc::c_long / 10 as libc::c_int as libc::c_long
                / 10 as libc::c_int as libc::c_long / 10 as libc::c_int as libc::c_long
                / 10 as libc::c_int as libc::c_long / 10 as libc::c_int as libc::c_long;
        let fresh167 = p;
        p = p.offset(1);
        *fresh167 = (n
            / (10000000 as libc::c_int as libc::c_long
                * 1000000000 as libc::c_int as wgint / 10 as libc::c_int as libc::c_long
                / 10 as libc::c_int as libc::c_long / 10 as libc::c_int as libc::c_long
                / 10 as libc::c_int as libc::c_long / 10 as libc::c_int as libc::c_long
                / 10 as libc::c_int as libc::c_long / 10 as libc::c_int as libc::c_long
                / 10 as libc::c_int as libc::c_long / 10 as libc::c_int as libc::c_long
                / 10 as libc::c_int as libc::c_long / 10 as libc::c_int as libc::c_long
                / 10 as libc::c_int as libc::c_long / 10 as libc::c_int as libc::c_long
                / 10 as libc::c_int as libc::c_long / 10 as libc::c_int as libc::c_long
                / 10 as libc::c_int as libc::c_long) + '0' as i32 as libc::c_long)
            as libc::c_char;
    } else if n
        < 1000000000 as libc::c_int as libc::c_long * 1000000000 as libc::c_int as wgint
    {
        let fresh168 = p;
        p = p.offset(1);
        *fresh168 = (n
            / (100000000 as libc::c_int as libc::c_long
                * 1000000000 as libc::c_int as wgint) + '0' as i32 as libc::c_long)
            as libc::c_char;
        n
            %= 100000000 as libc::c_int as libc::c_long
                * 1000000000 as libc::c_int as wgint;
        let fresh169 = p;
        p = p.offset(1);
        *fresh169 = (n
            / (100000000 as libc::c_int as libc::c_long
                * 1000000000 as libc::c_int as wgint / 10 as libc::c_int as libc::c_long)
            + '0' as i32 as libc::c_long) as libc::c_char;
        n
            %= 100000000 as libc::c_int as libc::c_long
                * 1000000000 as libc::c_int as wgint / 10 as libc::c_int as libc::c_long;
        let fresh170 = p;
        p = p.offset(1);
        *fresh170 = (n
            / (100000000 as libc::c_int as libc::c_long
                * 1000000000 as libc::c_int as wgint / 10 as libc::c_int as libc::c_long
                / 10 as libc::c_int as libc::c_long) + '0' as i32 as libc::c_long)
            as libc::c_char;
        n
            %= 100000000 as libc::c_int as libc::c_long
                * 1000000000 as libc::c_int as wgint / 10 as libc::c_int as libc::c_long
                / 10 as libc::c_int as libc::c_long;
        let fresh171 = p;
        p = p.offset(1);
        *fresh171 = (n
            / (100000000 as libc::c_int as libc::c_long
                * 1000000000 as libc::c_int as wgint / 10 as libc::c_int as libc::c_long
                / 10 as libc::c_int as libc::c_long / 10 as libc::c_int as libc::c_long)
            + '0' as i32 as libc::c_long) as libc::c_char;
        n
            %= 100000000 as libc::c_int as libc::c_long
                * 1000000000 as libc::c_int as wgint / 10 as libc::c_int as libc::c_long
                / 10 as libc::c_int as libc::c_long / 10 as libc::c_int as libc::c_long;
        let fresh172 = p;
        p = p.offset(1);
        *fresh172 = (n
            / (100000000 as libc::c_int as libc::c_long
                * 1000000000 as libc::c_int as wgint / 10 as libc::c_int as libc::c_long
                / 10 as libc::c_int as libc::c_long / 10 as libc::c_int as libc::c_long
                / 10 as libc::c_int as libc::c_long) + '0' as i32 as libc::c_long)
            as libc::c_char;
        n
            %= 100000000 as libc::c_int as libc::c_long
                * 1000000000 as libc::c_int as wgint / 10 as libc::c_int as libc::c_long
                / 10 as libc::c_int as libc::c_long / 10 as libc::c_int as libc::c_long
                / 10 as libc::c_int as libc::c_long;
        let fresh173 = p;
        p = p.offset(1);
        *fresh173 = (n
            / (100000000 as libc::c_int as libc::c_long
                * 1000000000 as libc::c_int as wgint / 10 as libc::c_int as libc::c_long
                / 10 as libc::c_int as libc::c_long / 10 as libc::c_int as libc::c_long
                / 10 as libc::c_int as libc::c_long / 10 as libc::c_int as libc::c_long)
            + '0' as i32 as libc::c_long) as libc::c_char;
        n
            %= 100000000 as libc::c_int as libc::c_long
                * 1000000000 as libc::c_int as wgint / 10 as libc::c_int as libc::c_long
                / 10 as libc::c_int as libc::c_long / 10 as libc::c_int as libc::c_long
                / 10 as libc::c_int as libc::c_long / 10 as libc::c_int as libc::c_long;
        let fresh174 = p;
        p = p.offset(1);
        *fresh174 = (n
            / (100000000 as libc::c_int as libc::c_long
                * 1000000000 as libc::c_int as wgint / 10 as libc::c_int as libc::c_long
                / 10 as libc::c_int as libc::c_long / 10 as libc::c_int as libc::c_long
                / 10 as libc::c_int as libc::c_long / 10 as libc::c_int as libc::c_long
                / 10 as libc::c_int as libc::c_long) + '0' as i32 as libc::c_long)
            as libc::c_char;
        n
            %= 100000000 as libc::c_int as libc::c_long
                * 1000000000 as libc::c_int as wgint / 10 as libc::c_int as libc::c_long
                / 10 as libc::c_int as libc::c_long / 10 as libc::c_int as libc::c_long
                / 10 as libc::c_int as libc::c_long / 10 as libc::c_int as libc::c_long
                / 10 as libc::c_int as libc::c_long;
        let fresh175 = p;
        p = p.offset(1);
        *fresh175 = (n
            / (100000000 as libc::c_int as libc::c_long
                * 1000000000 as libc::c_int as wgint / 10 as libc::c_int as libc::c_long
                / 10 as libc::c_int as libc::c_long / 10 as libc::c_int as libc::c_long
                / 10 as libc::c_int as libc::c_long / 10 as libc::c_int as libc::c_long
                / 10 as libc::c_int as libc::c_long / 10 as libc::c_int as libc::c_long)
            + '0' as i32 as libc::c_long) as libc::c_char;
        n
            %= 100000000 as libc::c_int as libc::c_long
                * 1000000000 as libc::c_int as wgint / 10 as libc::c_int as libc::c_long
                / 10 as libc::c_int as libc::c_long / 10 as libc::c_int as libc::c_long
                / 10 as libc::c_int as libc::c_long / 10 as libc::c_int as libc::c_long
                / 10 as libc::c_int as libc::c_long / 10 as libc::c_int as libc::c_long;
        let fresh176 = p;
        p = p.offset(1);
        *fresh176 = (n
            / (100000000 as libc::c_int as libc::c_long
                * 1000000000 as libc::c_int as wgint / 10 as libc::c_int as libc::c_long
                / 10 as libc::c_int as libc::c_long / 10 as libc::c_int as libc::c_long
                / 10 as libc::c_int as libc::c_long / 10 as libc::c_int as libc::c_long
                / 10 as libc::c_int as libc::c_long / 10 as libc::c_int as libc::c_long
                / 10 as libc::c_int as libc::c_long) + '0' as i32 as libc::c_long)
            as libc::c_char;
        n
            %= 100000000 as libc::c_int as libc::c_long
                * 1000000000 as libc::c_int as wgint / 10 as libc::c_int as libc::c_long
                / 10 as libc::c_int as libc::c_long / 10 as libc::c_int as libc::c_long
                / 10 as libc::c_int as libc::c_long / 10 as libc::c_int as libc::c_long
                / 10 as libc::c_int as libc::c_long / 10 as libc::c_int as libc::c_long
                / 10 as libc::c_int as libc::c_long;
        let fresh177 = p;
        p = p.offset(1);
        *fresh177 = (n
            / (100000000 as libc::c_int as libc::c_long
                * 1000000000 as libc::c_int as wgint / 10 as libc::c_int as libc::c_long
                / 10 as libc::c_int as libc::c_long / 10 as libc::c_int as libc::c_long
                / 10 as libc::c_int as libc::c_long / 10 as libc::c_int as libc::c_long
                / 10 as libc::c_int as libc::c_long / 10 as libc::c_int as libc::c_long
                / 10 as libc::c_int as libc::c_long / 10 as libc::c_int as libc::c_long)
            + '0' as i32 as libc::c_long) as libc::c_char;
        n
            %= 100000000 as libc::c_int as libc::c_long
                * 1000000000 as libc::c_int as wgint / 10 as libc::c_int as libc::c_long
                / 10 as libc::c_int as libc::c_long / 10 as libc::c_int as libc::c_long
                / 10 as libc::c_int as libc::c_long / 10 as libc::c_int as libc::c_long
                / 10 as libc::c_int as libc::c_long / 10 as libc::c_int as libc::c_long
                / 10 as libc::c_int as libc::c_long / 10 as libc::c_int as libc::c_long;
        let fresh178 = p;
        p = p.offset(1);
        *fresh178 = (n
            / (100000000 as libc::c_int as libc::c_long
                * 1000000000 as libc::c_int as wgint / 10 as libc::c_int as libc::c_long
                / 10 as libc::c_int as libc::c_long / 10 as libc::c_int as libc::c_long
                / 10 as libc::c_int as libc::c_long / 10 as libc::c_int as libc::c_long
                / 10 as libc::c_int as libc::c_long / 10 as libc::c_int as libc::c_long
                / 10 as libc::c_int as libc::c_long / 10 as libc::c_int as libc::c_long
                / 10 as libc::c_int as libc::c_long) + '0' as i32 as libc::c_long)
            as libc::c_char;
        n
            %= 100000000 as libc::c_int as libc::c_long
                * 1000000000 as libc::c_int as wgint / 10 as libc::c_int as libc::c_long
                / 10 as libc::c_int as libc::c_long / 10 as libc::c_int as libc::c_long
                / 10 as libc::c_int as libc::c_long / 10 as libc::c_int as libc::c_long
                / 10 as libc::c_int as libc::c_long / 10 as libc::c_int as libc::c_long
                / 10 as libc::c_int as libc::c_long / 10 as libc::c_int as libc::c_long
                / 10 as libc::c_int as libc::c_long;
        let fresh179 = p;
        p = p.offset(1);
        *fresh179 = (n
            / (100000000 as libc::c_int as libc::c_long
                * 1000000000 as libc::c_int as wgint / 10 as libc::c_int as libc::c_long
                / 10 as libc::c_int as libc::c_long / 10 as libc::c_int as libc::c_long
                / 10 as libc::c_int as libc::c_long / 10 as libc::c_int as libc::c_long
                / 10 as libc::c_int as libc::c_long / 10 as libc::c_int as libc::c_long
                / 10 as libc::c_int as libc::c_long / 10 as libc::c_int as libc::c_long
                / 10 as libc::c_int as libc::c_long / 10 as libc::c_int as libc::c_long)
            + '0' as i32 as libc::c_long) as libc::c_char;
        n
            %= 100000000 as libc::c_int as libc::c_long
                * 1000000000 as libc::c_int as wgint / 10 as libc::c_int as libc::c_long
                / 10 as libc::c_int as libc::c_long / 10 as libc::c_int as libc::c_long
                / 10 as libc::c_int as libc::c_long / 10 as libc::c_int as libc::c_long
                / 10 as libc::c_int as libc::c_long / 10 as libc::c_int as libc::c_long
                / 10 as libc::c_int as libc::c_long / 10 as libc::c_int as libc::c_long
                / 10 as libc::c_int as libc::c_long / 10 as libc::c_int as libc::c_long;
        let fresh180 = p;
        p = p.offset(1);
        *fresh180 = (n
            / (100000000 as libc::c_int as libc::c_long
                * 1000000000 as libc::c_int as wgint / 10 as libc::c_int as libc::c_long
                / 10 as libc::c_int as libc::c_long / 10 as libc::c_int as libc::c_long
                / 10 as libc::c_int as libc::c_long / 10 as libc::c_int as libc::c_long
                / 10 as libc::c_int as libc::c_long / 10 as libc::c_int as libc::c_long
                / 10 as libc::c_int as libc::c_long / 10 as libc::c_int as libc::c_long
                / 10 as libc::c_int as libc::c_long / 10 as libc::c_int as libc::c_long
                / 10 as libc::c_int as libc::c_long) + '0' as i32 as libc::c_long)
            as libc::c_char;
        n
            %= 100000000 as libc::c_int as libc::c_long
                * 1000000000 as libc::c_int as wgint / 10 as libc::c_int as libc::c_long
                / 10 as libc::c_int as libc::c_long / 10 as libc::c_int as libc::c_long
                / 10 as libc::c_int as libc::c_long / 10 as libc::c_int as libc::c_long
                / 10 as libc::c_int as libc::c_long / 10 as libc::c_int as libc::c_long
                / 10 as libc::c_int as libc::c_long / 10 as libc::c_int as libc::c_long
                / 10 as libc::c_int as libc::c_long / 10 as libc::c_int as libc::c_long
                / 10 as libc::c_int as libc::c_long;
        let fresh181 = p;
        p = p.offset(1);
        *fresh181 = (n
            / (100000000 as libc::c_int as libc::c_long
                * 1000000000 as libc::c_int as wgint / 10 as libc::c_int as libc::c_long
                / 10 as libc::c_int as libc::c_long / 10 as libc::c_int as libc::c_long
                / 10 as libc::c_int as libc::c_long / 10 as libc::c_int as libc::c_long
                / 10 as libc::c_int as libc::c_long / 10 as libc::c_int as libc::c_long
                / 10 as libc::c_int as libc::c_long / 10 as libc::c_int as libc::c_long
                / 10 as libc::c_int as libc::c_long / 10 as libc::c_int as libc::c_long
                / 10 as libc::c_int as libc::c_long / 10 as libc::c_int as libc::c_long)
            + '0' as i32 as libc::c_long) as libc::c_char;
        n
            %= 100000000 as libc::c_int as libc::c_long
                * 1000000000 as libc::c_int as wgint / 10 as libc::c_int as libc::c_long
                / 10 as libc::c_int as libc::c_long / 10 as libc::c_int as libc::c_long
                / 10 as libc::c_int as libc::c_long / 10 as libc::c_int as libc::c_long
                / 10 as libc::c_int as libc::c_long / 10 as libc::c_int as libc::c_long
                / 10 as libc::c_int as libc::c_long / 10 as libc::c_int as libc::c_long
                / 10 as libc::c_int as libc::c_long / 10 as libc::c_int as libc::c_long
                / 10 as libc::c_int as libc::c_long / 10 as libc::c_int as libc::c_long;
        let fresh182 = p;
        p = p.offset(1);
        *fresh182 = (n
            / (100000000 as libc::c_int as libc::c_long
                * 1000000000 as libc::c_int as wgint / 10 as libc::c_int as libc::c_long
                / 10 as libc::c_int as libc::c_long / 10 as libc::c_int as libc::c_long
                / 10 as libc::c_int as libc::c_long / 10 as libc::c_int as libc::c_long
                / 10 as libc::c_int as libc::c_long / 10 as libc::c_int as libc::c_long
                / 10 as libc::c_int as libc::c_long / 10 as libc::c_int as libc::c_long
                / 10 as libc::c_int as libc::c_long / 10 as libc::c_int as libc::c_long
                / 10 as libc::c_int as libc::c_long / 10 as libc::c_int as libc::c_long
                / 10 as libc::c_int as libc::c_long) + '0' as i32 as libc::c_long)
            as libc::c_char;
        n
            %= 100000000 as libc::c_int as libc::c_long
                * 1000000000 as libc::c_int as wgint / 10 as libc::c_int as libc::c_long
                / 10 as libc::c_int as libc::c_long / 10 as libc::c_int as libc::c_long
                / 10 as libc::c_int as libc::c_long / 10 as libc::c_int as libc::c_long
                / 10 as libc::c_int as libc::c_long / 10 as libc::c_int as libc::c_long
                / 10 as libc::c_int as libc::c_long / 10 as libc::c_int as libc::c_long
                / 10 as libc::c_int as libc::c_long / 10 as libc::c_int as libc::c_long
                / 10 as libc::c_int as libc::c_long / 10 as libc::c_int as libc::c_long
                / 10 as libc::c_int as libc::c_long;
        let fresh183 = p;
        p = p.offset(1);
        *fresh183 = (n
            / (100000000 as libc::c_int as libc::c_long
                * 1000000000 as libc::c_int as wgint / 10 as libc::c_int as libc::c_long
                / 10 as libc::c_int as libc::c_long / 10 as libc::c_int as libc::c_long
                / 10 as libc::c_int as libc::c_long / 10 as libc::c_int as libc::c_long
                / 10 as libc::c_int as libc::c_long / 10 as libc::c_int as libc::c_long
                / 10 as libc::c_int as libc::c_long / 10 as libc::c_int as libc::c_long
                / 10 as libc::c_int as libc::c_long / 10 as libc::c_int as libc::c_long
                / 10 as libc::c_int as libc::c_long / 10 as libc::c_int as libc::c_long
                / 10 as libc::c_int as libc::c_long / 10 as libc::c_int as libc::c_long)
            + '0' as i32 as libc::c_long) as libc::c_char;
        n
            %= 100000000 as libc::c_int as libc::c_long
                * 1000000000 as libc::c_int as wgint / 10 as libc::c_int as libc::c_long
                / 10 as libc::c_int as libc::c_long / 10 as libc::c_int as libc::c_long
                / 10 as libc::c_int as libc::c_long / 10 as libc::c_int as libc::c_long
                / 10 as libc::c_int as libc::c_long / 10 as libc::c_int as libc::c_long
                / 10 as libc::c_int as libc::c_long / 10 as libc::c_int as libc::c_long
                / 10 as libc::c_int as libc::c_long / 10 as libc::c_int as libc::c_long
                / 10 as libc::c_int as libc::c_long / 10 as libc::c_int as libc::c_long
                / 10 as libc::c_int as libc::c_long / 10 as libc::c_int as libc::c_long;
        let fresh184 = p;
        p = p.offset(1);
        *fresh184 = (n
            / (100000000 as libc::c_int as libc::c_long
                * 1000000000 as libc::c_int as wgint / 10 as libc::c_int as libc::c_long
                / 10 as libc::c_int as libc::c_long / 10 as libc::c_int as libc::c_long
                / 10 as libc::c_int as libc::c_long / 10 as libc::c_int as libc::c_long
                / 10 as libc::c_int as libc::c_long / 10 as libc::c_int as libc::c_long
                / 10 as libc::c_int as libc::c_long / 10 as libc::c_int as libc::c_long
                / 10 as libc::c_int as libc::c_long / 10 as libc::c_int as libc::c_long
                / 10 as libc::c_int as libc::c_long / 10 as libc::c_int as libc::c_long
                / 10 as libc::c_int as libc::c_long / 10 as libc::c_int as libc::c_long
                / 10 as libc::c_int as libc::c_long) + '0' as i32 as libc::c_long)
            as libc::c_char;
        n
            %= 100000000 as libc::c_int as libc::c_long
                * 1000000000 as libc::c_int as wgint / 10 as libc::c_int as libc::c_long
                / 10 as libc::c_int as libc::c_long / 10 as libc::c_int as libc::c_long
                / 10 as libc::c_int as libc::c_long / 10 as libc::c_int as libc::c_long
                / 10 as libc::c_int as libc::c_long / 10 as libc::c_int as libc::c_long
                / 10 as libc::c_int as libc::c_long / 10 as libc::c_int as libc::c_long
                / 10 as libc::c_int as libc::c_long / 10 as libc::c_int as libc::c_long
                / 10 as libc::c_int as libc::c_long / 10 as libc::c_int as libc::c_long
                / 10 as libc::c_int as libc::c_long / 10 as libc::c_int as libc::c_long
                / 10 as libc::c_int as libc::c_long;
        let fresh185 = p;
        p = p.offset(1);
        *fresh185 = (n
            / (100000000 as libc::c_int as libc::c_long
                * 1000000000 as libc::c_int as wgint / 10 as libc::c_int as libc::c_long
                / 10 as libc::c_int as libc::c_long / 10 as libc::c_int as libc::c_long
                / 10 as libc::c_int as libc::c_long / 10 as libc::c_int as libc::c_long
                / 10 as libc::c_int as libc::c_long / 10 as libc::c_int as libc::c_long
                / 10 as libc::c_int as libc::c_long / 10 as libc::c_int as libc::c_long
                / 10 as libc::c_int as libc::c_long / 10 as libc::c_int as libc::c_long
                / 10 as libc::c_int as libc::c_long / 10 as libc::c_int as libc::c_long
                / 10 as libc::c_int as libc::c_long / 10 as libc::c_int as libc::c_long
                / 10 as libc::c_int as libc::c_long / 10 as libc::c_int as libc::c_long)
            + '0' as i32 as libc::c_long) as libc::c_char;
    } else {
        let fresh186 = p;
        p = p.offset(1);
        *fresh186 = (n
            / (1000000000 as libc::c_int as libc::c_long
                * 1000000000 as libc::c_int as wgint) + '0' as i32 as libc::c_long)
            as libc::c_char;
        n
            %= 1000000000 as libc::c_int as libc::c_long
                * 1000000000 as libc::c_int as wgint;
        let fresh187 = p;
        p = p.offset(1);
        *fresh187 = (n
            / (1000000000 as libc::c_int as libc::c_long
                * 1000000000 as libc::c_int as wgint / 10 as libc::c_int as libc::c_long)
            + '0' as i32 as libc::c_long) as libc::c_char;
        n
            %= 1000000000 as libc::c_int as libc::c_long
                * 1000000000 as libc::c_int as wgint / 10 as libc::c_int as libc::c_long;
        let fresh188 = p;
        p = p.offset(1);
        *fresh188 = (n
            / (1000000000 as libc::c_int as libc::c_long
                * 1000000000 as libc::c_int as wgint / 10 as libc::c_int as libc::c_long
                / 10 as libc::c_int as libc::c_long) + '0' as i32 as libc::c_long)
            as libc::c_char;
        n
            %= 1000000000 as libc::c_int as libc::c_long
                * 1000000000 as libc::c_int as wgint / 10 as libc::c_int as libc::c_long
                / 10 as libc::c_int as libc::c_long;
        let fresh189 = p;
        p = p.offset(1);
        *fresh189 = (n
            / (1000000000 as libc::c_int as libc::c_long
                * 1000000000 as libc::c_int as wgint / 10 as libc::c_int as libc::c_long
                / 10 as libc::c_int as libc::c_long / 10 as libc::c_int as libc::c_long)
            + '0' as i32 as libc::c_long) as libc::c_char;
        n
            %= 1000000000 as libc::c_int as libc::c_long
                * 1000000000 as libc::c_int as wgint / 10 as libc::c_int as libc::c_long
                / 10 as libc::c_int as libc::c_long / 10 as libc::c_int as libc::c_long;
        let fresh190 = p;
        p = p.offset(1);
        *fresh190 = (n
            / (1000000000 as libc::c_int as libc::c_long
                * 1000000000 as libc::c_int as wgint / 10 as libc::c_int as libc::c_long
                / 10 as libc::c_int as libc::c_long / 10 as libc::c_int as libc::c_long
                / 10 as libc::c_int as libc::c_long) + '0' as i32 as libc::c_long)
            as libc::c_char;
        n
            %= 1000000000 as libc::c_int as libc::c_long
                * 1000000000 as libc::c_int as wgint / 10 as libc::c_int as libc::c_long
                / 10 as libc::c_int as libc::c_long / 10 as libc::c_int as libc::c_long
                / 10 as libc::c_int as libc::c_long;
        let fresh191 = p;
        p = p.offset(1);
        *fresh191 = (n
            / (1000000000 as libc::c_int as libc::c_long
                * 1000000000 as libc::c_int as wgint / 10 as libc::c_int as libc::c_long
                / 10 as libc::c_int as libc::c_long / 10 as libc::c_int as libc::c_long
                / 10 as libc::c_int as libc::c_long / 10 as libc::c_int as libc::c_long)
            + '0' as i32 as libc::c_long) as libc::c_char;
        n
            %= 1000000000 as libc::c_int as libc::c_long
                * 1000000000 as libc::c_int as wgint / 10 as libc::c_int as libc::c_long
                / 10 as libc::c_int as libc::c_long / 10 as libc::c_int as libc::c_long
                / 10 as libc::c_int as libc::c_long / 10 as libc::c_int as libc::c_long;
        let fresh192 = p;
        p = p.offset(1);
        *fresh192 = (n
            / (1000000000 as libc::c_int as libc::c_long
                * 1000000000 as libc::c_int as wgint / 10 as libc::c_int as libc::c_long
                / 10 as libc::c_int as libc::c_long / 10 as libc::c_int as libc::c_long
                / 10 as libc::c_int as libc::c_long / 10 as libc::c_int as libc::c_long
                / 10 as libc::c_int as libc::c_long) + '0' as i32 as libc::c_long)
            as libc::c_char;
        n
            %= 1000000000 as libc::c_int as libc::c_long
                * 1000000000 as libc::c_int as wgint / 10 as libc::c_int as libc::c_long
                / 10 as libc::c_int as libc::c_long / 10 as libc::c_int as libc::c_long
                / 10 as libc::c_int as libc::c_long / 10 as libc::c_int as libc::c_long
                / 10 as libc::c_int as libc::c_long;
        let fresh193 = p;
        p = p.offset(1);
        *fresh193 = (n
            / (1000000000 as libc::c_int as libc::c_long
                * 1000000000 as libc::c_int as wgint / 10 as libc::c_int as libc::c_long
                / 10 as libc::c_int as libc::c_long / 10 as libc::c_int as libc::c_long
                / 10 as libc::c_int as libc::c_long / 10 as libc::c_int as libc::c_long
                / 10 as libc::c_int as libc::c_long / 10 as libc::c_int as libc::c_long)
            + '0' as i32 as libc::c_long) as libc::c_char;
        n
            %= 1000000000 as libc::c_int as libc::c_long
                * 1000000000 as libc::c_int as wgint / 10 as libc::c_int as libc::c_long
                / 10 as libc::c_int as libc::c_long / 10 as libc::c_int as libc::c_long
                / 10 as libc::c_int as libc::c_long / 10 as libc::c_int as libc::c_long
                / 10 as libc::c_int as libc::c_long / 10 as libc::c_int as libc::c_long;
        let fresh194 = p;
        p = p.offset(1);
        *fresh194 = (n
            / (1000000000 as libc::c_int as libc::c_long
                * 1000000000 as libc::c_int as wgint / 10 as libc::c_int as libc::c_long
                / 10 as libc::c_int as libc::c_long / 10 as libc::c_int as libc::c_long
                / 10 as libc::c_int as libc::c_long / 10 as libc::c_int as libc::c_long
                / 10 as libc::c_int as libc::c_long / 10 as libc::c_int as libc::c_long
                / 10 as libc::c_int as libc::c_long) + '0' as i32 as libc::c_long)
            as libc::c_char;
        n
            %= 1000000000 as libc::c_int as libc::c_long
                * 1000000000 as libc::c_int as wgint / 10 as libc::c_int as libc::c_long
                / 10 as libc::c_int as libc::c_long / 10 as libc::c_int as libc::c_long
                / 10 as libc::c_int as libc::c_long / 10 as libc::c_int as libc::c_long
                / 10 as libc::c_int as libc::c_long / 10 as libc::c_int as libc::c_long
                / 10 as libc::c_int as libc::c_long;
        let fresh195 = p;
        p = p.offset(1);
        *fresh195 = (n
            / (1000000000 as libc::c_int as libc::c_long
                * 1000000000 as libc::c_int as wgint / 10 as libc::c_int as libc::c_long
                / 10 as libc::c_int as libc::c_long / 10 as libc::c_int as libc::c_long
                / 10 as libc::c_int as libc::c_long / 10 as libc::c_int as libc::c_long
                / 10 as libc::c_int as libc::c_long / 10 as libc::c_int as libc::c_long
                / 10 as libc::c_int as libc::c_long / 10 as libc::c_int as libc::c_long)
            + '0' as i32 as libc::c_long) as libc::c_char;
        n
            %= 1000000000 as libc::c_int as libc::c_long
                * 1000000000 as libc::c_int as wgint / 10 as libc::c_int as libc::c_long
                / 10 as libc::c_int as libc::c_long / 10 as libc::c_int as libc::c_long
                / 10 as libc::c_int as libc::c_long / 10 as libc::c_int as libc::c_long
                / 10 as libc::c_int as libc::c_long / 10 as libc::c_int as libc::c_long
                / 10 as libc::c_int as libc::c_long / 10 as libc::c_int as libc::c_long;
        let fresh196 = p;
        p = p.offset(1);
        *fresh196 = (n
            / (1000000000 as libc::c_int as libc::c_long
                * 1000000000 as libc::c_int as wgint / 10 as libc::c_int as libc::c_long
                / 10 as libc::c_int as libc::c_long / 10 as libc::c_int as libc::c_long
                / 10 as libc::c_int as libc::c_long / 10 as libc::c_int as libc::c_long
                / 10 as libc::c_int as libc::c_long / 10 as libc::c_int as libc::c_long
                / 10 as libc::c_int as libc::c_long / 10 as libc::c_int as libc::c_long
                / 10 as libc::c_int as libc::c_long) + '0' as i32 as libc::c_long)
            as libc::c_char;
        n
            %= 1000000000 as libc::c_int as libc::c_long
                * 1000000000 as libc::c_int as wgint / 10 as libc::c_int as libc::c_long
                / 10 as libc::c_int as libc::c_long / 10 as libc::c_int as libc::c_long
                / 10 as libc::c_int as libc::c_long / 10 as libc::c_int as libc::c_long
                / 10 as libc::c_int as libc::c_long / 10 as libc::c_int as libc::c_long
                / 10 as libc::c_int as libc::c_long / 10 as libc::c_int as libc::c_long
                / 10 as libc::c_int as libc::c_long;
        let fresh197 = p;
        p = p.offset(1);
        *fresh197 = (n
            / (1000000000 as libc::c_int as libc::c_long
                * 1000000000 as libc::c_int as wgint / 10 as libc::c_int as libc::c_long
                / 10 as libc::c_int as libc::c_long / 10 as libc::c_int as libc::c_long
                / 10 as libc::c_int as libc::c_long / 10 as libc::c_int as libc::c_long
                / 10 as libc::c_int as libc::c_long / 10 as libc::c_int as libc::c_long
                / 10 as libc::c_int as libc::c_long / 10 as libc::c_int as libc::c_long
                / 10 as libc::c_int as libc::c_long / 10 as libc::c_int as libc::c_long)
            + '0' as i32 as libc::c_long) as libc::c_char;
        n
            %= 1000000000 as libc::c_int as libc::c_long
                * 1000000000 as libc::c_int as wgint / 10 as libc::c_int as libc::c_long
                / 10 as libc::c_int as libc::c_long / 10 as libc::c_int as libc::c_long
                / 10 as libc::c_int as libc::c_long / 10 as libc::c_int as libc::c_long
                / 10 as libc::c_int as libc::c_long / 10 as libc::c_int as libc::c_long
                / 10 as libc::c_int as libc::c_long / 10 as libc::c_int as libc::c_long
                / 10 as libc::c_int as libc::c_long / 10 as libc::c_int as libc::c_long;
        let fresh198 = p;
        p = p.offset(1);
        *fresh198 = (n
            / (1000000000 as libc::c_int as libc::c_long
                * 1000000000 as libc::c_int as wgint / 10 as libc::c_int as libc::c_long
                / 10 as libc::c_int as libc::c_long / 10 as libc::c_int as libc::c_long
                / 10 as libc::c_int as libc::c_long / 10 as libc::c_int as libc::c_long
                / 10 as libc::c_int as libc::c_long / 10 as libc::c_int as libc::c_long
                / 10 as libc::c_int as libc::c_long / 10 as libc::c_int as libc::c_long
                / 10 as libc::c_int as libc::c_long / 10 as libc::c_int as libc::c_long
                / 10 as libc::c_int as libc::c_long) + '0' as i32 as libc::c_long)
            as libc::c_char;
        n
            %= 1000000000 as libc::c_int as libc::c_long
                * 1000000000 as libc::c_int as wgint / 10 as libc::c_int as libc::c_long
                / 10 as libc::c_int as libc::c_long / 10 as libc::c_int as libc::c_long
                / 10 as libc::c_int as libc::c_long / 10 as libc::c_int as libc::c_long
                / 10 as libc::c_int as libc::c_long / 10 as libc::c_int as libc::c_long
                / 10 as libc::c_int as libc::c_long / 10 as libc::c_int as libc::c_long
                / 10 as libc::c_int as libc::c_long / 10 as libc::c_int as libc::c_long
                / 10 as libc::c_int as libc::c_long;
        let fresh199 = p;
        p = p.offset(1);
        *fresh199 = (n
            / (1000000000 as libc::c_int as libc::c_long
                * 1000000000 as libc::c_int as wgint / 10 as libc::c_int as libc::c_long
                / 10 as libc::c_int as libc::c_long / 10 as libc::c_int as libc::c_long
                / 10 as libc::c_int as libc::c_long / 10 as libc::c_int as libc::c_long
                / 10 as libc::c_int as libc::c_long / 10 as libc::c_int as libc::c_long
                / 10 as libc::c_int as libc::c_long / 10 as libc::c_int as libc::c_long
                / 10 as libc::c_int as libc::c_long / 10 as libc::c_int as libc::c_long
                / 10 as libc::c_int as libc::c_long / 10 as libc::c_int as libc::c_long)
            + '0' as i32 as libc::c_long) as libc::c_char;
        n
            %= 1000000000 as libc::c_int as libc::c_long
                * 1000000000 as libc::c_int as wgint / 10 as libc::c_int as libc::c_long
                / 10 as libc::c_int as libc::c_long / 10 as libc::c_int as libc::c_long
                / 10 as libc::c_int as libc::c_long / 10 as libc::c_int as libc::c_long
                / 10 as libc::c_int as libc::c_long / 10 as libc::c_int as libc::c_long
                / 10 as libc::c_int as libc::c_long / 10 as libc::c_int as libc::c_long
                / 10 as libc::c_int as libc::c_long / 10 as libc::c_int as libc::c_long
                / 10 as libc::c_int as libc::c_long / 10 as libc::c_int as libc::c_long;
        let fresh200 = p;
        p = p.offset(1);
        *fresh200 = (n
            / (1000000000 as libc::c_int as libc::c_long
                * 1000000000 as libc::c_int as wgint / 10 as libc::c_int as libc::c_long
                / 10 as libc::c_int as libc::c_long / 10 as libc::c_int as libc::c_long
                / 10 as libc::c_int as libc::c_long / 10 as libc::c_int as libc::c_long
                / 10 as libc::c_int as libc::c_long / 10 as libc::c_int as libc::c_long
                / 10 as libc::c_int as libc::c_long / 10 as libc::c_int as libc::c_long
                / 10 as libc::c_int as libc::c_long / 10 as libc::c_int as libc::c_long
                / 10 as libc::c_int as libc::c_long / 10 as libc::c_int as libc::c_long
                / 10 as libc::c_int as libc::c_long) + '0' as i32 as libc::c_long)
            as libc::c_char;
        n
            %= 1000000000 as libc::c_int as libc::c_long
                * 1000000000 as libc::c_int as wgint / 10 as libc::c_int as libc::c_long
                / 10 as libc::c_int as libc::c_long / 10 as libc::c_int as libc::c_long
                / 10 as libc::c_int as libc::c_long / 10 as libc::c_int as libc::c_long
                / 10 as libc::c_int as libc::c_long / 10 as libc::c_int as libc::c_long
                / 10 as libc::c_int as libc::c_long / 10 as libc::c_int as libc::c_long
                / 10 as libc::c_int as libc::c_long / 10 as libc::c_int as libc::c_long
                / 10 as libc::c_int as libc::c_long / 10 as libc::c_int as libc::c_long
                / 10 as libc::c_int as libc::c_long;
        let fresh201 = p;
        p = p.offset(1);
        *fresh201 = (n
            / (1000000000 as libc::c_int as libc::c_long
                * 1000000000 as libc::c_int as wgint / 10 as libc::c_int as libc::c_long
                / 10 as libc::c_int as libc::c_long / 10 as libc::c_int as libc::c_long
                / 10 as libc::c_int as libc::c_long / 10 as libc::c_int as libc::c_long
                / 10 as libc::c_int as libc::c_long / 10 as libc::c_int as libc::c_long
                / 10 as libc::c_int as libc::c_long / 10 as libc::c_int as libc::c_long
                / 10 as libc::c_int as libc::c_long / 10 as libc::c_int as libc::c_long
                / 10 as libc::c_int as libc::c_long / 10 as libc::c_int as libc::c_long
                / 10 as libc::c_int as libc::c_long / 10 as libc::c_int as libc::c_long)
            + '0' as i32 as libc::c_long) as libc::c_char;
        n
            %= 1000000000 as libc::c_int as libc::c_long
                * 1000000000 as libc::c_int as wgint / 10 as libc::c_int as libc::c_long
                / 10 as libc::c_int as libc::c_long / 10 as libc::c_int as libc::c_long
                / 10 as libc::c_int as libc::c_long / 10 as libc::c_int as libc::c_long
                / 10 as libc::c_int as libc::c_long / 10 as libc::c_int as libc::c_long
                / 10 as libc::c_int as libc::c_long / 10 as libc::c_int as libc::c_long
                / 10 as libc::c_int as libc::c_long / 10 as libc::c_int as libc::c_long
                / 10 as libc::c_int as libc::c_long / 10 as libc::c_int as libc::c_long
                / 10 as libc::c_int as libc::c_long / 10 as libc::c_int as libc::c_long;
        let fresh202 = p;
        p = p.offset(1);
        *fresh202 = (n
            / (1000000000 as libc::c_int as libc::c_long
                * 1000000000 as libc::c_int as wgint / 10 as libc::c_int as libc::c_long
                / 10 as libc::c_int as libc::c_long / 10 as libc::c_int as libc::c_long
                / 10 as libc::c_int as libc::c_long / 10 as libc::c_int as libc::c_long
                / 10 as libc::c_int as libc::c_long / 10 as libc::c_int as libc::c_long
                / 10 as libc::c_int as libc::c_long / 10 as libc::c_int as libc::c_long
                / 10 as libc::c_int as libc::c_long / 10 as libc::c_int as libc::c_long
                / 10 as libc::c_int as libc::c_long / 10 as libc::c_int as libc::c_long
                / 10 as libc::c_int as libc::c_long / 10 as libc::c_int as libc::c_long
                / 10 as libc::c_int as libc::c_long) + '0' as i32 as libc::c_long)
            as libc::c_char;
        n
            %= 1000000000 as libc::c_int as libc::c_long
                * 1000000000 as libc::c_int as wgint / 10 as libc::c_int as libc::c_long
                / 10 as libc::c_int as libc::c_long / 10 as libc::c_int as libc::c_long
                / 10 as libc::c_int as libc::c_long / 10 as libc::c_int as libc::c_long
                / 10 as libc::c_int as libc::c_long / 10 as libc::c_int as libc::c_long
                / 10 as libc::c_int as libc::c_long / 10 as libc::c_int as libc::c_long
                / 10 as libc::c_int as libc::c_long / 10 as libc::c_int as libc::c_long
                / 10 as libc::c_int as libc::c_long / 10 as libc::c_int as libc::c_long
                / 10 as libc::c_int as libc::c_long / 10 as libc::c_int as libc::c_long
                / 10 as libc::c_int as libc::c_long;
        let fresh203 = p;
        p = p.offset(1);
        *fresh203 = (n
            / (1000000000 as libc::c_int as libc::c_long
                * 1000000000 as libc::c_int as wgint / 10 as libc::c_int as libc::c_long
                / 10 as libc::c_int as libc::c_long / 10 as libc::c_int as libc::c_long
                / 10 as libc::c_int as libc::c_long / 10 as libc::c_int as libc::c_long
                / 10 as libc::c_int as libc::c_long / 10 as libc::c_int as libc::c_long
                / 10 as libc::c_int as libc::c_long / 10 as libc::c_int as libc::c_long
                / 10 as libc::c_int as libc::c_long / 10 as libc::c_int as libc::c_long
                / 10 as libc::c_int as libc::c_long / 10 as libc::c_int as libc::c_long
                / 10 as libc::c_int as libc::c_long / 10 as libc::c_int as libc::c_long
                / 10 as libc::c_int as libc::c_long / 10 as libc::c_int as libc::c_long)
            + '0' as i32 as libc::c_long) as libc::c_char;
        n
            %= 1000000000 as libc::c_int as libc::c_long
                * 1000000000 as libc::c_int as wgint / 10 as libc::c_int as libc::c_long
                / 10 as libc::c_int as libc::c_long / 10 as libc::c_int as libc::c_long
                / 10 as libc::c_int as libc::c_long / 10 as libc::c_int as libc::c_long
                / 10 as libc::c_int as libc::c_long / 10 as libc::c_int as libc::c_long
                / 10 as libc::c_int as libc::c_long / 10 as libc::c_int as libc::c_long
                / 10 as libc::c_int as libc::c_long / 10 as libc::c_int as libc::c_long
                / 10 as libc::c_int as libc::c_long / 10 as libc::c_int as libc::c_long
                / 10 as libc::c_int as libc::c_long / 10 as libc::c_int as libc::c_long
                / 10 as libc::c_int as libc::c_long / 10 as libc::c_int as libc::c_long;
        let fresh204 = p;
        p = p.offset(1);
        *fresh204 = (n
            / (1000000000 as libc::c_int as libc::c_long
                * 1000000000 as libc::c_int as wgint / 10 as libc::c_int as libc::c_long
                / 10 as libc::c_int as libc::c_long / 10 as libc::c_int as libc::c_long
                / 10 as libc::c_int as libc::c_long / 10 as libc::c_int as libc::c_long
                / 10 as libc::c_int as libc::c_long / 10 as libc::c_int as libc::c_long
                / 10 as libc::c_int as libc::c_long / 10 as libc::c_int as libc::c_long
                / 10 as libc::c_int as libc::c_long / 10 as libc::c_int as libc::c_long
                / 10 as libc::c_int as libc::c_long / 10 as libc::c_int as libc::c_long
                / 10 as libc::c_int as libc::c_long / 10 as libc::c_int as libc::c_long
                / 10 as libc::c_int as libc::c_long / 10 as libc::c_int as libc::c_long
                / 10 as libc::c_int as libc::c_long) + '0' as i32 as libc::c_long)
            as libc::c_char;
    }
    if last_digit_char != 0 {
        let fresh205 = p;
        p = p.offset(1);
        *fresh205 = last_digit_char as libc::c_char;
    }
    *p = '\0' as i32 as libc::c_char;
    return p;
}
pub unsafe extern "C" fn number_to_static_string(
    mut number: wgint,
) -> *mut libc::c_char {
    static mut ring: [[libc::c_char; 24]; 3] = [[0; 24]; 3];
    static mut ringpos: libc::c_int = 0;
    let mut buf: *mut libc::c_char = (ring[ringpos as usize]).as_mut_ptr();
    number_to_string(buf, number);
    ringpos = (ringpos + 1 as libc::c_int) % 3 as libc::c_int;
    return buf;
}
pub unsafe extern "C" fn convert_to_bits(mut num: wgint) -> wgint {
    if opt.report_bps {
        return num * 8 as libc::c_int as libc::c_long;
    }
    return num;
}
pub unsafe extern "C" fn determine_screen_width() -> libc::c_int {
    let mut fd: libc::c_int = 0;
    let mut wsz: winsize = winsize {
        ws_row: 0,
        ws_col: 0,
        ws_xpixel: 0,
        ws_ypixel: 0,
    };
    if !(opt.lfilename).is_null() && opt.show_progress != 1 as libc::c_int {
        return 0 as libc::c_int;
    }
    fd = fileno(stderr);
    if rpl_ioctl(fd, 0x5413 as libc::c_int, &mut wsz as *mut winsize) < 0 as libc::c_int
    {
        return 0 as libc::c_int;
    }
    return wsz.ws_col as libc::c_int;
}
static mut rnd_seeded: libc::c_int = 0;
pub unsafe extern "C" fn random_number(mut max: libc::c_int) -> libc::c_int {
    if rnd_seeded == 0 {
        srandom((time(0 as *mut time_t) ^ getpid() as libc::c_long) as libc::c_uint);
        rnd_seeded = 1 as libc::c_int;
    }
    return (random() % max as libc::c_long) as libc::c_int;
}
pub unsafe extern "C" fn random_float() -> libc::c_double {
    return random_number(2147483647 as libc::c_int) as libc::c_double
        / 2147483647 as libc::c_int as libc::c_double;
}
static mut run_with_timeout_env: sigjmp_buf = [__jmp_buf_tag {
    __jmpbuf: [0; 8],
    __mask_was_saved: 0,
    __saved_mask: __sigset_t { __val: [0; 16] },
}; 1];
unsafe extern "C" fn abort_run_with_timeout(mut sig: libc::c_int) {
    siglongjmp(run_with_timeout_env.as_mut_ptr(), -(1 as libc::c_int));
}
unsafe extern "C" fn alarm_set(mut timeout: libc::c_double) {
    let mut itv: itimerval = itimerval {
        it_interval: timeval { tv_sec: 0, tv_usec: 0 },
        it_value: timeval { tv_sec: 0, tv_usec: 0 },
    };
    memset(
        &mut itv as *mut itimerval as *mut libc::c_void,
        '\0' as i32,
        ::std::mem::size_of::<itimerval>() as libc::c_ulong,
    );
    itv.it_value.tv_sec = timeout as libc::c_long;
    itv
        .it_value
        .tv_usec = (1000000 as libc::c_int as libc::c_double
        * (timeout - timeout as libc::c_long as libc::c_double)) as __suseconds_t;
    if itv.it_value.tv_sec == 0 as libc::c_int as libc::c_long
        && itv.it_value.tv_usec == 0 as libc::c_int as libc::c_long
    {
        itv.it_value.tv_usec = 1 as libc::c_int as __suseconds_t;
    }
    setitimer(ITIMER_REAL, &mut itv, 0 as *mut itimerval);
}
unsafe extern "C" fn alarm_cancel() {
    let mut disable: itimerval = itimerval {
        it_interval: timeval { tv_sec: 0, tv_usec: 0 },
        it_value: timeval { tv_sec: 0, tv_usec: 0 },
    };
    memset(
        &mut disable as *mut itimerval as *mut libc::c_void,
        '\0' as i32,
        ::std::mem::size_of::<itimerval>() as libc::c_ulong,
    );
    setitimer(ITIMER_REAL, &mut disable, 0 as *mut itimerval);
}
pub unsafe extern "C" fn run_with_timeout(
    mut timeout: libc::c_double,
    mut fun: Option::<unsafe extern "C" fn(*mut libc::c_void) -> ()>,
    mut arg: *mut libc::c_void,
) -> bool {
    let mut saved_errno: libc::c_int = 0;
    if timeout == 0 as libc::c_int as libc::c_double {
        fun.unwrap()(arg);
        return 0 as libc::c_int != 0;
    }
    if __sigsetjmp(run_with_timeout_env.as_mut_ptr(), 1 as libc::c_int)
        != 0 as libc::c_int
    {
        signal(14 as libc::c_int, None);
        return 1 as libc::c_int != 0;
    } else {
        signal(
            14 as libc::c_int,
            Some(abort_run_with_timeout as unsafe extern "C" fn(libc::c_int) -> ()),
        );
    }
    alarm_set(timeout);
    fun.unwrap()(arg);
    saved_errno = *__errno_location();
    alarm_cancel();
    signal(14 as libc::c_int, None);
    *__errno_location() = saved_errno;
    return 0 as libc::c_int != 0;
}
pub unsafe extern "C" fn xsleep(mut seconds: libc::c_double) {
    if seconds >= 1 as libc::c_int as libc::c_double {
        sleep(seconds as libc::c_uint);
        seconds -= seconds as libc::c_long as libc::c_double;
    }
    usleep((seconds * 1000000 as libc::c_int as libc::c_double) as __useconds_t);
}
pub unsafe extern "C" fn wget_base64_encode(
    mut data: *const libc::c_void,
    mut length: size_t,
    mut dest: *mut libc::c_char,
) -> size_t {
    static mut tbl: [libc::c_char; 64] = [
        'A' as i32 as libc::c_char,
        'B' as i32 as libc::c_char,
        'C' as i32 as libc::c_char,
        'D' as i32 as libc::c_char,
        'E' as i32 as libc::c_char,
        'F' as i32 as libc::c_char,
        'G' as i32 as libc::c_char,
        'H' as i32 as libc::c_char,
        'I' as i32 as libc::c_char,
        'J' as i32 as libc::c_char,
        'K' as i32 as libc::c_char,
        'L' as i32 as libc::c_char,
        'M' as i32 as libc::c_char,
        'N' as i32 as libc::c_char,
        'O' as i32 as libc::c_char,
        'P' as i32 as libc::c_char,
        'Q' as i32 as libc::c_char,
        'R' as i32 as libc::c_char,
        'S' as i32 as libc::c_char,
        'T' as i32 as libc::c_char,
        'U' as i32 as libc::c_char,
        'V' as i32 as libc::c_char,
        'W' as i32 as libc::c_char,
        'X' as i32 as libc::c_char,
        'Y' as i32 as libc::c_char,
        'Z' as i32 as libc::c_char,
        'a' as i32 as libc::c_char,
        'b' as i32 as libc::c_char,
        'c' as i32 as libc::c_char,
        'd' as i32 as libc::c_char,
        'e' as i32 as libc::c_char,
        'f' as i32 as libc::c_char,
        'g' as i32 as libc::c_char,
        'h' as i32 as libc::c_char,
        'i' as i32 as libc::c_char,
        'j' as i32 as libc::c_char,
        'k' as i32 as libc::c_char,
        'l' as i32 as libc::c_char,
        'm' as i32 as libc::c_char,
        'n' as i32 as libc::c_char,
        'o' as i32 as libc::c_char,
        'p' as i32 as libc::c_char,
        'q' as i32 as libc::c_char,
        'r' as i32 as libc::c_char,
        's' as i32 as libc::c_char,
        't' as i32 as libc::c_char,
        'u' as i32 as libc::c_char,
        'v' as i32 as libc::c_char,
        'w' as i32 as libc::c_char,
        'x' as i32 as libc::c_char,
        'y' as i32 as libc::c_char,
        'z' as i32 as libc::c_char,
        '0' as i32 as libc::c_char,
        '1' as i32 as libc::c_char,
        '2' as i32 as libc::c_char,
        '3' as i32 as libc::c_char,
        '4' as i32 as libc::c_char,
        '5' as i32 as libc::c_char,
        '6' as i32 as libc::c_char,
        '7' as i32 as libc::c_char,
        '8' as i32 as libc::c_char,
        '9' as i32 as libc::c_char,
        '+' as i32 as libc::c_char,
        '/' as i32 as libc::c_char,
    ];
    let mut s: *const libc::c_uchar = data as *const libc::c_uchar;
    let mut end: *const libc::c_uchar = (data as *const libc::c_uchar)
        .offset(length as isize)
        .offset(-(2 as libc::c_int as isize));
    let mut p: *mut libc::c_char = dest;
    while s < end {
        let fresh206 = p;
        p = p.offset(1);
        *fresh206 = tbl[(*s.offset(0 as libc::c_int as isize) as libc::c_int
            >> 2 as libc::c_int) as usize];
        let fresh207 = p;
        p = p.offset(1);
        *fresh207 = tbl[(((*s.offset(0 as libc::c_int as isize) as libc::c_int
            & 3 as libc::c_int) << 4 as libc::c_int)
            + (*s.offset(1 as libc::c_int as isize) as libc::c_int >> 4 as libc::c_int))
            as usize];
        let fresh208 = p;
        p = p.offset(1);
        *fresh208 = tbl[(((*s.offset(1 as libc::c_int as isize) as libc::c_int
            & 0xf as libc::c_int) << 2 as libc::c_int)
            + (*s.offset(2 as libc::c_int as isize) as libc::c_int >> 6 as libc::c_int))
            as usize];
        let fresh209 = p;
        p = p.offset(1);
        *fresh209 = tbl[(*s.offset(2 as libc::c_int as isize) as libc::c_int
            & 0x3f as libc::c_int) as usize];
        s = s.offset(3 as libc::c_int as isize);
    }
    match length.wrapping_rem(3 as libc::c_int as libc::c_ulong) {
        1 => {
            let fresh210 = p;
            p = p.offset(1);
            *fresh210 = tbl[(*s.offset(0 as libc::c_int as isize) as libc::c_int
                >> 2 as libc::c_int) as usize];
            let fresh211 = p;
            p = p.offset(1);
            *fresh211 = tbl[((*s.offset(0 as libc::c_int as isize) as libc::c_int
                & 3 as libc::c_int) << 4 as libc::c_int) as usize];
            let fresh212 = p;
            p = p.offset(1);
            *fresh212 = '=' as i32 as libc::c_char;
            let fresh213 = p;
            p = p.offset(1);
            *fresh213 = '=' as i32 as libc::c_char;
        }
        2 => {
            let fresh214 = p;
            p = p.offset(1);
            *fresh214 = tbl[(*s.offset(0 as libc::c_int as isize) as libc::c_int
                >> 2 as libc::c_int) as usize];
            let fresh215 = p;
            p = p.offset(1);
            *fresh215 = tbl[(((*s.offset(0 as libc::c_int as isize) as libc::c_int
                & 3 as libc::c_int) << 4 as libc::c_int)
                + (*s.offset(1 as libc::c_int as isize) as libc::c_int
                    >> 4 as libc::c_int)) as usize];
            let fresh216 = p;
            p = p.offset(1);
            *fresh216 = tbl[((*s.offset(1 as libc::c_int as isize) as libc::c_int
                & 0xf as libc::c_int) << 2 as libc::c_int) as usize];
            let fresh217 = p;
            p = p.offset(1);
            *fresh217 = '=' as i32 as libc::c_char;
        }
        _ => {}
    }
    *p = '\0' as i32 as libc::c_char;
    return p.offset_from(dest) as libc::c_long as size_t;
}
pub unsafe extern "C" fn wget_base64_decode(
    mut base64: *const libc::c_char,
    mut dest: *mut libc::c_void,
    mut size: size_t,
) -> ssize_t {
    static mut base64_char_to_value: [libc::c_schar; 128] = [
        -(1 as libc::c_int) as libc::c_schar,
        -(1 as libc::c_int) as libc::c_schar,
        -(1 as libc::c_int) as libc::c_schar,
        -(1 as libc::c_int) as libc::c_schar,
        -(1 as libc::c_int) as libc::c_schar,
        -(1 as libc::c_int) as libc::c_schar,
        -(1 as libc::c_int) as libc::c_schar,
        -(1 as libc::c_int) as libc::c_schar,
        -(1 as libc::c_int) as libc::c_schar,
        -(1 as libc::c_int) as libc::c_schar,
        -(1 as libc::c_int) as libc::c_schar,
        -(1 as libc::c_int) as libc::c_schar,
        -(1 as libc::c_int) as libc::c_schar,
        -(1 as libc::c_int) as libc::c_schar,
        -(1 as libc::c_int) as libc::c_schar,
        -(1 as libc::c_int) as libc::c_schar,
        -(1 as libc::c_int) as libc::c_schar,
        -(1 as libc::c_int) as libc::c_schar,
        -(1 as libc::c_int) as libc::c_schar,
        -(1 as libc::c_int) as libc::c_schar,
        -(1 as libc::c_int) as libc::c_schar,
        -(1 as libc::c_int) as libc::c_schar,
        -(1 as libc::c_int) as libc::c_schar,
        -(1 as libc::c_int) as libc::c_schar,
        -(1 as libc::c_int) as libc::c_schar,
        -(1 as libc::c_int) as libc::c_schar,
        -(1 as libc::c_int) as libc::c_schar,
        -(1 as libc::c_int) as libc::c_schar,
        -(1 as libc::c_int) as libc::c_schar,
        -(1 as libc::c_int) as libc::c_schar,
        -(1 as libc::c_int) as libc::c_schar,
        -(1 as libc::c_int) as libc::c_schar,
        -(1 as libc::c_int) as libc::c_schar,
        -(1 as libc::c_int) as libc::c_schar,
        -(1 as libc::c_int) as libc::c_schar,
        -(1 as libc::c_int) as libc::c_schar,
        -(1 as libc::c_int) as libc::c_schar,
        -(1 as libc::c_int) as libc::c_schar,
        -(1 as libc::c_int) as libc::c_schar,
        -(1 as libc::c_int) as libc::c_schar,
        -(1 as libc::c_int) as libc::c_schar,
        -(1 as libc::c_int) as libc::c_schar,
        -(1 as libc::c_int) as libc::c_schar,
        62 as libc::c_int as libc::c_schar,
        -(1 as libc::c_int) as libc::c_schar,
        -(1 as libc::c_int) as libc::c_schar,
        -(1 as libc::c_int) as libc::c_schar,
        63 as libc::c_int as libc::c_schar,
        52 as libc::c_int as libc::c_schar,
        53 as libc::c_int as libc::c_schar,
        54 as libc::c_int as libc::c_schar,
        55 as libc::c_int as libc::c_schar,
        56 as libc::c_int as libc::c_schar,
        57 as libc::c_int as libc::c_schar,
        58 as libc::c_int as libc::c_schar,
        59 as libc::c_int as libc::c_schar,
        60 as libc::c_int as libc::c_schar,
        61 as libc::c_int as libc::c_schar,
        -(1 as libc::c_int) as libc::c_schar,
        -(1 as libc::c_int) as libc::c_schar,
        -(1 as libc::c_int) as libc::c_schar,
        -(1 as libc::c_int) as libc::c_schar,
        -(1 as libc::c_int) as libc::c_schar,
        -(1 as libc::c_int) as libc::c_schar,
        -(1 as libc::c_int) as libc::c_schar,
        0 as libc::c_int as libc::c_schar,
        1 as libc::c_int as libc::c_schar,
        2 as libc::c_int as libc::c_schar,
        3 as libc::c_int as libc::c_schar,
        4 as libc::c_int as libc::c_schar,
        5 as libc::c_int as libc::c_schar,
        6 as libc::c_int as libc::c_schar,
        7 as libc::c_int as libc::c_schar,
        8 as libc::c_int as libc::c_schar,
        9 as libc::c_int as libc::c_schar,
        10 as libc::c_int as libc::c_schar,
        11 as libc::c_int as libc::c_schar,
        12 as libc::c_int as libc::c_schar,
        13 as libc::c_int as libc::c_schar,
        14 as libc::c_int as libc::c_schar,
        15 as libc::c_int as libc::c_schar,
        16 as libc::c_int as libc::c_schar,
        17 as libc::c_int as libc::c_schar,
        18 as libc::c_int as libc::c_schar,
        19 as libc::c_int as libc::c_schar,
        20 as libc::c_int as libc::c_schar,
        21 as libc::c_int as libc::c_schar,
        22 as libc::c_int as libc::c_schar,
        23 as libc::c_int as libc::c_schar,
        24 as libc::c_int as libc::c_schar,
        25 as libc::c_int as libc::c_schar,
        -(1 as libc::c_int) as libc::c_schar,
        -(1 as libc::c_int) as libc::c_schar,
        -(1 as libc::c_int) as libc::c_schar,
        -(1 as libc::c_int) as libc::c_schar,
        -(1 as libc::c_int) as libc::c_schar,
        -(1 as libc::c_int) as libc::c_schar,
        26 as libc::c_int as libc::c_schar,
        27 as libc::c_int as libc::c_schar,
        28 as libc::c_int as libc::c_schar,
        29 as libc::c_int as libc::c_schar,
        30 as libc::c_int as libc::c_schar,
        31 as libc::c_int as libc::c_schar,
        32 as libc::c_int as libc::c_schar,
        33 as libc::c_int as libc::c_schar,
        34 as libc::c_int as libc::c_schar,
        35 as libc::c_int as libc::c_schar,
        36 as libc::c_int as libc::c_schar,
        37 as libc::c_int as libc::c_schar,
        38 as libc::c_int as libc::c_schar,
        39 as libc::c_int as libc::c_schar,
        40 as libc::c_int as libc::c_schar,
        41 as libc::c_int as libc::c_schar,
        42 as libc::c_int as libc::c_schar,
        43 as libc::c_int as libc::c_schar,
        44 as libc::c_int as libc::c_schar,
        45 as libc::c_int as libc::c_schar,
        46 as libc::c_int as libc::c_schar,
        47 as libc::c_int as libc::c_schar,
        48 as libc::c_int as libc::c_schar,
        49 as libc::c_int as libc::c_schar,
        50 as libc::c_int as libc::c_schar,
        51 as libc::c_int as libc::c_schar,
        -(1 as libc::c_int) as libc::c_schar,
        -(1 as libc::c_int) as libc::c_schar,
        -(1 as libc::c_int) as libc::c_schar,
        -(1 as libc::c_int) as libc::c_schar,
        -(1 as libc::c_int) as libc::c_schar,
    ];
    let mut p: *const libc::c_char = base64;
    let mut q: *mut libc::c_uchar = dest as *mut libc::c_uchar;
    let mut n: ssize_t = 0 as libc::c_int as ssize_t;
    loop {
        let mut c: libc::c_uchar = 0;
        let mut value: libc::c_ulong = 0;
        loop {
            let fresh218 = p;
            p = p.offset(1);
            c = *fresh218 as libc::c_uchar;
            if !c_isspace(c as libc::c_int) {
                break;
            }
        }
        if c == 0 {
            break;
        }
        if c as libc::c_int == '=' as i32
            || !(c as libc::c_int & 0x80 as libc::c_int == 0 as libc::c_int
                && base64_char_to_value[c as usize] as libc::c_int >= 0 as libc::c_int
                || c as libc::c_int == '=' as i32)
        {
            return -(1 as libc::c_int) as ssize_t;
        }
        value = ((base64_char_to_value[c as usize] as libc::c_int) << 18 as libc::c_int)
            as libc::c_ulong;
        loop {
            let fresh219 = p;
            p = p.offset(1);
            c = *fresh219 as libc::c_uchar;
            if !c_isspace(c as libc::c_int) {
                break;
            }
        }
        if c == 0 {
            return -(1 as libc::c_int) as ssize_t;
        }
        if c as libc::c_int == '=' as i32
            || !(c as libc::c_int & 0x80 as libc::c_int == 0 as libc::c_int
                && base64_char_to_value[c as usize] as libc::c_int >= 0 as libc::c_int
                || c as libc::c_int == '=' as i32)
        {
            return -(1 as libc::c_int) as ssize_t;
        }
        value
            |= ((base64_char_to_value[c as usize] as libc::c_int) << 12 as libc::c_int)
                as libc::c_ulong;
        if size != 0 {
            let fresh220 = q;
            q = q.offset(1);
            *fresh220 = (value >> 16 as libc::c_int) as libc::c_uchar;
            size = size.wrapping_sub(1);
            size;
        }
        n += 1;
        n;
        loop {
            let fresh221 = p;
            p = p.offset(1);
            c = *fresh221 as libc::c_uchar;
            if !c_isspace(c as libc::c_int) {
                break;
            }
        }
        if c == 0 {
            return -(1 as libc::c_int) as ssize_t;
        }
        if !(c as libc::c_int & 0x80 as libc::c_int == 0 as libc::c_int
            && base64_char_to_value[c as usize] as libc::c_int >= 0 as libc::c_int
            || c as libc::c_int == '=' as i32)
        {
            return -(1 as libc::c_int) as ssize_t;
        }
        if c as libc::c_int == '=' as i32 {
            loop {
                let fresh222 = p;
                p = p.offset(1);
                c = *fresh222 as libc::c_uchar;
                if !c_isspace(c as libc::c_int) {
                    break;
                }
            }
            if c == 0 {
                return -(1 as libc::c_int) as ssize_t;
            }
            if c as libc::c_int != '=' as i32 {
                return -(1 as libc::c_int) as ssize_t;
            }
        } else {
            value
                |= ((base64_char_to_value[c as usize] as libc::c_int)
                    << 6 as libc::c_int) as libc::c_ulong;
            if size != 0 {
                let fresh223 = q;
                q = q.offset(1);
                *fresh223 = (0xff as libc::c_int as libc::c_ulong
                    & value >> 8 as libc::c_int) as libc::c_uchar;
                size = size.wrapping_sub(1);
                size;
            }
            n += 1;
            n;
            loop {
                let fresh224 = p;
                p = p.offset(1);
                c = *fresh224 as libc::c_uchar;
                if !c_isspace(c as libc::c_int) {
                    break;
                }
            }
            if c == 0 {
                return -(1 as libc::c_int) as ssize_t;
            }
            if c as libc::c_int == '=' as i32 {
                continue;
            }
            if !(c as libc::c_int & 0x80 as libc::c_int == 0 as libc::c_int
                && base64_char_to_value[c as usize] as libc::c_int >= 0 as libc::c_int
                || c as libc::c_int == '=' as i32)
            {
                return -(1 as libc::c_int) as ssize_t;
            }
            value |= base64_char_to_value[c as usize] as libc::c_int as libc::c_ulong;
            if size != 0 {
                let fresh225 = q;
                q = q.offset(1);
                *fresh225 = (0xff as libc::c_int as libc::c_ulong & value)
                    as libc::c_uchar;
                size = size.wrapping_sub(1);
                size;
            }
            n += 1;
            n;
        }
    }
    return n;
}
pub unsafe extern "C" fn compile_pcre_regex(
    mut str: *const libc::c_char,
) -> *mut libc::c_void {
    let mut errbuf: *const libc::c_char = 0 as *const libc::c_char;
    let mut erroffset: libc::c_int = 0;
    let mut regex: *mut pcre = pcre_compile(
        str,
        0 as libc::c_int,
        &mut errbuf,
        &mut erroffset,
        0 as *const libc::c_uchar,
    );
    if regex.is_null() {
        fprintf(
            stderr,
            dcgettext(
                0 as *const libc::c_char,
                b"Invalid regular expression %s, %s\n\0" as *const u8
                    as *const libc::c_char,
                5 as libc::c_int,
            ),
            quote(str),
            errbuf,
        );
    }
    return regex as *mut libc::c_void;
}
pub unsafe extern "C" fn compile_posix_regex(
    mut str: *const libc::c_char,
) -> *mut libc::c_void {
    let mut regex: *mut regex_t = xmalloc(
        ::std::mem::size_of::<regex_t>() as libc::c_ulong,
    ) as *mut regex_t;
    let mut errcode: libc::c_int = rpl_regcomp(
        regex,
        str,
        1 as libc::c_int | (1 as libc::c_int) << 3 as libc::c_int,
    );
    if errcode != 0 as libc::c_int {
        let mut errbuf_size: size_t = rpl_regerror(
            errcode,
            regex,
            0 as *mut libc::c_char,
            0 as libc::c_int as size_t,
        );
        let mut errbuf: *mut libc::c_char = xmalloc(errbuf_size) as *mut libc::c_char;
        rpl_regerror(errcode, regex, errbuf, errbuf_size);
        fprintf(
            stderr,
            dcgettext(
                0 as *const libc::c_char,
                b"Invalid regular expression %s, %s\n\0" as *const u8
                    as *const libc::c_char,
                5 as libc::c_int,
            ),
            quote(str),
            errbuf,
        );
        rpl_free(errbuf as *mut libc::c_void);
        errbuf = 0 as *mut libc::c_char;
        rpl_free(regex as *mut libc::c_void);
        regex = 0 as *mut regex_t;
        return 0 as *mut libc::c_void;
    }
    return regex as *mut libc::c_void;
}
pub unsafe extern "C" fn match_pcre_regex(
    mut regex: *const libc::c_void,
    mut str: *const libc::c_char,
) -> bool {
    let mut l: size_t = strlen(str);
    let mut ovector: [libc::c_int; 30] = [0; 30];
    let mut rc: libc::c_int = pcre_exec(
        regex as *mut pcre,
        0 as *const pcre_extra,
        str,
        l as libc::c_int,
        0 as libc::c_int,
        0 as libc::c_int,
        ovector.as_mut_ptr(),
        30 as libc::c_int,
    );
    if rc == -(1 as libc::c_int) {
        return 0 as libc::c_int != 0
    } else if rc < 0 as libc::c_int {
        logprintf(
            LOG_VERBOSE,
            dcgettext(
                0 as *const libc::c_char,
                b"Error while matching %s: %d\n\0" as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ),
            quote(str),
            rc,
        );
        return 0 as libc::c_int != 0;
    } else {
        return 1 as libc::c_int != 0
    };
}
pub unsafe extern "C" fn match_posix_regex(
    mut regex: *const libc::c_void,
    mut str: *const libc::c_char,
) -> bool {
    let mut rc: libc::c_int = rpl_regexec(
        regex as *mut regex_t,
        str,
        0 as libc::c_int as size_t,
        0 as *mut regmatch_t,
        0 as libc::c_int,
    );
    if rc == _REG_NOMATCH as libc::c_int {
        return 0 as libc::c_int != 0
    } else if rc == 0 as libc::c_int {
        return 1 as libc::c_int != 0
    } else {
        let mut errbuf_size: size_t = rpl_regerror(
            rc,
            opt.acceptregex as *const regex_t,
            0 as *mut libc::c_char,
            0 as libc::c_int as size_t,
        );
        let mut errbuf: *mut libc::c_char = xmalloc(errbuf_size) as *mut libc::c_char;
        rpl_regerror(rc, opt.acceptregex as *const regex_t, errbuf, errbuf_size);
        logprintf(
            LOG_VERBOSE,
            dcgettext(
                0 as *const libc::c_char,
                b"Error while matching %s: %d\n\0" as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ),
            quote(str),
            rc,
        );
        rpl_free(errbuf as *mut libc::c_void);
        errbuf = 0 as *mut libc::c_char;
        return 0 as libc::c_int != 0;
    };
}
unsafe extern "C" fn mergesort_internal(
    mut base: *mut libc::c_void,
    mut temp: *mut libc::c_void,
    mut size: size_t,
    mut from: size_t,
    mut to: size_t,
    mut cmpfun: Option::<
        unsafe extern "C" fn(*const libc::c_void, *const libc::c_void) -> libc::c_int,
    >,
) {
    if from < to {
        let mut i: size_t = 0;
        let mut j: size_t = 0;
        let mut k: size_t = 0;
        let mut mid: size_t = to
            .wrapping_add(from)
            .wrapping_div(2 as libc::c_int as libc::c_ulong);
        mergesort_internal(base, temp, size, from, mid, cmpfun);
        mergesort_internal(
            base,
            temp,
            size,
            mid.wrapping_add(1 as libc::c_int as libc::c_ulong),
            to,
            cmpfun,
        );
        i = from;
        j = mid.wrapping_add(1 as libc::c_int as libc::c_ulong);
        k = from;
        while i <= mid && j <= to {
            if cmpfun
                .unwrap()(
                (base as *mut libc::c_char).offset(i.wrapping_mul(size) as isize)
                    as *const libc::c_void,
                (base as *mut libc::c_char).offset(j.wrapping_mul(size) as isize)
                    as *const libc::c_void,
            ) <= 0 as libc::c_int
            {
                let fresh226 = i;
                i = i.wrapping_add(1);
                memcpy(
                    (temp as *mut libc::c_char).offset(k.wrapping_mul(size) as isize)
                        as *mut libc::c_void,
                    (base as *mut libc::c_char)
                        .offset(fresh226.wrapping_mul(size) as isize)
                        as *const libc::c_void,
                    size,
                );
            } else {
                let fresh227 = j;
                j = j.wrapping_add(1);
                memcpy(
                    (temp as *mut libc::c_char).offset(k.wrapping_mul(size) as isize)
                        as *mut libc::c_void,
                    (base as *mut libc::c_char)
                        .offset(fresh227.wrapping_mul(size) as isize)
                        as *const libc::c_void,
                    size,
                );
            }
            k = k.wrapping_add(1);
            k;
        }
        while i <= mid {
            let fresh228 = k;
            k = k.wrapping_add(1);
            let fresh229 = i;
            i = i.wrapping_add(1);
            memcpy(
                (temp as *mut libc::c_char).offset(fresh228.wrapping_mul(size) as isize)
                    as *mut libc::c_void,
                (base as *mut libc::c_char).offset(fresh229.wrapping_mul(size) as isize)
                    as *const libc::c_void,
                size,
            );
        }
        while j <= to {
            let fresh230 = k;
            k = k.wrapping_add(1);
            let fresh231 = j;
            j = j.wrapping_add(1);
            memcpy(
                (temp as *mut libc::c_char).offset(fresh230.wrapping_mul(size) as isize)
                    as *mut libc::c_void,
                (base as *mut libc::c_char).offset(fresh231.wrapping_mul(size) as isize)
                    as *const libc::c_void,
                size,
            );
        }
        k = from;
        while k <= to {
            memcpy(
                (base as *mut libc::c_char).offset(k.wrapping_mul(size) as isize)
                    as *mut libc::c_void,
                (temp as *mut libc::c_char).offset(k.wrapping_mul(size) as isize)
                    as *const libc::c_void,
                size,
            );
            k = k.wrapping_add(1);
            k;
        }
    }
}
pub unsafe extern "C" fn stable_sort(
    mut base: *mut libc::c_void,
    mut nmemb: size_t,
    mut size: size_t,
    mut cmpfun: Option::<
        unsafe extern "C" fn(*const libc::c_void, *const libc::c_void) -> libc::c_int,
    >,
) {
    if nmemb > 1 as libc::c_int as libc::c_ulong
        && size > 1 as libc::c_int as libc::c_ulong
    {
        let mut temp: *mut libc::c_void = xmalloc(nmemb.wrapping_mul(size));
        mergesort_internal(
            base,
            temp,
            size,
            0 as libc::c_int as size_t,
            nmemb.wrapping_sub(1 as libc::c_int as libc::c_ulong),
            cmpfun,
        );
        rpl_free(temp);
        temp = 0 as *mut libc::c_void;
    }
}
pub unsafe extern "C" fn print_decimal(
    mut number: libc::c_double,
) -> *const libc::c_char {
    static mut buf: [libc::c_char; 32] = [0; 32];
    let mut n: libc::c_double = if number >= 0 as libc::c_int as libc::c_double {
        number
    } else {
        -number
    };
    if n >= 9.95f64 {
        snprintf(
            buf.as_mut_ptr(),
            ::std::mem::size_of::<[libc::c_char; 32]>() as libc::c_ulong,
            b"%.0f\0" as *const u8 as *const libc::c_char,
            number,
        );
    } else if n >= 0.95f64 {
        snprintf(
            buf.as_mut_ptr(),
            ::std::mem::size_of::<[libc::c_char; 32]>() as libc::c_ulong,
            b"%.1f\0" as *const u8 as *const libc::c_char,
            number,
        );
    } else if n >= 0.001f64 {
        snprintf(
            buf.as_mut_ptr(),
            ::std::mem::size_of::<[libc::c_char; 32]>() as libc::c_ulong,
            b"%.1g\0" as *const u8 as *const libc::c_char,
            number,
        );
    } else if n >= 0.0005f64 {
        snprintf(
            buf.as_mut_ptr(),
            ::std::mem::size_of::<[libc::c_char; 32]>() as libc::c_ulong,
            b"%.3f\0" as *const u8 as *const libc::c_char,
            number,
        );
    } else {
        strcpy(buf.as_mut_ptr(), b"0\0" as *const u8 as *const libc::c_char);
    }
    return buf.as_mut_ptr();
}
pub unsafe extern "C" fn get_max_length(
    mut path: *const libc::c_char,
    mut length: libc::c_int,
    mut name: libc::c_int,
) -> libc::c_long {
    let mut ret: libc::c_long = 0;
    let mut p: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut d: *mut libc::c_char = 0 as *mut libc::c_char;
    p = if !path.is_null() {
        strdupdelim(path, path.offset(length as isize))
    } else {
        strdup(b"\0" as *const u8 as *const libc::c_char)
    };
    loop {
        *__errno_location() = 0 as libc::c_int;
        ret = pathconf(
            if *p as libc::c_int != 0 {
                p as *const libc::c_char
            } else {
                b".\0" as *const u8 as *const libc::c_char
            },
            name,
        );
        if !(ret < 0 as libc::c_int as libc::c_long
            && *__errno_location() == 2 as libc::c_int)
        {
            break;
        }
        if *p == 0
            || strcmp(p, b"/\0" as *const u8 as *const libc::c_char) == 0 as libc::c_int
        {
            break;
        }
        d = strrchr(p, '/' as i32);
        if d == p {
            *p.offset(1 as libc::c_int as isize) = '\0' as i32 as libc::c_char;
        } else if !d.is_null() {
            *d = '\0' as i32 as libc::c_char;
        } else {
            *p = '\0' as i32 as libc::c_char;
        }
    }
    rpl_free(p as *mut libc::c_void);
    p = 0 as *mut libc::c_char;
    if ret < 0 as libc::c_int as libc::c_long {
        if *__errno_location() != 0 as libc::c_int {
            perror(b"pathconf\0" as *const u8 as *const libc::c_char);
        }
        return 0 as libc::c_int as libc::c_long;
    }
    return ret;
}
pub unsafe extern "C" fn wg_hex_to_string(
    mut str_buffer: *mut libc::c_char,
    mut hex_buffer: *const libc::c_char,
    mut hex_len: size_t,
) {
    let mut i: size_t = 0;
    i = 0 as libc::c_int as size_t;
    while i < hex_len {
        sprintf(
            str_buffer
                .offset((2 as libc::c_int as libc::c_ulong).wrapping_mul(i) as isize),
            b"%02x\0" as *const u8 as *const libc::c_char,
            (*hex_buffer.offset(i as isize) as libc::c_int & 0xff as libc::c_int)
                as libc::c_uint,
        );
        i = i.wrapping_add(1);
        i;
    }
    *str_buffer
        .offset(
            (2 as libc::c_int as libc::c_ulong).wrapping_mul(i) as isize,
        ) = '\0' as i32 as libc::c_char;
}
