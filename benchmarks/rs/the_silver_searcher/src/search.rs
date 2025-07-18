use ::libc;
extern "C" {
    pub type real_pcre;
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
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
    fn __errno_location() -> *mut libc::c_int;
    fn open(__file: *const libc::c_char, __oflag: libc::c_int, _: ...) -> libc::c_int;
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    fn free(__ptr: *mut libc::c_void);
    fn exit(_: libc::c_int) -> !;
    fn pcre_exec(
        _: *const pcre,
        _: *const pcre_extra,
        _: *const libc::c_char,
        _: libc::c_int,
        _: libc::c_int,
        _: libc::c_int,
        _: *mut libc::c_int,
        _: libc::c_int,
    ) -> libc::c_int;
    fn fclose(__stream: *mut FILE) -> libc::c_int;
    fn fdopen(__fd: libc::c_int, __modes: *const libc::c_char) -> *mut FILE;
    fn fprintf(_: *mut FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn __getdelim(
        __lineptr: *mut *mut libc::c_char,
        __n: *mut size_t,
        __delimiter: libc::c_int,
        __stream: *mut FILE,
    ) -> __ssize_t;
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    fn strerror(_: libc::c_int) -> *mut libc::c_char;
    fn mmap(
        __addr: *mut libc::c_void,
        __len: size_t,
        __prot: libc::c_int,
        __flags: libc::c_int,
        __fd: libc::c_int,
        __offset: __off_t,
    ) -> *mut libc::c_void;
    fn munmap(__addr: *mut libc::c_void, __len: size_t) -> libc::c_int;
    fn madvise(
        __addr: *mut libc::c_void,
        __len: size_t,
        __advice: libc::c_int,
    ) -> libc::c_int;
    fn __xstat(
        __ver: libc::c_int,
        __filename: *const libc::c_char,
        __stat_buf: *mut stat,
    ) -> libc::c_int;
    fn __fxstat(
        __ver: libc::c_int,
        __fildes: libc::c_int,
        __stat_buf: *mut stat,
    ) -> libc::c_int;
    fn __lxstat(
        __ver: libc::c_int,
        __filename: *const libc::c_char,
        __stat_buf: *mut stat,
    ) -> libc::c_int;
    fn close(__fd: libc::c_int) -> libc::c_int;
    fn read(__fd: libc::c_int, __buf: *mut libc::c_void, __nbytes: size_t) -> ssize_t;
    fn pthread_exit(__retval: *mut libc::c_void) -> !;
    fn pthread_mutex_lock(__mutex: *mut pthread_mutex_t) -> libc::c_int;
    fn pthread_mutex_unlock(__mutex: *mut pthread_mutex_t) -> libc::c_int;
    fn pthread_cond_signal(__cond: *mut pthread_cond_t) -> libc::c_int;
    fn pthread_cond_wait(
        __cond: *mut pthread_cond_t,
        __mutex: *mut pthread_mutex_t,
    ) -> libc::c_int;
    static mut print_mtx: pthread_mutex_t;
    fn log_debug(fmt: *const libc::c_char, _: ...);
    fn log_err(fmt: *const libc::c_char, _: ...);
    static mut opts: cli_options;
    fn is_zipped(buf: *const libc::c_void, buf_len: libc::c_int) -> ag_compression_type;
    fn decompress_open(
        fd: libc::c_int,
        mode: *const libc::c_char,
        ctype: ag_compression_type,
    ) -> *mut FILE;
    static mut ignore_pattern_files: [*const libc::c_char; 0];
    fn init_ignore(
        parent: *mut ignores,
        dirname: *const libc::c_char,
        dirname_len: size_t,
    ) -> *mut ignores;
    fn cleanup_ignore(ig: *mut ignores);
    fn load_ignore_patterns(ig: *mut ignores, path: *const libc::c_char);
    fn filename_filter(
        path: *const libc::c_char,
        dir: *const dirent,
        baton: *mut libc::c_void,
    ) -> libc::c_int;
    static mut out_fd: *mut FILE;
    fn ag_malloc(size: size_t) -> *mut libc::c_void;
    static mut stats: ag_stats;
    fn ag_min(a: size_t, b: size_t) -> size_t;
    fn boyer_moore_strnstr(
        s: *const libc::c_char,
        find: *const libc::c_char,
        s_len: size_t,
        f_len: size_t,
        alpha_skip_lookup_0: *const size_t,
        find_skip_lookup_0: *const size_t,
        case_insensitive: libc::c_int,
    ) -> *const libc::c_char;
    fn hash_strnstr(
        s: *const libc::c_char,
        find: *const libc::c_char,
        s_len: size_t,
        f_len: size_t,
        h_table_0: *mut uint8_t,
        case_sensitive: libc::c_int,
    ) -> *const libc::c_char;
    fn invert_matches(
        buf: *const libc::c_char,
        buf_len: size_t,
        matches: *mut match_t,
        matches_len: size_t,
    ) -> size_t;
    fn realloc_matches(
        matches: *mut *mut match_t,
        matches_size: *mut size_t,
        matches_len: size_t,
    );
    fn is_binary(buf: *const libc::c_void, buf_len: size_t) -> libc::c_int;
    fn is_wordchar(ch: libc::c_char) -> libc::c_int;
    fn is_directory(path: *const libc::c_char, d: *const dirent) -> libc::c_int;
    fn is_symlink(path: *const libc::c_char, d: *const dirent) -> libc::c_int;
    fn die(fmt: *const libc::c_char, _: ...);
    fn ag_asprintf(ret: *mut *mut libc::c_char, fmt: *const libc::c_char, _: ...);
    fn buf_getline(
        line: *mut *const libc::c_char,
        buf: *const libc::c_char,
        buf_len: size_t,
        buf_offset: size_t,
    ) -> ssize_t;
    fn print_init_context();
    fn print_cleanup_context();
    fn print_context_append(line: *const libc::c_char, len: size_t);
    fn print_trailing_context(
        path: *const libc::c_char,
        buf: *const libc::c_char,
        n: size_t,
    );
    fn print_path(path: *const libc::c_char, sep: libc::c_char);
    fn print_path_count(path: *const libc::c_char, sep: libc::c_char, count: size_t);
    fn print_binary_file_matches(path: *const libc::c_char);
    fn print_file_matches(
        path: *const libc::c_char,
        buf: *const libc::c_char,
        buf_len: size_t,
        matches: *const match_t,
        matches_len: size_t,
    );
    fn ag_scandir(
        dirname: *const libc::c_char,
        namelist: *mut *mut *mut dirent,
        filter: filter_fp,
        baton: *mut libc::c_void,
    ) -> libc::c_int;
}
pub type __uint8_t = libc::c_uchar;
pub type __uint16_t = libc::c_ushort;
pub type __uint32_t = libc::c_uint;
pub type __dev_t = libc::c_ulong;
pub type __uid_t = libc::c_uint;
pub type __gid_t = libc::c_uint;
pub type __ino_t = libc::c_ulong;
pub type __mode_t = libc::c_uint;
pub type __nlink_t = libc::c_ulong;
pub type __off_t = libc::c_long;
pub type __off64_t = libc::c_long;
pub type __time_t = libc::c_long;
pub type __suseconds_t = libc::c_long;
pub type __blksize_t = libc::c_long;
pub type __blkcnt_t = libc::c_long;
pub type __ssize_t = libc::c_long;
pub type __syscall_slong_t = libc::c_long;
pub type ino_t = __ino_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct dirent {
    pub d_ino: __ino_t,
    pub d_off: __off_t,
    pub d_reclen: libc::c_ushort,
    pub d_type: libc::c_uchar,
    pub d_name: [libc::c_char; 256],
}
pub type size_t = libc::c_ulong;
pub type off_t = __off_t;
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
pub type dev_t = __dev_t;
pub type ssize_t = __ssize_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct timeval {
    pub tv_sec: __time_t,
    pub tv_usec: __suseconds_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __pthread_internal_list {
    pub __prev: *mut __pthread_internal_list,
    pub __next: *mut __pthread_internal_list,
}
pub type __pthread_list_t = __pthread_internal_list;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __pthread_mutex_s {
    pub __lock: libc::c_int,
    pub __count: libc::c_uint,
    pub __owner: libc::c_int,
    pub __nusers: libc::c_uint,
    pub __kind: libc::c_int,
    pub __spins: libc::c_short,
    pub __elision: libc::c_short,
    pub __list: __pthread_list_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __pthread_cond_s {
    pub c2rust_unnamed: C2RustUnnamed_1,
    pub c2rust_unnamed_0: C2RustUnnamed,
    pub __g_refs: [libc::c_uint; 2],
    pub __g_size: [libc::c_uint; 2],
    pub __g1_orig_size: libc::c_uint,
    pub __wrefs: libc::c_uint,
    pub __g_signals: [libc::c_uint; 2],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed {
    pub __g1_start: libc::c_ulonglong,
    pub __g1_start32: C2RustUnnamed_0,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_0 {
    pub __low: libc::c_uint,
    pub __high: libc::c_uint,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_1 {
    pub __wseq: libc::c_ulonglong,
    pub __wseq32: C2RustUnnamed_2,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_2 {
    pub __low: libc::c_uint,
    pub __high: libc::c_uint,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union pthread_mutex_t {
    pub __data: __pthread_mutex_s,
    pub __size: [libc::c_char; 40],
    pub __align: libc::c_long,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union pthread_cond_t {
    pub __data: __pthread_cond_s,
    pub __size: [libc::c_char; 48],
    pub __align: libc::c_longlong,
}
pub type pcre = real_pcre;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct pcre_extra {
    pub flags: libc::c_ulong,
    pub study_data: *mut libc::c_void,
    pub match_limit: libc::c_ulong,
    pub callout_data: *mut libc::c_void,
    pub tables: *const libc::c_uchar,
    pub match_limit_recursion: libc::c_ulong,
    pub mark: *mut *mut libc::c_uchar,
    pub executable_jit: *mut libc::c_void,
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
pub type C2RustUnnamed_3 = libc::c_uint;
pub const PTHREAD_MUTEX_FAST_NP: C2RustUnnamed_3 = 0;
pub const PTHREAD_MUTEX_DEFAULT: C2RustUnnamed_3 = 0;
pub const PTHREAD_MUTEX_ERRORCHECK: C2RustUnnamed_3 = 2;
pub const PTHREAD_MUTEX_RECURSIVE: C2RustUnnamed_3 = 1;
pub const PTHREAD_MUTEX_NORMAL: C2RustUnnamed_3 = 0;
pub const PTHREAD_MUTEX_ADAPTIVE_NP: C2RustUnnamed_3 = 3;
pub const PTHREAD_MUTEX_ERRORCHECK_NP: C2RustUnnamed_3 = 2;
pub const PTHREAD_MUTEX_RECURSIVE_NP: C2RustUnnamed_3 = 1;
pub const PTHREAD_MUTEX_TIMED_NP: C2RustUnnamed_3 = 0;
pub type case_behavior = libc::c_uint;
pub const CASE_SENSITIVE_RETRY_INSENSITIVE: case_behavior = 4;
pub const CASE_SMART: case_behavior = 3;
pub const CASE_INSENSITIVE: case_behavior = 2;
pub const CASE_SENSITIVE: case_behavior = 1;
pub const CASE_DEFAULT: case_behavior = 0;
pub type path_print_behavior = libc::c_uint;
pub const PATH_PRINT_NOTHING: path_print_behavior = 4;
pub const PATH_PRINT_EACH_LINE: path_print_behavior = 3;
pub const PATH_PRINT_TOP: path_print_behavior = 2;
pub const PATH_PRINT_DEFAULT_EACH_LINE: path_print_behavior = 1;
pub const PATH_PRINT_DEFAULT: path_print_behavior = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct cli_options {
    pub ackmate: libc::c_int,
    pub ackmate_dir_filter: *mut pcre,
    pub ackmate_dir_filter_extra: *mut pcre_extra,
    pub after: size_t,
    pub before: size_t,
    pub casing: case_behavior,
    pub file_search_string: *const libc::c_char,
    pub match_files: libc::c_int,
    pub file_search_regex: *mut pcre,
    pub file_search_regex_extra: *mut pcre_extra,
    pub color: libc::c_int,
    pub color_line_number: *mut libc::c_char,
    pub color_match: *mut libc::c_char,
    pub color_path: *mut libc::c_char,
    pub color_win_ansi: libc::c_int,
    pub column: libc::c_int,
    pub context: libc::c_int,
    pub follow_symlinks: libc::c_int,
    pub invert_match: libc::c_int,
    pub literal: libc::c_int,
    pub literal_starts_wordchar: libc::c_int,
    pub literal_ends_wordchar: libc::c_int,
    pub max_matches_per_file: size_t,
    pub max_search_depth: libc::c_int,
    pub mmap: libc::c_int,
    pub multiline: libc::c_int,
    pub one_dev: libc::c_int,
    pub only_matching: libc::c_int,
    pub path_sep: libc::c_char,
    pub path_to_ignore: libc::c_int,
    pub print_break: libc::c_int,
    pub print_count: libc::c_int,
    pub print_filename_only: libc::c_int,
    pub print_nonmatching_files: libc::c_int,
    pub print_path: libc::c_int,
    pub print_all_paths: libc::c_int,
    pub print_line_numbers: libc::c_int,
    pub print_long_lines: libc::c_int,
    pub passthrough: libc::c_int,
    pub re: *mut pcre,
    pub re_extra: *mut pcre_extra,
    pub recurse_dirs: libc::c_int,
    pub search_all_files: libc::c_int,
    pub skip_vcs_ignores: libc::c_int,
    pub search_binary_files: libc::c_int,
    pub search_zip_files: libc::c_int,
    pub search_hidden_files: libc::c_int,
    pub search_stream: libc::c_int,
    pub stats: libc::c_int,
    pub stream_line_num: size_t,
    pub match_found: libc::c_int,
    pub stdout_inode: ino_t,
    pub query: *mut libc::c_char,
    pub query_len: libc::c_int,
    pub pager: *mut libc::c_char,
    pub paths_len: libc::c_int,
    pub parallel: libc::c_int,
    pub use_thread_affinity: libc::c_int,
    pub vimgrep: libc::c_int,
    pub width: size_t,
    pub word_regexp: libc::c_int,
    pub workers: libc::c_int,
}
pub type ag_compression_type = libc::c_uint;
pub const AG_XZ: ag_compression_type = 4;
pub const AG_ZIP: ag_compression_type = 3;
pub const AG_COMPRESS: ag_compression_type = 2;
pub const AG_GZIP: ag_compression_type = 1;
pub const AG_NO_COMPRESSION: ag_compression_type = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ignores {
    pub extensions: *mut *mut libc::c_char,
    pub extensions_len: size_t,
    pub names: *mut *mut libc::c_char,
    pub names_len: size_t,
    pub slash_names: *mut *mut libc::c_char,
    pub slash_names_len: size_t,
    pub regexes: *mut *mut libc::c_char,
    pub regexes_len: size_t,
    pub invert_regexes: *mut *mut libc::c_char,
    pub invert_regexes_len: size_t,
    pub slash_regexes: *mut *mut libc::c_char,
    pub slash_regexes_len: size_t,
    pub dirname: *const libc::c_char,
    pub dirname_len: size_t,
    pub abs_path: *mut libc::c_char,
    pub abs_path_len: size_t,
    pub parent: *mut ignores,
}
pub type uint8_t = __uint8_t;
pub type uint16_t = __uint16_t;
pub type uint32_t = __uint32_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct match_t {
    pub start: size_t,
    pub end: size_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ag_stats {
    pub total_bytes: size_t,
    pub total_files: size_t,
    pub total_matches: size_t,
    pub total_file_matches: size_t,
    pub time_start: timeval,
    pub time_end: timeval,
}
pub type ptrdiff_t = libc::c_long;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct UT_hash_bucket {
    pub hh_head: *mut UT_hash_handle,
    pub count: libc::c_uint,
    pub expand_mult: libc::c_uint,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct UT_hash_handle {
    pub tbl: *mut UT_hash_table,
    pub prev: *mut libc::c_void,
    pub next: *mut libc::c_void,
    pub hh_prev: *mut UT_hash_handle,
    pub hh_next: *mut UT_hash_handle,
    pub key: *mut libc::c_void,
    pub keylen: libc::c_uint,
    pub hashv: libc::c_uint,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct UT_hash_table {
    pub buckets: *mut UT_hash_bucket,
    pub num_buckets: libc::c_uint,
    pub log2_num_buckets: libc::c_uint,
    pub num_items: libc::c_uint,
    pub tail: *mut UT_hash_handle,
    pub hho: ptrdiff_t,
    pub ideal_chain_maxlen: libc::c_uint,
    pub nonideal_items: libc::c_uint,
    pub ineff_expands: libc::c_uint,
    pub noexpand: libc::c_uint,
    pub signature: uint32_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct work_queue_t {
    pub path: *mut libc::c_char,
    pub next: *mut work_queue_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct dirkey_t {
    pub dev: dev_t,
    pub ino: ino_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct symdir_t {
    pub key: dirkey_t,
    pub hh: UT_hash_handle,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct scandir_baton_t {
    pub ig: *const ignores,
    pub base_path: *const libc::c_char,
    pub base_path_len: size_t,
    pub path_start: *const libc::c_char,
}
pub type filter_fp = Option::<
    unsafe extern "C" fn(
        *const libc::c_char,
        *const dirent,
        *mut libc::c_void,
    ) -> libc::c_int,
>;
#[inline]
#[linkage = "external"]
pub unsafe extern "C" fn getline(
    mut __lineptr: *mut *mut libc::c_char,
    mut __n: *mut size_t,
    mut __stream: *mut FILE,
) -> __ssize_t {
    return __getdelim(__lineptr, __n, '\n' as i32, __stream);
}
#[inline]
#[linkage = "external"]
pub unsafe extern "C" fn stat(
    mut __path: *const libc::c_char,
    mut __statbuf: *mut stat,
) -> libc::c_int {
    return __xstat(1 as libc::c_int, __path, __statbuf);
}
#[inline]
#[linkage = "external"]
pub unsafe extern "C" fn fstat(
    mut __fd: libc::c_int,
    mut __statbuf: *mut stat,
) -> libc::c_int {
    return __fxstat(1 as libc::c_int, __fd, __statbuf);
}
#[inline]
#[linkage = "external"]
pub unsafe extern "C" fn lstat(
    mut __path: *const libc::c_char,
    mut __statbuf: *mut stat,
) -> libc::c_int {
    return __lxstat(1 as libc::c_int, __path, __statbuf);
}
pub static mut alpha_skip_lookup: [size_t; 256] = [0; 256];
pub static mut find_skip_lookup: *mut size_t = 0 as *const size_t as *mut size_t;
pub static mut h_table: [uint8_t; 65536] = [0; 65536];
pub static mut work_queue: *mut work_queue_t = 0 as *const work_queue_t
    as *mut work_queue_t;
pub static mut work_queue_tail: *mut work_queue_t = 0 as *const work_queue_t
    as *mut work_queue_t;
pub static mut done_adding_files: libc::c_int = 0 as libc::c_int;
pub static mut files_ready: pthread_cond_t = pthread_cond_t {
    __data: {
        let mut init = __pthread_cond_s {
            c2rust_unnamed: C2RustUnnamed_1 {
                __wseq: 0 as libc::c_int as libc::c_ulonglong,
            },
            c2rust_unnamed_0: C2RustUnnamed {
                __g1_start: 0 as libc::c_int as libc::c_ulonglong,
            },
            __g_refs: [
                0 as libc::c_int as libc::c_uint,
                0 as libc::c_int as libc::c_uint,
            ],
            __g_size: [
                0 as libc::c_int as libc::c_uint,
                0 as libc::c_int as libc::c_uint,
            ],
            __g1_orig_size: 0 as libc::c_int as libc::c_uint,
            __wrefs: 0 as libc::c_int as libc::c_uint,
            __g_signals: [
                0 as libc::c_int as libc::c_uint,
                0 as libc::c_int as libc::c_uint,
            ],
        };
        init
    },
};
pub static mut stats_mtx: pthread_mutex_t = pthread_mutex_t {
    __data: {
        let mut init = __pthread_mutex_s {
            __lock: 0 as libc::c_int,
            __count: 0 as libc::c_int as libc::c_uint,
            __owner: 0 as libc::c_int,
            __nusers: 0 as libc::c_int as libc::c_uint,
            __kind: PTHREAD_MUTEX_TIMED_NP as libc::c_int,
            __spins: 0 as libc::c_int as libc::c_short,
            __elision: 0 as libc::c_int as libc::c_short,
            __list: {
                let mut init = __pthread_internal_list {
                    __prev: 0 as *const __pthread_internal_list
                        as *mut __pthread_internal_list,
                    __next: 0 as *const __pthread_internal_list
                        as *mut __pthread_internal_list,
                };
                init
            },
        };
        init
    },
};
pub static mut work_queue_mtx: pthread_mutex_t = pthread_mutex_t {
    __data: {
        let mut init = __pthread_mutex_s {
            __lock: 0 as libc::c_int,
            __count: 0 as libc::c_int as libc::c_uint,
            __owner: 0 as libc::c_int,
            __nusers: 0 as libc::c_int as libc::c_uint,
            __kind: PTHREAD_MUTEX_TIMED_NP as libc::c_int,
            __spins: 0 as libc::c_int as libc::c_short,
            __elision: 0 as libc::c_int as libc::c_short,
            __list: {
                let mut init = __pthread_internal_list {
                    __prev: 0 as *const __pthread_internal_list
                        as *mut __pthread_internal_list,
                    __next: 0 as *const __pthread_internal_list
                        as *mut __pthread_internal_list,
                };
                init
            },
        };
        init
    },
};
pub static mut symhash: *mut symdir_t = 0 as *const symdir_t as *mut symdir_t;
pub unsafe extern "C" fn search_buf(
    mut buf: *const libc::c_char,
    buf_len: size_t,
    mut dir_full_path: *const libc::c_char,
) -> ssize_t {
    let mut binary: libc::c_int = -(1 as libc::c_int);
    let mut buf_offset: size_t = 0 as libc::c_int as size_t;
    if opts.search_stream != 0 {
        binary = 0 as libc::c_int;
    } else if opts.search_binary_files == 0 && opts.mmap != 0 {
        binary = is_binary(buf as *const libc::c_void, buf_len);
        if binary != 0 {
            log_debug(
                b"File %s is binary. Skipping...\0" as *const u8 as *const libc::c_char,
                dir_full_path,
            );
            return -(1 as libc::c_int) as ssize_t;
        }
    }
    let mut matches_len: size_t = 0 as libc::c_int as size_t;
    let mut matches: *mut match_t = 0 as *mut match_t;
    let mut matches_size: size_t = 0;
    let mut matches_spare: size_t = 0;
    if opts.invert_match != 0 {
        matches_size = 100 as libc::c_int as size_t;
        matches = ag_malloc(
            matches_size.wrapping_mul(::std::mem::size_of::<match_t>() as libc::c_ulong),
        ) as *mut match_t;
        matches_spare = 1 as libc::c_int as size_t;
    } else {
        matches_size = 0 as libc::c_int as size_t;
        matches = 0 as *mut match_t;
        matches_spare = 0 as libc::c_int as size_t;
    }
    if opts.literal == 0 && opts.query_len == 1 as libc::c_int
        && *(opts.query).offset(0 as libc::c_int as isize) as libc::c_int == '.' as i32
    {
        matches_size = 1 as libc::c_int as size_t;
        matches = (if matches.is_null() {
            ag_malloc(
                matches_size
                    .wrapping_mul(::std::mem::size_of::<match_t>() as libc::c_ulong),
            )
        } else {
            matches as *mut libc::c_void
        }) as *mut match_t;
        (*matches.offset(0 as libc::c_int as isize)).start = 0 as libc::c_int as size_t;
        (*matches.offset(0 as libc::c_int as isize)).end = buf_len;
        matches_len = 1 as libc::c_int as size_t;
    } else if opts.literal != 0 {
        let mut match_ptr: *const libc::c_char = buf;
        while buf_offset < buf_len {
            if (opts.query_len as size_t)
                < (2 as libc::c_int as libc::c_ulong)
                    .wrapping_mul(::std::mem::size_of::<uint16_t>() as libc::c_ulong)
                    .wrapping_sub(1 as libc::c_int as libc::c_ulong)
                || opts.query_len
                    >= 127 as libc::c_int * 2 as libc::c_int + 1 as libc::c_int
            {
                match_ptr = boyer_moore_strnstr(
                    match_ptr,
                    opts.query,
                    buf_len.wrapping_sub(buf_offset),
                    opts.query_len as size_t,
                    alpha_skip_lookup.as_mut_ptr() as *const size_t,
                    find_skip_lookup,
                    (opts.casing as libc::c_uint
                        == CASE_INSENSITIVE as libc::c_int as libc::c_uint)
                        as libc::c_int,
                );
            } else {
                match_ptr = hash_strnstr(
                    match_ptr,
                    opts.query,
                    buf_len.wrapping_sub(buf_offset),
                    opts.query_len as size_t,
                    h_table.as_mut_ptr(),
                    (opts.casing as libc::c_uint
                        == CASE_SENSITIVE as libc::c_int as libc::c_uint) as libc::c_int,
                );
            }
            if match_ptr.is_null() {
                break;
            }
            if opts.word_regexp != 0 {
                let mut start: *const libc::c_char = match_ptr;
                let mut end: *const libc::c_char = match_ptr
                    .offset(opts.query_len as isize);
                if !((start == buf
                    || is_wordchar(*start.offset(-(1 as libc::c_int as isize)))
                        != opts.literal_starts_wordchar)
                    && (end == buf.offset(buf_len as isize)
                        || is_wordchar(*end) != opts.literal_ends_wordchar))
                {
                    match_ptr = match_ptr
                        .offset(
                            (*find_skip_lookup.offset(0 as libc::c_int as isize))
                                .wrapping_sub(opts.query_len as libc::c_ulong)
                                .wrapping_add(1 as libc::c_int as libc::c_ulong) as isize,
                        );
                    buf_offset = match_ptr.offset_from(buf) as libc::c_long as size_t;
                    continue;
                }
            }
            realloc_matches(
                &mut matches,
                &mut matches_size,
                matches_len.wrapping_add(matches_spare),
            );
            (*matches.offset(matches_len as isize))
                .start = match_ptr.offset_from(buf) as libc::c_long as size_t;
            (*matches.offset(matches_len as isize))
                .end = ((*matches.offset(matches_len as isize)).start)
                .wrapping_add(opts.query_len as libc::c_ulong);
            buf_offset = (*matches.offset(matches_len as isize)).end;
            log_debug(
                b"Match found. File %s, offset %lu bytes.\0" as *const u8
                    as *const libc::c_char,
                dir_full_path,
                (*matches.offset(matches_len as isize)).start,
            );
            matches_len = matches_len.wrapping_add(1);
            matches_len;
            match_ptr = match_ptr.offset(opts.query_len as isize);
            if !(opts.max_matches_per_file > 0 as libc::c_int as libc::c_ulong
                && matches_len >= opts.max_matches_per_file)
            {
                continue;
            }
            log_err(
                b"Too many matches in %s. Skipping the rest of this file.\0" as *const u8
                    as *const libc::c_char,
                dir_full_path,
            );
            break;
        }
    } else {
        let mut offset_vector: [libc::c_int; 3] = [0; 3];
        if opts.multiline != 0 {
            while buf_offset < buf_len
                && pcre_exec(
                    opts.re,
                    opts.re_extra,
                    buf,
                    buf_len as libc::c_int,
                    buf_offset as libc::c_int,
                    0 as libc::c_int,
                    offset_vector.as_mut_ptr(),
                    3 as libc::c_int,
                ) >= 0 as libc::c_int
            {
                log_debug(
                    b"Regex match found. File %s, offset %i bytes.\0" as *const u8
                        as *const libc::c_char,
                    dir_full_path,
                    offset_vector[0 as libc::c_int as usize],
                );
                buf_offset = offset_vector[1 as libc::c_int as usize] as size_t;
                if offset_vector[0 as libc::c_int as usize]
                    == offset_vector[1 as libc::c_int as usize]
                {
                    buf_offset = buf_offset.wrapping_add(1);
                    buf_offset;
                    log_debug(
                        b"Regex match is of length zero. Advancing offset one byte.\0"
                            as *const u8 as *const libc::c_char,
                    );
                }
                realloc_matches(
                    &mut matches,
                    &mut matches_size,
                    matches_len.wrapping_add(matches_spare),
                );
                (*matches.offset(matches_len as isize))
                    .start = offset_vector[0 as libc::c_int as usize] as size_t;
                (*matches.offset(matches_len as isize))
                    .end = offset_vector[1 as libc::c_int as usize] as size_t;
                matches_len = matches_len.wrapping_add(1);
                matches_len;
                if !(opts.max_matches_per_file > 0 as libc::c_int as libc::c_ulong
                    && matches_len >= opts.max_matches_per_file)
                {
                    continue;
                }
                log_err(
                    b"Too many matches in %s. Skipping the rest of this file.\0"
                        as *const u8 as *const libc::c_char,
                    dir_full_path,
                );
                break;
            }
        } else {
            's_273: while buf_offset < buf_len {
                let mut line: *const libc::c_char = 0 as *const libc::c_char;
                let mut line_len: size_t = buf_getline(
                    &mut line,
                    buf,
                    buf_len,
                    buf_offset,
                ) as size_t;
                if line.is_null() {
                    break;
                }
                let mut line_offset: size_t = 0 as libc::c_int as size_t;
                while line_offset < line_len {
                    let mut rv: libc::c_int = pcre_exec(
                        opts.re,
                        opts.re_extra,
                        line,
                        line_len as libc::c_int,
                        line_offset as libc::c_int,
                        0 as libc::c_int,
                        offset_vector.as_mut_ptr(),
                        3 as libc::c_int,
                    );
                    if rv < 0 as libc::c_int {
                        break;
                    }
                    let mut line_to_buf: size_t = buf_offset.wrapping_add(line_offset);
                    log_debug(
                        b"Regex match found. File %s, offset %i bytes.\0" as *const u8
                            as *const libc::c_char,
                        dir_full_path,
                        offset_vector[0 as libc::c_int as usize],
                    );
                    line_offset = offset_vector[1 as libc::c_int as usize] as size_t;
                    if offset_vector[0 as libc::c_int as usize]
                        == offset_vector[1 as libc::c_int as usize]
                    {
                        line_offset = line_offset.wrapping_add(1);
                        line_offset;
                        log_debug(
                            b"Regex match is of length zero. Advancing offset one byte.\0"
                                as *const u8 as *const libc::c_char,
                        );
                    }
                    realloc_matches(
                        &mut matches,
                        &mut matches_size,
                        matches_len.wrapping_add(matches_spare),
                    );
                    (*matches.offset(matches_len as isize))
                        .start = (offset_vector[0 as libc::c_int as usize]
                        as libc::c_ulong)
                        .wrapping_add(line_to_buf);
                    (*matches.offset(matches_len as isize))
                        .end = (offset_vector[1 as libc::c_int as usize]
                        as libc::c_ulong)
                        .wrapping_add(line_to_buf);
                    matches_len = matches_len.wrapping_add(1);
                    matches_len;
                    if !(opts.max_matches_per_file > 0 as libc::c_int as libc::c_ulong
                        && matches_len >= opts.max_matches_per_file)
                    {
                        continue;
                    }
                    log_err(
                        b"Too many matches in %s. Skipping the rest of this file.\0"
                            as *const u8 as *const libc::c_char,
                        dir_full_path,
                    );
                    break 's_273;
                }
                buf_offset = (buf_offset as libc::c_ulong)
                    .wrapping_add(
                        line_len.wrapping_add(1 as libc::c_int as libc::c_ulong),
                    ) as size_t as size_t;
            }
        }
    }
    if opts.invert_match != 0 {
        matches_len = invert_matches(buf, buf_len, matches, matches_len);
    }
    if opts.stats != 0 {
        pthread_mutex_lock(&mut stats_mtx);
        stats
            .total_bytes = (stats.total_bytes as libc::c_ulong).wrapping_add(buf_len)
            as size_t as size_t;
        stats.total_files = (stats.total_files).wrapping_add(1);
        stats.total_files;
        stats
            .total_matches = (stats.total_matches as libc::c_ulong)
            .wrapping_add(matches_len) as size_t as size_t;
        if matches_len > 0 as libc::c_int as libc::c_ulong {
            stats.total_file_matches = (stats.total_file_matches).wrapping_add(1);
            stats.total_file_matches;
        }
        pthread_mutex_unlock(&mut stats_mtx);
    }
    if opts.print_nonmatching_files == 0
        && (matches_len > 0 as libc::c_int as libc::c_ulong || opts.print_all_paths != 0)
    {
        if binary == -(1 as libc::c_int) && opts.print_filename_only == 0 {
            binary = is_binary(buf as *const libc::c_void, buf_len);
        }
        pthread_mutex_lock(&mut print_mtx);
        if opts.print_filename_only != 0 {
            if opts.print_count != 0 {
                print_path_count(dir_full_path, opts.path_sep, matches_len);
            } else {
                print_path(dir_full_path, opts.path_sep);
            }
        } else if binary != 0 {
            print_binary_file_matches(dir_full_path);
        } else {
            print_file_matches(
                dir_full_path,
                buf,
                buf_len,
                matches as *const match_t,
                matches_len,
            );
        }
        pthread_mutex_unlock(&mut print_mtx);
        opts.match_found = 1 as libc::c_int;
    } else if opts.search_stream != 0 && opts.passthrough != 0 {
        fprintf(out_fd, b"%s\0" as *const u8 as *const libc::c_char, buf);
    } else {
        log_debug(
            b"No match in %s\0" as *const u8 as *const libc::c_char,
            dir_full_path,
        );
    }
    if matches_len == 0 as libc::c_int as libc::c_ulong && opts.search_stream != 0 {
        print_context_append(
            buf,
            buf_len.wrapping_sub(1 as libc::c_int as libc::c_ulong),
        );
    }
    if matches_size > 0 as libc::c_int as libc::c_ulong {
        free(matches as *mut libc::c_void);
    }
    return matches_len as ssize_t;
}
pub unsafe extern "C" fn search_stream(
    mut stream: *mut FILE,
    mut path: *const libc::c_char,
) -> ssize_t {
    let mut line: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut matches_count: ssize_t = 0 as libc::c_int as ssize_t;
    let mut line_len: ssize_t = 0 as libc::c_int as ssize_t;
    let mut line_cap: size_t = 0 as libc::c_int as size_t;
    let mut i: size_t = 0;
    print_init_context();
    i = 1 as libc::c_int as size_t;
    loop {
        line_len = getline(&mut line, &mut line_cap, stream);
        if !(line_len > 0 as libc::c_int as libc::c_long) {
            break;
        }
        let mut result: ssize_t = 0;
        opts.stream_line_num = i;
        result = search_buf(line, line_len as size_t, path);
        if result > 0 as libc::c_int as libc::c_long {
            if matches_count == -(1 as libc::c_int) as libc::c_long {
                matches_count = 0 as libc::c_int as ssize_t;
            }
            matches_count += result;
        } else if matches_count <= 0 as libc::c_int as libc::c_long
            && result == -(1 as libc::c_int) as libc::c_long
        {
            matches_count = -(1 as libc::c_int) as ssize_t;
        }
        if *line.offset((line_len - 1 as libc::c_int as libc::c_long) as isize)
            as libc::c_int == '\n' as i32
        {
            line_len -= 1;
            line_len;
        }
        print_trailing_context(path, line, line_len as size_t);
        i = i.wrapping_add(1);
        i;
    }
    free(line as *mut libc::c_void);
    print_cleanup_context();
    return matches_count;
}
pub unsafe extern "C" fn search_file(mut file_full_path: *const libc::c_char) {
    let mut current_block: u64;
    let mut fd: libc::c_int = -(1 as libc::c_int);
    let mut f_len: off_t = 0 as libc::c_int as off_t;
    let mut buf: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut statbuf: stat = stat {
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
    let mut rv: libc::c_int = 0 as libc::c_int;
    let mut matches_count: libc::c_int = -(1 as libc::c_int);
    let mut fp: *mut FILE = 0 as *mut FILE;
    rv = stat(file_full_path, &mut statbuf);
    if rv != 0 as libc::c_int {
        log_err(
            b"Skipping %s: Error fstat()ing file.\0" as *const u8 as *const libc::c_char,
            file_full_path,
        );
    } else if opts.stdout_inode != 0 as libc::c_int as libc::c_ulong
        && opts.stdout_inode == statbuf.st_ino
    {
        log_debug(
            b"Skipping %s: stdout is redirected to it\0" as *const u8
                as *const libc::c_char,
            file_full_path,
        );
    } else if !(statbuf.st_mode & 0o170000 as libc::c_int as libc::c_uint
        == 0o100000 as libc::c_int as libc::c_uint)
        && !(statbuf.st_mode & 0o170000 as libc::c_int as libc::c_uint
            == 0o10000 as libc::c_int as libc::c_uint)
    {
        log_err(
            b"Skipping %s: Mode %u is not a file.\0" as *const u8 as *const libc::c_char,
            file_full_path,
            statbuf.st_mode,
        );
    } else {
        fd = open(file_full_path, 0 as libc::c_int);
        if fd < 0 as libc::c_int {
            log_err(
                b"Skipping %s: Error opening file: %s\0" as *const u8
                    as *const libc::c_char,
                file_full_path,
                strerror(*__errno_location()),
            );
        } else {
            rv = fstat(fd, &mut statbuf);
            if rv != 0 as libc::c_int {
                log_err(
                    b"Skipping %s: Error fstat()ing file.\0" as *const u8
                        as *const libc::c_char,
                    file_full_path,
                );
            } else if opts.stdout_inode != 0 as libc::c_int as libc::c_ulong
                && opts.stdout_inode == statbuf.st_ino
            {
                log_debug(
                    b"Skipping %s: stdout is redirected to it\0" as *const u8
                        as *const libc::c_char,
                    file_full_path,
                );
            } else if !(statbuf.st_mode & 0o170000 as libc::c_int as libc::c_uint
                == 0o100000 as libc::c_int as libc::c_uint)
                && !(statbuf.st_mode & 0o170000 as libc::c_int as libc::c_uint
                    == 0o10000 as libc::c_int as libc::c_uint)
            {
                log_err(
                    b"Skipping %s: Mode %u is not a file.\0" as *const u8
                        as *const libc::c_char,
                    file_full_path,
                    statbuf.st_mode,
                );
            } else {
                print_init_context();
                if statbuf.st_mode & 0o10000 as libc::c_int as libc::c_uint != 0 {
                    log_debug(
                        b"%s is a named pipe. stream searching\0" as *const u8
                            as *const libc::c_char,
                        file_full_path,
                    );
                    fp = fdopen(fd, b"r\0" as *const u8 as *const libc::c_char);
                    matches_count = search_stream(fp, file_full_path) as libc::c_int;
                    fclose(fp);
                } else {
                    f_len = statbuf.st_size;
                    if f_len == 0 as libc::c_int as libc::c_long {
                        if *(opts.query).offset(0 as libc::c_int as isize) as libc::c_int
                            == '.' as i32 && opts.query_len == 1 as libc::c_int
                            && opts.literal == 0 && opts.search_all_files != 0
                        {
                            matches_count = search_buf(
                                buf,
                                f_len as size_t,
                                file_full_path,
                            ) as libc::c_int;
                        } else {
                            log_debug(
                                b"Skipping %s: file is empty.\0" as *const u8
                                    as *const libc::c_char,
                                file_full_path,
                            );
                        }
                    } else if opts.literal == 0
                        && f_len > 2147483647 as libc::c_int as libc::c_long
                    {
                        log_err(
                            b"Skipping %s: pcre_exec() can't handle files larger than %i bytes.\0"
                                as *const u8 as *const libc::c_char,
                            file_full_path,
                            2147483647 as libc::c_int,
                        );
                    } else {
                        if opts.mmap != 0 {
                            buf = mmap(
                                0 as *mut libc::c_void,
                                f_len as size_t,
                                0x1 as libc::c_int,
                                0x2 as libc::c_int,
                                fd,
                                0 as libc::c_int as __off_t,
                            ) as *mut libc::c_char;
                            if buf
                                == -(1 as libc::c_int) as *mut libc::c_void
                                    as *mut libc::c_char
                            {
                                log_err(
                                    b"File %s failed to load: %s.\0" as *const u8
                                        as *const libc::c_char,
                                    file_full_path,
                                    strerror(*__errno_location()),
                                );
                                current_block = 1118849515257890855;
                            } else {
                                madvise(
                                    buf as *mut libc::c_void,
                                    f_len as size_t,
                                    2 as libc::c_int,
                                );
                                current_block = 2122094917359643297;
                            }
                        } else {
                            buf = ag_malloc(f_len as size_t) as *mut libc::c_char;
                            let mut bytes_read: ssize_t = 0 as libc::c_int as ssize_t;
                            if opts.search_binary_files == 0 {
                                bytes_read
                                    += read(
                                        fd,
                                        buf as *mut libc::c_void,
                                        ag_min(f_len as size_t, 512 as libc::c_int as size_t),
                                    );
                                if is_binary(buf as *const libc::c_void, f_len as size_t)
                                    != 0
                                {
                                    log_debug(
                                        b"File %s is binary. Skipping...\0" as *const u8
                                            as *const libc::c_char,
                                        file_full_path,
                                    );
                                    current_block = 1118849515257890855;
                                } else {
                                    current_block = 7226443171521532240;
                                }
                            } else {
                                current_block = 7226443171521532240;
                            }
                            match current_block {
                                1118849515257890855 => {}
                                _ => {
                                    while bytes_read < f_len {
                                        bytes_read
                                            += read(
                                                fd,
                                                buf.offset(bytes_read as isize) as *mut libc::c_void,
                                                f_len as size_t,
                                            );
                                    }
                                    if bytes_read != f_len {
                                        die(
                                            b"File %s read(): expected to read %u bytes but read %u\0"
                                                as *const u8 as *const libc::c_char,
                                            file_full_path,
                                            f_len,
                                            bytes_read,
                                        );
                                    }
                                    current_block = 2122094917359643297;
                                }
                            }
                        }
                        match current_block {
                            1118849515257890855 => {}
                            _ => {
                                if opts.search_zip_files != 0 {
                                    let mut zip_type: ag_compression_type = is_zipped(
                                        buf as *const libc::c_void,
                                        f_len as libc::c_int,
                                    );
                                    if zip_type as libc::c_uint
                                        != AG_NO_COMPRESSION as libc::c_int as libc::c_uint
                                    {
                                        log_debug(
                                            b"%s is a compressed file. stream searching\0" as *const u8
                                                as *const libc::c_char,
                                            file_full_path,
                                        );
                                        fp = decompress_open(
                                            fd,
                                            b"r\0" as *const u8 as *const libc::c_char,
                                            zip_type,
                                        );
                                        matches_count = search_stream(fp, file_full_path)
                                            as libc::c_int;
                                        fclose(fp);
                                        current_block = 1118849515257890855;
                                    } else {
                                        current_block = 11777552016271000781;
                                    }
                                } else {
                                    current_block = 11777552016271000781;
                                }
                                match current_block {
                                    1118849515257890855 => {}
                                    _ => {
                                        matches_count = search_buf(
                                            buf,
                                            f_len as size_t,
                                            file_full_path,
                                        ) as libc::c_int;
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
    if opts.print_nonmatching_files != 0 && matches_count == 0 as libc::c_int {
        pthread_mutex_lock(&mut print_mtx);
        print_path(file_full_path, opts.path_sep);
        pthread_mutex_unlock(&mut print_mtx);
        opts.match_found = 1 as libc::c_int;
    }
    print_cleanup_context();
    if !buf.is_null() {
        if opts.mmap != 0 {
            if buf != -(1 as libc::c_int) as *mut libc::c_void as *mut libc::c_char {
                munmap(buf as *mut libc::c_void, f_len as size_t);
            }
        } else {
            free(buf as *mut libc::c_void);
        }
    }
    if fd != -(1 as libc::c_int) {
        close(fd);
    }
}
pub unsafe extern "C" fn search_file_worker(
    mut i: *mut libc::c_void,
) -> *mut libc::c_void {
    let mut queue_item: *mut work_queue_t = 0 as *mut work_queue_t;
    let mut worker_id: libc::c_int = *(i as *mut libc::c_int);
    log_debug(b"Worker %i started\0" as *const u8 as *const libc::c_char, worker_id);
    loop {
        pthread_mutex_lock(&mut work_queue_mtx);
        while work_queue.is_null() {
            if done_adding_files != 0 {
                pthread_mutex_unlock(&mut work_queue_mtx);
                log_debug(
                    b"Worker %i finished.\0" as *const u8 as *const libc::c_char,
                    worker_id,
                );
                pthread_exit(0 as *mut libc::c_void);
            }
            pthread_cond_wait(&mut files_ready, &mut work_queue_mtx);
        }
        queue_item = work_queue;
        work_queue = (*work_queue).next;
        if work_queue.is_null() {
            work_queue_tail = 0 as *mut work_queue_t;
        }
        pthread_mutex_unlock(&mut work_queue_mtx);
        search_file((*queue_item).path);
        free((*queue_item).path as *mut libc::c_void);
        free(queue_item as *mut libc::c_void);
    };
}
unsafe extern "C" fn check_symloop_enter(
    mut path: *const libc::c_char,
    mut outkey: *mut dirkey_t,
) -> libc::c_int {
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
    let mut item_found: *mut symdir_t = 0 as *mut symdir_t;
    let mut new_item: *mut symdir_t = 0 as *mut symdir_t;
    memset(
        outkey as *mut libc::c_void,
        0 as libc::c_int,
        ::std::mem::size_of::<dirkey_t>() as libc::c_ulong,
    );
    (*outkey).dev = 0 as libc::c_int as dev_t;
    (*outkey).ino = 0 as libc::c_int as ino_t;
    let mut res: libc::c_int = stat(path, &mut buf);
    if res != 0 as libc::c_int {
        log_err(b"Error stat()ing: %s\0" as *const u8 as *const libc::c_char, path);
        return -(1 as libc::c_int);
    }
    (*outkey).dev = buf.st_dev;
    (*outkey).ino = buf.st_ino;
    let mut _hf_bkt: libc::c_uint = 0;
    let mut _hf_hashv: libc::c_uint = 0;
    item_found = 0 as *mut symdir_t;
    if !symhash.is_null() {
        let mut _hj_i: libc::c_uint = 0;
        let mut _hj_j: libc::c_uint = 0;
        let mut _hj_k: libc::c_uint = 0;
        let mut _hj_key: *mut libc::c_uchar = outkey as *mut libc::c_uchar;
        _hf_hashv = 0xfeedbeef as libc::c_uint;
        _hj_j = 0x9e3779b9 as libc::c_uint;
        _hj_i = _hj_j;
        _hj_k = ::std::mem::size_of::<dirkey_t>() as libc::c_ulong as libc::c_uint;
        while _hj_k >= 12 as libc::c_int as libc::c_uint {
            _hj_i = _hj_i
                .wrapping_add(
                    (*_hj_key.offset(0 as libc::c_int as isize) as libc::c_uint)
                        .wrapping_add(
                            (*_hj_key.offset(1 as libc::c_int as isize) as libc::c_uint)
                                << 8 as libc::c_int,
                        )
                        .wrapping_add(
                            (*_hj_key.offset(2 as libc::c_int as isize) as libc::c_uint)
                                << 16 as libc::c_int,
                        )
                        .wrapping_add(
                            (*_hj_key.offset(3 as libc::c_int as isize) as libc::c_uint)
                                << 24 as libc::c_int,
                        ),
                );
            _hj_j = _hj_j
                .wrapping_add(
                    (*_hj_key.offset(4 as libc::c_int as isize) as libc::c_uint)
                        .wrapping_add(
                            (*_hj_key.offset(5 as libc::c_int as isize) as libc::c_uint)
                                << 8 as libc::c_int,
                        )
                        .wrapping_add(
                            (*_hj_key.offset(6 as libc::c_int as isize) as libc::c_uint)
                                << 16 as libc::c_int,
                        )
                        .wrapping_add(
                            (*_hj_key.offset(7 as libc::c_int as isize) as libc::c_uint)
                                << 24 as libc::c_int,
                        ),
                );
            _hf_hashv = _hf_hashv
                .wrapping_add(
                    (*_hj_key.offset(8 as libc::c_int as isize) as libc::c_uint)
                        .wrapping_add(
                            (*_hj_key.offset(9 as libc::c_int as isize) as libc::c_uint)
                                << 8 as libc::c_int,
                        )
                        .wrapping_add(
                            (*_hj_key.offset(10 as libc::c_int as isize) as libc::c_uint)
                                << 16 as libc::c_int,
                        )
                        .wrapping_add(
                            (*_hj_key.offset(11 as libc::c_int as isize) as libc::c_uint)
                                << 24 as libc::c_int,
                        ),
                );
            _hj_i = _hj_i.wrapping_sub(_hj_j);
            _hj_i = _hj_i.wrapping_sub(_hf_hashv);
            _hj_i ^= _hf_hashv >> 13 as libc::c_int;
            _hj_j = _hj_j.wrapping_sub(_hf_hashv);
            _hj_j = _hj_j.wrapping_sub(_hj_i);
            _hj_j ^= _hj_i << 8 as libc::c_int;
            _hf_hashv = _hf_hashv.wrapping_sub(_hj_i);
            _hf_hashv = _hf_hashv.wrapping_sub(_hj_j);
            _hf_hashv ^= _hj_j >> 13 as libc::c_int;
            _hj_i = _hj_i.wrapping_sub(_hj_j);
            _hj_i = _hj_i.wrapping_sub(_hf_hashv);
            _hj_i ^= _hf_hashv >> 12 as libc::c_int;
            _hj_j = _hj_j.wrapping_sub(_hf_hashv);
            _hj_j = _hj_j.wrapping_sub(_hj_i);
            _hj_j ^= _hj_i << 16 as libc::c_int;
            _hf_hashv = _hf_hashv.wrapping_sub(_hj_i);
            _hf_hashv = _hf_hashv.wrapping_sub(_hj_j);
            _hf_hashv ^= _hj_j >> 5 as libc::c_int;
            _hj_i = _hj_i.wrapping_sub(_hj_j);
            _hj_i = _hj_i.wrapping_sub(_hf_hashv);
            _hj_i ^= _hf_hashv >> 3 as libc::c_int;
            _hj_j = _hj_j.wrapping_sub(_hf_hashv);
            _hj_j = _hj_j.wrapping_sub(_hj_i);
            _hj_j ^= _hj_i << 10 as libc::c_int;
            _hf_hashv = _hf_hashv.wrapping_sub(_hj_i);
            _hf_hashv = _hf_hashv.wrapping_sub(_hj_j);
            _hf_hashv ^= _hj_j >> 15 as libc::c_int;
            _hj_key = _hj_key.offset(12 as libc::c_int as isize);
            _hj_k = _hj_k.wrapping_sub(12 as libc::c_int as libc::c_uint);
        }
        _hf_hashv = (_hf_hashv as libc::c_ulong)
            .wrapping_add(::std::mem::size_of::<dirkey_t>() as libc::c_ulong)
            as libc::c_uint as libc::c_uint;
        let mut current_block_60: u64;
        match _hj_k {
            11 => {
                _hf_hashv = _hf_hashv
                    .wrapping_add(
                        (*_hj_key.offset(10 as libc::c_int as isize) as libc::c_uint)
                            << 24 as libc::c_int,
                    );
                current_block_60 = 10653674786005835551;
            }
            10 => {
                current_block_60 = 10653674786005835551;
            }
            9 => {
                current_block_60 = 5921640451881475705;
            }
            8 => {
                current_block_60 = 14554283306219632007;
            }
            7 => {
                current_block_60 = 12072240430087793380;
            }
            6 => {
                current_block_60 = 4483526687222536229;
            }
            5 => {
                current_block_60 = 2149107243143984000;
            }
            4 => {
                current_block_60 = 14079335053347667564;
            }
            3 => {
                current_block_60 = 13912080936097869512;
            }
            2 => {
                current_block_60 = 3683587023464022395;
            }
            1 => {
                current_block_60 = 16463618559373710392;
            }
            _ => {
                current_block_60 = 6476622998065200121;
            }
        }
        match current_block_60 {
            10653674786005835551 => {
                _hf_hashv = _hf_hashv
                    .wrapping_add(
                        (*_hj_key.offset(9 as libc::c_int as isize) as libc::c_uint)
                            << 16 as libc::c_int,
                    );
                current_block_60 = 5921640451881475705;
            }
            _ => {}
        }
        match current_block_60 {
            5921640451881475705 => {
                _hf_hashv = _hf_hashv
                    .wrapping_add(
                        (*_hj_key.offset(8 as libc::c_int as isize) as libc::c_uint)
                            << 8 as libc::c_int,
                    );
                current_block_60 = 14554283306219632007;
            }
            _ => {}
        }
        match current_block_60 {
            14554283306219632007 => {
                _hj_j = _hj_j
                    .wrapping_add(
                        (*_hj_key.offset(7 as libc::c_int as isize) as libc::c_uint)
                            << 24 as libc::c_int,
                    );
                current_block_60 = 12072240430087793380;
            }
            _ => {}
        }
        match current_block_60 {
            12072240430087793380 => {
                _hj_j = _hj_j
                    .wrapping_add(
                        (*_hj_key.offset(6 as libc::c_int as isize) as libc::c_uint)
                            << 16 as libc::c_int,
                    );
                current_block_60 = 4483526687222536229;
            }
            _ => {}
        }
        match current_block_60 {
            4483526687222536229 => {
                _hj_j = _hj_j
                    .wrapping_add(
                        (*_hj_key.offset(5 as libc::c_int as isize) as libc::c_uint)
                            << 8 as libc::c_int,
                    );
                current_block_60 = 2149107243143984000;
            }
            _ => {}
        }
        match current_block_60 {
            2149107243143984000 => {
                _hj_j = _hj_j
                    .wrapping_add(
                        *_hj_key.offset(4 as libc::c_int as isize) as libc::c_uint,
                    );
                current_block_60 = 14079335053347667564;
            }
            _ => {}
        }
        match current_block_60 {
            14079335053347667564 => {
                _hj_i = _hj_i
                    .wrapping_add(
                        (*_hj_key.offset(3 as libc::c_int as isize) as libc::c_uint)
                            << 24 as libc::c_int,
                    );
                current_block_60 = 13912080936097869512;
            }
            _ => {}
        }
        match current_block_60 {
            13912080936097869512 => {
                _hj_i = _hj_i
                    .wrapping_add(
                        (*_hj_key.offset(2 as libc::c_int as isize) as libc::c_uint)
                            << 16 as libc::c_int,
                    );
                current_block_60 = 3683587023464022395;
            }
            _ => {}
        }
        match current_block_60 {
            3683587023464022395 => {
                _hj_i = _hj_i
                    .wrapping_add(
                        (*_hj_key.offset(1 as libc::c_int as isize) as libc::c_uint)
                            << 8 as libc::c_int,
                    );
                current_block_60 = 16463618559373710392;
            }
            _ => {}
        }
        match current_block_60 {
            16463618559373710392 => {
                _hj_i = _hj_i
                    .wrapping_add(
                        *_hj_key.offset(0 as libc::c_int as isize) as libc::c_uint,
                    );
            }
            _ => {}
        }
        _hj_i = _hj_i.wrapping_sub(_hj_j);
        _hj_i = _hj_i.wrapping_sub(_hf_hashv);
        _hj_i ^= _hf_hashv >> 13 as libc::c_int;
        _hj_j = _hj_j.wrapping_sub(_hf_hashv);
        _hj_j = _hj_j.wrapping_sub(_hj_i);
        _hj_j ^= _hj_i << 8 as libc::c_int;
        _hf_hashv = _hf_hashv.wrapping_sub(_hj_i);
        _hf_hashv = _hf_hashv.wrapping_sub(_hj_j);
        _hf_hashv ^= _hj_j >> 13 as libc::c_int;
        _hj_i = _hj_i.wrapping_sub(_hj_j);
        _hj_i = _hj_i.wrapping_sub(_hf_hashv);
        _hj_i ^= _hf_hashv >> 12 as libc::c_int;
        _hj_j = _hj_j.wrapping_sub(_hf_hashv);
        _hj_j = _hj_j.wrapping_sub(_hj_i);
        _hj_j ^= _hj_i << 16 as libc::c_int;
        _hf_hashv = _hf_hashv.wrapping_sub(_hj_i);
        _hf_hashv = _hf_hashv.wrapping_sub(_hj_j);
        _hf_hashv ^= _hj_j >> 5 as libc::c_int;
        _hj_i = _hj_i.wrapping_sub(_hj_j);
        _hj_i = _hj_i.wrapping_sub(_hf_hashv);
        _hj_i ^= _hf_hashv >> 3 as libc::c_int;
        _hj_j = _hj_j.wrapping_sub(_hf_hashv);
        _hj_j = _hj_j.wrapping_sub(_hj_i);
        _hj_j ^= _hj_i << 10 as libc::c_int;
        _hf_hashv = _hf_hashv.wrapping_sub(_hj_i);
        _hf_hashv = _hf_hashv.wrapping_sub(_hj_j);
        _hf_hashv ^= _hj_j >> 15 as libc::c_int;
        _hf_bkt = _hf_hashv
            & ((*(*symhash).hh.tbl).num_buckets)
                .wrapping_sub(1 as libc::c_int as libc::c_uint);
        if !((*((*(*symhash).hh.tbl).buckets).offset(_hf_bkt as isize)).hh_head)
            .is_null()
        {
            item_found = ((*((*(*symhash).hh.tbl).buckets).offset(_hf_bkt as isize))
                .hh_head as *mut libc::c_char)
                .offset(-((*(*symhash).hh.tbl).hho as isize)) as *mut libc::c_void
                as *mut symdir_t;
        } else {
            item_found = 0 as *mut symdir_t;
        }
        while !item_found.is_null() {
            if (*item_found).hh.keylen as libc::c_ulong
                == ::std::mem::size_of::<dirkey_t>() as libc::c_ulong
            {
                if memcmp(
                    (*item_found).hh.key,
                    outkey as *const libc::c_void,
                    ::std::mem::size_of::<dirkey_t>() as libc::c_ulong,
                ) == 0 as libc::c_int
                {
                    break;
                }
            }
            if !((*item_found).hh.hh_next).is_null() {
                item_found = ((*item_found).hh.hh_next as *mut libc::c_char)
                    .offset(-((*(*symhash).hh.tbl).hho as isize)) as *mut libc::c_void
                    as *mut symdir_t;
            } else {
                item_found = 0 as *mut symdir_t;
            }
        }
    }
    if !item_found.is_null() {
        return 1 as libc::c_int;
    }
    new_item = ag_malloc(::std::mem::size_of::<symdir_t>() as libc::c_ulong)
        as *mut symdir_t;
    memcpy(
        &mut (*new_item).key as *mut dirkey_t as *mut libc::c_void,
        outkey as *const libc::c_void,
        ::std::mem::size_of::<dirkey_t>() as libc::c_ulong,
    );
    let mut _ha_bkt: libc::c_uint = 0;
    (*new_item).hh.next = 0 as *mut libc::c_void;
    (*new_item)
        .hh
        .key = &mut (*new_item).key as *mut dirkey_t as *mut libc::c_char
        as *mut libc::c_void;
    (*new_item)
        .hh
        .keylen = ::std::mem::size_of::<dirkey_t>() as libc::c_ulong as libc::c_uint;
    if symhash.is_null() {
        symhash = new_item;
        (*symhash).hh.prev = 0 as *mut libc::c_void;
        (*symhash)
            .hh
            .tbl = malloc(::std::mem::size_of::<UT_hash_table>() as libc::c_ulong)
            as *mut UT_hash_table;
        if ((*symhash).hh.tbl).is_null() {
            exit(-(1 as libc::c_int));
        }
        memset(
            (*symhash).hh.tbl as *mut libc::c_void,
            0 as libc::c_int,
            ::std::mem::size_of::<UT_hash_table>() as libc::c_ulong,
        );
        (*(*symhash).hh.tbl).tail = &mut (*symhash).hh;
        (*(*symhash).hh.tbl).num_buckets = 32 as libc::c_int as libc::c_uint;
        (*(*symhash).hh.tbl).log2_num_buckets = 5 as libc::c_int as libc::c_uint;
        (*(*symhash).hh.tbl)
            .hho = (&mut (*symhash).hh as *mut UT_hash_handle as *mut libc::c_char)
            .offset_from(symhash as *mut libc::c_char) as libc::c_long;
        (*(*symhash).hh.tbl)
            .buckets = malloc(
            (32 as libc::c_int as libc::c_ulong)
                .wrapping_mul(::std::mem::size_of::<UT_hash_bucket>() as libc::c_ulong),
        ) as *mut UT_hash_bucket;
        if ((*(*symhash).hh.tbl).buckets).is_null() {
            exit(-(1 as libc::c_int));
        }
        memset(
            (*(*symhash).hh.tbl).buckets as *mut libc::c_void,
            0 as libc::c_int,
            (32 as libc::c_int as libc::c_ulong)
                .wrapping_mul(::std::mem::size_of::<UT_hash_bucket>() as libc::c_ulong),
        );
        (*(*symhash).hh.tbl).signature = 0xa0111fe1 as libc::c_uint;
    } else {
        (*(*(*symhash).hh.tbl).tail).next = new_item as *mut libc::c_void;
        (*new_item)
            .hh
            .prev = ((*(*symhash).hh.tbl).tail as *mut libc::c_char)
            .offset(-((*(*symhash).hh.tbl).hho as isize)) as *mut libc::c_void;
        (*(*symhash).hh.tbl).tail = &mut (*new_item).hh;
    }
    (*(*symhash).hh.tbl).num_items = ((*(*symhash).hh.tbl).num_items).wrapping_add(1);
    (*(*symhash).hh.tbl).num_items;
    (*new_item).hh.tbl = (*symhash).hh.tbl;
    let mut _hj_i_0: libc::c_uint = 0;
    let mut _hj_j_0: libc::c_uint = 0;
    let mut _hj_k_0: libc::c_uint = 0;
    let mut _hj_key_0: *mut libc::c_uchar = &mut (*new_item).key as *mut dirkey_t
        as *mut libc::c_uchar;
    (*new_item).hh.hashv = 0xfeedbeef as libc::c_uint;
    _hj_j_0 = 0x9e3779b9 as libc::c_uint;
    _hj_i_0 = _hj_j_0;
    _hj_k_0 = ::std::mem::size_of::<dirkey_t>() as libc::c_ulong as libc::c_uint;
    while _hj_k_0 >= 12 as libc::c_int as libc::c_uint {
        _hj_i_0 = _hj_i_0
            .wrapping_add(
                (*_hj_key_0.offset(0 as libc::c_int as isize) as libc::c_uint)
                    .wrapping_add(
                        (*_hj_key_0.offset(1 as libc::c_int as isize) as libc::c_uint)
                            << 8 as libc::c_int,
                    )
                    .wrapping_add(
                        (*_hj_key_0.offset(2 as libc::c_int as isize) as libc::c_uint)
                            << 16 as libc::c_int,
                    )
                    .wrapping_add(
                        (*_hj_key_0.offset(3 as libc::c_int as isize) as libc::c_uint)
                            << 24 as libc::c_int,
                    ),
            );
        _hj_j_0 = _hj_j_0
            .wrapping_add(
                (*_hj_key_0.offset(4 as libc::c_int as isize) as libc::c_uint)
                    .wrapping_add(
                        (*_hj_key_0.offset(5 as libc::c_int as isize) as libc::c_uint)
                            << 8 as libc::c_int,
                    )
                    .wrapping_add(
                        (*_hj_key_0.offset(6 as libc::c_int as isize) as libc::c_uint)
                            << 16 as libc::c_int,
                    )
                    .wrapping_add(
                        (*_hj_key_0.offset(7 as libc::c_int as isize) as libc::c_uint)
                            << 24 as libc::c_int,
                    ),
            );
        (*new_item)
            .hh
            .hashv = ((*new_item).hh.hashv)
            .wrapping_add(
                (*_hj_key_0.offset(8 as libc::c_int as isize) as libc::c_uint)
                    .wrapping_add(
                        (*_hj_key_0.offset(9 as libc::c_int as isize) as libc::c_uint)
                            << 8 as libc::c_int,
                    )
                    .wrapping_add(
                        (*_hj_key_0.offset(10 as libc::c_int as isize) as libc::c_uint)
                            << 16 as libc::c_int,
                    )
                    .wrapping_add(
                        (*_hj_key_0.offset(11 as libc::c_int as isize) as libc::c_uint)
                            << 24 as libc::c_int,
                    ),
            );
        _hj_i_0 = _hj_i_0.wrapping_sub(_hj_j_0);
        _hj_i_0 = _hj_i_0.wrapping_sub((*new_item).hh.hashv);
        _hj_i_0 ^= (*new_item).hh.hashv >> 13 as libc::c_int;
        _hj_j_0 = _hj_j_0.wrapping_sub((*new_item).hh.hashv);
        _hj_j_0 = _hj_j_0.wrapping_sub(_hj_i_0);
        _hj_j_0 ^= _hj_i_0 << 8 as libc::c_int;
        (*new_item).hh.hashv = ((*new_item).hh.hashv).wrapping_sub(_hj_i_0);
        (*new_item).hh.hashv = ((*new_item).hh.hashv).wrapping_sub(_hj_j_0);
        (*new_item).hh.hashv ^= _hj_j_0 >> 13 as libc::c_int;
        _hj_i_0 = _hj_i_0.wrapping_sub(_hj_j_0);
        _hj_i_0 = _hj_i_0.wrapping_sub((*new_item).hh.hashv);
        _hj_i_0 ^= (*new_item).hh.hashv >> 12 as libc::c_int;
        _hj_j_0 = _hj_j_0.wrapping_sub((*new_item).hh.hashv);
        _hj_j_0 = _hj_j_0.wrapping_sub(_hj_i_0);
        _hj_j_0 ^= _hj_i_0 << 16 as libc::c_int;
        (*new_item).hh.hashv = ((*new_item).hh.hashv).wrapping_sub(_hj_i_0);
        (*new_item).hh.hashv = ((*new_item).hh.hashv).wrapping_sub(_hj_j_0);
        (*new_item).hh.hashv ^= _hj_j_0 >> 5 as libc::c_int;
        _hj_i_0 = _hj_i_0.wrapping_sub(_hj_j_0);
        _hj_i_0 = _hj_i_0.wrapping_sub((*new_item).hh.hashv);
        _hj_i_0 ^= (*new_item).hh.hashv >> 3 as libc::c_int;
        _hj_j_0 = _hj_j_0.wrapping_sub((*new_item).hh.hashv);
        _hj_j_0 = _hj_j_0.wrapping_sub(_hj_i_0);
        _hj_j_0 ^= _hj_i_0 << 10 as libc::c_int;
        (*new_item).hh.hashv = ((*new_item).hh.hashv).wrapping_sub(_hj_i_0);
        (*new_item).hh.hashv = ((*new_item).hh.hashv).wrapping_sub(_hj_j_0);
        (*new_item).hh.hashv ^= _hj_j_0 >> 15 as libc::c_int;
        _hj_key_0 = _hj_key_0.offset(12 as libc::c_int as isize);
        _hj_k_0 = _hj_k_0.wrapping_sub(12 as libc::c_int as libc::c_uint);
    }
    (*new_item)
        .hh
        .hashv = ((*new_item).hh.hashv as libc::c_ulong)
        .wrapping_add(::std::mem::size_of::<dirkey_t>() as libc::c_ulong) as libc::c_uint
        as libc::c_uint;
    let mut current_block_199: u64;
    match _hj_k_0 {
        11 => {
            (*new_item)
                .hh
                .hashv = ((*new_item).hh.hashv)
                .wrapping_add(
                    (*_hj_key_0.offset(10 as libc::c_int as isize) as libc::c_uint)
                        << 24 as libc::c_int,
                );
            current_block_199 = 4613046691852425349;
        }
        10 => {
            current_block_199 = 4613046691852425349;
        }
        9 => {
            current_block_199 = 16624850812192366308;
        }
        8 => {
            current_block_199 = 18348486251075996257;
        }
        7 => {
            current_block_199 = 239935657650551384;
        }
        6 => {
            current_block_199 = 832791514371635938;
        }
        5 => {
            current_block_199 = 6779615841179357261;
        }
        4 => {
            current_block_199 = 17432715877150996698;
        }
        3 => {
            current_block_199 = 12270305463401793827;
        }
        2 => {
            current_block_199 = 9891093644068351698;
        }
        1 => {
            current_block_199 = 18168187885293153237;
        }
        _ => {
            current_block_199 = 10265667325682070567;
        }
    }
    match current_block_199 {
        4613046691852425349 => {
            (*new_item)
                .hh
                .hashv = ((*new_item).hh.hashv)
                .wrapping_add(
                    (*_hj_key_0.offset(9 as libc::c_int as isize) as libc::c_uint)
                        << 16 as libc::c_int,
                );
            current_block_199 = 16624850812192366308;
        }
        _ => {}
    }
    match current_block_199 {
        16624850812192366308 => {
            (*new_item)
                .hh
                .hashv = ((*new_item).hh.hashv)
                .wrapping_add(
                    (*_hj_key_0.offset(8 as libc::c_int as isize) as libc::c_uint)
                        << 8 as libc::c_int,
                );
            current_block_199 = 18348486251075996257;
        }
        _ => {}
    }
    match current_block_199 {
        18348486251075996257 => {
            _hj_j_0 = _hj_j_0
                .wrapping_add(
                    (*_hj_key_0.offset(7 as libc::c_int as isize) as libc::c_uint)
                        << 24 as libc::c_int,
                );
            current_block_199 = 239935657650551384;
        }
        _ => {}
    }
    match current_block_199 {
        239935657650551384 => {
            _hj_j_0 = _hj_j_0
                .wrapping_add(
                    (*_hj_key_0.offset(6 as libc::c_int as isize) as libc::c_uint)
                        << 16 as libc::c_int,
                );
            current_block_199 = 832791514371635938;
        }
        _ => {}
    }
    match current_block_199 {
        832791514371635938 => {
            _hj_j_0 = _hj_j_0
                .wrapping_add(
                    (*_hj_key_0.offset(5 as libc::c_int as isize) as libc::c_uint)
                        << 8 as libc::c_int,
                );
            current_block_199 = 6779615841179357261;
        }
        _ => {}
    }
    match current_block_199 {
        6779615841179357261 => {
            _hj_j_0 = _hj_j_0
                .wrapping_add(
                    *_hj_key_0.offset(4 as libc::c_int as isize) as libc::c_uint,
                );
            current_block_199 = 17432715877150996698;
        }
        _ => {}
    }
    match current_block_199 {
        17432715877150996698 => {
            _hj_i_0 = _hj_i_0
                .wrapping_add(
                    (*_hj_key_0.offset(3 as libc::c_int as isize) as libc::c_uint)
                        << 24 as libc::c_int,
                );
            current_block_199 = 12270305463401793827;
        }
        _ => {}
    }
    match current_block_199 {
        12270305463401793827 => {
            _hj_i_0 = _hj_i_0
                .wrapping_add(
                    (*_hj_key_0.offset(2 as libc::c_int as isize) as libc::c_uint)
                        << 16 as libc::c_int,
                );
            current_block_199 = 9891093644068351698;
        }
        _ => {}
    }
    match current_block_199 {
        9891093644068351698 => {
            _hj_i_0 = _hj_i_0
                .wrapping_add(
                    (*_hj_key_0.offset(1 as libc::c_int as isize) as libc::c_uint)
                        << 8 as libc::c_int,
                );
            current_block_199 = 18168187885293153237;
        }
        _ => {}
    }
    match current_block_199 {
        18168187885293153237 => {
            _hj_i_0 = _hj_i_0
                .wrapping_add(
                    *_hj_key_0.offset(0 as libc::c_int as isize) as libc::c_uint,
                );
        }
        _ => {}
    }
    _hj_i_0 = _hj_i_0.wrapping_sub(_hj_j_0);
    _hj_i_0 = _hj_i_0.wrapping_sub((*new_item).hh.hashv);
    _hj_i_0 ^= (*new_item).hh.hashv >> 13 as libc::c_int;
    _hj_j_0 = _hj_j_0.wrapping_sub((*new_item).hh.hashv);
    _hj_j_0 = _hj_j_0.wrapping_sub(_hj_i_0);
    _hj_j_0 ^= _hj_i_0 << 8 as libc::c_int;
    (*new_item).hh.hashv = ((*new_item).hh.hashv).wrapping_sub(_hj_i_0);
    (*new_item).hh.hashv = ((*new_item).hh.hashv).wrapping_sub(_hj_j_0);
    (*new_item).hh.hashv ^= _hj_j_0 >> 13 as libc::c_int;
    _hj_i_0 = _hj_i_0.wrapping_sub(_hj_j_0);
    _hj_i_0 = _hj_i_0.wrapping_sub((*new_item).hh.hashv);
    _hj_i_0 ^= (*new_item).hh.hashv >> 12 as libc::c_int;
    _hj_j_0 = _hj_j_0.wrapping_sub((*new_item).hh.hashv);
    _hj_j_0 = _hj_j_0.wrapping_sub(_hj_i_0);
    _hj_j_0 ^= _hj_i_0 << 16 as libc::c_int;
    (*new_item).hh.hashv = ((*new_item).hh.hashv).wrapping_sub(_hj_i_0);
    (*new_item).hh.hashv = ((*new_item).hh.hashv).wrapping_sub(_hj_j_0);
    (*new_item).hh.hashv ^= _hj_j_0 >> 5 as libc::c_int;
    _hj_i_0 = _hj_i_0.wrapping_sub(_hj_j_0);
    _hj_i_0 = _hj_i_0.wrapping_sub((*new_item).hh.hashv);
    _hj_i_0 ^= (*new_item).hh.hashv >> 3 as libc::c_int;
    _hj_j_0 = _hj_j_0.wrapping_sub((*new_item).hh.hashv);
    _hj_j_0 = _hj_j_0.wrapping_sub(_hj_i_0);
    _hj_j_0 ^= _hj_i_0 << 10 as libc::c_int;
    (*new_item).hh.hashv = ((*new_item).hh.hashv).wrapping_sub(_hj_i_0);
    (*new_item).hh.hashv = ((*new_item).hh.hashv).wrapping_sub(_hj_j_0);
    (*new_item).hh.hashv ^= _hj_j_0 >> 15 as libc::c_int;
    _ha_bkt = (*new_item).hh.hashv
        & ((*(*symhash).hh.tbl).num_buckets)
            .wrapping_sub(1 as libc::c_int as libc::c_uint);
    let ref mut fresh0 = (*((*(*symhash).hh.tbl).buckets).offset(_ha_bkt as isize))
        .count;
    *fresh0 = (*fresh0).wrapping_add(1);
    *fresh0;
    (*new_item)
        .hh
        .hh_next = (*((*(*symhash).hh.tbl).buckets).offset(_ha_bkt as isize)).hh_head;
    (*new_item).hh.hh_prev = 0 as *mut UT_hash_handle;
    if !((*((*(*symhash).hh.tbl).buckets).offset(_ha_bkt as isize)).hh_head).is_null() {
        let ref mut fresh1 = (*(*((*(*symhash).hh.tbl).buckets).offset(_ha_bkt as isize))
            .hh_head)
            .hh_prev;
        *fresh1 = &mut (*new_item).hh;
    }
    let ref mut fresh2 = (*((*(*symhash).hh.tbl).buckets).offset(_ha_bkt as isize))
        .hh_head;
    *fresh2 = &mut (*new_item).hh;
    if (*((*(*symhash).hh.tbl).buckets).offset(_ha_bkt as isize)).count
        >= ((*((*(*symhash).hh.tbl).buckets).offset(_ha_bkt as isize)).expand_mult)
            .wrapping_add(1 as libc::c_int as libc::c_uint)
            .wrapping_mul(10 as libc::c_int as libc::c_uint)
        && (*(*new_item).hh.tbl).noexpand != 1 as libc::c_int as libc::c_uint
    {
        let mut _he_bkt: libc::c_uint = 0;
        let mut _he_bkt_i: libc::c_uint = 0;
        let mut _he_thh: *mut UT_hash_handle = 0 as *mut UT_hash_handle;
        let mut _he_hh_nxt: *mut UT_hash_handle = 0 as *mut UT_hash_handle;
        let mut _he_new_buckets: *mut UT_hash_bucket = 0 as *mut UT_hash_bucket;
        let mut _he_newbkt: *mut UT_hash_bucket = 0 as *mut UT_hash_bucket;
        _he_new_buckets = malloc(
            ((2 as libc::c_int as libc::c_uint)
                .wrapping_mul((*(*new_item).hh.tbl).num_buckets) as libc::c_ulong)
                .wrapping_mul(::std::mem::size_of::<UT_hash_bucket>() as libc::c_ulong),
        ) as *mut UT_hash_bucket;
        if _he_new_buckets.is_null() {
            exit(-(1 as libc::c_int));
        }
        memset(
            _he_new_buckets as *mut libc::c_void,
            0 as libc::c_int,
            ((2 as libc::c_int as libc::c_uint)
                .wrapping_mul((*(*new_item).hh.tbl).num_buckets) as libc::c_ulong)
                .wrapping_mul(::std::mem::size_of::<UT_hash_bucket>() as libc::c_ulong),
        );
        (*(*new_item).hh.tbl)
            .ideal_chain_maxlen = ((*(*new_item).hh.tbl).num_items
            >> ((*(*new_item).hh.tbl).log2_num_buckets)
                .wrapping_add(1 as libc::c_int as libc::c_uint))
            .wrapping_add(
                (if (*(*new_item).hh.tbl).num_items
                    & ((*(*new_item).hh.tbl).num_buckets)
                        .wrapping_mul(2 as libc::c_int as libc::c_uint)
                        .wrapping_sub(1 as libc::c_int as libc::c_uint) != 0
                {
                    1 as libc::c_int
                } else {
                    0 as libc::c_int
                }) as libc::c_uint,
            );
        (*(*new_item).hh.tbl).nonideal_items = 0 as libc::c_int as libc::c_uint;
        _he_bkt_i = 0 as libc::c_int as libc::c_uint;
        while _he_bkt_i < (*(*new_item).hh.tbl).num_buckets {
            _he_thh = (*((*(*new_item).hh.tbl).buckets).offset(_he_bkt_i as isize))
                .hh_head;
            while !_he_thh.is_null() {
                _he_hh_nxt = (*_he_thh).hh_next;
                _he_bkt = (*_he_thh).hashv
                    & ((*(*new_item).hh.tbl).num_buckets)
                        .wrapping_mul(2 as libc::c_int as libc::c_uint)
                        .wrapping_sub(1 as libc::c_int as libc::c_uint);
                _he_newbkt = &mut *_he_new_buckets.offset(_he_bkt as isize)
                    as *mut UT_hash_bucket;
                (*_he_newbkt).count = ((*_he_newbkt).count).wrapping_add(1);
                if (*_he_newbkt).count > (*(*new_item).hh.tbl).ideal_chain_maxlen {
                    (*(*new_item).hh.tbl)
                        .nonideal_items = ((*(*new_item).hh.tbl).nonideal_items)
                        .wrapping_add(1);
                    (*(*new_item).hh.tbl).nonideal_items;
                    (*_he_newbkt)
                        .expand_mult = ((*_he_newbkt).count)
                        .wrapping_div((*(*new_item).hh.tbl).ideal_chain_maxlen);
                }
                (*_he_thh).hh_prev = 0 as *mut UT_hash_handle;
                (*_he_thh).hh_next = (*_he_newbkt).hh_head;
                if !((*_he_newbkt).hh_head).is_null() {
                    (*(*_he_newbkt).hh_head).hh_prev = _he_thh;
                }
                (*_he_newbkt).hh_head = _he_thh;
                _he_thh = _he_hh_nxt;
            }
            _he_bkt_i = _he_bkt_i.wrapping_add(1);
            _he_bkt_i;
        }
        free((*(*new_item).hh.tbl).buckets as *mut libc::c_void);
        (*(*new_item).hh.tbl)
            .num_buckets = ((*(*new_item).hh.tbl).num_buckets)
            .wrapping_mul(2 as libc::c_int as libc::c_uint);
        (*(*new_item).hh.tbl)
            .log2_num_buckets = ((*(*new_item).hh.tbl).log2_num_buckets).wrapping_add(1);
        (*(*new_item).hh.tbl).log2_num_buckets;
        (*(*new_item).hh.tbl).buckets = _he_new_buckets;
        (*(*new_item).hh.tbl)
            .ineff_expands = if (*(*new_item).hh.tbl).nonideal_items
            > (*(*new_item).hh.tbl).num_items >> 1 as libc::c_int
        {
            ((*(*new_item).hh.tbl).ineff_expands)
                .wrapping_add(1 as libc::c_int as libc::c_uint)
        } else {
            0 as libc::c_int as libc::c_uint
        };
        if (*(*new_item).hh.tbl).ineff_expands > 1 as libc::c_int as libc::c_uint {
            (*(*new_item).hh.tbl).noexpand = 1 as libc::c_int as libc::c_uint;
        }
    }
    return 0 as libc::c_int;
}
unsafe extern "C" fn check_symloop_leave(mut dirkey: *mut dirkey_t) -> libc::c_int {
    let mut item_found: *mut symdir_t = 0 as *mut symdir_t;
    if (*dirkey).dev == 0 as libc::c_int as libc::c_ulong
        && (*dirkey).ino == 0 as libc::c_int as libc::c_ulong
    {
        return -(1 as libc::c_int);
    }
    let mut _hf_bkt: libc::c_uint = 0;
    let mut _hf_hashv: libc::c_uint = 0;
    item_found = 0 as *mut symdir_t;
    if !symhash.is_null() {
        let mut _hj_i: libc::c_uint = 0;
        let mut _hj_j: libc::c_uint = 0;
        let mut _hj_k: libc::c_uint = 0;
        let mut _hj_key: *mut libc::c_uchar = dirkey as *mut libc::c_uchar;
        _hf_hashv = 0xfeedbeef as libc::c_uint;
        _hj_j = 0x9e3779b9 as libc::c_uint;
        _hj_i = _hj_j;
        _hj_k = ::std::mem::size_of::<dirkey_t>() as libc::c_ulong as libc::c_uint;
        while _hj_k >= 12 as libc::c_int as libc::c_uint {
            _hj_i = _hj_i
                .wrapping_add(
                    (*_hj_key.offset(0 as libc::c_int as isize) as libc::c_uint)
                        .wrapping_add(
                            (*_hj_key.offset(1 as libc::c_int as isize) as libc::c_uint)
                                << 8 as libc::c_int,
                        )
                        .wrapping_add(
                            (*_hj_key.offset(2 as libc::c_int as isize) as libc::c_uint)
                                << 16 as libc::c_int,
                        )
                        .wrapping_add(
                            (*_hj_key.offset(3 as libc::c_int as isize) as libc::c_uint)
                                << 24 as libc::c_int,
                        ),
                );
            _hj_j = _hj_j
                .wrapping_add(
                    (*_hj_key.offset(4 as libc::c_int as isize) as libc::c_uint)
                        .wrapping_add(
                            (*_hj_key.offset(5 as libc::c_int as isize) as libc::c_uint)
                                << 8 as libc::c_int,
                        )
                        .wrapping_add(
                            (*_hj_key.offset(6 as libc::c_int as isize) as libc::c_uint)
                                << 16 as libc::c_int,
                        )
                        .wrapping_add(
                            (*_hj_key.offset(7 as libc::c_int as isize) as libc::c_uint)
                                << 24 as libc::c_int,
                        ),
                );
            _hf_hashv = _hf_hashv
                .wrapping_add(
                    (*_hj_key.offset(8 as libc::c_int as isize) as libc::c_uint)
                        .wrapping_add(
                            (*_hj_key.offset(9 as libc::c_int as isize) as libc::c_uint)
                                << 8 as libc::c_int,
                        )
                        .wrapping_add(
                            (*_hj_key.offset(10 as libc::c_int as isize) as libc::c_uint)
                                << 16 as libc::c_int,
                        )
                        .wrapping_add(
                            (*_hj_key.offset(11 as libc::c_int as isize) as libc::c_uint)
                                << 24 as libc::c_int,
                        ),
                );
            _hj_i = _hj_i.wrapping_sub(_hj_j);
            _hj_i = _hj_i.wrapping_sub(_hf_hashv);
            _hj_i ^= _hf_hashv >> 13 as libc::c_int;
            _hj_j = _hj_j.wrapping_sub(_hf_hashv);
            _hj_j = _hj_j.wrapping_sub(_hj_i);
            _hj_j ^= _hj_i << 8 as libc::c_int;
            _hf_hashv = _hf_hashv.wrapping_sub(_hj_i);
            _hf_hashv = _hf_hashv.wrapping_sub(_hj_j);
            _hf_hashv ^= _hj_j >> 13 as libc::c_int;
            _hj_i = _hj_i.wrapping_sub(_hj_j);
            _hj_i = _hj_i.wrapping_sub(_hf_hashv);
            _hj_i ^= _hf_hashv >> 12 as libc::c_int;
            _hj_j = _hj_j.wrapping_sub(_hf_hashv);
            _hj_j = _hj_j.wrapping_sub(_hj_i);
            _hj_j ^= _hj_i << 16 as libc::c_int;
            _hf_hashv = _hf_hashv.wrapping_sub(_hj_i);
            _hf_hashv = _hf_hashv.wrapping_sub(_hj_j);
            _hf_hashv ^= _hj_j >> 5 as libc::c_int;
            _hj_i = _hj_i.wrapping_sub(_hj_j);
            _hj_i = _hj_i.wrapping_sub(_hf_hashv);
            _hj_i ^= _hf_hashv >> 3 as libc::c_int;
            _hj_j = _hj_j.wrapping_sub(_hf_hashv);
            _hj_j = _hj_j.wrapping_sub(_hj_i);
            _hj_j ^= _hj_i << 10 as libc::c_int;
            _hf_hashv = _hf_hashv.wrapping_sub(_hj_i);
            _hf_hashv = _hf_hashv.wrapping_sub(_hj_j);
            _hf_hashv ^= _hj_j >> 15 as libc::c_int;
            _hj_key = _hj_key.offset(12 as libc::c_int as isize);
            _hj_k = _hj_k.wrapping_sub(12 as libc::c_int as libc::c_uint);
        }
        _hf_hashv = (_hf_hashv as libc::c_ulong)
            .wrapping_add(::std::mem::size_of::<dirkey_t>() as libc::c_ulong)
            as libc::c_uint as libc::c_uint;
        let mut current_block_54: u64;
        match _hj_k {
            11 => {
                _hf_hashv = _hf_hashv
                    .wrapping_add(
                        (*_hj_key.offset(10 as libc::c_int as isize) as libc::c_uint)
                            << 24 as libc::c_int,
                    );
                current_block_54 = 13915045319470059865;
            }
            10 => {
                current_block_54 = 13915045319470059865;
            }
            9 => {
                current_block_54 = 2769307282219457803;
            }
            8 => {
                current_block_54 = 10155705865254155418;
            }
            7 => {
                current_block_54 = 15288247840664631514;
            }
            6 => {
                current_block_54 = 14098920636698011777;
            }
            5 => {
                current_block_54 = 1837246113495745747;
            }
            4 => {
                current_block_54 = 17475575970274781643;
            }
            3 => {
                current_block_54 = 11295297480669280471;
            }
            2 => {
                current_block_54 = 11772310614980210932;
            }
            1 => {
                current_block_54 = 9898137967136635421;
            }
            _ => {
                current_block_54 = 1847472278776910194;
            }
        }
        match current_block_54 {
            13915045319470059865 => {
                _hf_hashv = _hf_hashv
                    .wrapping_add(
                        (*_hj_key.offset(9 as libc::c_int as isize) as libc::c_uint)
                            << 16 as libc::c_int,
                    );
                current_block_54 = 2769307282219457803;
            }
            _ => {}
        }
        match current_block_54 {
            2769307282219457803 => {
                _hf_hashv = _hf_hashv
                    .wrapping_add(
                        (*_hj_key.offset(8 as libc::c_int as isize) as libc::c_uint)
                            << 8 as libc::c_int,
                    );
                current_block_54 = 10155705865254155418;
            }
            _ => {}
        }
        match current_block_54 {
            10155705865254155418 => {
                _hj_j = _hj_j
                    .wrapping_add(
                        (*_hj_key.offset(7 as libc::c_int as isize) as libc::c_uint)
                            << 24 as libc::c_int,
                    );
                current_block_54 = 15288247840664631514;
            }
            _ => {}
        }
        match current_block_54 {
            15288247840664631514 => {
                _hj_j = _hj_j
                    .wrapping_add(
                        (*_hj_key.offset(6 as libc::c_int as isize) as libc::c_uint)
                            << 16 as libc::c_int,
                    );
                current_block_54 = 14098920636698011777;
            }
            _ => {}
        }
        match current_block_54 {
            14098920636698011777 => {
                _hj_j = _hj_j
                    .wrapping_add(
                        (*_hj_key.offset(5 as libc::c_int as isize) as libc::c_uint)
                            << 8 as libc::c_int,
                    );
                current_block_54 = 1837246113495745747;
            }
            _ => {}
        }
        match current_block_54 {
            1837246113495745747 => {
                _hj_j = _hj_j
                    .wrapping_add(
                        *_hj_key.offset(4 as libc::c_int as isize) as libc::c_uint,
                    );
                current_block_54 = 17475575970274781643;
            }
            _ => {}
        }
        match current_block_54 {
            17475575970274781643 => {
                _hj_i = _hj_i
                    .wrapping_add(
                        (*_hj_key.offset(3 as libc::c_int as isize) as libc::c_uint)
                            << 24 as libc::c_int,
                    );
                current_block_54 = 11295297480669280471;
            }
            _ => {}
        }
        match current_block_54 {
            11295297480669280471 => {
                _hj_i = _hj_i
                    .wrapping_add(
                        (*_hj_key.offset(2 as libc::c_int as isize) as libc::c_uint)
                            << 16 as libc::c_int,
                    );
                current_block_54 = 11772310614980210932;
            }
            _ => {}
        }
        match current_block_54 {
            11772310614980210932 => {
                _hj_i = _hj_i
                    .wrapping_add(
                        (*_hj_key.offset(1 as libc::c_int as isize) as libc::c_uint)
                            << 8 as libc::c_int,
                    );
                current_block_54 = 9898137967136635421;
            }
            _ => {}
        }
        match current_block_54 {
            9898137967136635421 => {
                _hj_i = _hj_i
                    .wrapping_add(
                        *_hj_key.offset(0 as libc::c_int as isize) as libc::c_uint,
                    );
            }
            _ => {}
        }
        _hj_i = _hj_i.wrapping_sub(_hj_j);
        _hj_i = _hj_i.wrapping_sub(_hf_hashv);
        _hj_i ^= _hf_hashv >> 13 as libc::c_int;
        _hj_j = _hj_j.wrapping_sub(_hf_hashv);
        _hj_j = _hj_j.wrapping_sub(_hj_i);
        _hj_j ^= _hj_i << 8 as libc::c_int;
        _hf_hashv = _hf_hashv.wrapping_sub(_hj_i);
        _hf_hashv = _hf_hashv.wrapping_sub(_hj_j);
        _hf_hashv ^= _hj_j >> 13 as libc::c_int;
        _hj_i = _hj_i.wrapping_sub(_hj_j);
        _hj_i = _hj_i.wrapping_sub(_hf_hashv);
        _hj_i ^= _hf_hashv >> 12 as libc::c_int;
        _hj_j = _hj_j.wrapping_sub(_hf_hashv);
        _hj_j = _hj_j.wrapping_sub(_hj_i);
        _hj_j ^= _hj_i << 16 as libc::c_int;
        _hf_hashv = _hf_hashv.wrapping_sub(_hj_i);
        _hf_hashv = _hf_hashv.wrapping_sub(_hj_j);
        _hf_hashv ^= _hj_j >> 5 as libc::c_int;
        _hj_i = _hj_i.wrapping_sub(_hj_j);
        _hj_i = _hj_i.wrapping_sub(_hf_hashv);
        _hj_i ^= _hf_hashv >> 3 as libc::c_int;
        _hj_j = _hj_j.wrapping_sub(_hf_hashv);
        _hj_j = _hj_j.wrapping_sub(_hj_i);
        _hj_j ^= _hj_i << 10 as libc::c_int;
        _hf_hashv = _hf_hashv.wrapping_sub(_hj_i);
        _hf_hashv = _hf_hashv.wrapping_sub(_hj_j);
        _hf_hashv ^= _hj_j >> 15 as libc::c_int;
        _hf_bkt = _hf_hashv
            & ((*(*symhash).hh.tbl).num_buckets)
                .wrapping_sub(1 as libc::c_int as libc::c_uint);
        if !((*((*(*symhash).hh.tbl).buckets).offset(_hf_bkt as isize)).hh_head)
            .is_null()
        {
            item_found = ((*((*(*symhash).hh.tbl).buckets).offset(_hf_bkt as isize))
                .hh_head as *mut libc::c_char)
                .offset(-((*(*symhash).hh.tbl).hho as isize)) as *mut libc::c_void
                as *mut symdir_t;
        } else {
            item_found = 0 as *mut symdir_t;
        }
        while !item_found.is_null() {
            if (*item_found).hh.keylen as libc::c_ulong
                == ::std::mem::size_of::<dirkey_t>() as libc::c_ulong
            {
                if memcmp(
                    (*item_found).hh.key,
                    dirkey as *const libc::c_void,
                    ::std::mem::size_of::<dirkey_t>() as libc::c_ulong,
                ) == 0 as libc::c_int
                {
                    break;
                }
            }
            if !((*item_found).hh.hh_next).is_null() {
                item_found = ((*item_found).hh.hh_next as *mut libc::c_char)
                    .offset(-((*(*symhash).hh.tbl).hho as isize)) as *mut libc::c_void
                    as *mut symdir_t;
            } else {
                item_found = 0 as *mut symdir_t;
            }
        }
    }
    if item_found.is_null() {
        log_err(
            b"item not found! weird stuff...\n\0" as *const u8 as *const libc::c_char,
        );
        return -(1 as libc::c_int);
    }
    let mut _hd_bkt: libc::c_uint = 0;
    let mut _hd_hh_del: *mut UT_hash_handle = 0 as *mut UT_hash_handle;
    if ((*item_found).hh.prev).is_null() && ((*item_found).hh.next).is_null() {
        free((*(*symhash).hh.tbl).buckets as *mut libc::c_void);
        free((*symhash).hh.tbl as *mut libc::c_void);
        symhash = 0 as *mut symdir_t;
    } else {
        _hd_hh_del = &mut (*item_found).hh;
        if item_found
            == ((*(*symhash).hh.tbl).tail as *mut libc::c_char)
                .offset(-((*(*symhash).hh.tbl).hho as isize)) as *mut libc::c_void
                as *mut symdir_t
        {
            (*(*symhash).hh.tbl)
                .tail = ((*item_found).hh.prev as ptrdiff_t + (*(*symhash).hh.tbl).hho)
                as *mut UT_hash_handle;
        }
        if !((*item_found).hh.prev).is_null() {
            let ref mut fresh3 = (*(((*item_found).hh.prev as ptrdiff_t
                + (*(*symhash).hh.tbl).hho) as *mut UT_hash_handle))
                .next;
            *fresh3 = (*item_found).hh.next;
        } else {
            symhash = (*item_found).hh.next as *mut symdir_t;
        }
        if !((*_hd_hh_del).next).is_null() {
            let ref mut fresh4 = (*(((*_hd_hh_del).next as ptrdiff_t
                + (*(*symhash).hh.tbl).hho) as *mut UT_hash_handle))
                .prev;
            *fresh4 = (*_hd_hh_del).prev;
        }
        _hd_bkt = (*_hd_hh_del).hashv
            & ((*(*symhash).hh.tbl).num_buckets)
                .wrapping_sub(1 as libc::c_int as libc::c_uint);
        let ref mut fresh5 = (*((*(*symhash).hh.tbl).buckets).offset(_hd_bkt as isize))
            .count;
        *fresh5 = (*fresh5).wrapping_sub(1);
        *fresh5;
        if (*((*(*symhash).hh.tbl).buckets).offset(_hd_bkt as isize)).hh_head
            == _hd_hh_del
        {
            let ref mut fresh6 = (*((*(*symhash).hh.tbl).buckets)
                .offset(_hd_bkt as isize))
                .hh_head;
            *fresh6 = (*_hd_hh_del).hh_next;
        }
        if !((*_hd_hh_del).hh_prev).is_null() {
            (*(*_hd_hh_del).hh_prev).hh_next = (*_hd_hh_del).hh_next;
        }
        if !((*_hd_hh_del).hh_next).is_null() {
            (*(*_hd_hh_del).hh_next).hh_prev = (*_hd_hh_del).hh_prev;
        }
        (*(*symhash).hh.tbl)
            .num_items = ((*(*symhash).hh.tbl).num_items).wrapping_sub(1);
        (*(*symhash).hh.tbl).num_items;
    }
    free(item_found as *mut libc::c_void);
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn search_dir(
    mut ig: *mut ignores,
    mut base_path: *const libc::c_char,
    mut path: *const libc::c_char,
    depth: libc::c_int,
    mut original_dev: dev_t,
) {
    let mut offset_vector: [libc::c_int; 3] = [0; 3];
    let mut rc: libc::c_int = 0;
    let mut queue_item: *mut work_queue_t = 0 as *mut work_queue_t;
    let mut current_block: u64;
    let mut dir_list: *mut *mut dirent = 0 as *mut *mut dirent;
    let mut dir: *mut dirent = 0 as *mut dirent;
    let mut scandir_baton: scandir_baton_t = scandir_baton_t {
        ig: 0 as *const ignores,
        base_path: 0 as *const libc::c_char,
        base_path_len: 0,
        path_start: 0 as *const libc::c_char,
    };
    let mut results: libc::c_int = 0 as libc::c_int;
    let mut base_path_len: size_t = 0 as libc::c_int as size_t;
    let mut path_start: *const libc::c_char = path;
    let mut dir_full_path: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut ignore_file: *const libc::c_char = 0 as *const libc::c_char;
    let mut i: libc::c_int = 0;
    let mut symres: libc::c_int = 0;
    let mut current_dirkey: dirkey_t = dirkey_t { dev: 0, ino: 0 };
    symres = check_symloop_enter(path, &mut current_dirkey);
    if symres == 1 as libc::c_int {
        log_err(
            b"Recursive directory loop: %s\0" as *const u8 as *const libc::c_char,
            path,
        );
        return;
    }
    i = 0 as libc::c_int;
    while if opts.skip_vcs_ignores != 0 {
        (i == 0 as libc::c_int) as libc::c_int
    } else {
        (*ignore_pattern_files.as_mut_ptr().offset(i as isize)
            != 0 as *mut libc::c_void as *const libc::c_char) as libc::c_int
    } != 0
    {
        ignore_file = *ignore_pattern_files.as_mut_ptr().offset(i as isize);
        ag_asprintf(
            &mut dir_full_path as *mut *mut libc::c_char,
            b"%s/%s\0" as *const u8 as *const libc::c_char,
            path,
            ignore_file,
        );
        load_ignore_patterns(ig, dir_full_path);
        free(dir_full_path as *mut libc::c_void);
        dir_full_path = 0 as *mut libc::c_char;
        i += 1;
        i;
    }
    base_path_len = if !base_path.is_null() {
        strlen(base_path)
    } else {
        0 as libc::c_int as libc::c_ulong
    };
    i = 0 as libc::c_int;
    while (i as size_t) < base_path_len && *path.offset(i as isize) as libc::c_int != 0
        && *base_path.offset(i as isize) as libc::c_int
            == *path.offset(i as isize) as libc::c_int
    {
        path_start = path.offset(i as isize).offset(1 as libc::c_int as isize);
        i += 1;
        i;
    }
    log_debug(
        b"search_dir: path is '%s', base_path is '%s', path_start is '%s'\0" as *const u8
            as *const libc::c_char,
        path,
        base_path,
        path_start,
    );
    scandir_baton.ig = ig;
    scandir_baton.base_path = base_path;
    scandir_baton.base_path_len = base_path_len;
    scandir_baton.path_start = path_start;
    results = ag_scandir(
        path,
        &mut dir_list,
        Some(
            filename_filter
                as unsafe extern "C" fn(
                    *const libc::c_char,
                    *const dirent,
                    *mut libc::c_void,
                ) -> libc::c_int,
        ),
        &mut scandir_baton as *mut scandir_baton_t as *mut libc::c_void,
    );
    if results == 0 as libc::c_int {
        log_debug(
            b"No results found in directory %s\0" as *const u8 as *const libc::c_char,
            path,
        );
    } else if results == -(1 as libc::c_int) {
        if *__errno_location() == 20 as libc::c_int {
            if depth == 0 as libc::c_int && opts.paths_len == 1 as libc::c_int {
                if opts.print_path == PATH_PRINT_DEFAULT as libc::c_int
                    || opts.print_path == PATH_PRINT_DEFAULT_EACH_LINE as libc::c_int
                {
                    opts.print_path = PATH_PRINT_NOTHING as libc::c_int;
                }
                if opts.only_matching != 0
                    && opts.print_path == PATH_PRINT_NOTHING as libc::c_int
                {
                    opts.print_line_numbers = 0 as libc::c_int;
                }
            }
            search_file(path);
        } else {
            log_err(
                b"Error opening directory %s: %s\0" as *const u8 as *const libc::c_char,
                path,
                strerror(*__errno_location()),
            );
        }
    } else {
        offset_vector = [0; 3];
        rc = 0 as libc::c_int;
        queue_item = 0 as *mut work_queue_t;
        i = 0 as libc::c_int;
        while i < results {
            queue_item = 0 as *mut work_queue_t;
            dir = *dir_list.offset(i as isize);
            ag_asprintf(
                &mut dir_full_path as *mut *mut libc::c_char,
                b"%s/%s\0" as *const u8 as *const libc::c_char,
                path,
                ((*dir).d_name).as_mut_ptr(),
            );
            if opts.one_dev != 0 {
                let mut s: stat = stat {
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
                if lstat(dir_full_path, &mut s) != 0 as libc::c_int {
                    log_err(
                        b"Failed to get device information for %s. Skipping...\0"
                            as *const u8 as *const libc::c_char,
                        ((*dir).d_name).as_mut_ptr(),
                    );
                    current_block = 5497320425290250838;
                } else if s.st_dev != original_dev {
                    log_debug(
                        b"File %s crosses a device boundary (is probably a mount point.) Skipping...\0"
                            as *const u8 as *const libc::c_char,
                        ((*dir).d_name).as_mut_ptr(),
                    );
                    current_block = 5497320425290250838;
                } else {
                    current_block = 12381812505308290051;
                }
            } else {
                current_block = 12381812505308290051;
            }
            match current_block {
                12381812505308290051 => {
                    if opts.follow_symlinks == 0 && is_symlink(path, dir) != 0 {
                        log_debug(
                            b"File %s ignored becaused it's a symlink\0" as *const u8
                                as *const libc::c_char,
                            ((*dir).d_name).as_mut_ptr(),
                        );
                    } else if is_directory(path, dir) == 0 {
                        if !(opts.file_search_regex).is_null() {
                            rc = pcre_exec(
                                opts.file_search_regex,
                                0 as *const pcre_extra,
                                dir_full_path,
                                strlen(dir_full_path) as libc::c_int,
                                0 as libc::c_int,
                                0 as libc::c_int,
                                offset_vector.as_mut_ptr(),
                                3 as libc::c_int,
                            );
                            if rc < 0 as libc::c_int {
                                log_debug(
                                    b"Skipping %s due to file_search_regex.\0" as *const u8
                                        as *const libc::c_char,
                                    dir_full_path,
                                );
                                current_block = 5497320425290250838;
                            } else if opts.match_files != 0 {
                                log_debug(
                                    b"match_files: file_search_regex matched for %s.\0"
                                        as *const u8 as *const libc::c_char,
                                    dir_full_path,
                                );
                                pthread_mutex_lock(&mut print_mtx);
                                print_path(dir_full_path, opts.path_sep);
                                pthread_mutex_unlock(&mut print_mtx);
                                opts.match_found = 1 as libc::c_int;
                                current_block = 5497320425290250838;
                            } else {
                                current_block = 5235537862154438448;
                            }
                        } else {
                            current_block = 5235537862154438448;
                        }
                        match current_block {
                            5497320425290250838 => {}
                            _ => {
                                queue_item = ag_malloc(
                                    ::std::mem::size_of::<work_queue_t>() as libc::c_ulong,
                                ) as *mut work_queue_t;
                                (*queue_item).path = dir_full_path;
                                (*queue_item).next = 0 as *mut work_queue_t;
                                pthread_mutex_lock(&mut work_queue_mtx);
                                if work_queue_tail.is_null() {
                                    work_queue = queue_item;
                                } else {
                                    (*work_queue_tail).next = queue_item;
                                }
                                work_queue_tail = queue_item;
                                pthread_cond_signal(&mut files_ready);
                                pthread_mutex_unlock(&mut work_queue_mtx);
                                log_debug(
                                    b"%s added to work queue\0" as *const u8
                                        as *const libc::c_char,
                                    dir_full_path,
                                );
                            }
                        }
                    } else if opts.recurse_dirs != 0 {
                        if depth < opts.max_search_depth
                            || opts.max_search_depth == -(1 as libc::c_int)
                        {
                            log_debug(
                                b"Searching dir %s\0" as *const u8 as *const libc::c_char,
                                dir_full_path,
                            );
                            let mut child_ig: *mut ignores = 0 as *mut ignores;
                            child_ig = init_ignore(
                                ig,
                                ((*dir).d_name).as_mut_ptr(),
                                strlen(((*dir).d_name).as_mut_ptr()),
                            );
                            search_dir(
                                child_ig,
                                base_path,
                                dir_full_path,
                                depth + 1 as libc::c_int,
                                original_dev,
                            );
                            cleanup_ignore(child_ig);
                        } else if opts.max_search_depth == 25 as libc::c_int {
                            log_err(
                                b"Skipping %s. Use the --depth option to search deeper.\0"
                                    as *const u8 as *const libc::c_char,
                                dir_full_path,
                            );
                        } else {
                            log_debug(
                                b"Skipping %s. Use the --depth option to search deeper.\0"
                                    as *const u8 as *const libc::c_char,
                                dir_full_path,
                            );
                        }
                    }
                }
                _ => {}
            }
            free(dir as *mut libc::c_void);
            dir = 0 as *mut dirent;
            if queue_item.is_null() {
                free(dir_full_path as *mut libc::c_void);
                dir_full_path = 0 as *mut libc::c_char;
            }
            i += 1;
            i;
        }
    }
    check_symloop_leave(&mut current_dirkey);
    free(dir_list as *mut libc::c_void);
    dir_list = 0 as *mut *mut dirent;
}
