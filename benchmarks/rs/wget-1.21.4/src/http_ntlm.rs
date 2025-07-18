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
    fn strchr(_: *const libc::c_char, _: libc::c_int) -> *mut libc::c_char;
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    static mut opt: options;
    fn xmalloc(s: size_t) -> *mut libc::c_void;
    fn snprintf(
        _: *mut libc::c_char,
        _: libc::c_ulong,
        _: *const libc::c_char,
        _: ...
    ) -> libc::c_int;
    fn debug_logprintf(_: *const libc::c_char, _: ...);
    fn wget_base64_encode(
        _: *const libc::c_void,
        _: size_t,
        _: *mut libc::c_char,
    ) -> size_t;
    fn wget_base64_decode(
        _: *const libc::c_char,
        _: *mut libc::c_void,
        _: size_t,
    ) -> ssize_t;
    fn nettle_md4_init(ctx: *mut md4_ctx);
    fn nettle_md4_update(ctx: *mut md4_ctx, length: size_t, data: *const uint8_t);
    fn nettle_md4_digest(ctx: *mut md4_ctx, length: size_t, digest: *mut uint8_t);
    fn nettle_des_set_key(ctx: *mut des_ctx, key: *const uint8_t) -> libc::c_int;
    fn nettle_des_encrypt(
        ctx: *const des_ctx,
        length: size_t,
        dst: *mut uint8_t,
        src: *const uint8_t,
    );
}
pub type __uint8_t = libc::c_uchar;
pub type __uint32_t = libc::c_uint;
pub type __int64_t = libc::c_long;
pub type __uint64_t = libc::c_ulong;
pub type __ssize_t = libc::c_long;
pub type ssize_t = __ssize_t;
pub type size_t = libc::c_ulong;
pub type int64_t = __int64_t;
pub type uint8_t = __uint8_t;
pub type uint32_t = __uint32_t;
pub type uint64_t = __uint64_t;
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
pub type wgetntlm = libc::c_uint;
pub const NTLMSTATE_LAST: wgetntlm = 4;
pub const NTLMSTATE_TYPE3: wgetntlm = 3;
pub const NTLMSTATE_TYPE2: wgetntlm = 2;
pub const NTLMSTATE_TYPE1: wgetntlm = 1;
pub const NTLMSTATE_NONE: wgetntlm = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ntlmdata {
    pub state: wgetntlm,
    pub nonce: [libc::c_uchar; 8],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct des_ctx {
    pub key: [uint32_t; 32],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct md4_ctx {
    pub state: [uint32_t; 4],
    pub count: uint64_t,
    pub index: libc::c_uint,
    pub block: [uint8_t; 64],
}
#[inline]
unsafe extern "C" fn c_isspace(mut c: libc::c_int) -> bool {
    match c {
        32 | 9 | 10 | 11 | 12 | 13 => return 1 as libc::c_int != 0,
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
pub unsafe extern "C" fn ntlm_input(
    mut ntlm: *mut ntlmdata,
    mut header: *const libc::c_char,
) -> bool {
    if 0 as libc::c_int
        != strncmp(
            header,
            b"NTLM\0" as *const u8 as *const libc::c_char,
            4 as libc::c_int as libc::c_ulong,
        )
    {
        return 0 as libc::c_int != 0;
    }
    header = header.offset(4 as libc::c_int as isize);
    while *header as libc::c_int != 0
        && c_isspace(*header as libc::c_int) as libc::c_int != 0
    {
        header = header.offset(1);
        header;
    }
    if *header != 0 {
        let mut size: ssize_t = 0;
        let mut buffer: [libc::c_char; 48] = [0; 48];
        if opt.debug as libc::c_long != 0 {
            debug_logprintf(
                b"Received a type-2 NTLM message.\n\0" as *const u8
                    as *const libc::c_char,
            );
        }
        size = wget_base64_decode(
            header,
            buffer.as_mut_ptr() as *mut libc::c_void,
            ::std::mem::size_of::<[libc::c_char; 48]>() as libc::c_ulong,
        );
        if size < 0 as libc::c_int as libc::c_long {
            return 0 as libc::c_int != 0;
        }
        (*ntlm).state = NTLMSTATE_TYPE2;
        if size as size_t >= ::std::mem::size_of::<[libc::c_char; 48]>() as libc::c_ulong
        {
            memcpy(
                ((*ntlm).nonce).as_mut_ptr() as *mut libc::c_void,
                &mut *buffer.as_mut_ptr().offset(24 as libc::c_int as isize)
                    as *mut libc::c_char as *const libc::c_void,
                8 as libc::c_int as libc::c_ulong,
            );
        }
    } else {
        if (*ntlm).state as libc::c_uint == NTLMSTATE_LAST as libc::c_int as libc::c_uint
        {
            if opt.debug as libc::c_long != 0 {
                debug_logprintf(
                    b"NTLM auth restarted.\n\0" as *const u8 as *const libc::c_char,
                );
            }
        } else if (*ntlm).state as libc::c_uint
            == NTLMSTATE_TYPE3 as libc::c_int as libc::c_uint
        {
            if opt.debug as libc::c_long != 0 {
                debug_logprintf(
                    b"NTLM handshake rejected.\n\0" as *const u8 as *const libc::c_char,
                );
            }
            (*ntlm).state = NTLMSTATE_NONE;
            return 0 as libc::c_int != 0;
        } else if (*ntlm).state as libc::c_uint
            >= NTLMSTATE_TYPE1 as libc::c_int as libc::c_uint
        {
            if opt.debug as libc::c_long != 0 {
                debug_logprintf(
                    b"Unexpected empty NTLM message.\n\0" as *const u8
                        as *const libc::c_char,
                );
            }
            return 0 as libc::c_int != 0;
        }
        if opt.debug as libc::c_long != 0 {
            debug_logprintf(
                b"Empty NTLM message, (re)starting transaction.\n\0" as *const u8
                    as *const libc::c_char,
            );
        }
        (*ntlm).state = NTLMSTATE_TYPE1;
    }
    return 1 as libc::c_int != 0;
}
unsafe extern "C" fn setup_des_key(
    mut key_56: *mut libc::c_uchar,
    mut des: *mut des_ctx,
) {
    let mut key: [libc::c_uchar; 8] = [0; 8];
    key[0 as libc::c_int as usize] = *key_56.offset(0 as libc::c_int as isize);
    key[1 as libc::c_int
        as usize] = ((*key_56.offset(0 as libc::c_int as isize) as libc::c_int)
        << 7 as libc::c_int & 0xff as libc::c_int
        | *key_56.offset(1 as libc::c_int as isize) as libc::c_int >> 1 as libc::c_int)
        as libc::c_uchar;
    key[2 as libc::c_int
        as usize] = ((*key_56.offset(1 as libc::c_int as isize) as libc::c_int)
        << 6 as libc::c_int & 0xff as libc::c_int
        | *key_56.offset(2 as libc::c_int as isize) as libc::c_int >> 2 as libc::c_int)
        as libc::c_uchar;
    key[3 as libc::c_int
        as usize] = ((*key_56.offset(2 as libc::c_int as isize) as libc::c_int)
        << 5 as libc::c_int & 0xff as libc::c_int
        | *key_56.offset(3 as libc::c_int as isize) as libc::c_int >> 3 as libc::c_int)
        as libc::c_uchar;
    key[4 as libc::c_int
        as usize] = ((*key_56.offset(3 as libc::c_int as isize) as libc::c_int)
        << 4 as libc::c_int & 0xff as libc::c_int
        | *key_56.offset(4 as libc::c_int as isize) as libc::c_int >> 4 as libc::c_int)
        as libc::c_uchar;
    key[5 as libc::c_int
        as usize] = ((*key_56.offset(4 as libc::c_int as isize) as libc::c_int)
        << 3 as libc::c_int & 0xff as libc::c_int
        | *key_56.offset(5 as libc::c_int as isize) as libc::c_int >> 5 as libc::c_int)
        as libc::c_uchar;
    key[6 as libc::c_int
        as usize] = ((*key_56.offset(5 as libc::c_int as isize) as libc::c_int)
        << 2 as libc::c_int & 0xff as libc::c_int
        | *key_56.offset(6 as libc::c_int as isize) as libc::c_int >> 6 as libc::c_int)
        as libc::c_uchar;
    key[7 as libc::c_int
        as usize] = ((*key_56.offset(6 as libc::c_int as isize) as libc::c_int)
        << 1 as libc::c_int & 0xff as libc::c_int) as libc::c_uchar;
    nettle_des_set_key(des, key.as_mut_ptr());
}
unsafe extern "C" fn calc_resp(
    mut keys: *mut libc::c_uchar,
    mut plaintext: *mut libc::c_uchar,
    mut results: *mut libc::c_uchar,
) {
    let mut des: des_ctx = des_ctx { key: [0; 32] };
    setup_des_key(keys, &mut des);
    nettle_des_encrypt(&mut des, 8 as libc::c_int as size_t, results, plaintext);
    setup_des_key(keys.offset(7 as libc::c_int as isize), &mut des);
    nettle_des_encrypt(
        &mut des,
        8 as libc::c_int as size_t,
        results.offset(8 as libc::c_int as isize),
        plaintext,
    );
    setup_des_key(keys.offset(14 as libc::c_int as isize), &mut des);
    nettle_des_encrypt(
        &mut des,
        8 as libc::c_int as size_t,
        results.offset(16 as libc::c_int as isize),
        plaintext,
    );
}
unsafe extern "C" fn mkhash(
    mut password: *const libc::c_char,
    mut nonce: *mut libc::c_uchar,
    mut lmresp: *mut libc::c_uchar,
    mut ntresp: *mut libc::c_uchar,
) {
    let mut lmbuffer: [libc::c_uchar; 21] = [0; 21];
    let mut ntbuffer: [libc::c_uchar; 21] = [0; 21];
    let mut pw: [libc::c_uchar; 14] = [0; 14];
    static mut magic: [libc::c_uchar; 8] = [
        0x4b as libc::c_int as libc::c_uchar,
        0x47 as libc::c_int as libc::c_uchar,
        0x53 as libc::c_int as libc::c_uchar,
        0x21 as libc::c_int as libc::c_uchar,
        0x40 as libc::c_int as libc::c_uchar,
        0x23 as libc::c_int as libc::c_uchar,
        0x24 as libc::c_int as libc::c_uchar,
        0x25 as libc::c_int as libc::c_uchar,
    ];
    let mut i: size_t = 0;
    let mut len: size_t = strlen(password);
    if len > ::std::mem::size_of::<[libc::c_uchar; 14]>() as libc::c_ulong {
        len = ::std::mem::size_of::<[libc::c_uchar; 14]>() as libc::c_ulong;
    }
    i = 0 as libc::c_int as size_t;
    while i < len {
        pw[i
            as usize] = c_toupper(*password.offset(i as isize) as libc::c_int)
            as libc::c_uchar;
        i = i.wrapping_add(1);
        i;
    }
    while i < ::std::mem::size_of::<[libc::c_uchar; 14]>() as libc::c_ulong {
        pw[i as usize] = 0 as libc::c_int as libc::c_uchar;
        i = i.wrapping_add(1);
        i;
    }
    let mut des: des_ctx = des_ctx { key: [0; 32] };
    setup_des_key(pw.as_mut_ptr(), &mut des);
    nettle_des_encrypt(
        &mut des,
        8 as libc::c_int as size_t,
        lmbuffer.as_mut_ptr(),
        magic.as_ptr(),
    );
    setup_des_key(pw.as_mut_ptr().offset(7 as libc::c_int as isize), &mut des);
    nettle_des_encrypt(
        &mut des,
        8 as libc::c_int as size_t,
        lmbuffer.as_mut_ptr().offset(8 as libc::c_int as isize),
        magic.as_ptr(),
    );
    memset(
        lmbuffer.as_mut_ptr().offset(16 as libc::c_int as isize) as *mut libc::c_void,
        0 as libc::c_int,
        5 as libc::c_int as libc::c_ulong,
    );
    calc_resp(lmbuffer.as_mut_ptr(), nonce, lmresp);
    let mut MD4: md4_ctx = md4_ctx {
        state: [0; 4],
        count: 0,
        index: 0,
        block: [0; 64],
    };
    let mut pw4: [libc::c_uchar; 64] = [0; 64];
    len = strlen(password);
    if len
        > (::std::mem::size_of::<[libc::c_uchar; 64]>() as libc::c_ulong)
            .wrapping_div(2 as libc::c_int as libc::c_ulong)
    {
        len = (::std::mem::size_of::<[libc::c_uchar; 64]>() as libc::c_ulong)
            .wrapping_div(2 as libc::c_int as libc::c_ulong);
    }
    i = 0 as libc::c_int as size_t;
    while i < len {
        pw4[(2 as libc::c_int as libc::c_ulong).wrapping_mul(i)
            as usize] = *password.offset(i as isize) as libc::c_uchar;
        pw4[(2 as libc::c_int as libc::c_ulong)
            .wrapping_mul(i)
            .wrapping_add(1 as libc::c_int as libc::c_ulong)
            as usize] = 0 as libc::c_int as libc::c_uchar;
        i = i.wrapping_add(1);
        i;
    }
    nettle_md4_init(&mut MD4);
    nettle_md4_update(
        &mut MD4,
        (2 as libc::c_int as libc::c_ulong).wrapping_mul(len) as libc::c_uint as size_t,
        pw4.as_mut_ptr(),
    );
    nettle_md4_digest(&mut MD4, 16 as libc::c_int as size_t, ntbuffer.as_mut_ptr());
    memset(
        ntbuffer.as_mut_ptr().offset(16 as libc::c_int as isize) as *mut libc::c_void,
        0 as libc::c_int,
        5 as libc::c_int as libc::c_ulong,
    );
    calc_resp(ntbuffer.as_mut_ptr(), nonce, ntresp);
}
pub unsafe extern "C" fn ntlm_output(
    mut ntlm: *mut ntlmdata,
    mut user: *const libc::c_char,
    mut passwd: *const libc::c_char,
    mut ready: *mut bool,
) -> *mut libc::c_char {
    let mut domain: *const libc::c_char = b"\0" as *const u8 as *const libc::c_char;
    let mut host: *const libc::c_char = b"\0" as *const u8 as *const libc::c_char;
    let mut domlen: size_t = strlen(domain);
    let mut hostlen: size_t = strlen(host);
    let mut hostoff: size_t = 0;
    let mut domoff: size_t = 0;
    let mut size: size_t = 0;
    let mut ntlmbuf: [libc::c_char; 256] = [0; 256];
    let mut output: *mut libc::c_char = 0 as *mut libc::c_char;
    *ready = 0 as libc::c_int != 0;
    if user.is_null() {
        user = b"\0" as *const u8 as *const libc::c_char;
    }
    if passwd.is_null() {
        passwd = b"\0" as *const u8 as *const libc::c_char;
    }
    match (*ntlm).state as libc::c_uint {
        1 | 0 | 4 => {
            hostoff = 32 as libc::c_int as size_t;
            domoff = hostoff.wrapping_add(hostlen);
            if opt.debug as libc::c_long != 0 {
                debug_logprintf(
                    b"Creating a type-1 NTLM message.\n\0" as *const u8
                        as *const libc::c_char,
                );
            }
            snprintf(
                ntlmbuf.as_mut_ptr(),
                ::std::mem::size_of::<[libc::c_char; 256]>() as libc::c_ulong,
                b"NTLMSSP%c\x01%c%c%c%c%c%c%c%c%c%c%c%c%c%c%c%c%c%c%c%c%c%c%c%s%s\0"
                    as *const u8 as *const libc::c_char,
                0 as libc::c_int,
                0 as libc::c_int,
                0 as libc::c_int,
                0 as libc::c_int,
                ((1 as libc::c_int) << 1 as libc::c_int
                    | (1 as libc::c_int) << 9 as libc::c_int) & 0xff as libc::c_int,
                ((1 as libc::c_int) << 1 as libc::c_int
                    | (1 as libc::c_int) << 9 as libc::c_int) >> 8 as libc::c_int
                    & 0xff as libc::c_int,
                ((1 as libc::c_int) << 1 as libc::c_int
                    | (1 as libc::c_int) << 9 as libc::c_int) >> 16 as libc::c_int
                    & 0xff as libc::c_int,
                ((1 as libc::c_int) << 1 as libc::c_int
                    | (1 as libc::c_int) << 9 as libc::c_int) >> 24 as libc::c_int,
                (domlen & 0xff as libc::c_int as libc::c_ulong) as libc::c_char
                    as libc::c_int,
                (domlen >> 8 as libc::c_int) as libc::c_char as libc::c_int,
                (domlen & 0xff as libc::c_int as libc::c_ulong) as libc::c_char
                    as libc::c_int,
                (domlen >> 8 as libc::c_int) as libc::c_char as libc::c_int,
                (domoff & 0xff as libc::c_int as libc::c_ulong) as libc::c_char
                    as libc::c_int,
                (domoff >> 8 as libc::c_int) as libc::c_char as libc::c_int,
                0 as libc::c_int,
                0 as libc::c_int,
                (hostlen & 0xff as libc::c_int as libc::c_ulong) as libc::c_char
                    as libc::c_int,
                (hostlen >> 8 as libc::c_int) as libc::c_char as libc::c_int,
                (hostlen & 0xff as libc::c_int as libc::c_ulong) as libc::c_char
                    as libc::c_int,
                (hostlen >> 8 as libc::c_int) as libc::c_char as libc::c_int,
                (hostoff & 0xff as libc::c_int as libc::c_ulong) as libc::c_char
                    as libc::c_int,
                (hostoff >> 8 as libc::c_int) as libc::c_char as libc::c_int,
                0 as libc::c_int,
                0 as libc::c_int,
                host,
                domain,
            );
            size = (32 as libc::c_int as libc::c_ulong)
                .wrapping_add(hostlen)
                .wrapping_add(domlen);
            output = xmalloc(
                (5 as libc::c_int as libc::c_ulong)
                    .wrapping_add(
                        (4 as libc::c_int as libc::c_ulong)
                            .wrapping_mul(
                                size
                                    .wrapping_add(2 as libc::c_int as libc::c_ulong)
                                    .wrapping_div(3 as libc::c_int as libc::c_ulong),
                            ),
                    )
                    .wrapping_add(1 as libc::c_int as libc::c_ulong),
            ) as *mut libc::c_char;
            memcpy(
                output as *mut libc::c_void,
                b"NTLM \0" as *const u8 as *const libc::c_char as *const libc::c_void,
                5 as libc::c_int as libc::c_ulong,
            );
            wget_base64_encode(
                ntlmbuf.as_mut_ptr() as *const libc::c_void,
                size,
                output.offset(5 as libc::c_int as isize),
            );
        }
        2 => {
            let mut lmrespoff: size_t = 0;
            let mut ntrespoff: size_t = 0;
            let mut useroff: size_t = 0;
            let mut lmresp: [libc::c_uchar; 24] = [0; 24];
            let mut ntresp: [libc::c_uchar; 24] = [0; 24];
            let mut usr: *const libc::c_char = 0 as *const libc::c_char;
            let mut userlen: size_t = 0;
            if opt.debug as libc::c_long != 0 {
                debug_logprintf(
                    b"Creating a type-3 NTLM message.\n\0" as *const u8
                        as *const libc::c_char,
                );
            }
            usr = strchr(user, '\\' as i32);
            if usr.is_null() {
                usr = strchr(user, '/' as i32);
            }
            if !usr.is_null() {
                domain = user;
                domlen = usr.offset_from(domain) as libc::c_long as size_t;
                usr = usr.offset(1);
                usr;
            } else {
                usr = user;
            }
            userlen = strlen(usr);
            mkhash(
                passwd,
                &mut *((*ntlm).nonce).as_mut_ptr().offset(0 as libc::c_int as isize),
                lmresp.as_mut_ptr(),
                ntresp.as_mut_ptr(),
            );
            domoff = 64 as libc::c_int as size_t;
            useroff = domoff.wrapping_add(domlen);
            hostoff = useroff.wrapping_add(userlen);
            lmrespoff = hostoff.wrapping_add(hostlen);
            ntrespoff = lmrespoff.wrapping_add(0x18 as libc::c_int as libc::c_ulong);
            snprintf(
                ntlmbuf.as_mut_ptr(),
                ::std::mem::size_of::<[libc::c_char; 256]>() as libc::c_ulong,
                b"NTLMSSP%c\x03%c%c%c%c%c%c%c%c%c%c%c%c%c%c%c%c%c%c%c%c%c%c%c%c%c%c%c%c%c%c%c%c%c%c%c%c%c%c%c%c%c%c%c%c%c%c%c\xFF\xFF%c%c\x01\x82%c%c\0"
                    as *const u8 as *const libc::c_char,
                0 as libc::c_int,
                0 as libc::c_int,
                0 as libc::c_int,
                0 as libc::c_int,
                (0x18 as libc::c_int & 0xff as libc::c_int) as libc::c_char
                    as libc::c_int,
                (0x18 as libc::c_int >> 8 as libc::c_int) as libc::c_char as libc::c_int,
                (0x18 as libc::c_int & 0xff as libc::c_int) as libc::c_char
                    as libc::c_int,
                (0x18 as libc::c_int >> 8 as libc::c_int) as libc::c_char as libc::c_int,
                (lmrespoff & 0xff as libc::c_int as libc::c_ulong) as libc::c_char
                    as libc::c_int,
                (lmrespoff >> 8 as libc::c_int) as libc::c_char as libc::c_int,
                0 as libc::c_int,
                0 as libc::c_int,
                (0x18 as libc::c_int & 0xff as libc::c_int) as libc::c_char
                    as libc::c_int,
                (0x18 as libc::c_int >> 8 as libc::c_int) as libc::c_char as libc::c_int,
                (0x18 as libc::c_int & 0xff as libc::c_int) as libc::c_char
                    as libc::c_int,
                (0x18 as libc::c_int >> 8 as libc::c_int) as libc::c_char as libc::c_int,
                (ntrespoff & 0xff as libc::c_int as libc::c_ulong) as libc::c_char
                    as libc::c_int,
                (ntrespoff >> 8 as libc::c_int) as libc::c_char as libc::c_int,
                0 as libc::c_int,
                0 as libc::c_int,
                (domlen & 0xff as libc::c_int as libc::c_ulong) as libc::c_char
                    as libc::c_int,
                (domlen >> 8 as libc::c_int) as libc::c_char as libc::c_int,
                (domlen & 0xff as libc::c_int as libc::c_ulong) as libc::c_char
                    as libc::c_int,
                (domlen >> 8 as libc::c_int) as libc::c_char as libc::c_int,
                (domoff & 0xff as libc::c_int as libc::c_ulong) as libc::c_char
                    as libc::c_int,
                (domoff >> 8 as libc::c_int) as libc::c_char as libc::c_int,
                0 as libc::c_int,
                0 as libc::c_int,
                (userlen & 0xff as libc::c_int as libc::c_ulong) as libc::c_char
                    as libc::c_int,
                (userlen >> 8 as libc::c_int) as libc::c_char as libc::c_int,
                (userlen & 0xff as libc::c_int as libc::c_ulong) as libc::c_char
                    as libc::c_int,
                (userlen >> 8 as libc::c_int) as libc::c_char as libc::c_int,
                (useroff & 0xff as libc::c_int as libc::c_ulong) as libc::c_char
                    as libc::c_int,
                (useroff >> 8 as libc::c_int) as libc::c_char as libc::c_int,
                0 as libc::c_int,
                0 as libc::c_int,
                (hostlen & 0xff as libc::c_int as libc::c_ulong) as libc::c_char
                    as libc::c_int,
                (hostlen >> 8 as libc::c_int) as libc::c_char as libc::c_int,
                (hostlen & 0xff as libc::c_int as libc::c_ulong) as libc::c_char
                    as libc::c_int,
                (hostlen >> 8 as libc::c_int) as libc::c_char as libc::c_int,
                (hostoff & 0xff as libc::c_int as libc::c_ulong) as libc::c_char
                    as libc::c_int,
                (hostoff >> 8 as libc::c_int) as libc::c_char as libc::c_int,
                0 as libc::c_int,
                0 as libc::c_int,
                0 as libc::c_int,
                0 as libc::c_int,
                0 as libc::c_int,
                0 as libc::c_int,
                0 as libc::c_int,
                0 as libc::c_int,
                0 as libc::c_int,
                0 as libc::c_int,
            );
            size = 64 as libc::c_int as size_t;
            ntlmbuf[63 as libc::c_int as usize] = 0 as libc::c_int as libc::c_char;
            ntlmbuf[62 as libc::c_int as usize] = ntlmbuf[63 as libc::c_int as usize];
            if size.wrapping_add(userlen).wrapping_add(domlen)
                >= ::std::mem::size_of::<[libc::c_char; 256]>() as libc::c_ulong
            {
                return 0 as *mut libc::c_char;
            }
            memcpy(
                &mut *ntlmbuf.as_mut_ptr().offset(size as isize) as *mut libc::c_char
                    as *mut libc::c_void,
                domain as *const libc::c_void,
                domlen,
            );
            size = (size as libc::c_ulong).wrapping_add(domlen) as size_t as size_t;
            memcpy(
                &mut *ntlmbuf.as_mut_ptr().offset(size as isize) as *mut libc::c_char
                    as *mut libc::c_void,
                usr as *const libc::c_void,
                userlen,
            );
            size = (size as libc::c_ulong).wrapping_add(userlen) as size_t as size_t;
            if size
                < (::std::mem::size_of::<[libc::c_char; 256]>() as libc::c_ulong)
                    .wrapping_sub(0x18 as libc::c_int as libc::c_ulong)
            {
                memcpy(
                    &mut *ntlmbuf.as_mut_ptr().offset(size as isize) as *mut libc::c_char
                        as *mut libc::c_void,
                    lmresp.as_mut_ptr() as *const libc::c_void,
                    0x18 as libc::c_int as libc::c_ulong,
                );
                size = (size as libc::c_ulong)
                    .wrapping_add(0x18 as libc::c_int as libc::c_ulong) as size_t
                    as size_t;
            }
            if size
                < (::std::mem::size_of::<[libc::c_char; 256]>() as libc::c_ulong)
                    .wrapping_sub(0x18 as libc::c_int as libc::c_ulong)
            {
                memcpy(
                    &mut *ntlmbuf.as_mut_ptr().offset(size as isize) as *mut libc::c_char
                        as *mut libc::c_void,
                    ntresp.as_mut_ptr() as *const libc::c_void,
                    0x18 as libc::c_int as libc::c_ulong,
                );
                size = (size as libc::c_ulong)
                    .wrapping_add(0x18 as libc::c_int as libc::c_ulong) as size_t
                    as size_t;
            }
            ntlmbuf[56 as libc::c_int
                as usize] = (size & 0xff as libc::c_int as libc::c_ulong)
                as libc::c_char;
            ntlmbuf[57 as libc::c_int
                as usize] = (size >> 8 as libc::c_int) as libc::c_char;
            output = xmalloc(
                (5 as libc::c_int as libc::c_ulong)
                    .wrapping_add(
                        (4 as libc::c_int as libc::c_ulong)
                            .wrapping_mul(
                                size
                                    .wrapping_add(2 as libc::c_int as libc::c_ulong)
                                    .wrapping_div(3 as libc::c_int as libc::c_ulong),
                            ),
                    )
                    .wrapping_add(1 as libc::c_int as libc::c_ulong),
            ) as *mut libc::c_char;
            memcpy(
                output as *mut libc::c_void,
                b"NTLM \0" as *const u8 as *const libc::c_char as *const libc::c_void,
                5 as libc::c_int as libc::c_ulong,
            );
            wget_base64_encode(
                ntlmbuf.as_mut_ptr() as *const libc::c_void,
                size,
                output.offset(5 as libc::c_int as isize),
            );
            (*ntlm).state = NTLMSTATE_TYPE3;
            *ready = 1 as libc::c_int != 0;
        }
        3 => {
            *ready = 1 as libc::c_int != 0;
            output = 0 as *mut libc::c_char;
        }
        _ => {}
    }
    return output;
}
