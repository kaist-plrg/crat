use ::libc;
use ::c2rust_bitfields;
extern "C" {
    pub type hash_table;
    fn dcgettext(
        __domainname: *const libc::c_char,
        __msgid: *const libc::c_char,
        __category: libc::c_int,
    ) -> *mut libc::c_char;
    fn memchr(
        _: *const libc::c_void,
        _: libc::c_int,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn strchr(_: *const libc::c_char, _: libc::c_int) -> *mut libc::c_char;
    fn strcspn(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_ulong;
    fn strspn(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_ulong;
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    fn strerror(_: libc::c_int) -> *mut libc::c_char;
    fn strcasecmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    fn rpl_free(_: *mut libc::c_void);
    static mut opt: options;
    fn rpl_strtol(
        string: *const libc::c_char,
        endptr: *mut *mut libc::c_char,
        base: libc::c_int,
    ) -> libc::c_long;
    fn xcalloc(n: size_t, s: size_t) -> *mut libc::c_void;
    fn xstrdup(str: *const libc::c_char) -> *mut libc::c_char;
    fn logprintf(_: log_options, _: *const libc::c_char, _: ...);
    fn debug_logprintf(_: *const libc::c_char, _: ...);
    fn quote_n(n: libc::c_int, arg: *const libc::c_char) -> *const libc::c_char;
    fn quote(arg: *const libc::c_char) -> *const libc::c_char;
    fn quotearg_n_style(
        n: libc::c_int,
        s: quoting_style,
        arg: *const libc::c_char,
    ) -> *mut libc::c_char;
    fn parse_charset(str: *const libc::c_char) -> *mut libc::c_char;
    fn iri_new() -> *mut iri;
    fn iri_free(i: *mut iri);
    fn set_uri_encoding(i: *mut iri, charset: *const libc::c_char, force: bool);
    fn set_content_encoding(i: *mut iri, charset: *const libc::c_char);
    fn __errno_location() -> *mut libc::c_int;
    fn inform_exit_status(err: uerr_t);
    fn map_html_tags(
        _: *const libc::c_char,
        _: libc::c_int,
        _: Option::<unsafe extern "C" fn(*mut taginfo, *mut libc::c_void) -> ()>,
        _: *mut libc::c_void,
        _: libc::c_int,
        _: *const hash_table,
        _: *const hash_table,
    );
    fn url_parse(
        _: *const libc::c_char,
        _: *mut libc::c_int,
        iri: *mut iri,
        percent_encode: bool,
    ) -> *mut url;
    fn url_error(_: libc::c_int) -> *const libc::c_char;
    fn url_has_scheme(_: *const libc::c_char) -> bool;
    fn uri_merge(_: *const libc::c_char, _: *const libc::c_char) -> *mut libc::c_char;
    fn rewrite_shorthand_url(_: *const libc::c_char) -> *mut libc::c_char;
    fn strdupdelim(_: *const libc::c_char, _: *const libc::c_char) -> *mut libc::c_char;
    fn wget_read_file(_: *const libc::c_char) -> *mut file_memory;
    fn wget_read_file_free(_: *mut file_memory);
    fn number_to_static_string(_: wgint) -> *mut libc::c_char;
    fn hash_table_destroy(_: *mut hash_table);
    fn hash_table_get(_: *const hash_table, _: *const libc::c_void) -> *mut libc::c_void;
    fn hash_table_put(
        _: *mut hash_table,
        _: *const libc::c_void,
        _: *const libc::c_void,
    );
    fn hash_table_remove(_: *mut hash_table, _: *const libc::c_void) -> libc::c_int;
    fn make_nocase_string_hash_table(_: libc::c_int) -> *mut hash_table;
    fn get_urls_css(_: *mut map_context, _: libc::c_int, _: libc::c_int);
    fn c_strcasecmp(s1: *const libc::c_char, s2: *const libc::c_char) -> libc::c_int;
    fn c_strncasecmp(
        s1: *const libc::c_char,
        s2: *const libc::c_char,
        n: size_t,
    ) -> libc::c_int;
}
pub type __int64_t = libc::c_long;
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
pub struct attr_pair {
    pub name: *mut libc::c_char,
    pub value: *mut libc::c_char,
    pub value_raw_beginning: *const libc::c_char,
    pub value_raw_size: libc::c_int,
    pub name_pool_index: libc::c_int,
    pub value_pool_index: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct taginfo {
    pub name: *mut libc::c_char,
    pub end_tag_p: libc::c_int,
    pub nattrs: libc::c_int,
    pub attrs: *mut attr_pair,
    pub start_position: *const libc::c_char,
    pub end_position: *const libc::c_char,
    pub contents_begin: *const libc::c_char,
    pub contents_end: *const libc::c_char,
}
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
pub struct file_memory {
    pub content: *mut libc::c_char,
    pub length: libc::c_long,
    pub mmap_p: libc::c_int,
}
pub type convert_options = libc::c_uint;
pub const CO_NULLIFY_BASE: convert_options = 4;
pub const CO_CONVERT_TO_COMPLETE: convert_options = 3;
pub const CO_CONVERT_BASENAME_ONLY: convert_options = 2;
pub const CO_CONVERT_TO_RELATIVE: convert_options = 1;
pub const CO_NOCONVERT: convert_options = 0;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct map_context {
    pub text: *mut libc::c_char,
    pub base: *mut libc::c_char,
    pub parent_base: *const libc::c_char,
    pub document_file: *const libc::c_char,
    pub nofollow: bool,
    pub head: *mut urlpos,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct known_tag {
    pub tagid: libc::c_int,
    pub name: *const libc::c_char,
    pub handler: tag_handler_t,
}
pub type tag_handler_t = Option::<
    unsafe extern "C" fn(libc::c_int, *mut taginfo, *mut map_context) -> (),
>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_3 {
    pub tagid: libc::c_int,
    pub attr_name: *const libc::c_char,
    pub flags: libc::c_int,
}
pub const TAG_SOURCE: C2RustUnnamed_4 = 24;
pub const TAG_AUDIO: C2RustUnnamed_4 = 23;
pub const TAG_VIDEO: C2RustUnnamed_4 = 22;
pub const TAG_TH: C2RustUnnamed_4 = 21;
pub const TAG_TD: C2RustUnnamed_4 = 20;
pub const TAG_TABLE: C2RustUnnamed_4 = 19;
pub const TAG_SCRIPT: C2RustUnnamed_4 = 18;
pub const TAG_OVERLAY: C2RustUnnamed_4 = 17;
pub const TAG_OBJECT: C2RustUnnamed_4 = 16;
pub const TAG_LAYER: C2RustUnnamed_4 = 13;
pub const TAG_INPUT: C2RustUnnamed_4 = 12;
pub const TAG_IMG: C2RustUnnamed_4 = 11;
pub const TAG_IFRAME: C2RustUnnamed_4 = 10;
pub const TAG_FRAME: C2RustUnnamed_4 = 9;
pub const TAG_FIG: C2RustUnnamed_4 = 7;
pub const TAG_EMBED: C2RustUnnamed_4 = 6;
pub const TAG_BODY: C2RustUnnamed_4 = 5;
pub const TAG_BGSOUND: C2RustUnnamed_4 = 4;
pub const TAG_AREA: C2RustUnnamed_4 = 2;
pub const TAG_APPLET: C2RustUnnamed_4 = 1;
pub const TAG_A: C2RustUnnamed_4 = 0;
pub const TAG_META: C2RustUnnamed_4 = 15;
pub const TAG_LINK: C2RustUnnamed_4 = 14;
pub const TAG_FORM: C2RustUnnamed_4 = 8;
pub const TAG_BASE: C2RustUnnamed_4 = 3;
pub type C2RustUnnamed_4 = libc::c_uint;
#[inline]
unsafe extern "C" fn c_isspace(mut c: libc::c_int) -> bool {
    match c {
        32 | 9 | 10 | 11 | 12 | 13 => return 1 as libc::c_int != 0,
        _ => return 0 as libc::c_int != 0,
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
static mut known_tags: [known_tag; 25] = unsafe {
    [
        {
            let mut init = known_tag {
                tagid: TAG_A as libc::c_int,
                name: b"a\0" as *const u8 as *const libc::c_char,
                handler: Some(
                    tag_find_urls
                        as unsafe extern "C" fn(
                            libc::c_int,
                            *mut taginfo,
                            *mut map_context,
                        ) -> (),
                ),
            };
            init
        },
        {
            let mut init = known_tag {
                tagid: TAG_APPLET as libc::c_int,
                name: b"applet\0" as *const u8 as *const libc::c_char,
                handler: Some(
                    tag_find_urls
                        as unsafe extern "C" fn(
                            libc::c_int,
                            *mut taginfo,
                            *mut map_context,
                        ) -> (),
                ),
            };
            init
        },
        {
            let mut init = known_tag {
                tagid: TAG_AREA as libc::c_int,
                name: b"area\0" as *const u8 as *const libc::c_char,
                handler: Some(
                    tag_find_urls
                        as unsafe extern "C" fn(
                            libc::c_int,
                            *mut taginfo,
                            *mut map_context,
                        ) -> (),
                ),
            };
            init
        },
        {
            let mut init = known_tag {
                tagid: TAG_BASE as libc::c_int,
                name: b"base\0" as *const u8 as *const libc::c_char,
                handler: Some(
                    tag_handle_base
                        as unsafe extern "C" fn(
                            libc::c_int,
                            *mut taginfo,
                            *mut map_context,
                        ) -> (),
                ),
            };
            init
        },
        {
            let mut init = known_tag {
                tagid: TAG_BGSOUND as libc::c_int,
                name: b"bgsound\0" as *const u8 as *const libc::c_char,
                handler: Some(
                    tag_find_urls
                        as unsafe extern "C" fn(
                            libc::c_int,
                            *mut taginfo,
                            *mut map_context,
                        ) -> (),
                ),
            };
            init
        },
        {
            let mut init = known_tag {
                tagid: TAG_BODY as libc::c_int,
                name: b"body\0" as *const u8 as *const libc::c_char,
                handler: Some(
                    tag_find_urls
                        as unsafe extern "C" fn(
                            libc::c_int,
                            *mut taginfo,
                            *mut map_context,
                        ) -> (),
                ),
            };
            init
        },
        {
            let mut init = known_tag {
                tagid: TAG_EMBED as libc::c_int,
                name: b"embed\0" as *const u8 as *const libc::c_char,
                handler: Some(
                    tag_find_urls
                        as unsafe extern "C" fn(
                            libc::c_int,
                            *mut taginfo,
                            *mut map_context,
                        ) -> (),
                ),
            };
            init
        },
        {
            let mut init = known_tag {
                tagid: TAG_FIG as libc::c_int,
                name: b"fig\0" as *const u8 as *const libc::c_char,
                handler: Some(
                    tag_find_urls
                        as unsafe extern "C" fn(
                            libc::c_int,
                            *mut taginfo,
                            *mut map_context,
                        ) -> (),
                ),
            };
            init
        },
        {
            let mut init = known_tag {
                tagid: TAG_FORM as libc::c_int,
                name: b"form\0" as *const u8 as *const libc::c_char,
                handler: Some(
                    tag_handle_form
                        as unsafe extern "C" fn(
                            libc::c_int,
                            *mut taginfo,
                            *mut map_context,
                        ) -> (),
                ),
            };
            init
        },
        {
            let mut init = known_tag {
                tagid: TAG_FRAME as libc::c_int,
                name: b"frame\0" as *const u8 as *const libc::c_char,
                handler: Some(
                    tag_find_urls
                        as unsafe extern "C" fn(
                            libc::c_int,
                            *mut taginfo,
                            *mut map_context,
                        ) -> (),
                ),
            };
            init
        },
        {
            let mut init = known_tag {
                tagid: TAG_IFRAME as libc::c_int,
                name: b"iframe\0" as *const u8 as *const libc::c_char,
                handler: Some(
                    tag_find_urls
                        as unsafe extern "C" fn(
                            libc::c_int,
                            *mut taginfo,
                            *mut map_context,
                        ) -> (),
                ),
            };
            init
        },
        {
            let mut init = known_tag {
                tagid: TAG_IMG as libc::c_int,
                name: b"img\0" as *const u8 as *const libc::c_char,
                handler: Some(
                    tag_handle_img
                        as unsafe extern "C" fn(
                            libc::c_int,
                            *mut taginfo,
                            *mut map_context,
                        ) -> (),
                ),
            };
            init
        },
        {
            let mut init = known_tag {
                tagid: TAG_INPUT as libc::c_int,
                name: b"input\0" as *const u8 as *const libc::c_char,
                handler: Some(
                    tag_find_urls
                        as unsafe extern "C" fn(
                            libc::c_int,
                            *mut taginfo,
                            *mut map_context,
                        ) -> (),
                ),
            };
            init
        },
        {
            let mut init = known_tag {
                tagid: TAG_LAYER as libc::c_int,
                name: b"layer\0" as *const u8 as *const libc::c_char,
                handler: Some(
                    tag_find_urls
                        as unsafe extern "C" fn(
                            libc::c_int,
                            *mut taginfo,
                            *mut map_context,
                        ) -> (),
                ),
            };
            init
        },
        {
            let mut init = known_tag {
                tagid: TAG_LINK as libc::c_int,
                name: b"link\0" as *const u8 as *const libc::c_char,
                handler: Some(
                    tag_handle_link
                        as unsafe extern "C" fn(
                            libc::c_int,
                            *mut taginfo,
                            *mut map_context,
                        ) -> (),
                ),
            };
            init
        },
        {
            let mut init = known_tag {
                tagid: TAG_META as libc::c_int,
                name: b"meta\0" as *const u8 as *const libc::c_char,
                handler: Some(
                    tag_handle_meta
                        as unsafe extern "C" fn(
                            libc::c_int,
                            *mut taginfo,
                            *mut map_context,
                        ) -> (),
                ),
            };
            init
        },
        {
            let mut init = known_tag {
                tagid: TAG_OBJECT as libc::c_int,
                name: b"object\0" as *const u8 as *const libc::c_char,
                handler: Some(
                    tag_find_urls
                        as unsafe extern "C" fn(
                            libc::c_int,
                            *mut taginfo,
                            *mut map_context,
                        ) -> (),
                ),
            };
            init
        },
        {
            let mut init = known_tag {
                tagid: TAG_OVERLAY as libc::c_int,
                name: b"overlay\0" as *const u8 as *const libc::c_char,
                handler: Some(
                    tag_find_urls
                        as unsafe extern "C" fn(
                            libc::c_int,
                            *mut taginfo,
                            *mut map_context,
                        ) -> (),
                ),
            };
            init
        },
        {
            let mut init = known_tag {
                tagid: TAG_SCRIPT as libc::c_int,
                name: b"script\0" as *const u8 as *const libc::c_char,
                handler: Some(
                    tag_find_urls
                        as unsafe extern "C" fn(
                            libc::c_int,
                            *mut taginfo,
                            *mut map_context,
                        ) -> (),
                ),
            };
            init
        },
        {
            let mut init = known_tag {
                tagid: TAG_TABLE as libc::c_int,
                name: b"table\0" as *const u8 as *const libc::c_char,
                handler: Some(
                    tag_find_urls
                        as unsafe extern "C" fn(
                            libc::c_int,
                            *mut taginfo,
                            *mut map_context,
                        ) -> (),
                ),
            };
            init
        },
        {
            let mut init = known_tag {
                tagid: TAG_TD as libc::c_int,
                name: b"td\0" as *const u8 as *const libc::c_char,
                handler: Some(
                    tag_find_urls
                        as unsafe extern "C" fn(
                            libc::c_int,
                            *mut taginfo,
                            *mut map_context,
                        ) -> (),
                ),
            };
            init
        },
        {
            let mut init = known_tag {
                tagid: TAG_TH as libc::c_int,
                name: b"th\0" as *const u8 as *const libc::c_char,
                handler: Some(
                    tag_find_urls
                        as unsafe extern "C" fn(
                            libc::c_int,
                            *mut taginfo,
                            *mut map_context,
                        ) -> (),
                ),
            };
            init
        },
        {
            let mut init = known_tag {
                tagid: TAG_VIDEO as libc::c_int,
                name: b"video\0" as *const u8 as *const libc::c_char,
                handler: Some(
                    tag_find_urls
                        as unsafe extern "C" fn(
                            libc::c_int,
                            *mut taginfo,
                            *mut map_context,
                        ) -> (),
                ),
            };
            init
        },
        {
            let mut init = known_tag {
                tagid: TAG_AUDIO as libc::c_int,
                name: b"audio\0" as *const u8 as *const libc::c_char,
                handler: Some(
                    tag_find_urls
                        as unsafe extern "C" fn(
                            libc::c_int,
                            *mut taginfo,
                            *mut map_context,
                        ) -> (),
                ),
            };
            init
        },
        {
            let mut init = known_tag {
                tagid: TAG_SOURCE as libc::c_int,
                name: b"source\0" as *const u8 as *const libc::c_char,
                handler: Some(
                    tag_find_urls
                        as unsafe extern "C" fn(
                            libc::c_int,
                            *mut taginfo,
                            *mut map_context,
                        ) -> (),
                ),
            };
            init
        },
    ]
};
static mut tag_url_attributes: [C2RustUnnamed_3; 26] = [
    {
        let mut init = C2RustUnnamed_3 {
            tagid: TAG_A as libc::c_int,
            attr_name: b"href\0" as *const u8 as *const libc::c_char,
            flags: 2 as libc::c_int,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_3 {
            tagid: TAG_APPLET as libc::c_int,
            attr_name: b"code\0" as *const u8 as *const libc::c_char,
            flags: 1 as libc::c_int,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_3 {
            tagid: TAG_AREA as libc::c_int,
            attr_name: b"href\0" as *const u8 as *const libc::c_char,
            flags: 2 as libc::c_int,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_3 {
            tagid: TAG_BGSOUND as libc::c_int,
            attr_name: b"src\0" as *const u8 as *const libc::c_char,
            flags: 1 as libc::c_int,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_3 {
            tagid: TAG_BODY as libc::c_int,
            attr_name: b"background\0" as *const u8 as *const libc::c_char,
            flags: 1 as libc::c_int,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_3 {
            tagid: TAG_EMBED as libc::c_int,
            attr_name: b"href\0" as *const u8 as *const libc::c_char,
            flags: 2 as libc::c_int,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_3 {
            tagid: TAG_EMBED as libc::c_int,
            attr_name: b"src\0" as *const u8 as *const libc::c_char,
            flags: 1 as libc::c_int | 2 as libc::c_int,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_3 {
            tagid: TAG_FIG as libc::c_int,
            attr_name: b"src\0" as *const u8 as *const libc::c_char,
            flags: 1 as libc::c_int,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_3 {
            tagid: TAG_FRAME as libc::c_int,
            attr_name: b"src\0" as *const u8 as *const libc::c_char,
            flags: 1 as libc::c_int | 2 as libc::c_int,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_3 {
            tagid: TAG_IFRAME as libc::c_int,
            attr_name: b"src\0" as *const u8 as *const libc::c_char,
            flags: 1 as libc::c_int | 2 as libc::c_int,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_3 {
            tagid: TAG_IMG as libc::c_int,
            attr_name: b"href\0" as *const u8 as *const libc::c_char,
            flags: 1 as libc::c_int,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_3 {
            tagid: TAG_IMG as libc::c_int,
            attr_name: b"lowsrc\0" as *const u8 as *const libc::c_char,
            flags: 1 as libc::c_int,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_3 {
            tagid: TAG_IMG as libc::c_int,
            attr_name: b"src\0" as *const u8 as *const libc::c_char,
            flags: 1 as libc::c_int,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_3 {
            tagid: TAG_INPUT as libc::c_int,
            attr_name: b"src\0" as *const u8 as *const libc::c_char,
            flags: 1 as libc::c_int,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_3 {
            tagid: TAG_LAYER as libc::c_int,
            attr_name: b"src\0" as *const u8 as *const libc::c_char,
            flags: 1 as libc::c_int | 2 as libc::c_int,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_3 {
            tagid: TAG_OBJECT as libc::c_int,
            attr_name: b"data\0" as *const u8 as *const libc::c_char,
            flags: 1 as libc::c_int,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_3 {
            tagid: TAG_OVERLAY as libc::c_int,
            attr_name: b"src\0" as *const u8 as *const libc::c_char,
            flags: 1 as libc::c_int | 2 as libc::c_int,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_3 {
            tagid: TAG_SCRIPT as libc::c_int,
            attr_name: b"src\0" as *const u8 as *const libc::c_char,
            flags: 1 as libc::c_int,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_3 {
            tagid: TAG_TABLE as libc::c_int,
            attr_name: b"background\0" as *const u8 as *const libc::c_char,
            flags: 1 as libc::c_int,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_3 {
            tagid: TAG_TD as libc::c_int,
            attr_name: b"background\0" as *const u8 as *const libc::c_char,
            flags: 1 as libc::c_int,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_3 {
            tagid: TAG_TH as libc::c_int,
            attr_name: b"background\0" as *const u8 as *const libc::c_char,
            flags: 1 as libc::c_int,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_3 {
            tagid: TAG_VIDEO as libc::c_int,
            attr_name: b"src\0" as *const u8 as *const libc::c_char,
            flags: 1 as libc::c_int,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_3 {
            tagid: TAG_VIDEO as libc::c_int,
            attr_name: b"poster\0" as *const u8 as *const libc::c_char,
            flags: 1 as libc::c_int,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_3 {
            tagid: TAG_AUDIO as libc::c_int,
            attr_name: b"src\0" as *const u8 as *const libc::c_char,
            flags: 1 as libc::c_int,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_3 {
            tagid: TAG_AUDIO as libc::c_int,
            attr_name: b"poster\0" as *const u8 as *const libc::c_char,
            flags: 1 as libc::c_int,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_3 {
            tagid: TAG_SOURCE as libc::c_int,
            attr_name: b"src\0" as *const u8 as *const libc::c_char,
            flags: 1 as libc::c_int,
        };
        init
    },
];
static mut additional_attributes: [*const libc::c_char; 8] = [
    b"rel\0" as *const u8 as *const libc::c_char,
    b"type\0" as *const u8 as *const libc::c_char,
    b"http-equiv\0" as *const u8 as *const libc::c_char,
    b"name\0" as *const u8 as *const libc::c_char,
    b"content\0" as *const u8 as *const libc::c_char,
    b"action\0" as *const u8 as *const libc::c_char,
    b"style\0" as *const u8 as *const libc::c_char,
    b"srcset\0" as *const u8 as *const libc::c_char,
];
static mut interesting_tags: *mut hash_table = 0 as *const hash_table as *mut hash_table;
static mut interesting_attributes: *mut hash_table = 0 as *const hash_table
    as *mut hash_table;
static mut meta_charset: *mut libc::c_char = 0 as *const libc::c_char
    as *mut libc::c_char;
unsafe extern "C" fn init_interesting() {
    let mut i: size_t = 0;
    interesting_tags = make_nocase_string_hash_table(
        (::std::mem::size_of::<[known_tag; 25]>() as libc::c_ulong)
            .wrapping_div(::std::mem::size_of::<known_tag>() as libc::c_ulong)
            as libc::c_int,
    );
    i = 0 as libc::c_int as size_t;
    while i
        < (::std::mem::size_of::<[known_tag; 25]>() as libc::c_ulong)
            .wrapping_div(::std::mem::size_of::<known_tag>() as libc::c_ulong)
    {
        hash_table_put(
            interesting_tags,
            known_tags[i as usize].name as *const libc::c_void,
            known_tags.as_mut_ptr().offset(i as isize) as *const libc::c_void,
        );
        i = i.wrapping_add(1);
        i;
    }
    if !(opt.ignore_tags).is_null() {
        let mut ignored: *mut *mut libc::c_char = 0 as *mut *mut libc::c_char;
        ignored = opt.ignore_tags;
        while !(*ignored).is_null() {
            hash_table_remove(interesting_tags, *ignored as *const libc::c_void);
            ignored = ignored.offset(1);
            ignored;
        }
    }
    if !(opt.follow_tags).is_null() {
        let mut intersect: *mut hash_table = make_nocase_string_hash_table(
            0 as libc::c_int,
        );
        let mut followed: *mut *mut libc::c_char = 0 as *mut *mut libc::c_char;
        followed = opt.follow_tags;
        while !(*followed).is_null() {
            let mut t: *mut known_tag = hash_table_get(
                interesting_tags,
                *followed as *const libc::c_void,
            ) as *mut known_tag;
            if !t.is_null() {
                hash_table_put(
                    intersect,
                    *followed as *const libc::c_void,
                    t as *const libc::c_void,
                );
            }
            followed = followed.offset(1);
            followed;
        }
        hash_table_destroy(interesting_tags);
        interesting_tags = intersect;
    }
    interesting_attributes = make_nocase_string_hash_table(10 as libc::c_int);
    i = 0 as libc::c_int as size_t;
    while i
        < (::std::mem::size_of::<[*const libc::c_char; 8]>() as libc::c_ulong)
            .wrapping_div(::std::mem::size_of::<*const libc::c_char>() as libc::c_ulong)
    {
        hash_table_put(
            interesting_attributes,
            additional_attributes[i as usize] as *const libc::c_void,
            b"1\0" as *const u8 as *const libc::c_char as *const libc::c_void,
        );
        i = i.wrapping_add(1);
        i;
    }
    i = 0 as libc::c_int as size_t;
    while i
        < (::std::mem::size_of::<[C2RustUnnamed_3; 26]>() as libc::c_ulong)
            .wrapping_div(::std::mem::size_of::<C2RustUnnamed_3>() as libc::c_ulong)
    {
        hash_table_put(
            interesting_attributes,
            tag_url_attributes[i as usize].attr_name as *const libc::c_void,
            b"1\0" as *const u8 as *const libc::c_char as *const libc::c_void,
        );
        i = i.wrapping_add(1);
        i;
    }
}
unsafe extern "C" fn find_attr(
    mut tag: *mut taginfo,
    mut name: *const libc::c_char,
    mut attrind: *mut libc::c_int,
) -> *mut libc::c_char {
    let mut i: libc::c_int = 0;
    i = 0 as libc::c_int;
    while i < (*tag).nattrs {
        if c_strcasecmp((*((*tag).attrs).offset(i as isize)).name, name) == 0 {
            if !attrind.is_null() {
                *attrind = i;
            }
            return (*((*tag).attrs).offset(i as isize)).value;
        }
        i += 1;
        i;
    }
    return 0 as *mut libc::c_char;
}
pub unsafe extern "C" fn append_url(
    mut link_uri: *const libc::c_char,
    mut position: libc::c_int,
    mut size: libc::c_int,
    mut ctx: *mut map_context,
) -> *mut urlpos {
    let mut link_has_scheme: libc::c_int = url_has_scheme(link_uri) as libc::c_int;
    let mut newel: *mut urlpos = 0 as *mut urlpos;
    let mut base: *const libc::c_char = if !((*ctx).base).is_null() {
        (*ctx).base as *const libc::c_char
    } else {
        (*ctx).parent_base
    };
    let mut url: *mut url = 0 as *mut url;
    let mut iri: *mut iri = iri_new();
    set_uri_encoding(iri, opt.locale, 1 as libc::c_int != 0);
    (*iri).utf8_encode = 1 as libc::c_int != 0;
    if base.is_null() {
        if opt.debug as libc::c_long != 0 {
            debug_logprintf(
                b"%s: no base, merge will use \"%s\".\n\0" as *const u8
                    as *const libc::c_char,
                (*ctx).document_file,
                link_uri,
            );
        }
        if link_has_scheme == 0 {
            logprintf(
                LOG_NOTQUIET,
                dcgettext(
                    0 as *const libc::c_char,
                    b"%s: Cannot resolve incomplete link %s.\n\0" as *const u8
                        as *const libc::c_char,
                    5 as libc::c_int,
                ),
                (*ctx).document_file,
                link_uri,
            );
            iri_free(iri);
            return 0 as *mut urlpos;
        }
        url = url_parse(link_uri, 0 as *mut libc::c_int, iri, 0 as libc::c_int != 0);
        if url.is_null() {
            if opt.debug as libc::c_long != 0 {
                debug_logprintf(
                    b"%s: link \"%s\" doesn't parse.\n\0" as *const u8
                        as *const libc::c_char,
                    (*ctx).document_file,
                    link_uri,
                );
            }
            iri_free(iri);
            return 0 as *mut urlpos;
        }
    } else {
        let mut complete_uri: *mut libc::c_char = uri_merge(base, link_uri);
        if opt.debug as libc::c_long != 0 {
            debug_logprintf(
                b"%s: merge(%s, %s) -> %s\n\0" as *const u8 as *const libc::c_char,
                quotearg_n_style(
                    0 as libc::c_int,
                    escape_quoting_style,
                    (*ctx).document_file,
                ),
                quote_n(1 as libc::c_int, base),
                quote_n(2 as libc::c_int, link_uri),
                quotearg_n_style(3 as libc::c_int, escape_quoting_style, complete_uri),
            );
        }
        url = url_parse(complete_uri, 0 as *mut libc::c_int, iri, 0 as libc::c_int != 0);
        if url.is_null() {
            if opt.debug as libc::c_long != 0 {
                debug_logprintf(
                    b"%s: merged link \"%s\" doesn't parse.\n\0" as *const u8
                        as *const libc::c_char,
                    (*ctx).document_file,
                    complete_uri,
                );
            }
            rpl_free(complete_uri as *mut libc::c_void);
            complete_uri = 0 as *mut libc::c_char;
            iri_free(iri);
            return 0 as *mut urlpos;
        }
        rpl_free(complete_uri as *mut libc::c_void);
        complete_uri = 0 as *mut libc::c_char;
    }
    iri_free(iri);
    if opt.debug as libc::c_long != 0 {
        debug_logprintf(
            b"appending %s to urlpos.\n\0" as *const u8 as *const libc::c_char,
            quote((*url).url),
        );
    }
    newel = xcalloc(
        1 as libc::c_int as size_t,
        ::std::mem::size_of::<urlpos>() as libc::c_ulong,
    ) as *mut urlpos;
    (*newel).url = url;
    (*newel).pos = position;
    (*newel).size = size;
    if link_has_scheme == 0 && *link_uri as libc::c_int != '/' as i32 {
        (*newel).set_link_relative_p(1 as libc::c_int as libc::c_uint);
    } else if link_has_scheme != 0 {
        (*newel).set_link_complete_p(1 as libc::c_int as libc::c_uint);
    }
    if ((*ctx).head).is_null() {
        (*ctx).head = newel;
    } else {
        let mut it: *mut urlpos = 0 as *mut urlpos;
        let mut prev: *mut urlpos = 0 as *mut urlpos;
        it = (*ctx).head;
        while !it.is_null() && position > (*it).pos {
            prev = it;
            it = (*it).next;
        }
        (*newel).next = it;
        if !prev.is_null() {
            (*prev).next = newel;
        } else {
            (*ctx).head = newel;
        }
    }
    return newel;
}
unsafe extern "C" fn check_style_attr(mut tag: *mut taginfo, mut ctx: *mut map_context) {
    let mut attrind: libc::c_int = 0;
    let mut raw_start: libc::c_int = 0;
    let mut raw_len: libc::c_int = 0;
    let mut style: *mut libc::c_char = find_attr(
        tag,
        b"style\0" as *const u8 as *const libc::c_char,
        &mut attrind,
    );
    if style.is_null() {
        return;
    }
    raw_start = ((*((*tag).attrs).offset(attrind as isize)).value_raw_beginning)
        .offset_from((*ctx).text) as libc::c_long as libc::c_int;
    raw_len = (*((*tag).attrs).offset(attrind as isize)).value_raw_size;
    if *((*ctx).text).offset(raw_start as isize) as libc::c_int == '\'' as i32
        || *((*ctx).text).offset(raw_start as isize) as libc::c_int == '"' as i32
    {
        raw_start += 1 as libc::c_int;
        raw_len -= 2 as libc::c_int;
    }
    if raw_len <= 0 as libc::c_int {
        return;
    }
    get_urls_css(ctx, raw_start, raw_len);
}
unsafe extern "C" fn tag_find_urls(
    mut tagid: libc::c_int,
    mut tag: *mut taginfo,
    mut ctx: *mut map_context,
) {
    let mut i: size_t = 0;
    let mut attrind: libc::c_int = 0;
    let mut first: libc::c_int = -(1 as libc::c_int);
    i = 0 as libc::c_int as size_t;
    while i
        < (::std::mem::size_of::<[C2RustUnnamed_3; 26]>() as libc::c_ulong)
            .wrapping_div(::std::mem::size_of::<C2RustUnnamed_3>() as libc::c_ulong)
    {
        if tag_url_attributes[i as usize].tagid == tagid {
            first = i as libc::c_int;
            break;
        } else {
            i = i.wrapping_add(1);
            i;
        }
    }
    attrind = 0 as libc::c_int;
    while attrind < (*tag).nattrs {
        let mut link: *mut libc::c_char = (*((*tag).attrs).offset(attrind as isize))
            .value;
        let size: size_t = (::std::mem::size_of::<[C2RustUnnamed_3; 26]>()
            as libc::c_ulong)
            .wrapping_div(::std::mem::size_of::<C2RustUnnamed_3>() as libc::c_ulong);
        i = first as size_t;
        while i < size && tag_url_attributes[i as usize].tagid == tagid {
            if 0 as libc::c_int
                == strcasecmp(
                    (*((*tag).attrs).offset(attrind as isize)).name,
                    tag_url_attributes[i as usize].attr_name,
                )
            {
                let mut up: *mut urlpos = append_url(
                    link,
                    ((*((*tag).attrs).offset(attrind as isize)).value_raw_beginning)
                        .offset_from((*ctx).text) as libc::c_long as libc::c_int,
                    (*((*tag).attrs).offset(attrind as isize)).value_raw_size,
                    ctx,
                );
                if !up.is_null() {
                    let mut flags: libc::c_int = tag_url_attributes[i as usize].flags;
                    if flags & 1 as libc::c_int != 0 {
                        (*up).set_link_inline_p(1 as libc::c_int as libc::c_uint);
                    }
                    if flags & 2 as libc::c_int != 0 {
                        (*up).set_link_expect_html(1 as libc::c_int as libc::c_uint);
                    }
                }
            }
            i = i.wrapping_add(1);
            i;
        }
        attrind += 1;
        attrind;
    }
}
unsafe extern "C" fn tag_handle_base(
    mut tagid: libc::c_int,
    mut tag: *mut taginfo,
    mut ctx: *mut map_context,
) {
    let mut base_urlpos: *mut urlpos = 0 as *mut urlpos;
    let mut attrind: libc::c_int = 0;
    let mut newbase: *mut libc::c_char = find_attr(
        tag,
        b"href\0" as *const u8 as *const libc::c_char,
        &mut attrind,
    );
    if newbase.is_null() {
        return;
    }
    base_urlpos = append_url(
        newbase,
        ((*((*tag).attrs).offset(attrind as isize)).value_raw_beginning)
            .offset_from((*ctx).text) as libc::c_long as libc::c_int,
        (*((*tag).attrs).offset(attrind as isize)).value_raw_size,
        ctx,
    );
    if base_urlpos.is_null() {
        return;
    }
    (*base_urlpos).set_ignore_when_downloading(1 as libc::c_int as libc::c_uint);
    (*base_urlpos).set_link_base_p(1 as libc::c_int as libc::c_uint);
    rpl_free((*ctx).base as *mut libc::c_void);
    (*ctx).base = 0 as *mut libc::c_char;
    if !((*ctx).parent_base).is_null() {
        (*ctx).base = uri_merge((*ctx).parent_base, newbase);
    } else {
        (*ctx).base = xstrdup(newbase);
    };
}
unsafe extern "C" fn tag_handle_form(
    mut tagid: libc::c_int,
    mut tag: *mut taginfo,
    mut ctx: *mut map_context,
) {
    let mut attrind: libc::c_int = 0;
    let mut action: *mut libc::c_char = find_attr(
        tag,
        b"action\0" as *const u8 as *const libc::c_char,
        &mut attrind,
    );
    if !action.is_null() {
        let mut up: *mut urlpos = append_url(
            action,
            ((*((*tag).attrs).offset(attrind as isize)).value_raw_beginning)
                .offset_from((*ctx).text) as libc::c_long as libc::c_int,
            (*((*tag).attrs).offset(attrind as isize)).value_raw_size,
            ctx,
        );
        if !up.is_null() {
            (*up).set_ignore_when_downloading(1 as libc::c_int as libc::c_uint);
        }
    }
}
unsafe extern "C" fn tag_handle_link(
    mut tagid: libc::c_int,
    mut tag: *mut taginfo,
    mut ctx: *mut map_context,
) {
    let mut attrind: libc::c_int = 0;
    let mut href: *mut libc::c_char = find_attr(
        tag,
        b"href\0" as *const u8 as *const libc::c_char,
        &mut attrind,
    );
    if !href.is_null() {
        let mut up: *mut urlpos = append_url(
            href,
            ((*((*tag).attrs).offset(attrind as isize)).value_raw_beginning)
                .offset_from((*ctx).text) as libc::c_long as libc::c_int,
            (*((*tag).attrs).offset(attrind as isize)).value_raw_size,
            ctx,
        );
        if !up.is_null() {
            let mut rel: *mut libc::c_char = find_attr(
                tag,
                b"rel\0" as *const u8 as *const libc::c_char,
                0 as *mut libc::c_int,
            );
            if !rel.is_null() {
                if 0 as libc::c_int
                    == c_strcasecmp(
                        rel,
                        b"stylesheet\0" as *const u8 as *const libc::c_char,
                    )
                    || 0 as libc::c_int
                        == c_strcasecmp(
                            rel,
                            b"alternate stylesheet\0" as *const u8 as *const libc::c_char,
                        )
                {
                    (*up).set_link_inline_p(1 as libc::c_int as libc::c_uint);
                    (*up).set_link_expect_css(1 as libc::c_int as libc::c_uint);
                } else if 0 as libc::c_int
                    == c_strcasecmp(
                        rel,
                        b"shortcut icon\0" as *const u8 as *const libc::c_char,
                    )
                    || 0 as libc::c_int
                        == c_strcasecmp(
                            rel,
                            b"icon\0" as *const u8 as *const libc::c_char,
                        )
                {
                    (*up).set_link_inline_p(1 as libc::c_int as libc::c_uint);
                } else if 0 as libc::c_int
                    == c_strcasecmp(
                        rel,
                        b"manifest\0" as *const u8 as *const libc::c_char,
                    )
                {
                    (*up).set_link_inline_p(1 as libc::c_int as libc::c_uint);
                } else {
                    let mut type_0: *mut libc::c_char = find_attr(
                        tag,
                        b"type\0" as *const u8 as *const libc::c_char,
                        0 as *mut libc::c_int,
                    );
                    if type_0.is_null()
                        || c_strcasecmp(
                            type_0,
                            b"text/html\0" as *const u8 as *const libc::c_char,
                        ) == 0 as libc::c_int
                    {
                        (*up).set_link_expect_html(1 as libc::c_int as libc::c_uint);
                    }
                }
            }
        }
    }
}
unsafe extern "C" fn tag_handle_meta(
    mut tagid: libc::c_int,
    mut tag: *mut taginfo,
    mut ctx: *mut map_context,
) {
    let mut name: *mut libc::c_char = find_attr(
        tag,
        b"name\0" as *const u8 as *const libc::c_char,
        0 as *mut libc::c_int,
    );
    let mut http_equiv: *mut libc::c_char = find_attr(
        tag,
        b"http-equiv\0" as *const u8 as *const libc::c_char,
        0 as *mut libc::c_int,
    );
    if !http_equiv.is_null()
        && 0 as libc::c_int
            == c_strcasecmp(http_equiv, b"refresh\0" as *const u8 as *const libc::c_char)
    {
        let mut entry: *mut urlpos = 0 as *mut urlpos;
        let mut attrind: libc::c_int = 0;
        let mut timeout: libc::c_int = 0;
        let mut p: *mut libc::c_char = 0 as *mut libc::c_char;
        let mut refresh: *mut libc::c_char = find_attr(
            tag,
            b"content\0" as *const u8 as *const libc::c_char,
            &mut attrind,
        );
        if refresh.is_null() {
            return;
        }
        timeout = rpl_strtol(refresh, &mut p, 10 as libc::c_int) as libc::c_int;
        if timeout < 0 as libc::c_int
            || {
                let fresh0 = p;
                p = p.offset(1);
                *fresh0 as libc::c_int != ';' as i32
            }
        {
            return;
        }
        while c_isspace(*p as libc::c_int) {
            p = p.offset(1);
            p;
        }
        if !(c_toupper(*p as libc::c_int) == 'U' as i32
            && c_toupper(*p.offset(1 as libc::c_int as isize) as libc::c_int)
                == 'R' as i32
            && c_toupper(*p.offset(2 as libc::c_int as isize) as libc::c_int)
                == 'L' as i32
            && *p.offset(3 as libc::c_int as isize) as libc::c_int == '=' as i32)
        {
            return;
        }
        p = p.offset(4 as libc::c_int as isize);
        while c_isspace(*p as libc::c_int) {
            p = p.offset(1);
            p;
        }
        entry = append_url(
            p,
            ((*((*tag).attrs).offset(attrind as isize)).value_raw_beginning)
                .offset_from((*ctx).text) as libc::c_long as libc::c_int,
            (*((*tag).attrs).offset(attrind as isize)).value_raw_size,
            ctx,
        );
        if !entry.is_null() {
            (*entry).set_link_refresh_p(1 as libc::c_int as libc::c_uint);
            (*entry).refresh_timeout = timeout;
            (*entry).set_link_expect_html(1 as libc::c_int as libc::c_uint);
        }
    } else if !http_equiv.is_null()
        && 0 as libc::c_int
            == c_strcasecmp(
                http_equiv,
                b"content-type\0" as *const u8 as *const libc::c_char,
            )
    {
        let mut mcharset: *mut libc::c_char = 0 as *mut libc::c_char;
        let mut content: *mut libc::c_char = find_attr(
            tag,
            b"content\0" as *const u8 as *const libc::c_char,
            0 as *mut libc::c_int,
        );
        if content.is_null() {
            return;
        }
        mcharset = parse_charset(content);
        if mcharset.is_null() {
            return;
        }
        rpl_free(meta_charset as *mut libc::c_void);
        meta_charset = 0 as *mut libc::c_char;
        meta_charset = mcharset;
    } else if !name.is_null()
        && 0 as libc::c_int
            == c_strcasecmp(name, b"robots\0" as *const u8 as *const libc::c_char)
    {
        let mut content_0: *mut libc::c_char = find_attr(
            tag,
            b"content\0" as *const u8 as *const libc::c_char,
            0 as *mut libc::c_int,
        );
        if content_0.is_null() {
            return;
        }
        if c_strcasecmp(content_0, b"none\0" as *const u8 as *const libc::c_char) == 0 {
            (*ctx).nofollow = 1 as libc::c_int != 0;
        } else {
            while *content_0 != 0 {
                let mut end: *mut libc::c_char = 0 as *mut libc::c_char;
                content_0 = content_0
                    .offset(
                        strspn(
                            content_0,
                            b" \x0C\n\r\t\x0B\0" as *const u8 as *const libc::c_char,
                        ) as isize,
                    );
                end = content_0
                    .offset(
                        strcspn(
                            content_0,
                            b", \x0C\n\r\t\x0B\0" as *const u8 as *const libc::c_char,
                        ) as isize,
                    );
                if c_strncasecmp(
                    content_0,
                    b"nofollow\0" as *const u8 as *const libc::c_char,
                    end.offset_from(content_0) as libc::c_long as size_t,
                ) == 0
                {
                    (*ctx).nofollow = 1 as libc::c_int != 0;
                }
                if *end as libc::c_int == ',' as i32 {
                    end = end.offset(1);
                    end;
                } else {
                    end = strchr(end, ',' as i32);
                    if !end.is_null() {
                        end = end.offset(1);
                        end;
                    } else {
                        end = content_0.offset(strlen(content_0) as isize);
                    }
                }
                content_0 = end;
            }
        }
    }
}
unsafe extern "C" fn tag_handle_img(
    mut tagid: libc::c_int,
    mut tag: *mut taginfo,
    mut ctx: *mut map_context,
) {
    let mut attrind: libc::c_int = 0;
    let mut srcset: *mut libc::c_char = 0 as *mut libc::c_char;
    tag_find_urls(tagid, tag, ctx);
    srcset = find_attr(
        tag,
        b"srcset\0" as *const u8 as *const libc::c_char,
        &mut attrind,
    );
    if !srcset.is_null() {
        let mut base_ind: libc::c_int = ((*((*tag).attrs).offset(attrind as isize))
            .value_raw_beginning)
            .offset_from((*ctx).text) as libc::c_long as libc::c_int;
        let mut size: libc::c_int = strlen(srcset) as libc::c_int;
        let mut offset: libc::c_int = 0;
        let mut url_start: libc::c_int = 0;
        let mut url_end: libc::c_int = 0;
        if *((*ctx).text).offset(base_ind as isize) as libc::c_int == '"' as i32
            || *((*ctx).text).offset(base_ind as isize) as libc::c_int == '\'' as i32
        {
            base_ind += 1;
            base_ind;
        }
        offset = 0 as libc::c_int;
        while offset < size {
            let mut has_descriptor: bool = 1 as libc::c_int != 0;
            url_start = (offset as libc::c_ulong)
                .wrapping_add(
                    strspn(
                        srcset.offset(offset as isize),
                        b" \x0C\n\r\t,\0" as *const u8 as *const libc::c_char,
                    ),
                ) as libc::c_int;
            if url_start == size {
                return;
            }
            url_end = (url_start as libc::c_ulong)
                .wrapping_add(
                    strcspn(
                        srcset.offset(url_start as isize),
                        b" \x0C\n\r\t\0" as *const u8 as *const libc::c_char,
                    ),
                ) as libc::c_int;
            while url_end - 1 as libc::c_int > url_start
                && *srcset.offset((url_end - 1 as libc::c_int) as isize) as libc::c_int
                    == ',' as i32
            {
                has_descriptor = 0 as libc::c_int != 0;
                url_end -= 1;
                url_end;
            }
            if url_end > url_start {
                let mut url_text: *mut libc::c_char = strdupdelim(
                    srcset.offset(url_start as isize),
                    srcset.offset(url_end as isize),
                );
                let mut up: *mut urlpos = append_url(
                    url_text,
                    base_ind + url_start,
                    url_end - url_start,
                    ctx,
                );
                if !up.is_null() {
                    (*up).set_link_inline_p(1 as libc::c_int as libc::c_uint);
                    (*up).set_link_noquote_html_p(1 as libc::c_int as libc::c_uint);
                }
                rpl_free(url_text as *mut libc::c_void);
                url_text = 0 as *mut libc::c_char;
            }
            if has_descriptor {
                let mut in_paren: bool = 0 as libc::c_int != 0;
                offset = url_end;
                while offset < size {
                    let mut c: libc::c_char = *srcset.offset(offset as isize);
                    if c as libc::c_int == '(' as i32 {
                        in_paren = 1 as libc::c_int != 0;
                    } else if c as libc::c_int == ')' as i32
                        && in_paren as libc::c_int != 0
                    {
                        in_paren = 0 as libc::c_int != 0;
                    } else if c as libc::c_int == ',' as i32 && !in_paren {
                        break;
                    }
                    offset += 1;
                    offset;
                }
            } else {
                offset = url_end;
            }
        }
    }
}
unsafe extern "C" fn collect_tags_mapper(
    mut tag: *mut taginfo,
    mut arg: *mut libc::c_void,
) {
    let mut ctx: *mut map_context = arg as *mut map_context;
    let mut t: *mut known_tag = hash_table_get(
        interesting_tags,
        (*tag).name as *const libc::c_void,
    ) as *mut known_tag;
    if !t.is_null() {
        ((*t).handler).unwrap()((*t).tagid, tag, ctx);
    }
    check_style_attr(tag, ctx);
    if (*tag).end_tag_p != 0
        && 0 as libc::c_int
            == c_strcasecmp((*tag).name, b"style\0" as *const u8 as *const libc::c_char)
        && !((*tag).contents_begin).is_null() && !((*tag).contents_end).is_null()
        && (*tag).contents_begin <= (*tag).contents_end
    {
        get_urls_css(
            ctx,
            ((*tag).contents_begin).offset_from((*ctx).text) as libc::c_long
                as libc::c_int,
            ((*tag).contents_end).offset_from((*tag).contents_begin) as libc::c_long
                as libc::c_int,
        );
    }
}
pub unsafe extern "C" fn get_urls_html_fm(
    mut file: *const libc::c_char,
    mut fm: *const file_memory,
    mut url: *const libc::c_char,
    mut meta_disallow_follow: *mut bool,
    mut iri: *mut iri,
) -> *mut urlpos {
    let mut ctx: map_context = map_context {
        text: 0 as *mut libc::c_char,
        base: 0 as *mut libc::c_char,
        parent_base: 0 as *const libc::c_char,
        document_file: 0 as *const libc::c_char,
        nofollow: false,
        head: 0 as *mut urlpos,
    };
    let mut flags: libc::c_int = 0;
    ctx.text = (*fm).content;
    ctx.head = 0 as *mut urlpos;
    ctx.base = 0 as *mut libc::c_char;
    ctx
        .parent_base = if !url.is_null() {
        url
    } else {
        opt.base_href as *const libc::c_char
    };
    ctx.document_file = file;
    ctx.nofollow = 0 as libc::c_int != 0;
    if interesting_tags.is_null() {
        init_interesting();
    }
    flags = 2 as libc::c_int;
    if opt.strict_comments {
        flags |= 1 as libc::c_int;
    }
    map_html_tags(
        (*fm).content,
        (*fm).length as libc::c_int,
        Some(
            collect_tags_mapper
                as unsafe extern "C" fn(*mut taginfo, *mut libc::c_void) -> (),
        ),
        &mut ctx as *mut map_context as *mut libc::c_void,
        flags,
        0 as *const hash_table,
        interesting_attributes,
    );
    if !iri.is_null() && ((*iri).content_encoding).is_null() && !meta_charset.is_null() {
        set_content_encoding(iri, meta_charset);
    }
    rpl_free(meta_charset as *mut libc::c_void);
    meta_charset = 0 as *mut libc::c_char;
    if opt.debug as libc::c_long != 0 {
        debug_logprintf(
            b"nofollow in %s: %d\n\0" as *const u8 as *const libc::c_char,
            file,
            ctx.nofollow as libc::c_int,
        );
    }
    if !meta_disallow_follow.is_null() {
        *meta_disallow_follow = ctx.nofollow;
    }
    rpl_free(ctx.base as *mut libc::c_void);
    ctx.base = 0 as *mut libc::c_char;
    return ctx.head;
}
pub unsafe extern "C" fn get_urls_html(
    mut file: *const libc::c_char,
    mut url: *const libc::c_char,
    mut meta_disallow_follow: *mut bool,
    mut iri: *mut iri,
) -> *mut urlpos {
    let mut urls: *mut urlpos = 0 as *mut urlpos;
    let mut fm: *mut file_memory = 0 as *mut file_memory;
    fm = wget_read_file(file);
    if fm.is_null() {
        logprintf(
            LOG_NOTQUIET,
            b"%s: %s\n\0" as *const u8 as *const libc::c_char,
            file,
            strerror(*__errno_location()),
        );
        return 0 as *mut urlpos;
    }
    if opt.debug as libc::c_long != 0 {
        debug_logprintf(
            b"Loaded %s (size %s).\n\0" as *const u8 as *const libc::c_char,
            file,
            number_to_static_string((*fm).length),
        );
    }
    urls = get_urls_html_fm(file, fm, url, meta_disallow_follow, iri);
    wget_read_file_free(fm);
    return urls;
}
pub unsafe extern "C" fn get_urls_file(mut file: *const libc::c_char) -> *mut urlpos {
    let mut fm: *mut file_memory = 0 as *mut file_memory;
    let mut head: *mut urlpos = 0 as *mut urlpos;
    let mut tail: *mut urlpos = 0 as *mut urlpos;
    let mut text: *const libc::c_char = 0 as *const libc::c_char;
    let mut text_end: *const libc::c_char = 0 as *const libc::c_char;
    fm = wget_read_file(file);
    if fm.is_null() {
        logprintf(
            LOG_NOTQUIET,
            b"%s: %s\n\0" as *const u8 as *const libc::c_char,
            file,
            strerror(*__errno_location()),
        );
        return 0 as *mut urlpos;
    }
    if opt.debug as libc::c_long != 0 {
        debug_logprintf(
            b"Loaded %s (size %s).\n\0" as *const u8 as *const libc::c_char,
            file,
            number_to_static_string((*fm).length),
        );
    }
    tail = 0 as *mut urlpos;
    head = tail;
    text = (*fm).content;
    text_end = ((*fm).content).offset((*fm).length as isize);
    while text < text_end {
        let mut up_error_code: libc::c_int = 0;
        let mut url_text: *mut libc::c_char = 0 as *mut libc::c_char;
        let mut new_url: *mut libc::c_char = 0 as *mut libc::c_char;
        let mut entry: *mut urlpos = 0 as *mut urlpos;
        let mut url: *mut url = 0 as *mut url;
        let mut line_beg: *const libc::c_char = text;
        let mut line_end: *const libc::c_char = memchr(
            text as *const libc::c_void,
            '\n' as i32,
            text_end.offset_from(text) as libc::c_long as libc::c_ulong,
        ) as *const libc::c_char;
        if line_end.is_null() {
            line_end = text_end;
        } else {
            line_end = line_end.offset(1);
            line_end;
        }
        text = line_end;
        while line_beg < line_end
            && c_isspace(*line_beg as libc::c_int) as libc::c_int != 0
        {
            line_beg = line_beg.offset(1);
            line_beg;
        }
        while line_end > line_beg
            && c_isspace(*line_end.offset(-(1 as libc::c_int as isize)) as libc::c_int)
                as libc::c_int != 0
        {
            line_end = line_end.offset(-1);
            line_end;
        }
        if line_beg == line_end {
            continue;
        }
        url_text = strdupdelim(line_beg, line_end);
        if !(opt.base_href).is_null() {
            let mut merged: *mut libc::c_char = uri_merge(opt.base_href, url_text);
            rpl_free(url_text as *mut libc::c_void);
            url_text = 0 as *mut libc::c_char;
            url_text = merged;
        }
        new_url = rewrite_shorthand_url(url_text);
        if !new_url.is_null() {
            rpl_free(url_text as *mut libc::c_void);
            url_text = 0 as *mut libc::c_char;
            url_text = new_url;
        }
        url = url_parse(
            url_text,
            &mut up_error_code,
            0 as *mut iri,
            0 as libc::c_int != 0,
        );
        if url.is_null() {
            logprintf(
                LOG_NOTQUIET,
                dcgettext(
                    0 as *const libc::c_char,
                    b"%s: Invalid URL %s: %s\n\0" as *const u8 as *const libc::c_char,
                    5 as libc::c_int,
                ),
                file,
                url_text,
                url_error(up_error_code),
            );
            rpl_free(url_text as *mut libc::c_void);
            url_text = 0 as *mut libc::c_char;
            inform_exit_status(URLERROR);
        } else {
            rpl_free(url_text as *mut libc::c_void);
            url_text = 0 as *mut libc::c_char;
            entry = xcalloc(
                1 as libc::c_int as size_t,
                ::std::mem::size_of::<urlpos>() as libc::c_ulong,
            ) as *mut urlpos;
            (*entry).url = url;
            if head.is_null() {
                head = entry;
            } else {
                (*tail).next = entry;
            }
            tail = entry;
        }
    }
    wget_read_file_free(fm);
    return head;
}
