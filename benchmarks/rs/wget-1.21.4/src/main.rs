use ::libc;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    pub type __spawn_action;
    pub type ptimer;
    fn time(__timer: *mut time_t) -> time_t;
    fn __fxstat(
        __ver: libc::c_int,
        __fildes: libc::c_int,
        __stat_buf: *mut stat,
    ) -> libc::c_int;
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
    fn strpbrk(_: *const libc::c_char, _: *const libc::c_char) -> *mut libc::c_char;
    fn strtok(_: *mut libc::c_char, _: *const libc::c_char) -> *mut libc::c_char;
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    fn strerror(_: libc::c_int) -> *mut libc::c_char;
    fn rpl_free(_: *mut libc::c_void);
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    fn exit(_: libc::c_int) -> !;
    fn xstrdup(str: *const libc::c_char) -> *mut libc::c_char;
    static mut stdout: *mut FILE;
    static mut stderr: *mut FILE;
    fn fprintf(_: *mut FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
    fn snprintf(
        _: *mut libc::c_char,
        _: libc::c_ulong,
        _: *const libc::c_char,
        _: ...
    ) -> libc::c_int;
    fn fputs(__s: *const libc::c_char, __stream: *mut FILE) -> libc::c_int;
    fn perror(__s: *const libc::c_char);
    fn fileno(__stream: *mut FILE) -> libc::c_int;
    fn rpl_fopen(filename: *const libc::c_char, mode: *const libc::c_char) -> *mut FILE;
    fn logprintf(_: log_options, _: *const libc::c_char, _: ...);
    fn debug_logprintf(_: *const libc::c_char, _: ...);
    fn log_init(_: *const libc::c_char, _: bool);
    fn redirect_output(_: bool, _: *const libc::c_char);
    fn quote(arg: *const libc::c_char) -> *const libc::c_char;
    fn find_locale() -> *const libc::c_char;
    fn check_encoding_name(encoding: *const libc::c_char) -> bool;
    fn iri_new() -> *mut iri;
    fn iri_free(i: *mut iri);
    fn set_uri_encoding(i: *mut iri, charset: *const libc::c_char, force: bool);
    fn close(__fd: libc::c_int) -> libc::c_int;
    fn read(__fd: libc::c_int, __buf: *mut libc::c_void, __nbytes: size_t) -> ssize_t;
    fn pipe(__pipedes: *mut libc::c_int) -> libc::c_int;
    static mut environ: *mut *mut libc::c_char;
    fn unlink(__name: *const libc::c_char) -> libc::c_int;
    fn getpass(__prompt: *const libc::c_char) -> *mut libc::c_char;
    static mut optarg: *mut libc::c_char;
    static mut opterr: libc::c_int;
    static mut optind: libc::c_int;
    fn signal(__sig: libc::c_int, __handler: __sighandler_t) -> __sighandler_t;
    fn posix_spawnp(
        __pid: *mut pid_t,
        __file: *const libc::c_char,
        __file_actions: *const posix_spawn_file_actions_t,
        __attrp: *const posix_spawnattr_t,
        __argv: *const *mut libc::c_char,
        __envp: *const *mut libc::c_char,
    ) -> libc::c_int;
    fn posix_spawn_file_actions_init(
        __file_actions: *mut posix_spawn_file_actions_t,
    ) -> libc::c_int;
    fn posix_spawn_file_actions_adddup2(
        __file_actions: *mut posix_spawn_file_actions_t,
        __fd: libc::c_int,
        __newfd: libc::c_int,
    ) -> libc::c_int;
    fn __errno_location() -> *mut libc::c_int;
    fn inform_exit_status(err: uerr_t);
    fn get_exit_status() -> libc::c_int;
    fn datetime_str(_: time_t) -> *mut libc::c_char;
    fn fork_to_background() -> bool;
    fn file_exists_p(_: *const libc::c_char, _: *mut file_stats_t) -> bool;
    fn human_readable(_: wgint, _: libc::c_int, _: libc::c_int) -> *mut libc::c_char;
    fn compile_pcre_regex(_: *const libc::c_char) -> *mut libc::c_void;
    fn match_pcre_regex(_: *const libc::c_void, _: *const libc::c_char) -> bool;
    fn compile_posix_regex(_: *const libc::c_char) -> *mut libc::c_void;
    fn match_posix_regex(_: *const libc::c_void, _: *const libc::c_char) -> bool;
    fn print_decimal(_: libc::c_double) -> *const libc::c_char;
    fn wgetrc_env_file_name() -> *mut libc::c_char;
    fn wgetrc_user_file_name() -> *mut libc::c_char;
    fn initialize() -> libc::c_int;
    fn run_command(_: *const libc::c_char);
    fn setoptval(_: *const libc::c_char, _: *const libc::c_char, _: *const libc::c_char);
    fn home_dir() -> *mut libc::c_char;
    fn cleanup();
    fn defaults();
    fn run_wgetrc(file: *const libc::c_char, _: *mut file_stats_t) -> bool;
    fn url_parse(
        _: *const libc::c_char,
        _: *mut libc::c_int,
        iri: *mut iri,
        percent_encode: bool,
    ) -> *mut url;
    fn url_error(_: libc::c_int) -> *const libc::c_char;
    fn url_free(_: *mut url);
    fn url_scheme(_: *const libc::c_char) -> url_scheme;
    fn scheme_leading_string(_: url_scheme) -> *const libc::c_char;
    fn rewrite_shorthand_url(_: *const libc::c_char) -> *mut libc::c_char;
    static mut total_downloaded_bytes: wgint;
    static mut total_download_time: libc::c_double;
    static mut output_stream: *mut FILE;
    static mut output_stream_regular: bool;
    fn retrieve_url(
        _: *mut url,
        _: *const libc::c_char,
        _: *mut *mut libc::c_char,
        _: *mut *mut libc::c_char,
        _: *const libc::c_char,
        _: *mut libc::c_int,
        _: bool,
        _: *mut iri,
        _: bool,
    ) -> uerr_t;
    fn retrieve_from_file(
        _: *const libc::c_char,
        _: bool,
        _: *mut libc::c_int,
    ) -> uerr_t;
    fn retr_rate(_: wgint, _: libc::c_double) -> *const libc::c_char;
    fn url_uses_proxy(_: *mut url) -> bool;
    fn retrieve_tree(_: *mut url, _: *mut iri) -> uerr_t;
    fn set_progress_implementation(_: *const libc::c_char);
    fn progress_schedule_redirect();
    fn progress_handle_sigwinch(_: libc::c_int);
    fn convert_all_links();
    fn print_broken_links();
    fn save_cookies();
    fn ptimer_new() -> *mut ptimer;
    fn ptimer_destroy(_: *mut ptimer);
    fn ptimer_measure(_: *mut ptimer) -> libc::c_double;
    fn warc_init();
    static mut version_string: *const libc::c_char;
    static mut compilation_string: *const libc::c_char;
    static mut link_string: *const libc::c_char;
    static mut compiled_features: [*const libc::c_char; 0];
    fn c_strcasecmp(s1: *const libc::c_char, s2: *const libc::c_char) -> libc::c_int;
    fn base_name(file: *const libc::c_char) -> *mut libc::c_char;
    fn xmemdup0(p: *const libc::c_void, s: size_t) -> *mut libc::c_char;
    fn getopt_long(
        ___argc: libc::c_int,
        ___argv: *const *mut libc::c_char,
        __shortopts: *const libc::c_char,
        __longopts: *const option,
        __longind: *mut libc::c_int,
    ) -> libc::c_int;
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
pub type __blksize_t = libc::c_long;
pub type __blkcnt_t = libc::c_long;
pub type __ssize_t = libc::c_long;
pub type __syscall_slong_t = libc::c_long;
pub type ino_t = __ino_t;
pub type dev_t = __dev_t;
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
pub type sigset_t = __sigset_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct timespec {
    pub tv_sec: __time_t,
    pub tv_nsec: __syscall_slong_t,
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
pub struct iri {
    pub uri_encoding: *mut libc::c_char,
    pub content_encoding: *mut libc::c_char,
    pub orig_url: *mut libc::c_char,
    pub utf8_encode: bool,
}
pub type uerr_t = libc::c_uint;
pub const METALINK_SIZE_ERROR: uerr_t = 62;
pub const RETR_WITH_METALINK: uerr_t = 61;
pub const METALINK_MISSING_RESOURCE: uerr_t = 60;
pub const METALINK_SIG_ERROR: uerr_t = 59;
pub const METALINK_CHKSUM_ERROR: uerr_t = 58;
pub const METALINK_RETR_ERROR: uerr_t = 57;
pub const METALINK_PARSE_ERROR: uerr_t = 56;
pub const TIMECONV_ERR: uerr_t = 55;
pub const WARC_TMP_FWRITEERR: uerr_t = 54;
pub const WARC_TMP_FOPENERR: uerr_t = 53;
pub const WARC_ERR: uerr_t = 52;
pub const UNKNOWNATTR: uerr_t = 51;
pub const ATTRMISSING: uerr_t = 50;
pub const CLOSEFAILED: uerr_t = 49;
pub const NEWLOCATION_KEEP_POST: uerr_t = 48;
pub const UNLINKERR: uerr_t = 47;
pub const VERIFCERTERR: uerr_t = 46;
pub const SSLINITFAILED: uerr_t = 45;
pub const WRITEFAILED: uerr_t = 44;
pub const QUOTEXC: uerr_t = 43;
pub const AUTHFAILED: uerr_t = 42;
pub const PROXERR: uerr_t = 41;
pub const RETRBADPATTERN: uerr_t = 40;
pub const RANGEERR: uerr_t = 39;
pub const FILEBADFILE: uerr_t = 38;
pub const TRYLIMEXC: uerr_t = 37;
pub const READERR: uerr_t = 36;
pub const RETRFINISHED: uerr_t = 35;
pub const RETRUNNEEDED: uerr_t = 34;
pub const CONTNOTSUPPORTED: uerr_t = 33;
pub const FTPNOAUTH: uerr_t = 32;
pub const FTPNOPROT: uerr_t = 31;
pub const FTPNOPBSZ: uerr_t = 30;
pub const FTPNOPASV: uerr_t = 29;
pub const FTPINVPASV: uerr_t = 28;
pub const WRONGCODE: uerr_t = 27;
pub const RECLEVELEXC: uerr_t = 26;
pub const RETROK: uerr_t = 25;
pub const HERR: uerr_t = 24;
pub const GATEWAYTIMEOUT: uerr_t = 23;
pub const HEOF: uerr_t = 22;
pub const FWRITEERR: uerr_t = 21;
pub const FOPEN_EXCL_ERR: uerr_t = 20;
pub const FOPENERR: uerr_t = 19;
pub const URLERROR: uerr_t = 18;
pub const FTPRESTFAIL: uerr_t = 17;
pub const FTPRETRINT: uerr_t = 16;
pub const FTPSRVERR: uerr_t = 15;
pub const FTPRERR: uerr_t = 14;
pub const FTPUNKNOWNTYPE: uerr_t = 13;
pub const FTPNSFOD: uerr_t = 12;
pub const FTPSYSERR: uerr_t = 11;
pub const FTPPORTERR: uerr_t = 10;
pub const FTPLOGREFUSED: uerr_t = 9;
pub const FTPLOGINC: uerr_t = 8;
pub const FTPOK: uerr_t = 7;
pub const NEWLOCATION: uerr_t = 6;
pub const CONIMPOSSIBLE: uerr_t = 5;
pub const CONSSLERR: uerr_t = 4;
pub const CONERROR: uerr_t = 3;
pub const CONSOCKERR: uerr_t = 2;
pub const HOSTERR: uerr_t = 1;
pub const NOCONERROR: uerr_t = 0;
pub type __sighandler_t = Option::<unsafe extern "C" fn(libc::c_int) -> ()>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sched_param {
    pub sched_priority: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct posix_spawnattr_t {
    pub __flags: libc::c_short,
    pub __pgrp: pid_t,
    pub __sd: sigset_t,
    pub __ss: sigset_t,
    pub __sp: sched_param,
    pub __policy: libc::c_int,
    pub __pad: [libc::c_int; 16],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct posix_spawn_file_actions_t {
    pub __allocated: libc::c_int,
    pub __used: libc::c_int,
    pub __actions: *mut __spawn_action,
    pub __pad: [libc::c_int; 16],
}
pub type C2RustUnnamed_3 = libc::c_uint;
pub const WGET_EXIT_UNKNOWN: C2RustUnnamed_3 = 9;
pub const WGET_EXIT_SERVER_ERROR: C2RustUnnamed_3 = 8;
pub const WGET_EXIT_PROTOCOL_ERROR: C2RustUnnamed_3 = 7;
pub const WGET_EXIT_SERVER_AUTH_FAIL: C2RustUnnamed_3 = 6;
pub const WGET_EXIT_SSL_AUTH_FAIL: C2RustUnnamed_3 = 5;
pub const WGET_EXIT_NETWORK_FAIL: C2RustUnnamed_3 = 4;
pub const WGET_EXIT_IO_FAIL: C2RustUnnamed_3 = 3;
pub const WGET_EXIT_PARSE_ERROR: C2RustUnnamed_3 = 2;
pub const WGET_EXIT_GENERIC_ERROR: C2RustUnnamed_3 = 1;
pub const WGET_EXIT_SUCCESS: C2RustUnnamed_3 = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct file_stat_s {
    pub access_err: libc::c_int,
    pub st_ino: ino_t,
    pub st_dev: dev_t,
}
pub type file_stats_t = file_stat_s;
pub type url_scheme = libc::c_uint;
pub const SCHEME_INVALID: url_scheme = 2;
pub const SCHEME_FTP: url_scheme = 1;
pub const SCHEME_HTTP: url_scheme = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct url {
    pub url: *mut libc::c_char,
    pub scheme: url_scheme,
    pub host: *mut libc::c_char,
    pub port: libc::c_int,
    pub path: *mut libc::c_char,
    pub params: *mut libc::c_char,
    pub query: *mut libc::c_char,
    pub fragment: *mut libc::c_char,
    pub dir: *mut libc::c_char,
    pub file: *mut libc::c_char,
    pub user: *mut libc::c_char,
    pub passwd: *mut libc::c_char,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct option {
    pub name: *const libc::c_char,
    pub has_arg: libc::c_int,
    pub flag: *mut libc::c_int,
    pub val: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct cmdline_option {
    pub long_name: [libc::c_char; 26],
    pub short_name: libc::c_char,
    pub type_0: C2RustUnnamed_4,
    pub data: *const libc::c_void,
    pub argtype: libc::c_int,
}
pub type C2RustUnnamed_4 = libc::c_uint;
pub const OPT__PARENT: C2RustUnnamed_4 = 8;
pub const OPT__NO: C2RustUnnamed_4 = 7;
pub const OPT__EXECUTE: C2RustUnnamed_4 = 6;
pub const OPT__DONT_REMOVE_LISTING: C2RustUnnamed_4 = 5;
pub const OPT__CLOBBER: C2RustUnnamed_4 = 4;
pub const OPT__APPEND_OUTPUT: C2RustUnnamed_4 = 3;
pub const OPT_FUNCALL: C2RustUnnamed_4 = 2;
pub const OPT_BOOLEAN: C2RustUnnamed_4 = 1;
pub const OPT_VALUE: C2RustUnnamed_4 = 0;
#[inline]
unsafe extern "C" fn fstat(
    mut __fd: libc::c_int,
    mut __statbuf: *mut stat,
) -> libc::c_int {
    return __fxstat(1 as libc::c_int, __fd, __statbuf);
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
pub static mut ares: *mut libc::c_void = 0 as *const libc::c_void as *mut libc::c_void;
pub static mut opt: options = options {
    verbose: 0,
    quiet: false,
    ntry: 0,
    retry_connrefused: false,
    retry_on_host_error: false,
    retry_on_http_error: 0 as *const libc::c_char as *mut libc::c_char,
    background: false,
    ignore_length: false,
    recursive: false,
    spanhost: false,
    max_redirect: 0,
    relative_only: false,
    no_parent: false,
    reclevel: 0,
    dirstruct: false,
    no_dirstruct: false,
    cut_dirs: 0,
    add_hostdir: false,
    protocol_directories: false,
    noclobber: false,
    unlink_requested: false,
    dir_prefix: 0 as *const libc::c_char as *mut libc::c_char,
    lfilename: 0 as *const libc::c_char as *mut libc::c_char,
    input_filename: 0 as *const libc::c_char as *mut libc::c_char,
    choose_config: 0 as *const libc::c_char as *mut libc::c_char,
    noconfig: false,
    force_html: false,
    default_page: 0 as *const libc::c_char as *mut libc::c_char,
    spider: false,
    accepts: 0 as *const *mut libc::c_char as *mut *mut libc::c_char,
    rejects: 0 as *const *mut libc::c_char as *mut *mut libc::c_char,
    excludes: 0 as *const *const libc::c_char as *mut *const libc::c_char,
    includes: 0 as *const *const libc::c_char as *mut *const libc::c_char,
    ignore_case: false,
    acceptregex_s: 0 as *const libc::c_char as *mut libc::c_char,
    rejectregex_s: 0 as *const libc::c_char as *mut libc::c_char,
    acceptregex: 0 as *const libc::c_void as *mut libc::c_void,
    rejectregex: 0 as *const libc::c_void as *mut libc::c_void,
    regex_type: regex_type_pcre,
    regex_compile_fun: None,
    regex_match_fun: None,
    domains: 0 as *const *mut libc::c_char as *mut *mut libc::c_char,
    exclude_domains: 0 as *const *mut libc::c_char as *mut *mut libc::c_char,
    dns_cache: false,
    follow_tags: 0 as *const *mut libc::c_char as *mut *mut libc::c_char,
    ignore_tags: 0 as *const *mut libc::c_char as *mut *mut libc::c_char,
    follow_ftp: false,
    retr_symlinks: false,
    output_document: 0 as *const libc::c_char as *mut libc::c_char,
    warc_filename: 0 as *const libc::c_char as *mut libc::c_char,
    warc_tempdir: 0 as *const libc::c_char as *mut libc::c_char,
    warc_cdx_dedup_filename: 0 as *const libc::c_char as *mut libc::c_char,
    warc_maxsize: 0,
    warc_compression_enabled: false,
    warc_digests_enabled: false,
    warc_cdx_enabled: false,
    warc_keep_log: false,
    warc_user_headers: 0 as *const *mut libc::c_char as *mut *mut libc::c_char,
    enable_xattr: false,
    user: 0 as *const libc::c_char as *mut libc::c_char,
    passwd: 0 as *const libc::c_char as *mut libc::c_char,
    ask_passwd: false,
    use_askpass: 0 as *const libc::c_char as *mut libc::c_char,
    always_rest: false,
    start_pos: 0,
    ftp_user: 0 as *const libc::c_char as *mut libc::c_char,
    ftp_passwd: 0 as *const libc::c_char as *mut libc::c_char,
    netrc: false,
    ftp_glob: false,
    ftp_pasv: false,
    http_user: 0 as *const libc::c_char as *mut libc::c_char,
    http_passwd: 0 as *const libc::c_char as *mut libc::c_char,
    user_headers: 0 as *const *mut libc::c_char as *mut *mut libc::c_char,
    http_keep_alive: false,
    use_proxy: false,
    allow_cache: false,
    http_proxy: 0 as *const libc::c_char as *mut libc::c_char,
    ftp_proxy: 0 as *const libc::c_char as *mut libc::c_char,
    https_proxy: 0 as *const libc::c_char as *mut libc::c_char,
    no_proxy: 0 as *const *mut libc::c_char as *mut *mut libc::c_char,
    base_href: 0 as *const libc::c_char as *mut libc::c_char,
    progress_type: 0 as *const libc::c_char as *mut libc::c_char,
    show_progress: 0,
    noscroll: false,
    proxy_user: 0 as *const libc::c_char as *mut libc::c_char,
    proxy_passwd: 0 as *const libc::c_char as *mut libc::c_char,
    read_timeout: 0.,
    dns_timeout: 0.,
    connect_timeout: 0.,
    random_wait: false,
    wait: 0.,
    waitretry: 0.,
    use_robots: false,
    limit_rate: 0,
    quota: 0,
    server_response: false,
    save_headers: false,
    content_on_error: false,
    debug: false,
    timestamping: false,
    if_modified_since: false,
    backup_converted: false,
    backups: 0,
    useragent: 0 as *const libc::c_char as *mut libc::c_char,
    referer: 0 as *const libc::c_char as *mut libc::c_char,
    convert_links: false,
    convert_file_only: false,
    remove_listing: false,
    htmlify: false,
    dot_style: 0 as *const libc::c_char as *mut libc::c_char,
    dot_bytes: 0,
    dots_in_line: 0,
    dot_spacing: 0,
    delete_after: false,
    adjust_extension: false,
    page_requisites: false,
    bind_address: 0 as *const libc::c_char as *mut libc::c_char,
    cookies: false,
    cookies_input: 0 as *const libc::c_char as *mut libc::c_char,
    cookies_output: 0 as *const libc::c_char as *mut libc::c_char,
    keep_badhash: false,
    keep_session_cookies: false,
    post_data: 0 as *const libc::c_char as *mut libc::c_char,
    post_file_name: 0 as *const libc::c_char as *mut libc::c_char,
    method: 0 as *const libc::c_char as *mut libc::c_char,
    body_data: 0 as *const libc::c_char as *mut libc::c_char,
    body_file: 0 as *const libc::c_char as *mut libc::c_char,
    restrict_files_os: restrict_unix,
    restrict_files_ctrl: false,
    restrict_files_nonascii: false,
    restrict_files_case: restrict_no_case_restriction,
    strict_comments: false,
    preserve_perm: false,
    ipv4_only: false,
    ipv6_only: false,
    prefer_family: prefer_ipv4,
    content_disposition: false,
    auth_without_challenge: false,
    enable_iri: false,
    encoding_remote: 0 as *const libc::c_char as *mut libc::c_char,
    locale: 0 as *const libc::c_char,
    trustservernames: false,
    useservertimestamps: false,
    show_all_dns_entries: false,
    report_bps: false,
    compression: compression_auto,
    rejected_log: 0 as *const libc::c_char as *mut libc::c_char,
    homedir: 0 as *const libc::c_char,
    wgetrcfile: 0 as *const libc::c_char,
};
pub static mut exec_name: *const libc::c_char = 0 as *const libc::c_char;
pub static mut numurls: libc::c_int = 0 as libc::c_int;
unsafe extern "C" fn redirect_output_signal(mut sig: libc::c_int) {
    let mut signal_name: *const libc::c_char = b"WTF?!\0" as *const u8
        as *const libc::c_char;
    if sig == 1 as libc::c_int {
        signal_name = b"SIGHUP\0" as *const u8 as *const libc::c_char;
    }
    if sig == 10 as libc::c_int {
        signal_name = b"SIGUSR1\0" as *const u8 as *const libc::c_char;
    }
    redirect_output(1 as libc::c_int != 0, signal_name);
    progress_schedule_redirect();
    signal(sig, Some(redirect_output_signal as unsafe extern "C" fn(libc::c_int) -> ()));
}
unsafe extern "C" fn i18n_initialize() {
    setlocale(6 as libc::c_int, b"\0" as *const u8 as *const libc::c_char);
    bindtextdomain(
        b"wget\0" as *const u8 as *const libc::c_char,
        b"/usr/local/share/locale\0" as *const u8 as *const libc::c_char,
    );
    bindtextdomain(
        b"wget-gnulib\0" as *const u8 as *const libc::c_char,
        b"/usr/local/share/locale\0" as *const u8 as *const libc::c_char,
    );
    textdomain(b"wget\0" as *const u8 as *const libc::c_char);
}
static mut option_data: [cmdline_option; 146] = unsafe {
    [
        {
            let mut init = cmdline_option {
                long_name: *::std::mem::transmute::<
                    &[u8; 26],
                    &mut [libc::c_char; 26],
                >(b"accept\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0"),
                short_name: 'A' as i32 as libc::c_char,
                type_0: OPT_VALUE,
                data: b"accept\0" as *const u8 as *const libc::c_char
                    as *const libc::c_void,
                argtype: -(1 as libc::c_int),
            };
            init
        },
        {
            let mut init = cmdline_option {
                long_name: *::std::mem::transmute::<
                    &[u8; 26],
                    &mut [libc::c_char; 26],
                >(b"accept-regex\0\0\0\0\0\0\0\0\0\0\0\0\0\0"),
                short_name: 0 as libc::c_int as libc::c_char,
                type_0: OPT_VALUE,
                data: b"acceptregex\0" as *const u8 as *const libc::c_char
                    as *const libc::c_void,
                argtype: -(1 as libc::c_int),
            };
            init
        },
        {
            let mut init = cmdline_option {
                long_name: *::std::mem::transmute::<
                    &[u8; 26],
                    &mut [libc::c_char; 26],
                >(b"adjust-extension\0\0\0\0\0\0\0\0\0\0"),
                short_name: 'E' as i32 as libc::c_char,
                type_0: OPT_BOOLEAN,
                data: b"adjustextension\0" as *const u8 as *const libc::c_char
                    as *const libc::c_void,
                argtype: -(1 as libc::c_int),
            };
            init
        },
        {
            let mut init = cmdline_option {
                long_name: *::std::mem::transmute::<
                    &[u8; 26],
                    &mut [libc::c_char; 26],
                >(b"append-output\0\0\0\0\0\0\0\0\0\0\0\0\0"),
                short_name: 'a' as i32 as libc::c_char,
                type_0: OPT__APPEND_OUTPUT,
                data: 0 as *const libc::c_void,
                argtype: 1 as libc::c_int,
            };
            init
        },
        {
            let mut init = cmdline_option {
                long_name: *::std::mem::transmute::<
                    &[u8; 26],
                    &mut [libc::c_char; 26],
                >(b"ask-password\0\0\0\0\0\0\0\0\0\0\0\0\0\0"),
                short_name: 0 as libc::c_int as libc::c_char,
                type_0: OPT_BOOLEAN,
                data: b"askpassword\0" as *const u8 as *const libc::c_char
                    as *const libc::c_void,
                argtype: -(1 as libc::c_int),
            };
            init
        },
        {
            let mut init = cmdline_option {
                long_name: *::std::mem::transmute::<
                    &[u8; 26],
                    &mut [libc::c_char; 26],
                >(b"auth-no-challenge\0\0\0\0\0\0\0\0\0"),
                short_name: 0 as libc::c_int as libc::c_char,
                type_0: OPT_BOOLEAN,
                data: b"authnochallenge\0" as *const u8 as *const libc::c_char
                    as *const libc::c_void,
                argtype: -(1 as libc::c_int),
            };
            init
        },
        {
            let mut init = cmdline_option {
                long_name: *::std::mem::transmute::<
                    &[u8; 26],
                    &mut [libc::c_char; 26],
                >(b"background\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0"),
                short_name: 'b' as i32 as libc::c_char,
                type_0: OPT_BOOLEAN,
                data: b"background\0" as *const u8 as *const libc::c_char
                    as *const libc::c_void,
                argtype: -(1 as libc::c_int),
            };
            init
        },
        {
            let mut init = cmdline_option {
                long_name: *::std::mem::transmute::<
                    &[u8; 26],
                    &mut [libc::c_char; 26],
                >(b"backup-converted\0\0\0\0\0\0\0\0\0\0"),
                short_name: 'K' as i32 as libc::c_char,
                type_0: OPT_BOOLEAN,
                data: b"backupconverted\0" as *const u8 as *const libc::c_char
                    as *const libc::c_void,
                argtype: -(1 as libc::c_int),
            };
            init
        },
        {
            let mut init = cmdline_option {
                long_name: *::std::mem::transmute::<
                    &[u8; 26],
                    &mut [libc::c_char; 26],
                >(b"backups\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0"),
                short_name: 0 as libc::c_int as libc::c_char,
                type_0: OPT_BOOLEAN,
                data: b"backups\0" as *const u8 as *const libc::c_char
                    as *const libc::c_void,
                argtype: -(1 as libc::c_int),
            };
            init
        },
        {
            let mut init = cmdline_option {
                long_name: *::std::mem::transmute::<
                    &[u8; 26],
                    &mut [libc::c_char; 26],
                >(b"base\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0"),
                short_name: 'B' as i32 as libc::c_char,
                type_0: OPT_VALUE,
                data: b"base\0" as *const u8 as *const libc::c_char
                    as *const libc::c_void,
                argtype: -(1 as libc::c_int),
            };
            init
        },
        {
            let mut init = cmdline_option {
                long_name: *::std::mem::transmute::<
                    &[u8; 26],
                    &mut [libc::c_char; 26],
                >(b"bind-address\0\0\0\0\0\0\0\0\0\0\0\0\0\0"),
                short_name: 0 as libc::c_int as libc::c_char,
                type_0: OPT_VALUE,
                data: b"bindaddress\0" as *const u8 as *const libc::c_char
                    as *const libc::c_void,
                argtype: -(1 as libc::c_int),
            };
            init
        },
        {
            let mut init = cmdline_option {
                long_name: *::std::mem::transmute::<
                    &[u8; 26],
                    &mut [libc::c_char; 26],
                >(b"body-data\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0"),
                short_name: 0 as libc::c_int as libc::c_char,
                type_0: OPT_VALUE,
                data: b"bodydata\0" as *const u8 as *const libc::c_char
                    as *const libc::c_void,
                argtype: -(1 as libc::c_int),
            };
            init
        },
        {
            let mut init = cmdline_option {
                long_name: *::std::mem::transmute::<
                    &[u8; 26],
                    &mut [libc::c_char; 26],
                >(b"body-file\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0"),
                short_name: 0 as libc::c_int as libc::c_char,
                type_0: OPT_VALUE,
                data: b"bodyfile\0" as *const u8 as *const libc::c_char
                    as *const libc::c_void,
                argtype: -(1 as libc::c_int),
            };
            init
        },
        {
            let mut init = cmdline_option {
                long_name: *::std::mem::transmute::<
                    &[u8; 26],
                    &mut [libc::c_char; 26],
                >(b"cache\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0"),
                short_name: 0 as libc::c_int as libc::c_char,
                type_0: OPT_BOOLEAN,
                data: b"cache\0" as *const u8 as *const libc::c_char
                    as *const libc::c_void,
                argtype: -(1 as libc::c_int),
            };
            init
        },
        {
            let mut init = cmdline_option {
                long_name: *::std::mem::transmute::<
                    &[u8; 26],
                    &mut [libc::c_char; 26],
                >(b"clobber\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0"),
                short_name: 0 as libc::c_int as libc::c_char,
                type_0: OPT__CLOBBER,
                data: 0 as *const libc::c_void,
                argtype: 2 as libc::c_int,
            };
            init
        },
        {
            let mut init = cmdline_option {
                long_name: *::std::mem::transmute::<
                    &[u8; 26],
                    &mut [libc::c_char; 26],
                >(b"compression\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0"),
                short_name: 0 as libc::c_int as libc::c_char,
                type_0: OPT_VALUE,
                data: b"compression\0" as *const u8 as *const libc::c_char
                    as *const libc::c_void,
                argtype: -(1 as libc::c_int),
            };
            init
        },
        {
            let mut init = cmdline_option {
                long_name: *::std::mem::transmute::<
                    &[u8; 26],
                    &mut [libc::c_char; 26],
                >(b"config\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0"),
                short_name: 0 as libc::c_int as libc::c_char,
                type_0: OPT_VALUE,
                data: b"chooseconfig\0" as *const u8 as *const libc::c_char
                    as *const libc::c_void,
                argtype: -(1 as libc::c_int),
            };
            init
        },
        {
            let mut init = cmdline_option {
                long_name: *::std::mem::transmute::<
                    &[u8; 26],
                    &mut [libc::c_char; 26],
                >(b"connect-timeout\0\0\0\0\0\0\0\0\0\0\0"),
                short_name: 0 as libc::c_int as libc::c_char,
                type_0: OPT_VALUE,
                data: b"connecttimeout\0" as *const u8 as *const libc::c_char
                    as *const libc::c_void,
                argtype: -(1 as libc::c_int),
            };
            init
        },
        {
            let mut init = cmdline_option {
                long_name: *::std::mem::transmute::<
                    &[u8; 26],
                    &mut [libc::c_char; 26],
                >(b"continue\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0"),
                short_name: 'c' as i32 as libc::c_char,
                type_0: OPT_BOOLEAN,
                data: b"continue\0" as *const u8 as *const libc::c_char
                    as *const libc::c_void,
                argtype: -(1 as libc::c_int),
            };
            init
        },
        {
            let mut init = cmdline_option {
                long_name: *::std::mem::transmute::<
                    &[u8; 26],
                    &mut [libc::c_char; 26],
                >(b"convert-file-only\0\0\0\0\0\0\0\0\0"),
                short_name: 0 as libc::c_int as libc::c_char,
                type_0: OPT_BOOLEAN,
                data: b"convertfileonly\0" as *const u8 as *const libc::c_char
                    as *const libc::c_void,
                argtype: -(1 as libc::c_int),
            };
            init
        },
        {
            let mut init = cmdline_option {
                long_name: *::std::mem::transmute::<
                    &[u8; 26],
                    &mut [libc::c_char; 26],
                >(b"convert-links\0\0\0\0\0\0\0\0\0\0\0\0\0"),
                short_name: 'k' as i32 as libc::c_char,
                type_0: OPT_BOOLEAN,
                data: b"convertlinks\0" as *const u8 as *const libc::c_char
                    as *const libc::c_void,
                argtype: -(1 as libc::c_int),
            };
            init
        },
        {
            let mut init = cmdline_option {
                long_name: *::std::mem::transmute::<
                    &[u8; 26],
                    &mut [libc::c_char; 26],
                >(b"content-disposition\0\0\0\0\0\0\0"),
                short_name: 0 as libc::c_int as libc::c_char,
                type_0: OPT_BOOLEAN,
                data: b"contentdisposition\0" as *const u8 as *const libc::c_char
                    as *const libc::c_void,
                argtype: -(1 as libc::c_int),
            };
            init
        },
        {
            let mut init = cmdline_option {
                long_name: *::std::mem::transmute::<
                    &[u8; 26],
                    &mut [libc::c_char; 26],
                >(b"content-on-error\0\0\0\0\0\0\0\0\0\0"),
                short_name: 0 as libc::c_int as libc::c_char,
                type_0: OPT_BOOLEAN,
                data: b"contentonerror\0" as *const u8 as *const libc::c_char
                    as *const libc::c_void,
                argtype: -(1 as libc::c_int),
            };
            init
        },
        {
            let mut init = cmdline_option {
                long_name: *::std::mem::transmute::<
                    &[u8; 26],
                    &mut [libc::c_char; 26],
                >(b"cookies\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0"),
                short_name: 0 as libc::c_int as libc::c_char,
                type_0: OPT_BOOLEAN,
                data: b"cookies\0" as *const u8 as *const libc::c_char
                    as *const libc::c_void,
                argtype: -(1 as libc::c_int),
            };
            init
        },
        {
            let mut init = cmdline_option {
                long_name: *::std::mem::transmute::<
                    &[u8; 26],
                    &mut [libc::c_char; 26],
                >(b"cut-dirs\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0"),
                short_name: 0 as libc::c_int as libc::c_char,
                type_0: OPT_VALUE,
                data: b"cutdirs\0" as *const u8 as *const libc::c_char
                    as *const libc::c_void,
                argtype: -(1 as libc::c_int),
            };
            init
        },
        {
            let mut init = cmdline_option {
                long_name: *::std::mem::transmute::<
                    &[u8; 26],
                    &mut [libc::c_char; 26],
                >(b"debug\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0"),
                short_name: 'd' as i32 as libc::c_char,
                type_0: OPT_BOOLEAN,
                data: b"debug\0" as *const u8 as *const libc::c_char
                    as *const libc::c_void,
                argtype: -(1 as libc::c_int),
            };
            init
        },
        {
            let mut init = cmdline_option {
                long_name: *::std::mem::transmute::<
                    &[u8; 26],
                    &mut [libc::c_char; 26],
                >(b"default-page\0\0\0\0\0\0\0\0\0\0\0\0\0\0"),
                short_name: 0 as libc::c_int as libc::c_char,
                type_0: OPT_VALUE,
                data: b"defaultpage\0" as *const u8 as *const libc::c_char
                    as *const libc::c_void,
                argtype: -(1 as libc::c_int),
            };
            init
        },
        {
            let mut init = cmdline_option {
                long_name: *::std::mem::transmute::<
                    &[u8; 26],
                    &mut [libc::c_char; 26],
                >(b"delete-after\0\0\0\0\0\0\0\0\0\0\0\0\0\0"),
                short_name: 0 as libc::c_int as libc::c_char,
                type_0: OPT_BOOLEAN,
                data: b"deleteafter\0" as *const u8 as *const libc::c_char
                    as *const libc::c_void,
                argtype: -(1 as libc::c_int),
            };
            init
        },
        {
            let mut init = cmdline_option {
                long_name: *::std::mem::transmute::<
                    &[u8; 26],
                    &mut [libc::c_char; 26],
                >(b"directories\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0"),
                short_name: 0 as libc::c_int as libc::c_char,
                type_0: OPT_BOOLEAN,
                data: b"dirstruct\0" as *const u8 as *const libc::c_char
                    as *const libc::c_void,
                argtype: -(1 as libc::c_int),
            };
            init
        },
        {
            let mut init = cmdline_option {
                long_name: *::std::mem::transmute::<
                    &[u8; 26],
                    &mut [libc::c_char; 26],
                >(b"directory-prefix\0\0\0\0\0\0\0\0\0\0"),
                short_name: 'P' as i32 as libc::c_char,
                type_0: OPT_VALUE,
                data: b"dirprefix\0" as *const u8 as *const libc::c_char
                    as *const libc::c_void,
                argtype: -(1 as libc::c_int),
            };
            init
        },
        {
            let mut init = cmdline_option {
                long_name: *::std::mem::transmute::<
                    &[u8; 26],
                    &mut [libc::c_char; 26],
                >(b"dns-cache\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0"),
                short_name: 0 as libc::c_int as libc::c_char,
                type_0: OPT_BOOLEAN,
                data: b"dnscache\0" as *const u8 as *const libc::c_char
                    as *const libc::c_void,
                argtype: -(1 as libc::c_int),
            };
            init
        },
        {
            let mut init = cmdline_option {
                long_name: *::std::mem::transmute::<
                    &[u8; 26],
                    &mut [libc::c_char; 26],
                >(b"dns-timeout\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0"),
                short_name: 0 as libc::c_int as libc::c_char,
                type_0: OPT_VALUE,
                data: b"dnstimeout\0" as *const u8 as *const libc::c_char
                    as *const libc::c_void,
                argtype: -(1 as libc::c_int),
            };
            init
        },
        {
            let mut init = cmdline_option {
                long_name: *::std::mem::transmute::<
                    &[u8; 26],
                    &mut [libc::c_char; 26],
                >(b"domains\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0"),
                short_name: 'D' as i32 as libc::c_char,
                type_0: OPT_VALUE,
                data: b"domains\0" as *const u8 as *const libc::c_char
                    as *const libc::c_void,
                argtype: -(1 as libc::c_int),
            };
            init
        },
        {
            let mut init = cmdline_option {
                long_name: *::std::mem::transmute::<
                    &[u8; 26],
                    &mut [libc::c_char; 26],
                >(b"dont-remove-listing\0\0\0\0\0\0\0"),
                short_name: 0 as libc::c_int as libc::c_char,
                type_0: OPT__DONT_REMOVE_LISTING,
                data: 0 as *const libc::c_void,
                argtype: 0 as libc::c_int,
            };
            init
        },
        {
            let mut init = cmdline_option {
                long_name: *::std::mem::transmute::<
                    &[u8; 26],
                    &mut [libc::c_char; 26],
                >(b"dot-style\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0"),
                short_name: 0 as libc::c_int as libc::c_char,
                type_0: OPT_VALUE,
                data: b"dotstyle\0" as *const u8 as *const libc::c_char
                    as *const libc::c_void,
                argtype: -(1 as libc::c_int),
            };
            init
        },
        {
            let mut init = cmdline_option {
                long_name: *::std::mem::transmute::<
                    &[u8; 26],
                    &mut [libc::c_char; 26],
                >(b"egd-file\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0"),
                short_name: 0 as libc::c_int as libc::c_char,
                type_0: OPT_VALUE,
                data: b"egdfile\0" as *const u8 as *const libc::c_char
                    as *const libc::c_void,
                argtype: -(1 as libc::c_int),
            };
            init
        },
        {
            let mut init = cmdline_option {
                long_name: *::std::mem::transmute::<
                    &[u8; 26],
                    &mut [libc::c_char; 26],
                >(b"exclude-directories\0\0\0\0\0\0\0"),
                short_name: 'X' as i32 as libc::c_char,
                type_0: OPT_VALUE,
                data: b"excludedirectories\0" as *const u8 as *const libc::c_char
                    as *const libc::c_void,
                argtype: -(1 as libc::c_int),
            };
            init
        },
        {
            let mut init = cmdline_option {
                long_name: *::std::mem::transmute::<
                    &[u8; 26],
                    &mut [libc::c_char; 26],
                >(b"exclude-domains\0\0\0\0\0\0\0\0\0\0\0"),
                short_name: 0 as libc::c_int as libc::c_char,
                type_0: OPT_VALUE,
                data: b"excludedomains\0" as *const u8 as *const libc::c_char
                    as *const libc::c_void,
                argtype: -(1 as libc::c_int),
            };
            init
        },
        {
            let mut init = cmdline_option {
                long_name: *::std::mem::transmute::<
                    &[u8; 26],
                    &mut [libc::c_char; 26],
                >(b"execute\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0"),
                short_name: 'e' as i32 as libc::c_char,
                type_0: OPT__EXECUTE,
                data: 0 as *const libc::c_void,
                argtype: 1 as libc::c_int,
            };
            init
        },
        {
            let mut init = cmdline_option {
                long_name: *::std::mem::transmute::<
                    &[u8; 26],
                    &mut [libc::c_char; 26],
                >(b"follow-ftp\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0"),
                short_name: 0 as libc::c_int as libc::c_char,
                type_0: OPT_BOOLEAN,
                data: b"followftp\0" as *const u8 as *const libc::c_char
                    as *const libc::c_void,
                argtype: -(1 as libc::c_int),
            };
            init
        },
        {
            let mut init = cmdline_option {
                long_name: *::std::mem::transmute::<
                    &[u8; 26],
                    &mut [libc::c_char; 26],
                >(b"follow-tags\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0"),
                short_name: 0 as libc::c_int as libc::c_char,
                type_0: OPT_VALUE,
                data: b"followtags\0" as *const u8 as *const libc::c_char
                    as *const libc::c_void,
                argtype: -(1 as libc::c_int),
            };
            init
        },
        {
            let mut init = cmdline_option {
                long_name: *::std::mem::transmute::<
                    &[u8; 26],
                    &mut [libc::c_char; 26],
                >(b"force-directories\0\0\0\0\0\0\0\0\0"),
                short_name: 'x' as i32 as libc::c_char,
                type_0: OPT_BOOLEAN,
                data: b"dirstruct\0" as *const u8 as *const libc::c_char
                    as *const libc::c_void,
                argtype: -(1 as libc::c_int),
            };
            init
        },
        {
            let mut init = cmdline_option {
                long_name: *::std::mem::transmute::<
                    &[u8; 26],
                    &mut [libc::c_char; 26],
                >(b"force-html\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0"),
                short_name: 'F' as i32 as libc::c_char,
                type_0: OPT_BOOLEAN,
                data: b"forcehtml\0" as *const u8 as *const libc::c_char
                    as *const libc::c_void,
                argtype: -(1 as libc::c_int),
            };
            init
        },
        {
            let mut init = cmdline_option {
                long_name: *::std::mem::transmute::<
                    &[u8; 26],
                    &mut [libc::c_char; 26],
                >(b"ftp-password\0\0\0\0\0\0\0\0\0\0\0\0\0\0"),
                short_name: 0 as libc::c_int as libc::c_char,
                type_0: OPT_VALUE,
                data: b"ftppassword\0" as *const u8 as *const libc::c_char
                    as *const libc::c_void,
                argtype: -(1 as libc::c_int),
            };
            init
        },
        {
            let mut init = cmdline_option {
                long_name: *::std::mem::transmute::<
                    &[u8; 26],
                    &mut [libc::c_char; 26],
                >(b"ftp-user\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0"),
                short_name: 0 as libc::c_int as libc::c_char,
                type_0: OPT_VALUE,
                data: b"ftpuser\0" as *const u8 as *const libc::c_char
                    as *const libc::c_void,
                argtype: -(1 as libc::c_int),
            };
            init
        },
        {
            let mut init = cmdline_option {
                long_name: *::std::mem::transmute::<
                    &[u8; 26],
                    &mut [libc::c_char; 26],
                >(b"glob\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0"),
                short_name: 0 as libc::c_int as libc::c_char,
                type_0: OPT_BOOLEAN,
                data: b"glob\0" as *const u8 as *const libc::c_char
                    as *const libc::c_void,
                argtype: -(1 as libc::c_int),
            };
            init
        },
        {
            let mut init = cmdline_option {
                long_name: *::std::mem::transmute::<
                    &[u8; 26],
                    &mut [libc::c_char; 26],
                >(b"header\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0"),
                short_name: 0 as libc::c_int as libc::c_char,
                type_0: OPT_VALUE,
                data: b"header\0" as *const u8 as *const libc::c_char
                    as *const libc::c_void,
                argtype: -(1 as libc::c_int),
            };
            init
        },
        {
            let mut init = cmdline_option {
                long_name: *::std::mem::transmute::<
                    &[u8; 26],
                    &mut [libc::c_char; 26],
                >(b"help\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0"),
                short_name: 'h' as i32 as libc::c_char,
                type_0: OPT_FUNCALL,
                data: ::std::mem::transmute::<
                    Option::<unsafe extern "C" fn() -> ()>,
                    *mut libc::c_void,
                >(Some(print_help as unsafe extern "C" fn() -> ())),
                argtype: 0 as libc::c_int,
            };
            init
        },
        {
            let mut init = cmdline_option {
                long_name: *::std::mem::transmute::<
                    &[u8; 26],
                    &mut [libc::c_char; 26],
                >(b"host-directories\0\0\0\0\0\0\0\0\0\0"),
                short_name: 0 as libc::c_int as libc::c_char,
                type_0: OPT_BOOLEAN,
                data: b"addhostdir\0" as *const u8 as *const libc::c_char
                    as *const libc::c_void,
                argtype: -(1 as libc::c_int),
            };
            init
        },
        {
            let mut init = cmdline_option {
                long_name: *::std::mem::transmute::<
                    &[u8; 26],
                    &mut [libc::c_char; 26],
                >(b"html-extension\0\0\0\0\0\0\0\0\0\0\0\0"),
                short_name: 'E' as i32 as libc::c_char,
                type_0: OPT_BOOLEAN,
                data: b"adjustextension\0" as *const u8 as *const libc::c_char
                    as *const libc::c_void,
                argtype: -(1 as libc::c_int),
            };
            init
        },
        {
            let mut init = cmdline_option {
                long_name: *::std::mem::transmute::<
                    &[u8; 26],
                    &mut [libc::c_char; 26],
                >(b"htmlify\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0"),
                short_name: 0 as libc::c_int as libc::c_char,
                type_0: OPT_BOOLEAN,
                data: b"htmlify\0" as *const u8 as *const libc::c_char
                    as *const libc::c_void,
                argtype: -(1 as libc::c_int),
            };
            init
        },
        {
            let mut init = cmdline_option {
                long_name: *::std::mem::transmute::<
                    &[u8; 26],
                    &mut [libc::c_char; 26],
                >(b"http-keep-alive\0\0\0\0\0\0\0\0\0\0\0"),
                short_name: 0 as libc::c_int as libc::c_char,
                type_0: OPT_BOOLEAN,
                data: b"httpkeepalive\0" as *const u8 as *const libc::c_char
                    as *const libc::c_void,
                argtype: -(1 as libc::c_int),
            };
            init
        },
        {
            let mut init = cmdline_option {
                long_name: *::std::mem::transmute::<
                    &[u8; 26],
                    &mut [libc::c_char; 26],
                >(b"http-passwd\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0"),
                short_name: 0 as libc::c_int as libc::c_char,
                type_0: OPT_VALUE,
                data: b"httppassword\0" as *const u8 as *const libc::c_char
                    as *const libc::c_void,
                argtype: -(1 as libc::c_int),
            };
            init
        },
        {
            let mut init = cmdline_option {
                long_name: *::std::mem::transmute::<
                    &[u8; 26],
                    &mut [libc::c_char; 26],
                >(b"http-password\0\0\0\0\0\0\0\0\0\0\0\0\0"),
                short_name: 0 as libc::c_int as libc::c_char,
                type_0: OPT_VALUE,
                data: b"httppassword\0" as *const u8 as *const libc::c_char
                    as *const libc::c_void,
                argtype: -(1 as libc::c_int),
            };
            init
        },
        {
            let mut init = cmdline_option {
                long_name: *::std::mem::transmute::<
                    &[u8; 26],
                    &mut [libc::c_char; 26],
                >(b"http-user\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0"),
                short_name: 0 as libc::c_int as libc::c_char,
                type_0: OPT_VALUE,
                data: b"httpuser\0" as *const u8 as *const libc::c_char
                    as *const libc::c_void,
                argtype: -(1 as libc::c_int),
            };
            init
        },
        {
            let mut init = cmdline_option {
                long_name: *::std::mem::transmute::<
                    &[u8; 26],
                    &mut [libc::c_char; 26],
                >(b"ignore-case\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0"),
                short_name: 0 as libc::c_int as libc::c_char,
                type_0: OPT_BOOLEAN,
                data: b"ignorecase\0" as *const u8 as *const libc::c_char
                    as *const libc::c_void,
                argtype: -(1 as libc::c_int),
            };
            init
        },
        {
            let mut init = cmdline_option {
                long_name: *::std::mem::transmute::<
                    &[u8; 26],
                    &mut [libc::c_char; 26],
                >(b"ignore-length\0\0\0\0\0\0\0\0\0\0\0\0\0"),
                short_name: 0 as libc::c_int as libc::c_char,
                type_0: OPT_BOOLEAN,
                data: b"ignorelength\0" as *const u8 as *const libc::c_char
                    as *const libc::c_void,
                argtype: -(1 as libc::c_int),
            };
            init
        },
        {
            let mut init = cmdline_option {
                long_name: *::std::mem::transmute::<
                    &[u8; 26],
                    &mut [libc::c_char; 26],
                >(b"ignore-tags\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0"),
                short_name: 0 as libc::c_int as libc::c_char,
                type_0: OPT_VALUE,
                data: b"ignoretags\0" as *const u8 as *const libc::c_char
                    as *const libc::c_void,
                argtype: -(1 as libc::c_int),
            };
            init
        },
        {
            let mut init = cmdline_option {
                long_name: *::std::mem::transmute::<
                    &[u8; 26],
                    &mut [libc::c_char; 26],
                >(b"include-directories\0\0\0\0\0\0\0"),
                short_name: 'I' as i32 as libc::c_char,
                type_0: OPT_VALUE,
                data: b"includedirectories\0" as *const u8 as *const libc::c_char
                    as *const libc::c_void,
                argtype: -(1 as libc::c_int),
            };
            init
        },
        {
            let mut init = cmdline_option {
                long_name: *::std::mem::transmute::<
                    &[u8; 26],
                    &mut [libc::c_char; 26],
                >(b"inet4-only\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0"),
                short_name: '4' as i32 as libc::c_char,
                type_0: OPT_BOOLEAN,
                data: b"inet4only\0" as *const u8 as *const libc::c_char
                    as *const libc::c_void,
                argtype: -(1 as libc::c_int),
            };
            init
        },
        {
            let mut init = cmdline_option {
                long_name: *::std::mem::transmute::<
                    &[u8; 26],
                    &mut [libc::c_char; 26],
                >(b"inet6-only\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0"),
                short_name: '6' as i32 as libc::c_char,
                type_0: OPT_BOOLEAN,
                data: b"inet6only\0" as *const u8 as *const libc::c_char
                    as *const libc::c_void,
                argtype: -(1 as libc::c_int),
            };
            init
        },
        {
            let mut init = cmdline_option {
                long_name: *::std::mem::transmute::<
                    &[u8; 26],
                    &mut [libc::c_char; 26],
                >(b"input-file\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0"),
                short_name: 'i' as i32 as libc::c_char,
                type_0: OPT_VALUE,
                data: b"input\0" as *const u8 as *const libc::c_char
                    as *const libc::c_void,
                argtype: -(1 as libc::c_int),
            };
            init
        },
        {
            let mut init = cmdline_option {
                long_name: *::std::mem::transmute::<
                    &[u8; 26],
                    &mut [libc::c_char; 26],
                >(b"iri\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0"),
                short_name: 0 as libc::c_int as libc::c_char,
                type_0: OPT_BOOLEAN,
                data: b"iri\0" as *const u8 as *const libc::c_char
                    as *const libc::c_void,
                argtype: -(1 as libc::c_int),
            };
            init
        },
        {
            let mut init = cmdline_option {
                long_name: *::std::mem::transmute::<
                    &[u8; 26],
                    &mut [libc::c_char; 26],
                >(b"keep-badhash\0\0\0\0\0\0\0\0\0\0\0\0\0\0"),
                short_name: 0 as libc::c_int as libc::c_char,
                type_0: OPT_BOOLEAN,
                data: b"keepbadhash\0" as *const u8 as *const libc::c_char
                    as *const libc::c_void,
                argtype: -(1 as libc::c_int),
            };
            init
        },
        {
            let mut init = cmdline_option {
                long_name: *::std::mem::transmute::<
                    &[u8; 26],
                    &mut [libc::c_char; 26],
                >(b"keep-session-cookies\0\0\0\0\0\0"),
                short_name: 0 as libc::c_int as libc::c_char,
                type_0: OPT_BOOLEAN,
                data: b"keepsessioncookies\0" as *const u8 as *const libc::c_char
                    as *const libc::c_void,
                argtype: -(1 as libc::c_int),
            };
            init
        },
        {
            let mut init = cmdline_option {
                long_name: *::std::mem::transmute::<
                    &[u8; 26],
                    &mut [libc::c_char; 26],
                >(b"level\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0"),
                short_name: 'l' as i32 as libc::c_char,
                type_0: OPT_VALUE,
                data: b"reclevel\0" as *const u8 as *const libc::c_char
                    as *const libc::c_void,
                argtype: -(1 as libc::c_int),
            };
            init
        },
        {
            let mut init = cmdline_option {
                long_name: *::std::mem::transmute::<
                    &[u8; 26],
                    &mut [libc::c_char; 26],
                >(b"limit-rate\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0"),
                short_name: 0 as libc::c_int as libc::c_char,
                type_0: OPT_VALUE,
                data: b"limitrate\0" as *const u8 as *const libc::c_char
                    as *const libc::c_void,
                argtype: -(1 as libc::c_int),
            };
            init
        },
        {
            let mut init = cmdline_option {
                long_name: *::std::mem::transmute::<
                    &[u8; 26],
                    &mut [libc::c_char; 26],
                >(b"load-cookies\0\0\0\0\0\0\0\0\0\0\0\0\0\0"),
                short_name: 0 as libc::c_int as libc::c_char,
                type_0: OPT_VALUE,
                data: b"loadcookies\0" as *const u8 as *const libc::c_char
                    as *const libc::c_void,
                argtype: -(1 as libc::c_int),
            };
            init
        },
        {
            let mut init = cmdline_option {
                long_name: *::std::mem::transmute::<
                    &[u8; 26],
                    &mut [libc::c_char; 26],
                >(b"local-encoding\0\0\0\0\0\0\0\0\0\0\0\0"),
                short_name: 0 as libc::c_int as libc::c_char,
                type_0: OPT_VALUE,
                data: b"localencoding\0" as *const u8 as *const libc::c_char
                    as *const libc::c_void,
                argtype: -(1 as libc::c_int),
            };
            init
        },
        {
            let mut init = cmdline_option {
                long_name: *::std::mem::transmute::<
                    &[u8; 26],
                    &mut [libc::c_char; 26],
                >(b"rejected-log\0\0\0\0\0\0\0\0\0\0\0\0\0\0"),
                short_name: 0 as libc::c_int as libc::c_char,
                type_0: OPT_VALUE,
                data: b"rejectedlog\0" as *const u8 as *const libc::c_char
                    as *const libc::c_void,
                argtype: -(1 as libc::c_int),
            };
            init
        },
        {
            let mut init = cmdline_option {
                long_name: *::std::mem::transmute::<
                    &[u8; 26],
                    &mut [libc::c_char; 26],
                >(b"max-redirect\0\0\0\0\0\0\0\0\0\0\0\0\0\0"),
                short_name: 0 as libc::c_int as libc::c_char,
                type_0: OPT_VALUE,
                data: b"maxredirect\0" as *const u8 as *const libc::c_char
                    as *const libc::c_void,
                argtype: -(1 as libc::c_int),
            };
            init
        },
        {
            let mut init = cmdline_option {
                long_name: *::std::mem::transmute::<
                    &[u8; 26],
                    &mut [libc::c_char; 26],
                >(b"method\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0"),
                short_name: 0 as libc::c_int as libc::c_char,
                type_0: OPT_VALUE,
                data: b"method\0" as *const u8 as *const libc::c_char
                    as *const libc::c_void,
                argtype: -(1 as libc::c_int),
            };
            init
        },
        {
            let mut init = cmdline_option {
                long_name: *::std::mem::transmute::<
                    &[u8; 26],
                    &mut [libc::c_char; 26],
                >(b"mirror\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0"),
                short_name: 'm' as i32 as libc::c_char,
                type_0: OPT_BOOLEAN,
                data: b"mirror\0" as *const u8 as *const libc::c_char
                    as *const libc::c_void,
                argtype: -(1 as libc::c_int),
            };
            init
        },
        {
            let mut init = cmdline_option {
                long_name: *::std::mem::transmute::<
                    &[u8; 26],
                    &mut [libc::c_char; 26],
                >(b"netrc\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0"),
                short_name: 0 as libc::c_int as libc::c_char,
                type_0: OPT_BOOLEAN,
                data: b"netrc\0" as *const u8 as *const libc::c_char
                    as *const libc::c_void,
                argtype: -(1 as libc::c_int),
            };
            init
        },
        {
            let mut init = cmdline_option {
                long_name: *::std::mem::transmute::<
                    &[u8; 26],
                    &mut [libc::c_char; 26],
                >(b"no\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0"),
                short_name: 'n' as i32 as libc::c_char,
                type_0: OPT__NO,
                data: 0 as *const libc::c_void,
                argtype: 1 as libc::c_int,
            };
            init
        },
        {
            let mut init = cmdline_option {
                long_name: *::std::mem::transmute::<
                    &[u8; 26],
                    &mut [libc::c_char; 26],
                >(b"no-clobber\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0"),
                short_name: 0 as libc::c_int as libc::c_char,
                type_0: OPT_BOOLEAN,
                data: b"noclobber\0" as *const u8 as *const libc::c_char
                    as *const libc::c_void,
                argtype: -(1 as libc::c_int),
            };
            init
        },
        {
            let mut init = cmdline_option {
                long_name: *::std::mem::transmute::<
                    &[u8; 26],
                    &mut [libc::c_char; 26],
                >(b"no-config\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0"),
                short_name: 0 as libc::c_int as libc::c_char,
                type_0: OPT_BOOLEAN,
                data: b"noconfig\0" as *const u8 as *const libc::c_char
                    as *const libc::c_void,
                argtype: -(1 as libc::c_int),
            };
            init
        },
        {
            let mut init = cmdline_option {
                long_name: *::std::mem::transmute::<
                    &[u8; 26],
                    &mut [libc::c_char; 26],
                >(b"no-parent\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0"),
                short_name: 0 as libc::c_int as libc::c_char,
                type_0: OPT_BOOLEAN,
                data: b"noparent\0" as *const u8 as *const libc::c_char
                    as *const libc::c_void,
                argtype: -(1 as libc::c_int),
            };
            init
        },
        {
            let mut init = cmdline_option {
                long_name: *::std::mem::transmute::<
                    &[u8; 26],
                    &mut [libc::c_char; 26],
                >(b"output-document\0\0\0\0\0\0\0\0\0\0\0"),
                short_name: 'O' as i32 as libc::c_char,
                type_0: OPT_VALUE,
                data: b"outputdocument\0" as *const u8 as *const libc::c_char
                    as *const libc::c_void,
                argtype: -(1 as libc::c_int),
            };
            init
        },
        {
            let mut init = cmdline_option {
                long_name: *::std::mem::transmute::<
                    &[u8; 26],
                    &mut [libc::c_char; 26],
                >(b"output-file\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0"),
                short_name: 'o' as i32 as libc::c_char,
                type_0: OPT_VALUE,
                data: b"logfile\0" as *const u8 as *const libc::c_char
                    as *const libc::c_void,
                argtype: -(1 as libc::c_int),
            };
            init
        },
        {
            let mut init = cmdline_option {
                long_name: *::std::mem::transmute::<
                    &[u8; 26],
                    &mut [libc::c_char; 26],
                >(b"page-requisites\0\0\0\0\0\0\0\0\0\0\0"),
                short_name: 'p' as i32 as libc::c_char,
                type_0: OPT_BOOLEAN,
                data: b"pagerequisites\0" as *const u8 as *const libc::c_char
                    as *const libc::c_void,
                argtype: -(1 as libc::c_int),
            };
            init
        },
        {
            let mut init = cmdline_option {
                long_name: *::std::mem::transmute::<
                    &[u8; 26],
                    &mut [libc::c_char; 26],
                >(b"parent\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0"),
                short_name: 0 as libc::c_int as libc::c_char,
                type_0: OPT__PARENT,
                data: 0 as *const libc::c_void,
                argtype: 2 as libc::c_int,
            };
            init
        },
        {
            let mut init = cmdline_option {
                long_name: *::std::mem::transmute::<
                    &[u8; 26],
                    &mut [libc::c_char; 26],
                >(b"passive-ftp\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0"),
                short_name: 0 as libc::c_int as libc::c_char,
                type_0: OPT_BOOLEAN,
                data: b"passiveftp\0" as *const u8 as *const libc::c_char
                    as *const libc::c_void,
                argtype: -(1 as libc::c_int),
            };
            init
        },
        {
            let mut init = cmdline_option {
                long_name: *::std::mem::transmute::<
                    &[u8; 26],
                    &mut [libc::c_char; 26],
                >(b"password\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0"),
                short_name: 0 as libc::c_int as libc::c_char,
                type_0: OPT_VALUE,
                data: b"password\0" as *const u8 as *const libc::c_char
                    as *const libc::c_void,
                argtype: -(1 as libc::c_int),
            };
            init
        },
        {
            let mut init = cmdline_option {
                long_name: *::std::mem::transmute::<
                    &[u8; 26],
                    &mut [libc::c_char; 26],
                >(b"post-data\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0"),
                short_name: 0 as libc::c_int as libc::c_char,
                type_0: OPT_VALUE,
                data: b"postdata\0" as *const u8 as *const libc::c_char
                    as *const libc::c_void,
                argtype: -(1 as libc::c_int),
            };
            init
        },
        {
            let mut init = cmdline_option {
                long_name: *::std::mem::transmute::<
                    &[u8; 26],
                    &mut [libc::c_char; 26],
                >(b"post-file\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0"),
                short_name: 0 as libc::c_int as libc::c_char,
                type_0: OPT_VALUE,
                data: b"postfile\0" as *const u8 as *const libc::c_char
                    as *const libc::c_void,
                argtype: -(1 as libc::c_int),
            };
            init
        },
        {
            let mut init = cmdline_option {
                long_name: *::std::mem::transmute::<
                    &[u8; 26],
                    &mut [libc::c_char; 26],
                >(b"prefer-family\0\0\0\0\0\0\0\0\0\0\0\0\0"),
                short_name: 0 as libc::c_int as libc::c_char,
                type_0: OPT_VALUE,
                data: b"preferfamily\0" as *const u8 as *const libc::c_char
                    as *const libc::c_void,
                argtype: -(1 as libc::c_int),
            };
            init
        },
        {
            let mut init = cmdline_option {
                long_name: *::std::mem::transmute::<
                    &[u8; 26],
                    &mut [libc::c_char; 26],
                >(b"preserve-permissions\0\0\0\0\0\0"),
                short_name: 0 as libc::c_int as libc::c_char,
                type_0: OPT_BOOLEAN,
                data: b"preservepermissions\0" as *const u8 as *const libc::c_char
                    as *const libc::c_void,
                argtype: -(1 as libc::c_int),
            };
            init
        },
        {
            let mut init = cmdline_option {
                long_name: *::std::mem::transmute::<
                    &[u8; 26],
                    &mut [libc::c_char; 26],
                >(b"progress\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0"),
                short_name: 0 as libc::c_int as libc::c_char,
                type_0: OPT_VALUE,
                data: b"progress\0" as *const u8 as *const libc::c_char
                    as *const libc::c_void,
                argtype: -(1 as libc::c_int),
            };
            init
        },
        {
            let mut init = cmdline_option {
                long_name: *::std::mem::transmute::<
                    &[u8; 26],
                    &mut [libc::c_char; 26],
                >(b"show-progress\0\0\0\0\0\0\0\0\0\0\0\0\0"),
                short_name: 0 as libc::c_int as libc::c_char,
                type_0: OPT_BOOLEAN,
                data: b"showprogress\0" as *const u8 as *const libc::c_char
                    as *const libc::c_void,
                argtype: -(1 as libc::c_int),
            };
            init
        },
        {
            let mut init = cmdline_option {
                long_name: *::std::mem::transmute::<
                    &[u8; 26],
                    &mut [libc::c_char; 26],
                >(b"protocol-directories\0\0\0\0\0\0"),
                short_name: 0 as libc::c_int as libc::c_char,
                type_0: OPT_BOOLEAN,
                data: b"protocoldirectories\0" as *const u8 as *const libc::c_char
                    as *const libc::c_void,
                argtype: -(1 as libc::c_int),
            };
            init
        },
        {
            let mut init = cmdline_option {
                long_name: *::std::mem::transmute::<
                    &[u8; 26],
                    &mut [libc::c_char; 26],
                >(b"proxy\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0"),
                short_name: 0 as libc::c_int as libc::c_char,
                type_0: OPT_BOOLEAN,
                data: b"useproxy\0" as *const u8 as *const libc::c_char
                    as *const libc::c_void,
                argtype: -(1 as libc::c_int),
            };
            init
        },
        {
            let mut init = cmdline_option {
                long_name: *::std::mem::transmute::<
                    &[u8; 26],
                    &mut [libc::c_char; 26],
                >(b"proxy__compat\0\0\0\0\0\0\0\0\0\0\0\0\0"),
                short_name: 'Y' as i32 as libc::c_char,
                type_0: OPT_VALUE,
                data: b"useproxy\0" as *const u8 as *const libc::c_char
                    as *const libc::c_void,
                argtype: -(1 as libc::c_int),
            };
            init
        },
        {
            let mut init = cmdline_option {
                long_name: *::std::mem::transmute::<
                    &[u8; 26],
                    &mut [libc::c_char; 26],
                >(b"proxy-passwd\0\0\0\0\0\0\0\0\0\0\0\0\0\0"),
                short_name: 0 as libc::c_int as libc::c_char,
                type_0: OPT_VALUE,
                data: b"proxypassword\0" as *const u8 as *const libc::c_char
                    as *const libc::c_void,
                argtype: -(1 as libc::c_int),
            };
            init
        },
        {
            let mut init = cmdline_option {
                long_name: *::std::mem::transmute::<
                    &[u8; 26],
                    &mut [libc::c_char; 26],
                >(b"proxy-password\0\0\0\0\0\0\0\0\0\0\0\0"),
                short_name: 0 as libc::c_int as libc::c_char,
                type_0: OPT_VALUE,
                data: b"proxypassword\0" as *const u8 as *const libc::c_char
                    as *const libc::c_void,
                argtype: -(1 as libc::c_int),
            };
            init
        },
        {
            let mut init = cmdline_option {
                long_name: *::std::mem::transmute::<
                    &[u8; 26],
                    &mut [libc::c_char; 26],
                >(b"proxy-user\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0"),
                short_name: 0 as libc::c_int as libc::c_char,
                type_0: OPT_VALUE,
                data: b"proxyuser\0" as *const u8 as *const libc::c_char
                    as *const libc::c_void,
                argtype: -(1 as libc::c_int),
            };
            init
        },
        {
            let mut init = cmdline_option {
                long_name: *::std::mem::transmute::<
                    &[u8; 26],
                    &mut [libc::c_char; 26],
                >(b"quiet\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0"),
                short_name: 'q' as i32 as libc::c_char,
                type_0: OPT_BOOLEAN,
                data: b"quiet\0" as *const u8 as *const libc::c_char
                    as *const libc::c_void,
                argtype: -(1 as libc::c_int),
            };
            init
        },
        {
            let mut init = cmdline_option {
                long_name: *::std::mem::transmute::<
                    &[u8; 26],
                    &mut [libc::c_char; 26],
                >(b"quota\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0"),
                short_name: 'Q' as i32 as libc::c_char,
                type_0: OPT_VALUE,
                data: b"quota\0" as *const u8 as *const libc::c_char
                    as *const libc::c_void,
                argtype: -(1 as libc::c_int),
            };
            init
        },
        {
            let mut init = cmdline_option {
                long_name: *::std::mem::transmute::<
                    &[u8; 26],
                    &mut [libc::c_char; 26],
                >(b"random-file\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0"),
                short_name: 0 as libc::c_int as libc::c_char,
                type_0: OPT_VALUE,
                data: b"randomfile\0" as *const u8 as *const libc::c_char
                    as *const libc::c_void,
                argtype: -(1 as libc::c_int),
            };
            init
        },
        {
            let mut init = cmdline_option {
                long_name: *::std::mem::transmute::<
                    &[u8; 26],
                    &mut [libc::c_char; 26],
                >(b"random-wait\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0"),
                short_name: 0 as libc::c_int as libc::c_char,
                type_0: OPT_BOOLEAN,
                data: b"randomwait\0" as *const u8 as *const libc::c_char
                    as *const libc::c_void,
                argtype: -(1 as libc::c_int),
            };
            init
        },
        {
            let mut init = cmdline_option {
                long_name: *::std::mem::transmute::<
                    &[u8; 26],
                    &mut [libc::c_char; 26],
                >(b"read-timeout\0\0\0\0\0\0\0\0\0\0\0\0\0\0"),
                short_name: 0 as libc::c_int as libc::c_char,
                type_0: OPT_VALUE,
                data: b"readtimeout\0" as *const u8 as *const libc::c_char
                    as *const libc::c_void,
                argtype: -(1 as libc::c_int),
            };
            init
        },
        {
            let mut init = cmdline_option {
                long_name: *::std::mem::transmute::<
                    &[u8; 26],
                    &mut [libc::c_char; 26],
                >(b"recursive\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0"),
                short_name: 'r' as i32 as libc::c_char,
                type_0: OPT_BOOLEAN,
                data: b"recursive\0" as *const u8 as *const libc::c_char
                    as *const libc::c_void,
                argtype: -(1 as libc::c_int),
            };
            init
        },
        {
            let mut init = cmdline_option {
                long_name: *::std::mem::transmute::<
                    &[u8; 26],
                    &mut [libc::c_char; 26],
                >(b"referer\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0"),
                short_name: 0 as libc::c_int as libc::c_char,
                type_0: OPT_VALUE,
                data: b"referer\0" as *const u8 as *const libc::c_char
                    as *const libc::c_void,
                argtype: -(1 as libc::c_int),
            };
            init
        },
        {
            let mut init = cmdline_option {
                long_name: *::std::mem::transmute::<
                    &[u8; 26],
                    &mut [libc::c_char; 26],
                >(b"regex-type\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0"),
                short_name: 0 as libc::c_int as libc::c_char,
                type_0: OPT_VALUE,
                data: b"regextype\0" as *const u8 as *const libc::c_char
                    as *const libc::c_void,
                argtype: -(1 as libc::c_int),
            };
            init
        },
        {
            let mut init = cmdline_option {
                long_name: *::std::mem::transmute::<
                    &[u8; 26],
                    &mut [libc::c_char; 26],
                >(b"reject\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0"),
                short_name: 'R' as i32 as libc::c_char,
                type_0: OPT_VALUE,
                data: b"reject\0" as *const u8 as *const libc::c_char
                    as *const libc::c_void,
                argtype: -(1 as libc::c_int),
            };
            init
        },
        {
            let mut init = cmdline_option {
                long_name: *::std::mem::transmute::<
                    &[u8; 26],
                    &mut [libc::c_char; 26],
                >(b"reject-regex\0\0\0\0\0\0\0\0\0\0\0\0\0\0"),
                short_name: 0 as libc::c_int as libc::c_char,
                type_0: OPT_VALUE,
                data: b"rejectregex\0" as *const u8 as *const libc::c_char
                    as *const libc::c_void,
                argtype: -(1 as libc::c_int),
            };
            init
        },
        {
            let mut init = cmdline_option {
                long_name: *::std::mem::transmute::<
                    &[u8; 26],
                    &mut [libc::c_char; 26],
                >(b"relative\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0"),
                short_name: 'L' as i32 as libc::c_char,
                type_0: OPT_BOOLEAN,
                data: b"relativeonly\0" as *const u8 as *const libc::c_char
                    as *const libc::c_void,
                argtype: -(1 as libc::c_int),
            };
            init
        },
        {
            let mut init = cmdline_option {
                long_name: *::std::mem::transmute::<
                    &[u8; 26],
                    &mut [libc::c_char; 26],
                >(b"remote-encoding\0\0\0\0\0\0\0\0\0\0\0"),
                short_name: 0 as libc::c_int as libc::c_char,
                type_0: OPT_VALUE,
                data: b"remoteencoding\0" as *const u8 as *const libc::c_char
                    as *const libc::c_void,
                argtype: -(1 as libc::c_int),
            };
            init
        },
        {
            let mut init = cmdline_option {
                long_name: *::std::mem::transmute::<
                    &[u8; 26],
                    &mut [libc::c_char; 26],
                >(b"remove-listing\0\0\0\0\0\0\0\0\0\0\0\0"),
                short_name: 0 as libc::c_int as libc::c_char,
                type_0: OPT_BOOLEAN,
                data: b"removelisting\0" as *const u8 as *const libc::c_char
                    as *const libc::c_void,
                argtype: -(1 as libc::c_int),
            };
            init
        },
        {
            let mut init = cmdline_option {
                long_name: *::std::mem::transmute::<
                    &[u8; 26],
                    &mut [libc::c_char; 26],
                >(b"report-speed\0\0\0\0\0\0\0\0\0\0\0\0\0\0"),
                short_name: 0 as libc::c_int as libc::c_char,
                type_0: OPT_BOOLEAN,
                data: b"reportspeed\0" as *const u8 as *const libc::c_char
                    as *const libc::c_void,
                argtype: -(1 as libc::c_int),
            };
            init
        },
        {
            let mut init = cmdline_option {
                long_name: *::std::mem::transmute::<
                    &[u8; 26],
                    &mut [libc::c_char; 26],
                >(b"restrict-file-names\0\0\0\0\0\0\0"),
                short_name: 0 as libc::c_int as libc::c_char,
                type_0: OPT_BOOLEAN,
                data: b"restrictfilenames\0" as *const u8 as *const libc::c_char
                    as *const libc::c_void,
                argtype: -(1 as libc::c_int),
            };
            init
        },
        {
            let mut init = cmdline_option {
                long_name: *::std::mem::transmute::<
                    &[u8; 26],
                    &mut [libc::c_char; 26],
                >(b"retr-symlinks\0\0\0\0\0\0\0\0\0\0\0\0\0"),
                short_name: 0 as libc::c_int as libc::c_char,
                type_0: OPT_BOOLEAN,
                data: b"retrsymlinks\0" as *const u8 as *const libc::c_char
                    as *const libc::c_void,
                argtype: -(1 as libc::c_int),
            };
            init
        },
        {
            let mut init = cmdline_option {
                long_name: *::std::mem::transmute::<
                    &[u8; 26],
                    &mut [libc::c_char; 26],
                >(b"retry-connrefused\0\0\0\0\0\0\0\0\0"),
                short_name: 0 as libc::c_int as libc::c_char,
                type_0: OPT_BOOLEAN,
                data: b"retryconnrefused\0" as *const u8 as *const libc::c_char
                    as *const libc::c_void,
                argtype: -(1 as libc::c_int),
            };
            init
        },
        {
            let mut init = cmdline_option {
                long_name: *::std::mem::transmute::<
                    &[u8; 26],
                    &mut [libc::c_char; 26],
                >(b"retry-on-host-error\0\0\0\0\0\0\0"),
                short_name: 0 as libc::c_int as libc::c_char,
                type_0: OPT_BOOLEAN,
                data: b"retryonhosterror\0" as *const u8 as *const libc::c_char
                    as *const libc::c_void,
                argtype: -(1 as libc::c_int),
            };
            init
        },
        {
            let mut init = cmdline_option {
                long_name: *::std::mem::transmute::<
                    &[u8; 26],
                    &mut [libc::c_char; 26],
                >(b"retry-on-http-error\0\0\0\0\0\0\0"),
                short_name: 0 as libc::c_int as libc::c_char,
                type_0: OPT_VALUE,
                data: b"retryonhttperror\0" as *const u8 as *const libc::c_char
                    as *const libc::c_void,
                argtype: -(1 as libc::c_int),
            };
            init
        },
        {
            let mut init = cmdline_option {
                long_name: *::std::mem::transmute::<
                    &[u8; 26],
                    &mut [libc::c_char; 26],
                >(b"save-cookies\0\0\0\0\0\0\0\0\0\0\0\0\0\0"),
                short_name: 0 as libc::c_int as libc::c_char,
                type_0: OPT_VALUE,
                data: b"savecookies\0" as *const u8 as *const libc::c_char
                    as *const libc::c_void,
                argtype: -(1 as libc::c_int),
            };
            init
        },
        {
            let mut init = cmdline_option {
                long_name: *::std::mem::transmute::<
                    &[u8; 26],
                    &mut [libc::c_char; 26],
                >(b"save-headers\0\0\0\0\0\0\0\0\0\0\0\0\0\0"),
                short_name: 0 as libc::c_int as libc::c_char,
                type_0: OPT_BOOLEAN,
                data: b"saveheaders\0" as *const u8 as *const libc::c_char
                    as *const libc::c_void,
                argtype: -(1 as libc::c_int),
            };
            init
        },
        {
            let mut init = cmdline_option {
                long_name: *::std::mem::transmute::<
                    &[u8; 26],
                    &mut [libc::c_char; 26],
                >(b"server-response\0\0\0\0\0\0\0\0\0\0\0"),
                short_name: 'S' as i32 as libc::c_char,
                type_0: OPT_BOOLEAN,
                data: b"serverresponse\0" as *const u8 as *const libc::c_char
                    as *const libc::c_void,
                argtype: -(1 as libc::c_int),
            };
            init
        },
        {
            let mut init = cmdline_option {
                long_name: *::std::mem::transmute::<
                    &[u8; 26],
                    &mut [libc::c_char; 26],
                >(b"span-hosts\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0"),
                short_name: 'H' as i32 as libc::c_char,
                type_0: OPT_BOOLEAN,
                data: b"spanhosts\0" as *const u8 as *const libc::c_char
                    as *const libc::c_void,
                argtype: -(1 as libc::c_int),
            };
            init
        },
        {
            let mut init = cmdline_option {
                long_name: *::std::mem::transmute::<
                    &[u8; 26],
                    &mut [libc::c_char; 26],
                >(b"spider\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0"),
                short_name: 0 as libc::c_int as libc::c_char,
                type_0: OPT_BOOLEAN,
                data: b"spider\0" as *const u8 as *const libc::c_char
                    as *const libc::c_void,
                argtype: -(1 as libc::c_int),
            };
            init
        },
        {
            let mut init = cmdline_option {
                long_name: *::std::mem::transmute::<
                    &[u8; 26],
                    &mut [libc::c_char; 26],
                >(b"start-pos\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0"),
                short_name: 0 as libc::c_int as libc::c_char,
                type_0: OPT_VALUE,
                data: b"startpos\0" as *const u8 as *const libc::c_char
                    as *const libc::c_void,
                argtype: -(1 as libc::c_int),
            };
            init
        },
        {
            let mut init = cmdline_option {
                long_name: *::std::mem::transmute::<
                    &[u8; 26],
                    &mut [libc::c_char; 26],
                >(b"strict-comments\0\0\0\0\0\0\0\0\0\0\0"),
                short_name: 0 as libc::c_int as libc::c_char,
                type_0: OPT_BOOLEAN,
                data: b"strictcomments\0" as *const u8 as *const libc::c_char
                    as *const libc::c_void,
                argtype: -(1 as libc::c_int),
            };
            init
        },
        {
            let mut init = cmdline_option {
                long_name: *::std::mem::transmute::<
                    &[u8; 26],
                    &mut [libc::c_char; 26],
                >(b"timeout\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0"),
                short_name: 'T' as i32 as libc::c_char,
                type_0: OPT_VALUE,
                data: b"timeout\0" as *const u8 as *const libc::c_char
                    as *const libc::c_void,
                argtype: -(1 as libc::c_int),
            };
            init
        },
        {
            let mut init = cmdline_option {
                long_name: *::std::mem::transmute::<
                    &[u8; 26],
                    &mut [libc::c_char; 26],
                >(b"timestamping\0\0\0\0\0\0\0\0\0\0\0\0\0\0"),
                short_name: 'N' as i32 as libc::c_char,
                type_0: OPT_BOOLEAN,
                data: b"timestamping\0" as *const u8 as *const libc::c_char
                    as *const libc::c_void,
                argtype: -(1 as libc::c_int),
            };
            init
        },
        {
            let mut init = cmdline_option {
                long_name: *::std::mem::transmute::<
                    &[u8; 26],
                    &mut [libc::c_char; 26],
                >(b"if-modified-since\0\0\0\0\0\0\0\0\0"),
                short_name: 0 as libc::c_int as libc::c_char,
                type_0: OPT_BOOLEAN,
                data: b"ifmodifiedsince\0" as *const u8 as *const libc::c_char
                    as *const libc::c_void,
                argtype: -(1 as libc::c_int),
            };
            init
        },
        {
            let mut init = cmdline_option {
                long_name: *::std::mem::transmute::<
                    &[u8; 26],
                    &mut [libc::c_char; 26],
                >(b"tries\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0"),
                short_name: 't' as i32 as libc::c_char,
                type_0: OPT_VALUE,
                data: b"tries\0" as *const u8 as *const libc::c_char
                    as *const libc::c_void,
                argtype: -(1 as libc::c_int),
            };
            init
        },
        {
            let mut init = cmdline_option {
                long_name: *::std::mem::transmute::<
                    &[u8; 26],
                    &mut [libc::c_char; 26],
                >(b"unlink\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0"),
                short_name: 0 as libc::c_int as libc::c_char,
                type_0: OPT_BOOLEAN,
                data: b"unlink\0" as *const u8 as *const libc::c_char
                    as *const libc::c_void,
                argtype: -(1 as libc::c_int),
            };
            init
        },
        {
            let mut init = cmdline_option {
                long_name: *::std::mem::transmute::<
                    &[u8; 26],
                    &mut [libc::c_char; 26],
                >(b"trust-server-names\0\0\0\0\0\0\0\0"),
                short_name: 0 as libc::c_int as libc::c_char,
                type_0: OPT_BOOLEAN,
                data: b"trustservernames\0" as *const u8 as *const libc::c_char
                    as *const libc::c_void,
                argtype: -(1 as libc::c_int),
            };
            init
        },
        {
            let mut init = cmdline_option {
                long_name: *::std::mem::transmute::<
                    &[u8; 26],
                    &mut [libc::c_char; 26],
                >(b"use-askpass\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0"),
                short_name: 0 as libc::c_int as libc::c_char,
                type_0: OPT_VALUE,
                data: b"useaskpass\0" as *const u8 as *const libc::c_char
                    as *const libc::c_void,
                argtype: -(1 as libc::c_int),
            };
            init
        },
        {
            let mut init = cmdline_option {
                long_name: *::std::mem::transmute::<
                    &[u8; 26],
                    &mut [libc::c_char; 26],
                >(b"use-server-timestamps\0\0\0\0\0"),
                short_name: 0 as libc::c_int as libc::c_char,
                type_0: OPT_BOOLEAN,
                data: b"useservertimestamps\0" as *const u8 as *const libc::c_char
                    as *const libc::c_void,
                argtype: -(1 as libc::c_int),
            };
            init
        },
        {
            let mut init = cmdline_option {
                long_name: *::std::mem::transmute::<
                    &[u8; 26],
                    &mut [libc::c_char; 26],
                >(b"user\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0"),
                short_name: 0 as libc::c_int as libc::c_char,
                type_0: OPT_VALUE,
                data: b"user\0" as *const u8 as *const libc::c_char
                    as *const libc::c_void,
                argtype: -(1 as libc::c_int),
            };
            init
        },
        {
            let mut init = cmdline_option {
                long_name: *::std::mem::transmute::<
                    &[u8; 26],
                    &mut [libc::c_char; 26],
                >(b"user-agent\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0"),
                short_name: 'U' as i32 as libc::c_char,
                type_0: OPT_VALUE,
                data: b"useragent\0" as *const u8 as *const libc::c_char
                    as *const libc::c_void,
                argtype: -(1 as libc::c_int),
            };
            init
        },
        {
            let mut init = cmdline_option {
                long_name: *::std::mem::transmute::<
                    &[u8; 26],
                    &mut [libc::c_char; 26],
                >(b"verbose\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0"),
                short_name: 'v' as i32 as libc::c_char,
                type_0: OPT_BOOLEAN,
                data: b"verbose\0" as *const u8 as *const libc::c_char
                    as *const libc::c_void,
                argtype: -(1 as libc::c_int),
            };
            init
        },
        {
            let mut init = cmdline_option {
                long_name: *::std::mem::transmute::<
                    &[u8; 26],
                    &mut [libc::c_char; 26],
                >(b"version\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0"),
                short_name: 'V' as i32 as libc::c_char,
                type_0: OPT_FUNCALL,
                data: ::std::mem::transmute::<
                    Option::<unsafe extern "C" fn() -> ()>,
                    *mut libc::c_void,
                >(Some(print_version as unsafe extern "C" fn() -> ())),
                argtype: 0 as libc::c_int,
            };
            init
        },
        {
            let mut init = cmdline_option {
                long_name: *::std::mem::transmute::<
                    &[u8; 26],
                    &mut [libc::c_char; 26],
                >(b"wait\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0"),
                short_name: 'w' as i32 as libc::c_char,
                type_0: OPT_VALUE,
                data: b"wait\0" as *const u8 as *const libc::c_char
                    as *const libc::c_void,
                argtype: -(1 as libc::c_int),
            };
            init
        },
        {
            let mut init = cmdline_option {
                long_name: *::std::mem::transmute::<
                    &[u8; 26],
                    &mut [libc::c_char; 26],
                >(b"waitretry\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0"),
                short_name: 0 as libc::c_int as libc::c_char,
                type_0: OPT_VALUE,
                data: b"waitretry\0" as *const u8 as *const libc::c_char
                    as *const libc::c_void,
                argtype: -(1 as libc::c_int),
            };
            init
        },
        {
            let mut init = cmdline_option {
                long_name: *::std::mem::transmute::<
                    &[u8; 26],
                    &mut [libc::c_char; 26],
                >(b"warc-cdx\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0"),
                short_name: 0 as libc::c_int as libc::c_char,
                type_0: OPT_BOOLEAN,
                data: b"warccdx\0" as *const u8 as *const libc::c_char
                    as *const libc::c_void,
                argtype: -(1 as libc::c_int),
            };
            init
        },
        {
            let mut init = cmdline_option {
                long_name: *::std::mem::transmute::<
                    &[u8; 26],
                    &mut [libc::c_char; 26],
                >(b"warc-compression\0\0\0\0\0\0\0\0\0\0"),
                short_name: 0 as libc::c_int as libc::c_char,
                type_0: OPT_BOOLEAN,
                data: b"warccompression\0" as *const u8 as *const libc::c_char
                    as *const libc::c_void,
                argtype: -(1 as libc::c_int),
            };
            init
        },
        {
            let mut init = cmdline_option {
                long_name: *::std::mem::transmute::<
                    &[u8; 26],
                    &mut [libc::c_char; 26],
                >(b"warc-dedup\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0"),
                short_name: 0 as libc::c_int as libc::c_char,
                type_0: OPT_VALUE,
                data: b"warccdxdedup\0" as *const u8 as *const libc::c_char
                    as *const libc::c_void,
                argtype: -(1 as libc::c_int),
            };
            init
        },
        {
            let mut init = cmdline_option {
                long_name: *::std::mem::transmute::<
                    &[u8; 26],
                    &mut [libc::c_char; 26],
                >(b"warc-digests\0\0\0\0\0\0\0\0\0\0\0\0\0\0"),
                short_name: 0 as libc::c_int as libc::c_char,
                type_0: OPT_BOOLEAN,
                data: b"warcdigests\0" as *const u8 as *const libc::c_char
                    as *const libc::c_void,
                argtype: -(1 as libc::c_int),
            };
            init
        },
        {
            let mut init = cmdline_option {
                long_name: *::std::mem::transmute::<
                    &[u8; 26],
                    &mut [libc::c_char; 26],
                >(b"warc-file\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0"),
                short_name: 0 as libc::c_int as libc::c_char,
                type_0: OPT_VALUE,
                data: b"warcfile\0" as *const u8 as *const libc::c_char
                    as *const libc::c_void,
                argtype: -(1 as libc::c_int),
            };
            init
        },
        {
            let mut init = cmdline_option {
                long_name: *::std::mem::transmute::<
                    &[u8; 26],
                    &mut [libc::c_char; 26],
                >(b"warc-header\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0"),
                short_name: 0 as libc::c_int as libc::c_char,
                type_0: OPT_VALUE,
                data: b"warcheader\0" as *const u8 as *const libc::c_char
                    as *const libc::c_void,
                argtype: -(1 as libc::c_int),
            };
            init
        },
        {
            let mut init = cmdline_option {
                long_name: *::std::mem::transmute::<
                    &[u8; 26],
                    &mut [libc::c_char; 26],
                >(b"warc-keep-log\0\0\0\0\0\0\0\0\0\0\0\0\0"),
                short_name: 0 as libc::c_int as libc::c_char,
                type_0: OPT_BOOLEAN,
                data: b"warckeeplog\0" as *const u8 as *const libc::c_char
                    as *const libc::c_void,
                argtype: -(1 as libc::c_int),
            };
            init
        },
        {
            let mut init = cmdline_option {
                long_name: *::std::mem::transmute::<
                    &[u8; 26],
                    &mut [libc::c_char; 26],
                >(b"warc-max-size\0\0\0\0\0\0\0\0\0\0\0\0\0"),
                short_name: 0 as libc::c_int as libc::c_char,
                type_0: OPT_VALUE,
                data: b"warcmaxsize\0" as *const u8 as *const libc::c_char
                    as *const libc::c_void,
                argtype: -(1 as libc::c_int),
            };
            init
        },
        {
            let mut init = cmdline_option {
                long_name: *::std::mem::transmute::<
                    &[u8; 26],
                    &mut [libc::c_char; 26],
                >(b"warc-tempdir\0\0\0\0\0\0\0\0\0\0\0\0\0\0"),
                short_name: 0 as libc::c_int as libc::c_char,
                type_0: OPT_VALUE,
                data: b"warctempdir\0" as *const u8 as *const libc::c_char
                    as *const libc::c_void,
                argtype: -(1 as libc::c_int),
            };
            init
        },
        {
            let mut init = cmdline_option {
                long_name: *::std::mem::transmute::<
                    &[u8; 26],
                    &mut [libc::c_char; 26],
                >(b"xattr\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0"),
                short_name: 0 as libc::c_int as libc::c_char,
                type_0: OPT_BOOLEAN,
                data: b"xattr\0" as *const u8 as *const libc::c_char
                    as *const libc::c_void,
                argtype: -(1 as libc::c_int),
            };
            init
        },
    ]
};
unsafe extern "C" fn no_prefix(mut s: *const libc::c_char) -> *mut libc::c_char {
    static mut buffer: [libc::c_char; 2048] = [0; 2048];
    static mut p: *mut libc::c_char = unsafe { buffer.as_ptr() as *mut _ };
    let mut cp: *mut libc::c_char = p;
    let mut size: libc::c_int = (3 as libc::c_int as libc::c_ulong)
        .wrapping_add(strlen(s))
        .wrapping_add(1 as libc::c_int as libc::c_ulong) as libc::c_int;
    *cp.offset(0 as libc::c_int as isize) = 'n' as i32 as libc::c_char;
    *cp.offset(1 as libc::c_int as isize) = 'o' as i32 as libc::c_char;
    *cp.offset(2 as libc::c_int as isize) = '-' as i32 as libc::c_char;
    strcpy(cp.offset(3 as libc::c_int as isize), s);
    p = p.offset(size as isize);
    return cp;
}
static mut long_options: [option; 293] = [option {
    name: 0 as *const libc::c_char,
    has_arg: 0,
    flag: 0 as *const libc::c_int as *mut libc::c_int,
    val: 0,
}; 293];
static mut short_options: [libc::c_char; 128] = [0; 128];
static mut optmap: [libc::c_uchar; 96] = [0; 96];
unsafe extern "C" fn init_switches() {
    static mut initialized: bool = false;
    let mut p: *mut libc::c_char = short_options.as_mut_ptr();
    let mut i: size_t = 0;
    let mut o: size_t = 0 as libc::c_int as size_t;
    if initialized {
        return;
    }
    initialized = 1 as libc::c_int != 0;
    i = 0 as libc::c_int as size_t;
    while i
        < (::std::mem::size_of::<[cmdline_option; 146]>() as libc::c_ulong)
            .wrapping_div(::std::mem::size_of::<cmdline_option>() as libc::c_ulong)
    {
        let mut cmdopt: *mut cmdline_option = &mut *option_data
            .as_mut_ptr()
            .offset(i as isize) as *mut cmdline_option;
        let mut longopt: *mut option = 0 as *mut option;
        let fresh0 = o;
        o = o.wrapping_add(1);
        longopt = &mut *long_options.as_mut_ptr().offset(fresh0 as isize) as *mut option;
        (*longopt).name = ((*cmdopt).long_name).as_mut_ptr();
        (*longopt).val = i as libc::c_int;
        if (*cmdopt).short_name != 0 {
            let fresh1 = p;
            p = p.offset(1);
            *fresh1 = (*cmdopt).short_name;
            optmap[((*cmdopt).short_name as libc::c_int - 32 as libc::c_int)
                as usize] = longopt.offset_from(long_options.as_mut_ptr())
                as libc::c_long as libc::c_uchar;
        }
        match (*cmdopt).type_0 as libc::c_uint {
            0 => {
                (*longopt).has_arg = 1 as libc::c_int;
                if (*cmdopt).short_name != 0 {
                    let fresh2 = p;
                    p = p.offset(1);
                    *fresh2 = ':' as i32 as libc::c_char;
                }
            }
            1 => {
                (*longopt).has_arg = 2 as libc::c_int;
                let fresh3 = o;
                o = o.wrapping_add(1);
                longopt = &mut *long_options.as_mut_ptr().offset(fresh3 as isize)
                    as *mut option;
                (*longopt).name = no_prefix(((*cmdopt).long_name).as_mut_ptr());
                (*longopt).has_arg = 0 as libc::c_int;
                (*longopt)
                    .val = (i | 1024 as libc::c_int as libc::c_ulong) as libc::c_int;
            }
            _ => {
                (*longopt).has_arg = (*cmdopt).argtype;
                if (*cmdopt).short_name != 0 {
                    if (*longopt).has_arg == 1 as libc::c_int {
                        let fresh4 = p;
                        p = p.offset(1);
                        *fresh4 = ':' as i32 as libc::c_char;
                    }
                }
            }
        }
        i = i.wrapping_add(1);
        i;
    }
    *p = '\0' as i32 as libc::c_char;
}
unsafe extern "C" fn print_usage(mut error: libc::c_int) -> libc::c_int {
    return fprintf(
        if error != 0 { stderr } else { stdout },
        dcgettext(
            0 as *const libc::c_char,
            b"Usage: %s [OPTION]... [URL]...\n\0" as *const u8 as *const libc::c_char,
            5 as libc::c_int,
        ),
        exec_name,
    );
}
unsafe extern "C" fn print_help() {
    static mut help: [*const libc::c_char; 155] = [
        b"\n\0" as *const u8 as *const libc::c_char,
        b"Mandatory arguments to long options are mandatory for short options too.\n\n\0"
            as *const u8 as *const libc::c_char,
        b"Startup:\n\0" as *const u8 as *const libc::c_char,
        b"  -V,  --version                   display the version of Wget and exit\n\0"
            as *const u8 as *const libc::c_char,
        b"  -h,  --help                      print this help\n\0" as *const u8
            as *const libc::c_char,
        b"  -b,  --background                go to background after startup\n\0"
            as *const u8 as *const libc::c_char,
        b"  -e,  --execute=COMMAND           execute a `.wgetrc'-style command\n\0"
            as *const u8 as *const libc::c_char,
        b"\n\0" as *const u8 as *const libc::c_char,
        b"Logging and input file:\n\0" as *const u8 as *const libc::c_char,
        b"  -o,  --output-file=FILE          log messages to FILE\n\0" as *const u8
            as *const libc::c_char,
        b"  -a,  --append-output=FILE        append messages to FILE\n\0" as *const u8
            as *const libc::c_char,
        b"  -d,  --debug                     print lots of debugging information\n\0"
            as *const u8 as *const libc::c_char,
        b"  -q,  --quiet                     quiet (no output)\n\0" as *const u8
            as *const libc::c_char,
        b"  -v,  --verbose                   be verbose (this is the default)\n\0"
            as *const u8 as *const libc::c_char,
        b"  -nv, --no-verbose                turn off verboseness, without being quiet\n\0"
            as *const u8 as *const libc::c_char,
        b"       --report-speed=TYPE         output bandwidth as TYPE.  TYPE can be bits\n\0"
            as *const u8 as *const libc::c_char,
        b"  -i,  --input-file=FILE           download URLs found in local or external FILE\n\0"
            as *const u8 as *const libc::c_char,
        b"  -F,  --force-html                treat input file as HTML\n\0" as *const u8
            as *const libc::c_char,
        b"  -B,  --base=URL                  resolves HTML input-file links (-i -F)\n                                     relative to URL\n\0"
            as *const u8 as *const libc::c_char,
        b"       --config=FILE               specify config file to use\n\0" as *const u8
            as *const libc::c_char,
        b"       --no-config                 do not read any config file\n\0"
            as *const u8 as *const libc::c_char,
        b"       --rejected-log=FILE         log reasons for URL rejection to FILE\n\0"
            as *const u8 as *const libc::c_char,
        b"\n\0" as *const u8 as *const libc::c_char,
        b"Download:\n\0" as *const u8 as *const libc::c_char,
        b"  -t,  --tries=NUMBER              set number of retries to NUMBER (0 unlimits)\n\0"
            as *const u8 as *const libc::c_char,
        b"       --retry-connrefused         retry even if connection is refused\n\0"
            as *const u8 as *const libc::c_char,
        b"       --retry-on-host-error       consider host errors as non-fatal, transient errors\n\0"
            as *const u8 as *const libc::c_char,
        b"       --retry-on-http-error=ERRORS    comma-separated list of HTTP errors to retry\n\0"
            as *const u8 as *const libc::c_char,
        b"  -O,  --output-document=FILE      write documents to FILE\n\0" as *const u8
            as *const libc::c_char,
        b"  -nc, --no-clobber                skip downloads that would download to\n                                     existing files (overwriting them)\n\0"
            as *const u8 as *const libc::c_char,
        b"       --no-netrc                  don't try to obtain credentials from .netrc\n\0"
            as *const u8 as *const libc::c_char,
        b"  -c,  --continue                  resume getting a partially-downloaded file\n\0"
            as *const u8 as *const libc::c_char,
        b"       --start-pos=OFFSET          start downloading from zero-based position OFFSET\n\0"
            as *const u8 as *const libc::c_char,
        b"       --progress=TYPE             select progress gauge type\n\0" as *const u8
            as *const libc::c_char,
        b"       --show-progress             display the progress bar in any verbosity mode\n\0"
            as *const u8 as *const libc::c_char,
        b"  -N,  --timestamping              don't re-retrieve files unless newer than\n                                     local\n\0"
            as *const u8 as *const libc::c_char,
        b"       --no-if-modified-since      don't use conditional if-modified-since get\n                                     requests in timestamping mode\n\0"
            as *const u8 as *const libc::c_char,
        b"       --no-use-server-timestamps  don't set the local file's timestamp by\n                                     the one on the server\n\0"
            as *const u8 as *const libc::c_char,
        b"  -S,  --server-response           print server response\n\0" as *const u8
            as *const libc::c_char,
        b"       --spider                    don't download anything\n\0" as *const u8
            as *const libc::c_char,
        b"  -T,  --timeout=SECONDS           set all timeout values to SECONDS\n\0"
            as *const u8 as *const libc::c_char,
        b"       --dns-timeout=SECS          set the DNS lookup timeout to SECS\n\0"
            as *const u8 as *const libc::c_char,
        b"       --connect-timeout=SECS      set the connect timeout to SECS\n\0"
            as *const u8 as *const libc::c_char,
        b"       --read-timeout=SECS         set the read timeout to SECS\n\0"
            as *const u8 as *const libc::c_char,
        b"  -w,  --wait=SECONDS              wait SECONDS between retrievals\n                                     (applies if more then 1 URL is to be retrieved)\n\0"
            as *const u8 as *const libc::c_char,
        b"       --waitretry=SECONDS         wait 1..SECONDS between retries of a retrieval\n                                     (applies if more then 1 URL is to be retrieved)\n\0"
            as *const u8 as *const libc::c_char,
        b"       --random-wait               wait from 0.5*WAIT...1.5*WAIT secs between retrievals\n                                     (applies if more then 1 URL is to be retrieved)\n\0"
            as *const u8 as *const libc::c_char,
        b"       --no-proxy                  explicitly turn off proxy\n\0" as *const u8
            as *const libc::c_char,
        b"  -Q,  --quota=NUMBER              set retrieval quota to NUMBER\n\0"
            as *const u8 as *const libc::c_char,
        b"       --bind-address=ADDRESS      bind to ADDRESS (hostname or IP) on local host\n\0"
            as *const u8 as *const libc::c_char,
        b"       --limit-rate=RATE           limit download rate to RATE\n\0"
            as *const u8 as *const libc::c_char,
        b"       --no-dns-cache              disable caching DNS lookups\n\0"
            as *const u8 as *const libc::c_char,
        b"       --restrict-file-names=OS    restrict chars in file names to ones OS allows\n\0"
            as *const u8 as *const libc::c_char,
        b"       --ignore-case               ignore case when matching files/directories\n\0"
            as *const u8 as *const libc::c_char,
        b"  -4,  --inet4-only                connect only to IPv4 addresses\n\0"
            as *const u8 as *const libc::c_char,
        b"  -6,  --inet6-only                connect only to IPv6 addresses\n\0"
            as *const u8 as *const libc::c_char,
        b"       --prefer-family=FAMILY      connect first to addresses of specified family,\n                                     one of IPv6, IPv4, or none\n\0"
            as *const u8 as *const libc::c_char,
        b"       --user=USER                 set both ftp and http user to USER\n\0"
            as *const u8 as *const libc::c_char,
        b"       --password=PASS             set both ftp and http password to PASS\n\0"
            as *const u8 as *const libc::c_char,
        b"       --ask-password              prompt for passwords\n\0" as *const u8
            as *const libc::c_char,
        b"       --use-askpass=COMMAND       specify credential handler for requesting \n                                     username and password.  If no COMMAND is \n                                     specified the WGET_ASKPASS or the SSH_ASKPASS \n                                     environment variable is used.\n\0"
            as *const u8 as *const libc::c_char,
        b"       --no-iri                    turn off IRI support\n\0" as *const u8
            as *const libc::c_char,
        b"       --local-encoding=ENC        use ENC as the local encoding for IRIs\n\0"
            as *const u8 as *const libc::c_char,
        b"       --remote-encoding=ENC       use ENC as the default remote encoding\n\0"
            as *const u8 as *const libc::c_char,
        b"       --unlink                    remove file before clobber\n\0" as *const u8
            as *const libc::c_char,
        b"       --xattr                     turn on storage of metadata in extended file attributes\n\0"
            as *const u8 as *const libc::c_char,
        b"\n\0" as *const u8 as *const libc::c_char,
        b"Directories:\n\0" as *const u8 as *const libc::c_char,
        b"  -nd, --no-directories            don't create directories\n\0" as *const u8
            as *const libc::c_char,
        b"  -x,  --force-directories         force creation of directories\n\0"
            as *const u8 as *const libc::c_char,
        b"  -nH, --no-host-directories       don't create host directories\n\0"
            as *const u8 as *const libc::c_char,
        b"       --protocol-directories      use protocol name in directories\n\0"
            as *const u8 as *const libc::c_char,
        b"  -P,  --directory-prefix=PREFIX   save files to PREFIX/..\n\0" as *const u8
            as *const libc::c_char,
        b"       --cut-dirs=NUMBER           ignore NUMBER remote directory components\n\0"
            as *const u8 as *const libc::c_char,
        b"\n\0" as *const u8 as *const libc::c_char,
        b"HTTP options:\n\0" as *const u8 as *const libc::c_char,
        b"       --http-user=USER            set http user to USER\n\0" as *const u8
            as *const libc::c_char,
        b"       --http-password=PASS        set http password to PASS\n\0" as *const u8
            as *const libc::c_char,
        b"       --no-cache                  disallow server-cached data\n\0"
            as *const u8 as *const libc::c_char,
        b"       --default-page=NAME         change the default page name (normally\n                                     this is 'index.html'.)\n\0"
            as *const u8 as *const libc::c_char,
        b"  -E,  --adjust-extension          save HTML/CSS documents with proper extensions\n\0"
            as *const u8 as *const libc::c_char,
        b"       --ignore-length             ignore 'Content-Length' header field\n\0"
            as *const u8 as *const libc::c_char,
        b"       --header=STRING             insert STRING among the headers\n\0"
            as *const u8 as *const libc::c_char,
        b"       --compression=TYPE          choose compression, one of auto, gzip and none. (default: none)\n\0"
            as *const u8 as *const libc::c_char,
        b"       --max-redirect              maximum redirections allowed per page\n\0"
            as *const u8 as *const libc::c_char,
        b"       --proxy-user=USER           set USER as proxy username\n\0" as *const u8
            as *const libc::c_char,
        b"       --proxy-password=PASS       set PASS as proxy password\n\0" as *const u8
            as *const libc::c_char,
        b"       --referer=URL               include 'Referer: URL' header in HTTP request\n\0"
            as *const u8 as *const libc::c_char,
        b"       --save-headers              save the HTTP headers to file\n\0"
            as *const u8 as *const libc::c_char,
        b"  -U,  --user-agent=AGENT          identify as AGENT instead of Wget/VERSION\n\0"
            as *const u8 as *const libc::c_char,
        b"       --no-http-keep-alive        disable HTTP keep-alive (persistent connections)\n\0"
            as *const u8 as *const libc::c_char,
        b"       --no-cookies                don't use cookies\n\0" as *const u8
            as *const libc::c_char,
        b"       --load-cookies=FILE         load cookies from FILE before session\n\0"
            as *const u8 as *const libc::c_char,
        b"       --save-cookies=FILE         save cookies to FILE after session\n\0"
            as *const u8 as *const libc::c_char,
        b"       --keep-session-cookies      load and save session (non-permanent) cookies\n\0"
            as *const u8 as *const libc::c_char,
        b"       --post-data=STRING          use the POST method; send STRING as the data\n\0"
            as *const u8 as *const libc::c_char,
        b"       --post-file=FILE            use the POST method; send contents of FILE\n\0"
            as *const u8 as *const libc::c_char,
        b"       --method=HTTPMethod         use method \"HTTPMethod\" in the request\n\0"
            as *const u8 as *const libc::c_char,
        b"       --body-data=STRING          send STRING as data. --method MUST be set\n\0"
            as *const u8 as *const libc::c_char,
        b"       --body-file=FILE            send contents of FILE. --method MUST be set\n\0"
            as *const u8 as *const libc::c_char,
        b"       --content-disposition       honor the Content-Disposition header when\n                                     choosing local file names (EXPERIMENTAL)\n\0"
            as *const u8 as *const libc::c_char,
        b"       --content-on-error          output the received content on server errors\n\0"
            as *const u8 as *const libc::c_char,
        b"       --auth-no-challenge         send Basic HTTP authentication information\n                                     without first waiting for the server's\n                                     challenge\n\0"
            as *const u8 as *const libc::c_char,
        b"\n\0" as *const u8 as *const libc::c_char,
        b"FTP options:\n\0" as *const u8 as *const libc::c_char,
        b"       --ftp-user=USER             set ftp user to USER\n\0" as *const u8
            as *const libc::c_char,
        b"       --ftp-password=PASS         set ftp password to PASS\n\0" as *const u8
            as *const libc::c_char,
        b"       --no-remove-listing         don't remove '.listing' files\n\0"
            as *const u8 as *const libc::c_char,
        b"       --no-glob                   turn off FTP file name globbing\n\0"
            as *const u8 as *const libc::c_char,
        b"       --no-passive-ftp            disable the \"passive\" transfer mode\n\0"
            as *const u8 as *const libc::c_char,
        b"       --preserve-permissions      preserve remote file permissions\n\0"
            as *const u8 as *const libc::c_char,
        b"       --retr-symlinks             when recursing, get linked-to files (not dir)\n\0"
            as *const u8 as *const libc::c_char,
        b"\n\0" as *const u8 as *const libc::c_char,
        b"WARC options:\n\0" as *const u8 as *const libc::c_char,
        b"       --warc-file=FILENAME        save request/response data to a .warc.gz file\n\0"
            as *const u8 as *const libc::c_char,
        b"       --warc-header=STRING        insert STRING into the warcinfo record\n\0"
            as *const u8 as *const libc::c_char,
        b"       --warc-max-size=NUMBER      set maximum size of WARC files to NUMBER\n\0"
            as *const u8 as *const libc::c_char,
        b"       --warc-cdx                  write CDX index files\n\0" as *const u8
            as *const libc::c_char,
        b"       --warc-dedup=FILENAME       do not store records listed in this CDX file\n\0"
            as *const u8 as *const libc::c_char,
        b"       --no-warc-compression       do not compress WARC files with GZIP\n\0"
            as *const u8 as *const libc::c_char,
        b"       --no-warc-digests           do not calculate SHA1 digests\n\0"
            as *const u8 as *const libc::c_char,
        b"       --no-warc-keep-log          do not store the log file in a WARC record\n\0"
            as *const u8 as *const libc::c_char,
        b"       --warc-tempdir=DIRECTORY    location for temporary files created by the\n                                     WARC writer\n\0"
            as *const u8 as *const libc::c_char,
        b"\n\0" as *const u8 as *const libc::c_char,
        b"Recursive download:\n\0" as *const u8 as *const libc::c_char,
        b"  -r,  --recursive                 specify recursive download\n\0" as *const u8
            as *const libc::c_char,
        b"  -l,  --level=NUMBER              maximum recursion depth (inf or 0 for infinite)\n\0"
            as *const u8 as *const libc::c_char,
        b"       --delete-after              delete files locally after downloading them\n\0"
            as *const u8 as *const libc::c_char,
        b"  -k,  --convert-links             make links in downloaded HTML or CSS point to\n                                     local files\n\0"
            as *const u8 as *const libc::c_char,
        b"       --convert-file-only         convert the file part of the URLs only (usually known as the basename)\n\0"
            as *const u8 as *const libc::c_char,
        b"       --backups=N                 before writing file X, rotate up to N backup files\n\0"
            as *const u8 as *const libc::c_char,
        b"  -K,  --backup-converted          before converting file X, back up as X.orig\n\0"
            as *const u8 as *const libc::c_char,
        b"  -m,  --mirror                    shortcut for -N -r -l inf --no-remove-listing\n\0"
            as *const u8 as *const libc::c_char,
        b"  -p,  --page-requisites           get all images, etc. needed to display HTML page\n\0"
            as *const u8 as *const libc::c_char,
        b"       --strict-comments           turn on strict (SGML) handling of HTML comments\n\0"
            as *const u8 as *const libc::c_char,
        b"\n\0" as *const u8 as *const libc::c_char,
        b"Recursive accept/reject:\n\0" as *const u8 as *const libc::c_char,
        b"  -A,  --accept=LIST               comma-separated list of accepted extensions\n\0"
            as *const u8 as *const libc::c_char,
        b"  -R,  --reject=LIST               comma-separated list of rejected extensions\n\0"
            as *const u8 as *const libc::c_char,
        b"       --accept-regex=REGEX        regex matching accepted URLs\n\0"
            as *const u8 as *const libc::c_char,
        b"       --reject-regex=REGEX        regex matching rejected URLs\n\0"
            as *const u8 as *const libc::c_char,
        b"       --regex-type=TYPE           regex type (posix|pcre)\n\0" as *const u8
            as *const libc::c_char,
        b"  -D,  --domains=LIST              comma-separated list of accepted domains\n\0"
            as *const u8 as *const libc::c_char,
        b"       --exclude-domains=LIST      comma-separated list of rejected domains\n\0"
            as *const u8 as *const libc::c_char,
        b"       --follow-ftp                follow FTP links from HTML documents\n\0"
            as *const u8 as *const libc::c_char,
        b"       --follow-tags=LIST          comma-separated list of followed HTML tags\n\0"
            as *const u8 as *const libc::c_char,
        b"       --ignore-tags=LIST          comma-separated list of ignored HTML tags\n\0"
            as *const u8 as *const libc::c_char,
        b"  -H,  --span-hosts                go to foreign hosts when recursive\n\0"
            as *const u8 as *const libc::c_char,
        b"  -L,  --relative                  follow relative links only\n\0" as *const u8
            as *const libc::c_char,
        b"  -I,  --include-directories=LIST  list of allowed directories\n\0"
            as *const u8 as *const libc::c_char,
        b"       --trust-server-names        use the name specified by the redirection\n                                     URL's last component\n\0"
            as *const u8 as *const libc::c_char,
        b"  -X,  --exclude-directories=LIST  list of excluded directories\n\0"
            as *const u8 as *const libc::c_char,
        b"  -np, --no-parent                 don't ascend to the parent directory\n\0"
            as *const u8 as *const libc::c_char,
        b"\n\0" as *const u8 as *const libc::c_char,
        b"Email bug reports, questions, discussions to <bug-wget@gnu.org>\nand/or open issues at https://savannah.gnu.org/bugs/?func=additem&group=wget.\n\0"
            as *const u8 as *const libc::c_char,
    ];
    let mut i: size_t = 0;
    if printf(
        dcgettext(
            0 as *const libc::c_char,
            b"GNU Wget %s, a non-interactive network retriever.\n\0" as *const u8
                as *const libc::c_char,
            5 as libc::c_int,
        ),
        version_string,
    ) < 0 as libc::c_int
    {
        exit(WGET_EXIT_IO_FAIL as libc::c_int);
    }
    if print_usage(0 as libc::c_int) < 0 as libc::c_int {
        exit(WGET_EXIT_IO_FAIL as libc::c_int);
    }
    i = 0 as libc::c_int as size_t;
    while i
        < (::std::mem::size_of::<[*const libc::c_char; 155]>() as libc::c_ulong)
            .wrapping_div(::std::mem::size_of::<*const libc::c_char>() as libc::c_ulong)
    {
        if fputs(
            dcgettext(0 as *const libc::c_char, help[i as usize], 5 as libc::c_int),
            stdout,
        ) < 0 as libc::c_int
        {
            exit(WGET_EXIT_IO_FAIL as libc::c_int);
        }
        i = i.wrapping_add(1);
        i;
    }
    exit(WGET_EXIT_SUCCESS as libc::c_int);
}
unsafe extern "C" fn secs_to_human_time(
    mut interval: libc::c_double,
) -> *mut libc::c_char {
    static mut buf: [libc::c_char; 32] = [0; 32];
    let mut secs: libc::c_int = (interval + 0.5f64) as libc::c_int;
    let mut hours: libc::c_int = 0;
    let mut mins: libc::c_int = 0;
    let mut days: libc::c_int = 0;
    days = secs / 86400 as libc::c_int;
    secs %= 86400 as libc::c_int;
    hours = secs / 3600 as libc::c_int;
    secs %= 3600 as libc::c_int;
    mins = secs / 60 as libc::c_int;
    secs %= 60 as libc::c_int;
    if days != 0 {
        snprintf(
            buf.as_mut_ptr(),
            ::std::mem::size_of::<[libc::c_char; 32]>() as libc::c_ulong,
            b"%dd %dh %dm %ds\0" as *const u8 as *const libc::c_char,
            days,
            hours,
            mins,
            secs,
        );
    } else if hours != 0 {
        snprintf(
            buf.as_mut_ptr(),
            ::std::mem::size_of::<[libc::c_char; 32]>() as libc::c_ulong,
            b"%dh %dm %ds\0" as *const u8 as *const libc::c_char,
            hours,
            mins,
            secs,
        );
    } else if mins != 0 {
        snprintf(
            buf.as_mut_ptr(),
            ::std::mem::size_of::<[libc::c_char; 32]>() as libc::c_ulong,
            b"%dm %ds\0" as *const u8 as *const libc::c_char,
            mins,
            secs,
        );
    } else {
        snprintf(
            buf.as_mut_ptr(),
            ::std::mem::size_of::<[libc::c_char; 32]>() as libc::c_ulong,
            b"%ss\0" as *const u8 as *const libc::c_char,
            print_decimal(interval),
        );
    }
    return buf.as_mut_ptr();
}
unsafe extern "C" fn prompt_for_password() -> *mut libc::c_char {
    if !(opt.user).is_null() {
        fprintf(
            stderr,
            dcgettext(
                0 as *const libc::c_char,
                b"Password for user %s: \0" as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ),
            quote(opt.user),
        );
    } else {
        fprintf(
            stderr,
            dcgettext(
                0 as *const libc::c_char,
                b"Password: \0" as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ),
        );
    }
    return getpass(b"\0" as *const u8 as *const libc::c_char);
}
unsafe extern "C" fn run_use_askpass(
    mut question: *mut libc::c_char,
    mut answer: *mut *mut libc::c_char,
) {
    let mut tmp: [libc::c_char; 1024] = [0; 1024];
    let mut pid: pid_t = 0;
    let mut status: libc::c_int = 0;
    let mut com: [libc::c_int; 2] = [0; 2];
    let mut bytes: ssize_t = 0 as libc::c_int as ssize_t;
    let mut argv: [*mut libc::c_char; 3] = [0 as *mut libc::c_char; 3];
    let mut p: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut fa: posix_spawn_file_actions_t = posix_spawn_file_actions_t {
        __allocated: 0,
        __used: 0,
        __actions: 0 as *mut __spawn_action,
        __pad: [0; 16],
    };
    if pipe(com.as_mut_ptr()) == -(1 as libc::c_int) {
        fprintf(
            stderr,
            dcgettext(
                0 as *const libc::c_char,
                b"Cannot create pipe\n\0" as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ),
        );
        exit(WGET_EXIT_GENERIC_ERROR as libc::c_int);
    }
    status = posix_spawn_file_actions_init(&mut fa);
    if status != 0 {
        fprintf(
            stderr,
            dcgettext(
                0 as *const libc::c_char,
                b"Error initializing spawn file actions for use-askpass: %d\n\0"
                    as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ),
            status,
        );
        exit(WGET_EXIT_GENERIC_ERROR as libc::c_int);
    }
    status = posix_spawn_file_actions_adddup2(
        &mut fa,
        com[1 as libc::c_int as usize],
        1 as libc::c_int,
    );
    if status != 0 {
        fprintf(
            stderr,
            dcgettext(
                0 as *const libc::c_char,
                b"Error setting spawn file actions for use-askpass: %d\n\0" as *const u8
                    as *const libc::c_char,
                5 as libc::c_int,
            ),
            status,
        );
        exit(WGET_EXIT_GENERIC_ERROR as libc::c_int);
    }
    argv[0 as libc::c_int as usize] = opt.use_askpass;
    argv[1 as libc::c_int as usize] = question;
    argv[2 as libc::c_int as usize] = 0 as *mut libc::c_char;
    status = posix_spawnp(
        &mut pid,
        opt.use_askpass,
        &mut fa,
        0 as *const posix_spawnattr_t,
        argv.as_mut_ptr() as *const *mut libc::c_char,
        environ as *const *mut libc::c_char,
    );
    if status != 0 {
        fprintf(
            stderr,
            b"Error spawning %s: %d\n\0" as *const u8 as *const libc::c_char,
            opt.use_askpass,
            status,
        );
        exit(WGET_EXIT_GENERIC_ERROR as libc::c_int);
    }
    close(com[1 as libc::c_int as usize]);
    bytes = read(
        com[0 as libc::c_int as usize],
        tmp.as_mut_ptr() as *mut libc::c_void,
        (::std::mem::size_of::<[libc::c_char; 1024]>() as libc::c_ulong)
            .wrapping_sub(1 as libc::c_int as libc::c_ulong),
    );
    if bytes <= 0 as libc::c_int as libc::c_long {
        fprintf(
            stderr,
            dcgettext(
                0 as *const libc::c_char,
                b"Error reading response from command \"%s %s\": %s\n\0" as *const u8
                    as *const libc::c_char,
                5 as libc::c_int,
            ),
            opt.use_askpass,
            question,
            strerror(*__errno_location()),
        );
        exit(WGET_EXIT_GENERIC_ERROR as libc::c_int);
    }
    tmp[bytes as usize] = '\0' as i32 as libc::c_char;
    p = strpbrk(tmp.as_mut_ptr(), b"\r\n\0" as *const u8 as *const libc::c_char);
    if !p.is_null() {
        bytes = p.offset_from(tmp.as_mut_ptr()) as libc::c_long;
    }
    *answer = xmemdup0(tmp.as_mut_ptr() as *const libc::c_void, bytes as size_t);
}
unsafe extern "C" fn use_askpass(mut u: *mut url) {
    static mut question: [libc::c_char; 1024] = [0; 1024];
    if ((*u).user).is_null()
        || *((*u).user).offset(0 as libc::c_int as isize) as libc::c_int == '\0' as i32
    {
        snprintf(
            question.as_mut_ptr(),
            ::std::mem::size_of::<[libc::c_char; 1024]>() as libc::c_ulong,
            dcgettext(
                0 as *const libc::c_char,
                b"Username for '%s%s': \0" as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ),
            scheme_leading_string((*u).scheme),
            (*u).host,
        );
        run_use_askpass(question.as_mut_ptr(), &mut (*u).user);
        if opt.recursive {
            opt.user = xstrdup((*u).user);
        }
    }
    if ((*u).passwd).is_null()
        || *((*u).passwd).offset(0 as libc::c_int as isize) as libc::c_int == '\0' as i32
    {
        snprintf(
            question.as_mut_ptr(),
            ::std::mem::size_of::<[libc::c_char; 1024]>() as libc::c_ulong,
            dcgettext(
                0 as *const libc::c_char,
                b"Password for '%s%s@%s': \0" as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ),
            scheme_leading_string((*u).scheme),
            (*u).user,
            (*u).host,
        );
        run_use_askpass(question.as_mut_ptr(), &mut (*u).passwd);
        if opt.recursive {
            opt.passwd = xstrdup((*u).passwd);
        }
    }
}
unsafe extern "C" fn format_and_print_line(
    mut prefix: *const libc::c_char,
    mut line: *const libc::c_char,
    mut line_length: libc::c_int,
) -> libc::c_int {
    let mut remaining_chars: libc::c_int = 0;
    let mut line_dup: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut token: *mut libc::c_char = 0 as *mut libc::c_char;
    line_dup = xstrdup(line);
    if printf(b"%s\0" as *const u8 as *const libc::c_char, prefix) < 0 as libc::c_int {
        rpl_free(line_dup as *mut libc::c_void);
        line_dup = 0 as *mut libc::c_char;
        return -(1 as libc::c_int);
    }
    remaining_chars = 0 as libc::c_int;
    token = strtok(line_dup, b" \0" as *const u8 as *const libc::c_char);
    while !token.is_null() {
        if remaining_chars <= strlen(token) as libc::c_int {
            if printf(
                b"\n%*c\0" as *const u8 as *const libc::c_char,
                4 as libc::c_int,
                ' ' as i32,
            ) < 0 as libc::c_int
            {
                rpl_free(line_dup as *mut libc::c_void);
                line_dup = 0 as *mut libc::c_char;
                return -(1 as libc::c_int);
            }
            remaining_chars = line_length - 4 as libc::c_int;
        }
        if printf(b"%s \0" as *const u8 as *const libc::c_char, token) < 0 as libc::c_int
        {
            rpl_free(line_dup as *mut libc::c_void);
            line_dup = 0 as *mut libc::c_char;
            return -(1 as libc::c_int);
        }
        remaining_chars = (remaining_chars as libc::c_ulong)
            .wrapping_sub(
                (strlen(token)).wrapping_add(1 as libc::c_int as libc::c_ulong),
            ) as libc::c_int as libc::c_int;
        token = strtok(
            0 as *mut libc::c_char,
            b" \0" as *const u8 as *const libc::c_char,
        );
    }
    if printf(b"\n\0" as *const u8 as *const libc::c_char) < 0 as libc::c_int {
        rpl_free(line_dup as *mut libc::c_void);
        line_dup = 0 as *mut libc::c_char;
        return -(1 as libc::c_int);
    }
    rpl_free(line_dup as *mut libc::c_void);
    line_dup = 0 as *mut libc::c_char;
    return 0 as libc::c_int;
}
unsafe extern "C" fn print_version() {
    let mut wgetrc_title: *const libc::c_char = dcgettext(
        0 as *const libc::c_char,
        b"Wgetrc: \0" as *const u8 as *const libc::c_char,
        5 as libc::c_int,
    );
    let mut locale_title: *const libc::c_char = dcgettext(
        0 as *const libc::c_char,
        b"Locale: \0" as *const u8 as *const libc::c_char,
        5 as libc::c_int,
    );
    let mut compile_title: *const libc::c_char = dcgettext(
        0 as *const libc::c_char,
        b"Compile: \0" as *const u8 as *const libc::c_char,
        5 as libc::c_int,
    );
    let mut link_title: *const libc::c_char = dcgettext(
        0 as *const libc::c_char,
        b"Link: \0" as *const u8 as *const libc::c_char,
        5 as libc::c_int,
    );
    let mut env_wgetrc: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut user_wgetrc: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut i: libc::c_int = 0;
    if printf(
        dcgettext(
            0 as *const libc::c_char,
            b"GNU Wget %s built on %s.\n\n\0" as *const u8 as *const libc::c_char,
            5 as libc::c_int,
        ),
        version_string,
        b"linux-gnu\0" as *const u8 as *const libc::c_char,
    ) < 0 as libc::c_int
    {
        exit(WGET_EXIT_IO_FAIL as libc::c_int);
    }
    i = 0 as libc::c_int;
    while !(*compiled_features.as_mut_ptr().offset(i as isize)).is_null() {
        let mut line_length: libc::c_int = 72 as libc::c_int;
        while line_length > 0 as libc::c_int
            && !(*compiled_features.as_mut_ptr().offset(i as isize)).is_null()
        {
            if printf(
                b"%s \0" as *const u8 as *const libc::c_char,
                *compiled_features.as_mut_ptr().offset(i as isize),
            ) < 0 as libc::c_int
            {
                exit(WGET_EXIT_IO_FAIL as libc::c_int);
            }
            line_length
                -= strlen(*compiled_features.as_mut_ptr().offset(i as isize))
                    as libc::c_int + 2 as libc::c_int;
            i += 1;
            i;
        }
        if printf(b"\n\0" as *const u8 as *const libc::c_char) < 0 as libc::c_int {
            exit(WGET_EXIT_IO_FAIL as libc::c_int);
        }
    }
    if printf(b"\n\0" as *const u8 as *const libc::c_char) < 0 as libc::c_int {
        exit(WGET_EXIT_IO_FAIL as libc::c_int);
    }
    if printf(b"%s\n\0" as *const u8 as *const libc::c_char, wgetrc_title)
        < 0 as libc::c_int
    {
        exit(WGET_EXIT_IO_FAIL as libc::c_int);
    }
    env_wgetrc = wgetrc_env_file_name();
    if !env_wgetrc.is_null() && *env_wgetrc as libc::c_int != 0 {
        if printf(
            dcgettext(
                0 as *const libc::c_char,
                b"    %s (env)\n\0" as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ),
            env_wgetrc,
        ) < 0 as libc::c_int
        {
            exit(WGET_EXIT_IO_FAIL as libc::c_int);
        }
        rpl_free(env_wgetrc as *mut libc::c_void);
        env_wgetrc = 0 as *mut libc::c_char;
    }
    user_wgetrc = wgetrc_user_file_name();
    if !user_wgetrc.is_null() {
        if printf(
            dcgettext(
                0 as *const libc::c_char,
                b"    %s (user)\n\0" as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ),
            user_wgetrc,
        ) < 0 as libc::c_int
        {
            exit(WGET_EXIT_IO_FAIL as libc::c_int);
        }
        rpl_free(user_wgetrc as *mut libc::c_void);
        user_wgetrc = 0 as *mut libc::c_char;
    }
    if printf(
        dcgettext(
            0 as *const libc::c_char,
            b"    %s (system)\n\0" as *const u8 as *const libc::c_char,
            5 as libc::c_int,
        ),
        b"/usr/local/etc/wgetrc\0" as *const u8 as *const libc::c_char,
    ) < 0 as libc::c_int
    {
        exit(WGET_EXIT_IO_FAIL as libc::c_int);
    }
    if format_and_print_line(
        locale_title,
        b"/usr/local/share/locale\0" as *const u8 as *const libc::c_char,
        72 as libc::c_int,
    ) < 0 as libc::c_int
    {
        exit(WGET_EXIT_IO_FAIL as libc::c_int);
    }
    if !compilation_string.is_null() {
        if format_and_print_line(compile_title, compilation_string, 72 as libc::c_int)
            < 0 as libc::c_int
        {
            exit(WGET_EXIT_IO_FAIL as libc::c_int);
        }
    }
    if !link_string.is_null() {
        if format_and_print_line(link_title, link_string, 72 as libc::c_int)
            < 0 as libc::c_int
        {
            exit(WGET_EXIT_IO_FAIL as libc::c_int);
        }
    }
    if printf(b"\n\0" as *const u8 as *const libc::c_char) < 0 as libc::c_int {
        exit(WGET_EXIT_IO_FAIL as libc::c_int);
    }
    if printf(
        dcgettext(
            0 as *const libc::c_char,
            b"Copyright (C) %s Free Software Foundation, Inc.\n\0" as *const u8
                as *const libc::c_char,
            5 as libc::c_int,
        ),
        b"2015\0" as *const u8 as *const libc::c_char,
    ) < 0 as libc::c_int
    {
        exit(WGET_EXIT_IO_FAIL as libc::c_int);
    }
    if fputs(
        dcgettext(
            0 as *const libc::c_char,
            b"License GPLv3+: GNU GPL version 3 or later\n<http://www.gnu.org/licenses/gpl.html>.\nThis is free software: you are free to change and redistribute it.\nThere is NO WARRANTY, to the extent permitted by law.\n\0"
                as *const u8 as *const libc::c_char,
            5 as libc::c_int,
        ),
        stdout,
    ) < 0 as libc::c_int
    {
        exit(WGET_EXIT_IO_FAIL as libc::c_int);
    }
    if fputs(
        dcgettext(
            0 as *const libc::c_char,
            b"\nOriginally written by Hrvoje Niksic <hniksic@xemacs.org>.\n\0"
                as *const u8 as *const libc::c_char,
            5 as libc::c_int,
        ),
        stdout,
    ) < 0 as libc::c_int
    {
        exit(WGET_EXIT_IO_FAIL as libc::c_int);
    }
    if fputs(
        dcgettext(
            0 as *const libc::c_char,
            b"Please send bug reports and questions to <bug-wget@gnu.org>.\n\0"
                as *const u8 as *const libc::c_char,
            5 as libc::c_int,
        ),
        stdout,
    ) < 0 as libc::c_int
    {
        exit(WGET_EXIT_IO_FAIL as libc::c_int);
    }
    exit(WGET_EXIT_SUCCESS as libc::c_int);
}
pub static mut program_name: *const libc::c_char = 0 as *const libc::c_char;
pub static mut program_argstring: *const libc::c_char = 0 as *const libc::c_char;
pub static mut timer: *mut ptimer = 0 as *const ptimer as *mut ptimer;
pub static mut cleaned_up: libc::c_int = 0;
unsafe fn main_0(
    mut argc: libc::c_int,
    mut argv: *mut *mut libc::c_char,
) -> libc::c_int {
    let mut p: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut i: libc::c_int = 0;
    let mut ret: libc::c_int = 0;
    let mut longindex: libc::c_int = 0;
    let mut nurls: libc::c_int = 0;
    let mut argstring_length: libc::c_int = 0;
    let mut use_userconfig: bool = 0 as libc::c_int != 0;
    let mut noconfig: bool = 0 as libc::c_int != 0;
    let mut append_to_log: bool = 0 as libc::c_int != 0;
    cleaned_up = 0 as libc::c_int;
    timer = ptimer_new();
    let mut start_time: libc::c_double = ptimer_measure(timer);
    total_downloaded_bytes = 0 as libc::c_int as wgint;
    program_name = *argv.offset(0 as libc::c_int as isize);
    i18n_initialize();
    exec_name = base_name(*argv.offset(0 as libc::c_int as isize));
    argstring_length = 1 as libc::c_int;
    i = 1 as libc::c_int;
    while i < argc {
        argstring_length = (argstring_length as libc::c_ulong)
            .wrapping_add(
                (strlen(*argv.offset(i as isize)))
                    .wrapping_add(3 as libc::c_int as libc::c_ulong)
                    .wrapping_add(1 as libc::c_int as libc::c_ulong),
            ) as libc::c_int as libc::c_int;
        i += 1;
        i;
    }
    p = malloc(argstring_length as libc::c_ulong) as *mut libc::c_char;
    program_argstring = p;
    if p.is_null() {
        fprintf(
            stderr,
            dcgettext(
                0 as *const libc::c_char,
                b"Memory allocation problem\n\0" as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ),
        );
        exit(WGET_EXIT_PARSE_ERROR as libc::c_int);
    }
    i = 1 as libc::c_int;
    while i < argc {
        let mut arglen: libc::c_int = 0;
        let fresh5 = p;
        p = p.offset(1);
        *fresh5 = '"' as i32 as libc::c_char;
        arglen = strlen(*argv.offset(i as isize)) as libc::c_int;
        memcpy(
            p as *mut libc::c_void,
            *argv.offset(i as isize) as *const libc::c_void,
            arglen as libc::c_ulong,
        );
        p = p.offset(arglen as isize);
        let fresh6 = p;
        p = p.offset(1);
        *fresh6 = '"' as i32 as libc::c_char;
        let fresh7 = p;
        p = p.offset(1);
        *fresh7 = ' ' as i32 as libc::c_char;
        i += 1;
        i;
    }
    *p = '\0' as i32 as libc::c_char;
    defaults();
    opt.homedir = home_dir();
    init_switches();
    longindex = -(1 as libc::c_int);
    while getopt_long(
        argc,
        argv,
        short_options.as_mut_ptr(),
        long_options.as_mut_ptr(),
        &mut longindex,
    ) != -(1 as libc::c_int)
    {
        let mut confval: libc::c_int = 0;
        let mut config_opt: *mut cmdline_option = 0 as *mut cmdline_option;
        if !(longindex >= 0 as libc::c_int) {
            continue;
        }
        confval = long_options[longindex as usize].val;
        config_opt = &mut *option_data
            .as_mut_ptr()
            .offset((confval & !(1024 as libc::c_int)) as isize) as *mut cmdline_option;
        if strcmp(
            ((*config_opt).long_name).as_mut_ptr(),
            b"no-config\0" as *const u8 as *const libc::c_char,
        ) == 0 as libc::c_int
        {
            noconfig = 1 as libc::c_int != 0;
            break;
        } else {
            if !(strcmp(
                ((*config_opt).long_name).as_mut_ptr(),
                b"config\0" as *const u8 as *const libc::c_char,
            ) == 0 as libc::c_int)
            {
                continue;
            }
            let mut flstats: file_stats_t = file_stats_t {
                access_err: 0,
                st_ino: 0,
                st_dev: 0,
            };
            use_userconfig = 1 as libc::c_int != 0;
            memset(
                &mut flstats as *mut file_stats_t as *mut libc::c_void,
                0 as libc::c_int,
                ::std::mem::size_of::<file_stats_t>() as libc::c_ulong,
            );
            if file_exists_p(optarg, &mut flstats) as libc::c_int != 0
                && run_wgetrc(optarg, &mut flstats) as libc::c_int != 0
            {
                break;
            }
            fprintf(
                stderr,
                dcgettext(
                    0 as *const libc::c_char,
                    b"Exiting due to error in %s\n\0" as *const u8
                        as *const libc::c_char,
                    5 as libc::c_int,
                ),
                optarg,
            );
            exit(WGET_EXIT_PARSE_ERROR as libc::c_int);
        }
    }
    if noconfig as libc::c_int == 0 as libc::c_int
        && use_userconfig as libc::c_int == 0 as libc::c_int
    {
        ret = initialize();
        if ret != 0 {
            return ret;
        }
    }
    opterr = 0 as libc::c_int;
    optind = 0 as libc::c_int;
    longindex = -(1 as libc::c_int);
    loop {
        ret = getopt_long(
            argc,
            argv,
            short_options.as_mut_ptr(),
            long_options.as_mut_ptr(),
            &mut longindex,
        );
        if !(ret != -(1 as libc::c_int)) {
            break;
        }
        let mut val: libc::c_int = 0;
        let mut cmdopt: *mut cmdline_option = 0 as *mut cmdline_option;
        if longindex == -(1 as libc::c_int) {
            if ret == '?' as i32 {
                print_usage(1 as libc::c_int);
                fprintf(stderr, b"\n\0" as *const u8 as *const libc::c_char);
                fprintf(
                    stderr,
                    dcgettext(
                        0 as *const libc::c_char,
                        b"Try `%s --help' for more options.\n\0" as *const u8
                            as *const libc::c_char,
                        5 as libc::c_int,
                    ),
                    exec_name,
                );
                exit(WGET_EXIT_PARSE_ERROR as libc::c_int);
            }
            longindex = optmap[(ret - 32 as libc::c_int) as usize] as libc::c_int;
        }
        val = long_options[longindex as usize].val;
        cmdopt = &mut *option_data
            .as_mut_ptr()
            .offset((val & !(1024 as libc::c_int)) as isize) as *mut cmdline_option;
        match (*cmdopt).type_0 as libc::c_uint {
            0 => {
                setoptval(
                    (*cmdopt).data as *const libc::c_char,
                    optarg,
                    ((*cmdopt).long_name).as_mut_ptr(),
                );
            }
            1 => {
                if !optarg.is_null() {
                    setoptval(
                        (*cmdopt).data as *const libc::c_char,
                        optarg,
                        ((*cmdopt).long_name).as_mut_ptr(),
                    );
                } else {
                    let mut neg: bool = val & 1024 as libc::c_int != 0;
                    setoptval(
                        (*cmdopt).data as *const libc::c_char,
                        if neg as libc::c_int != 0 {
                            b"0\0" as *const u8 as *const libc::c_char
                        } else {
                            b"1\0" as *const u8 as *const libc::c_char
                        },
                        ((*cmdopt).long_name).as_mut_ptr(),
                    );
                }
            }
            2 => {
                let mut func: Option::<unsafe extern "C" fn() -> ()> = ::std::mem::transmute::<
                    *const libc::c_void,
                    Option::<unsafe extern "C" fn() -> ()>,
                >((*cmdopt).data);
                func.unwrap()();
            }
            3 => {
                setoptval(
                    b"logfile\0" as *const u8 as *const libc::c_char,
                    optarg,
                    ((*cmdopt).long_name).as_mut_ptr(),
                );
                append_to_log = 1 as libc::c_int != 0;
            }
            6 => {
                if !optarg.is_null() {
                    run_command(optarg);
                }
            }
            7 => {
                p = optarg;
                while !p.is_null() && *p as libc::c_int != 0 {
                    match *p as libc::c_int {
                        118 => {
                            setoptval(
                                b"verbose\0" as *const u8 as *const libc::c_char,
                                b"0\0" as *const u8 as *const libc::c_char,
                                ((*cmdopt).long_name).as_mut_ptr(),
                            );
                        }
                        72 => {
                            setoptval(
                                b"addhostdir\0" as *const u8 as *const libc::c_char,
                                b"0\0" as *const u8 as *const libc::c_char,
                                ((*cmdopt).long_name).as_mut_ptr(),
                            );
                        }
                        100 => {
                            setoptval(
                                b"dirstruct\0" as *const u8 as *const libc::c_char,
                                b"0\0" as *const u8 as *const libc::c_char,
                                ((*cmdopt).long_name).as_mut_ptr(),
                            );
                        }
                        99 => {
                            setoptval(
                                b"noclobber\0" as *const u8 as *const libc::c_char,
                                b"1\0" as *const u8 as *const libc::c_char,
                                ((*cmdopt).long_name).as_mut_ptr(),
                            );
                        }
                        112 => {
                            setoptval(
                                b"noparent\0" as *const u8 as *const libc::c_char,
                                b"1\0" as *const u8 as *const libc::c_char,
                                ((*cmdopt).long_name).as_mut_ptr(),
                            );
                        }
                        _ => {
                            fprintf(
                                stderr,
                                dcgettext(
                                    0 as *const libc::c_char,
                                    b"%s: illegal option -- `-n%c'\n\0" as *const u8
                                        as *const libc::c_char,
                                    5 as libc::c_int,
                                ),
                                exec_name,
                                *p as libc::c_int,
                            );
                            print_usage(1 as libc::c_int);
                            fprintf(stderr, b"\n\0" as *const u8 as *const libc::c_char);
                            fprintf(
                                stderr,
                                dcgettext(
                                    0 as *const libc::c_char,
                                    b"Try `%s --help' for more options.\n\0" as *const u8
                                        as *const libc::c_char,
                                    5 as libc::c_int,
                                ),
                                exec_name,
                            );
                            exit(WGET_EXIT_GENERIC_ERROR as libc::c_int);
                        }
                    }
                    p = p.offset(1);
                    p;
                }
            }
            8 | 4 => {
                let mut flag: bool = 1 as libc::c_int != 0;
                if !optarg.is_null() {
                    flag = *optarg as libc::c_int == '1' as i32
                        || c_tolower(*optarg as libc::c_int) == 'y' as i32
                        || c_tolower(
                            *optarg.offset(0 as libc::c_int as isize) as libc::c_int,
                        ) == 'o' as i32
                            && c_tolower(
                                *optarg.offset(1 as libc::c_int as isize) as libc::c_int,
                            ) == 'n' as i32;
                }
                setoptval(
                    if (*cmdopt).type_0 as libc::c_uint
                        == OPT__PARENT as libc::c_int as libc::c_uint
                    {
                        b"noparent\0" as *const u8 as *const libc::c_char
                    } else {
                        b"noclobber\0" as *const u8 as *const libc::c_char
                    },
                    if flag as libc::c_int != 0 {
                        b"0\0" as *const u8 as *const libc::c_char
                    } else {
                        b"1\0" as *const u8 as *const libc::c_char
                    },
                    ((*cmdopt).long_name).as_mut_ptr(),
                );
            }
            5 => {
                setoptval(
                    b"removelisting\0" as *const u8 as *const libc::c_char,
                    b"0\0" as *const u8 as *const libc::c_char,
                    ((*cmdopt).long_name).as_mut_ptr(),
                );
            }
            _ => {}
        }
        longindex = -(1 as libc::c_int);
    }
    nurls = argc - optind;
    log_init(opt.lfilename, append_to_log);
    if opt.noclobber as libc::c_int != 0
        && (opt.convert_links as libc::c_int != 0
            || opt.convert_file_only as libc::c_int != 0)
    {
        fprintf(
            stderr,
            if opt.convert_links as libc::c_int != 0 {
                dcgettext(
                    0 as *const libc::c_char,
                    b"Both --no-clobber and --convert-links were specified, only --convert-links will be used.\n\0"
                        as *const u8 as *const libc::c_char,
                    5 as libc::c_int,
                )
            } else {
                dcgettext(
                    0 as *const libc::c_char,
                    b"Both --no-clobber and --convert-file-only were specified, only --convert-file-only will be used.\n\0"
                        as *const u8 as *const libc::c_char,
                    5 as libc::c_int,
                )
            },
        );
        opt.noclobber = 0 as libc::c_int != 0;
    }
    if opt.reclevel == 0 as libc::c_int {
        opt.reclevel = -(1 as libc::c_int);
    }
    if opt.spider as libc::c_int != 0 || opt.delete_after as libc::c_int != 0 {
        opt.no_dirstruct = 1 as libc::c_int != 0;
    }
    if opt.page_requisites as libc::c_int != 0 && !opt.recursive {
        opt.reclevel = 0 as libc::c_int;
        if !opt.no_dirstruct {
            opt.dirstruct = 1 as libc::c_int != 0;
        }
    }
    if opt.verbose == -(1 as libc::c_int) {
        opt.verbose = !opt.quiet as libc::c_int;
    }
    if opt.verbose == 0 && opt.show_progress == -(1 as libc::c_int) {
        opt.show_progress = 0 as libc::c_int;
    }
    if opt.quiet as libc::c_int != 0 && opt.show_progress == -(1 as libc::c_int) {
        opt.show_progress = 0 as libc::c_int;
    }
    if opt.verbose != 0 && opt.quiet as libc::c_int != 0 {
        fprintf(
            stderr,
            dcgettext(
                0 as *const libc::c_char,
                b"Can't be verbose and quiet at the same time.\n\0" as *const u8
                    as *const libc::c_char,
                5 as libc::c_int,
            ),
        );
        print_usage(1 as libc::c_int);
        exit(WGET_EXIT_GENERIC_ERROR as libc::c_int);
    }
    if opt.timestamping as libc::c_int != 0 && opt.noclobber as libc::c_int != 0 {
        fprintf(
            stderr,
            dcgettext(
                0 as *const libc::c_char,
                b"Can't timestamp and not clobber old files at the same time.\n\0"
                    as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ),
        );
        print_usage(1 as libc::c_int);
        exit(WGET_EXIT_GENERIC_ERROR as libc::c_int);
    }
    if opt.ipv4_only as libc::c_int != 0 && opt.ipv6_only as libc::c_int != 0 {
        fprintf(
            stderr,
            dcgettext(
                0 as *const libc::c_char,
                b"Cannot specify both --inet4-only and --inet6-only.\n\0" as *const u8
                    as *const libc::c_char,
                5 as libc::c_int,
            ),
        );
        print_usage(1 as libc::c_int);
        exit(WGET_EXIT_GENERIC_ERROR as libc::c_int);
    }
    if !(opt.output_document).is_null() {
        if (opt.convert_links as libc::c_int != 0
            || opt.convert_file_only as libc::c_int != 0)
            && (nurls > 1 as libc::c_int || opt.page_requisites as libc::c_int != 0
                || opt.recursive as libc::c_int != 0)
        {
            fputs(
                dcgettext(
                    0 as *const libc::c_char,
                    b"Cannot specify both -k or --convert-file-only and -O if multiple URLs are given, or in combination\nwith -p or -r. See the manual for details.\n\n\0"
                        as *const u8 as *const libc::c_char,
                    5 as libc::c_int,
                ),
                stderr,
            );
            print_usage(1 as libc::c_int);
            exit(WGET_EXIT_GENERIC_ERROR as libc::c_int);
        }
        if opt.page_requisites as libc::c_int != 0 || opt.recursive as libc::c_int != 0 {
            logprintf(
                LOG_NOTQUIET,
                b"%s\0" as *const u8 as *const libc::c_char,
                dcgettext(
                    0 as *const libc::c_char,
                    b"WARNING: combining -O with -r or -p will mean that all downloaded content\nwill be placed in the single file you specified.\n\n\0"
                        as *const u8 as *const libc::c_char,
                    5 as libc::c_int,
                ),
            );
        }
        if opt.timestamping {
            logprintf(
                LOG_NOTQUIET,
                b"%s\0" as *const u8 as *const libc::c_char,
                dcgettext(
                    0 as *const libc::c_char,
                    b"WARNING: timestamping does nothing in combination with -O. See the manual\nfor details.\n\n\0"
                        as *const u8 as *const libc::c_char,
                    5 as libc::c_int,
                ),
            );
            opt.timestamping = 0 as libc::c_int != 0;
        }
        if opt.noclobber as libc::c_int != 0
            && file_exists_p(opt.output_document, 0 as *mut file_stats_t) as libc::c_int
                != 0
        {
            logprintf(
                LOG_VERBOSE,
                dcgettext(
                    0 as *const libc::c_char,
                    b"File %s already there; not retrieving.\n\0" as *const u8
                        as *const libc::c_char,
                    5 as libc::c_int,
                ),
                quote(opt.output_document),
            );
            exit(WGET_EXIT_GENERIC_ERROR as libc::c_int);
        }
    }
    if !(opt.warc_filename).is_null() {
        if opt.noclobber {
            fprintf(
                stderr,
                dcgettext(
                    0 as *const libc::c_char,
                    b"WARC output does not work with --no-clobber, --no-clobber will be disabled.\n\0"
                        as *const u8 as *const libc::c_char,
                    5 as libc::c_int,
                ),
            );
            opt.noclobber = 0 as libc::c_int != 0;
        }
        if opt.timestamping {
            fprintf(
                stderr,
                dcgettext(
                    0 as *const libc::c_char,
                    b"WARC output does not work with timestamping, timestamping will be disabled.\n\0"
                        as *const u8 as *const libc::c_char,
                    5 as libc::c_int,
                ),
            );
            opt.timestamping = 0 as libc::c_int != 0;
        }
        if opt.spider {
            fprintf(
                stderr,
                dcgettext(
                    0 as *const libc::c_char,
                    b"WARC output does not work with --spider.\n\0" as *const u8
                        as *const libc::c_char,
                    5 as libc::c_int,
                ),
            );
            exit(WGET_EXIT_GENERIC_ERROR as libc::c_int);
        }
        if opt.always_rest as libc::c_int != 0
            || opt.start_pos >= 0 as libc::c_int as libc::c_long
        {
            fprintf(
                stderr,
                dcgettext(
                    0 as *const libc::c_char,
                    b"WARC output does not work with --continue or --start-pos, they will be disabled.\n\0"
                        as *const u8 as *const libc::c_char,
                    5 as libc::c_int,
                ),
            );
            opt.always_rest = 0 as libc::c_int != 0;
            opt.start_pos = -(1 as libc::c_int) as wgint;
        }
        if !(opt.warc_cdx_dedup_filename).is_null() && !opt.warc_digests_enabled {
            fprintf(
                stderr,
                dcgettext(
                    0 as *const libc::c_char,
                    b"Digests are disabled; WARC deduplication will not find duplicate records.\n\0"
                        as *const u8 as *const libc::c_char,
                    5 as libc::c_int,
                ),
            );
        }
        if opt.warc_keep_log {
            opt.progress_type = xstrdup(b"dot\0" as *const u8 as *const libc::c_char);
        }
    }
    if opt.always_rest as libc::c_int != 0
        || opt.start_pos >= 0 as libc::c_int as libc::c_long
    {
        if opt.compression as libc::c_uint
            == compression_auto as libc::c_int as libc::c_uint
        {
            opt.compression = compression_none;
        } else if opt.compression as libc::c_uint
            != compression_none as libc::c_int as libc::c_uint
        {
            fprintf(
                stderr,
                dcgettext(
                    0 as *const libc::c_char,
                    b"Compression does not work with --continue or --start-pos, they will be disabled.\n\0"
                        as *const u8 as *const libc::c_char,
                    5 as libc::c_int,
                ),
            );
            opt.always_rest = 0 as libc::c_int != 0;
            opt.start_pos = -(1 as libc::c_int) as wgint;
        }
    }
    if opt.ask_passwd as libc::c_int != 0 && !(opt.passwd).is_null() {
        fprintf(
            stderr,
            dcgettext(
                0 as *const libc::c_char,
                b"Cannot specify both --ask-password and --password.\n\0" as *const u8
                    as *const libc::c_char,
                5 as libc::c_int,
            ),
        );
        print_usage(1 as libc::c_int);
        exit(WGET_EXIT_GENERIC_ERROR as libc::c_int);
    }
    if opt.ask_passwd as libc::c_int != 0
        && !(!(opt.user).is_null() || !(opt.http_user).is_null()
            || !(opt.ftp_user).is_null())
    {
        fprintf(
            stderr,
            dcgettext(
                0 as *const libc::c_char,
                b"WARNING: No username set with --ask-password. This is usually not what you want.\n\0"
                    as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ),
        );
    }
    if opt.start_pos >= 0 as libc::c_int as libc::c_long
        && opt.always_rest as libc::c_int != 0
    {
        fprintf(
            stderr,
            dcgettext(
                0 as *const libc::c_char,
                b"Specifying both --start-pos and --continue is not recommended; --continue will be disabled.\n\0"
                    as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ),
        );
        opt.always_rest = 0 as libc::c_int != 0;
    }
    if nurls == 0 && (opt.input_filename).is_null() {
        fprintf(
            stderr,
            dcgettext(
                0 as *const libc::c_char,
                b"%s: missing URL\n\0" as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ),
            exec_name,
        );
        print_usage(1 as libc::c_int);
        fprintf(stderr, b"\n\0" as *const u8 as *const libc::c_char);
        fprintf(
            stderr,
            dcgettext(
                0 as *const libc::c_char,
                b"Try `%s --help' for more options.\n\0" as *const u8
                    as *const libc::c_char,
                5 as libc::c_int,
            ),
            exec_name,
        );
        exit(WGET_EXIT_GENERIC_ERROR as libc::c_int);
    }
    match opt.regex_type as libc::c_uint {
        0 => {
            opt
                .regex_compile_fun = Some(
                compile_pcre_regex
                    as unsafe extern "C" fn(*const libc::c_char) -> *mut libc::c_void,
            );
            opt
                .regex_match_fun = Some(
                match_pcre_regex
                    as unsafe extern "C" fn(
                        *const libc::c_void,
                        *const libc::c_char,
                    ) -> bool,
            );
        }
        1 | _ => {
            opt
                .regex_compile_fun = Some(
                compile_posix_regex
                    as unsafe extern "C" fn(*const libc::c_char) -> *mut libc::c_void,
            );
            opt
                .regex_match_fun = Some(
                match_posix_regex
                    as unsafe extern "C" fn(
                        *const libc::c_void,
                        *const libc::c_char,
                    ) -> bool,
            );
        }
    }
    if !(opt.acceptregex_s).is_null() {
        opt.acceptregex = (opt.regex_compile_fun).unwrap()(opt.acceptregex_s);
        if (opt.acceptregex).is_null() {
            exit(WGET_EXIT_GENERIC_ERROR as libc::c_int);
        }
    }
    if !(opt.rejectregex_s).is_null() {
        opt.rejectregex = (opt.regex_compile_fun).unwrap()(opt.rejectregex_s);
        if (opt.rejectregex).is_null() {
            exit(WGET_EXIT_GENERIC_ERROR as libc::c_int);
        }
    }
    if !(opt.post_data).is_null() || !(opt.post_file_name).is_null() {
        if !(opt.post_data).is_null() && !(opt.post_file_name).is_null() {
            fprintf(
                stderr,
                dcgettext(
                    0 as *const libc::c_char,
                    b"You cannot specify both --post-data and --post-file.\n\0"
                        as *const u8 as *const libc::c_char,
                    5 as libc::c_int,
                ),
            );
            exit(WGET_EXIT_GENERIC_ERROR as libc::c_int);
        } else if !(opt.method).is_null() {
            fprintf(
                stderr,
                dcgettext(
                    0 as *const libc::c_char,
                    b"You cannot use --post-data or --post-file along with --method. --method expects data through --body-data and --body-file options\n\0"
                        as *const u8 as *const libc::c_char,
                    5 as libc::c_int,
                ),
            );
            exit(WGET_EXIT_GENERIC_ERROR as libc::c_int);
        }
    }
    if !(opt.body_data).is_null() || !(opt.body_file).is_null() {
        if (opt.method).is_null() {
            fprintf(
                stderr,
                dcgettext(
                    0 as *const libc::c_char,
                    b"You must specify a method through --method=HTTPMethod to use with --body-data or --body-file.\n\0"
                        as *const u8 as *const libc::c_char,
                    5 as libc::c_int,
                ),
            );
            exit(WGET_EXIT_GENERIC_ERROR as libc::c_int);
        } else if !(opt.body_data).is_null() && !(opt.body_file).is_null() {
            fprintf(
                stderr,
                dcgettext(
                    0 as *const libc::c_char,
                    b"You cannot specify both --body-data and --body-file.\n\0"
                        as *const u8 as *const libc::c_char,
                    5 as libc::c_int,
                ),
            );
            exit(WGET_EXIT_GENERIC_ERROR as libc::c_int);
        }
    }
    if !(opt.method).is_null()
        && c_strcasecmp(opt.method, b"HEAD\0" as *const u8 as *const libc::c_char)
            == 0 as libc::c_int
    {
        setoptval(
            b"spider\0" as *const u8 as *const libc::c_char,
            b"1\0" as *const u8 as *const libc::c_char,
            b"spider\0" as *const u8 as *const libc::c_char,
        );
    }
    if !(opt.post_data).is_null() || !(opt.post_file_name).is_null() {
        setoptval(
            b"method\0" as *const u8 as *const libc::c_char,
            b"POST\0" as *const u8 as *const libc::c_char,
            b"method\0" as *const u8 as *const libc::c_char,
        );
        if !(opt.post_data).is_null() {
            setoptval(
                b"bodydata\0" as *const u8 as *const libc::c_char,
                opt.post_data,
                b"body-data\0" as *const u8 as *const libc::c_char,
            );
            rpl_free(opt.post_data as *mut libc::c_void);
            opt.post_data = 0 as *mut libc::c_char;
        } else {
            setoptval(
                b"bodyfile\0" as *const u8 as *const libc::c_char,
                opt.post_file_name,
                b"body-file\0" as *const u8 as *const libc::c_char,
            );
            rpl_free(opt.post_file_name as *mut libc::c_void);
            opt.post_file_name = 0 as *mut libc::c_char;
        }
    }
    if opt.enable_iri {
        if !(opt.locale).is_null() && !check_encoding_name(opt.locale) {
            rpl_free(opt.locale as *mut libc::c_void);
            opt.locale = 0 as *const libc::c_char;
        }
        if (opt.locale).is_null() {
            opt.locale = find_locale();
        }
        if !(opt.encoding_remote).is_null() && !check_encoding_name(opt.encoding_remote)
        {
            rpl_free(opt.encoding_remote as *mut libc::c_void);
            opt.encoding_remote = 0 as *mut libc::c_char;
        }
    }
    if opt.ask_passwd {
        opt.passwd = prompt_for_password();
        if (opt.passwd).is_null()
            || *(opt.passwd).offset(0 as libc::c_int as isize) as libc::c_int
                == '\0' as i32
        {
            exit(WGET_EXIT_GENERIC_ERROR as libc::c_int);
        }
    }
    if !(opt.use_askpass).is_null() {
        if *(opt.use_askpass).offset(0 as libc::c_int as isize) as libc::c_int
            == '\0' as i32
        {
            fprintf(
                stderr,
                dcgettext(
                    0 as *const libc::c_char,
                    b"use-askpass requires a string or either environment variable WGET_ASKPASS or SSH_ASKPASS to be set.\n\0"
                        as *const u8 as *const libc::c_char,
                    5 as libc::c_int,
                ),
            );
            exit(WGET_EXIT_GENERIC_ERROR as libc::c_int);
        }
    }
    if opt.background {
        let mut logfile_changed: bool = fork_to_background();
        if logfile_changed {
            log_init(opt.lfilename, append_to_log);
        }
    }
    if opt.show_progress != 0 {
        set_progress_implementation(opt.progress_type);
    }
    if !(opt.warc_filename).is_null() {
        warc_init();
    }
    if opt.debug as libc::c_long != 0 {
        debug_logprintf(
            b"DEBUG output created by Wget %s on %s.\n\n\0" as *const u8
                as *const libc::c_char,
            version_string,
            b"linux-gnu\0" as *const u8 as *const libc::c_char,
        );
    }
    if !(opt.output_document).is_null() {
        if *opt.output_document as libc::c_int == '-' as i32
            && *(opt.output_document).offset(1 as libc::c_int as isize) == 0
        {
            output_stream = stdout;
        } else {
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
            if opt.unlink_requested {
                unlink(opt.output_document);
            }
            output_stream = rpl_fopen(
                opt.output_document,
                if opt.always_rest as libc::c_int != 0 {
                    b"ab\0" as *const u8 as *const libc::c_char
                } else {
                    b"wb\0" as *const u8 as *const libc::c_char
                },
            );
            if output_stream.is_null() {
                perror(opt.output_document);
                exit(WGET_EXIT_GENERIC_ERROR as libc::c_int);
            }
            if fstat(fileno(output_stream), &mut st) == 0 as libc::c_int
                && st.st_mode & 0o170000 as libc::c_int as libc::c_uint
                    == 0o100000 as libc::c_int as libc::c_uint
            {
                output_stream_regular = 1 as libc::c_int != 0;
            }
        }
        if !output_stream_regular
            && (opt.convert_links as libc::c_int != 0
                || opt.recursive as libc::c_int != 0)
        {
            fprintf(
                stderr,
                dcgettext(
                    0 as *const libc::c_char,
                    b"-k or -r can be used together with -O only if outputting to a regular file.\n\0"
                        as *const u8 as *const libc::c_char,
                    5 as libc::c_int,
                ),
            );
            exit(WGET_EXIT_GENERIC_ERROR as libc::c_int);
        }
        if !output_stream_regular
            && (opt.convert_links as libc::c_int != 0
                || opt.convert_file_only as libc::c_int != 0)
        {
            fprintf(
                stderr,
                dcgettext(
                    0 as *const libc::c_char,
                    b"--convert-links or --convert-file-only can be used together only if outputting to a regular file.\n\0"
                        as *const u8 as *const libc::c_char,
                    5 as libc::c_int,
                ),
            );
            exit(WGET_EXIT_GENERIC_ERROR as libc::c_int);
        }
    }
    if signal(
        1 as libc::c_int,
        ::std::mem::transmute::<
            libc::intptr_t,
            __sighandler_t,
        >(1 as libc::c_int as libc::intptr_t),
    )
        != ::std::mem::transmute::<
            libc::intptr_t,
            __sighandler_t,
        >(1 as libc::c_int as libc::intptr_t)
    {
        signal(
            1 as libc::c_int,
            Some(redirect_output_signal as unsafe extern "C" fn(libc::c_int) -> ()),
        );
    }
    signal(
        10 as libc::c_int,
        Some(redirect_output_signal as unsafe extern "C" fn(libc::c_int) -> ()),
    );
    signal(
        13 as libc::c_int,
        ::std::mem::transmute::<
            libc::intptr_t,
            __sighandler_t,
        >(1 as libc::c_int as libc::intptr_t),
    );
    signal(
        28 as libc::c_int,
        Some(progress_handle_sigwinch as unsafe extern "C" fn(libc::c_int) -> ()),
    );
    i = 0 as libc::c_int;
    while i < nurls {
        let mut t: *mut libc::c_char = 0 as *mut libc::c_char;
        let mut filename: *mut libc::c_char = 0 as *mut libc::c_char;
        let mut redirected_URL: *mut libc::c_char = 0 as *mut libc::c_char;
        let mut dt: libc::c_int = 0 as libc::c_int;
        let mut url_err: libc::c_int = 0;
        let mut iri: *mut iri = iri_new();
        let mut url_parsed: *mut url = 0 as *mut url;
        t = rewrite_shorthand_url(*argv.offset(optind as isize));
        if t.is_null() {
            t = *argv.offset(optind as isize);
        }
        set_uri_encoding(iri, opt.locale, 1 as libc::c_int != 0);
        url_parsed = url_parse(t, &mut url_err, iri, 1 as libc::c_int != 0);
        if url_parsed.is_null() {
            logprintf(
                LOG_NOTQUIET,
                b"%s: %s.\n\0" as *const u8 as *const libc::c_char,
                t,
                url_error(url_err),
            );
            inform_exit_status(URLERROR);
        } else {
            if !(opt.use_askpass).is_null() {
                use_askpass(url_parsed);
            }
            if (opt.recursive as libc::c_int != 0
                || opt.page_requisites as libc::c_int != 0)
                && (url_scheme(t) as libc::c_uint
                    != SCHEME_FTP as libc::c_int as libc::c_uint
                    || url_uses_proxy(url_parsed) as libc::c_int != 0)
            {
                let mut old_follow_ftp: libc::c_int = opt.follow_ftp as libc::c_int;
                if url_scheme(t) as libc::c_uint
                    == SCHEME_FTP as libc::c_int as libc::c_uint
                {
                    opt.follow_ftp = 1 as libc::c_int != 0;
                }
                retrieve_tree(url_parsed, 0 as *mut iri);
                opt.follow_ftp = old_follow_ftp != 0;
            } else {
                retrieve_url(
                    url_parsed,
                    t,
                    &mut filename,
                    &mut redirected_URL,
                    0 as *const libc::c_char,
                    &mut dt,
                    opt.recursive,
                    iri,
                    1 as libc::c_int != 0,
                );
            }
            if opt.delete_after as libc::c_int != 0 && !filename.is_null()
                && file_exists_p(filename, 0 as *mut file_stats_t) as libc::c_int != 0
            {
                if opt.debug as libc::c_long != 0 {
                    debug_logprintf(
                        b"Removing file due to --delete-after in main():\n\0"
                            as *const u8 as *const libc::c_char,
                    );
                }
                logprintf(
                    LOG_VERBOSE,
                    dcgettext(
                        0 as *const libc::c_char,
                        b"Removing %s.\n\0" as *const u8 as *const libc::c_char,
                        5 as libc::c_int,
                    ),
                    filename,
                );
                if unlink(filename) != 0 {
                    logprintf(
                        LOG_NOTQUIET,
                        b"unlink: %s\n\0" as *const u8 as *const libc::c_char,
                        strerror(*__errno_location()),
                    );
                }
            }
            rpl_free(redirected_URL as *mut libc::c_void);
            redirected_URL = 0 as *mut libc::c_char;
            rpl_free(filename as *mut libc::c_void);
            filename = 0 as *mut libc::c_char;
            url_free(url_parsed);
        }
        iri_free(iri);
        if t != *argv.offset(optind as isize) {
            rpl_free(t as *mut libc::c_void);
            t = 0 as *mut libc::c_char;
        }
        i += 1;
        i;
        optind += 1;
        optind;
    }
    if !(opt.input_filename).is_null() {
        let mut count: libc::c_int = 0;
        let mut status: libc::c_int = 0;
        status = retrieve_from_file(opt.input_filename, opt.force_html, &mut count)
            as libc::c_int;
        inform_exit_status(status as uerr_t);
        if count == 0 {
            logprintf(
                LOG_NOTQUIET,
                dcgettext(
                    0 as *const libc::c_char,
                    b"No URLs found in %s.\n\0" as *const u8 as *const libc::c_char,
                    5 as libc::c_int,
                ),
                opt.input_filename,
            );
        }
    }
    if opt.recursive as libc::c_int != 0 && opt.spider as libc::c_int != 0 {
        print_broken_links();
    }
    if (opt.recursive as libc::c_int != 0 || opt.page_requisites as libc::c_int != 0
        || nurls > 1 as libc::c_int
        || !(opt.input_filename).is_null()
            && total_downloaded_bytes != 0 as libc::c_int as libc::c_long)
        && total_downloaded_bytes != 0 as libc::c_int as libc::c_long
    {
        let mut end_time: libc::c_double = ptimer_measure(timer);
        let mut wall_time: *mut libc::c_char = xstrdup(
            secs_to_human_time(end_time - start_time),
        );
        let mut download_time: *mut libc::c_char = xstrdup(
            secs_to_human_time(total_download_time),
        );
        ptimer_destroy(timer);
        timer = 0 as *mut ptimer;
        logprintf(
            LOG_NOTQUIET,
            dcgettext(
                0 as *const libc::c_char,
                b"FINISHED --%s--\nTotal wall clock time: %s\nDownloaded: %d files, %s in %s (%s)\n\0"
                    as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ),
            datetime_str(time(0 as *mut time_t)),
            wall_time,
            numurls,
            human_readable(total_downloaded_bytes, 10 as libc::c_int, 1 as libc::c_int),
            download_time,
            retr_rate(total_downloaded_bytes, total_download_time),
        );
        rpl_free(wall_time as *mut libc::c_void);
        wall_time = 0 as *mut libc::c_char;
        rpl_free(download_time as *mut libc::c_void);
        download_time = 0 as *mut libc::c_char;
        if opt.quota != 0 && total_downloaded_bytes > opt.quota {
            logprintf(
                LOG_NOTQUIET,
                dcgettext(
                    0 as *const libc::c_char,
                    b"Download quota of %s EXCEEDED!\n\0" as *const u8
                        as *const libc::c_char,
                    5 as libc::c_int,
                ),
                human_readable(opt.quota, 10 as libc::c_int, 1 as libc::c_int),
            );
        }
    }
    if !(opt.cookies_output).is_null() {
        save_cookies();
    }
    if (opt.convert_links as libc::c_int != 0
        || opt.convert_file_only as libc::c_int != 0) && !opt.delete_after
    {
        convert_all_links();
    }
    cleanup();
    exit(get_exit_status());
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
