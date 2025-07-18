use ::libc;
extern "C" {
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
    fn memcpy(
        _: *mut libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
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
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    fn strchr(_: *const libc::c_char, _: libc::c_int) -> *mut libc::c_char;
    fn strrchr(_: *const libc::c_char, _: libc::c_int) -> *mut libc::c_char;
    fn strspn(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_ulong;
    fn strpbrk(_: *const libc::c_char, _: *const libc::c_char) -> *mut libc::c_char;
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    fn strerror(_: libc::c_int) -> *mut libc::c_char;
    fn rpl_free(_: *mut libc::c_void);
    static mut opt: options;
    fn remote_to_utf8(
        i: *const iri,
        str: *const libc::c_char,
        newstr: *mut *mut libc::c_char,
    ) -> bool;
    fn idn_encode(i: *const iri, host: *const libc::c_char) -> *mut libc::c_char;
    fn quote_n(n: libc::c_int, arg: *const libc::c_char) -> *const libc::c_char;
    fn debug_logprintf(_: *const libc::c_char, _: ...);
    fn logprintf(_: log_options, _: *const libc::c_char, _: ...);
    fn xmalloc(s: size_t) -> *mut libc::c_void;
    fn xcalloc(n: size_t, s: size_t) -> *mut libc::c_void;
    fn xrealloc(p: *mut libc::c_void, s: size_t) -> *mut libc::c_void;
    fn xstrdup(str: *const libc::c_char) -> *mut libc::c_char;
    fn unlink(__name: *const libc::c_char) -> libc::c_int;
    fn __errno_location() -> *mut libc::c_int;
    fn strdupdelim(_: *const libc::c_char, _: *const libc::c_char) -> *mut libc::c_char;
    fn aprintf(_: *const libc::c_char, _: ...) -> *mut libc::c_char;
    fn concat_strings(_: *const libc::c_char, _: ...) -> *mut libc::c_char;
    fn file_exists_p(_: *const libc::c_char, _: *mut file_stats_t) -> bool;
    fn file_non_directory_p(_: *const libc::c_char) -> bool;
    fn make_directory(_: *const libc::c_char) -> libc::c_int;
    fn unique_name_passthrough(_: *const libc::c_char) -> *mut libc::c_char;
    fn numdigit(_: wgint) -> libc::c_int;
    fn number_to_string(_: *mut libc::c_char, _: wgint) -> *mut libc::c_char;
    fn get_max_length(
        path: *const libc::c_char,
        length: libc::c_int,
        name: libc::c_int,
    ) -> libc::c_long;
    fn is_valid_ipv6_address(_: *const libc::c_char, _: *const libc::c_char) -> bool;
    fn c_strncasecmp(
        s1: *const libc::c_char,
        s2: *const libc::c_char,
        n: size_t,
    ) -> libc::c_int;
    fn iconv_open(
        __tocode: *const libc::c_char,
        __fromcode: *const libc::c_char,
    ) -> iconv_t;
    fn iconv(
        __cd: iconv_t,
        __inbuf: *mut *mut libc::c_char,
        __inbytesleft: *mut size_t,
        __outbuf: *mut *mut libc::c_char,
        __outbytesleft: *mut size_t,
    ) -> size_t;
    fn iconv_close(__cd: iconv_t) -> libc::c_int;
    fn nl_langinfo(__item: nl_item) -> *mut libc::c_char;
}
pub type __int64_t = libc::c_long;
pub type __dev_t = libc::c_ulong;
pub type __uid_t = libc::c_uint;
pub type __gid_t = libc::c_uint;
pub type __ino_t = libc::c_ulong;
pub type __mode_t = libc::c_uint;
pub type __nlink_t = libc::c_ulong;
pub type __off_t = libc::c_long;
pub type __time_t = libc::c_long;
pub type __blksize_t = libc::c_long;
pub type __blkcnt_t = libc::c_long;
pub type __syscall_slong_t = libc::c_long;
pub type ino_t = __ino_t;
pub type dev_t = __dev_t;
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
pub type C2RustUnnamed_3 = libc::c_uint;
pub const _PC_2_SYMLINKS: C2RustUnnamed_3 = 20;
pub const _PC_SYMLINK_MAX: C2RustUnnamed_3 = 19;
pub const _PC_ALLOC_SIZE_MIN: C2RustUnnamed_3 = 18;
pub const _PC_REC_XFER_ALIGN: C2RustUnnamed_3 = 17;
pub const _PC_REC_MIN_XFER_SIZE: C2RustUnnamed_3 = 16;
pub const _PC_REC_MAX_XFER_SIZE: C2RustUnnamed_3 = 15;
pub const _PC_REC_INCR_XFER_SIZE: C2RustUnnamed_3 = 14;
pub const _PC_FILESIZEBITS: C2RustUnnamed_3 = 13;
pub const _PC_SOCK_MAXBUF: C2RustUnnamed_3 = 12;
pub const _PC_PRIO_IO: C2RustUnnamed_3 = 11;
pub const _PC_ASYNC_IO: C2RustUnnamed_3 = 10;
pub const _PC_SYNC_IO: C2RustUnnamed_3 = 9;
pub const _PC_VDISABLE: C2RustUnnamed_3 = 8;
pub const _PC_NO_TRUNC: C2RustUnnamed_3 = 7;
pub const _PC_CHOWN_RESTRICTED: C2RustUnnamed_3 = 6;
pub const _PC_PIPE_BUF: C2RustUnnamed_3 = 5;
pub const _PC_PATH_MAX: C2RustUnnamed_3 = 4;
pub const _PC_NAME_MAX: C2RustUnnamed_3 = 3;
pub const _PC_MAX_INPUT: C2RustUnnamed_3 = 2;
pub const _PC_MAX_CANON: C2RustUnnamed_3 = 1;
pub const _PC_LINK_MAX: C2RustUnnamed_3 = 0;
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
pub const urlchr_unsafe: C2RustUnnamed_6 = 2;
pub const urlchr_reserved: C2RustUnnamed_6 = 1;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct scheme_data {
    pub name: *const libc::c_char,
    pub leading_string: *const libc::c_char,
    pub default_port: libc::c_int,
    pub flags: libc::c_int,
}
pub const scm_has_fragment: C2RustUnnamed_5 = 8;
pub const scm_has_params: C2RustUnnamed_5 = 2;
pub const scm_has_query: C2RustUnnamed_5 = 4;
pub const PE_INVALID_HOST_NAME: C2RustUnnamed_7 = 5;
pub const PE_INVALID_USER_NAME: C2RustUnnamed_7 = 7;
pub const PE_BAD_PORT_NUMBER: C2RustUnnamed_7 = 6;
pub const PE_INVALID_IPV6_ADDRESS: C2RustUnnamed_7 = 10;
pub const PE_UNTERMINATED_IPV6_ADDRESS: C2RustUnnamed_7 = 8;
pub const PE_UNSUPPORTED_SCHEME: C2RustUnnamed_7 = 1;
pub const PE_UNSUPPORTED_SCHEME_FTPS: C2RustUnnamed_7 = 3;
pub const PE_UNSUPPORTED_SCHEME_HTTPS: C2RustUnnamed_7 = 2;
pub const PE_MISSING_SCHEME: C2RustUnnamed_7 = 4;
pub const scm_disabled: C2RustUnnamed_5 = 1;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct growable {
    pub base: *mut libc::c_char,
    pub size: libc::c_int,
    pub tail: libc::c_int,
}
pub type iconv_t = *mut libc::c_void;
pub const CODESET: C2RustUnnamed_4 = 14;
pub type nl_item = libc::c_int;
pub const filechr_control: C2RustUnnamed_8 = 8;
pub const filechr_not_windows: C2RustUnnamed_8 = 4;
pub const filechr_not_vms: C2RustUnnamed_8 = 2;
pub const filechr_not_unix: C2RustUnnamed_8 = 1;
pub type C2RustUnnamed_4 = libc::c_uint;
pub const _NL_NUM: C2RustUnnamed_4 = 786449;
pub const _NL_NUM_LC_IDENTIFICATION: C2RustUnnamed_4 = 786448;
pub const _NL_IDENTIFICATION_CODESET: C2RustUnnamed_4 = 786447;
pub const _NL_IDENTIFICATION_CATEGORY: C2RustUnnamed_4 = 786446;
pub const _NL_IDENTIFICATION_DATE: C2RustUnnamed_4 = 786445;
pub const _NL_IDENTIFICATION_REVISION: C2RustUnnamed_4 = 786444;
pub const _NL_IDENTIFICATION_ABBREVIATION: C2RustUnnamed_4 = 786443;
pub const _NL_IDENTIFICATION_APPLICATION: C2RustUnnamed_4 = 786442;
pub const _NL_IDENTIFICATION_AUDIENCE: C2RustUnnamed_4 = 786441;
pub const _NL_IDENTIFICATION_TERRITORY: C2RustUnnamed_4 = 786440;
pub const _NL_IDENTIFICATION_LANGUAGE: C2RustUnnamed_4 = 786439;
pub const _NL_IDENTIFICATION_FAX: C2RustUnnamed_4 = 786438;
pub const _NL_IDENTIFICATION_TEL: C2RustUnnamed_4 = 786437;
pub const _NL_IDENTIFICATION_EMAIL: C2RustUnnamed_4 = 786436;
pub const _NL_IDENTIFICATION_CONTACT: C2RustUnnamed_4 = 786435;
pub const _NL_IDENTIFICATION_ADDRESS: C2RustUnnamed_4 = 786434;
pub const _NL_IDENTIFICATION_SOURCE: C2RustUnnamed_4 = 786433;
pub const _NL_IDENTIFICATION_TITLE: C2RustUnnamed_4 = 786432;
pub const _NL_NUM_LC_MEASUREMENT: C2RustUnnamed_4 = 720898;
pub const _NL_MEASUREMENT_CODESET: C2RustUnnamed_4 = 720897;
pub const _NL_MEASUREMENT_MEASUREMENT: C2RustUnnamed_4 = 720896;
pub const _NL_NUM_LC_TELEPHONE: C2RustUnnamed_4 = 655365;
pub const _NL_TELEPHONE_CODESET: C2RustUnnamed_4 = 655364;
pub const _NL_TELEPHONE_INT_PREFIX: C2RustUnnamed_4 = 655363;
pub const _NL_TELEPHONE_INT_SELECT: C2RustUnnamed_4 = 655362;
pub const _NL_TELEPHONE_TEL_DOM_FMT: C2RustUnnamed_4 = 655361;
pub const _NL_TELEPHONE_TEL_INT_FMT: C2RustUnnamed_4 = 655360;
pub const _NL_NUM_LC_ADDRESS: C2RustUnnamed_4 = 589837;
pub const _NL_ADDRESS_CODESET: C2RustUnnamed_4 = 589836;
pub const _NL_ADDRESS_LANG_LIB: C2RustUnnamed_4 = 589835;
pub const _NL_ADDRESS_LANG_TERM: C2RustUnnamed_4 = 589834;
pub const _NL_ADDRESS_LANG_AB: C2RustUnnamed_4 = 589833;
pub const _NL_ADDRESS_LANG_NAME: C2RustUnnamed_4 = 589832;
pub const _NL_ADDRESS_COUNTRY_ISBN: C2RustUnnamed_4 = 589831;
pub const _NL_ADDRESS_COUNTRY_NUM: C2RustUnnamed_4 = 589830;
pub const _NL_ADDRESS_COUNTRY_CAR: C2RustUnnamed_4 = 589829;
pub const _NL_ADDRESS_COUNTRY_AB3: C2RustUnnamed_4 = 589828;
pub const _NL_ADDRESS_COUNTRY_AB2: C2RustUnnamed_4 = 589827;
pub const _NL_ADDRESS_COUNTRY_POST: C2RustUnnamed_4 = 589826;
pub const _NL_ADDRESS_COUNTRY_NAME: C2RustUnnamed_4 = 589825;
pub const _NL_ADDRESS_POSTAL_FMT: C2RustUnnamed_4 = 589824;
pub const _NL_NUM_LC_NAME: C2RustUnnamed_4 = 524295;
pub const _NL_NAME_CODESET: C2RustUnnamed_4 = 524294;
pub const _NL_NAME_NAME_MS: C2RustUnnamed_4 = 524293;
pub const _NL_NAME_NAME_MISS: C2RustUnnamed_4 = 524292;
pub const _NL_NAME_NAME_MRS: C2RustUnnamed_4 = 524291;
pub const _NL_NAME_NAME_MR: C2RustUnnamed_4 = 524290;
pub const _NL_NAME_NAME_GEN: C2RustUnnamed_4 = 524289;
pub const _NL_NAME_NAME_FMT: C2RustUnnamed_4 = 524288;
pub const _NL_NUM_LC_PAPER: C2RustUnnamed_4 = 458755;
pub const _NL_PAPER_CODESET: C2RustUnnamed_4 = 458754;
pub const _NL_PAPER_WIDTH: C2RustUnnamed_4 = 458753;
pub const _NL_PAPER_HEIGHT: C2RustUnnamed_4 = 458752;
pub const _NL_NUM_LC_MESSAGES: C2RustUnnamed_4 = 327685;
pub const _NL_MESSAGES_CODESET: C2RustUnnamed_4 = 327684;
pub const __NOSTR: C2RustUnnamed_4 = 327683;
pub const __YESSTR: C2RustUnnamed_4 = 327682;
pub const __NOEXPR: C2RustUnnamed_4 = 327681;
pub const __YESEXPR: C2RustUnnamed_4 = 327680;
pub const _NL_NUM_LC_NUMERIC: C2RustUnnamed_4 = 65542;
pub const _NL_NUMERIC_CODESET: C2RustUnnamed_4 = 65541;
pub const _NL_NUMERIC_THOUSANDS_SEP_WC: C2RustUnnamed_4 = 65540;
pub const _NL_NUMERIC_DECIMAL_POINT_WC: C2RustUnnamed_4 = 65539;
pub const __GROUPING: C2RustUnnamed_4 = 65538;
pub const THOUSEP: C2RustUnnamed_4 = 65537;
pub const __THOUSANDS_SEP: C2RustUnnamed_4 = 65537;
pub const RADIXCHAR: C2RustUnnamed_4 = 65536;
pub const __DECIMAL_POINT: C2RustUnnamed_4 = 65536;
pub const _NL_NUM_LC_MONETARY: C2RustUnnamed_4 = 262190;
pub const _NL_MONETARY_CODESET: C2RustUnnamed_4 = 262189;
pub const _NL_MONETARY_THOUSANDS_SEP_WC: C2RustUnnamed_4 = 262188;
pub const _NL_MONETARY_DECIMAL_POINT_WC: C2RustUnnamed_4 = 262187;
pub const _NL_MONETARY_CONVERSION_RATE: C2RustUnnamed_4 = 262186;
pub const _NL_MONETARY_DUO_VALID_TO: C2RustUnnamed_4 = 262185;
pub const _NL_MONETARY_DUO_VALID_FROM: C2RustUnnamed_4 = 262184;
pub const _NL_MONETARY_UNO_VALID_TO: C2RustUnnamed_4 = 262183;
pub const _NL_MONETARY_UNO_VALID_FROM: C2RustUnnamed_4 = 262182;
pub const _NL_MONETARY_DUO_INT_N_SIGN_POSN: C2RustUnnamed_4 = 262181;
pub const _NL_MONETARY_DUO_INT_P_SIGN_POSN: C2RustUnnamed_4 = 262180;
pub const _NL_MONETARY_DUO_N_SIGN_POSN: C2RustUnnamed_4 = 262179;
pub const _NL_MONETARY_DUO_P_SIGN_POSN: C2RustUnnamed_4 = 262178;
pub const _NL_MONETARY_DUO_INT_N_SEP_BY_SPACE: C2RustUnnamed_4 = 262177;
pub const _NL_MONETARY_DUO_INT_N_CS_PRECEDES: C2RustUnnamed_4 = 262176;
pub const _NL_MONETARY_DUO_INT_P_SEP_BY_SPACE: C2RustUnnamed_4 = 262175;
pub const _NL_MONETARY_DUO_INT_P_CS_PRECEDES: C2RustUnnamed_4 = 262174;
pub const _NL_MONETARY_DUO_N_SEP_BY_SPACE: C2RustUnnamed_4 = 262173;
pub const _NL_MONETARY_DUO_N_CS_PRECEDES: C2RustUnnamed_4 = 262172;
pub const _NL_MONETARY_DUO_P_SEP_BY_SPACE: C2RustUnnamed_4 = 262171;
pub const _NL_MONETARY_DUO_P_CS_PRECEDES: C2RustUnnamed_4 = 262170;
pub const _NL_MONETARY_DUO_FRAC_DIGITS: C2RustUnnamed_4 = 262169;
pub const _NL_MONETARY_DUO_INT_FRAC_DIGITS: C2RustUnnamed_4 = 262168;
pub const _NL_MONETARY_DUO_CURRENCY_SYMBOL: C2RustUnnamed_4 = 262167;
pub const _NL_MONETARY_DUO_INT_CURR_SYMBOL: C2RustUnnamed_4 = 262166;
pub const __INT_N_SIGN_POSN: C2RustUnnamed_4 = 262165;
pub const __INT_P_SIGN_POSN: C2RustUnnamed_4 = 262164;
pub const __INT_N_SEP_BY_SPACE: C2RustUnnamed_4 = 262163;
pub const __INT_N_CS_PRECEDES: C2RustUnnamed_4 = 262162;
pub const __INT_P_SEP_BY_SPACE: C2RustUnnamed_4 = 262161;
pub const __INT_P_CS_PRECEDES: C2RustUnnamed_4 = 262160;
pub const _NL_MONETARY_CRNCYSTR: C2RustUnnamed_4 = 262159;
pub const __N_SIGN_POSN: C2RustUnnamed_4 = 262158;
pub const __P_SIGN_POSN: C2RustUnnamed_4 = 262157;
pub const __N_SEP_BY_SPACE: C2RustUnnamed_4 = 262156;
pub const __N_CS_PRECEDES: C2RustUnnamed_4 = 262155;
pub const __P_SEP_BY_SPACE: C2RustUnnamed_4 = 262154;
pub const __P_CS_PRECEDES: C2RustUnnamed_4 = 262153;
pub const __FRAC_DIGITS: C2RustUnnamed_4 = 262152;
pub const __INT_FRAC_DIGITS: C2RustUnnamed_4 = 262151;
pub const __NEGATIVE_SIGN: C2RustUnnamed_4 = 262150;
pub const __POSITIVE_SIGN: C2RustUnnamed_4 = 262149;
pub const __MON_GROUPING: C2RustUnnamed_4 = 262148;
pub const __MON_THOUSANDS_SEP: C2RustUnnamed_4 = 262147;
pub const __MON_DECIMAL_POINT: C2RustUnnamed_4 = 262146;
pub const __CURRENCY_SYMBOL: C2RustUnnamed_4 = 262145;
pub const __INT_CURR_SYMBOL: C2RustUnnamed_4 = 262144;
pub const _NL_NUM_LC_CTYPE: C2RustUnnamed_4 = 86;
pub const _NL_CTYPE_EXTRA_MAP_14: C2RustUnnamed_4 = 85;
pub const _NL_CTYPE_EXTRA_MAP_13: C2RustUnnamed_4 = 84;
pub const _NL_CTYPE_EXTRA_MAP_12: C2RustUnnamed_4 = 83;
pub const _NL_CTYPE_EXTRA_MAP_11: C2RustUnnamed_4 = 82;
pub const _NL_CTYPE_EXTRA_MAP_10: C2RustUnnamed_4 = 81;
pub const _NL_CTYPE_EXTRA_MAP_9: C2RustUnnamed_4 = 80;
pub const _NL_CTYPE_EXTRA_MAP_8: C2RustUnnamed_4 = 79;
pub const _NL_CTYPE_EXTRA_MAP_7: C2RustUnnamed_4 = 78;
pub const _NL_CTYPE_EXTRA_MAP_6: C2RustUnnamed_4 = 77;
pub const _NL_CTYPE_EXTRA_MAP_5: C2RustUnnamed_4 = 76;
pub const _NL_CTYPE_EXTRA_MAP_4: C2RustUnnamed_4 = 75;
pub const _NL_CTYPE_EXTRA_MAP_3: C2RustUnnamed_4 = 74;
pub const _NL_CTYPE_EXTRA_MAP_2: C2RustUnnamed_4 = 73;
pub const _NL_CTYPE_EXTRA_MAP_1: C2RustUnnamed_4 = 72;
pub const _NL_CTYPE_NONASCII_CASE: C2RustUnnamed_4 = 71;
pub const _NL_CTYPE_MAP_TO_NONASCII: C2RustUnnamed_4 = 70;
pub const _NL_CTYPE_TRANSLIT_IGNORE: C2RustUnnamed_4 = 69;
pub const _NL_CTYPE_TRANSLIT_IGNORE_LEN: C2RustUnnamed_4 = 68;
pub const _NL_CTYPE_TRANSLIT_DEFAULT_MISSING: C2RustUnnamed_4 = 67;
pub const _NL_CTYPE_TRANSLIT_DEFAULT_MISSING_LEN: C2RustUnnamed_4 = 66;
pub const _NL_CTYPE_TRANSLIT_TO_TBL: C2RustUnnamed_4 = 65;
pub const _NL_CTYPE_TRANSLIT_TO_IDX: C2RustUnnamed_4 = 64;
pub const _NL_CTYPE_TRANSLIT_FROM_TBL: C2RustUnnamed_4 = 63;
pub const _NL_CTYPE_TRANSLIT_FROM_IDX: C2RustUnnamed_4 = 62;
pub const _NL_CTYPE_TRANSLIT_TAB_SIZE: C2RustUnnamed_4 = 61;
pub const _NL_CTYPE_OUTDIGIT9_WC: C2RustUnnamed_4 = 60;
pub const _NL_CTYPE_OUTDIGIT8_WC: C2RustUnnamed_4 = 59;
pub const _NL_CTYPE_OUTDIGIT7_WC: C2RustUnnamed_4 = 58;
pub const _NL_CTYPE_OUTDIGIT6_WC: C2RustUnnamed_4 = 57;
pub const _NL_CTYPE_OUTDIGIT5_WC: C2RustUnnamed_4 = 56;
pub const _NL_CTYPE_OUTDIGIT4_WC: C2RustUnnamed_4 = 55;
pub const _NL_CTYPE_OUTDIGIT3_WC: C2RustUnnamed_4 = 54;
pub const _NL_CTYPE_OUTDIGIT2_WC: C2RustUnnamed_4 = 53;
pub const _NL_CTYPE_OUTDIGIT1_WC: C2RustUnnamed_4 = 52;
pub const _NL_CTYPE_OUTDIGIT0_WC: C2RustUnnamed_4 = 51;
pub const _NL_CTYPE_OUTDIGIT9_MB: C2RustUnnamed_4 = 50;
pub const _NL_CTYPE_OUTDIGIT8_MB: C2RustUnnamed_4 = 49;
pub const _NL_CTYPE_OUTDIGIT7_MB: C2RustUnnamed_4 = 48;
pub const _NL_CTYPE_OUTDIGIT6_MB: C2RustUnnamed_4 = 47;
pub const _NL_CTYPE_OUTDIGIT5_MB: C2RustUnnamed_4 = 46;
pub const _NL_CTYPE_OUTDIGIT4_MB: C2RustUnnamed_4 = 45;
pub const _NL_CTYPE_OUTDIGIT3_MB: C2RustUnnamed_4 = 44;
pub const _NL_CTYPE_OUTDIGIT2_MB: C2RustUnnamed_4 = 43;
pub const _NL_CTYPE_OUTDIGIT1_MB: C2RustUnnamed_4 = 42;
pub const _NL_CTYPE_OUTDIGIT0_MB: C2RustUnnamed_4 = 41;
pub const _NL_CTYPE_INDIGITS9_WC: C2RustUnnamed_4 = 40;
pub const _NL_CTYPE_INDIGITS8_WC: C2RustUnnamed_4 = 39;
pub const _NL_CTYPE_INDIGITS7_WC: C2RustUnnamed_4 = 38;
pub const _NL_CTYPE_INDIGITS6_WC: C2RustUnnamed_4 = 37;
pub const _NL_CTYPE_INDIGITS5_WC: C2RustUnnamed_4 = 36;
pub const _NL_CTYPE_INDIGITS4_WC: C2RustUnnamed_4 = 35;
pub const _NL_CTYPE_INDIGITS3_WC: C2RustUnnamed_4 = 34;
pub const _NL_CTYPE_INDIGITS2_WC: C2RustUnnamed_4 = 33;
pub const _NL_CTYPE_INDIGITS1_WC: C2RustUnnamed_4 = 32;
pub const _NL_CTYPE_INDIGITS0_WC: C2RustUnnamed_4 = 31;
pub const _NL_CTYPE_INDIGITS_WC_LEN: C2RustUnnamed_4 = 30;
pub const _NL_CTYPE_INDIGITS9_MB: C2RustUnnamed_4 = 29;
pub const _NL_CTYPE_INDIGITS8_MB: C2RustUnnamed_4 = 28;
pub const _NL_CTYPE_INDIGITS7_MB: C2RustUnnamed_4 = 27;
pub const _NL_CTYPE_INDIGITS6_MB: C2RustUnnamed_4 = 26;
pub const _NL_CTYPE_INDIGITS5_MB: C2RustUnnamed_4 = 25;
pub const _NL_CTYPE_INDIGITS4_MB: C2RustUnnamed_4 = 24;
pub const _NL_CTYPE_INDIGITS3_MB: C2RustUnnamed_4 = 23;
pub const _NL_CTYPE_INDIGITS2_MB: C2RustUnnamed_4 = 22;
pub const _NL_CTYPE_INDIGITS1_MB: C2RustUnnamed_4 = 21;
pub const _NL_CTYPE_INDIGITS0_MB: C2RustUnnamed_4 = 20;
pub const _NL_CTYPE_INDIGITS_MB_LEN: C2RustUnnamed_4 = 19;
pub const _NL_CTYPE_MAP_OFFSET: C2RustUnnamed_4 = 18;
pub const _NL_CTYPE_CLASS_OFFSET: C2RustUnnamed_4 = 17;
pub const _NL_CTYPE_TOLOWER32: C2RustUnnamed_4 = 16;
pub const _NL_CTYPE_TOUPPER32: C2RustUnnamed_4 = 15;
pub const _NL_CTYPE_CODESET_NAME: C2RustUnnamed_4 = 14;
pub const _NL_CTYPE_MB_CUR_MAX: C2RustUnnamed_4 = 13;
pub const _NL_CTYPE_WIDTH: C2RustUnnamed_4 = 12;
pub const _NL_CTYPE_MAP_NAMES: C2RustUnnamed_4 = 11;
pub const _NL_CTYPE_CLASS_NAMES: C2RustUnnamed_4 = 10;
pub const _NL_CTYPE_GAP6: C2RustUnnamed_4 = 9;
pub const _NL_CTYPE_GAP5: C2RustUnnamed_4 = 8;
pub const _NL_CTYPE_GAP4: C2RustUnnamed_4 = 7;
pub const _NL_CTYPE_GAP3: C2RustUnnamed_4 = 6;
pub const _NL_CTYPE_CLASS32: C2RustUnnamed_4 = 5;
pub const _NL_CTYPE_GAP2: C2RustUnnamed_4 = 4;
pub const _NL_CTYPE_TOLOWER: C2RustUnnamed_4 = 3;
pub const _NL_CTYPE_GAP1: C2RustUnnamed_4 = 2;
pub const _NL_CTYPE_TOUPPER: C2RustUnnamed_4 = 1;
pub const _NL_CTYPE_CLASS: C2RustUnnamed_4 = 0;
pub const _NL_NUM_LC_COLLATE: C2RustUnnamed_4 = 196627;
pub const _NL_COLLATE_CODESET: C2RustUnnamed_4 = 196626;
pub const _NL_COLLATE_COLLSEQWC: C2RustUnnamed_4 = 196625;
pub const _NL_COLLATE_COLLSEQMB: C2RustUnnamed_4 = 196624;
pub const _NL_COLLATE_SYMB_EXTRAMB: C2RustUnnamed_4 = 196623;
pub const _NL_COLLATE_SYMB_TABLEMB: C2RustUnnamed_4 = 196622;
pub const _NL_COLLATE_SYMB_HASH_SIZEMB: C2RustUnnamed_4 = 196621;
pub const _NL_COLLATE_INDIRECTWC: C2RustUnnamed_4 = 196620;
pub const _NL_COLLATE_EXTRAWC: C2RustUnnamed_4 = 196619;
pub const _NL_COLLATE_WEIGHTWC: C2RustUnnamed_4 = 196618;
pub const _NL_COLLATE_TABLEWC: C2RustUnnamed_4 = 196617;
pub const _NL_COLLATE_GAP3: C2RustUnnamed_4 = 196616;
pub const _NL_COLLATE_GAP2: C2RustUnnamed_4 = 196615;
pub const _NL_COLLATE_GAP1: C2RustUnnamed_4 = 196614;
pub const _NL_COLLATE_INDIRECTMB: C2RustUnnamed_4 = 196613;
pub const _NL_COLLATE_EXTRAMB: C2RustUnnamed_4 = 196612;
pub const _NL_COLLATE_WEIGHTMB: C2RustUnnamed_4 = 196611;
pub const _NL_COLLATE_TABLEMB: C2RustUnnamed_4 = 196610;
pub const _NL_COLLATE_RULESETS: C2RustUnnamed_4 = 196609;
pub const _NL_COLLATE_NRULES: C2RustUnnamed_4 = 196608;
pub const _NL_NUM_LC_TIME: C2RustUnnamed_4 = 131231;
pub const _NL_WABALTMON_12: C2RustUnnamed_4 = 131230;
pub const _NL_WABALTMON_11: C2RustUnnamed_4 = 131229;
pub const _NL_WABALTMON_10: C2RustUnnamed_4 = 131228;
pub const _NL_WABALTMON_9: C2RustUnnamed_4 = 131227;
pub const _NL_WABALTMON_8: C2RustUnnamed_4 = 131226;
pub const _NL_WABALTMON_7: C2RustUnnamed_4 = 131225;
pub const _NL_WABALTMON_6: C2RustUnnamed_4 = 131224;
pub const _NL_WABALTMON_5: C2RustUnnamed_4 = 131223;
pub const _NL_WABALTMON_4: C2RustUnnamed_4 = 131222;
pub const _NL_WABALTMON_3: C2RustUnnamed_4 = 131221;
pub const _NL_WABALTMON_2: C2RustUnnamed_4 = 131220;
pub const _NL_WABALTMON_1: C2RustUnnamed_4 = 131219;
pub const _NL_ABALTMON_12: C2RustUnnamed_4 = 131218;
pub const _NL_ABALTMON_11: C2RustUnnamed_4 = 131217;
pub const _NL_ABALTMON_10: C2RustUnnamed_4 = 131216;
pub const _NL_ABALTMON_9: C2RustUnnamed_4 = 131215;
pub const _NL_ABALTMON_8: C2RustUnnamed_4 = 131214;
pub const _NL_ABALTMON_7: C2RustUnnamed_4 = 131213;
pub const _NL_ABALTMON_6: C2RustUnnamed_4 = 131212;
pub const _NL_ABALTMON_5: C2RustUnnamed_4 = 131211;
pub const _NL_ABALTMON_4: C2RustUnnamed_4 = 131210;
pub const _NL_ABALTMON_3: C2RustUnnamed_4 = 131209;
pub const _NL_ABALTMON_2: C2RustUnnamed_4 = 131208;
pub const _NL_ABALTMON_1: C2RustUnnamed_4 = 131207;
pub const _NL_WALTMON_12: C2RustUnnamed_4 = 131206;
pub const _NL_WALTMON_11: C2RustUnnamed_4 = 131205;
pub const _NL_WALTMON_10: C2RustUnnamed_4 = 131204;
pub const _NL_WALTMON_9: C2RustUnnamed_4 = 131203;
pub const _NL_WALTMON_8: C2RustUnnamed_4 = 131202;
pub const _NL_WALTMON_7: C2RustUnnamed_4 = 131201;
pub const _NL_WALTMON_6: C2RustUnnamed_4 = 131200;
pub const _NL_WALTMON_5: C2RustUnnamed_4 = 131199;
pub const _NL_WALTMON_4: C2RustUnnamed_4 = 131198;
pub const _NL_WALTMON_3: C2RustUnnamed_4 = 131197;
pub const _NL_WALTMON_2: C2RustUnnamed_4 = 131196;
pub const _NL_WALTMON_1: C2RustUnnamed_4 = 131195;
pub const __ALTMON_12: C2RustUnnamed_4 = 131194;
pub const __ALTMON_11: C2RustUnnamed_4 = 131193;
pub const __ALTMON_10: C2RustUnnamed_4 = 131192;
pub const __ALTMON_9: C2RustUnnamed_4 = 131191;
pub const __ALTMON_8: C2RustUnnamed_4 = 131190;
pub const __ALTMON_7: C2RustUnnamed_4 = 131189;
pub const __ALTMON_6: C2RustUnnamed_4 = 131188;
pub const __ALTMON_5: C2RustUnnamed_4 = 131187;
pub const __ALTMON_4: C2RustUnnamed_4 = 131186;
pub const __ALTMON_3: C2RustUnnamed_4 = 131185;
pub const __ALTMON_2: C2RustUnnamed_4 = 131184;
pub const __ALTMON_1: C2RustUnnamed_4 = 131183;
pub const _NL_TIME_CODESET: C2RustUnnamed_4 = 131182;
pub const _NL_W_DATE_FMT: C2RustUnnamed_4 = 131181;
pub const _DATE_FMT: C2RustUnnamed_4 = 131180;
pub const _NL_TIME_TIMEZONE: C2RustUnnamed_4 = 131179;
pub const _NL_TIME_CAL_DIRECTION: C2RustUnnamed_4 = 131178;
pub const _NL_TIME_FIRST_WORKDAY: C2RustUnnamed_4 = 131177;
pub const _NL_TIME_FIRST_WEEKDAY: C2RustUnnamed_4 = 131176;
pub const _NL_TIME_WEEK_1STWEEK: C2RustUnnamed_4 = 131175;
pub const _NL_TIME_WEEK_1STDAY: C2RustUnnamed_4 = 131174;
pub const _NL_TIME_WEEK_NDAYS: C2RustUnnamed_4 = 131173;
pub const _NL_WERA_T_FMT: C2RustUnnamed_4 = 131172;
pub const _NL_WERA_D_T_FMT: C2RustUnnamed_4 = 131171;
pub const _NL_WALT_DIGITS: C2RustUnnamed_4 = 131170;
pub const _NL_WERA_D_FMT: C2RustUnnamed_4 = 131169;
pub const _NL_WERA_YEAR: C2RustUnnamed_4 = 131168;
pub const _NL_WT_FMT_AMPM: C2RustUnnamed_4 = 131167;
pub const _NL_WT_FMT: C2RustUnnamed_4 = 131166;
pub const _NL_WD_FMT: C2RustUnnamed_4 = 131165;
pub const _NL_WD_T_FMT: C2RustUnnamed_4 = 131164;
pub const _NL_WPM_STR: C2RustUnnamed_4 = 131163;
pub const _NL_WAM_STR: C2RustUnnamed_4 = 131162;
pub const _NL_WMON_12: C2RustUnnamed_4 = 131161;
pub const _NL_WMON_11: C2RustUnnamed_4 = 131160;
pub const _NL_WMON_10: C2RustUnnamed_4 = 131159;
pub const _NL_WMON_9: C2RustUnnamed_4 = 131158;
pub const _NL_WMON_8: C2RustUnnamed_4 = 131157;
pub const _NL_WMON_7: C2RustUnnamed_4 = 131156;
pub const _NL_WMON_6: C2RustUnnamed_4 = 131155;
pub const _NL_WMON_5: C2RustUnnamed_4 = 131154;
pub const _NL_WMON_4: C2RustUnnamed_4 = 131153;
pub const _NL_WMON_3: C2RustUnnamed_4 = 131152;
pub const _NL_WMON_2: C2RustUnnamed_4 = 131151;
pub const _NL_WMON_1: C2RustUnnamed_4 = 131150;
pub const _NL_WABMON_12: C2RustUnnamed_4 = 131149;
pub const _NL_WABMON_11: C2RustUnnamed_4 = 131148;
pub const _NL_WABMON_10: C2RustUnnamed_4 = 131147;
pub const _NL_WABMON_9: C2RustUnnamed_4 = 131146;
pub const _NL_WABMON_8: C2RustUnnamed_4 = 131145;
pub const _NL_WABMON_7: C2RustUnnamed_4 = 131144;
pub const _NL_WABMON_6: C2RustUnnamed_4 = 131143;
pub const _NL_WABMON_5: C2RustUnnamed_4 = 131142;
pub const _NL_WABMON_4: C2RustUnnamed_4 = 131141;
pub const _NL_WABMON_3: C2RustUnnamed_4 = 131140;
pub const _NL_WABMON_2: C2RustUnnamed_4 = 131139;
pub const _NL_WABMON_1: C2RustUnnamed_4 = 131138;
pub const _NL_WDAY_7: C2RustUnnamed_4 = 131137;
pub const _NL_WDAY_6: C2RustUnnamed_4 = 131136;
pub const _NL_WDAY_5: C2RustUnnamed_4 = 131135;
pub const _NL_WDAY_4: C2RustUnnamed_4 = 131134;
pub const _NL_WDAY_3: C2RustUnnamed_4 = 131133;
pub const _NL_WDAY_2: C2RustUnnamed_4 = 131132;
pub const _NL_WDAY_1: C2RustUnnamed_4 = 131131;
pub const _NL_WABDAY_7: C2RustUnnamed_4 = 131130;
pub const _NL_WABDAY_6: C2RustUnnamed_4 = 131129;
pub const _NL_WABDAY_5: C2RustUnnamed_4 = 131128;
pub const _NL_WABDAY_4: C2RustUnnamed_4 = 131127;
pub const _NL_WABDAY_3: C2RustUnnamed_4 = 131126;
pub const _NL_WABDAY_2: C2RustUnnamed_4 = 131125;
pub const _NL_WABDAY_1: C2RustUnnamed_4 = 131124;
pub const _NL_TIME_ERA_ENTRIES: C2RustUnnamed_4 = 131123;
pub const _NL_TIME_ERA_NUM_ENTRIES: C2RustUnnamed_4 = 131122;
pub const ERA_T_FMT: C2RustUnnamed_4 = 131121;
pub const ERA_D_T_FMT: C2RustUnnamed_4 = 131120;
pub const ALT_DIGITS: C2RustUnnamed_4 = 131119;
pub const ERA_D_FMT: C2RustUnnamed_4 = 131118;
pub const __ERA_YEAR: C2RustUnnamed_4 = 131117;
pub const ERA: C2RustUnnamed_4 = 131116;
pub const T_FMT_AMPM: C2RustUnnamed_4 = 131115;
pub const T_FMT: C2RustUnnamed_4 = 131114;
pub const D_FMT: C2RustUnnamed_4 = 131113;
pub const D_T_FMT: C2RustUnnamed_4 = 131112;
pub const PM_STR: C2RustUnnamed_4 = 131111;
pub const AM_STR: C2RustUnnamed_4 = 131110;
pub const MON_12: C2RustUnnamed_4 = 131109;
pub const MON_11: C2RustUnnamed_4 = 131108;
pub const MON_10: C2RustUnnamed_4 = 131107;
pub const MON_9: C2RustUnnamed_4 = 131106;
pub const MON_8: C2RustUnnamed_4 = 131105;
pub const MON_7: C2RustUnnamed_4 = 131104;
pub const MON_6: C2RustUnnamed_4 = 131103;
pub const MON_5: C2RustUnnamed_4 = 131102;
pub const MON_4: C2RustUnnamed_4 = 131101;
pub const MON_3: C2RustUnnamed_4 = 131100;
pub const MON_2: C2RustUnnamed_4 = 131099;
pub const MON_1: C2RustUnnamed_4 = 131098;
pub const ABMON_12: C2RustUnnamed_4 = 131097;
pub const ABMON_11: C2RustUnnamed_4 = 131096;
pub const ABMON_10: C2RustUnnamed_4 = 131095;
pub const ABMON_9: C2RustUnnamed_4 = 131094;
pub const ABMON_8: C2RustUnnamed_4 = 131093;
pub const ABMON_7: C2RustUnnamed_4 = 131092;
pub const ABMON_6: C2RustUnnamed_4 = 131091;
pub const ABMON_5: C2RustUnnamed_4 = 131090;
pub const ABMON_4: C2RustUnnamed_4 = 131089;
pub const ABMON_3: C2RustUnnamed_4 = 131088;
pub const ABMON_2: C2RustUnnamed_4 = 131087;
pub const ABMON_1: C2RustUnnamed_4 = 131086;
pub const DAY_7: C2RustUnnamed_4 = 131085;
pub const DAY_6: C2RustUnnamed_4 = 131084;
pub const DAY_5: C2RustUnnamed_4 = 131083;
pub const DAY_4: C2RustUnnamed_4 = 131082;
pub const DAY_3: C2RustUnnamed_4 = 131081;
pub const DAY_2: C2RustUnnamed_4 = 131080;
pub const DAY_1: C2RustUnnamed_4 = 131079;
pub const ABDAY_7: C2RustUnnamed_4 = 131078;
pub const ABDAY_6: C2RustUnnamed_4 = 131077;
pub const ABDAY_5: C2RustUnnamed_4 = 131076;
pub const ABDAY_4: C2RustUnnamed_4 = 131075;
pub const ABDAY_3: C2RustUnnamed_4 = 131074;
pub const ABDAY_2: C2RustUnnamed_4 = 131073;
pub const ABDAY_1: C2RustUnnamed_4 = 131072;
pub type C2RustUnnamed_5 = libc::c_uint;
pub type C2RustUnnamed_6 = libc::c_uint;
pub type C2RustUnnamed_7 = libc::c_uint;
pub const PE_IPV6_NOT_SUPPORTED: C2RustUnnamed_7 = 9;
pub const PE_NO_ERROR: C2RustUnnamed_7 = 0;
pub type C2RustUnnamed_8 = libc::c_uint;
#[inline]
unsafe extern "C" fn stat(
    mut __path: *const libc::c_char,
    mut __statbuf: *mut stat,
) -> libc::c_int {
    return __xstat(1 as libc::c_int, __path, __statbuf);
}
#[inline]
unsafe extern "C" fn c_isalnum(mut c: libc::c_int) -> bool {
    match c {
        48 | 49 | 50 | 51 | 52 | 53 | 54 | 55 | 56 | 57 | 97 | 98 | 99 | 100 | 101 | 102
        | 103 | 104 | 105 | 106 | 107 | 108 | 109 | 110 | 111 | 112 | 113 | 114 | 115
        | 116 | 117 | 118 | 119 | 120 | 121 | 122 | 65 | 66 | 67 | 68 | 69 | 70 | 71 | 72
        | 73 | 74 | 75 | 76 | 77 | 78 | 79 | 80 | 81 | 82 | 83 | 84 | 85 | 86 | 87 | 88
        | 89 | 90 => return 1 as libc::c_int != 0,
        _ => return 0 as libc::c_int != 0,
    };
}
#[inline]
unsafe extern "C" fn c_isascii(mut c: libc::c_int) -> bool {
    match c {
        32 | 7 | 8 | 12 | 10 | 13 | 9 | 11 | 0 | 1 | 2 | 3 | 4 | 5 | 6 | 14 | 15 | 16
        | 17 | 18 | 19 | 20 | 21 | 22 | 23 | 24 | 25 | 26 | 27 | 28 | 29 | 30 | 31 | 127
        | 48 | 49 | 50 | 51 | 52 | 53 | 54 | 55 | 56 | 57 | 97 | 98 | 99 | 100 | 101
        | 102 | 103 | 104 | 105 | 106 | 107 | 108 | 109 | 110 | 111 | 112 | 113 | 114
        | 115 | 116 | 117 | 118 | 119 | 120 | 121 | 122 | 33 | 34 | 35 | 36 | 37 | 38
        | 39 | 40 | 41 | 42 | 43 | 44 | 45 | 46 | 47 | 58 | 59 | 60 | 61 | 62 | 63 | 64
        | 91 | 92 | 93 | 94 | 95 | 96 | 123 | 124 | 125 | 126 | 65 | 66 | 67 | 68 | 69
        | 70 | 71 | 72 | 73 | 74 | 75 | 76 | 77 | 78 | 79 | 80 | 81 | 82 | 83 | 84 | 85
        | 86 | 87 | 88 | 89 | 90 => return 1 as libc::c_int != 0,
        _ => return 0 as libc::c_int != 0,
    };
}
#[inline]
unsafe extern "C" fn c_iscntrl(mut c: libc::c_int) -> bool {
    match c {
        7 | 8 | 12 | 10 | 13 | 9 | 11 | 0 | 1 | 2 | 3 | 4 | 5 | 6 | 14 | 15 | 16 | 17
        | 18 | 19 | 20 | 21 | 22 | 23 | 24 | 25 | 26 | 27 | 28 | 29 | 30 | 31 | 127 => {
            return 1 as libc::c_int != 0;
        }
        _ => return 0 as libc::c_int != 0,
    };
}
#[inline]
unsafe extern "C" fn c_isdigit(mut c: libc::c_int) -> bool {
    match c {
        48 | 49 | 50 | 51 | 52 | 53 | 54 | 55 | 56 | 57 => return 1 as libc::c_int != 0,
        _ => return 0 as libc::c_int != 0,
    };
}
#[inline]
unsafe extern "C" fn c_isupper(mut c: libc::c_int) -> bool {
    match c {
        65 | 66 | 67 | 68 | 69 | 70 | 71 | 72 | 73 | 74 | 75 | 76 | 77 | 78 | 79 | 80
        | 81 | 82 | 83 | 84 | 85 | 86 | 87 | 88 | 89 | 90 => return 1 as libc::c_int != 0,
        _ => return 0 as libc::c_int != 0,
    };
}
#[inline]
unsafe extern "C" fn c_isxdigit(mut c: libc::c_int) -> bool {
    match c {
        48 | 49 | 50 | 51 | 52 | 53 | 54 | 55 | 56 | 57 | 97 | 98 | 99 | 100 | 101 | 102
        | 65 | 66 | 67 | 68 | 69 | 70 => return 1 as libc::c_int != 0,
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
unsafe extern "C" fn _unhex(mut c: libc::c_uchar) -> libc::c_uchar {
    return (if c as libc::c_int <= '9' as i32 {
        c as libc::c_int - '0' as i32
    } else if c as libc::c_int <= 'F' as i32 {
        c as libc::c_int - 'A' as i32 + 10 as libc::c_int
    } else {
        c as libc::c_int - 'a' as i32 + 10 as libc::c_int
    }) as libc::c_uchar;
}
static mut supported_schemes: [scheme_data; 3] = [
    {
        let mut init = scheme_data {
            name: b"http\0" as *const u8 as *const libc::c_char,
            leading_string: b"http://\0" as *const u8 as *const libc::c_char,
            default_port: 80 as libc::c_int,
            flags: scm_has_query as libc::c_int | scm_has_fragment as libc::c_int,
        };
        init
    },
    {
        let mut init = scheme_data {
            name: b"ftp\0" as *const u8 as *const libc::c_char,
            leading_string: b"ftp://\0" as *const u8 as *const libc::c_char,
            default_port: 21 as libc::c_int,
            flags: scm_has_params as libc::c_int | scm_has_fragment as libc::c_int,
        };
        init
    },
    {
        let mut init = scheme_data {
            name: 0 as *const libc::c_char,
            leading_string: 0 as *const libc::c_char,
            default_port: -(1 as libc::c_int),
            flags: 0 as libc::c_int,
        };
        init
    },
];
static mut urlchr_table: [libc::c_uchar; 256] = [
    urlchr_unsafe as libc::c_int as libc::c_uchar,
    urlchr_unsafe as libc::c_int as libc::c_uchar,
    urlchr_unsafe as libc::c_int as libc::c_uchar,
    urlchr_unsafe as libc::c_int as libc::c_uchar,
    urlchr_unsafe as libc::c_int as libc::c_uchar,
    urlchr_unsafe as libc::c_int as libc::c_uchar,
    urlchr_unsafe as libc::c_int as libc::c_uchar,
    urlchr_unsafe as libc::c_int as libc::c_uchar,
    urlchr_unsafe as libc::c_int as libc::c_uchar,
    urlchr_unsafe as libc::c_int as libc::c_uchar,
    urlchr_unsafe as libc::c_int as libc::c_uchar,
    urlchr_unsafe as libc::c_int as libc::c_uchar,
    urlchr_unsafe as libc::c_int as libc::c_uchar,
    urlchr_unsafe as libc::c_int as libc::c_uchar,
    urlchr_unsafe as libc::c_int as libc::c_uchar,
    urlchr_unsafe as libc::c_int as libc::c_uchar,
    urlchr_unsafe as libc::c_int as libc::c_uchar,
    urlchr_unsafe as libc::c_int as libc::c_uchar,
    urlchr_unsafe as libc::c_int as libc::c_uchar,
    urlchr_unsafe as libc::c_int as libc::c_uchar,
    urlchr_unsafe as libc::c_int as libc::c_uchar,
    urlchr_unsafe as libc::c_int as libc::c_uchar,
    urlchr_unsafe as libc::c_int as libc::c_uchar,
    urlchr_unsafe as libc::c_int as libc::c_uchar,
    urlchr_unsafe as libc::c_int as libc::c_uchar,
    urlchr_unsafe as libc::c_int as libc::c_uchar,
    urlchr_unsafe as libc::c_int as libc::c_uchar,
    urlchr_unsafe as libc::c_int as libc::c_uchar,
    urlchr_unsafe as libc::c_int as libc::c_uchar,
    urlchr_unsafe as libc::c_int as libc::c_uchar,
    urlchr_unsafe as libc::c_int as libc::c_uchar,
    urlchr_unsafe as libc::c_int as libc::c_uchar,
    urlchr_unsafe as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    urlchr_unsafe as libc::c_int as libc::c_uchar,
    (urlchr_reserved as libc::c_int | urlchr_unsafe as libc::c_int) as libc::c_uchar,
    urlchr_reserved as libc::c_int as libc::c_uchar,
    urlchr_unsafe as libc::c_int as libc::c_uchar,
    urlchr_reserved as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    urlchr_reserved as libc::c_int as libc::c_uchar,
    urlchr_reserved as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    urlchr_reserved as libc::c_int as libc::c_uchar,
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
    (urlchr_reserved as libc::c_int | urlchr_unsafe as libc::c_int) as libc::c_uchar,
    urlchr_reserved as libc::c_int as libc::c_uchar,
    urlchr_unsafe as libc::c_int as libc::c_uchar,
    urlchr_reserved as libc::c_int as libc::c_uchar,
    urlchr_unsafe as libc::c_int as libc::c_uchar,
    urlchr_reserved as libc::c_int as libc::c_uchar,
    (urlchr_reserved as libc::c_int | urlchr_unsafe as libc::c_int) as libc::c_uchar,
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
    (urlchr_reserved as libc::c_int | urlchr_unsafe as libc::c_int) as libc::c_uchar,
    urlchr_unsafe as libc::c_int as libc::c_uchar,
    (urlchr_reserved as libc::c_int | urlchr_unsafe as libc::c_int) as libc::c_uchar,
    urlchr_unsafe as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    urlchr_unsafe as libc::c_int as libc::c_uchar,
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
    urlchr_unsafe as libc::c_int as libc::c_uchar,
    urlchr_unsafe as libc::c_int as libc::c_uchar,
    urlchr_unsafe as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    urlchr_unsafe as libc::c_int as libc::c_uchar,
    urlchr_unsafe as libc::c_int as libc::c_uchar,
    urlchr_unsafe as libc::c_int as libc::c_uchar,
    urlchr_unsafe as libc::c_int as libc::c_uchar,
    urlchr_unsafe as libc::c_int as libc::c_uchar,
    urlchr_unsafe as libc::c_int as libc::c_uchar,
    urlchr_unsafe as libc::c_int as libc::c_uchar,
    urlchr_unsafe as libc::c_int as libc::c_uchar,
    urlchr_unsafe as libc::c_int as libc::c_uchar,
    urlchr_unsafe as libc::c_int as libc::c_uchar,
    urlchr_unsafe as libc::c_int as libc::c_uchar,
    urlchr_unsafe as libc::c_int as libc::c_uchar,
    urlchr_unsafe as libc::c_int as libc::c_uchar,
    urlchr_unsafe as libc::c_int as libc::c_uchar,
    urlchr_unsafe as libc::c_int as libc::c_uchar,
    urlchr_unsafe as libc::c_int as libc::c_uchar,
    urlchr_unsafe as libc::c_int as libc::c_uchar,
    urlchr_unsafe as libc::c_int as libc::c_uchar,
    urlchr_unsafe as libc::c_int as libc::c_uchar,
    urlchr_unsafe as libc::c_int as libc::c_uchar,
    urlchr_unsafe as libc::c_int as libc::c_uchar,
    urlchr_unsafe as libc::c_int as libc::c_uchar,
    urlchr_unsafe as libc::c_int as libc::c_uchar,
    urlchr_unsafe as libc::c_int as libc::c_uchar,
    urlchr_unsafe as libc::c_int as libc::c_uchar,
    urlchr_unsafe as libc::c_int as libc::c_uchar,
    urlchr_unsafe as libc::c_int as libc::c_uchar,
    urlchr_unsafe as libc::c_int as libc::c_uchar,
    urlchr_unsafe as libc::c_int as libc::c_uchar,
    urlchr_unsafe as libc::c_int as libc::c_uchar,
    urlchr_unsafe as libc::c_int as libc::c_uchar,
    urlchr_unsafe as libc::c_int as libc::c_uchar,
    urlchr_unsafe as libc::c_int as libc::c_uchar,
    urlchr_unsafe as libc::c_int as libc::c_uchar,
    urlchr_unsafe as libc::c_int as libc::c_uchar,
    urlchr_unsafe as libc::c_int as libc::c_uchar,
    urlchr_unsafe as libc::c_int as libc::c_uchar,
    urlchr_unsafe as libc::c_int as libc::c_uchar,
    urlchr_unsafe as libc::c_int as libc::c_uchar,
    urlchr_unsafe as libc::c_int as libc::c_uchar,
    urlchr_unsafe as libc::c_int as libc::c_uchar,
    urlchr_unsafe as libc::c_int as libc::c_uchar,
    urlchr_unsafe as libc::c_int as libc::c_uchar,
    urlchr_unsafe as libc::c_int as libc::c_uchar,
    urlchr_unsafe as libc::c_int as libc::c_uchar,
    urlchr_unsafe as libc::c_int as libc::c_uchar,
    urlchr_unsafe as libc::c_int as libc::c_uchar,
    urlchr_unsafe as libc::c_int as libc::c_uchar,
    urlchr_unsafe as libc::c_int as libc::c_uchar,
    urlchr_unsafe as libc::c_int as libc::c_uchar,
    urlchr_unsafe as libc::c_int as libc::c_uchar,
    urlchr_unsafe as libc::c_int as libc::c_uchar,
    urlchr_unsafe as libc::c_int as libc::c_uchar,
    urlchr_unsafe as libc::c_int as libc::c_uchar,
    urlchr_unsafe as libc::c_int as libc::c_uchar,
    urlchr_unsafe as libc::c_int as libc::c_uchar,
    urlchr_unsafe as libc::c_int as libc::c_uchar,
    urlchr_unsafe as libc::c_int as libc::c_uchar,
    urlchr_unsafe as libc::c_int as libc::c_uchar,
    urlchr_unsafe as libc::c_int as libc::c_uchar,
    urlchr_unsafe as libc::c_int as libc::c_uchar,
    urlchr_unsafe as libc::c_int as libc::c_uchar,
    urlchr_unsafe as libc::c_int as libc::c_uchar,
    urlchr_unsafe as libc::c_int as libc::c_uchar,
    urlchr_unsafe as libc::c_int as libc::c_uchar,
    urlchr_unsafe as libc::c_int as libc::c_uchar,
    urlchr_unsafe as libc::c_int as libc::c_uchar,
    urlchr_unsafe as libc::c_int as libc::c_uchar,
    urlchr_unsafe as libc::c_int as libc::c_uchar,
    urlchr_unsafe as libc::c_int as libc::c_uchar,
    urlchr_unsafe as libc::c_int as libc::c_uchar,
    urlchr_unsafe as libc::c_int as libc::c_uchar,
    urlchr_unsafe as libc::c_int as libc::c_uchar,
    urlchr_unsafe as libc::c_int as libc::c_uchar,
    urlchr_unsafe as libc::c_int as libc::c_uchar,
    urlchr_unsafe as libc::c_int as libc::c_uchar,
    urlchr_unsafe as libc::c_int as libc::c_uchar,
    urlchr_unsafe as libc::c_int as libc::c_uchar,
    urlchr_unsafe as libc::c_int as libc::c_uchar,
    urlchr_unsafe as libc::c_int as libc::c_uchar,
    urlchr_unsafe as libc::c_int as libc::c_uchar,
    urlchr_unsafe as libc::c_int as libc::c_uchar,
    urlchr_unsafe as libc::c_int as libc::c_uchar,
    urlchr_unsafe as libc::c_int as libc::c_uchar,
    urlchr_unsafe as libc::c_int as libc::c_uchar,
    urlchr_unsafe as libc::c_int as libc::c_uchar,
    urlchr_unsafe as libc::c_int as libc::c_uchar,
    urlchr_unsafe as libc::c_int as libc::c_uchar,
    urlchr_unsafe as libc::c_int as libc::c_uchar,
    urlchr_unsafe as libc::c_int as libc::c_uchar,
    urlchr_unsafe as libc::c_int as libc::c_uchar,
    urlchr_unsafe as libc::c_int as libc::c_uchar,
    urlchr_unsafe as libc::c_int as libc::c_uchar,
    urlchr_unsafe as libc::c_int as libc::c_uchar,
    urlchr_unsafe as libc::c_int as libc::c_uchar,
    urlchr_unsafe as libc::c_int as libc::c_uchar,
    urlchr_unsafe as libc::c_int as libc::c_uchar,
    urlchr_unsafe as libc::c_int as libc::c_uchar,
    urlchr_unsafe as libc::c_int as libc::c_uchar,
    urlchr_unsafe as libc::c_int as libc::c_uchar,
    urlchr_unsafe as libc::c_int as libc::c_uchar,
    urlchr_unsafe as libc::c_int as libc::c_uchar,
    urlchr_unsafe as libc::c_int as libc::c_uchar,
    urlchr_unsafe as libc::c_int as libc::c_uchar,
    urlchr_unsafe as libc::c_int as libc::c_uchar,
    urlchr_unsafe as libc::c_int as libc::c_uchar,
    urlchr_unsafe as libc::c_int as libc::c_uchar,
    urlchr_unsafe as libc::c_int as libc::c_uchar,
    urlchr_unsafe as libc::c_int as libc::c_uchar,
    urlchr_unsafe as libc::c_int as libc::c_uchar,
    urlchr_unsafe as libc::c_int as libc::c_uchar,
    urlchr_unsafe as libc::c_int as libc::c_uchar,
    urlchr_unsafe as libc::c_int as libc::c_uchar,
    urlchr_unsafe as libc::c_int as libc::c_uchar,
    urlchr_unsafe as libc::c_int as libc::c_uchar,
    urlchr_unsafe as libc::c_int as libc::c_uchar,
    urlchr_unsafe as libc::c_int as libc::c_uchar,
    urlchr_unsafe as libc::c_int as libc::c_uchar,
    urlchr_unsafe as libc::c_int as libc::c_uchar,
    urlchr_unsafe as libc::c_int as libc::c_uchar,
    urlchr_unsafe as libc::c_int as libc::c_uchar,
    urlchr_unsafe as libc::c_int as libc::c_uchar,
    urlchr_unsafe as libc::c_int as libc::c_uchar,
    urlchr_unsafe as libc::c_int as libc::c_uchar,
    urlchr_unsafe as libc::c_int as libc::c_uchar,
    urlchr_unsafe as libc::c_int as libc::c_uchar,
    urlchr_unsafe as libc::c_int as libc::c_uchar,
    urlchr_unsafe as libc::c_int as libc::c_uchar,
    urlchr_unsafe as libc::c_int as libc::c_uchar,
];
unsafe extern "C" fn url_unescape_1(mut s: *mut libc::c_char, mut mask: libc::c_uchar) {
    let mut current_block: u64;
    let mut t: *mut libc::c_uchar = s as *mut libc::c_uchar;
    let mut h: *mut libc::c_uchar = s as *mut libc::c_uchar;
    while *h != 0 {
        if *h as libc::c_int != '%' as i32 {
            current_block = 1440162514974828797;
        } else {
            let mut c: libc::c_uchar = 0;
            if *h.offset(1 as libc::c_int as isize) == 0
                || *h.offset(2 as libc::c_int as isize) == 0
                || !(c_isxdigit(*h.offset(1 as libc::c_int as isize) as libc::c_int)
                    as libc::c_int != 0
                    && c_isxdigit(*h.offset(2 as libc::c_int as isize) as libc::c_int)
                        as libc::c_int != 0)
            {
                current_block = 1440162514974828797;
            } else {
                c = (((_unhex(*h.offset(1 as libc::c_int as isize)) as libc::c_int)
                    << 4 as libc::c_int)
                    + _unhex(*h.offset(2 as libc::c_int as isize)) as libc::c_int)
                    as libc::c_uchar;
                if urlchr_table[c as usize] as libc::c_int & mask as libc::c_int != 0 {
                    current_block = 1440162514974828797;
                } else if c as libc::c_int == '\0' as i32 {
                    current_block = 1440162514974828797;
                } else {
                    *t = c;
                    h = h.offset(2 as libc::c_int as isize);
                    current_block = 16668937799742929182;
                }
            }
        }
        match current_block {
            1440162514974828797 => {
                *t = *h;
            }
            _ => {}
        }
        h = h.offset(1);
        h;
        t = t.offset(1);
        t;
    }
    *t = '\0' as i32 as libc::c_uchar;
}
pub unsafe extern "C" fn url_unescape(mut s: *mut libc::c_char) {
    url_unescape_1(s, 0 as libc::c_int as libc::c_uchar);
}
pub unsafe extern "C" fn url_unescape_except_reserved(mut s: *mut libc::c_char) {
    url_unescape_1(s, urlchr_reserved as libc::c_int as libc::c_uchar);
}
unsafe extern "C" fn url_escape_1(
    mut s: *const libc::c_char,
    mut mask: libc::c_uchar,
    mut allow_passthrough: bool,
) -> *mut libc::c_char {
    let mut p1: *const libc::c_char = 0 as *const libc::c_char;
    let mut p2: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut newstr: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut newlen: libc::c_int = 0;
    let mut addition: libc::c_int = 0 as libc::c_int;
    p1 = s;
    while *p1 != 0 {
        if urlchr_table[*p1 as libc::c_uchar as usize] as libc::c_int
            & mask as libc::c_int != 0
        {
            addition += 2 as libc::c_int;
        }
        p1 = p1.offset(1);
        p1;
    }
    if addition == 0 {
        return if allow_passthrough as libc::c_int != 0 {
            s as *mut libc::c_char
        } else {
            xstrdup(s)
        };
    }
    newlen = (p1.offset_from(s) as libc::c_long + addition as libc::c_long)
        as libc::c_int;
    newstr = xmalloc((newlen + 1 as libc::c_int) as size_t) as *mut libc::c_char;
    p1 = s;
    p2 = newstr;
    while *p1 != 0 {
        if urlchr_table[*p1 as libc::c_uchar as usize] as libc::c_int
            & mask as libc::c_int != 0
        {
            let fresh0 = p1;
            p1 = p1.offset(1);
            let mut c: libc::c_uchar = *fresh0 as libc::c_uchar;
            let fresh1 = p2;
            p2 = p2.offset(1);
            *fresh1 = '%' as i32 as libc::c_char;
            let fresh2 = p2;
            p2 = p2.offset(1);
            *fresh2 = ((*::std::mem::transmute::<
                &[u8; 17],
                &[libc::c_char; 17],
            >(b"0123456789ABCDEF\0"))[(c as libc::c_int >> 4 as libc::c_int) as usize]
                as libc::c_int + 0 as libc::c_int) as libc::c_char;
            let fresh3 = p2;
            p2 = p2.offset(1);
            *fresh3 = ((*::std::mem::transmute::<
                &[u8; 17],
                &[libc::c_char; 17],
            >(b"0123456789ABCDEF\0"))[(c as libc::c_int & 0xf as libc::c_int) as usize]
                as libc::c_int + 0 as libc::c_int) as libc::c_char;
        } else {
            let fresh4 = p1;
            p1 = p1.offset(1);
            let fresh5 = p2;
            p2 = p2.offset(1);
            *fresh5 = *fresh4;
        }
    }
    *p2 = '\0' as i32 as libc::c_char;
    return newstr;
}
pub unsafe extern "C" fn url_escape(mut s: *const libc::c_char) -> *mut libc::c_char {
    return url_escape_1(
        s,
        urlchr_unsafe as libc::c_int as libc::c_uchar,
        0 as libc::c_int != 0,
    );
}
pub unsafe extern "C" fn url_escape_unsafe_and_reserved(
    mut s: *const libc::c_char,
) -> *mut libc::c_char {
    return url_escape_1(
        s,
        (urlchr_unsafe as libc::c_int | urlchr_reserved as libc::c_int) as libc::c_uchar,
        0 as libc::c_int != 0,
    );
}
unsafe extern "C" fn url_escape_allow_passthrough(
    mut s: *const libc::c_char,
) -> *mut libc::c_char {
    return url_escape_1(
        s,
        urlchr_unsafe as libc::c_int as libc::c_uchar,
        1 as libc::c_int != 0,
    );
}
#[inline]
unsafe extern "C" fn char_needs_escaping(mut p: *const libc::c_char) -> bool {
    if *p as libc::c_int == '%' as i32 {
        if c_isxdigit(*p.offset(1 as libc::c_int as isize) as libc::c_int) as libc::c_int
            != 0
            && c_isxdigit(*p.offset(2 as libc::c_int as isize) as libc::c_int)
                as libc::c_int != 0
        {
            return 0 as libc::c_int != 0
        } else {
            return 1 as libc::c_int != 0
        }
    } else if urlchr_table[*p as libc::c_uchar as usize] as libc::c_int
        & urlchr_unsafe as libc::c_int != 0
        && urlchr_table[*p as libc::c_uchar as usize] as libc::c_int
            & urlchr_reserved as libc::c_int == 0
    {
        return 1 as libc::c_int != 0
    } else {
        return 0 as libc::c_int != 0
    };
}
unsafe extern "C" fn reencode_escapes(mut s: *const libc::c_char) -> *mut libc::c_char {
    let mut p1: *const libc::c_char = 0 as *const libc::c_char;
    let mut newstr: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut p2: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut oldlen: libc::c_int = 0;
    let mut newlen: libc::c_int = 0;
    let mut encode_count: libc::c_int = 0 as libc::c_int;
    p1 = s;
    while *p1 != 0 {
        if char_needs_escaping(p1) {
            encode_count += 1;
            encode_count;
        }
        p1 = p1.offset(1);
        p1;
    }
    if encode_count == 0 {
        return s as *mut libc::c_char;
    }
    oldlen = p1.offset_from(s) as libc::c_long as libc::c_int;
    newlen = oldlen + 2 as libc::c_int * encode_count;
    newstr = xmalloc((newlen + 1 as libc::c_int) as size_t) as *mut libc::c_char;
    p1 = s;
    p2 = newstr;
    while *p1 != 0 {
        if char_needs_escaping(p1) {
            let fresh6 = p1;
            p1 = p1.offset(1);
            let mut c: libc::c_uchar = *fresh6 as libc::c_uchar;
            let fresh7 = p2;
            p2 = p2.offset(1);
            *fresh7 = '%' as i32 as libc::c_char;
            let fresh8 = p2;
            p2 = p2.offset(1);
            *fresh8 = ((*::std::mem::transmute::<
                &[u8; 17],
                &[libc::c_char; 17],
            >(b"0123456789ABCDEF\0"))[(c as libc::c_int >> 4 as libc::c_int) as usize]
                as libc::c_int + 0 as libc::c_int) as libc::c_char;
            let fresh9 = p2;
            p2 = p2.offset(1);
            *fresh9 = ((*::std::mem::transmute::<
                &[u8; 17],
                &[libc::c_char; 17],
            >(b"0123456789ABCDEF\0"))[(c as libc::c_int & 0xf as libc::c_int) as usize]
                as libc::c_int + 0 as libc::c_int) as libc::c_char;
        } else {
            let fresh10 = p1;
            p1 = p1.offset(1);
            let fresh11 = p2;
            p2 = p2.offset(1);
            *fresh11 = *fresh10;
        }
    }
    *p2 = '\0' as i32 as libc::c_char;
    return newstr;
}
pub unsafe extern "C" fn url_scheme(mut url: *const libc::c_char) -> url_scheme {
    let mut i: libc::c_int = 0;
    i = 0 as libc::c_int;
    while !(supported_schemes[i as usize].leading_string).is_null() {
        if 0 as libc::c_int
            == c_strncasecmp(
                url,
                supported_schemes[i as usize].leading_string,
                strlen(supported_schemes[i as usize].leading_string),
            )
        {
            if supported_schemes[i as usize].flags & scm_disabled as libc::c_int == 0 {
                return i as url_scheme
            } else {
                return SCHEME_INVALID
            }
        }
        i += 1;
        i;
    }
    return SCHEME_INVALID;
}
pub unsafe extern "C" fn url_has_scheme(mut url: *const libc::c_char) -> bool {
    let mut p: *const libc::c_char = url;
    if *p == 0
        || !(c_isalnum(*p as libc::c_int) as libc::c_int != 0
            || *p as libc::c_int == '-' as i32 || *p as libc::c_int == '+' as i32)
    {
        return 0 as libc::c_int != 0;
    }
    p = p.offset(1);
    p;
    while *p as libc::c_int != 0
        && (c_isalnum(*p as libc::c_int) as libc::c_int != 0
            || *p as libc::c_int == '-' as i32 || *p as libc::c_int == '+' as i32)
    {
        p = p.offset(1);
        p;
    }
    return *p as libc::c_int == ':' as i32;
}
pub unsafe extern "C" fn url_valid_scheme(mut url: *const libc::c_char) -> bool {
    let mut scheme: url_scheme = url_scheme(url);
    return scheme as libc::c_uint != SCHEME_INVALID as libc::c_int as libc::c_uint;
}
pub unsafe extern "C" fn scheme_default_port(mut scheme: url_scheme) -> libc::c_int {
    return supported_schemes[scheme as usize].default_port;
}
pub unsafe extern "C" fn scheme_disable(mut scheme: url_scheme) {
    supported_schemes[scheme as usize].flags |= scm_disabled as libc::c_int;
}
pub unsafe extern "C" fn scheme_leading_string(
    mut scheme: url_scheme,
) -> *const libc::c_char {
    return supported_schemes[scheme as usize].leading_string;
}
unsafe extern "C" fn url_skip_credentials(
    mut url: *const libc::c_char,
) -> *const libc::c_char {
    let mut p: *const libc::c_char = strpbrk(
        url,
        b"@/?#;\0" as *const u8 as *const libc::c_char,
    ) as *const libc::c_char;
    if p.is_null() || *p as libc::c_int != '@' as i32 {
        return url;
    }
    return p.offset(1 as libc::c_int as isize);
}
unsafe extern "C" fn parse_credentials(
    mut beg: *const libc::c_char,
    mut end: *const libc::c_char,
    mut user: *mut *mut libc::c_char,
    mut passwd: *mut *mut libc::c_char,
) -> bool {
    let mut colon: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut userend: *const libc::c_char = 0 as *const libc::c_char;
    if beg == end {
        return 0 as libc::c_int != 0;
    }
    colon = memchr(
        beg as *const libc::c_void,
        ':' as i32,
        end.offset_from(beg) as libc::c_long as libc::c_ulong,
    ) as *mut libc::c_char;
    if colon == beg as *mut libc::c_char {
        return 0 as libc::c_int != 0;
    }
    if !colon.is_null() {
        *passwd = strdupdelim(colon.offset(1 as libc::c_int as isize), end);
        userend = colon;
        url_unescape(*passwd);
    } else {
        *passwd = 0 as *mut libc::c_char;
        userend = end;
    }
    *user = strdupdelim(beg, userend);
    url_unescape(*user);
    return 1 as libc::c_int != 0;
}
pub unsafe extern "C" fn rewrite_shorthand_url(
    mut url: *const libc::c_char,
) -> *mut libc::c_char {
    let mut p: *const libc::c_char = 0 as *const libc::c_char;
    let mut ret: *mut libc::c_char = 0 as *mut libc::c_char;
    if url_scheme(url) as libc::c_uint != SCHEME_INVALID as libc::c_int as libc::c_uint {
        return 0 as *mut libc::c_char;
    }
    p = strpbrk(url, b":/\0" as *const u8 as *const libc::c_char);
    if p == url {
        return 0 as *mut libc::c_char;
    }
    if !p.is_null() && *p.offset(0 as libc::c_int as isize) as libc::c_int == ':' as i32
        && *p.offset(1 as libc::c_int as isize) as libc::c_int == '/' as i32
        && *p.offset(2 as libc::c_int as isize) as libc::c_int == '/' as i32
    {
        return 0 as *mut libc::c_char;
    }
    let mut current_block_9: u64;
    if !p.is_null() && *p as libc::c_int == ':' as i32 {
        let mut digits: libc::c_int = strspn(
            p.offset(1 as libc::c_int as isize),
            b"0123456789\0" as *const u8 as *const libc::c_char,
        ) as libc::c_int;
        if digits != 0
            && (*p.offset((1 as libc::c_int + digits) as isize) as libc::c_int
                == '/' as i32
                || *p.offset((1 as libc::c_int + digits) as isize) as libc::c_int
                    == '\0' as i32)
        {
            current_block_9 = 6294032324224832640;
        } else {
            ret = aprintf(b"ftp://%s\0" as *const u8 as *const libc::c_char, url);
            if !ret.is_null() {
                *ret
                    .offset(
                        (6 as libc::c_int as libc::c_long
                            + p.offset_from(url) as libc::c_long) as isize,
                    ) = '/' as i32 as libc::c_char;
            }
            current_block_9 = 7746791466490516765;
        }
    } else {
        current_block_9 = 6294032324224832640;
    }
    match current_block_9 {
        6294032324224832640 => {
            ret = aprintf(b"http://%s\0" as *const u8 as *const libc::c_char, url);
        }
        _ => {}
    }
    return ret;
}
#[inline]
unsafe extern "C" fn strpbrk_or_eos(
    mut s: *const libc::c_char,
    mut accept: *const libc::c_char,
) -> *mut libc::c_char {
    let mut p: *mut libc::c_char = strpbrk(s, accept);
    if p.is_null() {
        p = strchr(s, '\0' as i32);
    }
    return p;
}
unsafe extern "C" fn lowercase_str(mut str: *mut libc::c_char) -> bool {
    let mut changed: bool = 0 as libc::c_int != 0;
    while *str != 0 {
        if c_isupper(*str as libc::c_int) {
            changed = 1 as libc::c_int != 0;
            *str = c_tolower(*str as libc::c_int) as libc::c_char;
        }
        str = str.offset(1);
        str;
    }
    return changed;
}
unsafe extern "C" fn init_seps(mut scheme: url_scheme) -> *const libc::c_char {
    static mut seps: [libc::c_char; 8] = unsafe {
        *::std::mem::transmute::<&[u8; 8], &mut [libc::c_char; 8]>(b":/\0\0\0\0\0\0")
    };
    let mut p: *mut libc::c_char = seps.as_mut_ptr().offset(2 as libc::c_int as isize);
    let mut flags: libc::c_int = supported_schemes[scheme as usize].flags;
    if flags & scm_has_params as libc::c_int != 0 {
        let fresh12 = p;
        p = p.offset(1);
        *fresh12 = ';' as i32 as libc::c_char;
    }
    if flags & scm_has_query as libc::c_int != 0 {
        let fresh13 = p;
        p = p.offset(1);
        *fresh13 = '?' as i32 as libc::c_char;
    }
    if flags & scm_has_fragment as libc::c_int != 0 {
        let fresh14 = p;
        p = p.offset(1);
        *fresh14 = '#' as i32 as libc::c_char;
    }
    *p = '\0' as i32 as libc::c_char;
    return seps.as_mut_ptr();
}
static mut parse_errors: [*const libc::c_char; 11] = [
    b"No error\0" as *const u8 as *const libc::c_char,
    b"Unsupported scheme\0" as *const u8 as *const libc::c_char,
    b"HTTPS support not compiled in\0" as *const u8 as *const libc::c_char,
    b"FTPS support not compiled in\0" as *const u8 as *const libc::c_char,
    b"Scheme missing\0" as *const u8 as *const libc::c_char,
    b"Invalid host name\0" as *const u8 as *const libc::c_char,
    b"Bad port number\0" as *const u8 as *const libc::c_char,
    b"Invalid user name\0" as *const u8 as *const libc::c_char,
    b"Unterminated IPv6 numeric address\0" as *const u8 as *const libc::c_char,
    b"IPv6 addresses not supported\0" as *const u8 as *const libc::c_char,
    b"Invalid IPv6 numeric address\0" as *const u8 as *const libc::c_char,
];
pub unsafe extern "C" fn url_parse(
    mut url: *const libc::c_char,
    mut error: *mut libc::c_int,
    mut iri: *mut iri,
    mut percent_encode: bool,
) -> *mut url {
    let mut current_block: u64;
    let mut u: *mut url = 0 as *mut url;
    let mut p: *const libc::c_char = 0 as *const libc::c_char;
    let mut path_modified: bool = false;
    let mut host_modified: bool = false;
    let mut scheme: url_scheme = SCHEME_HTTP;
    let mut seps: *const libc::c_char = 0 as *const libc::c_char;
    let mut uname_b: *const libc::c_char = 0 as *const libc::c_char;
    let mut uname_e: *const libc::c_char = 0 as *const libc::c_char;
    let mut host_b: *const libc::c_char = 0 as *const libc::c_char;
    let mut host_e: *const libc::c_char = 0 as *const libc::c_char;
    let mut path_b: *const libc::c_char = 0 as *const libc::c_char;
    let mut path_e: *const libc::c_char = 0 as *const libc::c_char;
    let mut params_b: *const libc::c_char = 0 as *const libc::c_char;
    let mut params_e: *const libc::c_char = 0 as *const libc::c_char;
    let mut query_b: *const libc::c_char = 0 as *const libc::c_char;
    let mut query_e: *const libc::c_char = 0 as *const libc::c_char;
    let mut fragment_b: *const libc::c_char = 0 as *const libc::c_char;
    let mut fragment_e: *const libc::c_char = 0 as *const libc::c_char;
    let mut port: libc::c_int = 0;
    let mut user: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut passwd: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut url_encoded: *const libc::c_char = 0 as *const libc::c_char;
    let mut error_code: libc::c_int = 0;
    scheme = url_scheme(url);
    if scheme as libc::c_uint == SCHEME_INVALID as libc::c_int as libc::c_uint {
        if !url_has_scheme(url) {
            error_code = PE_MISSING_SCHEME as libc::c_int;
        } else if c_strncasecmp(
            url,
            b"https:\0" as *const u8 as *const libc::c_char,
            6 as libc::c_int as size_t,
        ) == 0
        {
            error_code = PE_UNSUPPORTED_SCHEME_HTTPS as libc::c_int;
        } else if c_strncasecmp(
            url,
            b"ftps:\0" as *const u8 as *const libc::c_char,
            5 as libc::c_int as size_t,
        ) == 0
        {
            error_code = PE_UNSUPPORTED_SCHEME_FTPS as libc::c_int;
        } else {
            error_code = PE_UNSUPPORTED_SCHEME as libc::c_int;
        }
    } else {
        url_encoded = url;
        if !iri.is_null() && (*iri).utf8_encode as libc::c_int != 0 {
            let mut new_url: *mut libc::c_char = 0 as *mut libc::c_char;
            (*iri)
                .utf8_encode = remote_to_utf8(
                iri,
                if !((*iri).orig_url).is_null() {
                    (*iri).orig_url as *const libc::c_char
                } else {
                    url
                },
                &mut new_url,
            );
            if !(*iri).utf8_encode {
                new_url = 0 as *mut libc::c_char;
            } else {
                rpl_free((*iri).orig_url as *mut libc::c_void);
                (*iri).orig_url = 0 as *mut libc::c_char;
                (*iri).orig_url = xstrdup(url);
                url_encoded = reencode_escapes(new_url);
                if url_encoded != new_url as *const libc::c_char {
                    rpl_free(new_url as *mut libc::c_void);
                    new_url = 0 as *mut libc::c_char;
                }
                percent_encode = 0 as libc::c_int != 0;
            }
        }
        if percent_encode {
            url_encoded = reencode_escapes(url);
        }
        p = url_encoded;
        p = p.offset(strlen(supported_schemes[scheme as usize].leading_string) as isize);
        uname_b = p;
        p = url_skip_credentials(p);
        uname_e = p;
        path_e = 0 as *const libc::c_char;
        path_b = path_e;
        params_e = 0 as *const libc::c_char;
        params_b = params_e;
        query_e = 0 as *const libc::c_char;
        query_b = query_e;
        fragment_e = 0 as *const libc::c_char;
        fragment_b = fragment_e;
        seps = init_seps(scheme);
        host_b = p;
        if *p as libc::c_int == '[' as i32 {
            host_b = p.offset(1 as libc::c_int as isize);
            host_e = strchr(host_b, ']' as i32);
            if host_e.is_null() {
                error_code = PE_UNTERMINATED_IPV6_ADDRESS as libc::c_int;
                current_block = 4351953798900587230;
            } else if !is_valid_ipv6_address(host_b, host_e) {
                error_code = PE_INVALID_IPV6_ADDRESS as libc::c_int;
                current_block = 4351953798900587230;
            } else {
                p = host_e.offset(1 as libc::c_int as isize);
                if (strchr(seps, *p as libc::c_int)).is_null() {
                    error_code = PE_INVALID_HOST_NAME as libc::c_int;
                    current_block = 4351953798900587230;
                } else {
                    current_block = 1924505913685386279;
                }
            }
        } else {
            p = strpbrk_or_eos(p, seps);
            host_e = p;
            current_block = 1924505913685386279;
        }
        match current_block {
            4351953798900587230 => {}
            _ => {
                seps = seps.offset(1);
                seps;
                if host_b == host_e {
                    error_code = PE_INVALID_HOST_NAME as libc::c_int;
                } else {
                    port = scheme_default_port(scheme);
                    if *p as libc::c_int == ':' as i32 {
                        let mut port_b: *const libc::c_char = 0 as *const libc::c_char;
                        let mut port_e: *const libc::c_char = 0 as *const libc::c_char;
                        let mut pp: *const libc::c_char = 0 as *const libc::c_char;
                        p = p.offset(1);
                        p;
                        port_b = p;
                        p = strpbrk_or_eos(p, seps);
                        port_e = p;
                        if port_b != port_e {
                            port = 0 as libc::c_int;
                            pp = port_b;
                            loop {
                                if !(pp < port_e) {
                                    current_block = 3580086814630675314;
                                    break;
                                }
                                if !c_isdigit(*pp as libc::c_int) {
                                    error_code = PE_BAD_PORT_NUMBER as libc::c_int;
                                    current_block = 4351953798900587230;
                                    break;
                                } else {
                                    port = 10 as libc::c_int * port
                                        + (*pp as libc::c_int - '0' as i32);
                                    if port > 0xffff as libc::c_int {
                                        error_code = PE_BAD_PORT_NUMBER as libc::c_int;
                                        current_block = 4351953798900587230;
                                        break;
                                    } else {
                                        pp = pp.offset(1);
                                        pp;
                                    }
                                }
                            }
                        } else {
                            current_block = 3580086814630675314;
                        }
                    } else {
                        current_block = 3580086814630675314;
                    }
                    match current_block {
                        4351953798900587230 => {}
                        _ => {
                            seps = seps.offset(1);
                            seps;
                            if *p as libc::c_int == '/' as i32 {
                                p = p.offset(1);
                                path_b = p;
                                p = strpbrk_or_eos(p, seps);
                                path_e = p;
                            }
                            seps = seps.offset(1);
                            seps;
                            if supported_schemes[scheme as usize].flags
                                & scm_has_params as libc::c_int != 0
                            {
                                if *p as libc::c_int == ';' as i32 {
                                    p = p.offset(1);
                                    params_b = p;
                                    p = strpbrk_or_eos(p, seps);
                                    params_e = p;
                                }
                                seps = seps.offset(1);
                                seps;
                            }
                            if supported_schemes[scheme as usize].flags
                                & scm_has_query as libc::c_int != 0
                            {
                                if *p as libc::c_int == '?' as i32 {
                                    p = p.offset(1);
                                    query_b = p;
                                    p = strpbrk_or_eos(p, seps);
                                    query_e = p;
                                }
                                seps = seps.offset(1);
                                seps;
                            }
                            if supported_schemes[scheme as usize].flags
                                & scm_has_fragment as libc::c_int != 0
                            {
                                if *p as libc::c_int == '#' as i32 {
                                    p = p.offset(1);
                                    fragment_b = p;
                                    p = strpbrk_or_eos(p, seps);
                                    fragment_e = p;
                                }
                                seps = seps.offset(1);
                                seps;
                            }
                            if uname_b != uname_e {
                                if !parse_credentials(
                                    uname_b,
                                    uname_e.offset(-(1 as libc::c_int as isize)),
                                    &mut user,
                                    &mut passwd,
                                ) {
                                    error_code = PE_INVALID_USER_NAME as libc::c_int;
                                    current_block = 4351953798900587230;
                                } else {
                                    current_block = 200744462051969938;
                                }
                            } else {
                                current_block = 200744462051969938;
                            }
                            match current_block {
                                4351953798900587230 => {}
                                _ => {
                                    u = xcalloc(
                                        1 as libc::c_int as size_t,
                                        ::std::mem::size_of::<url>() as libc::c_ulong,
                                    ) as *mut url;
                                    (*u).scheme = scheme;
                                    (*u).host = strdupdelim(host_b, host_e);
                                    (*u).port = port;
                                    (*u).user = user;
                                    (*u).passwd = passwd;
                                    (*u).path = strdupdelim(path_b, path_e);
                                    path_modified = path_simplify(scheme, (*u).path);
                                    split_path((*u).path, &mut (*u).dir, &mut (*u).file);
                                    host_modified = lowercase_str((*u).host);
                                    if !(strchr((*u).host, '%' as i32)).is_null() {
                                        url_unescape((*u).host);
                                        host_modified = 1 as libc::c_int != 0;
                                        p = (*u).host;
                                        loop {
                                            if !(*p != 0) {
                                                current_block = 5700653730392116747;
                                                break;
                                            }
                                            if c_iscntrl(*p as libc::c_int) {
                                                url_free(u);
                                                error_code = PE_INVALID_HOST_NAME as libc::c_int;
                                                current_block = 4351953798900587230;
                                                break;
                                            } else {
                                                p = p.offset(1);
                                                p;
                                            }
                                        }
                                        match current_block {
                                            4351953798900587230 => {}
                                            _ => {
                                                if opt.enable_iri as libc::c_int != 0 && !iri.is_null() {
                                                    let mut new: *mut libc::c_char = idn_encode(iri, (*u).host);
                                                    if !new.is_null() {
                                                        rpl_free((*u).host as *mut libc::c_void);
                                                        (*u).host = 0 as *mut libc::c_char;
                                                        (*u).host = new;
                                                        host_modified = 1 as libc::c_int != 0;
                                                    }
                                                }
                                                current_block = 9775647934248138666;
                                            }
                                        }
                                    } else {
                                        current_block = 9775647934248138666;
                                    }
                                    match current_block {
                                        4351953798900587230 => {}
                                        _ => {
                                            if !params_b.is_null() {
                                                (*u).params = strdupdelim(params_b, params_e);
                                            }
                                            if !query_b.is_null() {
                                                (*u).query = strdupdelim(query_b, query_e);
                                            }
                                            if !fragment_b.is_null() {
                                                (*u).fragment = strdupdelim(fragment_b, fragment_e);
                                            }
                                            if opt.enable_iri as libc::c_int != 0
                                                || path_modified as libc::c_int != 0
                                                || !((*u).fragment).is_null()
                                                || host_modified as libc::c_int != 0 || path_b == path_e
                                            {
                                                (*u).url = url_string(u, URL_AUTH_SHOW);
                                                if url_encoded != url {
                                                    rpl_free(url_encoded as *mut libc::c_void);
                                                    url_encoded = 0 as *const libc::c_char;
                                                }
                                            } else if url_encoded == url {
                                                (*u).url = xstrdup(url);
                                            } else {
                                                (*u).url = url_encoded as *mut libc::c_char;
                                            }
                                            return u;
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
    if !url_encoded.is_null() && url_encoded != url {
        rpl_free(url_encoded as *mut libc::c_void);
        url_encoded = 0 as *const libc::c_char;
    }
    if !error.is_null() {
        *error = error_code;
    }
    return 0 as *mut url;
}
pub unsafe extern "C" fn url_error(mut error_code: libc::c_int) -> *const libc::c_char {
    if error_code >= 0 as libc::c_int
        && error_code
            < (::std::mem::size_of::<[*const libc::c_char; 11]>() as libc::c_ulong)
                .wrapping_div(
                    ::std::mem::size_of::<*const libc::c_char>() as libc::c_ulong,
                ) as libc::c_int
    {
        return dcgettext(
            0 as *const libc::c_char,
            parse_errors[error_code as usize],
            5 as libc::c_int,
        );
    }
    return b"\0" as *const u8 as *const libc::c_char;
}
unsafe extern "C" fn split_path(
    mut path: *const libc::c_char,
    mut dir: *mut *mut libc::c_char,
    mut file: *mut *mut libc::c_char,
) {
    let mut last_slash: *mut libc::c_char = strrchr(path, '/' as i32);
    if last_slash.is_null() {
        *dir = xstrdup(b"\0" as *const u8 as *const libc::c_char);
        *file = xstrdup(path);
    } else {
        *dir = strdupdelim(path, last_slash);
        *file = xstrdup(last_slash.offset(1 as libc::c_int as isize));
    }
    url_unescape(*dir);
    url_unescape(*file);
}
unsafe extern "C" fn full_path_length(mut url: *const url) -> libc::c_int {
    let mut len: libc::c_int = 0 as libc::c_int;
    if !((*url).path).is_null() {
        len = (len as libc::c_ulong)
            .wrapping_add(
                (1 as libc::c_int as libc::c_ulong).wrapping_add(strlen((*url).path)),
            ) as libc::c_int as libc::c_int;
    }
    if !((*url).params).is_null() {
        len = (len as libc::c_ulong)
            .wrapping_add(
                (1 as libc::c_int as libc::c_ulong).wrapping_add(strlen((*url).params)),
            ) as libc::c_int as libc::c_int;
    }
    if !((*url).query).is_null() {
        len = (len as libc::c_ulong)
            .wrapping_add(
                (1 as libc::c_int as libc::c_ulong).wrapping_add(strlen((*url).query)),
            ) as libc::c_int as libc::c_int;
    }
    return len;
}
unsafe extern "C" fn full_path_write(
    mut url: *const url,
    mut where_0: *mut libc::c_char,
) {
    let mut f_el: *mut libc::c_char = (*url).path;
    if !f_el.is_null() {
        let mut l: libc::c_int = strlen(f_el) as libc::c_int;
        let fresh15 = where_0;
        where_0 = where_0.offset(1);
        *fresh15 = '/' as i32 as libc::c_char;
        memcpy(
            where_0 as *mut libc::c_void,
            f_el as *const libc::c_void,
            l as libc::c_ulong,
        );
        where_0 = where_0.offset(l as isize);
    }
    let mut f_el_0: *mut libc::c_char = (*url).params;
    if !f_el_0.is_null() {
        let mut l_0: libc::c_int = strlen(f_el_0) as libc::c_int;
        let fresh16 = where_0;
        where_0 = where_0.offset(1);
        *fresh16 = ';' as i32 as libc::c_char;
        memcpy(
            where_0 as *mut libc::c_void,
            f_el_0 as *const libc::c_void,
            l_0 as libc::c_ulong,
        );
        where_0 = where_0.offset(l_0 as isize);
    }
    let mut f_el_1: *mut libc::c_char = (*url).query;
    if !f_el_1.is_null() {
        let mut l_1: libc::c_int = strlen(f_el_1) as libc::c_int;
        let fresh17 = where_0;
        where_0 = where_0.offset(1);
        *fresh17 = '?' as i32 as libc::c_char;
        memcpy(
            where_0 as *mut libc::c_void,
            f_el_1 as *const libc::c_void,
            l_1 as libc::c_ulong,
        );
        where_0 = where_0.offset(l_1 as isize);
    }
}
pub unsafe extern "C" fn url_full_path(mut url: *const url) -> *mut libc::c_char {
    let mut length: libc::c_int = full_path_length(url);
    let mut full_path: *mut libc::c_char = xmalloc((length + 1 as libc::c_int) as size_t)
        as *mut libc::c_char;
    full_path_write(url, full_path);
    *full_path.offset(length as isize) = '\0' as i32 as libc::c_char;
    return full_path;
}
unsafe extern "C" fn unescape_single_char(
    mut str: *mut libc::c_char,
    mut chr: libc::c_char,
) {
    let c1: libc::c_char = ((*::std::mem::transmute::<
        &[u8; 17],
        &[libc::c_char; 17],
    >(b"0123456789ABCDEF\0"))[(chr as libc::c_int >> 4 as libc::c_int) as usize]
        as libc::c_int + 0 as libc::c_int) as libc::c_char;
    let c2: libc::c_char = ((*::std::mem::transmute::<
        &[u8; 17],
        &[libc::c_char; 17],
    >(b"0123456789ABCDEF\0"))[(chr as libc::c_int & 0xf as libc::c_int) as usize]
        as libc::c_int + 0 as libc::c_int) as libc::c_char;
    let mut h: *mut libc::c_char = str;
    let mut t: *mut libc::c_char = str;
    while *h != 0 {
        if *h.offset(0 as libc::c_int as isize) as libc::c_int == '%' as i32
            && *h.offset(1 as libc::c_int as isize) as libc::c_int == c1 as libc::c_int
            && *h.offset(2 as libc::c_int as isize) as libc::c_int == c2 as libc::c_int
        {
            *t = chr;
            h = h.offset(2 as libc::c_int as isize);
        } else {
            *t = *h;
        }
        h = h.offset(1);
        h;
        t = t.offset(1);
        t;
    }
    *t = '\0' as i32 as libc::c_char;
}
unsafe extern "C" fn url_escape_dir(mut dir: *const libc::c_char) -> *mut libc::c_char {
    let mut newdir: *mut libc::c_char = url_escape_1(
        dir,
        (urlchr_unsafe as libc::c_int | urlchr_reserved as libc::c_int) as libc::c_uchar,
        1 as libc::c_int != 0,
    );
    if newdir == dir as *mut libc::c_char {
        return dir as *mut libc::c_char;
    }
    unescape_single_char(newdir, '/' as i32 as libc::c_char);
    return newdir;
}
unsafe extern "C" fn sync_path(mut u: *mut url) {
    let mut newpath: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut efile: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut edir: *mut libc::c_char = 0 as *mut libc::c_char;
    rpl_free((*u).path as *mut libc::c_void);
    (*u).path = 0 as *mut libc::c_char;
    edir = url_escape_dir((*u).dir);
    efile = url_escape_1(
        (*u).file,
        (urlchr_unsafe as libc::c_int | urlchr_reserved as libc::c_int) as libc::c_uchar,
        1 as libc::c_int != 0,
    );
    if *edir == 0 {
        newpath = xstrdup(efile);
    } else {
        let mut dirlen: libc::c_int = strlen(edir) as libc::c_int;
        let mut filelen: libc::c_int = strlen(efile) as libc::c_int;
        newpath = xmalloc(
            (dirlen + 1 as libc::c_int + filelen + 1 as libc::c_int) as size_t,
        ) as *mut libc::c_char;
        let mut p: *mut libc::c_char = newpath;
        memcpy(
            p as *mut libc::c_void,
            edir as *const libc::c_void,
            dirlen as libc::c_ulong,
        );
        p = p.offset(dirlen as isize);
        let fresh18 = p;
        p = p.offset(1);
        *fresh18 = '/' as i32 as libc::c_char;
        memcpy(
            p as *mut libc::c_void,
            efile as *const libc::c_void,
            filelen as libc::c_ulong,
        );
        p = p.offset(filelen as isize);
        *p = '\0' as i32 as libc::c_char;
    }
    (*u).path = newpath;
    if edir != (*u).dir {
        rpl_free(edir as *mut libc::c_void);
        edir = 0 as *mut libc::c_char;
    }
    if efile != (*u).file {
        rpl_free(efile as *mut libc::c_void);
        efile = 0 as *mut libc::c_char;
    }
    rpl_free((*u).url as *mut libc::c_void);
    (*u).url = 0 as *mut libc::c_char;
    (*u).url = url_string(u, URL_AUTH_SHOW);
}
pub unsafe extern "C" fn url_set_dir(
    mut url: *mut url,
    mut newdir: *const libc::c_char,
) {
    rpl_free((*url).dir as *mut libc::c_void);
    (*url).dir = 0 as *mut libc::c_char;
    (*url).dir = xstrdup(newdir);
    sync_path(url);
}
pub unsafe extern "C" fn url_set_file(
    mut url: *mut url,
    mut newfile: *const libc::c_char,
) {
    rpl_free((*url).file as *mut libc::c_void);
    (*url).file = 0 as *mut libc::c_char;
    (*url).file = xstrdup(newfile);
    sync_path(url);
}
pub unsafe extern "C" fn url_free(mut url: *mut url) {
    if !url.is_null() {
        rpl_free((*url).host as *mut libc::c_void);
        (*url).host = 0 as *mut libc::c_char;
        rpl_free((*url).path as *mut libc::c_void);
        (*url).path = 0 as *mut libc::c_char;
        rpl_free((*url).url as *mut libc::c_void);
        (*url).url = 0 as *mut libc::c_char;
        rpl_free((*url).params as *mut libc::c_void);
        (*url).params = 0 as *mut libc::c_char;
        rpl_free((*url).query as *mut libc::c_void);
        (*url).query = 0 as *mut libc::c_char;
        rpl_free((*url).fragment as *mut libc::c_void);
        (*url).fragment = 0 as *mut libc::c_char;
        rpl_free((*url).user as *mut libc::c_void);
        (*url).user = 0 as *mut libc::c_char;
        rpl_free((*url).passwd as *mut libc::c_void);
        (*url).passwd = 0 as *mut libc::c_char;
        rpl_free((*url).dir as *mut libc::c_void);
        (*url).dir = 0 as *mut libc::c_char;
        rpl_free((*url).file as *mut libc::c_void);
        (*url).file = 0 as *mut libc::c_char;
        rpl_free(url as *mut libc::c_void);
        url = 0 as *mut url;
    }
}
pub unsafe extern "C" fn mkalldirs(mut path: *const libc::c_char) -> libc::c_int {
    let mut p: *const libc::c_char = 0 as *const libc::c_char;
    let mut t: *mut libc::c_char = 0 as *mut libc::c_char;
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
    let mut res: libc::c_int = 0;
    p = strrchr(path, '/' as i32);
    p = if p.is_null() { path } else { p };
    if p == path && *p as libc::c_int != '/' as i32 {
        return 0 as libc::c_int;
    }
    t = strdupdelim(path, p);
    if stat(t, &mut st) == 0 as libc::c_int {
        if st.st_mode & 0o170000 as libc::c_int as libc::c_uint
            == 0o40000 as libc::c_int as libc::c_uint
        {
            rpl_free(t as *mut libc::c_void);
            t = 0 as *mut libc::c_char;
            return 0 as libc::c_int;
        } else {
            if opt.debug as libc::c_long != 0 {
                debug_logprintf(
                    b"Removing %s because of directory danger!\n\0" as *const u8
                        as *const libc::c_char,
                    t,
                );
            }
            if unlink(t) != 0 {
                logprintf(
                    LOG_NOTQUIET,
                    b"Failed to unlink %s (%d): %s\n\0" as *const u8
                        as *const libc::c_char,
                    t,
                    *__errno_location(),
                    strerror(*__errno_location()),
                );
            }
        }
    }
    res = make_directory(t);
    if res != 0 as libc::c_int {
        logprintf(
            LOG_NOTQUIET,
            b"%s: %s\n\0" as *const u8 as *const libc::c_char,
            t,
            strerror(*__errno_location()),
        );
    }
    rpl_free(t as *mut libc::c_void);
    t = 0 as *mut libc::c_char;
    return res;
}
unsafe extern "C" fn append_null(mut dest: *mut growable) {
    let mut G_: *mut growable = dest;
    let mut DR_needed_size: libc::c_long = ((*G_).tail + 1 as libc::c_int)
        as libc::c_long;
    let mut DR_newsize: libc::c_long = 0 as libc::c_int as libc::c_long;
    while ((*G_).size as libc::c_long) < DR_needed_size {
        DR_newsize = ((*G_).size << 1 as libc::c_int) as libc::c_long;
        if DR_newsize < 16 as libc::c_int as libc::c_long {
            DR_newsize = 16 as libc::c_int as libc::c_long;
        }
        (*G_).size = DR_newsize as libc::c_int;
    }
    if DR_newsize != 0 {
        (*G_)
            .base = xrealloc(
            (*G_).base as *mut libc::c_void,
            (DR_newsize as libc::c_ulong)
                .wrapping_mul(::std::mem::size_of::<libc::c_char>() as libc::c_ulong),
        ) as *mut libc::c_char;
    }
    *((*dest).base).offset((*dest).tail as isize) = 0 as libc::c_int as libc::c_char;
}
unsafe extern "C" fn append_char(mut ch: libc::c_char, mut dest: *mut growable) {
    if ch != 0 {
        let mut G_: *mut growable = dest;
        let mut DR_needed_size: libc::c_long = ((*G_).tail + 1 as libc::c_int)
            as libc::c_long;
        let mut DR_newsize: libc::c_long = 0 as libc::c_int as libc::c_long;
        while ((*G_).size as libc::c_long) < DR_needed_size {
            DR_newsize = ((*G_).size << 1 as libc::c_int) as libc::c_long;
            if DR_newsize < 16 as libc::c_int as libc::c_long {
                DR_newsize = 16 as libc::c_int as libc::c_long;
            }
            (*G_).size = DR_newsize as libc::c_int;
        }
        if DR_newsize != 0 {
            (*G_)
                .base = xrealloc(
                (*G_).base as *mut libc::c_void,
                (DR_newsize as libc::c_ulong)
                    .wrapping_mul(::std::mem::size_of::<libc::c_char>() as libc::c_ulong),
            ) as *mut libc::c_char;
        }
        *((*dest).base).offset((*dest).tail as isize) = ch;
        (*dest).tail += 1 as libc::c_int;
    }
    append_null(dest);
}
unsafe extern "C" fn append_string(
    mut str: *const libc::c_char,
    mut dest: *mut growable,
) {
    let mut l: libc::c_int = strlen(str) as libc::c_int;
    if l != 0 {
        let mut G_: *mut growable = dest;
        let mut DR_needed_size: libc::c_long = ((*G_).tail + l) as libc::c_long;
        let mut DR_newsize: libc::c_long = 0 as libc::c_int as libc::c_long;
        while ((*G_).size as libc::c_long) < DR_needed_size {
            DR_newsize = ((*G_).size << 1 as libc::c_int) as libc::c_long;
            if DR_newsize < 16 as libc::c_int as libc::c_long {
                DR_newsize = 16 as libc::c_int as libc::c_long;
            }
            (*G_).size = DR_newsize as libc::c_int;
        }
        if DR_newsize != 0 {
            (*G_)
                .base = xrealloc(
                (*G_).base as *mut libc::c_void,
                (DR_newsize as libc::c_ulong)
                    .wrapping_mul(::std::mem::size_of::<libc::c_char>() as libc::c_ulong),
            ) as *mut libc::c_char;
        }
        memcpy(
            ((*dest).base).offset((*dest).tail as isize) as *mut libc::c_void,
            str as *const libc::c_void,
            l as libc::c_ulong,
        );
        (*dest).tail += l;
    }
    append_null(dest);
}
static mut filechr_table: [libc::c_uchar; 256] = [
    (filechr_not_unix as libc::c_int | filechr_not_vms as libc::c_int
        | filechr_not_windows as libc::c_int | filechr_control as libc::c_int)
        as libc::c_uchar,
    (filechr_not_vms as libc::c_int | filechr_control as libc::c_int) as libc::c_uchar,
    (filechr_not_vms as libc::c_int | filechr_control as libc::c_int) as libc::c_uchar,
    (filechr_not_vms as libc::c_int | filechr_control as libc::c_int) as libc::c_uchar,
    (filechr_not_vms as libc::c_int | filechr_control as libc::c_int) as libc::c_uchar,
    (filechr_not_vms as libc::c_int | filechr_control as libc::c_int) as libc::c_uchar,
    (filechr_not_vms as libc::c_int | filechr_control as libc::c_int) as libc::c_uchar,
    (filechr_not_vms as libc::c_int | filechr_control as libc::c_int) as libc::c_uchar,
    (filechr_not_vms as libc::c_int | filechr_control as libc::c_int) as libc::c_uchar,
    (filechr_not_vms as libc::c_int | filechr_control as libc::c_int) as libc::c_uchar,
    (filechr_not_vms as libc::c_int | filechr_control as libc::c_int) as libc::c_uchar,
    (filechr_not_vms as libc::c_int | filechr_control as libc::c_int) as libc::c_uchar,
    (filechr_not_vms as libc::c_int | filechr_control as libc::c_int) as libc::c_uchar,
    (filechr_not_vms as libc::c_int | filechr_control as libc::c_int) as libc::c_uchar,
    (filechr_not_vms as libc::c_int | filechr_control as libc::c_int) as libc::c_uchar,
    (filechr_not_vms as libc::c_int | filechr_control as libc::c_int) as libc::c_uchar,
    (filechr_not_vms as libc::c_int | filechr_control as libc::c_int) as libc::c_uchar,
    (filechr_not_vms as libc::c_int | filechr_control as libc::c_int) as libc::c_uchar,
    (filechr_not_vms as libc::c_int | filechr_control as libc::c_int) as libc::c_uchar,
    (filechr_not_vms as libc::c_int | filechr_control as libc::c_int) as libc::c_uchar,
    (filechr_not_vms as libc::c_int | filechr_control as libc::c_int) as libc::c_uchar,
    (filechr_not_vms as libc::c_int | filechr_control as libc::c_int) as libc::c_uchar,
    (filechr_not_vms as libc::c_int | filechr_control as libc::c_int) as libc::c_uchar,
    (filechr_not_vms as libc::c_int | filechr_control as libc::c_int) as libc::c_uchar,
    (filechr_not_vms as libc::c_int | filechr_control as libc::c_int) as libc::c_uchar,
    (filechr_not_vms as libc::c_int | filechr_control as libc::c_int) as libc::c_uchar,
    (filechr_not_vms as libc::c_int | filechr_control as libc::c_int) as libc::c_uchar,
    (filechr_not_vms as libc::c_int | filechr_control as libc::c_int) as libc::c_uchar,
    (filechr_not_vms as libc::c_int | filechr_control as libc::c_int) as libc::c_uchar,
    (filechr_not_vms as libc::c_int | filechr_control as libc::c_int) as libc::c_uchar,
    (filechr_not_vms as libc::c_int | filechr_control as libc::c_int) as libc::c_uchar,
    (filechr_not_vms as libc::c_int | filechr_control as libc::c_int) as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    filechr_not_windows as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    (filechr_not_vms as libc::c_int | filechr_not_windows as libc::c_int)
        as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    (filechr_not_unix as libc::c_int | filechr_not_windows as libc::c_int)
        as libc::c_uchar,
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
    filechr_not_windows as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    filechr_not_windows as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    filechr_not_windows as libc::c_int as libc::c_uchar,
    (filechr_not_vms as libc::c_int | filechr_not_windows as libc::c_int)
        as libc::c_uchar,
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
    filechr_not_windows as libc::c_int as libc::c_uchar,
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
    filechr_not_windows as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    filechr_control as libc::c_int as libc::c_uchar,
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
];
unsafe extern "C" fn append_uri_pathel(
    mut b: *const libc::c_char,
    mut e: *const libc::c_char,
    mut escaped: bool,
    mut dest: *mut growable,
) {
    let mut p: *const libc::c_char = 0 as *const libc::c_char;
    let mut buf: [libc::c_char; 1024] = [0; 1024];
    let mut unescaped: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut quoted: libc::c_int = 0;
    let mut outlen: libc::c_int = 0;
    let mut mask: libc::c_int = 0;
    let mut max_length: libc::c_int = 0;
    if dest.is_null() {
        return;
    }
    if opt.restrict_files_os as libc::c_uint
        == restrict_unix as libc::c_int as libc::c_uint
    {
        mask = filechr_not_unix as libc::c_int;
    } else if opt.restrict_files_os as libc::c_uint
        == restrict_vms as libc::c_int as libc::c_uint
    {
        mask = filechr_not_vms as libc::c_int;
    } else {
        mask = filechr_not_windows as libc::c_int;
    }
    if opt.restrict_files_ctrl {
        mask |= filechr_control as libc::c_int;
    }
    if escaped {
        let mut len: size_t = e.offset_from(b) as libc::c_long as size_t;
        if len < ::std::mem::size_of::<[libc::c_char; 1024]>() as libc::c_ulong {
            unescaped = buf.as_mut_ptr();
        } else {
            unescaped = xmalloc(len.wrapping_add(1 as libc::c_int as libc::c_ulong))
                as *mut libc::c_char;
        }
        memcpy(unescaped as *mut libc::c_void, b as *const libc::c_void, len);
        *unescaped.offset(len as isize) = 0 as libc::c_int as libc::c_char;
        url_unescape(unescaped);
        b = unescaped;
        e = unescaped.offset(strlen(unescaped) as isize);
    }
    if e.offset_from(b) as libc::c_long == 2 as libc::c_int as libc::c_long
        && *b.offset(0 as libc::c_int as isize) as libc::c_int == '.' as i32
        && *b.offset(1 as libc::c_int as isize) as libc::c_int == '.' as i32
    {
        b = b"%2E%2E\0" as *const u8 as *const libc::c_char;
        e = b.offset(6 as libc::c_int as isize);
    }
    quoted = 0 as libc::c_int;
    p = b;
    while p < e {
        if opt.restrict_files_nonascii as libc::c_int != 0
            && !c_isascii(*p as libc::c_uchar as libc::c_int)
            || filechr_table[*p as libc::c_uchar as usize] as libc::c_int & mask != 0
        {
            quoted += 1;
            quoted;
        }
        p = p.offset(1);
        p;
    }
    outlen = (e.offset_from(b) as libc::c_long
        + (2 as libc::c_int * quoted) as libc::c_long) as libc::c_int;
    max_length = get_max_length((*dest).base, (*dest).tail, _PC_NAME_MAX as libc::c_int)
        as libc::c_int;
    max_length -= 19 as libc::c_int;
    if max_length > 0 as libc::c_int && outlen > max_length {
        logprintf(
            LOG_NOTQUIET,
            b"The destination name is too long (%d), reducing to %d\n\0" as *const u8
                as *const libc::c_char,
            outlen,
            max_length,
        );
        outlen = max_length;
    }
    let mut G_: *mut growable = dest;
    let mut DR_needed_size: libc::c_long = ((*G_).tail + outlen) as libc::c_long;
    let mut DR_newsize: libc::c_long = 0 as libc::c_int as libc::c_long;
    while ((*G_).size as libc::c_long) < DR_needed_size {
        DR_newsize = ((*G_).size << 1 as libc::c_int) as libc::c_long;
        if DR_newsize < 16 as libc::c_int as libc::c_long {
            DR_newsize = 16 as libc::c_int as libc::c_long;
        }
        (*G_).size = DR_newsize as libc::c_int;
    }
    if DR_newsize != 0 {
        (*G_)
            .base = xrealloc(
            (*G_).base as *mut libc::c_void,
            (DR_newsize as libc::c_ulong)
                .wrapping_mul(::std::mem::size_of::<libc::c_char>() as libc::c_ulong),
        ) as *mut libc::c_char;
    }
    if ((*dest).base).is_null() {
        return;
    }
    if quoted == 0 {
        memcpy(
            ((*dest).base).offset((*dest).tail as isize) as *mut libc::c_void,
            b as *const libc::c_void,
            outlen as libc::c_ulong,
        );
    } else {
        let mut q: *mut libc::c_char = ((*dest).base).offset((*dest).tail as isize);
        let mut i: libc::c_int = 0;
        i = 0 as libc::c_int;
        p = b;
        while p < e {
            if !(opt.restrict_files_nonascii as libc::c_int != 0
                && !c_isascii(*p as libc::c_uchar as libc::c_int)
                || filechr_table[*p as libc::c_uchar as usize] as libc::c_int & mask
                    != 0)
            {
                if i == outlen {
                    break;
                }
                let fresh19 = q;
                q = q.offset(1);
                *fresh19 = *p;
                i += 1;
                i;
            } else {
                if i + 3 as libc::c_int > outlen {
                    break;
                }
                let mut ch: libc::c_uchar = *p as libc::c_uchar;
                let fresh20 = q;
                q = q.offset(1);
                *fresh20 = '%' as i32 as libc::c_char;
                let fresh21 = q;
                q = q.offset(1);
                *fresh21 = ((*::std::mem::transmute::<
                    &[u8; 17],
                    &[libc::c_char; 17],
                >(
                    b"0123456789ABCDEF\0",
                ))[(ch as libc::c_int >> 4 as libc::c_int) as usize] as libc::c_int
                    + 0 as libc::c_int) as libc::c_char;
                let fresh22 = q;
                q = q.offset(1);
                *fresh22 = ((*::std::mem::transmute::<
                    &[u8; 17],
                    &[libc::c_char; 17],
                >(
                    b"0123456789ABCDEF\0",
                ))[(ch as libc::c_int & 0xf as libc::c_int) as usize] as libc::c_int
                    + 0 as libc::c_int) as libc::c_char;
                i += 3 as libc::c_int;
            }
            p = p.offset(1);
            p;
        }
    }
    if opt.restrict_files_case as libc::c_uint
        == restrict_lowercase as libc::c_int as libc::c_uint
        || opt.restrict_files_case as libc::c_uint
            == restrict_uppercase as libc::c_int as libc::c_uint
    {
        let mut q_0: *mut libc::c_char = 0 as *mut libc::c_char;
        q_0 = ((*dest).base).offset((*dest).tail as isize);
        while q_0 < ((*dest).base).offset((*dest).tail as isize).offset(outlen as isize)
        {
            if opt.restrict_files_case as libc::c_uint
                == restrict_lowercase as libc::c_int as libc::c_uint
            {
                *q_0 = c_tolower(*q_0 as libc::c_int) as libc::c_char;
            } else {
                *q_0 = c_toupper(*q_0 as libc::c_int) as libc::c_char;
            }
            q_0 = q_0.offset(1);
            q_0;
        }
    }
    (*dest).tail += outlen;
    append_null(dest);
    if !unescaped.is_null() && unescaped != buf.as_mut_ptr() {
        rpl_free(unescaped as *mut libc::c_void);
    }
}
unsafe extern "C" fn convert_fname(mut fname: *mut libc::c_char) -> *mut libc::c_char {
    let mut converted_fname: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut from_encoding: *const libc::c_char = opt.encoding_remote;
    let mut to_encoding: *const libc::c_char = opt.locale;
    let mut cd: iconv_t = 0 as *mut libc::c_void;
    let mut len: size_t = 0;
    let mut done: size_t = 0;
    let mut inlen: size_t = 0;
    let mut outlen: size_t = 0;
    let mut s: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut orig_fname: *const libc::c_char = 0 as *const libc::c_char;
    if from_encoding.is_null() {
        from_encoding = b"UTF-8\0" as *const u8 as *const libc::c_char;
    }
    if to_encoding.is_null() {
        to_encoding = nl_langinfo(CODESET as libc::c_int);
    }
    cd = iconv_open(to_encoding, from_encoding);
    if cd == -(1 as libc::c_int) as iconv_t {
        logprintf(
            LOG_VERBOSE,
            dcgettext(
                0 as *const libc::c_char,
                b"Conversion from %s to %s isn't supported\n\0" as *const u8
                    as *const libc::c_char,
                5 as libc::c_int,
            ),
            quote_n(0 as libc::c_int, from_encoding),
            quote_n(1 as libc::c_int, to_encoding),
        );
        return fname;
    }
    orig_fname = fname;
    inlen = strlen(fname);
    outlen = inlen.wrapping_mul(2 as libc::c_int as libc::c_ulong);
    len = outlen;
    s = xmalloc(outlen.wrapping_add(1 as libc::c_int as libc::c_ulong))
        as *mut libc::c_char;
    converted_fname = s;
    done = 0 as libc::c_int as size_t;
    loop {
        *__errno_location() = 0 as libc::c_int;
        if iconv(
            cd,
            &mut fname as *mut *mut libc::c_char,
            &mut inlen,
            &mut s,
            &mut outlen,
        ) == 0 as libc::c_int as libc::c_ulong
            && iconv(
                cd,
                0 as *mut *mut libc::c_char,
                0 as *mut size_t,
                &mut s,
                &mut outlen,
            ) == 0 as libc::c_int as libc::c_ulong
        {
            *converted_fname
                .offset(len as isize)
                .offset(-(outlen as isize))
                .offset(-(done as isize)) = '\0' as i32 as libc::c_char;
            iconv_close(cd);
            if opt.debug as libc::c_long != 0 {
                debug_logprintf(
                    b"Converted file name '%s' (%s) -> '%s' (%s)\n\0" as *const u8
                        as *const libc::c_char,
                    orig_fname,
                    from_encoding,
                    converted_fname,
                    to_encoding,
                );
            }
            rpl_free(orig_fname as *mut libc::c_void);
            orig_fname = 0 as *const libc::c_char;
            return converted_fname;
        }
        if *__errno_location() == 22 as libc::c_int
            || *__errno_location() == 84 as libc::c_int
            || *__errno_location() == 0 as libc::c_int
        {
            if *__errno_location() != 0 {
                logprintf(
                    LOG_VERBOSE,
                    dcgettext(
                        0 as *const libc::c_char,
                        b"Incomplete or invalid multibyte sequence encountered\n\0"
                            as *const u8 as *const libc::c_char,
                        5 as libc::c_int,
                    ),
                );
            } else {
                logprintf(
                    LOG_VERBOSE,
                    dcgettext(
                        0 as *const libc::c_char,
                        b"Unconvertable multibyte sequence encountered\n\0" as *const u8
                            as *const libc::c_char,
                        5 as libc::c_int,
                    ),
                );
            }
            rpl_free(converted_fname as *mut libc::c_void);
            converted_fname = 0 as *mut libc::c_char;
            converted_fname = orig_fname as *mut libc::c_char;
            break;
        } else if *__errno_location() == 7 as libc::c_int {
            done = len;
            outlen = done
                .wrapping_add(inlen.wrapping_mul(2 as libc::c_int as libc::c_ulong));
            len = outlen;
            converted_fname = xrealloc(
                converted_fname as *mut libc::c_void,
                outlen.wrapping_add(1 as libc::c_int as libc::c_ulong),
            ) as *mut libc::c_char;
            s = converted_fname.offset(done as isize);
        } else {
            logprintf(
                LOG_VERBOSE,
                dcgettext(
                    0 as *const libc::c_char,
                    b"Unhandled errno %d\n\0" as *const u8 as *const libc::c_char,
                    5 as libc::c_int,
                ),
                *__errno_location(),
            );
            rpl_free(converted_fname as *mut libc::c_void);
            converted_fname = 0 as *mut libc::c_char;
            converted_fname = orig_fname as *mut libc::c_char;
            break;
        }
    }
    if opt.debug as libc::c_long != 0 {
        debug_logprintf(
            b"Failed to convert file name '%s' (%s) -> '?' (%s)\n\0" as *const u8
                as *const libc::c_char,
            orig_fname,
            from_encoding,
            to_encoding,
        );
    }
    iconv_close(cd);
    return converted_fname;
}
unsafe extern "C" fn append_dir_structure(mut u: *const url, mut dest: *mut growable) {
    let mut pathel: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut next: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut cut: libc::c_int = opt.cut_dirs;
    pathel = (*u).path;
    loop {
        next = strchr(pathel, '/' as i32);
        if next.is_null() {
            break;
        }
        let fresh23 = cut;
        cut = cut - 1;
        if !(fresh23 > 0 as libc::c_int) {
            if !(pathel == next) {
                if (*dest).tail != 0 {
                    append_char('/' as i32 as libc::c_char, dest);
                }
                append_uri_pathel(pathel, next, 1 as libc::c_int != 0, dest);
            }
        }
        pathel = next.offset(1 as libc::c_int as isize);
    };
}
pub unsafe extern "C" fn url_file_name(
    mut u: *const url,
    mut replaced_filename: *mut libc::c_char,
) -> *mut libc::c_char {
    let mut fnres: growable = growable {
        base: 0 as *mut libc::c_char,
        size: 0,
        tail: 0,
    };
    let mut temp_fnres: growable = growable {
        base: 0 as *mut libc::c_char,
        size: 0,
        tail: 0,
    };
    let mut u_file: *const libc::c_char = 0 as *const libc::c_char;
    let mut fname: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut unique: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut fname_len_check: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut index_filename: *const libc::c_char = b"index.html\0" as *const u8
        as *const libc::c_char;
    fnres.base = 0 as *mut libc::c_char;
    fnres.size = 0 as libc::c_int;
    fnres.tail = 0 as libc::c_int;
    temp_fnres.base = 0 as *mut libc::c_char;
    temp_fnres.size = 0 as libc::c_int;
    temp_fnres.tail = 0 as libc::c_int;
    if !(opt.default_page).is_null() {
        index_filename = opt.default_page;
    }
    if !(opt.dir_prefix).is_null() {
        append_string(opt.dir_prefix, &mut fnres);
    }
    if opt.dirstruct {
        if opt.protocol_directories {
            if temp_fnres.tail != 0 {
                append_char('/' as i32 as libc::c_char, &mut temp_fnres);
            }
            append_string(supported_schemes[(*u).scheme as usize].name, &mut temp_fnres);
        }
        if opt.add_hostdir {
            if temp_fnres.tail != 0 {
                append_char('/' as i32 as libc::c_char, &mut temp_fnres);
            }
            if 0 as libc::c_int
                != strcmp((*u).host, b"..\0" as *const u8 as *const libc::c_char)
            {
                append_string((*u).host, &mut temp_fnres);
            } else {
                append_string(
                    b"%2E%2E\0" as *const u8 as *const libc::c_char,
                    &mut temp_fnres,
                );
            }
            if (*u).port != scheme_default_port((*u).scheme) {
                let mut portstr: [libc::c_char; 24] = [0; 24];
                number_to_string(portstr.as_mut_ptr(), (*u).port as wgint);
                append_char(
                    (if opt.restrict_files_os as libc::c_uint
                        != restrict_windows as libc::c_int as libc::c_uint
                    {
                        ':' as i32
                    } else {
                        '+' as i32
                    }) as libc::c_char,
                    &mut temp_fnres,
                );
                append_string(portstr.as_mut_ptr(), &mut temp_fnres);
            }
        }
        append_dir_structure(u, &mut temp_fnres);
    }
    if replaced_filename.is_null() {
        u_file = if *(*u).file as libc::c_int != 0 {
            (*u).file as *const libc::c_char
        } else {
            index_filename
        };
        if !((*u).query).is_null() {
            fname_len_check = concat_strings(
                u_file,
                if opt.restrict_files_os as libc::c_uint
                    != restrict_vms as libc::c_int as libc::c_uint
                    && opt.restrict_files_os as libc::c_uint
                        != restrict_windows as libc::c_int as libc::c_uint
                {
                    b"?\0" as *const u8 as *const libc::c_char
                } else {
                    b"@\0" as *const u8 as *const libc::c_char
                },
                (*u).query,
                0 as *mut libc::c_void,
            );
        } else {
            fname_len_check = strdupdelim(
                u_file,
                u_file.offset(strlen(u_file) as isize),
            );
        }
    } else {
        u_file = replaced_filename;
        fname_len_check = strdupdelim(u_file, u_file.offset(strlen(u_file) as isize));
    }
    if temp_fnres.tail != 0 {
        append_char('/' as i32 as libc::c_char, &mut temp_fnres);
    }
    append_uri_pathel(
        fname_len_check,
        fname_len_check.offset(strlen(fname_len_check) as isize),
        1 as libc::c_int != 0,
        &mut temp_fnres,
    );
    append_char('\0' as i32 as libc::c_char, &mut temp_fnres);
    fname = convert_fname(temp_fnres.base);
    temp_fnres.base = 0 as *mut libc::c_char;
    temp_fnres.size = 0 as libc::c_int;
    temp_fnres.tail = 0 as libc::c_int;
    append_string(fname, &mut temp_fnres);
    rpl_free(fname as *mut libc::c_void);
    fname = 0 as *mut libc::c_char;
    rpl_free(fname_len_check as *mut libc::c_void);
    fname_len_check = 0 as *mut libc::c_char;
    if fnres.tail != 0 {
        append_char('/' as i32 as libc::c_char, &mut fnres);
    }
    append_string(temp_fnres.base, &mut fnres);
    fname = fnres.base;
    rpl_free(temp_fnres.base as *mut libc::c_void);
    temp_fnres.base = 0 as *mut libc::c_char;
    if (opt.noclobber as libc::c_int != 0 || opt.always_rest as libc::c_int != 0
        || opt.timestamping as libc::c_int != 0 || opt.dirstruct as libc::c_int != 0
        || !(opt.output_document).is_null() || opt.backups > 0 as libc::c_int)
        && !(file_exists_p(fname, 0 as *mut file_stats_t) as libc::c_int != 0
            && !file_non_directory_p(fname))
    {
        unique = fname;
    } else {
        unique = unique_name_passthrough(fname);
        if unique != fname {
            rpl_free(fname as *mut libc::c_void);
            fname = 0 as *mut libc::c_char;
        }
    }
    return unique;
}
unsafe extern "C" fn path_simplify(
    mut scheme: url_scheme,
    mut path: *mut libc::c_char,
) -> bool {
    let mut h: *mut libc::c_char = path;
    let mut t: *mut libc::c_char = path;
    let mut beg: *mut libc::c_char = path;
    let mut end: *mut libc::c_char = strchr(path, '\0' as i32);
    while h < end {
        if *h.offset(0 as libc::c_int as isize) as libc::c_int == '.' as i32
            && (*h.offset(1 as libc::c_int as isize) as libc::c_int == '/' as i32
                || *h.offset(1 as libc::c_int as isize) as libc::c_int == '\0' as i32)
        {
            h = h.offset(2 as libc::c_int as isize);
        } else {
            let mut current_block_18: u64;
            if *h.offset(0 as libc::c_int as isize) as libc::c_int == '.' as i32
                && *h.offset(1 as libc::c_int as isize) as libc::c_int == '.' as i32
                && (*h.offset(2 as libc::c_int as isize) as libc::c_int == '/' as i32
                    || *h.offset(2 as libc::c_int as isize) as libc::c_int
                        == '\0' as i32)
            {
                if t > beg {
                    t = t.offset(-1);
                    t;
                    while t > beg
                        && *t.offset(-(1 as libc::c_int) as isize) as libc::c_int
                            != '/' as i32
                    {
                        t = t.offset(-1);
                        t;
                    }
                    current_block_18 = 2968425633554183086;
                } else if scheme as libc::c_uint
                    == SCHEME_FTP as libc::c_int as libc::c_uint
                {
                    beg = t.offset(3 as libc::c_int as isize);
                    current_block_18 = 15039305378162871812;
                } else {
                    current_block_18 = 2968425633554183086;
                }
                match current_block_18 {
                    15039305378162871812 => {}
                    _ => {
                        h = h.offset(3 as libc::c_int as isize);
                        current_block_18 = 11307063007268554308;
                    }
                }
            } else {
                current_block_18 = 15039305378162871812;
            }
            match current_block_18 {
                15039305378162871812 => {
                    if t == h {
                        while h < end && *h as libc::c_int != '/' as i32 {
                            t = t.offset(1);
                            t;
                            h = h.offset(1);
                            h;
                        }
                        if h < end {
                            t = t.offset(1);
                            t;
                            h = h.offset(1);
                            h;
                        }
                    } else {
                        while h < end && *h as libc::c_int != '/' as i32 {
                            let fresh24 = h;
                            h = h.offset(1);
                            let fresh25 = t;
                            t = t.offset(1);
                            *fresh25 = *fresh24;
                        }
                        if h < end {
                            let fresh26 = h;
                            h = h.offset(1);
                            let fresh27 = t;
                            t = t.offset(1);
                            *fresh27 = *fresh26;
                        }
                    }
                }
                _ => {}
            }
        }
    }
    if t != h {
        *t = '\0' as i32 as libc::c_char;
    }
    return t != h;
}
unsafe extern "C" fn path_end(mut url: *const libc::c_char) -> *const libc::c_char {
    let mut scheme: url_scheme = url_scheme(url);
    let mut seps: *const libc::c_char = 0 as *const libc::c_char;
    if scheme as libc::c_uint == SCHEME_INVALID as libc::c_int as libc::c_uint {
        scheme = SCHEME_HTTP;
    }
    seps = (init_seps(scheme)).offset(2 as libc::c_int as isize);
    return strpbrk_or_eos(url, seps);
}
pub unsafe extern "C" fn uri_merge(
    mut base: *const libc::c_char,
    mut link: *const libc::c_char,
) -> *mut libc::c_char {
    let mut linklength: libc::c_int = 0;
    let mut end: *const libc::c_char = 0 as *const libc::c_char;
    let mut merge: *mut libc::c_char = 0 as *mut libc::c_char;
    if url_has_scheme(link) {
        return xstrdup(link);
    }
    end = path_end(base);
    linklength = strlen(link) as libc::c_int;
    if *link == 0 {
        return xstrdup(base)
    } else if *link as libc::c_int == '?' as i32 {
        let mut baselength: libc::c_int = end.offset_from(base) as libc::c_long
            as libc::c_int;
        merge = xmalloc((baselength + linklength + 1 as libc::c_int) as size_t)
            as *mut libc::c_char;
        memcpy(
            merge as *mut libc::c_void,
            base as *const libc::c_void,
            baselength as libc::c_ulong,
        );
        memcpy(
            merge.offset(baselength as isize) as *mut libc::c_void,
            link as *const libc::c_void,
            linklength as libc::c_ulong,
        );
        *merge.offset((baselength + linklength) as isize) = '\0' as i32 as libc::c_char;
    } else if *link as libc::c_int == '#' as i32 {
        let mut baselength_0: libc::c_int = 0;
        let mut end1: *const libc::c_char = strchr(base, '#' as i32);
        if end1.is_null() {
            end1 = base.offset(strlen(base) as isize);
        }
        baselength_0 = end1.offset_from(base) as libc::c_long as libc::c_int;
        merge = xmalloc((baselength_0 + linklength + 1 as libc::c_int) as size_t)
            as *mut libc::c_char;
        memcpy(
            merge as *mut libc::c_void,
            base as *const libc::c_void,
            baselength_0 as libc::c_ulong,
        );
        memcpy(
            merge.offset(baselength_0 as isize) as *mut libc::c_void,
            link as *const libc::c_void,
            linklength as libc::c_ulong,
        );
        *merge
            .offset((baselength_0 + linklength) as isize) = '\0' as i32 as libc::c_char;
    } else if *link as libc::c_int == '/' as i32
        && *link.offset(1 as libc::c_int as isize) as libc::c_int == '/' as i32
    {
        let mut span: libc::c_int = 0;
        let mut slash: *const libc::c_char = 0 as *const libc::c_char;
        let mut start_insert: *const libc::c_char = 0 as *const libc::c_char;
        slash = memchr(
            base as *const libc::c_void,
            '/' as i32,
            end.offset_from(base) as libc::c_long as libc::c_ulong,
        ) as *const libc::c_char;
        if !slash.is_null()
            && *slash.offset(1 as libc::c_int as isize) as libc::c_int == '/' as i32
        {
            start_insert = slash;
        } else {
            start_insert = base;
        }
        span = start_insert.offset_from(base) as libc::c_long as libc::c_int;
        merge = xmalloc((span + linklength + 1 as libc::c_int) as size_t)
            as *mut libc::c_char;
        if span != 0 {
            memcpy(
                merge as *mut libc::c_void,
                base as *const libc::c_void,
                span as libc::c_ulong,
            );
        }
        memcpy(
            merge.offset(span as isize) as *mut libc::c_void,
            link as *const libc::c_void,
            linklength as libc::c_ulong,
        );
        *merge.offset((span + linklength) as isize) = '\0' as i32 as libc::c_char;
    } else if *link as libc::c_int == '/' as i32 {
        let mut span_0: libc::c_int = 0;
        let mut slash_0: *const libc::c_char = 0 as *const libc::c_char;
        let mut start_insert_0: *const libc::c_char = 0 as *const libc::c_char;
        let mut pos: *const libc::c_char = base;
        let mut seen_slash_slash: bool = 0 as libc::c_int != 0;
        loop {
            slash_0 = memchr(
                pos as *const libc::c_void,
                '/' as i32,
                end.offset_from(pos) as libc::c_long as libc::c_ulong,
            ) as *const libc::c_char;
            if !(!slash_0.is_null() && !seen_slash_slash) {
                break;
            }
            if !(*slash_0.offset(1 as libc::c_int as isize) as libc::c_int == '/' as i32)
            {
                break;
            }
            pos = slash_0.offset(2 as libc::c_int as isize);
            seen_slash_slash = 1 as libc::c_int != 0;
        }
        if slash_0.is_null() && !seen_slash_slash {
            start_insert_0 = base;
        } else if slash_0.is_null() && seen_slash_slash as libc::c_int != 0 {
            start_insert_0 = end;
        } else if !slash_0.is_null() && !seen_slash_slash {
            start_insert_0 = base;
        } else if !slash_0.is_null() && seen_slash_slash as libc::c_int != 0 {
            start_insert_0 = slash_0;
        }
        span_0 = start_insert_0.offset_from(base) as libc::c_long as libc::c_int;
        merge = xmalloc((span_0 + linklength + 1 as libc::c_int) as size_t)
            as *mut libc::c_char;
        if span_0 != 0 {
            memcpy(
                merge as *mut libc::c_void,
                base as *const libc::c_void,
                span_0 as libc::c_ulong,
            );
        }
        memcpy(
            merge.offset(span_0 as isize) as *mut libc::c_void,
            link as *const libc::c_void,
            linklength as libc::c_ulong,
        );
        *merge.offset((span_0 + linklength) as isize) = '\0' as i32 as libc::c_char;
    } else {
        let mut need_explicit_slash: bool = 0 as libc::c_int != 0;
        let mut span_1: libc::c_int = 0;
        let mut start_insert_1: *const libc::c_char = 0 as *const libc::c_char;
        let mut last_slash: *const libc::c_char = memrchr(
            base as *const libc::c_void,
            '/' as i32,
            end.offset_from(base) as libc::c_long as size_t,
        ) as *const libc::c_char;
        if last_slash.is_null() {
            start_insert_1 = base;
        } else if !last_slash.is_null()
            && last_slash >= base.offset(2 as libc::c_int as isize)
            && *last_slash.offset(-(2 as libc::c_int) as isize) as libc::c_int
                == ':' as i32
            && *last_slash.offset(-(1 as libc::c_int) as isize) as libc::c_int
                == '/' as i32
        {
            start_insert_1 = end.offset(1 as libc::c_int as isize);
            need_explicit_slash = 1 as libc::c_int != 0;
        } else {
            start_insert_1 = last_slash.offset(1 as libc::c_int as isize);
        }
        span_1 = start_insert_1.offset_from(base) as libc::c_long as libc::c_int;
        merge = xmalloc((span_1 + linklength + 1 as libc::c_int) as size_t)
            as *mut libc::c_char;
        if span_1 != 0 {
            memcpy(
                merge as *mut libc::c_void,
                base as *const libc::c_void,
                span_1 as libc::c_ulong,
            );
        }
        if need_explicit_slash {
            *merge
                .offset(
                    (span_1 - 1 as libc::c_int) as isize,
                ) = '/' as i32 as libc::c_char;
        }
        memcpy(
            merge.offset(span_1 as isize) as *mut libc::c_void,
            link as *const libc::c_void,
            linklength as libc::c_ulong,
        );
        *merge.offset((span_1 + linklength) as isize) = '\0' as i32 as libc::c_char;
    }
    return merge;
}
pub unsafe extern "C" fn url_string(
    mut url: *const url,
    mut auth_mode: url_auth_mode,
) -> *mut libc::c_char {
    let mut size: libc::c_int = 0;
    let mut result: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut p: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut quoted_host: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut quoted_user: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut quoted_passwd: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut scheme_port: libc::c_int = supported_schemes[(*url).scheme as usize]
        .default_port;
    let mut scheme_str: *const libc::c_char = supported_schemes[(*url).scheme as usize]
        .leading_string;
    let mut fplen: libc::c_int = full_path_length(url);
    let mut brackets_around_host: bool = false;
    if !((*url).user).is_null() {
        if auth_mode as libc::c_uint != URL_AUTH_HIDE as libc::c_int as libc::c_uint {
            quoted_user = url_escape_allow_passthrough((*url).user);
            if !((*url).passwd).is_null() {
                if auth_mode as libc::c_uint
                    == URL_AUTH_HIDE_PASSWD as libc::c_int as libc::c_uint
                {
                    quoted_passwd = b"*password*\0" as *const u8 as *const libc::c_char
                        as *mut libc::c_char;
                } else {
                    quoted_passwd = url_escape_allow_passthrough((*url).passwd);
                }
            }
        }
    }
    quoted_host = url_escape_allow_passthrough((*url).host);
    if quoted_host != (*url).host {
        unescape_single_char(quoted_host, ':' as i32 as libc::c_char);
    }
    brackets_around_host = !(strchr(quoted_host, ':' as i32)).is_null();
    size = (strlen(scheme_str))
        .wrapping_add(strlen(quoted_host))
        .wrapping_add(
            (if brackets_around_host as libc::c_int != 0 {
                2 as libc::c_int
            } else {
                0 as libc::c_int
            }) as libc::c_ulong,
        )
        .wrapping_add(fplen as libc::c_ulong)
        .wrapping_add(1 as libc::c_int as libc::c_ulong) as libc::c_int;
    if (*url).port != scheme_port {
        size += 1 as libc::c_int + numdigit((*url).port as wgint);
    }
    if !quoted_user.is_null() {
        size = (size as libc::c_ulong)
            .wrapping_add(
                (1 as libc::c_int as libc::c_ulong).wrapping_add(strlen(quoted_user)),
            ) as libc::c_int as libc::c_int;
        if !quoted_passwd.is_null() {
            size = (size as libc::c_ulong)
                .wrapping_add(
                    (1 as libc::c_int as libc::c_ulong)
                        .wrapping_add(strlen(quoted_passwd)),
                ) as libc::c_int as libc::c_int;
        }
    }
    result = xmalloc(size as size_t) as *mut libc::c_char;
    p = result;
    let mut len: libc::c_int = strlen(scheme_str) as libc::c_int;
    memcpy(
        p as *mut libc::c_void,
        scheme_str as *const libc::c_void,
        len as libc::c_ulong,
    );
    p = p.offset(len as isize);
    if !quoted_user.is_null() {
        let mut len_0: libc::c_int = strlen(quoted_user) as libc::c_int;
        memcpy(
            p as *mut libc::c_void,
            quoted_user as *const libc::c_void,
            len_0 as libc::c_ulong,
        );
        p = p.offset(len_0 as isize);
        if !quoted_passwd.is_null() {
            let fresh28 = p;
            p = p.offset(1);
            *fresh28 = ':' as i32 as libc::c_char;
            let mut len_1: libc::c_int = strlen(quoted_passwd) as libc::c_int;
            memcpy(
                p as *mut libc::c_void,
                quoted_passwd as *const libc::c_void,
                len_1 as libc::c_ulong,
            );
            p = p.offset(len_1 as isize);
        }
        let fresh29 = p;
        p = p.offset(1);
        *fresh29 = '@' as i32 as libc::c_char;
    }
    if brackets_around_host {
        let fresh30 = p;
        p = p.offset(1);
        *fresh30 = '[' as i32 as libc::c_char;
    }
    let mut len_2: libc::c_int = strlen(quoted_host) as libc::c_int;
    memcpy(
        p as *mut libc::c_void,
        quoted_host as *const libc::c_void,
        len_2 as libc::c_ulong,
    );
    p = p.offset(len_2 as isize);
    if brackets_around_host {
        let fresh31 = p;
        p = p.offset(1);
        *fresh31 = ']' as i32 as libc::c_char;
    }
    if (*url).port != scheme_port {
        let fresh32 = p;
        p = p.offset(1);
        *fresh32 = ':' as i32 as libc::c_char;
        p = number_to_string(p, (*url).port as wgint);
    }
    full_path_write(url, p);
    p = p.offset(fplen as isize);
    let fresh33 = p;
    p = p.offset(1);
    *fresh33 = '\0' as i32 as libc::c_char;
    if !quoted_user.is_null() && quoted_user != (*url).user {
        rpl_free(quoted_user as *mut libc::c_void);
        quoted_user = 0 as *mut libc::c_char;
    }
    if !quoted_passwd.is_null()
        && auth_mode as libc::c_uint == URL_AUTH_SHOW as libc::c_int as libc::c_uint
        && quoted_passwd != (*url).passwd
    {
        rpl_free(quoted_passwd as *mut libc::c_void);
        quoted_passwd = 0 as *mut libc::c_char;
    }
    if quoted_host != (*url).host {
        rpl_free(quoted_host as *mut libc::c_void);
        quoted_host = 0 as *mut libc::c_char;
    }
    return result;
}
pub unsafe extern "C" fn schemes_are_similar_p(
    mut a: url_scheme,
    mut b: url_scheme,
) -> bool {
    if a as libc::c_uint == b as libc::c_uint {
        return 1 as libc::c_int != 0;
    }
    return 0 as libc::c_int != 0;
}
unsafe extern "C" fn getchar_from_escaped_string(
    mut str: *const libc::c_char,
    mut c: *mut libc::c_char,
) -> libc::c_int {
    let mut p: *const libc::c_char = str;
    if *p.offset(0 as libc::c_int as isize) as libc::c_int == '%' as i32 {
        if !c_isxdigit(*p.offset(1 as libc::c_int as isize) as libc::c_int)
            || !c_isxdigit(*p.offset(2 as libc::c_int as isize) as libc::c_int)
        {
            *c = '%' as i32 as libc::c_char;
            return 1 as libc::c_int;
        } else {
            if *p.offset(2 as libc::c_int as isize) as libc::c_int == 0 as libc::c_int {
                return 0 as libc::c_int;
            }
            *c = (((_unhex(*p.offset(1 as libc::c_int as isize) as libc::c_uchar)
                as libc::c_int) << 4 as libc::c_int)
                + _unhex(*p.offset(2 as libc::c_int as isize) as libc::c_uchar)
                    as libc::c_int) as libc::c_char;
            if urlchr_table[*c as libc::c_uchar as usize] as libc::c_int
                & urlchr_reserved as libc::c_int != 0
            {
                *c = '%' as i32 as libc::c_char;
                return 1 as libc::c_int;
            } else {
                return 3 as libc::c_int
            }
        }
    } else {
        *c = *p.offset(0 as libc::c_int as isize);
    }
    return 1 as libc::c_int;
}
pub unsafe extern "C" fn are_urls_equal(
    mut u1: *const libc::c_char,
    mut u2: *const libc::c_char,
) -> bool {
    let mut p: *const libc::c_char = 0 as *const libc::c_char;
    let mut q: *const libc::c_char = 0 as *const libc::c_char;
    let mut pp: libc::c_int = 0;
    let mut qq: libc::c_int = 0;
    let mut ch1: libc::c_char = 0;
    let mut ch2: libc::c_char = 0;
    p = u1;
    q = u2;
    while *p as libc::c_int != 0 && *q as libc::c_int != 0
        && {
            pp = getchar_from_escaped_string(p, &mut ch1);
            pp != 0
        }
        && {
            qq = getchar_from_escaped_string(q, &mut ch2);
            qq != 0
        } && c_tolower(ch1 as libc::c_int) == c_tolower(ch2 as libc::c_int)
    {
        p = p.offset(pp as isize);
        q = q.offset(qq as isize);
    }
    return if *p as libc::c_int == 0 as libc::c_int
        && *q as libc::c_int == 0 as libc::c_int
    {
        1 as libc::c_int
    } else {
        0 as libc::c_int
    } != 0;
}
