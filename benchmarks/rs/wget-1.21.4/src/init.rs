use ::libc;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    static mut stderr: *mut FILE;
    fn fclose(__stream: *mut FILE) -> libc::c_int;
    fn fprintf(_: *mut FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn snprintf(
        _: *mut libc::c_char,
        _: libc::c_ulong,
        _: *const libc::c_char,
        _: ...
    ) -> libc::c_int;
    fn __getdelim(
        __lineptr: *mut *mut libc::c_char,
        __n: *mut size_t,
        __delimiter: libc::c_int,
        __stream: *mut FILE,
    ) -> __ssize_t;
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
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    fn strchr(_: *const libc::c_char, _: libc::c_int) -> *mut libc::c_char;
    fn strspn(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_ulong;
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    fn strerror(_: libc::c_int) -> *mut libc::c_char;
    fn rpl_free(_: *mut libc::c_void);
    static mut opt: options;
    fn abort() -> !;
    fn exit(_: libc::c_int) -> !;
    fn getenv(__name: *const libc::c_char) -> *mut libc::c_char;
    fn rpl_strtol(
        string: *const libc::c_char,
        endptr: *mut *mut libc::c_char,
        base: libc::c_int,
    ) -> libc::c_long;
    fn xmalloc(s: size_t) -> *mut libc::c_void;
    fn quote(arg: *const libc::c_char) -> *const libc::c_char;
    fn debug_logprintf(_: *const libc::c_char, _: ...);
    fn log_close();
    static mut exec_name: *const libc::c_char;
    fn xstrdup(str: *const libc::c_char) -> *mut libc::c_char;
    fn inform_exit_status(err: uerr_t);
    fn getuid() -> __uid_t;
    fn __errno_location() -> *mut libc::c_int;
    fn getpwuid(__uid: __uid_t) -> *mut passwd;
    fn strdupdelim(_: *const libc::c_char, _: *const libc::c_char) -> *mut libc::c_char;
    fn sepstring(_: *const libc::c_char) -> *mut *mut libc::c_char;
    fn aprintf(_: *const libc::c_char, _: ...) -> *mut libc::c_char;
    fn concat_strings(_: *const libc::c_char, _: ...) -> *mut libc::c_char;
    fn file_exists_p(_: *const libc::c_char, _: *mut file_stats_t) -> bool;
    fn fopen_stat(
        _: *const libc::c_char,
        _: *const libc::c_char,
        _: *mut file_stats_t,
    ) -> *mut FILE;
    fn free_vec(_: *mut *mut libc::c_char);
    fn merge_vecs(
        _: *mut *mut libc::c_char,
        _: *mut *mut libc::c_char,
    ) -> *mut *mut libc::c_char;
    fn vec_append(
        _: *mut *mut libc::c_char,
        _: *const libc::c_char,
    ) -> *mut *mut libc::c_char;
    fn valid_progress_implementation_p(_: *const libc::c_char) -> bool;
    static mut output_stream: *mut FILE;
    fn warc_close();
    fn c_strcasecmp(s1: *const libc::c_char, s2: *const libc::c_char) -> libc::c_int;
    static mut cleaned_up: libc::c_int;
}
pub type __int64_t = libc::c_long;
pub type __dev_t = libc::c_ulong;
pub type __uid_t = libc::c_uint;
pub type __gid_t = libc::c_uint;
pub type __ino_t = libc::c_ulong;
pub type __off_t = libc::c_long;
pub type __off64_t = libc::c_long;
pub type __ssize_t = libc::c_long;
pub type ino_t = __ino_t;
pub type dev_t = __dev_t;
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
pub struct passwd {
    pub pw_name: *mut libc::c_char,
    pub pw_passwd: *mut libc::c_char,
    pub pw_uid: __uid_t,
    pub pw_gid: __gid_t,
    pub pw_gecos: *mut libc::c_char,
    pub pw_dir: *mut libc::c_char,
    pub pw_shell: *mut libc::c_char,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct file_stat_s {
    pub access_err: libc::c_int,
    pub st_ino: ino_t,
    pub st_dev: dev_t,
}
pub type file_stats_t = file_stat_s;
pub const line_empty: parse_line = 1;
pub const line_unknown_command: parse_line = 3;
pub const line_syntax_error: parse_line = 2;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_4 {
    pub name: *const libc::c_char,
    pub place: *mut libc::c_void,
    pub action: Option::<
        unsafe extern "C" fn(
            *const libc::c_char,
            *const libc::c_char,
            *mut libc::c_void,
        ) -> bool,
    >,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct decode_item {
    pub name: *const libc::c_char,
    pub code: libc::c_int,
}
pub const line_ok: parse_line = 0;
pub type parse_line = libc::c_uint;
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
unsafe extern "C" fn getline(
    mut __lineptr: *mut *mut libc::c_char,
    mut __n: *mut size_t,
    mut __stream: *mut FILE,
) -> __ssize_t {
    return __getdelim(__lineptr, __n, '\n' as i32, __stream);
}
static mut commands: [C2RustUnnamed_4; 148] = [C2RustUnnamed_4 {
    name: 0 as *const libc::c_char,
    place: 0 as *mut libc::c_void,
    action: None,
}; 148];
unsafe extern "C" fn command_by_name(mut cmdname: *const libc::c_char) -> libc::c_int {
    let mut lo: libc::c_int = 0 as libc::c_int;
    let mut hi: libc::c_int = (::std::mem::size_of::<[C2RustUnnamed_4; 148]>()
        as libc::c_ulong)
        .wrapping_div(::std::mem::size_of::<C2RustUnnamed_4>() as libc::c_ulong)
        .wrapping_sub(1 as libc::c_int as libc::c_ulong) as libc::c_int;
    while lo <= hi {
        let mut mid: libc::c_int = lo + hi >> 1 as libc::c_int;
        let mut cmp: libc::c_int = c_strcasecmp(cmdname, commands[mid as usize].name);
        if cmp < 0 as libc::c_int {
            hi = mid - 1 as libc::c_int;
        } else if cmp > 0 as libc::c_int {
            lo = mid + 1 as libc::c_int;
        } else {
            return mid
        }
    }
    return -(1 as libc::c_int);
}
pub unsafe extern "C" fn defaults() {
    let mut tmp: *mut libc::c_char = 0 as *mut libc::c_char;
    memset(
        &mut opt as *mut options as *mut libc::c_void,
        '\0' as i32,
        ::std::mem::size_of::<options>() as libc::c_ulong,
    );
    opt.cookies = 1 as libc::c_int != 0;
    opt.verbose = -(1 as libc::c_int);
    opt.ntry = 20 as libc::c_int;
    opt.reclevel = 5 as libc::c_int;
    opt.add_hostdir = 1 as libc::c_int != 0;
    opt.netrc = 1 as libc::c_int != 0;
    opt.ftp_glob = 1 as libc::c_int != 0;
    opt.htmlify = 1 as libc::c_int != 0;
    opt.http_keep_alive = 1 as libc::c_int != 0;
    opt.use_proxy = 1 as libc::c_int != 0;
    opt.convert_file_only = 0 as libc::c_int != 0;
    tmp = getenv(b"no_proxy\0" as *const u8 as *const libc::c_char);
    if !tmp.is_null() {
        opt.no_proxy = sepstring(tmp);
    }
    opt.prefer_family = prefer_none;
    opt.allow_cache = 1 as libc::c_int != 0;
    opt.if_modified_since = 1 as libc::c_int != 0;
    opt.read_timeout = 900 as libc::c_int as libc::c_double;
    opt.use_robots = 1 as libc::c_int != 0;
    opt.remove_listing = 1 as libc::c_int != 0;
    opt.dot_bytes = 1024 as libc::c_int as wgint;
    opt.dot_spacing = 10 as libc::c_int;
    opt.dots_in_line = 50 as libc::c_int;
    opt.dns_cache = 1 as libc::c_int != 0;
    opt.ftp_pasv = 1 as libc::c_int != 0;
    opt.retr_symlinks = 1 as libc::c_int != 0;
    opt.compression = compression_none;
    opt.restrict_files_os = restrict_unix;
    opt.restrict_files_ctrl = 1 as libc::c_int != 0;
    opt.restrict_files_nonascii = 0 as libc::c_int != 0;
    opt.restrict_files_case = restrict_no_case_restriction;
    opt.regex_type = regex_type_posix;
    opt.max_redirect = 20 as libc::c_int;
    opt.waitretry = 10 as libc::c_int as libc::c_double;
    opt.enable_iri = 1 as libc::c_int != 0;
    opt.locale = 0 as *const libc::c_char;
    opt.encoding_remote = 0 as *mut libc::c_char;
    opt.useservertimestamps = 1 as libc::c_int != 0;
    opt.show_all_dns_entries = 0 as libc::c_int != 0;
    opt.warc_maxsize = 0 as libc::c_int as wgint;
    opt.warc_compression_enabled = 1 as libc::c_int != 0;
    opt.warc_digests_enabled = 1 as libc::c_int != 0;
    opt.warc_cdx_enabled = 0 as libc::c_int != 0;
    opt.warc_cdx_dedup_filename = 0 as *mut libc::c_char;
    opt.warc_tempdir = 0 as *mut libc::c_char;
    opt.warc_keep_log = 1 as libc::c_int != 0;
    opt.start_pos = -(1 as libc::c_int) as wgint;
    opt.show_progress = -(1 as libc::c_int);
    opt.noscroll = 0 as libc::c_int != 0;
    opt.enable_xattr = 0 as libc::c_int != 0;
}
pub unsafe extern "C" fn home_dir() -> *mut libc::c_char {
    static mut buf: *mut libc::c_char = 0 as *const libc::c_char as *mut libc::c_char;
    static mut home: *mut libc::c_char = 0 as *const libc::c_char as *mut libc::c_char;
    static mut ret: *mut libc::c_char = 0 as *const libc::c_char as *mut libc::c_char;
    if home.is_null() {
        home = getenv(b"HOME\0" as *const u8 as *const libc::c_char);
        if home.is_null() {
            let mut pwd: *mut passwd = getpwuid(getuid());
            if pwd.is_null() || ((*pwd).pw_dir).is_null() {
                return 0 as *mut libc::c_char;
            }
            home = (*pwd).pw_dir;
        }
    }
    ret = if !home.is_null() { xstrdup(home) } else { 0 as *mut libc::c_char };
    rpl_free(buf as *mut libc::c_void);
    buf = 0 as *mut libc::c_char;
    return ret;
}
pub unsafe extern "C" fn wgetrc_env_file_name() -> *mut libc::c_char {
    let mut env: *mut libc::c_char = getenv(
        b"WGETRC\0" as *const u8 as *const libc::c_char,
    );
    if !env.is_null() && *env as libc::c_int != 0 {
        let mut flstat: file_stats_t = file_stats_t {
            access_err: 0,
            st_ino: 0,
            st_dev: 0,
        };
        if !file_exists_p(env, &mut flstat) {
            fprintf(
                stderr,
                dcgettext(
                    0 as *const libc::c_char,
                    b"%s: WGETRC points to %s, which couldn't be accessed because of error: %s.\n\0"
                        as *const u8 as *const libc::c_char,
                    5 as libc::c_int,
                ),
                exec_name,
                env,
                strerror(flstat.access_err),
            );
            exit(WGET_EXIT_GENERIC_ERROR as libc::c_int);
        }
        return xstrdup(env);
    }
    return 0 as *mut libc::c_char;
}
pub unsafe extern "C" fn ajoin_dir_file(
    mut dir: *const libc::c_char,
    mut file: *const libc::c_char,
) -> *mut libc::c_char {
    let mut dir_file: *mut libc::c_char = 0 as *mut libc::c_char;
    dir_file = aprintf(b"%s/%s\0" as *const u8 as *const libc::c_char, dir, file);
    return dir_file;
}
pub unsafe extern "C" fn wgetrc_user_file_name() -> *mut libc::c_char {
    let mut file: *mut libc::c_char = 0 as *mut libc::c_char;
    if !(opt.homedir).is_null() {
        file = ajoin_dir_file(
            opt.homedir,
            b".wgetrc\0" as *const u8 as *const libc::c_char,
        );
    }
    if file.is_null() {
        return 0 as *mut libc::c_char;
    }
    if !file_exists_p(file, 0 as *mut file_stats_t) {
        rpl_free(file as *mut libc::c_void);
        file = 0 as *mut libc::c_char;
        return 0 as *mut libc::c_char;
    }
    return file;
}
pub unsafe extern "C" fn wgetrc_file_name() -> *mut libc::c_char {
    let mut file: *mut libc::c_char = wgetrc_env_file_name();
    if !file.is_null() && *file as libc::c_int != 0 {
        return file;
    }
    file = wgetrc_user_file_name();
    return file;
}
pub unsafe extern "C" fn run_wgetrc(
    mut file: *const libc::c_char,
    mut flstats: *mut file_stats_t,
) -> bool {
    let mut fp: *mut FILE = 0 as *mut FILE;
    let mut line: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut bufsize: size_t = 0 as libc::c_int as size_t;
    let mut ln: libc::c_int = 0;
    let mut errcnt: libc::c_int = 0 as libc::c_int;
    fp = fopen_stat(file, b"r\0" as *const u8 as *const libc::c_char, flstats);
    if fp.is_null() {
        fprintf(
            stderr,
            dcgettext(
                0 as *const libc::c_char,
                b"%s: Cannot read %s (%s).\n\0" as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ),
            exec_name,
            file,
            strerror(*__errno_location()),
        );
        return 1 as libc::c_int != 0;
    }
    ln = 1 as libc::c_int;
    while getline(&mut line, &mut bufsize, fp) > 0 as libc::c_int as libc::c_long {
        let mut com: *mut libc::c_char = 0 as *mut libc::c_char;
        let mut val: *mut libc::c_char = 0 as *mut libc::c_char;
        let mut comind: libc::c_int = 0;
        match parse_line(line, &mut com, &mut val, &mut comind) as libc::c_uint {
            0 => {
                if !setval_internal_tilde(comind, com, val) {
                    fprintf(
                        stderr,
                        dcgettext(
                            0 as *const libc::c_char,
                            b"%s: Error in %s at line %d.\n\0" as *const u8
                                as *const libc::c_char,
                            5 as libc::c_int,
                        ),
                        exec_name,
                        file,
                        ln,
                    );
                    errcnt += 1;
                    errcnt;
                }
            }
            2 => {
                fprintf(
                    stderr,
                    dcgettext(
                        0 as *const libc::c_char,
                        b"%s: Syntax error in %s at line %d.\n\0" as *const u8
                            as *const libc::c_char,
                        5 as libc::c_int,
                    ),
                    exec_name,
                    file,
                    ln,
                );
                errcnt += 1;
                errcnt;
            }
            3 => {
                fprintf(
                    stderr,
                    dcgettext(
                        0 as *const libc::c_char,
                        b"%s: Unknown command %s in %s at line %d.\n\0" as *const u8
                            as *const libc::c_char,
                        5 as libc::c_int,
                    ),
                    exec_name,
                    quote(com),
                    file,
                    ln,
                );
                errcnt += 1;
                errcnt;
            }
            1 => {}
            _ => {
                abort();
            }
        }
        rpl_free(com as *mut libc::c_void);
        com = 0 as *mut libc::c_char;
        rpl_free(val as *mut libc::c_void);
        val = 0 as *mut libc::c_char;
        ln += 1;
        ln;
    }
    rpl_free(line as *mut libc::c_void);
    line = 0 as *mut libc::c_char;
    fclose(fp);
    return errcnt == 0 as libc::c_int;
}
pub unsafe extern "C" fn initialize() -> libc::c_int {
    let mut env_sysrc: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut flstats: file_stats_t = file_stats_t {
        access_err: 0,
        st_ino: 0,
        st_dev: 0,
    };
    let mut ok: bool = 1 as libc::c_int != 0;
    memset(
        &mut flstats as *mut file_stats_t as *mut libc::c_void,
        0 as libc::c_int,
        ::std::mem::size_of::<file_stats_t>() as libc::c_ulong,
    );
    env_sysrc = getenv(b"SYSTEM_WGETRC\0" as *const u8 as *const libc::c_char);
    if !env_sysrc.is_null() && file_exists_p(env_sysrc, &mut flstats) as libc::c_int != 0
    {
        ok = (ok as libc::c_int & run_wgetrc(env_sysrc, &mut flstats) as libc::c_int) != 0;
        if !ok {
            fprintf(
                stderr,
                dcgettext(
                    0 as *const libc::c_char,
                    b"Parsing system wgetrc file (env SYSTEM_WGETRC) failed.  Please check\n'%s',\nor specify a different file using --config.\n\0"
                        as *const u8 as *const libc::c_char,
                    5 as libc::c_int,
                ),
                env_sysrc,
            );
            return WGET_EXIT_PARSE_ERROR as libc::c_int;
        }
    } else if file_exists_p(
        b"/usr/local/etc/wgetrc\0" as *const u8 as *const libc::c_char,
        &mut flstats,
    ) {
        ok = (ok as libc::c_int
            & run_wgetrc(
                b"/usr/local/etc/wgetrc\0" as *const u8 as *const libc::c_char,
                &mut flstats,
            ) as libc::c_int) != 0;
    }
    if !ok {
        fprintf(
            stderr,
            dcgettext(
                0 as *const libc::c_char,
                b"Parsing system wgetrc file failed.  Please check\n'%s',\nor specify a different file using --config.\n\0"
                    as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ),
            b"/usr/local/etc/wgetrc\0" as *const u8 as *const libc::c_char,
        );
        return WGET_EXIT_PARSE_ERROR as libc::c_int;
    }
    opt.wgetrcfile = wgetrc_file_name();
    if (opt.wgetrcfile).is_null() {
        return 0 as libc::c_int;
    }
    if strcmp(
        opt.wgetrcfile,
        b"/usr/local/etc/wgetrc\0" as *const u8 as *const libc::c_char,
    ) == 0
    {
        fprintf(
            stderr,
            dcgettext(
                0 as *const libc::c_char,
                b"%s: Warning: Both system and user wgetrc point to %s.\n\0" as *const u8
                    as *const libc::c_char,
                5 as libc::c_int,
            ),
            exec_name,
            quote(opt.wgetrcfile),
        );
    } else if file_exists_p(opt.wgetrcfile, &mut flstats) {
        ok = (ok as libc::c_int
            & run_wgetrc(opt.wgetrcfile, &mut flstats) as libc::c_int) != 0;
    }
    rpl_free(opt.wgetrcfile as *mut libc::c_void);
    opt.wgetrcfile = 0 as *const libc::c_char;
    if !ok {
        return WGET_EXIT_PARSE_ERROR as libc::c_int;
    }
    return 0 as libc::c_int;
}
unsafe extern "C" fn dehyphen(mut s: *mut libc::c_char) {
    let mut t: *mut libc::c_char = s;
    let mut h: *mut libc::c_char = s;
    while *h != 0 {
        if *h as libc::c_int == '_' as i32 || *h as libc::c_int == '-' as i32 {
            h = h.offset(1);
            h;
        } else {
            let fresh0 = h;
            h = h.offset(1);
            let fresh1 = t;
            t = t.offset(1);
            *fresh1 = *fresh0;
        }
    }
    *t = '\0' as i32 as libc::c_char;
}
unsafe extern "C" fn parse_line(
    mut line: *const libc::c_char,
    mut com: *mut *mut libc::c_char,
    mut val: *mut *mut libc::c_char,
    mut comind: *mut libc::c_int,
) -> parse_line {
    let mut p: *const libc::c_char = 0 as *const libc::c_char;
    let mut end: *const libc::c_char = line.offset(strlen(line) as isize);
    let mut cmdstart: *const libc::c_char = 0 as *const libc::c_char;
    let mut cmdend: *const libc::c_char = 0 as *const libc::c_char;
    let mut valstart: *const libc::c_char = 0 as *const libc::c_char;
    let mut valend: *const libc::c_char = 0 as *const libc::c_char;
    let mut buf: [libc::c_char; 1024] = [0; 1024];
    let mut len: size_t = 0;
    let mut cmdcopy: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut ind: libc::c_int = 0;
    while *line as libc::c_int != 0
        && c_isspace(*line as libc::c_int) as libc::c_int != 0
    {
        line = line.offset(1);
        line;
    }
    while end > line
        && c_isspace(*end.offset(-(1 as libc::c_int) as isize) as libc::c_int)
            as libc::c_int != 0
    {
        end = end.offset(-1);
        end;
    }
    if *line == 0 || *line as libc::c_int == '#' as i32 {
        return line_empty;
    }
    p = line;
    cmdstart = p;
    while p < end
        && (c_isalnum(*p as libc::c_int) as libc::c_int != 0
            || *p as libc::c_int == '_' as i32 || *p as libc::c_int == '-' as i32)
    {
        p = p.offset(1);
        p;
    }
    cmdend = p;
    while p < end && c_isspace(*p as libc::c_int) as libc::c_int != 0 {
        p = p.offset(1);
        p;
    }
    if p == end || *p as libc::c_int != '=' as i32 {
        return line_syntax_error;
    }
    p = p.offset(1);
    p;
    while p < end && c_isspace(*p as libc::c_int) as libc::c_int != 0 {
        p = p.offset(1);
        p;
    }
    valstart = p;
    valend = end;
    *com = strdupdelim(cmdstart, cmdend);
    *val = strdupdelim(valstart, valend);
    len = cmdend.offset_from(cmdstart) as libc::c_long as size_t;
    if len < ::std::mem::size_of::<[libc::c_char; 1024]>() as libc::c_ulong {
        cmdcopy = buf.as_mut_ptr();
    } else {
        cmdcopy = xmalloc(len.wrapping_add(1 as libc::c_int as libc::c_ulong))
            as *mut libc::c_char;
    }
    memcpy(cmdcopy as *mut libc::c_void, cmdstart as *const libc::c_void, len);
    *cmdcopy.offset(len as isize) = 0 as libc::c_int as libc::c_char;
    dehyphen(cmdcopy);
    ind = command_by_name(cmdcopy);
    if cmdcopy != buf.as_mut_ptr() {
        rpl_free(cmdcopy as *mut libc::c_void);
        cmdcopy = 0 as *mut libc::c_char;
    }
    if ind == -(1 as libc::c_int) {
        return line_unknown_command;
    }
    *comind = ind;
    return line_ok;
}
unsafe extern "C" fn setval_internal(
    mut comind: libc::c_int,
    mut com: *const libc::c_char,
    mut val: *const libc::c_char,
) -> bool {
    if comind as libc::c_uint as libc::c_ulong
        >= (::std::mem::size_of::<[C2RustUnnamed_4; 148]>() as libc::c_ulong)
            .wrapping_div(::std::mem::size_of::<C2RustUnnamed_4>() as libc::c_ulong)
    {
        return !(0 as *mut libc::c_void).is_null();
    }
    if opt.debug as libc::c_long != 0 {
        debug_logprintf(
            b"Setting %s (%s) to %s\n\0" as *const u8 as *const libc::c_char,
            com,
            commands[comind as usize].name,
            val,
        );
    }
    return (commands[comind as usize].action)
        .unwrap()(com, val, commands[comind as usize].place);
}
unsafe extern "C" fn setval_internal_tilde(
    mut comind: libc::c_int,
    mut com: *const libc::c_char,
    mut val: *const libc::c_char,
) -> bool {
    let mut ret: bool = false;
    let mut homelen: libc::c_int = 0;
    let mut pstring: *mut *mut libc::c_char = 0 as *mut *mut libc::c_char;
    ret = setval_internal(comind, com, val);
    if (commands[comind as usize].action
        == Some(
            cmd_file
                as unsafe extern "C" fn(
                    *const libc::c_char,
                    *const libc::c_char,
                    *mut libc::c_void,
                ) -> bool,
        )
        || commands[comind as usize].action
            == Some(
                cmd_directory
                    as unsafe extern "C" fn(
                        *const libc::c_char,
                        *const libc::c_char,
                        *mut libc::c_void,
                    ) -> bool,
            )) && ret as libc::c_int != 0
        && (*val as libc::c_int == '~' as i32
            && *val.offset(1 as libc::c_int as isize) as libc::c_int == '/' as i32)
    {
        pstring = commands[comind as usize].place as *mut *mut libc::c_char;
        if !(opt.homedir).is_null() {
            let mut home: *mut libc::c_char = xstrdup(opt.homedir);
            homelen = strlen(home) as libc::c_int;
            while homelen != 0
                && *home.offset((homelen - 1 as libc::c_int) as isize) as libc::c_int
                    == '/' as i32
            {
                homelen -= 1;
                *home.offset(homelen as isize) = '\0' as i32 as libc::c_char;
            }
            rpl_free(*pstring as *mut libc::c_void);
            *pstring = 0 as *mut libc::c_char;
            val = val
                .offset(
                    (strspn(
                        val.offset(1 as libc::c_int as isize),
                        b"/\0" as *const u8 as *const libc::c_char,
                    ))
                        .wrapping_add(1 as libc::c_int as libc::c_ulong) as isize,
                );
            *pstring = concat_strings(
                home,
                b"/\0" as *const u8 as *const libc::c_char,
                val,
                0 as *mut libc::c_char,
            );
            rpl_free(home as *mut libc::c_void);
            home = 0 as *mut libc::c_char;
        }
    }
    return ret;
}
pub unsafe extern "C" fn setoptval(
    mut com: *const libc::c_char,
    mut val: *const libc::c_char,
    mut optname: *const libc::c_char,
) {
    let mut dd_optname: [libc::c_char; 29] = [0; 29];
    if snprintf(
        dd_optname.as_mut_ptr(),
        ::std::mem::size_of::<[libc::c_char; 29]>() as libc::c_ulong,
        b"--%s\0" as *const u8 as *const libc::c_char,
        optname,
    ) as libc::c_uint as libc::c_ulong
        > ::std::mem::size_of::<[libc::c_char; 29]>() as libc::c_ulong
    {
        exit(WGET_EXIT_PARSE_ERROR as libc::c_int);
    }
    if !setval_internal(command_by_name(com), dd_optname.as_mut_ptr(), val) {
        exit(WGET_EXIT_PARSE_ERROR as libc::c_int);
    }
}
pub unsafe extern "C" fn run_command(mut cmdopt: *const libc::c_char) {
    let mut com: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut val: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut comind: libc::c_int = 0;
    match parse_line(cmdopt, &mut com, &mut val, &mut comind) as libc::c_uint {
        0 => {
            if !setval_internal(comind, com, val) {
                exit(WGET_EXIT_PARSE_ERROR as libc::c_int);
            }
            rpl_free(com as *mut libc::c_void);
            com = 0 as *mut libc::c_char;
            rpl_free(val as *mut libc::c_void);
            val = 0 as *mut libc::c_char;
        }
        _ => {
            fprintf(
                stderr,
                dcgettext(
                    0 as *const libc::c_char,
                    b"%s: Invalid --execute command %s\n\0" as *const u8
                        as *const libc::c_char,
                    5 as libc::c_int,
                ),
                exec_name,
                quote(cmdopt),
            );
            exit(WGET_EXIT_PARSE_ERROR as libc::c_int);
        }
    };
}
unsafe extern "C" fn cmd_boolean_internal(
    mut com: *const libc::c_char,
    mut val: *const libc::c_char,
    mut place: *mut libc::c_void,
) -> libc::c_int {
    if c_tolower(*val.offset(0 as libc::c_int as isize) as libc::c_int) == 'o' as i32
        && c_tolower(*val.offset(1 as libc::c_int as isize) as libc::c_int) == 'n' as i32
        && *val.offset(2 as libc::c_int as isize) as libc::c_int == '\0' as i32
        || c_tolower(*val.offset(0 as libc::c_int as isize) as libc::c_int) == 'y' as i32
            && c_tolower(*val.offset(1 as libc::c_int as isize) as libc::c_int)
                == 'e' as i32
            && c_tolower(*val.offset(2 as libc::c_int as isize) as libc::c_int)
                == 's' as i32
            && *val.offset(3 as libc::c_int as isize) as libc::c_int == '\0' as i32
        || c_tolower(*val.offset(0 as libc::c_int as isize) as libc::c_int) == '1' as i32
            && *val.offset(1 as libc::c_int as isize) as libc::c_int == '\0' as i32
    {
        return 1 as libc::c_int
    } else if c_tolower(*val.offset(0 as libc::c_int as isize) as libc::c_int)
        == 'o' as i32
        && c_tolower(*val.offset(1 as libc::c_int as isize) as libc::c_int) == 'f' as i32
        && c_tolower(*val.offset(2 as libc::c_int as isize) as libc::c_int) == 'f' as i32
        && *val.offset(3 as libc::c_int as isize) as libc::c_int == '\0' as i32
        || c_tolower(*val.offset(0 as libc::c_int as isize) as libc::c_int) == 'n' as i32
            && c_tolower(*val.offset(1 as libc::c_int as isize) as libc::c_int)
                == 'o' as i32
            && *val.offset(2 as libc::c_int as isize) as libc::c_int == '\0' as i32
        || c_tolower(*val.offset(0 as libc::c_int as isize) as libc::c_int) == '0' as i32
            && *val.offset(1 as libc::c_int as isize) as libc::c_int == '\0' as i32
    {
        return 0 as libc::c_int
    }
    return -(1 as libc::c_int);
}
unsafe extern "C" fn cmd_boolean(
    mut com: *const libc::c_char,
    mut val: *const libc::c_char,
    mut place: *mut libc::c_void,
) -> bool {
    let mut value: bool = false;
    match cmd_boolean_internal(com, val, place) {
        0 => {
            value = 0 as libc::c_int != 0;
        }
        1 => {
            value = 1 as libc::c_int != 0;
        }
        _ => {
            fprintf(
                stderr,
                dcgettext(
                    0 as *const libc::c_char,
                    b"%s: %s: Invalid boolean %s; use `on' or `off'.\n\0" as *const u8
                        as *const libc::c_char,
                    5 as libc::c_int,
                ),
                exec_name,
                com,
                quote(val),
            );
            return 0 as libc::c_int != 0;
        }
    }
    *(place as *mut bool) = value;
    return 1 as libc::c_int != 0;
}
unsafe extern "C" fn cmd_number(
    mut com: *const libc::c_char,
    mut val: *const libc::c_char,
    mut place: *mut libc::c_void,
) -> bool {
    let mut l: libc::c_long = rpl_strtol(
        val,
        0 as *mut *mut libc::c_char,
        10 as libc::c_int,
    );
    if (l == -(9223372036854775807 as libc::c_long) - 1 as libc::c_long
        || l == 9223372036854775807 as libc::c_long)
        && *__errno_location() == 34 as libc::c_int
        || l < 0 as libc::c_int as libc::c_long
        || l > 2147483647 as libc::c_int as libc::c_long
    {
        fprintf(
            stderr,
            dcgettext(
                0 as *const libc::c_char,
                b"%s: %s: Invalid number %s.\n\0" as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ),
            exec_name,
            com,
            quote(val),
        );
        return 0 as libc::c_int != 0;
    }
    *(place as *mut libc::c_int) = l as libc::c_int;
    return 1 as libc::c_int != 0;
}
unsafe extern "C" fn cmd_number_inf(
    mut com: *const libc::c_char,
    mut val: *const libc::c_char,
    mut place: *mut libc::c_void,
) -> bool {
    if c_strcasecmp(val, b"inf\0" as *const u8 as *const libc::c_char) == 0 {
        *(place as *mut libc::c_int) = 0 as libc::c_int;
        return 1 as libc::c_int != 0;
    }
    return cmd_number(com, val, place);
}
unsafe extern "C" fn cmd_string(
    mut com: *const libc::c_char,
    mut val: *const libc::c_char,
    mut place: *mut libc::c_void,
) -> bool {
    let mut pstring: *mut *mut libc::c_char = place as *mut *mut libc::c_char;
    rpl_free(*pstring as *mut libc::c_void);
    *pstring = 0 as *mut libc::c_char;
    *pstring = xstrdup(val);
    return 1 as libc::c_int != 0;
}
unsafe extern "C" fn cmd_string_uppercase(
    mut com: *const libc::c_char,
    mut val: *const libc::c_char,
    mut place: *mut libc::c_void,
) -> bool {
    let mut q: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut pstring: *mut *mut libc::c_char = 0 as *mut *mut libc::c_char;
    pstring = place as *mut *mut libc::c_char;
    rpl_free(*pstring as *mut libc::c_void);
    *pstring = 0 as *mut libc::c_char;
    *pstring = xmalloc((strlen(val)).wrapping_add(1 as libc::c_int as libc::c_ulong))
        as *mut libc::c_char;
    q = *pstring;
    while *val != 0 {
        *q = c_toupper(*val as libc::c_int) as libc::c_char;
        val = val.offset(1);
        val;
        q = q.offset(1);
        q;
    }
    *q = '\0' as i32 as libc::c_char;
    return 1 as libc::c_int != 0;
}
unsafe extern "C" fn cmd_file(
    mut com: *const libc::c_char,
    mut val: *const libc::c_char,
    mut place: *mut libc::c_void,
) -> bool {
    let mut pstring: *mut *mut libc::c_char = place as *mut *mut libc::c_char;
    rpl_free(*pstring as *mut libc::c_void);
    *pstring = 0 as *mut libc::c_char;
    *pstring = xstrdup(val);
    return 1 as libc::c_int != 0;
}
unsafe extern "C" fn cmd_directory(
    mut com: *const libc::c_char,
    mut val: *const libc::c_char,
    mut place: *mut libc::c_void,
) -> bool {
    let mut s: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut t: *mut libc::c_char = 0 as *mut libc::c_char;
    if !cmd_file(com, val, place) {
        return 0 as libc::c_int != 0;
    }
    s = *(place as *mut *mut libc::c_char);
    t = s.offset(strlen(s) as isize);
    while t > s
        && {
            t = t.offset(-1);
            *t as libc::c_int == '/' as i32
        }
    {
        *t = '\0' as i32 as libc::c_char;
    }
    return 1 as libc::c_int != 0;
}
unsafe extern "C" fn cmd_vector(
    mut com: *const libc::c_char,
    mut val: *const libc::c_char,
    mut place: *mut libc::c_void,
) -> bool {
    let mut pvec: *mut *mut *mut libc::c_char = place as *mut *mut *mut libc::c_char;
    if *val != 0 {
        *pvec = merge_vecs(*pvec, sepstring(val));
    } else {
        free_vec(*pvec);
        *pvec = 0 as *mut *mut libc::c_char;
    }
    return 1 as libc::c_int != 0;
}
unsafe extern "C" fn cmd_directory_vector(
    mut com: *const libc::c_char,
    mut val: *const libc::c_char,
    mut place: *mut libc::c_void,
) -> bool {
    let mut pvec: *mut *mut *mut libc::c_char = place as *mut *mut *mut libc::c_char;
    if *val != 0 {
        let mut t: *mut *mut libc::c_char = 0 as *mut *mut libc::c_char;
        let mut seps: *mut *mut libc::c_char = 0 as *mut *mut libc::c_char;
        seps = sepstring(val);
        t = seps;
        while !t.is_null() && !(*t).is_null() {
            let mut len: libc::c_int = strlen(*t) as libc::c_int;
            if len > 1 as libc::c_int {
                if *(*t).offset((len - 1 as libc::c_int) as isize) as libc::c_int
                    == '/' as i32
                {
                    *(*t)
                        .offset(
                            (len - 1 as libc::c_int) as isize,
                        ) = '\0' as i32 as libc::c_char;
                }
            }
            t = t.offset(1);
            t;
        }
        *pvec = merge_vecs(*pvec, seps);
    } else {
        free_vec(*pvec);
        *pvec = 0 as *mut *mut libc::c_char;
    }
    return 1 as libc::c_int != 0;
}
unsafe extern "C" fn parse_bytes_helper(
    mut val: *const libc::c_char,
    mut result: *mut libc::c_double,
) -> bool {
    let mut number: libc::c_double = 0.;
    let mut mult: libc::c_double = 0.;
    let mut end: *const libc::c_char = val.offset(strlen(val) as isize);
    if 0 as libc::c_int == strcmp(val, b"inf\0" as *const u8 as *const libc::c_char) {
        *result = 0 as libc::c_int as libc::c_double;
        return 1 as libc::c_int != 0;
    }
    while val < end
        && c_isspace(*end.offset(-(1 as libc::c_int) as isize) as libc::c_int)
            as libc::c_int != 0
    {
        end = end.offset(-1);
        end;
    }
    if val == end {
        return 0 as libc::c_int != 0;
    }
    match c_tolower(*end.offset(-(1 as libc::c_int) as isize) as libc::c_int) {
        107 => {
            end = end.offset(-1);
            end;
            mult = 1024.0f64;
        }
        109 => {
            end = end.offset(-1);
            end;
            mult = 1048576.0f64;
        }
        103 => {
            end = end.offset(-1);
            end;
            mult = 1073741824.0f64;
        }
        116 => {
            end = end.offset(-1);
            end;
            mult = 1099511627776.0f64;
        }
        _ => {
            mult = 1 as libc::c_int as libc::c_double;
        }
    }
    while val < end && c_isspace(*val as libc::c_int) as libc::c_int != 0 {
        val = val.offset(1);
        val;
    }
    while val < end
        && c_isspace(*end.offset(-(1 as libc::c_int) as isize) as libc::c_int)
            as libc::c_int != 0
    {
        end = end.offset(-1);
        end;
    }
    if val == end {
        return 0 as libc::c_int != 0;
    }
    if !simple_atof(val, end, &mut number) || number < 0 as libc::c_int as libc::c_double
    {
        return 0 as libc::c_int != 0;
    }
    *result = number * mult;
    return 1 as libc::c_int != 0;
}
unsafe extern "C" fn cmd_bytes(
    mut com: *const libc::c_char,
    mut val: *const libc::c_char,
    mut place: *mut libc::c_void,
) -> bool {
    let mut byte_value: libc::c_double = 0.;
    if !parse_bytes_helper(val, &mut byte_value) {
        fprintf(
            stderr,
            dcgettext(
                0 as *const libc::c_char,
                b"%s: %s: Invalid byte value %s\n\0" as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ),
            exec_name,
            com,
            quote(val),
        );
        return 0 as libc::c_int != 0;
    }
    *(place as *mut wgint) = byte_value as wgint;
    return 1 as libc::c_int != 0;
}
unsafe extern "C" fn cmd_bytes_sum(
    mut com: *const libc::c_char,
    mut val: *const libc::c_char,
    mut place: *mut libc::c_void,
) -> bool {
    let mut byte_value: libc::c_double = 0.;
    if !parse_bytes_helper(val, &mut byte_value)
        || byte_value
            < (-(9223372036854775807 as libc::c_long) - 1 as libc::c_int as libc::c_long)
                as libc::c_double
        || byte_value > 9223372036854775807 as libc::c_long as libc::c_double
    {
        fprintf(
            stderr,
            dcgettext(
                0 as *const libc::c_char,
                b"%s: %s: Invalid byte value %s\n\0" as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ),
            exec_name,
            com,
            quote(val),
        );
        return 0 as libc::c_int != 0;
    }
    *(place as *mut wgint) = byte_value as wgint;
    return 1 as libc::c_int != 0;
}
unsafe extern "C" fn cmd_time(
    mut com: *const libc::c_char,
    mut val: *const libc::c_char,
    mut place: *mut libc::c_void,
) -> bool {
    let mut number: libc::c_double = 0.;
    let mut mult: libc::c_double = 0.;
    let mut end: *const libc::c_char = val.offset(strlen(val) as isize);
    while val < end
        && c_isspace(*end.offset(-(1 as libc::c_int) as isize) as libc::c_int)
            as libc::c_int != 0
    {
        end = end.offset(-1);
        end;
    }
    if !(val == end) {
        match c_tolower(*end.offset(-(1 as libc::c_int) as isize) as libc::c_int) {
            115 => {
                end = end.offset(-1);
                end;
                mult = 1 as libc::c_int as libc::c_double;
            }
            109 => {
                end = end.offset(-1);
                end;
                mult = 60 as libc::c_int as libc::c_double;
            }
            104 => {
                end = end.offset(-1);
                end;
                mult = 3600 as libc::c_int as libc::c_double;
            }
            100 => {
                end = end.offset(-1);
                end;
                mult = 86400.0f64;
            }
            119 => {
                end = end.offset(-1);
                end;
                mult = 604800.0f64;
            }
            _ => {
                mult = 1 as libc::c_int as libc::c_double;
            }
        }
        while val < end && c_isspace(*val as libc::c_int) as libc::c_int != 0 {
            val = val.offset(1);
            val;
        }
        while val < end
            && c_isspace(*end.offset(-(1 as libc::c_int) as isize) as libc::c_int)
                as libc::c_int != 0
        {
            end = end.offset(-1);
            end;
        }
        if !(val == end) {
            if simple_atof(val, end, &mut number) {
                if number < 0 as libc::c_int as libc::c_double {
                    fprintf(
                        stderr,
                        dcgettext(
                            0 as *const libc::c_char,
                            b"%s: %s: Negative time period %s\n\0" as *const u8
                                as *const libc::c_char,
                            5 as libc::c_int,
                        ),
                        exec_name,
                        com,
                        quote(val),
                    );
                    return 0 as libc::c_int != 0;
                }
                *(place as *mut libc::c_double) = number * mult;
                return 1 as libc::c_int != 0;
            }
        }
    }
    fprintf(
        stderr,
        dcgettext(
            0 as *const libc::c_char,
            b"%s: %s: Invalid time period %s\n\0" as *const u8 as *const libc::c_char,
            5 as libc::c_int,
        ),
        exec_name,
        com,
        quote(val),
    );
    return 0 as libc::c_int != 0;
}
unsafe extern "C" fn cmd_use_askpass(
    mut com: *const libc::c_char,
    mut val: *const libc::c_char,
    mut place: *mut libc::c_void,
) -> bool {
    let mut env_name: *const libc::c_char = b"WGET_ASKPASS\0" as *const u8
        as *const libc::c_char;
    let mut env: *const libc::c_char = 0 as *const libc::c_char;
    if !val.is_null() && *val as libc::c_int != 0 {
        return cmd_string(com, val, place);
    }
    env = getenv(env_name);
    if !(!env.is_null() && *env as libc::c_int != 0) {
        env_name = b"SSH_ASKPASS\0" as *const u8 as *const libc::c_char;
        env = getenv(env_name);
    }
    if !(!env.is_null() && *env as libc::c_int != 0) {
        fprintf(
            stderr,
            dcgettext(
                0 as *const libc::c_char,
                b"use-askpass requires a string or either environment variable WGET_ASKPASS or SSH_ASKPASS to be set.\n\0"
                    as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ),
        );
        return 0 as libc::c_int != 0;
    }
    return cmd_string(com, env, place);
}
unsafe extern "C" fn cmd_spec_compression(
    mut com: *const libc::c_char,
    mut val: *const libc::c_char,
    mut place: *mut libc::c_void,
) -> bool {
    static mut choices: [decode_item; 3] = [
        {
            let mut init = decode_item {
                name: b"auto\0" as *const u8 as *const libc::c_char,
                code: compression_auto as libc::c_int,
            };
            init
        },
        {
            let mut init = decode_item {
                name: b"gzip\0" as *const u8 as *const libc::c_char,
                code: compression_gzip as libc::c_int,
            };
            init
        },
        {
            let mut init = decode_item {
                name: b"none\0" as *const u8 as *const libc::c_char,
                code: compression_none as libc::c_int,
            };
            init
        },
    ];
    let mut ok: libc::c_int = decode_string(
        val,
        choices.as_ptr(),
        (::std::mem::size_of::<[decode_item; 3]>() as libc::c_ulong)
            .wrapping_div(::std::mem::size_of::<decode_item>() as libc::c_ulong)
            as libc::c_int,
        place as *mut libc::c_int,
    ) as libc::c_int;
    if ok == 0 {
        fprintf(
            stderr,
            dcgettext(
                0 as *const libc::c_char,
                b"%s: %s: Invalid value %s.\n\0" as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ),
            exec_name,
            com,
            quote(val),
        );
    }
    return ok != 0;
}
unsafe extern "C" fn cmd_spec_dirstruct(
    mut com: *const libc::c_char,
    mut val: *const libc::c_char,
    mut place_ignored: *mut libc::c_void,
) -> bool {
    if !cmd_boolean(com, val, &mut opt.dirstruct as *mut bool as *mut libc::c_void) {
        return 0 as libc::c_int != 0;
    }
    if opt.dirstruct {
        opt.no_dirstruct = 0 as libc::c_int != 0;
    } else {
        opt.no_dirstruct = 1 as libc::c_int != 0;
    }
    return 1 as libc::c_int != 0;
}
unsafe extern "C" fn cmd_spec_header(
    mut com: *const libc::c_char,
    mut val: *const libc::c_char,
    mut place_ignored: *mut libc::c_void,
) -> bool {
    if *val as libc::c_int == '\0' as i32 {
        free_vec(opt.user_headers);
        opt.user_headers = 0 as *mut *mut libc::c_char;
        return 1 as libc::c_int != 0;
    }
    if !check_user_specified_header(val) {
        fprintf(
            stderr,
            dcgettext(
                0 as *const libc::c_char,
                b"%s: %s: Invalid header %s.\n\0" as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ),
            exec_name,
            com,
            quote(val),
        );
        return 0 as libc::c_int != 0;
    }
    opt.user_headers = vec_append(opt.user_headers, val);
    return 1 as libc::c_int != 0;
}
unsafe extern "C" fn cmd_spec_warc_header(
    mut com: *const libc::c_char,
    mut val: *const libc::c_char,
    mut place_ignored: *mut libc::c_void,
) -> bool {
    if *val as libc::c_int == '\0' as i32 {
        free_vec(opt.warc_user_headers);
        opt.warc_user_headers = 0 as *mut *mut libc::c_char;
        return 1 as libc::c_int != 0;
    }
    if !check_user_specified_header(val) {
        fprintf(
            stderr,
            dcgettext(
                0 as *const libc::c_char,
                b"%s: %s: Invalid WARC header %s.\n\0" as *const u8
                    as *const libc::c_char,
                5 as libc::c_int,
            ),
            exec_name,
            com,
            quote(val),
        );
        return 0 as libc::c_int != 0;
    }
    opt.warc_user_headers = vec_append(opt.warc_user_headers, val);
    return 1 as libc::c_int != 0;
}
unsafe extern "C" fn cmd_spec_htmlify(
    mut com: *const libc::c_char,
    mut val: *const libc::c_char,
    mut place_ignored: *mut libc::c_void,
) -> bool {
    let mut flag: libc::c_int = cmd_boolean(
        com,
        val,
        &mut opt.htmlify as *mut bool as *mut libc::c_void,
    ) as libc::c_int;
    if flag != 0 && !opt.htmlify {
        opt.remove_listing = 0 as libc::c_int != 0;
    }
    return flag != 0;
}
unsafe extern "C" fn cmd_spec_mirror(
    mut com: *const libc::c_char,
    mut val: *const libc::c_char,
    mut place_ignored: *mut libc::c_void,
) -> bool {
    let mut mirror: bool = false;
    if !cmd_boolean(com, val, &mut mirror as *mut bool as *mut libc::c_void) {
        return 0 as libc::c_int != 0;
    }
    if mirror {
        opt.recursive = 1 as libc::c_int != 0;
        if !opt.no_dirstruct {
            opt.dirstruct = 1 as libc::c_int != 0;
        }
        opt.timestamping = 1 as libc::c_int != 0;
        opt.reclevel = -(1 as libc::c_int);
        opt.remove_listing = 0 as libc::c_int != 0;
    }
    return 1 as libc::c_int != 0;
}
unsafe extern "C" fn cmd_spec_prefer_family(
    mut com: *const libc::c_char,
    mut val: *const libc::c_char,
    mut place_ignored: *mut libc::c_void,
) -> bool {
    static mut choices: [decode_item; 3] = [
        {
            let mut init = decode_item {
                name: b"IPv4\0" as *const u8 as *const libc::c_char,
                code: prefer_ipv4 as libc::c_int,
            };
            init
        },
        {
            let mut init = decode_item {
                name: b"IPv6\0" as *const u8 as *const libc::c_char,
                code: prefer_ipv6 as libc::c_int,
            };
            init
        },
        {
            let mut init = decode_item {
                name: b"none\0" as *const u8 as *const libc::c_char,
                code: prefer_none as libc::c_int,
            };
            init
        },
    ];
    let mut prefer_family: libc::c_int = prefer_none as libc::c_int;
    let mut ok: libc::c_int = decode_string(
        val,
        choices.as_ptr(),
        (::std::mem::size_of::<[decode_item; 3]>() as libc::c_ulong)
            .wrapping_div(::std::mem::size_of::<decode_item>() as libc::c_ulong)
            as libc::c_int,
        &mut prefer_family,
    ) as libc::c_int;
    if ok == 0 {
        fprintf(
            stderr,
            dcgettext(
                0 as *const libc::c_char,
                b"%s: %s: Invalid value %s.\n\0" as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ),
            exec_name,
            com,
            quote(val),
        );
    }
    opt.prefer_family = prefer_family as C2RustUnnamed;
    return ok != 0;
}
unsafe extern "C" fn cmd_spec_progress(
    mut com: *const libc::c_char,
    mut val: *const libc::c_char,
    mut place_ignored: *mut libc::c_void,
) -> bool {
    if !valid_progress_implementation_p(val) {
        fprintf(
            stderr,
            dcgettext(
                0 as *const libc::c_char,
                b"%s: %s: Invalid progress type %s.\n\0" as *const u8
                    as *const libc::c_char,
                5 as libc::c_int,
            ),
            exec_name,
            com,
            quote(val),
        );
        return 0 as libc::c_int != 0;
    }
    rpl_free(opt.progress_type as *mut libc::c_void);
    opt.progress_type = 0 as *mut libc::c_char;
    opt.progress_type = xstrdup(val);
    return 1 as libc::c_int != 0;
}
unsafe extern "C" fn cmd_spec_recursive(
    mut com: *const libc::c_char,
    mut val: *const libc::c_char,
    mut place_ignored: *mut libc::c_void,
) -> bool {
    if !cmd_boolean(com, val, &mut opt.recursive as *mut bool as *mut libc::c_void) {
        return 0 as libc::c_int != 0
    } else if opt.recursive as libc::c_int != 0 && !opt.no_dirstruct {
        opt.dirstruct = 1 as libc::c_int != 0;
    }
    return 1 as libc::c_int != 0;
}
unsafe extern "C" fn cmd_spec_regex_type(
    mut com: *const libc::c_char,
    mut val: *const libc::c_char,
    mut place_ignored: *mut libc::c_void,
) -> bool {
    static mut choices: [decode_item; 2] = [
        {
            let mut init = decode_item {
                name: b"posix\0" as *const u8 as *const libc::c_char,
                code: regex_type_posix as libc::c_int,
            };
            init
        },
        {
            let mut init = decode_item {
                name: b"pcre\0" as *const u8 as *const libc::c_char,
                code: regex_type_pcre as libc::c_int,
            };
            init
        },
    ];
    let mut regex_type: libc::c_int = regex_type_posix as libc::c_int;
    let mut ok: libc::c_int = decode_string(
        val,
        choices.as_ptr(),
        (::std::mem::size_of::<[decode_item; 2]>() as libc::c_ulong)
            .wrapping_div(::std::mem::size_of::<decode_item>() as libc::c_ulong)
            as libc::c_int,
        &mut regex_type,
    ) as libc::c_int;
    if ok == 0 {
        fprintf(
            stderr,
            dcgettext(
                0 as *const libc::c_char,
                b"%s: %s: Invalid value %s.\n\0" as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ),
            exec_name,
            com,
            quote(val),
        );
    }
    opt.regex_type = regex_type as C2RustUnnamed_2;
    return ok != 0;
}
unsafe extern "C" fn cmd_spec_restrict_file_names(
    mut com: *const libc::c_char,
    mut val: *const libc::c_char,
    mut place_ignored: *mut libc::c_void,
) -> bool {
    let mut restrict_os: libc::c_int = opt.restrict_files_os as libc::c_int;
    let mut restrict_ctrl: libc::c_int = opt.restrict_files_ctrl as libc::c_int;
    let mut restrict_case: libc::c_int = opt.restrict_files_case as libc::c_int;
    let mut restrict_nonascii: libc::c_int = opt.restrict_files_nonascii as libc::c_int;
    let mut end: *const libc::c_char = 0 as *const libc::c_char;
    loop {
        end = strchr(val, ',' as i32);
        if end.is_null() {
            end = val.offset(strlen(val) as isize);
        }
        if end.offset_from(val) as libc::c_long as libc::c_ulong
            == (::std::mem::size_of::<[libc::c_char; 5]>() as libc::c_ulong)
                .wrapping_sub(1 as libc::c_int as libc::c_ulong)
            && memcmp(
                val as *const libc::c_void,
                b"unix\0" as *const u8 as *const libc::c_char as *const libc::c_void,
                (::std::mem::size_of::<[libc::c_char; 5]>() as libc::c_ulong)
                    .wrapping_sub(1 as libc::c_int as libc::c_ulong),
            ) == 0
        {
            restrict_os = restrict_unix as libc::c_int;
        } else if end.offset_from(val) as libc::c_long as libc::c_ulong
            == (::std::mem::size_of::<[libc::c_char; 4]>() as libc::c_ulong)
                .wrapping_sub(1 as libc::c_int as libc::c_ulong)
            && memcmp(
                val as *const libc::c_void,
                b"vms\0" as *const u8 as *const libc::c_char as *const libc::c_void,
                (::std::mem::size_of::<[libc::c_char; 4]>() as libc::c_ulong)
                    .wrapping_sub(1 as libc::c_int as libc::c_ulong),
            ) == 0
        {
            restrict_os = restrict_vms as libc::c_int;
        } else if end.offset_from(val) as libc::c_long as libc::c_ulong
            == (::std::mem::size_of::<[libc::c_char; 8]>() as libc::c_ulong)
                .wrapping_sub(1 as libc::c_int as libc::c_ulong)
            && memcmp(
                val as *const libc::c_void,
                b"windows\0" as *const u8 as *const libc::c_char as *const libc::c_void,
                (::std::mem::size_of::<[libc::c_char; 8]>() as libc::c_ulong)
                    .wrapping_sub(1 as libc::c_int as libc::c_ulong),
            ) == 0
        {
            restrict_os = restrict_windows as libc::c_int;
        } else if end.offset_from(val) as libc::c_long as libc::c_ulong
            == (::std::mem::size_of::<[libc::c_char; 10]>() as libc::c_ulong)
                .wrapping_sub(1 as libc::c_int as libc::c_ulong)
            && memcmp(
                val as *const libc::c_void,
                b"lowercase\0" as *const u8 as *const libc::c_char
                    as *const libc::c_void,
                (::std::mem::size_of::<[libc::c_char; 10]>() as libc::c_ulong)
                    .wrapping_sub(1 as libc::c_int as libc::c_ulong),
            ) == 0
        {
            restrict_case = restrict_lowercase as libc::c_int;
        } else if end.offset_from(val) as libc::c_long as libc::c_ulong
            == (::std::mem::size_of::<[libc::c_char; 10]>() as libc::c_ulong)
                .wrapping_sub(1 as libc::c_int as libc::c_ulong)
            && memcmp(
                val as *const libc::c_void,
                b"uppercase\0" as *const u8 as *const libc::c_char
                    as *const libc::c_void,
                (::std::mem::size_of::<[libc::c_char; 10]>() as libc::c_ulong)
                    .wrapping_sub(1 as libc::c_int as libc::c_ulong),
            ) == 0
        {
            restrict_case = restrict_uppercase as libc::c_int;
        } else if end.offset_from(val) as libc::c_long as libc::c_ulong
            == (::std::mem::size_of::<[libc::c_char; 10]>() as libc::c_ulong)
                .wrapping_sub(1 as libc::c_int as libc::c_ulong)
            && memcmp(
                val as *const libc::c_void,
                b"nocontrol\0" as *const u8 as *const libc::c_char
                    as *const libc::c_void,
                (::std::mem::size_of::<[libc::c_char; 10]>() as libc::c_ulong)
                    .wrapping_sub(1 as libc::c_int as libc::c_ulong),
            ) == 0
        {
            restrict_ctrl = 0 as libc::c_int;
        } else if end.offset_from(val) as libc::c_long as libc::c_ulong
            == (::std::mem::size_of::<[libc::c_char; 6]>() as libc::c_ulong)
                .wrapping_sub(1 as libc::c_int as libc::c_ulong)
            && memcmp(
                val as *const libc::c_void,
                b"ascii\0" as *const u8 as *const libc::c_char as *const libc::c_void,
                (::std::mem::size_of::<[libc::c_char; 6]>() as libc::c_ulong)
                    .wrapping_sub(1 as libc::c_int as libc::c_ulong),
            ) == 0
        {
            restrict_nonascii = 1 as libc::c_int;
        } else {
            fprintf(
                stderr,
                dcgettext(
                    0 as *const libc::c_char,
                    b"%s: %s: Invalid restriction %s,\n    use [unix|vms|windows],[lowercase|uppercase],[nocontrol],[ascii].\n\0"
                        as *const u8 as *const libc::c_char,
                    5 as libc::c_int,
                ),
                exec_name,
                com,
                quote(val),
            );
            return 0 as libc::c_int != 0;
        }
        if *end != 0 {
            val = end.offset(1 as libc::c_int as isize);
        }
        if !(*val as libc::c_int != 0 && *end as libc::c_int != 0) {
            break;
        }
    }
    opt.restrict_files_os = restrict_os as C2RustUnnamed_1;
    opt.restrict_files_ctrl = restrict_ctrl != 0;
    opt.restrict_files_case = restrict_case as C2RustUnnamed_0;
    opt.restrict_files_nonascii = restrict_nonascii != 0;
    return 1 as libc::c_int != 0;
}
unsafe extern "C" fn cmd_spec_report_speed(
    mut com: *const libc::c_char,
    mut val: *const libc::c_char,
    mut place_ignored: *mut libc::c_void,
) -> bool {
    opt
        .report_bps = c_strcasecmp(val, b"bits\0" as *const u8 as *const libc::c_char)
        == 0 as libc::c_int;
    if !opt.report_bps {
        fprintf(
            stderr,
            dcgettext(
                0 as *const libc::c_char,
                b"%s: %s: Invalid value %s.\n\0" as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ),
            exec_name,
            com,
            quote(val),
        );
    }
    return opt.report_bps;
}
unsafe extern "C" fn cmd_spec_timeout(
    mut com: *const libc::c_char,
    mut val: *const libc::c_char,
    mut place_ignored: *mut libc::c_void,
) -> bool {
    let mut value: libc::c_double = 0.;
    if !cmd_time(com, val, &mut value as *mut libc::c_double as *mut libc::c_void) {
        return 0 as libc::c_int != 0;
    }
    opt.read_timeout = value;
    opt.connect_timeout = value;
    opt.dns_timeout = value;
    return 1 as libc::c_int != 0;
}
unsafe extern "C" fn cmd_spec_useragent(
    mut com: *const libc::c_char,
    mut val: *const libc::c_char,
    mut place_ignored: *mut libc::c_void,
) -> bool {
    if !(strchr(val, '\n' as i32)).is_null() {
        fprintf(
            stderr,
            dcgettext(
                0 as *const libc::c_char,
                b"%s: %s: Invalid value %s.\n\0" as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ),
            exec_name,
            com,
            quote(val),
        );
        return 0 as libc::c_int != 0;
    }
    rpl_free(opt.useragent as *mut libc::c_void);
    opt.useragent = 0 as *mut libc::c_char;
    opt.useragent = xstrdup(val);
    return 1 as libc::c_int != 0;
}
unsafe extern "C" fn cmd_spec_progressdisp(
    mut com: *const libc::c_char,
    mut val: *const libc::c_char,
    mut place: *mut libc::c_void,
) -> bool {
    let mut flag: bool = false;
    if cmd_boolean(com, val, &mut flag as *mut bool as *mut libc::c_void) {
        opt.show_progress = flag as libc::c_int;
        return 1 as libc::c_int != 0;
    }
    return 0 as libc::c_int != 0;
}
unsafe extern "C" fn cmd_spec_verbose(
    mut com: *const libc::c_char,
    mut val: *const libc::c_char,
    mut place_ignored: *mut libc::c_void,
) -> bool {
    let mut flag: bool = false;
    if cmd_boolean(com, val, &mut flag as *mut bool as *mut libc::c_void) {
        opt.verbose = flag as libc::c_int;
        opt.show_progress = -(1 as libc::c_int);
        return 1 as libc::c_int != 0;
    }
    return 0 as libc::c_int != 0;
}
unsafe extern "C" fn simple_atof(
    mut beg: *const libc::c_char,
    mut end: *const libc::c_char,
    mut dest: *mut libc::c_double,
) -> bool {
    let mut result: libc::c_double = 0 as libc::c_int as libc::c_double;
    let mut negative: bool = 0 as libc::c_int != 0;
    let mut seen_dot: bool = 0 as libc::c_int != 0;
    let mut seen_digit: bool = 0 as libc::c_int != 0;
    let mut divider: libc::c_double = 1 as libc::c_int as libc::c_double;
    let mut p: *const libc::c_char = beg;
    while p < end && c_isspace(*p as libc::c_int) as libc::c_int != 0 {
        p = p.offset(1);
        p;
    }
    if p < end && (*p as libc::c_int == '-' as i32 || *p as libc::c_int == '+' as i32) {
        negative = *p as libc::c_int == '-' as i32;
        p = p.offset(1);
        p;
    }
    while p < end {
        let mut ch: libc::c_char = *p;
        if c_isdigit(ch as libc::c_int) {
            if !seen_dot {
                result = 10 as libc::c_int as libc::c_double * result
                    + (ch as libc::c_int - '0' as i32) as libc::c_double;
            } else {
                divider *= 10 as libc::c_int as libc::c_double;
                result += (ch as libc::c_int - '0' as i32) as libc::c_double / divider;
            }
            seen_digit = 1 as libc::c_int != 0;
        } else if ch as libc::c_int == '.' as i32 {
            if !seen_dot {
                seen_dot = 1 as libc::c_int != 0;
            } else {
                return 0 as libc::c_int != 0
            }
        } else {
            return 0 as libc::c_int != 0
        }
        p = p.offset(1);
        p;
    }
    if !seen_digit {
        return 0 as libc::c_int != 0;
    }
    if negative {
        result = -result;
    }
    *dest = result;
    return 1 as libc::c_int != 0;
}
unsafe extern "C" fn check_user_specified_header(mut s: *const libc::c_char) -> bool {
    let mut p: *const libc::c_char = 0 as *const libc::c_char;
    p = s;
    while *p as libc::c_int != 0 && *p as libc::c_int != ':' as i32
        && !c_isspace(*p as libc::c_int)
    {
        p = p.offset(1);
        p;
    }
    if *p as libc::c_int != ':' as i32 || p == s {
        return 0 as libc::c_int != 0;
    }
    if !(strchr(s, '\n' as i32)).is_null() {
        return 0 as libc::c_int != 0;
    }
    return 1 as libc::c_int != 0;
}
unsafe extern "C" fn decode_string(
    mut val: *const libc::c_char,
    mut items: *const decode_item,
    mut itemcount: libc::c_int,
    mut place: *mut libc::c_int,
) -> bool {
    let mut i: libc::c_int = 0;
    i = 0 as libc::c_int;
    while i < itemcount {
        if 0 as libc::c_int == c_strcasecmp(val, (*items.offset(i as isize)).name) {
            *place = (*items.offset(i as isize)).code;
            return 1 as libc::c_int != 0;
        }
        i += 1;
        i;
    }
    return 0 as libc::c_int != 0;
}
pub unsafe extern "C" fn cleanup() {
    let fresh2 = cleaned_up;
    cleaned_up = cleaned_up + 1;
    if fresh2 != 0 {
        return;
    }
    if !(opt.warc_filename).is_null() {
        warc_close();
    }
    log_close();
    if !output_stream.is_null() && output_stream != stderr {
        let mut fp: *mut FILE = output_stream;
        output_stream = 0 as *mut FILE;
        if fclose(fp) == -(1 as libc::c_int) {
            inform_exit_status(CLOSEFAILED);
        }
    }
}
unsafe extern "C" fn run_static_initializers() {
    commands = [
        {
            let mut init = C2RustUnnamed_4 {
                name: b"accept\0" as *const u8 as *const libc::c_char,
                place: &mut opt.accepts as *mut *mut *mut libc::c_char
                    as *mut libc::c_void,
                action: Some(
                    cmd_vector
                        as unsafe extern "C" fn(
                            *const libc::c_char,
                            *const libc::c_char,
                            *mut libc::c_void,
                        ) -> bool,
                ),
            };
            init
        },
        {
            let mut init = C2RustUnnamed_4 {
                name: b"acceptregex\0" as *const u8 as *const libc::c_char,
                place: &mut opt.acceptregex_s as *mut *mut libc::c_char
                    as *mut libc::c_void,
                action: Some(
                    cmd_string
                        as unsafe extern "C" fn(
                            *const libc::c_char,
                            *const libc::c_char,
                            *mut libc::c_void,
                        ) -> bool,
                ),
            };
            init
        },
        {
            let mut init = C2RustUnnamed_4 {
                name: b"addhostdir\0" as *const u8 as *const libc::c_char,
                place: &mut opt.add_hostdir as *mut bool as *mut libc::c_void,
                action: Some(
                    cmd_boolean
                        as unsafe extern "C" fn(
                            *const libc::c_char,
                            *const libc::c_char,
                            *mut libc::c_void,
                        ) -> bool,
                ),
            };
            init
        },
        {
            let mut init = C2RustUnnamed_4 {
                name: b"adjustextension\0" as *const u8 as *const libc::c_char,
                place: &mut opt.adjust_extension as *mut bool as *mut libc::c_void,
                action: Some(
                    cmd_boolean
                        as unsafe extern "C" fn(
                            *const libc::c_char,
                            *const libc::c_char,
                            *mut libc::c_void,
                        ) -> bool,
                ),
            };
            init
        },
        {
            let mut init = C2RustUnnamed_4 {
                name: b"alwaysrest\0" as *const u8 as *const libc::c_char,
                place: &mut opt.always_rest as *mut bool as *mut libc::c_void,
                action: Some(
                    cmd_boolean
                        as unsafe extern "C" fn(
                            *const libc::c_char,
                            *const libc::c_char,
                            *mut libc::c_void,
                        ) -> bool,
                ),
            };
            init
        },
        {
            let mut init = C2RustUnnamed_4 {
                name: b"askpassword\0" as *const u8 as *const libc::c_char,
                place: &mut opt.ask_passwd as *mut bool as *mut libc::c_void,
                action: Some(
                    cmd_boolean
                        as unsafe extern "C" fn(
                            *const libc::c_char,
                            *const libc::c_char,
                            *mut libc::c_void,
                        ) -> bool,
                ),
            };
            init
        },
        {
            let mut init = C2RustUnnamed_4 {
                name: b"authnochallenge\0" as *const u8 as *const libc::c_char,
                place: &mut opt.auth_without_challenge as *mut bool as *mut libc::c_void,
                action: Some(
                    cmd_boolean
                        as unsafe extern "C" fn(
                            *const libc::c_char,
                            *const libc::c_char,
                            *mut libc::c_void,
                        ) -> bool,
                ),
            };
            init
        },
        {
            let mut init = C2RustUnnamed_4 {
                name: b"background\0" as *const u8 as *const libc::c_char,
                place: &mut opt.background as *mut bool as *mut libc::c_void,
                action: Some(
                    cmd_boolean
                        as unsafe extern "C" fn(
                            *const libc::c_char,
                            *const libc::c_char,
                            *mut libc::c_void,
                        ) -> bool,
                ),
            };
            init
        },
        {
            let mut init = C2RustUnnamed_4 {
                name: b"backupconverted\0" as *const u8 as *const libc::c_char,
                place: &mut opt.backup_converted as *mut bool as *mut libc::c_void,
                action: Some(
                    cmd_boolean
                        as unsafe extern "C" fn(
                            *const libc::c_char,
                            *const libc::c_char,
                            *mut libc::c_void,
                        ) -> bool,
                ),
            };
            init
        },
        {
            let mut init = C2RustUnnamed_4 {
                name: b"backups\0" as *const u8 as *const libc::c_char,
                place: &mut opt.backups as *mut libc::c_int as *mut libc::c_void,
                action: Some(
                    cmd_number
                        as unsafe extern "C" fn(
                            *const libc::c_char,
                            *const libc::c_char,
                            *mut libc::c_void,
                        ) -> bool,
                ),
            };
            init
        },
        {
            let mut init = C2RustUnnamed_4 {
                name: b"base\0" as *const u8 as *const libc::c_char,
                place: &mut opt.base_href as *mut *mut libc::c_char as *mut libc::c_void,
                action: Some(
                    cmd_string
                        as unsafe extern "C" fn(
                            *const libc::c_char,
                            *const libc::c_char,
                            *mut libc::c_void,
                        ) -> bool,
                ),
            };
            init
        },
        {
            let mut init = C2RustUnnamed_4 {
                name: b"bindaddress\0" as *const u8 as *const libc::c_char,
                place: &mut opt.bind_address as *mut *mut libc::c_char
                    as *mut libc::c_void,
                action: Some(
                    cmd_string
                        as unsafe extern "C" fn(
                            *const libc::c_char,
                            *const libc::c_char,
                            *mut libc::c_void,
                        ) -> bool,
                ),
            };
            init
        },
        {
            let mut init = C2RustUnnamed_4 {
                name: b"bodydata\0" as *const u8 as *const libc::c_char,
                place: &mut opt.body_data as *mut *mut libc::c_char as *mut libc::c_void,
                action: Some(
                    cmd_string
                        as unsafe extern "C" fn(
                            *const libc::c_char,
                            *const libc::c_char,
                            *mut libc::c_void,
                        ) -> bool,
                ),
            };
            init
        },
        {
            let mut init = C2RustUnnamed_4 {
                name: b"bodyfile\0" as *const u8 as *const libc::c_char,
                place: &mut opt.body_file as *mut *mut libc::c_char as *mut libc::c_void,
                action: Some(
                    cmd_string
                        as unsafe extern "C" fn(
                            *const libc::c_char,
                            *const libc::c_char,
                            *mut libc::c_void,
                        ) -> bool,
                ),
            };
            init
        },
        {
            let mut init = C2RustUnnamed_4 {
                name: b"cache\0" as *const u8 as *const libc::c_char,
                place: &mut opt.allow_cache as *mut bool as *mut libc::c_void,
                action: Some(
                    cmd_boolean
                        as unsafe extern "C" fn(
                            *const libc::c_char,
                            *const libc::c_char,
                            *mut libc::c_void,
                        ) -> bool,
                ),
            };
            init
        },
        {
            let mut init = C2RustUnnamed_4 {
                name: b"chooseconfig\0" as *const u8 as *const libc::c_char,
                place: &mut opt.choose_config as *mut *mut libc::c_char
                    as *mut libc::c_void,
                action: Some(
                    cmd_file
                        as unsafe extern "C" fn(
                            *const libc::c_char,
                            *const libc::c_char,
                            *mut libc::c_void,
                        ) -> bool,
                ),
            };
            init
        },
        {
            let mut init = C2RustUnnamed_4 {
                name: b"compression\0" as *const u8 as *const libc::c_char,
                place: &mut opt.compression as *mut compression_options
                    as *mut libc::c_void,
                action: Some(
                    cmd_spec_compression
                        as unsafe extern "C" fn(
                            *const libc::c_char,
                            *const libc::c_char,
                            *mut libc::c_void,
                        ) -> bool,
                ),
            };
            init
        },
        {
            let mut init = C2RustUnnamed_4 {
                name: b"connecttimeout\0" as *const u8 as *const libc::c_char,
                place: &mut opt.connect_timeout as *mut libc::c_double
                    as *mut libc::c_void,
                action: Some(
                    cmd_time
                        as unsafe extern "C" fn(
                            *const libc::c_char,
                            *const libc::c_char,
                            *mut libc::c_void,
                        ) -> bool,
                ),
            };
            init
        },
        {
            let mut init = C2RustUnnamed_4 {
                name: b"contentdisposition\0" as *const u8 as *const libc::c_char,
                place: &mut opt.content_disposition as *mut bool as *mut libc::c_void,
                action: Some(
                    cmd_boolean
                        as unsafe extern "C" fn(
                            *const libc::c_char,
                            *const libc::c_char,
                            *mut libc::c_void,
                        ) -> bool,
                ),
            };
            init
        },
        {
            let mut init = C2RustUnnamed_4 {
                name: b"contentonerror\0" as *const u8 as *const libc::c_char,
                place: &mut opt.content_on_error as *mut bool as *mut libc::c_void,
                action: Some(
                    cmd_boolean
                        as unsafe extern "C" fn(
                            *const libc::c_char,
                            *const libc::c_char,
                            *mut libc::c_void,
                        ) -> bool,
                ),
            };
            init
        },
        {
            let mut init = C2RustUnnamed_4 {
                name: b"continue\0" as *const u8 as *const libc::c_char,
                place: &mut opt.always_rest as *mut bool as *mut libc::c_void,
                action: Some(
                    cmd_boolean
                        as unsafe extern "C" fn(
                            *const libc::c_char,
                            *const libc::c_char,
                            *mut libc::c_void,
                        ) -> bool,
                ),
            };
            init
        },
        {
            let mut init = C2RustUnnamed_4 {
                name: b"convertfileonly\0" as *const u8 as *const libc::c_char,
                place: &mut opt.convert_file_only as *mut bool as *mut libc::c_void,
                action: Some(
                    cmd_boolean
                        as unsafe extern "C" fn(
                            *const libc::c_char,
                            *const libc::c_char,
                            *mut libc::c_void,
                        ) -> bool,
                ),
            };
            init
        },
        {
            let mut init = C2RustUnnamed_4 {
                name: b"convertlinks\0" as *const u8 as *const libc::c_char,
                place: &mut opt.convert_links as *mut bool as *mut libc::c_void,
                action: Some(
                    cmd_boolean
                        as unsafe extern "C" fn(
                            *const libc::c_char,
                            *const libc::c_char,
                            *mut libc::c_void,
                        ) -> bool,
                ),
            };
            init
        },
        {
            let mut init = C2RustUnnamed_4 {
                name: b"cookies\0" as *const u8 as *const libc::c_char,
                place: &mut opt.cookies as *mut bool as *mut libc::c_void,
                action: Some(
                    cmd_boolean
                        as unsafe extern "C" fn(
                            *const libc::c_char,
                            *const libc::c_char,
                            *mut libc::c_void,
                        ) -> bool,
                ),
            };
            init
        },
        {
            let mut init = C2RustUnnamed_4 {
                name: b"cutdirs\0" as *const u8 as *const libc::c_char,
                place: &mut opt.cut_dirs as *mut libc::c_int as *mut libc::c_void,
                action: Some(
                    cmd_number
                        as unsafe extern "C" fn(
                            *const libc::c_char,
                            *const libc::c_char,
                            *mut libc::c_void,
                        ) -> bool,
                ),
            };
            init
        },
        {
            let mut init = C2RustUnnamed_4 {
                name: b"debug\0" as *const u8 as *const libc::c_char,
                place: &mut opt.debug as *mut bool as *mut libc::c_void,
                action: Some(
                    cmd_boolean
                        as unsafe extern "C" fn(
                            *const libc::c_char,
                            *const libc::c_char,
                            *mut libc::c_void,
                        ) -> bool,
                ),
            };
            init
        },
        {
            let mut init = C2RustUnnamed_4 {
                name: b"defaultpage\0" as *const u8 as *const libc::c_char,
                place: &mut opt.default_page as *mut *mut libc::c_char
                    as *mut libc::c_void,
                action: Some(
                    cmd_string
                        as unsafe extern "C" fn(
                            *const libc::c_char,
                            *const libc::c_char,
                            *mut libc::c_void,
                        ) -> bool,
                ),
            };
            init
        },
        {
            let mut init = C2RustUnnamed_4 {
                name: b"deleteafter\0" as *const u8 as *const libc::c_char,
                place: &mut opt.delete_after as *mut bool as *mut libc::c_void,
                action: Some(
                    cmd_boolean
                        as unsafe extern "C" fn(
                            *const libc::c_char,
                            *const libc::c_char,
                            *mut libc::c_void,
                        ) -> bool,
                ),
            };
            init
        },
        {
            let mut init = C2RustUnnamed_4 {
                name: b"dirprefix\0" as *const u8 as *const libc::c_char,
                place: &mut opt.dir_prefix as *mut *mut libc::c_char
                    as *mut libc::c_void,
                action: Some(
                    cmd_directory
                        as unsafe extern "C" fn(
                            *const libc::c_char,
                            *const libc::c_char,
                            *mut libc::c_void,
                        ) -> bool,
                ),
            };
            init
        },
        {
            let mut init = C2RustUnnamed_4 {
                name: b"dirstruct\0" as *const u8 as *const libc::c_char,
                place: 0 as *mut libc::c_void,
                action: Some(
                    cmd_spec_dirstruct
                        as unsafe extern "C" fn(
                            *const libc::c_char,
                            *const libc::c_char,
                            *mut libc::c_void,
                        ) -> bool,
                ),
            };
            init
        },
        {
            let mut init = C2RustUnnamed_4 {
                name: b"dnscache\0" as *const u8 as *const libc::c_char,
                place: &mut opt.dns_cache as *mut bool as *mut libc::c_void,
                action: Some(
                    cmd_boolean
                        as unsafe extern "C" fn(
                            *const libc::c_char,
                            *const libc::c_char,
                            *mut libc::c_void,
                        ) -> bool,
                ),
            };
            init
        },
        {
            let mut init = C2RustUnnamed_4 {
                name: b"dnstimeout\0" as *const u8 as *const libc::c_char,
                place: &mut opt.dns_timeout as *mut libc::c_double as *mut libc::c_void,
                action: Some(
                    cmd_time
                        as unsafe extern "C" fn(
                            *const libc::c_char,
                            *const libc::c_char,
                            *mut libc::c_void,
                        ) -> bool,
                ),
            };
            init
        },
        {
            let mut init = C2RustUnnamed_4 {
                name: b"domains\0" as *const u8 as *const libc::c_char,
                place: &mut opt.domains as *mut *mut *mut libc::c_char
                    as *mut libc::c_void,
                action: Some(
                    cmd_vector
                        as unsafe extern "C" fn(
                            *const libc::c_char,
                            *const libc::c_char,
                            *mut libc::c_void,
                        ) -> bool,
                ),
            };
            init
        },
        {
            let mut init = C2RustUnnamed_4 {
                name: b"dotbytes\0" as *const u8 as *const libc::c_char,
                place: &mut opt.dot_bytes as *mut wgint as *mut libc::c_void,
                action: Some(
                    cmd_bytes
                        as unsafe extern "C" fn(
                            *const libc::c_char,
                            *const libc::c_char,
                            *mut libc::c_void,
                        ) -> bool,
                ),
            };
            init
        },
        {
            let mut init = C2RustUnnamed_4 {
                name: b"dotsinline\0" as *const u8 as *const libc::c_char,
                place: &mut opt.dots_in_line as *mut libc::c_int as *mut libc::c_void,
                action: Some(
                    cmd_number
                        as unsafe extern "C" fn(
                            *const libc::c_char,
                            *const libc::c_char,
                            *mut libc::c_void,
                        ) -> bool,
                ),
            };
            init
        },
        {
            let mut init = C2RustUnnamed_4 {
                name: b"dotspacing\0" as *const u8 as *const libc::c_char,
                place: &mut opt.dot_spacing as *mut libc::c_int as *mut libc::c_void,
                action: Some(
                    cmd_number
                        as unsafe extern "C" fn(
                            *const libc::c_char,
                            *const libc::c_char,
                            *mut libc::c_void,
                        ) -> bool,
                ),
            };
            init
        },
        {
            let mut init = C2RustUnnamed_4 {
                name: b"dotstyle\0" as *const u8 as *const libc::c_char,
                place: &mut opt.dot_style as *mut *mut libc::c_char as *mut libc::c_void,
                action: Some(
                    cmd_string
                        as unsafe extern "C" fn(
                            *const libc::c_char,
                            *const libc::c_char,
                            *mut libc::c_void,
                        ) -> bool,
                ),
            };
            init
        },
        {
            let mut init = C2RustUnnamed_4 {
                name: b"excludedirectories\0" as *const u8 as *const libc::c_char,
                place: &mut opt.excludes as *mut *mut *const libc::c_char
                    as *mut libc::c_void,
                action: Some(
                    cmd_directory_vector
                        as unsafe extern "C" fn(
                            *const libc::c_char,
                            *const libc::c_char,
                            *mut libc::c_void,
                        ) -> bool,
                ),
            };
            init
        },
        {
            let mut init = C2RustUnnamed_4 {
                name: b"excludedomains\0" as *const u8 as *const libc::c_char,
                place: &mut opt.exclude_domains as *mut *mut *mut libc::c_char
                    as *mut libc::c_void,
                action: Some(
                    cmd_vector
                        as unsafe extern "C" fn(
                            *const libc::c_char,
                            *const libc::c_char,
                            *mut libc::c_void,
                        ) -> bool,
                ),
            };
            init
        },
        {
            let mut init = C2RustUnnamed_4 {
                name: b"followftp\0" as *const u8 as *const libc::c_char,
                place: &mut opt.follow_ftp as *mut bool as *mut libc::c_void,
                action: Some(
                    cmd_boolean
                        as unsafe extern "C" fn(
                            *const libc::c_char,
                            *const libc::c_char,
                            *mut libc::c_void,
                        ) -> bool,
                ),
            };
            init
        },
        {
            let mut init = C2RustUnnamed_4 {
                name: b"followtags\0" as *const u8 as *const libc::c_char,
                place: &mut opt.follow_tags as *mut *mut *mut libc::c_char
                    as *mut libc::c_void,
                action: Some(
                    cmd_vector
                        as unsafe extern "C" fn(
                            *const libc::c_char,
                            *const libc::c_char,
                            *mut libc::c_void,
                        ) -> bool,
                ),
            };
            init
        },
        {
            let mut init = C2RustUnnamed_4 {
                name: b"forcehtml\0" as *const u8 as *const libc::c_char,
                place: &mut opt.force_html as *mut bool as *mut libc::c_void,
                action: Some(
                    cmd_boolean
                        as unsafe extern "C" fn(
                            *const libc::c_char,
                            *const libc::c_char,
                            *mut libc::c_void,
                        ) -> bool,
                ),
            };
            init
        },
        {
            let mut init = C2RustUnnamed_4 {
                name: b"ftppasswd\0" as *const u8 as *const libc::c_char,
                place: &mut opt.ftp_passwd as *mut *mut libc::c_char
                    as *mut libc::c_void,
                action: Some(
                    cmd_string
                        as unsafe extern "C" fn(
                            *const libc::c_char,
                            *const libc::c_char,
                            *mut libc::c_void,
                        ) -> bool,
                ),
            };
            init
        },
        {
            let mut init = C2RustUnnamed_4 {
                name: b"ftppassword\0" as *const u8 as *const libc::c_char,
                place: &mut opt.ftp_passwd as *mut *mut libc::c_char
                    as *mut libc::c_void,
                action: Some(
                    cmd_string
                        as unsafe extern "C" fn(
                            *const libc::c_char,
                            *const libc::c_char,
                            *mut libc::c_void,
                        ) -> bool,
                ),
            };
            init
        },
        {
            let mut init = C2RustUnnamed_4 {
                name: b"ftpproxy\0" as *const u8 as *const libc::c_char,
                place: &mut opt.ftp_proxy as *mut *mut libc::c_char as *mut libc::c_void,
                action: Some(
                    cmd_string
                        as unsafe extern "C" fn(
                            *const libc::c_char,
                            *const libc::c_char,
                            *mut libc::c_void,
                        ) -> bool,
                ),
            };
            init
        },
        {
            let mut init = C2RustUnnamed_4 {
                name: b"ftpuser\0" as *const u8 as *const libc::c_char,
                place: &mut opt.ftp_user as *mut *mut libc::c_char as *mut libc::c_void,
                action: Some(
                    cmd_string
                        as unsafe extern "C" fn(
                            *const libc::c_char,
                            *const libc::c_char,
                            *mut libc::c_void,
                        ) -> bool,
                ),
            };
            init
        },
        {
            let mut init = C2RustUnnamed_4 {
                name: b"glob\0" as *const u8 as *const libc::c_char,
                place: &mut opt.ftp_glob as *mut bool as *mut libc::c_void,
                action: Some(
                    cmd_boolean
                        as unsafe extern "C" fn(
                            *const libc::c_char,
                            *const libc::c_char,
                            *mut libc::c_void,
                        ) -> bool,
                ),
            };
            init
        },
        {
            let mut init = C2RustUnnamed_4 {
                name: b"header\0" as *const u8 as *const libc::c_char,
                place: 0 as *mut libc::c_void,
                action: Some(
                    cmd_spec_header
                        as unsafe extern "C" fn(
                            *const libc::c_char,
                            *const libc::c_char,
                            *mut libc::c_void,
                        ) -> bool,
                ),
            };
            init
        },
        {
            let mut init = C2RustUnnamed_4 {
                name: b"htmlextension\0" as *const u8 as *const libc::c_char,
                place: &mut opt.adjust_extension as *mut bool as *mut libc::c_void,
                action: Some(
                    cmd_boolean
                        as unsafe extern "C" fn(
                            *const libc::c_char,
                            *const libc::c_char,
                            *mut libc::c_void,
                        ) -> bool,
                ),
            };
            init
        },
        {
            let mut init = C2RustUnnamed_4 {
                name: b"htmlify\0" as *const u8 as *const libc::c_char,
                place: 0 as *mut libc::c_void,
                action: Some(
                    cmd_spec_htmlify
                        as unsafe extern "C" fn(
                            *const libc::c_char,
                            *const libc::c_char,
                            *mut libc::c_void,
                        ) -> bool,
                ),
            };
            init
        },
        {
            let mut init = C2RustUnnamed_4 {
                name: b"httpkeepalive\0" as *const u8 as *const libc::c_char,
                place: &mut opt.http_keep_alive as *mut bool as *mut libc::c_void,
                action: Some(
                    cmd_boolean
                        as unsafe extern "C" fn(
                            *const libc::c_char,
                            *const libc::c_char,
                            *mut libc::c_void,
                        ) -> bool,
                ),
            };
            init
        },
        {
            let mut init = C2RustUnnamed_4 {
                name: b"httppasswd\0" as *const u8 as *const libc::c_char,
                place: &mut opt.http_passwd as *mut *mut libc::c_char
                    as *mut libc::c_void,
                action: Some(
                    cmd_string
                        as unsafe extern "C" fn(
                            *const libc::c_char,
                            *const libc::c_char,
                            *mut libc::c_void,
                        ) -> bool,
                ),
            };
            init
        },
        {
            let mut init = C2RustUnnamed_4 {
                name: b"httppassword\0" as *const u8 as *const libc::c_char,
                place: &mut opt.http_passwd as *mut *mut libc::c_char
                    as *mut libc::c_void,
                action: Some(
                    cmd_string
                        as unsafe extern "C" fn(
                            *const libc::c_char,
                            *const libc::c_char,
                            *mut libc::c_void,
                        ) -> bool,
                ),
            };
            init
        },
        {
            let mut init = C2RustUnnamed_4 {
                name: b"httpproxy\0" as *const u8 as *const libc::c_char,
                place: &mut opt.http_proxy as *mut *mut libc::c_char
                    as *mut libc::c_void,
                action: Some(
                    cmd_string
                        as unsafe extern "C" fn(
                            *const libc::c_char,
                            *const libc::c_char,
                            *mut libc::c_void,
                        ) -> bool,
                ),
            };
            init
        },
        {
            let mut init = C2RustUnnamed_4 {
                name: b"httpsproxy\0" as *const u8 as *const libc::c_char,
                place: &mut opt.https_proxy as *mut *mut libc::c_char
                    as *mut libc::c_void,
                action: Some(
                    cmd_string
                        as unsafe extern "C" fn(
                            *const libc::c_char,
                            *const libc::c_char,
                            *mut libc::c_void,
                        ) -> bool,
                ),
            };
            init
        },
        {
            let mut init = C2RustUnnamed_4 {
                name: b"httpuser\0" as *const u8 as *const libc::c_char,
                place: &mut opt.http_user as *mut *mut libc::c_char as *mut libc::c_void,
                action: Some(
                    cmd_string
                        as unsafe extern "C" fn(
                            *const libc::c_char,
                            *const libc::c_char,
                            *mut libc::c_void,
                        ) -> bool,
                ),
            };
            init
        },
        {
            let mut init = C2RustUnnamed_4 {
                name: b"ifmodifiedsince\0" as *const u8 as *const libc::c_char,
                place: &mut opt.if_modified_since as *mut bool as *mut libc::c_void,
                action: Some(
                    cmd_boolean
                        as unsafe extern "C" fn(
                            *const libc::c_char,
                            *const libc::c_char,
                            *mut libc::c_void,
                        ) -> bool,
                ),
            };
            init
        },
        {
            let mut init = C2RustUnnamed_4 {
                name: b"ignorecase\0" as *const u8 as *const libc::c_char,
                place: &mut opt.ignore_case as *mut bool as *mut libc::c_void,
                action: Some(
                    cmd_boolean
                        as unsafe extern "C" fn(
                            *const libc::c_char,
                            *const libc::c_char,
                            *mut libc::c_void,
                        ) -> bool,
                ),
            };
            init
        },
        {
            let mut init = C2RustUnnamed_4 {
                name: b"ignorelength\0" as *const u8 as *const libc::c_char,
                place: &mut opt.ignore_length as *mut bool as *mut libc::c_void,
                action: Some(
                    cmd_boolean
                        as unsafe extern "C" fn(
                            *const libc::c_char,
                            *const libc::c_char,
                            *mut libc::c_void,
                        ) -> bool,
                ),
            };
            init
        },
        {
            let mut init = C2RustUnnamed_4 {
                name: b"ignoretags\0" as *const u8 as *const libc::c_char,
                place: &mut opt.ignore_tags as *mut *mut *mut libc::c_char
                    as *mut libc::c_void,
                action: Some(
                    cmd_vector
                        as unsafe extern "C" fn(
                            *const libc::c_char,
                            *const libc::c_char,
                            *mut libc::c_void,
                        ) -> bool,
                ),
            };
            init
        },
        {
            let mut init = C2RustUnnamed_4 {
                name: b"includedirectories\0" as *const u8 as *const libc::c_char,
                place: &mut opt.includes as *mut *mut *const libc::c_char
                    as *mut libc::c_void,
                action: Some(
                    cmd_directory_vector
                        as unsafe extern "C" fn(
                            *const libc::c_char,
                            *const libc::c_char,
                            *mut libc::c_void,
                        ) -> bool,
                ),
            };
            init
        },
        {
            let mut init = C2RustUnnamed_4 {
                name: b"inet4only\0" as *const u8 as *const libc::c_char,
                place: &mut opt.ipv4_only as *mut bool as *mut libc::c_void,
                action: Some(
                    cmd_boolean
                        as unsafe extern "C" fn(
                            *const libc::c_char,
                            *const libc::c_char,
                            *mut libc::c_void,
                        ) -> bool,
                ),
            };
            init
        },
        {
            let mut init = C2RustUnnamed_4 {
                name: b"inet6only\0" as *const u8 as *const libc::c_char,
                place: &mut opt.ipv6_only as *mut bool as *mut libc::c_void,
                action: Some(
                    cmd_boolean
                        as unsafe extern "C" fn(
                            *const libc::c_char,
                            *const libc::c_char,
                            *mut libc::c_void,
                        ) -> bool,
                ),
            };
            init
        },
        {
            let mut init = C2RustUnnamed_4 {
                name: b"input\0" as *const u8 as *const libc::c_char,
                place: &mut opt.input_filename as *mut *mut libc::c_char
                    as *mut libc::c_void,
                action: Some(
                    cmd_file
                        as unsafe extern "C" fn(
                            *const libc::c_char,
                            *const libc::c_char,
                            *mut libc::c_void,
                        ) -> bool,
                ),
            };
            init
        },
        {
            let mut init = C2RustUnnamed_4 {
                name: b"iri\0" as *const u8 as *const libc::c_char,
                place: &mut opt.enable_iri as *mut bool as *mut libc::c_void,
                action: Some(
                    cmd_boolean
                        as unsafe extern "C" fn(
                            *const libc::c_char,
                            *const libc::c_char,
                            *mut libc::c_void,
                        ) -> bool,
                ),
            };
            init
        },
        {
            let mut init = C2RustUnnamed_4 {
                name: b"keepbadhash\0" as *const u8 as *const libc::c_char,
                place: &mut opt.keep_badhash as *mut bool as *mut libc::c_void,
                action: Some(
                    cmd_boolean
                        as unsafe extern "C" fn(
                            *const libc::c_char,
                            *const libc::c_char,
                            *mut libc::c_void,
                        ) -> bool,
                ),
            };
            init
        },
        {
            let mut init = C2RustUnnamed_4 {
                name: b"keepsessioncookies\0" as *const u8 as *const libc::c_char,
                place: &mut opt.keep_session_cookies as *mut bool as *mut libc::c_void,
                action: Some(
                    cmd_boolean
                        as unsafe extern "C" fn(
                            *const libc::c_char,
                            *const libc::c_char,
                            *mut libc::c_void,
                        ) -> bool,
                ),
            };
            init
        },
        {
            let mut init = C2RustUnnamed_4 {
                name: b"limitrate\0" as *const u8 as *const libc::c_char,
                place: &mut opt.limit_rate as *mut wgint as *mut libc::c_void,
                action: Some(
                    cmd_bytes
                        as unsafe extern "C" fn(
                            *const libc::c_char,
                            *const libc::c_char,
                            *mut libc::c_void,
                        ) -> bool,
                ),
            };
            init
        },
        {
            let mut init = C2RustUnnamed_4 {
                name: b"loadcookies\0" as *const u8 as *const libc::c_char,
                place: &mut opt.cookies_input as *mut *mut libc::c_char
                    as *mut libc::c_void,
                action: Some(
                    cmd_file
                        as unsafe extern "C" fn(
                            *const libc::c_char,
                            *const libc::c_char,
                            *mut libc::c_void,
                        ) -> bool,
                ),
            };
            init
        },
        {
            let mut init = C2RustUnnamed_4 {
                name: b"localencoding\0" as *const u8 as *const libc::c_char,
                place: &mut opt.locale as *mut *const libc::c_char as *mut libc::c_void,
                action: Some(
                    cmd_string
                        as unsafe extern "C" fn(
                            *const libc::c_char,
                            *const libc::c_char,
                            *mut libc::c_void,
                        ) -> bool,
                ),
            };
            init
        },
        {
            let mut init = C2RustUnnamed_4 {
                name: b"logfile\0" as *const u8 as *const libc::c_char,
                place: &mut opt.lfilename as *mut *mut libc::c_char as *mut libc::c_void,
                action: Some(
                    cmd_file
                        as unsafe extern "C" fn(
                            *const libc::c_char,
                            *const libc::c_char,
                            *mut libc::c_void,
                        ) -> bool,
                ),
            };
            init
        },
        {
            let mut init = C2RustUnnamed_4 {
                name: b"login\0" as *const u8 as *const libc::c_char,
                place: &mut opt.ftp_user as *mut *mut libc::c_char as *mut libc::c_void,
                action: Some(
                    cmd_string
                        as unsafe extern "C" fn(
                            *const libc::c_char,
                            *const libc::c_char,
                            *mut libc::c_void,
                        ) -> bool,
                ),
            };
            init
        },
        {
            let mut init = C2RustUnnamed_4 {
                name: b"maxredirect\0" as *const u8 as *const libc::c_char,
                place: &mut opt.max_redirect as *mut libc::c_int as *mut libc::c_void,
                action: Some(
                    cmd_number
                        as unsafe extern "C" fn(
                            *const libc::c_char,
                            *const libc::c_char,
                            *mut libc::c_void,
                        ) -> bool,
                ),
            };
            init
        },
        {
            let mut init = C2RustUnnamed_4 {
                name: b"method\0" as *const u8 as *const libc::c_char,
                place: &mut opt.method as *mut *mut libc::c_char as *mut libc::c_void,
                action: Some(
                    cmd_string_uppercase
                        as unsafe extern "C" fn(
                            *const libc::c_char,
                            *const libc::c_char,
                            *mut libc::c_void,
                        ) -> bool,
                ),
            };
            init
        },
        {
            let mut init = C2RustUnnamed_4 {
                name: b"mirror\0" as *const u8 as *const libc::c_char,
                place: 0 as *mut libc::c_void,
                action: Some(
                    cmd_spec_mirror
                        as unsafe extern "C" fn(
                            *const libc::c_char,
                            *const libc::c_char,
                            *mut libc::c_void,
                        ) -> bool,
                ),
            };
            init
        },
        {
            let mut init = C2RustUnnamed_4 {
                name: b"netrc\0" as *const u8 as *const libc::c_char,
                place: &mut opt.netrc as *mut bool as *mut libc::c_void,
                action: Some(
                    cmd_boolean
                        as unsafe extern "C" fn(
                            *const libc::c_char,
                            *const libc::c_char,
                            *mut libc::c_void,
                        ) -> bool,
                ),
            };
            init
        },
        {
            let mut init = C2RustUnnamed_4 {
                name: b"noclobber\0" as *const u8 as *const libc::c_char,
                place: &mut opt.noclobber as *mut bool as *mut libc::c_void,
                action: Some(
                    cmd_boolean
                        as unsafe extern "C" fn(
                            *const libc::c_char,
                            *const libc::c_char,
                            *mut libc::c_void,
                        ) -> bool,
                ),
            };
            init
        },
        {
            let mut init = C2RustUnnamed_4 {
                name: b"noconfig\0" as *const u8 as *const libc::c_char,
                place: &mut opt.noconfig as *mut bool as *mut libc::c_void,
                action: Some(
                    cmd_boolean
                        as unsafe extern "C" fn(
                            *const libc::c_char,
                            *const libc::c_char,
                            *mut libc::c_void,
                        ) -> bool,
                ),
            };
            init
        },
        {
            let mut init = C2RustUnnamed_4 {
                name: b"noparent\0" as *const u8 as *const libc::c_char,
                place: &mut opt.no_parent as *mut bool as *mut libc::c_void,
                action: Some(
                    cmd_boolean
                        as unsafe extern "C" fn(
                            *const libc::c_char,
                            *const libc::c_char,
                            *mut libc::c_void,
                        ) -> bool,
                ),
            };
            init
        },
        {
            let mut init = C2RustUnnamed_4 {
                name: b"noproxy\0" as *const u8 as *const libc::c_char,
                place: &mut opt.no_proxy as *mut *mut *mut libc::c_char
                    as *mut libc::c_void,
                action: Some(
                    cmd_vector
                        as unsafe extern "C" fn(
                            *const libc::c_char,
                            *const libc::c_char,
                            *mut libc::c_void,
                        ) -> bool,
                ),
            };
            init
        },
        {
            let mut init = C2RustUnnamed_4 {
                name: b"numtries\0" as *const u8 as *const libc::c_char,
                place: &mut opt.ntry as *mut libc::c_int as *mut libc::c_void,
                action: Some(
                    cmd_number_inf
                        as unsafe extern "C" fn(
                            *const libc::c_char,
                            *const libc::c_char,
                            *mut libc::c_void,
                        ) -> bool,
                ),
            };
            init
        },
        {
            let mut init = C2RustUnnamed_4 {
                name: b"outputdocument\0" as *const u8 as *const libc::c_char,
                place: &mut opt.output_document as *mut *mut libc::c_char
                    as *mut libc::c_void,
                action: Some(
                    cmd_file
                        as unsafe extern "C" fn(
                            *const libc::c_char,
                            *const libc::c_char,
                            *mut libc::c_void,
                        ) -> bool,
                ),
            };
            init
        },
        {
            let mut init = C2RustUnnamed_4 {
                name: b"pagerequisites\0" as *const u8 as *const libc::c_char,
                place: &mut opt.page_requisites as *mut bool as *mut libc::c_void,
                action: Some(
                    cmd_boolean
                        as unsafe extern "C" fn(
                            *const libc::c_char,
                            *const libc::c_char,
                            *mut libc::c_void,
                        ) -> bool,
                ),
            };
            init
        },
        {
            let mut init = C2RustUnnamed_4 {
                name: b"passiveftp\0" as *const u8 as *const libc::c_char,
                place: &mut opt.ftp_pasv as *mut bool as *mut libc::c_void,
                action: Some(
                    cmd_boolean
                        as unsafe extern "C" fn(
                            *const libc::c_char,
                            *const libc::c_char,
                            *mut libc::c_void,
                        ) -> bool,
                ),
            };
            init
        },
        {
            let mut init = C2RustUnnamed_4 {
                name: b"passwd\0" as *const u8 as *const libc::c_char,
                place: &mut opt.ftp_passwd as *mut *mut libc::c_char
                    as *mut libc::c_void,
                action: Some(
                    cmd_string
                        as unsafe extern "C" fn(
                            *const libc::c_char,
                            *const libc::c_char,
                            *mut libc::c_void,
                        ) -> bool,
                ),
            };
            init
        },
        {
            let mut init = C2RustUnnamed_4 {
                name: b"password\0" as *const u8 as *const libc::c_char,
                place: &mut opt.passwd as *mut *mut libc::c_char as *mut libc::c_void,
                action: Some(
                    cmd_string
                        as unsafe extern "C" fn(
                            *const libc::c_char,
                            *const libc::c_char,
                            *mut libc::c_void,
                        ) -> bool,
                ),
            };
            init
        },
        {
            let mut init = C2RustUnnamed_4 {
                name: b"postdata\0" as *const u8 as *const libc::c_char,
                place: &mut opt.post_data as *mut *mut libc::c_char as *mut libc::c_void,
                action: Some(
                    cmd_string
                        as unsafe extern "C" fn(
                            *const libc::c_char,
                            *const libc::c_char,
                            *mut libc::c_void,
                        ) -> bool,
                ),
            };
            init
        },
        {
            let mut init = C2RustUnnamed_4 {
                name: b"postfile\0" as *const u8 as *const libc::c_char,
                place: &mut opt.post_file_name as *mut *mut libc::c_char
                    as *mut libc::c_void,
                action: Some(
                    cmd_file
                        as unsafe extern "C" fn(
                            *const libc::c_char,
                            *const libc::c_char,
                            *mut libc::c_void,
                        ) -> bool,
                ),
            };
            init
        },
        {
            let mut init = C2RustUnnamed_4 {
                name: b"preferfamily\0" as *const u8 as *const libc::c_char,
                place: 0 as *mut libc::c_void,
                action: Some(
                    cmd_spec_prefer_family
                        as unsafe extern "C" fn(
                            *const libc::c_char,
                            *const libc::c_char,
                            *mut libc::c_void,
                        ) -> bool,
                ),
            };
            init
        },
        {
            let mut init = C2RustUnnamed_4 {
                name: b"preservepermissions\0" as *const u8 as *const libc::c_char,
                place: &mut opt.preserve_perm as *mut bool as *mut libc::c_void,
                action: Some(
                    cmd_boolean
                        as unsafe extern "C" fn(
                            *const libc::c_char,
                            *const libc::c_char,
                            *mut libc::c_void,
                        ) -> bool,
                ),
            };
            init
        },
        {
            let mut init = C2RustUnnamed_4 {
                name: b"progress\0" as *const u8 as *const libc::c_char,
                place: &mut opt.progress_type as *mut *mut libc::c_char
                    as *mut libc::c_void,
                action: Some(
                    cmd_spec_progress
                        as unsafe extern "C" fn(
                            *const libc::c_char,
                            *const libc::c_char,
                            *mut libc::c_void,
                        ) -> bool,
                ),
            };
            init
        },
        {
            let mut init = C2RustUnnamed_4 {
                name: b"protocoldirectories\0" as *const u8 as *const libc::c_char,
                place: &mut opt.protocol_directories as *mut bool as *mut libc::c_void,
                action: Some(
                    cmd_boolean
                        as unsafe extern "C" fn(
                            *const libc::c_char,
                            *const libc::c_char,
                            *mut libc::c_void,
                        ) -> bool,
                ),
            };
            init
        },
        {
            let mut init = C2RustUnnamed_4 {
                name: b"proxypasswd\0" as *const u8 as *const libc::c_char,
                place: &mut opt.proxy_passwd as *mut *mut libc::c_char
                    as *mut libc::c_void,
                action: Some(
                    cmd_string
                        as unsafe extern "C" fn(
                            *const libc::c_char,
                            *const libc::c_char,
                            *mut libc::c_void,
                        ) -> bool,
                ),
            };
            init
        },
        {
            let mut init = C2RustUnnamed_4 {
                name: b"proxypassword\0" as *const u8 as *const libc::c_char,
                place: &mut opt.proxy_passwd as *mut *mut libc::c_char
                    as *mut libc::c_void,
                action: Some(
                    cmd_string
                        as unsafe extern "C" fn(
                            *const libc::c_char,
                            *const libc::c_char,
                            *mut libc::c_void,
                        ) -> bool,
                ),
            };
            init
        },
        {
            let mut init = C2RustUnnamed_4 {
                name: b"proxyuser\0" as *const u8 as *const libc::c_char,
                place: &mut opt.proxy_user as *mut *mut libc::c_char
                    as *mut libc::c_void,
                action: Some(
                    cmd_string
                        as unsafe extern "C" fn(
                            *const libc::c_char,
                            *const libc::c_char,
                            *mut libc::c_void,
                        ) -> bool,
                ),
            };
            init
        },
        {
            let mut init = C2RustUnnamed_4 {
                name: b"quiet\0" as *const u8 as *const libc::c_char,
                place: &mut opt.quiet as *mut bool as *mut libc::c_void,
                action: Some(
                    cmd_boolean
                        as unsafe extern "C" fn(
                            *const libc::c_char,
                            *const libc::c_char,
                            *mut libc::c_void,
                        ) -> bool,
                ),
            };
            init
        },
        {
            let mut init = C2RustUnnamed_4 {
                name: b"quota\0" as *const u8 as *const libc::c_char,
                place: &mut opt.quota as *mut wgint as *mut libc::c_void,
                action: Some(
                    cmd_bytes_sum
                        as unsafe extern "C" fn(
                            *const libc::c_char,
                            *const libc::c_char,
                            *mut libc::c_void,
                        ) -> bool,
                ),
            };
            init
        },
        {
            let mut init = C2RustUnnamed_4 {
                name: b"randomwait\0" as *const u8 as *const libc::c_char,
                place: &mut opt.random_wait as *mut bool as *mut libc::c_void,
                action: Some(
                    cmd_boolean
                        as unsafe extern "C" fn(
                            *const libc::c_char,
                            *const libc::c_char,
                            *mut libc::c_void,
                        ) -> bool,
                ),
            };
            init
        },
        {
            let mut init = C2RustUnnamed_4 {
                name: b"readtimeout\0" as *const u8 as *const libc::c_char,
                place: &mut opt.read_timeout as *mut libc::c_double as *mut libc::c_void,
                action: Some(
                    cmd_time
                        as unsafe extern "C" fn(
                            *const libc::c_char,
                            *const libc::c_char,
                            *mut libc::c_void,
                        ) -> bool,
                ),
            };
            init
        },
        {
            let mut init = C2RustUnnamed_4 {
                name: b"reclevel\0" as *const u8 as *const libc::c_char,
                place: &mut opt.reclevel as *mut libc::c_int as *mut libc::c_void,
                action: Some(
                    cmd_number_inf
                        as unsafe extern "C" fn(
                            *const libc::c_char,
                            *const libc::c_char,
                            *mut libc::c_void,
                        ) -> bool,
                ),
            };
            init
        },
        {
            let mut init = C2RustUnnamed_4 {
                name: b"recursive\0" as *const u8 as *const libc::c_char,
                place: 0 as *mut libc::c_void,
                action: Some(
                    cmd_spec_recursive
                        as unsafe extern "C" fn(
                            *const libc::c_char,
                            *const libc::c_char,
                            *mut libc::c_void,
                        ) -> bool,
                ),
            };
            init
        },
        {
            let mut init = C2RustUnnamed_4 {
                name: b"referer\0" as *const u8 as *const libc::c_char,
                place: &mut opt.referer as *mut *mut libc::c_char as *mut libc::c_void,
                action: Some(
                    cmd_string
                        as unsafe extern "C" fn(
                            *const libc::c_char,
                            *const libc::c_char,
                            *mut libc::c_void,
                        ) -> bool,
                ),
            };
            init
        },
        {
            let mut init = C2RustUnnamed_4 {
                name: b"regextype\0" as *const u8 as *const libc::c_char,
                place: &mut opt.regex_type as *mut C2RustUnnamed_2 as *mut libc::c_void,
                action: Some(
                    cmd_spec_regex_type
                        as unsafe extern "C" fn(
                            *const libc::c_char,
                            *const libc::c_char,
                            *mut libc::c_void,
                        ) -> bool,
                ),
            };
            init
        },
        {
            let mut init = C2RustUnnamed_4 {
                name: b"reject\0" as *const u8 as *const libc::c_char,
                place: &mut opt.rejects as *mut *mut *mut libc::c_char
                    as *mut libc::c_void,
                action: Some(
                    cmd_vector
                        as unsafe extern "C" fn(
                            *const libc::c_char,
                            *const libc::c_char,
                            *mut libc::c_void,
                        ) -> bool,
                ),
            };
            init
        },
        {
            let mut init = C2RustUnnamed_4 {
                name: b"rejectedlog\0" as *const u8 as *const libc::c_char,
                place: &mut opt.rejected_log as *mut *mut libc::c_char
                    as *mut libc::c_void,
                action: Some(
                    cmd_file
                        as unsafe extern "C" fn(
                            *const libc::c_char,
                            *const libc::c_char,
                            *mut libc::c_void,
                        ) -> bool,
                ),
            };
            init
        },
        {
            let mut init = C2RustUnnamed_4 {
                name: b"rejectregex\0" as *const u8 as *const libc::c_char,
                place: &mut opt.rejectregex_s as *mut *mut libc::c_char
                    as *mut libc::c_void,
                action: Some(
                    cmd_string
                        as unsafe extern "C" fn(
                            *const libc::c_char,
                            *const libc::c_char,
                            *mut libc::c_void,
                        ) -> bool,
                ),
            };
            init
        },
        {
            let mut init = C2RustUnnamed_4 {
                name: b"relativeonly\0" as *const u8 as *const libc::c_char,
                place: &mut opt.relative_only as *mut bool as *mut libc::c_void,
                action: Some(
                    cmd_boolean
                        as unsafe extern "C" fn(
                            *const libc::c_char,
                            *const libc::c_char,
                            *mut libc::c_void,
                        ) -> bool,
                ),
            };
            init
        },
        {
            let mut init = C2RustUnnamed_4 {
                name: b"remoteencoding\0" as *const u8 as *const libc::c_char,
                place: &mut opt.encoding_remote as *mut *mut libc::c_char
                    as *mut libc::c_void,
                action: Some(
                    cmd_string
                        as unsafe extern "C" fn(
                            *const libc::c_char,
                            *const libc::c_char,
                            *mut libc::c_void,
                        ) -> bool,
                ),
            };
            init
        },
        {
            let mut init = C2RustUnnamed_4 {
                name: b"removelisting\0" as *const u8 as *const libc::c_char,
                place: &mut opt.remove_listing as *mut bool as *mut libc::c_void,
                action: Some(
                    cmd_boolean
                        as unsafe extern "C" fn(
                            *const libc::c_char,
                            *const libc::c_char,
                            *mut libc::c_void,
                        ) -> bool,
                ),
            };
            init
        },
        {
            let mut init = C2RustUnnamed_4 {
                name: b"reportspeed\0" as *const u8 as *const libc::c_char,
                place: &mut opt.report_bps as *mut bool as *mut libc::c_void,
                action: Some(
                    cmd_spec_report_speed
                        as unsafe extern "C" fn(
                            *const libc::c_char,
                            *const libc::c_char,
                            *mut libc::c_void,
                        ) -> bool,
                ),
            };
            init
        },
        {
            let mut init = C2RustUnnamed_4 {
                name: b"restrictfilenames\0" as *const u8 as *const libc::c_char,
                place: 0 as *mut libc::c_void,
                action: Some(
                    cmd_spec_restrict_file_names
                        as unsafe extern "C" fn(
                            *const libc::c_char,
                            *const libc::c_char,
                            *mut libc::c_void,
                        ) -> bool,
                ),
            };
            init
        },
        {
            let mut init = C2RustUnnamed_4 {
                name: b"retrsymlinks\0" as *const u8 as *const libc::c_char,
                place: &mut opt.retr_symlinks as *mut bool as *mut libc::c_void,
                action: Some(
                    cmd_boolean
                        as unsafe extern "C" fn(
                            *const libc::c_char,
                            *const libc::c_char,
                            *mut libc::c_void,
                        ) -> bool,
                ),
            };
            init
        },
        {
            let mut init = C2RustUnnamed_4 {
                name: b"retryconnrefused\0" as *const u8 as *const libc::c_char,
                place: &mut opt.retry_connrefused as *mut bool as *mut libc::c_void,
                action: Some(
                    cmd_boolean
                        as unsafe extern "C" fn(
                            *const libc::c_char,
                            *const libc::c_char,
                            *mut libc::c_void,
                        ) -> bool,
                ),
            };
            init
        },
        {
            let mut init = C2RustUnnamed_4 {
                name: b"retryonhosterror\0" as *const u8 as *const libc::c_char,
                place: &mut opt.retry_on_host_error as *mut bool as *mut libc::c_void,
                action: Some(
                    cmd_boolean
                        as unsafe extern "C" fn(
                            *const libc::c_char,
                            *const libc::c_char,
                            *mut libc::c_void,
                        ) -> bool,
                ),
            };
            init
        },
        {
            let mut init = C2RustUnnamed_4 {
                name: b"retryonhttperror\0" as *const u8 as *const libc::c_char,
                place: &mut opt.retry_on_http_error as *mut *mut libc::c_char
                    as *mut libc::c_void,
                action: Some(
                    cmd_string
                        as unsafe extern "C" fn(
                            *const libc::c_char,
                            *const libc::c_char,
                            *mut libc::c_void,
                        ) -> bool,
                ),
            };
            init
        },
        {
            let mut init = C2RustUnnamed_4 {
                name: b"robots\0" as *const u8 as *const libc::c_char,
                place: &mut opt.use_robots as *mut bool as *mut libc::c_void,
                action: Some(
                    cmd_boolean
                        as unsafe extern "C" fn(
                            *const libc::c_char,
                            *const libc::c_char,
                            *mut libc::c_void,
                        ) -> bool,
                ),
            };
            init
        },
        {
            let mut init = C2RustUnnamed_4 {
                name: b"savecookies\0" as *const u8 as *const libc::c_char,
                place: &mut opt.cookies_output as *mut *mut libc::c_char
                    as *mut libc::c_void,
                action: Some(
                    cmd_file
                        as unsafe extern "C" fn(
                            *const libc::c_char,
                            *const libc::c_char,
                            *mut libc::c_void,
                        ) -> bool,
                ),
            };
            init
        },
        {
            let mut init = C2RustUnnamed_4 {
                name: b"saveheaders\0" as *const u8 as *const libc::c_char,
                place: &mut opt.save_headers as *mut bool as *mut libc::c_void,
                action: Some(
                    cmd_boolean
                        as unsafe extern "C" fn(
                            *const libc::c_char,
                            *const libc::c_char,
                            *mut libc::c_void,
                        ) -> bool,
                ),
            };
            init
        },
        {
            let mut init = C2RustUnnamed_4 {
                name: b"serverresponse\0" as *const u8 as *const libc::c_char,
                place: &mut opt.server_response as *mut bool as *mut libc::c_void,
                action: Some(
                    cmd_boolean
                        as unsafe extern "C" fn(
                            *const libc::c_char,
                            *const libc::c_char,
                            *mut libc::c_void,
                        ) -> bool,
                ),
            };
            init
        },
        {
            let mut init = C2RustUnnamed_4 {
                name: b"showalldnsentries\0" as *const u8 as *const libc::c_char,
                place: &mut opt.show_all_dns_entries as *mut bool as *mut libc::c_void,
                action: Some(
                    cmd_boolean
                        as unsafe extern "C" fn(
                            *const libc::c_char,
                            *const libc::c_char,
                            *mut libc::c_void,
                        ) -> bool,
                ),
            };
            init
        },
        {
            let mut init = C2RustUnnamed_4 {
                name: b"showprogress\0" as *const u8 as *const libc::c_char,
                place: &mut opt.show_progress as *mut libc::c_int as *mut libc::c_void,
                action: Some(
                    cmd_spec_progressdisp
                        as unsafe extern "C" fn(
                            *const libc::c_char,
                            *const libc::c_char,
                            *mut libc::c_void,
                        ) -> bool,
                ),
            };
            init
        },
        {
            let mut init = C2RustUnnamed_4 {
                name: b"spanhosts\0" as *const u8 as *const libc::c_char,
                place: &mut opt.spanhost as *mut bool as *mut libc::c_void,
                action: Some(
                    cmd_boolean
                        as unsafe extern "C" fn(
                            *const libc::c_char,
                            *const libc::c_char,
                            *mut libc::c_void,
                        ) -> bool,
                ),
            };
            init
        },
        {
            let mut init = C2RustUnnamed_4 {
                name: b"spider\0" as *const u8 as *const libc::c_char,
                place: &mut opt.spider as *mut bool as *mut libc::c_void,
                action: Some(
                    cmd_boolean
                        as unsafe extern "C" fn(
                            *const libc::c_char,
                            *const libc::c_char,
                            *mut libc::c_void,
                        ) -> bool,
                ),
            };
            init
        },
        {
            let mut init = C2RustUnnamed_4 {
                name: b"startpos\0" as *const u8 as *const libc::c_char,
                place: &mut opt.start_pos as *mut wgint as *mut libc::c_void,
                action: Some(
                    cmd_bytes
                        as unsafe extern "C" fn(
                            *const libc::c_char,
                            *const libc::c_char,
                            *mut libc::c_void,
                        ) -> bool,
                ),
            };
            init
        },
        {
            let mut init = C2RustUnnamed_4 {
                name: b"strictcomments\0" as *const u8 as *const libc::c_char,
                place: &mut opt.strict_comments as *mut bool as *mut libc::c_void,
                action: Some(
                    cmd_boolean
                        as unsafe extern "C" fn(
                            *const libc::c_char,
                            *const libc::c_char,
                            *mut libc::c_void,
                        ) -> bool,
                ),
            };
            init
        },
        {
            let mut init = C2RustUnnamed_4 {
                name: b"timeout\0" as *const u8 as *const libc::c_char,
                place: 0 as *mut libc::c_void,
                action: Some(
                    cmd_spec_timeout
                        as unsafe extern "C" fn(
                            *const libc::c_char,
                            *const libc::c_char,
                            *mut libc::c_void,
                        ) -> bool,
                ),
            };
            init
        },
        {
            let mut init = C2RustUnnamed_4 {
                name: b"timestamping\0" as *const u8 as *const libc::c_char,
                place: &mut opt.timestamping as *mut bool as *mut libc::c_void,
                action: Some(
                    cmd_boolean
                        as unsafe extern "C" fn(
                            *const libc::c_char,
                            *const libc::c_char,
                            *mut libc::c_void,
                        ) -> bool,
                ),
            };
            init
        },
        {
            let mut init = C2RustUnnamed_4 {
                name: b"tries\0" as *const u8 as *const libc::c_char,
                place: &mut opt.ntry as *mut libc::c_int as *mut libc::c_void,
                action: Some(
                    cmd_number_inf
                        as unsafe extern "C" fn(
                            *const libc::c_char,
                            *const libc::c_char,
                            *mut libc::c_void,
                        ) -> bool,
                ),
            };
            init
        },
        {
            let mut init = C2RustUnnamed_4 {
                name: b"trustservernames\0" as *const u8 as *const libc::c_char,
                place: &mut opt.trustservernames as *mut bool as *mut libc::c_void,
                action: Some(
                    cmd_boolean
                        as unsafe extern "C" fn(
                            *const libc::c_char,
                            *const libc::c_char,
                            *mut libc::c_void,
                        ) -> bool,
                ),
            };
            init
        },
        {
            let mut init = C2RustUnnamed_4 {
                name: b"unlink\0" as *const u8 as *const libc::c_char,
                place: &mut opt.unlink_requested as *mut bool as *mut libc::c_void,
                action: Some(
                    cmd_boolean
                        as unsafe extern "C" fn(
                            *const libc::c_char,
                            *const libc::c_char,
                            *mut libc::c_void,
                        ) -> bool,
                ),
            };
            init
        },
        {
            let mut init = C2RustUnnamed_4 {
                name: b"useaskpass\0" as *const u8 as *const libc::c_char,
                place: &mut opt.use_askpass as *mut *mut libc::c_char
                    as *mut libc::c_void,
                action: Some(
                    cmd_use_askpass
                        as unsafe extern "C" fn(
                            *const libc::c_char,
                            *const libc::c_char,
                            *mut libc::c_void,
                        ) -> bool,
                ),
            };
            init
        },
        {
            let mut init = C2RustUnnamed_4 {
                name: b"useproxy\0" as *const u8 as *const libc::c_char,
                place: &mut opt.use_proxy as *mut bool as *mut libc::c_void,
                action: Some(
                    cmd_boolean
                        as unsafe extern "C" fn(
                            *const libc::c_char,
                            *const libc::c_char,
                            *mut libc::c_void,
                        ) -> bool,
                ),
            };
            init
        },
        {
            let mut init = C2RustUnnamed_4 {
                name: b"user\0" as *const u8 as *const libc::c_char,
                place: &mut opt.user as *mut *mut libc::c_char as *mut libc::c_void,
                action: Some(
                    cmd_string
                        as unsafe extern "C" fn(
                            *const libc::c_char,
                            *const libc::c_char,
                            *mut libc::c_void,
                        ) -> bool,
                ),
            };
            init
        },
        {
            let mut init = C2RustUnnamed_4 {
                name: b"useragent\0" as *const u8 as *const libc::c_char,
                place: 0 as *mut libc::c_void,
                action: Some(
                    cmd_spec_useragent
                        as unsafe extern "C" fn(
                            *const libc::c_char,
                            *const libc::c_char,
                            *mut libc::c_void,
                        ) -> bool,
                ),
            };
            init
        },
        {
            let mut init = C2RustUnnamed_4 {
                name: b"useservertimestamps\0" as *const u8 as *const libc::c_char,
                place: &mut opt.useservertimestamps as *mut bool as *mut libc::c_void,
                action: Some(
                    cmd_boolean
                        as unsafe extern "C" fn(
                            *const libc::c_char,
                            *const libc::c_char,
                            *mut libc::c_void,
                        ) -> bool,
                ),
            };
            init
        },
        {
            let mut init = C2RustUnnamed_4 {
                name: b"verbose\0" as *const u8 as *const libc::c_char,
                place: 0 as *mut libc::c_void,
                action: Some(
                    cmd_spec_verbose
                        as unsafe extern "C" fn(
                            *const libc::c_char,
                            *const libc::c_char,
                            *mut libc::c_void,
                        ) -> bool,
                ),
            };
            init
        },
        {
            let mut init = C2RustUnnamed_4 {
                name: b"wait\0" as *const u8 as *const libc::c_char,
                place: &mut opt.wait as *mut libc::c_double as *mut libc::c_void,
                action: Some(
                    cmd_time
                        as unsafe extern "C" fn(
                            *const libc::c_char,
                            *const libc::c_char,
                            *mut libc::c_void,
                        ) -> bool,
                ),
            };
            init
        },
        {
            let mut init = C2RustUnnamed_4 {
                name: b"waitretry\0" as *const u8 as *const libc::c_char,
                place: &mut opt.waitretry as *mut libc::c_double as *mut libc::c_void,
                action: Some(
                    cmd_time
                        as unsafe extern "C" fn(
                            *const libc::c_char,
                            *const libc::c_char,
                            *mut libc::c_void,
                        ) -> bool,
                ),
            };
            init
        },
        {
            let mut init = C2RustUnnamed_4 {
                name: b"warccdx\0" as *const u8 as *const libc::c_char,
                place: &mut opt.warc_cdx_enabled as *mut bool as *mut libc::c_void,
                action: Some(
                    cmd_boolean
                        as unsafe extern "C" fn(
                            *const libc::c_char,
                            *const libc::c_char,
                            *mut libc::c_void,
                        ) -> bool,
                ),
            };
            init
        },
        {
            let mut init = C2RustUnnamed_4 {
                name: b"warccdxdedup\0" as *const u8 as *const libc::c_char,
                place: &mut opt.warc_cdx_dedup_filename as *mut *mut libc::c_char
                    as *mut libc::c_void,
                action: Some(
                    cmd_file
                        as unsafe extern "C" fn(
                            *const libc::c_char,
                            *const libc::c_char,
                            *mut libc::c_void,
                        ) -> bool,
                ),
            };
            init
        },
        {
            let mut init = C2RustUnnamed_4 {
                name: b"warccompression\0" as *const u8 as *const libc::c_char,
                place: &mut opt.warc_compression_enabled as *mut bool
                    as *mut libc::c_void,
                action: Some(
                    cmd_boolean
                        as unsafe extern "C" fn(
                            *const libc::c_char,
                            *const libc::c_char,
                            *mut libc::c_void,
                        ) -> bool,
                ),
            };
            init
        },
        {
            let mut init = C2RustUnnamed_4 {
                name: b"warcdigests\0" as *const u8 as *const libc::c_char,
                place: &mut opt.warc_digests_enabled as *mut bool as *mut libc::c_void,
                action: Some(
                    cmd_boolean
                        as unsafe extern "C" fn(
                            *const libc::c_char,
                            *const libc::c_char,
                            *mut libc::c_void,
                        ) -> bool,
                ),
            };
            init
        },
        {
            let mut init = C2RustUnnamed_4 {
                name: b"warcfile\0" as *const u8 as *const libc::c_char,
                place: &mut opt.warc_filename as *mut *mut libc::c_char
                    as *mut libc::c_void,
                action: Some(
                    cmd_file
                        as unsafe extern "C" fn(
                            *const libc::c_char,
                            *const libc::c_char,
                            *mut libc::c_void,
                        ) -> bool,
                ),
            };
            init
        },
        {
            let mut init = C2RustUnnamed_4 {
                name: b"warcheader\0" as *const u8 as *const libc::c_char,
                place: 0 as *mut libc::c_void,
                action: Some(
                    cmd_spec_warc_header
                        as unsafe extern "C" fn(
                            *const libc::c_char,
                            *const libc::c_char,
                            *mut libc::c_void,
                        ) -> bool,
                ),
            };
            init
        },
        {
            let mut init = C2RustUnnamed_4 {
                name: b"warckeeplog\0" as *const u8 as *const libc::c_char,
                place: &mut opt.warc_keep_log as *mut bool as *mut libc::c_void,
                action: Some(
                    cmd_boolean
                        as unsafe extern "C" fn(
                            *const libc::c_char,
                            *const libc::c_char,
                            *mut libc::c_void,
                        ) -> bool,
                ),
            };
            init
        },
        {
            let mut init = C2RustUnnamed_4 {
                name: b"warcmaxsize\0" as *const u8 as *const libc::c_char,
                place: &mut opt.warc_maxsize as *mut wgint as *mut libc::c_void,
                action: Some(
                    cmd_bytes
                        as unsafe extern "C" fn(
                            *const libc::c_char,
                            *const libc::c_char,
                            *mut libc::c_void,
                        ) -> bool,
                ),
            };
            init
        },
        {
            let mut init = C2RustUnnamed_4 {
                name: b"warctempdir\0" as *const u8 as *const libc::c_char,
                place: &mut opt.warc_tempdir as *mut *mut libc::c_char
                    as *mut libc::c_void,
                action: Some(
                    cmd_directory
                        as unsafe extern "C" fn(
                            *const libc::c_char,
                            *const libc::c_char,
                            *mut libc::c_void,
                        ) -> bool,
                ),
            };
            init
        },
        {
            let mut init = C2RustUnnamed_4 {
                name: b"xattr\0" as *const u8 as *const libc::c_char,
                place: &mut opt.enable_xattr as *mut bool as *mut libc::c_void,
                action: Some(
                    cmd_boolean
                        as unsafe extern "C" fn(
                            *const libc::c_char,
                            *const libc::c_char,
                            *mut libc::c_void,
                        ) -> bool,
                ),
            };
            init
        },
    ];
}
#[used]
#[cfg_attr(target_os = "linux", link_section = ".init_array")]
#[cfg_attr(target_os = "windows", link_section = ".CRT$XIB")]
#[cfg_attr(target_os = "macos", link_section = "__DATA,__mod_init_func")]
static INIT_ARRAY: [unsafe extern "C" fn(); 1] = [run_static_initializers];
