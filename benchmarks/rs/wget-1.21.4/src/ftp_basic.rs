use ::libc;
extern "C" {
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
    fn strncmp(
        _: *const libc::c_char,
        _: *const libc::c_char,
        _: libc::c_ulong,
    ) -> libc::c_int;
    fn strdup(_: *const libc::c_char) -> *mut libc::c_char;
    fn strchr(_: *const libc::c_char, _: libc::c_int) -> *mut libc::c_char;
    fn strpbrk(_: *const libc::c_char, _: *const libc::c_char) -> *mut libc::c_char;
    fn strtok(_: *mut libc::c_char, _: *const libc::c_char) -> *mut libc::c_char;
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    fn strncasecmp(
        _: *const libc::c_char,
        _: *const libc::c_char,
        _: libc::c_ulong,
    ) -> libc::c_int;
    fn rpl_free(_: *mut libc::c_void);
    static mut opt: options;
    fn abort() -> !;
    fn rpl_strtoll(
        string: *const libc::c_char,
        endptr: *mut *mut libc::c_char,
        base: libc::c_int,
    ) -> libc::c_longlong;
    fn xmalloc(s: size_t) -> *mut libc::c_void;
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
    fn quotearg_style(s: quoting_style, arg: *const libc::c_char) -> *mut libc::c_char;
    fn __errno_location() -> *mut libc::c_int;
    fn concat_strings(_: *const libc::c_char, _: ...) -> *mut libc::c_char;
    fn number_to_static_string(_: wgint) -> *mut libc::c_char;
    fn print_address(_: *const ip_address) -> *const libc::c_char;
    fn bind_local(_: *const ip_address, _: *mut libc::c_int) -> libc::c_int;
    fn socket_ip_address(_: libc::c_int, _: *mut ip_address, _: libc::c_int) -> bool;
    fn fd_write(
        _: libc::c_int,
        _: *mut libc::c_char,
        _: libc::c_int,
        _: libc::c_double,
    ) -> libc::c_int;
    fn fd_close(_: libc::c_int);
    fn skey_response(
        _: libc::c_int,
        _: *const libc::c_char,
        _: *const libc::c_char,
    ) -> *const libc::c_char;
    fn fd_read_line(_: libc::c_int) -> *mut libc::c_char;
    fn c_strcasecmp(s1: *const libc::c_char, s2: *const libc::c_char) -> libc::c_int;
    fn c_strncasecmp(
        s1: *const libc::c_char,
        s2: *const libc::c_char,
        n: size_t,
    ) -> libc::c_int;
}
pub type __uint8_t = libc::c_uchar;
pub type __uint16_t = libc::c_ushort;
pub type __uint32_t = libc::c_uint;
pub type __int64_t = libc::c_long;
pub type size_t = libc::c_ulong;
pub type int64_t = __int64_t;
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
pub struct in6_addr {
    pub __in6_u: C2RustUnnamed_3,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_3 {
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
    pub data: C2RustUnnamed_4,
    pub ipv6_scope: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_4 {
    pub d4: in_addr,
    pub d6: in6_addr,
}
pub type C2RustUnnamed_5 = libc::c_uint;
pub const ENDPOINT_PEER: C2RustUnnamed_5 = 1;
pub const ENDPOINT_LOCAL: C2RustUnnamed_5 = 0;
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
#[inline]
unsafe extern "C" fn c_isdigit(mut c: libc::c_int) -> bool {
    match c {
        48 | 49 | 50 | 51 | 52 | 53 | 54 | 55 | 56 | 57 => return 1 as libc::c_int != 0,
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
pub unsafe extern "C" fn ftp_response(
    mut fd: libc::c_int,
    mut ret_line: *mut *mut libc::c_char,
) -> uerr_t {
    loop {
        let mut p: *mut libc::c_char = 0 as *mut libc::c_char;
        let mut line: *mut libc::c_char = fd_read_line(fd);
        if line.is_null() {
            return FTPRERR;
        }
        p = strpbrk(line, b"\r\n\0" as *const u8 as *const libc::c_char);
        if !p.is_null() {
            *p = 0 as libc::c_int as libc::c_char;
        }
        if opt.server_response {
            logprintf(
                LOG_NOTQUIET,
                b"%s\n\0" as *const u8 as *const libc::c_char,
                quotearg_style(escape_quoting_style, line),
            );
        } else if opt.debug as libc::c_long != 0 {
            debug_logprintf(
                b"%s\n\0" as *const u8 as *const libc::c_char,
                quotearg_style(escape_quoting_style, line),
            );
        }
        if c_isdigit(*line.offset(0 as libc::c_int as isize) as libc::c_int)
            as libc::c_int != 0
            && c_isdigit(*line.offset(1 as libc::c_int as isize) as libc::c_int)
                as libc::c_int != 0
            && c_isdigit(*line.offset(2 as libc::c_int as isize) as libc::c_int)
                as libc::c_int != 0
            && *line.offset(3 as libc::c_int as isize) as libc::c_int == ' ' as i32
        {
            *ret_line = line;
            return FTPOK;
        }
        rpl_free(line as *mut libc::c_void);
        line = 0 as *mut libc::c_char;
    };
}
unsafe extern "C" fn ftp_request(
    mut command: *const libc::c_char,
    mut value: *const libc::c_char,
) -> *mut libc::c_char {
    let mut res: *mut libc::c_char = 0 as *mut libc::c_char;
    if !value.is_null() {
        let mut defanged: *mut libc::c_char = 0 as *mut libc::c_char;
        let mut buf: [libc::c_char; 256] = [0; 256];
        if !(strpbrk(value, b"\r\n\0" as *const u8 as *const libc::c_char)).is_null() {
            let mut p: *mut libc::c_char = 0 as *mut libc::c_char;
            let mut len: size_t = strlen(value);
            if len < ::std::mem::size_of::<[libc::c_char; 256]>() as libc::c_ulong {
                defanged = buf.as_mut_ptr();
            } else {
                defanged = xmalloc(len.wrapping_add(1 as libc::c_int as libc::c_ulong))
                    as *mut libc::c_char;
            }
            memcpy(
                defanged as *mut libc::c_void,
                value as *const libc::c_void,
                len.wrapping_add(1 as libc::c_int as libc::c_ulong),
            );
            p = defanged;
            while *p != 0 {
                if *p as libc::c_int == '\r' as i32 || *p as libc::c_int == '\n' as i32 {
                    *p = ' ' as i32 as libc::c_char;
                }
                p = p.offset(1);
                p;
            }
            if opt.debug as libc::c_long != 0 {
                debug_logprintf(
                    b"\nDetected newlines in %s \"%s\"; changing to %s \"%s\"\n\0"
                        as *const u8 as *const libc::c_char,
                    command,
                    quotearg_style(escape_quoting_style, value),
                    command,
                    quotearg_style(escape_quoting_style, defanged),
                );
            }
            value = defanged;
        }
        res = concat_strings(
            command,
            b" \0" as *const u8 as *const libc::c_char,
            value,
            b"\r\n\0" as *const u8 as *const libc::c_char,
            0 as *mut libc::c_char,
        );
        if defanged != buf.as_mut_ptr() {
            rpl_free(defanged as *mut libc::c_void);
            defanged = 0 as *mut libc::c_char;
        }
    } else {
        res = concat_strings(
            command,
            b"\r\n\0" as *const u8 as *const libc::c_char,
            0 as *mut libc::c_char,
        );
    }
    if opt.server_response {
        if strncmp(
            res,
            b"PASS\0" as *const u8 as *const libc::c_char,
            4 as libc::c_int as libc::c_ulong,
        ) != 0 as libc::c_int
        {
            logprintf(
                LOG_ALWAYS,
                b"--> %s\n\0" as *const u8 as *const libc::c_char,
                res,
            );
        } else {
            logputs(
                LOG_ALWAYS,
                b"--> PASS Turtle Power!\n\n\0" as *const u8 as *const libc::c_char,
            );
        }
    } else if opt.debug as libc::c_long != 0 {
        debug_logprintf(b"\n--> %s\n\0" as *const u8 as *const libc::c_char, res);
    }
    return res;
}
pub unsafe extern "C" fn ftp_greeting(mut csock: libc::c_int) -> uerr_t {
    let mut err: uerr_t = FTPOK;
    let mut response: *mut libc::c_char = 0 as *mut libc::c_char;
    err = ftp_response(csock, &mut response);
    if !(err as libc::c_uint != FTPOK as libc::c_int as libc::c_uint) {
        if *response as libc::c_int != '2' as i32 {
            err = FTPSRVERR;
        }
    }
    if !response.is_null() {
        rpl_free(response as *mut libc::c_void);
        response = 0 as *mut libc::c_char;
    }
    return err;
}
pub unsafe extern "C" fn ftp_login(
    mut csock: libc::c_int,
    mut acc: *const libc::c_char,
    mut pass: *const libc::c_char,
) -> uerr_t {
    let mut err: uerr_t = NOCONERROR;
    let mut request: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut respline: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut nwritten: libc::c_int = 0;
    request = ftp_request(b"USER\0" as *const u8 as *const libc::c_char, acc);
    nwritten = fd_write(
        csock,
        request,
        strlen(request) as libc::c_int,
        -(1 as libc::c_int) as libc::c_double,
    );
    if nwritten < 0 as libc::c_int {
        rpl_free(request as *mut libc::c_void);
        request = 0 as *mut libc::c_char;
        return WRITEFAILED;
    }
    rpl_free(request as *mut libc::c_void);
    request = 0 as *mut libc::c_char;
    err = ftp_response(csock, &mut respline);
    if err as libc::c_uint != FTPOK as libc::c_int as libc::c_uint {
        return err;
    }
    if *respline as libc::c_int == '2' as i32 {
        rpl_free(respline as *mut libc::c_void);
        respline = 0 as *mut libc::c_char;
        return FTPOK;
    }
    if *respline as libc::c_int != '3' as i32 {
        rpl_free(respline as *mut libc::c_void);
        respline = 0 as *mut libc::c_char;
        return FTPLOGREFUSED;
    }
    static mut skey_head: [*const libc::c_char; 2] = [
        b"331 s/key \0" as *const u8 as *const libc::c_char,
        b"331 opiekey \0" as *const u8 as *const libc::c_char,
    ];
    let mut i: size_t = 0;
    let mut seed: *const libc::c_char = 0 as *const libc::c_char;
    i = 0 as libc::c_int as size_t;
    while i
        < (::std::mem::size_of::<[*const libc::c_char; 2]>() as libc::c_ulong)
            .wrapping_div(::std::mem::size_of::<*const libc::c_char>() as libc::c_ulong)
    {
        let mut l: libc::c_int = strlen(skey_head[i as usize]) as libc::c_int;
        if 0 as libc::c_int
            == c_strncasecmp(skey_head[i as usize], respline, l as size_t)
        {
            seed = respline.offset(l as isize);
            break;
        } else {
            i = i.wrapping_add(1);
            i;
        }
    }
    if !seed.is_null() {
        let mut skey_sequence: libc::c_int = 0 as libc::c_int;
        while c_isdigit(*seed as libc::c_int) {
            skey_sequence = 10 as libc::c_int * skey_sequence + *seed as libc::c_int
                - '0' as i32;
            seed = seed.offset(1);
            seed;
        }
        if *seed as libc::c_int == ' ' as i32 {
            seed = seed.offset(1);
            seed;
        } else {
            rpl_free(respline as *mut libc::c_void);
            respline = 0 as *mut libc::c_char;
            return FTPLOGREFUSED;
        }
        pass = skey_response(skey_sequence, seed, pass);
    }
    rpl_free(respline as *mut libc::c_void);
    respline = 0 as *mut libc::c_char;
    request = ftp_request(b"PASS\0" as *const u8 as *const libc::c_char, pass);
    nwritten = fd_write(
        csock,
        request,
        strlen(request) as libc::c_int,
        -(1 as libc::c_int) as libc::c_double,
    );
    if nwritten < 0 as libc::c_int {
        rpl_free(request as *mut libc::c_void);
        request = 0 as *mut libc::c_char;
        return WRITEFAILED;
    }
    rpl_free(request as *mut libc::c_void);
    request = 0 as *mut libc::c_char;
    err = ftp_response(csock, &mut respline);
    if err as libc::c_uint != FTPOK as libc::c_int as libc::c_uint {
        return err;
    }
    if *respline as libc::c_int != '2' as i32 {
        rpl_free(respline as *mut libc::c_void);
        respline = 0 as *mut libc::c_char;
        return FTPLOGINC;
    }
    rpl_free(respline as *mut libc::c_void);
    respline = 0 as *mut libc::c_char;
    return FTPOK;
}
unsafe extern "C" fn ip_address_to_port_repr(
    mut addr: *const ip_address,
    mut port: libc::c_int,
    mut buf: *mut libc::c_char,
    mut buflen: size_t,
) {
    let mut ptr: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    ptr = &(*addr).data as *const C2RustUnnamed_4 as *mut libc::c_void
        as *mut libc::c_uchar;
    snprintf(
        buf,
        buflen,
        b"%d,%d,%d,%d,%d,%d\0" as *const u8 as *const libc::c_char,
        *ptr.offset(0 as libc::c_int as isize) as libc::c_int,
        *ptr.offset(1 as libc::c_int as isize) as libc::c_int,
        *ptr.offset(2 as libc::c_int as isize) as libc::c_int,
        *ptr.offset(3 as libc::c_int as isize) as libc::c_int,
        (port & 0xff00 as libc::c_int) >> 8 as libc::c_int,
        port & 0xff as libc::c_int,
    );
    *buf
        .offset(
            buflen.wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize,
        ) = '\0' as i32 as libc::c_char;
}
pub unsafe extern "C" fn ftp_port(
    mut csock: libc::c_int,
    mut local_sock: *mut libc::c_int,
) -> uerr_t {
    let mut err: uerr_t = NOCONERROR;
    let mut request: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut respline: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut addr: ip_address = ip_address {
        family: 0,
        data: C2RustUnnamed_4 {
            d4: in_addr { s_addr: 0 },
        },
        ipv6_scope: 0,
    };
    let mut nwritten: libc::c_int = 0;
    let mut port: libc::c_int = 0;
    let mut bytes: [libc::c_char; 25] = [0; 25];
    if !socket_ip_address(csock, &mut addr, ENDPOINT_LOCAL as libc::c_int) {
        return FTPSYSERR;
    }
    port = 0 as libc::c_int;
    *local_sock = bind_local(&mut addr, &mut port);
    if *local_sock < 0 as libc::c_int {
        return FTPSYSERR;
    }
    ip_address_to_port_repr(
        &mut addr,
        port,
        bytes.as_mut_ptr(),
        ::std::mem::size_of::<[libc::c_char; 25]>() as libc::c_ulong,
    );
    request = ftp_request(
        b"PORT\0" as *const u8 as *const libc::c_char,
        bytes.as_mut_ptr(),
    );
    nwritten = fd_write(
        csock,
        request,
        strlen(request) as libc::c_int,
        -(1 as libc::c_int) as libc::c_double,
    );
    if nwritten < 0 as libc::c_int {
        rpl_free(request as *mut libc::c_void);
        request = 0 as *mut libc::c_char;
        fd_close(*local_sock);
        return WRITEFAILED;
    }
    rpl_free(request as *mut libc::c_void);
    request = 0 as *mut libc::c_char;
    err = ftp_response(csock, &mut respline);
    if err as libc::c_uint != FTPOK as libc::c_int as libc::c_uint {
        fd_close(*local_sock);
        return err;
    }
    if *respline as libc::c_int != '2' as i32 {
        rpl_free(respline as *mut libc::c_void);
        respline = 0 as *mut libc::c_char;
        fd_close(*local_sock);
        return FTPPORTERR;
    }
    rpl_free(respline as *mut libc::c_void);
    respline = 0 as *mut libc::c_char;
    return FTPOK;
}
unsafe extern "C" fn ip_address_to_lprt_repr(
    mut addr: *const ip_address,
    mut port: libc::c_int,
    mut buf: *mut libc::c_char,
    mut buflen: size_t,
) {
    let mut ptr: *mut libc::c_uchar = &(*addr).data as *const C2RustUnnamed_4
        as *mut libc::c_void as *mut libc::c_uchar;
    match (*addr).family {
        2 => {
            snprintf(
                buf,
                buflen,
                b"%d,%d,%d,%d,%d,%d,%d,%d,%d\0" as *const u8 as *const libc::c_char,
                4 as libc::c_int,
                4 as libc::c_int,
                *ptr.offset(0 as libc::c_int as isize) as libc::c_int,
                *ptr.offset(1 as libc::c_int as isize) as libc::c_int,
                *ptr.offset(2 as libc::c_int as isize) as libc::c_int,
                *ptr.offset(3 as libc::c_int as isize) as libc::c_int,
                2 as libc::c_int,
                (port & 0xff00 as libc::c_int) >> 8 as libc::c_int,
                port & 0xff as libc::c_int,
            );
        }
        10 => {
            snprintf(
                buf,
                buflen,
                b"%d,%d,%d,%d,%d,%d,%d,%d,%d,%d,%d,%d,%d,%d,%d,%d,%d,%d,%d,%d,%d\0"
                    as *const u8 as *const libc::c_char,
                6 as libc::c_int,
                16 as libc::c_int,
                *ptr.offset(0 as libc::c_int as isize) as libc::c_int,
                *ptr.offset(1 as libc::c_int as isize) as libc::c_int,
                *ptr.offset(2 as libc::c_int as isize) as libc::c_int,
                *ptr.offset(3 as libc::c_int as isize) as libc::c_int,
                *ptr.offset(4 as libc::c_int as isize) as libc::c_int,
                *ptr.offset(5 as libc::c_int as isize) as libc::c_int,
                *ptr.offset(6 as libc::c_int as isize) as libc::c_int,
                *ptr.offset(7 as libc::c_int as isize) as libc::c_int,
                *ptr.offset(8 as libc::c_int as isize) as libc::c_int,
                *ptr.offset(9 as libc::c_int as isize) as libc::c_int,
                *ptr.offset(10 as libc::c_int as isize) as libc::c_int,
                *ptr.offset(11 as libc::c_int as isize) as libc::c_int,
                *ptr.offset(12 as libc::c_int as isize) as libc::c_int,
                *ptr.offset(13 as libc::c_int as isize) as libc::c_int,
                *ptr.offset(14 as libc::c_int as isize) as libc::c_int,
                *ptr.offset(15 as libc::c_int as isize) as libc::c_int,
                2 as libc::c_int,
                (port & 0xff00 as libc::c_int) >> 8 as libc::c_int,
                port & 0xff as libc::c_int,
            );
        }
        _ => {
            abort();
        }
    };
}
pub unsafe extern "C" fn ftp_lprt(
    mut csock: libc::c_int,
    mut local_sock: *mut libc::c_int,
) -> uerr_t {
    let mut err: uerr_t = NOCONERROR;
    let mut request: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut respline: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut addr: ip_address = ip_address {
        family: 0,
        data: C2RustUnnamed_4 {
            d4: in_addr { s_addr: 0 },
        },
        ipv6_scope: 0,
    };
    let mut nwritten: libc::c_int = 0;
    let mut port: libc::c_int = 0;
    let mut bytes: [libc::c_char; 85] = [0; 85];
    if !socket_ip_address(csock, &mut addr, ENDPOINT_LOCAL as libc::c_int) {
        return FTPSYSERR;
    }
    port = 0 as libc::c_int;
    *local_sock = bind_local(&mut addr, &mut port);
    if *local_sock < 0 as libc::c_int {
        return FTPSYSERR;
    }
    ip_address_to_lprt_repr(
        &mut addr,
        port,
        bytes.as_mut_ptr(),
        ::std::mem::size_of::<[libc::c_char; 85]>() as libc::c_ulong,
    );
    request = ftp_request(
        b"LPRT\0" as *const u8 as *const libc::c_char,
        bytes.as_mut_ptr(),
    );
    nwritten = fd_write(
        csock,
        request,
        strlen(request) as libc::c_int,
        -(1 as libc::c_int) as libc::c_double,
    );
    if nwritten < 0 as libc::c_int {
        rpl_free(request as *mut libc::c_void);
        request = 0 as *mut libc::c_char;
        fd_close(*local_sock);
        return WRITEFAILED;
    }
    rpl_free(request as *mut libc::c_void);
    request = 0 as *mut libc::c_char;
    err = ftp_response(csock, &mut respline);
    if err as libc::c_uint != FTPOK as libc::c_int as libc::c_uint {
        fd_close(*local_sock);
        return err;
    }
    if *respline as libc::c_int != '2' as i32 {
        rpl_free(respline as *mut libc::c_void);
        respline = 0 as *mut libc::c_char;
        fd_close(*local_sock);
        return FTPPORTERR;
    }
    rpl_free(respline as *mut libc::c_void);
    respline = 0 as *mut libc::c_char;
    return FTPOK;
}
unsafe extern "C" fn ip_address_to_eprt_repr(
    mut addr: *const ip_address,
    mut port: libc::c_int,
    mut buf: *mut libc::c_char,
    mut buflen: size_t,
) {
    let mut afnum: libc::c_int = 0;
    afnum = if (*addr).family == 2 as libc::c_int {
        1 as libc::c_int
    } else {
        2 as libc::c_int
    };
    snprintf(
        buf,
        buflen,
        b"|%d|%s|%d|\0" as *const u8 as *const libc::c_char,
        afnum,
        print_address(addr),
        port,
    );
    *buf
        .offset(
            buflen.wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize,
        ) = '\0' as i32 as libc::c_char;
}
pub unsafe extern "C" fn ftp_eprt(
    mut csock: libc::c_int,
    mut local_sock: *mut libc::c_int,
) -> uerr_t {
    let mut err: uerr_t = NOCONERROR;
    let mut request: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut respline: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut addr: ip_address = ip_address {
        family: 0,
        data: C2RustUnnamed_4 {
            d4: in_addr { s_addr: 0 },
        },
        ipv6_scope: 0,
    };
    let mut nwritten: libc::c_int = 0;
    let mut port: libc::c_int = 0;
    let mut bytes: [libc::c_char; 57] = [0; 57];
    if !socket_ip_address(csock, &mut addr, ENDPOINT_LOCAL as libc::c_int) {
        return FTPSYSERR;
    }
    port = 0 as libc::c_int;
    *local_sock = bind_local(&mut addr, &mut port);
    if *local_sock < 0 as libc::c_int {
        return FTPSYSERR;
    }
    ip_address_to_eprt_repr(
        &mut addr,
        port,
        bytes.as_mut_ptr(),
        ::std::mem::size_of::<[libc::c_char; 57]>() as libc::c_ulong,
    );
    request = ftp_request(
        b"EPRT\0" as *const u8 as *const libc::c_char,
        bytes.as_mut_ptr(),
    );
    nwritten = fd_write(
        csock,
        request,
        strlen(request) as libc::c_int,
        -(1 as libc::c_int) as libc::c_double,
    );
    if nwritten < 0 as libc::c_int {
        rpl_free(request as *mut libc::c_void);
        request = 0 as *mut libc::c_char;
        fd_close(*local_sock);
        return WRITEFAILED;
    }
    rpl_free(request as *mut libc::c_void);
    request = 0 as *mut libc::c_char;
    err = ftp_response(csock, &mut respline);
    if err as libc::c_uint != FTPOK as libc::c_int as libc::c_uint {
        fd_close(*local_sock);
        return err;
    }
    if *respline as libc::c_int != '2' as i32 {
        rpl_free(respline as *mut libc::c_void);
        respline = 0 as *mut libc::c_char;
        fd_close(*local_sock);
        return FTPPORTERR;
    }
    rpl_free(respline as *mut libc::c_void);
    respline = 0 as *mut libc::c_char;
    return FTPOK;
}
pub unsafe extern "C" fn ftp_pasv(
    mut csock: libc::c_int,
    mut addr: *mut ip_address,
    mut port: *mut libc::c_int,
) -> uerr_t {
    let mut request: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut respline: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut s: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut nwritten: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    let mut err: uerr_t = NOCONERROR;
    let mut tmp: [libc::c_uchar; 6] = [0; 6];
    memset(
        addr as *mut libc::c_void,
        '\0' as i32,
        ::std::mem::size_of::<ip_address>() as libc::c_ulong,
    );
    request = ftp_request(
        b"PASV\0" as *const u8 as *const libc::c_char,
        0 as *const libc::c_char,
    );
    nwritten = fd_write(
        csock,
        request,
        strlen(request) as libc::c_int,
        -(1 as libc::c_int) as libc::c_double,
    );
    if nwritten < 0 as libc::c_int {
        rpl_free(request as *mut libc::c_void);
        request = 0 as *mut libc::c_char;
        return WRITEFAILED;
    }
    rpl_free(request as *mut libc::c_void);
    request = 0 as *mut libc::c_char;
    err = ftp_response(csock, &mut respline);
    if err as libc::c_uint != FTPOK as libc::c_int as libc::c_uint {
        return err;
    }
    if *respline as libc::c_int != '2' as i32 {
        rpl_free(respline as *mut libc::c_void);
        respline = 0 as *mut libc::c_char;
        return FTPNOPASV;
    }
    s = respline;
    s = s.offset(4 as libc::c_int as isize);
    while *s as libc::c_int != 0 && !c_isdigit(*s as libc::c_int) {
        s = s.offset(1);
        s;
    }
    if *s == 0 {
        rpl_free(respline as *mut libc::c_void);
        respline = 0 as *mut libc::c_char;
        return FTPINVPASV;
    }
    i = 0 as libc::c_int;
    while i < 6 as libc::c_int {
        tmp[i as usize] = 0 as libc::c_int as libc::c_uchar;
        while c_isdigit(*s as libc::c_int) {
            tmp[i
                as usize] = (*s as libc::c_int - '0' as i32
                + 10 as libc::c_int * tmp[i as usize] as libc::c_int) as libc::c_uchar;
            s = s.offset(1);
            s;
        }
        if *s as libc::c_int == ',' as i32 {
            s = s.offset(1);
            s;
        } else if i < 5 as libc::c_int {
            rpl_free(respline as *mut libc::c_void);
            respline = 0 as *mut libc::c_char;
            return FTPINVPASV;
        }
        i += 1;
        i;
    }
    rpl_free(respline as *mut libc::c_void);
    respline = 0 as *mut libc::c_char;
    (*addr).family = 2 as libc::c_int;
    memcpy(
        &mut (*addr).data as *mut C2RustUnnamed_4 as *mut libc::c_void,
        tmp.as_mut_ptr() as *const libc::c_void,
        4 as libc::c_int as libc::c_ulong,
    );
    *port = ((tmp[4 as libc::c_int as usize] as libc::c_int) << 8 as libc::c_int
        & 0xff00 as libc::c_int) + tmp[5 as libc::c_int as usize] as libc::c_int;
    return FTPOK;
}
pub unsafe extern "C" fn ftp_lpsv(
    mut csock: libc::c_int,
    mut addr: *mut ip_address,
    mut port: *mut libc::c_int,
) -> uerr_t {
    let mut request: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut respline: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut s: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut nwritten: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    let mut af: libc::c_int = 0;
    let mut addrlen: libc::c_int = 0;
    let mut portlen: libc::c_int = 0;
    let mut err: uerr_t = NOCONERROR;
    let mut tmp: [libc::c_uchar; 16] = [0; 16];
    let mut tmpprt: [libc::c_uchar; 2] = [0; 2];
    memset(
        addr as *mut libc::c_void,
        '\0' as i32,
        ::std::mem::size_of::<ip_address>() as libc::c_ulong,
    );
    request = ftp_request(
        b"LPSV\0" as *const u8 as *const libc::c_char,
        0 as *const libc::c_char,
    );
    nwritten = fd_write(
        csock,
        request,
        strlen(request) as libc::c_int,
        -(1 as libc::c_int) as libc::c_double,
    );
    if nwritten < 0 as libc::c_int {
        rpl_free(request as *mut libc::c_void);
        request = 0 as *mut libc::c_char;
        return WRITEFAILED;
    }
    rpl_free(request as *mut libc::c_void);
    request = 0 as *mut libc::c_char;
    err = ftp_response(csock, &mut respline);
    if err as libc::c_uint != FTPOK as libc::c_int as libc::c_uint {
        return err;
    }
    if *respline as libc::c_int != '2' as i32 {
        rpl_free(respline as *mut libc::c_void);
        respline = 0 as *mut libc::c_char;
        return FTPNOPASV;
    }
    s = respline;
    s = s.offset(4 as libc::c_int as isize);
    while *s as libc::c_int != 0 && !c_isdigit(*s as libc::c_int) {
        s = s.offset(1);
        s;
    }
    if *s == 0 {
        rpl_free(respline as *mut libc::c_void);
        respline = 0 as *mut libc::c_char;
        return FTPINVPASV;
    }
    af = 0 as libc::c_int;
    while c_isdigit(*s as libc::c_int) {
        af = *s as libc::c_int - '0' as i32 + 10 as libc::c_int * af;
        s = s.offset(1);
        s;
    }
    if af != 4 as libc::c_int && af != 6 as libc::c_int {
        rpl_free(respline as *mut libc::c_void);
        respline = 0 as *mut libc::c_char;
        return FTPINVPASV;
    }
    if *s == 0
        || {
            let fresh0 = s;
            s = s.offset(1);
            *fresh0 as libc::c_int != ',' as i32
        }
    {
        rpl_free(respline as *mut libc::c_void);
        respline = 0 as *mut libc::c_char;
        return FTPINVPASV;
    }
    addrlen = 0 as libc::c_int;
    while c_isdigit(*s as libc::c_int) {
        addrlen = *s as libc::c_int - '0' as i32 + 10 as libc::c_int * addrlen;
        s = s.offset(1);
        s;
    }
    if *s == 0
        || {
            let fresh1 = s;
            s = s.offset(1);
            *fresh1 as libc::c_int != ',' as i32
        }
    {
        rpl_free(respline as *mut libc::c_void);
        respline = 0 as *mut libc::c_char;
        return FTPINVPASV;
    }
    if addrlen > 16 as libc::c_int {
        rpl_free(respline as *mut libc::c_void);
        respline = 0 as *mut libc::c_char;
        return FTPINVPASV;
    }
    if af == 4 as libc::c_int && addrlen != 4 as libc::c_int
        || af == 6 as libc::c_int && addrlen != 16 as libc::c_int
    {
        rpl_free(respline as *mut libc::c_void);
        respline = 0 as *mut libc::c_char;
        return FTPINVPASV;
    }
    i = 0 as libc::c_int;
    while i < addrlen {
        tmp[i as usize] = 0 as libc::c_int as libc::c_uchar;
        while c_isdigit(*s as libc::c_int) {
            tmp[i
                as usize] = (*s as libc::c_int - '0' as i32
                + 10 as libc::c_int * tmp[i as usize] as libc::c_int) as libc::c_uchar;
            s = s.offset(1);
            s;
        }
        if *s as libc::c_int == ',' as i32 {
            s = s.offset(1);
            s;
        } else {
            rpl_free(respline as *mut libc::c_void);
            respline = 0 as *mut libc::c_char;
            return FTPINVPASV;
        }
        i += 1;
        i;
    }
    portlen = 0 as libc::c_int;
    while c_isdigit(*s as libc::c_int) {
        portlen = *s as libc::c_int - '0' as i32 + 10 as libc::c_int * portlen;
        s = s.offset(1);
        s;
    }
    if *s == 0
        || {
            let fresh2 = s;
            s = s.offset(1);
            *fresh2 as libc::c_int != ',' as i32
        }
    {
        rpl_free(respline as *mut libc::c_void);
        respline = 0 as *mut libc::c_char;
        return FTPINVPASV;
    }
    if portlen > 2 as libc::c_int {
        rpl_free(respline as *mut libc::c_void);
        respline = 0 as *mut libc::c_char;
        return FTPINVPASV;
    }
    tmpprt[0 as libc::c_int as usize] = 0 as libc::c_int as libc::c_uchar;
    while c_isdigit(*s as libc::c_int) {
        tmpprt[0 as libc::c_int
            as usize] = (*s as libc::c_int - '0' as i32
            + 10 as libc::c_int * tmpprt[0 as libc::c_int as usize] as libc::c_int)
            as libc::c_uchar;
        s = s.offset(1);
        s;
    }
    if *s == 0
        || {
            let fresh3 = s;
            s = s.offset(1);
            *fresh3 as libc::c_int != ',' as i32
        }
    {
        rpl_free(respline as *mut libc::c_void);
        respline = 0 as *mut libc::c_char;
        return FTPINVPASV;
    }
    tmpprt[1 as libc::c_int as usize] = 0 as libc::c_int as libc::c_uchar;
    while c_isdigit(*s as libc::c_int) {
        tmpprt[1 as libc::c_int
            as usize] = (*s as libc::c_int - '0' as i32
            + 10 as libc::c_int * tmpprt[1 as libc::c_int as usize] as libc::c_int)
            as libc::c_uchar;
        s = s.offset(1);
        s;
    }
    if af == 4 as libc::c_int {
        (*addr).family = 2 as libc::c_int;
        memcpy(
            &mut (*addr).data as *mut C2RustUnnamed_4 as *mut libc::c_void,
            tmp.as_mut_ptr() as *const libc::c_void,
            4 as libc::c_int as libc::c_ulong,
        );
        *port = ((tmpprt[0 as libc::c_int as usize] as libc::c_int) << 8 as libc::c_int
            & 0xff00 as libc::c_int) + tmpprt[1 as libc::c_int as usize] as libc::c_int;
        if opt.debug as libc::c_long != 0 {
            debug_logprintf(
                b"lpsv addr is: %s\n\0" as *const u8 as *const libc::c_char,
                print_address(addr),
            );
        }
        if opt.debug as libc::c_long != 0 {
            debug_logprintf(
                b"tmpprt[0] is: %d\n\0" as *const u8 as *const libc::c_char,
                tmpprt[0 as libc::c_int as usize] as libc::c_int,
            );
        }
        if opt.debug as libc::c_long != 0 {
            debug_logprintf(
                b"tmpprt[1] is: %d\n\0" as *const u8 as *const libc::c_char,
                tmpprt[1 as libc::c_int as usize] as libc::c_int,
            );
        }
        if opt.debug as libc::c_long != 0 {
            debug_logprintf(
                b"*port is: %d\n\0" as *const u8 as *const libc::c_char,
                *port,
            );
        }
    } else {
        (*addr).family = 10 as libc::c_int;
        memcpy(
            &mut (*addr).data as *mut C2RustUnnamed_4 as *mut libc::c_void,
            tmp.as_mut_ptr() as *const libc::c_void,
            16 as libc::c_int as libc::c_ulong,
        );
        *port = ((tmpprt[0 as libc::c_int as usize] as libc::c_int) << 8 as libc::c_int
            & 0xff00 as libc::c_int) + tmpprt[1 as libc::c_int as usize] as libc::c_int;
        if opt.debug as libc::c_long != 0 {
            debug_logprintf(
                b"lpsv addr is: %s\n\0" as *const u8 as *const libc::c_char,
                print_address(addr),
            );
        }
        if opt.debug as libc::c_long != 0 {
            debug_logprintf(
                b"tmpprt[0] is: %d\n\0" as *const u8 as *const libc::c_char,
                tmpprt[0 as libc::c_int as usize] as libc::c_int,
            );
        }
        if opt.debug as libc::c_long != 0 {
            debug_logprintf(
                b"tmpprt[1] is: %d\n\0" as *const u8 as *const libc::c_char,
                tmpprt[1 as libc::c_int as usize] as libc::c_int,
            );
        }
        if opt.debug as libc::c_long != 0 {
            debug_logprintf(
                b"*port is: %d\n\0" as *const u8 as *const libc::c_char,
                *port,
            );
        }
    }
    rpl_free(respline as *mut libc::c_void);
    respline = 0 as *mut libc::c_char;
    return FTPOK;
}
pub unsafe extern "C" fn ftp_epsv(
    mut csock: libc::c_int,
    mut ip: *mut ip_address,
    mut port: *mut libc::c_int,
) -> uerr_t {
    let mut request: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut respline: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut start: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut delim: libc::c_char = 0;
    let mut s: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut nwritten: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    let mut err: uerr_t = NOCONERROR;
    let mut tport: libc::c_int = 0;
    request = ftp_request(
        b"EPSV\0" as *const u8 as *const libc::c_char,
        if (*ip).family == 2 as libc::c_int {
            b"1\0" as *const u8 as *const libc::c_char
        } else {
            b"2\0" as *const u8 as *const libc::c_char
        },
    );
    nwritten = fd_write(
        csock,
        request,
        strlen(request) as libc::c_int,
        -(1 as libc::c_int) as libc::c_double,
    );
    if nwritten < 0 as libc::c_int {
        rpl_free(request as *mut libc::c_void);
        request = 0 as *mut libc::c_char;
        return WRITEFAILED;
    }
    rpl_free(request as *mut libc::c_void);
    request = 0 as *mut libc::c_char;
    err = ftp_response(csock, &mut respline);
    if err as libc::c_uint != FTPOK as libc::c_int as libc::c_uint {
        return err;
    }
    if *respline as libc::c_int != '2' as i32 {
        rpl_free(respline as *mut libc::c_void);
        respline = 0 as *mut libc::c_char;
        return FTPNOPASV;
    }
    if opt.debug as libc::c_long != 0 {
        debug_logprintf(
            b"respline is %s\n\0" as *const u8 as *const libc::c_char,
            respline,
        );
    }
    start = strchr(respline, '(' as i32);
    if start.is_null() {
        rpl_free(respline as *mut libc::c_void);
        respline = 0 as *mut libc::c_char;
        return FTPINVPASV;
    }
    s = start.offset(1 as libc::c_int as isize);
    let fresh4 = s;
    s = s.offset(1);
    delim = *fresh4;
    if (delim as libc::c_int) < 33 as libc::c_int
        || delim as libc::c_int > 126 as libc::c_int
    {
        rpl_free(respline as *mut libc::c_void);
        respline = 0 as *mut libc::c_char;
        return FTPINVPASV;
    }
    i = 0 as libc::c_int;
    while i < 2 as libc::c_int {
        let fresh5 = s;
        s = s.offset(1);
        if *fresh5 as libc::c_int != delim as libc::c_int {
            rpl_free(respline as *mut libc::c_void);
            respline = 0 as *mut libc::c_char;
            return FTPINVPASV;
        }
        i += 1;
        i;
    }
    tport = 0 as libc::c_int;
    i = 0 as libc::c_int;
    while i < 5 as libc::c_int && c_isdigit(*s as libc::c_int) as libc::c_int != 0 {
        tport = *s as libc::c_int - '0' as i32 + 10 as libc::c_int * tport;
        i += 1;
        i;
        s = s.offset(1);
        s;
    }
    let fresh6 = s;
    s = s.offset(1);
    if *fresh6 as libc::c_int != delim as libc::c_int {
        rpl_free(respline as *mut libc::c_void);
        respline = 0 as *mut libc::c_char;
        return FTPINVPASV;
    }
    if *s as libc::c_int != ')' as i32 {
        rpl_free(respline as *mut libc::c_void);
        respline = 0 as *mut libc::c_char;
        return FTPINVPASV;
    }
    *port = tport;
    rpl_free(respline as *mut libc::c_void);
    respline = 0 as *mut libc::c_char;
    return FTPOK;
}
pub unsafe extern "C" fn ftp_type(
    mut csock: libc::c_int,
    mut type_0: libc::c_int,
) -> uerr_t {
    let mut request: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut respline: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut nwritten: libc::c_int = 0;
    let mut err: uerr_t = NOCONERROR;
    let mut stype: [libc::c_char; 2] = [0; 2];
    stype[0 as libc::c_int as usize] = type_0 as libc::c_char;
    stype[1 as libc::c_int as usize] = 0 as libc::c_int as libc::c_char;
    request = ftp_request(
        b"TYPE\0" as *const u8 as *const libc::c_char,
        stype.as_mut_ptr(),
    );
    nwritten = fd_write(
        csock,
        request,
        strlen(request) as libc::c_int,
        -(1 as libc::c_int) as libc::c_double,
    );
    if nwritten < 0 as libc::c_int {
        rpl_free(request as *mut libc::c_void);
        request = 0 as *mut libc::c_char;
        return WRITEFAILED;
    }
    rpl_free(request as *mut libc::c_void);
    request = 0 as *mut libc::c_char;
    err = ftp_response(csock, &mut respline);
    if err as libc::c_uint != FTPOK as libc::c_int as libc::c_uint {
        return err;
    }
    if *respline as libc::c_int != '2' as i32 {
        rpl_free(respline as *mut libc::c_void);
        respline = 0 as *mut libc::c_char;
        return FTPUNKNOWNTYPE;
    }
    rpl_free(respline as *mut libc::c_void);
    respline = 0 as *mut libc::c_char;
    return FTPOK;
}
pub unsafe extern "C" fn ftp_cwd(
    mut csock: libc::c_int,
    mut dir: *const libc::c_char,
) -> uerr_t {
    let mut request: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut respline: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut nwritten: libc::c_int = 0;
    let mut err: uerr_t = NOCONERROR;
    request = ftp_request(b"CWD\0" as *const u8 as *const libc::c_char, dir);
    nwritten = fd_write(
        csock,
        request,
        strlen(request) as libc::c_int,
        -(1 as libc::c_int) as libc::c_double,
    );
    if nwritten < 0 as libc::c_int {
        rpl_free(request as *mut libc::c_void);
        request = 0 as *mut libc::c_char;
        return WRITEFAILED;
    }
    rpl_free(request as *mut libc::c_void);
    request = 0 as *mut libc::c_char;
    err = ftp_response(csock, &mut respline);
    if err as libc::c_uint != FTPOK as libc::c_int as libc::c_uint {
        return err;
    }
    if *respline as libc::c_int == '5' as i32 {
        rpl_free(respline as *mut libc::c_void);
        respline = 0 as *mut libc::c_char;
        return FTPNSFOD;
    }
    if *respline as libc::c_int != '2' as i32 {
        rpl_free(respline as *mut libc::c_void);
        respline = 0 as *mut libc::c_char;
        return FTPRERR;
    }
    rpl_free(respline as *mut libc::c_void);
    respline = 0 as *mut libc::c_char;
    return FTPOK;
}
pub unsafe extern "C" fn ftp_rest(mut csock: libc::c_int, mut offset: wgint) -> uerr_t {
    let mut request: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut respline: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut nwritten: libc::c_int = 0;
    let mut err: uerr_t = NOCONERROR;
    request = ftp_request(
        b"REST\0" as *const u8 as *const libc::c_char,
        number_to_static_string(offset),
    );
    nwritten = fd_write(
        csock,
        request,
        strlen(request) as libc::c_int,
        -(1 as libc::c_int) as libc::c_double,
    );
    if nwritten < 0 as libc::c_int {
        rpl_free(request as *mut libc::c_void);
        request = 0 as *mut libc::c_char;
        return WRITEFAILED;
    }
    rpl_free(request as *mut libc::c_void);
    request = 0 as *mut libc::c_char;
    err = ftp_response(csock, &mut respline);
    if err as libc::c_uint != FTPOK as libc::c_int as libc::c_uint {
        return err;
    }
    if *respline as libc::c_int != '3' as i32 {
        rpl_free(respline as *mut libc::c_void);
        respline = 0 as *mut libc::c_char;
        return FTPRESTFAIL;
    }
    rpl_free(respline as *mut libc::c_void);
    respline = 0 as *mut libc::c_char;
    return FTPOK;
}
pub unsafe extern "C" fn ftp_retr(
    mut csock: libc::c_int,
    mut file: *const libc::c_char,
) -> uerr_t {
    let mut request: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut respline: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut nwritten: libc::c_int = 0;
    let mut err: uerr_t = NOCONERROR;
    request = ftp_request(b"RETR\0" as *const u8 as *const libc::c_char, file);
    nwritten = fd_write(
        csock,
        request,
        strlen(request) as libc::c_int,
        -(1 as libc::c_int) as libc::c_double,
    );
    if nwritten < 0 as libc::c_int {
        rpl_free(request as *mut libc::c_void);
        request = 0 as *mut libc::c_char;
        return WRITEFAILED;
    }
    rpl_free(request as *mut libc::c_void);
    request = 0 as *mut libc::c_char;
    err = ftp_response(csock, &mut respline);
    if err as libc::c_uint != FTPOK as libc::c_int as libc::c_uint {
        return err;
    }
    if *respline as libc::c_int == '5' as i32 {
        rpl_free(respline as *mut libc::c_void);
        respline = 0 as *mut libc::c_char;
        return FTPNSFOD;
    }
    if *respline as libc::c_int != '1' as i32 {
        rpl_free(respline as *mut libc::c_void);
        respline = 0 as *mut libc::c_char;
        return FTPRERR;
    }
    rpl_free(respline as *mut libc::c_void);
    respline = 0 as *mut libc::c_char;
    return FTPOK;
}
pub unsafe extern "C" fn ftp_list(
    mut csock: libc::c_int,
    mut file: *const libc::c_char,
    mut avoid_list_a: bool,
    mut avoid_list: bool,
    mut list_a_used: *mut bool,
) -> uerr_t {
    let mut request: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut respline: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut nwritten: libc::c_int = 0;
    let mut err: uerr_t = NOCONERROR;
    let mut ok: bool = 0 as libc::c_int != 0;
    let mut i: size_t = 0 as libc::c_int as size_t;
    static mut list_commands: [*const libc::c_char; 2] = [
        b"LIST -a\0" as *const u8 as *const libc::c_char,
        b"LIST\0" as *const u8 as *const libc::c_char,
    ];
    *list_a_used = 0 as libc::c_int != 0;
    if avoid_list_a {
        i = (::std::mem::size_of::<[*const libc::c_char; 2]>() as libc::c_ulong)
            .wrapping_div(::std::mem::size_of::<*const libc::c_char>() as libc::c_ulong)
            .wrapping_sub(1 as libc::c_int as libc::c_ulong);
        if opt.debug as libc::c_long != 0 {
            debug_logprintf(
                b"(skipping \"LIST -a\")\0" as *const u8 as *const libc::c_char,
            );
        }
    }
    loop {
        request = ftp_request(list_commands[i as usize], file);
        nwritten = fd_write(
            csock,
            request,
            strlen(request) as libc::c_int,
            -(1 as libc::c_int) as libc::c_double,
        );
        if nwritten < 0 as libc::c_int {
            rpl_free(request as *mut libc::c_void);
            request = 0 as *mut libc::c_char;
            return WRITEFAILED;
        }
        rpl_free(request as *mut libc::c_void);
        request = 0 as *mut libc::c_char;
        err = ftp_response(csock, &mut respline);
        if err as libc::c_uint == FTPOK as libc::c_int as libc::c_uint {
            if *respline as libc::c_int == '5' as i32 {
                err = FTPNSFOD;
            } else if *respline as libc::c_int == '1' as i32 {
                err = FTPOK;
                ok = 1 as libc::c_int != 0;
                *list_a_used = i == 0 as libc::c_int as libc::c_ulong;
            } else {
                err = FTPRERR;
            }
            rpl_free(respline as *mut libc::c_void);
            respline = 0 as *mut libc::c_char;
        }
        i = i.wrapping_add(1);
        i;
        if avoid_list as libc::c_int != 0 && i == 1 as libc::c_int as libc::c_ulong {
            i = i.wrapping_add(1);
            i;
            if opt.debug as libc::c_long != 0 {
                debug_logprintf(
                    b"(skipping \"LIST\")\0" as *const u8 as *const libc::c_char,
                );
            }
        }
        if !(i
            < (::std::mem::size_of::<[*const libc::c_char; 2]>() as libc::c_ulong)
                .wrapping_div(
                    ::std::mem::size_of::<*const libc::c_char>() as libc::c_ulong,
                ) && !ok)
        {
            break;
        }
    }
    return err;
}
pub unsafe extern "C" fn ftp_syst(
    mut csock: libc::c_int,
    mut server_type: *mut stype,
    mut unix_type: *mut ustype,
) -> uerr_t {
    let mut request: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut respline: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut nwritten: libc::c_int = 0;
    let mut err: uerr_t = NOCONERROR;
    let mut ftp_last_respline: *mut libc::c_char = 0 as *mut libc::c_char;
    request = ftp_request(
        b"SYST\0" as *const u8 as *const libc::c_char,
        0 as *const libc::c_char,
    );
    nwritten = fd_write(
        csock,
        request,
        strlen(request) as libc::c_int,
        -(1 as libc::c_int) as libc::c_double,
    );
    if nwritten < 0 as libc::c_int {
        rpl_free(request as *mut libc::c_void);
        request = 0 as *mut libc::c_char;
        return WRITEFAILED;
    }
    rpl_free(request as *mut libc::c_void);
    request = 0 as *mut libc::c_char;
    err = ftp_response(csock, &mut respline);
    if err as libc::c_uint != FTPOK as libc::c_int as libc::c_uint {
        return err;
    }
    if *respline as libc::c_int == '5' as i32 {
        rpl_free(respline as *mut libc::c_void);
        respline = 0 as *mut libc::c_char;
        return FTPSRVERR;
    }
    ftp_last_respline = strdup(respline);
    strtok(respline, b" \0" as *const u8 as *const libc::c_char);
    request = strtok(0 as *mut libc::c_char, b" \0" as *const u8 as *const libc::c_char);
    *unix_type = UST_OTHER;
    if request.is_null() {
        *server_type = ST_OTHER;
    } else if c_strcasecmp(request, b"VMS\0" as *const u8 as *const libc::c_char) == 0 {
        *server_type = ST_VMS;
    } else if c_strcasecmp(request, b"UNIX\0" as *const u8 as *const libc::c_char) == 0 {
        *server_type = ST_UNIX;
        if c_strncasecmp(
            ftp_last_respline,
            b"215 UNIX Type: L8\0" as *const u8 as *const libc::c_char,
            17 as libc::c_int as size_t,
        ) == 0
        {
            *unix_type = UST_TYPE_L8;
        } else if c_strncasecmp(
            ftp_last_respline,
            b"215 UNIX MultiNet Unix Emulation V5.3(93)\0" as *const u8
                as *const libc::c_char,
            41 as libc::c_int as size_t,
        ) == 0
        {
            *unix_type = UST_MULTINET;
        }
    } else if c_strcasecmp(request, b"WINDOWS_NT\0" as *const u8 as *const libc::c_char)
        == 0
        || c_strcasecmp(request, b"WINDOWS2000\0" as *const u8 as *const libc::c_char)
            == 0
    {
        *server_type = ST_WINNT;
    } else if c_strcasecmp(request, b"MACOS\0" as *const u8 as *const libc::c_char) == 0
    {
        *server_type = ST_MACOS;
    } else if c_strcasecmp(request, b"OS/400\0" as *const u8 as *const libc::c_char) == 0
    {
        *server_type = ST_OS400;
    } else {
        *server_type = ST_OTHER;
    }
    rpl_free(ftp_last_respline as *mut libc::c_void);
    ftp_last_respline = 0 as *mut libc::c_char;
    rpl_free(respline as *mut libc::c_void);
    respline = 0 as *mut libc::c_char;
    return FTPOK;
}
pub unsafe extern "C" fn ftp_pwd(
    mut csock: libc::c_int,
    mut pwd: *mut *mut libc::c_char,
) -> uerr_t {
    let mut request: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut respline: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut nwritten: libc::c_int = 0;
    let mut err: uerr_t = NOCONERROR;
    request = ftp_request(
        b"PWD\0" as *const u8 as *const libc::c_char,
        0 as *const libc::c_char,
    );
    nwritten = fd_write(
        csock,
        request,
        strlen(request) as libc::c_int,
        -(1 as libc::c_int) as libc::c_double,
    );
    if nwritten < 0 as libc::c_int {
        rpl_free(request as *mut libc::c_void);
        request = 0 as *mut libc::c_char;
        return WRITEFAILED;
    }
    rpl_free(request as *mut libc::c_void);
    request = 0 as *mut libc::c_char;
    err = ftp_response(csock, &mut respline);
    if err as libc::c_uint != FTPOK as libc::c_int as libc::c_uint {
        return err;
    }
    if !(*respline as libc::c_int == '5' as i32) {
        strtok(respline, b"\"\0" as *const u8 as *const libc::c_char);
        request = strtok(
            0 as *mut libc::c_char,
            b"\"\0" as *const u8 as *const libc::c_char,
        );
        if !request.is_null() {
            rpl_free(*pwd as *mut libc::c_void);
            *pwd = 0 as *mut libc::c_char;
            *pwd = xstrdup(request);
            rpl_free(respline as *mut libc::c_void);
            respline = 0 as *mut libc::c_char;
            return FTPOK;
        }
    }
    rpl_free(respline as *mut libc::c_void);
    respline = 0 as *mut libc::c_char;
    return FTPSRVERR;
}
pub unsafe extern "C" fn ftp_size(
    mut csock: libc::c_int,
    mut file: *const libc::c_char,
    mut size: *mut wgint,
) -> uerr_t {
    let mut request: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut respline: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut nwritten: libc::c_int = 0;
    let mut err: uerr_t = NOCONERROR;
    request = ftp_request(b"SIZE\0" as *const u8 as *const libc::c_char, file);
    nwritten = fd_write(
        csock,
        request,
        strlen(request) as libc::c_int,
        -(1 as libc::c_int) as libc::c_double,
    );
    if nwritten < 0 as libc::c_int {
        rpl_free(request as *mut libc::c_void);
        request = 0 as *mut libc::c_char;
        *size = 0 as libc::c_int as wgint;
        return WRITEFAILED;
    }
    rpl_free(request as *mut libc::c_void);
    request = 0 as *mut libc::c_char;
    err = ftp_response(csock, &mut respline);
    if err as libc::c_uint != FTPOK as libc::c_int as libc::c_uint {
        *size = 0 as libc::c_int as wgint;
        return err;
    }
    if *respline as libc::c_int == '5' as i32 {
        rpl_free(respline as *mut libc::c_void);
        respline = 0 as *mut libc::c_char;
        *size = 0 as libc::c_int as wgint;
        return FTPOK;
    }
    *__errno_location() = 0 as libc::c_int;
    *size = rpl_strtoll(
        respline.offset(4 as libc::c_int as isize),
        0 as *mut *mut libc::c_char,
        10 as libc::c_int,
    ) as wgint;
    if *__errno_location() != 0 {
        rpl_free(respline as *mut libc::c_void);
        respline = 0 as *mut libc::c_char;
        *size = 0 as libc::c_int as wgint;
        return FTPOK;
    }
    rpl_free(respline as *mut libc::c_void);
    respline = 0 as *mut libc::c_char;
    return FTPOK;
}
pub unsafe extern "C" fn ftp_process_type(
    mut params: *const libc::c_char,
) -> libc::c_char {
    if !params.is_null()
        && 0 as libc::c_int
            == strncasecmp(
                params,
                b"type=\0" as *const u8 as *const libc::c_char,
                5 as libc::c_int as libc::c_ulong,
            ) && *params.offset(5 as libc::c_int as isize) as libc::c_int != '\0' as i32
    {
        return c_toupper(*params.offset(5 as libc::c_int as isize) as libc::c_int)
            as libc::c_char
    } else {
        return 'I' as i32 as libc::c_char
    };
}
