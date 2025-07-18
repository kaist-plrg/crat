use ::libc;
use ::c2rust_bitfields;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    pub type hash_table;
    fn time(__timer: *mut time_t) -> time_t;
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
    fn memcmp(
        _: *const libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> libc::c_int;
    fn memchr(
        _: *const libc::c_void,
        _: libc::c_int,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    fn strncmp(
        _: *const libc::c_char,
        _: *const libc::c_char,
        _: libc::c_ulong,
    ) -> libc::c_int;
    fn strchr(_: *const libc::c_char, _: libc::c_int) -> *mut libc::c_char;
    fn strrchr(_: *const libc::c_char, _: libc::c_int) -> *mut libc::c_char;
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    fn strerror(_: libc::c_int) -> *mut libc::c_char;
    fn strcasecmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    fn rpl_free(_: *mut libc::c_void);
    static mut opt: options;
    fn qsort(
        __base: *mut libc::c_void,
        __nmemb: size_t,
        __size: size_t,
        __compar: __compar_fn_t,
    );
    fn xmalloc(s: size_t) -> *mut libc::c_void;
    fn xcalloc(n: size_t, s: size_t) -> *mut libc::c_void;
    fn xstrdup(str: *const libc::c_char) -> *mut libc::c_char;
    fn fclose(__stream: *mut FILE) -> libc::c_int;
    fn fprintf(_: *mut FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn sscanf(_: *const libc::c_char, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn fputc(__c: libc::c_int, __stream: *mut FILE) -> libc::c_int;
    fn __getdelim(
        __lineptr: *mut *mut libc::c_char,
        __n: *mut size_t,
        __delimiter: libc::c_int,
        __stream: *mut FILE,
    ) -> __ssize_t;
    fn fputs(__s: *const libc::c_char, __stream: *mut FILE) -> libc::c_int;
    fn rpl_fopen(filename: *const libc::c_char, mode: *const libc::c_char) -> *mut FILE;
    fn logprintf(_: log_options, _: *const libc::c_char, _: ...);
    fn debug_logprintf(_: *const libc::c_char, _: ...);
    fn quote(arg: *const libc::c_char) -> *const libc::c_char;
    fn quotearg_style(s: quoting_style, arg: *const libc::c_char) -> *mut libc::c_char;
    fn ferror(__stream: *mut FILE) -> libc::c_int;
    fn __errno_location() -> *mut libc::c_int;
    fn datetime_str(_: time_t) -> *mut libc::c_char;
    fn strdupdelim(_: *const libc::c_char, _: *const libc::c_char) -> *mut libc::c_char;
    fn match_tail(_: *const libc::c_char, _: *const libc::c_char, _: bool) -> bool;
    fn hash_table_destroy(_: *mut hash_table);
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
    fn hash_table_remove(_: *mut hash_table, _: *const libc::c_void) -> libc::c_int;
    fn hash_table_iterate(_: *mut hash_table, _: *mut hash_table_iterator);
    fn hash_table_iter_next(_: *mut hash_table_iterator) -> libc::c_int;
    fn hash_table_count(_: *const hash_table) -> libc::c_int;
    fn make_nocase_string_hash_table(_: libc::c_int) -> *mut hash_table;
    fn http_atotm(_: *const libc::c_char) -> time_t;
    fn extract_param(
        _: *mut *const libc::c_char,
        _: *mut param_token,
        _: *mut param_token,
        _: libc::c_char,
        _: *mut bool,
    ) -> bool;
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
pub type __compar_fn_t = Option::<
    unsafe extern "C" fn(*const libc::c_void, *const libc::c_void) -> libc::c_int,
>;
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
pub struct hash_table_iterator {
    pub key: *mut libc::c_void,
    pub value: *mut libc::c_void,
    pub pos: *mut libc::c_void,
    pub end: *mut libc::c_void,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct cookie_jar {
    pub chains: *mut hash_table,
    pub cookie_count: libc::c_int,
}
#[derive(Copy, Clone, BitfieldStruct)]
#[repr(C)]
pub struct cookie {
    pub domain: *mut libc::c_char,
    pub port: libc::c_int,
    pub path: *mut libc::c_char,
    #[bitfield(name = "discard_requested", ty = "libc::c_uint", bits = "0..=0")]
    #[bitfield(name = "secure", ty = "libc::c_uint", bits = "1..=1")]
    #[bitfield(name = "domain_exact", ty = "libc::c_uint", bits = "2..=2")]
    #[bitfield(name = "permanent", ty = "libc::c_uint", bits = "3..=3")]
    pub discard_requested_secure_domain_exact_permanent: [u8; 1],
    #[bitfield(padding)]
    pub c2rust_padding: [u8; 7],
    pub expiry_time: time_t,
    pub attr: *mut libc::c_char,
    pub value: *mut libc::c_char,
    pub next: *mut cookie,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct param_token {
    pub b: *const libc::c_char,
    pub e: *const libc::c_char,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct weighed_cookie {
    pub cookie: *mut cookie,
    pub domain_goodness: libc::c_int,
    pub path_goodness: libc::c_int,
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
unsafe extern "C" fn getline(
    mut __lineptr: *mut *mut libc::c_char,
    mut __n: *mut size_t,
    mut __stream: *mut FILE,
) -> __ssize_t {
    return __getdelim(__lineptr, __n, '\n' as i32, __stream);
}
static mut cookies_now: time_t = 0;
pub unsafe extern "C" fn cookie_jar_new() -> *mut cookie_jar {
    let mut jar: *mut cookie_jar = xmalloc(
        ::std::mem::size_of::<cookie_jar>() as libc::c_ulong,
    ) as *mut cookie_jar;
    (*jar).chains = make_nocase_string_hash_table(0 as libc::c_int);
    (*jar).cookie_count = 0 as libc::c_int;
    return jar;
}
unsafe extern "C" fn cookie_new() -> *mut cookie {
    let mut cookie: *mut cookie = xcalloc(
        1 as libc::c_int as size_t,
        ::std::mem::size_of::<cookie>() as libc::c_ulong,
    ) as *mut cookie;
    (*cookie).port = -(1 as libc::c_int);
    return cookie;
}
unsafe extern "C" fn cookie_expired_p(mut c: *const cookie) -> bool {
    return (*c).expiry_time != 0 as libc::c_int as libc::c_long
        && (*c).expiry_time < cookies_now;
}
unsafe extern "C" fn delete_cookie(mut cookie: *mut cookie) {
    rpl_free((*cookie).domain as *mut libc::c_void);
    (*cookie).domain = 0 as *mut libc::c_char;
    rpl_free((*cookie).path as *mut libc::c_void);
    (*cookie).path = 0 as *mut libc::c_char;
    rpl_free((*cookie).attr as *mut libc::c_void);
    (*cookie).attr = 0 as *mut libc::c_char;
    rpl_free((*cookie).value as *mut libc::c_void);
    (*cookie).value = 0 as *mut libc::c_char;
    rpl_free(cookie as *mut libc::c_void);
    cookie = 0 as *mut cookie;
}
unsafe extern "C" fn find_matching_cookie(
    mut jar: *mut cookie_jar,
    mut cookie: *mut cookie,
    mut prevptr: *mut *mut cookie,
) -> *mut cookie {
    let mut chain: *mut cookie = 0 as *mut cookie;
    let mut prev: *mut cookie = 0 as *mut cookie;
    chain = hash_table_get((*jar).chains, (*cookie).domain as *const libc::c_void)
        as *mut cookie;
    if !chain.is_null() {
        prev = 0 as *mut cookie;
        while !chain.is_null() {
            if 0 as libc::c_int == strcmp((*cookie).path, (*chain).path)
                && 0 as libc::c_int == strcmp((*cookie).attr, (*chain).attr)
                && (*cookie).port == (*chain).port
            {
                *prevptr = prev;
                return chain;
            }
            prev = chain;
            chain = (*chain).next;
        }
    }
    *prevptr = 0 as *mut cookie;
    return 0 as *mut cookie;
}
unsafe extern "C" fn store_cookie(mut jar: *mut cookie_jar, mut cookie: *mut cookie) {
    let mut chain_head: *mut cookie = 0 as *mut cookie;
    let mut chain_key: *mut libc::c_char = 0 as *mut libc::c_char;
    if hash_table_get_pair(
        (*jar).chains,
        (*cookie).domain as *const libc::c_void,
        &mut chain_key as *mut *mut libc::c_char as *mut libc::c_void,
        &mut chain_head as *mut *mut cookie as *mut libc::c_void,
    ) != 0
    {
        let mut prev: *mut cookie = 0 as *mut cookie;
        let mut victim: *mut cookie = find_matching_cookie(jar, cookie, &mut prev);
        if !victim.is_null() {
            if !prev.is_null() {
                (*prev).next = (*victim).next;
                (*cookie).next = chain_head;
            } else {
                (*cookie).next = (*victim).next;
            }
            delete_cookie(victim);
            (*jar).cookie_count -= 1;
            (*jar).cookie_count;
            if opt.debug as libc::c_long != 0 {
                debug_logprintf(
                    b"Deleted old cookie (to be replaced.)\n\0" as *const u8
                        as *const libc::c_char,
                );
            }
        } else {
            (*cookie).next = chain_head;
        }
    } else {
        (*cookie).next = 0 as *mut cookie;
        chain_key = xstrdup((*cookie).domain);
    }
    hash_table_put(
        (*jar).chains,
        chain_key as *const libc::c_void,
        cookie as *const libc::c_void,
    );
    (*jar).cookie_count += 1;
    (*jar).cookie_count;
    if opt.debug as libc::c_long != 0 {
        let mut exptime: time_t = (*cookie).expiry_time;
        if opt.debug as libc::c_long != 0 {
            debug_logprintf(
                b"\nStored cookie %s %d%s %s <%s> <%s> [expiry %s] %s %s\n\0"
                    as *const u8 as *const libc::c_char,
                (*cookie).domain,
                (*cookie).port,
                if (*cookie).port == -(1 as libc::c_int) {
                    b" (ANY)\0" as *const u8 as *const libc::c_char
                } else {
                    b"\0" as *const u8 as *const libc::c_char
                },
                (*cookie).path,
                if (*cookie).permanent() as libc::c_int != 0 {
                    b"permanent\0" as *const u8 as *const libc::c_char
                } else {
                    b"session\0" as *const u8 as *const libc::c_char
                },
                if (*cookie).secure() as libc::c_int != 0 {
                    b"secure\0" as *const u8 as *const libc::c_char
                } else {
                    b"insecure\0" as *const u8 as *const libc::c_char
                },
                if (*cookie).expiry_time != 0 {
                    datetime_str(exptime) as *const libc::c_char
                } else {
                    b"none\0" as *const u8 as *const libc::c_char
                },
                (*cookie).attr,
                (*cookie).value,
            );
        }
    }
}
unsafe extern "C" fn discard_matching_cookie(
    mut jar: *mut cookie_jar,
    mut cookie: *mut cookie,
) {
    let mut prev: *mut cookie = 0 as *mut cookie;
    let mut victim: *mut cookie = 0 as *mut cookie;
    if hash_table_count((*jar).chains) == 0 {
        return;
    }
    victim = find_matching_cookie(jar, cookie, &mut prev);
    if !victim.is_null() {
        if !prev.is_null() {
            (*prev).next = (*victim).next;
        } else {
            let mut chain_key: *mut libc::c_char = 0 as *mut libc::c_char;
            let mut res: libc::c_int = 0;
            res = hash_table_get_pair(
                (*jar).chains,
                (*victim).domain as *const libc::c_void,
                &mut chain_key as *mut *mut libc::c_char as *mut libc::c_void,
                0 as *mut libc::c_void,
            );
            if res == 0 as libc::c_int {
                logprintf(
                    LOG_VERBOSE,
                    dcgettext(
                        0 as *const libc::c_char,
                        b"Unable to get cookie for %s\n\0" as *const u8
                            as *const libc::c_char,
                        5 as libc::c_int,
                    ),
                    (*victim).domain,
                );
            }
            if ((*victim).next).is_null() {
                hash_table_remove(
                    (*jar).chains,
                    (*victim).domain as *const libc::c_void,
                );
                rpl_free(chain_key as *mut libc::c_void);
                chain_key = 0 as *mut libc::c_char;
            } else {
                hash_table_put(
                    (*jar).chains,
                    chain_key as *const libc::c_void,
                    (*victim).next as *const libc::c_void,
                );
            }
        }
        delete_cookie(victim);
        if opt.debug as libc::c_long != 0 {
            debug_logprintf(
                b"Discarded old cookie.\n\0" as *const u8 as *const libc::c_char,
            );
        }
    }
}
unsafe extern "C" fn parse_set_cookie(
    mut set_cookie: *const libc::c_char,
    mut silent: bool,
) -> *mut cookie {
    let mut current_block: u64;
    let mut ptr: *const libc::c_char = set_cookie;
    let mut cookie: *mut cookie = cookie_new();
    let mut name: param_token = param_token {
        b: 0 as *const libc::c_char,
        e: 0 as *const libc::c_char,
    };
    let mut value: param_token = param_token {
        b: 0 as *const libc::c_char,
        e: 0 as *const libc::c_char,
    };
    if extract_param(
        &mut ptr,
        &mut name,
        &mut value,
        ';' as i32 as libc::c_char,
        0 as *mut bool,
    ) {
        if !(value.b).is_null() {
            if *(value.b).offset(-(1 as libc::c_int as isize)) as libc::c_int
                == '"' as i32
            {
                value.b = (value.b).offset(-1);
                value.b;
            }
            if *value.e as libc::c_int == '"' as i32 {
                value.e = (value.e).offset(1);
                value.e;
            }
            (*cookie).attr = strdupdelim(name.b, name.e);
            (*cookie).value = strdupdelim(value.b, value.e);
            loop {
                if !extract_param(
                    &mut ptr,
                    &mut name,
                    &mut value,
                    ';' as i32 as libc::c_char,
                    0 as *mut bool,
                ) {
                    current_block = 16799951812150840583;
                    break;
                }
                if (name.e).offset_from(name.b) as libc::c_long as libc::c_ulong
                    == (::std::mem::size_of::<[libc::c_char; 7]>() as libc::c_ulong)
                        .wrapping_sub(1 as libc::c_int as libc::c_ulong)
                    && c_strncasecmp(
                        name.b,
                        b"domain\0" as *const u8 as *const libc::c_char,
                        (::std::mem::size_of::<[libc::c_char; 7]>() as libc::c_ulong)
                            .wrapping_sub(1 as libc::c_int as libc::c_ulong),
                    ) == 0
                {
                    if !(!(value.b).is_null() && value.b != value.e) {
                        current_block = 818193424693015907;
                        break;
                    }
                    rpl_free((*cookie).domain as *mut libc::c_void);
                    (*cookie).domain = 0 as *mut libc::c_char;
                    if *value.b as libc::c_int == '.' as i32 {
                        value.b = (value.b).offset(1);
                        value.b;
                    }
                    (*cookie).domain = strdupdelim(value.b, value.e);
                } else if (name.e).offset_from(name.b) as libc::c_long as libc::c_ulong
                    == (::std::mem::size_of::<[libc::c_char; 5]>() as libc::c_ulong)
                        .wrapping_sub(1 as libc::c_int as libc::c_ulong)
                    && c_strncasecmp(
                        name.b,
                        b"path\0" as *const u8 as *const libc::c_char,
                        (::std::mem::size_of::<[libc::c_char; 5]>() as libc::c_ulong)
                            .wrapping_sub(1 as libc::c_int as libc::c_ulong),
                    ) == 0
                {
                    if !(!(value.b).is_null() && value.b != value.e) {
                        current_block = 818193424693015907;
                        break;
                    }
                    rpl_free((*cookie).path as *mut libc::c_void);
                    (*cookie).path = 0 as *mut libc::c_char;
                    (*cookie).path = strdupdelim(value.b, value.e);
                } else if (name.e).offset_from(name.b) as libc::c_long as libc::c_ulong
                    == (::std::mem::size_of::<[libc::c_char; 8]>() as libc::c_ulong)
                        .wrapping_sub(1 as libc::c_int as libc::c_ulong)
                    && c_strncasecmp(
                        name.b,
                        b"expires\0" as *const u8 as *const libc::c_char,
                        (::std::mem::size_of::<[libc::c_char; 8]>() as libc::c_ulong)
                            .wrapping_sub(1 as libc::c_int as libc::c_ulong),
                    ) == 0
                {
                    let mut value_copy: [libc::c_char; 128] = [0; 128];
                    let mut value_len: size_t = (value.e).offset_from(value.b)
                        as libc::c_long as size_t;
                    let mut expires: time_t = 0;
                    if !(!(value.b).is_null() && value.b != value.e)
                        || value_len
                            >= ::std::mem::size_of::<[libc::c_char; 128]>()
                                as libc::c_ulong
                    {
                        current_block = 818193424693015907;
                        break;
                    }
                    memcpy(
                        value_copy.as_mut_ptr() as *mut libc::c_void,
                        value.b as *const libc::c_void,
                        value_len,
                    );
                    value_copy[value_len as usize] = 0 as libc::c_int as libc::c_char;
                    expires = http_atotm(value_copy.as_mut_ptr());
                    if expires != -(1 as libc::c_int) as time_t {
                        (*cookie).set_permanent(1 as libc::c_int as libc::c_uint);
                        (*cookie).expiry_time = expires;
                        if (*cookie).expiry_time < cookies_now {
                            (*cookie)
                                .set_discard_requested(1 as libc::c_int as libc::c_uint);
                        }
                    }
                } else if (name.e).offset_from(name.b) as libc::c_long as libc::c_ulong
                    == (::std::mem::size_of::<[libc::c_char; 8]>() as libc::c_ulong)
                        .wrapping_sub(1 as libc::c_int as libc::c_ulong)
                    && c_strncasecmp(
                        name.b,
                        b"max-age\0" as *const u8 as *const libc::c_char,
                        (::std::mem::size_of::<[libc::c_char; 8]>() as libc::c_ulong)
                            .wrapping_sub(1 as libc::c_int as libc::c_ulong),
                    ) == 0
                {
                    let mut maxage: libc::c_double = -(1 as libc::c_int)
                        as libc::c_double;
                    let mut value_copy_0: [libc::c_char; 32] = [0; 32];
                    let mut value_len_0: size_t = (value.e).offset_from(value.b)
                        as libc::c_long as size_t;
                    if !(!(value.b).is_null() && value.b != value.e)
                        || value_len_0
                            >= ::std::mem::size_of::<[libc::c_char; 32]>()
                                as libc::c_ulong
                    {
                        current_block = 818193424693015907;
                        break;
                    }
                    memcpy(
                        value_copy_0.as_mut_ptr() as *mut libc::c_void,
                        value.b as *const libc::c_void,
                        value_len_0,
                    );
                    value_copy_0[value_len_0
                        as usize] = 0 as libc::c_int as libc::c_char;
                    sscanf(
                        value_copy_0.as_mut_ptr(),
                        b"%lf\0" as *const u8 as *const libc::c_char,
                        &mut maxage as *mut libc::c_double,
                    );
                    if maxage == -(1 as libc::c_int) as libc::c_double {
                        current_block = 818193424693015907;
                        break;
                    } else {
                        (*cookie).set_permanent(1 as libc::c_int as libc::c_uint);
                        (*cookie).expiry_time = cookies_now + maxage as time_t;
                        if maxage == 0 as libc::c_int as libc::c_double {
                            (*cookie)
                                .set_discard_requested(1 as libc::c_int as libc::c_uint);
                        }
                    }
                } else if (name.e).offset_from(name.b) as libc::c_long as libc::c_ulong
                    == (::std::mem::size_of::<[libc::c_char; 7]>() as libc::c_ulong)
                        .wrapping_sub(1 as libc::c_int as libc::c_ulong)
                    && c_strncasecmp(
                        name.b,
                        b"secure\0" as *const u8 as *const libc::c_char,
                        (::std::mem::size_of::<[libc::c_char; 7]>() as libc::c_ulong)
                            .wrapping_sub(1 as libc::c_int as libc::c_ulong),
                    ) == 0
                {
                    (*cookie).set_secure(1 as libc::c_int as libc::c_uint);
                }
            }
            match current_block {
                818193424693015907 => {}
                _ => {
                    if !(*ptr != 0) {
                        return cookie;
                    }
                }
            }
        }
    }
    if !silent {
        logprintf(
            LOG_NOTQUIET,
            dcgettext(
                0 as *const libc::c_char,
                b"Syntax error in Set-Cookie: %s at position %d.\n\0" as *const u8
                    as *const libc::c_char,
                5 as libc::c_int,
            ),
            quotearg_style(escape_quoting_style, set_cookie),
            ptr.offset_from(set_cookie) as libc::c_long as libc::c_int,
        );
    }
    delete_cookie(cookie);
    return 0 as *mut cookie;
}
unsafe extern "C" fn numeric_address_p(mut addr: *const libc::c_char) -> bool {
    let mut p: *const libc::c_char = addr;
    if !c_isdigit(*p as libc::c_int) {
        return 0 as libc::c_int != 0;
    }
    p = p.offset(1);
    p;
    while c_isdigit(*p as libc::c_int) {
        p = p.offset(1);
        p;
    }
    let fresh0 = p;
    p = p.offset(1);
    if *fresh0 as libc::c_int != '.' as i32 {
        return 0 as libc::c_int != 0;
    }
    if !c_isdigit(*p as libc::c_int) {
        return 0 as libc::c_int != 0;
    }
    p = p.offset(1);
    p;
    while c_isdigit(*p as libc::c_int) {
        p = p.offset(1);
        p;
    }
    let fresh1 = p;
    p = p.offset(1);
    if *fresh1 as libc::c_int != '.' as i32 {
        return 0 as libc::c_int != 0;
    }
    if !c_isdigit(*p as libc::c_int) {
        return 0 as libc::c_int != 0;
    }
    p = p.offset(1);
    p;
    while c_isdigit(*p as libc::c_int) {
        p = p.offset(1);
        p;
    }
    let fresh2 = p;
    p = p.offset(1);
    if *fresh2 as libc::c_int != '.' as i32 {
        return 0 as libc::c_int != 0;
    }
    if !c_isdigit(*p as libc::c_int) {
        return 0 as libc::c_int != 0;
    }
    p = p.offset(1);
    p;
    while c_isdigit(*p as libc::c_int) {
        p = p.offset(1);
        p;
    }
    if *p as libc::c_int != '\0' as i32 {
        return 0 as libc::c_int != 0;
    }
    return 1 as libc::c_int != 0;
}
unsafe extern "C" fn check_domain_match(
    mut cookie_domain: *const libc::c_char,
    mut host: *const libc::c_char,
) -> bool {
    if opt.debug as libc::c_long != 0 {
        debug_logprintf(b"cdm: 2\n\0" as *const u8 as *const libc::c_char);
    }
    if 0 as libc::c_int == strcasecmp(cookie_domain, host) {
        return 1 as libc::c_int != 0;
    }
    if opt.debug as libc::c_long != 0 {
        debug_logprintf(b"cdm: 3\n\0" as *const u8 as *const libc::c_char);
    }
    if !match_tail(host, cookie_domain, 1 as libc::c_int != 0) {
        return 0 as libc::c_int != 0;
    }
    let mut p: *const libc::c_char = cookie_domain;
    let mut dccount: libc::c_int = 1 as libc::c_int;
    let mut ldcl: libc::c_int = 0 as libc::c_int;
    let mut nldcl: libc::c_int = 0 as libc::c_int;
    let mut out: libc::c_int = 0;
    if *p as libc::c_int == '.' as i32 {
        p = p.offset(1);
        p;
    }
    if opt.debug as libc::c_long != 0 {
        debug_logprintf(b"cdm: 4\n\0" as *const u8 as *const libc::c_char);
    }
    out = 0 as libc::c_int;
    while out == 0 {
        match *p as libc::c_int {
            0 => {
                out = 1 as libc::c_int;
            }
            46 => {
                if ldcl == 0 as libc::c_int {
                    return 0 as libc::c_int != 0;
                }
                if *p.offset(1 as libc::c_int as isize) as libc::c_int == '\0' as i32 {
                    out = 1 as libc::c_int;
                } else {
                    nldcl = ldcl;
                    ldcl = 0 as libc::c_int;
                    dccount += 1;
                    dccount;
                }
            }
            _ => {
                ldcl += 1;
                ldcl;
            }
        }
        p = p.offset(1);
        p;
    }
    if opt.debug as libc::c_long != 0 {
        debug_logprintf(b"cdm: 5\n\0" as *const u8 as *const libc::c_char);
    }
    if dccount < 2 as libc::c_int {
        return 0 as libc::c_int != 0;
    }
    if opt.debug as libc::c_long != 0 {
        debug_logprintf(b"cdm: 6\n\0" as *const u8 as *const libc::c_char);
    }
    if dccount == 2 as libc::c_int {
        let mut i: size_t = 0;
        let mut known_toplevel: libc::c_int = 0 as libc::c_int;
        static mut known_toplevel_domains: [*const libc::c_char; 7] = [
            b".com\0" as *const u8 as *const libc::c_char,
            b".edu\0" as *const u8 as *const libc::c_char,
            b".net\0" as *const u8 as *const libc::c_char,
            b".org\0" as *const u8 as *const libc::c_char,
            b".gov\0" as *const u8 as *const libc::c_char,
            b".mil\0" as *const u8 as *const libc::c_char,
            b".int\0" as *const u8 as *const libc::c_char,
        ];
        i = 0 as libc::c_int as size_t;
        while i
            < (::std::mem::size_of::<[*const libc::c_char; 7]>() as libc::c_ulong)
                .wrapping_div(
                    ::std::mem::size_of::<*const libc::c_char>() as libc::c_ulong,
                )
        {
            if match_tail(
                cookie_domain,
                known_toplevel_domains[i as usize],
                1 as libc::c_int != 0,
            ) {
                known_toplevel = 1 as libc::c_int;
                break;
            } else {
                i = i.wrapping_add(1);
                i;
            }
        }
        if known_toplevel == 0 && nldcl <= 3 as libc::c_int {
            return 0 as libc::c_int != 0;
        }
    }
    if opt.debug as libc::c_long != 0 {
        debug_logprintf(b"cdm: 7\n\0" as *const u8 as *const libc::c_char);
    }
    if *cookie_domain as libc::c_int != '.' as i32 {
        let mut dlen: libc::c_int = strlen(cookie_domain) as libc::c_int;
        let mut hlen: libc::c_int = strlen(host) as libc::c_int;
        if hlen > dlen
            && *host.offset((hlen - dlen - 1 as libc::c_int) as isize) as libc::c_int
                != '.' as i32
        {
            return 0 as libc::c_int != 0;
        }
    }
    if opt.debug as libc::c_long != 0 {
        debug_logprintf(b"cdm: 8\n\0" as *const u8 as *const libc::c_char);
    }
    return 1 as libc::c_int != 0;
}
unsafe extern "C" fn check_path_match(
    mut cookie_path: *const libc::c_char,
    mut path: *const libc::c_char,
) -> bool {
    return path_matches(path, cookie_path) != 0 as libc::c_int;
}
pub unsafe extern "C" fn cookie_handle_set_cookie(
    mut jar: *mut cookie_jar,
    mut host: *const libc::c_char,
    mut port: libc::c_int,
    mut path: *const libc::c_char,
    mut set_cookie: *const libc::c_char,
) {
    let mut current_block: u64;
    let mut cookie: *mut cookie = 0 as *mut cookie;
    cookies_now = time(0 as *mut time_t);
    let mut buf: [libc::c_char; 1024] = [0; 1024];
    let mut tmp: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut pathlen: size_t = strlen(path);
    if pathlen
        < (::std::mem::size_of::<[libc::c_char; 1024]>() as libc::c_ulong)
            .wrapping_sub(1 as libc::c_int as libc::c_ulong)
    {
        tmp = buf.as_mut_ptr();
    } else {
        tmp = xmalloc(pathlen.wrapping_add(2 as libc::c_int as libc::c_ulong))
            as *mut libc::c_char;
    }
    *tmp = '/' as i32 as libc::c_char;
    memcpy(
        tmp.offset(1 as libc::c_int as isize) as *mut libc::c_void,
        path as *const libc::c_void,
        pathlen.wrapping_add(1 as libc::c_int as libc::c_ulong),
    );
    path = tmp;
    cookie = parse_set_cookie(set_cookie, 0 as libc::c_int != 0);
    if !cookie.is_null() {
        if ((*cookie).domain).is_null() {
            (*cookie).domain = xstrdup(host);
            (*cookie).set_domain_exact(1 as libc::c_int as libc::c_uint);
            if port != 80 as libc::c_int && port != 443 as libc::c_int {
                (*cookie).port = port;
            }
        } else if !check_domain_match((*cookie).domain, host) {
            logprintf(
                LOG_NOTQUIET,
                dcgettext(
                    0 as *const libc::c_char,
                    b"Cookie coming from %s attempted to set domain to \0" as *const u8
                        as *const libc::c_char,
                    5 as libc::c_int,
                ),
                quotearg_style(escape_quoting_style, host),
            );
            logprintf(
                LOG_NOTQUIET,
                dcgettext(
                    0 as *const libc::c_char,
                    b"%s\n\0" as *const u8 as *const libc::c_char,
                    5 as libc::c_int,
                ),
                quotearg_style(escape_quoting_style, (*cookie).domain),
            );
            (*cookie).set_discard_requested(1 as libc::c_int as libc::c_uint);
        }
        if ((*cookie).path).is_null() {
            let mut trailing_slash: *mut libc::c_char = strrchr(path, '/' as i32);
            if !trailing_slash.is_null() {
                (*cookie)
                    .path = strdupdelim(
                    path,
                    trailing_slash.offset(1 as libc::c_int as isize),
                );
            } else {
                (*cookie).path = xstrdup(path);
            }
            current_block = 15925075030174552612;
        } else if !check_path_match((*cookie).path, path) {
            if opt.debug as libc::c_long != 0 {
                debug_logprintf(
                    b"Attempt to fake the path: %s, %s\n\0" as *const u8
                        as *const libc::c_char,
                    (*cookie).path,
                    path,
                );
            }
            current_block = 1957230209305718043;
        } else {
            current_block = 15925075030174552612;
        }
        match current_block {
            1957230209305718043 => {}
            _ => {
                if (*cookie).discard_requested() != 0 {
                    discard_matching_cookie(jar, cookie);
                } else {
                    store_cookie(jar, cookie);
                    if tmp != buf.as_mut_ptr() {
                        rpl_free(tmp as *mut libc::c_void);
                        tmp = 0 as *mut libc::c_char;
                    }
                    return;
                }
            }
        }
    }
    if !cookie.is_null() {
        delete_cookie(cookie);
    }
    if tmp != buf.as_mut_ptr() {
        rpl_free(tmp as *mut libc::c_void);
        tmp = 0 as *mut libc::c_char;
    }
}
unsafe extern "C" fn count_char(
    mut string: *const libc::c_char,
    mut chr: libc::c_char,
) -> libc::c_int {
    let mut p: *const libc::c_char = 0 as *const libc::c_char;
    let mut count: libc::c_int = 0 as libc::c_int;
    p = string;
    while *p != 0 {
        if *p as libc::c_int == chr as libc::c_int {
            count += 1;
            count;
        }
        p = p.offset(1);
        p;
    }
    return count;
}
unsafe extern "C" fn find_chains_of_host(
    mut jar: *mut cookie_jar,
    mut host: *const libc::c_char,
    mut dest: *mut *mut cookie,
) -> libc::c_int {
    let mut dest_count: libc::c_int = 0 as libc::c_int;
    let mut passes: libc::c_int = 0;
    let mut passcnt: libc::c_int = 0;
    if hash_table_count((*jar).chains) == 0 {
        return 0 as libc::c_int;
    }
    if numeric_address_p(host) {
        passes = 1 as libc::c_int;
    } else {
        passes = count_char(host, '.' as i32 as libc::c_char);
    }
    passcnt = 0 as libc::c_int;
    loop {
        let mut chain: *mut cookie = hash_table_get(
            (*jar).chains,
            host as *const libc::c_void,
        ) as *mut cookie;
        if !chain.is_null() {
            let fresh3 = dest_count;
            dest_count = dest_count + 1;
            let ref mut fresh4 = *dest.offset(fresh3 as isize);
            *fresh4 = chain;
        }
        passcnt += 1;
        if passcnt >= passes {
            break;
        }
        host = (strchr(host, '.' as i32)).offset(1 as libc::c_int as isize);
    }
    return dest_count;
}
unsafe extern "C" fn path_matches(
    mut full_path: *const libc::c_char,
    mut prefix: *const libc::c_char,
) -> libc::c_int {
    let mut len: libc::c_int = strlen(prefix) as libc::c_int;
    if 0 as libc::c_int != strncmp(full_path, prefix, len as libc::c_ulong) {
        return 0 as libc::c_int;
    }
    return len + 1 as libc::c_int;
}
unsafe extern "C" fn cookie_matches_url(
    mut cookie: *const cookie,
    mut host: *const libc::c_char,
    mut port: libc::c_int,
    mut path: *const libc::c_char,
    mut secflag: bool,
    mut path_goodness: *mut libc::c_int,
) -> bool {
    let mut pg: libc::c_int = 0;
    if cookie_expired_p(cookie) {
        return 0 as libc::c_int != 0;
    }
    if (*cookie).secure() as libc::c_int != 0 && !secflag {
        return 0 as libc::c_int != 0;
    }
    if (*cookie).port != -(1 as libc::c_int) && (*cookie).port != port {
        return 0 as libc::c_int != 0;
    }
    if (*cookie).domain_exact() as libc::c_int != 0
        && 0 as libc::c_int != strcasecmp(host, (*cookie).domain)
    {
        return 0 as libc::c_int != 0;
    }
    pg = path_matches(path, (*cookie).path);
    if pg == 0 as libc::c_int {
        return 0 as libc::c_int != 0;
    }
    if !path_goodness.is_null() {
        *path_goodness = pg;
    }
    return 1 as libc::c_int != 0;
}
unsafe extern "C" fn equality_comparator(
    mut p1: *const libc::c_void,
    mut p2: *const libc::c_void,
) -> libc::c_int {
    let mut wc1: *mut weighed_cookie = p1 as *mut weighed_cookie;
    let mut wc2: *mut weighed_cookie = p2 as *mut weighed_cookie;
    let mut namecmp: libc::c_int = strcmp((*(*wc1).cookie).attr, (*(*wc2).cookie).attr);
    let mut valuecmp: libc::c_int = strcmp(
        (*(*wc1).cookie).value,
        (*(*wc2).cookie).value,
    );
    return if namecmp != 0 { namecmp } else { valuecmp };
}
unsafe extern "C" fn eliminate_dups(
    mut outgoing: *mut weighed_cookie,
    mut count: libc::c_int,
) -> libc::c_int {
    let mut h: *mut weighed_cookie = 0 as *mut weighed_cookie;
    let mut t: *mut weighed_cookie = 0 as *mut weighed_cookie;
    let mut end: *mut weighed_cookie = outgoing.offset(count as isize);
    qsort(
        outgoing as *mut libc::c_void,
        count as size_t,
        ::std::mem::size_of::<weighed_cookie>() as libc::c_ulong,
        Some(
            equality_comparator
                as unsafe extern "C" fn(
                    *const libc::c_void,
                    *const libc::c_void,
                ) -> libc::c_int,
        ),
    );
    let mut current_block_4: u64;
    t = outgoing;
    h = t;
    while h < end {
        if h != end.offset(-(1 as libc::c_int as isize)) {
            let mut c0: *mut cookie = (*h.offset(0 as libc::c_int as isize)).cookie;
            let mut c1: *mut cookie = (*h.offset(1 as libc::c_int as isize)).cookie;
            if strcmp((*c0).attr, (*c1).attr) == 0
                && strcmp((*c0).value, (*c1).value) == 0
            {
                current_block_4 = 4988723283678924448;
            } else {
                current_block_4 = 2473556513754201174;
            }
        } else {
            current_block_4 = 2473556513754201174;
        }
        match current_block_4 {
            2473556513754201174 => {
                if h != t {
                    let fresh5 = t;
                    t = t.offset(1);
                    *fresh5 = *h;
                } else {
                    t = t.offset(1);
                    t;
                }
            }
            _ => {}
        }
        h = h.offset(1);
        h;
    }
    return t.offset_from(outgoing) as libc::c_long as libc::c_int;
}
unsafe extern "C" fn goodness_comparator(
    mut p1: *const libc::c_void,
    mut p2: *const libc::c_void,
) -> libc::c_int {
    let mut wc1: *mut weighed_cookie = p1 as *mut weighed_cookie;
    let mut wc2: *mut weighed_cookie = p2 as *mut weighed_cookie;
    let mut dgdiff: libc::c_int = (*wc2).domain_goodness - (*wc1).domain_goodness;
    let mut pgdiff: libc::c_int = (*wc2).path_goodness - (*wc1).path_goodness;
    return if dgdiff != 0 { dgdiff } else { pgdiff };
}
pub unsafe extern "C" fn cookie_header(
    mut jar: *mut cookie_jar,
    mut host: *const libc::c_char,
    mut port: libc::c_int,
    mut path: *const libc::c_char,
    mut secflag: bool,
) -> *mut libc::c_char {
    let mut chains: [*mut cookie; 32] = [0 as *mut cookie; 32];
    let mut chain_count: libc::c_int = 0;
    let mut cookie: *mut cookie = 0 as *mut cookie;
    let mut outgoing: *mut weighed_cookie = 0 as *mut weighed_cookie;
    let mut count: size_t = 0;
    let mut i: size_t = 0;
    let mut ocnt: size_t = 0;
    let mut result: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut result_size: libc::c_int = 0;
    let mut pos: libc::c_int = 0;
    let mut pathbuf: [libc::c_char; 1024] = [0; 1024];
    chain_count = 1 as libc::c_int + count_char(host, '.' as i32 as libc::c_char);
    if chain_count
        > (::std::mem::size_of::<[*mut cookie; 32]>() as libc::c_ulong)
            .wrapping_div(::std::mem::size_of::<*mut cookie>() as libc::c_ulong)
            as libc::c_int
    {
        return 0 as *mut libc::c_char;
    }
    chain_count = find_chains_of_host(jar, host, chains.as_mut_ptr());
    if chain_count <= 0 as libc::c_int {
        return 0 as *mut libc::c_char;
    }
    let mut tmp: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut pathlen: size_t = strlen(path);
    if pathlen
        < (::std::mem::size_of::<[libc::c_char; 1024]>() as libc::c_ulong)
            .wrapping_sub(1 as libc::c_int as libc::c_ulong)
    {
        tmp = pathbuf.as_mut_ptr();
    } else {
        tmp = xmalloc(pathlen.wrapping_add(2 as libc::c_int as libc::c_ulong))
            as *mut libc::c_char;
    }
    *tmp = '/' as i32 as libc::c_char;
    memcpy(
        tmp.offset(1 as libc::c_int as isize) as *mut libc::c_void,
        path as *const libc::c_void,
        pathlen.wrapping_add(1 as libc::c_int as libc::c_ulong),
    );
    path = tmp;
    cookies_now = time(0 as *mut time_t);
    count = 0 as libc::c_int as size_t;
    i = 0 as libc::c_int as size_t;
    while i < chain_count as libc::c_uint as libc::c_ulong {
        cookie = chains[i as usize];
        while !cookie.is_null() {
            if cookie_matches_url(
                cookie,
                host,
                port,
                path,
                secflag,
                0 as *mut libc::c_int,
            ) {
                count = count.wrapping_add(1);
                count;
            }
            cookie = (*cookie).next;
        }
        i = i.wrapping_add(1);
        i;
    }
    if !(count == 0) {
        if !(count
            > (18446744073709551615 as libc::c_ulong)
                .wrapping_div(::std::mem::size_of::<weighed_cookie>() as libc::c_ulong))
        {
            outgoing = xmalloc(
                count
                    .wrapping_mul(
                        ::std::mem::size_of::<weighed_cookie>() as libc::c_ulong,
                    ),
            ) as *mut weighed_cookie;
            ocnt = 0 as libc::c_int as size_t;
            i = 0 as libc::c_int as size_t;
            while i < chain_count as libc::c_uint as libc::c_ulong {
                cookie = chains[i as usize];
                while !cookie.is_null() {
                    let mut pg: libc::c_int = 0;
                    if cookie_matches_url(cookie, host, port, path, secflag, &mut pg) {
                        let ref mut fresh6 = (*outgoing.offset(ocnt as isize)).cookie;
                        *fresh6 = cookie;
                        (*outgoing.offset(ocnt as isize))
                            .domain_goodness = strlen((*cookie).domain) as libc::c_int;
                        (*outgoing.offset(ocnt as isize)).path_goodness = pg;
                        ocnt = ocnt.wrapping_add(1);
                        ocnt;
                    }
                    cookie = (*cookie).next;
                }
                i = i.wrapping_add(1);
                i;
            }
            count = eliminate_dups(outgoing, count as libc::c_int) as size_t;
            qsort(
                outgoing as *mut libc::c_void,
                count,
                ::std::mem::size_of::<weighed_cookie>() as libc::c_ulong,
                Some(
                    goodness_comparator
                        as unsafe extern "C" fn(
                            *const libc::c_void,
                            *const libc::c_void,
                        ) -> libc::c_int,
                ),
            );
            result_size = 0 as libc::c_int;
            i = 0 as libc::c_int as size_t;
            while i < count {
                let mut c: *mut cookie = (*outgoing.offset(i as isize)).cookie;
                result_size = (result_size as libc::c_ulong)
                    .wrapping_add(
                        (strlen((*c).attr))
                            .wrapping_add(1 as libc::c_int as libc::c_ulong)
                            .wrapping_add(strlen((*c).value)),
                    ) as libc::c_int as libc::c_int;
                i = i.wrapping_add(1);
                i;
            }
            result_size = (result_size as libc::c_ulong)
                .wrapping_add(
                    count
                        .wrapping_sub(1 as libc::c_int as libc::c_ulong)
                        .wrapping_mul(2 as libc::c_int as libc::c_ulong),
                )
                .wrapping_add(1 as libc::c_int as libc::c_ulong) as libc::c_int;
            result = xmalloc(result_size as size_t) as *mut libc::c_char;
            pos = 0 as libc::c_int;
            i = 0 as libc::c_int as size_t;
            while i < count {
                let mut c_0: *mut cookie = (*outgoing.offset(i as isize)).cookie;
                let mut namlen: libc::c_int = strlen((*c_0).attr) as libc::c_int;
                let mut vallen: libc::c_int = strlen((*c_0).value) as libc::c_int;
                memcpy(
                    result.offset(pos as isize) as *mut libc::c_void,
                    (*c_0).attr as *const libc::c_void,
                    namlen as libc::c_ulong,
                );
                pos += namlen;
                let fresh7 = pos;
                pos = pos + 1;
                *result.offset(fresh7 as isize) = '=' as i32 as libc::c_char;
                memcpy(
                    result.offset(pos as isize) as *mut libc::c_void,
                    (*c_0).value as *const libc::c_void,
                    vallen as libc::c_ulong,
                );
                pos += vallen;
                if i < count.wrapping_sub(1 as libc::c_int as libc::c_ulong) {
                    let fresh8 = pos;
                    pos = pos + 1;
                    *result.offset(fresh8 as isize) = ';' as i32 as libc::c_char;
                    let fresh9 = pos;
                    pos = pos + 1;
                    *result.offset(fresh9 as isize) = ' ' as i32 as libc::c_char;
                }
                i = i.wrapping_add(1);
                i;
            }
            let fresh10 = pos;
            pos = pos + 1;
            *result.offset(fresh10 as isize) = '\0' as i32 as libc::c_char;
            rpl_free(outgoing as *mut libc::c_void);
            outgoing = 0 as *mut weighed_cookie;
        }
    }
    if path != pathbuf.as_mut_ptr() as *const libc::c_char {
        rpl_free(path as *mut libc::c_void);
        path = 0 as *const libc::c_char;
    }
    return result;
}
unsafe extern "C" fn domain_port(
    mut domain_b: *const libc::c_char,
    mut domain_e: *const libc::c_char,
    mut domain_e_ptr: *mut *const libc::c_char,
) -> libc::c_int {
    let mut port: libc::c_int = 0 as libc::c_int;
    let mut p: *const libc::c_char = 0 as *const libc::c_char;
    let mut colon: *const libc::c_char = memchr(
        domain_b as *const libc::c_void,
        ':' as i32,
        domain_e.offset_from(domain_b) as libc::c_long as libc::c_ulong,
    ) as *const libc::c_char;
    if colon.is_null() {
        return 0 as libc::c_int;
    }
    p = colon.offset(1 as libc::c_int as isize);
    while p < domain_e && c_isdigit(*p as libc::c_int) as libc::c_int != 0 {
        port = 10 as libc::c_int * port + (*p as libc::c_int - '0' as i32);
        p = p.offset(1);
        p;
    }
    if p < domain_e {
        return 0 as libc::c_int;
    }
    *domain_e_ptr = colon;
    return port;
}
pub unsafe extern "C" fn cookie_jar_load(
    mut jar: *mut cookie_jar,
    mut file: *const libc::c_char,
) {
    let mut line: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut bufsize: size_t = 0 as libc::c_int as size_t;
    let mut fp: *mut FILE = rpl_fopen(file, b"r\0" as *const u8 as *const libc::c_char);
    if fp.is_null() {
        logprintf(
            LOG_NOTQUIET,
            dcgettext(
                0 as *const libc::c_char,
                b"Cannot open cookies file %s: %s\n\0" as *const u8
                    as *const libc::c_char,
                5 as libc::c_int,
            ),
            quote(file),
            strerror(*__errno_location()),
        );
        return;
    }
    cookies_now = time(0 as *mut time_t);
    while getline(&mut line, &mut bufsize, fp) > 0 as libc::c_int as libc::c_long {
        let mut cookie: *mut cookie = 0 as *mut cookie;
        let mut p: *mut libc::c_char = line;
        let mut expiry: libc::c_double = 0.;
        let mut port: libc::c_int = 0;
        let mut domain_b: *mut libc::c_char = 0 as *mut libc::c_char;
        let mut domain_e: *mut libc::c_char = 0 as *mut libc::c_char;
        let mut domflag_b: *mut libc::c_char = 0 as *mut libc::c_char;
        let mut domflag_e: *mut libc::c_char = 0 as *mut libc::c_char;
        let mut path_b: *mut libc::c_char = 0 as *mut libc::c_char;
        let mut path_e: *mut libc::c_char = 0 as *mut libc::c_char;
        let mut secure_b: *mut libc::c_char = 0 as *mut libc::c_char;
        let mut secure_e: *mut libc::c_char = 0 as *mut libc::c_char;
        let mut expires_b: *mut libc::c_char = 0 as *mut libc::c_char;
        let mut expires_e: *mut libc::c_char = 0 as *mut libc::c_char;
        let mut name_b: *mut libc::c_char = 0 as *mut libc::c_char;
        let mut name_e: *mut libc::c_char = 0 as *mut libc::c_char;
        let mut value_b: *mut libc::c_char = 0 as *mut libc::c_char;
        let mut value_e: *mut libc::c_char = 0 as *mut libc::c_char;
        while *p as libc::c_int != 0 && c_isspace(*p as libc::c_int) as libc::c_int != 0
        {
            p = p.offset(1);
            p;
        }
        if *p == 0 || *p as libc::c_int == '#' as i32 {
            continue;
        }
        domain_b = p;
        while *p as libc::c_int != 0 && *p as libc::c_int != '\t' as i32 {
            p = p.offset(1);
            p;
        }
        domain_e = p;
        if domain_b == domain_e || *p == 0 {
            continue;
        }
        p = p.offset(1);
        p;
        domflag_b = p;
        while *p as libc::c_int != 0 && *p as libc::c_int != '\t' as i32 {
            p = p.offset(1);
            p;
        }
        domflag_e = p;
        if domflag_b == domflag_e || *p == 0 {
            continue;
        }
        p = p.offset(1);
        p;
        path_b = p;
        while *p as libc::c_int != 0 && *p as libc::c_int != '\t' as i32 {
            p = p.offset(1);
            p;
        }
        path_e = p;
        if path_b == path_e || *p == 0 {
            continue;
        }
        p = p.offset(1);
        p;
        secure_b = p;
        while *p as libc::c_int != 0 && *p as libc::c_int != '\t' as i32 {
            p = p.offset(1);
            p;
        }
        secure_e = p;
        if secure_b == secure_e || *p == 0 {
            continue;
        }
        p = p.offset(1);
        p;
        expires_b = p;
        while *p as libc::c_int != 0 && *p as libc::c_int != '\t' as i32 {
            p = p.offset(1);
            p;
        }
        expires_e = p;
        if expires_b == expires_e || *p == 0 {
            continue;
        }
        p = p.offset(1);
        p;
        name_b = p;
        while *p as libc::c_int != 0 && *p as libc::c_int != '\t' as i32 {
            p = p.offset(1);
            p;
        }
        name_e = p;
        if name_b == name_e || *p == 0 {
            continue;
        }
        p = p.offset(1);
        p;
        value_b = p;
        value_e = p.offset(strlen(p) as isize);
        if value_e > value_b
            && *value_e.offset(-(1 as libc::c_int) as isize) as libc::c_int
                == '\n' as i32
        {
            value_e = value_e.offset(-1);
            value_e;
        }
        if value_e > value_b
            && *value_e.offset(-(1 as libc::c_int) as isize) as libc::c_int
                == '\r' as i32
        {
            value_e = value_e.offset(-1);
            value_e;
        }
        cookie = cookie_new();
        (*cookie).attr = strdupdelim(name_b, name_e);
        (*cookie).value = strdupdelim(value_b, value_e);
        (*cookie).path = strdupdelim(path_b, path_e);
        (*cookie)
            .set_secure(
                (secure_e.offset_from(secure_b) as libc::c_long as libc::c_ulong
                    == (::std::mem::size_of::<[libc::c_char; 5]>() as libc::c_ulong)
                        .wrapping_sub(1 as libc::c_int as libc::c_ulong)
                    && memcmp(
                        secure_b as *const libc::c_void,
                        b"TRUE\0" as *const u8 as *const libc::c_char
                            as *const libc::c_void,
                        (::std::mem::size_of::<[libc::c_char; 5]>() as libc::c_ulong)
                            .wrapping_sub(1 as libc::c_int as libc::c_ulong),
                    ) == 0) as libc::c_int as libc::c_uint,
            );
        (*cookie)
            .set_domain_exact(
                !(domflag_e.offset_from(domflag_b) as libc::c_long as libc::c_ulong
                    == (::std::mem::size_of::<[libc::c_char; 5]>() as libc::c_ulong)
                        .wrapping_sub(1 as libc::c_int as libc::c_ulong)
                    && memcmp(
                        domflag_b as *const libc::c_void,
                        b"TRUE\0" as *const u8 as *const libc::c_char
                            as *const libc::c_void,
                        (::std::mem::size_of::<[libc::c_char; 5]>() as libc::c_ulong)
                            .wrapping_sub(1 as libc::c_int as libc::c_ulong),
                    ) == 0) as libc::c_int as libc::c_uint,
            );
        port = domain_port(
            domain_b,
            domain_e,
            &mut domain_e as *mut *mut libc::c_char as *mut *const libc::c_char,
        );
        if port != 0 {
            (*cookie).port = port;
        }
        if *domain_b as libc::c_int == '.' as i32 {
            domain_b = domain_b.offset(1);
            domain_b;
        }
        (*cookie).domain = strdupdelim(domain_b, domain_e);
        expiry = cookies_now as libc::c_double - 1 as libc::c_int as libc::c_double;
        *expires_e = '\0' as i32 as libc::c_char;
        sscanf(
            expires_b,
            b"%lf\0" as *const u8 as *const libc::c_char,
            &mut expiry as *mut libc::c_double,
        );
        if !(expiry == 0 as libc::c_int as libc::c_double) {
            if expiry < cookies_now as libc::c_double {
                delete_cookie(cookie);
                continue;
            } else {
                (*cookie).expiry_time = expiry as time_t;
                (*cookie).set_permanent(1 as libc::c_int as libc::c_uint);
            }
        }
        store_cookie(jar, cookie);
    }
    rpl_free(line as *mut libc::c_void);
    line = 0 as *mut libc::c_char;
    fclose(fp);
}
pub unsafe extern "C" fn cookie_jar_save(
    mut jar: *mut cookie_jar,
    mut file: *const libc::c_char,
) {
    let mut fp: *mut FILE = 0 as *mut FILE;
    let mut iter: hash_table_iterator = hash_table_iterator {
        key: 0 as *mut libc::c_void,
        value: 0 as *mut libc::c_void,
        pos: 0 as *mut libc::c_void,
        end: 0 as *mut libc::c_void,
    };
    if opt.debug as libc::c_long != 0 {
        debug_logprintf(
            b"Saving cookies to %s.\n\0" as *const u8 as *const libc::c_char,
            file,
        );
    }
    cookies_now = time(0 as *mut time_t);
    fp = rpl_fopen(file, b"w\0" as *const u8 as *const libc::c_char);
    if fp.is_null() {
        logprintf(
            LOG_NOTQUIET,
            dcgettext(
                0 as *const libc::c_char,
                b"Cannot open cookies file %s: %s\n\0" as *const u8
                    as *const libc::c_char,
                5 as libc::c_int,
            ),
            quote(file),
            strerror(*__errno_location()),
        );
        return;
    }
    fputs(b"# HTTP Cookie File\n\0" as *const u8 as *const libc::c_char, fp);
    fprintf(
        fp,
        b"# Generated by Wget on %s.\n\0" as *const u8 as *const libc::c_char,
        datetime_str(cookies_now),
    );
    fputs(b"# Edit at your own risk.\n\n\0" as *const u8 as *const libc::c_char, fp);
    hash_table_iterate((*jar).chains, &mut iter);
    's_56: while hash_table_iter_next(&mut iter) != 0 {
        let mut domain: *const libc::c_char = iter.key as *const libc::c_char;
        let mut cookie: *mut cookie = iter.value as *mut cookie;
        while !cookie.is_null() {
            if !((*cookie).permanent() == 0 && !opt.keep_session_cookies) {
                if !cookie_expired_p(cookie) {
                    if (*cookie).domain_exact() == 0 {
                        fputc('.' as i32, fp);
                    }
                    fputs(domain, fp);
                    if (*cookie).port != -(1 as libc::c_int) {
                        fprintf(
                            fp,
                            b":%d\0" as *const u8 as *const libc::c_char,
                            (*cookie).port,
                        );
                    }
                    fprintf(
                        fp,
                        b"\t%s\t%s\t%s\t%.0f\t%s\t%s\n\0" as *const u8
                            as *const libc::c_char,
                        if (*cookie).domain_exact() as libc::c_int != 0 {
                            b"FALSE\0" as *const u8 as *const libc::c_char
                        } else {
                            b"TRUE\0" as *const u8 as *const libc::c_char
                        },
                        (*cookie).path,
                        if (*cookie).secure() as libc::c_int != 0 {
                            b"TRUE\0" as *const u8 as *const libc::c_char
                        } else {
                            b"FALSE\0" as *const u8 as *const libc::c_char
                        },
                        (*cookie).expiry_time as libc::c_double,
                        (*cookie).attr,
                        (*cookie).value,
                    );
                    if ferror(fp) != 0 {
                        break 's_56;
                    }
                }
            }
            cookie = (*cookie).next;
        }
    }
    if ferror(fp) != 0 {
        logprintf(
            LOG_NOTQUIET,
            dcgettext(
                0 as *const libc::c_char,
                b"Error writing to %s: %s\n\0" as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ),
            quote(file),
            strerror(*__errno_location()),
        );
    }
    if fclose(fp) < 0 as libc::c_int {
        logprintf(
            LOG_NOTQUIET,
            dcgettext(
                0 as *const libc::c_char,
                b"Error closing %s: %s\n\0" as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ),
            quote(file),
            strerror(*__errno_location()),
        );
    }
    if opt.debug as libc::c_long != 0 {
        debug_logprintf(b"Done saving cookies.\n\0" as *const u8 as *const libc::c_char);
    }
}
pub unsafe extern "C" fn cookie_jar_delete(mut jar: *mut cookie_jar) {
    let mut iter: hash_table_iterator = hash_table_iterator {
        key: 0 as *mut libc::c_void,
        value: 0 as *mut libc::c_void,
        pos: 0 as *mut libc::c_void,
        end: 0 as *mut libc::c_void,
    };
    hash_table_iterate((*jar).chains, &mut iter);
    while hash_table_iter_next(&mut iter) != 0 {
        let mut chain: *mut cookie = iter.value as *mut cookie;
        rpl_free(iter.key);
        iter.key = 0 as *mut libc::c_void;
        while !chain.is_null() {
            let mut next: *mut cookie = (*chain).next;
            delete_cookie(chain);
            chain = next;
        }
    }
    hash_table_destroy((*jar).chains);
    rpl_free(jar as *mut libc::c_void);
    jar = 0 as *mut cookie_jar;
}
