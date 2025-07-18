use ::libc;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    pub type hash_table;
    fn fclose(__stream: *mut FILE) -> libc::c_int;
    fn fdopen(__fd: libc::c_int, __modes: *const libc::c_char) -> *mut FILE;
    fn fprintf(_: *mut FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn sprintf(_: *mut libc::c_char, _: *const libc::c_char, _: ...) -> libc::c_int;
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
    fn fread(
        _: *mut libc::c_void,
        _: libc::c_ulong,
        _: libc::c_ulong,
        _: *mut FILE,
    ) -> libc::c_ulong;
    fn fwrite(
        _: *const libc::c_void,
        _: libc::c_ulong,
        _: libc::c_ulong,
        _: *mut FILE,
    ) -> libc::c_ulong;
    fn time(__timer: *mut time_t) -> time_t;
    fn strftime(
        __s: *mut libc::c_char,
        __maxsize: size_t,
        __format: *const libc::c_char,
        __tp: *const tm,
    ) -> size_t;
    fn gmtime(__timer: *const time_t) -> *mut tm;
    fn dcgettext(
        __domainname: *const libc::c_char,
        __msgid: *const libc::c_char,
        __category: libc::c_int,
    ) -> *mut libc::c_char;
    fn dcngettext(
        __domainname: *const libc::c_char,
        __msgid1: *const libc::c_char,
        __msgid2: *const libc::c_char,
        __n: libc::c_ulong,
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
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    fn strdup(_: *const libc::c_char) -> *mut libc::c_char;
    fn strtok_r(
        __s: *mut libc::c_char,
        __delim: *const libc::c_char,
        __save_ptr: *mut *mut libc::c_char,
    ) -> *mut libc::c_char;
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    fn rpl_free(_: *mut libc::c_void);
    static mut opt: options;
    fn log_set_warc_log_fp(_: *mut FILE);
    static mut program_argstring: *const libc::c_char;
    fn logprintf(_: log_options, _: *const libc::c_char, _: ...);
    fn quote(arg: *const libc::c_char) -> *const libc::c_char;
    fn rpl_fseeko(fp: *mut FILE, offset: off_t, whence: libc::c_int) -> libc::c_int;
    fn rpl_fseek(
        fp: *mut FILE,
        offset: libc::c_long,
        whence: libc::c_int,
    ) -> libc::c_int;
    fn rpl_fopen(filename: *const libc::c_char, mode: *const libc::c_char) -> *mut FILE;
    fn rpl_fflush(gl_stream: *mut FILE) -> libc::c_int;
    fn fileno(__stream: *mut FILE) -> libc::c_int;
    fn xmalloc(s: size_t) -> *mut libc::c_void;
    fn rewind(__stream: *mut FILE);
    fn ftello(__stream: *mut FILE) -> __off_t;
    fn feof(__stream: *mut FILE) -> libc::c_int;
    fn ferror(__stream: *mut FILE) -> libc::c_int;
    fn exit(_: libc::c_int) -> !;
    fn mkostemp(__template: *mut libc::c_char, __flags: libc::c_int) -> libc::c_int;
    fn hash_table_new(
        _: libc::c_int,
        _: Option::<unsafe extern "C" fn(*const libc::c_void) -> libc::c_ulong>,
        _: Option::<
            unsafe extern "C" fn(*const libc::c_void, *const libc::c_void) -> libc::c_int,
        >,
    ) -> *mut hash_table;
    fn hash_table_get(_: *const hash_table, _: *const libc::c_void) -> *mut libc::c_void;
    fn hash_table_put(
        _: *mut hash_table,
        _: *const libc::c_void,
        _: *const libc::c_void,
    );
    fn hash_table_count(_: *const hash_table) -> libc::c_int;
    fn aprintf(_: *const libc::c_char, _: ...) -> *mut libc::c_char;
    fn number_to_string(_: *mut libc::c_char, _: wgint) -> *mut libc::c_char;
    static mut version_string: *const libc::c_char;
    fn base_name(file: *const libc::c_char) -> *mut libc::c_char;
    fn url_escape(_: *const libc::c_char) -> *mut libc::c_char;
    fn path_search(
        tmpl: *mut libc::c_char,
        tmpl_len: size_t,
        dir: *const libc::c_char,
        pfx: *const libc::c_char,
        try_tmpdir: bool,
    ) -> libc::c_int;
    fn sha1_init_ctx(ctx: *mut sha1_ctx);
    fn sha1_process_block(buffer: *const libc::c_void, len: size_t, ctx: *mut sha1_ctx);
    fn sha1_process_bytes(buffer: *const libc::c_void, len: size_t, ctx: *mut sha1_ctx);
    fn sha1_finish_ctx(
        ctx: *mut sha1_ctx,
        resbuf: *mut libc::c_void,
    ) -> *mut libc::c_void;
    fn sha1_stream(stream: *mut FILE, resblock: *mut libc::c_void) -> libc::c_int;
    fn base32_encode(
        in_0: *const libc::c_char,
        inlen: idx_t,
        out: *mut libc::c_char,
        outlen: idx_t,
    );
    fn base32_decode_alloc_ctx(
        ctx: *mut base32_decode_context,
        in_0: *const libc::c_char,
        inlen: idx_t,
        out: *mut *mut libc::c_char,
        outlen: *mut idx_t,
    ) -> bool;
    fn close(__fd: libc::c_int) -> libc::c_int;
    fn dup(__fd: libc::c_int) -> libc::c_int;
    fn unlink(__name: *const libc::c_char) -> libc::c_int;
    fn ftruncate(__fd: libc::c_int, __length: __off_t) -> libc::c_int;
    fn gzdopen(fd: libc::c_int, mode: *const libc::c_char) -> gzFile;
    fn gzwrite(file: gzFile, buf: voidpc, len: libc::c_uint) -> libc::c_int;
    fn gzclose(file: gzFile) -> libc::c_int;
    fn uuid_generate(out: *mut libc::c_uchar);
    fn uuid_unparse(uu: *const libc::c_uchar, out: *mut libc::c_char);
    fn print_address(_: *const ip_address) -> *const libc::c_char;
}
pub type __uint8_t = libc::c_uchar;
pub type __uint16_t = libc::c_ushort;
pub type __uint32_t = libc::c_uint;
pub type __int64_t = libc::c_long;
pub type __off_t = libc::c_long;
pub type __off64_t = libc::c_long;
pub type __time_t = libc::c_long;
pub type __ssize_t = libc::c_long;
pub type off_t = __off_t;
pub type off64_t = __off64_t;
pub type ssize_t = __ssize_t;
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
pub type ptrdiff_t = libc::c_long;
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
pub type idx_t = ptrdiff_t;
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
pub struct sha1_ctx {
    pub A: uint32_t,
    pub B: uint32_t,
    pub C: uint32_t,
    pub D: uint32_t,
    pub E: uint32_t,
    pub total: [uint32_t; 2],
    pub buflen: uint32_t,
    pub buffer: [uint32_t; 32],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct base32_decode_context {
    pub i: libc::c_int,
    pub buf: [libc::c_char; 8],
}
pub type voidpc = *const libc::c_void;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct gzFile_s {
    pub have: libc::c_uint,
    pub next: *mut libc::c_uchar,
    pub pos: off64_t,
}
pub type gzFile = *mut gzFile_s;
pub type uuid_t = [libc::c_uchar; 16];
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct in_addr {
    pub s_addr: in_addr_t,
}
pub type in_addr_t = uint32_t;
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
pub const WGET_EXIT_GENERIC_ERROR: C2RustUnnamed_5 = 1;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct warc_cdx_record {
    pub url: *mut libc::c_char,
    pub uuid: *mut libc::c_char,
    pub digest: [libc::c_char; 20],
}
pub type C2RustUnnamed_5 = libc::c_uint;
pub const WGET_EXIT_UNKNOWN: C2RustUnnamed_5 = 9;
pub const WGET_EXIT_SERVER_ERROR: C2RustUnnamed_5 = 8;
pub const WGET_EXIT_PROTOCOL_ERROR: C2RustUnnamed_5 = 7;
pub const WGET_EXIT_SERVER_AUTH_FAIL: C2RustUnnamed_5 = 6;
pub const WGET_EXIT_SSL_AUTH_FAIL: C2RustUnnamed_5 = 5;
pub const WGET_EXIT_NETWORK_FAIL: C2RustUnnamed_5 = 4;
pub const WGET_EXIT_IO_FAIL: C2RustUnnamed_5 = 3;
pub const WGET_EXIT_PARSE_ERROR: C2RustUnnamed_5 = 2;
pub const WGET_EXIT_SUCCESS: C2RustUnnamed_5 = 0;
#[inline]
unsafe extern "C" fn getline(
    mut __lineptr: *mut *mut libc::c_char,
    mut __n: *mut size_t,
    mut __stream: *mut FILE,
) -> __ssize_t {
    return __getdelim(__lineptr, __n, '\n' as i32, __stream);
}
static mut warc_log_fp: *mut FILE = 0 as *const FILE as *mut FILE;
static mut warc_manifest_fp: *mut FILE = 0 as *const FILE as *mut FILE;
static mut warc_current_file: *mut FILE = 0 as *const FILE as *mut FILE;
static mut warc_current_gzfile: gzFile = 0 as *const gzFile_s as *mut gzFile_s;
static mut warc_current_gzfile_offset: off_t = 0;
static mut warc_current_gzfile_uncompressed_size: off_t = 0;
static mut warc_write_ok: bool = false;
static mut warc_current_cdx_file: *mut FILE = 0 as *const FILE as *mut FILE;
static mut warc_current_warcinfo_uuid_str: [libc::c_char; 48] = [0; 48];
static mut warc_current_filename: *mut libc::c_char = 0 as *const libc::c_char
    as *mut libc::c_char;
static mut warc_current_file_number: libc::c_int = 0;
static mut warc_cdx_dedup_table: *mut hash_table = 0 as *const hash_table
    as *mut hash_table;
unsafe extern "C" fn warc_hash_sha1_digest(
    mut key: *const libc::c_void,
) -> libc::c_ulong {
    let mut v: libc::c_ulong = 0 as libc::c_int as libc::c_ulong;
    memcpy(
        &mut v as *mut libc::c_ulong as *mut libc::c_void,
        key,
        ::std::mem::size_of::<libc::c_ulong>() as libc::c_ulong,
    );
    return v;
}
unsafe extern "C" fn warc_cmp_sha1_digest(
    mut digest1: *const libc::c_void,
    mut digest2: *const libc::c_void,
) -> libc::c_int {
    return (memcmp(digest1, digest2, 20 as libc::c_int as libc::c_ulong) == 0)
        as libc::c_int;
}
unsafe extern "C" fn warc_write_buffer(
    mut buffer: *const libc::c_char,
    mut size: size_t,
) -> size_t {
    if !warc_current_gzfile.is_null() {
        warc_current_gzfile_uncompressed_size = (warc_current_gzfile_uncompressed_size
            as libc::c_ulong)
            .wrapping_add(size) as off_t as off_t;
        return gzwrite(warc_current_gzfile, buffer as voidpc, size as libc::c_uint)
            as size_t;
    } else {
        return fwrite(
            buffer as *const libc::c_void,
            1 as libc::c_int as libc::c_ulong,
            size,
            warc_current_file,
        )
    };
}
unsafe extern "C" fn warc_write_string(mut str: *const libc::c_char) -> bool {
    let mut n: size_t = 0;
    if !warc_write_ok {
        return 0 as libc::c_int != 0;
    }
    n = strlen(str);
    if n != warc_write_buffer(str, n) {
        warc_write_ok = 0 as libc::c_int != 0;
    }
    return warc_write_ok;
}
unsafe extern "C" fn warc_write_start_record() -> bool {
    if !warc_write_ok {
        return 0 as libc::c_int != 0;
    }
    rpl_fflush(warc_current_file);
    if opt.warc_maxsize > 0 as libc::c_int as libc::c_long
        && ftello(warc_current_file) >= opt.warc_maxsize
    {
        warc_start_new_file(0 as libc::c_int != 0);
    }
    if opt.warc_compression_enabled {
        let mut dup_fd: libc::c_int = 0;
        warc_current_gzfile_offset = ftello(warc_current_file);
        if rpl_fseek(
            warc_current_file,
            14 as libc::c_int as libc::c_long,
            1 as libc::c_int,
        ) < 0 as libc::c_int
        {
            logprintf(
                LOG_NOTQUIET,
                dcgettext(
                    0 as *const libc::c_char,
                    b"Error setting WARC file position.\n\0" as *const u8
                        as *const libc::c_char,
                    5 as libc::c_int,
                ),
            );
            warc_write_ok = 0 as libc::c_int != 0;
            return 0 as libc::c_int != 0;
        }
        if rpl_fflush(warc_current_file) != 0 as libc::c_int {
            logprintf(
                LOG_NOTQUIET,
                dcgettext(
                    0 as *const libc::c_char,
                    b"Error flushing WARC file to disk.\n\0" as *const u8
                        as *const libc::c_char,
                    5 as libc::c_int,
                ),
            );
            warc_write_ok = 0 as libc::c_int != 0;
            return 0 as libc::c_int != 0;
        }
        dup_fd = dup(fileno(warc_current_file));
        if dup_fd < 0 as libc::c_int {
            logprintf(
                LOG_NOTQUIET,
                dcgettext(
                    0 as *const libc::c_char,
                    b"Error duplicating WARC file file descriptor.\n\0" as *const u8
                        as *const libc::c_char,
                    5 as libc::c_int,
                ),
            );
            warc_write_ok = 0 as libc::c_int != 0;
            return 0 as libc::c_int != 0;
        }
        warc_current_gzfile = gzdopen(
            dup_fd,
            b"wb9\0" as *const u8 as *const libc::c_char,
        );
        warc_current_gzfile_uncompressed_size = 0 as libc::c_int as off_t;
        if warc_current_gzfile.is_null() {
            logprintf(
                LOG_NOTQUIET,
                dcgettext(
                    0 as *const libc::c_char,
                    b"Error opening GZIP stream to WARC file.\n\0" as *const u8
                        as *const libc::c_char,
                    5 as libc::c_int,
                ),
            );
            close(dup_fd);
            warc_write_ok = 0 as libc::c_int != 0;
            return 0 as libc::c_int != 0;
        }
    }
    warc_write_string(b"WARC/1.0\r\n\0" as *const u8 as *const libc::c_char);
    return warc_write_ok;
}
unsafe extern "C" fn warc_write_header(
    mut name: *const libc::c_char,
    mut value: *const libc::c_char,
) -> bool {
    if !value.is_null() {
        warc_write_string(name);
        warc_write_string(b": \0" as *const u8 as *const libc::c_char);
        warc_write_string(value);
        warc_write_string(b"\r\n\0" as *const u8 as *const libc::c_char);
    }
    return warc_write_ok;
}
unsafe extern "C" fn warc_write_header_uri(
    mut name: *const libc::c_char,
    mut value: *const libc::c_char,
) -> bool {
    if !value.is_null() {
        warc_write_string(name);
        warc_write_string(b": <\0" as *const u8 as *const libc::c_char);
        warc_write_string(value);
        warc_write_string(b">\r\n\0" as *const u8 as *const libc::c_char);
    }
    return warc_write_ok;
}
unsafe extern "C" fn warc_write_block_from_file(mut data_in: *mut FILE) -> bool {
    let mut content_length: [libc::c_char; 21] = [0; 21];
    let mut buffer: [libc::c_char; 8192] = [0; 8192];
    let mut s: size_t = 0;
    rpl_fseeko(data_in, 0 as libc::c_long, 2 as libc::c_int);
    number_to_string(content_length.as_mut_ptr(), ftello(data_in));
    warc_write_header(
        b"Content-Length\0" as *const u8 as *const libc::c_char,
        content_length.as_mut_ptr(),
    );
    warc_write_string(b"\r\n\0" as *const u8 as *const libc::c_char);
    if rpl_fseeko(data_in, 0 as libc::c_long, 0 as libc::c_int) != 0 as libc::c_int {
        warc_write_ok = 0 as libc::c_int != 0;
    }
    while warc_write_ok as libc::c_int != 0
        && {
            s = fread(
                buffer.as_mut_ptr() as *mut libc::c_void,
                1 as libc::c_int as libc::c_ulong,
                8192 as libc::c_int as libc::c_ulong,
                data_in,
            );
            s > 0 as libc::c_int as libc::c_ulong
        }
    {
        if warc_write_buffer(buffer.as_mut_ptr(), s) < s {
            warc_write_ok = 0 as libc::c_int != 0;
        }
    }
    return warc_write_ok;
}
unsafe extern "C" fn warc_write_end_record() -> bool {
    if warc_write_buffer(
        b"\r\n\r\n\0" as *const u8 as *const libc::c_char,
        4 as libc::c_int as size_t,
    ) != 4 as libc::c_int as libc::c_ulong
    {
        warc_write_ok = 0 as libc::c_int != 0;
        return 0 as libc::c_int != 0;
    }
    if warc_write_ok as libc::c_int != 0 && !warc_current_gzfile.is_null() {
        let mut extra_header: [libc::c_char; 14] = [0; 14];
        let mut static_header: [libc::c_char; 10] = [0; 10];
        let mut current_offset: off_t = 0;
        let mut uncompressed_size: off_t = 0;
        let mut compressed_size: off_t = 0;
        let mut result: size_t = 0;
        if gzclose(warc_current_gzfile) != 0 as libc::c_int {
            warc_write_ok = 0 as libc::c_int != 0;
            return 0 as libc::c_int != 0;
        }
        rpl_fflush(warc_current_file);
        rpl_fseeko(warc_current_file, 0 as libc::c_int as off_t, 2 as libc::c_int);
        current_offset = ftello(warc_current_file);
        uncompressed_size = current_offset - warc_current_gzfile_offset;
        compressed_size = warc_current_gzfile_uncompressed_size;
        result = rpl_fseeko(
            warc_current_file,
            warc_current_gzfile_offset + 14 as libc::c_int as libc::c_long,
            0 as libc::c_int,
        ) as size_t;
        if result != 0 as libc::c_int as libc::c_ulong {
            warc_write_ok = 0 as libc::c_int != 0;
            return 0 as libc::c_int != 0;
        }
        result = fread(
            static_header.as_mut_ptr() as *mut libc::c_void,
            1 as libc::c_int as libc::c_ulong,
            10 as libc::c_int as libc::c_ulong,
            warc_current_file,
        );
        if result != 10 as libc::c_int as libc::c_ulong {
            warc_write_ok = 0 as libc::c_int != 0;
            return 0 as libc::c_int != 0;
        }
        static_header[3 as libc::c_int
            as usize] = (static_header[3 as libc::c_int as usize] as libc::c_int
            | 0x4 as libc::c_int) as libc::c_char;
        rpl_fseeko(warc_current_file, warc_current_gzfile_offset, 0 as libc::c_int);
        fwrite(
            static_header.as_mut_ptr() as *const libc::c_void,
            1 as libc::c_int as libc::c_ulong,
            10 as libc::c_int as libc::c_ulong,
            warc_current_file,
        );
        extra_header[0 as libc::c_int
            as usize] = (14 as libc::c_int - 2 as libc::c_int & 255 as libc::c_int)
            as libc::c_char;
        extra_header[1 as libc::c_int
            as usize] = (14 as libc::c_int - 2 as libc::c_int >> 8 as libc::c_int
            & 255 as libc::c_int) as libc::c_char;
        extra_header[2 as libc::c_int as usize] = 's' as i32 as libc::c_char;
        extra_header[3 as libc::c_int as usize] = 'l' as i32 as libc::c_char;
        extra_header[4 as libc::c_int
            as usize] = (8 as libc::c_int & 255 as libc::c_int) as libc::c_char;
        extra_header[5 as libc::c_int
            as usize] = (8 as libc::c_int >> 8 as libc::c_int & 255 as libc::c_int)
            as libc::c_char;
        extra_header[6 as libc::c_int
            as usize] = (uncompressed_size & 255 as libc::c_int as libc::c_long)
            as libc::c_char;
        extra_header[7 as libc::c_int
            as usize] = (uncompressed_size >> 8 as libc::c_int
            & 255 as libc::c_int as libc::c_long) as libc::c_char;
        extra_header[8 as libc::c_int
            as usize] = (uncompressed_size >> 16 as libc::c_int
            & 255 as libc::c_int as libc::c_long) as libc::c_char;
        extra_header[9 as libc::c_int
            as usize] = (uncompressed_size >> 24 as libc::c_int
            & 255 as libc::c_int as libc::c_long) as libc::c_char;
        extra_header[10 as libc::c_int
            as usize] = (compressed_size & 255 as libc::c_int as libc::c_long)
            as libc::c_char;
        extra_header[11 as libc::c_int
            as usize] = (compressed_size >> 8 as libc::c_int
            & 255 as libc::c_int as libc::c_long) as libc::c_char;
        extra_header[12 as libc::c_int
            as usize] = (compressed_size >> 16 as libc::c_int
            & 255 as libc::c_int as libc::c_long) as libc::c_char;
        extra_header[13 as libc::c_int
            as usize] = (compressed_size >> 24 as libc::c_int
            & 255 as libc::c_int as libc::c_long) as libc::c_char;
        rpl_fseeko(
            warc_current_file,
            warc_current_gzfile_offset + 10 as libc::c_int as libc::c_long,
            0 as libc::c_int,
        );
        fwrite(
            extra_header.as_mut_ptr() as *const libc::c_void,
            1 as libc::c_int as libc::c_ulong,
            14 as libc::c_int as libc::c_ulong,
            warc_current_file,
        );
        rpl_fflush(warc_current_file);
        rpl_fseeko(warc_current_file, 0 as libc::c_int as off_t, 2 as libc::c_int);
    }
    return warc_write_ok;
}
unsafe extern "C" fn warc_write_date_header(mut timestamp: *const libc::c_char) -> bool {
    let mut current_timestamp: [libc::c_char; 21] = [0; 21];
    return warc_write_header(
        b"WARC-Date\0" as *const u8 as *const libc::c_char,
        if !timestamp.is_null() {
            timestamp
        } else {
            warc_timestamp(
                current_timestamp.as_mut_ptr(),
                ::std::mem::size_of::<[libc::c_char; 21]>() as libc::c_ulong,
            ) as *const libc::c_char
        },
    );
}
unsafe extern "C" fn warc_write_ip_header(mut ip: *const ip_address) -> bool {
    if !ip.is_null() {
        return warc_write_header(
            b"WARC-IP-Address\0" as *const u8 as *const libc::c_char,
            print_address(ip),
        )
    } else {
        return warc_write_ok
    };
}
unsafe extern "C" fn warc_sha1_stream_with_payload(
    mut stream: *mut FILE,
    mut res_block: *mut libc::c_void,
    mut res_payload: *mut libc::c_void,
    mut payload_offset: off_t,
) -> libc::c_int {
    let mut ctx_block: sha1_ctx = sha1_ctx {
        A: 0,
        B: 0,
        C: 0,
        D: 0,
        E: 0,
        total: [0; 2],
        buflen: 0,
        buffer: [0; 32],
    };
    let mut ctx_payload: sha1_ctx = sha1_ctx {
        A: 0,
        B: 0,
        C: 0,
        D: 0,
        E: 0,
        total: [0; 2],
        buflen: 0,
        buffer: [0; 32],
    };
    let mut pos: off_t = 0;
    let mut sum: off_t = 0;
    let mut buffer: *mut libc::c_char = xmalloc(
        (32768 as libc::c_int + 72 as libc::c_int) as size_t,
    ) as *mut libc::c_char;
    sha1_init_ctx(&mut ctx_block);
    if payload_offset >= 0 as libc::c_int as libc::c_long {
        sha1_init_ctx(&mut ctx_payload);
    }
    pos = 0 as libc::c_int as off_t;
    's_28: loop {
        let mut n: off_t = 0;
        sum = 0 as libc::c_int as off_t;
        loop {
            n = fread(
                buffer.offset(sum as isize) as *mut libc::c_void,
                1 as libc::c_int as libc::c_ulong,
                (32768 as libc::c_int as libc::c_long - sum) as libc::c_ulong,
                stream,
            ) as off_t;
            sum += n;
            pos += n;
            if sum == 32768 as libc::c_int as libc::c_long {
                break;
            }
            if n == 0 as libc::c_int as libc::c_long {
                if ferror(stream) != 0 {
                    rpl_free(buffer as *mut libc::c_void);
                    buffer = 0 as *mut libc::c_char;
                    return 1 as libc::c_int;
                }
                break 's_28;
            } else if feof(stream) != 0 {
                break 's_28;
            }
        }
        sha1_process_block(
            buffer as *const libc::c_void,
            32768 as libc::c_int as size_t,
            &mut ctx_block,
        );
        if payload_offset >= 0 as libc::c_int as libc::c_long && payload_offset < pos {
            let mut start_of_payload: off_t = payload_offset
                - (pos - 32768 as libc::c_int as libc::c_long);
            if start_of_payload <= 0 as libc::c_int as libc::c_long {
                start_of_payload = 0 as libc::c_int as off_t;
            }
            sha1_process_bytes(
                buffer.offset(start_of_payload as isize) as *const libc::c_void,
                (32768 as libc::c_int as libc::c_long - start_of_payload) as size_t,
                &mut ctx_payload,
            );
        }
    }
    if sum > 0 as libc::c_int as libc::c_long {
        sha1_process_bytes(buffer as *const libc::c_void, sum as size_t, &mut ctx_block);
        if payload_offset >= 0 as libc::c_int as libc::c_long && payload_offset < pos {
            let mut start_of_payload_0: off_t = payload_offset - (pos - sum);
            if start_of_payload_0 <= 0 as libc::c_int as libc::c_long {
                start_of_payload_0 = 0 as libc::c_int as off_t;
            }
            sha1_process_bytes(
                buffer.offset(start_of_payload_0 as isize) as *const libc::c_void,
                (sum - start_of_payload_0) as size_t,
                &mut ctx_payload,
            );
        }
    }
    sha1_finish_ctx(&mut ctx_block, res_block);
    if payload_offset >= 0 as libc::c_int as libc::c_long {
        sha1_finish_ctx(&mut ctx_payload, res_payload);
    }
    rpl_free(buffer as *mut libc::c_void);
    buffer = 0 as *mut libc::c_char;
    return 0 as libc::c_int;
}
unsafe extern "C" fn warc_base32_sha1_digest(
    mut sha1_digest: *const libc::c_char,
    mut sha1_base32: *mut libc::c_char,
    mut sha1_base32_size: size_t,
) -> *mut libc::c_char {
    if sha1_base32_size
        >= ((20 as libc::c_int + 4 as libc::c_int) / 5 as libc::c_int * 8 as libc::c_int
            + 5 as libc::c_int + 1 as libc::c_int) as libc::c_ulong
    {
        memcpy(
            sha1_base32 as *mut libc::c_void,
            b"sha1:\0" as *const u8 as *const libc::c_char as *const libc::c_void,
            5 as libc::c_int as libc::c_ulong,
        );
        base32_encode(
            sha1_digest,
            20 as libc::c_int as idx_t,
            sha1_base32.offset(5 as libc::c_int as isize),
            sha1_base32_size.wrapping_sub(5 as libc::c_int as libc::c_ulong) as idx_t,
        );
    } else {
        *sha1_base32 = 0 as libc::c_int as libc::c_char;
    }
    return sha1_base32;
}
unsafe extern "C" fn warc_write_digest_headers(
    mut file: *mut FILE,
    mut payload_offset: libc::c_long,
) {
    if opt.warc_digests_enabled {
        let mut sha1_res_block: [libc::c_char; 20] = [0; 20];
        let mut sha1_res_payload: [libc::c_char; 20] = [0; 20];
        rewind(file);
        if warc_sha1_stream_with_payload(
            file,
            sha1_res_block.as_mut_ptr() as *mut libc::c_void,
            sha1_res_payload.as_mut_ptr() as *mut libc::c_void,
            payload_offset,
        ) == 0 as libc::c_int
        {
            let mut digest: [libc::c_char; 38] = [0; 38];
            warc_write_header(
                b"WARC-Block-Digest\0" as *const u8 as *const libc::c_char,
                warc_base32_sha1_digest(
                    sha1_res_block.as_mut_ptr(),
                    digest.as_mut_ptr(),
                    ::std::mem::size_of::<[libc::c_char; 38]>() as libc::c_ulong,
                ),
            );
            if payload_offset >= 0 as libc::c_int as libc::c_long {
                warc_write_header(
                    b"WARC-Payload-Digest\0" as *const u8 as *const libc::c_char,
                    warc_base32_sha1_digest(
                        sha1_res_payload.as_mut_ptr(),
                        digest.as_mut_ptr(),
                        ::std::mem::size_of::<[libc::c_char; 38]>() as libc::c_ulong,
                    ),
                );
            }
        }
    }
}
pub unsafe extern "C" fn warc_timestamp(
    mut timestamp: *mut libc::c_char,
    mut timestamp_size: size_t,
) -> *mut libc::c_char {
    let mut rawtime: time_t = time(0 as *mut time_t);
    let mut timeinfo: *mut tm = gmtime(&mut rawtime);
    if strftime(
        timestamp,
        timestamp_size,
        b"%Y-%m-%dT%H:%M:%SZ\0" as *const u8 as *const libc::c_char,
        timeinfo,
    ) == 0 as libc::c_int as libc::c_ulong
        && timestamp_size > 0 as libc::c_int as libc::c_ulong
    {
        *timestamp = 0 as libc::c_int as libc::c_char;
    }
    return timestamp;
}
pub unsafe extern "C" fn warc_uuid_str(
    mut urn_str: *mut libc::c_char,
    mut urn_size: size_t,
) {
    let mut uuid_str: [libc::c_char; 37] = [0; 37];
    let mut record_id: uuid_t = [0; 16];
    uuid_generate(record_id.as_mut_ptr());
    uuid_unparse(record_id.as_mut_ptr() as *const libc::c_uchar, uuid_str.as_mut_ptr());
    snprintf(
        urn_str,
        urn_size,
        b"<urn:uuid:%s>\0" as *const u8 as *const libc::c_char,
        uuid_str.as_mut_ptr(),
    );
}
unsafe extern "C" fn warc_write_warcinfo_record(
    mut filename: *const libc::c_char,
) -> bool {
    let mut warc_tmp: *mut FILE = 0 as *mut FILE;
    let mut timestamp: [libc::c_char; 22] = [0; 22];
    let mut filename_basename: *mut libc::c_char = 0 as *mut libc::c_char;
    warc_uuid_str(
        warc_current_warcinfo_uuid_str.as_mut_ptr(),
        ::std::mem::size_of::<[libc::c_char; 48]>() as libc::c_ulong,
    );
    warc_timestamp(
        timestamp.as_mut_ptr(),
        ::std::mem::size_of::<[libc::c_char; 22]>() as libc::c_ulong,
    );
    filename_basename = base_name(filename);
    warc_write_start_record();
    warc_write_header(
        b"WARC-Type\0" as *const u8 as *const libc::c_char,
        b"warcinfo\0" as *const u8 as *const libc::c_char,
    );
    warc_write_header(
        b"Content-Type\0" as *const u8 as *const libc::c_char,
        b"application/warc-fields\0" as *const u8 as *const libc::c_char,
    );
    warc_write_header(
        b"WARC-Date\0" as *const u8 as *const libc::c_char,
        timestamp.as_mut_ptr(),
    );
    warc_write_header(
        b"WARC-Record-ID\0" as *const u8 as *const libc::c_char,
        warc_current_warcinfo_uuid_str.as_mut_ptr(),
    );
    warc_write_header(
        b"WARC-Filename\0" as *const u8 as *const libc::c_char,
        filename_basename,
    );
    rpl_free(filename_basename as *mut libc::c_void);
    filename_basename = 0 as *mut libc::c_char;
    warc_tmp = warc_tempfile();
    if warc_tmp.is_null() {
        return 0 as libc::c_int != 0;
    }
    fprintf(
        warc_tmp,
        b"software: Wget/%s (%s)\r\n\0" as *const u8 as *const libc::c_char,
        version_string,
        b"linux-gnu\0" as *const u8 as *const libc::c_char,
    );
    fprintf(
        warc_tmp,
        b"format: WARC File Format 1.0\r\n\0" as *const u8 as *const libc::c_char,
    );
    fprintf(
        warc_tmp,
        b"conformsTo: http://bibnum.bnf.fr/WARC/WARC_ISO_28500_version1_latestdraft.pdf\r\n\0"
            as *const u8 as *const libc::c_char,
    );
    fprintf(
        warc_tmp,
        b"robots: %s\r\n\0" as *const u8 as *const libc::c_char,
        if opt.use_robots as libc::c_int != 0 {
            b"classic\0" as *const u8 as *const libc::c_char
        } else {
            b"off\0" as *const u8 as *const libc::c_char
        },
    );
    fprintf(
        warc_tmp,
        b"wget-arguments: %s\r\n\0" as *const u8 as *const libc::c_char,
        program_argstring,
    );
    if !(opt.warc_user_headers).is_null() {
        let mut i: libc::c_int = 0;
        i = 0 as libc::c_int;
        while !(*(opt.warc_user_headers).offset(i as isize)).is_null() {
            fprintf(
                warc_tmp,
                b"%s\r\n\0" as *const u8 as *const libc::c_char,
                *(opt.warc_user_headers).offset(i as isize),
            );
            i += 1;
            i;
        }
    }
    fprintf(warc_tmp, b"\r\n\0" as *const u8 as *const libc::c_char);
    warc_write_digest_headers(warc_tmp, -(1 as libc::c_int) as libc::c_long);
    warc_write_block_from_file(warc_tmp);
    warc_write_end_record();
    if !warc_write_ok {
        logprintf(
            LOG_NOTQUIET,
            dcgettext(
                0 as *const libc::c_char,
                b"Error writing warcinfo record to WARC file.\n\0" as *const u8
                    as *const libc::c_char,
                5 as libc::c_int,
            ),
        );
    }
    fclose(warc_tmp);
    return warc_write_ok;
}
unsafe extern "C" fn warc_start_new_file(mut meta: bool) -> bool {
    let mut extension: *const libc::c_char = if opt.warc_compression_enabled
        as libc::c_int != 0
    {
        b"warc.gz\0" as *const u8 as *const libc::c_char
    } else {
        b"warc\0" as *const u8 as *const libc::c_char
    };
    let mut base_filename_length: libc::c_int = 0;
    let mut new_filename: *mut libc::c_char = 0 as *mut libc::c_char;
    if (opt.warc_filename).is_null() {
        return 0 as libc::c_int != 0;
    }
    if !warc_current_file.is_null() {
        fclose(warc_current_file);
    }
    *warc_current_warcinfo_uuid_str.as_mut_ptr() = 0 as libc::c_int as libc::c_char;
    rpl_free(warc_current_filename as *mut libc::c_void);
    warc_current_filename = 0 as *mut libc::c_char;
    warc_current_file_number += 1;
    warc_current_file_number;
    base_filename_length = strlen(opt.warc_filename) as libc::c_int;
    new_filename = xmalloc(
        (base_filename_length + 1 as libc::c_int + 5 as libc::c_int + 8 as libc::c_int
            + 1 as libc::c_int) as size_t,
    ) as *mut libc::c_char;
    warc_current_filename = new_filename;
    if meta {
        sprintf(
            new_filename,
            b"%s-meta.%s\0" as *const u8 as *const libc::c_char,
            opt.warc_filename,
            extension,
        );
    } else if opt.warc_maxsize > 0 as libc::c_int as libc::c_long {
        sprintf(
            new_filename,
            b"%s-%05d.%s\0" as *const u8 as *const libc::c_char,
            opt.warc_filename,
            warc_current_file_number,
            extension,
        );
    } else {
        sprintf(
            new_filename,
            b"%s.%s\0" as *const u8 as *const libc::c_char,
            opt.warc_filename,
            extension,
        );
    }
    logprintf(
        LOG_VERBOSE,
        dcgettext(
            0 as *const libc::c_char,
            b"Opening WARC file %s.\n\n\0" as *const u8 as *const libc::c_char,
            5 as libc::c_int,
        ),
        quote(new_filename),
    );
    warc_current_file = rpl_fopen(
        new_filename,
        b"wb+\0" as *const u8 as *const libc::c_char,
    );
    if warc_current_file.is_null() {
        logprintf(
            LOG_NOTQUIET,
            dcgettext(
                0 as *const libc::c_char,
                b"Error opening WARC file %s.\n\0" as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ),
            quote(new_filename),
        );
        return 0 as libc::c_int != 0;
    }
    if !warc_write_warcinfo_record(new_filename) {
        return 0 as libc::c_int != 0;
    }
    if !warc_manifest_fp.is_null() {
        fprintf(
            warc_manifest_fp,
            b"%s\n\0" as *const u8 as *const libc::c_char,
            warc_current_warcinfo_uuid_str.as_mut_ptr(),
        );
    }
    return 1 as libc::c_int != 0;
}
unsafe extern "C" fn warc_start_cdx_file() -> bool {
    let mut cdx_filename: *mut libc::c_char = aprintf(
        b"%s.cdx\0" as *const u8 as *const libc::c_char,
        opt.warc_filename,
    );
    warc_current_cdx_file = rpl_fopen(
        cdx_filename,
        b"a+\0" as *const u8 as *const libc::c_char,
    );
    rpl_free(cdx_filename as *mut libc::c_void);
    if warc_current_cdx_file.is_null() {
        return 0 as libc::c_int != 0;
    }
    fprintf(
        warc_current_cdx_file,
        b" CDX a b a m s k r M V g u\n\0" as *const u8 as *const libc::c_char,
    );
    rpl_fflush(warc_current_cdx_file);
    return 1 as libc::c_int != 0;
}
unsafe extern "C" fn warc_parse_cdx_header(
    mut lineptr: *mut libc::c_char,
    mut field_num_original_url: *mut libc::c_int,
    mut field_num_checksum: *mut libc::c_int,
    mut field_num_record_id: *mut libc::c_int,
) -> bool {
    let mut token: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut save_ptr: *mut libc::c_char = 0 as *mut libc::c_char;
    *field_num_original_url = -(1 as libc::c_int);
    *field_num_checksum = -(1 as libc::c_int);
    *field_num_record_id = -(1 as libc::c_int);
    token = strtok_r(
        lineptr,
        b" \t\r\n\0" as *const u8 as *const libc::c_char,
        &mut save_ptr,
    );
    if !token.is_null()
        && strcmp(token, b"CDX\0" as *const u8 as *const libc::c_char)
            == 0 as libc::c_int
    {
        let mut field_num: libc::c_int = 0 as libc::c_int;
        while !token.is_null() {
            token = strtok_r(
                0 as *mut libc::c_char,
                b" \t\r\n\0" as *const u8 as *const libc::c_char,
                &mut save_ptr,
            );
            if !token.is_null() {
                match *token.offset(0 as libc::c_int as isize) as libc::c_int {
                    97 => {
                        *field_num_original_url = field_num;
                    }
                    107 => {
                        *field_num_checksum = field_num;
                    }
                    117 => {
                        *field_num_record_id = field_num;
                    }
                    _ => {}
                }
            }
            field_num += 1;
            field_num;
        }
    }
    return *field_num_original_url != -(1 as libc::c_int)
        && *field_num_checksum != -(1 as libc::c_int)
        && *field_num_record_id != -(1 as libc::c_int);
}
unsafe extern "C" fn warc_process_cdx_line(
    mut lineptr: *mut libc::c_char,
    mut field_num_original_url: libc::c_int,
    mut field_num_checksum: libc::c_int,
    mut field_num_record_id: libc::c_int,
) {
    let mut original_url: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut checksum: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut record_id: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut token: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut save_ptr: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut field_num: libc::c_int = 0 as libc::c_int;
    token = strtok_r(
        lineptr,
        b" \t\r\n\0" as *const u8 as *const libc::c_char,
        &mut save_ptr,
    );
    while !token.is_null() {
        let mut val: *mut *mut libc::c_char = 0 as *mut *mut libc::c_char;
        if field_num == field_num_original_url {
            val = &mut original_url;
        } else if field_num == field_num_checksum {
            val = &mut checksum;
        } else if field_num == field_num_record_id {
            val = &mut record_id;
        } else {
            val = 0 as *mut *mut libc::c_char;
        }
        if !val.is_null() {
            *val = strdup(token);
        }
        token = strtok_r(
            0 as *mut libc::c_char,
            b" \t\r\n\0" as *const u8 as *const libc::c_char,
            &mut save_ptr,
        );
        field_num += 1;
        field_num;
    }
    if !original_url.is_null() && !checksum.is_null() && !record_id.is_null() {
        let mut checksum_l: idx_t = 0;
        let mut checksum_v: *mut libc::c_char = 0 as *mut libc::c_char;
        base32_decode_alloc_ctx(
            0 as *mut base32_decode_context,
            checksum,
            strlen(checksum) as idx_t,
            &mut checksum_v,
            &mut checksum_l,
        );
        rpl_free(checksum as *mut libc::c_void);
        checksum = 0 as *mut libc::c_char;
        if !checksum_v.is_null() && checksum_l == 20 as libc::c_int as libc::c_long {
            let mut rec: *mut warc_cdx_record = 0 as *mut warc_cdx_record;
            rec = xmalloc(::std::mem::size_of::<warc_cdx_record>() as libc::c_ulong)
                as *mut warc_cdx_record;
            (*rec).url = original_url;
            (*rec).uuid = record_id;
            memcpy(
                ((*rec).digest).as_mut_ptr() as *mut libc::c_void,
                checksum_v as *const libc::c_void,
                20 as libc::c_int as libc::c_ulong,
            );
            hash_table_put(
                warc_cdx_dedup_table,
                ((*rec).digest).as_mut_ptr() as *const libc::c_void,
                rec as *const libc::c_void,
            );
            rpl_free(checksum_v as *mut libc::c_void);
            checksum_v = 0 as *mut libc::c_char;
        } else {
            rpl_free(original_url as *mut libc::c_void);
            original_url = 0 as *mut libc::c_char;
            rpl_free(checksum_v as *mut libc::c_void);
            checksum_v = 0 as *mut libc::c_char;
            rpl_free(record_id as *mut libc::c_void);
            record_id = 0 as *mut libc::c_char;
        }
    } else {
        rpl_free(checksum as *mut libc::c_void);
        checksum = 0 as *mut libc::c_char;
        rpl_free(original_url as *mut libc::c_void);
        original_url = 0 as *mut libc::c_char;
        rpl_free(record_id as *mut libc::c_void);
        record_id = 0 as *mut libc::c_char;
    };
}
unsafe extern "C" fn warc_load_cdx_dedup_file() -> bool {
    let mut f: *mut FILE = 0 as *mut FILE;
    let mut lineptr: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut n: size_t = 0 as libc::c_int as size_t;
    let mut line_length: ssize_t = 0;
    let mut field_num_original_url: libc::c_int = -(1 as libc::c_int);
    let mut field_num_checksum: libc::c_int = -(1 as libc::c_int);
    let mut field_num_record_id: libc::c_int = -(1 as libc::c_int);
    f = rpl_fopen(
        opt.warc_cdx_dedup_filename,
        b"r\0" as *const u8 as *const libc::c_char,
    );
    if f.is_null() {
        return 0 as libc::c_int != 0;
    }
    line_length = getline(&mut lineptr, &mut n, f);
    if line_length != -(1 as libc::c_int) as libc::c_long {
        warc_parse_cdx_header(
            lineptr,
            &mut field_num_original_url,
            &mut field_num_checksum,
            &mut field_num_record_id,
        );
    }
    if field_num_original_url == -(1 as libc::c_int)
        || field_num_checksum == -(1 as libc::c_int)
        || field_num_record_id == -(1 as libc::c_int)
    {
        if field_num_original_url == -(1 as libc::c_int) {
            logprintf(
                LOG_NOTQUIET,
                dcgettext(
                    0 as *const libc::c_char,
                    b"CDX file does not list original urls. (Missing column 'a'.)\n\0"
                        as *const u8 as *const libc::c_char,
                    5 as libc::c_int,
                ),
            );
        }
        if field_num_checksum == -(1 as libc::c_int) {
            logprintf(
                LOG_NOTQUIET,
                dcgettext(
                    0 as *const libc::c_char,
                    b"CDX file does not list checksums. (Missing column 'k'.)\n\0"
                        as *const u8 as *const libc::c_char,
                    5 as libc::c_int,
                ),
            );
        }
        if field_num_record_id == -(1 as libc::c_int) {
            logprintf(
                LOG_NOTQUIET,
                dcgettext(
                    0 as *const libc::c_char,
                    b"CDX file does not list record ids. (Missing column 'u'.)\n\0"
                        as *const u8 as *const libc::c_char,
                    5 as libc::c_int,
                ),
            );
        }
    } else {
        let mut nrecords: libc::c_int = 0;
        warc_cdx_dedup_table = hash_table_new(
            1000 as libc::c_int,
            Some(
                warc_hash_sha1_digest
                    as unsafe extern "C" fn(*const libc::c_void) -> libc::c_ulong,
            ),
            Some(
                warc_cmp_sha1_digest
                    as unsafe extern "C" fn(
                        *const libc::c_void,
                        *const libc::c_void,
                    ) -> libc::c_int,
            ),
        );
        loop {
            line_length = getline(&mut lineptr, &mut n, f);
            if line_length != -(1 as libc::c_int) as libc::c_long {
                warc_process_cdx_line(
                    lineptr,
                    field_num_original_url,
                    field_num_checksum,
                    field_num_record_id,
                );
            }
            if !(line_length != -(1 as libc::c_int) as libc::c_long) {
                break;
            }
        }
        nrecords = hash_table_count(warc_cdx_dedup_table);
        logprintf(
            LOG_VERBOSE,
            dcngettext(
                0 as *const libc::c_char,
                b"Loaded %d record from CDX.\n\n\0" as *const u8 as *const libc::c_char,
                b"Loaded %d records from CDX.\n\n\0" as *const u8 as *const libc::c_char,
                nrecords as libc::c_ulong,
                5 as libc::c_int,
            ),
            nrecords,
        );
    }
    rpl_free(lineptr as *mut libc::c_void);
    lineptr = 0 as *mut libc::c_char;
    fclose(f);
    return 1 as libc::c_int != 0;
}
unsafe extern "C" fn warc_find_duplicate_cdx_record(
    mut url: *const libc::c_char,
    mut sha1_digest_payload: *mut libc::c_char,
) -> *mut warc_cdx_record {
    let mut rec_existing: *mut warc_cdx_record = 0 as *mut warc_cdx_record;
    if warc_cdx_dedup_table.is_null() {
        return 0 as *mut warc_cdx_record;
    }
    rec_existing = hash_table_get(
        warc_cdx_dedup_table,
        sha1_digest_payload as *const libc::c_void,
    ) as *mut warc_cdx_record;
    if !rec_existing.is_null() && strcmp((*rec_existing).url, url) == 0 as libc::c_int {
        return rec_existing
    } else {
        return 0 as *mut warc_cdx_record
    };
}
pub unsafe extern "C" fn warc_init() {
    warc_write_ok = 1 as libc::c_int != 0;
    if !(opt.warc_filename).is_null() {
        if !(opt.warc_cdx_dedup_filename).is_null() {
            if !warc_load_cdx_dedup_file() {
                logprintf(
                    LOG_NOTQUIET,
                    dcgettext(
                        0 as *const libc::c_char,
                        b"Could not read CDX file %s for deduplication.\n\0" as *const u8
                            as *const libc::c_char,
                        5 as libc::c_int,
                    ),
                    quote(opt.warc_cdx_dedup_filename),
                );
                exit(WGET_EXIT_GENERIC_ERROR as libc::c_int);
            }
        }
        warc_manifest_fp = warc_tempfile();
        if warc_manifest_fp.is_null() {
            logprintf(
                LOG_NOTQUIET,
                dcgettext(
                    0 as *const libc::c_char,
                    b"Could not open temporary WARC manifest file.\n\0" as *const u8
                        as *const libc::c_char,
                    5 as libc::c_int,
                ),
            );
            exit(WGET_EXIT_GENERIC_ERROR as libc::c_int);
        }
        if opt.warc_keep_log {
            warc_log_fp = warc_tempfile();
            if warc_log_fp.is_null() {
                logprintf(
                    LOG_NOTQUIET,
                    dcgettext(
                        0 as *const libc::c_char,
                        b"Could not open temporary WARC log file.\n\0" as *const u8
                            as *const libc::c_char,
                        5 as libc::c_int,
                    ),
                );
                exit(WGET_EXIT_GENERIC_ERROR as libc::c_int);
            }
            log_set_warc_log_fp(warc_log_fp);
        }
        warc_current_file_number = -(1 as libc::c_int);
        if !warc_start_new_file(0 as libc::c_int != 0) {
            logprintf(
                LOG_NOTQUIET,
                dcgettext(
                    0 as *const libc::c_char,
                    b"Could not open WARC file.\n\0" as *const u8 as *const libc::c_char,
                    5 as libc::c_int,
                ),
            );
            exit(WGET_EXIT_GENERIC_ERROR as libc::c_int);
        }
        if opt.warc_cdx_enabled {
            if !warc_start_cdx_file() {
                logprintf(
                    LOG_NOTQUIET,
                    dcgettext(
                        0 as *const libc::c_char,
                        b"Could not open CDX file for output.\n\0" as *const u8
                            as *const libc::c_char,
                        5 as libc::c_int,
                    ),
                );
                exit(WGET_EXIT_GENERIC_ERROR as libc::c_int);
            }
        }
    }
}
unsafe extern "C" fn warc_write_metadata() {
    let mut manifest_uuid: [libc::c_char; 48] = [0; 48];
    let mut warc_tmp_fp: *mut FILE = 0 as *mut FILE;
    if opt.warc_maxsize > 0 as libc::c_int as libc::c_long {
        warc_start_new_file(1 as libc::c_int != 0);
    }
    warc_uuid_str(
        manifest_uuid.as_mut_ptr(),
        ::std::mem::size_of::<[libc::c_char; 48]>() as libc::c_ulong,
    );
    rpl_fflush(warc_manifest_fp);
    warc_write_metadata_record(
        manifest_uuid.as_mut_ptr(),
        b"metadata://gnu.org/software/wget/warc/MANIFEST.txt\0" as *const u8
            as *const libc::c_char,
        0 as *const libc::c_char,
        0 as *const libc::c_char,
        0 as *mut ip_address,
        b"text/plain\0" as *const u8 as *const libc::c_char,
        warc_manifest_fp,
        -(1 as libc::c_int) as off_t,
    );
    warc_tmp_fp = warc_tempfile();
    if warc_tmp_fp.is_null() {
        logprintf(
            LOG_NOTQUIET,
            dcgettext(
                0 as *const libc::c_char,
                b"Could not open temporary WARC file.\n\0" as *const u8
                    as *const libc::c_char,
                5 as libc::c_int,
            ),
        );
        exit(WGET_EXIT_GENERIC_ERROR as libc::c_int);
    }
    rpl_fflush(warc_tmp_fp);
    fprintf(
        warc_tmp_fp,
        b"%s\n\0" as *const u8 as *const libc::c_char,
        program_argstring,
    );
    warc_write_resource_record(
        0 as *const libc::c_char,
        b"metadata://gnu.org/software/wget/warc/wget_arguments.txt\0" as *const u8
            as *const libc::c_char,
        0 as *const libc::c_char,
        manifest_uuid.as_mut_ptr(),
        0 as *const ip_address,
        b"text/plain\0" as *const u8 as *const libc::c_char,
        warc_tmp_fp,
        -(1 as libc::c_int) as off_t,
    );
    if !warc_log_fp.is_null() {
        warc_write_resource_record(
            0 as *const libc::c_char,
            b"metadata://gnu.org/software/wget/warc/wget.log\0" as *const u8
                as *const libc::c_char,
            0 as *const libc::c_char,
            manifest_uuid.as_mut_ptr(),
            0 as *const ip_address,
            b"text/plain\0" as *const u8 as *const libc::c_char,
            warc_log_fp,
            -(1 as libc::c_int) as off_t,
        );
        warc_log_fp = 0 as *mut FILE;
        log_set_warc_log_fp(0 as *mut FILE);
    }
}
pub unsafe extern "C" fn warc_close() {
    if !warc_current_file.is_null() {
        warc_write_metadata();
        *warc_current_warcinfo_uuid_str.as_mut_ptr() = 0 as libc::c_int as libc::c_char;
        fclose(warc_current_file);
        warc_current_file = 0 as *mut FILE;
    }
    if !warc_current_cdx_file.is_null() {
        fclose(warc_current_cdx_file);
        warc_current_cdx_file = 0 as *mut FILE;
    }
    if !warc_log_fp.is_null() {
        fclose(warc_log_fp);
        log_set_warc_log_fp(0 as *mut FILE);
    }
}
pub unsafe extern "C" fn warc_tempfile() -> *mut FILE {
    let mut filename: [libc::c_char; 100] = [0; 100];
    let mut fd: libc::c_int = 0;
    if path_search(
        filename.as_mut_ptr(),
        100 as libc::c_int as size_t,
        opt.warc_tempdir,
        b"wget\0" as *const u8 as *const libc::c_char,
        1 as libc::c_int != 0,
    ) == -(1 as libc::c_int)
    {
        return 0 as *mut FILE;
    }
    fd = mkostemp(filename.as_mut_ptr(), 0 as libc::c_int);
    if fd < 0 as libc::c_int {
        return 0 as *mut FILE;
    }
    if unlink(filename.as_mut_ptr()) < 0 as libc::c_int {
        close(fd);
        return 0 as *mut FILE;
    }
    return fdopen(fd, b"wb+\0" as *const u8 as *const libc::c_char);
}
pub unsafe extern "C" fn warc_write_request_record(
    mut url: *const libc::c_char,
    mut timestamp_str: *const libc::c_char,
    mut record_uuid: *const libc::c_char,
    mut ip: *const ip_address,
    mut body: *mut FILE,
    mut payload_offset: off_t,
) -> bool {
    warc_write_start_record();
    warc_write_header(
        b"WARC-Type\0" as *const u8 as *const libc::c_char,
        b"request\0" as *const u8 as *const libc::c_char,
    );
    warc_write_header_uri(b"WARC-Target-URI\0" as *const u8 as *const libc::c_char, url);
    warc_write_header(
        b"Content-Type\0" as *const u8 as *const libc::c_char,
        b"application/http;msgtype=request\0" as *const u8 as *const libc::c_char,
    );
    warc_write_date_header(timestamp_str);
    warc_write_header(
        b"WARC-Record-ID\0" as *const u8 as *const libc::c_char,
        record_uuid,
    );
    warc_write_ip_header(ip);
    warc_write_header(
        b"WARC-Warcinfo-ID\0" as *const u8 as *const libc::c_char,
        warc_current_warcinfo_uuid_str.as_mut_ptr(),
    );
    warc_write_digest_headers(body, payload_offset);
    warc_write_block_from_file(body);
    warc_write_end_record();
    fclose(body);
    return warc_write_ok;
}
unsafe extern "C" fn warc_write_cdx_record(
    mut url: *const libc::c_char,
    mut timestamp_str: *const libc::c_char,
    mut mime_type: *const libc::c_char,
    mut response_code: libc::c_int,
    mut payload_digest: *const libc::c_char,
    mut redirect_location: *const libc::c_char,
    mut offset: off_t,
    mut warc_filename: *const libc::c_char,
    mut response_uuid: *const libc::c_char,
) -> bool {
    let mut timestamp_str_cdx: [libc::c_char; 15] = [0; 15];
    let mut offset_string: [libc::c_char; 21] = [0; 21];
    let mut checksum: *const libc::c_char = 0 as *const libc::c_char;
    let mut tmp_location: *mut libc::c_char = 0 as *mut libc::c_char;
    memcpy(
        timestamp_str_cdx.as_mut_ptr() as *mut libc::c_void,
        timestamp_str as *const libc::c_void,
        4 as libc::c_int as libc::c_ulong,
    );
    memcpy(
        timestamp_str_cdx.as_mut_ptr().offset(4 as libc::c_int as isize)
            as *mut libc::c_void,
        timestamp_str.offset(5 as libc::c_int as isize) as *const libc::c_void,
        2 as libc::c_int as libc::c_ulong,
    );
    memcpy(
        timestamp_str_cdx.as_mut_ptr().offset(6 as libc::c_int as isize)
            as *mut libc::c_void,
        timestamp_str.offset(8 as libc::c_int as isize) as *const libc::c_void,
        2 as libc::c_int as libc::c_ulong,
    );
    memcpy(
        timestamp_str_cdx.as_mut_ptr().offset(8 as libc::c_int as isize)
            as *mut libc::c_void,
        timestamp_str.offset(11 as libc::c_int as isize) as *const libc::c_void,
        2 as libc::c_int as libc::c_ulong,
    );
    memcpy(
        timestamp_str_cdx.as_mut_ptr().offset(10 as libc::c_int as isize)
            as *mut libc::c_void,
        timestamp_str.offset(14 as libc::c_int as isize) as *const libc::c_void,
        2 as libc::c_int as libc::c_ulong,
    );
    memcpy(
        timestamp_str_cdx.as_mut_ptr().offset(12 as libc::c_int as isize)
            as *mut libc::c_void,
        timestamp_str.offset(17 as libc::c_int as isize) as *const libc::c_void,
        2 as libc::c_int as libc::c_ulong,
    );
    timestamp_str_cdx[14 as libc::c_int as usize] = '\0' as i32 as libc::c_char;
    if !payload_digest.is_null() {
        checksum = payload_digest.offset(5 as libc::c_int as isize);
    } else {
        checksum = b"-\0" as *const u8 as *const libc::c_char;
    }
    if mime_type.is_null() || strlen(mime_type) == 0 as libc::c_int as libc::c_ulong {
        mime_type = b"-\0" as *const u8 as *const libc::c_char;
    }
    if redirect_location.is_null()
        || strlen(redirect_location) == 0 as libc::c_int as libc::c_ulong
    {
        tmp_location = strdup(b"-\0" as *const u8 as *const libc::c_char);
    } else {
        tmp_location = url_escape(redirect_location);
    }
    number_to_string(offset_string.as_mut_ptr(), offset);
    fprintf(
        warc_current_cdx_file,
        b"%s %s %s %s %d %s %s - %s %s %s\n\0" as *const u8 as *const libc::c_char,
        url,
        timestamp_str_cdx.as_mut_ptr(),
        url,
        mime_type,
        response_code,
        checksum,
        tmp_location,
        offset_string.as_mut_ptr(),
        warc_current_filename,
        response_uuid,
    );
    rpl_fflush(warc_current_cdx_file);
    rpl_free(tmp_location as *mut libc::c_void);
    return 1 as libc::c_int != 0;
}
unsafe extern "C" fn warc_write_revisit_record(
    mut url: *const libc::c_char,
    mut timestamp_str: *const libc::c_char,
    mut concurrent_to_uuid: *const libc::c_char,
    mut payload_digest: *const libc::c_char,
    mut refers_to: *const libc::c_char,
    mut ip: *const ip_address,
    mut body: *mut FILE,
) -> bool {
    let mut revisit_uuid: [libc::c_char; 48] = [0; 48];
    let mut block_digest: [libc::c_char; 38] = [0; 38];
    let mut sha1_res_block: [libc::c_char; 20] = [0; 20];
    warc_uuid_str(
        revisit_uuid.as_mut_ptr(),
        ::std::mem::size_of::<[libc::c_char; 48]>() as libc::c_ulong,
    );
    sha1_stream(body, sha1_res_block.as_mut_ptr() as *mut libc::c_void);
    warc_base32_sha1_digest(
        sha1_res_block.as_mut_ptr(),
        block_digest.as_mut_ptr(),
        ::std::mem::size_of::<[libc::c_char; 38]>() as libc::c_ulong,
    );
    warc_write_start_record();
    warc_write_header(
        b"WARC-Type\0" as *const u8 as *const libc::c_char,
        b"revisit\0" as *const u8 as *const libc::c_char,
    );
    warc_write_header(
        b"WARC-Record-ID\0" as *const u8 as *const libc::c_char,
        revisit_uuid.as_mut_ptr(),
    );
    warc_write_header(
        b"WARC-Warcinfo-ID\0" as *const u8 as *const libc::c_char,
        warc_current_warcinfo_uuid_str.as_mut_ptr(),
    );
    warc_write_header(
        b"WARC-Concurrent-To\0" as *const u8 as *const libc::c_char,
        concurrent_to_uuid,
    );
    warc_write_header(
        b"WARC-Refers-To\0" as *const u8 as *const libc::c_char,
        refers_to,
    );
    warc_write_header(
        b"WARC-Profile\0" as *const u8 as *const libc::c_char,
        b"http://netpreserve.org/warc/1.0/revisit/identical-payload-digest\0"
            as *const u8 as *const libc::c_char,
    );
    warc_write_header(
        b"WARC-Truncated\0" as *const u8 as *const libc::c_char,
        b"length\0" as *const u8 as *const libc::c_char,
    );
    warc_write_header_uri(b"WARC-Target-URI\0" as *const u8 as *const libc::c_char, url);
    warc_write_date_header(timestamp_str);
    warc_write_ip_header(ip);
    warc_write_header(
        b"Content-Type\0" as *const u8 as *const libc::c_char,
        b"application/http;msgtype=response\0" as *const u8 as *const libc::c_char,
    );
    warc_write_header(
        b"WARC-Block-Digest\0" as *const u8 as *const libc::c_char,
        block_digest.as_mut_ptr(),
    );
    warc_write_header(
        b"WARC-Payload-Digest\0" as *const u8 as *const libc::c_char,
        payload_digest,
    );
    warc_write_block_from_file(body);
    warc_write_end_record();
    fclose(body);
    return warc_write_ok;
}
pub unsafe extern "C" fn warc_write_response_record(
    mut url: *const libc::c_char,
    mut timestamp_str: *const libc::c_char,
    mut concurrent_to_uuid: *const libc::c_char,
    mut ip: *const ip_address,
    mut body: *mut FILE,
    mut payload_offset: off_t,
    mut mime_type: *const libc::c_char,
    mut response_code: libc::c_int,
    mut redirect_location: *const libc::c_char,
) -> bool {
    let mut block_digest: [libc::c_char; 38] = [0; 38];
    let mut payload_digest: [libc::c_char; 38] = [0; 38];
    let mut sha1_res_block: [libc::c_char; 20] = [0; 20];
    let mut sha1_res_payload: [libc::c_char; 20] = [0; 20];
    let mut response_uuid: [libc::c_char; 48] = [0; 48];
    let mut offset: off_t = 0;
    if opt.warc_digests_enabled {
        rewind(body);
        if warc_sha1_stream_with_payload(
            body,
            sha1_res_block.as_mut_ptr() as *mut libc::c_void,
            sha1_res_payload.as_mut_ptr() as *mut libc::c_void,
            payload_offset,
        ) == 0 as libc::c_int
        {
            let mut rec_existing: *mut warc_cdx_record = 0 as *mut warc_cdx_record;
            rec_existing = warc_find_duplicate_cdx_record(
                url,
                sha1_res_payload.as_mut_ptr(),
            );
            if !rec_existing.is_null() {
                let mut result: bool = false;
                logprintf(
                    LOG_VERBOSE,
                    dcgettext(
                        0 as *const libc::c_char,
                        b"Found exact match in CDX file. Saving revisit record to WARC.\n\0"
                            as *const u8 as *const libc::c_char,
                        5 as libc::c_int,
                    ),
                );
                if payload_offset > 0 as libc::c_int as libc::c_long {
                    if ftruncate(fileno(body), payload_offset) == -(1 as libc::c_int) {
                        return 0 as libc::c_int != 0;
                    }
                }
                warc_base32_sha1_digest(
                    sha1_res_payload.as_mut_ptr(),
                    payload_digest.as_mut_ptr(),
                    ::std::mem::size_of::<[libc::c_char; 38]>() as libc::c_ulong,
                );
                result = warc_write_revisit_record(
                    url,
                    timestamp_str,
                    concurrent_to_uuid,
                    payload_digest.as_mut_ptr(),
                    (*rec_existing).uuid,
                    ip,
                    body,
                );
                return result;
            }
            warc_base32_sha1_digest(
                sha1_res_block.as_mut_ptr(),
                block_digest.as_mut_ptr(),
                ::std::mem::size_of::<[libc::c_char; 38]>() as libc::c_ulong,
            );
            warc_base32_sha1_digest(
                sha1_res_payload.as_mut_ptr(),
                payload_digest.as_mut_ptr(),
                ::std::mem::size_of::<[libc::c_char; 38]>() as libc::c_ulong,
            );
        }
    }
    warc_uuid_str(
        response_uuid.as_mut_ptr(),
        ::std::mem::size_of::<[libc::c_char; 48]>() as libc::c_ulong,
    );
    rpl_fseeko(warc_current_file, 0 as libc::c_long, 2 as libc::c_int);
    offset = ftello(warc_current_file);
    warc_write_start_record();
    warc_write_header(
        b"WARC-Type\0" as *const u8 as *const libc::c_char,
        b"response\0" as *const u8 as *const libc::c_char,
    );
    warc_write_header(
        b"WARC-Record-ID\0" as *const u8 as *const libc::c_char,
        response_uuid.as_mut_ptr(),
    );
    warc_write_header(
        b"WARC-Warcinfo-ID\0" as *const u8 as *const libc::c_char,
        warc_current_warcinfo_uuid_str.as_mut_ptr(),
    );
    warc_write_header(
        b"WARC-Concurrent-To\0" as *const u8 as *const libc::c_char,
        concurrent_to_uuid,
    );
    warc_write_header_uri(b"WARC-Target-URI\0" as *const u8 as *const libc::c_char, url);
    warc_write_date_header(timestamp_str);
    warc_write_ip_header(ip);
    warc_write_header(
        b"WARC-Block-Digest\0" as *const u8 as *const libc::c_char,
        block_digest.as_mut_ptr(),
    );
    warc_write_header(
        b"WARC-Payload-Digest\0" as *const u8 as *const libc::c_char,
        payload_digest.as_mut_ptr(),
    );
    warc_write_header(
        b"Content-Type\0" as *const u8 as *const libc::c_char,
        b"application/http;msgtype=response\0" as *const u8 as *const libc::c_char,
    );
    warc_write_block_from_file(body);
    warc_write_end_record();
    fclose(body);
    if warc_write_ok as libc::c_int != 0 && opt.warc_cdx_enabled as libc::c_int != 0 {
        warc_write_cdx_record(
            url,
            timestamp_str,
            mime_type,
            response_code,
            payload_digest.as_mut_ptr(),
            redirect_location,
            offset,
            warc_current_filename,
            response_uuid.as_mut_ptr(),
        );
    }
    return warc_write_ok;
}
unsafe extern "C" fn warc_write_record(
    mut record_type: *const libc::c_char,
    mut resource_uuid: *const libc::c_char,
    mut url: *const libc::c_char,
    mut timestamp_str: *const libc::c_char,
    mut concurrent_to_uuid: *const libc::c_char,
    mut ip: *const ip_address,
    mut content_type: *const libc::c_char,
    mut body: *mut FILE,
    mut payload_offset: off_t,
) -> bool {
    let mut uuid_buf: [libc::c_char; 48] = [0; 48];
    if resource_uuid.is_null() {
        warc_uuid_str(
            uuid_buf.as_mut_ptr(),
            ::std::mem::size_of::<[libc::c_char; 48]>() as libc::c_ulong,
        );
        resource_uuid = uuid_buf.as_mut_ptr();
    }
    if content_type.is_null() {
        content_type = b"application/octet-stream\0" as *const u8 as *const libc::c_char;
    }
    warc_write_start_record();
    warc_write_header(b"WARC-Type\0" as *const u8 as *const libc::c_char, record_type);
    warc_write_header(
        b"WARC-Record-ID\0" as *const u8 as *const libc::c_char,
        resource_uuid,
    );
    warc_write_header(
        b"WARC-Warcinfo-ID\0" as *const u8 as *const libc::c_char,
        warc_current_warcinfo_uuid_str.as_mut_ptr(),
    );
    warc_write_header(
        b"WARC-Concurrent-To\0" as *const u8 as *const libc::c_char,
        concurrent_to_uuid,
    );
    warc_write_header_uri(b"WARC-Target-URI\0" as *const u8 as *const libc::c_char, url);
    warc_write_date_header(timestamp_str);
    warc_write_ip_header(ip);
    warc_write_digest_headers(body, payload_offset);
    warc_write_header(
        b"Content-Type\0" as *const u8 as *const libc::c_char,
        content_type,
    );
    warc_write_block_from_file(body);
    warc_write_end_record();
    fclose(body);
    return warc_write_ok;
}
pub unsafe extern "C" fn warc_write_resource_record(
    mut resource_uuid: *const libc::c_char,
    mut url: *const libc::c_char,
    mut timestamp_str: *const libc::c_char,
    mut concurrent_to_uuid: *const libc::c_char,
    mut ip: *const ip_address,
    mut content_type: *const libc::c_char,
    mut body: *mut FILE,
    mut payload_offset: off_t,
) -> bool {
    return warc_write_record(
        b"resource\0" as *const u8 as *const libc::c_char,
        resource_uuid,
        url,
        timestamp_str,
        concurrent_to_uuid,
        ip,
        content_type,
        body,
        payload_offset,
    );
}
pub unsafe extern "C" fn warc_write_metadata_record(
    mut record_uuid: *const libc::c_char,
    mut url: *const libc::c_char,
    mut timestamp_str: *const libc::c_char,
    mut concurrent_to_uuid: *const libc::c_char,
    mut ip: *mut ip_address,
    mut content_type: *const libc::c_char,
    mut body: *mut FILE,
    mut payload_offset: off_t,
) -> bool {
    return warc_write_record(
        b"metadata\0" as *const u8 as *const libc::c_char,
        record_uuid,
        url,
        timestamp_str,
        concurrent_to_uuid,
        ip,
        content_type,
        body,
        payload_offset,
    );
}
