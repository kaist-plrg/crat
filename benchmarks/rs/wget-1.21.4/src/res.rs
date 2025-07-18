use ::libc;
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
    fn strerror(_: libc::c_int) -> *mut libc::c_char;
    fn rpl_free(_: *mut libc::c_void);
    static mut opt: options;
    fn xmalloc(s: size_t) -> *mut libc::c_void;
    fn xcalloc(n: size_t, s: size_t) -> *mut libc::c_void;
    fn xrealloc(p: *mut libc::c_void, s: size_t) -> *mut libc::c_void;
    fn xstrdup(str: *const libc::c_char) -> *mut libc::c_char;
    fn snprintf(
        _: *mut libc::c_char,
        _: libc::c_ulong,
        _: *const libc::c_char,
        _: ...
    ) -> libc::c_int;
    fn logprintf(_: log_options, _: *const libc::c_char, _: ...);
    fn debug_logprintf(_: *const libc::c_char, _: ...);
    fn logputs(_: log_options, _: *const libc::c_char);
    fn quote(arg: *const libc::c_char) -> *const libc::c_char;
    fn iri_new() -> *mut iri;
    fn iri_free(i: *mut iri);
    fn set_uri_encoding(i: *mut iri, charset: *const libc::c_char, force: bool);
    fn __errno_location() -> *mut libc::c_int;
    fn strdupdelim(_: *const libc::c_char, _: *const libc::c_char) -> *mut libc::c_char;
    fn aprintf(_: *const libc::c_char, _: ...) -> *mut libc::c_char;
    fn wget_read_file(_: *const libc::c_char) -> *mut file_memory;
    fn wget_read_file_free(_: *mut file_memory);
    fn hash_table_get(_: *const hash_table, _: *const libc::c_void) -> *mut libc::c_void;
    fn hash_table_get_pair(
        _: *const hash_table,
        _: *const libc::c_void,
        _: *mut libc::c_void,
        _: *mut libc::c_void,
    ) -> libc::c_int;
    fn hash_table_put(
        _: *mut hash_table,
        _: *const libc::c_void,
        _: *const libc::c_void,
    );
    fn make_nocase_string_hash_table(_: libc::c_int) -> *mut hash_table;
    fn url_parse(
        _: *const libc::c_char,
        _: *mut libc::c_int,
        iri: *mut iri,
        percent_encode: bool,
    ) -> *mut url;
    fn url_error(_: libc::c_int) -> *const libc::c_char;
    fn url_free(_: *mut url);
    fn uri_merge(_: *const libc::c_char, _: *const libc::c_char) -> *mut libc::c_char;
    fn are_urls_equal(u1: *const libc::c_char, u2: *const libc::c_char) -> bool;
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
pub struct file_memory {
    pub content: *mut libc::c_char,
    pub length: libc::c_long,
    pub mmap_p: libc::c_int,
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
pub struct robot_specs {
    pub count: libc::c_int,
    pub size: libc::c_int,
    pub paths: *mut path_info,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct path_info {
    pub path: *mut libc::c_char,
    pub allowedp: bool,
    pub user_agent_exact_p: bool,
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
unsafe extern "C" fn c_isspace(mut c: libc::c_int) -> bool {
    match c {
        32 | 9 | 10 | 11 | 12 | 13 => return 1 as libc::c_int != 0,
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
unsafe extern "C" fn _unhex(mut c: libc::c_uchar) -> libc::c_uchar {
    return (if c as libc::c_int <= '9' as i32 {
        c as libc::c_int - '0' as i32
    } else if c as libc::c_int <= 'F' as i32 {
        c as libc::c_int - 'A' as i32 + 10 as libc::c_int
    } else {
        c as libc::c_int - 'a' as i32 + 10 as libc::c_int
    }) as libc::c_uchar;
}
unsafe extern "C" fn match_user_agent(
    mut agent: *const libc::c_char,
    mut length: libc::c_int,
    mut matches_0: *mut bool,
    mut exact_match: *mut bool,
) {
    if length == 1 as libc::c_int && *agent as libc::c_int == '*' as i32 {
        *matches_0 = 1 as libc::c_int != 0;
        *exact_match = 0 as libc::c_int != 0;
    } else if agent.offset(length as isize).offset_from(agent) as libc::c_long
        as libc::c_ulong
        == (::std::mem::size_of::<[libc::c_char; 5]>() as libc::c_ulong)
            .wrapping_sub(1 as libc::c_int as libc::c_ulong)
        && c_strncasecmp(
            agent,
            b"wget\0" as *const u8 as *const libc::c_char,
            (::std::mem::size_of::<[libc::c_char; 5]>() as libc::c_ulong)
                .wrapping_sub(1 as libc::c_int as libc::c_ulong),
        ) == 0
    {
        *matches_0 = 1 as libc::c_int != 0;
        *exact_match = 1 as libc::c_int != 0;
    } else {
        *matches_0 = 0 as libc::c_int != 0;
        *exact_match = 0 as libc::c_int != 0;
    };
}
unsafe extern "C" fn add_path(
    mut specs: *mut robot_specs,
    mut path_b: *const libc::c_char,
    mut path_e: *const libc::c_char,
    mut allowedp: bool,
    mut exactp: bool,
) {
    let mut pp: path_info = path_info {
        path: 0 as *mut libc::c_char,
        allowedp: false,
        user_agent_exact_p: false,
    };
    if path_b < path_e && *path_b as libc::c_int == '/' as i32 {
        path_b = path_b.offset(1);
        path_b;
    }
    pp.path = strdupdelim(path_b, path_e);
    pp.allowedp = allowedp;
    pp.user_agent_exact_p = exactp;
    (*specs).count += 1;
    (*specs).count;
    if (*specs).count > (*specs).size {
        if (*specs).size == 0 as libc::c_int {
            (*specs).size = 1 as libc::c_int;
        } else {
            (*specs).size <<= 1 as libc::c_int;
        }
        (*specs)
            .paths = xrealloc(
            (*specs).paths as *mut libc::c_void,
            ((*specs).size as libc::c_ulong)
                .wrapping_mul(::std::mem::size_of::<path_info>() as libc::c_ulong),
        ) as *mut path_info;
    }
    *((*specs).paths).offset(((*specs).count - 1 as libc::c_int) as isize) = pp;
}
unsafe extern "C" fn prune_non_exact(mut specs: *mut robot_specs) {
    let mut newpaths: *mut path_info = 0 as *mut path_info;
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut cnt: libc::c_int = 0;
    cnt = 0 as libc::c_int;
    i = 0 as libc::c_int;
    while i < (*specs).count {
        if (*((*specs).paths).offset(i as isize)).user_agent_exact_p {
            cnt += 1;
            cnt;
        }
        i += 1;
        i;
    }
    newpaths = xmalloc(
        (cnt as libc::c_ulong)
            .wrapping_mul(::std::mem::size_of::<path_info>() as libc::c_ulong),
    ) as *mut path_info;
    i = 0 as libc::c_int;
    j = 0 as libc::c_int;
    while i < (*specs).count {
        if (*((*specs).paths).offset(i as isize)).user_agent_exact_p {
            let fresh0 = j;
            j = j + 1;
            *newpaths.offset(fresh0 as isize) = *((*specs).paths).offset(i as isize);
        } else {
            rpl_free((*((*specs).paths).offset(i as isize)).path as *mut libc::c_void);
            let ref mut fresh1 = (*((*specs).paths).offset(i as isize)).path;
            *fresh1 = 0 as *mut libc::c_char;
        }
        i += 1;
        i;
    }
    rpl_free((*specs).paths as *mut libc::c_void);
    (*specs).paths = 0 as *mut path_info;
    (*specs).paths = newpaths;
    (*specs).count = cnt;
    (*specs).size = cnt;
}
pub unsafe extern "C" fn res_parse(
    mut source: *const libc::c_char,
    mut length: libc::c_int,
) -> *mut robot_specs {
    let mut line_count: libc::c_int = 1 as libc::c_int;
    let mut p: *const libc::c_char = source;
    let mut end: *const libc::c_char = source.offset(length as isize);
    let mut user_agent_applies: bool = 0 as libc::c_int != 0;
    let mut user_agent_exact: bool = 0 as libc::c_int != 0;
    let mut found_exact: bool = 0 as libc::c_int != 0;
    let mut record_count: libc::c_int = 0 as libc::c_int;
    let mut specs: *mut robot_specs = xcalloc(
        1 as libc::c_int as size_t,
        ::std::mem::size_of::<robot_specs>() as libc::c_ulong,
    ) as *mut robot_specs;
    loop {
        let mut lineend: *const libc::c_char = 0 as *const libc::c_char;
        let mut lineend_real: *const libc::c_char = 0 as *const libc::c_char;
        let mut field_b: *const libc::c_char = 0 as *const libc::c_char;
        let mut field_e: *const libc::c_char = 0 as *const libc::c_char;
        let mut value_b: *const libc::c_char = 0 as *const libc::c_char;
        let mut value_e: *const libc::c_char = 0 as *const libc::c_char;
        if p == end {
            break;
        }
        lineend_real = memchr(
            p as *const libc::c_void,
            '\n' as i32,
            end.offset_from(p) as libc::c_long as libc::c_ulong,
        ) as *const libc::c_char;
        if !lineend_real.is_null() {
            lineend_real = lineend_real.offset(1);
            lineend_real;
        } else {
            lineend_real = end;
        }
        lineend = lineend_real;
        while !(p >= lineend) && c_isspace(*p as libc::c_int) as libc::c_int != 0 {
            p = p.offset(1);
            p;
        }
        if !(p >= lineend || *p as libc::c_int == '#' as i32) {
            lineend = p;
            while lineend < lineend_real {
                if (lineend == p
                    || c_isspace(
                        *lineend.offset(-(1 as libc::c_int as isize)) as libc::c_int,
                    ) as libc::c_int != 0) && *lineend as libc::c_int == '#' as i32
                {
                    break;
                }
                lineend = lineend.offset(1);
                lineend;
            }
            while lineend > p
                && c_isspace(
                    *lineend.offset(-(1 as libc::c_int as isize)) as libc::c_int,
                ) as libc::c_int != 0
            {
                lineend = lineend.offset(-1);
                lineend;
            }
            field_b = p;
            while !(p >= lineend)
                && (c_isalnum(*p as libc::c_int) as libc::c_int != 0
                    || *p as libc::c_int == '-' as i32)
            {
                p = p.offset(1);
                p;
            }
            field_e = p;
            while !(p >= lineend) && c_isspace(*p as libc::c_int) as libc::c_int != 0 {
                p = p.offset(1);
                p;
            }
            if field_b == field_e || p >= lineend || *p as libc::c_int != ':' as i32 {
                if opt.debug as libc::c_long != 0 {
                    debug_logprintf(
                        b"Ignoring malformed line %d\n\0" as *const u8
                            as *const libc::c_char,
                        line_count,
                    );
                }
            } else {
                p = p.offset(1);
                p;
                while !(p >= lineend) && c_isspace(*p as libc::c_int) as libc::c_int != 0
                {
                    p = p.offset(1);
                    p;
                }
                value_b = p;
                while !(p >= lineend) {
                    p = p.offset(1);
                    p;
                }
                value_e = p;
                if field_e.offset_from(field_b) as libc::c_long as libc::c_ulong
                    == (::std::mem::size_of::<[libc::c_char; 11]>() as libc::c_ulong)
                        .wrapping_sub(1 as libc::c_int as libc::c_ulong)
                    && c_strncasecmp(
                        field_b,
                        b"user-agent\0" as *const u8 as *const libc::c_char,
                        (::std::mem::size_of::<[libc::c_char; 11]>() as libc::c_ulong)
                            .wrapping_sub(1 as libc::c_int as libc::c_ulong),
                    ) == 0
                {
                    if record_count != 0 as libc::c_int
                        || user_agent_applies as libc::c_int == 0 as libc::c_int
                    {
                        match_user_agent(
                            value_b,
                            value_e.offset_from(value_b) as libc::c_long as libc::c_int,
                            &mut user_agent_applies,
                            &mut user_agent_exact,
                        );
                    }
                    if user_agent_exact {
                        found_exact = 1 as libc::c_int != 0;
                    }
                    record_count = 0 as libc::c_int;
                } else if field_e.offset_from(field_b) as libc::c_long as libc::c_ulong
                    == (::std::mem::size_of::<[libc::c_char; 6]>() as libc::c_ulong)
                        .wrapping_sub(1 as libc::c_int as libc::c_ulong)
                    && c_strncasecmp(
                        field_b,
                        b"allow\0" as *const u8 as *const libc::c_char,
                        (::std::mem::size_of::<[libc::c_char; 6]>() as libc::c_ulong)
                            .wrapping_sub(1 as libc::c_int as libc::c_ulong),
                    ) == 0
                {
                    if user_agent_applies {
                        add_path(
                            specs,
                            value_b,
                            value_e,
                            1 as libc::c_int != 0,
                            user_agent_exact,
                        );
                    }
                    record_count += 1;
                    record_count;
                } else if field_e.offset_from(field_b) as libc::c_long as libc::c_ulong
                    == (::std::mem::size_of::<[libc::c_char; 9]>() as libc::c_ulong)
                        .wrapping_sub(1 as libc::c_int as libc::c_ulong)
                    && c_strncasecmp(
                        field_b,
                        b"disallow\0" as *const u8 as *const libc::c_char,
                        (::std::mem::size_of::<[libc::c_char; 9]>() as libc::c_ulong)
                            .wrapping_sub(1 as libc::c_int as libc::c_ulong),
                    ) == 0
                {
                    if user_agent_applies {
                        let mut allowed: bool = 0 as libc::c_int != 0;
                        if value_b == value_e {
                            allowed = 1 as libc::c_int != 0;
                        }
                        add_path(specs, value_b, value_e, allowed, user_agent_exact);
                    }
                    record_count += 1;
                    record_count;
                } else if opt.debug as libc::c_long != 0 {
                    debug_logprintf(
                        b"Ignoring unknown field at line %d\n\0" as *const u8
                            as *const libc::c_char,
                        line_count,
                    );
                }
            }
        }
        p = lineend_real;
        line_count += 1;
        line_count;
    }
    if found_exact {
        prune_non_exact(specs);
    } else if (*specs).size > (*specs).count {
        (*specs)
            .paths = xrealloc(
            (*specs).paths as *mut libc::c_void,
            ((*specs).count as libc::c_ulong)
                .wrapping_mul(::std::mem::size_of::<path_info>() as libc::c_ulong),
        ) as *mut path_info;
        (*specs).size = (*specs).count;
    }
    return specs;
}
pub unsafe extern "C" fn res_parse_from_file(
    mut filename: *const libc::c_char,
) -> *mut robot_specs {
    let mut specs: *mut robot_specs = 0 as *mut robot_specs;
    let mut fm: *mut file_memory = wget_read_file(filename);
    if fm.is_null() {
        logprintf(
            LOG_NOTQUIET,
            dcgettext(
                0 as *const libc::c_char,
                b"Cannot open %s: %s\n\0" as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ),
            filename,
            strerror(*__errno_location()),
        );
        return 0 as *mut robot_specs;
    }
    specs = res_parse((*fm).content, (*fm).length as libc::c_int);
    wget_read_file_free(fm);
    return specs;
}
unsafe extern "C" fn free_specs(mut specs: *mut robot_specs) {
    let mut i: libc::c_int = 0;
    i = 0 as libc::c_int;
    while i < (*specs).count {
        rpl_free((*((*specs).paths).offset(i as isize)).path as *mut libc::c_void);
        let ref mut fresh2 = (*((*specs).paths).offset(i as isize)).path;
        *fresh2 = 0 as *mut libc::c_char;
        i += 1;
        i;
    }
    rpl_free((*specs).paths as *mut libc::c_void);
    (*specs).paths = 0 as *mut path_info;
    rpl_free(specs as *mut libc::c_void);
    specs = 0 as *mut robot_specs;
}
unsafe extern "C" fn matches(
    mut record_path: *const libc::c_char,
    mut url_path: *const libc::c_char,
) -> bool {
    let mut rp: *const libc::c_char = record_path;
    let mut up: *const libc::c_char = url_path;
    loop {
        let mut rc: libc::c_char = *rp;
        let mut uc: libc::c_char = *up;
        if rc == 0 {
            return 1 as libc::c_int != 0;
        }
        if uc == 0 {
            return 0 as libc::c_int != 0;
        }
        if rc as libc::c_int == '%' as i32
            && c_isxdigit(*rp.offset(1 as libc::c_int as isize) as libc::c_int)
                as libc::c_int != 0
            && c_isxdigit(*rp.offset(2 as libc::c_int as isize) as libc::c_int)
                as libc::c_int != 0
        {
            let mut decoded: libc::c_uchar = (((_unhex(
                *rp.offset(1 as libc::c_int as isize) as libc::c_uchar,
            ) as libc::c_int) << 4 as libc::c_int)
                + _unhex(*rp.offset(2 as libc::c_int as isize) as libc::c_uchar)
                    as libc::c_int) as libc::c_uchar;
            if decoded as libc::c_int != '/' as i32 {
                rc = decoded as libc::c_char;
                rp = rp.offset(2 as libc::c_int as isize);
            }
        }
        if uc as libc::c_int == '%' as i32
            && c_isxdigit(*up.offset(1 as libc::c_int as isize) as libc::c_int)
                as libc::c_int != 0
            && c_isxdigit(*up.offset(2 as libc::c_int as isize) as libc::c_int)
                as libc::c_int != 0
        {
            let mut decoded_0: libc::c_uchar = (((_unhex(
                *up.offset(1 as libc::c_int as isize) as libc::c_uchar,
            ) as libc::c_int) << 4 as libc::c_int)
                + _unhex(*up.offset(2 as libc::c_int as isize) as libc::c_uchar)
                    as libc::c_int) as libc::c_uchar;
            if decoded_0 as libc::c_int != '/' as i32 {
                uc = decoded_0 as libc::c_char;
                up = up.offset(2 as libc::c_int as isize);
            }
        }
        if rc as libc::c_int != uc as libc::c_int {
            return 0 as libc::c_int != 0;
        }
        rp = rp.offset(1);
        rp;
        up = up.offset(1);
        up;
    };
}
pub unsafe extern "C" fn res_match_path(
    mut specs: *const robot_specs,
    mut path: *const libc::c_char,
) -> bool {
    let mut i: libc::c_int = 0;
    if specs.is_null() {
        return 1 as libc::c_int != 0;
    }
    i = 0 as libc::c_int;
    while i < (*specs).count {
        if matches((*((*specs).paths).offset(i as isize)).path, path) {
            let mut allowedp: bool = (*((*specs).paths).offset(i as isize)).allowedp;
            if opt.debug as libc::c_long != 0 {
                debug_logprintf(
                    b"%s path %s because of rule %s.\n\0" as *const u8
                        as *const libc::c_char,
                    if allowedp as libc::c_int != 0 {
                        b"Allowing\0" as *const u8 as *const libc::c_char
                    } else {
                        b"Rejecting\0" as *const u8 as *const libc::c_char
                    },
                    path,
                    quote((*((*specs).paths).offset(i as isize)).path),
                );
            }
            return allowedp;
        }
        i += 1;
        i;
    }
    return 1 as libc::c_int != 0;
}
static mut registered_specs: *mut hash_table = 0 as *const hash_table as *mut hash_table;
pub unsafe extern "C" fn res_register_specs(
    mut host: *const libc::c_char,
    mut port: libc::c_int,
    mut specs: *mut robot_specs,
) {
    let mut old: *mut robot_specs = 0 as *mut robot_specs;
    let mut buf: [libc::c_char; 256] = [0; 256];
    let mut hp: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut hp_old: *mut libc::c_char = 0 as *mut libc::c_char;
    if snprintf(
        buf.as_mut_ptr(),
        ::std::mem::size_of::<[libc::c_char; 256]>() as libc::c_ulong,
        b"%s:%d\0" as *const u8 as *const libc::c_char,
        host,
        port,
    ) as libc::c_uint as libc::c_ulong
        >= ::std::mem::size_of::<[libc::c_char; 256]>() as libc::c_ulong
    {
        hp = aprintf(b"%s:%d\0" as *const u8 as *const libc::c_char, host, port);
    } else {
        hp = buf.as_mut_ptr();
    }
    if registered_specs.is_null() {
        registered_specs = make_nocase_string_hash_table(0 as libc::c_int);
    }
    if hash_table_get_pair(
        registered_specs,
        hp as *const libc::c_void,
        &mut hp_old as *mut *mut libc::c_char as *mut libc::c_void,
        &mut old as *mut *mut robot_specs as *mut libc::c_void,
    ) != 0
    {
        if hp != buf.as_mut_ptr() {
            rpl_free(hp as *mut libc::c_void);
            hp = 0 as *mut libc::c_char;
        }
        if !old.is_null() {
            free_specs(old);
        }
        hash_table_put(
            registered_specs,
            hp_old as *const libc::c_void,
            specs as *const libc::c_void,
        );
    } else {
        hash_table_put(
            registered_specs,
            (if hp == buf.as_mut_ptr() { xstrdup(hp) } else { hp })
                as *const libc::c_void,
            specs as *const libc::c_void,
        );
    };
}
pub unsafe extern "C" fn res_get_specs(
    mut host: *const libc::c_char,
    mut port: libc::c_int,
) -> *mut robot_specs {
    let mut buf: [libc::c_char; 256] = [0; 256];
    let mut hp: *mut libc::c_char = 0 as *mut libc::c_char;
    if registered_specs.is_null() {
        return 0 as *mut robot_specs;
    }
    if snprintf(
        buf.as_mut_ptr(),
        ::std::mem::size_of::<[libc::c_char; 256]>() as libc::c_ulong,
        b"%s:%d\0" as *const u8 as *const libc::c_char,
        host,
        port,
    ) as libc::c_uint as libc::c_ulong
        >= ::std::mem::size_of::<[libc::c_char; 256]>() as libc::c_ulong
    {
        hp = aprintf(b"%s:%d\0" as *const u8 as *const libc::c_char, host, port);
    } else {
        hp = buf.as_mut_ptr();
    }
    return hash_table_get(registered_specs, hp as *const libc::c_void)
        as *mut robot_specs;
}
pub unsafe extern "C" fn res_retrieve_file(
    mut url: *const libc::c_char,
    mut file: *mut *mut libc::c_char,
    mut iri: *mut iri,
) -> bool {
    let mut i: *mut iri = iri_new();
    let mut err: uerr_t = NOCONERROR;
    let mut robots_url: *mut libc::c_char = uri_merge(
        url,
        b"/robots.txt\0" as *const u8 as *const libc::c_char,
    );
    let mut saved_ts_val: libc::c_int = opt.timestamping as libc::c_int;
    let mut saved_sp_val: libc::c_int = opt.spider as libc::c_int;
    let mut url_err: libc::c_int = 0;
    let mut url_parsed: *mut url = 0 as *mut url;
    set_uri_encoding(i, (*iri).uri_encoding, 0 as libc::c_int != 0);
    (*i).utf8_encode = 0 as libc::c_int != 0;
    logputs(
        LOG_VERBOSE,
        dcgettext(
            0 as *const libc::c_char,
            b"Loading robots.txt; please ignore errors.\n\0" as *const u8
                as *const libc::c_char,
            5 as libc::c_int,
        ),
    );
    *file = 0 as *mut libc::c_char;
    opt.timestamping = 0 as libc::c_int != 0;
    opt.spider = 0 as libc::c_int != 0;
    url_parsed = url_parse(robots_url, &mut url_err, i, 1 as libc::c_int != 0);
    if url_parsed.is_null() {
        logprintf(
            LOG_NOTQUIET,
            b"%s: %s.\n\0" as *const u8 as *const libc::c_char,
            robots_url,
            url_error(url_err),
        );
        err = URLERROR;
    } else {
        err = retrieve_url(
            url_parsed,
            robots_url,
            file,
            0 as *mut *mut libc::c_char,
            0 as *const libc::c_char,
            0 as *mut libc::c_int,
            0 as libc::c_int != 0,
            i,
            0 as libc::c_int != 0,
        );
        url_free(url_parsed);
    }
    opt.timestamping = saved_ts_val != 0;
    opt.spider = saved_sp_val != 0;
    rpl_free(robots_url as *mut libc::c_void);
    robots_url = 0 as *mut libc::c_char;
    iri_free(i);
    if err as libc::c_uint != RETROK as libc::c_int as libc::c_uint && !(*file).is_null()
    {
        rpl_free(*file as *mut libc::c_void);
        *file = 0 as *mut libc::c_char;
    }
    return err as libc::c_uint == RETROK as libc::c_int as libc::c_uint;
}
pub unsafe extern "C" fn is_robots_txt_url(mut url: *const libc::c_char) -> bool {
    let mut robots_url: *mut libc::c_char = uri_merge(
        url,
        b"/robots.txt\0" as *const u8 as *const libc::c_char,
    );
    let mut ret: bool = are_urls_equal(url, robots_url);
    rpl_free(robots_url as *mut libc::c_void);
    robots_url = 0 as *mut libc::c_char;
    return ret;
}
