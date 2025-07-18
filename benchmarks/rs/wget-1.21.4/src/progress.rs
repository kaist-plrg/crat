use ::libc;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    static mut stderr: *mut FILE;
    fn fprintf(_: *mut FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn sprintf(_: *mut libc::c_char, _: *const libc::c_char, _: ...) -> libc::c_int;
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
    fn strcpy(_: *mut libc::c_char, _: *const libc::c_char) -> *mut libc::c_char;
    fn strncmp(
        _: *const libc::c_char,
        _: *const libc::c_char,
        _: libc::c_ulong,
    ) -> libc::c_int;
    fn strchr(_: *const libc::c_char, _: libc::c_int) -> *mut libc::c_char;
    fn strchrnul(__s: *const libc::c_char, __c: libc::c_int) -> *mut libc::c_char;
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    fn rpl_free(_: *mut libc::c_void);
    static mut opt: options;
    fn abort() -> !;
    fn xmalloc(s: size_t) -> *mut libc::c_void;
    fn xcalloc(n: size_t, s: size_t) -> *mut libc::c_void;
    fn xrealloc(p: *mut libc::c_void, s: size_t) -> *mut libc::c_void;
    fn fileno(__stream: *mut FILE) -> libc::c_int;
    fn logprintf(_: log_options, _: *const libc::c_char, _: ...);
    fn logputs(_: log_options, _: *const libc::c_char);
    fn log_set_flush(_: bool);
    fn log_set_save_context(_: bool) -> bool;
    fn quote(arg: *const libc::c_char) -> *const libc::c_char;
    fn isatty(__fd: libc::c_int) -> libc::c_int;
    fn signal(__sig: libc::c_int, __handler: __sighandler_t) -> __sighandler_t;
    fn mbsinit(__ps: *const mbstate_t) -> libc::c_int;
    fn wcwidth(__c: wchar_t) -> libc::c_int;
    fn rpl_mbrtowc(
        pwc: *mut wchar_t,
        s: *const libc::c_char,
        n: size_t,
        ps: *mut mbstate_t,
    ) -> size_t;
    fn iswcntrl(__wc: wint_t) -> libc::c_int;
    fn iswprint(__wc: wint_t) -> libc::c_int;
    static is_basic_table: [libc::c_uint; 0];
    fn human_readable(_: wgint, _: libc::c_int, _: libc::c_int) -> *mut libc::c_char;
    fn numdigit(_: wgint) -> libc::c_int;
    fn number_to_static_string(_: wgint) -> *mut libc::c_char;
    fn determine_screen_width() -> libc::c_int;
    fn print_decimal(_: libc::c_double) -> *const libc::c_char;
    fn calc_rate(_: wgint, _: libc::c_double, _: *mut libc::c_int) -> libc::c_double;
    fn c_strcasecmp(s1: *const libc::c_char, s2: *const libc::c_char) -> libc::c_int;
}
pub type __int64_t = libc::c_long;
pub type __off_t = libc::c_long;
pub type __off64_t = libc::c_long;
pub type __sig_atomic_t = libc::c_int;
pub type size_t = libc::c_ulong;
pub type int64_t = __int64_t;
pub type wchar_t = libc::c_int;
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
pub struct mbstate_t {
    pub __count: libc::c_int,
    pub __value: C2RustUnnamed_3,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_3 {
    pub __wch: libc::c_uint,
    pub __wchb: [libc::c_char; 4],
}
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
pub type sig_atomic_t = __sig_atomic_t;
pub type __sighandler_t = Option::<unsafe extern "C" fn(libc::c_int) -> ()>;
pub type wint_t = libc::c_uint;
pub type __mbstate_t = mbstate_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct mbchar {
    pub ptr: *const libc::c_char,
    pub bytes: size_t,
    pub wc_valid: bool,
    pub wc: wchar_t,
    pub buf: [libc::c_char; 24],
}
pub type mbchar_t = mbchar;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct mbiter_multi {
    pub limit: *const libc::c_char,
    pub in_shift: bool,
    pub state: mbstate_t,
    pub next_done: bool,
    pub cur: mbchar,
}
pub type mbi_iterator_t = mbiter_multi;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct progress_implementation {
    pub name: *const libc::c_char,
    pub interactive: bool,
    pub create: Option::<
        unsafe extern "C" fn(*const libc::c_char, wgint, wgint) -> *mut libc::c_void,
    >,
    pub update: Option::<
        unsafe extern "C" fn(*mut libc::c_void, wgint, libc::c_double) -> (),
    >,
    pub draw: Option::<unsafe extern "C" fn(*mut libc::c_void) -> ()>,
    pub finish: Option::<unsafe extern "C" fn(*mut libc::c_void, libc::c_double) -> ()>,
    pub set_params: Option::<unsafe extern "C" fn(*const libc::c_char) -> ()>,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct bar_progress {
    pub f_download: *mut libc::c_char,
    pub initial_length: wgint,
    pub total_length: wgint,
    pub count: wgint,
    pub last_screen_update: libc::c_double,
    pub dltime: libc::c_double,
    pub width: libc::c_int,
    pub buffer: *mut libc::c_char,
    pub tick: libc::c_int,
    pub hist: bar_progress_hist,
    pub recent_start: libc::c_double,
    pub recent_bytes: wgint,
    pub stalled: bool,
    pub last_eta_time: libc::c_double,
    pub last_eta_value: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct bar_progress_hist {
    pub pos: libc::c_int,
    pub times: [libc::c_double; 20],
    pub bytes: [wgint; 20],
    pub total_time: libc::c_double,
    pub total_bytes: wgint,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct dot_progress {
    pub initial_length: wgint,
    pub total_length: wgint,
    pub accumulated: wgint,
    pub dltime: libc::c_double,
    pub rows: wgint,
    pub dots: libc::c_int,
    pub last_timer_value: libc::c_double,
}
#[inline]
unsafe extern "C" fn mbiter_multi_next(mut iter: *mut mbiter_multi) {
    let mut current_block: u64;
    if (*iter).next_done {
        return;
    }
    if (*iter).in_shift {
        current_block = 9223043375784382784;
    } else if is_basic(*(*iter).cur.ptr) {
        (*iter).cur.bytes = 1 as libc::c_int as size_t;
        (*iter).cur.wc = *(*iter).cur.ptr as wchar_t;
        (*iter).cur.wc_valid = 1 as libc::c_int != 0;
        current_block = 15089075282327824602;
    } else {
        (*iter).in_shift = 1 as libc::c_int != 0;
        current_block = 9223043375784382784;
    }
    match current_block {
        9223043375784382784 => {
            (*iter)
                .cur
                .bytes = rpl_mbrtowc(
                &mut (*iter).cur.wc,
                (*iter).cur.ptr,
                ((*iter).limit).offset_from((*iter).cur.ptr) as libc::c_long as size_t,
                &mut (*iter).state,
            );
            if (*iter).cur.bytes == -(1 as libc::c_int) as size_t {
                (*iter).cur.bytes = 1 as libc::c_int as size_t;
                (*iter).cur.wc_valid = 0 as libc::c_int != 0;
            } else if (*iter).cur.bytes == -(2 as libc::c_int) as size_t {
                (*iter)
                    .cur
                    .bytes = ((*iter).limit).offset_from((*iter).cur.ptr) as libc::c_long
                    as size_t;
                (*iter).cur.wc_valid = 0 as libc::c_int != 0;
            } else {
                if (*iter).cur.bytes == 0 as libc::c_int as libc::c_ulong {
                    (*iter).cur.bytes = 1 as libc::c_int as size_t;
                }
                (*iter).cur.wc_valid = 1 as libc::c_int != 0;
                if mbsinit(&mut (*iter).state) != 0 {
                    (*iter).in_shift = 0 as libc::c_int != 0;
                }
            }
        }
        _ => {}
    }
    (*iter).next_done = 1 as libc::c_int != 0;
}
#[inline]
unsafe extern "C" fn mb_width_aux(mut wc: wint_t) -> libc::c_int {
    let mut w: libc::c_int = wcwidth(wc as wchar_t);
    return if w >= 0 as libc::c_int {
        w
    } else if iswcntrl(wc) != 0 {
        0 as libc::c_int
    } else {
        1 as libc::c_int
    };
}
#[inline]
unsafe extern "C" fn is_basic(mut c: libc::c_char) -> bool {
    return *is_basic_table
        .as_ptr()
        .offset((c as libc::c_uchar as libc::c_int >> 5 as libc::c_int) as isize)
        >> (c as libc::c_uchar as libc::c_int & 31 as libc::c_int)
        & 1 as libc::c_int as libc::c_uint != 0;
}
static mut implementations: [progress_implementation; 2] = unsafe {
    [
        {
            let mut init = progress_implementation {
                name: b"dot\0" as *const u8 as *const libc::c_char,
                interactive: 0 as libc::c_int != 0,
                create: Some(
                    dot_create
                        as unsafe extern "C" fn(
                            *const libc::c_char,
                            wgint,
                            wgint,
                        ) -> *mut libc::c_void,
                ),
                update: Some(
                    dot_update
                        as unsafe extern "C" fn(
                            *mut libc::c_void,
                            wgint,
                            libc::c_double,
                        ) -> (),
                ),
                draw: Some(dot_draw as unsafe extern "C" fn(*mut libc::c_void) -> ()),
                finish: Some(
                    dot_finish
                        as unsafe extern "C" fn(*mut libc::c_void, libc::c_double) -> (),
                ),
                set_params: Some(
                    dot_set_params as unsafe extern "C" fn(*const libc::c_char) -> (),
                ),
            };
            init
        },
        {
            let mut init = progress_implementation {
                name: b"bar\0" as *const u8 as *const libc::c_char,
                interactive: 1 as libc::c_int != 0,
                create: Some(
                    bar_create
                        as unsafe extern "C" fn(
                            *const libc::c_char,
                            wgint,
                            wgint,
                        ) -> *mut libc::c_void,
                ),
                update: Some(
                    bar_update
                        as unsafe extern "C" fn(
                            *mut libc::c_void,
                            wgint,
                            libc::c_double,
                        ) -> (),
                ),
                draw: Some(bar_draw as unsafe extern "C" fn(*mut libc::c_void) -> ()),
                finish: Some(
                    bar_finish
                        as unsafe extern "C" fn(*mut libc::c_void, libc::c_double) -> (),
                ),
                set_params: Some(
                    bar_set_params as unsafe extern "C" fn(*const libc::c_char) -> (),
                ),
            };
            init
        },
    ]
};
static mut current_impl: *mut progress_implementation = 0
    as *const progress_implementation as *mut progress_implementation;
static mut current_impl_locked: libc::c_int = 0;
pub unsafe extern "C" fn valid_progress_implementation_p(
    mut name: *const libc::c_char,
) -> bool {
    let mut i: size_t = 0;
    let mut pi: *mut progress_implementation = implementations.as_mut_ptr();
    let mut colon: *mut libc::c_char = strchr(name, ':' as i32);
    let mut namelen: size_t = if !colon.is_null() {
        colon.offset_from(name) as libc::c_long as size_t
    } else {
        strlen(name)
    };
    i = 0 as libc::c_int as size_t;
    while i
        < (::std::mem::size_of::<[progress_implementation; 2]>() as libc::c_ulong)
            .wrapping_div(
                ::std::mem::size_of::<progress_implementation>() as libc::c_ulong,
            )
    {
        if strncmp((*pi).name, name, namelen) == 0 {
            return 1 as libc::c_int != 0;
        }
        i = i.wrapping_add(1);
        i;
        pi = pi.offset(1);
        pi;
    }
    return 0 as libc::c_int != 0;
}
pub unsafe extern "C" fn set_progress_implementation(mut name: *const libc::c_char) {
    let mut i: size_t = 0;
    let mut namelen: size_t = 0;
    let mut pi: *mut progress_implementation = implementations.as_mut_ptr();
    let mut colon: *const libc::c_char = 0 as *const libc::c_char;
    if name.is_null() {
        name = b"bar\0" as *const u8 as *const libc::c_char;
    }
    colon = strchr(name, ':' as i32);
    namelen = if !colon.is_null() {
        colon.offset_from(name) as libc::c_long as size_t
    } else {
        strlen(name)
    };
    i = 0 as libc::c_int as size_t;
    while i
        < (::std::mem::size_of::<[progress_implementation; 2]>() as libc::c_ulong)
            .wrapping_div(
                ::std::mem::size_of::<progress_implementation>() as libc::c_ulong,
            )
    {
        if strncmp((*pi).name, name, namelen) == 0 {
            current_impl = pi;
            current_impl_locked = 0 as libc::c_int;
            if !colon.is_null() {
                colon = colon.offset(1);
                colon;
            }
            if ((*pi).set_params).is_some() {
                ((*pi).set_params).unwrap()(colon);
            }
            return;
        }
        i = i.wrapping_add(1);
        i;
        pi = pi.offset(1);
        pi;
    }
    abort();
}
static mut output_redirected: libc::c_int = 0;
pub unsafe extern "C" fn progress_schedule_redirect() {
    output_redirected = 1 as libc::c_int;
}
pub unsafe extern "C" fn progress_create(
    mut f_download: *const libc::c_char,
    mut initial: wgint,
    mut total: wgint,
) -> *mut libc::c_void {
    if output_redirected != 0 {
        if current_impl_locked == 0 {
            set_progress_implementation(b"dot\0" as *const u8 as *const libc::c_char);
        }
        output_redirected = 0 as libc::c_int;
    }
    return ((*current_impl).create).unwrap()(f_download, initial, total);
}
pub unsafe extern "C" fn progress_interactive_p(
    mut progress: *mut libc::c_void,
) -> bool {
    return (*current_impl).interactive;
}
pub unsafe extern "C" fn progress_update(
    mut progress: *mut libc::c_void,
    mut howmuch: wgint,
    mut dltime: libc::c_double,
) {
    if dltime >= 2147483647 as libc::c_int as libc::c_double {
        dltime = (2147483647 as libc::c_int - 1 as libc::c_int) as libc::c_double;
    } else if dltime < 0 as libc::c_int as libc::c_double {
        dltime = 0 as libc::c_int as libc::c_double;
    }
    if howmuch < 0 as libc::c_int as libc::c_long {
        howmuch = 0 as libc::c_int as wgint;
    }
    ((*current_impl).update).unwrap()(progress, howmuch, dltime);
    ((*current_impl).draw).unwrap()(progress);
}
pub unsafe extern "C" fn progress_finish(
    mut progress: *mut libc::c_void,
    mut dltime: libc::c_double,
) {
    if dltime >= 2147483647 as libc::c_int as libc::c_double {
        dltime = (2147483647 as libc::c_int - 1 as libc::c_int) as libc::c_double;
    } else if dltime < 0 as libc::c_int as libc::c_double {
        dltime = 0 as libc::c_int as libc::c_double;
    }
    ((*current_impl).finish).unwrap()(progress, dltime);
}
unsafe extern "C" fn dot_create(
    mut f_download: *const libc::c_char,
    mut initial: wgint,
    mut total: wgint,
) -> *mut libc::c_void {
    let mut dp: *mut dot_progress = xcalloc(
        1 as libc::c_int as size_t,
        ::std::mem::size_of::<dot_progress>() as libc::c_ulong,
    ) as *mut dot_progress;
    (*dp).initial_length = initial;
    (*dp).total_length = total;
    if (*dp).initial_length != 0 {
        let mut dot_bytes: libc::c_int = opt.dot_bytes as libc::c_int;
        let ROW_BYTES: wgint = opt.dot_bytes * opt.dots_in_line as libc::c_long;
        let mut remainder: libc::c_int = ((*dp).initial_length % ROW_BYTES)
            as libc::c_int;
        let mut skipped: wgint = (*dp).initial_length - remainder as libc::c_long;
        if skipped != 0 {
            let mut skipped_k: wgint = skipped / 1024 as libc::c_int as libc::c_long;
            let mut skipped_k_len: libc::c_int = numdigit(skipped_k);
            if skipped_k_len < 6 as libc::c_int {
                skipped_k_len = 6 as libc::c_int;
            }
            logprintf(
                LOG_PROGRESS,
                dcgettext(
                    0 as *const libc::c_char,
                    b"\n%*s[ skipping %sK ]\0" as *const u8 as *const libc::c_char,
                    5 as libc::c_int,
                ),
                2 as libc::c_int + skipped_k_len,
                b"\0" as *const u8 as *const libc::c_char,
                number_to_static_string(skipped_k),
            );
        }
        logprintf(
            LOG_PROGRESS,
            b"\n%6sK\0" as *const u8 as *const libc::c_char,
            number_to_static_string(skipped / 1024 as libc::c_int as libc::c_long),
        );
        while remainder >= dot_bytes {
            if (*dp).dots % opt.dot_spacing == 0 as libc::c_int {
                logputs(LOG_PROGRESS, b" \0" as *const u8 as *const libc::c_char);
            }
            logputs(LOG_PROGRESS, b",\0" as *const u8 as *const libc::c_char);
            (*dp).dots += 1;
            (*dp).dots;
            remainder -= dot_bytes;
        }
        (*dp).accumulated = remainder as wgint;
        (*dp).rows = skipped / ROW_BYTES;
    }
    return dp as *mut libc::c_void;
}
unsafe extern "C" fn print_row_stats(
    mut dp: *mut dot_progress,
    mut dltime: libc::c_double,
    mut last: bool,
    mut added_rows: wgint,
) {
    let ROW_BYTES: wgint = opt.dot_bytes * opt.dots_in_line as libc::c_long;
    let mut bytes_displayed: wgint = (*dp).rows * ROW_BYTES
        + (*dp).dots as libc::c_long * opt.dot_bytes;
    if last {
        bytes_displayed += (*dp).accumulated;
    }
    if bytes_displayed < 0 as libc::c_int as libc::c_long {
        bytes_displayed = 0 as libc::c_int as wgint;
    }
    if (*dp).total_length != 0 {
        let mut percentage: libc::c_int = (100.0f64 * bytes_displayed as libc::c_double
            / (*dp).total_length as libc::c_double) as libc::c_int;
        logprintf(
            LOG_PROGRESS,
            b"%3d%%\0" as *const u8 as *const libc::c_char,
            percentage,
        );
    }
    static mut names: [libc::c_char; 5] = [
        ' ' as i32 as libc::c_char,
        'K' as i32 as libc::c_char,
        'M' as i32 as libc::c_char,
        'G' as i32 as libc::c_char,
        'T' as i32 as libc::c_char,
    ];
    let mut units: libc::c_int = 0;
    let mut rate: libc::c_double = 0.;
    let mut bytes_this_row: wgint = 0;
    if !last {
        bytes_this_row = ROW_BYTES * added_rows;
    } else {
        bytes_this_row = (*dp).dots as libc::c_long * opt.dot_bytes + (*dp).accumulated;
    }
    if (*dp).rows == (*dp).initial_length / ROW_BYTES {
        bytes_this_row -= (*dp).initial_length % ROW_BYTES;
    }
    rate = calc_rate(bytes_this_row, dltime - (*dp).last_timer_value, &mut units);
    logprintf(
        LOG_PROGRESS,
        b" %4.*f%c\0" as *const u8 as *const libc::c_char,
        if rate >= 99.95f64 {
            0 as libc::c_int
        } else if rate >= 9.995f64 {
            1 as libc::c_int
        } else {
            2 as libc::c_int
        },
        rate,
        names[units as usize] as libc::c_int,
    );
    (*dp).last_timer_value = dltime;
    if !last {
        if (*dp).total_length != 0 {
            let mut bytes_remaining: wgint = if (*dp).total_length > bytes_displayed {
                (*dp).total_length - bytes_displayed
            } else {
                0 as libc::c_int as libc::c_long
            };
            let mut bytes_sofar: wgint = if bytes_displayed > (*dp).initial_length {
                bytes_displayed - (*dp).initial_length
            } else {
                1 as libc::c_int as libc::c_long
            };
            let mut eta: libc::c_double = dltime * bytes_remaining as libc::c_double
                / bytes_sofar as libc::c_double;
            if eta < 0 as libc::c_int as libc::c_double {
                eta = 0 as libc::c_int as libc::c_double;
            }
            if eta < (2147483647 as libc::c_int - 1 as libc::c_int) as libc::c_double {
                logprintf(
                    LOG_PROGRESS,
                    b" %s\0" as *const u8 as *const libc::c_char,
                    eta_to_human_short(
                        (eta + 0.5f64) as libc::c_int,
                        1 as libc::c_int != 0,
                    ),
                );
            }
        }
    } else if dltime >= 10 as libc::c_int as libc::c_double {
        logprintf(
            LOG_PROGRESS,
            b"=%s\0" as *const u8 as *const libc::c_char,
            eta_to_human_short((dltime + 0.5f64) as libc::c_int, 1 as libc::c_int != 0),
        );
    } else {
        logprintf(
            LOG_PROGRESS,
            b"=%ss\0" as *const u8 as *const libc::c_char,
            print_decimal(dltime),
        );
    };
}
unsafe extern "C" fn dot_update(
    mut progress: *mut libc::c_void,
    mut howmuch: wgint,
    mut dltime: libc::c_double,
) {
    if dltime >= 2147483647 as libc::c_int as libc::c_double {
        dltime = (2147483647 as libc::c_int - 1 as libc::c_int) as libc::c_double;
    } else if dltime < 0 as libc::c_int as libc::c_double {
        dltime = 0 as libc::c_int as libc::c_double;
    }
    if howmuch < 0 as libc::c_int as libc::c_long {
        howmuch = 0 as libc::c_int as wgint;
    }
    let mut dp: *mut dot_progress = progress as *mut dot_progress;
    (*dp).accumulated += howmuch;
    (*dp).dltime = dltime;
}
unsafe extern "C" fn dot_draw(mut progress: *mut libc::c_void) {
    let mut dp: *mut dot_progress = progress as *mut dot_progress;
    let mut dot_bytes: libc::c_int = opt.dot_bytes as libc::c_int;
    let mut ROW_BYTES: wgint = opt.dot_bytes * opt.dots_in_line as libc::c_long;
    log_set_flush(0 as libc::c_int != 0);
    while (*dp).accumulated >= dot_bytes as libc::c_long {
        (*dp).accumulated -= dot_bytes as libc::c_long;
        if (*dp).dots == 0 as libc::c_int {
            logprintf(
                LOG_PROGRESS,
                b"\n%6sK\0" as *const u8 as *const libc::c_char,
                number_to_static_string(
                    (*dp).rows * ROW_BYTES / 1024 as libc::c_int as libc::c_long,
                ),
            );
        }
        if (*dp).dots % opt.dot_spacing == 0 as libc::c_int {
            logputs(LOG_PROGRESS, b" \0" as *const u8 as *const libc::c_char);
        }
        logputs(LOG_PROGRESS, b".\0" as *const u8 as *const libc::c_char);
        (*dp).dots += 1;
        (*dp).dots;
        if (*dp).dots >= opt.dots_in_line {
            (*dp).dots = 0 as libc::c_int;
            let mut added_rows: wgint = 1 as libc::c_int as wgint;
            if (*dp).accumulated >= ROW_BYTES << 2 as libc::c_int {
                added_rows += (*dp).accumulated / ROW_BYTES;
                (*dp).accumulated %= ROW_BYTES;
            }
            if 9223372036854775807 as libc::c_long - (*dp).rows >= added_rows {
                (*dp).rows += added_rows;
            } else {
                (*dp).rows = 9223372036854775807 as libc::c_long;
            }
            print_row_stats(dp, (*dp).dltime, 0 as libc::c_int != 0, added_rows);
        }
    }
    log_set_flush(1 as libc::c_int != 0);
}
unsafe extern "C" fn dot_finish(
    mut progress: *mut libc::c_void,
    mut dltime: libc::c_double,
) {
    let mut dp: *mut dot_progress = progress as *mut dot_progress;
    let mut ROW_BYTES: wgint = opt.dot_bytes * opt.dots_in_line as libc::c_long;
    let mut i: libc::c_int = 0;
    log_set_flush(0 as libc::c_int != 0);
    if (*dp).dots == 0 as libc::c_int {
        logprintf(
            LOG_PROGRESS,
            b"\n%6sK\0" as *const u8 as *const libc::c_char,
            number_to_static_string(
                (*dp).rows * ROW_BYTES / 1024 as libc::c_int as libc::c_long,
            ),
        );
    }
    i = (*dp).dots;
    while i < opt.dots_in_line {
        if i % opt.dot_spacing == 0 as libc::c_int {
            logputs(LOG_PROGRESS, b" \0" as *const u8 as *const libc::c_char);
        }
        logputs(LOG_PROGRESS, b" \0" as *const u8 as *const libc::c_char);
        i += 1;
        i;
    }
    if dltime >= 2147483647 as libc::c_int as libc::c_double {
        dltime = (2147483647 as libc::c_int - 1 as libc::c_int) as libc::c_double;
    } else if dltime < 0 as libc::c_int as libc::c_double {
        dltime = 0 as libc::c_int as libc::c_double;
    }
    print_row_stats(dp, dltime, 1 as libc::c_int != 0, 1 as libc::c_int as wgint);
    logputs(LOG_PROGRESS, b"\n\n\0" as *const u8 as *const libc::c_char);
    log_set_flush(0 as libc::c_int != 0);
    rpl_free(dp as *mut libc::c_void);
    dp = 0 as *mut dot_progress;
}
unsafe extern "C" fn dot_set_params(mut params: *const libc::c_char) {
    (*current_impl).interactive = 0 as libc::c_int != 0;
    if params.is_null() || *params == 0 {
        params = opt.dot_style;
    }
    if params.is_null() {
        return;
    }
    if c_strcasecmp(params, b"default\0" as *const u8 as *const libc::c_char) == 0 {
        opt.dot_bytes = 1024 as libc::c_int as wgint;
        opt.dot_spacing = 10 as libc::c_int;
        opt.dots_in_line = 50 as libc::c_int;
    } else if c_strcasecmp(params, b"binary\0" as *const u8 as *const libc::c_char) == 0
    {
        opt.dot_bytes = 8192 as libc::c_int as wgint;
        opt.dot_spacing = 16 as libc::c_int;
        opt.dots_in_line = 48 as libc::c_int;
    } else if c_strcasecmp(params, b"mega\0" as *const u8 as *const libc::c_char) == 0 {
        opt.dot_bytes = 65536 as libc::c_long;
        opt.dot_spacing = 8 as libc::c_int;
        opt.dots_in_line = 48 as libc::c_int;
    } else if c_strcasecmp(params, b"giga\0" as *const u8 as *const libc::c_char) == 0 {
        opt.dot_bytes = (1 as libc::c_long) << 20 as libc::c_int;
        opt.dot_spacing = 8 as libc::c_int;
        opt.dots_in_line = 32 as libc::c_int;
    } else {
        fprintf(
            stderr,
            dcgettext(
                0 as *const libc::c_char,
                b"Invalid dot style specification %s; leaving unchanged.\n\0"
                    as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ),
            quote(params),
        );
    };
}
static mut screen_width: libc::c_int = 0;
static mut received_sigwinch: sig_atomic_t = 0;
unsafe extern "C" fn prepare_filename(
    mut dest: *mut libc::c_char,
    mut src: *const libc::c_char,
) -> size_t {
    let mut ret: size_t = 1 as libc::c_int as size_t;
    if !src.is_null() {
        let mut iter: mbi_iterator_t = mbi_iterator_t {
            limit: 0 as *const libc::c_char,
            in_shift: false,
            state: mbstate_t {
                __count: 0,
                __value: C2RustUnnamed_3 { __wch: 0 },
            },
            next_done: false,
            cur: mbchar {
                ptr: 0 as *const libc::c_char,
                bytes: 0,
                wc_valid: false,
                wc: 0,
                buf: [0; 24],
            },
        };
        let mut mbc: mbchar_t = mbchar {
            ptr: 0 as *const libc::c_char,
            bytes: 0,
            wc_valid: false,
            wc: 0,
            buf: [0; 24],
        };
        iter.cur.ptr = src;
        iter.limit = (iter.cur.ptr).offset(strlen(src) as isize);
        iter.in_shift = 0 as libc::c_int != 0;
        memset(
            &mut iter.state as *mut mbstate_t as *mut libc::c_void,
            '\0' as i32,
            ::std::mem::size_of::<mbstate_t>() as libc::c_ulong,
        );
        iter.next_done = 0 as libc::c_int != 0;
        while iter.cur.ptr < iter.limit
            && {
                mbiter_multi_next(&mut iter);
                1 as libc::c_int != 0
            }
        {
            let mut i: size_t = 0;
            mbc = iter.cur;
            if !(mbc.wc_valid as libc::c_int != 0 && iswprint(mbc.wc as wint_t) != 0)
                || (if mbc.wc_valid as libc::c_int != 0 {
                    mb_width_aux(mbc.wc as wint_t)
                } else {
                    1 as libc::c_int
                }) == 0
            {
                i = 0 as libc::c_int as size_t;
                while i < mbc.bytes {
                    if !dest.is_null() {
                        dest = dest
                            .offset(
                                sprintf(
                                    dest,
                                    b"%%%02x\0" as *const u8 as *const libc::c_char,
                                    *(mbc.ptr).offset(i as isize) as libc::c_uchar
                                        as libc::c_int,
                                ) as isize,
                            );
                    }
                    ret = (ret as libc::c_ulong)
                        .wrapping_add(3 as libc::c_int as libc::c_ulong) as size_t
                        as size_t;
                    i = i.wrapping_add(1);
                    i;
                }
            } else {
                if !dest.is_null() {
                    i = 0 as libc::c_int as size_t;
                    while i < mbc.bytes {
                        let fresh0 = dest;
                        dest = dest.offset(1);
                        *fresh0 = *(mbc.ptr).offset(i as isize);
                        i = i.wrapping_add(1);
                        i;
                    }
                }
                ret = (ret as libc::c_ulong).wrapping_add(mbc.bytes) as size_t as size_t;
            }
            iter.cur.ptr = (iter.cur.ptr).offset(iter.cur.bytes as isize);
            iter.next_done = 0 as libc::c_int != 0;
        }
    }
    if !dest.is_null() {
        *dest = 0 as libc::c_int as libc::c_char;
    }
    return ret;
}
unsafe extern "C" fn bar_create(
    mut f_download: *const libc::c_char,
    mut initial: wgint,
    mut total: wgint,
) -> *mut libc::c_void {
    let mut bp: *mut bar_progress = xcalloc(
        1 as libc::c_int as size_t,
        ::std::mem::size_of::<bar_progress>() as libc::c_ulong,
    ) as *mut bar_progress;
    if initial > total {
        total = initial;
    }
    (*bp).initial_length = initial;
    (*bp).total_length = total;
    (*bp)
        .f_download = xmalloc(prepare_filename(0 as *mut libc::c_char, f_download))
        as *mut libc::c_char;
    prepare_filename((*bp).f_download, f_download);
    if screen_width == 0 || received_sigwinch != 0 {
        screen_width = determine_screen_width();
        if screen_width == 0 {
            screen_width = 80 as libc::c_int;
        } else if screen_width < 51 as libc::c_int {
            screen_width = 51 as libc::c_int;
        }
        ::std::ptr::write_volatile(
            &mut received_sigwinch as *mut sig_atomic_t,
            0 as libc::c_int,
        );
    }
    (*bp).width = screen_width - 1 as libc::c_int;
    (*bp)
        .buffer = xcalloc(
        ((*bp).width * 2 as libc::c_int + 100 as libc::c_int) as size_t,
        1 as libc::c_int as size_t,
    ) as *mut libc::c_char;
    logputs(LOG_VERBOSE, b"\n\0" as *const u8 as *const libc::c_char);
    create_image(bp, 0 as libc::c_int as libc::c_double, 0 as libc::c_int != 0);
    display_image((*bp).buffer);
    return bp as *mut libc::c_void;
}
unsafe extern "C" fn bar_update(
    mut progress: *mut libc::c_void,
    mut howmuch: wgint,
    mut dltime: libc::c_double,
) {
    let mut bp: *mut bar_progress = progress as *mut bar_progress;
    (*bp).dltime = dltime;
    if 9223372036854775807 as libc::c_long - ((*bp).count + (*bp).initial_length)
        >= howmuch
    {
        (*bp).count += howmuch;
    } else {
        (*bp).count = 9223372036854775807 as libc::c_long - (*bp).initial_length;
    }
    if (*bp).total_length > 0 as libc::c_int as libc::c_long
        && (*bp).count + (*bp).initial_length > (*bp).total_length
    {
        (*bp).total_length = (*bp).initial_length + (*bp).count;
    }
    update_speed_ring(bp, howmuch, dltime);
}
unsafe extern "C" fn bar_draw(mut progress: *mut libc::c_void) {
    let mut force_screen_update: bool = 0 as libc::c_int != 0;
    let mut bp: *mut bar_progress = progress as *mut bar_progress;
    if received_sigwinch != 0 {
        let mut old_width: libc::c_int = screen_width;
        screen_width = determine_screen_width();
        if screen_width == 0 {
            screen_width = 80 as libc::c_int;
        } else if screen_width < 51 as libc::c_int {
            screen_width = 51 as libc::c_int;
        }
        if screen_width != old_width {
            (*bp).width = screen_width - 1 as libc::c_int;
            (*bp)
                .buffer = xrealloc(
                (*bp).buffer as *mut libc::c_void,
                ((*bp).width * 2 as libc::c_int + 100 as libc::c_int) as size_t,
            ) as *mut libc::c_char;
            force_screen_update = 1 as libc::c_int != 0;
        }
        ::std::ptr::write_volatile(
            &mut received_sigwinch as *mut sig_atomic_t,
            0 as libc::c_int,
        );
    }
    if (*bp).dltime - (*bp).last_screen_update < 0.2f64 && !force_screen_update {
        return;
    }
    create_image(bp, (*bp).dltime, 0 as libc::c_int != 0);
    display_image((*bp).buffer);
    (*bp).last_screen_update = (*bp).dltime;
}
unsafe extern "C" fn bar_finish(
    mut progress: *mut libc::c_void,
    mut dltime: libc::c_double,
) {
    let mut bp: *mut bar_progress = progress as *mut bar_progress;
    if (*bp).total_length > 0 as libc::c_int as libc::c_long
        && (*bp).count + (*bp).initial_length > (*bp).total_length
    {
        (*bp).total_length = (*bp).initial_length + (*bp).count;
    }
    create_image(bp, dltime, 1 as libc::c_int != 0);
    display_image((*bp).buffer);
    logputs(LOG_VERBOSE, b"\n\0" as *const u8 as *const libc::c_char);
    logputs(LOG_PROGRESS, b"\n\0" as *const u8 as *const libc::c_char);
    rpl_free((*bp).f_download as *mut libc::c_void);
    (*bp).f_download = 0 as *mut libc::c_char;
    rpl_free((*bp).buffer as *mut libc::c_void);
    (*bp).buffer = 0 as *mut libc::c_char;
    rpl_free(bp as *mut libc::c_void);
    bp = 0 as *mut bar_progress;
}
unsafe extern "C" fn update_speed_ring(
    mut bp: *mut bar_progress,
    mut howmuch: wgint,
    mut dltime: libc::c_double,
) {
    let mut hist: *mut bar_progress_hist = &mut (*bp).hist;
    let mut recent_age: libc::c_double = dltime - (*bp).recent_start;
    (*bp).recent_bytes += howmuch;
    if recent_age < 0.15f64 {
        return;
    }
    if howmuch == 0 as libc::c_int as libc::c_long {
        if recent_age >= 5 as libc::c_int as libc::c_double {
            (*bp).stalled = 1 as libc::c_int != 0;
            memset(
                hist as *mut libc::c_void,
                '\0' as i32,
                ::std::mem::size_of::<bar_progress_hist>() as libc::c_ulong,
            );
            (*bp).recent_bytes = 0 as libc::c_int as wgint;
        }
        return;
    }
    if (*bp).stalled {
        (*bp).stalled = 0 as libc::c_int != 0;
        recent_age = 1 as libc::c_int as libc::c_double;
    }
    (*hist).total_time -= (*hist).times[(*hist).pos as usize];
    (*hist).total_bytes -= (*hist).bytes[(*hist).pos as usize];
    (*hist).times[(*hist).pos as usize] = recent_age;
    (*hist).bytes[(*hist).pos as usize] = (*bp).recent_bytes;
    (*hist).total_time += recent_age;
    (*hist).total_bytes += (*bp).recent_bytes;
    (*bp).recent_start = dltime;
    (*bp).recent_bytes = 0 as libc::c_int as wgint;
    (*hist).pos += 1;
    if (*hist).pos == 20 as libc::c_int {
        (*hist).pos = 0 as libc::c_int;
    }
}
unsafe extern "C" fn count_cols(mut mbs: *const libc::c_char) -> libc::c_int {
    let mut mbc: mbchar_t = mbchar {
        ptr: 0 as *const libc::c_char,
        bytes: 0,
        wc_valid: false,
        wc: 0,
        buf: [0; 24],
    };
    let mut iter: mbi_iterator_t = mbi_iterator_t {
        limit: 0 as *const libc::c_char,
        in_shift: false,
        state: mbstate_t {
            __count: 0,
            __value: C2RustUnnamed_3 { __wch: 0 },
        },
        next_done: false,
        cur: mbchar {
            ptr: 0 as *const libc::c_char,
            bytes: 0,
            wc_valid: false,
            wc: 0,
            buf: [0; 24],
        },
    };
    let mut cols: libc::c_int = 0 as libc::c_int;
    iter.cur.ptr = mbs;
    iter.limit = (iter.cur.ptr).offset(strlen(mbs) as isize);
    iter.in_shift = 0 as libc::c_int != 0;
    memset(
        &mut iter.state as *mut mbstate_t as *mut libc::c_void,
        '\0' as i32,
        ::std::mem::size_of::<mbstate_t>() as libc::c_ulong,
    );
    iter.next_done = 0 as libc::c_int != 0;
    while iter.cur.ptr < iter.limit
        && {
            mbiter_multi_next(&mut iter);
            1 as libc::c_int != 0
        }
    {
        mbc = iter.cur;
        cols
            += if mbc.wc_valid as libc::c_int != 0 {
                mb_width_aux(mbc.wc as wint_t)
            } else {
                1 as libc::c_int
            };
        iter.cur.ptr = (iter.cur.ptr).offset(iter.cur.bytes as isize);
        iter.next_done = 0 as libc::c_int != 0;
    }
    return cols;
}
unsafe extern "C" fn cols_to_bytes(
    mut mbs: *const libc::c_char,
    cols: libc::c_int,
    mut ncols: *mut libc::c_int,
) -> libc::c_int {
    let mut p_cols: libc::c_int = 0 as libc::c_int;
    let mut bytes: libc::c_int = 0 as libc::c_int;
    let mut mbc: mbchar_t = mbchar {
        ptr: 0 as *const libc::c_char,
        bytes: 0,
        wc_valid: false,
        wc: 0,
        buf: [0; 24],
    };
    let mut iter: mbi_iterator_t = mbi_iterator_t {
        limit: 0 as *const libc::c_char,
        in_shift: false,
        state: mbstate_t {
            __count: 0,
            __value: C2RustUnnamed_3 { __wch: 0 },
        },
        next_done: false,
        cur: mbchar {
            ptr: 0 as *const libc::c_char,
            bytes: 0,
            wc_valid: false,
            wc: 0,
            buf: [0; 24],
        },
    };
    iter.cur.ptr = mbs;
    iter.limit = (iter.cur.ptr).offset(strlen(mbs) as isize);
    iter.in_shift = 0 as libc::c_int != 0;
    memset(
        &mut iter.state as *mut mbstate_t as *mut libc::c_void,
        '\0' as i32,
        ::std::mem::size_of::<mbstate_t>() as libc::c_ulong,
    );
    iter.next_done = 0 as libc::c_int != 0;
    while p_cols < cols
        && (iter.cur.ptr < iter.limit
            && {
                mbiter_multi_next(&mut iter);
                1 as libc::c_int != 0
            })
    {
        mbc = iter.cur;
        p_cols
            += if mbc.wc_valid as libc::c_int != 0 {
                mb_width_aux(mbc.wc as wint_t)
            } else {
                1 as libc::c_int
            };
        if p_cols > cols {
            p_cols
                -= if mbc.wc_valid as libc::c_int != 0 {
                    mb_width_aux(mbc.wc as wint_t)
                } else {
                    1 as libc::c_int
                };
            break;
        } else {
            bytes = (bytes as libc::c_ulong).wrapping_add(mbc.bytes) as libc::c_int
                as libc::c_int;
            iter.cur.ptr = (iter.cur.ptr).offset(iter.cur.bytes as isize);
            iter.next_done = 0 as libc::c_int != 0;
        }
    }
    *ncols = p_cols;
    return bytes;
}
unsafe extern "C" fn get_eta(mut bcd: *mut libc::c_int) -> *const libc::c_char {
    static mut eta_str: [libc::c_char; 11] = unsafe {
        *::std::mem::transmute::<&[u8; 11], &[libc::c_char; 11]>(b"    eta %s\0")
    };
    static mut eta_trans: *const libc::c_char = 0 as *const libc::c_char;
    static mut bytes_cols_diff: libc::c_int = 0;
    if eta_trans.is_null() {
        let mut nbytes: libc::c_int = 0;
        let mut ncols: libc::c_int = 0;
        eta_trans = dcgettext(
            0 as *const libc::c_char,
            eta_str.as_ptr(),
            5 as libc::c_int,
        );
        nbytes = strlen(eta_trans) as libc::c_int;
        ncols = count_cols(eta_trans);
        bytes_cols_diff = nbytes - ncols;
    }
    if !bcd.is_null() {
        *bcd = bytes_cols_diff;
    }
    return eta_trans;
}
unsafe extern "C" fn create_image(
    mut bp: *mut bar_progress,
    mut dl_total_time: libc::c_double,
    mut done: bool,
) {
    let MAX_FILENAME_COLS: libc::c_int = (*bp).width / 4 as libc::c_int;
    let mut p: *mut libc::c_char = (*bp).buffer;
    let mut size: wgint = (*bp).initial_length + (*bp).count;
    let mut hist: *mut bar_progress_hist = &mut (*bp).hist;
    let mut orig_filename_cols: libc::c_int = count_cols((*bp).f_download);
    let mut padding: libc::c_int = 0;
    let mut progress_size: libc::c_int = (*bp).width
        - (MAX_FILENAME_COLS + 1 as libc::c_int + 4 as libc::c_int + 2 as libc::c_int
            + 7 as libc::c_int + 1 as libc::c_int + 8 as libc::c_int + 2 as libc::c_int
            + 15 as libc::c_int);
    let mut bytes_cols_diff: libc::c_int = 0 as libc::c_int;
    let mut cols_diff: libc::c_int = 0;
    let mut down_size: *const libc::c_char = 0 as *const libc::c_char;
    if progress_size < 5 as libc::c_int {
        progress_size = 0 as libc::c_int;
    }
    if dl_total_time >= 2147483647 as libc::c_int as libc::c_double {
        dl_total_time = (2147483647 as libc::c_int - 1 as libc::c_int) as libc::c_double;
    } else if dl_total_time < 0 as libc::c_int as libc::c_double {
        dl_total_time = 0 as libc::c_int as libc::c_double;
    }
    if orig_filename_cols < MAX_FILENAME_COLS {
        p = p
            .offset(
                sprintf(p, b"%s\0" as *const u8 as *const libc::c_char, (*bp).f_download)
                    as isize,
            );
        padding = MAX_FILENAME_COLS - orig_filename_cols + 1 as libc::c_int;
        memset(p as *mut libc::c_void, ' ' as i32, padding as libc::c_ulong);
        p = p.offset(padding as isize);
    } else {
        let mut offset_cols: libc::c_int = 0;
        let mut bytes_in_filename: libc::c_int = 0;
        let mut offset_bytes: libc::c_int = 0;
        let mut col: libc::c_int = 0;
        let mut cols_ret: *mut libc::c_int = &mut col;
        if orig_filename_cols > MAX_FILENAME_COLS + 5 as libc::c_int && !opt.noscroll
            && !done
        {
            offset_cols = ((*bp).tick + orig_filename_cols
                + MAX_FILENAME_COLS / 2 as libc::c_int)
                % (orig_filename_cols + MAX_FILENAME_COLS);
            if offset_cols > orig_filename_cols {
                padding = MAX_FILENAME_COLS - (offset_cols - orig_filename_cols);
                memset(p as *mut libc::c_void, ' ' as i32, padding as libc::c_ulong);
                p = p.offset(padding as isize);
                offset_cols = 0 as libc::c_int;
            } else {
                padding = 0 as libc::c_int;
            }
        } else {
            padding = 0 as libc::c_int;
            offset_cols = 0 as libc::c_int;
        }
        offset_bytes = cols_to_bytes((*bp).f_download, offset_cols, cols_ret);
        bytes_in_filename = cols_to_bytes(
            ((*bp).f_download).offset(offset_bytes as isize),
            MAX_FILENAME_COLS - padding,
            cols_ret,
        );
        memcpy(
            p as *mut libc::c_void,
            ((*bp).f_download).offset(offset_bytes as isize) as *const libc::c_void,
            bytes_in_filename as libc::c_ulong,
        );
        p = p.offset(bytes_in_filename as isize);
        padding = MAX_FILENAME_COLS - (padding + *cols_ret);
        memset(
            p as *mut libc::c_void,
            ' ' as i32,
            (padding + 1 as libc::c_int) as libc::c_ulong,
        );
        p = p.offset((padding + 1 as libc::c_int) as isize);
    }
    if (*bp).total_length > 0 as libc::c_int as libc::c_long {
        let mut percentage: libc::c_int = (100.0f64 * size as libc::c_double
            / (*bp).total_length as libc::c_double) as libc::c_int;
        p = p
            .offset(
                sprintf(p, b"%3d%%\0" as *const u8 as *const libc::c_char, percentage)
                    as isize,
            );
    } else {
        memset(p as *mut libc::c_void, ' ' as i32, 4 as libc::c_int as libc::c_ulong);
        p = p.offset(4 as libc::c_int as isize);
    }
    if progress_size != 0 && (*bp).total_length > 0 as libc::c_int as libc::c_long {
        let mut insz: libc::c_int = ((*bp).initial_length as libc::c_double
            / (*bp).total_length as libc::c_double * progress_size as libc::c_double)
            as libc::c_int;
        let mut dlsz: libc::c_int = (size as libc::c_double
            / (*bp).total_length as libc::c_double * progress_size as libc::c_double)
            as libc::c_int;
        let mut begin: *mut libc::c_char = 0 as *mut libc::c_char;
        let fresh1 = p;
        p = p.offset(1);
        *fresh1 = '[' as i32 as libc::c_char;
        begin = p;
        memset(p as *mut libc::c_void, '+' as i32, insz as libc::c_ulong);
        p = p.offset(insz as isize);
        dlsz -= insz;
        if dlsz > 0 as libc::c_int {
            memset(
                p as *mut libc::c_void,
                '=' as i32,
                (dlsz - 1 as libc::c_int) as libc::c_ulong,
            );
            p = p.offset((dlsz - 1 as libc::c_int) as isize);
            let fresh2 = p;
            p = p.offset(1);
            *fresh2 = '>' as i32 as libc::c_char;
        }
        memset(
            p as *mut libc::c_void,
            ' ' as i32,
            (progress_size as libc::c_long - p.offset_from(begin) as libc::c_long)
                as libc::c_ulong,
        );
        p = p
            .offset(
                (progress_size as libc::c_long - p.offset_from(begin) as libc::c_long)
                    as isize,
            );
        let fresh3 = p;
        p = p.offset(1);
        *fresh3 = ']' as i32 as libc::c_char;
    } else if progress_size != 0 {
        let mut ind: libc::c_int = (*bp).tick
            % (progress_size * 2 as libc::c_int - 6 as libc::c_int);
        let mut i: libc::c_int = 0;
        let mut pos: libc::c_int = 0;
        if ind < progress_size - 2 as libc::c_int {
            pos = ind + 1 as libc::c_int;
        } else {
            pos = progress_size - (ind - progress_size + 5 as libc::c_int);
        }
        let fresh4 = p;
        p = p.offset(1);
        *fresh4 = '[' as i32 as libc::c_char;
        i = 0 as libc::c_int;
        while i < progress_size {
            if i == pos - 1 as libc::c_int {
                let fresh5 = p;
                p = p.offset(1);
                *fresh5 = '<' as i32 as libc::c_char;
            } else if i == pos {
                let fresh6 = p;
                p = p.offset(1);
                *fresh6 = '=' as i32 as libc::c_char;
            } else if i == pos + 1 as libc::c_int {
                let fresh7 = p;
                p = p.offset(1);
                *fresh7 = '>' as i32 as libc::c_char;
            } else {
                let fresh8 = p;
                p = p.offset(1);
                *fresh8 = ' ' as i32 as libc::c_char;
            }
            i += 1;
            i;
        }
        let fresh9 = p;
        p = p.offset(1);
        *fresh9 = ']' as i32 as libc::c_char;
    }
    (*bp).tick += 1;
    (*bp).tick;
    down_size = human_readable(size, 1000 as libc::c_int, 2 as libc::c_int);
    cols_diff = 7 as libc::c_int + 1 as libc::c_int - count_cols(down_size);
    if cols_diff > 0 as libc::c_int {
        memset(p as *mut libc::c_void, ' ' as i32, cols_diff as libc::c_ulong);
        p = p.offset(cols_diff as isize);
    }
    p = p
        .offset(
            sprintf(p, b"%s\0" as *const u8 as *const libc::c_char, down_size) as isize,
        );
    if (*hist).total_time > 0 as libc::c_int as libc::c_double
        && (*hist).total_bytes != 0
    {
        static mut short_units: [*const libc::c_char; 5] = [
            b" B/s\0" as *const u8 as *const libc::c_char,
            b"KB/s\0" as *const u8 as *const libc::c_char,
            b"MB/s\0" as *const u8 as *const libc::c_char,
            b"GB/s\0" as *const u8 as *const libc::c_char,
            b"TB/s\0" as *const u8 as *const libc::c_char,
        ];
        static mut short_units_bits: [*const libc::c_char; 5] = [
            b" b/s\0" as *const u8 as *const libc::c_char,
            b"Kb/s\0" as *const u8 as *const libc::c_char,
            b"Mb/s\0" as *const u8 as *const libc::c_char,
            b"Gb/s\0" as *const u8 as *const libc::c_char,
            b"Tb/s\0" as *const u8 as *const libc::c_char,
        ];
        let mut units: libc::c_int = 0 as libc::c_int;
        let mut dlquant: wgint = (*hist).total_bytes + (*bp).recent_bytes;
        let mut dltime: libc::c_double = (*hist).total_time
            + (dl_total_time - (*bp).recent_start);
        let mut dlspeed: libc::c_double = calc_rate(dlquant, dltime, &mut units);
        p = p
            .offset(
                sprintf(
                    p,
                    b"  %4.*f%s\0" as *const u8 as *const libc::c_char,
                    if dlspeed >= 99.95f64 {
                        0 as libc::c_int
                    } else if dlspeed >= 9.995f64 {
                        1 as libc::c_int
                    } else {
                        2 as libc::c_int
                    },
                    dlspeed,
                    if !opt.report_bps {
                        short_units[units as usize]
                    } else {
                        short_units_bits[units as usize]
                    },
                ) as isize,
            );
    } else {
        memcpy(
            p as *mut libc::c_void,
            b"  --.-KB/s\0" as *const u8 as *const libc::c_char as *const libc::c_void,
            (::std::mem::size_of::<[libc::c_char; 11]>() as libc::c_ulong)
                .wrapping_sub(1 as libc::c_int as libc::c_ulong),
        );
        p = p
            .offset(
                (::std::mem::size_of::<[libc::c_char; 11]>() as libc::c_ulong)
                    .wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize,
            );
    }
    if !done {
        let mut current_block_95: u64;
        if (*bp).total_length > 0 as libc::c_int as libc::c_long
            && (*bp).count > 0 as libc::c_int as libc::c_long
            && dl_total_time > 3 as libc::c_int as libc::c_double
        {
            let mut eta: libc::c_int = 0;
            if (*bp).total_length != size && (*bp).last_eta_value != 0 as libc::c_int
                && dl_total_time - (*bp).last_eta_time < 0.99f64
            {
                eta = (*bp).last_eta_value;
                current_block_95 = 9879896046554623444;
            } else {
                let mut bytes_remaining: wgint = (*bp).total_length - size;
                let mut eta_: libc::c_double = dl_total_time
                    * bytes_remaining as libc::c_double / (*bp).count as libc::c_double;
                if eta_
                    >= (2147483647 as libc::c_int - 1 as libc::c_int) as libc::c_double
                {
                    current_block_95 = 13520255272251940982;
                } else {
                    eta = (eta_ + 0.5f64) as libc::c_int;
                    (*bp).last_eta_value = eta;
                    (*bp).last_eta_time = dl_total_time;
                    current_block_95 = 9879896046554623444;
                }
            }
            match current_block_95 {
                13520255272251940982 => {}
                _ => {
                    p = p
                        .offset(
                            sprintf(
                                p,
                                get_eta(&mut bytes_cols_diff),
                                eta_to_human_short(eta, 0 as libc::c_int != 0),
                            ) as isize,
                        );
                    current_block_95 = 1209030638129645089;
                }
            }
        } else if (*bp).total_length > 0 as libc::c_int as libc::c_long {
            current_block_95 = 13520255272251940982;
        } else {
            current_block_95 = 1209030638129645089;
        }
        match current_block_95 {
            13520255272251940982 => {
                memset(
                    p as *mut libc::c_void,
                    ' ' as i32,
                    15 as libc::c_int as libc::c_ulong,
                );
                p = p.offset(15 as libc::c_int as isize);
            }
            _ => {}
        }
    } else {
        let mut nbytes: libc::c_int = 0;
        let mut ncols: libc::c_int = 0;
        strcpy(
            p,
            dcgettext(
                0 as *const libc::c_char,
                b"    in \0" as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ),
        );
        nbytes = strlen(p) as libc::c_int;
        ncols = count_cols(p);
        bytes_cols_diff = nbytes - ncols;
        if dl_total_time >= 10 as libc::c_int as libc::c_double {
            ncols
                += sprintf(
                    p.offset(nbytes as isize),
                    b"%s\0" as *const u8 as *const libc::c_char,
                    eta_to_human_short(
                        (dl_total_time + 0.5f64) as libc::c_int,
                        0 as libc::c_int != 0,
                    ),
                );
        } else {
            ncols
                += sprintf(
                    p.offset(nbytes as isize),
                    b"%ss\0" as *const u8 as *const libc::c_char,
                    print_decimal(dl_total_time),
                );
        }
        p = p.offset((ncols + bytes_cols_diff) as isize);
        if ncols < 15 as libc::c_int {
            memset(
                p as *mut libc::c_void,
                ' ' as i32,
                (15 as libc::c_int - ncols) as libc::c_ulong,
            );
            p = p.offset((15 as libc::c_int - ncols) as isize);
        }
    }
    *p = '\0' as i32 as libc::c_char;
    padding = (*bp).width - count_cols((*bp).buffer);
    if padding > 0 as libc::c_int {
        memset(p as *mut libc::c_void, ' ' as i32, padding as libc::c_ulong);
        p = p.offset(padding as isize);
        *p = '\0' as i32 as libc::c_char;
    }
}
unsafe extern "C" fn display_image(mut buf: *mut libc::c_char) {
    let mut old: bool = log_set_save_context(0 as libc::c_int != 0);
    logputs(LOG_PROGRESS, b"\r\0" as *const u8 as *const libc::c_char);
    logputs(LOG_PROGRESS, buf);
    log_set_save_context(old);
}
unsafe extern "C" fn bar_set_params(mut params: *const libc::c_char) {
    (*current_impl).interactive = 1 as libc::c_int != 0;
    if !params.is_null() {
        let mut param: *const libc::c_char = params;
        while *param != 0 {
            if strncmp(
                param,
                b"force\0" as *const u8 as *const libc::c_char,
                5 as libc::c_int as libc::c_ulong,
            ) == 0
            {
                current_impl_locked = 1 as libc::c_int;
            } else if strncmp(
                param,
                b"noscroll\0" as *const u8 as *const libc::c_char,
                8 as libc::c_int as libc::c_ulong,
            ) == 0
            {
                opt.noscroll = 1 as libc::c_int != 0;
            }
            param = strchrnul(param, ':' as i32);
            if *param != 0 {
                param = param.offset(1);
                param;
            }
        }
    }
    if (!(opt.lfilename).is_null() && opt.show_progress != 1 as libc::c_int
        || isatty(fileno(stderr)) == 0) && current_impl_locked == 0
    {
        set_progress_implementation(b"dot\0" as *const u8 as *const libc::c_char);
        return;
    }
}
pub unsafe extern "C" fn progress_handle_sigwinch(mut sig: libc::c_int) {
    ::std::ptr::write_volatile(
        &mut received_sigwinch as *mut sig_atomic_t,
        1 as libc::c_int,
    );
    signal(
        28 as libc::c_int,
        Some(progress_handle_sigwinch as unsafe extern "C" fn(libc::c_int) -> ()),
    );
}
unsafe extern "C" fn eta_to_human_short(
    mut secs: libc::c_int,
    mut condensed: bool,
) -> *const libc::c_char {
    static mut buf: [libc::c_char; 16] = [0; 16];
    static mut last: libc::c_int = -(1 as libc::c_int);
    let mut space: *const libc::c_char = if condensed as libc::c_int != 0 {
        b"\0" as *const u8 as *const libc::c_char
    } else {
        b" \0" as *const u8 as *const libc::c_char
    };
    if secs == last {
        return buf.as_mut_ptr();
    }
    last = secs;
    if secs < 100 as libc::c_int {
        sprintf(buf.as_mut_ptr(), b"%ds\0" as *const u8 as *const libc::c_char, secs);
    } else if secs < 100 as libc::c_int * 60 as libc::c_int {
        sprintf(
            buf.as_mut_ptr(),
            b"%dm%s%ds\0" as *const u8 as *const libc::c_char,
            secs / 60 as libc::c_int,
            space,
            secs % 60 as libc::c_int,
        );
    } else if secs < 48 as libc::c_int * 3600 as libc::c_int {
        sprintf(
            buf.as_mut_ptr(),
            b"%dh%s%dm\0" as *const u8 as *const libc::c_char,
            secs / 3600 as libc::c_int,
            space,
            secs / 60 as libc::c_int % 60 as libc::c_int,
        );
    } else if secs < 100 as libc::c_int * 86400 as libc::c_int {
        sprintf(
            buf.as_mut_ptr(),
            b"%dd%s%dh\0" as *const u8 as *const libc::c_char,
            secs / 86400 as libc::c_int,
            space,
            secs / 3600 as libc::c_int % 24 as libc::c_int,
        );
    } else {
        sprintf(
            buf.as_mut_ptr(),
            b"%dd\0" as *const u8 as *const libc::c_char,
            secs / 86400 as libc::c_int,
        );
    }
    return buf.as_mut_ptr();
}
