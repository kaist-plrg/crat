use ::libc;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    pub type hash_table;
    pub type cookie_jar;
    pub type address_list;
    fn fclose(__stream: *mut FILE) -> libc::c_int;
    fn fdopen(__fd: libc::c_int, __modes: *const libc::c_char) -> *mut FILE;
    fn sprintf(_: *mut libc::c_char, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn snprintf(
        _: *mut libc::c_char,
        _: libc::c_ulong,
        _: *const libc::c_char,
        _: ...
    ) -> libc::c_int;
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
    fn ftello(__stream: *mut FILE) -> __off_t;
    fn feof(__stream: *mut FILE) -> libc::c_int;
    fn time(__timer: *mut time_t) -> time_t;
    fn strptime(
        __s: *const libc::c_char,
        __fmt: *const libc::c_char,
        __tp: *mut tm,
    ) -> *mut libc::c_char;
    fn gmtime(__timer: *const time_t) -> *mut tm;
    fn rpl_timegm(__tm: *mut tm) -> time_t;
    fn __xstat(
        __ver: libc::c_int,
        __filename: *const libc::c_char,
        __stat_buf: *mut stat,
    ) -> libc::c_int;
    fn dcgettext(
        __domainname: *const libc::c_char,
        __msgid: *const libc::c_char,
        __category: libc::c_int,
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
    fn memcmp(
        _: *const libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> libc::c_int;
    fn memchr(
        _: *const libc::c_void,
        _: libc::c_int,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn memrchr(
        __s: *const libc::c_void,
        __c: libc::c_int,
        __n: size_t,
    ) -> *mut libc::c_void;
    fn strcpy(_: *mut libc::c_char, _: *const libc::c_char) -> *mut libc::c_char;
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    fn strncmp(
        _: *const libc::c_char,
        _: *const libc::c_char,
        _: libc::c_ulong,
    ) -> libc::c_int;
    fn strchr(_: *const libc::c_char, _: libc::c_int) -> *mut libc::c_char;
    fn strrchr(_: *const libc::c_char, _: libc::c_int) -> *mut libc::c_char;
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    fn strerror(_: libc::c_int) -> *mut libc::c_char;
    fn strcasecmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    fn strncasecmp(
        _: *const libc::c_char,
        _: *const libc::c_char,
        _: libc::c_ulong,
    ) -> libc::c_int;
    fn rpl_free(_: *mut libc::c_void);
    static mut opt: options;
    static mut exec_name: *const libc::c_char;
    fn set_content_encoding(i: *mut iri, charset: *const libc::c_char);
    fn parse_charset(str: *const libc::c_char) -> *mut libc::c_char;
    fn quotearg_style(s: quoting_style, arg: *const libc::c_char) -> *mut libc::c_char;
    fn quote(arg: *const libc::c_char) -> *const libc::c_char;
    fn escnonprint_uri(_: *const libc::c_char) -> *const libc::c_char;
    fn logputs(_: log_options, _: *const libc::c_char);
    fn debug_logprintf(_: *const libc::c_char, _: ...);
    fn logprintf(_: log_options, _: *const libc::c_char, _: ...);
    fn rpl_fopen(filename: *const libc::c_char, mode: *const libc::c_char) -> *mut FILE;
    fn rpl_strtol(
        string: *const libc::c_char,
        endptr: *mut *mut libc::c_char,
        base: libc::c_int,
    ) -> libc::c_long;
    fn rpl_strtoll(
        string: *const libc::c_char,
        endptr: *mut *mut libc::c_char,
        base: libc::c_int,
    ) -> libc::c_longlong;
    fn xmalloc(s: size_t) -> *mut libc::c_void;
    fn xcalloc(n: size_t, s: size_t) -> *mut libc::c_void;
    fn xrealloc(p: *mut libc::c_void, s: size_t) -> *mut libc::c_void;
    fn xstrdup(str: *const libc::c_char) -> *mut libc::c_char;
    fn strtol(
        _: *const libc::c_char,
        _: *mut *mut libc::c_char,
        _: libc::c_int,
    ) -> libc::c_long;
    fn abort() -> !;
    fn unlink(__name: *const libc::c_char) -> libc::c_int;
    fn __errno_location() -> *mut libc::c_int;
    fn open(__file: *const libc::c_char, __oflag: libc::c_int, _: ...) -> libc::c_int;
    fn hash_table_contains(_: *const hash_table, _: *const libc::c_void) -> libc::c_int;
    fn hash_table_put(
        _: *mut hash_table,
        _: *const libc::c_void,
        _: *const libc::c_void,
    );
    fn make_nocase_string_hash_table(_: libc::c_int) -> *mut hash_table;
    fn datetime_str(_: time_t) -> *mut libc::c_char;
    fn strdupdelim(_: *const libc::c_char, _: *const libc::c_char) -> *mut libc::c_char;
    fn aprintf(_: *const libc::c_char, _: ...) -> *mut libc::c_char;
    fn concat_strings(_: *const libc::c_char, _: ...) -> *mut libc::c_char;
    fn touch(_: *const libc::c_char, _: time_t);
    fn file_exists_p(_: *const libc::c_char, _: *mut file_stats_t) -> bool;
    fn file_size(_: *const libc::c_char) -> wgint;
    fn unique_name_passthrough(_: *const libc::c_char) -> *mut libc::c_char;
    fn fopen_excl(_: *const libc::c_char, _: libc::c_int) -> *mut FILE;
    fn acceptable(_: *const libc::c_char) -> bool;
    fn has_wildcards_p(_: *const libc::c_char) -> bool;
    fn has_html_suffix_p(_: *const libc::c_char) -> bool;
    fn human_readable(_: wgint, _: libc::c_int, _: libc::c_int) -> *mut libc::c_char;
    fn number_to_static_string(_: wgint) -> *mut libc::c_char;
    fn random_number(_: libc::c_int) -> libc::c_int;
    fn wget_base64_encode(
        _: *const libc::c_void,
        _: size_t,
        _: *mut libc::c_char,
    ) -> size_t;
    fn url_unescape(_: *mut libc::c_char);
    fn url_full_path(_: *const url) -> *mut libc::c_char;
    fn scheme_default_port(_: url_scheme) -> libc::c_int;
    fn url_string(_: *const url, _: url_auth_mode) -> *mut libc::c_char;
    fn url_file_name(_: *const url, _: *mut libc::c_char) -> *mut libc::c_char;
    fn mkalldirs(_: *const libc::c_char) -> libc::c_int;
    fn address_list_release(_: *mut address_list);
    fn address_list_contains(_: *const address_list, _: *const ip_address) -> bool;
    fn lookup_host(_: *const libc::c_char, _: libc::c_int) -> *mut address_list;
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
    fn fd_read_hunk(
        _: libc::c_int,
        _: hunk_terminator_t,
        _: libc::c_long,
        _: libc::c_long,
    ) -> *mut libc::c_char;
    fn fd_read_line(_: libc::c_int) -> *mut libc::c_char;
    fn retr_rate(_: wgint, _: libc::c_double) -> *const libc::c_char;
    fn printwhat(_: libc::c_int, _: libc::c_int);
    fn sleep_between_retrievals(_: libc::c_int);
    fn rotate_backups(_: *const libc::c_char);
    fn set_local_file(_: *mut *const libc::c_char, _: *const libc::c_char);
    fn connect_to_host(_: *const libc::c_char, _: libc::c_int) -> libc::c_int;
    fn socket_ip_address(_: libc::c_int, _: *mut ip_address, _: libc::c_int) -> bool;
    fn socket_family(sock: libc::c_int, endpoint: libc::c_int) -> libc::c_int;
    fn retryable_socket_connect_error(_: libc::c_int) -> bool;
    fn test_socket_open(_: libc::c_int) -> bool;
    fn fd_read(
        _: libc::c_int,
        _: *mut libc::c_char,
        _: libc::c_int,
        _: libc::c_double,
    ) -> libc::c_int;
    fn fd_write(
        _: libc::c_int,
        _: *mut libc::c_char,
        _: libc::c_int,
        _: libc::c_double,
    ) -> libc::c_int;
    fn fd_errstr(_: libc::c_int) -> *const libc::c_char;
    fn fd_close(_: libc::c_int);
    fn search_netrc(
        _: *const libc::c_char,
        _: *mut *const libc::c_char,
        _: *mut *const libc::c_char,
        _: libc::c_int,
        _: *mut FILE,
    );
    fn ntlm_input(_: *mut ntlmdata, _: *const libc::c_char) -> bool;
    fn ntlm_output(
        _: *mut ntlmdata,
        _: *const libc::c_char,
        _: *const libc::c_char,
        _: *mut bool,
    ) -> *mut libc::c_char;
    fn cookie_jar_new() -> *mut cookie_jar;
    fn cookie_handle_set_cookie(
        _: *mut cookie_jar,
        _: *const libc::c_char,
        _: libc::c_int,
        _: *const libc::c_char,
        _: *const libc::c_char,
    );
    fn cookie_header(
        _: *mut cookie_jar,
        _: *const libc::c_char,
        _: libc::c_int,
        _: *const libc::c_char,
        _: bool,
    ) -> *mut libc::c_char;
    fn cookie_jar_load(_: *mut cookie_jar, _: *const libc::c_char);
    fn cookie_jar_save(_: *mut cookie_jar, _: *const libc::c_char);
    fn md5_init_ctx(ctx: *mut md5_ctx);
    fn md5_process_bytes(buffer: *const libc::c_void, len: size_t, ctx: *mut md5_ctx);
    fn md5_finish_ctx(ctx: *mut md5_ctx, resbuf: *mut libc::c_void) -> *mut libc::c_void;
    fn downloaded_file(
        _: downloaded_file_t,
        _: *const libc::c_char,
    ) -> downloaded_file_t;
    fn nonexisting_url(_: *const libc::c_char);
    fn warc_uuid_str(id_str: *mut libc::c_char, urn_size: size_t);
    fn warc_timestamp(
        timestamp: *mut libc::c_char,
        timestamp_size: size_t,
    ) -> *mut libc::c_char;
    fn warc_tempfile() -> *mut FILE;
    fn warc_write_request_record(
        url: *const libc::c_char,
        timestamp_str: *const libc::c_char,
        concurrent_to_uuid: *const libc::c_char,
        ip: *const ip_address,
        body: *mut FILE,
        payload_offset: off_t,
    ) -> bool;
    fn warc_write_response_record(
        url: *const libc::c_char,
        timestamp_str: *const libc::c_char,
        concurrent_to_uuid: *const libc::c_char,
        ip: *const ip_address,
        body: *mut FILE,
        payload_offset: off_t,
        mime_type: *const libc::c_char,
        response_code: libc::c_int,
        redirect_location: *const libc::c_char,
    ) -> bool;
    fn c_strcasecmp(s1: *const libc::c_char, s2: *const libc::c_char) -> libc::c_int;
    fn c_strncasecmp(
        s1: *const libc::c_char,
        s2: *const libc::c_char,
        n: size_t,
    ) -> libc::c_int;
    static mut version_string: *const libc::c_char;
    fn xstrndup(string: *const libc::c_char, n: size_t) -> *mut libc::c_char;
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
pub type __syscall_slong_t = libc::c_long;
pub type ino_t = __ino_t;
pub type dev_t = __dev_t;
pub type off_t = __off_t;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct iri {
    pub uri_encoding: *mut libc::c_char,
    pub content_encoding: *mut libc::c_char,
    pub orig_url: *mut libc::c_char,
    pub utf8_encode: bool,
}
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
pub type url_scheme = libc::c_uint;
pub const SCHEME_INVALID: url_scheme = 2;
pub const SCHEME_FTP: url_scheme = 1;
pub const SCHEME_HTTP: url_scheme = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct http_stat {
    pub len: wgint,
    pub contlen: wgint,
    pub restval: wgint,
    pub res: libc::c_int,
    pub rderrmsg: *mut libc::c_char,
    pub newloc: *mut libc::c_char,
    pub remote_time: *mut libc::c_char,
    pub error: *mut libc::c_char,
    pub statcode: libc::c_int,
    pub message: *mut libc::c_char,
    pub rd_size: wgint,
    pub dltime: libc::c_double,
    pub referer: *const libc::c_char,
    pub local_file: *mut libc::c_char,
    pub existence_checked: bool,
    pub timestamp_checked: bool,
    pub orig_file_name: *mut libc::c_char,
    pub orig_file_size: wgint,
    pub orig_file_tstamp: time_t,
    pub local_encoding: encoding_t,
    pub remote_encoding: encoding_t,
    pub temporary: bool,
}
pub type encoding_t = libc::c_int;
pub const ENC_BROTLI: encoding_t = 4;
pub const ENC_COMPRESS: encoding_t = 3;
pub const ENC_DEFLATE: encoding_t = 2;
pub const ENC_GZIP: encoding_t = 1;
pub const ENC_NONE: encoding_t = 0;
pub const ENC_INVALID: encoding_t = -1;
pub type downloaded_file_t = libc::c_uint;
pub const CHECK_FOR_FILE: downloaded_file_t = 3;
pub const FILE_DOWNLOADED_AND_HTML_EXTENSION_ADDED: downloaded_file_t = 2;
pub const FILE_DOWNLOADED_NORMALLY: downloaded_file_t = 1;
pub const FILE_NOT_ALREADY_DOWNLOADED: downloaded_file_t = 0;
pub type url_auth_mode = libc::c_uint;
pub const URL_AUTH_HIDE: url_auth_mode = 2;
pub const URL_AUTH_HIDE_PASSWD: url_auth_mode = 1;
pub const URL_AUTH_SHOW: url_auth_mode = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct request {
    pub method: *const libc::c_char,
    pub arg: *mut libc::c_char,
    pub headers: *mut request_header,
    pub hcount: libc::c_int,
    pub hcapacity: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct request_header {
    pub name: *mut libc::c_char,
    pub value: *mut libc::c_char,
    pub release_policy: rp,
}
pub type rp = libc::c_uint;
pub const rel_both: rp = 3;
pub const rel_value: rp = 2;
pub const rel_name: rp = 1;
pub const rel_none: rp = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct response {
    pub data: *const libc::c_char,
    pub headers: *mut *const libc::c_char,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_4 {
    pub socket: libc::c_int,
    pub host: *mut libc::c_char,
    pub port: libc::c_int,
    pub ssl: bool,
    pub authorized: bool,
    pub ntlm: ntlmdata,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ntlmdata {
    pub state: wgetntlm,
    pub nonce: [libc::c_uchar; 8],
}
pub type wgetntlm = libc::c_uint;
pub const NTLMSTATE_LAST: wgetntlm = 4;
pub const NTLMSTATE_TYPE3: wgetntlm = 3;
pub const NTLMSTATE_TYPE2: wgetntlm = 2;
pub const NTLMSTATE_TYPE1: wgetntlm = 1;
pub const NTLMSTATE_NONE: wgetntlm = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ip_address {
    pub family: libc::c_int,
    pub data: C2RustUnnamed_5,
    pub ipv6_scope: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_5 {
    pub d4: in_addr,
    pub d6: in6_addr,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct in6_addr {
    pub __in6_u: C2RustUnnamed_6,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_6 {
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
pub const rb_compressed_gzip: C2RustUnnamed_9 = 8;
pub const rb_chunked_transfer_encoding: C2RustUnnamed_9 = 4;
pub const rb_skip_startpos: C2RustUnnamed_9 = 2;
pub const rb_read_exactly: C2RustUnnamed_9 = 1;
pub type file_stats_t = file_stat_s;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct file_stat_s {
    pub access_err: libc::c_int,
    pub st_ino: ino_t,
    pub st_dev: dev_t,
}
pub const SKIP_SIZE: C2RustUnnamed_7 = 512;
pub const SKIP_THRESHOLD: C2RustUnnamed_7 = 4096;
pub type C2RustUnnamed_7 = libc::c_uint;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct param_token {
    pub b: *const libc::c_char,
    pub e: *const libc::c_char,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct md5_ctx {
    pub A: uint32_t,
    pub B: uint32_t,
    pub C: uint32_t,
    pub D: uint32_t,
    pub total: [uint32_t; 2],
    pub buflen: uint32_t,
    pub buffer: [uint32_t; 32],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_8 {
    pub name: *const libc::c_char,
    pub variable: *mut *mut libc::c_char,
}
pub type hunk_terminator_t = Option::<
    unsafe extern "C" fn(
        *const libc::c_char,
        *const libc::c_char,
        libc::c_int,
    ) -> *const libc::c_char,
>;
pub const ENDPOINT_PEER: C2RustUnnamed_11 = 1;
pub const E_HOST: C2RustUnnamed_10 = -100;
pub type C2RustUnnamed_9 = libc::c_uint;
pub type C2RustUnnamed_10 = libc::c_int;
pub type C2RustUnnamed_11 = libc::c_uint;
pub const ENDPOINT_LOCAL: C2RustUnnamed_11 = 0;
#[inline]
unsafe extern "C" fn stat(
    mut __path: *const libc::c_char,
    mut __statbuf: *mut stat,
) -> libc::c_int {
    return __xstat(1 as libc::c_int, __path, __statbuf);
}
#[inline]
unsafe extern "C" fn c_isdigit(mut c: libc::c_int) -> bool {
    match c {
        48 | 49 | 50 | 51 | 52 | 53 | 54 | 55 | 56 | 57 => return 1 as libc::c_int != 0,
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
unsafe extern "C" fn c_toupper(mut c: libc::c_int) -> libc::c_int {
    match c {
        97 | 98 | 99 | 100 | 101 | 102 | 103 | 104 | 105 | 106 | 107 | 108 | 109 | 110
        | 111 | 112 | 113 | 114 | 115 | 116 | 117 | 118 | 119 | 120 | 121 | 122 => {
            return c - 'a' as i32 + 'A' as i32;
        }
        _ => return c,
    };
}
#[inline]
unsafe extern "C" fn atoi(mut __nptr: *const libc::c_char) -> libc::c_int {
    return strtol(
        __nptr,
        0 as *mut libc::c_void as *mut *mut libc::c_char,
        10 as libc::c_int,
    ) as libc::c_int;
}
static mut cookies_loaded_p: bool = false;
static mut wget_cookie_jar: *mut cookie_jar = 0 as *const cookie_jar as *mut cookie_jar;
unsafe extern "C" fn request_new(
    mut method: *const libc::c_char,
    mut arg: *mut libc::c_char,
) -> *mut request {
    let mut req: *mut request = xcalloc(
        1 as libc::c_int as size_t,
        ::std::mem::size_of::<request>() as libc::c_ulong,
    ) as *mut request;
    (*req).hcapacity = 8 as libc::c_int;
    (*req)
        .headers = xmalloc(
        ((*req).hcapacity as libc::c_ulong)
            .wrapping_mul(::std::mem::size_of::<request_header>() as libc::c_ulong),
    ) as *mut request_header;
    (*req).method = method;
    (*req).arg = arg;
    return req;
}
unsafe extern "C" fn request_method(mut req: *const request) -> *const libc::c_char {
    return (*req).method;
}
unsafe extern "C" fn release_header(mut hdr: *mut request_header) {
    match (*hdr).release_policy as libc::c_uint {
        1 => {
            rpl_free((*hdr).name as *mut libc::c_void);
            (*hdr).name = 0 as *mut libc::c_char;
        }
        2 => {
            rpl_free((*hdr).value as *mut libc::c_void);
            (*hdr).value = 0 as *mut libc::c_char;
        }
        3 => {
            rpl_free((*hdr).name as *mut libc::c_void);
            (*hdr).name = 0 as *mut libc::c_char;
            rpl_free((*hdr).value as *mut libc::c_void);
            (*hdr).value = 0 as *mut libc::c_char;
        }
        0 | _ => {}
    };
}
unsafe extern "C" fn request_set_header(
    mut req: *mut request,
    mut name: *const libc::c_char,
    mut value: *const libc::c_char,
    mut release_policy: rp,
) {
    let mut hdr: *mut request_header = 0 as *mut request_header;
    let mut i: libc::c_int = 0;
    if value.is_null() {
        if release_policy as libc::c_uint == rel_name as libc::c_int as libc::c_uint
            || release_policy as libc::c_uint == rel_both as libc::c_int as libc::c_uint
        {
            rpl_free(name as *mut libc::c_void);
            name = 0 as *const libc::c_char;
        }
        return;
    }
    i = 0 as libc::c_int;
    while i < (*req).hcount {
        hdr = &mut *((*req).headers).offset(i as isize) as *mut request_header;
        if 0 as libc::c_int == c_strcasecmp(name, (*hdr).name) {
            release_header(hdr);
            (*hdr).name = name as *mut libc::c_void as *mut libc::c_char;
            (*hdr).value = value as *mut libc::c_void as *mut libc::c_char;
            (*hdr).release_policy = release_policy;
            return;
        }
        i += 1;
        i;
    }
    if (*req).hcount >= (*req).hcapacity {
        (*req).hcapacity <<= 1 as libc::c_int;
        (*req)
            .headers = xrealloc(
            (*req).headers as *mut libc::c_void,
            ((*req).hcapacity as libc::c_ulong)
                .wrapping_mul(::std::mem::size_of::<request_header>() as libc::c_ulong),
        ) as *mut request_header;
    }
    let fresh0 = (*req).hcount;
    (*req).hcount = (*req).hcount + 1;
    hdr = &mut *((*req).headers).offset(fresh0 as isize) as *mut request_header;
    (*hdr).name = name as *mut libc::c_void as *mut libc::c_char;
    (*hdr).value = value as *mut libc::c_void as *mut libc::c_char;
    (*hdr).release_policy = release_policy;
}
unsafe extern "C" fn request_set_user_header(
    mut req: *mut request,
    mut header: *const libc::c_char,
) {
    let mut name: *const libc::c_char = 0 as *const libc::c_char;
    let mut p: *const libc::c_char = 0 as *const libc::c_char;
    p = strchr(header, ':' as i32);
    if p.is_null() {
        return;
    }
    name = xstrndup(header, p.offset_from(header) as libc::c_long as size_t);
    p = p.offset(1);
    p;
    while c_isspace(*p as libc::c_int) {
        p = p.offset(1);
        p;
    }
    request_set_header(req, name, p, rel_name);
}
unsafe extern "C" fn request_remove_header(
    mut req: *mut request,
    mut name: *const libc::c_char,
) -> bool {
    let mut i: libc::c_int = 0;
    i = 0 as libc::c_int;
    while i < (*req).hcount {
        let mut hdr: *mut request_header = &mut *((*req).headers).offset(i as isize)
            as *mut request_header;
        if 0 as libc::c_int == c_strcasecmp(name, (*hdr).name) {
            release_header(hdr);
            if i < (*req).hcount - 1 as libc::c_int {
                memmove(
                    hdr as *mut libc::c_void,
                    hdr.offset(1 as libc::c_int as isize) as *const libc::c_void,
                    (((*req).hcount - i - 1 as libc::c_int) as libc::c_ulong)
                        .wrapping_mul(
                            ::std::mem::size_of::<request_header>() as libc::c_ulong,
                        ),
                );
            }
            (*req).hcount -= 1;
            (*req).hcount;
            return 1 as libc::c_int != 0;
        }
        i += 1;
        i;
    }
    return 0 as libc::c_int != 0;
}
unsafe extern "C" fn request_send(
    mut req: *const request,
    mut fd: libc::c_int,
    mut warc_tmp: *mut FILE,
) -> libc::c_int {
    let mut request_string: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut p: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut i: libc::c_int = 0;
    let mut size: libc::c_int = 0;
    let mut write_error: libc::c_int = 0;
    size = 0 as libc::c_int;
    size = (size as libc::c_ulong)
        .wrapping_add(
            (strlen((*req).method))
                .wrapping_add(1 as libc::c_int as libc::c_ulong)
                .wrapping_add(strlen((*req).arg))
                .wrapping_add(1 as libc::c_int as libc::c_ulong)
                .wrapping_add(8 as libc::c_int as libc::c_ulong)
                .wrapping_add(2 as libc::c_int as libc::c_ulong),
        ) as libc::c_int as libc::c_int;
    i = 0 as libc::c_int;
    while i < (*req).hcount {
        let mut hdr: *mut request_header = &mut *((*req).headers).offset(i as isize)
            as *mut request_header;
        size = (size as libc::c_ulong)
            .wrapping_add(
                (strlen((*hdr).name))
                    .wrapping_add(2 as libc::c_int as libc::c_ulong)
                    .wrapping_add(strlen((*hdr).value))
                    .wrapping_add(2 as libc::c_int as libc::c_ulong),
            ) as libc::c_int as libc::c_int;
        i += 1;
        i;
    }
    size += 3 as libc::c_int;
    request_string = xmalloc(size as size_t) as *mut libc::c_char;
    p = request_string;
    let mut A_len: libc::c_int = strlen((*req).method) as libc::c_int;
    memcpy(
        p as *mut libc::c_void,
        (*req).method as *const libc::c_void,
        A_len as libc::c_ulong,
    );
    p = p.offset(A_len as isize);
    let fresh1 = p;
    p = p.offset(1);
    *fresh1 = ' ' as i32 as libc::c_char;
    let mut A_len_0: libc::c_int = strlen((*req).arg) as libc::c_int;
    memcpy(
        p as *mut libc::c_void,
        (*req).arg as *const libc::c_void,
        A_len_0 as libc::c_ulong,
    );
    p = p.offset(A_len_0 as isize);
    let fresh2 = p;
    p = p.offset(1);
    *fresh2 = ' ' as i32 as libc::c_char;
    memcpy(
        p as *mut libc::c_void,
        b"HTTP/1.1\r\n\0" as *const u8 as *const libc::c_char as *const libc::c_void,
        10 as libc::c_int as libc::c_ulong,
    );
    p = p.offset(10 as libc::c_int as isize);
    i = 0 as libc::c_int;
    while i < (*req).hcount {
        let mut hdr_0: *mut request_header = &mut *((*req).headers).offset(i as isize)
            as *mut request_header;
        let mut A_len_1: libc::c_int = strlen((*hdr_0).name) as libc::c_int;
        memcpy(
            p as *mut libc::c_void,
            (*hdr_0).name as *const libc::c_void,
            A_len_1 as libc::c_ulong,
        );
        p = p.offset(A_len_1 as isize);
        let fresh3 = p;
        p = p.offset(1);
        *fresh3 = ':' as i32 as libc::c_char;
        let fresh4 = p;
        p = p.offset(1);
        *fresh4 = ' ' as i32 as libc::c_char;
        let mut A_len_2: libc::c_int = strlen((*hdr_0).value) as libc::c_int;
        memcpy(
            p as *mut libc::c_void,
            (*hdr_0).value as *const libc::c_void,
            A_len_2 as libc::c_ulong,
        );
        p = p.offset(A_len_2 as isize);
        let fresh5 = p;
        p = p.offset(1);
        *fresh5 = '\r' as i32 as libc::c_char;
        let fresh6 = p;
        p = p.offset(1);
        *fresh6 = '\n' as i32 as libc::c_char;
        i += 1;
        i;
    }
    let fresh7 = p;
    p = p.offset(1);
    *fresh7 = '\r' as i32 as libc::c_char;
    let fresh8 = p;
    p = p.offset(1);
    *fresh8 = '\n' as i32 as libc::c_char;
    let fresh9 = p;
    p = p.offset(1);
    *fresh9 = '\0' as i32 as libc::c_char;
    if opt.debug as libc::c_long != 0 {
        debug_logprintf(
            b"\n---request begin---\n%s---request end---\n\0" as *const u8
                as *const libc::c_char,
            request_string,
        );
    }
    write_error = fd_write(
        fd,
        request_string,
        size - 1 as libc::c_int,
        -(1 as libc::c_int) as libc::c_double,
    );
    if write_error < 0 as libc::c_int {
        logprintf(
            LOG_VERBOSE,
            dcgettext(
                0 as *const libc::c_char,
                b"Failed writing HTTP request: %s.\n\0" as *const u8
                    as *const libc::c_char,
                5 as libc::c_int,
            ),
            fd_errstr(fd),
        );
    } else if !warc_tmp.is_null() {
        let mut warc_tmp_written: libc::c_int = fwrite(
            request_string as *const libc::c_void,
            1 as libc::c_int as libc::c_ulong,
            (size - 1 as libc::c_int) as libc::c_ulong,
            warc_tmp,
        ) as libc::c_int;
        if warc_tmp_written != size - 1 as libc::c_int {
            write_error = -(2 as libc::c_int);
        }
    }
    rpl_free(request_string as *mut libc::c_void);
    request_string = 0 as *mut libc::c_char;
    return write_error;
}
unsafe extern "C" fn request_free(mut req_ref: *mut *mut request) {
    let mut i: libc::c_int = 0;
    let mut req: *mut request = *req_ref;
    if req.is_null() {
        return;
    }
    rpl_free((*req).arg as *mut libc::c_void);
    (*req).arg = 0 as *mut libc::c_char;
    i = 0 as libc::c_int;
    while i < (*req).hcount {
        release_header(&mut *((*req).headers).offset(i as isize));
        i += 1;
        i;
    }
    rpl_free((*req).headers as *mut libc::c_void);
    (*req).headers = 0 as *mut request_header;
    rpl_free(req as *mut libc::c_void);
    req = 0 as *mut request;
    *req_ref = 0 as *mut request;
}
static mut basic_authed_hosts: *mut hash_table = 0 as *const hash_table
    as *mut hash_table;
unsafe extern "C" fn maybe_send_basic_creds(
    mut hostname: *const libc::c_char,
    mut user: *const libc::c_char,
    mut passwd: *const libc::c_char,
    mut req: *mut request,
) -> bool {
    let mut do_challenge: bool = 0 as libc::c_int != 0;
    if opt.auth_without_challenge {
        if opt.debug as libc::c_long != 0 {
            debug_logprintf(
                b"Auth-without-challenge set, sending Basic credentials.\n\0"
                    as *const u8 as *const libc::c_char,
            );
        }
        do_challenge = 1 as libc::c_int != 0;
    } else if !basic_authed_hosts.is_null()
        && hash_table_contains(basic_authed_hosts, hostname as *const libc::c_void) != 0
    {
        if opt.debug as libc::c_long != 0 {
            debug_logprintf(
                b"Found %s in basic_authed_hosts.\n\0" as *const u8
                    as *const libc::c_char,
                quote(hostname),
            );
        }
        do_challenge = 1 as libc::c_int != 0;
    } else if opt.debug as libc::c_long != 0 {
        debug_logprintf(
            b"Host %s has not issued a general basic challenge.\n\0" as *const u8
                as *const libc::c_char,
            quote(hostname),
        );
    }
    if do_challenge {
        request_set_header(
            req,
            b"Authorization\0" as *const u8 as *const libc::c_char,
            basic_authentication_encode(user, passwd),
            rel_value,
        );
    }
    return do_challenge;
}
unsafe extern "C" fn register_basic_auth_host(mut hostname: *const libc::c_char) {
    if basic_authed_hosts.is_null() {
        basic_authed_hosts = make_nocase_string_hash_table(1 as libc::c_int);
    }
    if hash_table_contains(basic_authed_hosts, hostname as *const libc::c_void) == 0 {
        hash_table_put(
            basic_authed_hosts,
            xstrdup(hostname) as *const libc::c_void,
            0 as *const libc::c_void,
        );
        if opt.debug as libc::c_long != 0 {
            debug_logprintf(
                b"Inserted %s into basic_authed_hosts\n\0" as *const u8
                    as *const libc::c_char,
                quote(hostname),
            );
        }
    }
}
unsafe extern "C" fn body_file_send(
    mut sock: libc::c_int,
    mut file_name: *const libc::c_char,
    mut promised_size: wgint,
    mut warc_tmp: *mut FILE,
) -> libc::c_int {
    static mut chunk: [libc::c_char; 8192] = [0; 8192];
    let mut written: wgint = 0 as libc::c_int as wgint;
    let mut write_error: libc::c_int = 0;
    let mut fp: *mut FILE = 0 as *mut FILE;
    if opt.debug as libc::c_long != 0 {
        debug_logprintf(
            b"[writing BODY file %s ... \0" as *const u8 as *const libc::c_char,
            file_name,
        );
    }
    fp = rpl_fopen(file_name, b"rb\0" as *const u8 as *const libc::c_char);
    if fp.is_null() {
        return -(1 as libc::c_int);
    }
    while feof(fp) == 0 && written < promised_size {
        let mut towrite: libc::c_int = 0;
        let mut length: libc::c_int = fread(
            chunk.as_mut_ptr() as *mut libc::c_void,
            1 as libc::c_int as libc::c_ulong,
            ::std::mem::size_of::<[libc::c_char; 8192]>() as libc::c_ulong,
            fp,
        ) as libc::c_int;
        if length == 0 as libc::c_int {
            break;
        }
        towrite = (if promised_size - written <= length as libc::c_long {
            promised_size - written
        } else {
            length as libc::c_long
        }) as libc::c_int;
        write_error = fd_write(
            sock,
            chunk.as_mut_ptr(),
            towrite,
            -(1 as libc::c_int) as libc::c_double,
        );
        if write_error < 0 as libc::c_int {
            fclose(fp);
            return -(1 as libc::c_int);
        }
        if !warc_tmp.is_null() {
            let mut warc_tmp_written: libc::c_int = fwrite(
                chunk.as_mut_ptr() as *const libc::c_void,
                1 as libc::c_int as libc::c_ulong,
                towrite as libc::c_ulong,
                warc_tmp,
            ) as libc::c_int;
            if warc_tmp_written != towrite {
                fclose(fp);
                return -(2 as libc::c_int);
            }
        }
        written += towrite as libc::c_long;
    }
    fclose(fp);
    if written < promised_size {
        *__errno_location() = 22 as libc::c_int;
        return -(1 as libc::c_int);
    }
    if opt.debug as libc::c_long != 0 {
        debug_logprintf(b"done]\n\0" as *const u8 as *const libc::c_char);
    }
    return 0 as libc::c_int;
}
unsafe extern "C" fn response_head_terminator(
    mut start: *const libc::c_char,
    mut peeked: *const libc::c_char,
    mut peeklen: libc::c_int,
) -> *const libc::c_char {
    let mut p: *const libc::c_char = 0 as *const libc::c_char;
    let mut end: *const libc::c_char = 0 as *const libc::c_char;
    if start == peeked
        && 0 as libc::c_int
            != memcmp(
                start as *const libc::c_void,
                b"HTTP\0" as *const u8 as *const libc::c_char as *const libc::c_void,
                (if peeklen <= 4 as libc::c_int { peeklen } else { 4 as libc::c_int })
                    as libc::c_ulong,
            )
    {
        return start;
    }
    p = if (peeked.offset_from(start) as libc::c_long) < 2 as libc::c_int as libc::c_long
    {
        start
    } else {
        peeked.offset(-(2 as libc::c_int as isize))
    };
    end = peeked.offset(peeklen as isize);
    while p < end.offset(-(2 as libc::c_int as isize)) {
        if *p as libc::c_int == '\n' as i32 {
            if *p.offset(1 as libc::c_int as isize) as libc::c_int == '\r' as i32
                && *p.offset(2 as libc::c_int as isize) as libc::c_int == '\n' as i32
            {
                return p.offset(3 as libc::c_int as isize)
            } else if *p.offset(1 as libc::c_int as isize) as libc::c_int == '\n' as i32
            {
                return p.offset(2 as libc::c_int as isize)
            }
        }
        p = p.offset(1);
        p;
    }
    if peeklen >= 2 as libc::c_int
        && *p.offset(0 as libc::c_int as isize) as libc::c_int == '\n' as i32
        && *p.offset(1 as libc::c_int as isize) as libc::c_int == '\n' as i32
    {
        return p.offset(2 as libc::c_int as isize);
    }
    return 0 as *const libc::c_char;
}
unsafe extern "C" fn read_http_response_head(mut fd: libc::c_int) -> *mut libc::c_char {
    return fd_read_hunk(
        fd,
        Some(
            response_head_terminator
                as unsafe extern "C" fn(
                    *const libc::c_char,
                    *const libc::c_char,
                    libc::c_int,
                ) -> *const libc::c_char,
        ),
        512 as libc::c_int as libc::c_long,
        65536 as libc::c_int as libc::c_long,
    );
}
unsafe extern "C" fn resp_new(mut head: *mut libc::c_char) -> *mut response {
    let mut hdr: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut count: libc::c_int = 0;
    let mut size: libc::c_int = 0;
    let mut resp: *mut response = xcalloc(
        1 as libc::c_int as size_t,
        ::std::mem::size_of::<response>() as libc::c_ulong,
    ) as *mut response;
    (*resp).data = head;
    if *head as libc::c_int == '\0' as i32 {
        return resp;
    }
    count = 0 as libc::c_int;
    size = count;
    hdr = head;
    loop {
        let mut DR_needed_size: libc::c_long = (count + 1 as libc::c_int)
            as libc::c_long;
        let mut DR_newsize: libc::c_long = 0 as libc::c_int as libc::c_long;
        while (size as libc::c_long) < DR_needed_size {
            DR_newsize = (size << 1 as libc::c_int) as libc::c_long;
            if DR_newsize < 16 as libc::c_int as libc::c_long {
                DR_newsize = 16 as libc::c_int as libc::c_long;
            }
            size = DR_newsize as libc::c_int;
        }
        if DR_newsize != 0 {
            (*resp)
                .headers = xrealloc(
                (*resp).headers as *mut libc::c_void,
                (DR_newsize as libc::c_ulong)
                    .wrapping_mul(
                        ::std::mem::size_of::<*const libc::c_char>() as libc::c_ulong,
                    ),
            ) as *mut *const libc::c_char;
        }
        let fresh10 = count;
        count = count + 1;
        let ref mut fresh11 = *((*resp).headers).offset(fresh10 as isize);
        *fresh11 = hdr;
        if *hdr.offset(0 as libc::c_int as isize) == 0
            || *hdr.offset(0 as libc::c_int as isize) as libc::c_int == '\r' as i32
                && *hdr.offset(1 as libc::c_int as isize) as libc::c_int == '\n' as i32
            || *hdr.offset(0 as libc::c_int as isize) as libc::c_int == '\n' as i32
        {
            break;
        }
        loop {
            let mut end: *mut libc::c_char = strchr(hdr, '\n' as i32);
            if end.is_null() {
                hdr = hdr.offset(strlen(hdr) as isize);
                break;
            } else {
                hdr = end.offset(1 as libc::c_int as isize);
                if *hdr as libc::c_int != ' ' as i32
                    && *hdr as libc::c_int != '\t' as i32
                {
                    break;
                }
                *end = ' ' as i32 as libc::c_char;
                if end > head
                    && *end.offset(-(1 as libc::c_int) as isize) as libc::c_int
                        == '\r' as i32
                {
                    *end
                        .offset(
                            -(1 as libc::c_int) as isize,
                        ) = ' ' as i32 as libc::c_char;
                }
            }
        }
    }
    let mut DR_needed_size_0: libc::c_long = (count + 1 as libc::c_int) as libc::c_long;
    let mut DR_newsize_0: libc::c_long = 0 as libc::c_int as libc::c_long;
    while (size as libc::c_long) < DR_needed_size_0 {
        DR_newsize_0 = (size << 1 as libc::c_int) as libc::c_long;
        if DR_newsize_0 < 16 as libc::c_int as libc::c_long {
            DR_newsize_0 = 16 as libc::c_int as libc::c_long;
        }
        size = DR_newsize_0 as libc::c_int;
    }
    if DR_newsize_0 != 0 {
        (*resp)
            .headers = xrealloc(
            (*resp).headers as *mut libc::c_void,
            (DR_newsize_0 as libc::c_ulong)
                .wrapping_mul(
                    ::std::mem::size_of::<*const libc::c_char>() as libc::c_ulong,
                ),
        ) as *mut *const libc::c_char;
    }
    let ref mut fresh12 = *((*resp).headers).offset(count as isize);
    *fresh12 = 0 as *const libc::c_char;
    return resp;
}
unsafe extern "C" fn resp_header_locate(
    mut resp: *const response,
    mut name: *const libc::c_char,
    mut start: libc::c_int,
    mut begptr: *mut *const libc::c_char,
    mut endptr: *mut *const libc::c_char,
) -> libc::c_int {
    let mut i: libc::c_int = 0;
    let mut headers: *mut *const libc::c_char = (*resp).headers;
    let mut name_len: libc::c_int = 0;
    if headers.is_null() || (*headers.offset(1 as libc::c_int as isize)).is_null() {
        return -(1 as libc::c_int);
    }
    name_len = strlen(name) as libc::c_int;
    if start > 0 as libc::c_int {
        i = start;
    } else {
        i = 1 as libc::c_int;
    }
    while !(*headers.offset((i + 1 as libc::c_int) as isize)).is_null() {
        let mut b: *const libc::c_char = *headers.offset(i as isize);
        let mut e: *const libc::c_char = *headers
            .offset((i + 1 as libc::c_int) as isize);
        if e.offset_from(b) as libc::c_long > name_len as libc::c_long
            && *b.offset(name_len as isize) as libc::c_int == ':' as i32
            && 0 as libc::c_int == c_strncasecmp(b, name, name_len as size_t)
        {
            b = b.offset((name_len + 1 as libc::c_int) as isize);
            while b < e && c_isspace(*b as libc::c_int) as libc::c_int != 0 {
                b = b.offset(1);
                b;
            }
            while b < e
                && c_isspace(*e.offset(-(1 as libc::c_int) as isize) as libc::c_int)
                    as libc::c_int != 0
            {
                e = e.offset(-1);
                e;
            }
            *begptr = b;
            *endptr = e;
            return i;
        }
        i += 1;
        i;
    }
    return -(1 as libc::c_int);
}
unsafe extern "C" fn resp_header_get(
    mut resp: *const response,
    mut name: *const libc::c_char,
    mut begptr: *mut *const libc::c_char,
    mut endptr: *mut *const libc::c_char,
) -> bool {
    let mut pos: libc::c_int = resp_header_locate(
        resp,
        name,
        0 as libc::c_int,
        begptr,
        endptr,
    );
    return pos != -(1 as libc::c_int);
}
unsafe extern "C" fn resp_header_copy(
    mut resp: *const response,
    mut name: *const libc::c_char,
    mut buf: *mut libc::c_char,
    mut bufsize: libc::c_int,
) -> bool {
    let mut b: *const libc::c_char = 0 as *const libc::c_char;
    let mut e: *const libc::c_char = 0 as *const libc::c_char;
    if !resp_header_get(resp, name, &mut b, &mut e) {
        return 0 as libc::c_int != 0;
    }
    if bufsize != 0 {
        let mut len: libc::c_int = (if e.offset_from(b) as libc::c_long
            <= (bufsize - 1 as libc::c_int) as libc::c_long
        {
            e.offset_from(b) as libc::c_long
        } else {
            (bufsize - 1 as libc::c_int) as libc::c_long
        }) as libc::c_int;
        memcpy(buf as *mut libc::c_void, b as *const libc::c_void, len as libc::c_ulong);
        *buf.offset(len as isize) = '\0' as i32 as libc::c_char;
    }
    return 1 as libc::c_int != 0;
}
unsafe extern "C" fn resp_header_strdup(
    mut resp: *const response,
    mut name: *const libc::c_char,
) -> *mut libc::c_char {
    let mut b: *const libc::c_char = 0 as *const libc::c_char;
    let mut e: *const libc::c_char = 0 as *const libc::c_char;
    if !resp_header_get(resp, name, &mut b, &mut e) {
        return 0 as *mut libc::c_char;
    }
    return strdupdelim(b, e);
}
unsafe extern "C" fn resp_status(
    mut resp: *const response,
    mut message: *mut *mut libc::c_char,
) -> libc::c_int {
    let mut status: libc::c_int = 0;
    let mut p: *const libc::c_char = 0 as *const libc::c_char;
    let mut end: *const libc::c_char = 0 as *const libc::c_char;
    if ((*resp).headers).is_null() {
        if !message.is_null() {
            *message = xstrdup(
                dcgettext(
                    0 as *const libc::c_char,
                    b"No headers, assuming HTTP/0.9\0" as *const u8
                        as *const libc::c_char,
                    5 as libc::c_int,
                ),
            );
        }
        return 200 as libc::c_int;
    }
    p = *((*resp).headers).offset(0 as libc::c_int as isize);
    end = *((*resp).headers).offset(1 as libc::c_int as isize);
    if end.is_null() {
        return -(1 as libc::c_int);
    }
    if (end.offset_from(p) as libc::c_long) < 4 as libc::c_int as libc::c_long
        || 0 as libc::c_int
            != strncmp(
                p,
                b"HTTP\0" as *const u8 as *const libc::c_char,
                4 as libc::c_int as libc::c_ulong,
            )
    {
        return -(1 as libc::c_int);
    }
    p = p.offset(4 as libc::c_int as isize);
    if p < end && *p as libc::c_int == '/' as i32 {
        p = p.offset(1);
        p;
        while p < end && c_isdigit(*p as libc::c_int) as libc::c_int != 0 {
            p = p.offset(1);
            p;
        }
        if p < end && *p as libc::c_int == '.' as i32 {
            p = p.offset(1);
            p;
        }
        while p < end && c_isdigit(*p as libc::c_int) as libc::c_int != 0 {
            p = p.offset(1);
            p;
        }
    }
    while p < end && c_isspace(*p as libc::c_int) as libc::c_int != 0 {
        p = p.offset(1);
        p;
    }
    if (end.offset_from(p) as libc::c_long) < 3 as libc::c_int as libc::c_long
        || !c_isdigit(*p.offset(0 as libc::c_int as isize) as libc::c_int)
        || !c_isdigit(*p.offset(1 as libc::c_int as isize) as libc::c_int)
        || !c_isdigit(*p.offset(2 as libc::c_int as isize) as libc::c_int)
    {
        return -(1 as libc::c_int);
    }
    status = 100 as libc::c_int
        * (*p.offset(0 as libc::c_int as isize) as libc::c_int - '0' as i32)
        + 10 as libc::c_int
            * (*p.offset(1 as libc::c_int as isize) as libc::c_int - '0' as i32)
        + (*p.offset(2 as libc::c_int as isize) as libc::c_int - '0' as i32);
    p = p.offset(3 as libc::c_int as isize);
    if !message.is_null() {
        while p < end && c_isspace(*p as libc::c_int) as libc::c_int != 0 {
            p = p.offset(1);
            p;
        }
        while p < end
            && c_isspace(*end.offset(-(1 as libc::c_int) as isize) as libc::c_int)
                as libc::c_int != 0
        {
            end = end.offset(-1);
            end;
        }
        *message = strdupdelim(p, end);
    }
    return status;
}
unsafe extern "C" fn resp_free(mut resp_ref: *mut *mut response) {
    let mut resp: *mut response = *resp_ref;
    if resp.is_null() {
        return;
    }
    rpl_free((*resp).headers as *mut libc::c_void);
    (*resp).headers = 0 as *mut *const libc::c_char;
    rpl_free(resp as *mut libc::c_void);
    resp = 0 as *mut response;
    *resp_ref = 0 as *mut response;
}
unsafe extern "C" fn print_response_line(
    mut prefix: *const libc::c_char,
    mut b: *const libc::c_char,
    mut e: *const libc::c_char,
) {
    let mut buf: [libc::c_char; 1024] = [0; 1024];
    let mut copy: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut len: size_t = e.offset_from(b) as libc::c_long as size_t;
    if len < ::std::mem::size_of::<[libc::c_char; 1024]>() as libc::c_ulong {
        copy = buf.as_mut_ptr();
    } else {
        copy = xmalloc(len.wrapping_add(1 as libc::c_int as libc::c_ulong))
            as *mut libc::c_char;
    }
    memcpy(copy as *mut libc::c_void, b as *const libc::c_void, len);
    *copy.offset(len as isize) = 0 as libc::c_int as libc::c_char;
    logprintf(
        LOG_ALWAYS,
        b"%s%s\n\0" as *const u8 as *const libc::c_char,
        prefix,
        quotearg_style(escape_quoting_style, copy),
    );
    if copy != buf.as_mut_ptr() {
        rpl_free(copy as *mut libc::c_void);
        copy = 0 as *mut libc::c_char;
    }
}
unsafe extern "C" fn print_server_response(
    mut resp: *const response,
    mut prefix: *const libc::c_char,
) {
    let mut i: libc::c_int = 0;
    if ((*resp).headers).is_null() {
        return;
    }
    i = 0 as libc::c_int;
    while !(*((*resp).headers).offset((i + 1 as libc::c_int) as isize)).is_null() {
        let mut b: *const libc::c_char = *((*resp).headers).offset(i as isize);
        let mut e: *const libc::c_char = *((*resp).headers)
            .offset((i + 1 as libc::c_int) as isize);
        if b < e && *e.offset(-(1 as libc::c_int) as isize) as libc::c_int == '\n' as i32
        {
            e = e.offset(-1);
            e;
        }
        if b < e && *e.offset(-(1 as libc::c_int) as isize) as libc::c_int == '\r' as i32
        {
            e = e.offset(-1);
            e;
        }
        print_response_line(prefix, b, e);
        i += 1;
        i;
    }
}
unsafe extern "C" fn parse_content_range(
    mut hdr: *const libc::c_char,
    mut first_byte_ptr: *mut wgint,
    mut last_byte_ptr: *mut wgint,
    mut entity_length_ptr: *mut wgint,
) -> bool {
    let mut num: wgint = 0;
    if 0 as libc::c_int
        == strncasecmp(
            hdr,
            b"bytes\0" as *const u8 as *const libc::c_char,
            5 as libc::c_int as libc::c_ulong,
        )
    {
        hdr = hdr.offset(5 as libc::c_int as isize);
        if *hdr as libc::c_int == ':' as i32 {
            hdr = hdr.offset(1);
            hdr;
        }
        while c_isspace(*hdr as libc::c_int) {
            hdr = hdr.offset(1);
            hdr;
        }
        if *hdr == 0 {
            return 0 as libc::c_int != 0;
        }
    }
    if !c_isdigit(*hdr as libc::c_int) {
        return 0 as libc::c_int != 0;
    }
    num = 0 as libc::c_int as wgint;
    while c_isdigit(*hdr as libc::c_int) {
        num = 10 as libc::c_int as libc::c_long * num
            + (*hdr as libc::c_int - '0' as i32) as libc::c_long;
        hdr = hdr.offset(1);
        hdr;
    }
    if *hdr as libc::c_int != '-' as i32
        || !c_isdigit(*hdr.offset(1 as libc::c_int as isize) as libc::c_int)
    {
        return 0 as libc::c_int != 0;
    }
    *first_byte_ptr = num;
    hdr = hdr.offset(1);
    hdr;
    num = 0 as libc::c_int as wgint;
    while c_isdigit(*hdr as libc::c_int) {
        num = 10 as libc::c_int as libc::c_long * num
            + (*hdr as libc::c_int - '0' as i32) as libc::c_long;
        hdr = hdr.offset(1);
        hdr;
    }
    if *hdr as libc::c_int != '/' as i32 {
        return 0 as libc::c_int != 0;
    }
    *last_byte_ptr = num;
    if !(c_isdigit(*hdr.offset(1 as libc::c_int as isize) as libc::c_int) as libc::c_int
        != 0 || *hdr.offset(1 as libc::c_int as isize) as libc::c_int == '*' as i32)
    {
        return 0 as libc::c_int != 0;
    }
    if *last_byte_ptr < *first_byte_ptr {
        return 0 as libc::c_int != 0;
    }
    hdr = hdr.offset(1);
    hdr;
    if *hdr as libc::c_int == '*' as i32 {
        num = -(1 as libc::c_int) as wgint;
    } else {
        num = 0 as libc::c_int as wgint;
        while c_isdigit(*hdr as libc::c_int) {
            num = 10 as libc::c_int as libc::c_long * num
                + (*hdr as libc::c_int - '0' as i32) as libc::c_long;
            hdr = hdr.offset(1);
            hdr;
        }
    }
    *entity_length_ptr = num;
    if *entity_length_ptr <= *last_byte_ptr
        && *entity_length_ptr != -(1 as libc::c_int) as libc::c_long
    {
        return 0 as libc::c_int != 0;
    }
    return 1 as libc::c_int != 0;
}
unsafe extern "C" fn skip_short_body(
    mut fd: libc::c_int,
    mut contlen: wgint,
    mut chunked: bool,
) -> bool {
    let mut remaining_chunk_size: wgint = 0 as libc::c_int as wgint;
    let mut dlbuf: [libc::c_char; 513] = [0; 513];
    dlbuf[SKIP_SIZE as libc::c_int as usize] = '\0' as i32 as libc::c_char;
    if contlen > SKIP_THRESHOLD as libc::c_int as libc::c_long {
        return 0 as libc::c_int != 0;
    }
    while contlen > 0 as libc::c_int as libc::c_long || chunked as libc::c_int != 0 {
        let mut ret: libc::c_int = 0;
        if chunked {
            if remaining_chunk_size == 0 as libc::c_int as libc::c_long {
                let mut line: *mut libc::c_char = fd_read_line(fd);
                let mut endl: *mut libc::c_char = 0 as *mut libc::c_char;
                if line.is_null() {
                    break;
                }
                remaining_chunk_size = rpl_strtol(line, &mut endl, 16 as libc::c_int);
                rpl_free(line as *mut libc::c_void);
                line = 0 as *mut libc::c_char;
                if remaining_chunk_size < 0 as libc::c_int as libc::c_long {
                    return 0 as libc::c_int != 0;
                }
                if remaining_chunk_size == 0 as libc::c_int as libc::c_long {
                    line = fd_read_line(fd);
                    rpl_free(line as *mut libc::c_void);
                    line = 0 as *mut libc::c_char;
                    break;
                }
            }
            contlen = if remaining_chunk_size <= SKIP_SIZE as libc::c_int as libc::c_long
            {
                remaining_chunk_size
            } else {
                SKIP_SIZE as libc::c_int as libc::c_long
            };
        }
        if opt.debug as libc::c_long != 0 {
            debug_logprintf(
                b"Skipping %s bytes of body: [\0" as *const u8 as *const libc::c_char,
                number_to_static_string(contlen),
            );
        }
        ret = fd_read(
            fd,
            dlbuf.as_mut_ptr(),
            (if contlen <= SKIP_SIZE as libc::c_int as libc::c_long {
                contlen
            } else {
                SKIP_SIZE as libc::c_int as libc::c_long
            }) as libc::c_int,
            -(1 as libc::c_int) as libc::c_double,
        );
        if ret <= 0 as libc::c_int {
            if opt.debug as libc::c_long != 0 {
                debug_logprintf(
                    b"] aborting (%s).\n\0" as *const u8 as *const libc::c_char,
                    if ret < 0 as libc::c_int {
                        fd_errstr(fd)
                    } else {
                        b"EOF received\0" as *const u8 as *const libc::c_char
                    },
                );
            }
            return 0 as libc::c_int != 0;
        }
        contlen -= ret as libc::c_long;
        if chunked {
            remaining_chunk_size -= ret as libc::c_long;
            if remaining_chunk_size == 0 as libc::c_int as libc::c_long {
                let mut line_0: *mut libc::c_char = fd_read_line(fd);
                if line_0.is_null() {
                    return 0 as libc::c_int != 0
                } else {
                    rpl_free(line_0 as *mut libc::c_void);
                    line_0 = 0 as *mut libc::c_char;
                }
            }
        }
        if opt.debug as libc::c_long != 0 {
            debug_logprintf(
                b"%.*s\0" as *const u8 as *const libc::c_char,
                ret,
                dlbuf.as_mut_ptr(),
            );
        }
    }
    if opt.debug as libc::c_long != 0 {
        debug_logprintf(b"] done.\n\0" as *const u8 as *const libc::c_char);
    }
    return 1 as libc::c_int != 0;
}
unsafe extern "C" fn modify_param_name(mut name: *mut param_token) -> libc::c_int {
    let mut delim1: *const libc::c_char = memchr(
        (*name).b as *const libc::c_void,
        '*' as i32,
        ((*name).e).offset_from((*name).b) as libc::c_long as libc::c_ulong,
    ) as *const libc::c_char;
    let mut delim2: *const libc::c_char = memrchr(
        (*name).b as *const libc::c_void,
        '*' as i32,
        ((*name).e).offset_from((*name).b) as libc::c_long as size_t,
    ) as *const libc::c_char;
    let mut result: libc::c_int = 0;
    if delim1.is_null() {
        result = 0 as libc::c_int;
    } else if delim1 == delim2 {
        if ((*name).e).offset(-(1 as libc::c_int as isize)) == delim1 {
            result = 2 as libc::c_int;
        } else {
            result = 1 as libc::c_int;
        }
        (*name).e = delim1;
    } else {
        (*name).e = delim1;
        result = 2 as libc::c_int;
    }
    return result;
}
unsafe extern "C" fn modify_param_value(
    mut value: *mut param_token,
    mut encoding_type: libc::c_int,
) {
    if encoding_type == 2 as libc::c_int {
        let mut delim: *const libc::c_char = memrchr(
            (*value).b as *const libc::c_void,
            '\'' as i32,
            ((*value).e).offset_from((*value).b) as libc::c_long as size_t,
        ) as *const libc::c_char;
        if !delim.is_null() {
            (*value).b = delim.offset(1 as libc::c_int as isize);
        }
    }
}
pub unsafe extern "C" fn extract_param(
    mut source: *mut *const libc::c_char,
    mut name: *mut param_token,
    mut value: *mut param_token,
    mut separator: libc::c_char,
    mut is_url_encoded: *mut bool,
) -> bool {
    let mut p: *const libc::c_char = *source;
    let mut param_type: libc::c_int = 0;
    if !is_url_encoded.is_null() {
        *is_url_encoded = 0 as libc::c_int != 0;
    }
    while c_isspace(*p as libc::c_int) {
        p = p.offset(1);
        p;
    }
    if *p == 0 {
        *source = p;
        return 0 as libc::c_int != 0;
    }
    (*name).b = p;
    while *p as libc::c_int != 0 && !c_isspace(*p as libc::c_int)
        && *p as libc::c_int != '=' as i32
        && *p as libc::c_int != separator as libc::c_int
    {
        p = p.offset(1);
        p;
    }
    (*name).e = p;
    if (*name).b == (*name).e {
        return 0 as libc::c_int != 0;
    }
    while c_isspace(*p as libc::c_int) {
        p = p.offset(1);
        p;
    }
    if *p as libc::c_int == separator as libc::c_int || *p == 0 {
        memset(
            value as *mut libc::c_void,
            '\0' as i32,
            ::std::mem::size_of::<param_token>() as libc::c_ulong,
        );
        if *p as libc::c_int == separator as libc::c_int {
            p = p.offset(1);
            p;
        }
        *source = p;
        return 1 as libc::c_int != 0;
    }
    if *p as libc::c_int != '=' as i32 {
        return 0 as libc::c_int != 0;
    }
    p = p.offset(1);
    p;
    while c_isspace(*p as libc::c_int) {
        p = p.offset(1);
        p;
    }
    if *p as libc::c_int == '"' as i32 {
        p = p.offset(1);
        (*value).b = p;
        while *p as libc::c_int != 0 && *p as libc::c_int != '"' as i32 {
            p = p.offset(1);
            p;
        }
        if *p == 0 {
            return 0 as libc::c_int != 0;
        }
        let fresh13 = p;
        p = p.offset(1);
        (*value).e = fresh13;
        while c_isspace(*p as libc::c_int) {
            p = p.offset(1);
            p;
        }
        while *p as libc::c_int != 0 && *p as libc::c_int != separator as libc::c_int {
            p = p.offset(1);
            p;
        }
        if *p as libc::c_int == separator as libc::c_int {
            p = p.offset(1);
            p;
        } else if *p != 0 {
            return 0 as libc::c_int != 0
        }
    } else {
        (*value).b = p;
        while *p as libc::c_int != 0 && *p as libc::c_int != separator as libc::c_int {
            p = p.offset(1);
            p;
        }
        (*value).e = p;
        while (*value).e != (*value).b
            && c_isspace(
                *((*value).e).offset(-(1 as libc::c_int) as isize) as libc::c_int,
            ) as libc::c_int != 0
        {
            (*value).e = ((*value).e).offset(-1);
            (*value).e;
        }
        if *p as libc::c_int == separator as libc::c_int {
            p = p.offset(1);
            p;
        }
    }
    *source = p;
    param_type = modify_param_name(name);
    if param_type != 0 as libc::c_int {
        if param_type == 2 as libc::c_int && !is_url_encoded.is_null() {
            *is_url_encoded = 1 as libc::c_int != 0;
        }
        modify_param_value(value, param_type);
    }
    return 1 as libc::c_int != 0;
}
unsafe extern "C" fn append_value_to_filename(
    mut filename: *mut *mut libc::c_char,
    value: *const param_token,
    mut is_url_encoded: bool,
) {
    let mut original_length: libc::c_int = strlen(*filename) as libc::c_int;
    let mut new_length: libc::c_int = (strlen(*filename))
        .wrapping_add(
            ((*value).e).offset_from((*value).b) as libc::c_long as libc::c_ulong,
        ) as libc::c_int;
    *filename = xrealloc(
        *filename as *mut libc::c_void,
        (new_length + 1 as libc::c_int) as size_t,
    ) as *mut libc::c_char;
    memcpy(
        (*filename).offset(original_length as isize) as *mut libc::c_void,
        (*value).b as *const libc::c_void,
        ((*value).e).offset_from((*value).b) as libc::c_long as libc::c_ulong,
    );
    *(*filename).offset(new_length as isize) = '\0' as i32 as libc::c_char;
    if is_url_encoded {
        url_unescape((*filename).offset(original_length as isize));
    }
}
unsafe extern "C" fn parse_content_disposition(
    mut hdr: *const libc::c_char,
    mut filename: *mut *mut libc::c_char,
) -> bool {
    let mut name: param_token = param_token {
        b: 0 as *const libc::c_char,
        e: 0 as *const libc::c_char,
    };
    let mut value: param_token = param_token {
        b: 0 as *const libc::c_char,
        e: 0 as *const libc::c_char,
    };
    let mut is_url_encoded: bool = 0 as libc::c_int != 0;
    let mut encodedFilename: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut unencodedFilename: *mut libc::c_char = 0 as *mut libc::c_char;
    while extract_param(
        &mut hdr,
        &mut name,
        &mut value,
        ';' as i32 as libc::c_char,
        &mut is_url_encoded,
    ) {
        let mut isFilename: libc::c_int = ((name.e).offset_from(name.b) as libc::c_long
            as libc::c_ulong
            == (::std::mem::size_of::<[libc::c_char; 9]>() as libc::c_ulong)
                .wrapping_sub(1 as libc::c_int as libc::c_ulong)
            && c_strncasecmp(
                name.b,
                b"filename\0" as *const u8 as *const libc::c_char,
                (::std::mem::size_of::<[libc::c_char; 9]>() as libc::c_ulong)
                    .wrapping_sub(1 as libc::c_int as libc::c_ulong),
            ) == 0) as libc::c_int;
        if isFilename != 0 && !(value.b).is_null() {
            let mut isEncodedFilename: bool = false;
            let mut outFilename: *mut *mut libc::c_char = 0 as *mut *mut libc::c_char;
            let mut last_slash: *const libc::c_char = memrchr(
                value.b as *const libc::c_void,
                '/' as i32,
                (value.e).offset_from(value.b) as libc::c_long as size_t,
            ) as *const libc::c_char;
            let mut last_bs: *const libc::c_char = memrchr(
                value.b as *const libc::c_void,
                '\\' as i32,
                (value.e).offset_from(value.b) as libc::c_long as size_t,
            ) as *const libc::c_char;
            if !last_slash.is_null() && !last_bs.is_null() {
                value
                    .b = (if last_slash >= last_bs { last_slash } else { last_bs })
                    .offset(1 as libc::c_int as isize);
            } else if !last_slash.is_null() || !last_bs.is_null() {
                value
                    .b = (if !last_slash.is_null() { last_slash } else { last_bs })
                    .offset(1 as libc::c_int as isize);
            }
            if !(value.b == value.e) {
                isEncodedFilename = *name.e as libc::c_int == '*' as i32
                    && !c_isdigit(
                        *(name.e).offset(1 as libc::c_int as isize) as libc::c_int,
                    );
                outFilename = if isEncodedFilename as libc::c_int != 0 {
                    &mut encodedFilename
                } else {
                    &mut unencodedFilename
                };
                if !(*outFilename).is_null() {
                    append_value_to_filename(outFilename, &mut value, is_url_encoded);
                } else {
                    *outFilename = strdupdelim(value.b, value.e);
                    if is_url_encoded {
                        url_unescape(*outFilename);
                    }
                }
            }
        }
        is_url_encoded = 0 as libc::c_int != 0;
    }
    if !encodedFilename.is_null() {
        rpl_free(unencodedFilename as *mut libc::c_void);
        unencodedFilename = 0 as *mut libc::c_char;
        *filename = encodedFilename;
    } else {
        rpl_free(encodedFilename as *mut libc::c_void);
        encodedFilename = 0 as *mut libc::c_char;
        *filename = unencodedFilename;
    }
    if !(*filename).is_null() {
        return 1 as libc::c_int != 0
    } else {
        return 0 as libc::c_int != 0
    };
}
static mut pconn_active: bool = false;
static mut pconn: C2RustUnnamed_4 = C2RustUnnamed_4 {
    socket: 0,
    host: 0 as *const libc::c_char as *mut libc::c_char,
    port: 0,
    ssl: false,
    authorized: false,
    ntlm: ntlmdata {
        state: NTLMSTATE_NONE,
        nonce: [0; 8],
    },
};
unsafe extern "C" fn invalidate_persistent() {
    if opt.debug as libc::c_long != 0 {
        debug_logprintf(
            b"Disabling further reuse of socket %d.\n\0" as *const u8
                as *const libc::c_char,
            pconn.socket,
        );
    }
    pconn_active = 0 as libc::c_int != 0;
    fd_close(pconn.socket);
    rpl_free(pconn.host as *mut libc::c_void);
    pconn.host = 0 as *mut libc::c_char;
    memset(
        &mut pconn as *mut C2RustUnnamed_4 as *mut libc::c_void,
        '\0' as i32,
        ::std::mem::size_of::<C2RustUnnamed_4>() as libc::c_ulong,
    );
}
unsafe extern "C" fn register_persistent(
    mut host: *const libc::c_char,
    mut port: libc::c_int,
    mut fd: libc::c_int,
    mut ssl: bool,
) {
    if pconn_active {
        if pconn.socket == fd {
            return
        } else {
            invalidate_persistent();
        }
    }
    pconn_active = 1 as libc::c_int != 0;
    pconn.socket = fd;
    pconn.host = xstrdup(host);
    pconn.port = port;
    pconn.ssl = ssl;
    pconn.authorized = 0 as libc::c_int != 0;
    if opt.debug as libc::c_long != 0 {
        debug_logprintf(
            b"Registered socket %d for persistent reuse.\n\0" as *const u8
                as *const libc::c_char,
            fd,
        );
    }
}
unsafe extern "C" fn persistent_available_p(
    mut host: *const libc::c_char,
    mut port: libc::c_int,
    mut ssl: bool,
    mut host_lookup_failed: *mut bool,
) -> bool {
    if !pconn_active {
        return 0 as libc::c_int != 0;
    }
    if ssl as libc::c_int != pconn.ssl as libc::c_int {
        return 0 as libc::c_int != 0;
    }
    if port != pconn.port {
        return 0 as libc::c_int != 0;
    }
    if 0 as libc::c_int != strcasecmp(host, pconn.host) {
        let mut found: bool = false;
        let mut ip: ip_address = ip_address {
            family: 0,
            data: C2RustUnnamed_5 {
                d4: in_addr { s_addr: 0 },
            },
            ipv6_scope: 0,
        };
        let mut al: *mut address_list = 0 as *mut address_list;
        if ssl {
            return 0 as libc::c_int != 0;
        }
        if !socket_ip_address(pconn.socket, &mut ip, ENDPOINT_PEER as libc::c_int) {
            invalidate_persistent();
            return 0 as libc::c_int != 0;
        }
        al = lookup_host(host, 0 as libc::c_int);
        if al.is_null() {
            *host_lookup_failed = 1 as libc::c_int != 0;
            return 0 as libc::c_int != 0;
        }
        found = address_list_contains(al, &mut ip);
        address_list_release(al);
        if !found {
            return 0 as libc::c_int != 0;
        }
    }
    if !test_socket_open(pconn.socket) {
        invalidate_persistent();
        return 0 as libc::c_int != 0;
    }
    return 1 as libc::c_int != 0;
}
unsafe extern "C" fn free_hstat(mut hs: *mut http_stat) {
    rpl_free((*hs).newloc as *mut libc::c_void);
    (*hs).newloc = 0 as *mut libc::c_char;
    rpl_free((*hs).remote_time as *mut libc::c_void);
    (*hs).remote_time = 0 as *mut libc::c_char;
    rpl_free((*hs).error as *mut libc::c_void);
    (*hs).error = 0 as *mut libc::c_char;
    rpl_free((*hs).rderrmsg as *mut libc::c_void);
    (*hs).rderrmsg = 0 as *mut libc::c_char;
    rpl_free((*hs).local_file as *mut libc::c_void);
    (*hs).local_file = 0 as *mut libc::c_char;
    rpl_free((*hs).orig_file_name as *mut libc::c_void);
    (*hs).orig_file_name = 0 as *mut libc::c_char;
    rpl_free((*hs).message as *mut libc::c_void);
    (*hs).message = 0 as *mut libc::c_char;
}
unsafe extern "C" fn get_file_flags(
    mut filename: *const libc::c_char,
    mut dt: *mut libc::c_int,
) {
    logprintf(
        LOG_VERBOSE,
        dcgettext(
            0 as *const libc::c_char,
            b"File %s already there; not retrieving.\n\n\0" as *const u8
                as *const libc::c_char,
            5 as libc::c_int,
        ),
        quote(filename),
    );
    *dt |= RETROKF as libc::c_int;
    if has_html_suffix_p(filename) {
        *dt |= TEXTHTML as libc::c_int;
    }
}
unsafe extern "C" fn read_response_body(
    mut hs: *mut http_stat,
    mut sock: libc::c_int,
    mut fp: *mut FILE,
    mut contlen: wgint,
    mut contrange: wgint,
    mut chunked_transfer_encoding: bool,
    mut url: *mut libc::c_char,
    mut warc_timestamp_str: *mut libc::c_char,
    mut warc_request_uuid: *mut libc::c_char,
    mut warc_ip: *mut ip_address,
    mut type_0: *mut libc::c_char,
    mut statcode: libc::c_int,
    mut head: *mut libc::c_char,
) -> libc::c_int {
    let mut warc_payload_offset: libc::c_int = 0 as libc::c_int;
    let mut warc_tmp: *mut FILE = 0 as *mut FILE;
    let mut warcerr: libc::c_int = 0 as libc::c_int;
    let mut flags: libc::c_int = 0 as libc::c_int;
    if !(opt.warc_filename).is_null() {
        warc_tmp = warc_tempfile();
        if warc_tmp.is_null() {
            warcerr = WARC_TMP_FOPENERR as libc::c_int;
        }
        if warcerr == 0 as libc::c_int {
            let mut head_len: libc::c_int = strlen(head) as libc::c_int;
            let mut warc_tmp_written: libc::c_int = fwrite(
                head as *const libc::c_void,
                1 as libc::c_int as libc::c_ulong,
                head_len as libc::c_ulong,
                warc_tmp,
            ) as libc::c_int;
            if warc_tmp_written != head_len {
                warcerr = WARC_TMP_FWRITEERR as libc::c_int;
            }
            warc_payload_offset = head_len;
        }
        if warcerr != 0 as libc::c_int {
            if !warc_tmp.is_null() {
                fclose(warc_tmp);
            }
            return warcerr;
        }
    }
    if !fp.is_null() {
        if opt.save_headers as libc::c_int != 0
            && (*hs).restval == 0 as libc::c_int as libc::c_long
        {
            fwrite(
                head as *const libc::c_void,
                1 as libc::c_int as libc::c_ulong,
                strlen(head),
                fp,
            );
        }
    }
    if contlen != -(1 as libc::c_int) as libc::c_long {
        flags |= rb_read_exactly as libc::c_int;
    }
    if !fp.is_null() && (*hs).restval > 0 as libc::c_int as libc::c_long
        && contrange == 0 as libc::c_int as libc::c_long
    {
        flags |= rb_skip_startpos as libc::c_int;
    }
    if chunked_transfer_encoding {
        flags |= rb_chunked_transfer_encoding as libc::c_int;
    }
    if (*hs).remote_encoding as libc::c_int == ENC_GZIP as libc::c_int {
        flags |= rb_compressed_gzip as libc::c_int;
    }
    (*hs).len = (*hs).restval;
    (*hs).rd_size = 0 as libc::c_int as wgint;
    (*hs)
        .res = fd_read_body(
        (*hs).local_file,
        sock,
        fp,
        if contlen != -(1 as libc::c_int) as libc::c_long {
            contlen
        } else {
            0 as libc::c_int as libc::c_long
        },
        (*hs).restval,
        &mut (*hs).rd_size,
        &mut (*hs).len,
        &mut (*hs).dltime,
        flags,
        warc_tmp,
    );
    if (*hs).res >= 0 as libc::c_int {
        if !warc_tmp.is_null() {
            let mut r: bool = warc_write_response_record(
                url,
                warc_timestamp_str,
                warc_request_uuid,
                warc_ip,
                warc_tmp,
                warc_payload_offset as off_t,
                type_0,
                statcode,
                (*hs).newloc,
            );
            if !r {
                return WARC_ERR as libc::c_int;
            }
        }
        return RETRFINISHED as libc::c_int;
    }
    if !warc_tmp.is_null() {
        fclose(warc_tmp);
    }
    if (*hs).res == -(2 as libc::c_int) {
        return FWRITEERR as libc::c_int
    } else if (*hs).res == -(3 as libc::c_int) {
        return WARC_TMP_FWRITEERR as libc::c_int
    } else {
        rpl_free((*hs).rderrmsg as *mut libc::c_void);
        (*hs).rderrmsg = 0 as *mut libc::c_char;
        (*hs).rderrmsg = xstrdup(fd_errstr(sock));
        return RETRFINISHED as libc::c_int;
    };
}
unsafe extern "C" fn time_to_rfc1123(
    mut time_0: time_t,
    mut buf: *mut libc::c_char,
    mut bufsize: size_t,
) -> uerr_t {
    static mut wkday: [*const libc::c_char; 7] = [
        b"Sun\0" as *const u8 as *const libc::c_char,
        b"Mon\0" as *const u8 as *const libc::c_char,
        b"Tue\0" as *const u8 as *const libc::c_char,
        b"Wed\0" as *const u8 as *const libc::c_char,
        b"Thu\0" as *const u8 as *const libc::c_char,
        b"Fri\0" as *const u8 as *const libc::c_char,
        b"Sat\0" as *const u8 as *const libc::c_char,
    ];
    static mut month: [*const libc::c_char; 12] = [
        b"Jan\0" as *const u8 as *const libc::c_char,
        b"Feb\0" as *const u8 as *const libc::c_char,
        b"Mar\0" as *const u8 as *const libc::c_char,
        b"Apr\0" as *const u8 as *const libc::c_char,
        b"May\0" as *const u8 as *const libc::c_char,
        b"Jun\0" as *const u8 as *const libc::c_char,
        b"Jul\0" as *const u8 as *const libc::c_char,
        b"Aug\0" as *const u8 as *const libc::c_char,
        b"Sep\0" as *const u8 as *const libc::c_char,
        b"Oct\0" as *const u8 as *const libc::c_char,
        b"Nov\0" as *const u8 as *const libc::c_char,
        b"Dec\0" as *const u8 as *const libc::c_char,
    ];
    let mut gtm: *mut tm = gmtime(&mut time_0);
    if gtm.is_null() {
        logprintf(
            LOG_NOTQUIET,
            dcgettext(
                0 as *const libc::c_char,
                b"gmtime failed. This is probably a bug.\n\0" as *const u8
                    as *const libc::c_char,
                5 as libc::c_int,
            ),
        );
        return TIMECONV_ERR;
    }
    snprintf(
        buf,
        bufsize,
        b"%s, %02d %s %04d %02d:%02d:%02d GMT\0" as *const u8 as *const libc::c_char,
        wkday[(*gtm).tm_wday as usize],
        (*gtm).tm_mday,
        month[(*gtm).tm_mon as usize],
        (*gtm).tm_year + 1900 as libc::c_int,
        (*gtm).tm_hour,
        (*gtm).tm_min,
        (*gtm).tm_sec,
    );
    return RETROK;
}
unsafe extern "C" fn initialize_request(
    mut u: *const url,
    mut hs: *mut http_stat,
    mut dt: *mut libc::c_int,
    mut proxy: *mut url,
    mut inhibit_keep_alive: bool,
    mut basic_auth_finished: *mut bool,
    mut body_data_size: *mut wgint,
    mut user: *mut *mut libc::c_char,
    mut passwd: *mut *mut libc::c_char,
    mut ret: *mut uerr_t,
) -> *mut request {
    let mut head_only: bool = *dt & HEAD_ONLY as libc::c_int != 0;
    let mut req: *mut request = 0 as *mut request;
    let mut meth_arg: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut meth: *const libc::c_char = b"GET\0" as *const u8 as *const libc::c_char;
    if head_only {
        meth = b"HEAD\0" as *const u8 as *const libc::c_char;
    } else if !(opt.method).is_null() {
        meth = opt.method;
    }
    if !proxy.is_null() {
        meth_arg = xstrdup((*u).url);
    } else {
        meth_arg = url_full_path(u);
    }
    req = request_new(meth, meth_arg);
    static mut hfmt: [[*const libc::c_char; 2]; 2] = [
        [
            b"%s\0" as *const u8 as *const libc::c_char,
            b"[%s]\0" as *const u8 as *const libc::c_char,
        ],
        [
            b"%s:%d\0" as *const u8 as *const libc::c_char,
            b"[%s]:%d\0" as *const u8 as *const libc::c_char,
        ],
    ];
    let mut add_port: libc::c_int = ((*u).port != scheme_default_port((*u).scheme))
        as libc::c_int;
    let mut add_squares: libc::c_int = (strchr((*u).host, ':' as i32)
        != 0 as *mut libc::c_void as *mut libc::c_char) as libc::c_int;
    request_set_header(
        req,
        b"Host\0" as *const u8 as *const libc::c_char,
        aprintf(hfmt[add_port as usize][add_squares as usize], (*u).host, (*u).port),
        rel_value,
    );
    request_set_header(
        req,
        b"Referer\0" as *const u8 as *const libc::c_char,
        (*hs).referer,
        rel_none,
    );
    if *dt & SEND_NOCACHE as libc::c_int != 0 {
        request_set_header(
            req,
            b"Cache-Control\0" as *const u8 as *const libc::c_char,
            b"no-cache\0" as *const u8 as *const libc::c_char,
            rel_none,
        );
        request_set_header(
            req,
            b"Pragma\0" as *const u8 as *const libc::c_char,
            b"no-cache\0" as *const u8 as *const libc::c_char,
            rel_none,
        );
    }
    if *dt & IF_MODIFIED_SINCE as libc::c_int != 0 {
        let mut strtime: [libc::c_char; 32] = [0; 32];
        let mut err: uerr_t = time_to_rfc1123(
            (*hs).orig_file_tstamp,
            strtime.as_mut_ptr(),
            (::std::mem::size_of::<[libc::c_char; 32]>() as libc::c_ulong)
                .wrapping_div(::std::mem::size_of::<libc::c_char>() as libc::c_ulong),
        );
        if err as libc::c_uint != RETROK as libc::c_int as libc::c_uint {
            logputs(
                LOG_VERBOSE,
                dcgettext(
                    0 as *const libc::c_char,
                    b"Cannot convert timestamp to http format. Falling back to time 0 as last modification time.\n\0"
                        as *const u8 as *const libc::c_char,
                    5 as libc::c_int,
                ),
            );
            strcpy(
                strtime.as_mut_ptr(),
                b"Thu, 01 Jan 1970 00:00:00 GMT\0" as *const u8 as *const libc::c_char,
            );
        }
        request_set_header(
            req,
            b"If-Modified-Since\0" as *const u8 as *const libc::c_char,
            xstrdup(strtime.as_mut_ptr()),
            rel_value,
        );
    }
    if (*hs).restval != 0 {
        request_set_header(
            req,
            b"Range\0" as *const u8 as *const libc::c_char,
            aprintf(
                b"bytes=%s-\0" as *const u8 as *const libc::c_char,
                number_to_static_string((*hs).restval),
            ),
            rel_value,
        );
    }
    if (opt.useragent).is_null() {
        request_set_header(
            req,
            b"User-Agent\0" as *const u8 as *const libc::c_char,
            aprintf(b"Wget/%s\0" as *const u8 as *const libc::c_char, version_string),
            rel_value,
        );
    } else if *opt.useragent != 0 {
        request_set_header(
            req,
            b"User-Agent\0" as *const u8 as *const libc::c_char,
            opt.useragent,
            rel_none,
        );
    }
    request_set_header(
        req,
        b"Accept\0" as *const u8 as *const libc::c_char,
        b"*/*\0" as *const u8 as *const libc::c_char,
        rel_none,
    );
    if opt.compression as libc::c_uint != compression_none as libc::c_int as libc::c_uint
    {
        request_set_header(
            req,
            b"Accept-Encoding\0" as *const u8 as *const libc::c_char,
            b"gzip\0" as *const u8 as *const libc::c_char,
            rel_none,
        );
    } else {
        request_set_header(
            req,
            b"Accept-Encoding\0" as *const u8 as *const libc::c_char,
            b"identity\0" as *const u8 as *const libc::c_char,
            rel_none,
        );
    }
    if !((*u).user).is_null() {
        *user = (*u).user;
    } else if !(opt.user).is_null()
        && (!(opt.use_askpass).is_null() || opt.ask_passwd as libc::c_int != 0)
    {
        *user = opt.user;
    } else if !(opt.http_user).is_null() {
        *user = opt.http_user;
    } else if !(opt.user).is_null() {
        *user = opt.user;
    } else {
        *user = 0 as *mut libc::c_char;
    }
    if !((*u).passwd).is_null() {
        *passwd = (*u).passwd;
    } else if !(opt.passwd).is_null()
        && (!(opt.use_askpass).is_null() || opt.ask_passwd as libc::c_int != 0)
    {
        *passwd = opt.passwd;
    } else if !(opt.http_passwd).is_null() {
        *passwd = opt.http_passwd;
    } else if !(opt.passwd).is_null() {
        *passwd = opt.passwd;
    } else {
        *passwd = 0 as *mut libc::c_char;
    }
    if opt.netrc as libc::c_int != 0 && ((*user).is_null() || (*passwd).is_null()) {
        search_netrc(
            (*u).host,
            user as *mut *const libc::c_char,
            passwd as *mut *const libc::c_char,
            0 as libc::c_int,
            0 as *mut FILE,
        );
    }
    if !(*user).is_null() && !(*passwd).is_null()
        && (((*u).user).is_null() || opt.auth_without_challenge as libc::c_int != 0)
    {
        *basic_auth_finished = maybe_send_basic_creds((*u).host, *user, *passwd, req);
    }
    if inhibit_keep_alive {
        request_set_header(
            req,
            b"Connection\0" as *const u8 as *const libc::c_char,
            b"Close\0" as *const u8 as *const libc::c_char,
            rel_none,
        );
    } else {
        request_set_header(
            req,
            b"Connection\0" as *const u8 as *const libc::c_char,
            b"Keep-Alive\0" as *const u8 as *const libc::c_char,
            rel_none,
        );
        if !proxy.is_null() {
            request_set_header(
                req,
                b"Proxy-Connection\0" as *const u8 as *const libc::c_char,
                b"Keep-Alive\0" as *const u8 as *const libc::c_char,
                rel_none,
            );
        }
    }
    if !(opt.method).is_null() {
        if !(opt.body_data).is_null() || !(opt.body_file).is_null() {
            request_set_header(
                req,
                b"Content-Type\0" as *const u8 as *const libc::c_char,
                b"application/x-www-form-urlencoded\0" as *const u8
                    as *const libc::c_char,
                rel_none,
            );
            if !(opt.body_data).is_null() {
                *body_data_size = strlen(opt.body_data) as wgint;
            } else {
                *body_data_size = file_size(opt.body_file);
                if *body_data_size == -(1 as libc::c_int) as libc::c_long {
                    logprintf(
                        LOG_NOTQUIET,
                        dcgettext(
                            0 as *const libc::c_char,
                            b"BODY data file %s missing: %s\n\0" as *const u8
                                as *const libc::c_char,
                            5 as libc::c_int,
                        ),
                        quote(opt.body_file),
                        strerror(*__errno_location()),
                    );
                    request_free(&mut req);
                    *ret = FILEBADFILE;
                    return 0 as *mut request;
                }
            }
            request_set_header(
                req,
                b"Content-Length\0" as *const u8 as *const libc::c_char,
                xstrdup(number_to_static_string(*body_data_size)),
                rel_value,
            );
        } else if c_strcasecmp(opt.method, b"post\0" as *const u8 as *const libc::c_char)
            == 0 as libc::c_int
            || c_strcasecmp(opt.method, b"put\0" as *const u8 as *const libc::c_char)
                == 0 as libc::c_int
            || c_strcasecmp(opt.method, b"patch\0" as *const u8 as *const libc::c_char)
                == 0 as libc::c_int
        {
            request_set_header(
                req,
                b"Content-Length\0" as *const u8 as *const libc::c_char,
                b"0\0" as *const u8 as *const libc::c_char,
                rel_none,
            );
        }
    }
    return req;
}
unsafe extern "C" fn initialize_proxy_configuration(
    mut u: *const url,
    mut req: *mut request,
    mut proxy: *mut url,
    mut proxyauth: *mut *mut libc::c_char,
) {
    let mut proxy_user: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut proxy_passwd: *mut libc::c_char = 0 as *mut libc::c_char;
    if !(opt.proxy_user).is_null() && !(opt.proxy_passwd).is_null() {
        proxy_user = opt.proxy_user;
        proxy_passwd = opt.proxy_passwd;
    } else {
        proxy_user = (*proxy).user;
        proxy_passwd = (*proxy).passwd;
    }
    if !proxy_user.is_null() && !proxy_passwd.is_null() {
        *proxyauth = basic_authentication_encode(proxy_user, proxy_passwd);
    }
    request_set_header(
        req,
        b"Proxy-Authorization\0" as *const u8 as *const libc::c_char,
        *proxyauth,
        rel_value,
    );
}
unsafe extern "C" fn establish_connection(
    mut u: *const url,
    mut conn_ref: *mut *const url,
    mut hs: *mut http_stat,
    mut proxy: *mut url,
    mut proxyauth: *mut *mut libc::c_char,
    mut req_ref: *mut *mut request,
    mut using_ssl: *mut bool,
    mut inhibit_keep_alive: bool,
    mut sock_ref: *mut libc::c_int,
) -> uerr_t {
    let mut host_lookup_failed: bool = 0 as libc::c_int != 0;
    let mut sock: libc::c_int = *sock_ref;
    let mut req: *mut request = *req_ref;
    let mut conn: *const url = *conn_ref;
    let mut resp: *mut response = 0 as *mut response;
    let mut write_error: libc::c_int = 0;
    let mut statcode: libc::c_int = 0;
    if !inhibit_keep_alive {
        let mut relevant: *const url = conn;
        if persistent_available_p(
            (*relevant).host,
            (*relevant).port,
            0 as libc::c_int != 0,
            &mut host_lookup_failed,
        ) {
            let mut family: libc::c_int = socket_family(
                pconn.socket,
                ENDPOINT_PEER as libc::c_int,
            );
            sock = pconn.socket;
            *using_ssl = pconn.ssl;
            if family == 10 as libc::c_int {
                logprintf(
                    LOG_VERBOSE,
                    dcgettext(
                        0 as *const libc::c_char,
                        b"Reusing existing connection to [%s]:%d.\n\0" as *const u8
                            as *const libc::c_char,
                        5 as libc::c_int,
                    ),
                    quotearg_style(escape_quoting_style, pconn.host),
                    pconn.port,
                );
            } else {
                logprintf(
                    LOG_VERBOSE,
                    dcgettext(
                        0 as *const libc::c_char,
                        b"Reusing existing connection to %s:%d.\n\0" as *const u8
                            as *const libc::c_char,
                        5 as libc::c_int,
                    ),
                    quotearg_style(escape_quoting_style, pconn.host),
                    pconn.port,
                );
            }
            if opt.debug as libc::c_long != 0 {
                debug_logprintf(
                    b"Reusing fd %d.\n\0" as *const u8 as *const libc::c_char,
                    sock,
                );
            }
            if pconn.authorized {
                request_remove_header(
                    req,
                    b"Authorization\0" as *const u8 as *const libc::c_char,
                );
            }
        } else if host_lookup_failed {
            logprintf(
                LOG_NOTQUIET,
                dcgettext(
                    0 as *const libc::c_char,
                    b"%s: unable to resolve host address %s\n\0" as *const u8
                        as *const libc::c_char,
                    5 as libc::c_int,
                ),
                exec_name,
                quote((*relevant).host),
            );
            return HOSTERR;
        } else if sock != -(1 as libc::c_int) {
            sock = -(1 as libc::c_int);
        }
    }
    if sock < 0 as libc::c_int {
        sock = connect_to_host((*conn).host, (*conn).port);
        if sock == E_HOST as libc::c_int {
            return HOSTERR
        } else if sock < 0 as libc::c_int {
            return (if retryable_socket_connect_error(*__errno_location()) as libc::c_int
                != 0
            {
                CONERROR as libc::c_int
            } else {
                CONIMPOSSIBLE as libc::c_int
            }) as uerr_t
        }
    }
    *conn_ref = conn;
    *req_ref = req;
    *sock_ref = sock;
    return RETROK;
}
unsafe extern "C" fn set_file_timestamp(mut hs: *mut http_stat) -> uerr_t {
    let mut local_dot_orig_file_exists: bool = 0 as libc::c_int != 0;
    let mut local_filename: *mut libc::c_char = 0 as *mut libc::c_char;
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
    let mut buf: [libc::c_char; 1024] = [0; 1024];
    if opt.backup_converted {
        let mut filename_len: size_t = strlen((*hs).local_file);
        let mut filename_plus_orig_suffix: *mut libc::c_char = 0 as *mut libc::c_char;
        if filename_len
            .wrapping_add(::std::mem::size_of::<[libc::c_char; 6]>() as libc::c_ulong)
            > ::std::mem::size_of::<[libc::c_char; 1024]>() as libc::c_ulong
        {
            filename_plus_orig_suffix = xmalloc(
                filename_len
                    .wrapping_add(
                        ::std::mem::size_of::<[libc::c_char; 6]>() as libc::c_ulong,
                    ),
            ) as *mut libc::c_char;
        } else {
            filename_plus_orig_suffix = buf.as_mut_ptr();
        }
        memcpy(
            filename_plus_orig_suffix as *mut libc::c_void,
            (*hs).local_file as *const libc::c_void,
            filename_len,
        );
        memcpy(
            filename_plus_orig_suffix.offset(filename_len as isize) as *mut libc::c_void,
            b".orig\0" as *const u8 as *const libc::c_char as *const libc::c_void,
            ::std::mem::size_of::<[libc::c_char; 6]>() as libc::c_ulong,
        );
        if stat(filename_plus_orig_suffix, &mut st) == 0 as libc::c_int {
            local_dot_orig_file_exists = 1 as libc::c_int != 0;
            local_filename = filename_plus_orig_suffix;
        }
    }
    if !local_dot_orig_file_exists {
        if stat((*hs).local_file, &mut st) == 0 as libc::c_int {
            if local_filename != buf.as_mut_ptr() {
                rpl_free(local_filename as *mut libc::c_void);
                local_filename = 0 as *mut libc::c_char;
            }
            local_filename = (*hs).local_file;
        }
    }
    if !local_filename.is_null() {
        if local_filename == buf.as_mut_ptr() || local_filename == (*hs).local_file {
            (*hs).orig_file_name = xstrdup(local_filename);
        } else {
            (*hs).orig_file_name = local_filename;
        }
        (*hs).orig_file_size = st.st_size;
        (*hs).orig_file_tstamp = st.st_mtim.tv_sec;
        (*hs).timestamp_checked = 1 as libc::c_int != 0;
    }
    return RETROK;
}
unsafe extern "C" fn check_file_output(
    mut u: *const url,
    mut hs: *mut http_stat,
    mut resp: *mut response,
    mut hdrval: *mut libc::c_char,
    mut hdrsize: size_t,
) -> uerr_t {
    if ((*hs).local_file).is_null() {
        let mut local_file: *mut libc::c_char = 0 as *mut libc::c_char;
        if !opt.content_disposition
            || !resp_header_copy(
                resp,
                b"Content-Disposition\0" as *const u8 as *const libc::c_char,
                hdrval,
                hdrsize as libc::c_int,
            ) || !parse_content_disposition(hdrval, &mut local_file)
        {
            (*hs).local_file = url_file_name(u, 0 as *mut libc::c_char);
        } else {
            if opt.debug as libc::c_long != 0 {
                debug_logprintf(
                    b"Parsed filename from Content-Disposition: %s\n\0" as *const u8
                        as *const libc::c_char,
                    local_file,
                );
            }
            (*hs).local_file = url_file_name(u, local_file);
        }
        rpl_free(local_file as *mut libc::c_void);
        local_file = 0 as *mut libc::c_char;
    }
    (*hs)
        .temporary = opt.delete_after as libc::c_int != 0
        || opt.spider as libc::c_int != 0 || !acceptable((*hs).local_file);
    if (*hs).temporary {
        let mut tmp: *mut libc::c_char = aprintf(
            b"%s.tmp\0" as *const u8 as *const libc::c_char,
            (*hs).local_file,
        );
        rpl_free((*hs).local_file as *mut libc::c_void);
        (*hs).local_file = 0 as *mut libc::c_char;
        (*hs).local_file = tmp;
    }
    if !(*hs).existence_checked
        && file_exists_p((*hs).local_file, 0 as *mut file_stats_t) as libc::c_int != 0
    {
        if opt.noclobber as libc::c_int != 0 && (opt.output_document).is_null() {
            return RETRUNNEEDED
        } else if !(opt.noclobber as libc::c_int != 0
            || opt.always_rest as libc::c_int != 0
            || opt.timestamping as libc::c_int != 0 || opt.dirstruct as libc::c_int != 0
            || !(opt.output_document).is_null() || opt.backups > 0 as libc::c_int)
        {
            let mut unique: *mut libc::c_char = unique_name_passthrough(
                (*hs).local_file,
            );
            if unique != (*hs).local_file {
                rpl_free((*hs).local_file as *mut libc::c_void);
                (*hs).local_file = 0 as *mut libc::c_char;
            }
            (*hs).local_file = unique;
        }
    }
    (*hs).existence_checked = 1 as libc::c_int != 0;
    if opt.timestamping as libc::c_int != 0 && !(*hs).timestamp_checked {
        let mut timestamp_err: uerr_t = set_file_timestamp(hs);
        if timestamp_err as libc::c_uint != RETROK as libc::c_int as libc::c_uint {
            return timestamp_err;
        }
    }
    return RETROK;
}
unsafe extern "C" fn check_auth(
    mut u: *const url,
    mut user: *mut libc::c_char,
    mut passwd: *mut libc::c_char,
    mut resp: *mut response,
    mut req: *mut request,
    mut ntlm_seen_ref: *mut bool,
    mut retry: *mut bool,
    mut basic_auth_finished_ref: *mut bool,
    mut auth_finished_ref: *mut bool,
) -> uerr_t {
    let mut auth_err: uerr_t = RETROK;
    let mut basic_auth_finished: bool = *basic_auth_finished_ref;
    let mut auth_finished: bool = *auth_finished_ref;
    let mut ntlm_seen: bool = *ntlm_seen_ref;
    let mut buf: [libc::c_char; 256] = [0; 256];
    let mut tmp: *mut libc::c_char = 0 as *mut libc::c_char;
    *retry = 0 as libc::c_int != 0;
    if !auth_finished && (!user.is_null() && !passwd.is_null()) {
        let mut wapos: libc::c_int = 0;
        let mut www_authenticate: *const libc::c_char = 0 as *const libc::c_char;
        let mut wabeg: *const libc::c_char = 0 as *const libc::c_char;
        let mut waend: *const libc::c_char = 0 as *const libc::c_char;
        let mut digest: *const libc::c_char = 0 as *const libc::c_char;
        let mut basic: *const libc::c_char = 0 as *const libc::c_char;
        let mut ntlm: *const libc::c_char = 0 as *const libc::c_char;
        wapos = 0 as libc::c_int;
        while ntlm.is_null()
            && {
                wapos = resp_header_locate(
                    resp,
                    b"WWW-Authenticate\0" as *const u8 as *const libc::c_char,
                    wapos,
                    &mut wabeg,
                    &mut waend,
                );
                wapos != -(1 as libc::c_int)
            }
        {
            let mut name: param_token = param_token {
                b: 0 as *const libc::c_char,
                e: 0 as *const libc::c_char,
            };
            let mut value: param_token = param_token {
                b: 0 as *const libc::c_char,
                e: 0 as *const libc::c_char,
            };
            let mut len: size_t = waend.offset_from(wabeg) as libc::c_long as size_t;
            if tmp != buf.as_mut_ptr() {
                rpl_free(tmp as *mut libc::c_void);
                tmp = 0 as *mut libc::c_char;
            }
            if len < ::std::mem::size_of::<[libc::c_char; 256]>() as libc::c_ulong {
                tmp = buf.as_mut_ptr();
            } else {
                tmp = xmalloc(len.wrapping_add(1 as libc::c_int as libc::c_ulong))
                    as *mut libc::c_char;
            }
            memcpy(tmp as *mut libc::c_void, wabeg as *const libc::c_void, len);
            *tmp.offset(len as isize) = 0 as libc::c_int as libc::c_char;
            www_authenticate = tmp;
            while ntlm.is_null() {
                while c_isspace(*www_authenticate as libc::c_int) {
                    www_authenticate = www_authenticate.offset(1);
                    www_authenticate;
                }
                name.b = www_authenticate;
                name.e = name.b;
                while *name.e as libc::c_int != 0 && !c_isspace(*name.e as libc::c_int) {
                    name.e = (name.e).offset(1);
                    name.e;
                }
                if name.b == name.e {
                    break;
                }
                if opt.debug as libc::c_long != 0 {
                    debug_logprintf(
                        b"Auth scheme found '%.*s'\n\0" as *const u8
                            as *const libc::c_char,
                        (name.e).offset_from(name.b) as libc::c_long as libc::c_int,
                        name.b,
                    );
                }
                if known_authentication_scheme_p(name.b, name.e) {
                    if c_strncasecmp(
                        name.b,
                        b"NTLM\0" as *const u8 as *const libc::c_char,
                        (::std::mem::size_of::<[libc::c_char; 5]>() as libc::c_ulong)
                            .wrapping_sub(1 as libc::c_int as libc::c_ulong),
                    ) == 0
                        && (c_isspace(
                            *(name.b)
                                .offset(
                                    (::std::mem::size_of::<[libc::c_char; 5]>()
                                        as libc::c_ulong)
                                        .wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize,
                                ) as libc::c_int,
                        ) as libc::c_int != 0
                            || *(name.b)
                                .offset(
                                    (::std::mem::size_of::<[libc::c_char; 5]>()
                                        as libc::c_ulong)
                                        .wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize,
                                ) == 0)
                    {
                        ntlm = name.b;
                        break;
                    } else if digest.is_null()
                        && (c_strncasecmp(
                            name.b,
                            b"Digest\0" as *const u8 as *const libc::c_char,
                            (::std::mem::size_of::<[libc::c_char; 7]>() as libc::c_ulong)
                                .wrapping_sub(1 as libc::c_int as libc::c_ulong),
                        ) == 0
                            && (c_isspace(
                                *(name.b)
                                    .offset(
                                        (::std::mem::size_of::<[libc::c_char; 7]>()
                                            as libc::c_ulong)
                                            .wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize,
                                    ) as libc::c_int,
                            ) as libc::c_int != 0
                                || *(name.b)
                                    .offset(
                                        (::std::mem::size_of::<[libc::c_char; 7]>()
                                            as libc::c_ulong)
                                            .wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize,
                                    ) == 0))
                    {
                        digest = name.b;
                    } else if basic.is_null()
                        && (c_strncasecmp(
                            name.b,
                            b"Basic\0" as *const u8 as *const libc::c_char,
                            (::std::mem::size_of::<[libc::c_char; 6]>() as libc::c_ulong)
                                .wrapping_sub(1 as libc::c_int as libc::c_ulong),
                        ) == 0
                            && (c_isspace(
                                *(name.b)
                                    .offset(
                                        (::std::mem::size_of::<[libc::c_char; 6]>()
                                            as libc::c_ulong)
                                            .wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize,
                                    ) as libc::c_int,
                            ) as libc::c_int != 0
                                || *(name.b)
                                    .offset(
                                        (::std::mem::size_of::<[libc::c_char; 6]>()
                                            as libc::c_ulong)
                                            .wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize,
                                    ) == 0))
                    {
                        basic = name.b;
                    }
                }
                www_authenticate = name.e;
                if opt.debug as libc::c_long != 0 {
                    debug_logprintf(
                        b"Auth param list '%s'\n\0" as *const u8 as *const libc::c_char,
                        www_authenticate,
                    );
                }
                while extract_param(
                    &mut www_authenticate,
                    &mut name,
                    &mut value,
                    ',' as i32 as libc::c_char,
                    0 as *mut bool,
                ) as libc::c_int != 0 && !(name.b).is_null() && !(value.b).is_null()
                {
                    if opt.debug as libc::c_long != 0 {
                        debug_logprintf(
                            b"Auth param %.*s=%.*s\n\0" as *const u8
                                as *const libc::c_char,
                            (name.e).offset_from(name.b) as libc::c_long as libc::c_int,
                            name.b,
                            (value.e).offset_from(value.b) as libc::c_long
                                as libc::c_int,
                            value.b,
                        );
                    }
                }
            }
            wapos += 1;
            wapos;
        }
        if basic.is_null() && digest.is_null() && ntlm.is_null() {
            logputs(
                LOG_NOTQUIET,
                dcgettext(
                    0 as *const libc::c_char,
                    b"Unknown authentication scheme.\n\0" as *const u8
                        as *const libc::c_char,
                    5 as libc::c_int,
                ),
            );
        } else if !basic_auth_finished || basic.is_null() {
            let mut pth: *mut libc::c_char = url_full_path(u);
            let mut value_0: *const libc::c_char = 0 as *const libc::c_char;
            let mut auth_stat: *mut uerr_t = 0 as *mut uerr_t;
            auth_stat = xmalloc(::std::mem::size_of::<uerr_t>() as libc::c_ulong)
                as *mut uerr_t;
            *auth_stat = RETROK;
            if !ntlm.is_null() {
                www_authenticate = ntlm;
            } else if !digest.is_null() {
                www_authenticate = digest;
            } else {
                www_authenticate = basic;
            }
            logprintf(
                LOG_NOTQUIET,
                dcgettext(
                    0 as *const libc::c_char,
                    b"Authentication selected: %s\n\0" as *const u8
                        as *const libc::c_char,
                    5 as libc::c_int,
                ),
                www_authenticate,
            );
            value_0 = create_authorization_line(
                www_authenticate,
                user,
                passwd,
                request_method(req),
                pth,
                &mut auth_finished,
                auth_stat,
            );
            auth_err = *auth_stat;
            rpl_free(auth_stat as *mut libc::c_void);
            auth_stat = 0 as *mut uerr_t;
            rpl_free(pth as *mut libc::c_void);
            pth = 0 as *mut libc::c_char;
            if auth_err as libc::c_uint == RETROK as libc::c_int as libc::c_uint {
                request_set_header(
                    req,
                    b"Authorization\0" as *const u8 as *const libc::c_char,
                    value_0,
                    rel_value,
                );
                if c_strncasecmp(
                    www_authenticate,
                    b"NTLM\0" as *const u8 as *const libc::c_char,
                    (::std::mem::size_of::<[libc::c_char; 5]>() as libc::c_ulong)
                        .wrapping_sub(1 as libc::c_int as libc::c_ulong),
                ) == 0
                    && (c_isspace(
                        *www_authenticate
                            .offset(
                                (::std::mem::size_of::<[libc::c_char; 5]>()
                                    as libc::c_ulong)
                                    .wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize,
                            ) as libc::c_int,
                    ) as libc::c_int != 0
                        || *www_authenticate
                            .offset(
                                (::std::mem::size_of::<[libc::c_char; 5]>()
                                    as libc::c_ulong)
                                    .wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize,
                            ) == 0)
                {
                    ntlm_seen = 1 as libc::c_int != 0;
                } else if ((*u).user).is_null()
                    && (c_strncasecmp(
                        www_authenticate,
                        b"Basic\0" as *const u8 as *const libc::c_char,
                        (::std::mem::size_of::<[libc::c_char; 6]>() as libc::c_ulong)
                            .wrapping_sub(1 as libc::c_int as libc::c_ulong),
                    ) == 0
                        && (c_isspace(
                            *www_authenticate
                                .offset(
                                    (::std::mem::size_of::<[libc::c_char; 6]>()
                                        as libc::c_ulong)
                                        .wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize,
                                ) as libc::c_int,
                        ) as libc::c_int != 0
                            || *www_authenticate
                                .offset(
                                    (::std::mem::size_of::<[libc::c_char; 6]>()
                                        as libc::c_ulong)
                                        .wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize,
                                ) == 0))
                {
                    register_basic_auth_host((*u).host);
                }
                *retry = 1 as libc::c_int != 0;
            } else {
                rpl_free(value_0 as *mut libc::c_void);
                value_0 = 0 as *const libc::c_char;
            }
        }
    }
    if tmp != buf.as_mut_ptr() {
        rpl_free(tmp as *mut libc::c_void);
        tmp = 0 as *mut libc::c_char;
    }
    *ntlm_seen_ref = ntlm_seen;
    *basic_auth_finished_ref = basic_auth_finished;
    *auth_finished_ref = auth_finished;
    return auth_err;
}
unsafe extern "C" fn open_output_stream(
    mut hs: *mut http_stat,
    mut count: libc::c_int,
    mut fp: *mut *mut FILE,
) -> uerr_t {
    if output_stream.is_null() {
        mkalldirs((*hs).local_file);
        if opt.backups != 0 {
            rotate_backups((*hs).local_file);
        }
        if (*hs).restval != 0 {
            *fp = rpl_fopen(
                (*hs).local_file,
                b"ab\0" as *const u8 as *const libc::c_char,
            );
        } else if opt.noclobber as libc::c_int != 0
            || opt.always_rest as libc::c_int != 0
            || opt.timestamping as libc::c_int != 0 || opt.dirstruct as libc::c_int != 0
            || !(opt.output_document).is_null() || opt.backups > 0 as libc::c_int
            || count > 0 as libc::c_int
        {
            if opt.unlink_requested as libc::c_int != 0
                && file_exists_p((*hs).local_file, 0 as *mut file_stats_t) as libc::c_int
                    != 0
            {
                if unlink((*hs).local_file) < 0 as libc::c_int {
                    logprintf(
                        LOG_NOTQUIET,
                        b"%s: %s\n\0" as *const u8 as *const libc::c_char,
                        (*hs).local_file,
                        strerror(*__errno_location()),
                    );
                    return UNLINKERR;
                }
            }
            if (*hs).temporary {
                *fp = fdopen(
                    open(
                        (*hs).local_file,
                        0 as libc::c_int | 0o100 as libc::c_int | 0o1000 as libc::c_int
                            | 0o1 as libc::c_int,
                        0o400 as libc::c_int | 0o200 as libc::c_int,
                    ),
                    b"wb\0" as *const u8 as *const libc::c_char,
                );
            } else {
                *fp = rpl_fopen(
                    (*hs).local_file,
                    b"wb\0" as *const u8 as *const libc::c_char,
                );
            }
        } else {
            *fp = fopen_excl((*hs).local_file, 1 as libc::c_int);
            if (*fp).is_null() && *__errno_location() == 17 as libc::c_int {
                logprintf(
                    LOG_NOTQUIET,
                    dcgettext(
                        0 as *const libc::c_char,
                        b"%s has sprung into existence.\n\0" as *const u8
                            as *const libc::c_char,
                        5 as libc::c_int,
                    ),
                    (*hs).local_file,
                );
                return FOPEN_EXCL_ERR;
            }
        }
        if (*fp).is_null() {
            logprintf(
                LOG_NOTQUIET,
                b"%s: %s\n\0" as *const u8 as *const libc::c_char,
                (*hs).local_file,
                strerror(*__errno_location()),
            );
            return FOPENERR;
        }
    } else {
        *fp = output_stream;
    }
    logprintf(
        LOG_VERBOSE,
        dcgettext(
            0 as *const libc::c_char,
            b"Saving to: %s\n\0" as *const u8 as *const libc::c_char,
            5 as libc::c_int,
        ),
        if *(*hs).local_file as libc::c_int == '-' as i32
            && *((*hs).local_file).offset(1 as libc::c_int as isize) == 0
        {
            quote(b"STDOUT\0" as *const u8 as *const libc::c_char)
        } else {
            quote((*hs).local_file)
        },
    );
    return RETROK;
}
unsafe extern "C" fn set_content_type(
    mut dt: *mut libc::c_int,
    mut type_0: *const libc::c_char,
) {
    if type_0.is_null()
        || 0 as libc::c_int
            == c_strcasecmp(type_0, b"text/html\0" as *const u8 as *const libc::c_char)
        || 0 as libc::c_int
            == c_strcasecmp(
                type_0,
                b"application/xhtml+xml\0" as *const u8 as *const libc::c_char,
            )
    {
        *dt |= TEXTHTML as libc::c_int;
    } else {
        *dt &= !(TEXTHTML as libc::c_int);
    }
    if !type_0.is_null()
        && 0 as libc::c_int
            == c_strcasecmp(type_0, b"text/css\0" as *const u8 as *const libc::c_char)
    {
        *dt |= TEXTCSS as libc::c_int;
    } else {
        *dt &= !(TEXTCSS as libc::c_int);
    };
}
unsafe extern "C" fn gethttp(
    mut u: *const url,
    mut original_url: *mut url,
    mut hs: *mut http_stat,
    mut dt: *mut libc::c_int,
    mut proxy: *mut url,
    mut iri: *mut iri,
    mut count: libc::c_int,
) -> uerr_t {
    let mut ret_0: uerr_t = NOCONERROR;
    let mut current_block: u64;
    let mut req: *mut request = 0 as *mut request;
    let mut type_0: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut user: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut passwd: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut proxyauth: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut statcode: libc::c_int = 0;
    let mut write_error: libc::c_int = 0;
    let mut contlen: wgint = 0;
    let mut contrange: wgint = 0;
    let mut conn: *const url = 0 as *const url;
    let mut fp: *mut FILE = 0 as *mut FILE;
    let mut err: libc::c_int = 0;
    let mut retval: uerr_t = NOCONERROR;
    let mut sock: libc::c_int = -(1 as libc::c_int);
    let mut auth_finished: bool = 0 as libc::c_int != 0;
    let mut basic_auth_finished: bool = 0 as libc::c_int != 0;
    let mut ntlm_seen: bool = 0 as libc::c_int != 0;
    let mut using_ssl: bool = 0 as libc::c_int != 0;
    let mut head_only: bool = *dt & HEAD_ONLY as libc::c_int != 0;
    let mut cond_get: bool = *dt & IF_MODIFIED_SINCE as libc::c_int != 0;
    let mut head: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut resp: *mut response = 0 as *mut response;
    let mut hdrval: [libc::c_char; 512] = [0; 512];
    let mut message: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut warc_enabled: bool = !(opt.warc_filename).is_null();
    let mut warc_tmp: *mut FILE = 0 as *mut FILE;
    let mut warc_timestamp_str: [libc::c_char; 21] = [0; 21];
    let mut warc_request_uuid: [libc::c_char; 48] = [0; 48];
    let mut warc_ip_buf: ip_address = ip_address {
        family: 0,
        data: C2RustUnnamed_5 {
            d4: in_addr { s_addr: 0 },
        },
        ipv6_scope: 0,
    };
    let mut warc_ip: *mut ip_address = 0 as *mut ip_address;
    let mut warc_payload_offset: off_t = -(1 as libc::c_int) as off_t;
    let mut keep_alive: bool = false;
    let mut chunked_transfer_encoding: bool = 0 as libc::c_int != 0;
    let mut inhibit_keep_alive: bool = !opt.http_keep_alive
        || opt.ignore_length as libc::c_int != 0;
    let mut body_data_size: wgint = 0 as libc::c_int as wgint;
    (*hs).len = 0 as libc::c_int as wgint;
    (*hs).contlen = -(1 as libc::c_int) as wgint;
    (*hs).res = -(1 as libc::c_int);
    rpl_free((*hs).rderrmsg as *mut libc::c_void);
    (*hs).rderrmsg = 0 as *mut libc::c_char;
    rpl_free((*hs).newloc as *mut libc::c_void);
    (*hs).newloc = 0 as *mut libc::c_char;
    rpl_free((*hs).remote_time as *mut libc::c_void);
    (*hs).remote_time = 0 as *mut libc::c_char;
    rpl_free((*hs).error as *mut libc::c_void);
    (*hs).error = 0 as *mut libc::c_char;
    rpl_free((*hs).message as *mut libc::c_void);
    (*hs).message = 0 as *mut libc::c_char;
    (*hs).local_encoding = ENC_NONE;
    (*hs).remote_encoding = ENC_NONE;
    conn = u;
    let mut ret: uerr_t = NOCONERROR;
    req = initialize_request(
        u,
        hs,
        dt,
        proxy,
        inhibit_keep_alive,
        &mut basic_auth_finished,
        &mut body_data_size,
        &mut user,
        &mut passwd,
        &mut ret,
    );
    if req.is_null() {
        retval = ret;
    } else {
        '_retry_with_auth: loop {
            if opt.cookies {
                request_set_header(
                    req,
                    b"Cookie\0" as *const u8 as *const libc::c_char,
                    cookie_header(
                        wget_cookie_jar,
                        (*u).host,
                        (*u).port,
                        (*u).path,
                        0 as libc::c_int != 0,
                    ),
                    rel_value,
                );
            }
            if !(opt.user_headers).is_null() {
                let mut i: libc::c_int = 0;
                i = 0 as libc::c_int;
                while !(*(opt.user_headers).offset(i as isize)).is_null() {
                    request_set_user_header(req, *(opt.user_headers).offset(i as isize));
                    i += 1;
                    i;
                }
            }
            proxyauth = 0 as *mut libc::c_char;
            if !proxy.is_null() {
                conn = proxy;
                initialize_proxy_configuration(u, req, proxy, &mut proxyauth);
            }
            keep_alive = 1 as libc::c_int != 0;
            if inhibit_keep_alive {
                keep_alive = 0 as libc::c_int != 0;
            }
            let mut conn_err: uerr_t = establish_connection(
                u,
                &mut conn,
                hs,
                proxy,
                &mut proxyauth,
                &mut req,
                &mut using_ssl,
                inhibit_keep_alive,
                &mut sock,
            );
            if conn_err as libc::c_uint != RETROK as libc::c_int as libc::c_uint {
                retval = conn_err;
                current_block = 14092045471819376021;
                break;
            } else {
                if warc_enabled {
                    warc_tmp = warc_tempfile();
                    if warc_tmp.is_null() {
                        if pconn_active as libc::c_int != 0 && sock == pconn.socket {
                            invalidate_persistent();
                        } else {
                            fd_close(sock);
                        }
                        sock = -(1 as libc::c_int);
                        retval = WARC_TMP_FOPENERR;
                        current_block = 14092045471819376021;
                        break;
                    } else if proxy.is_null() {
                        warc_ip = &mut warc_ip_buf;
                        socket_ip_address(sock, warc_ip, ENDPOINT_PEER as libc::c_int);
                    }
                }
                write_error = request_send(req, sock, warc_tmp);
                if write_error >= 0 as libc::c_int {
                    if !(opt.body_data).is_null() {
                        if opt.debug as libc::c_long != 0 {
                            debug_logprintf(
                                b"[BODY data: %s]\n\0" as *const u8 as *const libc::c_char,
                                opt.body_data,
                            );
                        }
                        write_error = fd_write(
                            sock,
                            opt.body_data,
                            body_data_size as libc::c_int,
                            -(1 as libc::c_int) as libc::c_double,
                        );
                        if write_error >= 0 as libc::c_int && !warc_tmp.is_null() {
                            let mut warc_tmp_written: libc::c_int = 0;
                            warc_payload_offset = ftello(warc_tmp);
                            warc_tmp_written = fwrite(
                                opt.body_data as *const libc::c_void,
                                1 as libc::c_int as libc::c_ulong,
                                body_data_size as libc::c_ulong,
                                warc_tmp,
                            ) as libc::c_int;
                            if warc_tmp_written as libc::c_long != body_data_size {
                                write_error = -(2 as libc::c_int);
                            }
                        }
                    } else if !(opt.body_file).is_null()
                        && body_data_size != 0 as libc::c_int as libc::c_long
                    {
                        if !warc_tmp.is_null() {
                            warc_payload_offset = ftello(warc_tmp);
                        }
                        write_error = body_file_send(
                            sock,
                            opt.body_file,
                            body_data_size,
                            warc_tmp,
                        );
                    }
                }
                if write_error < 0 as libc::c_int {
                    if pconn_active as libc::c_int != 0 && sock == pconn.socket {
                        invalidate_persistent();
                    } else {
                        fd_close(sock);
                    }
                    sock = -(1 as libc::c_int);
                    if !warc_tmp.is_null() {
                        fclose(warc_tmp);
                    }
                    if write_error == -(2 as libc::c_int) {
                        retval = WARC_TMP_FWRITEERR;
                    } else {
                        retval = WRITEFAILED;
                    }
                    current_block = 14092045471819376021;
                    break;
                } else {
                    logprintf(
                        LOG_VERBOSE,
                        dcgettext(
                            0 as *const libc::c_char,
                            b"%s request sent, awaiting response... \0" as *const u8
                                as *const libc::c_char,
                            5 as libc::c_int,
                        ),
                        if !proxy.is_null() {
                            b"Proxy\0" as *const u8 as *const libc::c_char
                        } else {
                            b"HTTP\0" as *const u8 as *const libc::c_char
                        },
                    );
                    contlen = -(1 as libc::c_int) as wgint;
                    contrange = 0 as libc::c_int as wgint;
                    *dt &= !(RETROKF as libc::c_int);
                    if warc_enabled {
                        let mut warc_result: bool = false;
                        warc_timestamp(
                            warc_timestamp_str.as_mut_ptr(),
                            ::std::mem::size_of::<[libc::c_char; 21]>() as libc::c_ulong,
                        );
                        warc_uuid_str(
                            warc_request_uuid.as_mut_ptr(),
                            ::std::mem::size_of::<[libc::c_char; 48]>() as libc::c_ulong,
                        );
                        warc_result = warc_write_request_record(
                            (*u).url,
                            warc_timestamp_str.as_mut_ptr(),
                            warc_request_uuid.as_mut_ptr(),
                            warc_ip,
                            warc_tmp,
                            warc_payload_offset,
                        );
                        if !warc_result {
                            if pconn_active as libc::c_int != 0 && sock == pconn.socket {
                                invalidate_persistent();
                            } else {
                                fd_close(sock);
                            }
                            sock = -(1 as libc::c_int);
                            retval = WARC_ERR;
                            current_block = 14092045471819376021;
                            break;
                        }
                    }
                    let mut _repeat: bool = false;
                    loop {
                        head = read_http_response_head(sock);
                        if head.is_null() {
                            if *__errno_location() == 0 as libc::c_int {
                                logputs(
                                    LOG_NOTQUIET,
                                    dcgettext(
                                        0 as *const libc::c_char,
                                        b"No data received.\n\0" as *const u8
                                            as *const libc::c_char,
                                        5 as libc::c_int,
                                    ),
                                );
                                if pconn_active as libc::c_int != 0 && sock == pconn.socket
                                {
                                    invalidate_persistent();
                                } else {
                                    fd_close(sock);
                                }
                                sock = -(1 as libc::c_int);
                                retval = HEOF;
                            } else {
                                logprintf(
                                    LOG_NOTQUIET,
                                    dcgettext(
                                        0 as *const libc::c_char,
                                        b"Read error (%s) in headers.\n\0" as *const u8
                                            as *const libc::c_char,
                                        5 as libc::c_int,
                                    ),
                                    fd_errstr(sock),
                                );
                                if pconn_active as libc::c_int != 0 && sock == pconn.socket
                                {
                                    invalidate_persistent();
                                } else {
                                    fd_close(sock);
                                }
                                sock = -(1 as libc::c_int);
                                retval = HERR;
                            }
                            current_block = 14092045471819376021;
                            break '_retry_with_auth;
                        } else {
                            if opt.debug as libc::c_long != 0 {
                                debug_logprintf(
                                    b"\n---response begin---\n%s---response end---\n\0"
                                        as *const u8 as *const libc::c_char,
                                    head,
                                );
                            }
                            resp = resp_new(head);
                            rpl_free(message as *mut libc::c_void);
                            message = 0 as *mut libc::c_char;
                            statcode = resp_status(resp, &mut message);
                            if statcode < 0 as libc::c_int {
                                let mut tms: *mut libc::c_char = datetime_str(
                                    time(0 as *mut time_t),
                                );
                                logprintf(
                                    LOG_VERBOSE,
                                    b"%d\n\0" as *const u8 as *const libc::c_char,
                                    statcode,
                                );
                                logprintf(
                                    LOG_NOTQUIET,
                                    dcgettext(
                                        0 as *const libc::c_char,
                                        b"%s ERROR %d: %s.\n\0" as *const u8 as *const libc::c_char,
                                        5 as libc::c_int,
                                    ),
                                    tms,
                                    statcode,
                                    quotearg_style(
                                        escape_quoting_style,
                                        dcgettext(
                                            0 as *const libc::c_char,
                                            b"Malformed status line\0" as *const u8
                                                as *const libc::c_char,
                                            5 as libc::c_int,
                                        ),
                                    ),
                                );
                                if pconn_active as libc::c_int != 0 && sock == pconn.socket
                                {
                                    invalidate_persistent();
                                } else {
                                    fd_close(sock);
                                }
                                sock = -(1 as libc::c_int);
                                retval = HERR;
                                current_block = 14092045471819376021;
                                break '_retry_with_auth;
                            } else {
                                if statcode >= 100 as libc::c_int
                                    && statcode < 200 as libc::c_int
                                {
                                    rpl_free(head as *mut libc::c_void);
                                    head = 0 as *mut libc::c_char;
                                    resp_free(&mut resp);
                                    _repeat = 1 as libc::c_int != 0;
                                    if opt.debug as libc::c_long != 0 {
                                        debug_logprintf(
                                            b"Ignoring response\n\0" as *const u8 as *const libc::c_char,
                                        );
                                    }
                                } else {
                                    _repeat = 0 as libc::c_int != 0;
                                }
                                if !_repeat {
                                    break;
                                }
                            }
                        }
                    }
                    rpl_free((*hs).message as *mut libc::c_void);
                    (*hs).message = 0 as *mut libc::c_char;
                    (*hs).message = xstrdup(message);
                    if !opt.server_response {
                        logprintf(
                            LOG_VERBOSE,
                            b"%2d %s\n\0" as *const u8 as *const libc::c_char,
                            statcode,
                            if !message.is_null() {
                                quotearg_style(escape_quoting_style, message)
                                    as *const libc::c_char
                            } else {
                                b"\0" as *const u8 as *const libc::c_char
                            },
                        );
                    } else {
                        logprintf(
                            LOG_VERBOSE,
                            b"\n\0" as *const u8 as *const libc::c_char,
                        );
                        print_server_response(
                            resp,
                            b"  \0" as *const u8 as *const libc::c_char,
                        );
                    }
                    if !opt.ignore_length
                        && resp_header_copy(
                            resp,
                            b"Content-Length\0" as *const u8 as *const libc::c_char,
                            hdrval.as_mut_ptr(),
                            ::std::mem::size_of::<[libc::c_char; 512]>() as libc::c_ulong
                                as libc::c_int,
                        ) as libc::c_int != 0
                    {
                        let mut parsed: wgint = 0;
                        *__errno_location() = 0 as libc::c_int;
                        parsed = rpl_strtoll(
                            hdrval.as_mut_ptr(),
                            0 as *mut *mut libc::c_char,
                            10 as libc::c_int,
                        ) as wgint;
                        if parsed == 9223372036854775807 as libc::c_long
                            && *__errno_location() == 34 as libc::c_int
                        {
                            contlen = -(1 as libc::c_int) as wgint;
                        } else if parsed < 0 as libc::c_int as libc::c_long {
                            contlen = -(1 as libc::c_int) as wgint;
                        } else {
                            contlen = parsed;
                        }
                    }
                    if !inhibit_keep_alive {
                        if resp_header_copy(
                            resp,
                            b"Connection\0" as *const u8 as *const libc::c_char,
                            hdrval.as_mut_ptr(),
                            ::std::mem::size_of::<[libc::c_char; 512]>() as libc::c_ulong
                                as libc::c_int,
                        ) {
                            if 0 as libc::c_int
                                == c_strcasecmp(
                                    hdrval.as_mut_ptr(),
                                    b"Close\0" as *const u8 as *const libc::c_char,
                                )
                            {
                                keep_alive = 0 as libc::c_int != 0;
                            }
                        }
                    }
                    chunked_transfer_encoding = 0 as libc::c_int != 0;
                    if resp_header_copy(
                        resp,
                        b"Transfer-Encoding\0" as *const u8 as *const libc::c_char,
                        hdrval.as_mut_ptr(),
                        ::std::mem::size_of::<[libc::c_char; 512]>() as libc::c_ulong
                            as libc::c_int,
                    ) as libc::c_int != 0
                        && 0 as libc::c_int
                            == c_strcasecmp(
                                hdrval.as_mut_ptr(),
                                b"chunked\0" as *const u8 as *const libc::c_char,
                            )
                    {
                        chunked_transfer_encoding = 1 as libc::c_int != 0;
                    }
                    if opt.cookies {
                        let mut scpos: libc::c_int = 0;
                        let mut scbeg: *const libc::c_char = 0 as *const libc::c_char;
                        let mut scend: *const libc::c_char = 0 as *const libc::c_char;
                        scpos = 0 as libc::c_int;
                        loop {
                            scpos = resp_header_locate(
                                resp,
                                b"Set-Cookie\0" as *const u8 as *const libc::c_char,
                                scpos,
                                &mut scbeg,
                                &mut scend,
                            );
                            if !(scpos != -(1 as libc::c_int)) {
                                break;
                            }
                            let mut buf: [libc::c_char; 1024] = [0; 1024];
                            let mut set_cookie: *mut libc::c_char = 0
                                as *mut libc::c_char;
                            let mut len: size_t = scend.offset_from(scbeg)
                                as libc::c_long as size_t;
                            if len
                                < ::std::mem::size_of::<[libc::c_char; 1024]>()
                                    as libc::c_ulong
                            {
                                set_cookie = buf.as_mut_ptr();
                            } else {
                                set_cookie = xmalloc(
                                    len.wrapping_add(1 as libc::c_int as libc::c_ulong),
                                ) as *mut libc::c_char;
                            }
                            memcpy(
                                set_cookie as *mut libc::c_void,
                                scbeg as *const libc::c_void,
                                len,
                            );
                            *set_cookie
                                .offset(len as isize) = 0 as libc::c_int as libc::c_char;
                            cookie_handle_set_cookie(
                                wget_cookie_jar,
                                (*u).host,
                                (*u).port,
                                (*u).path,
                                set_cookie,
                            );
                            if set_cookie != buf.as_mut_ptr() {
                                rpl_free(set_cookie as *mut libc::c_void);
                                set_cookie = 0 as *mut libc::c_char;
                            }
                            scpos += 1;
                            scpos;
                        }
                    }
                    if keep_alive {
                        register_persistent((*conn).host, (*conn).port, sock, using_ssl);
                    }
                    if statcode == 401 as libc::c_int {
                        let mut auth_err: uerr_t = RETROK;
                        let mut retry: bool = false;
                        if warc_enabled {
                            let mut _err: libc::c_int = 0;
                            type_0 = resp_header_strdup(
                                resp,
                                b"Content-Type\0" as *const u8 as *const libc::c_char,
                            );
                            _err = read_response_body(
                                hs,
                                sock,
                                0 as *mut FILE,
                                contlen,
                                0 as libc::c_int as wgint,
                                chunked_transfer_encoding,
                                (*u).url,
                                warc_timestamp_str.as_mut_ptr(),
                                warc_request_uuid.as_mut_ptr(),
                                warc_ip,
                                type_0,
                                statcode,
                                head,
                            );
                            rpl_free(type_0 as *mut libc::c_void);
                            type_0 = 0 as *mut libc::c_char;
                            if _err != RETRFINISHED as libc::c_int
                                || (*hs).res < 0 as libc::c_int
                            {
                                if pconn_active as libc::c_int != 0 && sock == pconn.socket
                                {
                                    invalidate_persistent();
                                } else {
                                    fd_close(sock);
                                }
                                sock = -(1 as libc::c_int);
                                retval = _err as uerr_t;
                                current_block = 14092045471819376021;
                                break;
                            } else if !keep_alive {
                                if pconn_active as libc::c_int != 0 && sock == pconn.socket
                                {
                                    invalidate_persistent();
                                } else {
                                    fd_close(sock);
                                }
                                sock = -(1 as libc::c_int);
                            }
                        } else if keep_alive as libc::c_int != 0 && !head_only
                            && skip_short_body(sock, contlen, chunked_transfer_encoding)
                                as libc::c_int != 0
                        {
                            if !keep_alive {
                                if pconn_active as libc::c_int != 0 && sock == pconn.socket
                                {
                                    invalidate_persistent();
                                } else {
                                    fd_close(sock);
                                }
                                sock = -(1 as libc::c_int);
                            }
                        } else {
                            if pconn_active as libc::c_int != 0 && sock == pconn.socket {
                                invalidate_persistent();
                            } else {
                                fd_close(sock);
                            }
                            sock = -(1 as libc::c_int);
                        }
                        pconn.authorized = 0 as libc::c_int != 0;
                        auth_err = check_auth(
                            u,
                            user,
                            passwd,
                            resp,
                            req,
                            &mut ntlm_seen,
                            &mut retry,
                            &mut basic_auth_finished,
                            &mut auth_finished,
                        );
                        if auth_err as libc::c_uint
                            == RETROK as libc::c_int as libc::c_uint
                            && retry as libc::c_int != 0
                        {
                            resp_free(&mut resp);
                            rpl_free(message as *mut libc::c_void);
                            message = 0 as *mut libc::c_char;
                            rpl_free(head as *mut libc::c_void);
                            head = 0 as *mut libc::c_char;
                        } else {
                            if auth_err as libc::c_uint
                                == RETROK as libc::c_int as libc::c_uint
                            {
                                retval = AUTHFAILED;
                            } else {
                                retval = auth_err;
                            }
                            current_block = 14092045471819376021;
                            break;
                        }
                    } else {
                        if ntlm_seen {
                            pconn.authorized = 1 as libc::c_int != 0;
                        }
                        ret_0 = check_file_output(
                            u,
                            hs,
                            resp,
                            hdrval.as_mut_ptr(),
                            ::std::mem::size_of::<[libc::c_char; 512]>() as libc::c_ulong,
                        );
                        if ret_0 as libc::c_uint != RETROK as libc::c_int as libc::c_uint
                        {
                            current_block = 11256927402564620217;
                            break;
                        } else {
                            current_block = 4050268373526174;
                            break;
                        }
                    }
                }
            }
        }
        match current_block {
            14092045471819376021 => {}
            _ => {
                match current_block {
                    11256927402564620217 => {
                        retval = ret_0;
                    }
                    _ => {
                        (*hs).statcode = statcode;
                        rpl_free((*hs).error as *mut libc::c_void);
                        (*hs).error = 0 as *mut libc::c_char;
                        if statcode == -(1 as libc::c_int) {
                            (*hs)
                                .error = xstrdup(
                                dcgettext(
                                    0 as *const libc::c_char,
                                    b"Malformed status line\0" as *const u8
                                        as *const libc::c_char,
                                    5 as libc::c_int,
                                ),
                            );
                        } else if message.is_null() || *message == 0 {
                            (*hs)
                                .error = xstrdup(
                                dcgettext(
                                    0 as *const libc::c_char,
                                    b"(no description)\0" as *const u8 as *const libc::c_char,
                                    5 as libc::c_int,
                                ),
                            );
                        } else {
                            (*hs).error = xstrdup(message);
                        }
                        type_0 = resp_header_strdup(
                            resp,
                            b"Content-Type\0" as *const u8 as *const libc::c_char,
                        );
                        if !type_0.is_null() {
                            let mut tmp: *mut libc::c_char = strchr(type_0, ';' as i32);
                            if !tmp.is_null() {
                                let mut tmp2: *mut libc::c_char = tmp
                                    .offset(1 as libc::c_int as isize);
                                while tmp > type_0
                                    && c_isspace(
                                        *tmp.offset(-(1 as libc::c_int) as isize) as libc::c_int,
                                    ) as libc::c_int != 0
                                {
                                    tmp = tmp.offset(-1);
                                    tmp;
                                }
                                *tmp = '\0' as i32 as libc::c_char;
                                if opt.enable_iri as libc::c_int != 0
                                    && (opt.encoding_remote).is_null()
                                {
                                    tmp = parse_charset(tmp2);
                                    if !tmp.is_null() {
                                        set_content_encoding(iri, tmp);
                                    }
                                    rpl_free(tmp as *mut libc::c_void);
                                    tmp = 0 as *mut libc::c_char;
                                }
                            }
                        }
                        rpl_free((*hs).newloc as *mut libc::c_void);
                        (*hs).newloc = 0 as *mut libc::c_char;
                        (*hs)
                            .newloc = resp_header_strdup(
                            resp,
                            b"Location\0" as *const u8 as *const libc::c_char,
                        );
                        rpl_free((*hs).remote_time as *mut libc::c_void);
                        (*hs).remote_time = 0 as *mut libc::c_char;
                        (*hs)
                            .remote_time = resp_header_strdup(
                            resp,
                            b"Last-Modified\0" as *const u8 as *const libc::c_char,
                        );
                        if ((*hs).remote_time).is_null() {
                            (*hs)
                                .remote_time = resp_header_strdup(
                                resp,
                                b"X-Archive-Orig-last-modified\0" as *const u8
                                    as *const libc::c_char,
                            );
                        }
                        if resp_header_copy(
                            resp,
                            b"Content-Range\0" as *const u8 as *const libc::c_char,
                            hdrval.as_mut_ptr(),
                            ::std::mem::size_of::<[libc::c_char; 512]>() as libc::c_ulong
                                as libc::c_int,
                        ) {
                            let mut first_byte_pos: wgint = 0;
                            let mut last_byte_pos: wgint = 0;
                            let mut entity_length: wgint = 0;
                            if parse_content_range(
                                hdrval.as_mut_ptr(),
                                &mut first_byte_pos,
                                &mut last_byte_pos,
                                &mut entity_length,
                            ) {
                                contrange = first_byte_pos;
                                contlen = last_byte_pos - first_byte_pos
                                    + 1 as libc::c_int as libc::c_long;
                            }
                        }
                        if resp_header_copy(
                            resp,
                            b"Content-Encoding\0" as *const u8 as *const libc::c_char,
                            hdrval.as_mut_ptr(),
                            ::std::mem::size_of::<[libc::c_char; 512]>() as libc::c_ulong
                                as libc::c_int,
                        ) {
                            (*hs).local_encoding = ENC_INVALID;
                            match hdrval[0 as libc::c_int as usize] as libc::c_int {
                                98 | 66 => {
                                    if 0 as libc::c_int
                                        == c_strcasecmp(
                                            hdrval.as_mut_ptr(),
                                            b"br\0" as *const u8 as *const libc::c_char,
                                        )
                                    {
                                        (*hs).local_encoding = ENC_BROTLI;
                                    }
                                }
                                99 | 67 => {
                                    if 0 as libc::c_int
                                        == c_strcasecmp(
                                            hdrval.as_mut_ptr(),
                                            b"compress\0" as *const u8 as *const libc::c_char,
                                        )
                                    {
                                        (*hs).local_encoding = ENC_COMPRESS;
                                    }
                                }
                                100 | 68 => {
                                    if 0 as libc::c_int
                                        == c_strcasecmp(
                                            hdrval.as_mut_ptr(),
                                            b"deflate\0" as *const u8 as *const libc::c_char,
                                        )
                                    {
                                        (*hs).local_encoding = ENC_DEFLATE;
                                    }
                                }
                                103 | 71 => {
                                    if 0 as libc::c_int
                                        == c_strcasecmp(
                                            hdrval.as_mut_ptr(),
                                            b"gzip\0" as *const u8 as *const libc::c_char,
                                        )
                                    {
                                        (*hs).local_encoding = ENC_GZIP;
                                    }
                                }
                                105 | 73 => {
                                    if 0 as libc::c_int
                                        == c_strcasecmp(
                                            hdrval.as_mut_ptr(),
                                            b"identity\0" as *const u8 as *const libc::c_char,
                                        )
                                    {
                                        (*hs).local_encoding = ENC_NONE;
                                    }
                                }
                                120 | 88 => {
                                    if 0 as libc::c_int
                                        == c_strcasecmp(
                                            hdrval.as_mut_ptr(),
                                            b"x-compress\0" as *const u8 as *const libc::c_char,
                                        )
                                    {
                                        (*hs).local_encoding = ENC_COMPRESS;
                                    } else if 0 as libc::c_int
                                        == c_strcasecmp(
                                            hdrval.as_mut_ptr(),
                                            b"x-gzip\0" as *const u8 as *const libc::c_char,
                                        )
                                    {
                                        (*hs).local_encoding = ENC_GZIP;
                                    }
                                }
                                0 => {
                                    (*hs).local_encoding = ENC_NONE;
                                }
                                _ => {}
                            }
                            if (*hs).local_encoding as libc::c_int
                                == ENC_INVALID as libc::c_int
                            {
                                if opt.debug as libc::c_long != 0 {
                                    debug_logprintf(
                                        b"Unrecognized Content-Encoding: %s\n\0" as *const u8
                                            as *const libc::c_char,
                                        hdrval.as_mut_ptr(),
                                    );
                                }
                                (*hs).local_encoding = ENC_NONE;
                            } else if (*hs).local_encoding as libc::c_int
                                == ENC_GZIP as libc::c_int
                                && opt.compression as libc::c_uint
                                    != compression_none as libc::c_int as libc::c_uint
                            {
                                let mut p: *const libc::c_char = 0 as *const libc::c_char;
                                if !type_0.is_null() {
                                    p = strchr(type_0, '/' as i32);
                                    if p.is_null() {
                                        (*hs).remote_encoding = ENC_GZIP;
                                        (*hs).local_encoding = ENC_NONE;
                                    } else {
                                        p = p.offset(1);
                                        p;
                                        if c_tolower(
                                            *p.offset(0 as libc::c_int as isize) as libc::c_int,
                                        ) == 'x' as i32
                                            && *p.offset(1 as libc::c_int as isize) as libc::c_int
                                                == '-' as i32
                                        {
                                            p = p.offset(2 as libc::c_int as isize);
                                        }
                                        if 0 as libc::c_int
                                            != c_strcasecmp(
                                                p,
                                                b"gzip\0" as *const u8 as *const libc::c_char,
                                            )
                                        {
                                            (*hs).remote_encoding = ENC_GZIP;
                                            (*hs).local_encoding = ENC_NONE;
                                        }
                                    }
                                } else {
                                    (*hs).remote_encoding = ENC_GZIP;
                                    (*hs).local_encoding = ENC_NONE;
                                }
                                if (*hs).remote_encoding as libc::c_int
                                    == ENC_GZIP as libc::c_int
                                    && {
                                        p = strrchr((*u).file, '.' as i32);
                                        !p.is_null()
                                    }
                                    && (c_strcasecmp(
                                        p,
                                        b".gz\0" as *const u8 as *const libc::c_char,
                                    ) == 0 as libc::c_int
                                        || c_strcasecmp(
                                            p,
                                            b".tgz\0" as *const u8 as *const libc::c_char,
                                        ) == 0 as libc::c_int)
                                {
                                    if opt.debug as libc::c_long != 0 {
                                        debug_logprintf(
                                            b"Enabling broken server workaround. Will not decompress this GZip file.\n\0"
                                                as *const u8 as *const libc::c_char,
                                        );
                                    }
                                    (*hs).remote_encoding = ENC_NONE;
                                }
                            }
                        }
                        if statcode >= 200 as libc::c_int
                            && statcode < 300 as libc::c_int
                        {
                            *dt |= RETROKF as libc::c_int;
                        }
                        if statcode == 204 as libc::c_int {
                            (*hs).len = 0 as libc::c_int as wgint;
                            (*hs).res = 0 as libc::c_int;
                            (*hs).restval = 0 as libc::c_int as wgint;
                            if !keep_alive {
                                if pconn_active as libc::c_int != 0 && sock == pconn.socket
                                {
                                    invalidate_persistent();
                                } else {
                                    fd_close(sock);
                                }
                                sock = -(1 as libc::c_int);
                            }
                            retval = RETRFINISHED;
                        } else {
                            if statcode == 301 as libc::c_int
                                || statcode == 302 as libc::c_int
                                || statcode == 303 as libc::c_int
                                || statcode == 307 as libc::c_int
                                || statcode == 308 as libc::c_int
                                || statcode == 300 as libc::c_int
                            {
                                if statcode == 300 as libc::c_int
                                    && ((*hs).newloc).is_null()
                                {
                                    *dt |= RETROKF as libc::c_int;
                                    current_block = 10245939215942870705;
                                } else {
                                    logprintf(
                                        LOG_VERBOSE,
                                        dcgettext(
                                            0 as *const libc::c_char,
                                            b"Location: %s%s\n\0" as *const u8 as *const libc::c_char,
                                            5 as libc::c_int,
                                        ),
                                        if !((*hs).newloc).is_null() {
                                            escnonprint_uri((*hs).newloc)
                                        } else {
                                            dcgettext(
                                                0 as *const libc::c_char,
                                                b"unspecified\0" as *const u8 as *const libc::c_char,
                                                5 as libc::c_int,
                                            ) as *const libc::c_char
                                        },
                                        if !((*hs).newloc).is_null() {
                                            dcgettext(
                                                0 as *const libc::c_char,
                                                b" [following]\0" as *const u8 as *const libc::c_char,
                                                5 as libc::c_int,
                                            ) as *const libc::c_char
                                        } else {
                                            b"\0" as *const u8 as *const libc::c_char
                                        },
                                    );
                                    (*hs).len = 0 as libc::c_int as wgint;
                                    (*hs).res = 0 as libc::c_int;
                                    (*hs).restval = 0 as libc::c_int as wgint;
                                    if warc_enabled {
                                        let mut _err_0: libc::c_int = read_response_body(
                                            hs,
                                            sock,
                                            0 as *mut FILE,
                                            contlen,
                                            0 as libc::c_int as wgint,
                                            chunked_transfer_encoding,
                                            (*u).url,
                                            warc_timestamp_str.as_mut_ptr(),
                                            warc_request_uuid.as_mut_ptr(),
                                            warc_ip,
                                            type_0,
                                            statcode,
                                            head,
                                        );
                                        if _err_0 != RETRFINISHED as libc::c_int
                                            || (*hs).res < 0 as libc::c_int
                                        {
                                            if pconn_active as libc::c_int != 0 && sock == pconn.socket
                                            {
                                                invalidate_persistent();
                                            } else {
                                                fd_close(sock);
                                            }
                                            sock = -(1 as libc::c_int);
                                            retval = _err_0 as uerr_t;
                                            current_block = 14092045471819376021;
                                        } else {
                                            if !keep_alive {
                                                if pconn_active as libc::c_int != 0 && sock == pconn.socket
                                                {
                                                    invalidate_persistent();
                                                } else {
                                                    fd_close(sock);
                                                }
                                                sock = -(1 as libc::c_int);
                                            }
                                            current_block = 494012601817399562;
                                        }
                                    } else {
                                        if keep_alive as libc::c_int != 0 && !head_only
                                            && skip_short_body(sock, contlen, chunked_transfer_encoding)
                                                as libc::c_int != 0
                                        {
                                            if !keep_alive {
                                                if pconn_active as libc::c_int != 0 && sock == pconn.socket
                                                {
                                                    invalidate_persistent();
                                                } else {
                                                    fd_close(sock);
                                                }
                                                sock = -(1 as libc::c_int);
                                            }
                                        } else {
                                            if pconn_active as libc::c_int != 0 && sock == pconn.socket
                                            {
                                                invalidate_persistent();
                                            } else {
                                                fd_close(sock);
                                            }
                                            sock = -(1 as libc::c_int);
                                        }
                                        current_block = 494012601817399562;
                                    }
                                    match current_block {
                                        14092045471819376021 => {}
                                        _ => {
                                            match statcode {
                                                307 | 308 => {
                                                    retval = NEWLOCATION_KEEP_POST;
                                                    current_block = 14092045471819376021;
                                                }
                                                301 => {
                                                    if !(opt.method).is_null()
                                                        && c_strcasecmp(
                                                            opt.method,
                                                            b"post\0" as *const u8 as *const libc::c_char,
                                                        ) != 0 as libc::c_int
                                                    {
                                                        retval = NEWLOCATION_KEEP_POST;
                                                        current_block = 14092045471819376021;
                                                    } else {
                                                        current_block = 4267368308198710847;
                                                    }
                                                }
                                                302 => {
                                                    if !(opt.method).is_null()
                                                        && c_strcasecmp(
                                                            opt.method,
                                                            b"post\0" as *const u8 as *const libc::c_char,
                                                        ) != 0 as libc::c_int
                                                    {
                                                        retval = NEWLOCATION_KEEP_POST;
                                                        current_block = 14092045471819376021;
                                                    } else {
                                                        current_block = 4267368308198710847;
                                                    }
                                                }
                                                _ => {
                                                    current_block = 4267368308198710847;
                                                }
                                            }
                                            match current_block {
                                                14092045471819376021 => {}
                                                _ => {
                                                    retval = NEWLOCATION;
                                                    current_block = 14092045471819376021;
                                                }
                                            }
                                        }
                                    }
                                }
                            } else {
                                current_block = 10245939215942870705;
                            }
                            match current_block {
                                14092045471819376021 => {}
                                _ => {
                                    if cond_get {
                                        if statcode == 304 as libc::c_int {
                                            logprintf(
                                                LOG_VERBOSE,
                                                dcgettext(
                                                    0 as *const libc::c_char,
                                                    b"File %s not modified on server. Omitting download.\n\n\0"
                                                        as *const u8 as *const libc::c_char,
                                                    5 as libc::c_int,
                                                ),
                                                quote((*hs).local_file),
                                            );
                                            *dt |= RETROKF as libc::c_int;
                                            if !keep_alive {
                                                if pconn_active as libc::c_int != 0 && sock == pconn.socket
                                                {
                                                    invalidate_persistent();
                                                } else {
                                                    fd_close(sock);
                                                }
                                                sock = -(1 as libc::c_int);
                                            }
                                            retval = RETRUNNEEDED;
                                            current_block = 14092045471819376021;
                                        } else {
                                            current_block = 14540000294252553875;
                                        }
                                    } else {
                                        current_block = 14540000294252553875;
                                    }
                                    match current_block {
                                        14092045471819376021 => {}
                                        _ => {
                                            set_content_type(dt, type_0);
                                            if opt.adjust_extension {
                                                let mut encoding_ext: *const libc::c_char = 0
                                                    as *const libc::c_char;
                                                match (*hs).local_encoding as libc::c_int {
                                                    -1 | 0 => {}
                                                    4 => {
                                                        encoding_ext = b".br\0" as *const u8 as *const libc::c_char;
                                                    }
                                                    3 => {
                                                        encoding_ext = b".Z\0" as *const u8 as *const libc::c_char;
                                                    }
                                                    2 => {
                                                        encoding_ext = b".zlib\0" as *const u8
                                                            as *const libc::c_char;
                                                    }
                                                    1 => {
                                                        encoding_ext = b".gz\0" as *const u8 as *const libc::c_char;
                                                    }
                                                    _ => {
                                                        if opt.debug as libc::c_long != 0 {
                                                            debug_logprintf(
                                                                b"No extension found for encoding %d\n\0" as *const u8
                                                                    as *const libc::c_char,
                                                                (*hs).local_encoding as libc::c_int,
                                                            );
                                                        }
                                                    }
                                                }
                                                if !encoding_ext.is_null() {
                                                    let mut file_ext: *mut libc::c_char = strrchr(
                                                        (*hs).local_file,
                                                        '.' as i32,
                                                    );
                                                    if !file_ext.is_null()
                                                        && 0 as libc::c_int == strcasecmp(file_ext, encoding_ext)
                                                    {
                                                        *file_ext = '\0' as i32 as libc::c_char;
                                                    }
                                                }
                                                if *dt & TEXTHTML as libc::c_int != 0 {
                                                    ensure_extension(
                                                        hs,
                                                        b".html\0" as *const u8 as *const libc::c_char,
                                                        dt,
                                                    );
                                                } else if *dt & TEXTCSS as libc::c_int != 0 {
                                                    ensure_extension(
                                                        hs,
                                                        b".css\0" as *const u8 as *const libc::c_char,
                                                        dt,
                                                    );
                                                }
                                                if !encoding_ext.is_null() {
                                                    ensure_extension(hs, encoding_ext, dt);
                                                }
                                            }
                                            if cond_get {
                                                if statcode == 200 as libc::c_int
                                                    && !((*hs).remote_time).is_null()
                                                {
                                                    let mut tmr: time_t = http_atotm((*hs).remote_time);
                                                    if tmr != -(1 as libc::c_int) as time_t
                                                        && tmr <= (*hs).orig_file_tstamp
                                                        && (contlen == -(1 as libc::c_int) as libc::c_long
                                                            || contlen == (*hs).orig_file_size)
                                                    {
                                                        logprintf(
                                                            LOG_VERBOSE,
                                                            dcgettext(
                                                                0 as *const libc::c_char,
                                                                b"Server ignored If-Modified-Since header for file %s.\nYou might want to add --no-if-modified-since option.\n\n\0"
                                                                    as *const u8 as *const libc::c_char,
                                                                5 as libc::c_int,
                                                            ),
                                                            quote((*hs).local_file),
                                                        );
                                                        *dt |= RETROKF as libc::c_int;
                                                        if pconn_active as libc::c_int != 0 && sock == pconn.socket
                                                        {
                                                            invalidate_persistent();
                                                        } else {
                                                            fd_close(sock);
                                                        }
                                                        sock = -(1 as libc::c_int);
                                                        retval = RETRUNNEEDED;
                                                        current_block = 14092045471819376021;
                                                    } else {
                                                        current_block = 4077408264086944124;
                                                    }
                                                } else {
                                                    current_block = 4077408264086944124;
                                                }
                                            } else {
                                                current_block = 4077408264086944124;
                                            }
                                            match current_block {
                                                14092045471819376021 => {}
                                                _ => {
                                                    if statcode == 416 as libc::c_int
                                                        || !opt.timestamping
                                                            && (*hs).restval > 0 as libc::c_int as libc::c_long
                                                            && statcode == 200 as libc::c_int
                                                            && contrange == 0 as libc::c_int as libc::c_long
                                                            && contlen >= 0 as libc::c_int as libc::c_long
                                                            && (*hs).restval >= contlen
                                                    {
                                                        logputs(
                                                            LOG_VERBOSE,
                                                            dcgettext(
                                                                0 as *const libc::c_char,
                                                                b"\n    The file is already fully retrieved; nothing to do.\n\n\0"
                                                                    as *const u8 as *const libc::c_char,
                                                                5 as libc::c_int,
                                                            ),
                                                        );
                                                        (*hs).len = contlen;
                                                        (*hs).res = 0 as libc::c_int;
                                                        *dt |= RETROKF as libc::c_int;
                                                        if keep_alive as libc::c_int != 0
                                                            && skip_short_body(sock, contlen, chunked_transfer_encoding)
                                                                as libc::c_int != 0
                                                        {
                                                            if !keep_alive {
                                                                if pconn_active as libc::c_int != 0 && sock == pconn.socket
                                                                {
                                                                    invalidate_persistent();
                                                                } else {
                                                                    fd_close(sock);
                                                                }
                                                                sock = -(1 as libc::c_int);
                                                            }
                                                        } else {
                                                            if pconn_active as libc::c_int != 0 && sock == pconn.socket
                                                            {
                                                                invalidate_persistent();
                                                            } else {
                                                                fd_close(sock);
                                                            }
                                                            sock = -(1 as libc::c_int);
                                                        }
                                                        retval = RETRUNNEEDED;
                                                    } else if contrange != 0 as libc::c_int as libc::c_long
                                                        && contrange != (*hs).restval
                                                        || statcode == 206 as libc::c_int && contrange == 0
                                                            && (*hs).restval != 0
                                                    {
                                                        if pconn_active as libc::c_int != 0 && sock == pconn.socket
                                                        {
                                                            invalidate_persistent();
                                                        } else {
                                                            fd_close(sock);
                                                        }
                                                        sock = -(1 as libc::c_int);
                                                        retval = RANGEERR;
                                                    } else {
                                                        if contlen == -(1 as libc::c_int) as libc::c_long {
                                                            (*hs).contlen = -(1 as libc::c_int) as wgint;
                                                        } else if (*hs).remote_encoding as libc::c_int
                                                            == ENC_GZIP as libc::c_int
                                                        {
                                                            (*hs).contlen = -(1 as libc::c_int) as wgint;
                                                        } else {
                                                            (*hs).contlen = contlen + contrange;
                                                        }
                                                        if opt.verbose != 0 {
                                                            if *dt & RETROKF as libc::c_int != 0 {
                                                                logputs(
                                                                    LOG_VERBOSE,
                                                                    dcgettext(
                                                                        0 as *const libc::c_char,
                                                                        b"Length: \0" as *const u8 as *const libc::c_char,
                                                                        5 as libc::c_int,
                                                                    ),
                                                                );
                                                                if contlen != -(1 as libc::c_int) as libc::c_long {
                                                                    logputs(
                                                                        LOG_VERBOSE,
                                                                        number_to_static_string(contlen + contrange),
                                                                    );
                                                                    if contlen + contrange
                                                                        >= 1024 as libc::c_int as libc::c_long
                                                                    {
                                                                        logprintf(
                                                                            LOG_VERBOSE,
                                                                            b" (%s)\0" as *const u8 as *const libc::c_char,
                                                                            human_readable(
                                                                                contlen + contrange,
                                                                                10 as libc::c_int,
                                                                                1 as libc::c_int,
                                                                            ),
                                                                        );
                                                                    }
                                                                    if contrange != 0 {
                                                                        if contlen >= 1024 as libc::c_int as libc::c_long {
                                                                            logprintf(
                                                                                LOG_VERBOSE,
                                                                                dcgettext(
                                                                                    0 as *const libc::c_char,
                                                                                    b", %s (%s) remaining\0" as *const u8
                                                                                        as *const libc::c_char,
                                                                                    5 as libc::c_int,
                                                                                ),
                                                                                number_to_static_string(contlen),
                                                                                human_readable(contlen, 10 as libc::c_int, 1 as libc::c_int),
                                                                            );
                                                                        } else {
                                                                            logprintf(
                                                                                LOG_VERBOSE,
                                                                                dcgettext(
                                                                                    0 as *const libc::c_char,
                                                                                    b", %s remaining\0" as *const u8 as *const libc::c_char,
                                                                                    5 as libc::c_int,
                                                                                ),
                                                                                number_to_static_string(contlen),
                                                                            );
                                                                        }
                                                                    }
                                                                } else {
                                                                    logputs(
                                                                        LOG_VERBOSE,
                                                                        if opt.ignore_length as libc::c_int != 0 {
                                                                            dcgettext(
                                                                                0 as *const libc::c_char,
                                                                                b"ignored\0" as *const u8 as *const libc::c_char,
                                                                                5 as libc::c_int,
                                                                            )
                                                                        } else {
                                                                            dcgettext(
                                                                                0 as *const libc::c_char,
                                                                                b"unspecified\0" as *const u8 as *const libc::c_char,
                                                                                5 as libc::c_int,
                                                                            )
                                                                        },
                                                                    );
                                                                }
                                                                if !type_0.is_null() {
                                                                    logprintf(
                                                                        LOG_VERBOSE,
                                                                        b" [%s]\n\0" as *const u8 as *const libc::c_char,
                                                                        quotearg_style(escape_quoting_style, type_0),
                                                                    );
                                                                } else {
                                                                    logputs(
                                                                        LOG_VERBOSE,
                                                                        b"\n\0" as *const u8 as *const libc::c_char,
                                                                    );
                                                                }
                                                            }
                                                        }
                                                        if *dt & RETROKF as libc::c_int == 0
                                                            && !opt.content_on_error || head_only as libc::c_int != 0
                                                            || opt.spider as libc::c_int != 0 && !opt.recursive
                                                        {
                                                            (*hs).len = 0 as libc::c_int as wgint;
                                                            (*hs).res = 0 as libc::c_int;
                                                            (*hs).restval = 0 as libc::c_int as wgint;
                                                            if warc_enabled {
                                                                let mut _err_1: libc::c_int = read_response_body(
                                                                    hs,
                                                                    sock,
                                                                    0 as *mut FILE,
                                                                    contlen,
                                                                    0 as libc::c_int as wgint,
                                                                    chunked_transfer_encoding,
                                                                    (*u).url,
                                                                    warc_timestamp_str.as_mut_ptr(),
                                                                    warc_request_uuid.as_mut_ptr(),
                                                                    warc_ip,
                                                                    type_0,
                                                                    statcode,
                                                                    head,
                                                                );
                                                                if _err_1 != RETRFINISHED as libc::c_int
                                                                    || (*hs).res < 0 as libc::c_int
                                                                {
                                                                    if pconn_active as libc::c_int != 0 && sock == pconn.socket
                                                                    {
                                                                        invalidate_persistent();
                                                                    } else {
                                                                        fd_close(sock);
                                                                    }
                                                                    sock = -(1 as libc::c_int);
                                                                    retval = _err_1 as uerr_t;
                                                                    current_block = 14092045471819376021;
                                                                } else {
                                                                    if !keep_alive {
                                                                        if pconn_active as libc::c_int != 0 && sock == pconn.socket
                                                                        {
                                                                            invalidate_persistent();
                                                                        } else {
                                                                            fd_close(sock);
                                                                        }
                                                                        sock = -(1 as libc::c_int);
                                                                    }
                                                                    current_block = 1606036238936509124;
                                                                }
                                                            } else {
                                                                if head_only {
                                                                    if !keep_alive {
                                                                        if pconn_active as libc::c_int != 0 && sock == pconn.socket
                                                                        {
                                                                            invalidate_persistent();
                                                                        } else {
                                                                            fd_close(sock);
                                                                        }
                                                                        sock = -(1 as libc::c_int);
                                                                    }
                                                                } else if opt.spider as libc::c_int != 0 && !opt.recursive {
                                                                    if pconn_active as libc::c_int != 0 && sock == pconn.socket
                                                                    {
                                                                        invalidate_persistent();
                                                                    } else {
                                                                        fd_close(sock);
                                                                    }
                                                                    sock = -(1 as libc::c_int);
                                                                } else if keep_alive as libc::c_int != 0
                                                                    && skip_short_body(sock, contlen, chunked_transfer_encoding)
                                                                        as libc::c_int != 0
                                                                {
                                                                    if !keep_alive {
                                                                        if pconn_active as libc::c_int != 0 && sock == pconn.socket
                                                                        {
                                                                            invalidate_persistent();
                                                                        } else {
                                                                            fd_close(sock);
                                                                        }
                                                                        sock = -(1 as libc::c_int);
                                                                    }
                                                                } else {
                                                                    if pconn_active as libc::c_int != 0 && sock == pconn.socket
                                                                    {
                                                                        invalidate_persistent();
                                                                    } else {
                                                                        fd_close(sock);
                                                                    }
                                                                    sock = -(1 as libc::c_int);
                                                                }
                                                                current_block = 1606036238936509124;
                                                            }
                                                            match current_block {
                                                                14092045471819376021 => {}
                                                                _ => {
                                                                    if statcode == 504 as libc::c_int {
                                                                        retval = GATEWAYTIMEOUT;
                                                                    } else {
                                                                        retval = RETRFINISHED;
                                                                    }
                                                                }
                                                            }
                                                        } else {
                                                            err = open_output_stream(hs, count, &mut fp) as libc::c_int;
                                                            if err != RETROK as libc::c_int {
                                                                if pconn_active as libc::c_int != 0 && sock == pconn.socket
                                                                {
                                                                    invalidate_persistent();
                                                                } else {
                                                                    fd_close(sock);
                                                                }
                                                                sock = -(1 as libc::c_int);
                                                                retval = err as uerr_t;
                                                            } else {
                                                                if opt.enable_xattr {
                                                                    if original_url != u as *mut url {
                                                                        set_file_metadata(u, original_url, fp);
                                                                    } else {
                                                                        set_file_metadata(u, 0 as *const url, fp);
                                                                    }
                                                                }
                                                                err = read_response_body(
                                                                    hs,
                                                                    sock,
                                                                    fp,
                                                                    contlen,
                                                                    contrange,
                                                                    chunked_transfer_encoding,
                                                                    (*u).url,
                                                                    warc_timestamp_str.as_mut_ptr(),
                                                                    warc_request_uuid.as_mut_ptr(),
                                                                    warc_ip,
                                                                    type_0,
                                                                    statcode,
                                                                    head,
                                                                );
                                                                if (*hs).res >= 0 as libc::c_int {
                                                                    if !keep_alive {
                                                                        if pconn_active as libc::c_int != 0 && sock == pconn.socket
                                                                        {
                                                                            invalidate_persistent();
                                                                        } else {
                                                                            fd_close(sock);
                                                                        }
                                                                        sock = -(1 as libc::c_int);
                                                                    }
                                                                } else {
                                                                    if pconn_active as libc::c_int != 0 && sock == pconn.socket
                                                                    {
                                                                        invalidate_persistent();
                                                                    } else {
                                                                        fd_close(sock);
                                                                    }
                                                                    sock = -(1 as libc::c_int);
                                                                }
                                                                if output_stream.is_null() {
                                                                    fclose(fp);
                                                                }
                                                                retval = err as uerr_t;
                                                            }
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
    rpl_free(head as *mut libc::c_void);
    head = 0 as *mut libc::c_char;
    rpl_free(type_0 as *mut libc::c_void);
    type_0 = 0 as *mut libc::c_char;
    rpl_free(message as *mut libc::c_void);
    message = 0 as *mut libc::c_char;
    resp_free(&mut resp);
    request_free(&mut req);
    return retval;
}
unsafe extern "C" fn check_retry_on_http_error(statcode: libc::c_int) -> bool {
    let mut tok: *const libc::c_char = opt.retry_on_http_error;
    while !tok.is_null() && *tok as libc::c_int != 0 {
        if atoi(tok) == statcode {
            return 1 as libc::c_int != 0;
        }
        tok = strchr(tok, ',' as i32);
        if !tok.is_null() {
            tok = tok.offset(1);
            tok;
        }
    }
    return 0 as libc::c_int != 0;
}
pub unsafe extern "C" fn http_loop(
    mut u: *const url,
    mut original_url: *mut url,
    mut newloc: *mut *mut libc::c_char,
    mut local_file: *mut *mut libc::c_char,
    mut referer: *const libc::c_char,
    mut dt: *mut libc::c_int,
    mut proxy: *mut url,
    mut iri: *mut iri,
) -> uerr_t {
    let mut current_block: u64;
    let mut count: libc::c_int = 0;
    let mut got_head: bool = 0 as libc::c_int != 0;
    let mut time_came_from_head: bool = 0 as libc::c_int != 0;
    let mut got_name: bool = 0 as libc::c_int != 0;
    let mut tms: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut tmrate: *const libc::c_char = 0 as *const libc::c_char;
    let mut err: uerr_t = NOCONERROR;
    let mut ret: uerr_t = TRYLIMEXC;
    let mut tmr: time_t = -(1 as libc::c_int) as time_t;
    let mut hstat: http_stat = http_stat {
        len: 0,
        contlen: 0,
        restval: 0,
        res: 0,
        rderrmsg: 0 as *mut libc::c_char,
        newloc: 0 as *mut libc::c_char,
        remote_time: 0 as *mut libc::c_char,
        error: 0 as *mut libc::c_char,
        statcode: 0,
        message: 0 as *mut libc::c_char,
        rd_size: 0,
        dltime: 0.,
        referer: 0 as *const libc::c_char,
        local_file: 0 as *mut libc::c_char,
        existence_checked: false,
        timestamp_checked: false,
        orig_file_name: 0 as *mut libc::c_char,
        orig_file_size: 0,
        orig_file_tstamp: 0,
        local_encoding: ENC_NONE,
        remote_encoding: ENC_NONE,
        temporary: false,
    };
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
    let mut send_head_first: bool = 1 as libc::c_int != 0;
    let mut force_full_retrieve: bool = 0 as libc::c_int != 0;
    if !(opt.warc_filename).is_null() {
        force_full_retrieve = 1 as libc::c_int != 0;
    }
    if !local_file.is_null() && !(opt.output_document).is_null() {
        *local_file = if *opt.output_document as libc::c_int == '-' as i32
            && *(opt.output_document).offset(1 as libc::c_int as isize) == 0
        {
            0 as *mut libc::c_char
        } else {
            xstrdup(opt.output_document)
        };
    }
    *newloc = 0 as *mut libc::c_char;
    if opt.cookies {
        load_cookies();
    }
    if opt.ftp_glob as libc::c_int != 0 && has_wildcards_p((*u).path) as libc::c_int != 0
    {
        logputs(
            LOG_VERBOSE,
            dcgettext(
                0 as *const libc::c_char,
                b"Warning: wildcards not supported in HTTP.\n\0" as *const u8
                    as *const libc::c_char,
                5 as libc::c_int,
            ),
        );
    }
    memset(
        &mut hstat as *mut http_stat as *mut libc::c_void,
        '\0' as i32,
        ::std::mem::size_of::<http_stat>() as libc::c_ulong,
    );
    hstat.referer = referer;
    if !(opt.output_document).is_null() {
        hstat.local_file = xstrdup(opt.output_document);
        got_name = 1 as libc::c_int != 0;
    } else if !opt.content_disposition {
        hstat
            .local_file = url_file_name(
            if opt.trustservernames as libc::c_int != 0 {
                u
            } else {
                original_url as *const url
            },
            0 as *mut libc::c_char,
        );
        got_name = 1 as libc::c_int != 0;
    }
    if got_name as libc::c_int != 0
        && file_exists_p(hstat.local_file, 0 as *mut file_stats_t) as libc::c_int != 0
        && opt.noclobber as libc::c_int != 0 && (opt.output_document).is_null()
    {
        get_file_flags(hstat.local_file, dt);
        ret = RETROK;
    } else {
        count = 0 as libc::c_int;
        *dt = 0 as libc::c_int;
        if !opt.spider {
            send_head_first = 0 as libc::c_int != 0;
        }
        if opt.content_disposition as libc::c_int != 0
            && opt.always_rest as libc::c_int != 0
        {
            send_head_first = 1 as libc::c_int != 0;
        }
        if opt.timestamping {
            if opt.if_modified_since as libc::c_int != 0 && !send_head_first
                && got_name as libc::c_int != 0
                && file_exists_p(hstat.local_file, 0 as *mut file_stats_t) as libc::c_int
                    != 0
            {
                *dt |= IF_MODIFIED_SINCE as libc::c_int;
                let mut timestamp_err: uerr_t = set_file_timestamp(&mut hstat);
                if timestamp_err as libc::c_uint != RETROK as libc::c_int as libc::c_uint
                {
                    return timestamp_err;
                }
            } else if opt.content_disposition as libc::c_int != 0
                || file_exists_p(hstat.local_file, 0 as *mut file_stats_t) as libc::c_int
                    != 0
            {
                send_head_first = 1 as libc::c_int != 0;
            }
        }
        loop {
            count += 1;
            count;
            sleep_between_retrievals(count);
            tms = datetime_str(time(0 as *mut time_t));
            if opt.spider as libc::c_int != 0 && !got_head {
                logprintf(
                    LOG_VERBOSE,
                    dcgettext(
                        0 as *const libc::c_char,
                        b"Spider mode enabled. Check if remote file exists.\n\0"
                            as *const u8 as *const libc::c_char,
                        5 as libc::c_int,
                    ),
                );
            }
            if opt.verbose != 0 {
                let mut hurl: *mut libc::c_char = url_string(u, URL_AUTH_HIDE_PASSWD);
                if count > 1 as libc::c_int {
                    let mut tmp: [libc::c_char; 256] = [0; 256];
                    sprintf(
                        tmp.as_mut_ptr(),
                        dcgettext(
                            0 as *const libc::c_char,
                            b"(try:%2d)\0" as *const u8 as *const libc::c_char,
                            5 as libc::c_int,
                        ),
                        count,
                    );
                    logprintf(
                        LOG_NOTQUIET,
                        b"--%s--  %s  %s\n\0" as *const u8 as *const libc::c_char,
                        tms,
                        tmp.as_mut_ptr(),
                        hurl,
                    );
                } else {
                    logprintf(
                        LOG_NOTQUIET,
                        b"--%s--  %s\n\0" as *const u8 as *const libc::c_char,
                        tms,
                        hurl,
                    );
                }
                rpl_free(hurl as *mut libc::c_void);
                hurl = 0 as *mut libc::c_char;
            }
            if send_head_first as libc::c_int != 0 && !got_head {
                *dt |= HEAD_ONLY as libc::c_int;
            } else {
                *dt &= !(HEAD_ONLY as libc::c_int);
            }
            if force_full_retrieve {
                hstat.restval = hstat.len;
            } else if opt.start_pos >= 0 as libc::c_int as libc::c_long {
                hstat.restval = opt.start_pos;
            } else if opt.always_rest as libc::c_int != 0 && got_name as libc::c_int != 0
                && stat(hstat.local_file, &mut st) == 0 as libc::c_int
                && st.st_mode & 0o170000 as libc::c_int as libc::c_uint
                    == 0o100000 as libc::c_int as libc::c_uint
            {
                hstat.restval = st.st_size;
            } else if count > 1 as libc::c_int {
                if hstat.len < hstat.restval {
                    hstat.restval -= hstat.len;
                } else {
                    hstat.restval = hstat.len;
                }
            } else {
                hstat.restval = 0 as libc::c_int as wgint;
            }
            if !proxy.is_null() && count > 1 as libc::c_int || !opt.allow_cache {
                *dt |= SEND_NOCACHE as libc::c_int;
            } else {
                *dt &= !(SEND_NOCACHE as libc::c_int);
            }
            err = gethttp(u, original_url, &mut hstat, dt, proxy, iri, count);
            tms = datetime_str(time(0 as *mut time_t));
            if !(hstat.newloc).is_null() {
                *newloc = xstrdup(hstat.newloc);
            }
            match err as libc::c_uint {
                24 | 22 | 2 | 3 | 36 | 44 | 39 | 20 | 23 => {
                    printwhat(count, opt.ntry);
                }
                21 | 19 => {
                    logputs(LOG_VERBOSE, b"\n\0" as *const u8 as *const libc::c_char);
                    logprintf(
                        LOG_NOTQUIET,
                        dcgettext(
                            0 as *const libc::c_char,
                            b"Cannot write to %s (%s).\n\0" as *const u8
                                as *const libc::c_char,
                            5 as libc::c_int,
                        ),
                        quote(hstat.local_file),
                        strerror(*__errno_location()),
                    );
                    ret = err;
                    break;
                }
                1 => {
                    if opt.retry_on_host_error {
                        printwhat(count, opt.ntry);
                    } else {
                        ret = err;
                        break;
                    }
                }
                5 | 41 | 45 | 33 | 46 | 38 | 51 => {
                    ret = err;
                    break;
                }
                50 => {
                    logputs(LOG_VERBOSE, b"\n\0" as *const u8 as *const libc::c_char);
                    logprintf(
                        LOG_NOTQUIET,
                        dcgettext(
                            0 as *const libc::c_char,
                            b"Required attribute missing from Header received.\n\0"
                                as *const u8 as *const libc::c_char,
                            5 as libc::c_int,
                        ),
                    );
                    ret = err;
                    break;
                }
                42 => {
                    logputs(LOG_VERBOSE, b"\n\0" as *const u8 as *const libc::c_char);
                    logprintf(
                        LOG_NOTQUIET,
                        dcgettext(
                            0 as *const libc::c_char,
                            b"Username/Password Authentication Failed.\n\0" as *const u8
                                as *const libc::c_char,
                            5 as libc::c_int,
                        ),
                    );
                    ret = err;
                    break;
                }
                52 => {
                    logputs(LOG_VERBOSE, b"\n\0" as *const u8 as *const libc::c_char);
                    logprintf(
                        LOG_NOTQUIET,
                        dcgettext(
                            0 as *const libc::c_char,
                            b"Cannot write to WARC file.\n\0" as *const u8
                                as *const libc::c_char,
                            5 as libc::c_int,
                        ),
                    );
                    ret = err;
                    break;
                }
                53 | 54 => {
                    logputs(LOG_VERBOSE, b"\n\0" as *const u8 as *const libc::c_char);
                    logprintf(
                        LOG_NOTQUIET,
                        dcgettext(
                            0 as *const libc::c_char,
                            b"Cannot write to temporary WARC file.\n\0" as *const u8
                                as *const libc::c_char,
                            5 as libc::c_int,
                        ),
                    );
                    ret = err;
                    break;
                }
                4 => {
                    logprintf(
                        LOG_NOTQUIET,
                        dcgettext(
                            0 as *const libc::c_char,
                            b"Unable to establish SSL connection.\n\0" as *const u8
                                as *const libc::c_char,
                            5 as libc::c_int,
                        ),
                    );
                    ret = err;
                    break;
                }
                47 => {
                    logputs(LOG_VERBOSE, b"\n\0" as *const u8 as *const libc::c_char);
                    logprintf(
                        LOG_NOTQUIET,
                        dcgettext(
                            0 as *const libc::c_char,
                            b"Cannot unlink %s (%s).\n\0" as *const u8
                                as *const libc::c_char,
                            5 as libc::c_int,
                        ),
                        quote(hstat.local_file),
                        strerror(*__errno_location()),
                    );
                    ret = err;
                    break;
                }
                6 | 48 => {
                    if (*newloc).is_null() {
                        logprintf(
                            LOG_NOTQUIET,
                            dcgettext(
                                0 as *const libc::c_char,
                                b"ERROR: Redirection (%d) without location.\n\0"
                                    as *const u8 as *const libc::c_char,
                                5 as libc::c_int,
                            ),
                            hstat.statcode,
                        );
                        ret = WRONGCODE;
                    } else {
                        ret = err;
                    }
                    break;
                }
                34 => {
                    ret = RETROK;
                    break;
                }
                35 => {
                    if *dt & RETROKF as libc::c_int == 0 {
                        let mut hurl_0: *mut libc::c_char = 0 as *mut libc::c_char;
                        if opt.verbose == 0 {
                            hurl_0 = url_string(u, URL_AUTH_HIDE_PASSWD);
                            logprintf(
                                LOG_NONVERBOSE,
                                b"%s:\n\0" as *const u8 as *const libc::c_char,
                                hurl_0,
                            );
                        }
                        if *dt & HEAD_ONLY as libc::c_int != 0
                            && (hstat.statcode == 500 as libc::c_int
                                || hstat.statcode == 501 as libc::c_int)
                        {
                            got_head = 1 as libc::c_int != 0;
                            rpl_free(hurl_0 as *mut libc::c_void);
                            hurl_0 = 0 as *mut libc::c_char;
                        } else {
                            if opt.spider as libc::c_int != 0 && !(*iri).utf8_encode {
                                if hurl_0.is_null() {
                                    hurl_0 = url_string(u, URL_AUTH_HIDE_PASSWD);
                                }
                                nonexisting_url(hurl_0);
                                logprintf(
                                    LOG_NOTQUIET,
                                    dcgettext(
                                        0 as *const libc::c_char,
                                        b"Remote file does not exist -- broken link!!!\n\0"
                                            as *const u8 as *const libc::c_char,
                                        5 as libc::c_int,
                                    ),
                                );
                                current_block = 6988365858197790817;
                            } else if check_retry_on_http_error(hstat.statcode) {
                                printwhat(count, opt.ntry);
                                rpl_free(hurl_0 as *mut libc::c_void);
                                hurl_0 = 0 as *mut libc::c_char;
                                current_block = 7828949454673616476;
                            } else {
                                logprintf(
                                    LOG_NOTQUIET,
                                    dcgettext(
                                        0 as *const libc::c_char,
                                        b"%s ERROR %d: %s.\n\0" as *const u8 as *const libc::c_char,
                                        5 as libc::c_int,
                                    ),
                                    tms,
                                    hstat.statcode,
                                    quotearg_style(escape_quoting_style, hstat.error),
                                );
                                current_block = 6988365858197790817;
                            }
                            match current_block {
                                7828949454673616476 => {}
                                _ => {
                                    logputs(
                                        LOG_VERBOSE,
                                        b"\n\0" as *const u8 as *const libc::c_char,
                                    );
                                    ret = WRONGCODE;
                                    rpl_free(hurl_0 as *mut libc::c_void);
                                    hurl_0 = 0 as *mut libc::c_char;
                                    break;
                                }
                            }
                        }
                    } else {
                        if !got_head || opt.spider as libc::c_int != 0 && !opt.recursive
                        {
                            got_head = 1 as libc::c_int != 0;
                            if opt.timestamping as libc::c_int != 0
                                && (hstat.remote_time).is_null()
                            {
                                logputs(
                                    LOG_NOTQUIET,
                                    dcgettext(
                                        0 as *const libc::c_char,
                                        b"Last-modified header missing -- time-stamps turned off.\n\0"
                                            as *const u8 as *const libc::c_char,
                                        5 as libc::c_int,
                                    ),
                                );
                            } else if !(hstat.remote_time).is_null() {
                                tmr = http_atotm(hstat.remote_time);
                                if tmr == -(1 as libc::c_int) as time_t {
                                    logputs(
                                        LOG_VERBOSE,
                                        dcgettext(
                                            0 as *const libc::c_char,
                                            b"Last-modified header invalid -- time-stamp ignored.\n\0"
                                                as *const u8 as *const libc::c_char,
                                            5 as libc::c_int,
                                        ),
                                    );
                                }
                                if *dt & HEAD_ONLY as libc::c_int != 0 {
                                    time_came_from_head = 1 as libc::c_int != 0;
                                }
                            }
                            if send_head_first {
                                if opt.timestamping {
                                    if !(hstat.orig_file_name).is_null() {
                                        if !(hstat.remote_time).is_null()
                                            && tmr != -(1 as libc::c_int) as time_t
                                        {
                                            if hstat.orig_file_tstamp >= tmr {
                                                if hstat.contlen == -(1 as libc::c_int) as libc::c_long
                                                    || hstat.orig_file_size == hstat.contlen
                                                {
                                                    logprintf(
                                                        LOG_VERBOSE,
                                                        dcgettext(
                                                            0 as *const libc::c_char,
                                                            b"Server file no newer than local file %s -- not retrieving.\n\n\0"
                                                                as *const u8 as *const libc::c_char,
                                                            5 as libc::c_int,
                                                        ),
                                                        quote(hstat.orig_file_name),
                                                    );
                                                    ret = RETROK;
                                                    break;
                                                } else {
                                                    logprintf(
                                                        LOG_VERBOSE,
                                                        dcgettext(
                                                            0 as *const libc::c_char,
                                                            b"The sizes do not match (local %s) -- retrieving.\n\0"
                                                                as *const u8 as *const libc::c_char,
                                                            5 as libc::c_int,
                                                        ),
                                                        number_to_static_string(hstat.orig_file_size),
                                                    );
                                                }
                                            } else {
                                                force_full_retrieve = 1 as libc::c_int != 0;
                                                logputs(
                                                    LOG_VERBOSE,
                                                    dcgettext(
                                                        0 as *const libc::c_char,
                                                        b"Remote file is newer, retrieving.\n\0" as *const u8
                                                            as *const libc::c_char,
                                                        5 as libc::c_int,
                                                    ),
                                                );
                                            }
                                            logputs(
                                                LOG_VERBOSE,
                                                b"\n\0" as *const u8 as *const libc::c_char,
                                            );
                                        }
                                    }
                                    hstat.timestamp_checked = 1 as libc::c_int != 0;
                                }
                                if opt.spider {
                                    let mut finished: bool = 1 as libc::c_int != 0;
                                    if opt.recursive {
                                        if *dt & TEXTHTML as libc::c_int != 0
                                            || *dt & TEXTCSS as libc::c_int != 0
                                        {
                                            logputs(
                                                LOG_VERBOSE,
                                                dcgettext(
                                                    0 as *const libc::c_char,
                                                    b"Remote file exists and could contain links to other resources -- retrieving.\n\n\0"
                                                        as *const u8 as *const libc::c_char,
                                                    5 as libc::c_int,
                                                ),
                                            );
                                            finished = 0 as libc::c_int != 0;
                                        } else {
                                            logprintf(
                                                LOG_VERBOSE,
                                                dcgettext(
                                                    0 as *const libc::c_char,
                                                    b"Remote file exists but does not contain any link -- not retrieving.\n\n\0"
                                                        as *const u8 as *const libc::c_char,
                                                    5 as libc::c_int,
                                                ),
                                            );
                                            ret = RETROK;
                                        }
                                    } else {
                                        if *dt & TEXTHTML as libc::c_int != 0
                                            || *dt & TEXTCSS as libc::c_int != 0
                                        {
                                            logprintf(
                                                LOG_VERBOSE,
                                                dcgettext(
                                                    0 as *const libc::c_char,
                                                    b"Remote file exists and could contain further links,\nbut recursion is disabled -- not retrieving.\n\n\0"
                                                        as *const u8 as *const libc::c_char,
                                                    5 as libc::c_int,
                                                ),
                                            );
                                        } else {
                                            logprintf(
                                                LOG_VERBOSE,
                                                dcgettext(
                                                    0 as *const libc::c_char,
                                                    b"Remote file exists.\n\n\0" as *const u8
                                                        as *const libc::c_char,
                                                    5 as libc::c_int,
                                                ),
                                            );
                                        }
                                        ret = RETROK;
                                    }
                                    if finished {
                                        logprintf(
                                            LOG_NONVERBOSE,
                                            dcgettext(
                                                0 as *const libc::c_char,
                                                b"%s URL: %s %2d %s\n\0" as *const u8
                                                    as *const libc::c_char,
                                                5 as libc::c_int,
                                            ),
                                            tms,
                                            (*u).url,
                                            hstat.statcode,
                                            if !(hstat.message).is_null() {
                                                quotearg_style(escape_quoting_style, hstat.message)
                                                    as *const libc::c_char
                                            } else {
                                                b"\0" as *const u8 as *const libc::c_char
                                            },
                                        );
                                        break;
                                    }
                                }
                                got_name = 1 as libc::c_int != 0;
                                *dt &= !(HEAD_ONLY as libc::c_int);
                                count = 0 as libc::c_int;
                                current_block = 7828949454673616476;
                            } else {
                                current_block = 14698008245370361992;
                            }
                        } else {
                            current_block = 14698008245370361992;
                        }
                        match current_block {
                            7828949454673616476 => {}
                            _ => {
                                if opt.useservertimestamps as libc::c_int != 0
                                    && tmr != -(1 as libc::c_int) as time_t
                                    && (hstat.len == hstat.contlen
                                        || hstat.res == 0 as libc::c_int
                                            && hstat.contlen == -(1 as libc::c_int) as libc::c_long)
                                {
                                    let mut fl: *const libc::c_char = 0 as *const libc::c_char;
                                    set_local_file(&mut fl, hstat.local_file);
                                    if !fl.is_null() {
                                        let mut newtmr: time_t = -(1 as libc::c_int) as time_t;
                                        if time_came_from_head as libc::c_int != 0
                                            && !(hstat.remote_time).is_null()
                                            && *(hstat.remote_time).offset(0 as libc::c_int as isize)
                                                as libc::c_int != 0
                                        {
                                            newtmr = http_atotm(hstat.remote_time);
                                            if newtmr != -(1 as libc::c_int) as time_t {
                                                tmr = newtmr;
                                            }
                                        }
                                        touch(fl, tmr);
                                    }
                                }
                                tmrate = retr_rate(hstat.rd_size, hstat.dltime);
                                total_download_time += hstat.dltime;
                                if hstat.len == hstat.contlen {
                                    if *dt & RETROKF as libc::c_int != 0
                                        || opt.content_on_error as libc::c_int != 0
                                    {
                                        let mut write_to_stdout: bool = !(opt.output_document)
                                            .is_null()
                                            && (*opt.output_document as libc::c_int == '-' as i32
                                                && *(opt.output_document).offset(1 as libc::c_int as isize)
                                                    == 0);
                                        logprintf(
                                            LOG_VERBOSE,
                                            if write_to_stdout as libc::c_int != 0 {
                                                dcgettext(
                                                    0 as *const libc::c_char,
                                                    b"%s (%s) - written to stdout %s[%s/%s]\n\n\0" as *const u8
                                                        as *const libc::c_char,
                                                    5 as libc::c_int,
                                                )
                                            } else {
                                                dcgettext(
                                                    0 as *const libc::c_char,
                                                    b"%s (%s) - %s saved [%s/%s]\n\n\0" as *const u8
                                                        as *const libc::c_char,
                                                    5 as libc::c_int,
                                                )
                                            },
                                            tms,
                                            tmrate,
                                            if write_to_stdout as libc::c_int != 0 {
                                                b"\0" as *const u8 as *const libc::c_char
                                            } else {
                                                quote(hstat.local_file)
                                            },
                                            number_to_static_string(hstat.len),
                                            number_to_static_string(hstat.contlen),
                                        );
                                        logprintf(
                                            LOG_NONVERBOSE,
                                            b"%s URL:%s [%s/%s] -> \"%s\" [%d]\n\0" as *const u8
                                                as *const libc::c_char,
                                            tms,
                                            (*u).url,
                                            number_to_static_string(hstat.len),
                                            number_to_static_string(hstat.contlen),
                                            hstat.local_file,
                                            count,
                                        );
                                    }
                                    numurls += 1;
                                    numurls;
                                    total_downloaded_bytes += hstat.rd_size;
                                    if *dt & ADDED_HTML_EXTENSION as libc::c_int != 0 {
                                        downloaded_file(
                                            FILE_DOWNLOADED_AND_HTML_EXTENSION_ADDED,
                                            hstat.local_file,
                                        );
                                    } else {
                                        downloaded_file(FILE_DOWNLOADED_NORMALLY, hstat.local_file);
                                    }
                                    ret = RETROK;
                                    break;
                                } else if hstat.res == 0 as libc::c_int {
                                    if hstat.contlen == -(1 as libc::c_int) as libc::c_long {
                                        if *dt & RETROKF as libc::c_int != 0
                                            || opt.content_on_error as libc::c_int != 0
                                        {
                                            let mut write_to_stdout_0: bool = !(opt.output_document)
                                                .is_null()
                                                && (*opt.output_document as libc::c_int == '-' as i32
                                                    && *(opt.output_document).offset(1 as libc::c_int as isize)
                                                        == 0);
                                            logprintf(
                                                LOG_VERBOSE,
                                                if write_to_stdout_0 as libc::c_int != 0 {
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
                                                if write_to_stdout_0 as libc::c_int != 0 {
                                                    b"\0" as *const u8 as *const libc::c_char
                                                } else {
                                                    quote(hstat.local_file)
                                                },
                                                number_to_static_string(hstat.len),
                                            );
                                            if !(opt.verbose != 0 || opt.quiet as libc::c_int != 0) {
                                                let mut url: *mut libc::c_char = url_string(
                                                    u,
                                                    URL_AUTH_HIDE_PASSWD,
                                                );
                                                logprintf(
                                                    LOG_NONVERBOSE,
                                                    b"%s URL:%s [%s] -> \"%s\" [%d]\n\0" as *const u8
                                                        as *const libc::c_char,
                                                    tms,
                                                    url,
                                                    number_to_static_string(hstat.len),
                                                    hstat.local_file,
                                                    count,
                                                );
                                                rpl_free(url as *mut libc::c_void);
                                                url = 0 as *mut libc::c_char;
                                            }
                                        }
                                        numurls += 1;
                                        numurls;
                                        total_downloaded_bytes += hstat.rd_size;
                                        if *dt & ADDED_HTML_EXTENSION as libc::c_int != 0 {
                                            downloaded_file(
                                                FILE_DOWNLOADED_AND_HTML_EXTENSION_ADDED,
                                                hstat.local_file,
                                            );
                                        } else {
                                            downloaded_file(FILE_DOWNLOADED_NORMALLY, hstat.local_file);
                                        }
                                        ret = RETROK;
                                        break;
                                    } else if hstat.len < hstat.contlen {
                                        logprintf(
                                            LOG_VERBOSE,
                                            dcgettext(
                                                0 as *const libc::c_char,
                                                b"%s (%s) - Connection closed at byte %s. \0" as *const u8
                                                    as *const libc::c_char,
                                                5 as libc::c_int,
                                            ),
                                            tms,
                                            tmrate,
                                            number_to_static_string(hstat.len),
                                        );
                                        printwhat(count, opt.ntry);
                                    } else if hstat.len != hstat.restval {
                                        abort();
                                    } else {
                                        ret = RETROK;
                                        break;
                                    }
                                } else if hstat.contlen
                                    == -(1 as libc::c_int) as libc::c_long
                                {
                                    logprintf(
                                        LOG_VERBOSE,
                                        dcgettext(
                                            0 as *const libc::c_char,
                                            b"%s (%s) - Read error at byte %s (%s).\0" as *const u8
                                                as *const libc::c_char,
                                            5 as libc::c_int,
                                        ),
                                        tms,
                                        tmrate,
                                        number_to_static_string(hstat.len),
                                        hstat.rderrmsg,
                                    );
                                    printwhat(count, opt.ntry);
                                } else {
                                    logprintf(
                                        LOG_VERBOSE,
                                        dcgettext(
                                            0 as *const libc::c_char,
                                            b"%s (%s) - Read error at byte %s/%s (%s). \0" as *const u8
                                                as *const libc::c_char,
                                            5 as libc::c_int,
                                        ),
                                        tms,
                                        tmrate,
                                        number_to_static_string(hstat.len),
                                        number_to_static_string(hstat.contlen),
                                        hstat.rderrmsg,
                                    );
                                    printwhat(count, opt.ntry);
                                }
                            }
                        }
                    }
                }
                _ => {
                    abort();
                }
            }
            if !(opt.ntry == 0 || count < opt.ntry) {
                break;
            }
        }
    }
    if (ret as libc::c_uint == RETROK as libc::c_int as libc::c_uint
        || opt.content_on_error as libc::c_int != 0) && !local_file.is_null()
    {
        rpl_free(*local_file as *mut libc::c_void);
        *local_file = 0 as *mut libc::c_char;
        if !(hstat.local_file).is_null() {
            *local_file = hstat.local_file;
            hstat.local_file = 0 as *mut libc::c_char;
        }
    }
    free_hstat(&mut hstat);
    return ret;
}
unsafe extern "C" fn check_end(mut p: *const libc::c_char) -> bool {
    if p.is_null() {
        return 0 as libc::c_int != 0;
    }
    while c_isspace(*p as libc::c_int) {
        p = p.offset(1);
        p;
    }
    if *p == 0
        || *p.offset(0 as libc::c_int as isize) as libc::c_int == 'G' as i32
            && *p.offset(1 as libc::c_int as isize) as libc::c_int == 'M' as i32
            && *p.offset(2 as libc::c_int as isize) as libc::c_int == 'T' as i32
        || (*p.offset(0 as libc::c_int as isize) as libc::c_int == '+' as i32
            || *p.offset(0 as libc::c_int as isize) as libc::c_int == '-' as i32)
            && c_isdigit(*p.offset(1 as libc::c_int as isize) as libc::c_int)
                as libc::c_int != 0
    {
        return 1 as libc::c_int != 0
    } else {
        return 0 as libc::c_int != 0
    };
}
pub unsafe extern "C" fn http_atotm(mut time_string: *const libc::c_char) -> time_t {
    static mut time_formats: [*const libc::c_char; 4] = [
        b"%a, %d %b %Y %T\0" as *const u8 as *const libc::c_char,
        b"%A, %d-%b-%y %T\0" as *const u8 as *const libc::c_char,
        b"%a %b %d %T %Y\0" as *const u8 as *const libc::c_char,
        b"%a, %d-%b-%Y %T\0" as *const u8 as *const libc::c_char,
    ];
    let mut oldlocale: *const libc::c_char = 0 as *const libc::c_char;
    let mut savedlocale: [libc::c_char; 256] = [0; 256];
    let mut i: size_t = 0;
    let mut ret: time_t = -(1 as libc::c_int) as time_t;
    oldlocale = setlocale(2 as libc::c_int, 0 as *const libc::c_char);
    if !oldlocale.is_null() {
        let mut l: size_t = (strlen(oldlocale))
            .wrapping_add(1 as libc::c_int as libc::c_ulong);
        if l >= ::std::mem::size_of::<[libc::c_char; 256]>() as libc::c_ulong {
            savedlocale[0 as libc::c_int as usize] = '\0' as i32 as libc::c_char;
        } else {
            memcpy(
                savedlocale.as_mut_ptr() as *mut libc::c_void,
                oldlocale as *const libc::c_void,
                l,
            );
        }
    } else {
        savedlocale[0 as libc::c_int as usize] = '\0' as i32 as libc::c_char;
    }
    setlocale(2 as libc::c_int, b"C\0" as *const u8 as *const libc::c_char);
    i = 0 as libc::c_int as size_t;
    while i
        < (::std::mem::size_of::<[*const libc::c_char; 4]>() as libc::c_ulong)
            .wrapping_div(::std::mem::size_of::<*const libc::c_char>() as libc::c_ulong)
    {
        let mut t: tm = tm {
            tm_sec: 0,
            tm_min: 0,
            tm_hour: 0,
            tm_mday: 0,
            tm_mon: 0,
            tm_year: 0,
            tm_wday: 0,
            tm_yday: 0,
            tm_isdst: 0,
            tm_gmtoff: 0,
            tm_zone: 0 as *const libc::c_char,
        };
        memset(
            &mut t as *mut tm as *mut libc::c_void,
            '\0' as i32,
            ::std::mem::size_of::<tm>() as libc::c_ulong,
        );
        if check_end(strptime(time_string, time_formats[i as usize], &mut t)) {
            ret = rpl_timegm(&mut t);
            break;
        } else {
            i = i.wrapping_add(1);
            i;
        }
    }
    if savedlocale[0 as libc::c_int as usize] != 0 {
        setlocale(2 as libc::c_int, savedlocale.as_mut_ptr());
    }
    return ret;
}
unsafe extern "C" fn basic_authentication_encode(
    mut user: *const libc::c_char,
    mut passwd: *const libc::c_char,
) -> *mut libc::c_char {
    let mut buf_t1: [libc::c_char; 256] = [0; 256];
    let mut buf_t2: [libc::c_char; 256] = [0; 256];
    let mut t1: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut t2: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut ret: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut len1: size_t = (strlen(user))
        .wrapping_add(1 as libc::c_int as libc::c_ulong)
        .wrapping_add(strlen(passwd));
    if len1 < ::std::mem::size_of::<[libc::c_char; 256]>() as libc::c_ulong {
        t1 = buf_t1.as_mut_ptr();
    } else {
        t1 = xmalloc(len1.wrapping_add(1 as libc::c_int as libc::c_ulong))
            as *mut libc::c_char;
    }
    if (4 as libc::c_int as libc::c_ulong)
        .wrapping_mul(
            len1
                .wrapping_add(2 as libc::c_int as libc::c_ulong)
                .wrapping_div(3 as libc::c_int as libc::c_ulong),
        ) < ::std::mem::size_of::<[libc::c_char; 256]>() as libc::c_ulong
    {
        t2 = buf_t2.as_mut_ptr();
    } else {
        t2 = xmalloc(
            (4 as libc::c_int as libc::c_ulong)
                .wrapping_mul(
                    len1
                        .wrapping_add(2 as libc::c_int as libc::c_ulong)
                        .wrapping_div(3 as libc::c_int as libc::c_ulong),
                )
                .wrapping_add(1 as libc::c_int as libc::c_ulong),
        ) as *mut libc::c_char;
    }
    sprintf(t1, b"%s:%s\0" as *const u8 as *const libc::c_char, user, passwd);
    wget_base64_encode(t1 as *const libc::c_void, len1, t2);
    ret = concat_strings(
        b"Basic \0" as *const u8 as *const libc::c_char,
        t2,
        0 as *mut libc::c_char,
    );
    if t2 != buf_t2.as_mut_ptr() {
        rpl_free(t2 as *mut libc::c_void);
        t2 = 0 as *mut libc::c_char;
    }
    if t1 != buf_t1.as_mut_ptr() {
        rpl_free(t1 as *mut libc::c_void);
        t1 = 0 as *mut libc::c_char;
    }
    return ret;
}
unsafe extern "C" fn dump_hash(
    mut buf: *mut libc::c_char,
    mut hash: *const libc::c_uchar,
) {
    let mut i: libc::c_int = 0;
    i = 0 as libc::c_int;
    while i < 16 as libc::c_int {
        let fresh14 = buf;
        buf = buf.offset(1);
        *fresh14 = ((*::std::mem::transmute::<
            &[u8; 17],
            &[libc::c_char; 17],
        >(b"0123456789abcdef\0"))[(*hash as libc::c_int >> 4 as libc::c_int) as usize]
            as libc::c_int + 0 as libc::c_int) as libc::c_char;
        let fresh15 = buf;
        buf = buf.offset(1);
        *fresh15 = ((*::std::mem::transmute::<
            &[u8; 17],
            &[libc::c_char; 17],
        >(b"0123456789abcdef\0"))[(*hash as libc::c_int & 0xf as libc::c_int) as usize]
            as libc::c_int + 0 as libc::c_int) as libc::c_char;
        i += 1;
        i;
        hash = hash.offset(1);
        hash;
    }
    *buf = '\0' as i32 as libc::c_char;
}
unsafe extern "C" fn digest_authentication_encode(
    mut au: *const libc::c_char,
    mut user: *const libc::c_char,
    mut passwd: *const libc::c_char,
    mut method: *const libc::c_char,
    mut path: *const libc::c_char,
    mut auth_err: *mut uerr_t,
) -> *mut libc::c_char {
    static mut realm: *mut libc::c_char = 0 as *const libc::c_char as *mut libc::c_char;
    static mut opaque: *mut libc::c_char = 0 as *const libc::c_char as *mut libc::c_char;
    static mut nonce: *mut libc::c_char = 0 as *const libc::c_char as *mut libc::c_char;
    static mut qop: *mut libc::c_char = 0 as *const libc::c_char as *mut libc::c_char;
    static mut algorithm: *mut libc::c_char = 0 as *const libc::c_char
        as *mut libc::c_char;
    static mut options: [C2RustUnnamed_8; 5] = unsafe {
        [
            {
                let mut init = C2RustUnnamed_8 {
                    name: b"realm\0" as *const u8 as *const libc::c_char,
                    variable: &realm as *const *mut libc::c_char
                        as *mut *mut libc::c_char,
                };
                init
            },
            {
                let mut init = C2RustUnnamed_8 {
                    name: b"opaque\0" as *const u8 as *const libc::c_char,
                    variable: &opaque as *const *mut libc::c_char
                        as *mut *mut libc::c_char,
                };
                init
            },
            {
                let mut init = C2RustUnnamed_8 {
                    name: b"nonce\0" as *const u8 as *const libc::c_char,
                    variable: &nonce as *const *mut libc::c_char
                        as *mut *mut libc::c_char,
                };
                init
            },
            {
                let mut init = C2RustUnnamed_8 {
                    name: b"qop\0" as *const u8 as *const libc::c_char,
                    variable: &qop as *const *mut libc::c_char as *mut *mut libc::c_char,
                };
                init
            },
            {
                let mut init = C2RustUnnamed_8 {
                    name: b"algorithm\0" as *const u8 as *const libc::c_char,
                    variable: &algorithm as *const *mut libc::c_char
                        as *mut *mut libc::c_char,
                };
                init
            },
        ]
    };
    let mut cnonce: [libc::c_char; 16] = *::std::mem::transmute::<
        &[u8; 16],
        &mut [libc::c_char; 16],
    >(b"\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0");
    let mut res: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut res_len: libc::c_int = 0;
    let mut res_size: size_t = 0;
    let mut name: param_token = param_token {
        b: 0 as *const libc::c_char,
        e: 0 as *const libc::c_char,
    };
    let mut value: param_token = param_token {
        b: 0 as *const libc::c_char,
        e: 0 as *const libc::c_char,
    };
    qop = 0 as *mut libc::c_char;
    algorithm = qop;
    nonce = algorithm;
    opaque = nonce;
    realm = opaque;
    au = au.offset(6 as libc::c_int as isize);
    while extract_param(
        &mut au,
        &mut name,
        &mut value,
        ',' as i32 as libc::c_char,
        0 as *mut bool,
    ) {
        let mut i: size_t = 0;
        let mut namelen: size_t = (name.e).offset_from(name.b) as libc::c_long as size_t;
        i = 0 as libc::c_int as size_t;
        while i
            < (::std::mem::size_of::<[C2RustUnnamed_8; 5]>() as libc::c_ulong)
                .wrapping_div(::std::mem::size_of::<C2RustUnnamed_8>() as libc::c_ulong)
        {
            if namelen == strlen(options[i as usize].name)
                && 0 as libc::c_int == strncmp(name.b, options[i as usize].name, namelen)
            {
                *options[i as usize].variable = strdupdelim(value.b, value.e);
                break;
            } else {
                i = i.wrapping_add(1);
                i;
            }
        }
    }
    if !qop.is_null() && strcmp(qop, b"auth\0" as *const u8 as *const libc::c_char) != 0
    {
        logprintf(
            LOG_NOTQUIET,
            dcgettext(
                0 as *const libc::c_char,
                b"Unsupported quality of protection '%s'.\n\0" as *const u8
                    as *const libc::c_char,
                5 as libc::c_int,
            ),
            qop,
        );
        rpl_free(qop as *mut libc::c_void);
        qop = 0 as *mut libc::c_char;
    } else if !algorithm.is_null()
        && strcmp(algorithm, b"MD5\0" as *const u8 as *const libc::c_char) != 0
        && strcmp(algorithm, b"MD5-sess\0" as *const u8 as *const libc::c_char) != 0
    {
        logprintf(
            LOG_NOTQUIET,
            dcgettext(
                0 as *const libc::c_char,
                b"Unsupported algorithm '%s'.\n\0" as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ),
            algorithm,
        );
        rpl_free(algorithm as *mut libc::c_void);
        algorithm = 0 as *mut libc::c_char;
    }
    if realm.is_null() || nonce.is_null() || user.is_null() || passwd.is_null()
        || path.is_null() || method.is_null()
    {
        *auth_err = ATTRMISSING;
    } else {
        let mut ctx: md5_ctx = md5_ctx {
            A: 0,
            B: 0,
            C: 0,
            D: 0,
            total: [0; 2],
            buflen: 0,
            buffer: [0; 32],
        };
        let mut hash: [libc::c_uchar; 16] = [0; 16];
        let mut a1buf: [libc::c_char; 33] = [0; 33];
        let mut a2buf: [libc::c_char; 33] = [0; 33];
        let mut response_digest: [libc::c_char; 33] = [0; 33];
        md5_init_ctx(&mut ctx);
        md5_process_bytes(
            user as *mut libc::c_uchar as *const libc::c_void,
            strlen(user),
            &mut ctx,
        );
        md5_process_bytes(
            b":\0" as *const u8 as *const libc::c_char as *mut libc::c_uchar
                as *const libc::c_void,
            1 as libc::c_int as size_t,
            &mut ctx,
        );
        md5_process_bytes(
            realm as *mut libc::c_uchar as *const libc::c_void,
            strlen(realm),
            &mut ctx,
        );
        md5_process_bytes(
            b":\0" as *const u8 as *const libc::c_char as *mut libc::c_uchar
                as *const libc::c_void,
            1 as libc::c_int as size_t,
            &mut ctx,
        );
        md5_process_bytes(
            passwd as *mut libc::c_uchar as *const libc::c_void,
            strlen(passwd),
            &mut ctx,
        );
        md5_finish_ctx(&mut ctx, hash.as_mut_ptr() as *mut libc::c_void);
        dump_hash(a1buf.as_mut_ptr(), hash.as_mut_ptr());
        if !algorithm.is_null()
            && strcmp(algorithm, b"MD5-sess\0" as *const u8 as *const libc::c_char) == 0
        {
            snprintf(
                cnonce.as_mut_ptr(),
                ::std::mem::size_of::<[libc::c_char; 16]>() as libc::c_ulong,
                b"%08x\0" as *const u8 as *const libc::c_char,
                random_number(2147483647 as libc::c_int) as libc::c_uint,
            );
            md5_init_ctx(&mut ctx);
            md5_process_bytes(
                a1buf.as_mut_ptr() as *const libc::c_void,
                (16 as libc::c_int * 2 as libc::c_int) as size_t,
                &mut ctx,
            );
            md5_process_bytes(
                b":\0" as *const u8 as *const libc::c_char as *mut libc::c_uchar
                    as *const libc::c_void,
                1 as libc::c_int as size_t,
                &mut ctx,
            );
            md5_process_bytes(
                nonce as *mut libc::c_uchar as *const libc::c_void,
                strlen(nonce),
                &mut ctx,
            );
            md5_process_bytes(
                b":\0" as *const u8 as *const libc::c_char as *mut libc::c_uchar
                    as *const libc::c_void,
                1 as libc::c_int as size_t,
                &mut ctx,
            );
            md5_process_bytes(
                cnonce.as_mut_ptr() as *mut libc::c_uchar as *const libc::c_void,
                strlen(cnonce.as_mut_ptr()),
                &mut ctx,
            );
            md5_finish_ctx(&mut ctx, hash.as_mut_ptr() as *mut libc::c_void);
            dump_hash(a1buf.as_mut_ptr(), hash.as_mut_ptr());
        }
        md5_init_ctx(&mut ctx);
        md5_process_bytes(
            method as *mut libc::c_uchar as *const libc::c_void,
            strlen(method),
            &mut ctx,
        );
        md5_process_bytes(
            b":\0" as *const u8 as *const libc::c_char as *mut libc::c_uchar
                as *const libc::c_void,
            1 as libc::c_int as size_t,
            &mut ctx,
        );
        md5_process_bytes(
            path as *mut libc::c_uchar as *const libc::c_void,
            strlen(path),
            &mut ctx,
        );
        md5_finish_ctx(&mut ctx, hash.as_mut_ptr() as *mut libc::c_void);
        dump_hash(a2buf.as_mut_ptr(), hash.as_mut_ptr());
        if !qop.is_null()
            && strcmp(qop, b"auth\0" as *const u8 as *const libc::c_char) == 0
        {
            if *cnonce.as_mut_ptr() == 0 {
                snprintf(
                    cnonce.as_mut_ptr(),
                    ::std::mem::size_of::<[libc::c_char; 16]>() as libc::c_ulong,
                    b"%08x\0" as *const u8 as *const libc::c_char,
                    random_number(2147483647 as libc::c_int) as libc::c_uint,
                );
            }
            md5_init_ctx(&mut ctx);
            md5_process_bytes(
                a1buf.as_mut_ptr() as *mut libc::c_uchar as *const libc::c_void,
                (16 as libc::c_int * 2 as libc::c_int) as size_t,
                &mut ctx,
            );
            md5_process_bytes(
                b":\0" as *const u8 as *const libc::c_char as *mut libc::c_uchar
                    as *const libc::c_void,
                1 as libc::c_int as size_t,
                &mut ctx,
            );
            md5_process_bytes(
                nonce as *mut libc::c_uchar as *const libc::c_void,
                strlen(nonce),
                &mut ctx,
            );
            md5_process_bytes(
                b":\0" as *const u8 as *const libc::c_char as *mut libc::c_uchar
                    as *const libc::c_void,
                1 as libc::c_int as size_t,
                &mut ctx,
            );
            md5_process_bytes(
                b"00000001\0" as *const u8 as *const libc::c_char as *mut libc::c_uchar
                    as *const libc::c_void,
                8 as libc::c_int as size_t,
                &mut ctx,
            );
            md5_process_bytes(
                b":\0" as *const u8 as *const libc::c_char as *mut libc::c_uchar
                    as *const libc::c_void,
                1 as libc::c_int as size_t,
                &mut ctx,
            );
            md5_process_bytes(
                cnonce.as_mut_ptr() as *mut libc::c_uchar as *const libc::c_void,
                strlen(cnonce.as_mut_ptr()),
                &mut ctx,
            );
            md5_process_bytes(
                b":\0" as *const u8 as *const libc::c_char as *mut libc::c_uchar
                    as *const libc::c_void,
                1 as libc::c_int as size_t,
                &mut ctx,
            );
            md5_process_bytes(
                qop as *mut libc::c_uchar as *const libc::c_void,
                strlen(qop),
                &mut ctx,
            );
            md5_process_bytes(
                b":\0" as *const u8 as *const libc::c_char as *mut libc::c_uchar
                    as *const libc::c_void,
                1 as libc::c_int as size_t,
                &mut ctx,
            );
            md5_process_bytes(
                a2buf.as_mut_ptr() as *mut libc::c_uchar as *const libc::c_void,
                (16 as libc::c_int * 2 as libc::c_int) as size_t,
                &mut ctx,
            );
            md5_finish_ctx(&mut ctx, hash.as_mut_ptr() as *mut libc::c_void);
        } else {
            md5_init_ctx(&mut ctx);
            md5_process_bytes(
                a1buf.as_mut_ptr() as *mut libc::c_uchar as *const libc::c_void,
                (16 as libc::c_int * 2 as libc::c_int) as size_t,
                &mut ctx,
            );
            md5_process_bytes(
                b":\0" as *const u8 as *const libc::c_char as *mut libc::c_uchar
                    as *const libc::c_void,
                1 as libc::c_int as size_t,
                &mut ctx,
            );
            md5_process_bytes(
                nonce as *mut libc::c_uchar as *const libc::c_void,
                strlen(nonce),
                &mut ctx,
            );
            md5_process_bytes(
                b":\0" as *const u8 as *const libc::c_char as *mut libc::c_uchar
                    as *const libc::c_void,
                1 as libc::c_int as size_t,
                &mut ctx,
            );
            md5_process_bytes(
                a2buf.as_mut_ptr() as *mut libc::c_uchar as *const libc::c_void,
                (16 as libc::c_int * 2 as libc::c_int) as size_t,
                &mut ctx,
            );
            md5_finish_ctx(&mut ctx, hash.as_mut_ptr() as *mut libc::c_void);
        }
        dump_hash(response_digest.as_mut_ptr(), hash.as_mut_ptr());
        res_size = (strlen(user))
            .wrapping_add(strlen(realm))
            .wrapping_add(strlen(nonce))
            .wrapping_add(strlen(path))
            .wrapping_add((2 as libc::c_int * 16 as libc::c_int) as libc::c_ulong)
            .wrapping_add(
                if !opaque.is_null() {
                    strlen(opaque)
                } else {
                    0 as libc::c_int as libc::c_ulong
                },
            )
            .wrapping_add(
                if !algorithm.is_null() {
                    strlen(algorithm)
                } else {
                    0 as libc::c_int as libc::c_ulong
                },
            )
            .wrapping_add(
                (if !qop.is_null() { 128 as libc::c_int } else { 0 as libc::c_int })
                    as libc::c_ulong,
            )
            .wrapping_add(strlen(cnonce.as_mut_ptr()))
            .wrapping_add(128 as libc::c_int as libc::c_ulong);
        res = xmalloc(res_size) as *mut libc::c_char;
        if !qop.is_null()
            && strcmp(qop, b"auth\0" as *const u8 as *const libc::c_char) == 0
        {
            res_len = snprintf(
                res,
                res_size,
                b"Digest username=\"%s\", realm=\"%s\", nonce=\"%s\", uri=\"%s\", response=\"%s\", qop=auth, nc=00000001, cnonce=\"%s\"\0"
                    as *const u8 as *const libc::c_char,
                user,
                realm,
                nonce,
                path,
                response_digest.as_mut_ptr(),
                cnonce.as_mut_ptr(),
            );
        } else {
            res_len = snprintf(
                res,
                res_size,
                b"Digest username=\"%s\", realm=\"%s\", nonce=\"%s\", uri=\"%s\", response=\"%s\"\0"
                    as *const u8 as *const libc::c_char,
                user,
                realm,
                nonce,
                path,
                response_digest.as_mut_ptr(),
            );
        }
        if !opaque.is_null() {
            res_len
                += snprintf(
                    res.offset(res_len as isize),
                    res_size.wrapping_sub(res_len as libc::c_ulong),
                    b", opaque=\"%s\"\0" as *const u8 as *const libc::c_char,
                    opaque,
                );
        }
        if !algorithm.is_null() {
            snprintf(
                res.offset(res_len as isize),
                res_size.wrapping_sub(res_len as libc::c_ulong),
                b", algorithm=\"%s\"\0" as *const u8 as *const libc::c_char,
                algorithm,
            );
        }
    }
    rpl_free(realm as *mut libc::c_void);
    realm = 0 as *mut libc::c_char;
    rpl_free(opaque as *mut libc::c_void);
    opaque = 0 as *mut libc::c_char;
    rpl_free(nonce as *mut libc::c_void);
    nonce = 0 as *mut libc::c_char;
    rpl_free(qop as *mut libc::c_void);
    qop = 0 as *mut libc::c_char;
    rpl_free(algorithm as *mut libc::c_void);
    algorithm = 0 as *mut libc::c_char;
    return res;
}
unsafe extern "C" fn known_authentication_scheme_p(
    mut hdrbeg: *const libc::c_char,
    mut hdrend: *const libc::c_char,
) -> bool {
    return hdrend > hdrbeg
        && hdrend.offset_from(hdrbeg) as libc::c_long as size_t
            >= (::std::mem::size_of::<[libc::c_char; 6]>() as libc::c_ulong)
                .wrapping_sub(1 as libc::c_int as libc::c_ulong)
        && 0 as libc::c_int
            == c_strncasecmp(
                hdrbeg,
                b"Basic\0" as *const u8 as *const libc::c_char,
                (::std::mem::size_of::<[libc::c_char; 6]>() as libc::c_ulong)
                    .wrapping_sub(1 as libc::c_int as libc::c_ulong),
            )
        && (hdrend.offset_from(hdrbeg) as libc::c_long as size_t
            == (::std::mem::size_of::<[libc::c_char; 6]>() as libc::c_ulong)
                .wrapping_sub(1 as libc::c_int as libc::c_ulong)
            || c_isspace(
                *hdrbeg
                    .offset(
                        (::std::mem::size_of::<[libc::c_char; 6]>() as libc::c_ulong)
                            .wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize,
                    ) as libc::c_int,
            ) as libc::c_int != 0)
        || hdrend > hdrbeg
            && hdrend.offset_from(hdrbeg) as libc::c_long as size_t
                >= (::std::mem::size_of::<[libc::c_char; 7]>() as libc::c_ulong)
                    .wrapping_sub(1 as libc::c_int as libc::c_ulong)
            && 0 as libc::c_int
                == c_strncasecmp(
                    hdrbeg,
                    b"Digest\0" as *const u8 as *const libc::c_char,
                    (::std::mem::size_of::<[libc::c_char; 7]>() as libc::c_ulong)
                        .wrapping_sub(1 as libc::c_int as libc::c_ulong),
                )
            && (hdrend.offset_from(hdrbeg) as libc::c_long as size_t
                == (::std::mem::size_of::<[libc::c_char; 7]>() as libc::c_ulong)
                    .wrapping_sub(1 as libc::c_int as libc::c_ulong)
                || c_isspace(
                    *hdrbeg
                        .offset(
                            (::std::mem::size_of::<[libc::c_char; 7]>() as libc::c_ulong)
                                .wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize,
                        ) as libc::c_int,
                ) as libc::c_int != 0)
        || hdrend > hdrbeg
            && hdrend.offset_from(hdrbeg) as libc::c_long as size_t
                >= (::std::mem::size_of::<[libc::c_char; 5]>() as libc::c_ulong)
                    .wrapping_sub(1 as libc::c_int as libc::c_ulong)
            && 0 as libc::c_int
                == c_strncasecmp(
                    hdrbeg,
                    b"NTLM\0" as *const u8 as *const libc::c_char,
                    (::std::mem::size_of::<[libc::c_char; 5]>() as libc::c_ulong)
                        .wrapping_sub(1 as libc::c_int as libc::c_ulong),
                )
            && (hdrend.offset_from(hdrbeg) as libc::c_long as size_t
                == (::std::mem::size_of::<[libc::c_char; 5]>() as libc::c_ulong)
                    .wrapping_sub(1 as libc::c_int as libc::c_ulong)
                || c_isspace(
                    *hdrbeg
                        .offset(
                            (::std::mem::size_of::<[libc::c_char; 5]>() as libc::c_ulong)
                                .wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize,
                        ) as libc::c_int,
                ) as libc::c_int != 0);
}
unsafe extern "C" fn create_authorization_line(
    mut au: *const libc::c_char,
    mut user: *const libc::c_char,
    mut passwd: *const libc::c_char,
    mut method: *const libc::c_char,
    mut path: *const libc::c_char,
    mut finished: *mut bool,
    mut auth_err: *mut uerr_t,
) -> *mut libc::c_char {
    match c_toupper(*au as libc::c_int) {
        66 => {
            *finished = 1 as libc::c_int != 0;
            return basic_authentication_encode(user, passwd);
        }
        68 => {
            *finished = 1 as libc::c_int != 0;
            return digest_authentication_encode(
                au,
                user,
                passwd,
                method,
                path,
                auth_err,
            );
        }
        78 => {
            if !ntlm_input(&mut pconn.ntlm, au) {
                *finished = 1 as libc::c_int != 0;
                return 0 as *mut libc::c_char;
            }
            return ntlm_output(&mut pconn.ntlm, user, passwd, finished);
        }
        _ => {
            abort();
        }
    };
}
unsafe extern "C" fn load_cookies() {
    if wget_cookie_jar.is_null() {
        wget_cookie_jar = cookie_jar_new();
    }
    if !(opt.cookies_input).is_null() && !cookies_loaded_p {
        cookie_jar_load(wget_cookie_jar, opt.cookies_input);
        cookies_loaded_p = 1 as libc::c_int != 0;
    }
}
pub unsafe extern "C" fn save_cookies() {
    if !wget_cookie_jar.is_null() {
        cookie_jar_save(wget_cookie_jar, opt.cookies_output);
    }
}
unsafe extern "C" fn ensure_extension(
    mut hs: *mut http_stat,
    mut ext: *const libc::c_char,
    mut dt: *mut libc::c_int,
) {
    let mut last_period_in_local_filename: *mut libc::c_char = strrchr(
        (*hs).local_file,
        '.' as i32,
    );
    let mut shortext: [libc::c_char; 8] = [0; 8];
    let mut len: libc::c_int = 0;
    shortext[0 as libc::c_int as usize] = '\0' as i32 as libc::c_char;
    len = strlen(ext) as libc::c_int;
    if len == 5 as libc::c_int {
        memcpy(
            shortext.as_mut_ptr() as *mut libc::c_void,
            ext as *const libc::c_void,
            (len - 1 as libc::c_int) as libc::c_ulong,
        );
        shortext[(len - 1 as libc::c_int) as usize] = '\0' as i32 as libc::c_char;
    }
    if last_period_in_local_filename.is_null()
        || !(0 as libc::c_int
            == strcasecmp(last_period_in_local_filename, shortext.as_mut_ptr())
            || 0 as libc::c_int == strcasecmp(last_period_in_local_filename, ext))
    {
        let mut local_filename_len: libc::c_int = strlen((*hs).local_file)
            as libc::c_int;
        (*hs)
            .local_file = xrealloc(
            (*hs).local_file as *mut libc::c_void,
            (local_filename_len + 24 as libc::c_int + len) as size_t,
        ) as *mut libc::c_char;
        strcpy(((*hs).local_file).offset(local_filename_len as isize), ext);
        if !(opt.noclobber as libc::c_int != 0 || opt.always_rest as libc::c_int != 0
            || opt.timestamping as libc::c_int != 0 || opt.dirstruct as libc::c_int != 0
            || !(opt.output_document).is_null() || opt.backups > 0 as libc::c_int)
            && file_exists_p((*hs).local_file, 0 as *mut file_stats_t) as libc::c_int
                != 0
        {
            let mut ext_num: libc::c_int = 1 as libc::c_int;
            loop {
                let fresh16 = ext_num;
                ext_num = ext_num + 1;
                sprintf(
                    ((*hs).local_file).offset(local_filename_len as isize),
                    b".%d%s\0" as *const u8 as *const libc::c_char,
                    fresh16,
                    ext,
                );
                if !file_exists_p((*hs).local_file, 0 as *mut file_stats_t) {
                    break;
                }
            }
        }
        *dt |= ADDED_HTML_EXTENSION as libc::c_int;
    }
}
