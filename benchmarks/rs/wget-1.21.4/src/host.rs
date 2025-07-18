use ::libc;
extern "C" {
    pub type hash_table;
    fn abort() -> !;
    fn idn_decode(host: *const libc::c_char) -> *mut libc::c_char;
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
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    fn strerror(_: libc::c_int) -> *mut libc::c_char;
    fn rpl_free(_: *mut libc::c_void);
    static mut opt: options;
    fn xmalloc(s: size_t) -> *mut libc::c_void;
    fn xcalloc(n: size_t, s: size_t) -> *mut libc::c_void;
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
    fn inet_ntop(
        __af: libc::c_int,
        __cp: *const libc::c_void,
        __buf: *mut libc::c_char,
        __len: socklen_t,
    ) -> *const libc::c_char;
    fn getaddrinfo(
        __name: *const libc::c_char,
        __service: *const libc::c_char,
        __req: *const addrinfo,
        __pai: *mut *mut addrinfo,
    ) -> libc::c_int;
    fn freeaddrinfo(__ai: *mut addrinfo);
    fn gai_strerror(__ecode: libc::c_int) -> *const libc::c_char;
    fn __errno_location() -> *mut libc::c_int;
    fn xstrdup_lower(_: *const libc::c_char) -> *mut libc::c_char;
    fn aprintf(_: *const libc::c_char, _: ...) -> *mut libc::c_char;
    fn run_with_timeout(
        _: libc::c_double,
        _: Option::<unsafe extern "C" fn(*mut libc::c_void) -> ()>,
        _: *mut libc::c_void,
    ) -> bool;
    fn stable_sort(
        _: *mut libc::c_void,
        _: size_t,
        _: size_t,
        _: Option::<
            unsafe extern "C" fn(*const libc::c_void, *const libc::c_void) -> libc::c_int,
        >,
    );
    fn hash_table_get(_: *const hash_table, _: *const libc::c_void) -> *mut libc::c_void;
    fn hash_table_put(
        _: *mut hash_table,
        _: *const libc::c_void,
        _: *const libc::c_void,
    );
    fn hash_table_remove(_: *mut hash_table, _: *const libc::c_void) -> libc::c_int;
    fn make_nocase_string_hash_table(_: libc::c_int) -> *mut hash_table;
}
pub type __uint8_t = libc::c_uchar;
pub type __uint16_t = libc::c_ushort;
pub type __uint32_t = libc::c_uint;
pub type __int64_t = libc::c_long;
pub type __socklen_t = libc::c_uint;
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
pub type socklen_t = __socklen_t;
pub type __socket_type = libc::c_uint;
pub const SOCK_NONBLOCK: __socket_type = 2048;
pub const SOCK_CLOEXEC: __socket_type = 524288;
pub const SOCK_PACKET: __socket_type = 10;
pub const SOCK_DCCP: __socket_type = 6;
pub const SOCK_SEQPACKET: __socket_type = 5;
pub const SOCK_RDM: __socket_type = 4;
pub const SOCK_RAW: __socket_type = 3;
pub const SOCK_DGRAM: __socket_type = 2;
pub const SOCK_STREAM: __socket_type = 1;
pub type sa_family_t = libc::c_ushort;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sockaddr {
    pub sa_family: sa_family_t,
    pub sa_data: [libc::c_char; 14],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sockaddr_in6 {
    pub sin6_family: sa_family_t,
    pub sin6_port: in_port_t,
    pub sin6_flowinfo: uint32_t,
    pub sin6_addr: in6_addr,
    pub sin6_scope_id: uint32_t,
}
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
pub type in_port_t = uint16_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sockaddr_in {
    pub sin_family: sa_family_t,
    pub sin_port: in_port_t,
    pub sin_addr: in_addr,
    pub sin_zero: [libc::c_uchar; 8],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct in_addr {
    pub s_addr: in_addr_t,
}
pub type in_addr_t = uint32_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct addrinfo {
    pub ai_flags: libc::c_int,
    pub ai_family: libc::c_int,
    pub ai_socktype: libc::c_int,
    pub ai_protocol: libc::c_int,
    pub ai_addrlen: socklen_t,
    pub ai_addr: *mut sockaddr,
    pub ai_canonname: *mut libc::c_char,
    pub ai_next: *mut addrinfo,
}
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct address_list {
    pub count: libc::c_int,
    pub addresses: *mut ip_address,
    pub faulty: libc::c_int,
    pub connected: bool,
    pub refcount: libc::c_int,
}
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
pub const LH_REFRESH: C2RustUnnamed_5 = 4;
pub const LH_BIND: C2RustUnnamed_5 = 2;
pub const LH_SILENT: C2RustUnnamed_5 = 1;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct gaiwt_context {
    pub node: *const libc::c_char,
    pub service: *const libc::c_char,
    pub hints: *const addrinfo,
    pub res: *mut *mut addrinfo,
    pub exit_code: libc::c_int,
}
pub const ns_in6addrsz: C2RustUnnamed_6 = 16;
pub const ns_int16sz: C2RustUnnamed_6 = 2;
pub const ns_inaddrsz: C2RustUnnamed_6 = 4;
pub type C2RustUnnamed_6 = libc::c_uint;
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
#[inline]
unsafe extern "C" fn c_isxdigit(mut c: libc::c_int) -> bool {
    match c {
        48 | 49 | 50 | 51 | 52 | 53 | 54 | 55 | 56 | 57 | 97 | 98 | 99 | 100 | 101 | 102
        | 65 | 66 | 67 | 68 | 69 | 70 => return 1 as libc::c_int != 0,
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
pub unsafe extern "C" fn address_list_get_bounds(
    mut al: *const address_list,
    mut start: *mut libc::c_int,
    mut end: *mut libc::c_int,
) {
    *start = (*al).faulty;
    *end = (*al).count;
}
pub unsafe extern "C" fn address_list_address_at(
    mut al: *const address_list,
    mut pos: libc::c_int,
) -> *const ip_address {
    return ((*al).addresses).offset(pos as isize);
}
pub unsafe extern "C" fn address_list_contains(
    mut al: *const address_list,
    mut ip: *const ip_address,
) -> bool {
    let mut i: libc::c_int = 0;
    match (*ip).family {
        2 => {
            i = 0 as libc::c_int;
            while i < (*al).count {
                let mut cur: *mut ip_address = ((*al).addresses).offset(i as isize);
                if (*cur).family == 2 as libc::c_int
                    && (*cur).data.d4.s_addr == (*ip).data.d4.s_addr
                {
                    return 1 as libc::c_int != 0;
                }
                i += 1;
                i;
            }
            return 0 as libc::c_int != 0;
        }
        10 => {
            i = 0 as libc::c_int;
            while i < (*al).count {
                let mut cur_0: *mut ip_address = ((*al).addresses).offset(i as isize);
                if (*cur_0).family == 10 as libc::c_int
                    && (*cur_0).ipv6_scope == (*ip).ipv6_scope
                    && ({
                        let mut __a: *const in6_addr = &mut (*cur_0).data.d6
                            as *mut in6_addr as *const in6_addr;
                        let mut __b: *const in6_addr = &(*ip).data.d6 as *const in6_addr;
                        ((*__a).__in6_u.__u6_addr32[0 as libc::c_int as usize]
                            == (*__b).__in6_u.__u6_addr32[0 as libc::c_int as usize]
                            && (*__a).__in6_u.__u6_addr32[1 as libc::c_int as usize]
                                == (*__b).__in6_u.__u6_addr32[1 as libc::c_int as usize]
                            && (*__a).__in6_u.__u6_addr32[2 as libc::c_int as usize]
                                == (*__b).__in6_u.__u6_addr32[2 as libc::c_int as usize]
                            && (*__a).__in6_u.__u6_addr32[3 as libc::c_int as usize]
                                == (*__b).__in6_u.__u6_addr32[3 as libc::c_int as usize])
                            as libc::c_int
                    }) != 0
                {
                    return 1 as libc::c_int != 0;
                }
                i += 1;
                i;
            }
            return 0 as libc::c_int != 0;
        }
        _ => {
            abort();
        }
    };
}
pub unsafe extern "C" fn address_list_set_faulty(
    mut al: *mut address_list,
    mut index: libc::c_int,
) {
    if index != (*al).faulty {
        logprintf(
            LOG_ALWAYS,
            b"index: %d\nal->faulty: %d\n\0" as *const u8 as *const libc::c_char,
            index,
            (*al).faulty,
        );
        logprintf(
            LOG_ALWAYS,
            dcgettext(
                0 as *const libc::c_char,
                b"Error in handling the address list.\n\0" as *const u8
                    as *const libc::c_char,
                5 as libc::c_int,
            ),
        );
        logprintf(
            LOG_ALWAYS,
            dcgettext(
                0 as *const libc::c_char,
                b"Please report this issue to bug-wget@gnu.org\n\0" as *const u8
                    as *const libc::c_char,
                5 as libc::c_int,
            ),
        );
        abort();
    }
    (*al).faulty += 1;
    (*al).faulty;
    if (*al).faulty >= (*al).count {
        (*al).faulty = 0 as libc::c_int;
    }
}
pub unsafe extern "C" fn address_list_set_connected(mut al: *mut address_list) {
    (*al).connected = 1 as libc::c_int != 0;
}
pub unsafe extern "C" fn address_list_connected_p(mut al: *const address_list) -> bool {
    return (*al).connected;
}
unsafe extern "C" fn address_list_from_addrinfo(
    mut ai: *const addrinfo,
) -> *mut address_list {
    let mut al: *mut address_list = 0 as *mut address_list;
    let mut ptr: *const addrinfo = 0 as *const addrinfo;
    let mut cnt: libc::c_int = 0;
    let mut ip: *mut ip_address = 0 as *mut ip_address;
    cnt = 0 as libc::c_int;
    ptr = ai;
    while !ptr.is_null() {
        if (*ptr).ai_family == 2 as libc::c_int || (*ptr).ai_family == 10 as libc::c_int
        {
            cnt += 1;
            cnt;
        }
        ptr = (*ptr).ai_next;
    }
    if cnt == 0 as libc::c_int {
        return 0 as *mut address_list;
    }
    al = xcalloc(
        1 as libc::c_int as size_t,
        ::std::mem::size_of::<address_list>() as libc::c_ulong,
    ) as *mut address_list;
    (*al)
        .addresses = xmalloc(
        (cnt as libc::c_ulong)
            .wrapping_mul(::std::mem::size_of::<ip_address>() as libc::c_ulong),
    ) as *mut ip_address;
    (*al).count = cnt;
    (*al).refcount = 1 as libc::c_int;
    ip = (*al).addresses;
    ptr = ai;
    while !ptr.is_null() {
        if (*ptr).ai_family == 10 as libc::c_int {
            let mut sin6: *const sockaddr_in6 = (*ptr).ai_addr as *const sockaddr_in6;
            (*ip).family = 10 as libc::c_int;
            (*ip).data.d6 = (*sin6).sin6_addr;
            (*ip).ipv6_scope = (*sin6).sin6_scope_id as libc::c_int;
            ip = ip.offset(1);
            ip;
        } else if (*ptr).ai_family == 2 as libc::c_int {
            let mut sin: *const sockaddr_in = (*ptr).ai_addr as *const sockaddr_in;
            (*ip).family = 2 as libc::c_int;
            (*ip).data.d4 = (*sin).sin_addr;
            ip = ip.offset(1);
            ip;
        }
        ptr = (*ptr).ai_next;
    }
    return al;
}
unsafe extern "C" fn cmp_prefer_ipv4(
    mut addr1: *const libc::c_void,
    mut addr2: *const libc::c_void,
) -> libc::c_int {
    return !((*(addr1 as *const ip_address)).family == 2 as libc::c_int) as libc::c_int
        - !((*(addr2 as *const ip_address)).family == 2 as libc::c_int) as libc::c_int;
}
unsafe extern "C" fn cmp_prefer_ipv6(
    mut addr1: *const libc::c_void,
    mut addr2: *const libc::c_void,
) -> libc::c_int {
    return !((*(addr1 as *const ip_address)).family == 10 as libc::c_int) as libc::c_int
        - !((*(addr2 as *const ip_address)).family == 10 as libc::c_int) as libc::c_int;
}
unsafe extern "C" fn address_list_delete(mut al: *mut address_list) {
    rpl_free((*al).addresses as *mut libc::c_void);
    (*al).addresses = 0 as *mut ip_address;
    rpl_free(al as *mut libc::c_void);
    al = 0 as *mut address_list;
}
pub unsafe extern "C" fn address_list_release(mut al: *mut address_list) {
    (*al).refcount -= 1;
    (*al).refcount;
    if opt.debug as libc::c_long != 0 {
        debug_logprintf(
            b"Releasing 0x%0*lx (new refcount %d).\n\0" as *const u8
                as *const libc::c_char,
            (2 as libc::c_int as libc::c_ulong)
                .wrapping_mul(
                    ::std::mem::size_of::<*mut libc::c_void>() as libc::c_ulong,
                ) as libc::c_int,
            al as libc::c_ulong,
            (*al).refcount,
        );
    }
    if (*al).refcount <= 0 as libc::c_int {
        if opt.debug as libc::c_long != 0 {
            debug_logprintf(
                b"Deleting unused 0x%0*lx.\n\0" as *const u8 as *const libc::c_char,
                (2 as libc::c_int as libc::c_ulong)
                    .wrapping_mul(
                        ::std::mem::size_of::<*mut libc::c_void>() as libc::c_ulong,
                    ) as libc::c_int,
                al as libc::c_ulong,
            );
        }
        address_list_delete(al);
    }
}
unsafe extern "C" fn getaddrinfo_with_timeout_callback(mut arg: *mut libc::c_void) {
    let mut ctx: *mut gaiwt_context = arg as *mut gaiwt_context;
    (*ctx)
        .exit_code = getaddrinfo((*ctx).node, (*ctx).service, (*ctx).hints, (*ctx).res);
}
unsafe extern "C" fn getaddrinfo_with_timeout(
    mut node: *const libc::c_char,
    mut service: *const libc::c_char,
    mut hints: *const addrinfo,
    mut res: *mut *mut addrinfo,
    mut timeout: libc::c_double,
) -> libc::c_int {
    let mut ctx: gaiwt_context = gaiwt_context {
        node: 0 as *const libc::c_char,
        service: 0 as *const libc::c_char,
        hints: 0 as *const addrinfo,
        res: 0 as *mut *mut addrinfo,
        exit_code: 0,
    };
    ctx.node = node;
    ctx.service = service;
    ctx.hints = hints;
    ctx.res = res;
    if run_with_timeout(
        timeout,
        Some(
            getaddrinfo_with_timeout_callback
                as unsafe extern "C" fn(*mut libc::c_void) -> (),
        ),
        &mut ctx as *mut gaiwt_context as *mut libc::c_void,
    ) {
        *__errno_location() = 110 as libc::c_int;
        return -(11 as libc::c_int);
    }
    return ctx.exit_code;
}
pub unsafe extern "C" fn print_address(
    mut addr: *const ip_address,
) -> *const libc::c_char {
    static mut buf: [libc::c_char; 64] = [0; 64];
    if (inet_ntop(
        (*addr).family,
        &(*addr).data as *const C2RustUnnamed_4 as *mut libc::c_void,
        buf.as_mut_ptr(),
        ::std::mem::size_of::<[libc::c_char; 64]>() as libc::c_ulong as socklen_t,
    ))
        .is_null()
    {
        snprintf(
            buf.as_mut_ptr(),
            ::std::mem::size_of::<[libc::c_char; 64]>() as libc::c_ulong,
            b"<error: %s>\0" as *const u8 as *const libc::c_char,
            strerror(*__errno_location()),
        );
    }
    return buf.as_mut_ptr();
}
unsafe extern "C" fn is_valid_ipv4_address(
    mut str: *const libc::c_char,
    mut end: *const libc::c_char,
) -> bool {
    let mut saw_digit: bool = 0 as libc::c_int != 0;
    let mut octets: libc::c_int = 0 as libc::c_int;
    let mut val: libc::c_int = 0 as libc::c_int;
    while str < end {
        let fresh0 = str;
        str = str.offset(1);
        let mut ch: libc::c_int = *fresh0 as libc::c_int;
        if ch >= '0' as i32 && ch <= '9' as i32 {
            val = val * 10 as libc::c_int + (ch - '0' as i32);
            if val > 255 as libc::c_int {
                return 0 as libc::c_int != 0;
            }
            if !saw_digit {
                octets += 1;
                if octets > 4 as libc::c_int {
                    return 0 as libc::c_int != 0;
                }
                saw_digit = 1 as libc::c_int != 0;
            }
        } else if ch == '.' as i32 && saw_digit as libc::c_int != 0 {
            if octets == 4 as libc::c_int {
                return 0 as libc::c_int != 0;
            }
            val = 0 as libc::c_int;
            saw_digit = 0 as libc::c_int != 0;
        } else {
            return 0 as libc::c_int != 0
        }
    }
    if octets < 4 as libc::c_int {
        return 0 as libc::c_int != 0;
    }
    return 1 as libc::c_int != 0;
}
pub unsafe extern "C" fn is_valid_ipv6_address(
    mut str: *const libc::c_char,
    mut end: *const libc::c_char,
) -> bool {
    let mut curtok: *const libc::c_char = 0 as *const libc::c_char;
    let mut tp: libc::c_int = 0;
    let mut colonp: *const libc::c_char = 0 as *const libc::c_char;
    let mut saw_xdigit: bool = false;
    let mut val: libc::c_uint = 0;
    tp = 0 as libc::c_int;
    colonp = 0 as *const libc::c_char;
    if str == end {
        return 0 as libc::c_int != 0;
    }
    if *str as libc::c_int == ':' as i32 {
        str = str.offset(1);
        str;
        if str == end || *str as libc::c_int != ':' as i32 {
            return 0 as libc::c_int != 0;
        }
    }
    curtok = str;
    saw_xdigit = 0 as libc::c_int != 0;
    val = 0 as libc::c_int as libc::c_uint;
    while str < end {
        let fresh1 = str;
        str = str.offset(1);
        let mut ch: libc::c_int = *fresh1 as libc::c_int;
        if c_isxdigit(ch) {
            val <<= 4 as libc::c_int;
            val |= _unhex(ch as libc::c_uchar) as libc::c_uint;
            if val > 0xffff as libc::c_int as libc::c_uint {
                return 0 as libc::c_int != 0;
            }
            saw_xdigit = 1 as libc::c_int != 0;
        } else if ch == ':' as i32 {
            curtok = str;
            if !saw_xdigit {
                if !colonp.is_null() {
                    return 0 as libc::c_int != 0;
                }
                colonp = str.offset(tp as isize);
            } else {
                if str == end {
                    return 0 as libc::c_int != 0;
                }
                if tp > ns_in6addrsz as libc::c_int - ns_int16sz as libc::c_int {
                    return 0 as libc::c_int != 0;
                }
                tp += ns_int16sz as libc::c_int;
                saw_xdigit = 0 as libc::c_int != 0;
                val = 0 as libc::c_int as libc::c_uint;
            }
        } else if ch == '.' as i32
            && tp <= ns_in6addrsz as libc::c_int - ns_inaddrsz as libc::c_int
            && is_valid_ipv4_address(curtok, end) as libc::c_int == 1 as libc::c_int
        {
            tp += ns_inaddrsz as libc::c_int;
            saw_xdigit = 0 as libc::c_int != 0;
            break;
        } else {
            return 0 as libc::c_int != 0
        }
    }
    if saw_xdigit {
        if tp > ns_in6addrsz as libc::c_int - ns_int16sz as libc::c_int {
            return 0 as libc::c_int != 0;
        }
        tp += ns_int16sz as libc::c_int;
    }
    if !colonp.is_null() {
        if tp == ns_in6addrsz as libc::c_int {
            return 0 as libc::c_int != 0;
        }
        tp = ns_in6addrsz as libc::c_int;
    }
    if tp != ns_in6addrsz as libc::c_int {
        return 0 as libc::c_int != 0;
    }
    return 1 as libc::c_int != 0;
}
static mut host_name_addresses_map: *mut hash_table = 0 as *const hash_table
    as *mut hash_table;
unsafe extern "C" fn cache_query(mut host: *const libc::c_char) -> *mut address_list {
    let mut al: *mut address_list = 0 as *mut address_list;
    if host_name_addresses_map.is_null() {
        return 0 as *mut address_list;
    }
    al = hash_table_get(host_name_addresses_map, host as *const libc::c_void)
        as *mut address_list;
    if !al.is_null() {
        if opt.debug as libc::c_long != 0 {
            debug_logprintf(
                b"Found %s in host_name_addresses_map (%p)\n\0" as *const u8
                    as *const libc::c_char,
                host,
                al as *mut libc::c_void,
            );
        }
        (*al).refcount += 1;
        (*al).refcount;
        return al;
    }
    return 0 as *mut address_list;
}
unsafe extern "C" fn cache_store(
    mut host: *const libc::c_char,
    mut al: *mut address_list,
) {
    if host_name_addresses_map.is_null() {
        host_name_addresses_map = make_nocase_string_hash_table(0 as libc::c_int);
    }
    (*al).refcount += 1;
    (*al).refcount;
    hash_table_put(
        host_name_addresses_map,
        xstrdup_lower(host) as *const libc::c_void,
        al as *const libc::c_void,
    );
    if opt.debug as libc::c_long != 0 {
        let mut i: libc::c_int = 0;
        debug_logprintf(b"Caching %s =>\0" as *const u8 as *const libc::c_char, host);
        i = 0 as libc::c_int;
        while i < (*al).count {
            debug_logprintf(
                b" %s\0" as *const u8 as *const libc::c_char,
                print_address(((*al).addresses).offset(i as isize)),
            );
            i += 1;
            i;
        }
        debug_logprintf(b"\n\0" as *const u8 as *const libc::c_char);
    }
}
unsafe extern "C" fn cache_remove(mut host: *const libc::c_char) {
    let mut al: *mut address_list = 0 as *mut address_list;
    if host_name_addresses_map.is_null() {
        return;
    }
    al = hash_table_get(host_name_addresses_map, host as *const libc::c_void)
        as *mut address_list;
    if !al.is_null() {
        address_list_release(al);
        hash_table_remove(host_name_addresses_map, host as *const libc::c_void);
    }
}
pub unsafe extern "C" fn lookup_host(
    mut host: *const libc::c_char,
    mut flags: libc::c_int,
) -> *mut address_list {
    let mut al: *mut address_list = 0 as *mut address_list;
    let mut silent: bool = flags & LH_SILENT as libc::c_int != 0;
    let mut use_cache: bool = false;
    let mut numeric_address: bool = 0 as libc::c_int != 0;
    let mut timeout: libc::c_double = opt.dns_timeout;
    let mut end: *const libc::c_char = host.offset(strlen(host) as isize);
    if is_valid_ipv4_address(host, end) as libc::c_int != 0
        || is_valid_ipv6_address(host, end) as libc::c_int != 0
    {
        numeric_address = 1 as libc::c_int != 0;
    }
    use_cache = opt.dns_cache;
    if flags & LH_BIND as libc::c_int != 0 || numeric_address as libc::c_int != 0 {
        use_cache = 0 as libc::c_int != 0;
    }
    if use_cache {
        if flags & LH_REFRESH as libc::c_int == 0 {
            al = cache_query(host);
            if !al.is_null() {
                return al;
            }
        } else {
            cache_remove(host);
        }
    }
    if !silent && !numeric_address {
        let mut str: *mut libc::c_char = 0 as *mut libc::c_char;
        let mut name: *mut libc::c_char = 0 as *mut libc::c_char;
        if opt.enable_iri as libc::c_int != 0
            && {
                name = idn_decode(host as *mut libc::c_char);
                !name.is_null()
            }
        {
            str = aprintf(b"%s (%s)\0" as *const u8 as *const libc::c_char, name, host);
            rpl_free(name as *mut libc::c_void);
            name = 0 as *mut libc::c_char;
        }
        logprintf(
            LOG_VERBOSE,
            dcgettext(
                0 as *const libc::c_char,
                b"Resolving %s... \0" as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ),
            quotearg_style(
                escape_quoting_style,
                if !str.is_null() { str as *const libc::c_char } else { host },
            ),
        );
        rpl_free(str as *mut libc::c_void);
        str = 0 as *mut libc::c_char;
    }
    let mut err: libc::c_int = 0;
    let mut hints: addrinfo = addrinfo {
        ai_flags: 0,
        ai_family: 0,
        ai_socktype: 0,
        ai_protocol: 0,
        ai_addrlen: 0,
        ai_addr: 0 as *mut sockaddr,
        ai_canonname: 0 as *mut libc::c_char,
        ai_next: 0 as *mut addrinfo,
    };
    let mut res: *mut addrinfo = 0 as *mut addrinfo;
    memset(
        &mut hints as *mut addrinfo as *mut libc::c_void,
        '\0' as i32,
        ::std::mem::size_of::<addrinfo>() as libc::c_ulong,
    );
    hints.ai_socktype = SOCK_STREAM as libc::c_int;
    if opt.ipv4_only {
        hints.ai_family = 2 as libc::c_int;
    } else if opt.ipv6_only {
        hints.ai_family = 10 as libc::c_int;
    } else {
        hints.ai_family = 0 as libc::c_int;
    }
    if flags & LH_BIND as libc::c_int != 0 {
        hints.ai_flags |= 0x1 as libc::c_int;
    }
    if numeric_address {
        hints.ai_flags |= 0x4 as libc::c_int;
        timeout = 0 as libc::c_int as libc::c_double;
    }
    err = getaddrinfo_with_timeout(
        host,
        0 as *const libc::c_char,
        &mut hints,
        &mut res,
        timeout,
    );
    if err != 0 as libc::c_int || res.is_null() {
        if !silent {
            logprintf(
                LOG_VERBOSE,
                dcgettext(
                    0 as *const libc::c_char,
                    b"failed: %s.\n\0" as *const u8 as *const libc::c_char,
                    5 as libc::c_int,
                ),
                if err != -(11 as libc::c_int) {
                    gai_strerror(err)
                } else {
                    strerror(*__errno_location()) as *const libc::c_char
                },
            );
        }
        return 0 as *mut address_list;
    }
    al = address_list_from_addrinfo(res);
    freeaddrinfo(res);
    if al.is_null() {
        logprintf(
            LOG_VERBOSE,
            dcgettext(
                0 as *const libc::c_char,
                b"failed: No IPv4/IPv6 addresses for host.\n\0" as *const u8
                    as *const libc::c_char,
                5 as libc::c_int,
            ),
        );
        return 0 as *mut address_list;
    }
    if (*al).count > 1 as libc::c_int
        && opt.prefer_family as libc::c_uint
            != prefer_none as libc::c_int as libc::c_uint
    {
        stable_sort(
            (*al).addresses as *mut libc::c_void,
            (*al).count as size_t,
            ::std::mem::size_of::<ip_address>() as libc::c_ulong,
            if opt.prefer_family as libc::c_uint
                == prefer_ipv4 as libc::c_int as libc::c_uint
            {
                Some(
                    cmp_prefer_ipv4
                        as unsafe extern "C" fn(
                            *const libc::c_void,
                            *const libc::c_void,
                        ) -> libc::c_int,
                )
            } else {
                Some(
                    cmp_prefer_ipv6
                        as unsafe extern "C" fn(
                            *const libc::c_void,
                            *const libc::c_void,
                        ) -> libc::c_int,
                )
            },
        );
    }
    if !silent && !numeric_address {
        let mut i: libc::c_int = 0;
        let mut printmax: libc::c_int = (*al).count;
        if !opt.show_all_dns_entries && printmax > 3 as libc::c_int {
            printmax = 3 as libc::c_int;
        }
        i = 0 as libc::c_int;
        while i < printmax {
            logputs(LOG_VERBOSE, print_address(((*al).addresses).offset(i as isize)));
            if i < printmax - 1 as libc::c_int {
                logputs(LOG_VERBOSE, b", \0" as *const u8 as *const libc::c_char);
            }
            i += 1;
            i;
        }
        if printmax != (*al).count {
            logputs(LOG_VERBOSE, b", ...\0" as *const u8 as *const libc::c_char);
        }
        logputs(LOG_VERBOSE, b"\n\0" as *const u8 as *const libc::c_char);
    }
    if use_cache {
        cache_store(host, al);
    }
    return al;
}
pub unsafe extern "C" fn accept_domain(mut u: *mut url) -> bool {
    if !(opt.domains).is_null() {
        if !sufmatch(opt.domains as *mut *const libc::c_char, (*u).host) {
            return 0 as libc::c_int != 0;
        }
    }
    if !(opt.exclude_domains).is_null() {
        if sufmatch(opt.exclude_domains as *mut *const libc::c_char, (*u).host) {
            return 0 as libc::c_int != 0;
        }
    }
    return 1 as libc::c_int != 0;
}
pub unsafe extern "C" fn sufmatch(
    mut list: *mut *const libc::c_char,
    mut what: *const libc::c_char,
) -> bool {
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut k: libc::c_int = 0;
    let mut lw: libc::c_int = 0;
    lw = strlen(what) as libc::c_int;
    i = 0 as libc::c_int;
    while !(*list.offset(i as isize)).is_null() {
        j = strlen(*list.offset(i as isize)) as libc::c_int;
        if !(lw < j) {
            k = lw;
            while j >= 0 as libc::c_int && k >= 0 as libc::c_int {
                if c_tolower(
                    *(*list.offset(i as isize)).offset(j as isize) as libc::c_int,
                ) != c_tolower(*what.offset(k as isize) as libc::c_int)
                {
                    break;
                }
                j -= 1;
                j;
                k -= 1;
                k;
            }
            if j == -(1 as libc::c_int)
                && (k == -(1 as libc::c_int)
                    || *what.offset(k as isize) as libc::c_int == '.' as i32
                    || *(*list.offset(i as isize)).offset(0 as libc::c_int as isize)
                        as libc::c_int == '.' as i32)
            {
                return 1 as libc::c_int != 0;
            }
        }
        i += 1;
        i;
    }
    return 0 as libc::c_int != 0;
}
pub unsafe extern "C" fn is_valid_ip_address(mut name: *const libc::c_char) -> bool {
    let mut endp: *const libc::c_char = 0 as *const libc::c_char;
    endp = name.offset(strlen(name) as isize);
    if is_valid_ipv4_address(name, endp) {
        return 1 as libc::c_int != 0;
    }
    if is_valid_ipv6_address(name, endp) {
        return 1 as libc::c_int != 0;
    }
    return 0 as libc::c_int != 0;
}
