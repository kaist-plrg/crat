use ::libc;
use ::c2rust_bitfields;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    pub type hash_table;
    pub type ptimer;
    fn abort() -> !;
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
    fn strcpy(_: *mut libc::c_char, _: *const libc::c_char) -> *mut libc::c_char;
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    fn strdup(_: *const libc::c_char) -> *mut libc::c_char;
    fn strrchr(_: *const libc::c_char, _: libc::c_int) -> *mut libc::c_char;
    fn strpbrk(_: *const libc::c_char, _: *const libc::c_char) -> *mut libc::c_char;
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    fn strerror(_: libc::c_int) -> *mut libc::c_char;
    fn rpl_free(_: *mut libc::c_void);
    static mut opt: options;
    fn xmalloc(s: size_t) -> *mut libc::c_void;
    fn xstrdup(str: *const libc::c_char) -> *mut libc::c_char;
    fn rename(__old: *const libc::c_char, __new: *const libc::c_char) -> libc::c_int;
    fn fclose(__stream: *mut FILE) -> libc::c_int;
    fn snprintf(
        _: *mut libc::c_char,
        _: libc::c_ulong,
        _: *const libc::c_char,
        _: ...
    ) -> libc::c_int;
    fn set_uri_encoding(i: *mut iri, charset: *const libc::c_char, force: bool);
    fn iri_free(i: *mut iri);
    fn iri_new() -> *mut iri;
    fn rpl_fopen(filename: *const libc::c_char, mode: *const libc::c_char) -> *mut FILE;
    fn logprintf(_: log_options, _: *const libc::c_char, _: ...);
    fn debug_logprintf(_: *const libc::c_char, _: ...);
    fn logputs(_: log_options, _: *const libc::c_char);
    fn quote(arg: *const libc::c_char) -> *const libc::c_char;
    fn putc(__c: libc::c_int, __stream: *mut FILE) -> libc::c_int;
    fn fputs(__s: *const libc::c_char, __stream: *mut FILE) -> libc::c_int;
    fn fwrite(
        _: *const libc::c_void,
        _: libc::c_ulong,
        _: libc::c_ulong,
        _: *mut FILE,
    ) -> libc::c_ulong;
    fn unlink(__name: *const libc::c_char) -> libc::c_int;
    fn __errno_location() -> *mut libc::c_int;
    fn url_parse(
        _: *const libc::c_char,
        _: *mut libc::c_int,
        iri: *mut iri,
        percent_encode: bool,
    ) -> *mut url;
    fn url_free(_: *mut url);
    fn uri_merge(_: *const libc::c_char, _: *const libc::c_char) -> *mut libc::c_char;
    fn aprintf(_: *const libc::c_char, _: ...) -> *mut libc::c_char;
    fn wget_read_file(_: *const libc::c_char) -> *mut file_memory;
    fn wget_read_file_free(_: *mut file_memory);
    fn string_set_add(_: *mut hash_table, _: *const libc::c_char);
    fn string_set_contains(_: *mut hash_table, _: *const libc::c_char) -> libc::c_int;
    fn string_set_to_array(_: *mut hash_table, _: *mut *mut libc::c_char);
    fn print_decimal(_: libc::c_double) -> *const libc::c_char;
    fn hash_table_get(_: *const hash_table, _: *const libc::c_void) -> *mut libc::c_void;
    fn hash_table_get_pair(
        _: *const hash_table,
        _: *const libc::c_void,
        _: *mut libc::c_void,
        _: *mut libc::c_void,
    ) -> libc::c_int;
    fn hash_table_contains(_: *const hash_table, _: *const libc::c_void) -> libc::c_int;
    fn hash_table_put(
        _: *mut hash_table,
        _: *const libc::c_void,
        _: *const libc::c_void,
    );
    fn hash_table_remove(_: *mut hash_table, _: *const libc::c_void) -> libc::c_int;
    fn hash_table_for_each(
        _: *mut hash_table,
        _: Option::<
            unsafe extern "C" fn(
                *mut libc::c_void,
                *mut libc::c_void,
                *mut libc::c_void,
            ) -> libc::c_int,
        >,
        _: *mut libc::c_void,
    );
    fn hash_table_count(_: *const hash_table) -> libc::c_int;
    fn make_string_hash_table(_: libc::c_int) -> *mut hash_table;
    fn ptimer_new() -> *mut ptimer;
    fn ptimer_destroy(_: *mut ptimer);
    fn ptimer_measure(_: *mut ptimer) -> libc::c_double;
    fn get_urls_html(
        _: *const libc::c_char,
        _: *const libc::c_char,
        _: *mut bool,
        _: *mut iri,
    ) -> *mut urlpos;
    fn free_urlpos(_: *mut urlpos);
    fn get_urls_css_file(_: *const libc::c_char, _: *const libc::c_char) -> *mut urlpos;
    fn xstrndup(string: *const libc::c_char, n: size_t) -> *mut libc::c_char;
}
pub type __int64_t = libc::c_long;
pub type __off_t = libc::c_long;
pub type __off64_t = libc::c_long;
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
pub type convert_options = libc::c_uint;
pub const CO_NULLIFY_BASE: convert_options = 4;
pub const CO_CONVERT_TO_COMPLETE: convert_options = 3;
pub const CO_CONVERT_BASENAME_ONLY: convert_options = 2;
pub const CO_CONVERT_TO_RELATIVE: convert_options = 1;
pub const CO_NOCONVERT: convert_options = 0;
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
pub type downloaded_file_t = libc::c_uint;
pub const CHECK_FOR_FILE: downloaded_file_t = 3;
pub const FILE_DOWNLOADED_AND_HTML_EXTENSION_ADDED: downloaded_file_t = 2;
pub const FILE_DOWNLOADED_NORMALLY: downloaded_file_t = 1;
pub const FILE_NOT_ALREADY_DOWNLOADED: downloaded_file_t = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct file_memory {
    pub content: *mut libc::c_char,
    pub length: libc::c_long,
    pub mmap_p: libc::c_int,
}
static mut dl_file_url_map: *mut hash_table = 0 as *const hash_table as *mut hash_table;
pub static mut dl_url_file_map: *mut hash_table = 0 as *const hash_table
    as *mut hash_table;
