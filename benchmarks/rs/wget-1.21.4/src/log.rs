use ::libc;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    fn abort() -> !;
    fn exit(_: libc::c_int) -> !;
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
    fn strchr(_: *const libc::c_char, _: libc::c_int) -> *mut libc::c_char;
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    fn strerror(_: libc::c_int) -> *mut libc::c_char;
    fn rpl_free(_: *mut libc::c_void);
    static mut opt: options;
    fn xmalloc(s: size_t) -> *mut libc::c_void;
    fn xrealloc(p: *mut libc::c_void, s: size_t) -> *mut libc::c_void;
    static mut stdout: *mut FILE;
    static mut stderr: *mut FILE;
    fn fclose(__stream: *mut FILE) -> libc::c_int;
    fn fprintf(_: *mut FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn vfprintf(
        _: *mut FILE,
        _: *const libc::c_char,
        _: ::std::ffi::VaList,
    ) -> libc::c_int;
    fn vsnprintf(
        _: *mut libc::c_char,
        _: libc::c_ulong,
        _: *const libc::c_char,
        _: ::std::ffi::VaList,
    ) -> libc::c_int;
    fn fputs(__s: *const libc::c_char, __stream: *mut FILE) -> libc::c_int;
    fn fileno(__stream: *mut FILE) -> libc::c_int;
    fn rpl_fflush(gl_stream: *mut FILE) -> libc::c_int;
    fn rpl_fopen(filename: *const libc::c_char, mode: *const libc::c_char) -> *mut FILE;
    fn quote(arg: *const libc::c_char) -> *const libc::c_char;
    static mut exec_name: *const libc::c_char;
    fn getpgrp() -> __pid_t;
    fn isatty(__fd: libc::c_int) -> libc::c_int;
    fn tcgetpgrp(__fd: libc::c_int) -> __pid_t;
    fn __errno_location() -> *mut libc::c_int;
    fn strdupdelim(_: *const libc::c_char, _: *const libc::c_char) -> *mut libc::c_char;
    fn unique_create(
        _: *const libc::c_char,
        _: bool,
        _: *mut *mut libc::c_char,
    ) -> *mut FILE;
}
pub type __builtin_va_list = [__va_list_tag; 1];
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __va_list_tag {
    pub gp_offset: libc::c_uint,
    pub fp_offset: libc::c_uint,
    pub overflow_arg_area: *mut libc::c_void,
    pub reg_save_area: *mut libc::c_void,
}
pub type __int64_t = libc::c_long;
pub type __off_t = libc::c_long;
pub type __off64_t = libc::c_long;
pub type __pid_t = libc::c_int;
pub type pid_t = __pid_t;
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
pub type va_list = __builtin_va_list;
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
pub const WGET_EXIT_GENERIC_ERROR: C2RustUnnamed_3 = 1;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct logvprintf_state {
    pub bigmsg: *mut libc::c_char,
    pub expected_size: libc::c_int,
    pub allocated: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct log_ln {
    pub static_line: [libc::c_char; 129],
    pub malloced_line: *mut libc::c_char,
    pub content: *mut libc::c_char,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ringel {
    pub buffer: *mut libc::c_char,
    pub size: libc::c_int,
}
pub type C2RustUnnamed_3 = libc::c_uint;
pub const WGET_EXIT_UNKNOWN: C2RustUnnamed_3 = 9;
pub const WGET_EXIT_SERVER_ERROR: C2RustUnnamed_3 = 8;
pub const WGET_EXIT_PROTOCOL_ERROR: C2RustUnnamed_3 = 7;
pub const WGET_EXIT_SERVER_AUTH_FAIL: C2RustUnnamed_3 = 6;
pub const WGET_EXIT_SSL_AUTH_FAIL: C2RustUnnamed_3 = 5;
pub const WGET_EXIT_NETWORK_FAIL: C2RustUnnamed_3 = 4;
pub const WGET_EXIT_IO_FAIL: C2RustUnnamed_3 = 3;
pub const WGET_EXIT_PARSE_ERROR: C2RustUnnamed_3 = 2;
pub const WGET_EXIT_SUCCESS: C2RustUnnamed_3 = 0;
#[inline]
unsafe extern "C" fn c_isprint(mut c: libc::c_int) -> bool {
    match c {
        32 | 48 | 49 | 50 | 51 | 52 | 53 | 54 | 55 | 56 | 57 | 97 | 98 | 99 | 100 | 101
        | 102 | 103 | 104 | 105 | 106 | 107 | 108 | 109 | 110 | 111 | 112 | 113 | 114
        | 115 | 116 | 117 | 118 | 119 | 120 | 121 | 122 | 33 | 34 | 35 | 36 | 37 | 38
        | 39 | 40 | 41 | 42 | 43 | 44 | 45 | 46 | 47 | 58 | 59 | 60 | 61 | 62 | 63 | 64
        | 91 | 92 | 93 | 94 | 95 | 96 | 123 | 124 | 125 | 126 | 65 | 66 | 67 | 68 | 69
        | 70 | 71 | 72 | 73 | 74 | 75 | 76 | 77 | 78 | 79 | 80 | 81 | 82 | 83 | 84 | 85
        | 86 | 87 | 88 | 89 | 90 => return 1 as libc::c_int != 0,
        _ => return 0 as libc::c_int != 0,
    };
}
static mut logfp: *mut FILE = 0 as *const FILE as *mut FILE;
static mut stdlogfp: *mut FILE = 0 as *const FILE as *mut FILE;
static mut filelogfp: *mut FILE = 0 as *const FILE as *mut FILE;
static mut logfile: *mut libc::c_char = 0 as *const libc::c_char as *mut libc::c_char;
static mut shell_is_interactive: libc::c_int = 0;
static mut warclogfp: *mut FILE = 0 as *const FILE as *mut FILE;
static mut inhibit_logging: bool = false;
static mut save_context_p: bool = false;
static mut flush_log_p: bool = 1 as libc::c_int != 0;
static mut needs_flushing: bool = false;
static mut log_lines: [log_ln; 24] = [log_ln {
    static_line: [0; 129],
    malloced_line: 0 as *const libc::c_char as *mut libc::c_char,
    content: 0 as *const libc::c_char as *mut libc::c_char,
}; 24];
static mut log_line_current: libc::c_int = -(1 as libc::c_int);
static mut trailing_line: bool = false;
unsafe extern "C" fn free_log_line(mut num: libc::c_int) {
    let mut ln: *mut log_ln = log_lines.as_mut_ptr().offset(num as isize);
    rpl_free((*ln).malloced_line as *mut libc::c_void);
    (*ln).malloced_line = 0 as *mut libc::c_char;
    (*ln).content = 0 as *mut libc::c_char;
}
unsafe extern "C" fn saved_append_1(
    mut start: *const libc::c_char,
    mut end: *const libc::c_char,
) {
    let mut len: libc::c_int = end.offset_from(start) as libc::c_long as libc::c_int;
    if len == 0 {
        return;
    }
    if !trailing_line {
        let mut ln: *mut log_ln = 0 as *mut log_ln;
        if log_line_current == -(1 as libc::c_int) {
            log_line_current = 0 as libc::c_int;
        } else {
            free_log_line(log_line_current);
        }
        ln = log_lines.as_mut_ptr().offset(log_line_current as isize);
        if len > 128 as libc::c_int {
            (*ln).malloced_line = strdupdelim(start, end);
            (*ln).content = (*ln).malloced_line;
        } else {
            memcpy(
                ((*ln).static_line).as_mut_ptr() as *mut libc::c_void,
                start as *const libc::c_void,
                len as libc::c_ulong,
            );
            (*ln).static_line[len as usize] = '\0' as i32 as libc::c_char;
            (*ln).content = ((*ln).static_line).as_mut_ptr();
        }
    } else {
        let mut ln_0: *mut log_ln = log_lines
            .as_mut_ptr()
            .offset(log_line_current as isize);
        if !((*ln_0).malloced_line).is_null() {
            let mut old_len: libc::c_int = strlen((*ln_0).malloced_line) as libc::c_int;
            (*ln_0)
                .malloced_line = xrealloc(
                (*ln_0).malloced_line as *mut libc::c_void,
                (old_len + len + 1 as libc::c_int) as size_t,
            ) as *mut libc::c_char;
            memcpy(
                ((*ln_0).malloced_line).offset(old_len as isize) as *mut libc::c_void,
                start as *const libc::c_void,
                len as libc::c_ulong,
            );
            *((*ln_0).malloced_line)
                .offset((old_len + len) as isize) = '\0' as i32 as libc::c_char;
            (*ln_0).content = (*ln_0).malloced_line;
        } else {
            let mut old_len_0: libc::c_int = strlen(((*ln_0).static_line).as_mut_ptr())
                as libc::c_int;
            if old_len_0 + len > 128 as libc::c_int {
                (*ln_0)
                    .malloced_line = xmalloc(
                    (old_len_0 + len + 1 as libc::c_int) as size_t,
                ) as *mut libc::c_char;
                memcpy(
                    (*ln_0).malloced_line as *mut libc::c_void,
                    ((*ln_0).static_line).as_mut_ptr() as *const libc::c_void,
                    old_len_0 as libc::c_ulong,
                );
                memcpy(
                    ((*ln_0).malloced_line).offset(old_len_0 as isize)
                        as *mut libc::c_void,
                    start as *const libc::c_void,
                    len as libc::c_ulong,
                );
                *((*ln_0).malloced_line)
                    .offset((old_len_0 + len) as isize) = '\0' as i32 as libc::c_char;
                (*ln_0).content = (*ln_0).malloced_line;
            } else {
                memcpy(
                    ((*ln_0).static_line).as_mut_ptr().offset(old_len_0 as isize)
                        as *mut libc::c_void,
                    start as *const libc::c_void,
                    len as libc::c_ulong,
                );
                (*ln_0)
                    .static_line[(old_len_0 + len)
                    as usize] = '\0' as i32 as libc::c_char;
                (*ln_0).content = ((*ln_0).static_line).as_mut_ptr();
            }
        }
    }
    trailing_line = !(*end.offset(-(1 as libc::c_int) as isize) as libc::c_int
        == '\n' as i32);
    if !trailing_line {
        log_line_current += 1;
        if log_line_current >= 24 as libc::c_int {
            log_line_current = 0 as libc::c_int;
        }
    }
}
unsafe extern "C" fn saved_append(mut s: *const libc::c_char) {
    while *s != 0 {
        let mut end: *const libc::c_char = strchr(s, '\n' as i32);
        if end.is_null() {
            end = s.offset(strlen(s) as isize);
        } else {
            end = end.offset(1);
            end;
        }
        saved_append_1(s, end);
        s = end;
    }
}
unsafe extern "C" fn get_log_fp() -> *mut FILE {
    if inhibit_logging {
        return 0 as *mut FILE;
    }
    if !logfp.is_null() {
        return logfp;
    }
    return stderr;
}
unsafe extern "C" fn get_progress_fp() -> *mut FILE {
    if opt.show_progress == 1 as libc::c_int {
        return stderr;
    }
    return get_log_fp();
}
unsafe extern "C" fn get_warc_log_fp() -> *mut FILE {
    if inhibit_logging {
        return 0 as *mut FILE;
    }
    if !warclogfp.is_null() {
        return warclogfp;
    }
    if !logfp.is_null() {
        return 0 as *mut FILE;
    }
    return stderr;
}
pub unsafe extern "C" fn log_set_warc_log_fp(mut fp: *mut FILE) {
    warclogfp = fp;
}
pub unsafe extern "C" fn logputs(mut o: log_options, mut s: *const libc::c_char) {
    let mut fp: *mut FILE = 0 as *mut FILE;
    let mut warcfp: *mut FILE = 0 as *mut FILE;
    let mut errno_save: libc::c_int = *__errno_location();
    check_redirect_output();
    if o as libc::c_uint == LOG_PROGRESS as libc::c_int as libc::c_uint {
        fp = get_progress_fp();
    } else {
        fp = get_log_fp();
    }
    *__errno_location() = errno_save;
    if fp.is_null() {
        return;
    }
    warcfp = get_warc_log_fp();
    *__errno_location() = errno_save;
    match o as libc::c_uint {
        4 => {
            if opt.show_progress == 0 {
                return;
            }
        }
        1 => {
            if opt.quiet {
                return;
            }
        }
        2 => {
            if opt.verbose != 0 || opt.quiet as libc::c_int != 0 {
                return;
            }
        }
        0 => {
            if opt.verbose == 0 {
                return;
            }
        }
        3 | _ => {}
    }
    fputs(s, fp);
    if !warcfp.is_null() {
        fputs(s, warcfp);
    }
    if save_context_p {
        saved_append(s);
    }
    if flush_log_p {
        logflush();
    } else {
        needs_flushing = 1 as libc::c_int != 0;
    }
    *__errno_location() = errno_save;
}
unsafe extern "C" fn log_vprintf_internal(
    mut state: *mut logvprintf_state,
    mut fmt: *const libc::c_char,
    mut args: ::std::ffi::VaList,
) -> bool {
    let mut smallmsg: [libc::c_char; 128] = [0; 128];
    let mut write_ptr: *mut libc::c_char = smallmsg.as_mut_ptr();
    let mut available_size: libc::c_int = ::std::mem::size_of::<[libc::c_char; 128]>()
        as libc::c_ulong as libc::c_int;
    let mut numwritten: libc::c_int = 0;
    let mut fp: *mut FILE = get_log_fp();
    let mut warcfp: *mut FILE = get_warc_log_fp();
    if fp.is_null() {
        return 0 as libc::c_int != 0;
    }
    if !save_context_p && warcfp.is_null() {
        vfprintf(fp, fmt, args.as_va_list());
    } else {
        if (*state).allocated != 0 as libc::c_int {
            write_ptr = (*state).bigmsg;
            available_size = (*state).allocated;
        }
        numwritten = vsnprintf(
            write_ptr,
            available_size as libc::c_ulong,
            fmt,
            args.as_va_list(),
        );
        if numwritten == -(1 as libc::c_int) {
            let mut newsize: libc::c_int = available_size << 1 as libc::c_int;
            (*state)
                .bigmsg = xrealloc(
                (*state).bigmsg as *mut libc::c_void,
                newsize as size_t,
            ) as *mut libc::c_char;
            (*state).allocated = newsize;
            return 0 as libc::c_int != 0;
        } else if numwritten >= available_size {
            let mut newsize_0: libc::c_int = numwritten + 1 as libc::c_int;
            (*state)
                .bigmsg = xrealloc(
                (*state).bigmsg as *mut libc::c_void,
                newsize_0 as size_t,
            ) as *mut libc::c_char;
            (*state).allocated = newsize_0;
            return 0 as libc::c_int != 0;
        }
        if save_context_p {
            saved_append(write_ptr);
        }
        fputs(write_ptr, fp);
        if !warcfp.is_null() && warcfp != fp {
            fputs(write_ptr, warcfp);
        }
        rpl_free((*state).bigmsg as *mut libc::c_void);
        (*state).bigmsg = 0 as *mut libc::c_char;
    }
    if flush_log_p {
        logflush();
    } else {
        needs_flushing = 1 as libc::c_int != 0;
    }
    return 1 as libc::c_int != 0;
}
pub unsafe extern "C" fn logflush() {
    let mut fp: *mut FILE = get_log_fp();
    let mut warcfp: *mut FILE = get_warc_log_fp();
    if !fp.is_null() {
        rpl_fflush(fp);
    }
    if !warcfp.is_null() {
        rpl_fflush(warcfp);
    }
    needs_flushing = 0 as libc::c_int != 0;
}
pub unsafe extern "C" fn log_set_flush(mut flush: bool) {
    if flush as libc::c_int == flush_log_p as libc::c_int {
        return;
    }
    if flush as libc::c_int == 0 as libc::c_int {
        flush_log_p = 0 as libc::c_int != 0;
    } else {
        if needs_flushing {
            logflush();
        }
        flush_log_p = 1 as libc::c_int != 0;
    };
}
pub unsafe extern "C" fn log_set_save_context(mut savep: bool) -> bool {
    let mut old: bool = save_context_p;
    save_context_p = savep;
    return old;
}
pub unsafe extern "C" fn logprintf(
    mut o: log_options,
    mut fmt: *const libc::c_char,
    mut args: ...
) {
    let mut args_0: ::std::ffi::VaListImpl;
    let mut lpstate: logvprintf_state = logvprintf_state {
        bigmsg: 0 as *mut libc::c_char,
        expected_size: 0,
        allocated: 0,
    };
    let mut done: bool = false;
    let mut errno_saved: libc::c_int = *__errno_location();
    match o as libc::c_uint {
        4 => {
            if opt.show_progress == 0 {
                return;
            }
        }
        1 => {
            if opt.quiet {
                return;
            }
        }
        2 => {
            if opt.verbose != 0 || opt.quiet as libc::c_int != 0 {
                return;
            }
        }
        0 => {
            if opt.verbose == 0 {
                return;
            }
        }
        3 | _ => {}
    }
    check_redirect_output();
    *__errno_location() = errno_saved;
    if inhibit_logging {
        return;
    }
    memset(
        &mut lpstate as *mut logvprintf_state as *mut libc::c_void,
        '\0' as i32,
        ::std::mem::size_of::<logvprintf_state>() as libc::c_ulong,
    );
    *__errno_location() = 0 as libc::c_int;
    loop {
        args_0 = args.clone();
        done = log_vprintf_internal(&mut lpstate, fmt, args_0.as_va_list());
        if done as libc::c_int != 0 && *__errno_location() == 32 as libc::c_int {
            exit(WGET_EXIT_GENERIC_ERROR as libc::c_int);
        }
        if done {
            break;
        }
    }
    *__errno_location() = errno_saved;
}
pub unsafe extern "C" fn debug_logprintf(mut fmt: *const libc::c_char, mut args: ...) {
    if opt.debug {
        let mut args_0: ::std::ffi::VaListImpl;
        let mut lpstate: logvprintf_state = logvprintf_state {
            bigmsg: 0 as *mut libc::c_char,
            expected_size: 0,
            allocated: 0,
        };
        let mut done: bool = false;
        check_redirect_output();
        if inhibit_logging {
            return;
        }
        memset(
            &mut lpstate as *mut logvprintf_state as *mut libc::c_void,
            '\0' as i32,
            ::std::mem::size_of::<logvprintf_state>() as libc::c_ulong,
        );
        loop {
            args_0 = args.clone();
            done = log_vprintf_internal(&mut lpstate, fmt, args_0.as_va_list());
            if done {
                break;
            }
        }
    }
}
pub unsafe extern "C" fn log_init(mut file: *const libc::c_char, mut appendp: bool) {
    if !file.is_null() {
        if *file as libc::c_int == '-' as i32
            && *file.offset(1 as libc::c_int as isize) == 0
        {
            stdlogfp = stdout;
            logfp = stdlogfp;
        } else {
            filelogfp = rpl_fopen(
                file,
                if appendp as libc::c_int != 0 {
                    b"a\0" as *const u8 as *const libc::c_char
                } else {
                    b"w\0" as *const u8 as *const libc::c_char
                },
            );
            if filelogfp.is_null() {
                fprintf(
                    stderr,
                    b"%s: %s: %s\n\0" as *const u8 as *const libc::c_char,
                    exec_name,
                    file,
                    strerror(*__errno_location()),
                );
                exit(WGET_EXIT_GENERIC_ERROR as libc::c_int);
            }
            logfp = filelogfp;
        }
    } else {
        stdlogfp = stderr;
        logfp = stdlogfp;
        if 1 as libc::c_int != 0 && isatty(fileno(logfp)) != 0 {
            save_context_p = 1 as libc::c_int != 0;
        }
    }
    shell_is_interactive = isatty(0 as libc::c_int);
}
pub unsafe extern "C" fn log_close() {
    let mut i: libc::c_int = 0;
    if !logfp.is_null() && logfp != stderr && logfp != stdout {
        if logfp == stdlogfp {
            stdlogfp = 0 as *mut FILE;
        }
        if logfp == filelogfp {
            filelogfp = 0 as *mut FILE;
        }
        fclose(logfp);
    }
    logfp = 0 as *mut FILE;
    inhibit_logging = 1 as libc::c_int != 0;
    save_context_p = 0 as libc::c_int != 0;
    i = 0 as libc::c_int;
    while i < 24 as libc::c_int {
        free_log_line(i);
        i += 1;
        i;
    }
    log_line_current = -(1 as libc::c_int);
    trailing_line = 0 as libc::c_int != 0;
}
unsafe extern "C" fn log_dump_context() {
    let mut num: libc::c_int = log_line_current;
    let mut fp: *mut FILE = get_log_fp();
    let mut warcfp: *mut FILE = get_warc_log_fp();
    if fp.is_null() {
        return;
    }
    if num == -(1 as libc::c_int) {
        return;
    }
    if trailing_line {
        num += 1;
        if num >= 24 as libc::c_int {
            num = 0 as libc::c_int;
        }
    }
    loop {
        let mut ln: *mut log_ln = log_lines.as_mut_ptr().offset(num as isize);
        if !((*ln).content).is_null() {
            fputs((*ln).content, fp);
            if !warcfp.is_null() {
                fputs((*ln).content, warcfp);
            }
        }
        num += 1;
        if num >= 24 as libc::c_int {
            num = 0 as libc::c_int;
        }
        if !(num != log_line_current) {
            break;
        }
    }
    if trailing_line {
        if !(log_lines[log_line_current as usize].content).is_null() {
            fputs(log_lines[log_line_current as usize].content, fp);
            if !warcfp.is_null() {
                fputs(log_lines[log_line_current as usize].content, warcfp);
            }
        }
    }
    rpl_fflush(fp);
    rpl_fflush(warcfp);
}
unsafe extern "C" fn count_nonprint(mut source: *const libc::c_char) -> libc::c_int {
    let mut p: *const libc::c_char = 0 as *const libc::c_char;
    let mut cnt: libc::c_int = 0;
    p = source;
    cnt = 0 as libc::c_int;
    while *p != 0 {
        if !c_isprint(*p as libc::c_int) {
            cnt += 1;
            cnt;
        }
        p = p.offset(1);
        p;
    }
    return cnt;
}
unsafe extern "C" fn copy_and_escape(
    mut source: *const libc::c_char,
    mut dest: *mut libc::c_char,
    mut escape: libc::c_char,
    mut base: libc::c_int,
) {
    let mut from: *const libc::c_char = source;
    let mut to: *mut libc::c_char = dest;
    let mut c: libc::c_uchar = 0;
    match base {
        8 => {
            loop {
                let fresh0 = from;
                from = from.offset(1);
                c = *fresh0 as libc::c_uchar;
                if !(c as libc::c_int != '\0' as i32) {
                    break;
                }
                if c_isprint(c as libc::c_int) {
                    let fresh1 = to;
                    to = to.offset(1);
                    *fresh1 = c as libc::c_char;
                } else {
                    let fresh2 = to;
                    to = to.offset(1);
                    *fresh2 = escape;
                    let fresh3 = to;
                    to = to.offset(1);
                    *fresh3 = ('0' as i32 + (c as libc::c_int >> 6 as libc::c_int))
                        as libc::c_char;
                    let fresh4 = to;
                    to = to.offset(1);
                    *fresh4 = ('0' as i32
                        + (c as libc::c_int >> 3 as libc::c_int & 7 as libc::c_int))
                        as libc::c_char;
                    let fresh5 = to;
                    to = to.offset(1);
                    *fresh5 = ('0' as i32 + (c as libc::c_int & 7 as libc::c_int))
                        as libc::c_char;
                }
            }
        }
        16 => {
            loop {
                let fresh6 = from;
                from = from.offset(1);
                c = *fresh6 as libc::c_uchar;
                if !(c as libc::c_int != '\0' as i32) {
                    break;
                }
                if c_isprint(c as libc::c_int) {
                    let fresh7 = to;
                    to = to.offset(1);
                    *fresh7 = c as libc::c_char;
                } else {
                    let fresh8 = to;
                    to = to.offset(1);
                    *fresh8 = escape;
                    let fresh9 = to;
                    to = to.offset(1);
                    *fresh9 = ((*::std::mem::transmute::<
                        &[u8; 17],
                        &[libc::c_char; 17],
                    >(
                        b"0123456789ABCDEF\0",
                    ))[(c as libc::c_int >> 4 as libc::c_int) as usize] as libc::c_int
                        + 0 as libc::c_int) as libc::c_char;
                    let fresh10 = to;
                    to = to.offset(1);
                    *fresh10 = ((*::std::mem::transmute::<
                        &[u8; 17],
                        &[libc::c_char; 17],
                    >(
                        b"0123456789ABCDEF\0",
                    ))[(c as libc::c_int & 0xf as libc::c_int) as usize] as libc::c_int
                        + 0 as libc::c_int) as libc::c_char;
                }
            }
        }
        _ => {
            abort();
        }
    }
    *to = '\0' as i32 as libc::c_char;
}
static mut ring: [ringel; 3] = [ringel {
    buffer: 0 as *const libc::c_char as *mut libc::c_char,
    size: 0,
}; 3];
unsafe extern "C" fn escnonprint_internal(
    mut str: *const libc::c_char,
    mut escape: libc::c_char,
    mut base: libc::c_int,
) -> *const libc::c_char {
    static mut ringpos: libc::c_int = 0;
    let mut nprcnt: libc::c_int = 0;
    nprcnt = count_nonprint(str);
    if nprcnt == 0 as libc::c_int {
        return str;
    }
    let mut r: *mut ringel = ring.as_mut_ptr().offset(ringpos as isize);
    let mut needed_size: libc::c_int = (strlen(str))
        .wrapping_add(1 as libc::c_int as libc::c_ulong)
        .wrapping_add(
            (if base == 8 as libc::c_int {
                3 as libc::c_int * nprcnt
            } else {
                2 as libc::c_int * nprcnt
            }) as libc::c_ulong,
        ) as libc::c_int;
    if ((*r).buffer).is_null() || (*r).size < needed_size {
        (*r)
            .buffer = xrealloc((*r).buffer as *mut libc::c_void, needed_size as size_t)
            as *mut libc::c_char;
        (*r).size = needed_size;
    }
    copy_and_escape(str, (*r).buffer, escape, base);
    ringpos = (ringpos + 1 as libc::c_int) % 3 as libc::c_int;
    return (*r).buffer;
}
pub unsafe extern "C" fn escnonprint(
    mut str: *const libc::c_char,
) -> *const libc::c_char {
    return escnonprint_internal(str, '\\' as i32 as libc::c_char, 8 as libc::c_int);
}
pub unsafe extern "C" fn escnonprint_uri(
    mut str: *const libc::c_char,
) -> *const libc::c_char {
    return escnonprint_internal(str, '%' as i32 as libc::c_char, 16 as libc::c_int);
}
static mut redirect_request_signal_name: *const libc::c_char = 0 as *const libc::c_char;
pub unsafe extern "C" fn redirect_output(
    mut to_file: bool,
    mut signal_name: *const libc::c_char,
) {
    if to_file as libc::c_int != 0 && logfp != filelogfp {
        if !signal_name.is_null() {
            fprintf(
                stderr,
                b"\n%s received.\0" as *const u8 as *const libc::c_char,
                signal_name,
            );
        }
        if filelogfp.is_null() {
            filelogfp = unique_create(
                b"wget-log\0" as *const u8 as *const libc::c_char,
                0 as libc::c_int != 0,
                &mut logfile,
            );
            if !filelogfp.is_null() {
                fprintf(
                    stderr,
                    dcgettext(
                        0 as *const libc::c_char,
                        b"\nRedirecting output to %s.\n\0" as *const u8
                            as *const libc::c_char,
                        5 as libc::c_int,
                    ),
                    quote(logfile),
                );
                redirect_request_signal_name = signal_name;
                logfp = filelogfp;
                log_dump_context();
            } else {
                fprintf(
                    stderr,
                    dcgettext(
                        0 as *const libc::c_char,
                        b"%s: %s; disabling logging.\n\0" as *const u8
                            as *const libc::c_char,
                        5 as libc::c_int,
                    ),
                    if !logfile.is_null() {
                        logfile as *const libc::c_char
                    } else {
                        b"wget-log\0" as *const u8 as *const libc::c_char
                    },
                    strerror(*__errno_location()),
                );
                inhibit_logging = 1 as libc::c_int != 0;
            }
        } else {
            fprintf(
                stderr,
                dcgettext(
                    0 as *const libc::c_char,
                    b"\nRedirecting output to %s.\n\0" as *const u8
                        as *const libc::c_char,
                    5 as libc::c_int,
                ),
                quote(logfile),
            );
            logfp = filelogfp;
            log_dump_context();
        }
    } else if !to_file && logfp != stdlogfp {
        logfp = stdlogfp;
        log_dump_context();
    }
}
unsafe extern "C" fn check_redirect_output() {
    if redirect_request_signal_name.is_null() && shell_is_interactive != 0
        && (opt.lfilename).is_null()
    {
        let mut foreground_pgrp: pid_t = tcgetpgrp(0 as libc::c_int);
        if foreground_pgrp != -(1 as libc::c_int) && foreground_pgrp != getpgrp()
            && !opt.quiet
        {
            redirect_output(1 as libc::c_int != 0, 0 as *const libc::c_char);
        } else {
            redirect_output(0 as libc::c_int != 0, 0 as *const libc::c_char);
        }
    }
}
