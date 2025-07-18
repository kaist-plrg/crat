use ::libc;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    pub type real_pcre;
    fn fflush(__stream: *mut FILE) -> libc::c_int;
    fn fprintf(_: *mut FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn fputc(__c: libc::c_int, __stream: *mut FILE) -> libc::c_int;
    fn fputs(__s: *const libc::c_char, __stream: *mut FILE) -> libc::c_int;
    fn fwrite(
        _: *const libc::c_void,
        _: libc::c_ulong,
        _: libc::c_ulong,
        _: *mut FILE,
    ) -> libc::c_ulong;
    fn free(__ptr: *mut libc::c_void);
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    static mut opts: cli_options;
    static mut out_fd: *mut FILE;
    fn ag_calloc(nelem: size_t, elsize: size_t) -> *mut libc::c_void;
    fn ag_strndup(s: *const libc::c_char, size: size_t) -> *mut libc::c_char;
}
pub type size_t = libc::c_ulong;
pub type __ino_t = libc::c_ulong;
pub type __off_t = libc::c_long;
pub type __off64_t = libc::c_long;
pub type __ssize_t = libc::c_long;
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
pub type ino_t = __ino_t;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct match_t {
    pub start: size_t,
    pub end: size_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct print_context {
    pub line: size_t,
    pub context_prev_lines: *mut *mut libc::c_char,
    pub prev_line: size_t,
    pub last_prev_line: size_t,
    pub prev_line_offset: size_t,
    pub line_preceding_current_match_offset: size_t,
    pub lines_since_last_match: size_t,
    pub last_printed_match: size_t,
    pub in_a_match: libc::c_int,
    pub printing_a_match: libc::c_int,
}
pub static mut first_file_match: libc::c_int = 1 as libc::c_int;
pub static mut color_reset: *const libc::c_char = b"\x1B[0m\x1B[K\0" as *const u8
    as *const libc::c_char;
pub static mut truncate_marker: *const libc::c_char = b" [...]\0" as *const u8
    as *const libc::c_char;
#[thread_local]
pub static mut print_context: print_context = print_context {
    line: 0,
    context_prev_lines: 0 as *const *mut libc::c_char as *mut *mut libc::c_char,
    prev_line: 0,
    last_prev_line: 0,
    prev_line_offset: 0,
    line_preceding_current_match_offset: 0,
    lines_since_last_match: 0,
    last_printed_match: 0,
    in_a_match: 0,
    printing_a_match: 0,
};
pub unsafe extern "C" fn print_init_context() {
    if !(print_context.context_prev_lines).is_null() {
        return;
    }
    print_context
        .context_prev_lines = ag_calloc(
        ::std::mem::size_of::<*mut libc::c_char>() as libc::c_ulong,
        (opts.before).wrapping_add(1 as libc::c_int as libc::c_ulong),
    ) as *mut *mut libc::c_char;
    print_context.line = 1 as libc::c_int as size_t;
    print_context.prev_line = 0 as libc::c_int as size_t;
    print_context.last_prev_line = 0 as libc::c_int as size_t;
    print_context.prev_line_offset = 0 as libc::c_int as size_t;
    print_context.line_preceding_current_match_offset = 0 as libc::c_int as size_t;
    print_context.lines_since_last_match = 2147483647 as libc::c_int as size_t;
    print_context.last_printed_match = 0 as libc::c_int as size_t;
    print_context.in_a_match = 0 as libc::c_int;
    print_context.printing_a_match = 0 as libc::c_int;
}
pub unsafe extern "C" fn print_cleanup_context() {
    let mut i: size_t = 0;
    if (print_context.context_prev_lines).is_null() {
        return;
    }
    i = 0 as libc::c_int as size_t;
    while i < opts.before {
        if !(*(print_context.context_prev_lines).offset(i as isize)).is_null() {
            free(
                *(print_context.context_prev_lines).offset(i as isize)
                    as *mut libc::c_void,
            );
        }
        i = i.wrapping_add(1);
        i;
    }
    free(print_context.context_prev_lines as *mut libc::c_void);
    print_context.context_prev_lines = 0 as *mut *mut libc::c_char;
}
pub unsafe extern "C" fn print_context_append(
    mut line: *const libc::c_char,
    mut len: size_t,
) {
    if opts.before == 0 as libc::c_int as libc::c_ulong {
        return;
    }
    if !(*(print_context.context_prev_lines)
        .offset(print_context.last_prev_line as isize))
        .is_null()
    {
        free(
            *(print_context.context_prev_lines)
                .offset(print_context.last_prev_line as isize) as *mut libc::c_void,
        );
    }
    let ref mut fresh0 = *(print_context.context_prev_lines)
        .offset(print_context.last_prev_line as isize);
    *fresh0 = ag_strndup(line, len);
    print_context
        .last_prev_line = (print_context.last_prev_line)
        .wrapping_add(1 as libc::c_int as libc::c_ulong)
        .wrapping_rem(opts.before);
}
pub unsafe extern "C" fn print_trailing_context(
    mut path: *const libc::c_char,
    mut buf: *const libc::c_char,
    mut n: size_t,
) {
    let mut sep: libc::c_char = '-' as i32 as libc::c_char;
    if opts.ackmate != 0 || opts.vimgrep != 0 {
        sep = ':' as i32 as libc::c_char;
    }
    if print_context.lines_since_last_match != 0 as libc::c_int as libc::c_ulong
        && print_context.lines_since_last_match <= opts.after
    {
        if opts.print_path == PATH_PRINT_EACH_LINE as libc::c_int {
            print_path(path, ':' as i32 as libc::c_char);
        }
        print_line_number(print_context.line, sep);
        fwrite(buf as *const libc::c_void, 1 as libc::c_int as libc::c_ulong, n, out_fd);
        fputc('\n' as i32, out_fd);
    }
    print_context.line = (print_context.line).wrapping_add(1);
    print_context.line;
    if print_context.in_a_match == 0
        && print_context.lines_since_last_match
            < 2147483647 as libc::c_int as libc::c_ulong
    {
        print_context
            .lines_since_last_match = (print_context.lines_since_last_match)
            .wrapping_add(1);
        print_context.lines_since_last_match;
    }
}
pub unsafe extern "C" fn print_path(mut path: *const libc::c_char, sep: libc::c_char) {
    if opts.print_path == PATH_PRINT_NOTHING as libc::c_int && opts.vimgrep == 0 {
        return;
    }
    path = normalize_path(path);
    if opts.ackmate != 0 {
        fprintf(
            out_fd,
            b":%s%c\0" as *const u8 as *const libc::c_char,
            path,
            sep as libc::c_int,
        );
    } else if opts.vimgrep != 0 {
        fprintf(
            out_fd,
            b"%s%c\0" as *const u8 as *const libc::c_char,
            path,
            sep as libc::c_int,
        );
    } else if opts.color != 0 {
        fprintf(
            out_fd,
            b"%s%s%s%c\0" as *const u8 as *const libc::c_char,
            opts.color_path,
            path,
            color_reset,
            sep as libc::c_int,
        );
    } else {
        fprintf(
            out_fd,
            b"%s%c\0" as *const u8 as *const libc::c_char,
            path,
            sep as libc::c_int,
        );
    };
}
pub unsafe extern "C" fn print_path_count(
    mut path: *const libc::c_char,
    sep: libc::c_char,
    count: size_t,
) {
    if *path != 0 {
        print_path(path, ':' as i32 as libc::c_char);
    }
    if opts.color != 0 {
        fprintf(
            out_fd,
            b"%s%lu%s%c\0" as *const u8 as *const libc::c_char,
            opts.color_line_number,
            count,
            color_reset,
            sep as libc::c_int,
        );
    } else {
        fprintf(
            out_fd,
            b"%lu%c\0" as *const u8 as *const libc::c_char,
            count,
            sep as libc::c_int,
        );
    };
}
pub unsafe extern "C" fn print_line(
    mut buf: *const libc::c_char,
    mut buf_pos: size_t,
    mut prev_line_offset: size_t,
) {
    let mut write_chars: size_t = buf_pos
        .wrapping_sub(prev_line_offset)
        .wrapping_add(1 as libc::c_int as libc::c_ulong);
    if opts.width > 0 as libc::c_int as libc::c_ulong && opts.width < write_chars {
        write_chars = opts.width;
    }
    fwrite(
        buf.offset(prev_line_offset as isize) as *const libc::c_void,
        1 as libc::c_int as libc::c_ulong,
        write_chars,
        out_fd,
    );
}
pub unsafe extern "C" fn print_binary_file_matches(mut path: *const libc::c_char) {
    path = normalize_path(path);
    print_file_separator();
    fprintf(
        out_fd,
        b"Binary file %s matches.\n\0" as *const u8 as *const libc::c_char,
        path,
    );
}
pub unsafe extern "C" fn print_file_matches(
    mut path: *const libc::c_char,
    mut buf: *const libc::c_char,
    buf_len: size_t,
    mut matches: *const match_t,
    matches_len: size_t,
) {
    let mut cur_match: size_t = 0 as libc::c_int as size_t;
    let mut lines_to_print: ssize_t = 0 as libc::c_int as ssize_t;
    let mut sep: libc::c_char = '-' as i32 as libc::c_char;
    let mut i: size_t = 0;
    let mut j: size_t = 0;
    let mut blanks_between_matches: libc::c_int = (opts.context != 0 || opts.after != 0
        || opts.before != 0) as libc::c_int;
    if opts.ackmate != 0 || opts.vimgrep != 0 {
        sep = ':' as i32 as libc::c_char;
    }
    print_file_separator();
    if opts.print_path == PATH_PRINT_DEFAULT as libc::c_int {
        opts.print_path = PATH_PRINT_TOP as libc::c_int;
    } else if opts.print_path == PATH_PRINT_DEFAULT_EACH_LINE as libc::c_int {
        opts.print_path = PATH_PRINT_EACH_LINE as libc::c_int;
    }
    if opts.print_path == PATH_PRINT_TOP as libc::c_int {
        if opts.print_count != 0 {
            print_path_count(path, opts.path_sep, matches_len);
        } else {
            print_path(path, opts.path_sep);
        }
    }
    i = 0 as libc::c_int as size_t;
    while i <= buf_len
        && (cur_match < matches_len
            || print_context.lines_since_last_match <= opts.after)
    {
        if cur_match < matches_len && i == (*matches.offset(cur_match as isize)).start {
            print_context.in_a_match = 1 as libc::c_int;
            if cur_match > 0 as libc::c_int as libc::c_ulong
                && blanks_between_matches != 0
                && print_context.lines_since_last_match
                    > (opts.before)
                        .wrapping_add(opts.after)
                        .wrapping_add(1 as libc::c_int as libc::c_ulong)
            {
                fprintf(out_fd, b"--\n\0" as *const u8 as *const libc::c_char);
            }
            if print_context.lines_since_last_match > 0 as libc::c_int as libc::c_ulong
                && opts.before > 0 as libc::c_int as libc::c_ulong
            {
                lines_to_print = (print_context.lines_since_last_match)
                    .wrapping_sub(
                        (opts.after).wrapping_add(1 as libc::c_int as libc::c_ulong),
                    ) as ssize_t;
                if lines_to_print < 0 as libc::c_int as libc::c_long {
                    lines_to_print = 0 as libc::c_int as ssize_t;
                } else if lines_to_print as size_t > opts.before {
                    lines_to_print = opts.before as ssize_t;
                }
                j = (opts.before).wrapping_sub(lines_to_print as libc::c_ulong);
                while j < opts.before {
                    print_context
                        .prev_line = (print_context.last_prev_line)
                        .wrapping_add(j)
                        .wrapping_rem(opts.before);
                    if !(*(print_context.context_prev_lines)
                        .offset(print_context.prev_line as isize))
                        .is_null()
                    {
                        if opts.print_path == PATH_PRINT_EACH_LINE as libc::c_int {
                            print_path(path, ':' as i32 as libc::c_char);
                        }
                        print_line_number(
                            (print_context.line)
                                .wrapping_sub((opts.before).wrapping_sub(j)),
                            sep,
                        );
                        fprintf(
                            out_fd,
                            b"%s\n\0" as *const u8 as *const libc::c_char,
                            *(print_context.context_prev_lines)
                                .offset(print_context.prev_line as isize),
                        );
                    }
                    j = j.wrapping_add(1);
                    j;
                }
            }
            print_context.lines_since_last_match = 0 as libc::c_int as size_t;
        }
        if cur_match < matches_len && i == (*matches.offset(cur_match as isize)).end {
            cur_match = cur_match.wrapping_add(1);
            cur_match;
            print_context.in_a_match = 0 as libc::c_int;
        }
        if (i == buf_len || *buf.offset(i as isize) as libc::c_int == '\n' as i32)
            && opts.before > 0 as libc::c_int as libc::c_ulong
        {
            print_context_append(
                &*buf.offset(print_context.prev_line_offset as isize),
                i.wrapping_sub(print_context.prev_line_offset),
            );
        }
        if i == buf_len || *buf.offset(i as isize) as libc::c_int == '\n' as i32 {
            if print_context.lines_since_last_match == 0 as libc::c_int as libc::c_ulong
            {
                if opts.print_path == PATH_PRINT_EACH_LINE as libc::c_int
                    && opts.search_stream == 0
                {
                    print_path(path, ':' as i32 as libc::c_char);
                }
                if opts.ackmate != 0 {
                    print_line_number(print_context.line, ';' as i32 as libc::c_char);
                    while print_context.last_printed_match < cur_match {
                        let mut start: size_t = ((*matches
                            .offset(print_context.last_printed_match as isize))
                            .start)
                            .wrapping_sub(
                                print_context.line_preceding_current_match_offset,
                            );
                        fprintf(
                            out_fd,
                            b"%lu %lu\0" as *const u8 as *const libc::c_char,
                            start,
                            ((*matches.offset(print_context.last_printed_match as isize))
                                .end)
                                .wrapping_sub(
                                    (*matches.offset(print_context.last_printed_match as isize))
                                        .start,
                                ),
                        );
                        if print_context.last_printed_match
                            == cur_match.wrapping_sub(1 as libc::c_int as libc::c_ulong)
                        {
                            fputc(':' as i32, out_fd);
                        } else {
                            fputc(',' as i32, out_fd);
                        };
                        print_context
                            .last_printed_match = (print_context.last_printed_match)
                            .wrapping_add(1);
                        print_context.last_printed_match;
                    }
                    print_line(buf, i, print_context.prev_line_offset);
                } else if opts.vimgrep != 0 {
                    while print_context.last_printed_match < cur_match {
                        print_path(path, sep);
                        print_line_number(print_context.line, sep);
                        print_column_number(
                            matches,
                            print_context.last_printed_match,
                            print_context.prev_line_offset,
                            sep,
                        );
                        print_line(buf, i, print_context.prev_line_offset);
                        print_context
                            .last_printed_match = (print_context.last_printed_match)
                            .wrapping_add(1);
                        print_context.last_printed_match;
                    }
                } else {
                    print_line_number(print_context.line, ':' as i32 as libc::c_char);
                    let mut printed_match: libc::c_int = 0 as libc::c_int;
                    if opts.column != 0 {
                        print_column_number(
                            matches,
                            print_context.last_printed_match,
                            print_context.prev_line_offset,
                            ':' as i32 as libc::c_char,
                        );
                    }
                    if print_context.printing_a_match != 0 && opts.color != 0 {
                        fprintf(
                            out_fd,
                            b"%s\0" as *const u8 as *const libc::c_char,
                            opts.color_match,
                        );
                    }
                    j = print_context.prev_line_offset;
                    while j <= i {
                        if print_context.last_printed_match < matches_len
                            && j
                                == (*matches
                                    .offset(print_context.last_printed_match as isize))
                                    .end
                        {
                            if opts.color != 0 {
                                fprintf(
                                    out_fd,
                                    b"%s\0" as *const u8 as *const libc::c_char,
                                    color_reset,
                                );
                            }
                            print_context.printing_a_match = 0 as libc::c_int;
                            print_context
                                .last_printed_match = (print_context.last_printed_match)
                                .wrapping_add(1);
                            print_context.last_printed_match;
                            printed_match = 1 as libc::c_int;
                            if opts.only_matching != 0 {
                                fputc('\n' as i32, out_fd);
                            }
                        }
                        if j < buf_len && opts.width > 0 as libc::c_int as libc::c_ulong
                            && j.wrapping_sub(print_context.prev_line_offset)
                                >= opts.width
                        {
                            if j < i {
                                fputs(truncate_marker, out_fd);
                            }
                            fputc('\n' as i32, out_fd);
                            j = i;
                            print_context.last_printed_match = matches_len;
                        }
                        if print_context.last_printed_match < matches_len
                            && j
                                == (*matches
                                    .offset(print_context.last_printed_match as isize))
                                    .start
                        {
                            if opts.only_matching != 0 && printed_match != 0 {
                                if opts.print_path == PATH_PRINT_EACH_LINE as libc::c_int {
                                    print_path(path, ':' as i32 as libc::c_char);
                                }
                                print_line_number(
                                    print_context.line,
                                    ':' as i32 as libc::c_char,
                                );
                                if opts.column != 0 {
                                    print_column_number(
                                        matches,
                                        print_context.last_printed_match,
                                        print_context.prev_line_offset,
                                        ':' as i32 as libc::c_char,
                                    );
                                }
                            }
                            if opts.color != 0 {
                                fprintf(
                                    out_fd,
                                    b"%s\0" as *const u8 as *const libc::c_char,
                                    opts.color_match,
                                );
                            }
                            print_context.printing_a_match = 1 as libc::c_int;
                        }
                        if j < buf_len {
                            if opts.only_matching == 0
                                || print_context.printing_a_match != 0
                            {
                                if opts.width == 0 as libc::c_int as libc::c_ulong
                                    || j.wrapping_sub(print_context.prev_line_offset)
                                        < opts.width
                                {
                                    fputc(*buf.offset(j as isize) as libc::c_int, out_fd);
                                }
                            }
                        }
                        j = j.wrapping_add(1);
                        j;
                    }
                    if print_context.printing_a_match != 0 && opts.color != 0 {
                        fprintf(
                            out_fd,
                            b"%s\0" as *const u8 as *const libc::c_char,
                            color_reset,
                        );
                    }
                }
            }
            if opts.search_stream != 0 {
                print_context.last_printed_match = 0 as libc::c_int as size_t;
                break;
            } else {
                print_trailing_context(
                    path,
                    &*buf.offset(print_context.prev_line_offset as isize),
                    i.wrapping_sub(print_context.prev_line_offset),
                );
                print_context
                    .prev_line_offset = i
                    .wrapping_add(1 as libc::c_int as libc::c_ulong);
                if print_context.in_a_match == 0 {
                    print_context
                        .line_preceding_current_match_offset = i
                        .wrapping_add(1 as libc::c_int as libc::c_ulong);
                }
                if i == buf_len
                    && *buf
                        .offset(
                            i.wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize,
                        ) as libc::c_int != '\n' as i32
                {
                    fputc('\n' as i32, out_fd);
                }
            }
        }
        i = i.wrapping_add(1);
        i;
    }
    if opts.stdout_inode != 0 {
        fflush(out_fd);
    }
}
pub unsafe extern "C" fn print_line_number(mut line: size_t, sep: libc::c_char) {
    if opts.print_line_numbers == 0 {
        return;
    }
    if opts.color != 0 {
        fprintf(
            out_fd,
            b"%s%lu%s%c\0" as *const u8 as *const libc::c_char,
            opts.color_line_number,
            line,
            color_reset,
            sep as libc::c_int,
        );
    } else {
        fprintf(
            out_fd,
            b"%lu%c\0" as *const u8 as *const libc::c_char,
            line,
            sep as libc::c_int,
        );
    };
}
pub unsafe extern "C" fn print_column_number(
    mut matches: *const match_t,
    mut last_printed_match: size_t,
    mut prev_line_offset: size_t,
    sep: libc::c_char,
) {
    let mut column: size_t = 0 as libc::c_int as size_t;
    if prev_line_offset <= (*matches.offset(last_printed_match as isize)).start {
        column = ((*matches.offset(last_printed_match as isize)).start)
            .wrapping_sub(prev_line_offset)
            .wrapping_add(1 as libc::c_int as libc::c_ulong);
    }
    fprintf(
        out_fd,
        b"%lu%c\0" as *const u8 as *const libc::c_char,
        column,
        sep as libc::c_int,
    );
}
pub unsafe extern "C" fn print_file_separator() {
    if first_file_match == 0 as libc::c_int && opts.print_break != 0 {
        fprintf(out_fd, b"\n\0" as *const u8 as *const libc::c_char);
    }
    first_file_match = 0 as libc::c_int;
}
pub unsafe extern "C" fn normalize_path(
    mut path: *const libc::c_char,
) -> *const libc::c_char {
    if strlen(path) < 3 as libc::c_int as libc::c_ulong {
        return path;
    }
    if *path.offset(0 as libc::c_int as isize) as libc::c_int == '.' as i32
        && *path.offset(1 as libc::c_int as isize) as libc::c_int == '/' as i32
    {
        return path.offset(2 as libc::c_int as isize);
    }
    if *path.offset(0 as libc::c_int as isize) as libc::c_int == '/' as i32
        && *path.offset(1 as libc::c_int as isize) as libc::c_int == '/' as i32
    {
        return path.offset(1 as libc::c_int as isize);
    }
    return path;
}
