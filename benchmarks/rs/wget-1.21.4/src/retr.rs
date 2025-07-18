use ::libc;
use ::c2rust_bitfields;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    pub type internal_state;
    pub type ptimer;
    fn rename(__old: *const libc::c_char, __new: *const libc::c_char) -> libc::c_int;
    fn snprintf(
        _: *mut libc::c_char,
        _: libc::c_ulong,
        _: *const libc::c_char,
        _: ...
    ) -> libc::c_int;
    fn fwrite(
        _: *const libc::c_void,
        _: libc::c_ulong,
        _: libc::c_ulong,
        _: *mut FILE,
    ) -> libc::c_ulong;
    fn ferror(__stream: *mut FILE) -> libc::c_int;
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
    fn memset(
        _: *mut libc::c_void,
        _: libc::c_int,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn memchr(
        _: *const libc::c_void,
        _: libc::c_int,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    fn strdup(_: *const libc::c_char) -> *mut libc::c_char;
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    fn strerror(_: libc::c_int) -> *mut libc::c_char;
    fn rpl_free(_: *mut libc::c_void);
    static mut opt: options;
    fn getenv(__name: *const libc::c_char) -> *mut libc::c_char;
    fn rpl_strtol(
        string: *const libc::c_char,
        endptr: *mut *mut libc::c_char,
        base: libc::c_int,
    ) -> libc::c_long;
    fn xmalloc(s: size_t) -> *mut libc::c_void;
    fn xcalloc(n: size_t, s: size_t) -> *mut libc::c_void;
    fn xrealloc(p: *mut libc::c_void, s: size_t) -> *mut libc::c_void;
    fn rpl_fflush(gl_stream: *mut FILE) -> libc::c_int;
    fn logprintf(_: log_options, _: *const libc::c_char, _: ...);
    fn debug_logprintf(_: *const libc::c_char, _: ...);
    fn logputs(_: log_options, _: *const libc::c_char);
    fn escnonprint_uri(_: *const libc::c_char) -> *const libc::c_char;
    fn quote(arg: *const libc::c_char) -> *const libc::c_char;
    fn iri_new() -> *mut iri;
    fn iri_dup(_: *const iri) -> *mut iri;
    fn iri_free(i: *mut iri);
    fn set_uri_encoding(i: *mut iri, charset: *const libc::c_char, force: bool);
    fn set_content_encoding(i: *mut iri, charset: *const libc::c_char);
    fn xstrdup(str: *const libc::c_char) -> *mut libc::c_char;
    fn unlink(__name: *const libc::c_char) -> libc::c_int;
    fn __errno_location() -> *mut libc::c_int;
    fn inflateInit2_(
        strm: z_streamp,
        windowBits: libc::c_int,
        version: *const libc::c_char,
        stream_size: libc::c_int,
    ) -> libc::c_int;
    fn inflate(strm: z_streamp, flush: libc::c_int) -> libc::c_int;
    fn inflateEnd(strm: z_streamp) -> libc::c_int;
    fn inform_exit_status(err: uerr_t);
    fn file_exists_p(_: *const libc::c_char, _: *mut file_stats_t) -> bool;
    fn has_html_suffix_p(_: *const libc::c_char) -> bool;
    fn number_to_static_string(_: wgint) -> *mut libc::c_char;
    fn convert_to_bits(_: wgint) -> wgint;
    fn random_float() -> libc::c_double;
    fn xsleep(_: libc::c_double);
    fn url_parse(
        _: *const libc::c_char,
        _: *mut libc::c_int,
        iri: *mut iri,
        percent_encode: bool,
    ) -> *mut url;
    fn url_error(_: libc::c_int) -> *const libc::c_char;
    fn url_free(_: *mut url);
    fn url_has_scheme(_: *const libc::c_char) -> bool;
    fn url_valid_scheme(_: *const libc::c_char) -> bool;
    fn uri_merge(_: *const libc::c_char, _: *const libc::c_char) -> *mut libc::c_char;
    fn rewrite_shorthand_url(_: *const libc::c_char) -> *mut libc::c_char;
    fn progress_create(_: *const libc::c_char, _: wgint, _: wgint) -> *mut libc::c_void;
    fn progress_interactive_p(_: *mut libc::c_void) -> bool;
    fn progress_update(_: *mut libc::c_void, _: wgint, _: libc::c_double);
    fn progress_finish(_: *mut libc::c_void, _: libc::c_double);
    fn retrieve_tree(_: *mut url, _: *mut iri) -> uerr_t;
    fn ftp_loop(
        _: *mut url,
        _: *mut url,
        _: *mut *mut libc::c_char,
        _: *mut libc::c_int,
        _: *mut url,
        _: bool,
        _: bool,
    ) -> uerr_t;
    fn sufmatch(_: *mut *const libc::c_char, _: *const libc::c_char) -> bool;
    fn http_loop(
        _: *const url,
        _: *mut url,
        _: *mut *mut libc::c_char,
        _: *mut *mut libc::c_char,
        _: *const libc::c_char,
        _: *mut libc::c_int,
        _: *mut url,
        _: *mut iri,
    ) -> uerr_t;
    fn fd_read(
        _: libc::c_int,
        _: *mut libc::c_char,
        _: libc::c_int,
        _: libc::c_double,
    ) -> libc::c_int;
    fn fd_peek(
        _: libc::c_int,
        _: *mut libc::c_char,
        _: libc::c_int,
        _: libc::c_double,
    ) -> libc::c_int;
    fn register_download(_: *const libc::c_char, _: *const libc::c_char);
    fn register_redirection(_: *const libc::c_char, _: *const libc::c_char);
    fn register_html(_: *const libc::c_char);
    fn register_css(_: *const libc::c_char);
    fn ptimer_new() -> *mut ptimer;
    fn ptimer_destroy(_: *mut ptimer);
    fn ptimer_measure(_: *mut ptimer) -> libc::c_double;
    fn ptimer_read(_: *const ptimer) -> libc::c_double;
    fn ptimer_resolution() -> libc::c_double;
    fn get_urls_file(_: *const libc::c_char) -> *mut urlpos;
    fn get_urls_html(
        _: *const libc::c_char,
        _: *const libc::c_char,
        _: *mut bool,
        _: *mut iri,
    ) -> *mut urlpos;
}
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
pub type Byte = libc::c_uchar;
pub type uInt = libc::c_uint;
pub type uLong = libc::c_ulong;
pub type Bytef = Byte;
pub type voidpf = *mut libc::c_void;
pub type alloc_func = Option::<unsafe extern "C" fn(voidpf, uInt, uInt) -> voidpf>;
pub type free_func = Option::<unsafe extern "C" fn(voidpf, voidpf) -> ()>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct z_stream_s {
    pub next_in: *mut Bytef,
    pub avail_in: uInt,
    pub total_in: uLong,
    pub next_out: *mut Bytef,
    pub avail_out: uInt,
    pub total_out: uLong,
    pub msg: *mut libc::c_char,
    pub state: *mut internal_state,
    pub zalloc: alloc_func,
    pub zfree: free_func,
    pub opaque: voidpf,
    pub data_type: libc::c_int,
    pub adler: uLong,
    pub reserved: uLong,
}
pub type z_stream = z_stream_s;
pub type z_streamp = *mut z_stream;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct file_stat_s {
    pub access_err: libc::c_int,
    pub st_ino: ino_t,
    pub st_dev: dev_t,
}
pub type file_stats_t = file_stat_s;
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
pub struct C2RustUnnamed_5 {
    pub chunk_bytes: wgint,
    pub chunk_start: libc::c_double,
    pub sleep_adjust: libc::c_double,
}
pub type hunk_terminator_t = Option::<
    unsafe extern "C" fn(
        *const libc::c_char,
        *const libc::c_char,
        libc::c_int,
    ) -> *const libc::c_char,
>;
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
#[inline]
unsafe extern "C" fn stat(
    mut __path: *const libc::c_char,
    mut __statbuf: *mut stat,
) -> libc::c_int {
    return __xstat(1 as libc::c_int, __path, __statbuf);
}
pub static mut total_downloaded_bytes: wgint = 0;
pub static mut total_download_time: libc::c_double = 0.;
pub static mut output_stream: *mut FILE = 0 as *const FILE as *mut FILE;
pub static mut output_stream_regular: bool = false;
static mut limit_data: C2RustUnnamed_5 = C2RustUnnamed_5 {
    chunk_bytes: 0,
    chunk_start: 0.,
    sleep_adjust: 0.,
};
unsafe extern "C" fn limit_bandwidth_reset() {
    memset(
        &mut limit_data as *mut C2RustUnnamed_5 as *mut libc::c_void,
        '\0' as i32,
        ::std::mem::size_of::<C2RustUnnamed_5>() as libc::c_ulong,
    );
}
unsafe extern "C" fn zalloc(
    mut opaque: voidpf,
    mut items: libc::c_uint,
    mut size: libc::c_uint,
) -> voidpf {
    return xcalloc(items as size_t, size as size_t);
}
unsafe extern "C" fn zfree(mut opaque: voidpf, mut address: voidpf) {
    rpl_free(address);
    address = 0 as *mut libc::c_void;
}
unsafe extern "C" fn limit_bandwidth(mut bytes: wgint, mut timer: *mut ptimer) {
    let mut delta_t: libc::c_double = ptimer_read(timer) - limit_data.chunk_start;
    let mut expected: libc::c_double = 0.;
    limit_data.chunk_bytes += bytes;
    expected = limit_data.chunk_bytes as libc::c_double
        / opt.limit_rate as libc::c_double;
    if expected > delta_t {
        let mut slp: libc::c_double = expected - delta_t + limit_data.sleep_adjust;
        let mut t0: libc::c_double = 0.;
        let mut t1: libc::c_double = 0.;
        if slp < 0.2f64 {
            if opt.debug as libc::c_long != 0 {
                debug_logprintf(
                    b"deferring a %.2f ms sleep (%s/%.2f).\n\0" as *const u8
                        as *const libc::c_char,
                    slp * 1000 as libc::c_int as libc::c_double,
                    number_to_static_string(limit_data.chunk_bytes),
                    delta_t,
                );
            }
            return;
        }
        if opt.debug as libc::c_long != 0 {
            debug_logprintf(
                b"\nsleeping %.2f ms for %s bytes, adjust %.2f ms\n\0" as *const u8
                    as *const libc::c_char,
                slp * 1000 as libc::c_int as libc::c_double,
                number_to_static_string(limit_data.chunk_bytes),
                limit_data.sleep_adjust,
            );
        }
        t0 = ptimer_read(timer);
        xsleep(slp);
        t1 = ptimer_measure(timer);
        limit_data.sleep_adjust = slp - (t1 - t0);
        if limit_data.sleep_adjust > 0.5f64 {
            limit_data.sleep_adjust = 0.5f64;
        } else if limit_data.sleep_adjust < -0.5f64 {
            limit_data.sleep_adjust = -0.5f64;
        }
    }
    limit_data.chunk_bytes = 0 as libc::c_int as wgint;
    limit_data.chunk_start = ptimer_read(timer);
}
unsafe extern "C" fn write_data(
    mut out: *mut FILE,
    mut out2: *mut FILE,
    mut buf: *const libc::c_char,
    mut bufsize: libc::c_int,
    mut skip: *mut wgint,
    mut written: *mut wgint,
) -> libc::c_int {
    if out.is_null() && out2.is_null() {
        return 1 as libc::c_int;
    }
    if !skip.is_null() {
        if *skip > bufsize as libc::c_long {
            *skip -= bufsize as libc::c_long;
            return 1 as libc::c_int;
        }
        if *skip != 0 {
            buf = buf.offset(*skip as isize);
            bufsize = (bufsize as libc::c_long - *skip) as libc::c_int;
            *skip = 0 as libc::c_int as wgint;
            if bufsize == 0 as libc::c_int {
                return 1 as libc::c_int;
            }
        }
    }
    if !out.is_null() {
        fwrite(
            buf as *const libc::c_void,
            1 as libc::c_int as libc::c_ulong,
            bufsize as libc::c_ulong,
            out,
        );
    }
    if !out2.is_null() {
        fwrite(
            buf as *const libc::c_void,
            1 as libc::c_int as libc::c_ulong,
            bufsize as libc::c_ulong,
            out2,
        );
    }
    if !written.is_null() {
        *written += bufsize as libc::c_long;
    }
    if !out.is_null() {
        rpl_fflush(out);
    }
    if !out2.is_null() {
        rpl_fflush(out2);
    }
    if !out.is_null() && ferror(out) != 0 {
        return -(2 as libc::c_int)
    } else if !out2.is_null() && ferror(out2) != 0 {
        return -(3 as libc::c_int)
    }
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn fd_read_body(
    mut downloaded_filename: *const libc::c_char,
    mut fd: libc::c_int,
    mut out: *mut FILE,
    mut toread: wgint,
    mut startpos: wgint,
    mut qtyread: *mut wgint,
    mut qtywritten: *mut wgint,
    mut elapsed: *mut libc::c_double,
    mut flags: libc::c_int,
    mut out2: *mut FILE,
) -> libc::c_int {
    let mut current_block: u64;
    let mut ret: libc::c_int = 0 as libc::c_int;
    let mut dlbufsize: libc::c_int = if 8192 as libc::c_int
        >= 64 as libc::c_int * 1024 as libc::c_int
    {
        8192 as libc::c_int
    } else {
        64 as libc::c_int * 1024 as libc::c_int
    };
    let mut dlbuf: *mut libc::c_char = xmalloc(dlbufsize as size_t) as *mut libc::c_char;
    let mut timer: *mut ptimer = 0 as *mut ptimer;
    let mut last_successful_read_tm: libc::c_double = 0 as libc::c_int as libc::c_double;
    let mut progress: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut progress_interactive: bool = 0 as libc::c_int != 0;
    let mut exact: bool = flags & rb_read_exactly as libc::c_int != 0;
    let mut chunked: bool = flags & rb_chunked_transfer_encoding as libc::c_int != 0;
    let mut skip: wgint = 0 as libc::c_int as wgint;
    let mut sum_read: wgint = 0 as libc::c_int as wgint;
    let mut sum_written: wgint = 0 as libc::c_int as wgint;
    let mut remaining_chunk_size: wgint = 0 as libc::c_int as wgint;
    let mut gzbufsize: libc::c_uint = (dlbufsize * 4 as libc::c_int) as libc::c_uint;
    let mut gzbuf: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut gzstream: z_stream = z_stream {
        next_in: 0 as *mut Bytef,
        avail_in: 0,
        total_in: 0,
        next_out: 0 as *mut Bytef,
        avail_out: 0,
        total_out: 0,
        msg: 0 as *mut libc::c_char,
        state: 0 as *mut internal_state,
        zalloc: None,
        zfree: None,
        opaque: 0 as *mut libc::c_void,
        data_type: 0,
        adler: 0,
        reserved: 0,
    };
    if flags & rb_compressed_gzip as libc::c_int != 0 {
        gzbuf = xmalloc(gzbufsize as size_t) as *mut libc::c_char;
        gzstream
            .zalloc = Some(
            zalloc as unsafe extern "C" fn(voidpf, libc::c_uint, libc::c_uint) -> voidpf,
        );
        gzstream.zfree = Some(zfree as unsafe extern "C" fn(voidpf, voidpf) -> ());
        gzstream.opaque = 0 as voidpf;
        gzstream.next_in = 0 as *mut Bytef;
        gzstream.avail_in = 0 as libc::c_int as uInt;
        ret = inflateInit2_(
            &mut gzstream,
            32 as libc::c_int | 15 as libc::c_int,
            b"1.2.11\0" as *const u8 as *const libc::c_char,
            ::std::mem::size_of::<z_stream>() as libc::c_ulong as libc::c_int,
        );
        if ret != 0 as libc::c_int {
            rpl_free(gzbuf as *mut libc::c_void);
            gzbuf = 0 as *mut libc::c_char;
            *__errno_location() = if ret == -(4 as libc::c_int) {
                12 as libc::c_int
            } else {
                22 as libc::c_int
            };
            ret = -(1 as libc::c_int);
            current_block = 5797027243652429374;
        } else {
            current_block = 17833034027772472439;
        }
    } else {
        current_block = 17833034027772472439;
    }
    match current_block {
        17833034027772472439 => {
            if flags & rb_skip_startpos as libc::c_int != 0 {
                skip = startpos;
            }
            if opt.show_progress != 0 {
                let mut filename_progress: *const libc::c_char = 0
                    as *const libc::c_char;
                let mut start: wgint = if skip != 0 {
                    0 as libc::c_int as libc::c_long
                } else {
                    startpos
                };
                if !(opt.dir_prefix).is_null() {
                    filename_progress = downloaded_filename
                        .offset(strlen(opt.dir_prefix) as isize)
                        .offset(1 as libc::c_int as isize);
                } else {
                    filename_progress = downloaded_filename;
                }
                progress = progress_create(filename_progress, start, start + toread);
                progress_interactive = progress_interactive_p(progress);
            }
            if opt.limit_rate != 0 {
                limit_bandwidth_reset();
            }
            if !progress.is_null() || opt.limit_rate != 0 || !elapsed.is_null() {
                timer = ptimer_new();
                last_successful_read_tm = 0 as libc::c_int as libc::c_double;
            }
            if opt.limit_rate != 0 && opt.limit_rate < dlbufsize as libc::c_long {
                dlbufsize = opt.limit_rate as libc::c_int;
            }
            's_160: loop {
                if !(!exact || sum_read < toread) {
                    current_block = 16667286137552459707;
                    break;
                }
                let mut rdsize: libc::c_int = 0;
                let mut tmout: libc::c_double = opt.read_timeout;
                if chunked {
                    if remaining_chunk_size == 0 as libc::c_int as libc::c_long {
                        let mut line: *mut libc::c_char = fd_read_line(fd);
                        let mut endl: *mut libc::c_char = 0 as *mut libc::c_char;
                        if line.is_null() {
                            ret = -(1 as libc::c_int);
                            current_block = 16667286137552459707;
                            break;
                        } else {
                            if !out2.is_null() {
                                fwrite(
                                    line as *const libc::c_void,
                                    1 as libc::c_int as libc::c_ulong,
                                    strlen(line),
                                    out2,
                                );
                            }
                            remaining_chunk_size = rpl_strtol(
                                line,
                                &mut endl,
                                16 as libc::c_int,
                            );
                            rpl_free(line as *mut libc::c_void);
                            line = 0 as *mut libc::c_char;
                            if remaining_chunk_size < 0 as libc::c_int as libc::c_long {
                                ret = -(1 as libc::c_int);
                                current_block = 16667286137552459707;
                                break;
                            } else if remaining_chunk_size
                                == 0 as libc::c_int as libc::c_long
                            {
                                ret = 0 as libc::c_int;
                                line = fd_read_line(fd);
                                if line.is_null() {
                                    ret = -(1 as libc::c_int);
                                } else {
                                    if !out2.is_null() {
                                        fwrite(
                                            line as *const libc::c_void,
                                            1 as libc::c_int as libc::c_ulong,
                                            strlen(line),
                                            out2,
                                        );
                                    }
                                    rpl_free(line as *mut libc::c_void);
                                    line = 0 as *mut libc::c_char;
                                }
                                current_block = 16667286137552459707;
                                break;
                            }
                        }
                    }
                    rdsize = (if remaining_chunk_size <= dlbufsize as libc::c_long {
                        remaining_chunk_size
                    } else {
                        dlbufsize as libc::c_long
                    }) as libc::c_int;
                } else {
                    rdsize = (if exact as libc::c_int != 0 {
                        if toread - sum_read <= dlbufsize as libc::c_long {
                            toread - sum_read
                        } else {
                            dlbufsize as libc::c_long
                        }
                    } else {
                        dlbufsize as libc::c_long
                    }) as libc::c_int;
                }
                if progress_interactive {
                    tmout = 0.95f64;
                    *__errno_location() = 0 as libc::c_int;
                    if opt.read_timeout != 0. {
                        let mut waittm: libc::c_double = 0.;
                        waittm = ptimer_read(timer) - last_successful_read_tm;
                        if waittm + tmout > opt.read_timeout {
                            tmout = opt.read_timeout - waittm;
                            if tmout <= 0 as libc::c_int as libc::c_double {
                                ret = -(1 as libc::c_int);
                                *__errno_location() = 110 as libc::c_int;
                                current_block = 16667286137552459707;
                                break;
                            }
                        }
                    }
                }
                ret = fd_read(fd, dlbuf, rdsize, tmout);
                if progress_interactive as libc::c_int != 0 && ret < 0 as libc::c_int
                    && *__errno_location() == 110 as libc::c_int
                {
                    ret = 0 as libc::c_int;
                } else if ret <= 0 as libc::c_int {
                    current_block = 16667286137552459707;
                    break;
                }
                if !progress.is_null() || opt.limit_rate != 0 || !elapsed.is_null() {
                    ptimer_measure(timer);
                    if ret > 0 as libc::c_int {
                        last_successful_read_tm = ptimer_read(timer);
                    }
                }
                if ret > 0 as libc::c_int {
                    let mut write_res: libc::c_int = 0;
                    sum_read += ret as libc::c_long;
                    if !gzbuf.is_null() {
                        let mut err: libc::c_int = 0;
                        let mut towrite: libc::c_int = 0;
                        write_res = write_data(
                            0 as *mut FILE,
                            out2,
                            dlbuf,
                            ret,
                            0 as *mut wgint,
                            0 as *mut wgint,
                        );
                        if write_res < 0 as libc::c_int {
                            ret = write_res;
                            current_block = 5797027243652429374;
                            break;
                        } else {
                            gzstream.avail_in = ret as uInt;
                            gzstream.next_in = dlbuf as *mut libc::c_uchar;
                            loop {
                                gzstream.avail_out = gzbufsize;
                                gzstream.next_out = gzbuf as *mut libc::c_uchar;
                                err = inflate(&mut gzstream, 0 as libc::c_int);
                                match err {
                                    -4 => {
                                        *__errno_location() = 12 as libc::c_int;
                                        ret = -(1 as libc::c_int);
                                        current_block = 5797027243652429374;
                                        break 's_160;
                                    }
                                    2 | -3 => {
                                        *__errno_location() = 22 as libc::c_int;
                                        ret = -(1 as libc::c_int);
                                        current_block = 5797027243652429374;
                                        break 's_160;
                                    }
                                    1 => {
                                        if exact as libc::c_int != 0 && sum_read != toread {
                                            if opt.debug as libc::c_long != 0 {
                                                debug_logprintf(
                                                    b"zlib stream ended unexpectedly after %ld/%ld bytes\n\0"
                                                        as *const u8 as *const libc::c_char,
                                                    sum_read,
                                                    toread,
                                                );
                                            }
                                        }
                                    }
                                    _ => {}
                                }
                                towrite = gzbufsize.wrapping_sub(gzstream.avail_out)
                                    as libc::c_int;
                                write_res = write_data(
                                    out,
                                    0 as *mut FILE,
                                    gzbuf,
                                    towrite,
                                    &mut skip,
                                    &mut sum_written,
                                );
                                if write_res < 0 as libc::c_int {
                                    ret = write_res;
                                    current_block = 5797027243652429374;
                                    break 's_160;
                                } else if !(gzstream.avail_out
                                    == 0 as libc::c_int as libc::c_uint)
                                {
                                    break;
                                }
                            }
                        }
                    } else {
                        write_res = write_data(
                            out,
                            out2,
                            dlbuf,
                            ret,
                            &mut skip,
                            &mut sum_written,
                        );
                        if write_res < 0 as libc::c_int {
                            ret = write_res;
                            current_block = 5797027243652429374;
                            break;
                        }
                    }
                    if chunked {
                        remaining_chunk_size -= ret as libc::c_long;
                        if remaining_chunk_size == 0 as libc::c_int as libc::c_long {
                            let mut line_0: *mut libc::c_char = fd_read_line(fd);
                            if line_0.is_null() {
                                ret = -(1 as libc::c_int);
                                current_block = 16667286137552459707;
                                break;
                            } else {
                                if !out2.is_null() {
                                    fwrite(
                                        line_0 as *const libc::c_void,
                                        1 as libc::c_int as libc::c_ulong,
                                        strlen(line_0),
                                        out2,
                                    );
                                }
                                rpl_free(line_0 as *mut libc::c_void);
                                line_0 = 0 as *mut libc::c_char;
                            }
                        }
                    }
                }
                if opt.limit_rate != 0 {
                    limit_bandwidth(ret as wgint, timer);
                }
                if !progress.is_null() {
                    progress_update(progress, ret as wgint, ptimer_read(timer));
                }
            }
            match current_block {
                5797027243652429374 => {}
                _ => {
                    if ret < -(1 as libc::c_int) {
                        ret = -(1 as libc::c_int);
                    }
                }
            }
        }
        _ => {}
    }
    if !progress.is_null() {
        progress_finish(progress, ptimer_read(timer));
    }
    if !timer.is_null() {
        if !elapsed.is_null() {
            *elapsed = ptimer_read(timer);
        }
        ptimer_destroy(timer);
    }
    if !gzbuf.is_null() {
        let mut err_0: libc::c_int = inflateEnd(&mut gzstream);
        if ret >= 0 as libc::c_int {
            if err_0 == 0 as libc::c_int {
                ret = 0 as libc::c_int;
            } else {
                *__errno_location() = 22 as libc::c_int;
                ret = -(1 as libc::c_int);
            }
        }
        rpl_free(gzbuf as *mut libc::c_void);
        gzbuf = 0 as *mut libc::c_char;
        if gzstream.total_in != sum_read as uLong {
            if opt.debug as libc::c_long != 0 {
                debug_logprintf(
                    b"zlib read size differs from raw read size (%lu/%ld)\n\0"
                        as *const u8 as *const libc::c_char,
                    gzstream.total_in,
                    sum_read,
                );
            }
        }
    }
    if !qtyread.is_null() {
        *qtyread += sum_read;
    }
    if !qtywritten.is_null() {
        *qtywritten += sum_written;
    }
    rpl_free(dlbuf as *mut libc::c_void);
    dlbuf = 0 as *mut libc::c_char;
    return ret;
}
pub unsafe extern "C" fn fd_read_hunk(
    mut fd: libc::c_int,
    mut terminator: hunk_terminator_t,
    mut sizehint: libc::c_long,
    mut maxsize: libc::c_long,
) -> *mut libc::c_char {
    let mut bufsize: libc::c_long = sizehint;
    let mut hunk: *mut libc::c_char = xmalloc(bufsize as size_t) as *mut libc::c_char;
    let mut tail: libc::c_int = 0 as libc::c_int;
    loop {
        let mut end: *const libc::c_char = 0 as *const libc::c_char;
        let mut pklen: libc::c_int = 0;
        let mut rdlen: libc::c_int = 0;
        let mut remain: libc::c_int = 0;
        pklen = fd_peek(
            fd,
            hunk.offset(tail as isize),
            (bufsize - 1 as libc::c_int as libc::c_long - tail as libc::c_long)
                as libc::c_int,
            -(1 as libc::c_int) as libc::c_double,
        );
        if pklen < 0 as libc::c_int {
            rpl_free(hunk as *mut libc::c_void);
            hunk = 0 as *mut libc::c_char;
            return 0 as *mut libc::c_char;
        }
        end = terminator.unwrap()(hunk, hunk.offset(tail as isize), pklen);
        if !end.is_null() {
            remain = end.offset_from(hunk.offset(tail as isize)) as libc::c_long
                as libc::c_int;
            if remain == 0 as libc::c_int {
                *hunk.offset(tail as isize) = '\0' as i32 as libc::c_char;
                return hunk;
            }
            if (bufsize - 1 as libc::c_int as libc::c_long)
                < (tail + remain) as libc::c_long
            {
                bufsize = (tail + remain + 1 as libc::c_int) as libc::c_long;
                hunk = xrealloc(hunk as *mut libc::c_void, bufsize as size_t)
                    as *mut libc::c_char;
            }
        } else {
            remain = pklen;
        }
        rdlen = fd_read(
            fd,
            hunk.offset(tail as isize),
            remain,
            0 as libc::c_int as libc::c_double,
        );
        if rdlen < 0 as libc::c_int {
            rpl_free(hunk as *mut libc::c_void);
            hunk = 0 as *mut libc::c_char;
            return 0 as *mut libc::c_char;
        }
        tail += rdlen;
        *hunk.offset(tail as isize) = '\0' as i32 as libc::c_char;
        if rdlen == 0 as libc::c_int {
            if tail == 0 as libc::c_int {
                rpl_free(hunk as *mut libc::c_void);
                hunk = 0 as *mut libc::c_char;
                *__errno_location() = 0 as libc::c_int;
                return 0 as *mut libc::c_char;
            } else {
                return hunk
            }
        }
        if !end.is_null() && rdlen == remain {
            return hunk;
        }
        if tail as libc::c_long == bufsize - 1 as libc::c_int as libc::c_long {
            if maxsize != 0 && bufsize >= maxsize {
                rpl_free(hunk as *mut libc::c_void);
                hunk = 0 as *mut libc::c_char;
                *__errno_location() = 12 as libc::c_int;
                return 0 as *mut libc::c_char;
            }
            bufsize <<= 1 as libc::c_int;
            if maxsize != 0 && bufsize > maxsize {
                bufsize = maxsize;
            }
            hunk = xrealloc(hunk as *mut libc::c_void, bufsize as size_t)
                as *mut libc::c_char;
        }
    };
}
unsafe extern "C" fn line_terminator(
    mut start: *const libc::c_char,
    mut peeked: *const libc::c_char,
    mut peeklen: libc::c_int,
) -> *const libc::c_char {
    let mut p: *const libc::c_char = memchr(
        peeked as *const libc::c_void,
        '\n' as i32,
        peeklen as libc::c_ulong,
    ) as *const libc::c_char;
    if !p.is_null() {
        return p.offset(1 as libc::c_int as isize);
    }
    return 0 as *const libc::c_char;
}
pub unsafe extern "C" fn fd_read_line(mut fd: libc::c_int) -> *mut libc::c_char {
    return fd_read_hunk(
        fd,
        Some(
            line_terminator
                as unsafe extern "C" fn(
                    *const libc::c_char,
                    *const libc::c_char,
                    libc::c_int,
                ) -> *const libc::c_char,
        ),
        128 as libc::c_int as libc::c_long,
        4096 as libc::c_int as libc::c_long,
    );
}
pub unsafe extern "C" fn retr_rate(
    mut bytes: wgint,
    mut secs: libc::c_double,
) -> *const libc::c_char {
    static mut res: [libc::c_char; 20] = [0; 20];
    static mut rate_names: [*const libc::c_char; 4] = [
        b"B/s\0" as *const u8 as *const libc::c_char,
        b"KB/s\0" as *const u8 as *const libc::c_char,
        b"MB/s\0" as *const u8 as *const libc::c_char,
        b"GB/s\0" as *const u8 as *const libc::c_char,
    ];
    static mut rate_names_bits: [*const libc::c_char; 4] = [
        b"b/s\0" as *const u8 as *const libc::c_char,
        b"Kb/s\0" as *const u8 as *const libc::c_char,
        b"Mb/s\0" as *const u8 as *const libc::c_char,
        b"Gb/s\0" as *const u8 as *const libc::c_char,
    ];
    let mut units: libc::c_int = 0;
    let mut dlrate: libc::c_double = calc_rate(bytes, secs, &mut units);
    snprintf(
        res.as_mut_ptr(),
        ::std::mem::size_of::<[libc::c_char; 20]>() as libc::c_ulong,
        b"%.*f %s\0" as *const u8 as *const libc::c_char,
        if dlrate >= 99.95f64 {
            0 as libc::c_int
        } else if dlrate >= 9.995f64 {
            1 as libc::c_int
        } else {
            2 as libc::c_int
        },
        dlrate,
        if !opt.report_bps {
            rate_names[units as usize]
        } else {
            rate_names_bits[units as usize]
        },
    );
    return res.as_mut_ptr();
}
pub unsafe extern "C" fn calc_rate(
    mut bytes: wgint,
    mut secs: libc::c_double,
    mut units: *mut libc::c_int,
) -> libc::c_double {
    let mut dlrate: libc::c_double = 0.;
    let mut bibyte: libc::c_double = 0.;
    if !opt.report_bps {
        bibyte = 1024.0f64;
    } else {
        bibyte = 1000.0f64;
    }
    if secs == 0 as libc::c_int as libc::c_double {
        secs = ptimer_resolution() / 2.0f64;
    }
    dlrate = if secs != 0. {
        convert_to_bits(bytes) as libc::c_double / secs
    } else {
        0 as libc::c_int as libc::c_double
    };
    if dlrate < bibyte {
        *units = 0 as libc::c_int;
    } else if dlrate < bibyte * bibyte {
        *units = 1 as libc::c_int;
        dlrate /= bibyte;
    } else if dlrate < bibyte * bibyte * bibyte {
        *units = 2 as libc::c_int;
        dlrate /= bibyte * bibyte;
    } else if dlrate < bibyte * bibyte * bibyte * bibyte {
        *units = 3 as libc::c_int;
        dlrate /= bibyte * bibyte * bibyte;
    } else {
        *units = 4 as libc::c_int;
        dlrate /= bibyte * bibyte * bibyte * bibyte;
        if dlrate > 99.99f64 {
            dlrate = 99.99f64;
        }
    }
    return dlrate;
}
pub unsafe extern "C" fn retrieve_url(
    mut orig_parsed: *mut url,
    mut origurl: *const libc::c_char,
    mut file: *mut *mut libc::c_char,
    mut newloc: *mut *mut libc::c_char,
    mut refurl: *const libc::c_char,
    mut dt: *mut libc::c_int,
    mut recursive: bool,
    mut iri: *mut iri,
    mut register_status: bool,
) -> uerr_t {
    let mut current_block: u64;
    let mut result: uerr_t = NOCONERROR;
    let mut url: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut location_changed: bool = false;
    let mut iri_fallbacked: bool = 0 as libc::c_int != 0;
    let mut dummy: libc::c_int = 0;
    let mut mynewloc: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut proxy: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut u: *mut url = orig_parsed;
    let mut proxy_url: *mut url = 0 as *mut url;
    let mut up_error_code: libc::c_int = 0;
    let mut local_file: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut redirection_count: libc::c_int = 0 as libc::c_int;
    let mut method_suspended: bool = 0 as libc::c_int != 0;
    let mut saved_body_data: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut saved_method: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut saved_body_file_name: *mut libc::c_char = 0 as *mut libc::c_char;
    if dt.is_null() {
        dt = &mut dummy;
        dummy = 0 as libc::c_int;
    }
    url = xstrdup(origurl);
    if !newloc.is_null() {
        *newloc = 0 as *mut libc::c_char;
    }
    if !file.is_null() {
        *file = 0 as *mut libc::c_char;
    }
    if refurl.is_null() {
        refurl = opt.referer;
    }
    loop {
        result = NOCONERROR;
        mynewloc = 0 as *mut libc::c_char;
        rpl_free(local_file as *mut libc::c_void);
        local_file = 0 as *mut libc::c_char;
        proxy_url = 0 as *mut url;
        proxy = getproxy(u);
        if !proxy.is_null() {
            let mut pi: *mut iri = iri_new();
            set_uri_encoding(pi, opt.locale, 1 as libc::c_int != 0);
            (*pi).utf8_encode = 0 as libc::c_int != 0;
            proxy_url = url_parse(proxy, &mut up_error_code, pi, 1 as libc::c_int != 0);
            if proxy_url.is_null() {
                logprintf(
                    LOG_NOTQUIET,
                    dcgettext(
                        0 as *const libc::c_char,
                        b"Error parsing proxy URL %s: %s.\n\0" as *const u8
                            as *const libc::c_char,
                        5 as libc::c_int,
                    ),
                    proxy,
                    url_error(up_error_code),
                );
                rpl_free(url as *mut libc::c_void);
                url = 0 as *mut libc::c_char;
                rpl_free(proxy as *mut libc::c_void);
                proxy = 0 as *mut libc::c_char;
                iri_free(pi);
                if method_suspended {
                    opt.body_data = saved_body_data;
                    opt.body_file = saved_body_file_name;
                    opt.method = saved_method;
                    method_suspended = 0 as libc::c_int != 0;
                }
                result = PROXERR;
                if orig_parsed != u {
                    url_free(u);
                }
                current_block = 16538064955688531103;
                break;
            } else if (*proxy_url).scheme as libc::c_uint
                != SCHEME_HTTP as libc::c_int as libc::c_uint
                && (*proxy_url).scheme as libc::c_uint != (*u).scheme as libc::c_uint
            {
                logprintf(
                    LOG_NOTQUIET,
                    dcgettext(
                        0 as *const libc::c_char,
                        b"Error in proxy URL %s: Must be HTTP.\n\0" as *const u8
                            as *const libc::c_char,
                        5 as libc::c_int,
                    ),
                    proxy,
                );
                url_free(proxy_url);
                rpl_free(url as *mut libc::c_void);
                url = 0 as *mut libc::c_char;
                rpl_free(proxy as *mut libc::c_void);
                proxy = 0 as *mut libc::c_char;
                iri_free(pi);
                if method_suspended {
                    opt.body_data = saved_body_data;
                    opt.body_file = saved_body_file_name;
                    opt.method = saved_method;
                    method_suspended = 0 as libc::c_int != 0;
                }
                result = PROXERR;
                if orig_parsed != u {
                    url_free(u);
                }
                current_block = 16538064955688531103;
                break;
            } else {
                iri_free(pi);
                rpl_free(proxy as *mut libc::c_void);
                proxy = 0 as *mut libc::c_char;
            }
        }
        if (*u).scheme as libc::c_uint == SCHEME_HTTP as libc::c_int as libc::c_uint
            || !proxy_url.is_null()
                && (*proxy_url).scheme as libc::c_uint
                    == SCHEME_HTTP as libc::c_int as libc::c_uint
        {
            result = http_loop(
                u,
                orig_parsed,
                &mut mynewloc,
                &mut local_file,
                refurl,
                dt,
                proxy_url,
                iri,
            );
        } else if (*u).scheme as libc::c_uint
            == SCHEME_FTP as libc::c_int as libc::c_uint
        {
            let mut oldrec: bool = recursive;
            let mut glob: bool = opt.ftp_glob;
            if redirection_count != 0 {
                glob = 0 as libc::c_int != 0;
                oldrec = glob;
            }
            result = ftp_loop(
                u,
                orig_parsed,
                &mut local_file,
                dt,
                proxy_url,
                recursive,
                glob,
            );
            recursive = oldrec;
            if redirection_count != 0 && !local_file.is_null()
                && (*u).scheme as libc::c_uint
                    == SCHEME_FTP as libc::c_int as libc::c_uint
            {
                if has_html_suffix_p(local_file) {
                    *dt |= TEXTHTML as libc::c_int;
                }
            }
        }
        if !proxy_url.is_null() {
            url_free(proxy_url);
            proxy_url = 0 as *mut url;
        }
        location_changed = result as libc::c_uint
            == NEWLOCATION as libc::c_int as libc::c_uint
            || result as libc::c_uint
                == NEWLOCATION_KEEP_POST as libc::c_int as libc::c_uint;
        if location_changed {
            let mut construced_newloc: *mut libc::c_char = 0 as *mut libc::c_char;
            let mut newloc_parsed: *mut url = 0 as *mut url;
            rpl_free(local_file as *mut libc::c_void);
            local_file = 0 as *mut libc::c_char;
            construced_newloc = uri_merge(
                url,
                if !mynewloc.is_null() {
                    mynewloc as *const libc::c_char
                } else {
                    b"\0" as *const u8 as *const libc::c_char
                },
            );
            rpl_free(mynewloc as *mut libc::c_void);
            mynewloc = 0 as *mut libc::c_char;
            mynewloc = construced_newloc;
            (*iri).utf8_encode = opt.enable_iri;
            if !(opt.encoding_remote).is_null() {
                set_uri_encoding(iri, opt.encoding_remote, 1 as libc::c_int != 0);
            }
            set_content_encoding(iri, 0 as *const libc::c_char);
            rpl_free((*iri).orig_url as *mut libc::c_void);
            (*iri).orig_url = 0 as *mut libc::c_char;
            newloc_parsed = url_parse(
                mynewloc,
                &mut up_error_code,
                iri,
                1 as libc::c_int != 0,
            );
            if newloc_parsed.is_null() {
                logprintf(
                    LOG_NOTQUIET,
                    b"%s: %s.\n\0" as *const u8 as *const libc::c_char,
                    escnonprint_uri(mynewloc),
                    url_error(up_error_code),
                );
                if orig_parsed != u {
                    url_free(u);
                }
                rpl_free(url as *mut libc::c_void);
                url = 0 as *mut libc::c_char;
                rpl_free(mynewloc as *mut libc::c_void);
                mynewloc = 0 as *mut libc::c_char;
                if method_suspended {
                    opt.body_data = saved_body_data;
                    opt.body_file = saved_body_file_name;
                    opt.method = saved_method;
                    method_suspended = 0 as libc::c_int != 0;
                }
                current_block = 16538064955688531103;
                break;
            } else {
                rpl_free(mynewloc as *mut libc::c_void);
                mynewloc = 0 as *mut libc::c_char;
                mynewloc = xstrdup((*newloc_parsed).url);
                redirection_count += 1;
                if redirection_count > opt.max_redirect {
                    logprintf(
                        LOG_NOTQUIET,
                        dcgettext(
                            0 as *const libc::c_char,
                            b"%d redirections exceeded.\n\0" as *const u8
                                as *const libc::c_char,
                            5 as libc::c_int,
                        ),
                        opt.max_redirect,
                    );
                    url_free(newloc_parsed);
                    if orig_parsed != u {
                        url_free(u);
                    }
                    rpl_free(url as *mut libc::c_void);
                    url = 0 as *mut libc::c_char;
                    rpl_free(mynewloc as *mut libc::c_void);
                    mynewloc = 0 as *mut libc::c_char;
                    if method_suspended {
                        opt.body_data = saved_body_data;
                        opt.body_file = saved_body_file_name;
                        opt.method = saved_method;
                        method_suspended = 0 as libc::c_int != 0;
                    }
                    result = WRONGCODE;
                    current_block = 16538064955688531103;
                    break;
                } else {
                    rpl_free(url as *mut libc::c_void);
                    url = 0 as *mut libc::c_char;
                    url = mynewloc;
                    if orig_parsed != u {
                        url_free(u);
                    }
                    u = newloc_parsed;
                    if result as libc::c_uint
                        != NEWLOCATION_KEEP_POST as libc::c_int as libc::c_uint
                        && !method_suspended
                    {
                        method_suspended = 1 as libc::c_int != 0;
                        saved_body_data = opt.body_data;
                        saved_body_file_name = opt.body_file;
                        saved_method = opt.method;
                        opt.body_data = 0 as *mut libc::c_char;
                        opt.body_file = 0 as *mut libc::c_char;
                        opt.method = 0 as *mut libc::c_char;
                    }
                }
            }
        } else {
            rpl_free(mynewloc as *mut libc::c_void);
            mynewloc = 0 as *mut libc::c_char;
            if !(*dt & RETROKF as libc::c_int == 0
                && (*iri).utf8_encode as libc::c_int != 0)
            {
                current_block = 16098508315428432339;
                break;
            }
            (*iri).utf8_encode = 0 as libc::c_int != 0;
            if orig_parsed != u {
                url_free(u);
            }
            u = url_parse(origurl, 0 as *mut libc::c_int, iri, 1 as libc::c_int != 0);
            if !u.is_null() {
                if strcmp((*u).url, (*orig_parsed).url) != 0 {
                    if opt.debug as libc::c_long != 0 {
                        debug_logprintf(
                            b"[IRI fallbacking to non-utf8 for %s\n\0" as *const u8
                                as *const libc::c_char,
                            quote(url),
                        );
                    }
                    rpl_free(url as *mut libc::c_void);
                    url = 0 as *mut libc::c_char;
                    url = xstrdup((*u).url);
                    iri_fallbacked = 1 as libc::c_int != 0;
                } else {
                    if opt.debug as libc::c_long != 0 {
                        debug_logprintf(
                            b"[Needn't fallback to non-utf8 for %s\n\0" as *const u8
                                as *const libc::c_char,
                            quote(url),
                        );
                    }
                    current_block = 16098508315428432339;
                    break;
                }
            } else {
                if opt.debug as libc::c_long != 0 {
                    debug_logprintf(
                        b"[Couldn't fallback to non-utf8 for %s\n\0" as *const u8
                            as *const libc::c_char,
                        quote(url),
                    );
                }
                current_block = 16098508315428432339;
                break;
            }
        }
    }
    match current_block {
        16098508315428432339 => {
            if !local_file.is_null() && !u.is_null()
                && (*dt & RETROKF as libc::c_int != 0
                    || opt.content_on_error as libc::c_int != 0)
            {
                register_download((*u).url, local_file);
                if !opt.spider && redirection_count != 0
                    && 0 as libc::c_int != strcmp(origurl, (*u).url)
                {
                    register_redirection(origurl, (*u).url);
                }
                if *dt & TEXTHTML as libc::c_int != 0 {
                    register_html(local_file);
                }
                if *dt & TEXTCSS as libc::c_int != 0 {
                    register_css(local_file);
                }
            }
            if !file.is_null() {
                *file = if !local_file.is_null() {
                    local_file
                } else {
                    0 as *mut libc::c_char
                };
            } else {
                rpl_free(local_file as *mut libc::c_void);
                local_file = 0 as *mut libc::c_char;
            }
            if orig_parsed != u {
                url_free(u);
            }
            if redirection_count != 0 || iri_fallbacked as libc::c_int != 0 {
                if !newloc.is_null() {
                    *newloc = url;
                } else {
                    rpl_free(url as *mut libc::c_void);
                    url = 0 as *mut libc::c_char;
                }
            } else {
                if !newloc.is_null() {
                    *newloc = 0 as *mut libc::c_char;
                }
                rpl_free(url as *mut libc::c_void);
                url = 0 as *mut libc::c_char;
            }
            if method_suspended {
                opt.body_data = saved_body_data;
                opt.body_file = saved_body_file_name;
                opt.method = saved_method;
                method_suspended = 0 as libc::c_int != 0;
            }
        }
        _ => {}
    }
    if register_status {
        inform_exit_status(result);
    }
    return result;
}
pub unsafe extern "C" fn retrieve_from_file(
    mut file: *const libc::c_char,
    mut html: bool,
    mut count: *mut libc::c_int,
) -> uerr_t {
    let mut status: uerr_t = NOCONERROR;
    let mut url_list: *mut urlpos = 0 as *mut urlpos;
    let mut cur_url: *mut urlpos = 0 as *mut urlpos;
    let mut iri: *mut iri = iri_new();
    let mut input_file: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut url_file: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut url: *const libc::c_char = file;
    status = RETROK;
    *count = 0 as libc::c_int;
    set_uri_encoding(iri, opt.locale, 1 as libc::c_int != 0);
    set_content_encoding(iri, opt.locale);
    if url_valid_scheme(url) {
        let mut dt: libc::c_int = 0;
        let mut url_err: libc::c_int = 0;
        let mut url_parsed: *mut url = url_parse(
            url,
            &mut url_err,
            iri,
            1 as libc::c_int != 0,
        );
        if url_parsed.is_null() {
            logprintf(
                LOG_NOTQUIET,
                b"%s: %s.\n\0" as *const u8 as *const libc::c_char,
                url,
                url_error(url_err),
            );
            iri_free(iri);
            return URLERROR;
        }
        if (opt.base_href).is_null() {
            opt.base_href = xstrdup(url);
        }
        status = retrieve_url(
            url_parsed,
            url,
            &mut url_file,
            0 as *mut *mut libc::c_char,
            0 as *const libc::c_char,
            &mut dt,
            0 as libc::c_int != 0,
            iri,
            1 as libc::c_int != 0,
        );
        url_free(url_parsed);
        if url_file.is_null()
            || status as libc::c_uint != RETROK as libc::c_int as libc::c_uint
        {
            return status;
        }
        if dt & TEXTHTML as libc::c_int != 0 {
            html = 1 as libc::c_int != 0;
        }
        if (*iri).content_encoding != opt.locale as *mut libc::c_char {
            set_uri_encoding(iri, (*iri).content_encoding, 0 as libc::c_int != 0);
        }
        (*iri).utf8_encode = opt.enable_iri;
        rpl_free((*iri).orig_url as *mut libc::c_void);
        (*iri).orig_url = 0 as *mut libc::c_char;
        input_file = url_file;
    } else {
        input_file = file as *mut libc::c_char;
    }
    url_list = if html as libc::c_int != 0 {
        get_urls_html(input_file, 0 as *const libc::c_char, 0 as *mut bool, iri)
    } else {
        get_urls_file(input_file)
    };
    rpl_free(url_file as *mut libc::c_void);
    url_file = 0 as *mut libc::c_char;
    cur_url = url_list;
    while !cur_url.is_null() {
        let mut filename: *mut libc::c_char = 0 as *mut libc::c_char;
        let mut new_file: *mut libc::c_char = 0 as *mut libc::c_char;
        let mut proxy: *mut libc::c_char = 0 as *mut libc::c_char;
        let mut dt_0: libc::c_int = 0 as libc::c_int;
        let mut tmpiri: *mut iri = iri_dup(iri);
        let mut parsed_url: *mut url = 0 as *mut url;
        if !((*cur_url).ignore_when_downloading() != 0) {
            if opt.quota != 0 && total_downloaded_bytes > opt.quota {
                status = QUOTEXC;
                break;
            } else {
                parsed_url = url_parse(
                    (*(*cur_url).url).url,
                    0 as *mut libc::c_int,
                    tmpiri,
                    1 as libc::c_int != 0,
                );
                proxy = getproxy((*cur_url).url);
                if (opt.recursive as libc::c_int != 0
                    || opt.page_requisites as libc::c_int != 0)
                    && ((*(*cur_url).url).scheme as libc::c_uint
                        != SCHEME_FTP as libc::c_int as libc::c_uint || !proxy.is_null())
                {
                    let mut old_follow_ftp: libc::c_int = opt.follow_ftp as libc::c_int;
                    if (*(*cur_url).url).scheme as libc::c_uint
                        == SCHEME_FTP as libc::c_int as libc::c_uint
                    {
                        opt.follow_ftp = 1 as libc::c_int != 0;
                    }
                    status = retrieve_tree(
                        if !parsed_url.is_null() { parsed_url } else { (*cur_url).url },
                        tmpiri,
                    );
                    opt.follow_ftp = old_follow_ftp != 0;
                } else {
                    status = retrieve_url(
                        if !parsed_url.is_null() { parsed_url } else { (*cur_url).url },
                        (*(*cur_url).url).url,
                        &mut filename,
                        &mut new_file,
                        0 as *const libc::c_char,
                        &mut dt_0,
                        opt.recursive,
                        tmpiri,
                        1 as libc::c_int != 0,
                    );
                }
                rpl_free(proxy as *mut libc::c_void);
                proxy = 0 as *mut libc::c_char;
                if !parsed_url.is_null() {
                    url_free(parsed_url);
                }
                if !filename.is_null() && opt.delete_after as libc::c_int != 0
                    && file_exists_p(filename, 0 as *mut file_stats_t) as libc::c_int
                        != 0
                {
                    if opt.debug as libc::c_long != 0 {
                        debug_logprintf(
                            b"Removing file due to --delete-after in retrieve_from_file():\n\0"
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
                        filename,
                    );
                    if unlink(filename) != 0 {
                        logprintf(
                            LOG_NOTQUIET,
                            b"Failed to unlink %s: (%d) %s\n\0" as *const u8
                                as *const libc::c_char,
                            filename,
                            *__errno_location(),
                            strerror(*__errno_location()),
                        );
                    }
                    dt_0 &= !(RETROKF as libc::c_int);
                }
                rpl_free(new_file as *mut libc::c_void);
                new_file = 0 as *mut libc::c_char;
                rpl_free(filename as *mut libc::c_void);
                filename = 0 as *mut libc::c_char;
                iri_free(tmpiri);
            }
        }
        cur_url = (*cur_url).next;
        *count += 1;
        *count;
    }
    free_urlpos(url_list);
    iri_free(iri);
    return status;
}
pub unsafe extern "C" fn printwhat(mut n1: libc::c_int, mut n2: libc::c_int) {
    logputs(
        LOG_VERBOSE,
        if n1 == n2 {
            dcgettext(
                0 as *const libc::c_char,
                b"Giving up.\n\n\0" as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            )
        } else {
            dcgettext(
                0 as *const libc::c_char,
                b"Retrying.\n\n\0" as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            )
        },
    );
}
pub unsafe extern "C" fn sleep_between_retrievals(mut count: libc::c_int) {
    static mut first_retrieval: bool = 1 as libc::c_int != 0;
    if first_retrieval {
        first_retrieval = 0 as libc::c_int != 0;
        return;
    }
    if opt.waitretry != 0. && count > 1 as libc::c_int {
        if count as libc::c_double <= opt.waitretry {
            xsleep((count - 1 as libc::c_int) as libc::c_double);
        } else {
            xsleep(opt.waitretry);
        }
    } else if opt.wait != 0. {
        if !opt.random_wait || count > 1 as libc::c_int {
            xsleep(opt.wait);
        } else {
            let mut waitsecs: libc::c_double = (0.5f64 + random_float()) * opt.wait;
            if opt.debug as libc::c_long != 0 {
                debug_logprintf(
                    b"sleep_between_retrievals: avg=%f,sleep=%f\n\0" as *const u8
                        as *const libc::c_char,
                    opt.wait,
                    waitsecs,
                );
            }
            xsleep(waitsecs);
        }
    }
}
pub unsafe extern "C" fn free_urlpos(mut l: *mut urlpos) {
    while !l.is_null() {
        let mut next: *mut urlpos = (*l).next;
        if !((*l).url).is_null() {
            url_free((*l).url);
        }
        rpl_free((*l).local_name as *mut libc::c_void);
        (*l).local_name = 0 as *mut libc::c_char;
        rpl_free(l as *mut libc::c_void);
        l = 0 as *mut urlpos;
        l = next;
    }
}
pub unsafe extern "C" fn rotate_backups(mut fname: *const libc::c_char) {
    let mut from: [libc::c_char; 1024] = [0; 1024];
    let mut to: [libc::c_char; 1024] = [0; 1024];
    let mut sb: stat = stat {
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
    let mut overflow: bool = false;
    let mut i: libc::c_int = 0;
    if stat(fname, &mut sb) == 0 as libc::c_int {
        if (sb.st_mode & 0o170000 as libc::c_int as libc::c_uint
            == 0o100000 as libc::c_int as libc::c_uint) as libc::c_int
            == 0 as libc::c_int
        {
            return;
        }
    }
    i = opt.backups;
    while i > 1 as libc::c_int {
        overflow = snprintf(
            to.as_mut_ptr(),
            1024 as libc::c_int as libc::c_ulong,
            b"%s%s%d\0" as *const u8 as *const libc::c_char,
            fname,
            b".\0" as *const u8 as *const libc::c_char,
            i,
        ) as libc::c_uint >= 1024 as libc::c_int as libc::c_uint;
        overflow = (overflow as libc::c_int
            | (snprintf(
                from.as_mut_ptr(),
                1024 as libc::c_int as libc::c_ulong,
                b"%s%s%d\0" as *const u8 as *const libc::c_char,
                fname,
                b".\0" as *const u8 as *const libc::c_char,
                i - 1 as libc::c_int,
            ) as libc::c_uint >= 1024 as libc::c_int as libc::c_uint) as libc::c_int) != 0;
        if overflow {
            *__errno_location() = 36 as libc::c_int;
        }
        if overflow as libc::c_int != 0
            || rename(from.as_mut_ptr(), to.as_mut_ptr()) != 0
        {
            if *__errno_location() != 2 as libc::c_int {
                logprintf(
                    LOG_NOTQUIET,
                    b"Failed to rename %s to %s: (%d) %s\n\0" as *const u8
                        as *const libc::c_char,
                    from.as_mut_ptr(),
                    to.as_mut_ptr(),
                    *__errno_location(),
                    strerror(*__errno_location()),
                );
            }
        }
        i -= 1;
        i;
    }
    overflow = snprintf(
        to.as_mut_ptr(),
        1024 as libc::c_int as libc::c_ulong,
        b"%s%s%d\0" as *const u8 as *const libc::c_char,
        fname,
        b".\0" as *const u8 as *const libc::c_char,
        1 as libc::c_int,
    ) as libc::c_uint >= 1024 as libc::c_int as libc::c_uint;
    if overflow {
        *__errno_location() = 36 as libc::c_int;
    }
    if overflow as libc::c_int != 0 || rename(fname, to.as_mut_ptr()) != 0 {
        if *__errno_location() != 2 as libc::c_int {
            logprintf(
                LOG_NOTQUIET,
                b"Failed to rename %s to %s: (%d) %s\n\0" as *const u8
                    as *const libc::c_char,
                from.as_mut_ptr(),
                to.as_mut_ptr(),
                *__errno_location(),
                strerror(*__errno_location()),
            );
        }
    }
}
unsafe extern "C" fn getproxy(mut u: *mut url) -> *mut libc::c_char {
    let mut proxy: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut rewritten_url: *mut libc::c_char = 0 as *mut libc::c_char;
    if !opt.use_proxy {
        return 0 as *mut libc::c_char;
    }
    if no_proxy_match((*u).host, opt.no_proxy as *mut *const libc::c_char) {
        return 0 as *mut libc::c_char;
    }
    match (*u).scheme as libc::c_uint {
        0 => {
            proxy = if !(opt.http_proxy).is_null() {
                opt.http_proxy
            } else {
                getenv(b"http_proxy\0" as *const u8 as *const libc::c_char)
            };
        }
        1 => {
            proxy = if !(opt.ftp_proxy).is_null() {
                opt.ftp_proxy
            } else {
                getenv(b"ftp_proxy\0" as *const u8 as *const libc::c_char)
            };
        }
        2 | _ => {}
    }
    if proxy.is_null() || *proxy == 0 {
        return 0 as *mut libc::c_char;
    }
    rewritten_url = rewrite_shorthand_url(proxy);
    if !rewritten_url.is_null() {
        return rewritten_url;
    }
    return strdup(proxy);
}
pub unsafe extern "C" fn url_uses_proxy(mut u: *mut url) -> bool {
    let mut ret: bool = false;
    let mut proxy: *mut libc::c_char = 0 as *mut libc::c_char;
    if u.is_null() {
        return 0 as libc::c_int != 0;
    }
    proxy = getproxy(u);
    ret = !proxy.is_null();
    rpl_free(proxy as *mut libc::c_void);
    proxy = 0 as *mut libc::c_char;
    return ret;
}
unsafe extern "C" fn no_proxy_match(
    mut host: *const libc::c_char,
    mut no_proxy: *mut *const libc::c_char,
) -> bool {
    if no_proxy.is_null() {
        return 0 as libc::c_int != 0
    } else {
        return sufmatch(no_proxy, host)
    };
}
pub unsafe extern "C" fn set_local_file(
    mut file: *mut *const libc::c_char,
    mut default_file: *const libc::c_char,
) {
    if !(opt.output_document).is_null() {
        if output_stream_regular {
            *file = opt.output_document;
        }
    } else {
        *file = default_file;
    };
}
pub unsafe extern "C" fn input_file_url(mut input_file: *const libc::c_char) -> bool {
    static mut first: bool = 1 as libc::c_int != 0;
    if !input_file.is_null() && url_has_scheme(input_file) as libc::c_int != 0
        && first as libc::c_int != 0
    {
        first = 0 as libc::c_int != 0;
        return 1 as libc::c_int != 0;
    } else {
        return 0 as libc::c_int != 0
    };
}