pub static mut downloaded_html_set: *mut hash_table = 0 as *const hash_table
    as *mut hash_table;
pub static mut downloaded_css_set: *mut hash_table = 0 as *const hash_table
    as *mut hash_table;
unsafe extern "C" fn convert_links_in_hashtable(
    mut downloaded_set: *mut hash_table,
    mut is_css: libc::c_int,
    mut file_count: *mut libc::c_int,
) {
    let mut i: libc::c_int = 0;
    let mut cnt: libc::c_int = 0 as libc::c_int;
    let mut arr: [*mut libc::c_char; 1024] = [0 as *mut libc::c_char; 1024];
    let mut file_array: *mut *mut libc::c_char = 0 as *mut *mut libc::c_char;
    if downloaded_set.is_null()
        || {
            cnt = hash_table_count(downloaded_set);
            cnt == 0 as libc::c_int
        }
    {
        return;
    }
    if cnt
        <= (::std::mem::size_of::<[*mut libc::c_char; 1024]>() as libc::c_ulong)
            .wrapping_div(::std::mem::size_of::<*mut libc::c_char>() as libc::c_ulong)
            as libc::c_int
    {
        file_array = arr.as_mut_ptr();
    } else {
        file_array = xmalloc(
            (cnt as libc::c_ulong)
                .wrapping_mul(
                    ::std::mem::size_of::<*mut libc::c_char>() as libc::c_ulong,
                ),
        ) as *mut *mut libc::c_char;
    }
    string_set_to_array(downloaded_set, file_array);
    i = 0 as libc::c_int;
    while i < cnt {
        let mut urls: *mut urlpos = 0 as *mut urlpos;
        let mut cur_url: *mut urlpos = 0 as *mut urlpos;
        let mut url: *mut libc::c_char = 0 as *mut libc::c_char;
        let mut file: *mut libc::c_char = *file_array.offset(i as isize);
        url = hash_table_get(dl_file_url_map, file as *const libc::c_void)
            as *mut libc::c_char;
        if url.is_null() {
            if opt.debug as libc::c_long != 0 {
                debug_logprintf(
                    b"Apparently %s has been removed.\n\0" as *const u8
                        as *const libc::c_char,
                    file,
                );
            }
        } else {
            if opt.debug as libc::c_long != 0 {
                debug_logprintf(
                    b"Scanning %s (from %s)\n\0" as *const u8 as *const libc::c_char,
                    file,
                    url,
                );
            }
            urls = if is_css != 0 {
                get_urls_css_file(file, url)
            } else {
                get_urls_html(file, url, 0 as *mut bool, 0 as *mut iri)
            };
            cur_url = urls;
            while !cur_url.is_null() {
                let mut local_name: *mut libc::c_char = 0 as *mut libc::c_char;
                let mut u: *mut url = 0 as *mut url;
                let mut pi: *mut iri = 0 as *mut iri;
                if (*cur_url).link_base_p() != 0 {
                    (*cur_url).convert = CO_NULLIFY_BASE;
                } else {
                    pi = iri_new();
                    set_uri_encoding(pi, opt.locale, 1 as libc::c_int != 0);
                    u = url_parse(
                        (*(*cur_url).url).url,
                        0 as *mut libc::c_int,
                        pi,
                        1 as libc::c_int != 0,
                    );
                    if !u.is_null() {
                        local_name = hash_table_get(
                            dl_url_file_map,
                            (*u).url as *const libc::c_void,
                        ) as *mut libc::c_char;
                        if !local_name.is_null() {
                            (*cur_url)
                                .convert = (if opt.convert_file_only as libc::c_int != 0 {
                                CO_CONVERT_BASENAME_ONLY as libc::c_int
                            } else {
                                CO_CONVERT_TO_RELATIVE as libc::c_int
                            }) as convert_options;
                            (*cur_url).local_name = xstrdup(local_name);
                            if opt.debug as libc::c_long != 0 {
                                debug_logprintf(
                                    b"will convert url %s to local %s\n\0" as *const u8
                                        as *const libc::c_char,
                                    (*u).url,
                                    local_name,
                                );
                            }
                        } else {
                            if (*cur_url).link_complete_p() == 0 {
                                (*cur_url).convert = CO_CONVERT_TO_COMPLETE;
                            }
                            (*cur_url).local_name = 0 as *mut libc::c_char;
                            if opt.debug as libc::c_long != 0 {
                                debug_logprintf(
                                    b"will convert url %s to complete\n\0" as *const u8
                                        as *const libc::c_char,
                                    (*u).url,
                                );
                            }
                        }
                        url_free(u);
                        iri_free(pi);
                    }
                }
                cur_url = (*cur_url).next;
            }
            convert_links(file, urls);
            *file_count += 1;
            *file_count;
            free_urlpos(urls);
        }
        i += 1;
        i;
    }
    if file_array != arr.as_mut_ptr() {
        rpl_free(file_array as *mut libc::c_void);
        file_array = 0 as *mut *mut libc::c_char;
    }
}
pub unsafe extern "C" fn convert_all_links() {
    let mut secs: libc::c_double = 0.;
    let mut file_count: libc::c_int = 0 as libc::c_int;
    let mut timer: *mut ptimer = ptimer_new();
    convert_links_in_hashtable(downloaded_html_set, 0 as libc::c_int, &mut file_count);
    convert_links_in_hashtable(downloaded_css_set, 1 as libc::c_int, &mut file_count);
    secs = ptimer_measure(timer);
    logprintf(
        LOG_VERBOSE,
        dcgettext(
            0 as *const libc::c_char,
            b"Converted links in %d files in %s seconds.\n\0" as *const u8
                as *const libc::c_char,
            5 as libc::c_int,
        ),
        file_count,
        print_decimal(secs),
    );
    ptimer_destroy(timer);
}
unsafe extern "C" fn convert_links(
    mut file: *const libc::c_char,
    mut links: *mut urlpos,
) {
    let mut fm: *mut file_memory = 0 as *mut file_memory;
    let mut fp: *mut FILE = 0 as *mut FILE;
    let mut p: *const libc::c_char = 0 as *const libc::c_char;
    let mut downloaded_file_return: downloaded_file_t = FILE_NOT_ALREADY_DOWNLOADED;
    let mut link: *mut urlpos = 0 as *mut urlpos;
    let mut to_url_count: libc::c_int = 0 as libc::c_int;
    let mut to_file_count: libc::c_int = 0 as libc::c_int;
    logprintf(
        LOG_VERBOSE,
        dcgettext(
            0 as *const libc::c_char,
            b"Converting links in %s... \0" as *const u8 as *const libc::c_char,
            5 as libc::c_int,
        ),
        file,
    );
    let mut dry_count: libc::c_int = 0 as libc::c_int;
    let mut dry: *mut urlpos = 0 as *mut urlpos;
    dry = links;
    while !dry.is_null() {
        if (*dry).convert as libc::c_uint != CO_NOCONVERT as libc::c_int as libc::c_uint
        {
            dry_count += 1;
            dry_count;
        }
        dry = (*dry).next;
    }
    if dry_count == 0 {
        logputs(
            LOG_VERBOSE,
            dcgettext(
                0 as *const libc::c_char,
                b"nothing to do.\n\0" as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ),
        );
        return;
    }
    logprintf(
        LOG_VERBOSE,
        dcgettext(
            0 as *const libc::c_char,
            b"%d.\n\0" as *const u8 as *const libc::c_char,
            5 as libc::c_int,
        ),
        dry_count,
    );
    fm = wget_read_file(file);
    if fm.is_null() {
        logprintf(
            LOG_NOTQUIET,
            dcgettext(
                0 as *const libc::c_char,
                b"Cannot convert links in %s: %s\n\0" as *const u8
                    as *const libc::c_char,
                5 as libc::c_int,
            ),
            file,
            strerror(*__errno_location()),
        );
        return;
    }
    downloaded_file_return = downloaded_file(CHECK_FOR_FILE, file);
    if opt.backup_converted as libc::c_int != 0
        && downloaded_file_return as libc::c_uint != 0
    {
        write_backup_file(file, downloaded_file_return);
    }
    if unlink(file) < 0 as libc::c_int && *__errno_location() != 2 as libc::c_int {
        logprintf(
            LOG_NOTQUIET,
            dcgettext(
                0 as *const libc::c_char,
                b"Unable to delete %s: %s\n\0" as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ),
            quote(file),
            strerror(*__errno_location()),
        );
        wget_read_file_free(fm);
        return;
    }
    fp = rpl_fopen(file, b"wb\0" as *const u8 as *const libc::c_char);
    if fp.is_null() {
        logprintf(
            LOG_NOTQUIET,
            dcgettext(
                0 as *const libc::c_char,
                b"Cannot convert links in %s: %s\n\0" as *const u8
                    as *const libc::c_char,
                5 as libc::c_int,
            ),
            file,
            strerror(*__errno_location()),
        );
        wget_read_file_free(fm);
        return;
    }
    p = (*fm).content;
    link = links;
    while !link.is_null() {
        let mut url_start: *mut libc::c_char = ((*fm).content)
            .offset((*link).pos as isize);
        if (*link).pos as libc::c_long >= (*fm).length {
            if opt.debug as libc::c_long != 0 {
                debug_logprintf(
                    b"Something strange is going on.  Please investigate.\0" as *const u8
                        as *const libc::c_char,
                );
            }
            break;
        } else {
            if (*link).convert as libc::c_uint
                == CO_NOCONVERT as libc::c_int as libc::c_uint
            {
                if opt.debug as libc::c_long != 0 {
                    debug_logprintf(
                        b"Skipping %s at position %d.\n\0" as *const u8
                            as *const libc::c_char,
                        (*(*link).url).url,
                        (*link).pos,
                    );
                }
            } else {
                fwrite(
                    p as *const libc::c_void,
                    1 as libc::c_int as libc::c_ulong,
                    url_start.offset_from(p) as libc::c_long as libc::c_ulong,
                    fp,
                );
                p = url_start;
                match (*link).convert as libc::c_uint {
                    1 => {
                        if !((*link).local_name).is_null() {
                            let mut newname: *mut libc::c_char = construct_relative(
                                file,
                                (*link).local_name,
                            );
                            let mut quoted_newname: *mut libc::c_char = local_quote_string(
                                newname,
                                (*link).link_css_p() != 0,
                            );
                            if (*link).link_css_p() as libc::c_int != 0
                                || (*link).link_noquote_html_p() as libc::c_int != 0
                            {
                                p = replace_plain(p, (*link).size, fp, quoted_newname);
                            } else if (*link).link_refresh_p() == 0 {
                                p = replace_attr(p, (*link).size, fp, quoted_newname);
                            } else {
                                p = replace_attr_refresh_hack(
                                    p,
                                    (*link).size,
                                    fp,
                                    quoted_newname,
                                    (*link).refresh_timeout,
                                );
                            }
                            if opt.debug as libc::c_long != 0 {
                                debug_logprintf(
                                    b"TO_RELATIVE: %s to %s at position %d in %s.\n\0"
                                        as *const u8 as *const libc::c_char,
                                    (*(*link).url).url,
                                    newname,
                                    (*link).pos,
                                    file,
                                );
                            }
                            rpl_free(newname as *mut libc::c_void);
                            newname = 0 as *mut libc::c_char;
                            rpl_free(quoted_newname as *mut libc::c_void);
                            quoted_newname = 0 as *mut libc::c_char;
                            to_file_count += 1;
                            to_file_count;
                        }
                    }
                    2 => {
                        let mut newname_0: *mut libc::c_char = convert_basename(p, link);
                        let mut quoted_newname_0: *mut libc::c_char = local_quote_string(
                            newname_0,
                            (*link).link_css_p() != 0,
                        );
                        if (*link).link_css_p() as libc::c_int != 0
                            || (*link).link_noquote_html_p() as libc::c_int != 0
                        {
                            p = replace_plain(p, (*link).size, fp, quoted_newname_0);
                        } else if (*link).link_refresh_p() == 0 {
                            p = replace_attr(p, (*link).size, fp, quoted_newname_0);
                        } else {
                            p = replace_attr_refresh_hack(
                                p,
                                (*link).size,
                                fp,
                                quoted_newname_0,
                                (*link).refresh_timeout,
                            );
                        }
                        if opt.debug as libc::c_long != 0 {
                            debug_logprintf(
                                b"Converted file part only: %s to %s at position %d in %s.\n\0"
                                    as *const u8 as *const libc::c_char,
                                (*(*link).url).url,
                                newname_0,
                                (*link).pos,
                                file,
                            );
                        }
                        rpl_free(newname_0 as *mut libc::c_void);
                        newname_0 = 0 as *mut libc::c_char;
                        rpl_free(quoted_newname_0 as *mut libc::c_void);
                        quoted_newname_0 = 0 as *mut libc::c_char;
                        to_file_count += 1;
                        to_file_count;
                    }
                    3 => {
                        let mut newlink: *mut libc::c_char = (*(*link).url).url;
                        let mut quoted_newlink: *mut libc::c_char = html_quote_string(
                            newlink,
                        );
                        if (*link).link_css_p() as libc::c_int != 0
                            || (*link).link_noquote_html_p() as libc::c_int != 0
                        {
                            p = replace_plain(p, (*link).size, fp, newlink);
                        } else if (*link).link_refresh_p() == 0 {
                            p = replace_attr(p, (*link).size, fp, quoted_newlink);
                        } else {
                            p = replace_attr_refresh_hack(
                                p,
                                (*link).size,
                                fp,
                                quoted_newlink,
                                (*link).refresh_timeout,
                            );
                        }
                        if opt.debug as libc::c_long != 0 {
                            debug_logprintf(
                                b"TO_COMPLETE: <something> to %s at position %d in %s.\n\0"
                                    as *const u8 as *const libc::c_char,
                                newlink,
                                (*link).pos,
                                file,
                            );
                        }
                        rpl_free(quoted_newlink as *mut libc::c_void);
                        quoted_newlink = 0 as *mut libc::c_char;
                        to_url_count += 1;
                        to_url_count;
                    }
                    4 => {
                        p = replace_attr(
                            p,
                            (*link).size,
                            fp,
                            b"\0" as *const u8 as *const libc::c_char,
                        );
                    }
                    0 => {
                        abort();
                    }
                    _ => {}
                }
            }
            link = (*link).next;
        }
    }
    if (p.offset_from((*fm).content) as libc::c_long) < (*fm).length {
        fwrite(
            p as *const libc::c_void,
            1 as libc::c_int as libc::c_ulong,
            ((*fm).length - p.offset_from((*fm).content) as libc::c_long)
                as libc::c_ulong,
            fp,
        );
    }
    fclose(fp);
    wget_read_file_free(fm);
    logprintf(
        LOG_VERBOSE,
        b"%d-%d\n\0" as *const u8 as *const libc::c_char,
        to_file_count,
        to_url_count,
    );
}
unsafe extern "C" fn construct_relative(
    mut basefile: *const libc::c_char,
    mut linkfile: *const libc::c_char,
) -> *mut libc::c_char {
    let mut link: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut basedirs: libc::c_int = 0;
    let mut b: *const libc::c_char = 0 as *const libc::c_char;
    let mut l: *const libc::c_char = 0 as *const libc::c_char;
    let mut i: libc::c_int = 0;
    let mut start: libc::c_int = 0;
    start = 0 as libc::c_int;
    b = basefile;
    l = linkfile;
    while *b as libc::c_int == *l as libc::c_int && *b as libc::c_int != '\0' as i32 {
        if *b as libc::c_int == '/' as i32 {
            start = (b.offset_from(basefile) as libc::c_long
                + 1 as libc::c_int as libc::c_long) as libc::c_int;
        }
        b = b.offset(1);
        b;
        l = l.offset(1);
        l;
    }
    basefile = basefile.offset(start as isize);
    linkfile = linkfile.offset(start as isize);
    basedirs = 0 as libc::c_int;
    b = basefile;
    while *b != 0 {
        if *b as libc::c_int == '/' as i32 {
            basedirs += 1;
            basedirs;
        }
        b = b.offset(1);
        b;
    }
    if basedirs == 0
        && {
            b = strpbrk(linkfile, b"/:\0" as *const u8 as *const libc::c_char);
            !b.is_null()
        } && *b as libc::c_int == ':' as i32
    {
        link = xmalloc(
            (2 as libc::c_int as libc::c_ulong)
                .wrapping_add(strlen(linkfile))
                .wrapping_add(1 as libc::c_int as libc::c_ulong),
        ) as *mut libc::c_char;
        memcpy(
            link as *mut libc::c_void,
            b"./\0" as *const u8 as *const libc::c_char as *const libc::c_void,
            2 as libc::c_int as libc::c_ulong,
        );
        strcpy(link.offset(2 as libc::c_int as isize), linkfile);
    } else {
        link = xmalloc(
            ((3 as libc::c_int * basedirs) as libc::c_ulong)
                .wrapping_add(strlen(linkfile))
                .wrapping_add(1 as libc::c_int as libc::c_ulong),
        ) as *mut libc::c_char;
        i = 0 as libc::c_int;
        while i < basedirs {
            memcpy(
                link.offset((3 as libc::c_int * i) as isize) as *mut libc::c_void,
                b"../\0" as *const u8 as *const libc::c_char as *const libc::c_void,
                3 as libc::c_int as libc::c_ulong,
            );
            i += 1;
            i;
        }
        strcpy(link.offset((3 as libc::c_int * i) as isize), linkfile);
    }
    return link;
}
unsafe extern "C" fn convert_basename(
    mut p: *const libc::c_char,
    mut link: *const urlpos,
) -> *mut libc::c_char {
    let mut len: libc::c_int = (*link).size;
    let mut url: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut org_basename: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut local_basename: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut result: *mut libc::c_char = 0 as *mut libc::c_char;
    if *p as libc::c_int == '"' as i32 || *p as libc::c_int == '\'' as i32 {
        len -= 2 as libc::c_int;
        p = p.offset(1);
        p;
    }
    url = xstrndup(p, len as size_t);
    org_basename = strrchr(url, '/' as i32);
    if !org_basename.is_null() {
        org_basename = org_basename.offset(1);
        org_basename;
    } else {
        org_basename = url;
    }
    local_basename = if !((*link).local_name).is_null() {
        strrchr((*link).local_name, '/' as i32)
    } else {
        0 as *mut libc::c_char
    };
    if !local_basename.is_null() {
        local_basename = local_basename.offset(1);
        local_basename;
    } else {
        local_basename = url;
    }
    if strcmp(org_basename, local_basename) == 0 as libc::c_int {
        result = url;
    } else {
        result = uri_merge(url, local_basename);
        rpl_free(url as *mut libc::c_void);
        url = 0 as *mut libc::c_char;
    }
    return result;
}
static mut converted_files: *mut hash_table = 0 as *const hash_table as *mut hash_table;
unsafe extern "C" fn write_backup_file(
    mut file: *const libc::c_char,
    mut downloaded_file_return: downloaded_file_t,
) {
    if converted_files.is_null() {
        converted_files = make_string_hash_table(0 as libc::c_int);
    }
    if string_set_contains(converted_files, file) == 0 {
        let mut buf: [libc::c_char; 1024] = [0; 1024];
        let mut filename_len: size_t = strlen(file);
        let mut filename_plus_orig_suffix: *mut libc::c_char = 0 as *mut libc::c_char;
        if filename_len
            < (::std::mem::size_of::<[libc::c_char; 1024]>() as libc::c_ulong)
                .wrapping_sub(5 as libc::c_int as libc::c_ulong)
        {
            filename_plus_orig_suffix = buf.as_mut_ptr();
        } else {
            filename_plus_orig_suffix = xmalloc(
                filename_len
                    .wrapping_add(5 as libc::c_int as libc::c_ulong)
                    .wrapping_add(1 as libc::c_int as libc::c_ulong),
            ) as *mut libc::c_char;
        }
        if downloaded_file_return as libc::c_uint
            == FILE_DOWNLOADED_AND_HTML_EXTENSION_ADDED as libc::c_int as libc::c_uint
        {
            memcpy(
                filename_plus_orig_suffix as *mut libc::c_void,
                file as *const libc::c_void,
                filename_len.wrapping_sub(4 as libc::c_int as libc::c_ulong),
            );
            memcpy(
                filename_plus_orig_suffix
                    .offset(filename_len as isize)
                    .offset(-(4 as libc::c_int as isize)) as *mut libc::c_void,
                b"orig\0" as *const u8 as *const libc::c_char as *const libc::c_void,
                5 as libc::c_int as libc::c_ulong,
            );
        } else {
            memcpy(
                filename_plus_orig_suffix as *mut libc::c_void,
                file as *const libc::c_void,
                filename_len,
            );
            strcpy(
                filename_plus_orig_suffix.offset(filename_len as isize),
                b".orig\0" as *const u8 as *const libc::c_char,
            );
        }
        if rename(file, filename_plus_orig_suffix) != 0 as libc::c_int {
            logprintf(
                LOG_NOTQUIET,
                dcgettext(
                    0 as *const libc::c_char,
                    b"Cannot back up %s as %s: %s\n\0" as *const u8
                        as *const libc::c_char,
                    5 as libc::c_int,
                ),
                file,
                filename_plus_orig_suffix,
                strerror(*__errno_location()),
            );
        }
        if filename_plus_orig_suffix != buf.as_mut_ptr() {
            rpl_free(filename_plus_orig_suffix as *mut libc::c_void);
            filename_plus_orig_suffix = 0 as *mut libc::c_char;
        }
        string_set_add(converted_files, file);
    }
}
unsafe extern "C" fn replace_plain(
    mut p: *const libc::c_char,
    mut size: libc::c_int,
    mut fp: *mut FILE,
    mut new_text: *const libc::c_char,
) -> *const libc::c_char {
    fputs(new_text, fp);
    p = p.offset(size as isize);
    return p;
}
unsafe extern "C" fn replace_attr(
    mut p: *const libc::c_char,
    mut size: libc::c_int,
    mut fp: *mut FILE,
    mut new_text: *const libc::c_char,
) -> *const libc::c_char {
    let mut quote_flag: bool = 0 as libc::c_int != 0;
    let mut quote_char: libc::c_char = '"' as i32 as libc::c_char;
    let mut frag_beg: *const libc::c_char = 0 as *const libc::c_char;
    let mut frag_end: *const libc::c_char = 0 as *const libc::c_char;
    if *p as libc::c_int == '"' as i32 || *p as libc::c_int == '\'' as i32 {
        quote_char = *p;
        quote_flag = 1 as libc::c_int != 0;
        p = p.offset(1);
        p;
        size -= 2 as libc::c_int;
    }
    putc(quote_char as libc::c_int, fp);
    fputs(new_text, fp);
    if find_fragment(p, size, &mut frag_beg, &mut frag_end) {
        fwrite(
            frag_beg as *const libc::c_void,
            1 as libc::c_int as libc::c_ulong,
            frag_end.offset_from(frag_beg) as libc::c_long as libc::c_ulong,
            fp,
        );
    }
    p = p.offset(size as isize);
    if quote_flag {
        p = p.offset(1);
        p;
    }
    putc(quote_char as libc::c_int, fp);
    return p;
}
unsafe extern "C" fn replace_attr_refresh_hack(
    mut p: *const libc::c_char,
    mut size: libc::c_int,
    mut fp: *mut FILE,
    mut new_text: *const libc::c_char,
    mut timeout: libc::c_int,
) -> *const libc::c_char {
    let mut new_with_timeout: [libc::c_char; 1024] = [0; 1024];
    if snprintf(
        new_with_timeout.as_mut_ptr(),
        ::std::mem::size_of::<[libc::c_char; 1024]>() as libc::c_ulong,
        b"%d; URL=%s\0" as *const u8 as *const libc::c_char,
        timeout,
        new_text,
    ) as libc::c_uint as libc::c_ulong
        >= ::std::mem::size_of::<[libc::c_char; 1024]>() as libc::c_ulong
    {
        let mut tmp: *mut libc::c_char = aprintf(
            b"%d; URL=%s\0" as *const u8 as *const libc::c_char,
            timeout,
            new_text,
        );
        let mut res: *const libc::c_char = replace_attr(p, size, fp, tmp);
        rpl_free(tmp as *mut libc::c_void);
        tmp = 0 as *mut libc::c_char;
        return res;
    }
    return replace_attr(p, size, fp, new_with_timeout.as_mut_ptr());
}
unsafe extern "C" fn find_fragment(
    mut beg: *const libc::c_char,
    mut size: libc::c_int,
    mut bp: *mut *const libc::c_char,
    mut ep: *mut *const libc::c_char,
) -> bool {
    let mut end: *const libc::c_char = beg.offset(size as isize);
    let mut saw_amp: bool = 0 as libc::c_int != 0;
    while beg < end {
        let mut current_block_6: u64;
        match *beg as libc::c_int {
            38 => {
                saw_amp = 1 as libc::c_int != 0;
                current_block_6 = 5720623009719927633;
            }
            35 => {
                if !saw_amp {
                    *bp = beg;
                    *ep = end;
                    return 1 as libc::c_int != 0;
                }
                current_block_6 = 1865269215649587179;
            }
            _ => {
                current_block_6 = 1865269215649587179;
            }
        }
        match current_block_6 {
            1865269215649587179 => {
                saw_amp = 0 as libc::c_int != 0;
            }
            _ => {}
        }
        beg = beg.offset(1);
        beg;
    }
    return 0 as libc::c_int != 0;
}
unsafe extern "C" fn local_quote_string(
    mut file: *const libc::c_char,
    mut no_html_quote: bool,
) -> *mut libc::c_char {
    let mut from: *const libc::c_char = 0 as *const libc::c_char;
    let mut newname: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut to: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut res: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut buf: [libc::c_char; 1024] = [0; 1024];
    let mut tolen: size_t = 0;
    let mut any: *mut libc::c_char = strpbrk(
        file,
        b"?#%;\0" as *const u8 as *const libc::c_char,
    );
    if any.is_null() {
        return if no_html_quote as libc::c_int != 0 {
            strdup(file)
        } else {
            html_quote_string(file)
        };
    }
    tolen = (3 as libc::c_int as libc::c_ulong).wrapping_mul(strlen(file));
    if tolen < ::std::mem::size_of::<[libc::c_char; 1024]>() as libc::c_ulong {
        newname = buf.as_mut_ptr();
        to = newname;
    } else {
        newname = xmalloc(tolen.wrapping_add(1 as libc::c_int as libc::c_ulong))
            as *mut libc::c_char;
        to = newname;
    }
    from = file;
    while *from != 0 {
        let mut current_block_19: u64;
        match *from as libc::c_int {
            37 => {
                let fresh0 = to;
                to = to.offset(1);
                *fresh0 = '%' as i32 as libc::c_char;
                let fresh1 = to;
                to = to.offset(1);
                *fresh1 = '2' as i32 as libc::c_char;
                let fresh2 = to;
                to = to.offset(1);
                *fresh2 = '5' as i32 as libc::c_char;
                current_block_19 = 4495394744059808450;
            }
            35 => {
                let fresh3 = to;
                to = to.offset(1);
                *fresh3 = '%' as i32 as libc::c_char;
                let fresh4 = to;
                to = to.offset(1);
                *fresh4 = '2' as i32 as libc::c_char;
                let fresh5 = to;
                to = to.offset(1);
                *fresh5 = '3' as i32 as libc::c_char;
                current_block_19 = 4495394744059808450;
            }
            59 => {
                let fresh6 = to;
                to = to.offset(1);
                *fresh6 = '%' as i32 as libc::c_char;
                let fresh7 = to;
                to = to.offset(1);
                *fresh7 = '3' as i32 as libc::c_char;
                let fresh8 = to;
                to = to.offset(1);
                *fresh8 = 'B' as i32 as libc::c_char;
                current_block_19 = 4495394744059808450;
            }
            63 => {
                if opt.adjust_extension {
                    let fresh9 = to;
                    to = to.offset(1);
                    *fresh9 = '%' as i32 as libc::c_char;
                    let fresh10 = to;
                    to = to.offset(1);
                    *fresh10 = '3' as i32 as libc::c_char;
                    let fresh11 = to;
                    to = to.offset(1);
                    *fresh11 = 'F' as i32 as libc::c_char;
                    current_block_19 = 4495394744059808450;
                } else {
                    current_block_19 = 3723248803545331543;
                }
            }
            _ => {
                current_block_19 = 3723248803545331543;
            }
        }
        match current_block_19 {
            3723248803545331543 => {
                let fresh12 = to;
                to = to.offset(1);
                *fresh12 = *from;
            }
            _ => {}
        }
        from = from.offset(1);
        from;
    }
    *to = '\0' as i32 as libc::c_char;
    if newname == buf.as_mut_ptr() {
        return if no_html_quote as libc::c_int != 0 {
            strdup(newname)
        } else {
            html_quote_string(newname)
        };
    }
    if no_html_quote {
        return newname;
    }
    res = html_quote_string(newname);
    rpl_free(newname as *mut libc::c_void);
    newname = 0 as *mut libc::c_char;
    return res;
}
unsafe extern "C" fn match_except_index(
    mut s1: *const libc::c_char,
    mut s2: *const libc::c_char,
) -> bool {
    let mut i: libc::c_int = 0;
    let mut lng: *const libc::c_char = 0 as *const libc::c_char;
    i = 0 as libc::c_int;
    while *s1 as libc::c_int != 0 && *s2 as libc::c_int != 0
        && *s1 as libc::c_int == *s2 as libc::c_int
    {
        s1 = s1.offset(1);
        s1;
        s2 = s2.offset(1);
        s2;
        i += 1;
        i;
    }
    if i == 0 as libc::c_int {
        return 0 as libc::c_int != 0;
    }
    if *s1 == 0 && *s2 == 0 {
        return 1 as libc::c_int != 0
    } else if *s1 as libc::c_int != 0 && *s2 as libc::c_int != 0 {
        return 0 as libc::c_int != 0
    } else if *s1 != 0 {
        lng = s1;
    } else {
        lng = s2;
    }
    if *lng as libc::c_int != '/' as i32 {
        lng = lng.offset(-1);
        lng;
    }
    if *lng as libc::c_int == '/' as i32
        && *lng.offset(1 as libc::c_int as isize) as libc::c_int == '\0' as i32
    {
        return 1 as libc::c_int != 0;
    }
    return 0 as libc::c_int
        == strcmp(lng, b"/index.html\0" as *const u8 as *const libc::c_char);
}
unsafe extern "C" fn dissociate_urls_from_file_mapper(
    mut key: *mut libc::c_void,
    mut value: *mut libc::c_void,
    mut arg: *mut libc::c_void,
) -> libc::c_int {
    let mut mapping_url: *mut libc::c_char = key as *mut libc::c_char;
    let mut mapping_file: *mut libc::c_char = value as *mut libc::c_char;
    let mut file: *mut libc::c_char = arg as *mut libc::c_char;
    if 0 as libc::c_int == strcmp(mapping_file, file) {
        hash_table_remove(dl_url_file_map, mapping_url as *const libc::c_void);
        rpl_free(mapping_url as *mut libc::c_void);
        mapping_url = 0 as *mut libc::c_char;
        rpl_free(mapping_file as *mut libc::c_void);
        mapping_file = 0 as *mut libc::c_char;
    }
    return 0 as libc::c_int;
}
unsafe extern "C" fn dissociate_urls_from_file(mut file: *const libc::c_char) {
    hash_table_for_each(
        dl_url_file_map,
        Some(
            dissociate_urls_from_file_mapper
                as unsafe extern "C" fn(
                    *mut libc::c_void,
                    *mut libc::c_void,
                    *mut libc::c_void,
                ) -> libc::c_int,
        ),
        file as *mut libc::c_char as *mut libc::c_void,
    );
}
pub unsafe extern "C" fn register_download(
    mut url: *const libc::c_char,
    mut file: *const libc::c_char,
) {
    let mut current_block: u64;
    let mut old_file: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut old_url: *mut libc::c_char = 0 as *mut libc::c_char;
    if dl_file_url_map.is_null() {
        dl_file_url_map = make_string_hash_table(0 as libc::c_int);
    }
    if dl_url_file_map.is_null() {
        dl_url_file_map = make_string_hash_table(0 as libc::c_int);
    }
    if hash_table_get_pair(
        dl_file_url_map,
        file as *const libc::c_void,
        &mut old_file as *mut *mut libc::c_char as *mut libc::c_void,
        &mut old_url as *mut *mut libc::c_char as *mut libc::c_void,
    ) != 0
    {
        if 0 as libc::c_int == strcmp(url, old_url) {
            return;
        }
        if match_except_index(url, old_url) as libc::c_int != 0
            && hash_table_contains(dl_url_file_map, url as *const libc::c_void) == 0
        {
            current_block = 15734768397315445535;
        } else {
            hash_table_remove(dl_file_url_map, file as *const libc::c_void);
            rpl_free(old_file as *mut libc::c_void);
            old_file = 0 as *mut libc::c_char;
            rpl_free(old_url as *mut libc::c_void);
            old_url = 0 as *mut libc::c_char;
            dissociate_urls_from_file(file);
            current_block = 5143058163439228106;
        }
    } else {
        current_block = 5143058163439228106;
    }
    match current_block {
        5143058163439228106 => {
            hash_table_put(
                dl_file_url_map,
                xstrdup(file) as *const libc::c_void,
                xstrdup(url) as *const libc::c_void,
            );
        }
        _ => {}
    }
    if hash_table_get_pair(
        dl_url_file_map,
        url as *const libc::c_void,
        &mut old_url as *mut *mut libc::c_char as *mut libc::c_void,
        &mut old_file as *mut *mut libc::c_char as *mut libc::c_void,
    ) != 0
    {
        hash_table_remove(dl_url_file_map, url as *const libc::c_void);
        rpl_free(old_url as *mut libc::c_void);
        old_url = 0 as *mut libc::c_char;
        rpl_free(old_file as *mut libc::c_void);
        old_file = 0 as *mut libc::c_char;
    }
    hash_table_put(
        dl_url_file_map,
        xstrdup(url) as *const libc::c_void,
        xstrdup(file) as *const libc::c_void,
    );
}
pub unsafe extern "C" fn register_redirection(
    mut from: *const libc::c_char,
    mut to: *const libc::c_char,
) {
    let mut file: *mut libc::c_char = 0 as *mut libc::c_char;
    if dl_file_url_map.is_null() {
        dl_file_url_map = make_string_hash_table(0 as libc::c_int);
    }
    if dl_url_file_map.is_null() {
        dl_url_file_map = make_string_hash_table(0 as libc::c_int);
    }
    file = hash_table_get(dl_url_file_map, to as *const libc::c_void)
        as *mut libc::c_char;
    if hash_table_contains(dl_url_file_map, from as *const libc::c_void) == 0 {
        hash_table_put(
            dl_url_file_map,
            xstrdup(from) as *const libc::c_void,
            xstrdup(file) as *const libc::c_void,
        );
    }
}
pub unsafe extern "C" fn register_delete_file(mut file: *const libc::c_char) {
    let mut old_url: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut old_file: *mut libc::c_char = 0 as *mut libc::c_char;
    if dl_file_url_map.is_null() {
        dl_file_url_map = make_string_hash_table(0 as libc::c_int);
    }
    if dl_url_file_map.is_null() {
        dl_url_file_map = make_string_hash_table(0 as libc::c_int);
    }
    if hash_table_get_pair(
        dl_file_url_map,
        file as *const libc::c_void,
        &mut old_file as *mut *mut libc::c_char as *mut libc::c_void,
        &mut old_url as *mut *mut libc::c_char as *mut libc::c_void,
    ) == 0
    {
        return;
    }
    hash_table_remove(dl_file_url_map, file as *const libc::c_void);
    rpl_free(old_file as *mut libc::c_void);
    old_file = 0 as *mut libc::c_char;
    rpl_free(old_url as *mut libc::c_void);
    old_url = 0 as *mut libc::c_char;
    dissociate_urls_from_file(file);
}
pub unsafe extern "C" fn register_html(mut file: *const libc::c_char) {
    if downloaded_html_set.is_null() {
        downloaded_html_set = make_string_hash_table(0 as libc::c_int);
    }
    string_set_add(downloaded_html_set, file);
}
pub unsafe extern "C" fn register_css(mut file: *const libc::c_char) {
    if downloaded_css_set.is_null() {
        downloaded_css_set = make_string_hash_table(0 as libc::c_int);
    }
    string_set_add(downloaded_css_set, file);
}
static mut downloaded_files_hash: *mut hash_table = 0 as *const hash_table
    as *mut hash_table;
