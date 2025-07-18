use ::libc;
use ::c2rust_bitfields;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    pub type hash_table;
    pub type robot_specs;
    fn fclose(__stream: *mut FILE) -> libc::c_int;
    fn fprintf(_: *mut FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn dcgettext(
        __domainname: *const libc::c_char,
        __msgid: *const libc::c_char,
        __category: libc::c_int,
    ) -> *mut libc::c_char;
    fn strerror(_: libc::c_int) -> *mut libc::c_char;
    fn strcasecmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    fn rpl_free(_: *mut libc::c_void);
    static mut opt: options;
    fn xmalloc(s: size_t) -> *mut libc::c_void;
    fn xcalloc(n: size_t, s: size_t) -> *mut libc::c_void;
    fn xstrdup(str: *const libc::c_char) -> *mut libc::c_char;
    fn rpl_fopen(filename: *const libc::c_char, mode: *const libc::c_char) -> *mut FILE;
    fn logprintf(_: log_options, _: *const libc::c_char, _: ...);
    fn debug_logprintf(_: *const libc::c_char, _: ...);
    fn logputs(_: log_options, _: *const libc::c_char);
    fn quote_n(n: libc::c_int, arg: *const libc::c_char) -> *const libc::c_char;
    fn quote(arg: *const libc::c_char) -> *const libc::c_char;
    fn quotearg_n_style(
        n: libc::c_int,
        s: quoting_style,
        arg: *const libc::c_char,
    ) -> *mut libc::c_char;
    fn iri_new() -> *mut iri;
    fn iri_free(i: *mut iri);
    fn set_uri_encoding(i: *mut iri, charset: *const libc::c_char, force: bool);
    fn unlink(__name: *const libc::c_char) -> libc::c_int;
    fn __errno_location() -> *mut libc::c_int;
    fn url_escape(_: *const libc::c_char) -> *mut libc::c_char;
    fn url_unescape(_: *mut libc::c_char);
    fn url_parse(
        _: *const libc::c_char,
        _: *mut libc::c_int,
        iri: *mut iri,
        percent_encode: bool,
    ) -> *mut url;
    fn url_error(_: libc::c_int) -> *const libc::c_char;
    fn url_free(_: *mut url);
    fn url_string(_: *const url, _: url_auth_mode) -> *mut libc::c_char;
    fn schemes_are_similar_p(a: url_scheme, b: url_scheme) -> bool;
    fn subdir_p(_: *const libc::c_char, _: *const libc::c_char) -> bool;
    fn acceptable(_: *const libc::c_char) -> bool;
    fn accept_url(_: *const libc::c_char) -> bool;
    fn accdir(s: *const libc::c_char) -> bool;
    fn match_tail(_: *const libc::c_char, _: *const libc::c_char, _: bool) -> bool;
    fn has_html_suffix_p(_: *const libc::c_char) -> bool;
    fn string_set_add(_: *mut hash_table, _: *const libc::c_char);
    fn string_set_contains(_: *mut hash_table, _: *const libc::c_char) -> libc::c_int;
    fn string_set_free(_: *mut hash_table);
    static mut total_downloaded_bytes: wgint;
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
    fn accept_domain(_: *mut url) -> bool;
    fn hash_table_get(_: *const hash_table, _: *const libc::c_void) -> *mut libc::c_void;
    fn hash_table_contains(_: *const hash_table, _: *const libc::c_void) -> libc::c_int;
    fn make_string_hash_table(_: libc::c_int) -> *mut hash_table;
    fn res_parse(_: *const libc::c_char, _: libc::c_int) -> *mut robot_specs;
    fn res_parse_from_file(_: *const libc::c_char) -> *mut robot_specs;
    fn res_match_path(_: *const robot_specs, _: *const libc::c_char) -> bool;
    fn res_register_specs(_: *const libc::c_char, _: libc::c_int, _: *mut robot_specs);
    fn res_get_specs(_: *const libc::c_char, _: libc::c_int) -> *mut robot_specs;
    fn res_retrieve_file(
        _: *const libc::c_char,
        _: *mut *mut libc::c_char,
        _: *mut iri,
    ) -> bool;
    static mut dl_url_file_map: *mut hash_table;
    static mut downloaded_html_set: *mut hash_table;
    static mut downloaded_css_set: *mut hash_table;
    fn register_delete_file(_: *const libc::c_char);
    fn get_urls_html(
        _: *const libc::c_char,
        _: *const libc::c_char,
        _: *mut bool,
        _: *mut iri,
    ) -> *mut urlpos;
    fn free_urlpos(_: *mut urlpos);
    fn get_urls_css_file(_: *const libc::c_char, _: *const libc::c_char) -> *mut urlpos;
    fn inform_exit_status(err: uerr_t);
}
pub type __int64_t = libc::c_long;
pub type __off_t = libc::c_long;
pub type __off64_t = libc::c_long;
pub type size_t = libc::c_ulong;
pub type int64_t = __int64_t;
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
#[derive(Copy, Clone, BitfieldStruct)]
#[repr(C)]
pub struct urlpos {
    pub url: *mut url,
    pub local_name: *mut libc::c_char,
    #[bitfield(name = "ignore_when_downloading", ty = "libc::c_uint", bits = "0..=0")]
    #[bitfield(name = "link_relative_p", ty = "libc::c_uint", bits = "1..=1")]
    #[bitfield(name = "link_complete_p", ty = "libc::c_uint", bits = "2..=2")]
    #[bitfield(name = "link_base_p", ty = "libc::c_uint", bits = "3..=3")]
    #[bitfield(name = "link_inline_p", ty = "libc::c_uint", bits = "4..=4")]
    #[bitfield(name = "link_css_p", ty = "libc::c_uint", bits = "5..=5")]
    #[bitfield(name = "link_noquote_html_p", ty = "libc::c_uint", bits = "6..=6")]
    #[bitfield(name = "link_expect_html", ty = "libc::c_uint", bits = "7..=7")]
    #[bitfield(name = "link_expect_css", ty = "libc::c_uint", bits = "8..=8")]
    #[bitfield(name = "link_refresh_p", ty = "libc::c_uint", bits = "9..=9")]
    pub ignore_when_downloading_link_relative_p_link_complete_p_link_base_p_link_inline_p_link_css_p_link_noquote_html_p_link_expect_html_link_expect_css_link_refresh_p: [u8; 2],
    #[bitfield(padding)]
    pub c2rust_padding: [u8; 2],
    pub refresh_timeout: libc::c_int,
    pub convert: convert_options,
    pub pos: libc::c_int,
    pub size: libc::c_int,
    pub next: *mut urlpos,
}
pub type convert_options = libc::c_uint;
pub const CO_NULLIFY_BASE: convert_options = 4;
pub const CO_CONVERT_TO_COMPLETE: convert_options = 3;
pub const CO_CONVERT_BASENAME_ONLY: convert_options = 2;
pub const CO_CONVERT_TO_RELATIVE: convert_options = 1;
pub const CO_NOCONVERT: convert_options = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct url_queue {
    pub head: *mut queue_element,
    pub tail: *mut queue_element,
    pub count: libc::c_int,
    pub maxcount: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct queue_element {
    pub url: *const libc::c_char,
    pub referer: *const libc::c_char,
    pub depth: libc::c_int,
    pub html_allowed: bool,
    pub iri: *mut iri,
    pub css_allowed: bool,
    pub next: *mut queue_element,
}
pub type reject_reason = libc::c_uint;
pub const WG_RR_ROBOTS: reject_reason = 11;
pub const WG_RR_SPANNEDHOST: reject_reason = 10;
pub const WG_RR_RULES: reject_reason = 9;
pub const WG_RR_REGEX: reject_reason = 8;
pub const WG_RR_LIST: reject_reason = 7;
pub const WG_RR_PARENT: reject_reason = 6;
pub const WG_RR_DOMAIN: reject_reason = 5;
pub const WG_RR_ABSOLUTE: reject_reason = 4;
pub const WG_RR_NONHTTP: reject_reason = 3;
pub const WG_RR_NOTHTTPS: reject_reason = 2;
pub const WG_RR_BLACKLIST: reject_reason = 1;
pub const WG_RR_SUCCESS: reject_reason = 0;
unsafe extern "C" fn url_queue_new() -> *mut url_queue {
    let mut queue: *mut url_queue = xcalloc(
        1 as libc::c_int as size_t,
        ::std::mem::size_of::<url_queue>() as libc::c_ulong,
    ) as *mut url_queue;
    return queue;
}
unsafe extern "C" fn url_queue_delete(mut queue: *mut url_queue) {
    rpl_free(queue as *mut libc::c_void);
    queue = 0 as *mut url_queue;
}
unsafe extern "C" fn url_enqueue(
    mut queue: *mut url_queue,
    mut i: *mut iri,
    mut url: *const libc::c_char,
    mut referer: *const libc::c_char,
    mut depth: libc::c_int,
    mut html_allowed: bool,
    mut css_allowed: bool,
) {
    let mut qel: *mut queue_element = xmalloc(
        ::std::mem::size_of::<queue_element>() as libc::c_ulong,
    ) as *mut queue_element;
    (*qel).iri = i;
    (*qel).url = url;
    (*qel).referer = referer;
    (*qel).depth = depth;
    (*qel).html_allowed = html_allowed;
    (*qel).css_allowed = css_allowed;
    (*qel).next = 0 as *mut queue_element;
    (*queue).count += 1;
    (*queue).count;
    if (*queue).count > (*queue).maxcount {
        (*queue).maxcount = (*queue).count;
    }
    if opt.debug as libc::c_long != 0 {
        debug_logprintf(
            b"Enqueuing %s at depth %d\n\0" as *const u8 as *const libc::c_char,
            quotearg_n_style(0 as libc::c_int, escape_quoting_style, url),
            depth,
        );
    }
    if opt.debug as libc::c_long != 0 {
        debug_logprintf(
            b"Queue count %d, maxcount %d.\n\0" as *const u8 as *const libc::c_char,
            (*queue).count,
            (*queue).maxcount,
        );
    }
    if !i.is_null() {
        if opt.debug as libc::c_long != 0 {
            debug_logprintf(
                b"[IRI Enqueuing %s with %s\n\0" as *const u8 as *const libc::c_char,
                quote_n(0 as libc::c_int, url),
                if !((*i).uri_encoding).is_null() {
                    quote_n(1 as libc::c_int, (*i).uri_encoding)
                } else {
                    b"None\0" as *const u8 as *const libc::c_char
                },
            );
        }
    }
    if !((*queue).tail).is_null() {
        (*(*queue).tail).next = qel;
    }
    (*queue).tail = qel;
    if ((*queue).head).is_null() {
        (*queue).head = (*queue).tail;
    }
}
unsafe extern "C" fn url_dequeue(
    mut queue: *mut url_queue,
    mut i: *mut *mut iri,
    mut url: *mut *const libc::c_char,
    mut referer: *mut *const libc::c_char,
    mut depth: *mut libc::c_int,
    mut html_allowed: *mut bool,
    mut css_allowed: *mut bool,
) -> bool {
    let mut qel: *mut queue_element = (*queue).head;
    if qel.is_null() {
        return 0 as libc::c_int != 0;
    }
    (*queue).head = (*(*queue).head).next;
    if ((*queue).head).is_null() {
        (*queue).tail = 0 as *mut queue_element;
    }
    *i = (*qel).iri;
    *url = (*qel).url;
    *referer = (*qel).referer;
    *depth = (*qel).depth;
    *html_allowed = (*qel).html_allowed;
    *css_allowed = (*qel).css_allowed;
    (*queue).count -= 1;
    (*queue).count;
    if opt.debug as libc::c_long != 0 {
        debug_logprintf(
            b"Dequeuing %s at depth %d\n\0" as *const u8 as *const libc::c_char,
            quotearg_n_style(0 as libc::c_int, escape_quoting_style, (*qel).url),
            (*qel).depth,
        );
    }
    if opt.debug as libc::c_long != 0 {
        debug_logprintf(
            b"Queue count %d, maxcount %d.\n\0" as *const u8 as *const libc::c_char,
            (*queue).count,
            (*queue).maxcount,
        );
    }
    rpl_free(qel as *mut libc::c_void);
    qel = 0 as *mut queue_element;
    return 1 as libc::c_int != 0;
}
unsafe extern "C" fn blacklist_add(
    mut blacklist: *mut hash_table,
    mut url: *const libc::c_char,
) {
    let mut url_unescaped: *mut libc::c_char = xstrdup(url);
    url_unescape(url_unescaped);
    string_set_add(blacklist, url_unescaped);
    rpl_free(url_unescaped as *mut libc::c_void);
    url_unescaped = 0 as *mut libc::c_char;
}
unsafe extern "C" fn blacklist_contains(
    mut blacklist: *mut hash_table,
    mut url: *const libc::c_char,
) -> libc::c_int {
    let mut url_unescaped: *mut libc::c_char = xstrdup(url);
    let mut ret: libc::c_int = 0;
    url_unescape(url_unescaped);
    ret = string_set_contains(blacklist, url_unescaped);
    rpl_free(url_unescaped as *mut libc::c_void);
    url_unescaped = 0 as *mut libc::c_char;
    return ret;
}
pub unsafe extern "C" fn retrieve_tree(
    mut start_url_parsed: *mut url,
    mut pi: *mut iri,
) -> uerr_t {
    let mut status: uerr_t = RETROK;
    let mut queue: *mut url_queue = 0 as *mut url_queue;
    let mut blacklist: *mut hash_table = 0 as *mut hash_table;
    let mut i: *mut iri = iri_new();
    let mut rejectedlog: *mut FILE = 0 as *mut FILE;
    if !pi.is_null() {
        (*i)
            .uri_encoding = if !((*pi).uri_encoding).is_null() {
            xstrdup((*pi).uri_encoding)
        } else {
            0 as *mut libc::c_char
        };
        (*i)
            .content_encoding = if !((*pi).content_encoding).is_null() {
            xstrdup((*pi).content_encoding)
        } else {
            0 as *mut libc::c_char
        };
        (*i).utf8_encode = (*pi).utf8_encode;
    } else {
        set_uri_encoding(i, opt.locale, 1 as libc::c_int != 0);
    }
    queue = url_queue_new();
    blacklist = make_string_hash_table(0 as libc::c_int);
    url_enqueue(
        queue,
        i,
        xstrdup((*start_url_parsed).url),
        0 as *const libc::c_char,
        0 as libc::c_int,
        1 as libc::c_int != 0,
        0 as libc::c_int != 0,
    );
    blacklist_add(blacklist, (*start_url_parsed).url);
    if !(opt.rejected_log).is_null() {
        rejectedlog = rpl_fopen(
            opt.rejected_log,
            b"w\0" as *const u8 as *const libc::c_char,
        );
        write_reject_log_header(rejectedlog);
        if rejectedlog.is_null() {
            logprintf(
                LOG_NOTQUIET,
                b"%s: %s\n\0" as *const u8 as *const libc::c_char,
                opt.rejected_log,
                strerror(*__errno_location()),
            );
        }
    }
    loop {
        let mut descend: bool = 0 as libc::c_int != 0;
        let mut url: *mut libc::c_char = 0 as *mut libc::c_char;
        let mut referer: *mut libc::c_char = 0 as *mut libc::c_char;
        let mut file: *mut libc::c_char = 0 as *mut libc::c_char;
        let mut depth: libc::c_int = 0;
        let mut html_allowed: bool = false;
        let mut css_allowed: bool = false;
        let mut is_css: bool = 0 as libc::c_int != 0;
        let mut dash_p_leaf_HTML: bool = 0 as libc::c_int != 0;
        if opt.quota != 0 && total_downloaded_bytes > opt.quota {
            break;
        }
        if status as libc::c_uint == FWRITEERR as libc::c_int as libc::c_uint {
            break;
        }
        if !url_dequeue(
            queue,
            &mut i as *mut *mut iri,
            &mut url as *mut *mut libc::c_char as *mut *const libc::c_char,
            &mut referer as *mut *mut libc::c_char as *mut *const libc::c_char,
            &mut depth,
            &mut html_allowed,
            &mut css_allowed,
        ) {
            break;
        }
        if !dl_url_file_map.is_null()
            && hash_table_contains(dl_url_file_map, url as *const libc::c_void) != 0
        {
            let mut is_css_bool: bool = false;
            file = xstrdup(
                hash_table_get(dl_url_file_map, url as *const libc::c_void)
                    as *const libc::c_char,
            );
            if opt.debug as libc::c_long != 0 {
                debug_logprintf(
                    b"Already downloaded \"%s\", reusing it from \"%s\".\n\0"
                        as *const u8 as *const libc::c_char,
                    url,
                    file,
                );
            }
            is_css_bool = css_allowed as libc::c_int != 0
                && !downloaded_css_set.is_null()
                && string_set_contains(downloaded_css_set, file) != 0;
            if is_css_bool as libc::c_int != 0
                || html_allowed as libc::c_int != 0 && !downloaded_html_set.is_null()
                    && string_set_contains(downloaded_html_set, file) != 0
            {
                descend = 1 as libc::c_int != 0;
                is_css = is_css_bool;
            }
        } else {
            let mut dt: libc::c_int = 0 as libc::c_int;
            let mut url_err: libc::c_int = 0;
            let mut redirected: *mut libc::c_char = 0 as *mut libc::c_char;
            let mut url_parsed: *mut url = url_parse(
                url,
                &mut url_err,
                i,
                1 as libc::c_int != 0,
            );
            if url_parsed.is_null() {
                logprintf(
                    LOG_NOTQUIET,
                    b"%s: %s.\n\0" as *const u8 as *const libc::c_char,
                    url,
                    url_error(url_err),
                );
                inform_exit_status(URLERROR);
            } else {
                status = retrieve_url(
                    url_parsed,
                    url,
                    &mut file,
                    &mut redirected,
                    referer,
                    &mut dt,
                    0 as libc::c_int != 0,
                    i,
                    1 as libc::c_int != 0,
                );
                if html_allowed as libc::c_int != 0 && !file.is_null()
                    && status as libc::c_uint == RETROK as libc::c_int as libc::c_uint
                    && dt & RETROKF as libc::c_int != 0
                    && dt & TEXTHTML as libc::c_int != 0
                {
                    descend = 1 as libc::c_int != 0;
                    is_css = 0 as libc::c_int != 0;
                }
                if !file.is_null()
                    && status as libc::c_uint == RETROK as libc::c_int as libc::c_uint
                    && dt & RETROKF as libc::c_int != 0
                    && (dt & TEXTCSS as libc::c_int != 0
                        || css_allowed as libc::c_int != 0)
                {
                    descend = 1 as libc::c_int != 0;
                    is_css = 1 as libc::c_int != 0;
                }
                if !redirected.is_null() {
                    if descend {
                        let mut r: reject_reason = descend_redirect(
                            redirected,
                            url_parsed,
                            depth,
                            start_url_parsed,
                            blacklist,
                            i,
                        );
                        if r as libc::c_uint
                            == WG_RR_SUCCESS as libc::c_int as libc::c_uint
                        {
                            blacklist_add(blacklist, url);
                        } else {
                            write_reject_log_reason(
                                rejectedlog,
                                r,
                                url_parsed,
                                start_url_parsed,
                            );
                            descend = 0 as libc::c_int != 0;
                        }
                    }
                    rpl_free(url as *mut libc::c_void);
                    url = 0 as *mut libc::c_char;
                    url = redirected;
                } else {
                    rpl_free(url as *mut libc::c_void);
                    url = 0 as *mut libc::c_char;
                    url = xstrdup((*url_parsed).url);
                }
                url_free(url_parsed);
            }
        }
        opt.spider;
        if descend as libc::c_int != 0 && depth >= opt.reclevel
            && opt.reclevel != -(1 as libc::c_int)
        {
            if opt.page_requisites as libc::c_int != 0
                && (depth == opt.reclevel || depth == opt.reclevel + 1 as libc::c_int)
            {
                dash_p_leaf_HTML = 1 as libc::c_int != 0;
            } else {
                if opt.debug as libc::c_long != 0 {
                    debug_logprintf(
                        b"Not descending further; at depth %d, max. %d.\n\0" as *const u8
                            as *const libc::c_char,
                        depth,
                        opt.reclevel,
                    );
                }
                descend = 0 as libc::c_int != 0;
            }
        }
        if descend {
            let mut meta_disallow_follow: bool = 0 as libc::c_int != 0;
            let mut children: *mut urlpos = if is_css as libc::c_int != 0 {
                get_urls_css_file(file, url)
            } else {
                get_urls_html(file, url, &mut meta_disallow_follow, i)
            };
            if opt.use_robots as libc::c_int != 0
                && meta_disallow_follow as libc::c_int != 0
            {
                logprintf(
                    LOG_VERBOSE,
                    dcgettext(
                        0 as *const libc::c_char,
                        b"nofollow attribute found in %s. Will not follow any links on this page\n\0"
                            as *const u8 as *const libc::c_char,
                        5 as libc::c_int,
                    ),
                    file,
                );
                free_urlpos(children);
                children = 0 as *mut urlpos;
            }
            if !children.is_null() {
                let mut child: *mut urlpos = children;
                let mut url_parsed_0: *mut url = url_parse(
                    url,
                    0 as *mut libc::c_int,
                    i,
                    1 as libc::c_int != 0,
                );
                let mut ci: *mut iri = 0 as *mut iri;
                let mut referer_url: *mut libc::c_char = url;
                let mut strip_auth: bool = false;
                if url_parsed_0.is_null() {
                    continue;
                }
                strip_auth = !url_parsed_0.is_null()
                    && !((*url_parsed_0).user).is_null();
                if strip_auth {
                    referer_url = url_string(url_parsed_0, URL_AUTH_HIDE);
                }
                while !child.is_null() {
                    let mut r_0: reject_reason = WG_RR_SUCCESS;
                    if (*child).ignore_when_downloading() != 0 {
                        if opt.debug as libc::c_long != 0 {
                            debug_logprintf(
                                b"Not following due to 'ignore' flag: %s\n\0" as *const u8
                                    as *const libc::c_char,
                                (*(*child).url).url,
                            );
                        }
                    } else if dash_p_leaf_HTML as libc::c_int != 0
                        && (*child).link_inline_p() == 0
                    {
                        if opt.debug as libc::c_long != 0 {
                            debug_logprintf(
                                b"Not following due to 'link inline' flag: %s\n\0"
                                    as *const u8 as *const libc::c_char,
                                (*(*child).url).url,
                            );
                        }
                    } else {
                        r_0 = download_child(
                            child,
                            url_parsed_0,
                            depth,
                            start_url_parsed,
                            blacklist,
                            i,
                        );
                        if r_0 as libc::c_uint
                            == WG_RR_SUCCESS as libc::c_int as libc::c_uint
                        {
                            ci = iri_new();
                            set_uri_encoding(
                                ci,
                                (*i).content_encoding,
                                0 as libc::c_int != 0,
                            );
                            url_enqueue(
                                queue,
                                ci,
                                xstrdup((*(*child).url).url),
                                xstrdup(referer_url),
                                depth + 1 as libc::c_int,
                                (*child).link_expect_html() != 0,
                                (*child).link_expect_css() != 0,
                            );
                            blacklist_add(blacklist, (*(*child).url).url);
                        } else {
                            write_reject_log_reason(
                                rejectedlog,
                                r_0,
                                (*child).url,
                                url_parsed_0,
                            );
                        }
                    }
                    child = (*child).next;
                }
                if strip_auth {
                    rpl_free(referer_url as *mut libc::c_void);
                    referer_url = 0 as *mut libc::c_char;
                }
                url_free(url_parsed_0);
                free_urlpos(children);
            }
        }
        if !file.is_null()
            && (opt.delete_after as libc::c_int != 0 || opt.spider as libc::c_int != 0
                || !acceptable(file))
        {
            if opt.debug as libc::c_long != 0 {
                debug_logprintf(
                    b"Removing file due to %s in recursive_retrieve():\n\0" as *const u8
                        as *const libc::c_char,
                    if opt.delete_after as libc::c_int != 0 {
                        b"--delete-after\0" as *const u8 as *const libc::c_char
                    } else if opt.spider as libc::c_int != 0 {
                        b"--spider\0" as *const u8 as *const libc::c_char
                    } else {
                        b"recursive rejection criteria\0" as *const u8
                            as *const libc::c_char
                    },
                );
            }
            logprintf(
                LOG_VERBOSE,
                if opt.delete_after as libc::c_int != 0 || opt.spider as libc::c_int != 0
                {
                    dcgettext(
                        0 as *const libc::c_char,
                        b"Removing %s.\n\0" as *const u8 as *const libc::c_char,
                        5 as libc::c_int,
                    )
                } else {
                    dcgettext(
                        0 as *const libc::c_char,
                        b"Removing %s since it should be rejected.\n\0" as *const u8
                            as *const libc::c_char,
                        5 as libc::c_int,
                    )
                },
                file,
            );
            if unlink(file) != 0 {
                logprintf(
                    LOG_NOTQUIET,
                    b"unlink: %s\n\0" as *const u8 as *const libc::c_char,
                    strerror(*__errno_location()),
                );
            }
            logputs(LOG_VERBOSE, b"\n\0" as *const u8 as *const libc::c_char);
            register_delete_file(file);
        }
        rpl_free(url as *mut libc::c_void);
        url = 0 as *mut libc::c_char;
        rpl_free(referer as *mut libc::c_void);
        referer = 0 as *mut libc::c_char;
        rpl_free(file as *mut libc::c_void);
        file = 0 as *mut libc::c_char;
        iri_free(i);
    }
    if !rejectedlog.is_null() {
        fclose(rejectedlog);
        rejectedlog = 0 as *mut FILE;
    }
    let mut d1: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut d2: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut d3: libc::c_int = 0;
    let mut d4: bool = false;
    let mut d5: bool = false;
    let mut d6: *mut iri = 0 as *mut iri;
    while url_dequeue(
        queue,
        &mut d6 as *mut *mut iri,
        &mut d1 as *mut *mut libc::c_char as *mut *const libc::c_char,
        &mut d2 as *mut *mut libc::c_char as *mut *const libc::c_char,
        &mut d3,
        &mut d4,
        &mut d5,
    ) {
        iri_free(d6);
        rpl_free(d1 as *mut libc::c_void);
        d1 = 0 as *mut libc::c_char;
        rpl_free(d2 as *mut libc::c_void);
        d2 = 0 as *mut libc::c_char;
    }
    url_queue_delete(queue);
    string_set_free(blacklist);
    if opt.quota != 0 && total_downloaded_bytes > opt.quota {
        return QUOTEXC
    } else if status as libc::c_uint == FWRITEERR as libc::c_int as libc::c_uint {
        return FWRITEERR
    } else {
        return RETROK
    };
}
unsafe extern "C" fn download_child(
    mut upos: *const urlpos,
    mut parent: *mut url,
    mut depth: libc::c_int,
    mut start_url_parsed: *mut url,
    mut blacklist: *mut hash_table,
    mut iri: *mut iri,
) -> reject_reason {
    let mut current_block: u64;
    let mut u: *mut url = (*upos).url;
    let mut url: *const libc::c_char = (*u).url;
    let mut u_scheme_like_http: bool = false;
    let mut reason: reject_reason = WG_RR_SUCCESS;
    if opt.debug as libc::c_long != 0 {
        debug_logprintf(
            b"Deciding whether to enqueue \"%s\".\n\0" as *const u8
                as *const libc::c_char,
            url,
        );
    }
    if blacklist_contains(blacklist, url) != 0 {
        if opt.spider {
            let mut referrer: *mut libc::c_char = url_string(
                parent,
                URL_AUTH_HIDE_PASSWD,
            );
            if opt.debug as libc::c_long != 0 {
                debug_logprintf(
                    b"download_child: parent->url is: %s\n\0" as *const u8
                        as *const libc::c_char,
                    quote((*parent).url),
                );
            }
            rpl_free(referrer as *mut libc::c_void);
            referrer = 0 as *mut libc::c_char;
        }
        if opt.debug as libc::c_long != 0 {
            debug_logprintf(
                b"Already on the black list.\n\0" as *const u8 as *const libc::c_char,
            );
        }
        reason = WG_RR_BLACKLIST;
    } else {
        u_scheme_like_http = schemes_are_similar_p((*u).scheme, SCHEME_HTTP);
        if !u_scheme_like_http
            && !((*u).scheme as libc::c_uint == SCHEME_FTP as libc::c_int as libc::c_uint
                && opt.follow_ftp as libc::c_int != 0)
        {
            if opt.debug as libc::c_long != 0 {
                debug_logprintf(
                    b"Not following non-HTTP schemes.\n\0" as *const u8
                        as *const libc::c_char,
                );
            }
            reason = WG_RR_NONHTTP;
        } else {
            if u_scheme_like_http {
                if opt.relative_only as libc::c_int != 0
                    && (*upos).link_relative_p() == 0
                {
                    if opt.debug as libc::c_long != 0 {
                        debug_logprintf(
                            b"It doesn't really look like a relative link.\n\0"
                                as *const u8 as *const libc::c_char,
                        );
                    }
                    reason = WG_RR_ABSOLUTE;
                    current_block = 8589333034055499102;
                } else {
                    current_block = 11048769245176032998;
                }
            } else {
                current_block = 11048769245176032998;
            }
            match current_block {
                8589333034055499102 => {}
                _ => {
                    if !accept_domain(u) {
                        if opt.debug as libc::c_long != 0 {
                            debug_logprintf(
                                b"The domain was not accepted.\n\0" as *const u8
                                    as *const libc::c_char,
                            );
                        }
                        reason = WG_RR_DOMAIN;
                    } else {
                        if opt.no_parent as libc::c_int != 0
                            && schemes_are_similar_p(
                                (*u).scheme,
                                (*start_url_parsed).scheme,
                            ) as libc::c_int != 0
                            && 0 as libc::c_int
                                == strcasecmp((*u).host, (*start_url_parsed).host)
                            && ((*u).scheme as libc::c_uint
                                != (*start_url_parsed).scheme as libc::c_uint
                                || (*u).port == (*start_url_parsed).port)
                            && !(opt.page_requisites as libc::c_int != 0
                                && (*upos).link_inline_p() as libc::c_int != 0)
                        {
                            if !subdir_p((*start_url_parsed).dir, (*u).dir) {
                                if opt.debug as libc::c_long != 0 {
                                    debug_logprintf(
                                        b"Going to \"%s\" would escape \"%s\" with no_parent on.\n\0"
                                            as *const u8 as *const libc::c_char,
                                        (*u).dir,
                                        (*start_url_parsed).dir,
                                    );
                                }
                                reason = WG_RR_PARENT;
                                current_block = 8589333034055499102;
                            } else {
                                current_block = 5141539773904409130;
                            }
                        } else {
                            current_block = 5141539773904409130;
                        }
                        match current_block {
                            8589333034055499102 => {}
                            _ => {
                                if !(opt.includes).is_null() || !(opt.excludes).is_null() {
                                    if !accdir((*u).dir) {
                                        if opt.debug as libc::c_long != 0 {
                                            debug_logprintf(
                                                b"%s (%s) is excluded/not-included.\n\0" as *const u8
                                                    as *const libc::c_char,
                                                url,
                                                (*u).dir,
                                            );
                                        }
                                        reason = WG_RR_LIST;
                                        current_block = 8589333034055499102;
                                    } else {
                                        current_block = 6721012065216013753;
                                    }
                                } else {
                                    current_block = 6721012065216013753;
                                }
                                match current_block {
                                    8589333034055499102 => {}
                                    _ => {
                                        if !accept_url(url) {
                                            if opt.debug as libc::c_long != 0 {
                                                debug_logprintf(
                                                    b"%s is excluded/not-included through regex.\n\0"
                                                        as *const u8 as *const libc::c_char,
                                                    url,
                                                );
                                            }
                                            reason = WG_RR_REGEX;
                                        } else {
                                            if *((*u).file).offset(0 as libc::c_int as isize)
                                                as libc::c_int != '\0' as i32
                                                && !(has_html_suffix_p((*u).file) as libc::c_int != 0
                                                    && (opt.reclevel == -(1 as libc::c_int)
                                                        || depth < opt.reclevel - 1 as libc::c_int
                                                        || opt.page_requisites as libc::c_int != 0))
                                            {
                                                if !acceptable((*u).file) {
                                                    if opt.debug as libc::c_long != 0 {
                                                        debug_logprintf(
                                                            b"%s (%s) does not match acc/rej rules.\n\0" as *const u8
                                                                as *const libc::c_char,
                                                            url,
                                                            (*u).file,
                                                        );
                                                    }
                                                    reason = WG_RR_RULES;
                                                    current_block = 8589333034055499102;
                                                } else {
                                                    current_block = 7158658067966855297;
                                                }
                                            } else {
                                                current_block = 7158658067966855297;
                                            }
                                            match current_block {
                                                8589333034055499102 => {}
                                                _ => {
                                                    if schemes_are_similar_p((*u).scheme, (*parent).scheme) {
                                                        if !opt.spanhost
                                                            && 0 as libc::c_int != strcasecmp((*parent).host, (*u).host)
                                                        {
                                                            if opt.debug as libc::c_long != 0 {
                                                                debug_logprintf(
                                                                    b"This is not the same hostname as the parent's (%s and %s).\n\0"
                                                                        as *const u8 as *const libc::c_char,
                                                                    (*u).host,
                                                                    (*parent).host,
                                                                );
                                                            }
                                                            reason = WG_RR_SPANNEDHOST;
                                                            current_block = 8589333034055499102;
                                                        } else {
                                                            current_block = 17485376261910781866;
                                                        }
                                                    } else {
                                                        current_block = 17485376261910781866;
                                                    }
                                                    match current_block {
                                                        8589333034055499102 => {}
                                                        _ => {
                                                            if opt.use_robots as libc::c_int != 0
                                                                && u_scheme_like_http as libc::c_int != 0
                                                            {
                                                                let mut specs: *mut robot_specs = res_get_specs(
                                                                    (*u).host,
                                                                    (*u).port,
                                                                );
                                                                if specs.is_null() {
                                                                    let mut rfile: *mut libc::c_char = 0 as *mut libc::c_char;
                                                                    if res_retrieve_file(url, &mut rfile, iri) {
                                                                        specs = res_parse_from_file(rfile);
                                                                        if opt.delete_after as libc::c_int != 0
                                                                            || opt.spider as libc::c_int != 0
                                                                            || match_tail(
                                                                                rfile,
                                                                                b".tmp\0" as *const u8 as *const libc::c_char,
                                                                                0 as libc::c_int != 0,
                                                                            ) as libc::c_int != 0
                                                                        {
                                                                            logprintf(
                                                                                LOG_VERBOSE,
                                                                                dcgettext(
                                                                                    0 as *const libc::c_char,
                                                                                    b"Removing %s.\n\0" as *const u8 as *const libc::c_char,
                                                                                    5 as libc::c_int,
                                                                                ),
                                                                                rfile,
                                                                            );
                                                                            if unlink(rfile) != 0 {
                                                                                logprintf(
                                                                                    LOG_NOTQUIET,
                                                                                    b"unlink: %s\n\0" as *const u8 as *const libc::c_char,
                                                                                    strerror(*__errno_location()),
                                                                                );
                                                                            }
                                                                        }
                                                                        rpl_free(rfile as *mut libc::c_void);
                                                                        rfile = 0 as *mut libc::c_char;
                                                                    } else {
                                                                        specs = res_parse(
                                                                            b"\0" as *const u8 as *const libc::c_char,
                                                                            0 as libc::c_int,
                                                                        );
                                                                    }
                                                                    res_register_specs((*u).host, (*u).port, specs);
                                                                }
                                                                if !res_match_path(specs, (*u).path) {
                                                                    if opt.debug as libc::c_long != 0 {
                                                                        debug_logprintf(
                                                                            b"Not following %s because robots.txt forbids it.\n\0"
                                                                                as *const u8 as *const libc::c_char,
                                                                            url,
                                                                        );
                                                                    }
                                                                    blacklist_add(blacklist, url);
                                                                    reason = WG_RR_ROBOTS;
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
    }
    if reason as libc::c_uint == WG_RR_SUCCESS as libc::c_int as libc::c_uint {
        if opt.debug as libc::c_long != 0 {
            debug_logprintf(
                b"Decided to load it.\n\0" as *const u8 as *const libc::c_char,
            );
        }
    } else if opt.debug as libc::c_long != 0 {
        debug_logprintf(
            b"Decided NOT to load it.\n\0" as *const u8 as *const libc::c_char,
        );
    }
    return reason;
}
unsafe extern "C" fn descend_redirect(
    mut redirected: *const libc::c_char,
    mut orig_parsed: *mut url,
    mut depth: libc::c_int,
    mut start_url_parsed: *mut url,
    mut blacklist: *mut hash_table,
    mut iri: *mut iri,
) -> reject_reason {
    let mut new_parsed: *mut url = 0 as *mut url;
    let mut upos: *mut urlpos = 0 as *mut urlpos;
    let mut reason: reject_reason = WG_RR_SUCCESS;
    new_parsed = url_parse(
        redirected,
        0 as *mut libc::c_int,
        0 as *mut iri,
        0 as libc::c_int != 0,
    );
    upos = xcalloc(
        1 as libc::c_int as size_t,
        ::std::mem::size_of::<urlpos>() as libc::c_ulong,
    ) as *mut urlpos;
    (*upos).url = new_parsed;
    reason = download_child(upos, orig_parsed, depth, start_url_parsed, blacklist, iri);
    if reason as libc::c_uint == WG_RR_SUCCESS as libc::c_int as libc::c_uint {
        blacklist_add(blacklist, (*(*upos).url).url);
    } else if reason as libc::c_uint == WG_RR_LIST as libc::c_int as libc::c_uint
        || reason as libc::c_uint == WG_RR_REGEX as libc::c_int as libc::c_uint
    {
        if opt.debug as libc::c_long != 0 {
            debug_logprintf(
                b"Ignoring decision for redirects, decided to load it.\n\0" as *const u8
                    as *const libc::c_char,
            );
        }
        blacklist_add(blacklist, (*(*upos).url).url);
        reason = WG_RR_SUCCESS;
    } else if opt.debug as libc::c_long != 0 {
        debug_logprintf(
            b"Redirection \"%s\" failed the test.\n\0" as *const u8
                as *const libc::c_char,
            redirected,
        );
    }
    url_free(new_parsed);
    rpl_free(upos as *mut libc::c_void);
    upos = 0 as *mut urlpos;
    return reason;
}
unsafe extern "C" fn write_reject_log_header(mut f: *mut FILE) {
    if f.is_null() {
        return;
    }
    fprintf(
        f,
        b"REASON\tU_URL\tU_SCHEME\tU_HOST\tU_PORT\tU_PATH\tU_PARAMS\tU_QUERY\tU_FRAGMENT\tP_URL\tP_SCHEME\tP_HOST\tP_PORT\tP_PATH\tP_PARAMS\tP_QUERY\tP_FRAGMENT\n\0"
            as *const u8 as *const libc::c_char,
    );
}
unsafe extern "C" fn write_reject_log_url(mut fp: *mut FILE, mut url: *const url) {
    let mut escaped_str: *const libc::c_char = 0 as *const libc::c_char;
    let mut scheme_str: *const libc::c_char = 0 as *const libc::c_char;
    if fp.is_null() {
        return;
    }
    escaped_str = url_escape((*url).url);
    match (*url).scheme as libc::c_uint {
        0 => {
            scheme_str = b"SCHEME_HTTP\0" as *const u8 as *const libc::c_char;
        }
        1 => {
            scheme_str = b"SCHEME_FTP\0" as *const u8 as *const libc::c_char;
        }
        _ => {
            scheme_str = b"SCHEME_INVALID\0" as *const u8 as *const libc::c_char;
        }
    }
    fprintf(
        fp,
        b"%s\t%s\t%s\t%i\t%s\t%s\t%s\t%s\0" as *const u8 as *const libc::c_char,
        escaped_str,
        scheme_str,
        (*url).host,
        (*url).port,
        (*url).path,
        if !((*url).params).is_null() {
            (*url).params as *const libc::c_char
        } else {
            b"\0" as *const u8 as *const libc::c_char
        },
        if !((*url).query).is_null() {
            (*url).query as *const libc::c_char
        } else {
            b"\0" as *const u8 as *const libc::c_char
        },
        if !((*url).fragment).is_null() {
            (*url).fragment as *const libc::c_char
        } else {
            b"\0" as *const u8 as *const libc::c_char
        },
    );
    rpl_free(escaped_str as *mut libc::c_void);
    escaped_str = 0 as *const libc::c_char;
}
unsafe extern "C" fn write_reject_log_reason(
    mut fp: *mut FILE,
    mut reason: reject_reason,
    mut url: *const url,
    mut parent: *const url,
) {
    let mut reason_str: *const libc::c_char = 0 as *const libc::c_char;
    if fp.is_null() {
        return;
    }
    match reason as libc::c_uint {
        0 => {
            reason_str = b"SUCCESS\0" as *const u8 as *const libc::c_char;
        }
        1 => {
            reason_str = b"BLACKLIST\0" as *const u8 as *const libc::c_char;
        }
        2 => {
            reason_str = b"NOTHTTPS\0" as *const u8 as *const libc::c_char;
        }
        3 => {
            reason_str = b"NONHTTP\0" as *const u8 as *const libc::c_char;
        }
        4 => {
            reason_str = b"ABSOLUTE\0" as *const u8 as *const libc::c_char;
        }
        5 => {
            reason_str = b"DOMAIN\0" as *const u8 as *const libc::c_char;
        }
        6 => {
            reason_str = b"PARENT\0" as *const u8 as *const libc::c_char;
        }
        7 => {
            reason_str = b"LIST\0" as *const u8 as *const libc::c_char;
        }
        8 => {
            reason_str = b"REGEX\0" as *const u8 as *const libc::c_char;
        }
        9 => {
            reason_str = b"RULES\0" as *const u8 as *const libc::c_char;
        }
        10 => {
            reason_str = b"SPANNEDHOST\0" as *const u8 as *const libc::c_char;
        }
        11 => {
            reason_str = b"ROBOTS\0" as *const u8 as *const libc::c_char;
        }
        _ => {
            reason_str = b"UNKNOWN\0" as *const u8 as *const libc::c_char;
        }
    }
    fprintf(fp, b"%s\t\0" as *const u8 as *const libc::c_char, reason_str);
    write_reject_log_url(fp, url);
    fprintf(fp, b"\t\0" as *const u8 as *const libc::c_char);
    write_reject_log_url(fp, parent);
    fprintf(fp, b"\n\0" as *const u8 as *const libc::c_char);
}
