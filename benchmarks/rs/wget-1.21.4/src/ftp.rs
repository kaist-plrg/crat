use ::libc;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    fn fclose(__stream: *mut FILE) -> libc::c_int;
    fn sprintf(_: *mut libc::c_char, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn snprintf(
        _: *mut libc::c_char,
        _: libc::c_ulong,
        _: *const libc::c_char,
        _: ...
    ) -> libc::c_int;
    fn time(__timer: *mut time_t) -> time_t;
    fn __xstat(
        __ver: libc::c_int,
        __filename: *const libc::c_char,
        __stat_buf: *mut stat,
    ) -> libc::c_int;
    fn __lxstat(
        __ver: libc::c_int,
        __filename: *const libc::c_char,
        __stat_buf: *mut stat,
    ) -> libc::c_int;
    fn chmod(__file: *const libc::c_char, __mode: __mode_t) -> libc::c_int;
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
    fn memcmp(
        _: *const libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> libc::c_int;
    fn strcpy(_: *mut libc::c_char, _: *const libc::c_char) -> *mut libc::c_char;
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    fn strchr(_: *const libc::c_char, _: libc::c_int) -> *mut libc::c_char;
    fn strrchr(_: *const libc::c_char, _: libc::c_int) -> *mut libc::c_char;
    fn strstr(_: *const libc::c_char, _: *const libc::c_char) -> *mut libc::c_char;
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    fn strerror(_: libc::c_int) -> *mut libc::c_char;
    fn strcasecmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    fn rpl_free(_: *mut libc::c_void);
    static mut opt: options;
    fn abort() -> !;
    fn rpl_strtoll(
        string: *const libc::c_char,
        endptr: *mut *mut libc::c_char,
        base: libc::c_int,
    ) -> libc::c_longlong;
    fn xmalloc(s: size_t) -> *mut libc::c_void;
    fn xrealloc(p: *mut libc::c_void, s: size_t) -> *mut libc::c_void;
    fn xstrdup(str: *const libc::c_char) -> *mut libc::c_char;
    fn __getdelim(
        __lineptr: *mut *mut libc::c_char,
        __n: *mut size_t,
        __delimiter: libc::c_int,
        __stream: *mut FILE,
    ) -> __ssize_t;
    fn rpl_fopen(filename: *const libc::c_char, mode: *const libc::c_char) -> *mut FILE;
    fn logprintf(_: log_options, _: *const libc::c_char, _: ...);
    fn debug_logprintf(_: *const libc::c_char, _: ...);
    fn logputs(_: log_options, _: *const libc::c_char);
    fn quote_n(n: libc::c_int, arg: *const libc::c_char) -> *const libc::c_char;
    fn quote(arg: *const libc::c_char) -> *const libc::c_char;
    fn quotearg_style(s: quoting_style, arg: *const libc::c_char) -> *mut libc::c_char;
    fn symlink(__from: *const libc::c_char, __to: *const libc::c_char) -> libc::c_int;
    fn readlink(
        __path: *const libc::c_char,
        __buf: *mut libc::c_char,
        __len: size_t,
    ) -> ssize_t;
    fn unlink(__name: *const libc::c_char) -> libc::c_int;
    fn __errno_location() -> *mut libc::c_int;
    fn datetime_str(_: time_t) -> *mut libc::c_char;
    fn aprintf(_: *const libc::c_char, _: ...) -> *mut libc::c_char;
    fn concat_strings(_: *const libc::c_char, _: ...) -> *mut libc::c_char;
    fn touch(_: *const libc::c_char, _: time_t);
    fn remove_link(_: *const libc::c_char) -> libc::c_int;
    fn file_exists_p(_: *const libc::c_char, _: *mut file_stats_t) -> bool;
    fn fopen_excl(_: *const libc::c_char, _: libc::c_int) -> *mut FILE;
    fn file_merge(_: *const libc::c_char, _: *const libc::c_char) -> *mut libc::c_char;
    fn fnmatch_nocase(
        _: *const libc::c_char,
        _: *const libc::c_char,
        _: libc::c_int,
    ) -> libc::c_int;
    fn acceptable(_: *const libc::c_char) -> bool;
    fn accept_url(_: *const libc::c_char) -> bool;
    fn accdir(s: *const libc::c_char) -> bool;
    fn has_wildcards_p(_: *const libc::c_char) -> bool;
    fn human_readable(_: wgint, _: libc::c_int, _: libc::c_int) -> *mut libc::c_char;
    fn number_to_static_string(_: wgint) -> *mut libc::c_char;
    fn url_set_dir(_: *mut url, _: *const libc::c_char);
    fn url_set_file(_: *mut url, _: *const libc::c_char);
    fn url_string(_: *const url, _: url_auth_mode) -> *mut libc::c_char;
    fn url_file_name(_: *const url, _: *mut libc::c_char) -> *mut libc::c_char;
    fn mkalldirs(_: *const libc::c_char) -> libc::c_int;
    static mut numurls: libc::c_int;
    static mut total_downloaded_bytes: wgint;
    static mut total_download_time: libc::c_double;
    static mut output_stream: *mut FILE;
    fn fd_read_body(
        _: *const libc::c_char,
        _: libc::c_int,
        _: *mut FILE,
        _: wgint,
        _: wgint,
        _: *mut wgint,
        _: *mut wgint,
        _: *mut libc::c_double,
        _: libc::c_int,
        _: *mut FILE,
    ) -> libc::c_int;
    fn retr_rate(_: wgint, _: libc::c_double) -> *const libc::c_char;
    fn printwhat(_: libc::c_int, _: libc::c_int);
    fn sleep_between_retrievals(_: libc::c_int);
    fn rotate_backups(_: *const libc::c_char);
    fn set_local_file(_: *mut *const libc::c_char, _: *const libc::c_char);
    fn input_file_url(_: *const libc::c_char) -> bool;
    fn ftp_index(_: *const libc::c_char, _: *mut url, _: *mut fileinfo) -> uerr_t;
    fn ftp_process_type(_: *const libc::c_char) -> libc::c_char;
    fn ftp_parse_ls(_: *const libc::c_char, _: stype) -> *mut fileinfo;
    fn print_address(_: *const ip_address) -> *const libc::c_char;
    fn ftp_response(_: libc::c_int, _: *mut *mut libc::c_char) -> uerr_t;
    fn ftp_greeting(_: libc::c_int) -> uerr_t;
    fn ftp_login(
        _: libc::c_int,
        _: *const libc::c_char,
        _: *const libc::c_char,
    ) -> uerr_t;
    fn ftp_port(_: libc::c_int, _: *mut libc::c_int) -> uerr_t;
    fn ftp_pasv(_: libc::c_int, _: *mut ip_address, _: *mut libc::c_int) -> uerr_t;
    fn ftp_lprt(_: libc::c_int, _: *mut libc::c_int) -> uerr_t;
    fn ftp_lpsv(_: libc::c_int, _: *mut ip_address, _: *mut libc::c_int) -> uerr_t;
    fn ftp_eprt(_: libc::c_int, _: *mut libc::c_int) -> uerr_t;
    fn ftp_epsv(_: libc::c_int, _: *mut ip_address, _: *mut libc::c_int) -> uerr_t;
    fn ftp_type(_: libc::c_int, _: libc::c_int) -> uerr_t;
    fn ftp_cwd(_: libc::c_int, _: *const libc::c_char) -> uerr_t;
    fn ftp_retr(_: libc::c_int, _: *const libc::c_char) -> uerr_t;
    fn ftp_rest(_: libc::c_int, _: wgint) -> uerr_t;
    fn ftp_list(
        _: libc::c_int,
        _: *const libc::c_char,
        _: bool,
        _: bool,
        _: *mut bool,
    ) -> uerr_t;
    fn ftp_syst(_: libc::c_int, _: *mut stype, _: *mut ustype) -> uerr_t;
    fn ftp_pwd(_: libc::c_int, _: *mut *mut libc::c_char) -> uerr_t;
    fn ftp_size(_: libc::c_int, _: *const libc::c_char, _: *mut wgint) -> uerr_t;
    fn connect_to_host(_: *const libc::c_char, _: libc::c_int) -> libc::c_int;
    fn connect_to_ip(
        _: *const ip_address,
        _: libc::c_int,
        _: *const libc::c_char,
    ) -> libc::c_int;
    fn accept_connection(_: libc::c_int) -> libc::c_int;
    fn socket_ip_address(_: libc::c_int, _: *mut ip_address, _: libc::c_int) -> bool;
    fn retryable_socket_connect_error(_: libc::c_int) -> bool;
    fn fd_errstr(_: libc::c_int) -> *const libc::c_char;
    fn fd_close(_: libc::c_int);
    fn search_netrc(
        _: *const libc::c_char,
        _: *mut *const libc::c_char,
        _: *mut *const libc::c_char,
        _: libc::c_int,
        _: *mut FILE,
    );
    fn downloaded_file(
        _: downloaded_file_t,
        _: *const libc::c_char,
    ) -> downloaded_file_t;
    fn warc_tempfile() -> *mut FILE;
    fn warc_write_resource_record(
        resource_uuid: *const libc::c_char,
        url: *const libc::c_char,
        timestamp_str: *const libc::c_char,
        concurrent_to_uuid: *const libc::c_char,
        ip: *const ip_address,
        content_type: *const libc::c_char,
        body: *mut FILE,
        payload_offset: off_t,
    ) -> bool;
    fn c_strncasecmp(
        s1: *const libc::c_char,
        s2: *const libc::c_char,
        n: size_t,
    ) -> libc::c_int;
    fn set_file_metadata(
        origin_url: *const url,
        referrer_url: *const url,
        fp: *mut FILE,
    ) -> libc::c_int;
}
pub type __uint8_t = libc::c_uchar;
pub type __uint16_t = libc::c_ushort;
pub type __uint32_t = libc::c_uint;
pub type __int64_t = libc::c_long;
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
pub type ino_t = __ino_t;
pub type dev_t = __dev_t;
pub type off_t = __off_t;
pub type ssize_t = __ssize_t;
pub type time_t = __time_t;
pub type size_t = libc::c_ulong;
pub type int64_t = __int64_t;
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
pub type uint8_t = __uint8_t;
pub type uint16_t = __uint16_t;
pub type uint32_t = __uint32_t;
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
pub type C2RustUnnamed_3 = libc::c_uint;
pub const METALINK_METADATA: C2RustUnnamed_3 = 256;
pub const IF_MODIFIED_SINCE: C2RustUnnamed_3 = 128;
pub const TEXTCSS: C2RustUnnamed_3 = 64;
pub const ADDED_HTML_EXTENSION: C2RustUnnamed_3 = 32;
pub const ACCEPTRANGES: C2RustUnnamed_3 = 16;
pub const SEND_NOCACHE: C2RustUnnamed_3 = 8;
pub const HEAD_ONLY: C2RustUnnamed_3 = 4;
pub const RETROKF: C2RustUnnamed_3 = 2;
pub const TEXTHTML: C2RustUnnamed_3 = 1;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct file_stat_s {
    pub access_err: libc::c_int,
    pub st_ino: ino_t,
    pub st_dev: dev_t,
}
pub type file_stats_t = file_stat_s;
pub type url_auth_mode = libc::c_uint;
pub const URL_AUTH_HIDE: url_auth_mode = 2;
pub const URL_AUTH_HIDE_PASSWD: url_auth_mode = 1;
pub const URL_AUTH_SHOW: url_auth_mode = 0;
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
pub type C2RustUnnamed_4 = libc::c_uint;
pub const rb_compressed_gzip: C2RustUnnamed_4 = 8;
pub const rb_chunked_transfer_encoding: C2RustUnnamed_4 = 4;
pub const rb_skip_startpos: C2RustUnnamed_4 = 2;
pub const rb_read_exactly: C2RustUnnamed_4 = 1;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct in6_addr {
    pub __in6_u: C2RustUnnamed_5,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_5 {
    pub __u6_addr8: [uint8_t; 16],
    pub __u6_addr16: [uint16_t; 8],
    pub __u6_addr32: [uint32_t; 4],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct in_addr {
    pub s_addr: in_addr_t,
}
pub type in_addr_t = uint32_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ip_address {
    pub family: libc::c_int,
    pub data: C2RustUnnamed_6,
    pub ipv6_scope: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_6 {
    pub d4: in_addr,
    pub d6: in6_addr,
}
pub type stype = libc::c_uint;
pub const ST_OTHER: stype = 5;
pub const ST_OS400: stype = 4;
pub const ST_MACOS: stype = 3;
pub const ST_WINNT: stype = 2;
pub const ST_VMS: stype = 1;
pub const ST_UNIX: stype = 0;
pub type ustype = libc::c_uint;
pub const UST_OTHER: ustype = 2;
pub const UST_MULTINET: ustype = 1;
pub const UST_TYPE_L8: ustype = 0;
pub type ftype = libc::c_uint;
pub const FT_UNKNOWN: ftype = 3;
pub const FT_SYMLINK: ftype = 2;
pub const FT_DIRECTORY: ftype = 1;
pub const FT_PLAINFILE: ftype = 0;
pub type C2RustUnnamed_7 = libc::c_uint;
pub const GLOB_GETONE: C2RustUnnamed_7 = 2;
pub const GLOB_GETALL: C2RustUnnamed_7 = 1;
pub const GLOB_GLOBALL: C2RustUnnamed_7 = 0;
pub type parsetype = libc::c_uint;
pub const TT_DAY: parsetype = 1;
pub const TT_HOUR_MIN: parsetype = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct fileinfo {
    pub type_0: ftype,
    pub name: *mut libc::c_char,
    pub size: wgint,
    pub tstamp: libc::c_long,
    pub ptype: parsetype,
    pub perms: libc::c_int,
    pub linkto: *mut libc::c_char,
    pub prev: *mut fileinfo,
    pub next: *mut fileinfo,
}
pub type wget_ftp_command = libc::c_uint;
pub const LEAVE_PENDING: wget_ftp_command = 16;
pub const DO_LIST: wget_ftp_command = 8;
pub const DO_RETR: wget_ftp_command = 4;
pub const DO_CWD: wget_ftp_command = 2;
pub const DO_LOGIN: wget_ftp_command = 1;
pub type wget_ftp_fstatus = libc::c_uint;
pub const DATA_CHANNEL_SECURITY: wget_ftp_fstatus = 32;
pub const LIST_AFTER_LIST_A_CHECK_DONE: wget_ftp_fstatus = 16;
pub const AVOID_LIST: wget_ftp_fstatus = 8;
pub const AVOID_LIST_A: wget_ftp_fstatus = 4;
pub const DONE_CWD: wget_ftp_fstatus = 2;
pub const ON_YOUR_OWN: wget_ftp_fstatus = 1;
pub const NOTHING: wget_ftp_fstatus = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ccon {
    pub st: libc::c_int,
    pub cmd: libc::c_int,
    pub csock: libc::c_int,
    pub dltime: libc::c_double,
    pub rs: stype,
    pub rsu: ustype,
    pub id: *mut libc::c_char,
    pub target: *mut libc::c_char,
    pub proxy: *mut url,
}
pub type downloaded_file_t = libc::c_uint;
pub const CHECK_FOR_FILE: downloaded_file_t = 3;
pub const FILE_DOWNLOADED_AND_HTML_EXTENSION_ADDED: downloaded_file_t = 2;
pub const FILE_DOWNLOADED_NORMALLY: downloaded_file_t = 1;
pub const FILE_NOT_ALREADY_DOWNLOADED: downloaded_file_t = 0;
pub const ENDPOINT_PEER: C2RustUnnamed_9 = 1;
pub const E_HOST: C2RustUnnamed_8 = -100;
pub type C2RustUnnamed_8 = libc::c_int;
pub type C2RustUnnamed_9 = libc::c_uint;
pub const ENDPOINT_LOCAL: C2RustUnnamed_9 = 0;
#[inline]
unsafe extern "C" fn stat(
    mut __path: *const libc::c_char,
    mut __statbuf: *mut stat,
) -> libc::c_int {
    return __xstat(1 as libc::c_int, __path, __statbuf);
}
#[inline]
unsafe extern "C" fn lstat(
    mut __path: *const libc::c_char,
    mut __statbuf: *mut stat,
) -> libc::c_int {
    return __lxstat(1 as libc::c_int, __path, __statbuf);
}
#[inline]
unsafe extern "C" fn c_isalpha(mut c: libc::c_int) -> bool {
    match c {
        97 | 98 | 99 | 100 | 101 | 102 | 103 | 104 | 105 | 106 | 107 | 108 | 109 | 110
        | 111 | 112 | 113 | 114 | 115 | 116 | 117 | 118 | 119 | 120 | 121 | 122 | 65 | 66
        | 67 | 68 | 69 | 70 | 71 | 72 | 73 | 74 | 75 | 76 | 77 | 78 | 79 | 80 | 81 | 82
        | 83 | 84 | 85 | 86 | 87 | 88 | 89 | 90 => return 1 as libc::c_int != 0,
        _ => return 0 as libc::c_int != 0,
    };
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
#[inline]
unsafe extern "C" fn getline(
    mut __lineptr: *mut *mut libc::c_char,
    mut __n: *mut size_t,
    mut __stream: *mut FILE,
) -> __ssize_t {
    return __getdelim(__lineptr, __n, '\n' as i32, __stream);
}
unsafe extern "C" fn ftp_expected_bytes(mut s: *const libc::c_char) -> wgint {
    let mut res: wgint = 0;
    loop {
        while *s as libc::c_int != 0 && *s as libc::c_int != '(' as i32 {
            s = s.offset(1);
            s;
        }
        if *s == 0 {
            return 0 as libc::c_int as wgint;
        }
        s = s.offset(1);
        s;
        res = rpl_strtoll(
            s,
            &mut s as *mut *const libc::c_char as *mut *mut libc::c_char,
            10 as libc::c_int,
        ) as wgint;
        if *s == 0 {
            return 0 as libc::c_int as wgint;
        }
        while *s as libc::c_int != 0 && c_isspace(*s as libc::c_int) as libc::c_int != 0
        {
            s = s.offset(1);
            s;
        }
        if *s == 0 {
            return 0 as libc::c_int as wgint;
        }
        if c_tolower(*s as libc::c_int) != 'b' as i32 {
            continue;
        }
        if !(c_strncasecmp(
            s,
            b"byte\0" as *const u8 as *const libc::c_char,
            4 as libc::c_int as size_t,
        ) != 0)
        {
            break;
        }
    }
    return res;
}
unsafe extern "C" fn ftp_do_pasv(
    mut csock: libc::c_int,
    mut addr: *mut ip_address,
    mut port: *mut libc::c_int,
) -> uerr_t {
    let mut err: uerr_t = NOCONERROR;
    if !socket_ip_address(csock, addr, ENDPOINT_PEER as libc::c_int) {
        abort();
    }
    match (*addr).family {
        2 => {
            if !opt.server_response {
                logputs(
                    LOG_VERBOSE,
                    b"==> PASV ... \0" as *const u8 as *const libc::c_char,
                );
            }
            err = ftp_pasv(csock, addr, port);
        }
        10 => {
            if !opt.server_response {
                logputs(
                    LOG_VERBOSE,
                    b"==> EPSV ... \0" as *const u8 as *const libc::c_char,
                );
            }
            err = ftp_epsv(csock, addr, port);
            if err as libc::c_uint == FTPNOPASV as libc::c_int as libc::c_uint {
                if !opt.server_response {
                    logputs(
                        LOG_VERBOSE,
                        b"==> LPSV ... \0" as *const u8 as *const libc::c_char,
                    );
                }
                err = ftp_lpsv(csock, addr, port);
            }
        }
        _ => {
            abort();
        }
    }
    return err;
}
unsafe extern "C" fn ftp_do_port(
    mut csock: libc::c_int,
    mut local_sock: *mut libc::c_int,
) -> uerr_t {
    let mut err: uerr_t = NOCONERROR;
    let mut cip: ip_address = ip_address {
        family: 0,
        data: C2RustUnnamed_6 {
            d4: in_addr { s_addr: 0 },
        },
        ipv6_scope: 0,
    };
    if !socket_ip_address(csock, &mut cip, ENDPOINT_PEER as libc::c_int) {
        abort();
    }
    match cip.family {
        2 => {
            if !opt.server_response {
                logputs(
                    LOG_VERBOSE,
                    b"==> PORT ... \0" as *const u8 as *const libc::c_char,
                );
            }
            err = ftp_port(csock, local_sock);
        }
        10 => {
            if !opt.server_response {
                logputs(
                    LOG_VERBOSE,
                    b"==> EPRT ... \0" as *const u8 as *const libc::c_char,
                );
            }
            err = ftp_eprt(csock, local_sock);
            if err as libc::c_uint == FTPPORTERR as libc::c_int as libc::c_uint {
                if !opt.server_response {
                    logputs(
                        LOG_VERBOSE,
                        b"==> LPRT ... \0" as *const u8 as *const libc::c_char,
                    );
                }
                err = ftp_lprt(csock, local_sock);
            }
        }
        _ => {
            abort();
        }
    }
    return err;
}
unsafe extern "C" fn print_length(
    mut size: wgint,
    mut start: wgint,
    mut authoritative: bool,
) {
    logprintf(
        LOG_VERBOSE,
        dcgettext(
            0 as *const libc::c_char,
            b"Length: %s\0" as *const u8 as *const libc::c_char,
            5 as libc::c_int,
        ),
        number_to_static_string(size),
    );
    if size >= 1024 as libc::c_int as libc::c_long {
        logprintf(
            LOG_VERBOSE,
            b" (%s)\0" as *const u8 as *const libc::c_char,
            human_readable(size, 10 as libc::c_int, 1 as libc::c_int),
        );
    }
    if start > 0 as libc::c_int as libc::c_long {
        if size - start >= 1024 as libc::c_int as libc::c_long {
            logprintf(
                LOG_VERBOSE,
                dcgettext(
                    0 as *const libc::c_char,
                    b", %s (%s) remaining\0" as *const u8 as *const libc::c_char,
                    5 as libc::c_int,
                ),
                number_to_static_string(size - start),
                human_readable(size - start, 10 as libc::c_int, 1 as libc::c_int),
            );
        } else {
            logprintf(
                LOG_VERBOSE,
                dcgettext(
                    0 as *const libc::c_char,
                    b", %s remaining\0" as *const u8 as *const libc::c_char,
                    5 as libc::c_int,
                ),
                number_to_static_string(size - start),
            );
        }
    }
    logputs(
        LOG_VERBOSE,
        if !authoritative {
            dcgettext(
                0 as *const libc::c_char,
                b" (unauthoritative)\n\0" as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ) as *const libc::c_char
        } else {
            b"\n\0" as *const u8 as *const libc::c_char
        },
    );
}
unsafe extern "C" fn get_ftp_greeting(
    mut csock: libc::c_int,
    mut con: *mut ccon,
) -> uerr_t {
    let mut err: uerr_t = NOCONERROR;
    err = ftp_greeting(csock);
    if err as libc::c_uint != FTPOK as libc::c_int as libc::c_uint {
        logputs(
            LOG_NOTQUIET,
            b"Error in server response. Closing.\n\0" as *const u8 as *const libc::c_char,
        );
        fd_close(csock);
        (*con).csock = -(1 as libc::c_int);
    }
    return err;
}
unsafe extern "C" fn getftp(
    mut u: *mut url,
    mut original_url: *mut url,
    mut passed_expected_bytes: wgint,
    mut qtyread: *mut wgint,
    mut restval: wgint,
    mut con: *mut ccon,
    mut count: libc::c_int,
    mut last_expected_bytes: *mut wgint,
    mut warc_tmp: *mut FILE,
) -> uerr_t {
    let mut csock: libc::c_int = 0;
    let mut dtsock: libc::c_int = 0;
    let mut local_sock: libc::c_int = 0;
    let mut res: libc::c_int = 0;
    let mut err: uerr_t = RETROK;
    let mut fp: *mut FILE = 0 as *mut FILE;
    let mut respline: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut tms: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut user: *const libc::c_char = 0 as *const libc::c_char;
    let mut passwd: *const libc::c_char = 0 as *const libc::c_char;
    let mut tmrate: *const libc::c_char = 0 as *const libc::c_char;
    let mut cmd: libc::c_int = (*con).cmd;
    let mut expected_bytes: wgint = 0 as libc::c_int as wgint;
    let mut got_expected_bytes: bool = 0 as libc::c_int != 0;
    let mut rest_failed: bool = 0 as libc::c_int != 0;
    let mut flags: libc::c_int = 0;
    let mut rd_size: wgint = 0;
    let mut previous_rd_size: wgint = 0 as libc::c_int as wgint;
    let mut type_char: libc::c_char = 0;
    let mut try_again: bool = false;
    let mut list_a_used: bool = 0 as libc::c_int != 0;
    *qtyread = restval;
    if !((*u).user).is_null() {
        user = (*u).user;
    } else if !(opt.user).is_null()
        && (!(opt.use_askpass).is_null() || opt.ask_passwd as libc::c_int != 0)
    {
        user = opt.user;
    } else if !(opt.ftp_user).is_null() {
        user = opt.ftp_user;
    } else if !(opt.user).is_null() {
        user = opt.user;
    } else {
        user = 0 as *const libc::c_char;
    }
    if !((*u).passwd).is_null() {
        passwd = (*u).passwd;
    } else if !(opt.passwd).is_null()
        && (!(opt.use_askpass).is_null() || opt.ask_passwd as libc::c_int != 0)
    {
        passwd = opt.passwd;
    } else if !(opt.ftp_passwd).is_null() {
        passwd = opt.ftp_passwd;
    } else if !(opt.passwd).is_null() {
        passwd = opt.passwd;
    } else {
        passwd = 0 as *const libc::c_char;
    }
    if opt.netrc as libc::c_int != 0 && (user.is_null() || passwd.is_null()) {
        search_netrc(
            (*u).host,
            &mut user as *mut *const libc::c_char,
            &mut passwd as *mut *const libc::c_char,
            1 as libc::c_int,
            0 as *mut FILE,
        );
    }
    if user.is_null() {
        user = b"anonymous\0" as *const u8 as *const libc::c_char;
    }
    if passwd.is_null() {
        passwd = b"-wget@\0" as *const u8 as *const libc::c_char;
    }
    dtsock = -(1 as libc::c_int);
    local_sock = -(1 as libc::c_int);
    (*con).dltime = 0 as libc::c_int as libc::c_double;
    if cmd & DO_LOGIN as libc::c_int == 0 {
        csock = (*con).csock;
    } else {
        let mut host: *mut libc::c_char = if !((*con).proxy).is_null() {
            (*(*con).proxy).host
        } else {
            (*u).host
        };
        let mut port: libc::c_int = if !((*con).proxy).is_null() {
            (*(*con).proxy).port
        } else {
            (*u).port
        };
        csock = connect_to_host(host, port);
        if csock == E_HOST as libc::c_int {
            return HOSTERR
        } else if csock < 0 as libc::c_int {
            return (if retryable_socket_connect_error(*__errno_location()) as libc::c_int
                != 0
            {
                CONERROR as libc::c_int
            } else {
                CONIMPOSSIBLE as libc::c_int
            }) as uerr_t
        }
        if cmd & LEAVE_PENDING as libc::c_int != 0 {
            (*con).csock = csock;
        } else {
            (*con).csock = -(1 as libc::c_int);
        }
        err = get_ftp_greeting(csock, con);
        if err as libc::c_uint != FTPOK as libc::c_int as libc::c_uint {
            return err;
        }
        logprintf(
            LOG_VERBOSE,
            dcgettext(
                0 as *const libc::c_char,
                b"Logging in as %s ... \0" as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ),
            quotearg_style(escape_quoting_style, user),
        );
        if opt.server_response {
            logputs(LOG_ALWAYS, b"\n\0" as *const u8 as *const libc::c_char);
        }
        if !((*con).proxy).is_null() {
            let mut logname: *mut libc::c_char = concat_strings(
                user,
                b"@\0" as *const u8 as *const libc::c_char,
                (*u).host,
                0 as *mut libc::c_char,
            );
            err = ftp_login(csock, logname, passwd);
            rpl_free(logname as *mut libc::c_void);
            logname = 0 as *mut libc::c_char;
        } else {
            err = ftp_login(csock, user, passwd);
        }
        match err as libc::c_uint {
            14 => {
                logputs(LOG_VERBOSE, b"\n\0" as *const u8 as *const libc::c_char);
                logputs(
                    LOG_NOTQUIET,
                    dcgettext(
                        0 as *const libc::c_char,
                        b"Error in server response, closing control connection.\n\0"
                            as *const u8 as *const libc::c_char,
                        5 as libc::c_int,
                    ),
                );
                fd_close(csock);
                (*con).csock = -(1 as libc::c_int);
                return err;
            }
            15 => {
                logputs(LOG_VERBOSE, b"\n\0" as *const u8 as *const libc::c_char);
                logputs(
                    LOG_NOTQUIET,
                    dcgettext(
                        0 as *const libc::c_char,
                        b"Error in server greeting.\n\0" as *const u8
                            as *const libc::c_char,
                        5 as libc::c_int,
                    ),
                );
                fd_close(csock);
                (*con).csock = -(1 as libc::c_int);
                return err;
            }
            44 => {
                logputs(LOG_VERBOSE, b"\n\0" as *const u8 as *const libc::c_char);
                logputs(
                    LOG_NOTQUIET,
                    dcgettext(
                        0 as *const libc::c_char,
                        b"Write failed, closing control connection.\n\0" as *const u8
                            as *const libc::c_char,
                        5 as libc::c_int,
                    ),
                );
                fd_close(csock);
                (*con).csock = -(1 as libc::c_int);
                return err;
            }
            9 => {
                logputs(LOG_VERBOSE, b"\n\0" as *const u8 as *const libc::c_char);
                logputs(
                    LOG_NOTQUIET,
                    dcgettext(
                        0 as *const libc::c_char,
                        b"The server refuses login.\n\0" as *const u8
                            as *const libc::c_char,
                        5 as libc::c_int,
                    ),
                );
                fd_close(csock);
                (*con).csock = -(1 as libc::c_int);
                return FTPLOGREFUSED;
            }
            8 => {
                logputs(LOG_VERBOSE, b"\n\0" as *const u8 as *const libc::c_char);
                logputs(
                    LOG_NOTQUIET,
                    dcgettext(
                        0 as *const libc::c_char,
                        b"Login incorrect.\n\0" as *const u8 as *const libc::c_char,
                        5 as libc::c_int,
                    ),
                );
                fd_close(csock);
                (*con).csock = -(1 as libc::c_int);
                return FTPLOGINC;
            }
            7 => {
                if !opt.server_response {
                    logputs(
                        LOG_VERBOSE,
                        dcgettext(
                            0 as *const libc::c_char,
                            b"Logged in!\n\0" as *const u8 as *const libc::c_char,
                            5 as libc::c_int,
                        ),
                    );
                }
            }
            _ => {
                abort();
            }
        }
        if !opt.server_response {
            logprintf(
                LOG_VERBOSE,
                b"==> SYST ... \0" as *const u8 as *const libc::c_char,
            );
        }
        err = ftp_syst(csock, &mut (*con).rs, &mut (*con).rsu);
        match err as libc::c_uint {
            14 => {
                logputs(LOG_VERBOSE, b"\n\0" as *const u8 as *const libc::c_char);
                logputs(
                    LOG_NOTQUIET,
                    dcgettext(
                        0 as *const libc::c_char,
                        b"Error in server response, closing control connection.\n\0"
                            as *const u8 as *const libc::c_char,
                        5 as libc::c_int,
                    ),
                );
                fd_close(csock);
                (*con).csock = -(1 as libc::c_int);
                return err;
            }
            15 => {
                logputs(LOG_VERBOSE, b"\n\0" as *const u8 as *const libc::c_char);
                logputs(
                    LOG_NOTQUIET,
                    dcgettext(
                        0 as *const libc::c_char,
                        b"Server error, can't determine system type.\n\0" as *const u8
                            as *const libc::c_char,
                        5 as libc::c_int,
                    ),
                );
            }
            7 => {}
            _ => {
                abort();
            }
        }
        if !opt.server_response
            && err as libc::c_uint != FTPSRVERR as libc::c_int as libc::c_uint
        {
            logputs(
                LOG_VERBOSE,
                dcgettext(
                    0 as *const libc::c_char,
                    b"done.    \0" as *const u8 as *const libc::c_char,
                    5 as libc::c_int,
                ),
            );
        }
        match (*con).rs as libc::c_uint {
            1 => {
                if opt.debug as libc::c_long != 0 {
                    debug_logprintf(
                        b"\nVMS: I know it and I will use \"LIST\" as standard list command\n\0"
                            as *const u8 as *const libc::c_char,
                    );
                }
                (*con).st |= LIST_AFTER_LIST_A_CHECK_DONE as libc::c_int;
                (*con).st |= AVOID_LIST_A as libc::c_int;
            }
            0 => {
                if (*con).rsu as libc::c_uint
                    == UST_MULTINET as libc::c_int as libc::c_uint
                {
                    if opt.debug as libc::c_long != 0 {
                        debug_logprintf(
                            b"\nUNIX MultiNet: I know it and I will use \"LIST\" as standard list command\n\0"
                                as *const u8 as *const libc::c_char,
                        );
                    }
                    (*con).st |= LIST_AFTER_LIST_A_CHECK_DONE as libc::c_int;
                    (*con).st |= AVOID_LIST_A as libc::c_int;
                } else if (*con).rsu as libc::c_uint
                    == UST_TYPE_L8 as libc::c_int as libc::c_uint
                {
                    if opt.debug as libc::c_long != 0 {
                        debug_logprintf(
                            b"\nUNIX TYPE L8: I know it and I will use \"LIST -a\" as standard list command\n\0"
                                as *const u8 as *const libc::c_char,
                        );
                    }
                    (*con).st |= LIST_AFTER_LIST_A_CHECK_DONE as libc::c_int;
                    (*con).st |= AVOID_LIST as libc::c_int;
                }
            }
            _ => {}
        }
        if !opt.server_response {
            logprintf(
                LOG_VERBOSE,
                b"==> PWD ... \0" as *const u8 as *const libc::c_char,
            );
        }
        err = ftp_pwd(csock, &mut (*con).id);
        match err as libc::c_uint {
            14 => {
                logputs(LOG_VERBOSE, b"\n\0" as *const u8 as *const libc::c_char);
                logputs(
                    LOG_NOTQUIET,
                    dcgettext(
                        0 as *const libc::c_char,
                        b"Error in server response, closing control connection.\n\0"
                            as *const u8 as *const libc::c_char,
                        5 as libc::c_int,
                    ),
                );
                fd_close(csock);
                (*con).csock = -(1 as libc::c_int);
                return err;
            }
            15 => {
                rpl_free((*con).id as *mut libc::c_void);
                (*con).id = 0 as *mut libc::c_char;
                (*con).id = xstrdup(b"/\0" as *const u8 as *const libc::c_char);
            }
            7 => {}
            _ => {
                abort();
            }
        }
        if !opt.server_response {
            logputs(
                LOG_VERBOSE,
                dcgettext(
                    0 as *const libc::c_char,
                    b"done.\n\0" as *const u8 as *const libc::c_char,
                    5 as libc::c_int,
                ),
            );
        }
        type_char = ftp_process_type((*u).params);
        if !opt.server_response {
            logprintf(
                LOG_VERBOSE,
                b"==> TYPE %c ... \0" as *const u8 as *const libc::c_char,
                type_char as libc::c_int,
            );
        }
        err = ftp_type(csock, type_char as libc::c_int);
        match err as libc::c_uint {
            14 => {
                logputs(LOG_VERBOSE, b"\n\0" as *const u8 as *const libc::c_char);
                logputs(
                    LOG_NOTQUIET,
                    dcgettext(
                        0 as *const libc::c_char,
                        b"Error in server response, closing control connection.\n\0"
                            as *const u8 as *const libc::c_char,
                        5 as libc::c_int,
                    ),
                );
                fd_close(csock);
                (*con).csock = -(1 as libc::c_int);
                return err;
            }
            44 => {
                logputs(LOG_VERBOSE, b"\n\0" as *const u8 as *const libc::c_char);
                logputs(
                    LOG_NOTQUIET,
                    dcgettext(
                        0 as *const libc::c_char,
                        b"Write failed, closing control connection.\n\0" as *const u8
                            as *const libc::c_char,
                        5 as libc::c_int,
                    ),
                );
                fd_close(csock);
                (*con).csock = -(1 as libc::c_int);
                return err;
            }
            13 => {
                logputs(LOG_VERBOSE, b"\n\0" as *const u8 as *const libc::c_char);
                logprintf(
                    LOG_NOTQUIET,
                    dcgettext(
                        0 as *const libc::c_char,
                        b"Unknown type `%c', closing control connection.\n\0"
                            as *const u8 as *const libc::c_char,
                        5 as libc::c_int,
                    ),
                    type_char as libc::c_int,
                );
                fd_close(csock);
                (*con).csock = -(1 as libc::c_int);
                return err;
            }
            7 => {}
            _ => {
                abort();
            }
        }
        if !opt.server_response {
            logputs(
                LOG_VERBOSE,
                dcgettext(
                    0 as *const libc::c_char,
                    b"done.  \0" as *const u8 as *const libc::c_char,
                    5 as libc::c_int,
                ),
            );
        }
    }
    if cmd & DO_CWD as libc::c_int != 0 {
        if *(*u).dir == 0 {
            logputs(
                LOG_VERBOSE,
                dcgettext(
                    0 as *const libc::c_char,
                    b"==> CWD not needed.\n\0" as *const u8 as *const libc::c_char,
                    5 as libc::c_int,
                ),
            );
        } else {
            let mut targ: *const libc::c_char = 0 as *const libc::c_char;
            let mut target: *mut libc::c_char = (*u).dir;
            let mut targetbuf: [libc::c_char; 1024] = [0; 1024];
            let mut cwd_count: libc::c_int = 0;
            let mut cwd_end: libc::c_int = 0;
            let mut cwd_start: libc::c_int = 0;
            if opt.debug as libc::c_long != 0 {
                debug_logprintf(
                    b"changing working directory\n\0" as *const u8 as *const libc::c_char,
                );
            }
            if *target.offset(0 as libc::c_int as isize) as libc::c_int != '/' as i32
                && !((*con).rs as libc::c_uint != ST_UNIX as libc::c_int as libc::c_uint
                    && c_isalpha(
                        *target.offset(0 as libc::c_int as isize) as libc::c_int,
                    ) as libc::c_int != 0
                    && *target.offset(1 as libc::c_int as isize) as libc::c_int
                        == ':' as i32)
                && (*con).rs as libc::c_uint != ST_OS400 as libc::c_int as libc::c_uint
                && (*con).rs as libc::c_uint != ST_VMS as libc::c_int as libc::c_uint
            {
                let mut ntarget: *mut libc::c_char = 0 as *mut libc::c_char;
                let mut p: *mut libc::c_char = 0 as *mut libc::c_char;
                let mut idlen: size_t = strlen((*con).id);
                let mut len: size_t = 0;
                while idlen > 0 as libc::c_int as libc::c_ulong
                    && *((*con).id)
                        .offset(
                            idlen.wrapping_sub(1 as libc::c_int as libc::c_ulong)
                                as isize,
                        ) as libc::c_int == '/' as i32
                {
                    idlen = idlen.wrapping_sub(1);
                    idlen;
                }
                len = idlen
                    .wrapping_add(1 as libc::c_int as libc::c_ulong)
                    .wrapping_add(strlen(target));
                if len < ::std::mem::size_of::<[libc::c_char; 1024]>() as libc::c_ulong {
                    ntarget = targetbuf.as_mut_ptr();
                    p = ntarget;
                } else {
                    ntarget = xmalloc(
                        len.wrapping_add(1 as libc::c_int as libc::c_ulong),
                    ) as *mut libc::c_char;
                    p = ntarget;
                }
                memcpy(p as *mut libc::c_void, (*con).id as *const libc::c_void, idlen);
                p = p.offset(idlen as isize);
                let fresh0 = p;
                p = p.offset(1);
                *fresh0 = '/' as i32 as libc::c_char;
                strcpy(p, target);
                if opt.debug as libc::c_long != 0 {
                    debug_logprintf(
                        b"Prepended initial PWD to relative path:\n\0" as *const u8
                            as *const libc::c_char,
                    );
                }
                if opt.debug as libc::c_long != 0 {
                    debug_logprintf(
                        b"   pwd: '%s'\n   old: '%s'\n  new: '%s'\n\0" as *const u8
                            as *const libc::c_char,
                        (*con).id,
                        target,
                        ntarget,
                    );
                }
                target = ntarget;
            }
            if (*con).rs as libc::c_uint == ST_VMS as libc::c_int as libc::c_uint
                && *target.offset(0 as libc::c_int as isize) as libc::c_int != '/' as i32
            {
                cwd_start = 0 as libc::c_int;
                if opt.debug as libc::c_long != 0 {
                    debug_logprintf(
                        b"Using two-step CWD for relative path.\n\0" as *const u8
                            as *const libc::c_char,
                    );
                }
            } else {
                cwd_start = 1 as libc::c_int;
            }
            if (*con).rs as libc::c_uint == ST_VMS as libc::c_int as libc::c_uint
                && !(strchr(target, '/' as i32)).is_null()
            {
                cwd_end = 3 as libc::c_int;
                if opt.debug as libc::c_long != 0 {
                    debug_logprintf(
                        b"Using extra \"CWD []\" step for VMS server.\n\0" as *const u8
                            as *const libc::c_char,
                    );
                }
            } else {
                cwd_end = 2 as libc::c_int;
            }
            cwd_count = cwd_start;
            while cwd_count < cwd_end {
                match cwd_count {
                    0 => {
                        targ = (*con).id;
                    }
                    1 => {
                        targ = target;
                    }
                    2 => {
                        targ = b"[]\0" as *const u8 as *const libc::c_char;
                    }
                    _ => {
                        logprintf(
                            LOG_ALWAYS,
                            dcgettext(
                                0 as *const libc::c_char,
                                b"Logically impossible section reached in getftp()\0"
                                    as *const u8 as *const libc::c_char,
                                5 as libc::c_int,
                            ),
                        );
                        logprintf(
                            LOG_ALWAYS,
                            dcgettext(
                                0 as *const libc::c_char,
                                b"cwd_count: %d\ncwd_start: %d\ncwd_end: %d\n\0"
                                    as *const u8 as *const libc::c_char,
                                5 as libc::c_int,
                            ),
                            cwd_count,
                            cwd_start,
                            cwd_end,
                        );
                        abort();
                    }
                }
                if !opt.server_response {
                    logprintf(
                        LOG_VERBOSE,
                        b"==> CWD (%d) %s ... \0" as *const u8 as *const libc::c_char,
                        cwd_count,
                        quotearg_style(escape_quoting_style, target),
                    );
                }
                err = ftp_cwd(csock, targ);
                match err as libc::c_uint {
                    14 => {
                        logputs(
                            LOG_VERBOSE,
                            b"\n\0" as *const u8 as *const libc::c_char,
                        );
                        logputs(
                            LOG_NOTQUIET,
                            dcgettext(
                                0 as *const libc::c_char,
                                b"Error in server response, closing control connection.\n\0"
                                    as *const u8 as *const libc::c_char,
                                5 as libc::c_int,
                            ),
                        );
                        fd_close(csock);
                        (*con).csock = -(1 as libc::c_int);
                        return err;
                    }
                    44 => {
                        logputs(
                            LOG_VERBOSE,
                            b"\n\0" as *const u8 as *const libc::c_char,
                        );
                        logputs(
                            LOG_NOTQUIET,
                            dcgettext(
                                0 as *const libc::c_char,
                                b"Write failed, closing control connection.\n\0"
                                    as *const u8 as *const libc::c_char,
                                5 as libc::c_int,
                            ),
                        );
                        fd_close(csock);
                        (*con).csock = -(1 as libc::c_int);
                        return err;
                    }
                    12 => {
                        logputs(
                            LOG_VERBOSE,
                            b"\n\0" as *const u8 as *const libc::c_char,
                        );
                        logprintf(
                            LOG_NOTQUIET,
                            dcgettext(
                                0 as *const libc::c_char,
                                b"No such directory %s.\n\n\0" as *const u8
                                    as *const libc::c_char,
                                5 as libc::c_int,
                            ),
                            quote((*u).dir),
                        );
                        fd_close(csock);
                        (*con).csock = -(1 as libc::c_int);
                        return err;
                    }
                    7 => {}
                    _ => {
                        abort();
                    }
                }
                if !opt.server_response {
                    logputs(
                        LOG_VERBOSE,
                        dcgettext(
                            0 as *const libc::c_char,
                            b"done.\n\0" as *const u8 as *const libc::c_char,
                            5 as libc::c_int,
                        ),
                    );
                }
                cwd_count += 1;
                cwd_count;
            }
        }
    } else {
        logputs(
            LOG_VERBOSE,
            dcgettext(
                0 as *const libc::c_char,
                b"==> CWD not required.\n\0" as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ),
        );
    }
    if cmd & DO_RETR as libc::c_int != 0
        && passed_expected_bytes == 0 as libc::c_int as libc::c_long
    {
        if opt.verbose != 0 {
            if !opt.server_response {
                logprintf(
                    LOG_VERBOSE,
                    b"==> SIZE %s ... \0" as *const u8 as *const libc::c_char,
                    quotearg_style(escape_quoting_style, (*u).file),
                );
            }
        }
        err = ftp_size(csock, (*u).file, &mut expected_bytes);
        match err as libc::c_uint {
            14 | 15 => {
                logputs(LOG_VERBOSE, b"\n\0" as *const u8 as *const libc::c_char);
                logputs(
                    LOG_NOTQUIET,
                    dcgettext(
                        0 as *const libc::c_char,
                        b"Error in server response, closing control connection.\n\0"
                            as *const u8 as *const libc::c_char,
                        5 as libc::c_int,
                    ),
                );
                fd_close(csock);
                (*con).csock = -(1 as libc::c_int);
                return err;
            }
            7 => {
                got_expected_bytes = 1 as libc::c_int != 0;
            }
            _ => {
                abort();
            }
        }
        if !opt.server_response {
            logprintf(
                LOG_VERBOSE,
                b"%s\n\0" as *const u8 as *const libc::c_char,
                if expected_bytes != 0 {
                    number_to_static_string(expected_bytes)
                } else {
                    dcgettext(
                        0 as *const libc::c_char,
                        b"done.\n\0" as *const u8 as *const libc::c_char,
                        5 as libc::c_int,
                    )
                },
            );
        }
    }
    if cmd & DO_RETR as libc::c_int != 0 && restval > 0 as libc::c_int as libc::c_long
        && restval == expected_bytes
    {
        logputs(
            LOG_VERBOSE,
            dcgettext(
                0 as *const libc::c_char,
                b"File has already been retrieved.\n\0" as *const u8
                    as *const libc::c_char,
                5 as libc::c_int,
            ),
        );
        fd_close(csock);
        (*con).csock = -(1 as libc::c_int);
        return RETRFINISHED;
    }
    loop {
        try_again = 0 as libc::c_int != 0;
        if cmd & (DO_LIST as libc::c_int | DO_RETR as libc::c_int) != 0 {
            if opt.ftp_pasv {
                let mut passive_addr: ip_address = ip_address {
                    family: 0,
                    data: C2RustUnnamed_6 {
                        d4: in_addr { s_addr: 0 },
                    },
                    ipv6_scope: 0,
                };
                let mut passive_port: libc::c_int = 0;
                err = ftp_do_pasv(csock, &mut passive_addr, &mut passive_port);
                match err as libc::c_uint {
                    14 => {
                        logputs(
                            LOG_VERBOSE,
                            b"\n\0" as *const u8 as *const libc::c_char,
                        );
                        logputs(
                            LOG_NOTQUIET,
                            dcgettext(
                                0 as *const libc::c_char,
                                b"Error in server response, closing control connection.\n\0"
                                    as *const u8 as *const libc::c_char,
                                5 as libc::c_int,
                            ),
                        );
                        fd_close(csock);
                        (*con).csock = -(1 as libc::c_int);
                        return err;
                    }
                    44 => {
                        logputs(
                            LOG_VERBOSE,
                            b"\n\0" as *const u8 as *const libc::c_char,
                        );
                        logputs(
                            LOG_NOTQUIET,
                            dcgettext(
                                0 as *const libc::c_char,
                                b"Write failed, closing control connection.\n\0"
                                    as *const u8 as *const libc::c_char,
                                5 as libc::c_int,
                            ),
                        );
                        fd_close(csock);
                        (*con).csock = -(1 as libc::c_int);
                        return err;
                    }
                    29 => {
                        logputs(
                            LOG_VERBOSE,
                            b"\n\0" as *const u8 as *const libc::c_char,
                        );
                        logputs(
                            LOG_NOTQUIET,
                            dcgettext(
                                0 as *const libc::c_char,
                                b"Cannot initiate PASV transfer.\n\0" as *const u8
                                    as *const libc::c_char,
                                5 as libc::c_int,
                            ),
                        );
                    }
                    28 => {
                        logputs(
                            LOG_VERBOSE,
                            b"\n\0" as *const u8 as *const libc::c_char,
                        );
                        logputs(
                            LOG_NOTQUIET,
                            dcgettext(
                                0 as *const libc::c_char,
                                b"Cannot parse PASV response.\n\0" as *const u8
                                    as *const libc::c_char,
                                5 as libc::c_int,
                            ),
                        );
                    }
                    7 => {}
                    _ => {
                        abort();
                    }
                }
                if err as libc::c_uint == FTPOK as libc::c_int as libc::c_uint {
                    if opt.debug as libc::c_long != 0 {
                        debug_logprintf(
                            b"trying to connect to %s port %d\n\0" as *const u8
                                as *const libc::c_char,
                            print_address(&mut passive_addr),
                            passive_port,
                        );
                    }
                    dtsock = connect_to_ip(
                        &mut passive_addr,
                        passive_port,
                        0 as *const libc::c_char,
                    );
                    if dtsock < 0 as libc::c_int {
                        let mut save_errno: libc::c_int = *__errno_location();
                        fd_close(csock);
                        (*con).csock = -(1 as libc::c_int);
                        logprintf(
                            LOG_VERBOSE,
                            dcgettext(
                                0 as *const libc::c_char,
                                b"couldn't connect to %s port %d: %s\n\0" as *const u8
                                    as *const libc::c_char,
                                5 as libc::c_int,
                            ),
                            print_address(&mut passive_addr),
                            passive_port,
                            strerror(save_errno),
                        );
                        return (if retryable_socket_connect_error(save_errno)
                            as libc::c_int != 0
                        {
                            CONERROR as libc::c_int
                        } else {
                            CONIMPOSSIBLE as libc::c_int
                        }) as uerr_t;
                    }
                    if !opt.server_response {
                        logputs(
                            LOG_VERBOSE,
                            dcgettext(
                                0 as *const libc::c_char,
                                b"done.    \0" as *const u8 as *const libc::c_char,
                                5 as libc::c_int,
                            ),
                        );
                    }
                } else {
                    return err
                }
            } else {
                err = ftp_do_port(csock, &mut local_sock);
                match err as libc::c_uint {
                    14 => {
                        logputs(
                            LOG_VERBOSE,
                            b"\n\0" as *const u8 as *const libc::c_char,
                        );
                        logputs(
                            LOG_NOTQUIET,
                            dcgettext(
                                0 as *const libc::c_char,
                                b"Error in server response, closing control connection.\n\0"
                                    as *const u8 as *const libc::c_char,
                                5 as libc::c_int,
                            ),
                        );
                        fd_close(csock);
                        (*con).csock = -(1 as libc::c_int);
                        fd_close(dtsock);
                        fd_close(local_sock);
                        return err;
                    }
                    44 => {
                        logputs(
                            LOG_VERBOSE,
                            b"\n\0" as *const u8 as *const libc::c_char,
                        );
                        logputs(
                            LOG_NOTQUIET,
                            dcgettext(
                                0 as *const libc::c_char,
                                b"Write failed, closing control connection.\n\0"
                                    as *const u8 as *const libc::c_char,
                                5 as libc::c_int,
                            ),
                        );
                        fd_close(csock);
                        (*con).csock = -(1 as libc::c_int);
                        fd_close(dtsock);
                        fd_close(local_sock);
                        return err;
                    }
                    2 => {
                        logputs(
                            LOG_VERBOSE,
                            b"\n\0" as *const u8 as *const libc::c_char,
                        );
                        logprintf(
                            LOG_NOTQUIET,
                            b"socket: %s\n\0" as *const u8 as *const libc::c_char,
                            strerror(*__errno_location()),
                        );
                        fd_close(csock);
                        (*con).csock = -(1 as libc::c_int);
                        fd_close(dtsock);
                        fd_close(local_sock);
                        return err;
                    }
                    11 => {
                        logputs(
                            LOG_VERBOSE,
                            b"\n\0" as *const u8 as *const libc::c_char,
                        );
                        logprintf(
                            LOG_NOTQUIET,
                            dcgettext(
                                0 as *const libc::c_char,
                                b"Bind error (%s).\n\0" as *const u8 as *const libc::c_char,
                                5 as libc::c_int,
                            ),
                            strerror(*__errno_location()),
                        );
                        fd_close(dtsock);
                        return err;
                    }
                    10 => {
                        logputs(
                            LOG_VERBOSE,
                            b"\n\0" as *const u8 as *const libc::c_char,
                        );
                        logputs(
                            LOG_NOTQUIET,
                            dcgettext(
                                0 as *const libc::c_char,
                                b"Invalid PORT.\n\0" as *const u8 as *const libc::c_char,
                                5 as libc::c_int,
                            ),
                        );
                        fd_close(csock);
                        (*con).csock = -(1 as libc::c_int);
                        fd_close(dtsock);
                        fd_close(local_sock);
                        return err;
                    }
                    7 => {}
                    _ => {
                        abort();
                    }
                }
                if !opt.server_response {
                    logputs(
                        LOG_VERBOSE,
                        dcgettext(
                            0 as *const libc::c_char,
                            b"done.    \0" as *const u8 as *const libc::c_char,
                            5 as libc::c_int,
                        ),
                    );
                }
            }
        }
        if restval != 0 && cmd & DO_RETR as libc::c_int != 0 {
            if !opt.server_response {
                logprintf(
                    LOG_VERBOSE,
                    b"==> REST %s ... \0" as *const u8 as *const libc::c_char,
                    number_to_static_string(restval),
                );
            }
            err = ftp_rest(csock, restval);
            match err as libc::c_uint {
                14 => {
                    logputs(LOG_VERBOSE, b"\n\0" as *const u8 as *const libc::c_char);
                    logputs(
                        LOG_NOTQUIET,
                        dcgettext(
                            0 as *const libc::c_char,
                            b"Error in server response, closing control connection.\n\0"
                                as *const u8 as *const libc::c_char,
                            5 as libc::c_int,
                        ),
                    );
                    fd_close(csock);
                    (*con).csock = -(1 as libc::c_int);
                    fd_close(dtsock);
                    fd_close(local_sock);
                    return err;
                }
                44 => {
                    logputs(LOG_VERBOSE, b"\n\0" as *const u8 as *const libc::c_char);
                    logputs(
                        LOG_NOTQUIET,
                        dcgettext(
                            0 as *const libc::c_char,
                            b"Write failed, closing control connection.\n\0" as *const u8
                                as *const libc::c_char,
                            5 as libc::c_int,
                        ),
                    );
                    fd_close(csock);
                    (*con).csock = -(1 as libc::c_int);
                    fd_close(dtsock);
                    fd_close(local_sock);
                    return err;
                }
                17 => {
                    logputs(
                        LOG_VERBOSE,
                        dcgettext(
                            0 as *const libc::c_char,
                            b"\nREST failed, starting from scratch.\n\0" as *const u8
                                as *const libc::c_char,
                            5 as libc::c_int,
                        ),
                    );
                    rest_failed = 1 as libc::c_int != 0;
                }
                7 => {}
                _ => {
                    abort();
                }
            }
            if err as libc::c_uint != FTPRESTFAIL as libc::c_int as libc::c_uint
                && !opt.server_response
            {
                logputs(
                    LOG_VERBOSE,
                    dcgettext(
                        0 as *const libc::c_char,
                        b"done.    \0" as *const u8 as *const libc::c_char,
                        5 as libc::c_int,
                    ),
                );
            }
        }
        if cmd & DO_RETR as libc::c_int != 0 {
            if opt.spider {
                let mut exists: bool = 0 as libc::c_int != 0;
                let mut all_exist: bool = 1 as libc::c_int != 0;
                let mut f: *mut fileinfo = 0 as *mut fileinfo;
                let mut _res: uerr_t = ftp_get_listing(u, original_url, con, &mut f);
                (*con).cmd |= DO_RETR as libc::c_int;
                if _res as libc::c_uint == RETROK as libc::c_int as libc::c_uint {
                    while !f.is_null() {
                        if strcmp((*f).name, (*u).file) == 0 {
                            exists = 1 as libc::c_int != 0;
                            break;
                        } else {
                            all_exist = 0 as libc::c_int != 0;
                            f = (*f).next;
                        }
                    }
                    if exists {
                        logputs(
                            LOG_VERBOSE,
                            b"\n\0" as *const u8 as *const libc::c_char,
                        );
                        logprintf(
                            LOG_NOTQUIET,
                            dcgettext(
                                0 as *const libc::c_char,
                                b"File %s exists.\n\0" as *const u8 as *const libc::c_char,
                                5 as libc::c_int,
                            ),
                            quote((*u).file),
                        );
                    } else {
                        logputs(
                            LOG_VERBOSE,
                            b"\n\0" as *const u8 as *const libc::c_char,
                        );
                        logprintf(
                            LOG_NOTQUIET,
                            dcgettext(
                                0 as *const libc::c_char,
                                b"No such file %s.\n\0" as *const u8 as *const libc::c_char,
                                5 as libc::c_int,
                            ),
                            quote((*u).file),
                        );
                    }
                }
                fd_close(csock);
                (*con).csock = -(1 as libc::c_int);
                fd_close(dtsock);
                fd_close(local_sock);
                if all_exist { return RETRFINISHED } else { return FTPNSFOD }
            }
            if opt.verbose != 0 {
                if !opt.server_response {
                    if restval != 0 {
                        logputs(
                            LOG_VERBOSE,
                            b"\n\0" as *const u8 as *const libc::c_char,
                        );
                    }
                    logprintf(
                        LOG_VERBOSE,
                        b"==> RETR %s ... \0" as *const u8 as *const libc::c_char,
                        quotearg_style(escape_quoting_style, (*u).file),
                    );
                }
            }
            err = ftp_retr(csock, (*u).file);
            match err as libc::c_uint {
                14 => {
                    logputs(LOG_VERBOSE, b"\n\0" as *const u8 as *const libc::c_char);
                    logputs(
                        LOG_NOTQUIET,
                        dcgettext(
                            0 as *const libc::c_char,
                            b"Error in server response, closing control connection.\n\0"
                                as *const u8 as *const libc::c_char,
                            5 as libc::c_int,
                        ),
                    );
                    fd_close(csock);
                    (*con).csock = -(1 as libc::c_int);
                    fd_close(dtsock);
                    fd_close(local_sock);
                    return err;
                }
                44 => {
                    logputs(LOG_VERBOSE, b"\n\0" as *const u8 as *const libc::c_char);
                    logputs(
                        LOG_NOTQUIET,
                        dcgettext(
                            0 as *const libc::c_char,
                            b"Write failed, closing control connection.\n\0" as *const u8
                                as *const libc::c_char,
                            5 as libc::c_int,
                        ),
                    );
                    fd_close(csock);
                    (*con).csock = -(1 as libc::c_int);
                    fd_close(dtsock);
                    fd_close(local_sock);
                    return err;
                }
                12 => {
                    logputs(LOG_VERBOSE, b"\n\0" as *const u8 as *const libc::c_char);
                    logprintf(
                        LOG_NOTQUIET,
                        dcgettext(
                            0 as *const libc::c_char,
                            b"No such file %s.\n\n\0" as *const u8
                                as *const libc::c_char,
                            5 as libc::c_int,
                        ),
                        quote((*u).file),
                    );
                    fd_close(dtsock);
                    fd_close(local_sock);
                    return err;
                }
                7 => {}
                _ => {
                    abort();
                }
            }
            if !opt.server_response {
                logputs(
                    LOG_VERBOSE,
                    dcgettext(
                        0 as *const libc::c_char,
                        b"done.\n\0" as *const u8 as *const libc::c_char,
                        5 as libc::c_int,
                    ),
                );
            }
            if !got_expected_bytes {
                expected_bytes = *last_expected_bytes;
            }
        }
        if cmd & DO_LIST as libc::c_int != 0 {
            if !opt.server_response {
                logputs(
                    LOG_VERBOSE,
                    b"==> LIST ... \0" as *const u8 as *const libc::c_char,
                );
            }
            err = ftp_list(
                csock,
                0 as *const libc::c_char,
                (*con).st & AVOID_LIST_A as libc::c_int != 0,
                (*con).st & AVOID_LIST as libc::c_int != 0,
                &mut list_a_used,
            );
            match err as libc::c_uint {
                14 => {
                    logputs(LOG_VERBOSE, b"\n\0" as *const u8 as *const libc::c_char);
                    logputs(
                        LOG_NOTQUIET,
                        dcgettext(
                            0 as *const libc::c_char,
                            b"Error in server response, closing control connection.\n\0"
                                as *const u8 as *const libc::c_char,
                            5 as libc::c_int,
                        ),
                    );
                    fd_close(csock);
                    (*con).csock = -(1 as libc::c_int);
                    fd_close(dtsock);
                    fd_close(local_sock);
                    return err;
                }
                44 => {
                    logputs(LOG_VERBOSE, b"\n\0" as *const u8 as *const libc::c_char);
                    logputs(
                        LOG_NOTQUIET,
                        dcgettext(
                            0 as *const libc::c_char,
                            b"Write failed, closing control connection.\n\0" as *const u8
                                as *const libc::c_char,
                            5 as libc::c_int,
                        ),
                    );
                    fd_close(csock);
                    (*con).csock = -(1 as libc::c_int);
                    fd_close(dtsock);
                    fd_close(local_sock);
                    return err;
                }
                12 => {
                    logputs(LOG_VERBOSE, b"\n\0" as *const u8 as *const libc::c_char);
                    logprintf(
                        LOG_NOTQUIET,
                        dcgettext(
                            0 as *const libc::c_char,
                            b"No such file or directory %s.\n\n\0" as *const u8
                                as *const libc::c_char,
                            5 as libc::c_int,
                        ),
                        quote(b".\0" as *const u8 as *const libc::c_char),
                    );
                    fd_close(dtsock);
                    fd_close(local_sock);
                    return err;
                }
                7 => {}
                _ => {
                    abort();
                }
            }
            if !opt.server_response {
                logputs(
                    LOG_VERBOSE,
                    dcgettext(
                        0 as *const libc::c_char,
                        b"done.\n\0" as *const u8 as *const libc::c_char,
                        5 as libc::c_int,
                    ),
                );
            }
            if !got_expected_bytes {
                expected_bytes = *last_expected_bytes;
            }
        }
        if cmd & (DO_LIST as libc::c_int | DO_RETR as libc::c_int) == 0
            || opt.spider as libc::c_int != 0 && cmd & DO_LIST as libc::c_int == 0
        {
            return RETRFINISHED;
        }
        if passed_expected_bytes != 0 && restval != 0 && expected_bytes != 0
            && expected_bytes == passed_expected_bytes - restval
        {
            if opt.debug as libc::c_long != 0 {
                debug_logprintf(
                    b"Lying FTP server found, adjusting.\n\0" as *const u8
                        as *const libc::c_char,
                );
            }
            expected_bytes = passed_expected_bytes;
        }
        if !opt.ftp_pasv {
            dtsock = accept_connection(local_sock);
            if dtsock < 0 as libc::c_int {
                logprintf(
                    LOG_NOTQUIET,
                    b"accept: %s\n\0" as *const u8 as *const libc::c_char,
                    strerror(*__errno_location()),
                );
                return CONERROR;
            }
        }
        if output_stream.is_null() || (*con).cmd & DO_LIST as libc::c_int != 0 {
            mkalldirs((*con).target);
            if opt.backups != 0 {
                rotate_backups((*con).target);
            }
            if restval != 0 && (*con).cmd & DO_LIST as libc::c_int == 0 {
                fp = rpl_fopen(
                    (*con).target,
                    b"ab\0" as *const u8 as *const libc::c_char,
                );
            } else if opt.noclobber as libc::c_int != 0
                || opt.always_rest as libc::c_int != 0
                || opt.timestamping as libc::c_int != 0
                || opt.dirstruct as libc::c_int != 0 || !(opt.output_document).is_null()
                || count > 0 as libc::c_int
            {
                if opt.unlink_requested as libc::c_int != 0
                    && file_exists_p((*con).target, 0 as *mut file_stats_t)
                        as libc::c_int != 0
                {
                    if unlink((*con).target) < 0 as libc::c_int {
                        logprintf(
                            LOG_NOTQUIET,
                            b"%s: %s\n\0" as *const u8 as *const libc::c_char,
                            (*con).target,
                            strerror(*__errno_location()),
                        );
                        fd_close(csock);
                        (*con).csock = -(1 as libc::c_int);
                        fd_close(dtsock);
                        fd_close(local_sock);
                        return UNLINKERR;
                    }
                }
                fp = rpl_fopen(
                    (*con).target,
                    b"wb\0" as *const u8 as *const libc::c_char,
                );
            } else {
                fp = fopen_excl((*con).target, 1 as libc::c_int);
                if fp.is_null() && *__errno_location() == 17 as libc::c_int {
                    logprintf(
                        LOG_NOTQUIET,
                        dcgettext(
                            0 as *const libc::c_char,
                            b"%s has sprung into existence.\n\0" as *const u8
                                as *const libc::c_char,
                            5 as libc::c_int,
                        ),
                        (*con).target,
                    );
                    fd_close(csock);
                    (*con).csock = -(1 as libc::c_int);
                    fd_close(dtsock);
                    fd_close(local_sock);
                    return FOPEN_EXCL_ERR;
                }
            }
            if fp.is_null() {
                logprintf(
                    LOG_NOTQUIET,
                    b"%s: %s\n\0" as *const u8 as *const libc::c_char,
                    (*con).target,
                    strerror(*__errno_location()),
                );
                fd_close(csock);
                (*con).csock = -(1 as libc::c_int);
                fd_close(dtsock);
                fd_close(local_sock);
                return FOPENERR;
            }
        } else {
            fp = output_stream;
        }
        if passed_expected_bytes != 0 {
            print_length(passed_expected_bytes, restval, 1 as libc::c_int != 0);
            expected_bytes = passed_expected_bytes;
        } else if expected_bytes != 0 {
            print_length(expected_bytes, restval, 0 as libc::c_int != 0);
        }
        flags = 0 as libc::c_int;
        if restval != 0 && rest_failed as libc::c_int != 0 {
            flags |= rb_skip_startpos as libc::c_int;
        }
        rd_size = 0 as libc::c_int as wgint;
        res = fd_read_body(
            (*con).target,
            dtsock,
            fp,
            if expected_bytes != 0 {
                expected_bytes - restval
            } else {
                0 as libc::c_int as libc::c_long
            },
            restval,
            &mut rd_size,
            qtyread,
            &mut (*con).dltime,
            flags,
            warc_tmp,
        );
        tms = datetime_str(time(0 as *mut time_t));
        tmrate = retr_rate(rd_size, (*con).dltime);
        total_download_time += (*con).dltime;
        if opt.enable_xattr {
            set_file_metadata(u, 0 as *const url, fp);
        }
        fd_close(local_sock);
        if output_stream.is_null() || (*con).cmd & DO_LIST as libc::c_int != 0 {
            fclose(fp);
        }
        if res == -(2 as libc::c_int)
            || !warc_tmp.is_null() && res == -(3 as libc::c_int)
        {
            logprintf(
                LOG_NOTQUIET,
                dcgettext(
                    0 as *const libc::c_char,
                    b"%s: %s, closing control connection.\n\0" as *const u8
                        as *const libc::c_char,
                    5 as libc::c_int,
                ),
                (*con).target,
                strerror(*__errno_location()),
            );
            fd_close(csock);
            (*con).csock = -(1 as libc::c_int);
            fd_close(dtsock);
            if res == -(2 as libc::c_int) {
                return FWRITEERR
            } else if res == -(3 as libc::c_int) {
                return WARC_TMP_FWRITEERR
            }
        } else if res == -(1 as libc::c_int) {
            logprintf(
                LOG_NOTQUIET,
                dcgettext(
                    0 as *const libc::c_char,
                    b"%s (%s) - Data connection: %s; \0" as *const u8
                        as *const libc::c_char,
                    5 as libc::c_int,
                ),
                tms,
                tmrate,
                fd_errstr(dtsock),
            );
            if opt.server_response {
                logputs(LOG_ALWAYS, b"\n\0" as *const u8 as *const libc::c_char);
            }
        }
        fd_close(dtsock);
        err = ftp_response(csock, &mut respline);
        if err as libc::c_uint != FTPOK as libc::c_int as libc::c_uint {
            if res != -(1 as libc::c_int) {
                logprintf(
                    LOG_NOTQUIET,
                    b"%s (%s) - \0" as *const u8 as *const libc::c_char,
                    tms,
                    tmrate,
                );
            }
            logputs(
                LOG_NOTQUIET,
                dcgettext(
                    0 as *const libc::c_char,
                    b"Control connection closed.\n\0" as *const u8
                        as *const libc::c_char,
                    5 as libc::c_int,
                ),
            );
            fd_close(csock);
            (*con).csock = -(1 as libc::c_int);
            return FTPRETRINT;
        }
        *last_expected_bytes = ftp_expected_bytes(respline);
        if *respline as libc::c_int != '2' as i32 {
            if res != -(1 as libc::c_int) {
                logprintf(
                    LOG_NOTQUIET,
                    b"%s (%s) - \0" as *const u8 as *const libc::c_char,
                    tms,
                    tmrate,
                );
            }
            logputs(
                LOG_NOTQUIET,
                dcgettext(
                    0 as *const libc::c_char,
                    b"Data transfer aborted.\n\0" as *const u8 as *const libc::c_char,
                    5 as libc::c_int,
                ),
            );
            rpl_free(respline as *mut libc::c_void);
            respline = 0 as *mut libc::c_char;
            return FTPRETRINT;
        }
        rpl_free(respline as *mut libc::c_void);
        respline = 0 as *mut libc::c_char;
        if res == -(1 as libc::c_int) {
            return FTPRETRINT;
        }
        if cmd & LEAVE_PENDING as libc::c_int == 0 {
            fd_close(csock);
            (*con).csock = -(1 as libc::c_int);
        }
        if (*con).cmd & DO_LIST as libc::c_int != 0 {
            if opt.server_response {
                mkalldirs((*con).target);
                fp = rpl_fopen(
                    (*con).target,
                    b"r\0" as *const u8 as *const libc::c_char,
                );
                if fp.is_null() {
                    logprintf(
                        LOG_ALWAYS,
                        b"%s: %s\n\0" as *const u8 as *const libc::c_char,
                        (*con).target,
                        strerror(*__errno_location()),
                    );
                } else {
                    let mut line: *mut libc::c_char = 0 as *mut libc::c_char;
                    let mut bufsize: size_t = 0 as libc::c_int as size_t;
                    let mut len_0: ssize_t = 0;
                    loop {
                        len_0 = getline(&mut line, &mut bufsize, fp);
                        if !(len_0 > 0 as libc::c_int as libc::c_long) {
                            break;
                        }
                        while len_0 > 0 as libc::c_int as libc::c_long
                            && (*line
                                .offset((len_0 - 1 as libc::c_int as libc::c_long) as isize)
                                as libc::c_int == '\n' as i32
                                || *line
                                    .offset((len_0 - 1 as libc::c_int as libc::c_long) as isize)
                                    as libc::c_int == '\r' as i32)
                        {
                            len_0 -= 1;
                            *line.offset(len_0 as isize) = '\0' as i32 as libc::c_char;
                        }
                        logprintf(
                            LOG_ALWAYS,
                            b"%s\n\0" as *const u8 as *const libc::c_char,
                            quotearg_style(escape_quoting_style, line),
                        );
                    }
                    rpl_free(line as *mut libc::c_void);
                    line = 0 as *mut libc::c_char;
                    fclose(fp);
                }
            }
            if (*con).st & LIST_AFTER_LIST_A_CHECK_DONE as libc::c_int == 0 {
                if (*con).st & AVOID_LIST_A as libc::c_int != 0 {
                    if rd_size > previous_rd_size {
                        (*con).st |= LIST_AFTER_LIST_A_CHECK_DONE as libc::c_int;
                        if opt.debug as libc::c_long != 0 {
                            debug_logprintf(
                                b"LIST returned more data than \"LIST -a\": I will use \"LIST\" as standard list command\n\0"
                                    as *const u8 as *const libc::c_char,
                            );
                        }
                    } else if previous_rd_size > rd_size {
                        (*con).st |= LIST_AFTER_LIST_A_CHECK_DONE as libc::c_int;
                        (*con).st |= AVOID_LIST as libc::c_int;
                        (*con).st &= !(AVOID_LIST_A as libc::c_int);
                        try_again = 1 as libc::c_int != 0;
                        if opt.debug as libc::c_long != 0 {
                            debug_logprintf(
                                b"LIST returned less data than \"LIST -a\": I will use \"LIST -a\" as standard list command\n\0"
                                    as *const u8 as *const libc::c_char,
                            );
                        }
                    } else if rd_size == 0 as libc::c_int as libc::c_long {
                        (*con).st &= !(AVOID_LIST_A as libc::c_int);
                    } else {
                        (*con).st |= LIST_AFTER_LIST_A_CHECK_DONE as libc::c_int;
                        (*con).st |= AVOID_LIST as libc::c_int;
                        (*con).st &= !(AVOID_LIST_A as libc::c_int);
                        if opt.debug as libc::c_long != 0 {
                            debug_logprintf(
                                b"LIST returned the same amount of data of \"LIST -a\": I will use \"LIST -a\" as standard list command\n\0"
                                    as *const u8 as *const libc::c_char,
                            );
                        }
                    }
                } else if list_a_used {
                    previous_rd_size = rd_size;
                    try_again = 1 as libc::c_int != 0;
                    (*con).st |= AVOID_LIST_A as libc::c_int;
                } else {
                    (*con).st |= LIST_AFTER_LIST_A_CHECK_DONE as libc::c_int;
                    (*con).st |= AVOID_LIST_A as libc::c_int;
                    if opt.debug as libc::c_long != 0 {
                        debug_logprintf(
                            b"\"LIST -a\" failed: I will use \"LIST\" as standard list command\n\0"
                                as *const u8 as *const libc::c_char,
                        );
                    }
                }
            }
        }
        if !try_again {
            break;
        }
    }
    return RETRFINISHED;
}
unsafe extern "C" fn ftp_loop_internal(
    mut u: *mut url,
    mut original_url: *mut url,
    mut f: *mut fileinfo,
    mut con: *mut ccon,
    mut local_file: *mut *mut libc::c_char,
    mut force_full_retrieve: bool,
) -> uerr_t {
    let mut count: libc::c_int = 0;
    let mut orig_lp: libc::c_int = 0;
    let mut restval: wgint = 0;
    let mut len: wgint = 0 as libc::c_int as wgint;
    let mut qtyread: wgint = 0 as libc::c_int as wgint;
    let mut tms: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut locf: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut tmrate: *const libc::c_char = 0 as *const libc::c_char;
    let mut err: uerr_t = NOCONERROR;
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
    let mut warc_enabled: bool = !(opt.warc_filename).is_null();
    let mut warc_tmp: *mut FILE = 0 as *mut FILE;
    let mut warc_ip_buf: ip_address = ip_address {
        family: 0,
        data: C2RustUnnamed_6 {
            d4: in_addr { s_addr: 0 },
        },
        ipv6_scope: 0,
    };
    let mut warc_ip: *mut ip_address = 0 as *mut ip_address;
    let mut last_expected_bytes: wgint = 0 as libc::c_int as wgint;
    if f.is_null() && !((*con).target).is_null() {
        locf = (*con).target;
    } else {
        rpl_free((*con).target as *mut libc::c_void);
        (*con).target = 0 as *mut libc::c_char;
        (*con)
            .target = url_file_name(
            if opt.trustservernames as libc::c_int != 0 || original_url.is_null() {
                u
            } else {
                original_url
            },
            0 as *mut libc::c_char,
        );
        if (opt.output_document).is_null() {
            locf = (*con).target;
        } else {
            locf = opt.output_document;
        }
    }
    if opt.noclobber as libc::c_int != 0 && (opt.output_document).is_null()
        && file_exists_p((*con).target, 0 as *mut file_stats_t) as libc::c_int != 0
        && !((*con).cmd & DO_LIST as libc::c_int != 0
            && (*con).cmd & DO_RETR as libc::c_int == 0)
    {
        logprintf(
            LOG_VERBOSE,
            dcgettext(
                0 as *const libc::c_char,
                b"File %s already there; not retrieving.\n\0" as *const u8
                    as *const libc::c_char,
                5 as libc::c_int,
            ),
            quote((*con).target),
        );
        return RETROK;
    }
    remove_link((*con).target);
    count = 0 as libc::c_int;
    if (*con).st & ON_YOUR_OWN as libc::c_int != 0 {
        (*con).st = ON_YOUR_OWN as libc::c_int;
    }
    orig_lp = if (*con).cmd & LEAVE_PENDING as libc::c_int != 0 {
        1 as libc::c_int
    } else {
        0 as libc::c_int
    };
    let mut current_block_144: u64;
    loop {
        count += 1;
        count;
        sleep_between_retrievals(count);
        if (*con).st & ON_YOUR_OWN as libc::c_int != 0 {
            (*con).cmd = 0 as libc::c_int;
            (*con).cmd |= DO_RETR as libc::c_int | LEAVE_PENDING as libc::c_int;
            if (*con).csock != -(1 as libc::c_int) {
                (*con).cmd &= !(DO_LOGIN as libc::c_int | DO_CWD as libc::c_int);
            } else {
                (*con).cmd |= DO_LOGIN as libc::c_int | DO_CWD as libc::c_int;
            }
        } else {
            if (*con).csock != -(1 as libc::c_int) {
                (*con).cmd &= !(DO_LOGIN as libc::c_int);
            } else {
                (*con).cmd |= DO_LOGIN as libc::c_int;
            }
            if (*con).st & DONE_CWD as libc::c_int != 0 {
                (*con).cmd &= !(DO_CWD as libc::c_int);
            } else {
                (*con).cmd |= DO_CWD as libc::c_int;
            }
        }
        if warc_enabled as libc::c_int != 0 && (*con).cmd & DO_RETR as libc::c_int != 0
            && warc_tmp.is_null()
        {
            warc_tmp = warc_tempfile();
            if warc_tmp.is_null() {
                return WARC_TMP_FOPENERR;
            }
            if ((*con).proxy).is_null() && (*con).csock != -(1 as libc::c_int) {
                warc_ip = &mut warc_ip_buf;
                socket_ip_address((*con).csock, warc_ip, ENDPOINT_PEER as libc::c_int);
            }
        }
        if (*con).cmd & DO_LIST as libc::c_int != 0 {
            restval = 0 as libc::c_int as wgint;
        } else if force_full_retrieve {
            restval = 0 as libc::c_int as wgint;
        } else if opt.start_pos >= 0 as libc::c_int as libc::c_long {
            restval = opt.start_pos;
        } else if opt.always_rest as libc::c_int != 0
            && stat(locf, &mut st) == 0 as libc::c_int
            && st.st_mode & 0o170000 as libc::c_int as libc::c_uint
                == 0o100000 as libc::c_int as libc::c_uint
        {
            restval = st.st_size;
        } else if count > 1 as libc::c_int {
            restval = qtyread;
        } else {
            restval = 0 as libc::c_int as wgint;
        }
        tms = datetime_str(time(0 as *mut time_t));
        if opt.verbose != 0 {
            let mut hurl: *mut libc::c_char = url_string(u, URL_AUTH_HIDE_PASSWD);
            let mut tmp: [libc::c_char; 256] = [0; 256];
            strcpy(tmp.as_mut_ptr(), b"        \0" as *const u8 as *const libc::c_char);
            if count > 1 as libc::c_int {
                sprintf(
                    tmp.as_mut_ptr(),
                    dcgettext(
                        0 as *const libc::c_char,
                        b"(try:%2d)\0" as *const u8 as *const libc::c_char,
                        5 as libc::c_int,
                    ),
                    count,
                );
            }
            logprintf(
                LOG_VERBOSE,
                b"--%s--  %s\n  %s => %s\n\0" as *const u8 as *const libc::c_char,
                tms,
                hurl,
                tmp.as_mut_ptr(),
                quote(locf),
            );
            rpl_free(hurl as *mut libc::c_void);
            hurl = 0 as *mut libc::c_char;
        }
        if !f.is_null()
            && (*f).type_0 as libc::c_uint != FT_SYMLINK as libc::c_int as libc::c_uint
        {
            len = (*f).size;
        } else {
            len = 0 as libc::c_int as wgint;
        }
        err = getftp(
            u,
            original_url,
            len,
            &mut qtyread,
            restval,
            con,
            count,
            &mut last_expected_bytes,
            warc_tmp,
        );
        if (*con).csock == -(1 as libc::c_int) {
            (*con).st &= !(DONE_CWD as libc::c_int);
        } else {
            (*con).st |= DONE_CWD as libc::c_int;
        }
        match err as libc::c_uint {
            1 | 5 | 21 | 19 | 12 | 8 | 29 | 32 | 30 | 31 | 47 | 54 | 4 | 33 | 46 => {
                if !warc_tmp.is_null() {
                    fclose(warc_tmp);
                    warc_tmp = 0 as *mut FILE;
                }
                return err;
            }
            2 | 3 | 15 | 14 | 44 | 13 | 11 | 10 | 9 | 28 | 20 => {
                printwhat(count, opt.ntry);
                if err as libc::c_uint == FOPEN_EXCL_ERR as libc::c_int as libc::c_uint {
                    rpl_free((*con).target as *mut libc::c_void);
                    (*con).target = 0 as *mut libc::c_char;
                    (*con).target = url_file_name(u, 0 as *mut libc::c_char);
                    locf = (*con).target;
                }
                current_block_144 = 2838571290723028321;
            }
            16 => {
                if f.is_null() || qtyread != (*f).size {
                    printwhat(count, opt.ntry);
                    current_block_144 = 2838571290723028321;
                } else {
                    current_block_144 = 12070711452894729854;
                }
            }
            35 => {
                current_block_144 = 12070711452894729854;
            }
            _ => {
                abort();
            }
        }
        match current_block_144 {
            2838571290723028321 => {
                if !(opt.ntry == 0 || count < opt.ntry) {
                    break;
                }
            }
            _ => {
                tms = datetime_str(time(0 as *mut time_t));
                if !opt.spider {
                    tmrate = retr_rate(qtyread - restval, (*con).dltime);
                }
                downloaded_file(FILE_DOWNLOADED_NORMALLY, locf);
                if (*con).st & ON_YOUR_OWN as libc::c_int != 0 {
                    fd_close((*con).csock);
                    (*con).csock = -(1 as libc::c_int);
                }
                if !opt.spider {
                    let mut write_to_stdout: bool = !(opt.output_document).is_null()
                        && (*opt.output_document as libc::c_int == '-' as i32
                            && *(opt.output_document).offset(1 as libc::c_int as isize)
                                == 0);
                    logprintf(
                        LOG_VERBOSE,
                        if write_to_stdout as libc::c_int != 0 {
                            dcgettext(
                                0 as *const libc::c_char,
                                b"%s (%s) - written to stdout %s[%s]\n\n\0" as *const u8
                                    as *const libc::c_char,
                                5 as libc::c_int,
                            )
                        } else {
                            dcgettext(
                                0 as *const libc::c_char,
                                b"%s (%s) - %s saved [%s]\n\n\0" as *const u8
                                    as *const libc::c_char,
                                5 as libc::c_int,
                            )
                        },
                        tms,
                        tmrate,
                        if write_to_stdout as libc::c_int != 0 {
                            b"\0" as *const u8 as *const libc::c_char
                        } else {
                            quote(locf)
                        },
                        number_to_static_string(qtyread),
                    );
                }
                if opt.verbose == 0 && !opt.quiet {
                    let mut hurl_0: *mut libc::c_char = url_string(
                        u,
                        URL_AUTH_HIDE_PASSWD,
                    );
                    logprintf(
                        LOG_NONVERBOSE,
                        b"%s URL: %s [%s] -> \"%s\" [%d]\n\0" as *const u8
                            as *const libc::c_char,
                        tms,
                        hurl_0,
                        number_to_static_string(qtyread),
                        locf,
                        count,
                    );
                    rpl_free(hurl_0 as *mut libc::c_void);
                    hurl_0 = 0 as *mut libc::c_char;
                }
                if warc_enabled as libc::c_int != 0
                    && (*con).cmd & DO_RETR as libc::c_int != 0
                {
                    let mut warc_res: bool = false;
                    warc_res = warc_write_resource_record(
                        0 as *const libc::c_char,
                        (*u).url,
                        0 as *const libc::c_char,
                        0 as *const libc::c_char,
                        warc_ip,
                        0 as *const libc::c_char,
                        warc_tmp,
                        -(1 as libc::c_int) as off_t,
                    );
                    if !warc_res {
                        return WARC_ERR;
                    }
                    warc_tmp = 0 as *mut FILE;
                }
                if (*con).cmd & DO_LIST as libc::c_int != 0 {
                    if !opt.remove_listing {
                        total_downloaded_bytes += qtyread - restval;
                        numurls += 1;
                        numurls;
                    }
                } else if !opt.spider {
                    total_downloaded_bytes += qtyread - restval;
                    numurls += 1;
                    numurls;
                    if opt.delete_after as libc::c_int != 0
                        && !input_file_url(opt.input_filename)
                    {
                        if opt.debug as libc::c_long != 0 {
                            debug_logprintf(
                                b"Removing file due to --delete-after in ftp_loop_internal():\n\0"
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
                            locf,
                        );
                        if unlink(locf) != 0 {
                            logprintf(
                                LOG_NOTQUIET,
                                b"unlink: %s\n\0" as *const u8 as *const libc::c_char,
                                strerror(*__errno_location()),
                            );
                        }
                    }
                }
                if orig_lp != 0 {
                    (*con).cmd |= LEAVE_PENDING as libc::c_int;
                } else {
                    (*con).cmd &= !(LEAVE_PENDING as libc::c_int);
                }
                if !local_file.is_null() {
                    *local_file = xstrdup(locf);
                }
                if !warc_tmp.is_null() {
                    fclose(warc_tmp);
                    warc_tmp = 0 as *mut FILE;
                }
                return RETROK;
            }
        }
    }
    if (*con).csock != -(1 as libc::c_int) && (*con).st & ON_YOUR_OWN as libc::c_int != 0
    {
        fd_close((*con).csock);
        (*con).csock = -(1 as libc::c_int);
    }
    if !warc_tmp.is_null() {
        fclose(warc_tmp);
    }
    return TRYLIMEXC;
}
unsafe extern "C" fn ftp_get_listing(
    mut u: *mut url,
    mut original_url: *mut url,
    mut con: *mut ccon,
    mut f: *mut *mut fileinfo,
) -> uerr_t {
    let mut err: uerr_t = NOCONERROR;
    let mut uf: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut lf: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut old_target: *mut libc::c_char = (*con).target;
    (*con).st &= !(ON_YOUR_OWN as libc::c_int);
    (*con).cmd |= DO_LIST as libc::c_int | LEAVE_PENDING as libc::c_int;
    (*con).cmd &= !(DO_RETR as libc::c_int);
    uf = url_file_name(u, 0 as *mut libc::c_char);
    lf = file_merge(uf, b".listing\0" as *const u8 as *const libc::c_char);
    rpl_free(uf as *mut libc::c_void);
    uf = 0 as *mut libc::c_char;
    if opt.debug as libc::c_long != 0 {
        debug_logprintf(
            dcgettext(
                0 as *const libc::c_char,
                b"Using %s as listing tmp file.\n\0" as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ),
            quote(lf),
        );
    }
    (*con).target = xstrdup(lf);
    rpl_free(lf as *mut libc::c_void);
    lf = 0 as *mut libc::c_char;
    err = ftp_loop_internal(
        u,
        original_url,
        0 as *mut fileinfo,
        con,
        0 as *mut *mut libc::c_char,
        0 as libc::c_int != 0,
    );
    lf = xstrdup((*con).target);
    rpl_free((*con).target as *mut libc::c_void);
    (*con).target = 0 as *mut libc::c_char;
    (*con).target = old_target;
    if err as libc::c_uint == RETROK as libc::c_int as libc::c_uint {
        *f = ftp_parse_ls(lf, (*con).rs);
        if opt.remove_listing {
            if unlink(lf) != 0 {
                logprintf(
                    LOG_NOTQUIET,
                    b"unlink: %s\n\0" as *const u8 as *const libc::c_char,
                    strerror(*__errno_location()),
                );
            } else {
                logprintf(
                    LOG_VERBOSE,
                    dcgettext(
                        0 as *const libc::c_char,
                        b"Removed %s.\n\0" as *const u8 as *const libc::c_char,
                        5 as libc::c_int,
                    ),
                    quote(lf),
                );
            }
        }
    } else {
        *f = 0 as *mut fileinfo;
    }
    rpl_free(lf as *mut libc::c_void);
    lf = 0 as *mut libc::c_char;
    (*con).cmd &= !(DO_LIST as libc::c_int);
    return err;
}
unsafe extern "C" fn ftp_retrieve_list(
    mut u: *mut url,
    mut original_url: *mut url,
    mut f: *mut fileinfo,
    mut con: *mut ccon,
) -> uerr_t {
    static mut depth: libc::c_int = 0 as libc::c_int;
    let mut err: uerr_t = NOCONERROR;
    let mut orig: *mut fileinfo = 0 as *mut fileinfo;
    let mut local_size: wgint = 0;
    let mut tml: time_t = 0;
    let mut dlthis: bool = false;
    let mut actual_target: *const libc::c_char = 0 as *const libc::c_char;
    let mut force_full_retrieve: bool = 0 as libc::c_int != 0;
    depth += 1;
    depth;
    if opt.reclevel != -(1 as libc::c_int) && depth > opt.reclevel {
        if opt.debug as libc::c_long != 0 {
            debug_logprintf(
                dcgettext(
                    0 as *const libc::c_char,
                    b"Recursion depth %d exceeded max. depth %d.\n\0" as *const u8
                        as *const libc::c_char,
                    5 as libc::c_int,
                ),
                depth,
                opt.reclevel,
            );
        }
        depth -= 1;
        depth;
        return RECLEVELEXC;
    }
    orig = f;
    (*con).st &= !(ON_YOUR_OWN as libc::c_int);
    if (*con).st & DONE_CWD as libc::c_int == 0 {
        (*con).cmd |= DO_CWD as libc::c_int;
    } else {
        (*con).cmd &= !(DO_CWD as libc::c_int);
    }
    (*con).cmd |= DO_RETR as libc::c_int | LEAVE_PENDING as libc::c_int;
    if (*con).csock < 0 as libc::c_int {
        (*con).cmd |= DO_LOGIN as libc::c_int;
    } else {
        (*con).cmd &= !(DO_LOGIN as libc::c_int);
    }
    err = RETROK;
    while !f.is_null() {
        let mut old_target: *mut libc::c_char = 0 as *mut libc::c_char;
        let mut ofile: *mut libc::c_char = 0 as *mut libc::c_char;
        if opt.quota != 0 && total_downloaded_bytes > opt.quota {
            depth -= 1;
            depth;
            return QUOTEXC;
        }
        old_target = (*con).target;
        ofile = xstrdup((*u).file);
        url_set_file(u, (*f).name);
        (*con).target = url_file_name(u, 0 as *mut libc::c_char);
        err = RETROK;
        dlthis = 1 as libc::c_int != 0;
        if opt.timestamping as libc::c_int != 0
            && (*f).type_0 as libc::c_uint == FT_PLAINFILE as libc::c_int as libc::c_uint
        {
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
            if stat((*con).target, &mut st) == 0 {
                let mut eq_size: bool = false;
                let mut cor_val: bool = false;
                local_size = st.st_size;
                tml = st.st_mtim.tv_sec;
                cor_val = (*con).rs as libc::c_uint
                    == ST_UNIX as libc::c_int as libc::c_uint
                    || (*con).rs as libc::c_uint
                        == ST_WINNT as libc::c_int as libc::c_uint;
                eq_size = if cor_val as libc::c_int != 0 {
                    (local_size == (*f).size) as libc::c_int
                } else {
                    1 as libc::c_int
                } != 0;
                if (*f).tstamp <= tml && eq_size as libc::c_int != 0 {
                    logprintf(
                        LOG_VERBOSE,
                        dcgettext(
                            0 as *const libc::c_char,
                            b"Remote file no newer than local file %s -- not retrieving.\n\0"
                                as *const u8 as *const libc::c_char,
                            5 as libc::c_int,
                        ),
                        quote((*con).target),
                    );
                    dlthis = 0 as libc::c_int != 0;
                } else if (*f).tstamp > tml {
                    force_full_retrieve = 1 as libc::c_int != 0;
                    logprintf(
                        LOG_VERBOSE,
                        dcgettext(
                            0 as *const libc::c_char,
                            b"Remote file is newer than local file %s -- retrieving.\n\n\0"
                                as *const u8 as *const libc::c_char,
                            5 as libc::c_int,
                        ),
                        quote((*con).target),
                    );
                } else {
                    logprintf(
                        LOG_VERBOSE,
                        dcgettext(
                            0 as *const libc::c_char,
                            b"The sizes do not match (local %s) -- retrieving.\n\n\0"
                                as *const u8 as *const libc::c_char,
                            5 as libc::c_int,
                        ),
                        number_to_static_string(local_size),
                    );
                }
            }
        }
        let mut current_block_76: u64;
        match (*f).type_0 as libc::c_uint {
            2 => {
                if !opt.retr_symlinks {
                    if ((*f).linkto).is_null() {
                        logputs(
                            LOG_NOTQUIET,
                            dcgettext(
                                0 as *const libc::c_char,
                                b"Invalid name of the symlink, skipping.\n\0" as *const u8
                                    as *const libc::c_char,
                                5 as libc::c_int,
                            ),
                        );
                    } else {
                        let mut st_0: stat = stat {
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
                        let mut rc: libc::c_int = lstat((*con).target, &mut st_0);
                        if rc == 0 as libc::c_int {
                            let mut len: size_t = (strlen((*f).linkto))
                                .wrapping_add(1 as libc::c_int as libc::c_ulong);
                            if st_0.st_mode & 0o170000 as libc::c_int as libc::c_uint
                                == 0o120000 as libc::c_int as libc::c_uint
                            {
                                let mut buf: [libc::c_char; 1024] = [0; 1024];
                                let mut link_target: *mut libc::c_char = 0
                                    as *mut libc::c_char;
                                let mut n: size_t = 0;
                                let mut res: bool = false;
                                if len
                                    < ::std::mem::size_of::<[libc::c_char; 1024]>()
                                        as libc::c_ulong
                                {
                                    link_target = buf.as_mut_ptr();
                                } else {
                                    link_target = xmalloc(len) as *mut libc::c_char;
                                }
                                n = readlink((*con).target, link_target, len) as size_t;
                                res = n
                                    == len.wrapping_sub(1 as libc::c_int as libc::c_ulong)
                                    && memcmp(
                                        link_target as *const libc::c_void,
                                        (*f).linkto as *const libc::c_void,
                                        n,
                                    ) == 0 as libc::c_int;
                                if link_target != buf.as_mut_ptr() {
                                    rpl_free(link_target as *mut libc::c_void);
                                    link_target = 0 as *mut libc::c_char;
                                }
                                if res {
                                    logprintf(
                                        LOG_VERBOSE,
                                        dcgettext(
                                            0 as *const libc::c_char,
                                            b"Already have correct symlink %s -> %s\n\n\0" as *const u8
                                                as *const libc::c_char,
                                            5 as libc::c_int,
                                        ),
                                        quote_n(0 as libc::c_int, (*con).target),
                                        quote_n(1 as libc::c_int, (*f).linkto),
                                    );
                                    dlthis = 0 as libc::c_int != 0;
                                    current_block_76 = 919954187481050311;
                                } else {
                                    current_block_76 = 2116367355679836638;
                                }
                            } else {
                                current_block_76 = 2116367355679836638;
                            }
                        } else {
                            current_block_76 = 2116367355679836638;
                        }
                        match current_block_76 {
                            919954187481050311 => {}
                            _ => {
                                logprintf(
                                    LOG_VERBOSE,
                                    dcgettext(
                                        0 as *const libc::c_char,
                                        b"Creating symlink %s -> %s\n\0" as *const u8
                                            as *const libc::c_char,
                                        5 as libc::c_int,
                                    ),
                                    quote_n(0 as libc::c_int, (*con).target),
                                    quote_n(1 as libc::c_int, (*f).linkto),
                                );
                                unlink((*con).target);
                                if symlink((*f).linkto, (*con).target)
                                    == -(1 as libc::c_int)
                                {
                                    logprintf(
                                        LOG_NOTQUIET,
                                        b"symlink: %s\n\0" as *const u8 as *const libc::c_char,
                                        strerror(*__errno_location()),
                                    );
                                }
                                logputs(
                                    LOG_VERBOSE,
                                    b"\n\0" as *const u8 as *const libc::c_char,
                                );
                            }
                        }
                    }
                } else if dlthis {
                    err = ftp_loop_internal(
                        u,
                        original_url,
                        f,
                        con,
                        0 as *mut *mut libc::c_char,
                        force_full_retrieve,
                    );
                }
            }
            1 => {
                if !opt.recursive {
                    logprintf(
                        LOG_NOTQUIET,
                        dcgettext(
                            0 as *const libc::c_char,
                            b"Skipping directory %s.\n\0" as *const u8
                                as *const libc::c_char,
                            5 as libc::c_int,
                        ),
                        quote((*f).name),
                    );
                }
            }
            0 => {
                if dlthis {
                    err = ftp_loop_internal(
                        u,
                        original_url,
                        f,
                        con,
                        0 as *mut *mut libc::c_char,
                        force_full_retrieve,
                    );
                }
            }
            3 | _ => {
                logprintf(
                    LOG_NOTQUIET,
                    dcgettext(
                        0 as *const libc::c_char,
                        b"%s: unknown/unsupported file type.\n\0" as *const u8
                            as *const libc::c_char,
                        5 as libc::c_int,
                    ),
                    quote((*f).name),
                );
            }
        }
        set_local_file(&mut actual_target, (*con).target);
        if dlthis as libc::c_int != 0 && !actual_target.is_null()
            && (*f).type_0 as libc::c_uint == FT_PLAINFILE as libc::c_int as libc::c_uint
            && opt.preserve_perm as libc::c_int != 0
        {
            if (*f).perms != 0 {
                if chmod(actual_target, (*f).perms as __mode_t) != 0 {
                    logprintf(
                        LOG_NOTQUIET,
                        dcgettext(
                            0 as *const libc::c_char,
                            b"Failed to set permissions for %s.\n\0" as *const u8
                                as *const libc::c_char,
                            5 as libc::c_int,
                        ),
                        actual_target,
                    );
                }
            } else if opt.debug as libc::c_long != 0 {
                debug_logprintf(
                    b"Unrecognized permissions for %s.\n\0" as *const u8
                        as *const libc::c_char,
                    actual_target,
                );
            }
        }
        if !actual_target.is_null() {
            if opt.useservertimestamps as libc::c_int != 0
                && !((*f).type_0 as libc::c_uint
                    == FT_SYMLINK as libc::c_int as libc::c_uint && !opt.retr_symlinks)
                && (*f).tstamp != -(1 as libc::c_int) as libc::c_long
                && dlthis as libc::c_int != 0
                && file_exists_p((*con).target, 0 as *mut file_stats_t) as libc::c_int
                    != 0
            {
                touch(actual_target, (*f).tstamp);
            } else if (*f).tstamp == -(1 as libc::c_int) as libc::c_long {
                logprintf(
                    LOG_NOTQUIET,
                    dcgettext(
                        0 as *const libc::c_char,
                        b"%s: corrupt time-stamp.\n\0" as *const u8
                            as *const libc::c_char,
                        5 as libc::c_int,
                    ),
                    actual_target,
                );
            }
        }
        rpl_free((*con).target as *mut libc::c_void);
        (*con).target = 0 as *mut libc::c_char;
        (*con).target = old_target;
        url_set_file(u, ofile);
        rpl_free(ofile as *mut libc::c_void);
        ofile = 0 as *mut libc::c_char;
        if err as libc::c_uint == QUOTEXC as libc::c_int as libc::c_uint
            || err as libc::c_uint == HOSTERR as libc::c_int as libc::c_uint
            || err as libc::c_uint == FWRITEERR as libc::c_int as libc::c_uint
            || err as libc::c_uint == WARC_ERR as libc::c_int as libc::c_uint
            || err as libc::c_uint == WARC_TMP_FOPENERR as libc::c_int as libc::c_uint
            || err as libc::c_uint == WARC_TMP_FWRITEERR as libc::c_int as libc::c_uint
        {
            break;
        }
        (*con).cmd &= !(DO_CWD as libc::c_int | DO_LOGIN as libc::c_int);
        f = (*f).next;
    }
    if opt.recursive as libc::c_int != 0
        && !(opt.reclevel != -(1 as libc::c_int) && depth >= opt.reclevel)
    {
        err = ftp_retrieve_dirs(u, original_url, orig, con);
    } else if opt.recursive {
        if opt.debug as libc::c_long != 0 {
            debug_logprintf(
                dcgettext(
                    0 as *const libc::c_char,
                    b"Will not retrieve dirs since depth is %d (max %d).\n\0"
                        as *const u8 as *const libc::c_char,
                    5 as libc::c_int,
                ),
                depth,
                opt.reclevel,
            );
        }
    }
    depth -= 1;
    depth;
    return err;
}
unsafe extern "C" fn ftp_retrieve_dirs(
    mut u: *mut url,
    mut original_url: *mut url,
    mut f: *mut fileinfo,
    mut con: *mut ccon,
) -> uerr_t {
    let mut buf: [libc::c_char; 1024] = [0; 1024];
    let mut container: *mut libc::c_char = buf.as_mut_ptr();
    let mut container_size: libc::c_int = ::std::mem::size_of::<[libc::c_char; 1024]>()
        as libc::c_ulong as libc::c_int;
    while !f.is_null() {
        let mut size: libc::c_int = 0;
        let mut odir: *mut libc::c_char = 0 as *mut libc::c_char;
        let mut newdir: *mut libc::c_char = 0 as *mut libc::c_char;
        if opt.quota != 0 && total_downloaded_bytes > opt.quota {
            break;
        }
        if !((*f).type_0 as libc::c_uint != FT_DIRECTORY as libc::c_int as libc::c_uint)
        {
            size = (strlen((*u).dir))
                .wrapping_add(1 as libc::c_int as libc::c_ulong)
                .wrapping_add(strlen((*f).name))
                .wrapping_add(1 as libc::c_int as libc::c_ulong) as libc::c_int;
            if size > container_size {
                if container == buf.as_mut_ptr() {
                    container = xmalloc(size as size_t) as *mut libc::c_char;
                } else {
                    container = xrealloc(container as *mut libc::c_void, size as size_t)
                        as *mut libc::c_char;
                }
                container_size = size;
            }
            newdir = container;
            odir = (*u).dir;
            if *odir as libc::c_int == '\0' as i32
                || *odir as libc::c_int == '/' as i32
                    && *odir.offset(1 as libc::c_int as isize) as libc::c_int
                        == '\0' as i32
            {
                sprintf(
                    newdir,
                    b"%s%s\0" as *const u8 as *const libc::c_char,
                    odir,
                    (*f).name,
                );
            } else {
                sprintf(
                    newdir,
                    b"%s/%s\0" as *const u8 as *const libc::c_char,
                    odir,
                    (*f).name,
                );
            }
            if opt.debug as libc::c_long != 0 {
                debug_logprintf(
                    b"Composing new CWD relative to the initial directory.\n\0"
                        as *const u8 as *const libc::c_char,
                );
            }
            if opt.debug as libc::c_long != 0 {
                debug_logprintf(
                    b"  odir = '%s'\n  f->name = '%s'\n  newdir = '%s'\n\n\0"
                        as *const u8 as *const libc::c_char,
                    odir,
                    (*f).name,
                    newdir,
                );
            }
            if !accdir(newdir) {
                logprintf(
                    LOG_VERBOSE,
                    dcgettext(
                        0 as *const libc::c_char,
                        b"Not descending to %s as it is excluded/not-included.\n\0"
                            as *const u8 as *const libc::c_char,
                        5 as libc::c_int,
                    ),
                    quote(newdir),
                );
            } else {
                (*con).st &= !(DONE_CWD as libc::c_int);
                odir = xstrdup((*u).dir);
                url_set_dir(u, newdir);
                ftp_retrieve_glob(u, original_url, con, GLOB_GETALL as libc::c_int);
                url_set_dir(u, odir);
                rpl_free(odir as *mut libc::c_void);
                odir = 0 as *mut libc::c_char;
            }
        }
        f = (*f).next;
    }
    if container != buf.as_mut_ptr() {
        rpl_free(container as *mut libc::c_void);
        container = 0 as *mut libc::c_char;
    }
    if opt.quota != 0 && total_downloaded_bytes > opt.quota {
        return QUOTEXC
    } else {
        return RETROK
    };
}
unsafe extern "C" fn has_insecure_name_p(mut s: *const libc::c_char) -> bool {
    if *s as libc::c_int == '/' as i32 {
        return 1 as libc::c_int != 0;
    }
    if !(strstr(s, b"../\0" as *const u8 as *const libc::c_char)).is_null() {
        return 1 as libc::c_int != 0;
    }
    return 0 as libc::c_int != 0;
}
unsafe extern "C" fn is_invalid_entry(mut f: *mut fileinfo) -> bool {
    let mut cur: *mut fileinfo = f;
    let mut f_name: *mut libc::c_char = (*f).name;
    while !((*cur).next).is_null() {
        cur = (*cur).next;
        if strcmp(f_name, (*cur).name) == 0 as libc::c_int {
            return 1 as libc::c_int != 0;
        }
    }
    return 0 as libc::c_int != 0;
}
unsafe extern "C" fn ftp_retrieve_glob(
    mut u: *mut url,
    mut original_url: *mut url,
    mut con: *mut ccon,
    mut action: libc::c_int,
) -> uerr_t {
    let mut f: *mut fileinfo = 0 as *mut fileinfo;
    let mut start: *mut fileinfo = 0 as *mut fileinfo;
    let mut res: uerr_t = NOCONERROR;
    (*con).cmd |= LEAVE_PENDING as libc::c_int;
    res = ftp_get_listing(u, original_url, con, &mut start);
    if res as libc::c_uint != RETROK as libc::c_int as libc::c_uint {
        return res;
    }
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
    let mut cmp: Option::<
        unsafe extern "C" fn(*const libc::c_char, *const libc::c_char) -> libc::c_int,
    > = if opt.ignore_case as libc::c_int != 0 {
        Some(
            strcasecmp
                as unsafe extern "C" fn(
                    *const libc::c_char,
                    *const libc::c_char,
                ) -> libc::c_int,
        )
    } else {
        Some(
            strcmp
                as unsafe extern "C" fn(
                    *const libc::c_char,
                    *const libc::c_char,
                ) -> libc::c_int,
        )
    };
    f = start;
    while !f.is_null() {
        if (!(opt.accepts).is_null() || !(opt.rejects).is_null())
            && (*f).type_0 as libc::c_uint != FT_DIRECTORY as libc::c_int as libc::c_uint
            && !acceptable((*f).name)
        {
            logprintf(
                LOG_VERBOSE,
                dcgettext(
                    0 as *const libc::c_char,
                    b"Rejecting %s.\n\0" as *const u8 as *const libc::c_char,
                    5 as libc::c_int,
                ),
                quote((*f).name),
            );
            f = delelement(&mut f, &mut start);
        } else if has_insecure_name_p((*f).name) as libc::c_int != 0
            || is_invalid_entry(f) as libc::c_int != 0
        {
            logprintf(
                LOG_VERBOSE,
                dcgettext(
                    0 as *const libc::c_char,
                    b"Rejecting %s (Invalid Entry).\n\0" as *const u8
                        as *const libc::c_char,
                    5 as libc::c_int,
                ),
                quote((*f).name),
            );
            f = delelement(&mut f, &mut start);
        } else {
            if !(opt.acceptregex).is_null() || !(opt.rejectregex).is_null() {
                let mut buf: [libc::c_char; 1024] = [0; 1024];
                let mut url: *mut libc::c_char = buf.as_mut_ptr();
                if snprintf(
                    buf.as_mut_ptr(),
                    ::std::mem::size_of::<[libc::c_char; 1024]>() as libc::c_ulong,
                    b"%s%s%s\0" as *const u8 as *const libc::c_char,
                    (*u).url,
                    (*f).name,
                    if (*f).type_0 as libc::c_uint
                        == FT_DIRECTORY as libc::c_int as libc::c_uint
                    {
                        b"/\0" as *const u8 as *const libc::c_char
                    } else {
                        b"\0" as *const u8 as *const libc::c_char
                    },
                ) as libc::c_uint as libc::c_ulong
                    >= ::std::mem::size_of::<[libc::c_char; 1024]>() as libc::c_ulong
                {
                    url = aprintf(
                        b"%s%s%s\0" as *const u8 as *const libc::c_char,
                        (*u).url,
                        (*f).name,
                        if (*f).type_0 as libc::c_uint
                            == FT_DIRECTORY as libc::c_int as libc::c_uint
                        {
                            b"/\0" as *const u8 as *const libc::c_char
                        } else {
                            b"\0" as *const u8 as *const libc::c_char
                        },
                    );
                }
                if !accept_url(url) {
                    logprintf(
                        LOG_VERBOSE,
                        dcgettext(
                            0 as *const libc::c_char,
                            b"%s is excluded/not-included through regex.\n\0"
                                as *const u8 as *const libc::c_char,
                            5 as libc::c_int,
                        ),
                        url,
                    );
                    f = delelement(&mut f, &mut start);
                    if url != buf.as_mut_ptr() {
                        rpl_free(url as *mut libc::c_void);
                        url = 0 as *mut libc::c_char;
                    }
                    continue;
                } else if url != buf.as_mut_ptr() {
                    rpl_free(url as *mut libc::c_void);
                    url = 0 as *mut libc::c_char;
                }
            }
            if *(*u).file != 0 {
                if action == GLOB_GLOBALL as libc::c_int {
                    let mut matchres: libc::c_int = matcher
                        .unwrap()((*u).file, (*f).name, 0 as libc::c_int);
                    if matchres == -(1 as libc::c_int) {
                        logprintf(
                            LOG_NOTQUIET,
                            dcgettext(
                                0 as *const libc::c_char,
                                b"Error matching %s against %s: %s\n\0" as *const u8
                                    as *const libc::c_char,
                                5 as libc::c_int,
                            ),
                            (*u).file,
                            quotearg_style(escape_quoting_style, (*f).name),
                            strerror(*__errno_location()),
                        );
                        freefileinfo(start);
                        return RETRBADPATTERN;
                    }
                    if matchres == 1 as libc::c_int {
                        f = delelement(&mut f, &mut start);
                        continue;
                    }
                } else if action == GLOB_GETONE as libc::c_int {
                    if 0 as libc::c_int != cmp.unwrap()((*u).file, (*f).name) {
                        f = delelement(&mut f, &mut start);
                        continue;
                    }
                }
            }
            f = (*f).next;
        }
    }
    if !start.is_null() {
        res = ftp_retrieve_list(u, original_url, start, con);
    } else if action == GLOB_GLOBALL as libc::c_int {
        logprintf(
            LOG_VERBOSE,
            dcgettext(
                0 as *const libc::c_char,
                b"No matches on pattern %s.\n\0" as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ),
            quote((*u).file),
        );
    } else if action == GLOB_GETONE as libc::c_int {
        (*con).st |= ON_YOUR_OWN as libc::c_int;
        res = ftp_loop_internal(
            u,
            original_url,
            0 as *mut fileinfo,
            con,
            0 as *mut *mut libc::c_char,
            0 as libc::c_int != 0,
        );
        return res;
    }
    freefileinfo(start);
    if opt.quota != 0 && total_downloaded_bytes > opt.quota {
        return QUOTEXC
    } else {
        return res
    };
}
pub unsafe extern "C" fn ftp_loop(
    mut u: *mut url,
    mut original_url: *mut url,
    mut local_file: *mut *mut libc::c_char,
    mut dt: *mut libc::c_int,
    mut proxy: *mut url,
    mut recursive: bool,
    mut glob: bool,
) -> uerr_t {
    let mut con: ccon = ccon {
        st: 0,
        cmd: 0,
        csock: 0,
        dltime: 0.,
        rs: ST_UNIX,
        rsu: UST_TYPE_L8,
        id: 0 as *mut libc::c_char,
        target: 0 as *mut libc::c_char,
        proxy: 0 as *mut url,
    };
    let mut res: uerr_t = NOCONERROR;
    *dt = 0 as libc::c_int;
    memset(
        &mut con as *mut ccon as *mut libc::c_void,
        '\0' as i32,
        ::std::mem::size_of::<ccon>() as libc::c_ulong,
    );
    con.csock = -(1 as libc::c_int);
    con.st = ON_YOUR_OWN as libc::c_int;
    con.rs = ST_UNIX;
    con.id = 0 as *mut libc::c_char;
    con.proxy = proxy;
    if *(*u).file == 0 && !recursive {
        let mut f: *mut fileinfo = 0 as *mut fileinfo;
        res = ftp_get_listing(u, original_url, &mut con, &mut f);
        if res as libc::c_uint == RETROK as libc::c_int as libc::c_uint {
            if opt.htmlify as libc::c_int != 0 && !opt.spider {
                let mut url_file: *mut url = if opt.trustservernames as libc::c_int != 0
                {
                    u
                } else {
                    original_url
                };
                let mut filename: *mut libc::c_char = if !(opt.output_document).is_null()
                {
                    xstrdup(opt.output_document)
                } else if !(con.target).is_null() {
                    xstrdup(con.target)
                } else {
                    url_file_name(url_file, 0 as *mut libc::c_char)
                };
                res = ftp_index(filename, u, f);
                if res as libc::c_uint == FTPOK as libc::c_int as libc::c_uint
                    && opt.verbose != 0
                {
                    if (opt.output_document).is_null() {
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
                        let mut sz: wgint = 0;
                        if stat(filename, &mut st) == 0 as libc::c_int {
                            sz = st.st_size;
                        } else {
                            sz = -(1 as libc::c_int) as wgint;
                        }
                        logprintf(
                            LOG_NOTQUIET,
                            dcgettext(
                                0 as *const libc::c_char,
                                b"Wrote HTML-ized index to %s [%s].\n\0" as *const u8
                                    as *const libc::c_char,
                                5 as libc::c_int,
                            ),
                            quote(filename),
                            number_to_static_string(sz),
                        );
                    } else {
                        logprintf(
                            LOG_NOTQUIET,
                            dcgettext(
                                0 as *const libc::c_char,
                                b"Wrote HTML-ized index to %s.\n\0" as *const u8
                                    as *const libc::c_char,
                                5 as libc::c_int,
                            ),
                            quote(filename),
                        );
                    }
                }
                rpl_free(filename as *mut libc::c_void);
                filename = 0 as *mut libc::c_char;
            }
            freefileinfo(f);
        }
    } else {
        let mut ispattern: bool = 0 as libc::c_int != 0;
        if glob {
            let mut file_part: *mut libc::c_char = strrchr((*u).path, '/' as i32);
            if file_part.is_null() {
                file_part = (*u).path;
            }
            ispattern = has_wildcards_p(file_part);
        }
        if ispattern as libc::c_int != 0 || recursive as libc::c_int != 0
            || opt.timestamping as libc::c_int != 0
            || opt.preserve_perm as libc::c_int != 0
        {
            res = ftp_retrieve_glob(
                u,
                original_url,
                &mut con,
                if ispattern as libc::c_int != 0 {
                    GLOB_GLOBALL as libc::c_int
                } else {
                    GLOB_GETONE as libc::c_int
                },
            );
        } else {
            res = ftp_loop_internal(
                u,
                original_url,
                0 as *mut fileinfo,
                &mut con,
                local_file,
                0 as libc::c_int != 0,
            );
        }
    }
    if res as libc::c_uint == FTPOK as libc::c_int as libc::c_uint {
        res = RETROK;
    }
    if res as libc::c_uint == RETROK as libc::c_int as libc::c_uint {
        *dt |= RETROKF as libc::c_int;
    }
    if con.csock != -(1 as libc::c_int) {
        fd_close(con.csock);
    }
    rpl_free(con.id as *mut libc::c_void);
    con.id = 0 as *mut libc::c_char;
    rpl_free(con.target as *mut libc::c_void);
    con.target = 0 as *mut libc::c_char;
    return res;
}
unsafe extern "C" fn delelement(
    mut f: *mut *mut fileinfo,
    mut start: *mut *mut fileinfo,
) -> *mut fileinfo {
    let mut prev: *mut fileinfo = (**f).prev;
    let mut next: *mut fileinfo = (**f).next;
    rpl_free((**f).name as *mut libc::c_void);
    (**f).name = 0 as *mut libc::c_char;
    rpl_free((**f).linkto as *mut libc::c_void);
    (**f).linkto = 0 as *mut libc::c_char;
    rpl_free(*f as *mut libc::c_void);
    *f = 0 as *mut fileinfo;
    if !next.is_null() {
        (*next).prev = prev;
    }
    if !prev.is_null() {
        (*prev).next = next;
    } else {
        *start = next;
    }
    return next;
}
pub unsafe extern "C" fn freefileinfo(mut f: *mut fileinfo) {
    while !f.is_null() {
        let mut next: *mut fileinfo = (*f).next;
        rpl_free((*f).name as *mut libc::c_void);
        (*f).name = 0 as *mut libc::c_char;
        rpl_free((*f).linkto as *mut libc::c_void);
        (*f).linkto = 0 as *mut libc::c_char;
        rpl_free(f as *mut libc::c_void);
        f = 0 as *mut fileinfo;
        f = next;
    }
}