unsafe extern "C" fn downloaded_mode_to_ptr(
    mut mode: downloaded_file_t,
) -> *mut downloaded_file_t {
    static mut v1: downloaded_file_t = FILE_NOT_ALREADY_DOWNLOADED;
    static mut v2: downloaded_file_t = FILE_DOWNLOADED_NORMALLY;
    static mut v3: downloaded_file_t = FILE_DOWNLOADED_AND_HTML_EXTENSION_ADDED;
    static mut v4: downloaded_file_t = CHECK_FOR_FILE;
    match mode as libc::c_uint {
        0 => return &mut v1,
        1 => return &mut v2,
        2 => return &mut v3,
        3 => return &mut v4,
        _ => {}
    }
    return 0 as *mut downloaded_file_t;
}
pub unsafe extern "C" fn downloaded_file(
    mut mode: downloaded_file_t,
    mut file: *const libc::c_char,
) -> downloaded_file_t {
    let mut ptr: *mut downloaded_file_t = 0 as *mut downloaded_file_t;
    if mode as libc::c_uint == CHECK_FOR_FILE as libc::c_int as libc::c_uint {
        if downloaded_files_hash.is_null() {
            return FILE_NOT_ALREADY_DOWNLOADED;
        }
        ptr = hash_table_get(downloaded_files_hash, file as *const libc::c_void)
            as *mut downloaded_file_t;
        if ptr.is_null() {
            return FILE_NOT_ALREADY_DOWNLOADED;
        }
        return *ptr;
    }
    if downloaded_files_hash.is_null() {
        downloaded_files_hash = make_string_hash_table(0 as libc::c_int);
    }
    ptr = hash_table_get(downloaded_files_hash, file as *const libc::c_void)
        as *mut downloaded_file_t;
    if !ptr.is_null() {
        return *ptr;
    }
    ptr = downloaded_mode_to_ptr(mode);
    hash_table_put(
        downloaded_files_hash,
        xstrdup(file) as *const libc::c_void,
        ptr as *const libc::c_void,
    );
    return FILE_NOT_ALREADY_DOWNLOADED;
}
pub unsafe extern "C" fn html_quote_string(
    mut s: *const libc::c_char,
) -> *mut libc::c_char {
    let mut b: *const libc::c_char = s;
    let mut p: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut res: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut i: libc::c_int = 0;
    i = 0 as libc::c_int;
    while *s != 0 {
        if *s as libc::c_int == '&' as i32 {
            i += 4 as libc::c_int;
        } else if *s as libc::c_int == '<' as i32 || *s as libc::c_int == '>' as i32 {
            i += 3 as libc::c_int;
        } else if *s as libc::c_int == '"' as i32 {
            i += 5 as libc::c_int;
        } else if *s as libc::c_int == ' ' as i32 {
            i += 4 as libc::c_int;
        }
        s = s.offset(1);
        s;
        i += 1;
        i;
    }
    res = xmalloc((i + 1 as libc::c_int) as size_t) as *mut libc::c_char;
    s = b;
    p = res;
    while *s != 0 {
        match *s as libc::c_int {
            38 => {
                let fresh13 = p;
                p = p.offset(1);
                *fresh13 = '&' as i32 as libc::c_char;
                let fresh14 = p;
                p = p.offset(1);
                *fresh14 = 'a' as i32 as libc::c_char;
                let fresh15 = p;
                p = p.offset(1);
                *fresh15 = 'm' as i32 as libc::c_char;
                let fresh16 = p;
                p = p.offset(1);
                *fresh16 = 'p' as i32 as libc::c_char;
                let fresh17 = p;
                p = p.offset(1);
                *fresh17 = ';' as i32 as libc::c_char;
            }
            60 | 62 => {
                let fresh18 = p;
                p = p.offset(1);
                *fresh18 = '&' as i32 as libc::c_char;
                let fresh19 = p;
                p = p.offset(1);
                *fresh19 = (if *s as libc::c_int == '<' as i32 {
                    'l' as i32
                } else {
                    'g' as i32
                }) as libc::c_char;
                let fresh20 = p;
                p = p.offset(1);
                *fresh20 = 't' as i32 as libc::c_char;
                let fresh21 = p;
                p = p.offset(1);
                *fresh21 = ';' as i32 as libc::c_char;
            }
            34 => {
                let fresh22 = p;
                p = p.offset(1);
                *fresh22 = '&' as i32 as libc::c_char;
                let fresh23 = p;
                p = p.offset(1);
                *fresh23 = 'q' as i32 as libc::c_char;
                let fresh24 = p;
                p = p.offset(1);
                *fresh24 = 'u' as i32 as libc::c_char;
                let fresh25 = p;
                p = p.offset(1);
                *fresh25 = 'o' as i32 as libc::c_char;
                let fresh26 = p;
                p = p.offset(1);
                *fresh26 = 't' as i32 as libc::c_char;
                let fresh27 = p;
                p = p.offset(1);
                *fresh27 = ';' as i32 as libc::c_char;
            }
            32 => {
                let fresh28 = p;
                p = p.offset(1);
                *fresh28 = '&' as i32 as libc::c_char;
                let fresh29 = p;
                p = p.offset(1);
                *fresh29 = '#' as i32 as libc::c_char;
                let fresh30 = p;
                p = p.offset(1);
                *fresh30 = '3' as i32 as libc::c_char;
                let fresh31 = p;
                p = p.offset(1);
                *fresh31 = '2' as i32 as libc::c_char;
                let fresh32 = p;
                p = p.offset(1);
                *fresh32 = ';' as i32 as libc::c_char;
            }
            _ => {
                let fresh33 = p;
                p = p.offset(1);
                *fresh33 = *s;
            }
        }
        s = s.offset(1);
        s;
    }
    *p = '\0' as i32 as libc::c_char;
    return res;
}
