use ::libc;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
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
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    fn strerror(_: libc::c_int) -> *mut libc::c_char;
    fn strcasecmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    fn rpl_free(_: *mut libc::c_void);
    static mut opt: options;
    fn xmalloc(s: size_t) -> *mut libc::c_void;
    fn xstrdup(str: *const libc::c_char) -> *mut libc::c_char;
    static mut stderr: *mut FILE;
    fn fclose(__stream: *mut FILE) -> libc::c_int;
    fn fprintf(_: *mut FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn __getdelim(
        __lineptr: *mut *mut libc::c_char,
        __n: *mut size_t,
        __delimiter: libc::c_int,
        __stream: *mut FILE,
    ) -> __ssize_t;
    fn rpl_fopen(filename: *const libc::c_char, mode: *const libc::c_char) -> *mut FILE;
    fn quote(arg: *const libc::c_char) -> *const libc::c_char;
    static mut exec_name: *const libc::c_char;
    fn __errno_location() -> *mut libc::c_int;
    fn aprintf(_: *const libc::c_char, _: ...) -> *mut libc::c_char;
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
pub type __ssize_t = libc::c_long;
pub type __syscall_slong_t = libc::c_long;
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
pub type acc_t = _acc_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _acc_t {
    pub host: *mut libc::c_char,
    pub acc: *mut libc::c_char,
    pub passwd: *mut libc::c_char,
    pub next: *mut _acc_t,
}
pub type C2RustUnnamed_3 = libc::c_uint;
pub const tok_force: C2RustUnnamed_3 = 7;
pub const tok_port: C2RustUnnamed_3 = 6;
pub const tok_password: C2RustUnnamed_3 = 5;
pub const tok_machine: C2RustUnnamed_3 = 4;
pub const tok_macdef: C2RustUnnamed_3 = 3;
pub const tok_login: C2RustUnnamed_3 = 2;
pub const tok_account: C2RustUnnamed_3 = 1;
pub const tok_nothing: C2RustUnnamed_3 = 0;
#[inline]
unsafe extern "C" fn stat(
    mut __path: *const libc::c_char,
    mut __statbuf: *mut stat,
) -> libc::c_int {
    return __xstat(1 as libc::c_int, __path, __statbuf);
}
#[inline]
unsafe extern "C" fn c_isspace(mut c: libc::c_int) -> bool {
    match c {
        32 | 9 | 10 | 11 | 12 | 13 => return 1 as libc::c_int != 0,
        _ => return 0 as libc::c_int != 0,
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
static mut netrc_list: *mut acc_t = 0 as *const acc_t as *mut acc_t;
static mut processed_netrc: libc::c_int = 0;
pub unsafe extern "C" fn search_netrc(
    mut host: *const libc::c_char,
    mut acc: *mut *const libc::c_char,
    mut passwd: *mut *const libc::c_char,
    mut slack_default: libc::c_int,
    mut fp_netrc: *mut FILE,
) {
    let mut l: *mut acc_t = 0 as *mut acc_t;
    if !opt.netrc {
        return;
    }
    if processed_netrc == 0 {
        netrc_list = 0 as *mut acc_t;
        processed_netrc = 1 as libc::c_int;
        if !fp_netrc.is_null() {
            netrc_list = parse_netrc_fp(
                b".netrc\0" as *const u8 as *const libc::c_char,
                fp_netrc,
            );
        } else if !(opt.homedir).is_null() {
            let mut buf: stat = stat {
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
            let mut path: *mut libc::c_char = aprintf(
                b"%s/%s\0" as *const u8 as *const libc::c_char,
                opt.homedir,
                b".netrc\0" as *const u8 as *const libc::c_char,
            );
            if stat(path, &mut buf) == 0 as libc::c_int {
                netrc_list = parse_netrc(path);
            }
            rpl_free(path as *mut libc::c_void);
            path = 0 as *mut libc::c_char;
        }
    }
    if netrc_list.is_null() {
        return;
    }
    if !(*acc).is_null() && !(*passwd).is_null() {
        return;
    }
    l = netrc_list;
    while !l.is_null() {
        if !((*l).host).is_null() {
            if strcasecmp((*l).host, host) == 0 {
                break;
            }
        }
        l = (*l).next;
    }
    if !l.is_null() {
        if !(*acc).is_null() {
            if strcmp((*l).acc, *acc) == 0 {
                *passwd = (*l).passwd;
            } else {
                *passwd = 0 as *const libc::c_char;
            }
        } else {
            *acc = (*l).acc;
            if !((*l).passwd).is_null() {
                *passwd = (*l).passwd;
            }
        }
        return;
    } else {
        if slack_default == 0 {
            return;
        }
        if !(*acc).is_null() {
            return;
        }
        l = netrc_list;
        while !l.is_null() {
            if ((*l).host).is_null() {
                break;
            }
            l = (*l).next;
        }
        if l.is_null() {
            return;
        }
        *acc = (*l).acc;
        if (*passwd).is_null() {
            *passwd = (*l).passwd;
        }
        return;
    };
}
unsafe extern "C" fn maybe_add_to_list(
    mut newentry: *mut *mut acc_t,
    mut list: *mut *mut acc_t,
) {
    let mut a: *mut acc_t = 0 as *mut acc_t;
    let mut l: *mut acc_t = 0 as *mut acc_t;
    a = *newentry;
    l = *list;
    if !a.is_null() && ((*a).acc).is_null() {
        rpl_free((*a).host as *mut libc::c_void);
        (*a).host = 0 as *mut libc::c_char;
        rpl_free((*a).acc as *mut libc::c_void);
        (*a).acc = 0 as *mut libc::c_char;
        rpl_free((*a).passwd as *mut libc::c_void);
        (*a).passwd = 0 as *mut libc::c_char;
    } else {
        if !a.is_null() {
            (*a).next = l;
            l = a;
        }
        a = xmalloc(::std::mem::size_of::<acc_t>() as libc::c_ulong) as *mut acc_t;
    }
    memset(
        a as *mut libc::c_void,
        0 as libc::c_int,
        ::std::mem::size_of::<acc_t>() as libc::c_ulong,
    );
    *newentry = a;
    *list = l;
}
unsafe extern "C" fn shift_left(mut string: *mut libc::c_char) {
    let mut p: *mut libc::c_char = 0 as *mut libc::c_char;
    p = string;
    while *p != 0 {
        *p = *p.offset(1 as libc::c_int as isize);
        p = p.offset(1);
        p;
    }
}
unsafe extern "C" fn parse_netrc_fp(
    mut path: *const libc::c_char,
    mut fp: *mut FILE,
) -> *mut acc_t {
    let mut line: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut p: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut tok: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut premature_token: *const libc::c_char = 0 as *const libc::c_char;
    let mut current: *mut acc_t = 0 as *mut acc_t;
    let mut retval: *mut acc_t = 0 as *mut acc_t;
    let mut ln: libc::c_int = 0 as libc::c_int;
    let mut qmark: libc::c_int = 0;
    let mut bufsize: size_t = 0 as libc::c_int as size_t;
    let mut last_token: C2RustUnnamed_3 = tok_nothing;
    while getline(&mut line, &mut bufsize, fp) > 0 as libc::c_int as libc::c_long {
        ln += 1;
        ln;
        p = line;
        qmark = 0 as libc::c_int;
        while *p as libc::c_int != 0 && c_isspace(*p as libc::c_int) as libc::c_int != 0
        {
            p = p.offset(1);
            p;
        }
        if last_token as libc::c_uint == tok_macdef as libc::c_int as libc::c_uint
            && *p == 0
        {
            last_token = tok_nothing;
        }
        while *p as libc::c_int != 0
            && last_token as libc::c_uint != tok_macdef as libc::c_int as libc::c_uint
        {
            while *p as libc::c_int != 0
                && c_isspace(*p as libc::c_int) as libc::c_int != 0
            {
                p = p.offset(1);
                p;
            }
            if *p as libc::c_int == '#' as i32 || *p == 0 {
                break;
            }
            if *p as libc::c_int == '"' as i32 {
                qmark = 1 as libc::c_int;
                shift_left(p);
            }
            tok = p;
            while *p as libc::c_int != 0
                && (if qmark != 0 {
                    (*p as libc::c_int != '"' as i32) as libc::c_int
                } else {
                    !c_isspace(*p as libc::c_int) as libc::c_int
                }) != 0
            {
                if *p as libc::c_int == '\\' as i32 {
                    shift_left(p);
                }
                p = p.offset(1);
                p;
            }
            if qmark != 0 {
                shift_left(p);
                qmark = 0 as libc::c_int;
            }
            if *p != 0 {
                let fresh0 = p;
                p = p.offset(1);
                *fresh0 = '\0' as i32 as libc::c_char;
            }
            match last_token as libc::c_uint {
                2 => {
                    if !current.is_null() {
                        rpl_free((*current).acc as *mut libc::c_void);
                        (*current).acc = 0 as *mut libc::c_char;
                        (*current).acc = xstrdup(tok);
                    } else {
                        premature_token = b"login\0" as *const u8 as *const libc::c_char;
                    }
                }
                4 => {
                    maybe_add_to_list(&mut current, &mut retval);
                    (*current).host = xstrdup(tok);
                }
                5 => {
                    if !current.is_null() {
                        rpl_free((*current).passwd as *mut libc::c_void);
                        (*current).passwd = 0 as *mut libc::c_char;
                        (*current).passwd = xstrdup(tok);
                    } else {
                        premature_token = b"password\0" as *const u8
                            as *const libc::c_char;
                    }
                }
                3 => {
                    if current.is_null() {
                        premature_token = b"macdef\0" as *const u8
                            as *const libc::c_char;
                    }
                }
                1 => {
                    if current.is_null() {
                        premature_token = b"account\0" as *const u8
                            as *const libc::c_char;
                    }
                }
                6 => {
                    if current.is_null() {
                        premature_token = b"port\0" as *const u8 as *const libc::c_char;
                    }
                }
                7 => {
                    if current.is_null() {
                        premature_token = b"force\0" as *const u8 as *const libc::c_char;
                    }
                }
                0 | _ => {}
            }
            if !premature_token.is_null() {
                fprintf(
                    stderr,
                    dcgettext(
                        0 as *const libc::c_char,
                        b"%s: %s:%d: warning: %s token appears before any machine name\n\0"
                            as *const u8 as *const libc::c_char,
                        5 as libc::c_int,
                    ),
                    exec_name,
                    path,
                    ln,
                    quote(premature_token),
                );
                premature_token = 0 as *const libc::c_char;
            }
            if last_token as libc::c_uint != tok_nothing as libc::c_int as libc::c_uint {
                last_token = tok_nothing;
            } else if strcmp(tok, b"account\0" as *const u8 as *const libc::c_char) == 0
            {
                last_token = tok_account;
            } else if strcmp(tok, b"default\0" as *const u8 as *const libc::c_char) == 0
            {
                maybe_add_to_list(&mut current, &mut retval);
            } else if strcmp(tok, b"login\0" as *const u8 as *const libc::c_char) == 0 {
                last_token = tok_login;
            } else if strcmp(tok, b"macdef\0" as *const u8 as *const libc::c_char) == 0 {
                last_token = tok_macdef;
            } else if strcmp(tok, b"machine\0" as *const u8 as *const libc::c_char) == 0
            {
                last_token = tok_machine;
            } else if strcmp(tok, b"password\0" as *const u8 as *const libc::c_char) == 0
            {
                last_token = tok_password;
            } else if strcmp(tok, b"port\0" as *const u8 as *const libc::c_char) == 0 {
                last_token = tok_port;
            } else if strcmp(tok, b"force\0" as *const u8 as *const libc::c_char) == 0 {
                last_token = tok_force;
            } else {
                fprintf(
                    stderr,
                    dcgettext(
                        0 as *const libc::c_char,
                        b"%s: %s:%d: unknown token \"%s\"\n\0" as *const u8
                            as *const libc::c_char,
                        5 as libc::c_int,
                    ),
                    exec_name,
                    path,
                    ln,
                    tok,
                );
            }
        }
    }
    rpl_free(line as *mut libc::c_void);
    line = 0 as *mut libc::c_char;
    maybe_add_to_list(&mut current, &mut retval);
    rpl_free(current as *mut libc::c_void);
    current = 0 as *mut acc_t;
    current = retval;
    retval = 0 as *mut acc_t;
    while !current.is_null() {
        let mut saved_reference: *mut acc_t = 0 as *mut acc_t;
        saved_reference = (*current).next;
        (*current).next = retval;
        retval = current;
        current = saved_reference;
    }
    return retval;
}
unsafe extern "C" fn parse_netrc(mut path: *const libc::c_char) -> *mut acc_t {
    let mut fp: *mut FILE = 0 as *mut FILE;
    let mut acc: *mut acc_t = 0 as *mut acc_t;
    fp = rpl_fopen(path, b"r\0" as *const u8 as *const libc::c_char);
    if fp.is_null() {
        fprintf(
            stderr,
            dcgettext(
                0 as *const libc::c_char,
                b"%s: Cannot read %s (%s).\n\0" as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ),
            exec_name,
            path,
            strerror(*__errno_location()),
        );
        return 0 as *mut acc_t;
    }
    acc = parse_netrc_fp(path, fp);
    fclose(fp);
    return acc;
}
