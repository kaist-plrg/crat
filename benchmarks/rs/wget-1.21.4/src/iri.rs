use ::libc;
extern "C" {
    fn dcgettext(
        __domainname: *const libc::c_char,
        __msgid: *const libc::c_char,
        __category: libc::c_int,
    ) -> *mut libc::c_char;
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    fn strdup(_: *const libc::c_char) -> *mut libc::c_char;
    fn strchr(_: *const libc::c_char, _: libc::c_int) -> *mut libc::c_char;
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    fn rpl_free(_: *mut libc::c_void);
    static mut opt: options;
    fn xmalloc(s: size_t) -> *mut libc::c_void;
    fn xrealloc(p: *mut libc::c_void, s: size_t) -> *mut libc::c_void;
    fn xstrdup(str: *const libc::c_char) -> *mut libc::c_char;
    fn logprintf(_: log_options, _: *const libc::c_char, _: ...);
    fn debug_logprintf(_: *const libc::c_char, _: ...);
    fn quote_n(n: libc::c_int, arg: *const libc::c_char) -> *const libc::c_char;
    fn quote(arg: *const libc::c_char) -> *const libc::c_char;
    fn nl_langinfo(__item: nl_item) -> *mut libc::c_char;
    fn __errno_location() -> *mut libc::c_int;
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
    fn idn2_lookup_u8(
        src: *const uint8_t,
        lookupname: *mut *mut uint8_t,
        flags: libc::c_int,
    ) -> libc::c_int;
    fn idn2_strerror(rc: libc::c_int) -> *const libc::c_char;
    fn idn2_free(ptr: *mut libc::c_void);
    fn strdupdelim(_: *const libc::c_char, _: *const libc::c_char) -> *mut libc::c_char;
    fn url_unescape_except_reserved(_: *mut libc::c_char);
    fn c_strcasecmp(s1: *const libc::c_char, s2: *const libc::c_char) -> libc::c_int;
    fn c_strcasestr(
        haystack: *const libc::c_char,
        needle: *const libc::c_char,
    ) -> *mut libc::c_char;
    fn xstrndup(string: *const libc::c_char, n: size_t) -> *mut libc::c_char;
}
pub type __uint8_t = libc::c_uchar;
pub type __int64_t = libc::c_long;
pub type size_t = libc::c_ulong;
pub type int64_t = __int64_t;
pub type uint8_t = __uint8_t;
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
pub const CODESET: C2RustUnnamed_3 = 14;
pub type nl_item = libc::c_int;
pub type iconv_t = *mut libc::c_void;
pub const IDN2_OK: C2RustUnnamed_5 = 0;
pub const IDN2_TRANSITIONAL: C2RustUnnamed_4 = 4;
pub const IDN2_NONTRANSITIONAL: C2RustUnnamed_4 = 8;
pub type C2RustUnnamed_3 = libc::c_uint;
pub const _NL_NUM: C2RustUnnamed_3 = 786449;
pub const _NL_NUM_LC_IDENTIFICATION: C2RustUnnamed_3 = 786448;
pub const _NL_IDENTIFICATION_CODESET: C2RustUnnamed_3 = 786447;
pub const _NL_IDENTIFICATION_CATEGORY: C2RustUnnamed_3 = 786446;
pub const _NL_IDENTIFICATION_DATE: C2RustUnnamed_3 = 786445;
pub const _NL_IDENTIFICATION_REVISION: C2RustUnnamed_3 = 786444;
pub const _NL_IDENTIFICATION_ABBREVIATION: C2RustUnnamed_3 = 786443;
pub const _NL_IDENTIFICATION_APPLICATION: C2RustUnnamed_3 = 786442;
pub const _NL_IDENTIFICATION_AUDIENCE: C2RustUnnamed_3 = 786441;
pub const _NL_IDENTIFICATION_TERRITORY: C2RustUnnamed_3 = 786440;
pub const _NL_IDENTIFICATION_LANGUAGE: C2RustUnnamed_3 = 786439;
pub const _NL_IDENTIFICATION_FAX: C2RustUnnamed_3 = 786438;
pub const _NL_IDENTIFICATION_TEL: C2RustUnnamed_3 = 786437;
pub const _NL_IDENTIFICATION_EMAIL: C2RustUnnamed_3 = 786436;
pub const _NL_IDENTIFICATION_CONTACT: C2RustUnnamed_3 = 786435;
pub const _NL_IDENTIFICATION_ADDRESS: C2RustUnnamed_3 = 786434;
pub const _NL_IDENTIFICATION_SOURCE: C2RustUnnamed_3 = 786433;
pub const _NL_IDENTIFICATION_TITLE: C2RustUnnamed_3 = 786432;
pub const _NL_NUM_LC_MEASUREMENT: C2RustUnnamed_3 = 720898;
pub const _NL_MEASUREMENT_CODESET: C2RustUnnamed_3 = 720897;
pub const _NL_MEASUREMENT_MEASUREMENT: C2RustUnnamed_3 = 720896;
pub const _NL_NUM_LC_TELEPHONE: C2RustUnnamed_3 = 655365;
pub const _NL_TELEPHONE_CODESET: C2RustUnnamed_3 = 655364;
pub const _NL_TELEPHONE_INT_PREFIX: C2RustUnnamed_3 = 655363;
pub const _NL_TELEPHONE_INT_SELECT: C2RustUnnamed_3 = 655362;
pub const _NL_TELEPHONE_TEL_DOM_FMT: C2RustUnnamed_3 = 655361;
pub const _NL_TELEPHONE_TEL_INT_FMT: C2RustUnnamed_3 = 655360;
pub const _NL_NUM_LC_ADDRESS: C2RustUnnamed_3 = 589837;
pub const _NL_ADDRESS_CODESET: C2RustUnnamed_3 = 589836;
pub const _NL_ADDRESS_LANG_LIB: C2RustUnnamed_3 = 589835;
pub const _NL_ADDRESS_LANG_TERM: C2RustUnnamed_3 = 589834;
pub const _NL_ADDRESS_LANG_AB: C2RustUnnamed_3 = 589833;
pub const _NL_ADDRESS_LANG_NAME: C2RustUnnamed_3 = 589832;
pub const _NL_ADDRESS_COUNTRY_ISBN: C2RustUnnamed_3 = 589831;
pub const _NL_ADDRESS_COUNTRY_NUM: C2RustUnnamed_3 = 589830;
pub const _NL_ADDRESS_COUNTRY_CAR: C2RustUnnamed_3 = 589829;
pub const _NL_ADDRESS_COUNTRY_AB3: C2RustUnnamed_3 = 589828;
pub const _NL_ADDRESS_COUNTRY_AB2: C2RustUnnamed_3 = 589827;
pub const _NL_ADDRESS_COUNTRY_POST: C2RustUnnamed_3 = 589826;
pub const _NL_ADDRESS_COUNTRY_NAME: C2RustUnnamed_3 = 589825;
pub const _NL_ADDRESS_POSTAL_FMT: C2RustUnnamed_3 = 589824;
pub const _NL_NUM_LC_NAME: C2RustUnnamed_3 = 524295;
pub const _NL_NAME_CODESET: C2RustUnnamed_3 = 524294;
pub const _NL_NAME_NAME_MS: C2RustUnnamed_3 = 524293;
pub const _NL_NAME_NAME_MISS: C2RustUnnamed_3 = 524292;
pub const _NL_NAME_NAME_MRS: C2RustUnnamed_3 = 524291;
pub const _NL_NAME_NAME_MR: C2RustUnnamed_3 = 524290;
pub const _NL_NAME_NAME_GEN: C2RustUnnamed_3 = 524289;
pub const _NL_NAME_NAME_FMT: C2RustUnnamed_3 = 524288;
pub const _NL_NUM_LC_PAPER: C2RustUnnamed_3 = 458755;
pub const _NL_PAPER_CODESET: C2RustUnnamed_3 = 458754;
pub const _NL_PAPER_WIDTH: C2RustUnnamed_3 = 458753;
pub const _NL_PAPER_HEIGHT: C2RustUnnamed_3 = 458752;
pub const _NL_NUM_LC_MESSAGES: C2RustUnnamed_3 = 327685;
pub const _NL_MESSAGES_CODESET: C2RustUnnamed_3 = 327684;
pub const __NOSTR: C2RustUnnamed_3 = 327683;
pub const __YESSTR: C2RustUnnamed_3 = 327682;
pub const __NOEXPR: C2RustUnnamed_3 = 327681;
pub const __YESEXPR: C2RustUnnamed_3 = 327680;
pub const _NL_NUM_LC_NUMERIC: C2RustUnnamed_3 = 65542;
pub const _NL_NUMERIC_CODESET: C2RustUnnamed_3 = 65541;
pub const _NL_NUMERIC_THOUSANDS_SEP_WC: C2RustUnnamed_3 = 65540;
pub const _NL_NUMERIC_DECIMAL_POINT_WC: C2RustUnnamed_3 = 65539;
pub const __GROUPING: C2RustUnnamed_3 = 65538;
pub const THOUSEP: C2RustUnnamed_3 = 65537;
pub const __THOUSANDS_SEP: C2RustUnnamed_3 = 65537;
pub const RADIXCHAR: C2RustUnnamed_3 = 65536;
pub const __DECIMAL_POINT: C2RustUnnamed_3 = 65536;
pub const _NL_NUM_LC_MONETARY: C2RustUnnamed_3 = 262190;
pub const _NL_MONETARY_CODESET: C2RustUnnamed_3 = 262189;
pub const _NL_MONETARY_THOUSANDS_SEP_WC: C2RustUnnamed_3 = 262188;
pub const _NL_MONETARY_DECIMAL_POINT_WC: C2RustUnnamed_3 = 262187;
pub const _NL_MONETARY_CONVERSION_RATE: C2RustUnnamed_3 = 262186;
pub const _NL_MONETARY_DUO_VALID_TO: C2RustUnnamed_3 = 262185;
pub const _NL_MONETARY_DUO_VALID_FROM: C2RustUnnamed_3 = 262184;
pub const _NL_MONETARY_UNO_VALID_TO: C2RustUnnamed_3 = 262183;
pub const _NL_MONETARY_UNO_VALID_FROM: C2RustUnnamed_3 = 262182;
pub const _NL_MONETARY_DUO_INT_N_SIGN_POSN: C2RustUnnamed_3 = 262181;
pub const _NL_MONETARY_DUO_INT_P_SIGN_POSN: C2RustUnnamed_3 = 262180;
pub const _NL_MONETARY_DUO_N_SIGN_POSN: C2RustUnnamed_3 = 262179;
pub const _NL_MONETARY_DUO_P_SIGN_POSN: C2RustUnnamed_3 = 262178;
pub const _NL_MONETARY_DUO_INT_N_SEP_BY_SPACE: C2RustUnnamed_3 = 262177;
pub const _NL_MONETARY_DUO_INT_N_CS_PRECEDES: C2RustUnnamed_3 = 262176;
pub const _NL_MONETARY_DUO_INT_P_SEP_BY_SPACE: C2RustUnnamed_3 = 262175;
pub const _NL_MONETARY_DUO_INT_P_CS_PRECEDES: C2RustUnnamed_3 = 262174;
pub const _NL_MONETARY_DUO_N_SEP_BY_SPACE: C2RustUnnamed_3 = 262173;
pub const _NL_MONETARY_DUO_N_CS_PRECEDES: C2RustUnnamed_3 = 262172;
pub const _NL_MONETARY_DUO_P_SEP_BY_SPACE: C2RustUnnamed_3 = 262171;
pub const _NL_MONETARY_DUO_P_CS_PRECEDES: C2RustUnnamed_3 = 262170;
pub const _NL_MONETARY_DUO_FRAC_DIGITS: C2RustUnnamed_3 = 262169;
pub const _NL_MONETARY_DUO_INT_FRAC_DIGITS: C2RustUnnamed_3 = 262168;
pub const _NL_MONETARY_DUO_CURRENCY_SYMBOL: C2RustUnnamed_3 = 262167;
pub const _NL_MONETARY_DUO_INT_CURR_SYMBOL: C2RustUnnamed_3 = 262166;
pub const __INT_N_SIGN_POSN: C2RustUnnamed_3 = 262165;
pub const __INT_P_SIGN_POSN: C2RustUnnamed_3 = 262164;
pub const __INT_N_SEP_BY_SPACE: C2RustUnnamed_3 = 262163;
pub const __INT_N_CS_PRECEDES: C2RustUnnamed_3 = 262162;
pub const __INT_P_SEP_BY_SPACE: C2RustUnnamed_3 = 262161;
pub const __INT_P_CS_PRECEDES: C2RustUnnamed_3 = 262160;
pub const _NL_MONETARY_CRNCYSTR: C2RustUnnamed_3 = 262159;
pub const __N_SIGN_POSN: C2RustUnnamed_3 = 262158;
pub const __P_SIGN_POSN: C2RustUnnamed_3 = 262157;
pub const __N_SEP_BY_SPACE: C2RustUnnamed_3 = 262156;
pub const __N_CS_PRECEDES: C2RustUnnamed_3 = 262155;
pub const __P_SEP_BY_SPACE: C2RustUnnamed_3 = 262154;
pub const __P_CS_PRECEDES: C2RustUnnamed_3 = 262153;
pub const __FRAC_DIGITS: C2RustUnnamed_3 = 262152;
pub const __INT_FRAC_DIGITS: C2RustUnnamed_3 = 262151;
pub const __NEGATIVE_SIGN: C2RustUnnamed_3 = 262150;
pub const __POSITIVE_SIGN: C2RustUnnamed_3 = 262149;
pub const __MON_GROUPING: C2RustUnnamed_3 = 262148;
pub const __MON_THOUSANDS_SEP: C2RustUnnamed_3 = 262147;
pub const __MON_DECIMAL_POINT: C2RustUnnamed_3 = 262146;
pub const __CURRENCY_SYMBOL: C2RustUnnamed_3 = 262145;
pub const __INT_CURR_SYMBOL: C2RustUnnamed_3 = 262144;
pub const _NL_NUM_LC_CTYPE: C2RustUnnamed_3 = 86;
pub const _NL_CTYPE_EXTRA_MAP_14: C2RustUnnamed_3 = 85;
pub const _NL_CTYPE_EXTRA_MAP_13: C2RustUnnamed_3 = 84;
pub const _NL_CTYPE_EXTRA_MAP_12: C2RustUnnamed_3 = 83;
pub const _NL_CTYPE_EXTRA_MAP_11: C2RustUnnamed_3 = 82;
pub const _NL_CTYPE_EXTRA_MAP_10: C2RustUnnamed_3 = 81;
pub const _NL_CTYPE_EXTRA_MAP_9: C2RustUnnamed_3 = 80;
pub const _NL_CTYPE_EXTRA_MAP_8: C2RustUnnamed_3 = 79;
pub const _NL_CTYPE_EXTRA_MAP_7: C2RustUnnamed_3 = 78;
pub const _NL_CTYPE_EXTRA_MAP_6: C2RustUnnamed_3 = 77;
pub const _NL_CTYPE_EXTRA_MAP_5: C2RustUnnamed_3 = 76;
pub const _NL_CTYPE_EXTRA_MAP_4: C2RustUnnamed_3 = 75;
pub const _NL_CTYPE_EXTRA_MAP_3: C2RustUnnamed_3 = 74;
pub const _NL_CTYPE_EXTRA_MAP_2: C2RustUnnamed_3 = 73;
pub const _NL_CTYPE_EXTRA_MAP_1: C2RustUnnamed_3 = 72;
pub const _NL_CTYPE_NONASCII_CASE: C2RustUnnamed_3 = 71;
pub const _NL_CTYPE_MAP_TO_NONASCII: C2RustUnnamed_3 = 70;
pub const _NL_CTYPE_TRANSLIT_IGNORE: C2RustUnnamed_3 = 69;
pub const _NL_CTYPE_TRANSLIT_IGNORE_LEN: C2RustUnnamed_3 = 68;
pub const _NL_CTYPE_TRANSLIT_DEFAULT_MISSING: C2RustUnnamed_3 = 67;
pub const _NL_CTYPE_TRANSLIT_DEFAULT_MISSING_LEN: C2RustUnnamed_3 = 66;
pub const _NL_CTYPE_TRANSLIT_TO_TBL: C2RustUnnamed_3 = 65;
pub const _NL_CTYPE_TRANSLIT_TO_IDX: C2RustUnnamed_3 = 64;
pub const _NL_CTYPE_TRANSLIT_FROM_TBL: C2RustUnnamed_3 = 63;
pub const _NL_CTYPE_TRANSLIT_FROM_IDX: C2RustUnnamed_3 = 62;
pub const _NL_CTYPE_TRANSLIT_TAB_SIZE: C2RustUnnamed_3 = 61;
pub const _NL_CTYPE_OUTDIGIT9_WC: C2RustUnnamed_3 = 60;
pub const _NL_CTYPE_OUTDIGIT8_WC: C2RustUnnamed_3 = 59;
pub const _NL_CTYPE_OUTDIGIT7_WC: C2RustUnnamed_3 = 58;
pub const _NL_CTYPE_OUTDIGIT6_WC: C2RustUnnamed_3 = 57;
pub const _NL_CTYPE_OUTDIGIT5_WC: C2RustUnnamed_3 = 56;
pub const _NL_CTYPE_OUTDIGIT4_WC: C2RustUnnamed_3 = 55;
pub const _NL_CTYPE_OUTDIGIT3_WC: C2RustUnnamed_3 = 54;
pub const _NL_CTYPE_OUTDIGIT2_WC: C2RustUnnamed_3 = 53;
pub const _NL_CTYPE_OUTDIGIT1_WC: C2RustUnnamed_3 = 52;
pub const _NL_CTYPE_OUTDIGIT0_WC: C2RustUnnamed_3 = 51;
pub const _NL_CTYPE_OUTDIGIT9_MB: C2RustUnnamed_3 = 50;
pub const _NL_CTYPE_OUTDIGIT8_MB: C2RustUnnamed_3 = 49;
pub const _NL_CTYPE_OUTDIGIT7_MB: C2RustUnnamed_3 = 48;
pub const _NL_CTYPE_OUTDIGIT6_MB: C2RustUnnamed_3 = 47;
pub const _NL_CTYPE_OUTDIGIT5_MB: C2RustUnnamed_3 = 46;
pub const _NL_CTYPE_OUTDIGIT4_MB: C2RustUnnamed_3 = 45;
pub const _NL_CTYPE_OUTDIGIT3_MB: C2RustUnnamed_3 = 44;
pub const _NL_CTYPE_OUTDIGIT2_MB: C2RustUnnamed_3 = 43;
pub const _NL_CTYPE_OUTDIGIT1_MB: C2RustUnnamed_3 = 42;
pub const _NL_CTYPE_OUTDIGIT0_MB: C2RustUnnamed_3 = 41;
pub const _NL_CTYPE_INDIGITS9_WC: C2RustUnnamed_3 = 40;
pub const _NL_CTYPE_INDIGITS8_WC: C2RustUnnamed_3 = 39;
pub const _NL_CTYPE_INDIGITS7_WC: C2RustUnnamed_3 = 38;
pub const _NL_CTYPE_INDIGITS6_WC: C2RustUnnamed_3 = 37;
pub const _NL_CTYPE_INDIGITS5_WC: C2RustUnnamed_3 = 36;
pub const _NL_CTYPE_INDIGITS4_WC: C2RustUnnamed_3 = 35;
pub const _NL_CTYPE_INDIGITS3_WC: C2RustUnnamed_3 = 34;
pub const _NL_CTYPE_INDIGITS2_WC: C2RustUnnamed_3 = 33;
pub const _NL_CTYPE_INDIGITS1_WC: C2RustUnnamed_3 = 32;
pub const _NL_CTYPE_INDIGITS0_WC: C2RustUnnamed_3 = 31;
pub const _NL_CTYPE_INDIGITS_WC_LEN: C2RustUnnamed_3 = 30;
pub const _NL_CTYPE_INDIGITS9_MB: C2RustUnnamed_3 = 29;
pub const _NL_CTYPE_INDIGITS8_MB: C2RustUnnamed_3 = 28;
pub const _NL_CTYPE_INDIGITS7_MB: C2RustUnnamed_3 = 27;
pub const _NL_CTYPE_INDIGITS6_MB: C2RustUnnamed_3 = 26;
pub const _NL_CTYPE_INDIGITS5_MB: C2RustUnnamed_3 = 25;
pub const _NL_CTYPE_INDIGITS4_MB: C2RustUnnamed_3 = 24;
pub const _NL_CTYPE_INDIGITS3_MB: C2RustUnnamed_3 = 23;
pub const _NL_CTYPE_INDIGITS2_MB: C2RustUnnamed_3 = 22;
pub const _NL_CTYPE_INDIGITS1_MB: C2RustUnnamed_3 = 21;
pub const _NL_CTYPE_INDIGITS0_MB: C2RustUnnamed_3 = 20;
pub const _NL_CTYPE_INDIGITS_MB_LEN: C2RustUnnamed_3 = 19;
pub const _NL_CTYPE_MAP_OFFSET: C2RustUnnamed_3 = 18;
pub const _NL_CTYPE_CLASS_OFFSET: C2RustUnnamed_3 = 17;
pub const _NL_CTYPE_TOLOWER32: C2RustUnnamed_3 = 16;
pub const _NL_CTYPE_TOUPPER32: C2RustUnnamed_3 = 15;
pub const _NL_CTYPE_CODESET_NAME: C2RustUnnamed_3 = 14;
pub const _NL_CTYPE_MB_CUR_MAX: C2RustUnnamed_3 = 13;
pub const _NL_CTYPE_WIDTH: C2RustUnnamed_3 = 12;
pub const _NL_CTYPE_MAP_NAMES: C2RustUnnamed_3 = 11;
pub const _NL_CTYPE_CLASS_NAMES: C2RustUnnamed_3 = 10;
pub const _NL_CTYPE_GAP6: C2RustUnnamed_3 = 9;
pub const _NL_CTYPE_GAP5: C2RustUnnamed_3 = 8;
pub const _NL_CTYPE_GAP4: C2RustUnnamed_3 = 7;
pub const _NL_CTYPE_GAP3: C2RustUnnamed_3 = 6;
pub const _NL_CTYPE_CLASS32: C2RustUnnamed_3 = 5;
pub const _NL_CTYPE_GAP2: C2RustUnnamed_3 = 4;
pub const _NL_CTYPE_TOLOWER: C2RustUnnamed_3 = 3;
pub const _NL_CTYPE_GAP1: C2RustUnnamed_3 = 2;
pub const _NL_CTYPE_TOUPPER: C2RustUnnamed_3 = 1;
pub const _NL_CTYPE_CLASS: C2RustUnnamed_3 = 0;
pub const _NL_NUM_LC_COLLATE: C2RustUnnamed_3 = 196627;
pub const _NL_COLLATE_CODESET: C2RustUnnamed_3 = 196626;
pub const _NL_COLLATE_COLLSEQWC: C2RustUnnamed_3 = 196625;
pub const _NL_COLLATE_COLLSEQMB: C2RustUnnamed_3 = 196624;
pub const _NL_COLLATE_SYMB_EXTRAMB: C2RustUnnamed_3 = 196623;
pub const _NL_COLLATE_SYMB_TABLEMB: C2RustUnnamed_3 = 196622;
pub const _NL_COLLATE_SYMB_HASH_SIZEMB: C2RustUnnamed_3 = 196621;
pub const _NL_COLLATE_INDIRECTWC: C2RustUnnamed_3 = 196620;
pub const _NL_COLLATE_EXTRAWC: C2RustUnnamed_3 = 196619;
pub const _NL_COLLATE_WEIGHTWC: C2RustUnnamed_3 = 196618;
pub const _NL_COLLATE_TABLEWC: C2RustUnnamed_3 = 196617;
pub const _NL_COLLATE_GAP3: C2RustUnnamed_3 = 196616;
pub const _NL_COLLATE_GAP2: C2RustUnnamed_3 = 196615;
pub const _NL_COLLATE_GAP1: C2RustUnnamed_3 = 196614;
pub const _NL_COLLATE_INDIRECTMB: C2RustUnnamed_3 = 196613;
pub const _NL_COLLATE_EXTRAMB: C2RustUnnamed_3 = 196612;
pub const _NL_COLLATE_WEIGHTMB: C2RustUnnamed_3 = 196611;
pub const _NL_COLLATE_TABLEMB: C2RustUnnamed_3 = 196610;
pub const _NL_COLLATE_RULESETS: C2RustUnnamed_3 = 196609;
pub const _NL_COLLATE_NRULES: C2RustUnnamed_3 = 196608;
pub const _NL_NUM_LC_TIME: C2RustUnnamed_3 = 131231;
pub const _NL_WABALTMON_12: C2RustUnnamed_3 = 131230;
pub const _NL_WABALTMON_11: C2RustUnnamed_3 = 131229;
pub const _NL_WABALTMON_10: C2RustUnnamed_3 = 131228;
pub const _NL_WABALTMON_9: C2RustUnnamed_3 = 131227;
pub const _NL_WABALTMON_8: C2RustUnnamed_3 = 131226;
pub const _NL_WABALTMON_7: C2RustUnnamed_3 = 131225;
pub const _NL_WABALTMON_6: C2RustUnnamed_3 = 131224;
pub const _NL_WABALTMON_5: C2RustUnnamed_3 = 131223;
pub const _NL_WABALTMON_4: C2RustUnnamed_3 = 131222;
pub const _NL_WABALTMON_3: C2RustUnnamed_3 = 131221;
pub const _NL_WABALTMON_2: C2RustUnnamed_3 = 131220;
pub const _NL_WABALTMON_1: C2RustUnnamed_3 = 131219;
pub const _NL_ABALTMON_12: C2RustUnnamed_3 = 131218;
pub const _NL_ABALTMON_11: C2RustUnnamed_3 = 131217;
pub const _NL_ABALTMON_10: C2RustUnnamed_3 = 131216;
pub const _NL_ABALTMON_9: C2RustUnnamed_3 = 131215;
pub const _NL_ABALTMON_8: C2RustUnnamed_3 = 131214;
pub const _NL_ABALTMON_7: C2RustUnnamed_3 = 131213;
pub const _NL_ABALTMON_6: C2RustUnnamed_3 = 131212;
pub const _NL_ABALTMON_5: C2RustUnnamed_3 = 131211;
pub const _NL_ABALTMON_4: C2RustUnnamed_3 = 131210;
pub const _NL_ABALTMON_3: C2RustUnnamed_3 = 131209;
pub const _NL_ABALTMON_2: C2RustUnnamed_3 = 131208;
pub const _NL_ABALTMON_1: C2RustUnnamed_3 = 131207;
pub const _NL_WALTMON_12: C2RustUnnamed_3 = 131206;
pub const _NL_WALTMON_11: C2RustUnnamed_3 = 131205;
pub const _NL_WALTMON_10: C2RustUnnamed_3 = 131204;
pub const _NL_WALTMON_9: C2RustUnnamed_3 = 131203;
pub const _NL_WALTMON_8: C2RustUnnamed_3 = 131202;
pub const _NL_WALTMON_7: C2RustUnnamed_3 = 131201;
pub const _NL_WALTMON_6: C2RustUnnamed_3 = 131200;
pub const _NL_WALTMON_5: C2RustUnnamed_3 = 131199;
pub const _NL_WALTMON_4: C2RustUnnamed_3 = 131198;
pub const _NL_WALTMON_3: C2RustUnnamed_3 = 131197;
pub const _NL_WALTMON_2: C2RustUnnamed_3 = 131196;
pub const _NL_WALTMON_1: C2RustUnnamed_3 = 131195;
pub const __ALTMON_12: C2RustUnnamed_3 = 131194;
pub const __ALTMON_11: C2RustUnnamed_3 = 131193;
pub const __ALTMON_10: C2RustUnnamed_3 = 131192;
pub const __ALTMON_9: C2RustUnnamed_3 = 131191;
pub const __ALTMON_8: C2RustUnnamed_3 = 131190;
pub const __ALTMON_7: C2RustUnnamed_3 = 131189;
pub const __ALTMON_6: C2RustUnnamed_3 = 131188;
pub const __ALTMON_5: C2RustUnnamed_3 = 131187;
pub const __ALTMON_4: C2RustUnnamed_3 = 131186;
pub const __ALTMON_3: C2RustUnnamed_3 = 131185;
pub const __ALTMON_2: C2RustUnnamed_3 = 131184;
pub const __ALTMON_1: C2RustUnnamed_3 = 131183;
pub const _NL_TIME_CODESET: C2RustUnnamed_3 = 131182;
pub const _NL_W_DATE_FMT: C2RustUnnamed_3 = 131181;
pub const _DATE_FMT: C2RustUnnamed_3 = 131180;
pub const _NL_TIME_TIMEZONE: C2RustUnnamed_3 = 131179;
pub const _NL_TIME_CAL_DIRECTION: C2RustUnnamed_3 = 131178;
pub const _NL_TIME_FIRST_WORKDAY: C2RustUnnamed_3 = 131177;
pub const _NL_TIME_FIRST_WEEKDAY: C2RustUnnamed_3 = 131176;
pub const _NL_TIME_WEEK_1STWEEK: C2RustUnnamed_3 = 131175;
pub const _NL_TIME_WEEK_1STDAY: C2RustUnnamed_3 = 131174;
pub const _NL_TIME_WEEK_NDAYS: C2RustUnnamed_3 = 131173;
pub const _NL_WERA_T_FMT: C2RustUnnamed_3 = 131172;
pub const _NL_WERA_D_T_FMT: C2RustUnnamed_3 = 131171;
pub const _NL_WALT_DIGITS: C2RustUnnamed_3 = 131170;
pub const _NL_WERA_D_FMT: C2RustUnnamed_3 = 131169;
pub const _NL_WERA_YEAR: C2RustUnnamed_3 = 131168;
pub const _NL_WT_FMT_AMPM: C2RustUnnamed_3 = 131167;
pub const _NL_WT_FMT: C2RustUnnamed_3 = 131166;
pub const _NL_WD_FMT: C2RustUnnamed_3 = 131165;
pub const _NL_WD_T_FMT: C2RustUnnamed_3 = 131164;
pub const _NL_WPM_STR: C2RustUnnamed_3 = 131163;
pub const _NL_WAM_STR: C2RustUnnamed_3 = 131162;
pub const _NL_WMON_12: C2RustUnnamed_3 = 131161;
pub const _NL_WMON_11: C2RustUnnamed_3 = 131160;
pub const _NL_WMON_10: C2RustUnnamed_3 = 131159;
pub const _NL_WMON_9: C2RustUnnamed_3 = 131158;
pub const _NL_WMON_8: C2RustUnnamed_3 = 131157;
pub const _NL_WMON_7: C2RustUnnamed_3 = 131156;
pub const _NL_WMON_6: C2RustUnnamed_3 = 131155;
pub const _NL_WMON_5: C2RustUnnamed_3 = 131154;
pub const _NL_WMON_4: C2RustUnnamed_3 = 131153;
pub const _NL_WMON_3: C2RustUnnamed_3 = 131152;
pub const _NL_WMON_2: C2RustUnnamed_3 = 131151;
pub const _NL_WMON_1: C2RustUnnamed_3 = 131150;
pub const _NL_WABMON_12: C2RustUnnamed_3 = 131149;
pub const _NL_WABMON_11: C2RustUnnamed_3 = 131148;
pub const _NL_WABMON_10: C2RustUnnamed_3 = 131147;
pub const _NL_WABMON_9: C2RustUnnamed_3 = 131146;
pub const _NL_WABMON_8: C2RustUnnamed_3 = 131145;
pub const _NL_WABMON_7: C2RustUnnamed_3 = 131144;
pub const _NL_WABMON_6: C2RustUnnamed_3 = 131143;
pub const _NL_WABMON_5: C2RustUnnamed_3 = 131142;
pub const _NL_WABMON_4: C2RustUnnamed_3 = 131141;
pub const _NL_WABMON_3: C2RustUnnamed_3 = 131140;
pub const _NL_WABMON_2: C2RustUnnamed_3 = 131139;
pub const _NL_WABMON_1: C2RustUnnamed_3 = 131138;
pub const _NL_WDAY_7: C2RustUnnamed_3 = 131137;
pub const _NL_WDAY_6: C2RustUnnamed_3 = 131136;
pub const _NL_WDAY_5: C2RustUnnamed_3 = 131135;
pub const _NL_WDAY_4: C2RustUnnamed_3 = 131134;
pub const _NL_WDAY_3: C2RustUnnamed_3 = 131133;
pub const _NL_WDAY_2: C2RustUnnamed_3 = 131132;
pub const _NL_WDAY_1: C2RustUnnamed_3 = 131131;
pub const _NL_WABDAY_7: C2RustUnnamed_3 = 131130;
pub const _NL_WABDAY_6: C2RustUnnamed_3 = 131129;
pub const _NL_WABDAY_5: C2RustUnnamed_3 = 131128;
pub const _NL_WABDAY_4: C2RustUnnamed_3 = 131127;
pub const _NL_WABDAY_3: C2RustUnnamed_3 = 131126;
pub const _NL_WABDAY_2: C2RustUnnamed_3 = 131125;
pub const _NL_WABDAY_1: C2RustUnnamed_3 = 131124;
pub const _NL_TIME_ERA_ENTRIES: C2RustUnnamed_3 = 131123;
pub const _NL_TIME_ERA_NUM_ENTRIES: C2RustUnnamed_3 = 131122;
pub const ERA_T_FMT: C2RustUnnamed_3 = 131121;
pub const ERA_D_T_FMT: C2RustUnnamed_3 = 131120;
pub const ALT_DIGITS: C2RustUnnamed_3 = 131119;
pub const ERA_D_FMT: C2RustUnnamed_3 = 131118;
pub const __ERA_YEAR: C2RustUnnamed_3 = 131117;
pub const ERA: C2RustUnnamed_3 = 131116;
pub const T_FMT_AMPM: C2RustUnnamed_3 = 131115;
pub const T_FMT: C2RustUnnamed_3 = 131114;
pub const D_FMT: C2RustUnnamed_3 = 131113;
pub const D_T_FMT: C2RustUnnamed_3 = 131112;
pub const PM_STR: C2RustUnnamed_3 = 131111;
pub const AM_STR: C2RustUnnamed_3 = 131110;
pub const MON_12: C2RustUnnamed_3 = 131109;
pub const MON_11: C2RustUnnamed_3 = 131108;
pub const MON_10: C2RustUnnamed_3 = 131107;
pub const MON_9: C2RustUnnamed_3 = 131106;
pub const MON_8: C2RustUnnamed_3 = 131105;
pub const MON_7: C2RustUnnamed_3 = 131104;
pub const MON_6: C2RustUnnamed_3 = 131103;
pub const MON_5: C2RustUnnamed_3 = 131102;
pub const MON_4: C2RustUnnamed_3 = 131101;
pub const MON_3: C2RustUnnamed_3 = 131100;
pub const MON_2: C2RustUnnamed_3 = 131099;
pub const MON_1: C2RustUnnamed_3 = 131098;
pub const ABMON_12: C2RustUnnamed_3 = 131097;
pub const ABMON_11: C2RustUnnamed_3 = 131096;
pub const ABMON_10: C2RustUnnamed_3 = 131095;
pub const ABMON_9: C2RustUnnamed_3 = 131094;
pub const ABMON_8: C2RustUnnamed_3 = 131093;
pub const ABMON_7: C2RustUnnamed_3 = 131092;
pub const ABMON_6: C2RustUnnamed_3 = 131091;
pub const ABMON_5: C2RustUnnamed_3 = 131090;
pub const ABMON_4: C2RustUnnamed_3 = 131089;
pub const ABMON_3: C2RustUnnamed_3 = 131088;
pub const ABMON_2: C2RustUnnamed_3 = 131087;
pub const ABMON_1: C2RustUnnamed_3 = 131086;
pub const DAY_7: C2RustUnnamed_3 = 131085;
pub const DAY_6: C2RustUnnamed_3 = 131084;
pub const DAY_5: C2RustUnnamed_3 = 131083;
pub const DAY_4: C2RustUnnamed_3 = 131082;
pub const DAY_3: C2RustUnnamed_3 = 131081;
pub const DAY_2: C2RustUnnamed_3 = 131080;
pub const DAY_1: C2RustUnnamed_3 = 131079;
pub const ABDAY_7: C2RustUnnamed_3 = 131078;
pub const ABDAY_6: C2RustUnnamed_3 = 131077;
pub const ABDAY_5: C2RustUnnamed_3 = 131076;
pub const ABDAY_4: C2RustUnnamed_3 = 131075;
pub const ABDAY_3: C2RustUnnamed_3 = 131074;
pub const ABDAY_2: C2RustUnnamed_3 = 131073;
pub const ABDAY_1: C2RustUnnamed_3 = 131072;
pub type C2RustUnnamed_4 = libc::c_uint;
pub const IDN2_NO_ALABEL_ROUNDTRIP: C2RustUnnamed_4 = 128;
pub const IDN2_NO_TR46: C2RustUnnamed_4 = 64;
pub const IDN2_USE_STD3_ASCII_RULES: C2RustUnnamed_4 = 32;
pub const IDN2_ALLOW_UNASSIGNED: C2RustUnnamed_4 = 16;
pub const IDN2_ALABEL_ROUNDTRIP: C2RustUnnamed_4 = 2;
pub const IDN2_NFC_INPUT: C2RustUnnamed_4 = 1;
pub type C2RustUnnamed_5 = libc::c_int;
pub const IDN2_ALABEL_ROUNDTRIP_FAILED: C2RustUnnamed_5 = -314;
pub const IDN2_INVALID_NONTRANSITIONAL: C2RustUnnamed_5 = -313;
pub const IDN2_INVALID_TRANSITIONAL: C2RustUnnamed_5 = -312;
pub const IDN2_DOT_IN_LABEL: C2RustUnnamed_5 = -311;
pub const IDN2_BIDI: C2RustUnnamed_5 = -310;
pub const IDN2_UNASSIGNED: C2RustUnnamed_5 = -309;
pub const IDN2_CONTEXTO_NO_RULE: C2RustUnnamed_5 = -308;
pub const IDN2_CONTEXTO: C2RustUnnamed_5 = -307;
pub const IDN2_CONTEXTJ_NO_RULE: C2RustUnnamed_5 = -306;
pub const IDN2_CONTEXTJ: C2RustUnnamed_5 = -305;
pub const IDN2_DISALLOWED: C2RustUnnamed_5 = -304;
pub const IDN2_LEADING_COMBINING: C2RustUnnamed_5 = -303;
pub const IDN2_HYPHEN_STARTEND: C2RustUnnamed_5 = -302;
pub const IDN2_2HYPHEN: C2RustUnnamed_5 = -301;
pub const IDN2_NOT_NFC: C2RustUnnamed_5 = -300;
pub const IDN2_INVALID_FLAGS: C2RustUnnamed_5 = -209;
pub const IDN2_UALABEL_MISMATCH: C2RustUnnamed_5 = -208;
pub const IDN2_INVALID_ALABEL: C2RustUnnamed_5 = -207;
pub const IDN2_TOO_BIG_LABEL: C2RustUnnamed_5 = -206;
pub const IDN2_TOO_BIG_DOMAIN: C2RustUnnamed_5 = -205;
pub const IDN2_PUNYCODE_OVERFLOW: C2RustUnnamed_5 = -204;
pub const IDN2_PUNYCODE_BIG_OUTPUT: C2RustUnnamed_5 = -203;
pub const IDN2_PUNYCODE_BAD_INPUT: C2RustUnnamed_5 = -202;
pub const IDN2_NFC: C2RustUnnamed_5 = -201;
pub const IDN2_ENCODING_ERROR: C2RustUnnamed_5 = -200;
pub const IDN2_ICONV_FAIL: C2RustUnnamed_5 = -102;
pub const IDN2_NO_CODESET: C2RustUnnamed_5 = -101;
pub const IDN2_MALLOC: C2RustUnnamed_5 = -100;
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
unsafe extern "C" fn c_isspace(mut c: libc::c_int) -> bool {
    match c {
        32 | 9 | 10 | 11 | 12 | 13 => return 1 as libc::c_int != 0,
        _ => return 0 as libc::c_int != 0,
    };
}
pub unsafe extern "C" fn parse_charset(
    mut str: *const libc::c_char,
) -> *mut libc::c_char {
    let mut end: *const libc::c_char = 0 as *const libc::c_char;
    let mut charset: *mut libc::c_char = 0 as *mut libc::c_char;
    if str.is_null() || *str == 0 {
        return 0 as *mut libc::c_char;
    }
    str = c_strcasestr(str, b"charset=\0" as *const u8 as *const libc::c_char);
    if str.is_null() {
        return 0 as *mut libc::c_char;
    }
    str = str.offset(8 as libc::c_int as isize);
    end = str;
    while *end as libc::c_int != 0 && !c_isspace(*end as libc::c_int) {
        end = end.offset(1);
        end;
    }
    charset = strdupdelim(str, end);
    if !check_encoding_name(charset) {
        rpl_free(charset as *mut libc::c_void);
        charset = 0 as *mut libc::c_char;
        return 0 as *mut libc::c_char;
    }
    return charset;
}
pub unsafe extern "C" fn find_locale() -> *const libc::c_char {
    let mut encoding: *const libc::c_char = nl_langinfo(CODESET as libc::c_int);
    if encoding.is_null() || *encoding == 0 {
        return xstrdup(b"ASCII\0" as *const u8 as *const libc::c_char);
    }
    return xstrdup(encoding);
}
pub unsafe extern "C" fn check_encoding_name(mut encoding: *const libc::c_char) -> bool {
    let mut s: *const libc::c_char = encoding;
    while *s != 0 {
        if !c_isascii(*s as libc::c_int)
            || c_isspace(*s as libc::c_int) as libc::c_int != 0
        {
            logprintf(
                LOG_VERBOSE,
                dcgettext(
                    0 as *const libc::c_char,
                    b"Encoding %s isn't valid\n\0" as *const u8 as *const libc::c_char,
                    5 as libc::c_int,
                ),
                quote(encoding),
            );
            return 0 as libc::c_int != 0;
        }
        s = s.offset(1);
        s;
    }
    return 1 as libc::c_int != 0;
}
unsafe extern "C" fn do_conversion(
    mut tocode: *const libc::c_char,
    mut fromcode: *const libc::c_char,
    mut in_org: *const libc::c_char,
    mut inlen: size_t,
    mut out: *mut *mut libc::c_char,
) -> bool {
    let mut cd: iconv_t = 0 as *mut libc::c_void;
    let mut len: size_t = 0;
    let mut done: size_t = 0;
    let mut outlen: size_t = 0;
    let mut invalid: libc::c_int = 0 as libc::c_int;
    let mut tooshort: libc::c_int = 0 as libc::c_int;
    let mut s: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut in_0: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut in_save: *mut libc::c_char = 0 as *mut libc::c_char;
    cd = iconv_open(tocode, fromcode);
    if cd == -(1 as libc::c_int) as iconv_t {
        logprintf(
            LOG_VERBOSE,
            dcgettext(
                0 as *const libc::c_char,
                b"Conversion from %s to %s isn't supported\n\0" as *const u8
                    as *const libc::c_char,
                5 as libc::c_int,
            ),
            quote_n(0 as libc::c_int, fromcode),
            quote_n(1 as libc::c_int, tocode),
        );
        *out = 0 as *mut libc::c_char;
        return 0 as libc::c_int != 0;
    }
    in_0 = xstrndup(in_org, inlen);
    in_save = in_0;
    url_unescape_except_reserved(in_0);
    inlen = strlen(in_0);
    outlen = inlen.wrapping_mul(2 as libc::c_int as libc::c_ulong);
    len = outlen;
    s = xmalloc(outlen.wrapping_add(1 as libc::c_int as libc::c_ulong))
        as *mut libc::c_char;
    *out = s;
    done = 0 as libc::c_int as size_t;
    loop {
        if iconv(cd, &mut in_0 as *mut *mut libc::c_char, &mut inlen, out, &mut outlen)
            != -(1 as libc::c_int) as size_t
            && iconv(cd, 0 as *mut *mut libc::c_char, 0 as *mut size_t, out, &mut outlen)
                != -(1 as libc::c_int) as size_t
        {
            *out = s;
            *s
                .offset(len as isize)
                .offset(-(outlen as isize))
                .offset(-(done as isize)) = '\0' as i32 as libc::c_char;
            rpl_free(in_save as *mut libc::c_void);
            in_save = 0 as *mut libc::c_char;
            iconv_close(cd);
            if opt.debug as libc::c_long != 0 {
                if (strchr(in_org, '@' as i32)).is_null()
                    && (strchr(*out, '@' as i32)).is_null()
                {
                    debug_logprintf(
                        b"converted '%s' (%s) -> '%s' (%s)\n\0" as *const u8
                            as *const libc::c_char,
                        in_org,
                        fromcode,
                        *out,
                        tocode,
                    );
                } else {
                    debug_logprintf(
                        b"logging suppressed, strings may contain password\n\0"
                            as *const u8 as *const libc::c_char,
                    );
                }
            }
            return 1 as libc::c_int != 0;
        }
        if *__errno_location() == 22 as libc::c_int
            || *__errno_location() == 84 as libc::c_int
        {
            if invalid == 0 {
                logprintf(
                    LOG_VERBOSE,
                    dcgettext(
                        0 as *const libc::c_char,
                        b"Incomplete or invalid multibyte sequence encountered\n\0"
                            as *const u8 as *const libc::c_char,
                        5 as libc::c_int,
                    ),
                );
            }
            invalid += 1;
            invalid;
            **out = *in_0;
            in_0 = in_0.offset(1);
            in_0;
            inlen = inlen.wrapping_sub(1);
            inlen;
            *out = (*out).offset(1);
            *out;
            outlen = outlen.wrapping_sub(1);
            outlen;
        } else if *__errno_location() == 7 as libc::c_int {
            tooshort += 1;
            tooshort;
            done = len;
            len = done
                .wrapping_add(inlen.wrapping_mul(2 as libc::c_int as libc::c_ulong));
            s = xrealloc(
                s as *mut libc::c_void,
                len.wrapping_add(1 as libc::c_int as libc::c_ulong),
            ) as *mut libc::c_char;
            *out = s.offset(done as isize).offset(-(outlen as isize));
            outlen = (outlen as libc::c_ulong)
                .wrapping_add(inlen.wrapping_mul(2 as libc::c_int as libc::c_ulong))
                as size_t as size_t;
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
            break;
        }
    }
    rpl_free(in_save as *mut libc::c_void);
    in_save = 0 as *mut libc::c_char;
    iconv_close(cd);
    if opt.debug as libc::c_long != 0 {
        if (strchr(in_org, '@' as i32)).is_null() && (strchr(*out, '@' as i32)).is_null()
        {
            debug_logprintf(
                b"converted '%s' (%s) -> '%s' (%s)\n\0" as *const u8
                    as *const libc::c_char,
                in_org,
                fromcode,
                *out,
                tocode,
            );
        } else {
            debug_logprintf(
                b"logging suppressed, strings may contain password\n\0" as *const u8
                    as *const libc::c_char,
            );
        }
    }
    return 0 as libc::c_int != 0;
}
pub unsafe extern "C" fn locale_to_utf8(
    mut str: *const libc::c_char,
) -> *const libc::c_char {
    let mut new: *mut libc::c_char = 0 as *mut libc::c_char;
    if (opt.locale).is_null() {
        logprintf(
            LOG_VERBOSE,
            dcgettext(
                0 as *const libc::c_char,
                b"locale_to_utf8: locale is unset\n\0" as *const u8
                    as *const libc::c_char,
                5 as libc::c_int,
            ),
        );
        opt.locale = find_locale();
    }
    if (opt.locale).is_null()
        || c_strcasecmp(opt.locale, b"utf-8\0" as *const u8 as *const libc::c_char) == 0
    {
        return str;
    }
    if do_conversion(
        b"UTF-8\0" as *const u8 as *const libc::c_char,
        opt.locale,
        str as *mut libc::c_char,
        strlen(str as *mut libc::c_char),
        &mut new,
    ) {
        return new as *const libc::c_char;
    }
    rpl_free(new as *mut libc::c_void);
    new = 0 as *mut libc::c_char;
    return str;
}
pub unsafe extern "C" fn idn_encode(
    mut i: *const iri,
    mut host: *const libc::c_char,
) -> *mut libc::c_char {
    let mut ret: libc::c_int = 0;
    let mut ascii_encoded: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut utf8_encoded: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut src: *const libc::c_char = 0 as *const libc::c_char;
    if !(*i).utf8_encode {
        if !remote_to_utf8(i, host, &mut utf8_encoded) {
            return 0 as *mut libc::c_char;
        }
        src = utf8_encoded;
    } else {
        src = host;
    }
    ret = idn2_lookup_u8(
        src as *mut uint8_t,
        &mut ascii_encoded as *mut *mut libc::c_char as *mut *mut uint8_t,
        IDN2_NONTRANSITIONAL as libc::c_int,
    );
    if ret != IDN2_OK as libc::c_int {
        ret = idn2_lookup_u8(
            src as *mut uint8_t,
            &mut ascii_encoded as *mut *mut libc::c_char as *mut *mut uint8_t,
            IDN2_TRANSITIONAL as libc::c_int,
        );
    }
    if ret != IDN2_OK as libc::c_int {
        logprintf(
            LOG_VERBOSE,
            dcgettext(
                0 as *const libc::c_char,
                b"idn_encode failed (%d): %s\n\0" as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ),
            ret,
            quote(idn2_strerror(ret)),
        );
    }
    rpl_free(utf8_encoded as *mut libc::c_void);
    utf8_encoded = 0 as *mut libc::c_char;
    if ret == IDN2_OK as libc::c_int && !ascii_encoded.is_null() {
        let mut tmp: *mut libc::c_char = xstrdup(ascii_encoded);
        idn2_free(ascii_encoded as *mut libc::c_void);
        ascii_encoded = tmp;
    }
    return if ret == IDN2_OK as libc::c_int {
        ascii_encoded
    } else {
        0 as *mut libc::c_char
    };
}
pub unsafe extern "C" fn idn_decode(mut host: *const libc::c_char) -> *mut libc::c_char {
    return xstrdup(host);
}
pub unsafe extern "C" fn remote_to_utf8(
    mut iri: *const iri,
    mut str: *const libc::c_char,
    mut new: *mut *mut libc::c_char,
) -> bool {
    let mut ret: bool = 0 as libc::c_int != 0;
    if ((*iri).uri_encoding).is_null() {
        return 0 as libc::c_int != 0;
    }
    if c_strcasecmp((*iri).uri_encoding, b"UTF-8\0" as *const u8 as *const libc::c_char)
        == 0
    {
        let mut p: *const libc::c_uchar = 0 as *const libc::c_uchar;
        p = str as *mut libc::c_uchar;
        while *p != 0 {
            if *p as libc::c_int > 127 as libc::c_int {
                *new = strdup(str);
                return 1 as libc::c_int != 0;
            }
            p = p.offset(1);
            p;
        }
        return 0 as libc::c_int != 0;
    }
    if do_conversion(
        b"UTF-8\0" as *const u8 as *const libc::c_char,
        (*iri).uri_encoding,
        str,
        strlen(str),
        new,
    ) {
        ret = 1 as libc::c_int != 0;
    }
    if !(*new).is_null() && strcmp(str, *new) == 0 {
        rpl_free(*new as *mut libc::c_void);
        *new = 0 as *mut libc::c_char;
        return 0 as libc::c_int != 0;
    }
    return ret;
}
pub unsafe extern "C" fn iri_new() -> *mut iri {
    let mut i: *mut iri = xmalloc(::std::mem::size_of::<iri>() as libc::c_ulong)
        as *mut iri;
    (*i)
        .uri_encoding = if !(opt.encoding_remote).is_null() {
        xstrdup(opt.encoding_remote)
    } else {
        0 as *mut libc::c_char
    };
    (*i).content_encoding = 0 as *mut libc::c_char;
    (*i).orig_url = 0 as *mut libc::c_char;
    (*i).utf8_encode = opt.enable_iri;
    return i;
}
pub unsafe extern "C" fn iri_dup(mut src: *const iri) -> *mut iri {
    let mut i: *mut iri = xmalloc(::std::mem::size_of::<iri>() as libc::c_ulong)
        as *mut iri;
    (*i)
        .uri_encoding = if !((*src).uri_encoding).is_null() {
        xstrdup((*src).uri_encoding)
    } else {
        0 as *mut libc::c_char
    };
    (*i)
        .content_encoding = if !((*src).content_encoding).is_null() {
        xstrdup((*src).content_encoding)
    } else {
        0 as *mut libc::c_char
    };
    (*i)
        .orig_url = if !((*src).orig_url).is_null() {
        xstrdup((*src).orig_url)
    } else {
        0 as *mut libc::c_char
    };
    (*i).utf8_encode = (*src).utf8_encode;
    return i;
}
pub unsafe extern "C" fn iri_free(mut i: *mut iri) {
    if !i.is_null() {
        rpl_free((*i).uri_encoding as *mut libc::c_void);
        (*i).uri_encoding = 0 as *mut libc::c_char;
        rpl_free((*i).content_encoding as *mut libc::c_void);
        (*i).content_encoding = 0 as *mut libc::c_char;
        rpl_free((*i).orig_url as *mut libc::c_void);
        (*i).orig_url = 0 as *mut libc::c_char;
        rpl_free(i as *mut libc::c_void);
        i = 0 as *mut iri;
    }
}
pub unsafe extern "C" fn set_uri_encoding(
    mut i: *mut iri,
    mut charset: *const libc::c_char,
    mut force: bool,
) {
    if opt.debug as libc::c_long != 0 {
        debug_logprintf(
            b"URI encoding = %s\n\0" as *const u8 as *const libc::c_char,
            if !charset.is_null() {
                quote(charset)
            } else {
                b"None\0" as *const u8 as *const libc::c_char
            },
        );
    }
    if !force && !(opt.encoding_remote).is_null() {
        return;
    }
    if !((*i).uri_encoding).is_null() {
        if !charset.is_null() && c_strcasecmp((*i).uri_encoding, charset) == 0 {
            return;
        }
        rpl_free((*i).uri_encoding as *mut libc::c_void);
        (*i).uri_encoding = 0 as *mut libc::c_char;
    }
    (*i)
        .uri_encoding = if !charset.is_null() {
        xstrdup(charset)
    } else {
        0 as *mut libc::c_char
    };
}
pub unsafe extern "C" fn set_content_encoding(
    mut i: *mut iri,
    mut charset: *const libc::c_char,
) {
    if opt.debug as libc::c_long != 0 {
        debug_logprintf(
            b"URI content encoding = %s\n\0" as *const u8 as *const libc::c_char,
            if !charset.is_null() {
                quote(charset)
            } else {
                b"None\0" as *const u8 as *const libc::c_char
            },
        );
    }
    if !(opt.encoding_remote).is_null() {
        return;
    }
    if !((*i).content_encoding).is_null() {
        if !charset.is_null() && c_strcasecmp((*i).content_encoding, charset) == 0 {
            return;
        }
        rpl_free((*i).content_encoding as *mut libc::c_void);
        (*i).content_encoding = 0 as *mut libc::c_char;
    }
    (*i)
        .content_encoding = if !charset.is_null() {
        xstrdup(charset)
    } else {
        0 as *mut libc::c_char
    };
}
