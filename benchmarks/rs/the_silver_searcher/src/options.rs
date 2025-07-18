use ::libc;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    pub type real_pcre;
    fn __errno_location() -> *mut libc::c_int;
    static mut stdin: *mut FILE;
    static mut stdout: *mut FILE;
    fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
    fn fread(
        _: *mut libc::c_void,
        _: libc::c_ulong,
        _: libc::c_ulong,
        _: *mut FILE,
    ) -> libc::c_ulong;
    fn feof(__stream: *mut FILE) -> libc::c_int;
    fn perror(__s: *const libc::c_char);
    fn fileno(__stream: *mut FILE) -> libc::c_int;
    fn popen(__command: *const libc::c_char, __modes: *const libc::c_char) -> *mut FILE;
    fn pclose(__stream: *mut FILE) -> libc::c_int;
    fn strtol(
        _: *const libc::c_char,
        _: *mut *mut libc::c_char,
        _: libc::c_int,
    ) -> libc::c_long;
    fn free(__ptr: *mut libc::c_void);
    fn exit(_: libc::c_int) -> !;
    fn getenv(__name: *const libc::c_char) -> *mut libc::c_char;
    fn realpath(
        __name: *const libc::c_char,
        __resolved: *mut libc::c_char,
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
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    fn __fxstat(
        __ver: libc::c_int,
        __fildes: libc::c_int,
        __stat_buf: *mut stat,
    ) -> libc::c_int;
    fn isatty(__fd: libc::c_int) -> libc::c_int;
    static mut optarg: *mut libc::c_char;
    static mut optind: libc::c_int;
    fn cleanup_ignore(ig: *mut ignores);
    static mut root_ignores: *mut ignores;
    fn add_ignore_pattern(ig: *mut ignores, pattern: *const libc::c_char);
    fn load_ignore_patterns(ig: *mut ignores, path: *const libc::c_char);
    static mut langs: [lang_spec_t; 0];
    fn get_lang_count() -> size_t;
    fn make_lang_regex(
        ext_array: *mut libc::c_char,
        num_exts: size_t,
    ) -> *mut libc::c_char;
    fn combine_file_extensions(
        extension_index: *mut size_t,
        len: size_t,
        exts: *mut *mut libc::c_char,
    ) -> size_t;
    fn log_err(fmt: *const libc::c_char, _: ...);
    fn log_debug(fmt: *const libc::c_char, _: ...);
    fn set_log_level(threshold: log_level);
    static mut pcre_free: Option::<unsafe extern "C" fn(*mut libc::c_void) -> ()>;
    fn getopt_long(
        ___argc: libc::c_int,
        ___argv: *const *mut libc::c_char,
        __shortopts: *const libc::c_char,
        __longopts: *const option,
        __longind: *mut libc::c_int,
    ) -> libc::c_int;
    static mut out_fd: *mut FILE;
    fn ag_malloc(size: size_t) -> *mut libc::c_void;
    fn ag_realloc(ptr: *mut libc::c_void, size: size_t) -> *mut libc::c_void;
    fn ag_calloc(nelem: size_t, elsize: size_t) -> *mut libc::c_void;
    fn ag_strdup(s: *const libc::c_char) -> *mut libc::c_char;
    fn is_regex(query: *const libc::c_char) -> libc::c_int;
    fn ag_asprintf(ret: *mut *mut libc::c_char, fmt: *const libc::c_char, _: ...);
    fn compile_study(
        re: *mut *mut pcre,
        re_extra: *mut *mut pcre_extra,
        q: *mut libc::c_char,
        pcre_opts: libc::c_int,
        study_opts: libc::c_int,
    );
    fn is_lowercase(s: *const libc::c_char) -> libc::c_int;
    fn die(fmt: *const libc::c_char, _: ...);
}
pub type size_t = libc::c_ulong;
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
pub type ino_t = __ino_t;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct lang_spec_t {
    pub name: *const libc::c_char,
    pub extensions: [*const libc::c_char; 12],
}
pub type log_level = libc::c_uint;
pub const LOG_LEVEL_NONE: log_level = 100;
pub const LOG_LEVEL_ERR: log_level = 40;
pub const LOG_LEVEL_WARN: log_level = 30;
pub const LOG_LEVEL_MSG: log_level = 20;
pub const LOG_LEVEL_DEBUG: log_level = 10;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct option {
    pub name: *const libc::c_char,
    pub has_arg: libc::c_int,
    pub flag: *mut libc::c_int,
    pub val: libc::c_int,
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
pub type option_t = option;
#[inline]
#[linkage = "external"]
pub unsafe extern "C" fn atoi(mut __nptr: *const libc::c_char) -> libc::c_int {
    return strtol(
        __nptr,
        0 as *mut libc::c_void as *mut *mut libc::c_char,
        10 as libc::c_int,
    ) as libc::c_int;
}
#[inline]
#[linkage = "external"]
pub unsafe extern "C" fn fstat(
    mut __fd: libc::c_int,
    mut __statbuf: *mut stat,
) -> libc::c_int {
    return __fxstat(1 as libc::c_int, __fd, __statbuf);
}
pub static mut color_line_number: *const libc::c_char = b"\x1B[1;33m\0" as *const u8
    as *const libc::c_char;
pub static mut color_match: *const libc::c_char = b"\x1B[30;43m\0" as *const u8
    as *const libc::c_char;
pub static mut color_path: *const libc::c_char = b"\x1B[1;32m\0" as *const u8
    as *const libc::c_char;
pub static mut opts: cli_options = cli_options {
    ackmate: 0,
    ackmate_dir_filter: 0 as *const pcre as *mut pcre,
    ackmate_dir_filter_extra: 0 as *const pcre_extra as *mut pcre_extra,
    after: 0,
    before: 0,
    casing: CASE_DEFAULT,
    file_search_string: 0 as *const libc::c_char,
    match_files: 0,
    file_search_regex: 0 as *const pcre as *mut pcre,
    file_search_regex_extra: 0 as *const pcre_extra as *mut pcre_extra,
    color: 0,
    color_line_number: 0 as *const libc::c_char as *mut libc::c_char,
    color_match: 0 as *const libc::c_char as *mut libc::c_char,
    color_path: 0 as *const libc::c_char as *mut libc::c_char,
    color_win_ansi: 0,
    column: 0,
    context: 0,
    follow_symlinks: 0,
    invert_match: 0,
    literal: 0,
    literal_starts_wordchar: 0,
    literal_ends_wordchar: 0,
    max_matches_per_file: 0,
    max_search_depth: 0,
    mmap: 0,
    multiline: 0,
    one_dev: 0,
    only_matching: 0,
    path_sep: 0,
    path_to_ignore: 0,
    print_break: 0,
    print_count: 0,
    print_filename_only: 0,
    print_nonmatching_files: 0,
    print_path: 0,
    print_all_paths: 0,
    print_line_numbers: 0,
    print_long_lines: 0,
    passthrough: 0,
    re: 0 as *const pcre as *mut pcre,
    re_extra: 0 as *const pcre_extra as *mut pcre_extra,
    recurse_dirs: 0,
    search_all_files: 0,
    skip_vcs_ignores: 0,
    search_binary_files: 0,
    search_zip_files: 0,
    search_hidden_files: 0,
    search_stream: 0,
    stats: 0,
    stream_line_num: 0,
    match_found: 0,
    stdout_inode: 0,
    query: 0 as *const libc::c_char as *mut libc::c_char,
    query_len: 0,
    pager: 0 as *const libc::c_char as *mut libc::c_char,
    paths_len: 0,
    parallel: 0,
    use_thread_affinity: 0,
    vimgrep: 0,
    width: 0,
    word_regexp: 0,
    workers: 0,
};
pub unsafe extern "C" fn usage() {
    printf(b"\n\0" as *const u8 as *const libc::c_char);
    printf(
        b"Usage: ag [FILE-TYPE] [OPTIONS] PATTERN [PATH]\n\n\0" as *const u8
            as *const libc::c_char,
    );
    printf(
        b"  Recursively search for PATTERN in PATH.\n\0" as *const u8
            as *const libc::c_char,
    );
    printf(b"  Like grep or ack, but faster.\n\n\0" as *const u8 as *const libc::c_char);
    printf(b"Example:\n  ag -i foo /bar/\n\n\0" as *const u8 as *const libc::c_char);
    printf(
        b"Output Options:\n     --ackmate            Print results in AckMate-parseable format\n  -A --after [LINES]      Print lines after match (Default: 2)\n  -B --before [LINES]     Print lines before match (Default: 2)\n     --[no]break          Print newlines between matches in different files\n                          (Enabled by default)\n  -c --count              Only print the number of matches in each file.\n                          (This often differs from the number of matching lines)\n     --[no]color          Print color codes in results (Enabled by default)\n     --color-line-number  Color codes for line numbers (Default: 1;33)\n     --color-match        Color codes for result match numbers (Default: 30;43)\n     --color-path         Color codes for path names (Default: 1;32)\n\0"
            as *const u8 as *const libc::c_char,
    );
    printf(
        b"     --column             Print column numbers in results\n     --[no]filename       Print file names (Enabled unless searching a single file)\n  -H --[no]heading        Print file names before each file's matches\n                          (Enabled by default)\n  -C --context [LINES]    Print lines before and after matches (Default: 2)\n     --[no]group          Same as --[no]break --[no]heading\n  -g --filename-pattern PATTERN\n                          Print filenames matching PATTERN\n  -l --files-with-matches Only print filenames that contain matches\n                          (don't print the matching lines)\n  -L --files-without-matches\n                          Only print filenames that don't contain matches\n     --print-all-files    Print headings for all files searched, even those that\n                          don't contain matches\n     --[no]numbers        Print line numbers. Default is to omit line numbers\n                          when searching streams\n  -o --only-matching      Prints only the matching part of the lines\n     --print-long-lines   Print matches on very long lines (Default: >2k characters)\n     --passthrough        When searching a stream, print all lines even if they\n                          don't match\n     --silent             Suppress all log messages, including errors\n     --stats              Print stats (files scanned, time taken, etc.)\n     --stats-only         Print stats and nothing else.\n                          (Same as --count when searching a single file)\n     --vimgrep            Print results like vim's :vimgrep /pattern/g would\n                          (it reports every match on the line)\n  -0 --null --print0      Separate filenames with null (for 'xargs -0')\n\nSearch Options:\n  -a --all-types          Search all files (doesn't include hidden files\n                          or patterns from ignore files)\n  -D --debug              Ridiculous debugging (probably not useful)\n     --depth NUM          Search up to NUM directories deep (Default: 25)\n  -f --follow             Follow symlinks\n  -F --fixed-strings      Alias for --literal for compatibility with grep\n  -G --file-search-regex  PATTERN Limit search to filenames matching PATTERN\n     --hidden             Search hidden files (obeys .*ignore files)\n  -i --ignore-case        Match case insensitively\n     --ignore PATTERN     Ignore files/directories matching PATTERN\n                          (literal file/directory names also allowed)\n     --ignore-dir NAME    Alias for --ignore for compatibility with ack.\n  -m --max-count NUM      Skip the rest of a file after NUM matches (Default: 10,000)\n     --one-device         Don't follow links to other devices.\n  -p --path-to-ignore STRING\n                          Use .ignore file at STRING\n  -Q --literal            Don't parse PATTERN as a regular expression\n  -s --case-sensitive     Match case sensitively\n  -S --smart-case         Match case insensitively unless PATTERN contains\n                          uppercase characters (Enabled by default)\n     --search-binary      Search binary files for matches\n  -t --all-text           Search all text files (doesn't include hidden files)\n  -u --unrestricted       Search all files (ignore .ignore, .gitignore, etc.;\n                          searches binary and hidden files as well)\n  -U --skip-vcs-ignores   Ignore VCS ignore files\n                          (.gitignore, .hgignore; still obey .ignore)\n  -v --invert-match\n  -w --word-regexp        Only match whole words\n  -W --width NUM          Truncate match lines after NUM characters\n  -z --search-zip         Search contents of compressed (e.g., gzip) files\n\n\0"
            as *const u8 as *const libc::c_char,
    );
    printf(
        b"File Types:\nThe search can be restricted to certain types of files. Example:\n  ag --html needle\n  - Searches for 'needle' in files with suffix .htm, .html, .shtml or .xhtml.\n\nFor a list of supported file types run:\n  ag --list-file-types\n\nag was originally created by Geoff Greer. More information (and the latest release)\ncan be found at http://geoff.greer.fm/ag\n\0"
            as *const u8 as *const libc::c_char,
    );
}
pub unsafe extern "C" fn print_version() {
    let mut jit: libc::c_char = '-' as i32 as libc::c_char;
    let mut lzma: libc::c_char = '-' as i32 as libc::c_char;
    let mut zlib: libc::c_char = '-' as i32 as libc::c_char;
    jit = '+' as i32 as libc::c_char;
    lzma = '+' as i32 as libc::c_char;
    zlib = '+' as i32 as libc::c_char;
    printf(
        b"ag version %s\n\n\0" as *const u8 as *const libc::c_char,
        b"2.2.0\0" as *const u8 as *const libc::c_char,
    );
    printf(b"Features:\n\0" as *const u8 as *const libc::c_char);
    printf(
        b"  %cjit %clzma %czlib\n\0" as *const u8 as *const libc::c_char,
        jit as libc::c_int,
        lzma as libc::c_int,
        zlib as libc::c_int,
    );
}
pub unsafe extern "C" fn init_options() {
    let mut term: *mut libc::c_char = getenv(
        b"TERM\0" as *const u8 as *const libc::c_char,
    );
    memset(
        &mut opts as *mut cli_options as *mut libc::c_void,
        0 as libc::c_int,
        ::std::mem::size_of::<cli_options>() as libc::c_ulong,
    );
    opts.casing = CASE_DEFAULT;
    opts.color = 1 as libc::c_int;
    if !term.is_null()
        && strcmp(term, b"dumb\0" as *const u8 as *const libc::c_char) == 0
    {
        opts.color = 0 as libc::c_int;
    }
    opts.color_win_ansi = 0 as libc::c_int;
    opts.max_matches_per_file = 0 as libc::c_int as size_t;
    opts.max_search_depth = 25 as libc::c_int;
    opts.mmap = 1 as libc::c_int;
    opts.multiline = 1 as libc::c_int;
    opts.width = 0 as libc::c_int as size_t;
    opts.path_sep = '\n' as i32 as libc::c_char;
    opts.print_break = 1 as libc::c_int;
    opts.print_path = PATH_PRINT_DEFAULT as libc::c_int;
    opts.print_all_paths = 0 as libc::c_int;
    opts.print_line_numbers = 1 as libc::c_int;
    opts.recurse_dirs = 1 as libc::c_int;
    opts.color_path = ag_strdup(color_path);
    opts.color_match = ag_strdup(color_match);
    opts.color_line_number = ag_strdup(color_line_number);
    opts.use_thread_affinity = 1 as libc::c_int;
}
pub unsafe extern "C" fn cleanup_options() {
    free(opts.color_path as *mut libc::c_void);
    free(opts.color_match as *mut libc::c_void);
    free(opts.color_line_number as *mut libc::c_void);
    if !(opts.query).is_null() {
        free(opts.query as *mut libc::c_void);
    }
    pcre_free.unwrap()(opts.re as *mut libc::c_void);
    if !(opts.re_extra).is_null() {
        pcre_free.unwrap()(opts.re_extra as *mut libc::c_void);
    }
    if !(opts.ackmate_dir_filter).is_null() {
        pcre_free.unwrap()(opts.ackmate_dir_filter as *mut libc::c_void);
    }
    if !(opts.ackmate_dir_filter_extra).is_null() {
        pcre_free.unwrap()(opts.ackmate_dir_filter_extra as *mut libc::c_void);
    }
    if !(opts.file_search_regex).is_null() {
        pcre_free.unwrap()(opts.file_search_regex as *mut libc::c_void);
    }
    if !(opts.file_search_regex_extra).is_null() {
        pcre_free.unwrap()(opts.file_search_regex_extra as *mut libc::c_void);
    }
}
pub unsafe extern "C" fn parse_options(
    mut argc: libc::c_int,
    mut argv: *mut *mut libc::c_char,
    mut base_paths: *mut *mut *mut libc::c_char,
    mut paths: *mut *mut *mut libc::c_char,
) {
    let mut ch: libc::c_int = 0;
    let mut i: size_t = 0;
    let mut path_len: libc::c_int = 0 as libc::c_int;
    let mut base_path_len: libc::c_int = 0 as libc::c_int;
    let mut useless: libc::c_int = 0 as libc::c_int;
    let mut group: libc::c_int = 1 as libc::c_int;
    let mut help: libc::c_int = 0 as libc::c_int;
    let mut version: libc::c_int = 0 as libc::c_int;
    let mut list_file_types: libc::c_int = 0 as libc::c_int;
    let mut opt_index: libc::c_int = 0 as libc::c_int;
    let mut num_end: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut home_dir: *const libc::c_char = getenv(
        b"HOME\0" as *const u8 as *const libc::c_char,
    );
    let mut ignore_file_path: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut accepts_query: libc::c_int = 1 as libc::c_int;
    let mut needs_query: libc::c_int = 1 as libc::c_int;
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
    let mut rv: libc::c_int = 0;
    let mut lang_count: size_t = 0;
    let mut lang_num: size_t = 0 as libc::c_int as size_t;
    let mut has_filetype: libc::c_int = 0 as libc::c_int;
    let mut longopts_len: size_t = 0;
    let mut full_len: size_t = 0;
    let mut longopts: *mut option_t = 0 as *mut option_t;
    let mut lang_regex: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut ext_index: *mut size_t = 0 as *mut size_t;
    let mut extensions: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut num_exts: size_t = 0 as libc::c_int as size_t;
    init_options();
    let mut base_longopts: [option_t; 92] = [
        {
            let mut init = option {
                name: b"ackmate\0" as *const u8 as *const libc::c_char,
                has_arg: 0 as libc::c_int,
                flag: &mut opts.ackmate,
                val: 1 as libc::c_int,
            };
            init
        },
        {
            let mut init = option {
                name: b"ackmate-dir-filter\0" as *const u8 as *const libc::c_char,
                has_arg: 1 as libc::c_int,
                flag: 0 as *mut libc::c_int,
                val: 0 as libc::c_int,
            };
            init
        },
        {
            let mut init = option {
                name: b"affinity\0" as *const u8 as *const libc::c_char,
                has_arg: 0 as libc::c_int,
                flag: &mut opts.use_thread_affinity,
                val: 1 as libc::c_int,
            };
            init
        },
        {
            let mut init = option {
                name: b"after\0" as *const u8 as *const libc::c_char,
                has_arg: 2 as libc::c_int,
                flag: 0 as *mut libc::c_int,
                val: 'A' as i32,
            };
            init
        },
        {
            let mut init = option {
                name: b"all-text\0" as *const u8 as *const libc::c_char,
                has_arg: 0 as libc::c_int,
                flag: 0 as *mut libc::c_int,
                val: 't' as i32,
            };
            init
        },
        {
            let mut init = option {
                name: b"all-types\0" as *const u8 as *const libc::c_char,
                has_arg: 0 as libc::c_int,
                flag: 0 as *mut libc::c_int,
                val: 'a' as i32,
            };
            init
        },
        {
            let mut init = option {
                name: b"before\0" as *const u8 as *const libc::c_char,
                has_arg: 2 as libc::c_int,
                flag: 0 as *mut libc::c_int,
                val: 'B' as i32,
            };
            init
        },
        {
            let mut init = option {
                name: b"break\0" as *const u8 as *const libc::c_char,
                has_arg: 0 as libc::c_int,
                flag: &mut opts.print_break,
                val: 1 as libc::c_int,
            };
            init
        },
        {
            let mut init = option {
                name: b"case-sensitive\0" as *const u8 as *const libc::c_char,
                has_arg: 0 as libc::c_int,
                flag: 0 as *mut libc::c_int,
                val: 's' as i32,
            };
            init
        },
        {
            let mut init = option {
                name: b"color\0" as *const u8 as *const libc::c_char,
                has_arg: 0 as libc::c_int,
                flag: &mut opts.color,
                val: 1 as libc::c_int,
            };
            init
        },
        {
            let mut init = option {
                name: b"color-line-number\0" as *const u8 as *const libc::c_char,
                has_arg: 1 as libc::c_int,
                flag: 0 as *mut libc::c_int,
                val: 0 as libc::c_int,
            };
            init
        },
        {
            let mut init = option {
                name: b"color-match\0" as *const u8 as *const libc::c_char,
                has_arg: 1 as libc::c_int,
                flag: 0 as *mut libc::c_int,
                val: 0 as libc::c_int,
            };
            init
        },
        {
            let mut init = option {
                name: b"color-path\0" as *const u8 as *const libc::c_char,
                has_arg: 1 as libc::c_int,
                flag: 0 as *mut libc::c_int,
                val: 0 as libc::c_int,
            };
            init
        },
        {
            let mut init = option {
                name: b"color-win-ansi\0" as *const u8 as *const libc::c_char,
                has_arg: 0 as libc::c_int,
                flag: &mut opts.color_win_ansi,
                val: 1 as libc::c_int,
            };
            init
        },
        {
            let mut init = option {
                name: b"column\0" as *const u8 as *const libc::c_char,
                has_arg: 0 as libc::c_int,
                flag: &mut opts.column,
                val: 1 as libc::c_int,
            };
            init
        },
        {
            let mut init = option {
                name: b"context\0" as *const u8 as *const libc::c_char,
                has_arg: 2 as libc::c_int,
                flag: 0 as *mut libc::c_int,
                val: 'C' as i32,
            };
            init
        },
        {
            let mut init = option {
                name: b"count\0" as *const u8 as *const libc::c_char,
                has_arg: 0 as libc::c_int,
                flag: 0 as *mut libc::c_int,
                val: 'c' as i32,
            };
            init
        },
        {
            let mut init = option {
                name: b"debug\0" as *const u8 as *const libc::c_char,
                has_arg: 0 as libc::c_int,
                flag: 0 as *mut libc::c_int,
                val: 'D' as i32,
            };
            init
        },
        {
            let mut init = option {
                name: b"depth\0" as *const u8 as *const libc::c_char,
                has_arg: 1 as libc::c_int,
                flag: 0 as *mut libc::c_int,
                val: 0 as libc::c_int,
            };
            init
        },
        {
            let mut init = option {
                name: b"filename\0" as *const u8 as *const libc::c_char,
                has_arg: 0 as libc::c_int,
                flag: 0 as *mut libc::c_int,
                val: 0 as libc::c_int,
            };
            init
        },
        {
            let mut init = option {
                name: b"filename-pattern\0" as *const u8 as *const libc::c_char,
                has_arg: 1 as libc::c_int,
                flag: 0 as *mut libc::c_int,
                val: 'g' as i32,
            };
            init
        },
        {
            let mut init = option {
                name: b"file-search-regex\0" as *const u8 as *const libc::c_char,
                has_arg: 1 as libc::c_int,
                flag: 0 as *mut libc::c_int,
                val: 'G' as i32,
            };
            init
        },
        {
            let mut init = option {
                name: b"files-with-matches\0" as *const u8 as *const libc::c_char,
                has_arg: 0 as libc::c_int,
                flag: 0 as *mut libc::c_int,
                val: 'l' as i32,
            };
            init
        },
        {
            let mut init = option {
                name: b"files-without-matches\0" as *const u8 as *const libc::c_char,
                has_arg: 0 as libc::c_int,
                flag: 0 as *mut libc::c_int,
                val: 'L' as i32,
            };
            init
        },
        {
            let mut init = option {
                name: b"fixed-strings\0" as *const u8 as *const libc::c_char,
                has_arg: 0 as libc::c_int,
                flag: 0 as *mut libc::c_int,
                val: 'F' as i32,
            };
            init
        },
        {
            let mut init = option {
                name: b"follow\0" as *const u8 as *const libc::c_char,
                has_arg: 0 as libc::c_int,
                flag: &mut opts.follow_symlinks,
                val: 1 as libc::c_int,
            };
            init
        },
        {
            let mut init = option {
                name: b"group\0" as *const u8 as *const libc::c_char,
                has_arg: 0 as libc::c_int,
                flag: &mut group,
                val: 1 as libc::c_int,
            };
            init
        },
        {
            let mut init = option {
                name: b"heading\0" as *const u8 as *const libc::c_char,
                has_arg: 0 as libc::c_int,
                flag: &mut opts.print_path,
                val: PATH_PRINT_TOP as libc::c_int,
            };
            init
        },
        {
            let mut init = option {
                name: b"help\0" as *const u8 as *const libc::c_char,
                has_arg: 0 as libc::c_int,
                flag: 0 as *mut libc::c_int,
                val: 'h' as i32,
            };
            init
        },
        {
            let mut init = option {
                name: b"hidden\0" as *const u8 as *const libc::c_char,
                has_arg: 0 as libc::c_int,
                flag: &mut opts.search_hidden_files,
                val: 1 as libc::c_int,
            };
            init
        },
        {
            let mut init = option {
                name: b"ignore\0" as *const u8 as *const libc::c_char,
                has_arg: 1 as libc::c_int,
                flag: 0 as *mut libc::c_int,
                val: 0 as libc::c_int,
            };
            init
        },
        {
            let mut init = option {
                name: b"ignore-case\0" as *const u8 as *const libc::c_char,
                has_arg: 0 as libc::c_int,
                flag: 0 as *mut libc::c_int,
                val: 'i' as i32,
            };
            init
        },
        {
            let mut init = option {
                name: b"ignore-dir\0" as *const u8 as *const libc::c_char,
                has_arg: 1 as libc::c_int,
                flag: 0 as *mut libc::c_int,
                val: 0 as libc::c_int,
            };
            init
        },
        {
            let mut init = option {
                name: b"invert-match\0" as *const u8 as *const libc::c_char,
                has_arg: 0 as libc::c_int,
                flag: 0 as *mut libc::c_int,
                val: 'v' as i32,
            };
            init
        },
        {
            let mut init = option {
                name: b"line-numbers\0" as *const u8 as *const libc::c_char,
                has_arg: 0 as libc::c_int,
                flag: &mut opts.print_line_numbers,
                val: 2 as libc::c_int,
            };
            init
        },
        {
            let mut init = option {
                name: b"list-file-types\0" as *const u8 as *const libc::c_char,
                has_arg: 0 as libc::c_int,
                flag: &mut list_file_types,
                val: 1 as libc::c_int,
            };
            init
        },
        {
            let mut init = option {
                name: b"literal\0" as *const u8 as *const libc::c_char,
                has_arg: 0 as libc::c_int,
                flag: 0 as *mut libc::c_int,
                val: 'Q' as i32,
            };
            init
        },
        {
            let mut init = option {
                name: b"match\0" as *const u8 as *const libc::c_char,
                has_arg: 0 as libc::c_int,
                flag: &mut useless,
                val: 0 as libc::c_int,
            };
            init
        },
        {
            let mut init = option {
                name: b"max-count\0" as *const u8 as *const libc::c_char,
                has_arg: 1 as libc::c_int,
                flag: 0 as *mut libc::c_int,
                val: 'm' as i32,
            };
            init
        },
        {
            let mut init = option {
                name: b"mmap\0" as *const u8 as *const libc::c_char,
                has_arg: 0 as libc::c_int,
                flag: &mut opts.mmap,
                val: 1 as libc::c_int,
            };
            init
        },
        {
            let mut init = option {
                name: b"multiline\0" as *const u8 as *const libc::c_char,
                has_arg: 0 as libc::c_int,
                flag: &mut opts.multiline,
                val: 1 as libc::c_int,
            };
            init
        },
        {
            let mut init = option {
                name: b"no-affinity\0" as *const u8 as *const libc::c_char,
                has_arg: 0 as libc::c_int,
                flag: &mut opts.use_thread_affinity,
                val: 0 as libc::c_int,
            };
            init
        },
        {
            let mut init = option {
                name: b"noaffinity\0" as *const u8 as *const libc::c_char,
                has_arg: 0 as libc::c_int,
                flag: &mut opts.use_thread_affinity,
                val: 0 as libc::c_int,
            };
            init
        },
        {
            let mut init = option {
                name: b"no-break\0" as *const u8 as *const libc::c_char,
                has_arg: 0 as libc::c_int,
                flag: &mut opts.print_break,
                val: 0 as libc::c_int,
            };
            init
        },
        {
            let mut init = option {
                name: b"nobreak\0" as *const u8 as *const libc::c_char,
                has_arg: 0 as libc::c_int,
                flag: &mut opts.print_break,
                val: 0 as libc::c_int,
            };
            init
        },
        {
            let mut init = option {
                name: b"no-color\0" as *const u8 as *const libc::c_char,
                has_arg: 0 as libc::c_int,
                flag: &mut opts.color,
                val: 0 as libc::c_int,
            };
            init
        },
        {
            let mut init = option {
                name: b"nocolor\0" as *const u8 as *const libc::c_char,
                has_arg: 0 as libc::c_int,
                flag: &mut opts.color,
                val: 0 as libc::c_int,
            };
            init
        },
        {
            let mut init = option {
                name: b"no-filename\0" as *const u8 as *const libc::c_char,
                has_arg: 0 as libc::c_int,
                flag: 0 as *mut libc::c_int,
                val: 0 as libc::c_int,
            };
            init
        },
        {
            let mut init = option {
                name: b"nofilename\0" as *const u8 as *const libc::c_char,
                has_arg: 0 as libc::c_int,
                flag: 0 as *mut libc::c_int,
                val: 0 as libc::c_int,
            };
            init
        },
        {
            let mut init = option {
                name: b"no-follow\0" as *const u8 as *const libc::c_char,
                has_arg: 0 as libc::c_int,
                flag: &mut opts.follow_symlinks,
                val: 0 as libc::c_int,
            };
            init
        },
        {
            let mut init = option {
                name: b"nofollow\0" as *const u8 as *const libc::c_char,
                has_arg: 0 as libc::c_int,
                flag: &mut opts.follow_symlinks,
                val: 0 as libc::c_int,
            };
            init
        },
        {
            let mut init = option {
                name: b"no-group\0" as *const u8 as *const libc::c_char,
                has_arg: 0 as libc::c_int,
                flag: &mut group,
                val: 0 as libc::c_int,
            };
            init
        },
        {
            let mut init = option {
                name: b"nogroup\0" as *const u8 as *const libc::c_char,
                has_arg: 0 as libc::c_int,
                flag: &mut group,
                val: 0 as libc::c_int,
            };
            init
        },
        {
            let mut init = option {
                name: b"no-heading\0" as *const u8 as *const libc::c_char,
                has_arg: 0 as libc::c_int,
                flag: &mut opts.print_path,
                val: PATH_PRINT_EACH_LINE as libc::c_int,
            };
            init
        },
        {
            let mut init = option {
                name: b"noheading\0" as *const u8 as *const libc::c_char,
                has_arg: 0 as libc::c_int,
                flag: &mut opts.print_path,
                val: PATH_PRINT_EACH_LINE as libc::c_int,
            };
            init
        },
        {
            let mut init = option {
                name: b"no-mmap\0" as *const u8 as *const libc::c_char,
                has_arg: 0 as libc::c_int,
                flag: &mut opts.mmap,
                val: 0 as libc::c_int,
            };
            init
        },
        {
            let mut init = option {
                name: b"nommap\0" as *const u8 as *const libc::c_char,
                has_arg: 0 as libc::c_int,
                flag: &mut opts.mmap,
                val: 0 as libc::c_int,
            };
            init
        },
        {
            let mut init = option {
                name: b"no-multiline\0" as *const u8 as *const libc::c_char,
                has_arg: 0 as libc::c_int,
                flag: &mut opts.multiline,
                val: 0 as libc::c_int,
            };
            init
        },
        {
            let mut init = option {
                name: b"nomultiline\0" as *const u8 as *const libc::c_char,
                has_arg: 0 as libc::c_int,
                flag: &mut opts.multiline,
                val: 0 as libc::c_int,
            };
            init
        },
        {
            let mut init = option {
                name: b"no-numbers\0" as *const u8 as *const libc::c_char,
                has_arg: 0 as libc::c_int,
                flag: &mut opts.print_line_numbers,
                val: 0 as libc::c_int,
            };
            init
        },
        {
            let mut init = option {
                name: b"nonumbers\0" as *const u8 as *const libc::c_char,
                has_arg: 0 as libc::c_int,
                flag: &mut opts.print_line_numbers,
                val: 0 as libc::c_int,
            };
            init
        },
        {
            let mut init = option {
                name: b"no-pager\0" as *const u8 as *const libc::c_char,
                has_arg: 0 as libc::c_int,
                flag: 0 as *mut libc::c_int,
                val: 0 as libc::c_int,
            };
            init
        },
        {
            let mut init = option {
                name: b"nopager\0" as *const u8 as *const libc::c_char,
                has_arg: 0 as libc::c_int,
                flag: 0 as *mut libc::c_int,
                val: 0 as libc::c_int,
            };
            init
        },
        {
            let mut init = option {
                name: b"no-recurse\0" as *const u8 as *const libc::c_char,
                has_arg: 0 as libc::c_int,
                flag: 0 as *mut libc::c_int,
                val: 'n' as i32,
            };
            init
        },
        {
            let mut init = option {
                name: b"norecurse\0" as *const u8 as *const libc::c_char,
                has_arg: 0 as libc::c_int,
                flag: 0 as *mut libc::c_int,
                val: 'n' as i32,
            };
            init
        },
        {
            let mut init = option {
                name: b"null\0" as *const u8 as *const libc::c_char,
                has_arg: 0 as libc::c_int,
                flag: 0 as *mut libc::c_int,
                val: '0' as i32,
            };
            init
        },
        {
            let mut init = option {
                name: b"numbers\0" as *const u8 as *const libc::c_char,
                has_arg: 0 as libc::c_int,
                flag: &mut opts.print_line_numbers,
                val: 2 as libc::c_int,
            };
            init
        },
        {
            let mut init = option {
                name: b"only-matching\0" as *const u8 as *const libc::c_char,
                has_arg: 0 as libc::c_int,
                flag: 0 as *mut libc::c_int,
                val: 'o' as i32,
            };
            init
        },
        {
            let mut init = option {
                name: b"one-device\0" as *const u8 as *const libc::c_char,
                has_arg: 0 as libc::c_int,
                flag: &mut opts.one_dev,
                val: 1 as libc::c_int,
            };
            init
        },
        {
            let mut init = option {
                name: b"pager\0" as *const u8 as *const libc::c_char,
                has_arg: 1 as libc::c_int,
                flag: 0 as *mut libc::c_int,
                val: 0 as libc::c_int,
            };
            init
        },
        {
            let mut init = option {
                name: b"parallel\0" as *const u8 as *const libc::c_char,
                has_arg: 0 as libc::c_int,
                flag: &mut opts.parallel,
                val: 1 as libc::c_int,
            };
            init
        },
        {
            let mut init = option {
                name: b"passthrough\0" as *const u8 as *const libc::c_char,
                has_arg: 0 as libc::c_int,
                flag: &mut opts.passthrough,
                val: 1 as libc::c_int,
            };
            init
        },
        {
            let mut init = option {
                name: b"passthru\0" as *const u8 as *const libc::c_char,
                has_arg: 0 as libc::c_int,
                flag: &mut opts.passthrough,
                val: 1 as libc::c_int,
            };
            init
        },
        {
            let mut init = option {
                name: b"path-to-ignore\0" as *const u8 as *const libc::c_char,
                has_arg: 1 as libc::c_int,
                flag: 0 as *mut libc::c_int,
                val: 'p' as i32,
            };
            init
        },
        {
            let mut init = option {
                name: b"print0\0" as *const u8 as *const libc::c_char,
                has_arg: 0 as libc::c_int,
                flag: 0 as *mut libc::c_int,
                val: '0' as i32,
            };
            init
        },
        {
            let mut init = option {
                name: b"print-all-files\0" as *const u8 as *const libc::c_char,
                has_arg: 0 as libc::c_int,
                flag: 0 as *mut libc::c_int,
                val: 0 as libc::c_int,
            };
            init
        },
        {
            let mut init = option {
                name: b"print-long-lines\0" as *const u8 as *const libc::c_char,
                has_arg: 0 as libc::c_int,
                flag: &mut opts.print_long_lines,
                val: 1 as libc::c_int,
            };
            init
        },
        {
            let mut init = option {
                name: b"recurse\0" as *const u8 as *const libc::c_char,
                has_arg: 0 as libc::c_int,
                flag: 0 as *mut libc::c_int,
                val: 'r' as i32,
            };
            init
        },
        {
            let mut init = option {
                name: b"search-binary\0" as *const u8 as *const libc::c_char,
                has_arg: 0 as libc::c_int,
                flag: &mut opts.search_binary_files,
                val: 1 as libc::c_int,
            };
            init
        },
        {
            let mut init = option {
                name: b"search-files\0" as *const u8 as *const libc::c_char,
                has_arg: 0 as libc::c_int,
                flag: &mut opts.search_stream,
                val: 0 as libc::c_int,
            };
            init
        },
        {
            let mut init = option {
                name: b"search-zip\0" as *const u8 as *const libc::c_char,
                has_arg: 0 as libc::c_int,
                flag: &mut opts.search_zip_files,
                val: 1 as libc::c_int,
            };
            init
        },
        {
            let mut init = option {
                name: b"silent\0" as *const u8 as *const libc::c_char,
                has_arg: 0 as libc::c_int,
                flag: 0 as *mut libc::c_int,
                val: 0 as libc::c_int,
            };
            init
        },
        {
            let mut init = option {
                name: b"skip-vcs-ignores\0" as *const u8 as *const libc::c_char,
                has_arg: 0 as libc::c_int,
                flag: 0 as *mut libc::c_int,
                val: 'U' as i32,
            };
            init
        },
        {
            let mut init = option {
                name: b"smart-case\0" as *const u8 as *const libc::c_char,
                has_arg: 0 as libc::c_int,
                flag: 0 as *mut libc::c_int,
                val: 'S' as i32,
            };
            init
        },
        {
            let mut init = option {
                name: b"stats\0" as *const u8 as *const libc::c_char,
                has_arg: 0 as libc::c_int,
                flag: &mut opts.stats,
                val: 1 as libc::c_int,
            };
            init
        },
        {
            let mut init = option {
                name: b"stats-only\0" as *const u8 as *const libc::c_char,
                has_arg: 0 as libc::c_int,
                flag: 0 as *mut libc::c_int,
                val: 0 as libc::c_int,
            };
            init
        },
        {
            let mut init = option {
                name: b"unrestricted\0" as *const u8 as *const libc::c_char,
                has_arg: 0 as libc::c_int,
                flag: 0 as *mut libc::c_int,
                val: 'u' as i32,
            };
            init
        },
        {
            let mut init = option {
                name: b"version\0" as *const u8 as *const libc::c_char,
                has_arg: 0 as libc::c_int,
                flag: &mut version,
                val: 1 as libc::c_int,
            };
            init
        },
        {
            let mut init = option {
                name: b"vimgrep\0" as *const u8 as *const libc::c_char,
                has_arg: 0 as libc::c_int,
                flag: &mut opts.vimgrep,
                val: 1 as libc::c_int,
            };
            init
        },
        {
            let mut init = option {
                name: b"width\0" as *const u8 as *const libc::c_char,
                has_arg: 1 as libc::c_int,
                flag: 0 as *mut libc::c_int,
                val: 'W' as i32,
            };
            init
        },
        {
            let mut init = option {
                name: b"word-regexp\0" as *const u8 as *const libc::c_char,
                has_arg: 0 as libc::c_int,
                flag: 0 as *mut libc::c_int,
                val: 'w' as i32,
            };
            init
        },
        {
            let mut init = option {
                name: b"workers\0" as *const u8 as *const libc::c_char,
                has_arg: 1 as libc::c_int,
                flag: 0 as *mut libc::c_int,
                val: 0 as libc::c_int,
            };
            init
        },
    ];
    lang_count = get_lang_count();
    longopts_len = (::std::mem::size_of::<[option_t; 92]>() as libc::c_ulong)
        .wrapping_div(::std::mem::size_of::<option_t>() as libc::c_ulong);
    full_len = longopts_len
        .wrapping_add(lang_count)
        .wrapping_add(1 as libc::c_int as libc::c_ulong);
    longopts = ag_malloc(
        full_len.wrapping_mul(::std::mem::size_of::<option_t>() as libc::c_ulong),
    ) as *mut option_t;
    memcpy(
        longopts as *mut libc::c_void,
        base_longopts.as_mut_ptr() as *const libc::c_void,
        ::std::mem::size_of::<[option_t; 92]>() as libc::c_ulong,
    );
    ext_index = ag_malloc(
        (::std::mem::size_of::<size_t>() as libc::c_ulong).wrapping_mul(lang_count),
    ) as *mut size_t;
    memset(
        ext_index as *mut libc::c_void,
        0 as libc::c_int,
        (::std::mem::size_of::<size_t>() as libc::c_ulong).wrapping_mul(lang_count),
    );
    i = 0 as libc::c_int as size_t;
    while i < lang_count {
        let mut opt: option_t = {
            let mut init = option {
                name: (*langs.as_mut_ptr().offset(i as isize)).name,
                has_arg: 0 as libc::c_int,
                flag: 0 as *mut libc::c_int,
                val: 0 as libc::c_int,
            };
            init
        };
        *longopts.offset(i.wrapping_add(longopts_len) as isize) = opt;
        i = i.wrapping_add(1);
        i;
    }
    *longopts
        .offset(
            full_len.wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize,
        ) = {
        let mut init = option {
            name: 0 as *const libc::c_char,
            has_arg: 0 as libc::c_int,
            flag: 0 as *mut libc::c_int,
            val: 0 as libc::c_int,
        };
        init
    };
    if argc < 2 as libc::c_int {
        usage();
        cleanup_ignore(root_ignores);
        cleanup_options();
        exit(1 as libc::c_int);
    }
    rv = fstat(fileno(stdin), &mut statbuf);
    if rv == 0 as libc::c_int {
        if statbuf.st_mode & 0o170000 as libc::c_int as libc::c_uint
            == 0o10000 as libc::c_int as libc::c_uint
            || statbuf.st_mode & 0o170000 as libc::c_int as libc::c_uint
                == 0o100000 as libc::c_int as libc::c_uint
        {
            opts.search_stream = 1 as libc::c_int;
        }
    }
    if isatty(fileno(stdout)) == 0 {
        opts.color = 0 as libc::c_int;
        group = 0 as libc::c_int;
        rv = fstat(fileno(stdout), &mut statbuf);
        if rv != 0 as libc::c_int {
            die(b"Error fstat()ing stdout\0" as *const u8 as *const libc::c_char);
        }
        opts.stdout_inode = statbuf.st_ino;
    }
    let mut file_search_regex: *mut libc::c_char = 0 as *mut libc::c_char;
    loop {
        ch = getopt_long(
            argc,
            argv,
            b"A:aB:C:cDG:g:FfHhiLlm:nop:QRrSsvVtuUwW:z0\0" as *const u8
                as *const libc::c_char,
            longopts,
            &mut opt_index,
        );
        if !(ch != -(1 as libc::c_int)) {
            break;
        }
        let mut current_block_136: u64;
        match ch {
            65 => {
                if !optarg.is_null() {
                    opts
                        .after = strtol(optarg, &mut num_end, 10 as libc::c_int)
                        as size_t;
                    if num_end == optarg || *num_end as libc::c_int != '\0' as i32
                        || *__errno_location() == 34 as libc::c_int
                    {
                        optind -= 1;
                        optind;
                        opts.after = 2 as libc::c_int as size_t;
                    }
                } else {
                    opts.after = 2 as libc::c_int as size_t;
                }
                current_block_136 = 12265727859147633668;
            }
            97 => {
                opts.search_all_files = 1 as libc::c_int;
                opts.search_binary_files = 1 as libc::c_int;
                current_block_136 = 12265727859147633668;
            }
            66 => {
                if !optarg.is_null() {
                    opts
                        .before = strtol(optarg, &mut num_end, 10 as libc::c_int)
                        as size_t;
                    if num_end == optarg || *num_end as libc::c_int != '\0' as i32
                        || *__errno_location() == 34 as libc::c_int
                    {
                        optind -= 1;
                        optind;
                        opts.before = 2 as libc::c_int as size_t;
                    }
                } else {
                    opts.before = 2 as libc::c_int as size_t;
                }
                current_block_136 = 12265727859147633668;
            }
            67 => {
                if !optarg.is_null() {
                    opts
                        .context = strtol(optarg, &mut num_end, 10 as libc::c_int)
                        as libc::c_int;
                    if num_end == optarg || *num_end as libc::c_int != '\0' as i32
                        || *__errno_location() == 34 as libc::c_int
                    {
                        optind -= 1;
                        optind;
                        opts.context = 2 as libc::c_int;
                    }
                } else {
                    opts.context = 2 as libc::c_int;
                }
                current_block_136 = 12265727859147633668;
            }
            99 => {
                opts.print_count = 1 as libc::c_int;
                opts.print_filename_only = 1 as libc::c_int;
                current_block_136 = 12265727859147633668;
            }
            68 => {
                set_log_level(LOG_LEVEL_DEBUG);
                current_block_136 = 12265727859147633668;
            }
            102 => {
                opts.follow_symlinks = 1 as libc::c_int;
                current_block_136 = 12265727859147633668;
            }
            103 => {
                accepts_query = 0 as libc::c_int;
                needs_query = accepts_query;
                opts.match_files = 1 as libc::c_int;
                current_block_136 = 387121447279139603;
            }
            71 => {
                current_block_136 = 387121447279139603;
            }
            72 => {
                opts.print_path = PATH_PRINT_TOP as libc::c_int;
                current_block_136 = 12265727859147633668;
            }
            104 => {
                help = 1 as libc::c_int;
                current_block_136 = 12265727859147633668;
            }
            105 => {
                opts.casing = CASE_INSENSITIVE;
                current_block_136 = 12265727859147633668;
            }
            76 => {
                opts.print_nonmatching_files = 1 as libc::c_int;
                opts.print_path = PATH_PRINT_TOP as libc::c_int;
                current_block_136 = 12265727859147633668;
            }
            108 => {
                needs_query = 0 as libc::c_int;
                opts.print_filename_only = 1 as libc::c_int;
                opts.print_path = PATH_PRINT_TOP as libc::c_int;
                current_block_136 = 12265727859147633668;
            }
            109 => {
                opts.max_matches_per_file = atoi(optarg) as size_t;
                current_block_136 = 12265727859147633668;
            }
            110 => {
                opts.recurse_dirs = 0 as libc::c_int;
                current_block_136 = 12265727859147633668;
            }
            112 => {
                opts.path_to_ignore = 1 as libc::c_int;
                load_ignore_patterns(root_ignores, optarg);
                current_block_136 = 12265727859147633668;
            }
            111 => {
                opts.only_matching = 1 as libc::c_int;
                current_block_136 = 12265727859147633668;
            }
            70 | 81 => {
                opts.literal = 1 as libc::c_int;
                current_block_136 = 12265727859147633668;
            }
            82 | 114 => {
                opts.recurse_dirs = 1 as libc::c_int;
                current_block_136 = 12265727859147633668;
            }
            83 => {
                opts.casing = CASE_SMART;
                current_block_136 = 12265727859147633668;
            }
            115 => {
                opts.casing = CASE_SENSITIVE;
                current_block_136 = 12265727859147633668;
            }
            116 => {
                opts.search_all_files = 1 as libc::c_int;
                current_block_136 = 12265727859147633668;
            }
            117 => {
                opts.search_binary_files = 1 as libc::c_int;
                opts.search_all_files = 1 as libc::c_int;
                opts.search_hidden_files = 1 as libc::c_int;
                current_block_136 = 12265727859147633668;
            }
            85 => {
                opts.skip_vcs_ignores = 1 as libc::c_int;
                current_block_136 = 12265727859147633668;
            }
            118 => {
                opts.invert_match = 1 as libc::c_int;
                opts.color = 0 as libc::c_int;
                current_block_136 = 12265727859147633668;
            }
            86 => {
                version = 1 as libc::c_int;
                current_block_136 = 12265727859147633668;
            }
            119 => {
                opts.word_regexp = 1 as libc::c_int;
                current_block_136 = 12265727859147633668;
            }
            87 => {
                opts.width = strtol(optarg, &mut num_end, 10 as libc::c_int) as size_t;
                if num_end == optarg || *num_end as libc::c_int != '\0' as i32
                    || *__errno_location() == 34 as libc::c_int
                {
                    die(b"Invalid width\n\0" as *const u8 as *const libc::c_char);
                }
                current_block_136 = 12265727859147633668;
            }
            122 => {
                opts.search_zip_files = 1 as libc::c_int;
                current_block_136 = 12265727859147633668;
            }
            48 => {
                opts.path_sep = '\0' as i32 as libc::c_char;
                current_block_136 = 12265727859147633668;
            }
            0 => {
                if strcmp(
                    (*longopts.offset(opt_index as isize)).name,
                    b"ackmate-dir-filter\0" as *const u8 as *const libc::c_char,
                ) == 0 as libc::c_int
                {
                    compile_study(
                        &mut opts.ackmate_dir_filter,
                        &mut opts.ackmate_dir_filter_extra,
                        optarg,
                        0 as libc::c_int,
                        0 as libc::c_int,
                    );
                    current_block_136 = 12265727859147633668;
                } else if strcmp(
                    (*longopts.offset(opt_index as isize)).name,
                    b"depth\0" as *const u8 as *const libc::c_char,
                ) == 0 as libc::c_int
                {
                    opts.max_search_depth = atoi(optarg);
                    current_block_136 = 12265727859147633668;
                } else if strcmp(
                    (*longopts.offset(opt_index as isize)).name,
                    b"filename\0" as *const u8 as *const libc::c_char,
                ) == 0 as libc::c_int
                {
                    opts.print_path = PATH_PRINT_DEFAULT as libc::c_int;
                    opts.print_line_numbers = 1 as libc::c_int;
                    current_block_136 = 12265727859147633668;
                } else if strcmp(
                    (*longopts.offset(opt_index as isize)).name,
                    b"ignore-dir\0" as *const u8 as *const libc::c_char,
                ) == 0 as libc::c_int
                {
                    add_ignore_pattern(root_ignores, optarg);
                    current_block_136 = 12265727859147633668;
                } else if strcmp(
                    (*longopts.offset(opt_index as isize)).name,
                    b"ignore\0" as *const u8 as *const libc::c_char,
                ) == 0 as libc::c_int
                {
                    add_ignore_pattern(root_ignores, optarg);
                    current_block_136 = 12265727859147633668;
                } else if strcmp(
                    (*longopts.offset(opt_index as isize)).name,
                    b"no-filename\0" as *const u8 as *const libc::c_char,
                ) == 0 as libc::c_int
                    || strcmp(
                        (*longopts.offset(opt_index as isize)).name,
                        b"nofilename\0" as *const u8 as *const libc::c_char,
                    ) == 0 as libc::c_int
                {
                    opts.print_path = PATH_PRINT_NOTHING as libc::c_int;
                    opts.print_line_numbers = 0 as libc::c_int;
                    current_block_136 = 12265727859147633668;
                } else if strcmp(
                    (*longopts.offset(opt_index as isize)).name,
                    b"no-pager\0" as *const u8 as *const libc::c_char,
                ) == 0 as libc::c_int
                    || strcmp(
                        (*longopts.offset(opt_index as isize)).name,
                        b"nopager\0" as *const u8 as *const libc::c_char,
                    ) == 0 as libc::c_int
                {
                    out_fd = stdout;
                    opts.pager = 0 as *mut libc::c_char;
                    current_block_136 = 12265727859147633668;
                } else if strcmp(
                    (*longopts.offset(opt_index as isize)).name,
                    b"pager\0" as *const u8 as *const libc::c_char,
                ) == 0 as libc::c_int
                {
                    opts.pager = optarg;
                    current_block_136 = 12265727859147633668;
                } else if strcmp(
                    (*longopts.offset(opt_index as isize)).name,
                    b"print-all-files\0" as *const u8 as *const libc::c_char,
                ) == 0 as libc::c_int
                {
                    opts.print_all_paths = 1 as libc::c_int;
                    current_block_136 = 12265727859147633668;
                } else if strcmp(
                    (*longopts.offset(opt_index as isize)).name,
                    b"workers\0" as *const u8 as *const libc::c_char,
                ) == 0 as libc::c_int
                {
                    opts.workers = atoi(optarg);
                    current_block_136 = 12265727859147633668;
                } else if strcmp(
                    (*longopts.offset(opt_index as isize)).name,
                    b"color-line-number\0" as *const u8 as *const libc::c_char,
                ) == 0 as libc::c_int
                {
                    free(opts.color_line_number as *mut libc::c_void);
                    ag_asprintf(
                        &mut opts.color_line_number as *mut *mut libc::c_char,
                        b"\x1B[%sm\0" as *const u8 as *const libc::c_char,
                        optarg,
                    );
                    current_block_136 = 12265727859147633668;
                } else if strcmp(
                    (*longopts.offset(opt_index as isize)).name,
                    b"color-match\0" as *const u8 as *const libc::c_char,
                ) == 0 as libc::c_int
                {
                    free(opts.color_match as *mut libc::c_void);
                    ag_asprintf(
                        &mut opts.color_match as *mut *mut libc::c_char,
                        b"\x1B[%sm\0" as *const u8 as *const libc::c_char,
                        optarg,
                    );
                    current_block_136 = 12265727859147633668;
                } else if strcmp(
                    (*longopts.offset(opt_index as isize)).name,
                    b"color-path\0" as *const u8 as *const libc::c_char,
                ) == 0 as libc::c_int
                {
                    free(opts.color_path as *mut libc::c_void);
                    ag_asprintf(
                        &mut opts.color_path as *mut *mut libc::c_char,
                        b"\x1B[%sm\0" as *const u8 as *const libc::c_char,
                        optarg,
                    );
                    current_block_136 = 12265727859147633668;
                } else if strcmp(
                    (*longopts.offset(opt_index as isize)).name,
                    b"silent\0" as *const u8 as *const libc::c_char,
                ) == 0 as libc::c_int
                {
                    set_log_level(LOG_LEVEL_NONE);
                    current_block_136 = 12265727859147633668;
                } else if strcmp(
                    (*longopts.offset(opt_index as isize)).name,
                    b"stats-only\0" as *const u8 as *const libc::c_char,
                ) == 0 as libc::c_int
                {
                    opts.print_filename_only = 1 as libc::c_int;
                    opts.print_path = PATH_PRINT_NOTHING as libc::c_int;
                    opts.stats = 1 as libc::c_int;
                    current_block_136 = 12265727859147633668;
                } else if !((*longopts.offset(opt_index as isize)).flag).is_null() {
                    current_block_136 = 12265727859147633668;
                } else {
                    i = 0 as libc::c_int as size_t;
                    while i < lang_count {
                        if strcmp(
                            (*longopts.offset(opt_index as isize)).name,
                            (*langs.as_mut_ptr().offset(i as isize)).name,
                        ) == 0 as libc::c_int
                        {
                            has_filetype = 1 as libc::c_int;
                            let fresh0 = lang_num;
                            lang_num = lang_num.wrapping_add(1);
                            *ext_index.offset(fresh0 as isize) = i;
                            break;
                        } else {
                            i = i.wrapping_add(1);
                            i;
                        }
                    }
                    if i != lang_count {
                        current_block_136 = 12265727859147633668;
                    } else {
                        log_err(
                            b"option %s does not take a value\0" as *const u8
                                as *const libc::c_char,
                            (*longopts.offset(opt_index as isize)).name,
                        );
                        current_block_136 = 18310731252746830695;
                    }
                }
            }
            _ => {
                current_block_136 = 18310731252746830695;
            }
        }
        match current_block_136 {
            18310731252746830695 => {
                usage();
                exit(1 as libc::c_int);
            }
            387121447279139603 => {
                if !file_search_regex.is_null() {
                    log_err(
                        b"File search regex (-g or -G) already specified.\0" as *const u8
                            as *const libc::c_char,
                    );
                    usage();
                    exit(1 as libc::c_int);
                }
                file_search_regex = ag_strdup(optarg);
            }
            _ => {}
        }
    }
    if opts.casing as libc::c_uint == CASE_DEFAULT as libc::c_int as libc::c_uint {
        opts.casing = CASE_SMART;
    }
    if !file_search_regex.is_null() {
        let mut pcre_opts: libc::c_int = 0 as libc::c_int;
        if opts.casing as libc::c_uint == CASE_INSENSITIVE as libc::c_int as libc::c_uint
            || opts.casing as libc::c_uint == CASE_SMART as libc::c_int as libc::c_uint
                && is_lowercase(file_search_regex) != 0
        {
            pcre_opts |= 0x1 as libc::c_int;
        }
        if opts.word_regexp != 0 {
            let mut old_file_search_regex: *mut libc::c_char = file_search_regex;
            ag_asprintf(
                &mut file_search_regex as *mut *mut libc::c_char,
                b"\\b%s\\b\0" as *const u8 as *const libc::c_char,
                file_search_regex,
            );
            free(old_file_search_regex as *mut libc::c_void);
        }
        compile_study(
            &mut opts.file_search_regex,
            &mut opts.file_search_regex_extra,
            file_search_regex,
            pcre_opts,
            0 as libc::c_int,
        );
        free(file_search_regex as *mut libc::c_void);
    }
    if has_filetype != 0 {
        num_exts = combine_file_extensions(ext_index, lang_num, &mut extensions);
        lang_regex = make_lang_regex(extensions, num_exts);
        compile_study(
            &mut opts.file_search_regex,
            &mut opts.file_search_regex_extra,
            lang_regex,
            0 as libc::c_int,
            0 as libc::c_int,
        );
    }
    if !extensions.is_null() {
        free(extensions as *mut libc::c_void);
    }
    free(ext_index as *mut libc::c_void);
    if !lang_regex.is_null() {
        free(lang_regex as *mut libc::c_void);
    }
    free(longopts as *mut libc::c_void);
    argc -= optind;
    argv = argv.offset(optind as isize);
    if !(opts.pager).is_null() {
        out_fd = popen(opts.pager, b"w\0" as *const u8 as *const libc::c_char);
        if out_fd.is_null() {
            perror(b"Failed to run pager\0" as *const u8 as *const libc::c_char);
            exit(1 as libc::c_int);
        }
    }
    if help != 0 {
        usage();
        exit(0 as libc::c_int);
    }
    if version != 0 {
        print_version();
        exit(0 as libc::c_int);
    }
    if list_file_types != 0 {
        let mut lang_index: size_t = 0;
        printf(
            b"The following file types are supported:\n\0" as *const u8
                as *const libc::c_char,
        );
        lang_index = 0 as libc::c_int as size_t;
        while lang_index < lang_count {
            printf(
                b"  --%s\n    \0" as *const u8 as *const libc::c_char,
                (*langs.as_mut_ptr().offset(lang_index as isize)).name,
            );
            let mut j: libc::c_int = 0;
            j = 0 as libc::c_int;
            while j < 12 as libc::c_int
                && !((*langs.as_mut_ptr().offset(lang_index as isize))
                    .extensions[j as usize])
                    .is_null()
            {
                printf(
                    b"  .%s\0" as *const u8 as *const libc::c_char,
                    (*langs.as_mut_ptr().offset(lang_index as isize))
                        .extensions[j as usize],
                );
                j += 1;
                j;
            }
            printf(b"\n\n\0" as *const u8 as *const libc::c_char);
            lang_index = lang_index.wrapping_add(1);
            lang_index;
        }
        exit(0 as libc::c_int);
    }
    if needs_query != 0 && argc == 0 as libc::c_int {
        log_err(
            b"What do you want to search for?\0" as *const u8 as *const libc::c_char,
        );
        exit(1 as libc::c_int);
    }
    if !home_dir.is_null() && opts.search_all_files == 0 {
        log_debug(
            b"Found user's home dir: %s\0" as *const u8 as *const libc::c_char,
            home_dir,
        );
        ag_asprintf(
            &mut ignore_file_path as *mut *mut libc::c_char,
            b"%s/.agignore\0" as *const u8 as *const libc::c_char,
            home_dir,
        );
        load_ignore_patterns(root_ignores, ignore_file_path);
        free(ignore_file_path as *mut libc::c_void);
    }
    if opts.skip_vcs_ignores == 0 {
        let mut gitconfig_file: *mut FILE = 0 as *mut FILE;
        let mut buf_len: size_t = 0 as libc::c_int as size_t;
        let mut gitconfig_res: *mut libc::c_char = 0 as *mut libc::c_char;
        gitconfig_file = popen(
            b"git config -z --path --get core.excludesfile 2>/dev/null\0" as *const u8
                as *const libc::c_char,
            b"r\0" as *const u8 as *const libc::c_char,
        );
        if !gitconfig_file.is_null() {
            loop {
                gitconfig_res = ag_realloc(
                    gitconfig_res as *mut libc::c_void,
                    buf_len.wrapping_add(65 as libc::c_int as libc::c_ulong),
                ) as *mut libc::c_char;
                buf_len = (buf_len as libc::c_ulong)
                    .wrapping_add(
                        fread(
                            gitconfig_res.offset(buf_len as isize) as *mut libc::c_void,
                            1 as libc::c_int as libc::c_ulong,
                            64 as libc::c_int as libc::c_ulong,
                            gitconfig_file,
                        ),
                    ) as size_t as size_t;
                if !(feof(gitconfig_file) == 0
                    && buf_len > 0 as libc::c_int as libc::c_ulong
                    && buf_len.wrapping_rem(64 as libc::c_int as libc::c_ulong)
                        == 0 as libc::c_int as libc::c_ulong)
                {
                    break;
                }
            }
            *gitconfig_res.offset(buf_len as isize) = '\0' as i32 as libc::c_char;
            if buf_len == 0 as libc::c_int as libc::c_ulong {
                free(gitconfig_res as *mut libc::c_void);
                let mut config_home: *const libc::c_char = getenv(
                    b"XDG_CONFIG_HOME\0" as *const u8 as *const libc::c_char,
                );
                if !config_home.is_null() {
                    ag_asprintf(
                        &mut gitconfig_res as *mut *mut libc::c_char,
                        b"%s/%s\0" as *const u8 as *const libc::c_char,
                        config_home,
                        b"git/ignore\0" as *const u8 as *const libc::c_char,
                    );
                } else if !home_dir.is_null() {
                    ag_asprintf(
                        &mut gitconfig_res as *mut *mut libc::c_char,
                        b"%s/%s\0" as *const u8 as *const libc::c_char,
                        home_dir,
                        b".config/git/ignore\0" as *const u8 as *const libc::c_char,
                    );
                } else {
                    gitconfig_res = ag_strdup(b"\0" as *const u8 as *const libc::c_char);
                }
            }
            log_debug(
                b"global core.excludesfile: %s\0" as *const u8 as *const libc::c_char,
                gitconfig_res,
            );
            load_ignore_patterns(root_ignores, gitconfig_res);
            free(gitconfig_res as *mut libc::c_void);
            pclose(gitconfig_file);
        }
    }
    if opts.context > 0 as libc::c_int {
        opts.before = opts.context as size_t;
        opts.after = opts.context as size_t;
    }
    if opts.ackmate != 0 {
        opts.color = 0 as libc::c_int;
        opts.print_break = 1 as libc::c_int;
        group = 1 as libc::c_int;
        opts.search_stream = 0 as libc::c_int;
    }
    if opts.vimgrep != 0 {
        opts.color = 0 as libc::c_int;
        opts.print_break = 0 as libc::c_int;
        group = 1 as libc::c_int;
        opts.search_stream = 0 as libc::c_int;
        opts.print_path = PATH_PRINT_NOTHING as libc::c_int;
    }
    if opts.parallel != 0 {
        opts.search_stream = 0 as libc::c_int;
    }
    if !(opts.print_path != PATH_PRINT_DEFAULT as libc::c_int
        || opts.print_break == 0 as libc::c_int)
    {
        if group != 0 {
            opts.print_break = 1 as libc::c_int;
        } else {
            opts.print_path = PATH_PRINT_DEFAULT_EACH_LINE as libc::c_int;
            opts.print_break = 0 as libc::c_int;
        }
    }
    if opts.search_stream != 0 {
        opts.print_break = 0 as libc::c_int;
        opts.print_path = PATH_PRINT_NOTHING as libc::c_int;
        if opts.print_line_numbers != 2 as libc::c_int {
            opts.print_line_numbers = 0 as libc::c_int;
        }
    }
    if accepts_query != 0 && argc > 0 as libc::c_int {
        if needs_query == 0
            && strlen(*argv.offset(0 as libc::c_int as isize))
                == 0 as libc::c_int as libc::c_ulong
        {
            opts.query = ag_strdup(b".\0" as *const u8 as *const libc::c_char);
        } else {
            opts.query = ag_strdup(*argv.offset(0 as libc::c_int as isize));
        }
        argc -= 1;
        argc;
        argv = argv.offset(1);
        argv;
    } else if needs_query == 0 {
        opts.query = ag_strdup(b".\0" as *const u8 as *const libc::c_char);
    }
    opts.query_len = strlen(opts.query) as libc::c_int;
    log_debug(b"Query is %s\0" as *const u8 as *const libc::c_char, opts.query);
    if opts.query_len == 0 as libc::c_int {
        log_err(
            b"Error: No query. What do you want to search for?\0" as *const u8
                as *const libc::c_char,
        );
        exit(1 as libc::c_int);
    }
    if is_regex(opts.query) == 0 {
        opts.literal = 1 as libc::c_int;
    }
    let mut path: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut base_path: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut tmp: *mut libc::c_char = 0 as *mut libc::c_char;
    opts.paths_len = argc;
    if argc > 0 as libc::c_int {
        *paths = ag_calloc(
            ::std::mem::size_of::<*mut libc::c_char>() as libc::c_ulong,
            (argc + 1 as libc::c_int) as size_t,
        ) as *mut *mut libc::c_char;
        *base_paths = ag_calloc(
            ::std::mem::size_of::<*mut libc::c_char>() as libc::c_ulong,
            (argc + 1 as libc::c_int) as size_t,
        ) as *mut *mut libc::c_char;
        i = 0 as libc::c_int as size_t;
        while i < argc as size_t {
            path = ag_strdup(*argv.offset(i as isize));
            path_len = strlen(path) as libc::c_int;
            if path_len > 1 as libc::c_int
                && *path.offset((path_len - 1 as libc::c_int) as isize) as libc::c_int
                    == '/' as i32
            {
                *path
                    .offset(
                        (path_len - 1 as libc::c_int) as isize,
                    ) = '\0' as i32 as libc::c_char;
            }
            let ref mut fresh1 = *(*paths).offset(i as isize);
            *fresh1 = path;
            tmp = ag_malloc(4096 as libc::c_int as size_t) as *mut libc::c_char;
            base_path = realpath(path, tmp);
            if !base_path.is_null() {
                base_path_len = strlen(base_path) as libc::c_int;
                if base_path_len > 1 as libc::c_int
                    && *base_path.offset((base_path_len - 1 as libc::c_int) as isize)
                        as libc::c_int != '/' as i32
                {
                    base_path = ag_realloc(
                        base_path as *mut libc::c_void,
                        (base_path_len + 2 as libc::c_int) as size_t,
                    ) as *mut libc::c_char;
                    *base_path
                        .offset(base_path_len as isize) = '/' as i32 as libc::c_char;
                    *base_path
                        .offset(
                            (base_path_len + 1 as libc::c_int) as isize,
                        ) = '\0' as i32 as libc::c_char;
                }
            }
            let ref mut fresh2 = *(*base_paths).offset(i as isize);
            *fresh2 = base_path;
            i = i.wrapping_add(1);
            i;
        }
        opts.search_stream = 0 as libc::c_int;
    } else {
        path = ag_strdup(b".\0" as *const u8 as *const libc::c_char);
        *paths = ag_malloc(
            (::std::mem::size_of::<*mut libc::c_char>() as libc::c_ulong)
                .wrapping_mul(2 as libc::c_int as libc::c_ulong),
        ) as *mut *mut libc::c_char;
        *base_paths = ag_malloc(
            (::std::mem::size_of::<*mut libc::c_char>() as libc::c_ulong)
                .wrapping_mul(2 as libc::c_int as libc::c_ulong),
        ) as *mut *mut libc::c_char;
        let ref mut fresh3 = *(*paths).offset(0 as libc::c_int as isize);
        *fresh3 = path;
        tmp = ag_malloc(4096 as libc::c_int as size_t) as *mut libc::c_char;
        let ref mut fresh4 = *(*base_paths).offset(0 as libc::c_int as isize);
        *fresh4 = realpath(path, tmp);
        i = 1 as libc::c_int as size_t;
    }
    let ref mut fresh5 = *(*paths).offset(i as isize);
    *fresh5 = 0 as *mut libc::c_char;
    let ref mut fresh6 = *(*base_paths).offset(i as isize);
    *fresh6 = 0 as *mut libc::c_char;
}
