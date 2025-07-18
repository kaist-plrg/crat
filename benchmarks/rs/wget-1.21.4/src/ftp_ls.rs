use ::libc;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    fn fclose(__stream: *mut FILE) -> libc::c_int;
    fn time(__timer: *mut time_t) -> time_t;
    fn strptime(
        __s: *const libc::c_char,
        __fmt: *const libc::c_char,
        __tp: *mut tm,
    ) -> *mut libc::c_char;
    fn localtime(__timer: *const time_t) -> *mut tm;
    fn rpl_mktime(__tp: *mut tm) -> time_t;
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
    fn strncat(
        _: *mut libc::c_char,
        _: *const libc::c_char,
        _: libc::c_ulong,
    ) -> *mut libc::c_char;
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    fn strncmp(
        _: *const libc::c_char,
        _: *const libc::c_char,
        _: libc::c_ulong,
    ) -> libc::c_int;
    fn strchr(_: *const libc::c_char, _: libc::c_int) -> *mut libc::c_char;
    fn strstr(_: *const libc::c_char, _: *const libc::c_char) -> *mut libc::c_char;
    fn strtok(_: *mut libc::c_char, _: *const libc::c_char) -> *mut libc::c_char;
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    fn strerror(_: libc::c_int) -> *mut libc::c_char;
    fn rpl_free(_: *mut libc::c_void);
    static mut opt: options;
    fn strtol(
        _: *const libc::c_char,
        _: *mut *mut libc::c_char,
        _: libc::c_int,
    ) -> libc::c_long;
    fn abort() -> !;
    fn getenv(__name: *const libc::c_char) -> *mut libc::c_char;
    fn rpl_strtoll(
        string: *const libc::c_char,
        endptr: *mut *mut libc::c_char,
        base: libc::c_int,
    ) -> libc::c_longlong;
    fn xmalloc(s: size_t) -> *mut libc::c_void;
    fn xstrdup(str: *const libc::c_char) -> *mut libc::c_char;
    fn fprintf(_: *mut FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn snprintf(
        _: *mut libc::c_char,
        _: libc::c_ulong,
        _: *const libc::c_char,
        _: ...
    ) -> libc::c_int;
    fn fgetc(__stream: *mut FILE) -> libc::c_int;
    fn putc(__c: libc::c_int, __stream: *mut FILE) -> libc::c_int;
    fn __getdelim(
        __lineptr: *mut *mut libc::c_char,
        __n: *mut size_t,
        __delimiter: libc::c_int,
        __stream: *mut FILE,
    ) -> __ssize_t;
    fn rewind(__stream: *mut FILE);
    fn rpl_fflush(gl_stream: *mut FILE) -> libc::c_int;
    fn rpl_fopen(filename: *const libc::c_char, mode: *const libc::c_char) -> *mut FILE;
    fn logprintf(_: log_options, _: *const libc::c_char, _: ...);
    fn debug_logprintf(_: *const libc::c_char, _: ...);
    fn __errno_location() -> *mut libc::c_int;
    fn concat_strings(_: *const libc::c_char, _: ...) -> *mut libc::c_char;
    fn number_to_static_string(_: wgint) -> *mut libc::c_char;
    static mut char_prop: [libc::c_uchar; 0];
    fn url_escape(_: *const libc::c_char) -> *mut libc::c_char;
    fn url_escape_unsafe_and_reserved(_: *const libc::c_char) -> *mut libc::c_char;
    fn html_quote_string(_: *const libc::c_char) -> *mut libc::c_char;
    static mut output_stream: *mut FILE;
    fn c_strcasecmp(s1: *const libc::c_char, s2: *const libc::c_char) -> libc::c_int;
    fn c_strncasecmp(
        s1: *const libc::c_char,
        s2: *const libc::c_char,
        n: size_t,
    ) -> libc::c_int;
}
pub type __int64_t = libc::c_long;
pub type __off_t = libc::c_long;
pub type __off64_t = libc::c_long;
pub type __time_t = libc::c_long;
pub type __ssize_t = libc::c_long;
pub type time_t = __time_t;
pub type size_t = libc::c_ulong;
pub type int64_t = __int64_t;
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
pub type stype = libc::c_uint;
pub const ST_OTHER: stype = 5;
pub const ST_OS400: stype = 4;
pub const ST_MACOS: stype = 3;
pub const ST_WINNT: stype = 2;
pub const ST_VMS: stype = 1;
pub const ST_UNIX: stype = 0;
pub type ftype = libc::c_uint;
pub const FT_UNKNOWN: ftype = 3;
pub const FT_SYMLINK: ftype = 2;
pub const FT_DIRECTORY: ftype = 1;
pub const FT_PLAINFILE: ftype = 0;
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
#[inline]
unsafe extern "C" fn c_isdigit(mut c: libc::c_int) -> bool {
    match c {
        48 | 49 | 50 | 51 | 52 | 53 | 54 | 55 | 56 | 57 => return 1 as libc::c_int != 0,
        _ => return 0 as libc::c_int != 0,
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
#[inline]
unsafe extern "C" fn getline(
    mut __lineptr: *mut *mut libc::c_char,
    mut __n: *mut size_t,
    mut __stream: *mut FILE,
) -> __ssize_t {
    return __getdelim(__lineptr, __n, '\n' as i32, __stream);
}
unsafe extern "C" fn symperms(mut s: *const libc::c_char) -> libc::c_int {
    let mut perms: libc::c_int = 0 as libc::c_int;
    let mut i: libc::c_int = 0;
    if strlen(s) < 9 as libc::c_int as libc::c_ulong {
        return 0 as libc::c_int;
    }
    i = 0 as libc::c_int;
    while i < 3 as libc::c_int {
        perms <<= 3 as libc::c_int;
        perms
            += (((*s.offset(0 as libc::c_int as isize) as libc::c_int == 'r' as i32)
                as libc::c_int) << 2 as libc::c_int)
                + (((*s.offset(1 as libc::c_int as isize) as libc::c_int == 'w' as i32)
                    as libc::c_int) << 1 as libc::c_int)
                + (*s.offset(2 as libc::c_int as isize) as libc::c_int == 'x' as i32
                    || *s.offset(2 as libc::c_int as isize) as libc::c_int == 's' as i32)
                    as libc::c_int;
        i += 1;
        i;
        s = s.offset(3 as libc::c_int as isize);
    }
    return perms;
}
unsafe extern "C" fn clean_line(
    mut line: *mut libc::c_char,
    mut len: libc::c_int,
) -> libc::c_int {
    if len <= 0 as libc::c_int {
        return 0 as libc::c_int;
    }
    while len > 0 as libc::c_int
        && (*line.offset((len - 1 as libc::c_int) as isize) as libc::c_int == '\n' as i32
            || *line.offset((len - 1 as libc::c_int) as isize) as libc::c_int
                == '\r' as i32)
    {
        len -= 1;
        *line.offset(len as isize) = '\0' as i32 as libc::c_char;
    }
    if len == 0 {
        return 0 as libc::c_int;
    }
    while *line != 0 {
        if *line as libc::c_int == '\t' as i32 {
            *line = ' ' as i32 as libc::c_char;
        }
        line = line.offset(1);
        line;
    }
    return len;
}
unsafe extern "C" fn ftp_parse_unix_ls(
    mut fp: *mut FILE,
    mut ignore_perms: libc::c_int,
) -> *mut fileinfo {
    static mut months: [*const libc::c_char; 12] = [
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
    let mut next: libc::c_int = 0;
    let mut len: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    let mut error: libc::c_int = 0;
    let mut ignore: libc::c_int = 0;
    let mut year: libc::c_int = 0;
    let mut month: libc::c_int = 0;
    let mut day: libc::c_int = 0;
    let mut hour: libc::c_int = 0;
    let mut min: libc::c_int = 0;
    let mut sec: libc::c_int = 0;
    let mut ptype: libc::c_int = 0;
    let mut timestruct: tm = tm {
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
    let mut tnow: *mut tm = 0 as *mut tm;
    let mut timenow: time_t = 0;
    let mut bufsize: size_t = 0 as libc::c_int as size_t;
    let mut line: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut tok: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut ptok: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut dir: *mut fileinfo = 0 as *mut fileinfo;
    let mut l: *mut fileinfo = 0 as *mut fileinfo;
    let mut cur: fileinfo = fileinfo {
        type_0: FT_PLAINFILE,
        name: 0 as *mut libc::c_char,
        size: 0,
        tstamp: 0,
        ptype: TT_HOUR_MIN,
        perms: 0,
        linkto: 0 as *mut libc::c_char,
        prev: 0 as *mut fileinfo,
        next: 0 as *mut fileinfo,
    };
    l = 0 as *mut fileinfo;
    dir = l;
    loop {
        len = getline(&mut line, &mut bufsize, fp) as libc::c_int;
        if !(len > 0 as libc::c_int) {
            break;
        }
        len = clean_line(line, len);
        if c_strncasecmp(
            line,
            b"total\0" as *const u8 as *const libc::c_char,
            5 as libc::c_int as size_t,
        ) == 0
        {
            continue;
        }
        tok = strtok(line, b" \0" as *const u8 as *const libc::c_char);
        if tok.is_null() {
            continue;
        }
        cur.name = 0 as *mut libc::c_char;
        cur.linkto = 0 as *mut libc::c_char;
        match *tok as libc::c_int {
            45 => {
                cur.type_0 = FT_PLAINFILE;
                if opt.debug as libc::c_long != 0 {
                    debug_logprintf(
                        b"PLAINFILE; \0" as *const u8 as *const libc::c_char,
                    );
                }
            }
            100 => {
                cur.type_0 = FT_DIRECTORY;
                if opt.debug as libc::c_long != 0 {
                    debug_logprintf(
                        b"DIRECTORY; \0" as *const u8 as *const libc::c_char,
                    );
                }
            }
            108 => {
                cur.type_0 = FT_SYMLINK;
                if opt.debug as libc::c_long != 0 {
                    debug_logprintf(b"SYMLINK; \0" as *const u8 as *const libc::c_char);
                }
            }
            _ => {
                cur.type_0 = FT_UNKNOWN;
                if opt.debug as libc::c_long != 0 {
                    debug_logprintf(b"UNKNOWN; \0" as *const u8 as *const libc::c_char);
                }
            }
        }
        if ignore_perms != 0 {
            match cur.type_0 as libc::c_uint {
                0 => {
                    cur.perms = 0o644 as libc::c_int;
                }
                1 => {
                    cur.perms = 0o755 as libc::c_int;
                }
                _ => {
                    cur.perms = 0o644 as libc::c_int;
                }
            }
            if opt.debug as libc::c_long != 0 {
                debug_logprintf(
                    b"implicit perms %0o; \0" as *const u8 as *const libc::c_char,
                    cur.perms as libc::c_uint,
                );
            }
        } else {
            cur.perms = symperms(tok.offset(1 as libc::c_int as isize));
            if opt.debug as libc::c_long != 0 {
                debug_logprintf(
                    b"perms %0o; \0" as *const u8 as *const libc::c_char,
                    cur.perms as libc::c_uint,
                );
            }
        }
        ignore = 0 as libc::c_int;
        error = ignore;
        sec = 0 as libc::c_int;
        min = sec;
        hour = min;
        year = hour;
        day = 0 as libc::c_int;
        month = day;
        ptype = TT_DAY as libc::c_int;
        next = -(1 as libc::c_int);
        tok = line;
        loop {
            ptok = tok;
            tok = strtok(
                0 as *mut libc::c_char,
                b" \0" as *const u8 as *const libc::c_char,
            );
            if !(tok != 0 as *mut libc::c_void as *mut libc::c_char) {
                break;
            }
            next -= 1;
            next;
            if next < 0 as libc::c_int {
                i = 0 as libc::c_int;
                while i < 12 as libc::c_int {
                    if c_strcasecmp(tok, months[i as usize]) == 0 {
                        break;
                    }
                    i += 1;
                    i;
                }
                if !(i != 12 as libc::c_int) {
                    continue;
                }
                let mut size: wgint = 0;
                if ptok == line {
                    error = 1 as libc::c_int;
                    break;
                } else {
                    *__errno_location() = 0 as libc::c_int;
                    size = rpl_strtoll(
                        ptok,
                        0 as *mut *mut libc::c_char,
                        10 as libc::c_int,
                    ) as wgint;
                    if size == 9223372036854775807 as libc::c_long
                        && *__errno_location() == 34 as libc::c_int
                    {
                        cur.size = 0 as libc::c_int as wgint;
                    } else {
                        cur.size = size;
                    }
                    if opt.debug as libc::c_long != 0 {
                        debug_logprintf(
                            b"size: %s; \0" as *const u8 as *const libc::c_char,
                            number_to_static_string(cur.size),
                        );
                    }
                    month = i;
                    next = 5 as libc::c_int;
                    if opt.debug as libc::c_long != 0 {
                        debug_logprintf(
                            b"month: %s; \0" as *const u8 as *const libc::c_char,
                            months[month as usize],
                        );
                    }
                }
            } else if next == 4 as libc::c_int {
                if *tok.offset(1 as libc::c_int as isize) != 0 {
                    day = 10 as libc::c_int * (*tok as libc::c_int - '0' as i32)
                        + *tok.offset(1 as libc::c_int as isize) as libc::c_int
                        - '0' as i32;
                } else {
                    day = *tok as libc::c_int - '0' as i32;
                }
                if opt.debug as libc::c_long != 0 {
                    debug_logprintf(
                        b"day: %d; \0" as *const u8 as *const libc::c_char,
                        day,
                    );
                }
            } else if next == 3 as libc::c_int {
                year = 0 as libc::c_int;
                sec = 0 as libc::c_int;
                hour = sec;
                min = hour;
                if c_isdigit(*tok as libc::c_int) {
                    while c_isdigit(*tok as libc::c_int) as libc::c_int != 0
                        && year <= 99999 as libc::c_int
                    {
                        year = *tok as libc::c_int - '0' as i32
                            + 10 as libc::c_int * year;
                        tok = tok.offset(1);
                        tok;
                    }
                    if *tok as libc::c_int == ':' as i32 {
                        let mut n: libc::c_int = 0;
                        hour = year;
                        year = 0 as libc::c_int;
                        ptype = TT_HOUR_MIN as libc::c_int;
                        tok = tok.offset(1);
                        tok;
                        n = 0 as libc::c_int;
                        while c_isdigit(*tok as libc::c_int) as libc::c_int != 0
                            && n < 2 as libc::c_int
                        {
                            min = *tok as libc::c_int - '0' as i32
                                + 10 as libc::c_int * min;
                            tok = tok.offset(1);
                            tok;
                            n += 1;
                            n;
                        }
                        if *tok as libc::c_int == ':' as i32 {
                            tok = tok.offset(1);
                            tok;
                            n = 0 as libc::c_int;
                            while c_isdigit(*tok as libc::c_int) as libc::c_int != 0
                                && n < 2 as libc::c_int
                            {
                                sec = *tok as libc::c_int - '0' as i32
                                    + 10 as libc::c_int * sec;
                                tok = tok.offset(1);
                                tok;
                                n += 1;
                                n;
                            }
                        }
                    }
                }
                if year != 0 {
                    if opt.debug as libc::c_long != 0 {
                        debug_logprintf(
                            b"year: %d (no tm); \0" as *const u8 as *const libc::c_char,
                            year,
                        );
                    }
                } else if opt.debug as libc::c_long != 0 {
                    debug_logprintf(
                        b"time: %02d:%02d:%02d (no yr); \0" as *const u8
                            as *const libc::c_char,
                        hour,
                        min,
                        sec,
                    );
                }
            } else if next == 2 as libc::c_int {
                let mut fnlen: libc::c_int = 0;
                let mut p: *mut libc::c_char = 0 as *mut libc::c_char;
                fnlen = strlen(tok) as libc::c_int;
                if (fnlen as libc::c_long)
                    < len as libc::c_long - tok.offset_from(line) as libc::c_long
                {
                    *tok.offset(fnlen as isize) = ' ' as i32 as libc::c_char;
                    if cur.type_0 as libc::c_uint
                        == FT_SYMLINK as libc::c_int as libc::c_uint
                    {
                        p = strstr(tok, b" -> \0" as *const u8 as *const libc::c_char);
                        if p.is_null() {
                            error = 1 as libc::c_int;
                            break;
                        } else {
                            cur.linkto = xstrdup(p.offset(4 as libc::c_int as isize));
                            if opt.debug as libc::c_long != 0 {
                                debug_logprintf(
                                    b"link to: %s\n\0" as *const u8 as *const libc::c_char,
                                    cur.linkto,
                                );
                            }
                            *p = '\0' as i32 as libc::c_char;
                        }
                    }
                }
                if strcmp(tok, b".\0" as *const u8 as *const libc::c_char) == 0
                    || strcmp(tok, b"..\0" as *const u8 as *const libc::c_char) == 0
                {
                    if opt.debug as libc::c_long != 0 {
                        debug_logprintf(
                            b"\nIgnoring `.' and `..'; \0" as *const u8
                                as *const libc::c_char,
                        );
                    }
                    ignore = 1 as libc::c_int;
                    break;
                } else {
                    fnlen = strlen(tok) as libc::c_int;
                    cur
                        .name = xmalloc((fnlen + 1 as libc::c_int) as size_t)
                        as *mut libc::c_char;
                    memcpy(
                        cur.name as *mut libc::c_void,
                        tok as *const libc::c_void,
                        (fnlen + 1 as libc::c_int) as libc::c_ulong,
                    );
                    if fnlen != 0 {
                        if cur.type_0 as libc::c_uint
                            == FT_DIRECTORY as libc::c_int as libc::c_uint
                            && *(cur.name).offset((fnlen - 1 as libc::c_int) as isize)
                                as libc::c_int == '/' as i32
                        {
                            *(cur.name)
                                .offset(
                                    (fnlen - 1 as libc::c_int) as isize,
                                ) = '\0' as i32 as libc::c_char;
                            if opt.debug as libc::c_long != 0 {
                                debug_logprintf(
                                    b"trailing `/' on dir.\n\0" as *const u8
                                        as *const libc::c_char,
                                );
                            }
                        } else if cur.type_0 as libc::c_uint
                            == FT_SYMLINK as libc::c_int as libc::c_uint
                            && *(cur.name).offset((fnlen - 1 as libc::c_int) as isize)
                                as libc::c_int == '@' as i32
                        {
                            *(cur.name)
                                .offset(
                                    (fnlen - 1 as libc::c_int) as isize,
                                ) = '\0' as i32 as libc::c_char;
                            if opt.debug as libc::c_long != 0 {
                                debug_logprintf(
                                    b"trailing `@' on link.\n\0" as *const u8
                                        as *const libc::c_char,
                                );
                            }
                        } else if cur.type_0 as libc::c_uint
                            == FT_PLAINFILE as libc::c_int as libc::c_uint
                            && cur.perms & 0o111 as libc::c_int != 0
                            && *(cur.name).offset((fnlen - 1 as libc::c_int) as isize)
                                as libc::c_int == '*' as i32
                        {
                            *(cur.name)
                                .offset(
                                    (fnlen - 1 as libc::c_int) as isize,
                                ) = '\0' as i32 as libc::c_char;
                            if opt.debug as libc::c_long != 0 {
                                debug_logprintf(
                                    b"trailing `*' on exec.\n\0" as *const u8
                                        as *const libc::c_char,
                                );
                            }
                        }
                    } else {
                        error = 1 as libc::c_int;
                    }
                    break;
                }
            } else {
                abort();
            }
        }
        if (cur.name).is_null()
            || cur.type_0 as libc::c_uint == FT_SYMLINK as libc::c_int as libc::c_uint
                && (cur.linkto).is_null()
        {
            error = 1 as libc::c_int;
        }
        if opt.debug as libc::c_long != 0 {
            debug_logprintf(
                b"%s\n\0" as *const u8 as *const libc::c_char,
                if !(cur.name).is_null() {
                    cur.name as *const libc::c_char
                } else {
                    b"\0" as *const u8 as *const libc::c_char
                },
            );
        }
        if error != 0 || ignore != 0 {
            if opt.debug as libc::c_long != 0 {
                debug_logprintf(b"Skipping.\n\0" as *const u8 as *const libc::c_char);
            }
            rpl_free(cur.name as *mut libc::c_void);
            cur.name = 0 as *mut libc::c_char;
            rpl_free(cur.linkto as *mut libc::c_void);
            cur.linkto = 0 as *mut libc::c_char;
        } else {
            if dir.is_null() {
                dir = xmalloc(::std::mem::size_of::<fileinfo>() as libc::c_ulong)
                    as *mut fileinfo;
                l = dir;
                memcpy(
                    l as *mut libc::c_void,
                    &mut cur as *mut fileinfo as *const libc::c_void,
                    ::std::mem::size_of::<fileinfo>() as libc::c_ulong,
                );
                (*l).next = 0 as *mut fileinfo;
                (*l).prev = (*l).next;
            } else {
                cur.prev = l;
                (*l)
                    .next = xmalloc(::std::mem::size_of::<fileinfo>() as libc::c_ulong)
                    as *mut fileinfo;
                l = (*l).next;
                memcpy(
                    l as *mut libc::c_void,
                    &mut cur as *mut fileinfo as *const libc::c_void,
                    ::std::mem::size_of::<fileinfo>() as libc::c_ulong,
                );
                (*l).next = 0 as *mut fileinfo;
            }
            timenow = time(0 as *mut time_t);
            tnow = localtime(&mut timenow);
            timestruct.tm_sec = sec;
            timestruct.tm_min = min;
            timestruct.tm_hour = hour;
            timestruct.tm_mday = day;
            timestruct.tm_mon = month;
            if year == 0 as libc::c_int {
                if month > (*tnow).tm_mon {
                    timestruct.tm_year = (*tnow).tm_year - 1 as libc::c_int;
                } else {
                    timestruct.tm_year = (*tnow).tm_year;
                }
            } else {
                timestruct.tm_year = year;
            }
            if timestruct.tm_year >= 1900 as libc::c_int {
                timestruct.tm_year -= 1900 as libc::c_int;
            }
            timestruct.tm_wday = 0 as libc::c_int;
            timestruct.tm_yday = 0 as libc::c_int;
            timestruct.tm_isdst = -(1 as libc::c_int);
            (*l).tstamp = rpl_mktime(&mut timestruct);
            (*l).ptype = ptype as parsetype;
        }
    }
    rpl_free(line as *mut libc::c_void);
    line = 0 as *mut libc::c_char;
    return dir;
}
unsafe extern "C" fn ftp_parse_winnt_ls(mut fp: *mut FILE) -> *mut fileinfo {
    let mut len: libc::c_int = 0;
    let mut year: libc::c_int = 0;
    let mut month: libc::c_int = 0;
    let mut day: libc::c_int = 0;
    let mut hour: libc::c_int = 0;
    let mut min: libc::c_int = 0;
    let mut bufsize: size_t = 0 as libc::c_int as size_t;
    let mut timestruct: tm = tm {
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
    let mut line: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut tok: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut filename: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut dir: *mut fileinfo = 0 as *mut fileinfo;
    let mut l: *mut fileinfo = 0 as *mut fileinfo;
    let mut cur: fileinfo = fileinfo {
        type_0: FT_PLAINFILE,
        name: 0 as *mut libc::c_char,
        size: 0,
        tstamp: 0,
        ptype: TT_HOUR_MIN,
        perms: 0,
        linkto: 0 as *mut libc::c_char,
        prev: 0 as *mut fileinfo,
        next: 0 as *mut fileinfo,
    };
    l = 0 as *mut fileinfo;
    dir = l;
    cur.name = 0 as *mut libc::c_char;
    loop {
        len = getline(&mut line, &mut bufsize, fp) as libc::c_int;
        if !(len > 0 as libc::c_int) {
            break;
        }
        len = clean_line(line, len);
        if len < 40 as libc::c_int {
            continue;
        }
        filename = line.offset(39 as libc::c_int as isize);
        tok = strtok(line, b"-\0" as *const u8 as *const libc::c_char);
        if tok.is_null() {
            continue;
        }
        month = atoi(tok);
        if month < 0 as libc::c_int {
            month = 0 as libc::c_int;
        } else {
            month -= 1;
            month;
        }
        tok = strtok(0 as *mut libc::c_char, b"-\0" as *const u8 as *const libc::c_char);
        if tok.is_null() {
            continue;
        }
        day = atoi(tok);
        tok = strtok(0 as *mut libc::c_char, b" \0" as *const u8 as *const libc::c_char);
        if tok.is_null() {
            continue;
        }
        year = atoi(tok);
        if year <= 70 as libc::c_int {
            year += 100 as libc::c_int;
        } else if year >= 1900 as libc::c_int {
            year -= 1900 as libc::c_int;
            if len < 42 as libc::c_int {
                continue;
            }
            filename = filename.offset(2 as libc::c_int as isize);
        }
        rpl_free(cur.name as *mut libc::c_void);
        cur.name = 0 as *mut libc::c_char;
        memset(
            &mut cur as *mut fileinfo as *mut libc::c_void,
            0 as libc::c_int,
            ::std::mem::size_of::<fileinfo>() as libc::c_ulong,
        );
        cur.name = xstrdup(filename);
        if opt.debug as libc::c_long != 0 {
            debug_logprintf(
                b"Name: '%s'\n\0" as *const u8 as *const libc::c_char,
                cur.name,
            );
        }
        tok = strtok(0 as *mut libc::c_char, b":\0" as *const u8 as *const libc::c_char);
        if tok.is_null() {
            continue;
        }
        hour = atoi(tok);
        tok = strtok(0 as *mut libc::c_char, b"M\0" as *const u8 as *const libc::c_char);
        if tok.is_null() {
            continue;
        }
        min = atoi(tok);
        if *tok.offset(0 as libc::c_int as isize) as libc::c_int != 0
            && *tok.offset(1 as libc::c_int as isize) as libc::c_int != 0
        {
            tok = tok.offset(2 as libc::c_int as isize);
        }
        if hour >= 12 as libc::c_int || hour < 0 as libc::c_int {
            hour = 0 as libc::c_int;
        }
        if *tok as libc::c_int == 'P' as i32 {
            hour += 12 as libc::c_int;
        }
        if opt.debug as libc::c_long != 0 {
            debug_logprintf(
                b"YYYY/MM/DD HH:MM - %d/%02d/%02d %02d:%02d\n\0" as *const u8
                    as *const libc::c_char,
                year + 1900 as libc::c_int,
                month,
                day,
                hour,
                min,
            );
        }
        timestruct.tm_sec = 0 as libc::c_int;
        timestruct.tm_min = min;
        timestruct.tm_hour = hour;
        timestruct.tm_mday = day;
        timestruct.tm_mon = month;
        timestruct.tm_year = year;
        timestruct.tm_wday = 0 as libc::c_int;
        timestruct.tm_yday = 0 as libc::c_int;
        timestruct.tm_isdst = -(1 as libc::c_int);
        cur.tstamp = rpl_mktime(&mut timestruct);
        cur.ptype = TT_HOUR_MIN;
        if opt.debug as libc::c_long != 0 {
            debug_logprintf(
                b"Timestamp: %ld\n\0" as *const u8 as *const libc::c_char,
                cur.tstamp,
            );
        }
        tok = strtok(0 as *mut libc::c_char, b" \0" as *const u8 as *const libc::c_char);
        if tok.is_null() {
            continue;
        }
        while !tok.is_null() && *tok as libc::c_int == '\0' as i32 {
            tok = strtok(
                0 as *mut libc::c_char,
                b" \0" as *const u8 as *const libc::c_char,
            );
        }
        if tok.is_null() {
            continue;
        }
        if *tok as libc::c_int == '<' as i32 {
            cur.type_0 = FT_DIRECTORY;
            cur.size = 0 as libc::c_int as wgint;
            cur.perms = 0o755 as libc::c_int;
            if opt.debug as libc::c_long != 0 {
                debug_logprintf(b"Directory\n\0" as *const u8 as *const libc::c_char);
            }
        } else {
            let mut size: wgint = 0;
            cur.type_0 = FT_PLAINFILE;
            *__errno_location() = 0 as libc::c_int;
            size = rpl_strtoll(tok, 0 as *mut *mut libc::c_char, 10 as libc::c_int)
                as wgint;
            if size == 9223372036854775807 as libc::c_long
                && *__errno_location() == 34 as libc::c_int
            {
                cur.size = 0 as libc::c_int as wgint;
            } else {
                cur.size = size;
            }
            cur.perms = 0o644 as libc::c_int;
            if opt.debug as libc::c_long != 0 {
                debug_logprintf(
                    b"File, size %s bytes\n\0" as *const u8 as *const libc::c_char,
                    number_to_static_string(cur.size),
                );
            }
        }
        cur.linkto = 0 as *mut libc::c_char;
        if dir.is_null() {
            dir = xmalloc(::std::mem::size_of::<fileinfo>() as libc::c_ulong)
                as *mut fileinfo;
            l = dir;
            memcpy(
                l as *mut libc::c_void,
                &mut cur as *mut fileinfo as *const libc::c_void,
                ::std::mem::size_of::<fileinfo>() as libc::c_ulong,
            );
            (*l).next = 0 as *mut fileinfo;
            (*l).prev = (*l).next;
        } else {
            cur.prev = l;
            (*l)
                .next = xmalloc(::std::mem::size_of::<fileinfo>() as libc::c_ulong)
                as *mut fileinfo;
            l = (*l).next;
            memcpy(
                l as *mut libc::c_void,
                &mut cur as *mut fileinfo as *const libc::c_void,
                ::std::mem::size_of::<fileinfo>() as libc::c_ulong,
            );
            (*l).next = 0 as *mut fileinfo;
        }
        cur.name = 0 as *mut libc::c_char;
    }
    rpl_free(cur.name as *mut libc::c_void);
    cur.name = 0 as *mut libc::c_char;
    rpl_free(line as *mut libc::c_void);
    line = 0 as *mut libc::c_char;
    return dir;
}
unsafe extern "C" fn eat_carets(mut str: *mut libc::c_char) {
    let mut strd: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut hdgt: libc::c_char = 0;
    let mut uchr: libc::c_uchar = 0;
    while *str as libc::c_int != '\0' as i32 && *str as libc::c_int != '^' as i32 {
        str = str.offset(1);
        str;
    }
    if *str as libc::c_int != '\0' as i32 {
        strd = str;
        while *str as libc::c_int != '\0' as i32 {
            uchr = *str as libc::c_uchar;
            if uchr as libc::c_int == '^' as i32 {
                if *char_prop
                    .as_mut_ptr()
                    .offset(
                        *str.offset(1 as libc::c_int as isize) as libc::c_uchar as isize,
                    ) as libc::c_int & 64 as libc::c_int != 0
                    && *char_prop
                        .as_mut_ptr()
                        .offset(
                            *str.offset(2 as libc::c_int as isize) as libc::c_uchar
                                as isize,
                        ) as libc::c_int & 64 as libc::c_int != 0
                {
                    str = str.offset(1);
                    uchr = *str as libc::c_uchar;
                    if uchr as libc::c_int <= '9' as i32 {
                        hdgt = (uchr as libc::c_int - '0' as i32) as libc::c_char;
                    } else {
                        hdgt = ((uchr as libc::c_int - 'A' as i32 & 7 as libc::c_int)
                            + 10 as libc::c_int) as libc::c_char;
                    }
                    hdgt = ((hdgt as libc::c_int) << 4 as libc::c_int) as libc::c_char;
                    str = str.offset(1);
                    uchr = *str as libc::c_uchar;
                    if uchr as libc::c_int <= '9' as i32 {
                        uchr = (hdgt as libc::c_int + uchr as libc::c_int - '0' as i32)
                            as libc::c_uchar;
                    } else {
                        uchr = (hdgt as libc::c_int
                            + (uchr as libc::c_int - 'A' as i32 & 15 as libc::c_int)
                            + 10 as libc::c_int) as libc::c_uchar;
                    }
                } else if uchr as libc::c_int == '_' as i32 {
                    uchr = ' ' as i32 as libc::c_uchar;
                } else if uchr as libc::c_int == '/' as i32 {
                    uchr = '?' as i32 as libc::c_uchar;
                }
            }
            *strd = uchr as libc::c_char;
            strd = strd.offset(1);
            strd;
            str = str.offset(1);
            str;
        }
        *strd = '\0' as i32 as libc::c_char;
    }
}
unsafe extern "C" fn ftp_parse_vms_ls(mut fp: *mut FILE) -> *mut fileinfo {
    let mut dt: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut len: libc::c_int = 0;
    let mut perms: libc::c_int = 0;
    let mut bufsize: size_t = 0 as libc::c_int as size_t;
    let mut timenow: time_t = 0;
    let mut timestruct: *mut tm = 0 as *mut tm;
    let mut date_str: [libc::c_char; 32] = [0; 32];
    let mut line: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut tok: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut dir: *mut fileinfo = 0 as *mut fileinfo;
    let mut l: *mut fileinfo = 0 as *mut fileinfo;
    let mut cur: fileinfo = fileinfo {
        type_0: FT_PLAINFILE,
        name: 0 as *mut libc::c_char,
        size: 0,
        tstamp: 0,
        ptype: TT_HOUR_MIN,
        perms: 0,
        linkto: 0 as *mut libc::c_char,
        prev: 0 as *mut fileinfo,
        next: 0 as *mut fileinfo,
    };
    l = 0 as *mut fileinfo;
    dir = l;
    j = 0 as libc::c_int;
    loop {
        i = getline(&mut line, &mut bufsize, fp) as libc::c_int;
        if !(i > 0 as libc::c_int) {
            break;
        }
        i = clean_line(line, i);
        if i <= 0 as libc::c_int {
            continue;
        }
        if j == 0 as libc::c_int
            && *line.offset((i - 1 as libc::c_int) as isize) as libc::c_int == ']' as i32
        {
            j = 1 as libc::c_int;
        } else {
            if !(strncmp(
                line,
                b"Total of \0" as *const u8 as *const libc::c_char,
                9 as libc::c_int as libc::c_ulong,
            ) == 0)
            {
                break;
            }
            i = 0 as libc::c_int;
            break;
        }
    }
    cur.name = 0 as *mut libc::c_char;
    while i > 0 as libc::c_int {
        let mut p: *mut libc::c_char = 0 as *mut libc::c_char;
        tok = strtok(line, b" \0" as *const u8 as *const libc::c_char);
        if tok.is_null() {
            tok = line;
        }
        if opt.debug as libc::c_long != 0 {
            debug_logprintf(
                b"file name:   '%s'\n\0" as *const u8 as *const libc::c_char,
                tok,
            );
        }
        p = tok.offset(strlen(tok) as isize);
        loop {
            p = p.offset(-1);
            if !(p > tok && c_isdigit(*p as libc::c_int) as libc::c_int != 0) {
                break;
            }
        }
        if p > tok && *p as libc::c_int == ';' as i32
            && *p.offset(-(1 as libc::c_int as isize)) as libc::c_int != '^' as i32
        {
            *p = '\0' as i32 as libc::c_char;
        }
        eat_carets(tok);
        if opt.debug as libc::c_long != 0 {
            debug_logprintf(
                b"file name-^: '%s'\n\0" as *const u8 as *const libc::c_char,
                tok,
            );
        }
        len = strlen(tok) as libc::c_int;
        if len >= 4 as libc::c_int
            && c_strncasecmp(
                tok.offset((len - 4 as libc::c_int) as isize),
                b".DIR\0" as *const u8 as *const libc::c_char,
                4 as libc::c_int as size_t,
            ) == 0
        {
            *tok.offset((len - 4 as libc::c_int) as isize) = '\0' as i32 as libc::c_char;
            cur.type_0 = FT_DIRECTORY;
            cur.perms = 0o755 as libc::c_int;
            if opt.debug as libc::c_long != 0 {
                debug_logprintf(
                    b"Directory (nv)\n\0" as *const u8 as *const libc::c_char,
                );
            }
        } else if len >= 6 as libc::c_int
            && c_strncasecmp(
                tok.offset(len as isize).offset(-(6 as libc::c_int as isize)),
                b".DIR;1\0" as *const u8 as *const libc::c_char,
                6 as libc::c_int as size_t,
            ) == 0
        {
            *tok.offset((len - 6 as libc::c_int) as isize) = '\0' as i32 as libc::c_char;
            cur.type_0 = FT_DIRECTORY;
            cur.perms = 0o755 as libc::c_int;
            if opt.debug as libc::c_long != 0 {
                debug_logprintf(
                    b"Directory (v)\n\0" as *const u8 as *const libc::c_char,
                );
            }
        } else {
            cur.type_0 = FT_PLAINFILE;
            cur.perms = 0o644 as libc::c_int;
            if opt.debug as libc::c_long != 0 {
                debug_logprintf(b"File\n\0" as *const u8 as *const libc::c_char);
            }
        }
        rpl_free(cur.name as *mut libc::c_void);
        cur.name = 0 as *mut libc::c_char;
        cur.name = xstrdup(tok);
        if opt.debug as libc::c_long != 0 {
            debug_logprintf(
                b"Name: '%s'\n\0" as *const u8 as *const libc::c_char,
                cur.name,
            );
        }
        *date_str.as_mut_ptr() = '\0' as i32 as libc::c_char;
        cur.linkto = 0 as *mut libc::c_char;
        cur.size = 0 as libc::c_int as wgint;
        tok = strtok(0 as *mut libc::c_char, b" \0" as *const u8 as *const libc::c_char);
        if tok.is_null() {
            if opt.debug as libc::c_long != 0 {
                debug_logprintf(
                    b"Getting additional line.\n\0" as *const u8 as *const libc::c_char,
                );
            }
            i = getline(&mut line, &mut bufsize, fp) as libc::c_int;
            if i <= 0 as libc::c_int {
                if opt.debug as libc::c_long != 0 {
                    debug_logprintf(
                        b"EOF.  Leaving listing parser.\n\0" as *const u8
                            as *const libc::c_char,
                    );
                }
                break;
            } else {
                i = clean_line(line, i);
                if i <= 0 as libc::c_int {
                    if opt.debug as libc::c_long != 0 {
                        debug_logprintf(
                            b"Blank line.  Leaving listing parser.\n\0" as *const u8
                                as *const libc::c_char,
                        );
                    }
                    break;
                } else if *line.offset(0 as libc::c_int as isize) as libc::c_int
                    != ' ' as i32
                {
                    if opt.debug as libc::c_long != 0 {
                        debug_logprintf(
                            b"Non-blank in column 1.  Must be a new file name?\n\0"
                                as *const u8 as *const libc::c_char,
                        );
                    }
                    continue;
                } else {
                    tok = strtok(line, b" \0" as *const u8 as *const libc::c_char);
                    if tok.is_null() {
                        if opt.debug as libc::c_long != 0 {
                            debug_logprintf(
                                b"Null token.  Leaving listing parser.\n\0" as *const u8
                                    as *const libc::c_char,
                            );
                        }
                        break;
                    }
                }
            }
        }
        while !tok.is_null() {
            if opt.debug as libc::c_long != 0 {
                debug_logprintf(
                    b"Token: >%s<: \0" as *const u8 as *const libc::c_char,
                    tok,
                );
            }
            if strlen(tok) < 12 as libc::c_int as libc::c_ulong
                && !(strchr(tok, '-' as i32)).is_null()
            {
                if opt.debug as libc::c_long != 0 {
                    debug_logprintf(b"Date.\n\0" as *const u8 as *const libc::c_char);
                }
                snprintf(
                    date_str.as_mut_ptr(),
                    ::std::mem::size_of::<[libc::c_char; 32]>() as libc::c_ulong,
                    b"%s \0" as *const u8 as *const libc::c_char,
                    tok,
                );
            } else if strlen(tok) < 12 as libc::c_int as libc::c_ulong
                && !(strchr(tok, ':' as i32)).is_null()
            {
                if opt.debug as libc::c_long != 0 {
                    debug_logprintf(b"Time. \0" as *const u8 as *const libc::c_char);
                }
                strncat(
                    date_str.as_mut_ptr(),
                    tok,
                    (::std::mem::size_of::<[libc::c_char; 32]>() as libc::c_ulong)
                        .wrapping_sub(strlen(date_str.as_mut_ptr()))
                        .wrapping_sub(1 as libc::c_int as libc::c_ulong),
                );
                if opt.debug as libc::c_long != 0 {
                    debug_logprintf(
                        b"Date time: >%s<\n\0" as *const u8 as *const libc::c_char,
                        date_str.as_mut_ptr(),
                    );
                }
            } else if !(strchr(tok, '[' as i32)).is_null() {
                if opt.debug as libc::c_long != 0 {
                    debug_logprintf(b"Owner.\n\0" as *const u8 as *const libc::c_char);
                }
            } else if !(strchr(tok, '(' as i32)).is_null() {
                perms = 0 as libc::c_int;
                j = 0 as libc::c_int;
                i = 0 as libc::c_int;
                while i < strlen(tok) as libc::c_int {
                    match *tok.offset(i as isize) as libc::c_int {
                        44 => {
                            if j == 0 as libc::c_int {
                                perms = 0 as libc::c_int;
                            } else if j < 4 as libc::c_int {
                                perms <<= 3 as libc::c_int;
                            }
                            j += 1;
                            j;
                        }
                        82 => {
                            perms |= 4 as libc::c_int;
                        }
                        87 => {
                            perms |= 2 as libc::c_int;
                        }
                        69 => {
                            perms |= 1 as libc::c_int;
                        }
                        68 => {
                            perms |= 2 as libc::c_int;
                        }
                        40 | 41 | _ => {}
                    }
                    i += 1;
                    i;
                }
                cur.perms = perms;
                if opt.debug as libc::c_long != 0 {
                    debug_logprintf(
                        b"Prot.  perms = %0o.\n\0" as *const u8 as *const libc::c_char,
                        cur.perms as libc::c_uint,
                    );
                }
            } else if opt.debug as libc::c_long != 0 {
                debug_logprintf(
                    b"Ignored (size?).\n\0" as *const u8 as *const libc::c_char,
                );
            }
            tok = strtok(
                0 as *mut libc::c_char,
                b" \0" as *const u8 as *const libc::c_char,
            );
        }
        timenow = time(0 as *mut time_t);
        timestruct = localtime(&mut timenow);
        strptime(
            date_str.as_mut_ptr(),
            b"%d-%b-%Y %H:%M:%S\0" as *const u8 as *const libc::c_char,
            timestruct,
        );
        timenow = rpl_mktime(timestruct);
        tok = getenv(
            b"WGET_TIMEZONE_DIFFERENTIAL\0" as *const u8 as *const libc::c_char,
        );
        if !tok.is_null() {
            dt = atoi(tok);
            if opt.debug as libc::c_long != 0 {
                debug_logprintf(
                    b"Time differential = %d.\n\0" as *const u8 as *const libc::c_char,
                    dt,
                );
            }
        } else {
            dt = 0 as libc::c_int;
        }
        if dt >= 0 as libc::c_int {
            timenow += dt as libc::c_long;
        } else {
            timenow -= -dt as libc::c_long;
        }
        cur.tstamp = timenow;
        if opt.debug as libc::c_long != 0 {
            debug_logprintf(
                b"Timestamp: %ld\n\0" as *const u8 as *const libc::c_char,
                cur.tstamp,
            );
        }
        cur.ptype = TT_HOUR_MIN;
        if dir.is_null() {
            dir = xmalloc(::std::mem::size_of::<fileinfo>() as libc::c_ulong)
                as *mut fileinfo;
            l = dir;
            cur.next = 0 as *mut fileinfo;
            cur.prev = cur.next;
            memcpy(
                l as *mut libc::c_void,
                &mut cur as *mut fileinfo as *const libc::c_void,
                ::std::mem::size_of::<fileinfo>() as libc::c_ulong,
            );
        } else {
            cur.prev = l;
            cur.next = 0 as *mut fileinfo;
            (*l)
                .next = xmalloc(::std::mem::size_of::<fileinfo>() as libc::c_ulong)
                as *mut fileinfo;
            l = (*l).next;
            memcpy(
                l as *mut libc::c_void,
                &mut cur as *mut fileinfo as *const libc::c_void,
                ::std::mem::size_of::<fileinfo>() as libc::c_ulong,
            );
        }
        cur.name = 0 as *mut libc::c_char;
        i = getline(&mut line, &mut bufsize, fp) as libc::c_int;
        if !(i > 0 as libc::c_int) {
            continue;
        }
        i = clean_line(line, i);
        if i <= 0 as libc::c_int {
            break;
        }
    }
    rpl_free(cur.name as *mut libc::c_void);
    cur.name = 0 as *mut libc::c_char;
    rpl_free(line as *mut libc::c_void);
    line = 0 as *mut libc::c_char;
    return dir;
}
pub unsafe extern "C" fn ftp_parse_ls(
    mut file: *const libc::c_char,
    system_type: stype,
) -> *mut fileinfo {
    let mut fp: *mut FILE = 0 as *mut FILE;
    let mut fi: *mut fileinfo = 0 as *mut fileinfo;
    fp = rpl_fopen(file, b"rb\0" as *const u8 as *const libc::c_char);
    if fp.is_null() {
        logprintf(
            LOG_NOTQUIET,
            b"%s: %s\n\0" as *const u8 as *const libc::c_char,
            file,
            strerror(*__errno_location()),
        );
        return 0 as *mut fileinfo;
    }
    fi = ftp_parse_ls_fp(fp, system_type);
    fclose(fp);
    return fi;
}
pub unsafe extern "C" fn ftp_parse_ls_fp(
    mut fp: *mut FILE,
    system_type: stype,
) -> *mut fileinfo {
    match system_type as libc::c_uint {
        0 => return ftp_parse_unix_ls(fp, 0 as libc::c_int),
        2 => {
            let mut c: libc::c_int = fgetc(fp);
            rewind(fp);
            if c >= '0' as i32 && c <= '9' as i32 {
                return ftp_parse_winnt_ls(fp)
            } else {
                return ftp_parse_unix_ls(fp, 1 as libc::c_int)
            }
        }
        1 => return ftp_parse_vms_ls(fp),
        3 => return ftp_parse_unix_ls(fp, 1 as libc::c_int),
        _ => {
            logprintf(
                LOG_NOTQUIET,
                dcgettext(
                    0 as *const libc::c_char,
                    b"Unsupported listing type, trying Unix listing parser.\n\0"
                        as *const u8 as *const libc::c_char,
                    5 as libc::c_int,
                ),
            );
            return ftp_parse_unix_ls(fp, 0 as libc::c_int);
        }
    };
}
pub unsafe extern "C" fn ftp_index(
    mut file: *const libc::c_char,
    mut u: *mut url,
    mut f: *mut fileinfo,
) -> uerr_t {
    let mut fp: *mut FILE = 0 as *mut FILE;
    let mut upwd: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut htcldir: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut htclfile: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut urlclfile: *mut libc::c_char = 0 as *mut libc::c_char;
    if output_stream.is_null() {
        fp = rpl_fopen(file, b"wb\0" as *const u8 as *const libc::c_char);
        if fp.is_null() {
            logprintf(
                LOG_NOTQUIET,
                b"%s: %s\n\0" as *const u8 as *const libc::c_char,
                file,
                strerror(*__errno_location()),
            );
            return FOPENERR;
        }
    } else {
        fp = output_stream;
    }
    if !((*u).user).is_null() {
        let mut tmpu: *mut libc::c_char = 0 as *mut libc::c_char;
        let mut tmpp: *mut libc::c_char = 0 as *mut libc::c_char;
        tmpu = url_escape((*u).user);
        tmpp = if !((*u).passwd).is_null() {
            url_escape((*u).passwd)
        } else {
            0 as *mut libc::c_char
        };
        if !tmpp.is_null() {
            upwd = concat_strings(
                tmpu,
                b":\0" as *const u8 as *const libc::c_char,
                tmpp,
                b"@\0" as *const u8 as *const libc::c_char,
                0 as *mut libc::c_char,
            );
        } else {
            upwd = concat_strings(
                tmpu,
                b"@\0" as *const u8 as *const libc::c_char,
                0 as *mut libc::c_char,
            );
        }
        rpl_free(tmpu as *mut libc::c_void);
        tmpu = 0 as *mut libc::c_char;
        rpl_free(tmpp as *mut libc::c_void);
        tmpp = 0 as *mut libc::c_char;
    } else {
        upwd = xstrdup(b"\0" as *const u8 as *const libc::c_char);
    }
    htcldir = html_quote_string((*u).dir);
    fprintf(
        fp,
        b"<!DOCTYPE HTML PUBLIC \"-//IETF//DTD HTML 2.0//EN\">\n\0" as *const u8
            as *const libc::c_char,
    );
    fprintf(fp, b"<html>\n<head>\n<title>\0" as *const u8 as *const libc::c_char);
    fprintf(
        fp,
        dcgettext(
            0 as *const libc::c_char,
            b"Index of /%s on %s:%d\0" as *const u8 as *const libc::c_char,
            5 as libc::c_int,
        ),
        htcldir,
        (*u).host,
        (*u).port,
    );
    fprintf(
        fp,
        b"</title>\n</head>\n<body>\n<h1>\0" as *const u8 as *const libc::c_char,
    );
    fprintf(
        fp,
        dcgettext(
            0 as *const libc::c_char,
            b"Index of /%s on %s:%d\0" as *const u8 as *const libc::c_char,
            5 as libc::c_int,
        ),
        htcldir,
        (*u).host,
        (*u).port,
    );
    fprintf(fp, b"</h1>\n<hr>\n<pre>\n\0" as *const u8 as *const libc::c_char);
    while !f.is_null() {
        fprintf(fp, b"  \0" as *const u8 as *const libc::c_char);
        if (*f).tstamp != -(1 as libc::c_int) as libc::c_long {
            static mut months: [*const libc::c_char; 12] = [
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
            let mut tstamp: time_t = (*f).tstamp;
            let mut ptm: *mut tm = localtime(&mut tstamp);
            fprintf(
                fp,
                b"%d %s %02d \0" as *const u8 as *const libc::c_char,
                (*ptm).tm_year + 1900 as libc::c_int,
                months[(*ptm).tm_mon as usize],
                (*ptm).tm_mday,
            );
            if (*f).ptype as libc::c_uint == TT_HOUR_MIN as libc::c_int as libc::c_uint {
                fprintf(
                    fp,
                    b"%02d:%02d  \0" as *const u8 as *const libc::c_char,
                    (*ptm).tm_hour,
                    (*ptm).tm_min,
                );
            } else {
                fprintf(fp, b"       \0" as *const u8 as *const libc::c_char);
            }
        } else {
            fprintf(
                fp,
                dcgettext(
                    0 as *const libc::c_char,
                    b"time unknown       \0" as *const u8 as *const libc::c_char,
                    5 as libc::c_int,
                ),
            );
        }
        match (*f).type_0 as libc::c_uint {
            0 => {
                fprintf(
                    fp,
                    dcgettext(
                        0 as *const libc::c_char,
                        b"File        \0" as *const u8 as *const libc::c_char,
                        5 as libc::c_int,
                    ),
                );
            }
            1 => {
                fprintf(
                    fp,
                    dcgettext(
                        0 as *const libc::c_char,
                        b"Directory   \0" as *const u8 as *const libc::c_char,
                        5 as libc::c_int,
                    ),
                );
            }
            2 => {
                fprintf(
                    fp,
                    dcgettext(
                        0 as *const libc::c_char,
                        b"Link        \0" as *const u8 as *const libc::c_char,
                        5 as libc::c_int,
                    ),
                );
            }
            _ => {
                fprintf(
                    fp,
                    dcgettext(
                        0 as *const libc::c_char,
                        b"Not sure    \0" as *const u8 as *const libc::c_char,
                        5 as libc::c_int,
                    ),
                );
            }
        }
        htclfile = html_quote_string((*f).name);
        urlclfile = url_escape_unsafe_and_reserved((*f).name);
        fprintf(
            fp,
            b"<a href=\"ftp://%s%s:%d\0" as *const u8 as *const libc::c_char,
            upwd,
            (*u).host,
            (*u).port,
        );
        if *(*u).dir as libc::c_int != '/' as i32 {
            putc('/' as i32, fp);
        }
        fprintf(fp, b"%s\0" as *const u8 as *const libc::c_char, htcldir);
        if *(*u).dir != 0 {
            putc('/' as i32, fp);
        }
        fprintf(fp, b"%s\0" as *const u8 as *const libc::c_char, urlclfile);
        if (*f).type_0 as libc::c_uint == FT_DIRECTORY as libc::c_int as libc::c_uint {
            putc('/' as i32, fp);
        }
        fprintf(fp, b"\">%s\0" as *const u8 as *const libc::c_char, htclfile);
        if (*f).type_0 as libc::c_uint == FT_DIRECTORY as libc::c_int as libc::c_uint {
            putc('/' as i32, fp);
        }
        fprintf(fp, b"</a> \0" as *const u8 as *const libc::c_char);
        if (*f).type_0 as libc::c_uint == FT_PLAINFILE as libc::c_int as libc::c_uint {
            fprintf(
                fp,
                dcgettext(
                    0 as *const libc::c_char,
                    b" (%s bytes)\0" as *const u8 as *const libc::c_char,
                    5 as libc::c_int,
                ),
                number_to_static_string((*f).size),
            );
        } else if (*f).type_0 as libc::c_uint
            == FT_SYMLINK as libc::c_int as libc::c_uint
        {
            fprintf(
                fp,
                b"-> %s\0" as *const u8 as *const libc::c_char,
                if !((*f).linkto).is_null() {
                    (*f).linkto as *const libc::c_char
                } else {
                    b"(nil)\0" as *const u8 as *const libc::c_char
                },
            );
        }
        putc('\n' as i32, fp);
        rpl_free(htclfile as *mut libc::c_void);
        htclfile = 0 as *mut libc::c_char;
        rpl_free(urlclfile as *mut libc::c_void);
        urlclfile = 0 as *mut libc::c_char;
        f = (*f).next;
    }
    fprintf(fp, b"</pre>\n</body>\n</html>\n\0" as *const u8 as *const libc::c_char);
    rpl_free(htcldir as *mut libc::c_void);
    htcldir = 0 as *mut libc::c_char;
    rpl_free(upwd as *mut libc::c_void);
    upwd = 0 as *mut libc::c_char;
    if output_stream.is_null() {
        fclose(fp);
    } else {
        rpl_fflush(fp);
    }
    return FTPOK;
}
