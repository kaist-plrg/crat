use ::libc;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    pub type real_pcre;
    fn __ctype_b_loc() -> *mut *const libc::c_ushort;
    fn fclose(__stream: *mut FILE) -> libc::c_int;
    fn fopen(_: *const libc::c_char, _: *const libc::c_char) -> *mut FILE;
    fn __getdelim(
        __lineptr: *mut *mut libc::c_char,
        __n: *mut size_t,
        __delimiter: libc::c_int,
        __stream: *mut FILE,
    ) -> __ssize_t;
    fn free(__ptr: *mut libc::c_void);
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    fn strncmp(
        _: *const libc::c_char,
        _: *const libc::c_char,
        _: libc::c_ulong,
    ) -> libc::c_int;
    fn strstr(_: *const libc::c_char, _: *const libc::c_char) -> *mut libc::c_char;
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    fn strchr(_: *const libc::c_char, _: libc::c_int) -> *mut libc::c_char;
    fn log_debug(fmt: *const libc::c_char, _: ...);
    static mut opts: cli_options;
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
    fn is_symlink(path: *const libc::c_char, d: *const dirent) -> libc::c_int;
    fn ag_malloc(size: size_t) -> *mut libc::c_void;
    fn ag_asprintf(ret: *mut *mut libc::c_char, fmt: *const libc::c_char, _: ...);
    fn is_fnmatch(filename: *const libc::c_char) -> libc::c_int;
    fn ag_realloc(ptr: *mut libc::c_void, size: size_t) -> *mut libc::c_void;
    fn ag_strndup(s: *const libc::c_char, size: size_t) -> *mut libc::c_char;
    fn free_strings(strs: *mut *mut libc::c_char, strs_len: size_t);
    fn binary_search(
        needle: *const libc::c_char,
        haystack: *mut *mut libc::c_char,
        start: libc::c_int,
        end: libc::c_int,
    ) -> libc::c_int;
    fn is_directory(path: *const libc::c_char, d: *const dirent) -> libc::c_int;
    fn is_named_pipe(path: *const libc::c_char, d: *const dirent) -> libc::c_int;
    fn fnmatch(
        __pattern: *const libc::c_char,
        __name: *const libc::c_char,
        __flags: libc::c_int,
    ) -> libc::c_int;
}
pub type __ino_t = libc::c_ulong;
pub type __off_t = libc::c_long;
pub type __off64_t = libc::c_long;
pub type __ssize_t = libc::c_long;
pub type C2RustUnnamed = libc::c_uint;
pub const _ISalnum: C2RustUnnamed = 8;
pub const _ISpunct: C2RustUnnamed = 4;
pub const _IScntrl: C2RustUnnamed = 2;
pub const _ISblank: C2RustUnnamed = 1;
pub const _ISgraph: C2RustUnnamed = 32768;
pub const _ISprint: C2RustUnnamed = 16384;
pub const _ISspace: C2RustUnnamed = 8192;
pub const _ISxdigit: C2RustUnnamed = 4096;
pub const _ISdigit: C2RustUnnamed = 2048;
pub const _ISalpha: C2RustUnnamed = 1024;
pub const _ISlower: C2RustUnnamed = 512;
pub const _ISupper: C2RustUnnamed = 256;
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
pub type ssize_t = __ssize_t;
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
pub struct scandir_baton_t {
    pub ig: *const ignores,
    pub base_path: *const libc::c_char,
    pub base_path_len: size_t,
    pub path_start: *const libc::c_char,
}
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
pub type pcre = real_pcre;
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
pub type case_behavior = libc::c_uint;
pub const CASE_SENSITIVE_RETRY_INSENSITIVE: case_behavior = 4;
pub const CASE_SMART: case_behavior = 3;
pub const CASE_INSENSITIVE: case_behavior = 2;
pub const CASE_SENSITIVE: case_behavior = 1;
pub const CASE_DEFAULT: case_behavior = 0;
#[inline]
#[linkage = "external"]
pub unsafe extern "C" fn getline(
    mut __lineptr: *mut *mut libc::c_char,
    mut __n: *mut size_t,
    mut __stream: *mut FILE,
) -> __ssize_t {
    return __getdelim(__lineptr, __n, '\n' as i32, __stream);
}
pub static mut fnmatch_flags: libc::c_int = (1 as libc::c_int) << 0 as libc::c_int;
pub static mut root_ignores: *mut ignores = 0 as *const ignores as *mut ignores;
pub static mut evil_hardcoded_ignore_files: [*const libc::c_char; 3] = [
    b".\0" as *const u8 as *const libc::c_char,
    b"..\0" as *const u8 as *const libc::c_char,
    0 as *const libc::c_char,
];
pub static mut ignore_pattern_files: [*const libc::c_char; 5] = [
    b".ignore\0" as *const u8 as *const libc::c_char,
    b".gitignore\0" as *const u8 as *const libc::c_char,
    b".git/info/exclude\0" as *const u8 as *const libc::c_char,
    b".hgignore\0" as *const u8 as *const libc::c_char,
    0 as *const libc::c_char,
];
pub unsafe extern "C" fn is_empty(mut ig: *mut ignores) -> libc::c_int {
    return (((*ig).extensions_len)
        .wrapping_add((*ig).names_len)
        .wrapping_add((*ig).slash_names_len)
        .wrapping_add((*ig).regexes_len)
        .wrapping_add((*ig).slash_regexes_len) == 0 as libc::c_int as libc::c_ulong)
        as libc::c_int;
}
pub unsafe extern "C" fn init_ignore(
    mut parent: *mut ignores,
    mut dirname: *const libc::c_char,
    dirname_len: size_t,
) -> *mut ignores {
    let mut ig: *mut ignores = ag_malloc(
        ::std::mem::size_of::<ignores>() as libc::c_ulong,
    ) as *mut ignores;
    (*ig).extensions = 0 as *mut *mut libc::c_char;
    (*ig).extensions_len = 0 as libc::c_int as size_t;
    (*ig).names = 0 as *mut *mut libc::c_char;
    (*ig).names_len = 0 as libc::c_int as size_t;
    (*ig).slash_names = 0 as *mut *mut libc::c_char;
    (*ig).slash_names_len = 0 as libc::c_int as size_t;
    (*ig).regexes = 0 as *mut *mut libc::c_char;
    (*ig).regexes_len = 0 as libc::c_int as size_t;
    (*ig).invert_regexes = 0 as *mut *mut libc::c_char;
    (*ig).invert_regexes_len = 0 as libc::c_int as size_t;
    (*ig).slash_regexes = 0 as *mut *mut libc::c_char;
    (*ig).slash_regexes_len = 0 as libc::c_int as size_t;
    (*ig).dirname = dirname;
    (*ig).dirname_len = dirname_len;
    if !parent.is_null() && is_empty(parent) != 0 && !((*parent).parent).is_null() {
        (*ig).parent = (*parent).parent;
    } else {
        (*ig).parent = parent;
    }
    if !parent.is_null() && (*parent).abs_path_len > 0 as libc::c_int as libc::c_ulong {
        ag_asprintf(
            &mut (*ig).abs_path as *mut *mut libc::c_char,
            b"%s/%s\0" as *const u8 as *const libc::c_char,
            (*parent).abs_path,
            dirname,
        );
        (*ig)
            .abs_path_len = ((*parent).abs_path_len)
            .wrapping_add(1 as libc::c_int as libc::c_ulong)
            .wrapping_add(dirname_len);
    } else if dirname_len == 1 as libc::c_int as libc::c_ulong
        && *dirname.offset(0 as libc::c_int as isize) as libc::c_int == '.' as i32
    {
        (*ig)
            .abs_path = ag_malloc(::std::mem::size_of::<libc::c_char>() as libc::c_ulong)
            as *mut libc::c_char;
        *((*ig).abs_path)
            .offset(0 as libc::c_int as isize) = '\0' as i32 as libc::c_char;
        (*ig).abs_path_len = 0 as libc::c_int as size_t;
    } else {
        ag_asprintf(
            &mut (*ig).abs_path as *mut *mut libc::c_char,
            b"%s\0" as *const u8 as *const libc::c_char,
            dirname,
        );
        (*ig).abs_path_len = dirname_len;
    }
    return ig;
}
pub unsafe extern "C" fn cleanup_ignore(mut ig: *mut ignores) {
    if ig.is_null() {
        return;
    }
    free_strings((*ig).extensions, (*ig).extensions_len);
    free_strings((*ig).names, (*ig).names_len);
    free_strings((*ig).slash_names, (*ig).slash_names_len);
    free_strings((*ig).regexes, (*ig).regexes_len);
    free_strings((*ig).invert_regexes, (*ig).invert_regexes_len);
    free_strings((*ig).slash_regexes, (*ig).slash_regexes_len);
    if !((*ig).abs_path).is_null() {
        free((*ig).abs_path as *mut libc::c_void);
    }
    free(ig as *mut libc::c_void);
}
pub unsafe extern "C" fn add_ignore_pattern(
    mut ig: *mut ignores,
    mut pattern: *const libc::c_char,
) {
    let mut i: libc::c_int = 0;
    let mut pattern_len: libc::c_int = 0;
    if strncmp(
        pattern,
        b"./\0" as *const u8 as *const libc::c_char,
        2 as libc::c_int as libc::c_ulong,
    ) == 0 as libc::c_int
    {
        pattern = pattern.offset(1);
        pattern;
    }
    pattern_len = strlen(pattern) as libc::c_int;
    while pattern_len > 0 as libc::c_int {
        if *(*__ctype_b_loc())
            .offset(
                *pattern.offset((pattern_len - 1 as libc::c_int) as isize) as libc::c_int
                    as isize,
            ) as libc::c_int & _ISspace as libc::c_int as libc::c_ushort as libc::c_int
            == 0
        {
            break;
        }
        pattern_len -= 1;
        pattern_len;
    }
    if pattern_len == 0 as libc::c_int {
        log_debug(
            b"Pattern is empty. Not adding any ignores.\0" as *const u8
                as *const libc::c_char,
        );
        return;
    }
    let mut patterns_p: *mut *mut *mut libc::c_char = 0 as *mut *mut *mut libc::c_char;
    let mut patterns_len: *mut size_t = 0 as *mut size_t;
    if is_fnmatch(pattern) != 0 {
        if *pattern.offset(0 as libc::c_int as isize) as libc::c_int == '*' as i32
            && *pattern.offset(1 as libc::c_int as isize) as libc::c_int == '.' as i32
            && !(strchr(pattern.offset(2 as libc::c_int as isize), '.' as i32)).is_null()
            && is_fnmatch(pattern.offset(2 as libc::c_int as isize)) == 0
        {
            patterns_p = &mut (*ig).extensions;
            patterns_len = &mut (*ig).extensions_len;
            pattern = pattern.offset(2 as libc::c_int as isize);
            pattern_len -= 2 as libc::c_int;
        } else if *pattern.offset(0 as libc::c_int as isize) as libc::c_int == '/' as i32
        {
            patterns_p = &mut (*ig).slash_regexes;
            patterns_len = &mut (*ig).slash_regexes_len;
            pattern = pattern.offset(1);
            pattern;
            pattern_len -= 1;
            pattern_len;
        } else if *pattern.offset(0 as libc::c_int as isize) as libc::c_int == '!' as i32
        {
            patterns_p = &mut (*ig).invert_regexes;
            patterns_len = &mut (*ig).invert_regexes_len;
            pattern = pattern.offset(1);
            pattern;
            pattern_len -= 1;
            pattern_len;
        } else {
            patterns_p = &mut (*ig).regexes;
            patterns_len = &mut (*ig).regexes_len;
        }
    } else if *pattern.offset(0 as libc::c_int as isize) as libc::c_int == '/' as i32 {
        patterns_p = &mut (*ig).slash_names;
        patterns_len = &mut (*ig).slash_names_len;
        pattern = pattern.offset(1);
        pattern;
        pattern_len -= 1;
        pattern_len;
    } else {
        patterns_p = &mut (*ig).names;
        patterns_len = &mut (*ig).names_len;
    }
    *patterns_len = (*patterns_len).wrapping_add(1);
    *patterns_len;
    let mut patterns: *mut *mut libc::c_char = 0 as *mut *mut libc::c_char;
    patterns = ag_realloc(
        *patterns_p as *mut libc::c_void,
        (*patterns_len)
            .wrapping_mul(::std::mem::size_of::<*mut libc::c_char>() as libc::c_ulong),
    ) as *mut *mut libc::c_char;
    *patterns_p = patterns;
    i = (*patterns_len).wrapping_sub(1 as libc::c_int as libc::c_ulong) as libc::c_int;
    while i > 0 as libc::c_int {
        if strcmp(pattern, *patterns.offset((i - 1 as libc::c_int) as isize))
            > 0 as libc::c_int
        {
            break;
        }
        let ref mut fresh0 = *patterns.offset(i as isize);
        *fresh0 = *patterns.offset((i - 1 as libc::c_int) as isize);
        i -= 1;
        i;
    }
    let ref mut fresh1 = *patterns.offset(i as isize);
    *fresh1 = ag_strndup(pattern, pattern_len as size_t);
    log_debug(
        b"added ignore pattern %s to %s\0" as *const u8 as *const libc::c_char,
        pattern,
        if ig == root_ignores {
            b"root ignores\0" as *const u8 as *const libc::c_char
        } else {
            (*ig).abs_path as *const libc::c_char
        },
    );
}
pub unsafe extern "C" fn load_ignore_patterns(
    mut ig: *mut ignores,
    mut path: *const libc::c_char,
) {
    let mut fp: *mut FILE = 0 as *mut FILE;
    fp = fopen(path, b"r\0" as *const u8 as *const libc::c_char);
    if fp.is_null() {
        log_debug(
            b"Skipping ignore file %s: not readable\0" as *const u8
                as *const libc::c_char,
            path,
        );
        return;
    }
    log_debug(b"Loading ignore file %s.\0" as *const u8 as *const libc::c_char, path);
    let mut line: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut line_len: ssize_t = 0 as libc::c_int as ssize_t;
    let mut line_cap: size_t = 0 as libc::c_int as size_t;
    loop {
        line_len = getline(&mut line, &mut line_cap, fp);
        if !(line_len > 0 as libc::c_int as libc::c_long) {
            break;
        }
        if line_len == 0 as libc::c_int as libc::c_long
            || *line.offset(0 as libc::c_int as isize) as libc::c_int == '\n' as i32
            || *line.offset(0 as libc::c_int as isize) as libc::c_int == '#' as i32
        {
            continue;
        }
        if *line.offset((line_len - 1 as libc::c_int as libc::c_long) as isize)
            as libc::c_int == '\n' as i32
        {
            *line
                .offset(
                    (line_len - 1 as libc::c_int as libc::c_long) as isize,
                ) = '\0' as i32 as libc::c_char;
        }
        add_ignore_pattern(ig, line);
    }
    free(line as *mut libc::c_void);
    fclose(fp);
}
unsafe extern "C" fn ackmate_dir_match(
    mut dir_name: *const libc::c_char,
) -> libc::c_int {
    if (opts.ackmate_dir_filter).is_null() {
        return 0 as libc::c_int;
    }
    return pcre_exec(
        opts.ackmate_dir_filter,
        0 as *const pcre_extra,
        dir_name,
        strlen(dir_name) as libc::c_int,
        0 as libc::c_int,
        0 as libc::c_int,
        0 as *mut libc::c_int,
        0 as libc::c_int,
    );
}
unsafe extern "C" fn path_ignore_search(
    mut ig: *const ignores,
    mut path: *const libc::c_char,
    mut filename: *const libc::c_char,
) -> libc::c_int {
    let mut temp: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut temp_start_pos: libc::c_int = 0;
    let mut i: size_t = 0;
    let mut match_pos: libc::c_int = 0;
    match_pos = binary_search(
        filename,
        (*ig).names,
        0 as libc::c_int,
        (*ig).names_len as libc::c_int,
    );
    if match_pos >= 0 as libc::c_int {
        log_debug(
            b"file %s ignored because name matches static pattern %s\0" as *const u8
                as *const libc::c_char,
            filename,
            *((*ig).names).offset(match_pos as isize),
        );
        return 1 as libc::c_int;
    }
    ag_asprintf(
        &mut temp as *mut *mut libc::c_char,
        b"%s/%s\0" as *const u8 as *const libc::c_char,
        if *path.offset(0 as libc::c_int as isize) as libc::c_int == '.' as i32 {
            path.offset(1 as libc::c_int as isize)
        } else {
            path
        },
        filename,
    );
    temp_start_pos = if *temp.offset(0 as libc::c_int as isize) as libc::c_int
        == '/' as i32
    {
        1 as libc::c_int
    } else {
        0 as libc::c_int
    };
    if strncmp(temp.offset(temp_start_pos as isize), (*ig).abs_path, (*ig).abs_path_len)
        == 0 as libc::c_int
    {
        let mut slash_filename: *mut libc::c_char = temp
            .offset(temp_start_pos as isize)
            .offset((*ig).abs_path_len as isize);
        if *slash_filename.offset(0 as libc::c_int as isize) as libc::c_int == '/' as i32
        {
            slash_filename = slash_filename.offset(1);
            slash_filename;
        }
        match_pos = binary_search(
            slash_filename,
            (*ig).names,
            0 as libc::c_int,
            (*ig).names_len as libc::c_int,
        );
        if match_pos >= 0 as libc::c_int {
            log_debug(
                b"file %s ignored because name matches static pattern %s\0" as *const u8
                    as *const libc::c_char,
                temp,
                *((*ig).names).offset(match_pos as isize),
            );
            free(temp as *mut libc::c_void);
            return 1 as libc::c_int;
        }
        match_pos = binary_search(
            slash_filename,
            (*ig).slash_names,
            0 as libc::c_int,
            (*ig).slash_names_len as libc::c_int,
        );
        if match_pos >= 0 as libc::c_int {
            log_debug(
                b"file %s ignored because name matches slash static pattern %s\0"
                    as *const u8 as *const libc::c_char,
                slash_filename,
                *((*ig).slash_names).offset(match_pos as isize),
            );
            free(temp as *mut libc::c_void);
            return 1 as libc::c_int;
        }
        i = 0 as libc::c_int as size_t;
        while i < (*ig).names_len {
            let mut pos: *mut libc::c_char = strstr(
                slash_filename,
                *((*ig).names).offset(i as isize),
            );
            if pos == slash_filename
                || !pos.is_null()
                    && *pos.offset(-(1 as libc::c_int as isize)) as libc::c_int
                        == '/' as i32
            {
                pos = pos.offset(strlen(*((*ig).names).offset(i as isize)) as isize);
                if *pos as libc::c_int == '\0' as i32
                    || *pos as libc::c_int == '/' as i32
                {
                    log_debug(
                        b"file %s ignored because path somewhere matches name %s\0"
                            as *const u8 as *const libc::c_char,
                        slash_filename,
                        *((*ig).names).offset(i as isize),
                    );
                    free(temp as *mut libc::c_void);
                    return 1 as libc::c_int;
                }
            }
            log_debug(
                b"pattern %s doesn't match path %s\0" as *const u8
                    as *const libc::c_char,
                *((*ig).names).offset(i as isize),
                slash_filename,
            );
            i = i.wrapping_add(1);
            i;
        }
        i = 0 as libc::c_int as size_t;
        while i < (*ig).slash_regexes_len {
            if fnmatch(
                *((*ig).slash_regexes).offset(i as isize),
                slash_filename,
                fnmatch_flags,
            ) == 0 as libc::c_int
            {
                log_debug(
                    b"file %s ignored because name matches slash regex pattern %s\0"
                        as *const u8 as *const libc::c_char,
                    slash_filename,
                    *((*ig).slash_regexes).offset(i as isize),
                );
                free(temp as *mut libc::c_void);
                return 1 as libc::c_int;
            }
            log_debug(
                b"pattern %s doesn't match slash file %s\0" as *const u8
                    as *const libc::c_char,
                *((*ig).slash_regexes).offset(i as isize),
                slash_filename,
            );
            i = i.wrapping_add(1);
            i;
        }
    }
    i = 0 as libc::c_int as size_t;
    while i < (*ig).invert_regexes_len {
        if fnmatch(*((*ig).invert_regexes).offset(i as isize), filename, fnmatch_flags)
            == 0 as libc::c_int
        {
            log_debug(
                b"file %s not ignored because name matches regex pattern !%s\0"
                    as *const u8 as *const libc::c_char,
                filename,
                *((*ig).invert_regexes).offset(i as isize),
            );
            free(temp as *mut libc::c_void);
            return 0 as libc::c_int;
        }
        log_debug(
            b"pattern !%s doesn't match file %s\0" as *const u8 as *const libc::c_char,
            *((*ig).invert_regexes).offset(i as isize),
            filename,
        );
        i = i.wrapping_add(1);
        i;
    }
    i = 0 as libc::c_int as size_t;
    while i < (*ig).regexes_len {
        if fnmatch(*((*ig).regexes).offset(i as isize), filename, fnmatch_flags)
            == 0 as libc::c_int
        {
            log_debug(
                b"file %s ignored because name matches regex pattern %s\0" as *const u8
                    as *const libc::c_char,
                filename,
                *((*ig).regexes).offset(i as isize),
            );
            free(temp as *mut libc::c_void);
            return 1 as libc::c_int;
        }
        log_debug(
            b"pattern %s doesn't match file %s\0" as *const u8 as *const libc::c_char,
            *((*ig).regexes).offset(i as isize),
            filename,
        );
        i = i.wrapping_add(1);
        i;
    }
    let mut rv: libc::c_int = ackmate_dir_match(temp);
    free(temp as *mut libc::c_void);
    return rv;
}
pub unsafe extern "C" fn filename_filter(
    mut path: *const libc::c_char,
    mut dir: *const dirent,
    mut baton: *mut libc::c_void,
) -> libc::c_int {
    let mut filename: *const libc::c_char = ((*dir).d_name).as_ptr();
    if opts.search_hidden_files == 0
        && *filename.offset(0 as libc::c_int as isize) as libc::c_int == '.' as i32
    {
        return 0 as libc::c_int;
    }
    let mut i: size_t = 0;
    i = 0 as libc::c_int as size_t;
    while !(evil_hardcoded_ignore_files[i as usize]).is_null() {
        if strcmp(filename, evil_hardcoded_ignore_files[i as usize]) == 0 as libc::c_int
        {
            return 0 as libc::c_int;
        }
        i = i.wrapping_add(1);
        i;
    }
    if opts.follow_symlinks == 0 && is_symlink(path, dir) != 0 {
        log_debug(
            b"File %s ignored becaused it's a symlink\0" as *const u8
                as *const libc::c_char,
            ((*dir).d_name).as_ptr(),
        );
        return 0 as libc::c_int;
    }
    if is_named_pipe(path, dir) != 0 {
        log_debug(
            b"%s ignored because it's a named pipe or socket\0" as *const u8
                as *const libc::c_char,
            path,
        );
        return 0 as libc::c_int;
    }
    if opts.search_all_files != 0 && opts.path_to_ignore == 0 {
        return 1 as libc::c_int;
    }
    let mut scandir_baton: *mut scandir_baton_t = baton as *mut scandir_baton_t;
    let mut path_start: *const libc::c_char = (*scandir_baton).path_start;
    let mut extension: *const libc::c_char = strchr(filename, '.' as i32);
    if !extension.is_null() {
        if *extension.offset(1 as libc::c_int as isize) != 0 {
            extension = extension.offset(1);
            extension;
        } else {
            extension = 0 as *const libc::c_char;
        }
    }
    let mut filename_len: size_t = 0 as libc::c_int as size_t;
    if strncmp(
        filename,
        b"./\0" as *const u8 as *const libc::c_char,
        2 as libc::c_int as libc::c_ulong,
    ) == 0 as libc::c_int
    {
        filename_len = strlen(filename);
        filename = filename.offset(1);
        filename;
        filename_len = filename_len.wrapping_sub(1);
        filename_len;
    }
    let mut ig: *const ignores = (*scandir_baton).ig;
    while !ig.is_null() {
        if !extension.is_null() {
            let mut match_pos: libc::c_int = binary_search(
                extension,
                (*ig).extensions,
                0 as libc::c_int,
                (*ig).extensions_len as libc::c_int,
            );
            if match_pos >= 0 as libc::c_int {
                log_debug(
                    b"file %s ignored because name matches extension %s\0" as *const u8
                        as *const libc::c_char,
                    filename,
                    *((*ig).extensions).offset(match_pos as isize),
                );
                return 0 as libc::c_int;
            }
        }
        if path_ignore_search(ig, path_start, filename) != 0 {
            return 0 as libc::c_int;
        }
        if is_directory(path, dir) != 0 {
            if filename_len == 0 {
                filename_len = strlen(filename);
            }
            if *filename
                .offset(
                    filename_len.wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize,
                ) as libc::c_int != '/' as i32
            {
                let mut temp: *mut libc::c_char = 0 as *mut libc::c_char;
                ag_asprintf(
                    &mut temp as *mut *mut libc::c_char,
                    b"%s/\0" as *const u8 as *const libc::c_char,
                    filename,
                );
                let mut rv: libc::c_int = path_ignore_search(ig, path_start, temp);
                free(temp as *mut libc::c_void);
                if rv != 0 {
                    return 0 as libc::c_int;
                }
            }
        }
        ig = (*ig).parent;
    }
    log_debug(b"%s not ignored\0" as *const u8 as *const libc::c_char, filename);
    return 1 as libc::c_int;
}
