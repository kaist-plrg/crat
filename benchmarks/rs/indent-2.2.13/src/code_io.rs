use ::libc;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    static mut stdin: *mut FILE;
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    fn memcpy(
        _: *mut libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn strncmp(
        _: *const libc::c_char,
        _: *const libc::c_char,
        _: libc::c_ulong,
    ) -> libc::c_int;
    fn getc(__stream: *mut FILE) -> libc::c_int;
    fn free(__ptr: *mut libc::c_void);
    fn close(__fd: libc::c_int) -> libc::c_int;
    fn read(__fd: libc::c_int, __buf: *mut libc::c_void, __nbytes: size_t) -> ssize_t;
    fn __errno_location() -> *mut libc::c_int;
    fn __fxstat(
        __ver: libc::c_int,
        __fildes: libc::c_int,
        __stat_buf: *mut stat,
    ) -> libc::c_int;
    fn open(__file: *const libc::c_char, __oflag: libc::c_int, _: ...) -> libc::c_int;
    static mut current_input: *mut file_buffer_ty;
    static mut save_com: buf_ty;
    static mut bp_save: *mut libc::c_char;
    static mut be_save: *mut libc::c_char;
    static mut settings: user_options_ty;
    fn dcgettext(
        __domainname: *const libc::c_char,
        __msgid: *const libc::c_char,
        __category: libc::c_int,
    ) -> *mut libc::c_char;
    fn xmalloc(size: libc::c_uint) -> *mut libc::c_void;
    fn xrealloc(ptr: *mut libc::c_void, size: libc::c_uint) -> *mut libc::c_void;
    fn xfree(ptr: *mut libc::c_void);
    fn fatal(string: *const libc::c_char, a0: *const libc::c_char);
    fn message(
        kind: *mut libc::c_char,
        string: *mut libc::c_char,
        a0: *mut libc::c_char,
        a1: *mut libc::c_char,
    );
    fn inhibit_indenting(flag: BOOLEAN);
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
pub type __ssize_t = libc::c_long;
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
pub type ssize_t = __ssize_t;
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
pub type BOOLEAN = libc::c_uchar;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct file_buffer {
    pub name: *mut libc::c_char,
    pub size: size_t,
    pub data: *mut libc::c_char,
}
pub type file_buffer_ty = file_buffer;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct buf {
    pub ptr: *mut libc::c_char,
    pub end: *mut libc::c_char,
    pub size: libc::c_int,
    pub len: libc::c_int,
    pub start_column: libc::c_int,
    pub column: libc::c_int,
}
pub type buf_ty = buf;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct user_options_st {
    pub verbose: libc::c_int,
    pub use_tabs: libc::c_int,
    pub tabsize: libc::c_int,
    pub use_stdout: libc::c_int,
    pub space_sp_semicolon: libc::c_int,
    pub swallow_optional_blanklines: libc::c_int,
    pub star_comment_cont: libc::c_int,
    pub struct_brace_indent: libc::c_int,
    pub space_after_while: libc::c_int,
    pub space_after_if: libc::c_int,
    pub space_after_for: libc::c_int,
    pub procnames_start_line: libc::c_int,
    pub parentheses_space: libc::c_int,
    pub preserve_mtime: libc::c_int,
    pub paren_indent: libc::c_int,
    pub proc_calls_space: libc::c_int,
    pub leave_preproc_space: libc::c_int,
    pub force_preproc_width: libc::c_int,
    pub lineup_to_parens: libc::c_int,
    pub honour_newlines: libc::c_int,
    pub fix_nested_comments: libc::c_int,
    pub format_comments: libc::c_int,
    pub format_col1_comments: libc::c_int,
    pub extra_expression_indent: libc::c_int,
    pub ljust_decl: libc::c_int,
    pub cast_space: libc::c_int,
    pub cuddle_else: libc::c_int,
    pub cuddle_do_while: libc::c_int,
    pub comment_delimiter_on_blankline: libc::c_int,
    pub blank_after_sizeof: libc::c_int,
    pub break_function_decl_args: libc::c_int,
    pub break_function_decl_args_end: libc::c_int,
    pub leave_comma: libc::c_int,
    pub break_before_boolean_operator: libc::c_int,
    pub blanklines_before_blockcomments: libc::c_int,
    pub blanklines_after_declarations: libc::c_int,
    pub blanklines_after_procs: libc::c_int,
    pub blanklines_after_declarations_at_proctop: libc::c_int,
    pub blanklines_around_conditional_compilation: libc::c_int,
    pub comment_max_col: libc::c_int,
    pub max_col: libc::c_int,
    pub label_offset: libc::c_int,
    pub ind_size: libc::c_int,
    pub indent_parameters: libc::c_int,
    pub decl_indent: libc::c_int,
    pub unindent_displace: libc::c_int,
    pub else_endif_col: libc::c_int,
    pub case_indent: libc::c_int,
    pub continuation_indent: libc::c_int,
    pub decl_com_ind: libc::c_int,
    pub case_brace_indent: libc::c_int,
    pub c_plus_plus: libc::c_int,
    pub com_ind: libc::c_int,
    pub braces_on_struct_decl_line: libc::c_int,
    pub braces_on_func_def_line: libc::c_int,
    pub btype_2: libc::c_int,
    pub brace_indent: libc::c_int,
    pub expect_output_file: libc::c_int,
    pub pointer_align_right: libc::c_int,
    pub gettext_strings: libc::c_int,
    pub allow_single_line_conditionals: libc::c_int,
    pub align_with_spaces: libc::c_int,
    pub spaces_around_initializers: libc::c_int,
    pub dont_tab_align_comments: libc::c_int,
}
pub type user_options_ty = user_options_st;
#[inline]
unsafe extern "C" fn fstat(
    mut __fd: libc::c_int,
    mut __statbuf: *mut stat,
) -> libc::c_int {
    return __fxstat(1 as libc::c_int, __fd, __statbuf);
}
pub static mut in_prog_pos: *mut libc::c_char = 0 as *const libc::c_char
    as *mut libc::c_char;
pub static mut buf_ptr: *mut libc::c_char = 0 as *const libc::c_char
    as *mut libc::c_char;
pub static mut buf_end: *mut libc::c_char = 0 as *const libc::c_char
    as *mut libc::c_char;
pub static mut had_eof: BOOLEAN = 0 as libc::c_int as BOOLEAN;
pub static mut cur_line: *mut libc::c_char = 0 as *const libc::c_char
    as *mut libc::c_char;
pub unsafe extern "C" fn skip_horiz_space(
    mut p: *const libc::c_char,
) -> *mut libc::c_char {
    while *p as libc::c_int == ' ' as i32 || *p as libc::c_int == '\t' as i32 {
        p = p.offset(1);
        p;
    }
    return p as *mut libc::c_char;
}
pub unsafe extern "C" fn skip_buffered_space() {
    while *buf_ptr as libc::c_int == ' ' as i32 || *buf_ptr as libc::c_int == '\t' as i32
    {
        buf_ptr = buf_ptr.offset(1);
        buf_ptr;
        if buf_ptr >= buf_end {
            fill_buffer();
        }
    }
}
unsafe extern "C" fn is_comment_start(mut p: *const libc::c_char) -> BOOLEAN {
    let mut ret: BOOLEAN = 0;
    if *p as libc::c_int == '/' as i32
        && (*p.offset(1 as libc::c_int as isize) as libc::c_int == '*' as i32
            || *p.offset(1 as libc::c_int as isize) as libc::c_int == '/' as i32)
    {
        ret = 1 as libc::c_int as BOOLEAN;
    } else {
        ret = 0 as libc::c_int as BOOLEAN;
    }
    return ret;
}
pub unsafe extern "C" fn current_column() -> libc::c_int {
    let mut p: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut column: libc::c_int = 0;
    if buf_ptr >= save_com.ptr && buf_ptr <= (save_com.ptr).offset(save_com.len as isize)
    {
        p = save_com.ptr;
        column = save_com.start_column;
    } else {
        p = cur_line;
        column = 1 as libc::c_int;
    }
    while p < buf_ptr {
        match *p as libc::c_int {
            10 | 12 => {
                column = 1 as libc::c_int;
            }
            9 => {
                column
                    += settings.tabsize - (column - 1 as libc::c_int) % settings.tabsize;
            }
            8 => {
                column -= 1;
                column;
            }
            _ => {
                column += 1;
                column;
            }
        }
        p = p.offset(1);
        p;
    }
    return column;
}
pub unsafe extern "C" fn read_file(
    mut filename: *mut libc::c_char,
    mut file_stats: *mut stat,
) -> *mut file_buffer_ty {
    static mut fileptr: file_buffer_ty = {
        let mut init = file_buffer {
            name: 0 as *const libc::c_char as *mut libc::c_char,
            size: 0 as libc::c_int as size_t,
            data: 0 as *const libc::c_char as *mut libc::c_char,
        };
        init
    };
    let mut size: ssize_t = 0 as libc::c_int as ssize_t;
    let mut size_to_read: size_t = 0 as libc::c_int as size_t;
    let mut namelen: libc::c_uint = strlen(filename) as libc::c_uint;
    let mut fd: libc::c_int = open(filename, 0 as libc::c_int, 0o777 as libc::c_int);
    if fd < 0 as libc::c_int {
        fatal(
            dcgettext(
                0 as *const libc::c_char,
                b"Can't open input file %s\0" as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ),
            filename,
        );
    }
    if fstat(fd, file_stats) < 0 as libc::c_int {
        fatal(
            dcgettext(
                0 as *const libc::c_char,
                b"Can't stat input file %s\0" as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ),
            filename,
        );
    }
    if (*file_stats).st_size == 0 as libc::c_int as libc::c_long {
        message(
            dcgettext(
                0 as *const libc::c_char,
                b"Error\0" as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ),
            dcgettext(
                0 as *const libc::c_char,
                b"Zero-length file %s\0" as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ),
            filename,
            0 as *mut libc::c_char,
        );
    }
    if ::std::mem::size_of::<libc::c_int>() as libc::c_ulong
        == 2 as libc::c_int as libc::c_ulong
    {
        if (*file_stats).st_size < 0 as libc::c_int as libc::c_long
            || (*file_stats).st_size
                > (0xffff as libc::c_int - 1 as libc::c_int) as libc::c_long
        {
            fatal(
                dcgettext(
                    0 as *const libc::c_char,
                    b"File %s is too big to read\0" as *const u8 as *const libc::c_char,
                    5 as libc::c_int,
                ),
                filename,
            );
        }
    } else if (*file_stats).st_size < 0 as libc::c_int as libc::c_long {
        fatal(
            dcgettext(
                0 as *const libc::c_char,
                b"System problem reading file %s\0" as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ),
            filename,
        );
    }
    if (*file_stats).st_size as size_t
        >= 9223372036854775807 as libc::c_long as libc::c_ulong
    {
        fatal(
            dcgettext(
                0 as *const libc::c_char,
                b"File %s is too big to read\0" as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ),
            filename,
        );
    }
    fileptr.size = (*file_stats).st_size as size_t;
    if !(fileptr.data).is_null() {
        fileptr
            .data = xrealloc(
            fileptr.data as *mut libc::c_void,
            ((*file_stats).st_size as libc::c_uint)
                .wrapping_add(2 as libc::c_int as libc::c_uint),
        ) as *mut libc::c_char;
    } else {
        fileptr
            .data = xmalloc(
            ((*file_stats).st_size as libc::c_uint)
                .wrapping_add(2 as libc::c_int as libc::c_uint),
        ) as *mut libc::c_char;
    }
    size_to_read = fileptr.size;
    while size_to_read > 0 as libc::c_int as libc::c_ulong {
        size = read(
            fd,
            (fileptr.data).offset(fileptr.size as isize).offset(-(size_to_read as isize))
                as *mut libc::c_void,
            size_to_read,
        );
        if size == -(1 as libc::c_int) as libc::c_long {
            if *__errno_location() == 4 as libc::c_int {
                continue;
            }
            xfree(fileptr.data as *mut libc::c_void);
            fatal(
                dcgettext(
                    0 as *const libc::c_char,
                    b"Error reading input file %s\0" as *const u8 as *const libc::c_char,
                    5 as libc::c_int,
                ),
                filename,
            );
        }
        size_to_read = (size_to_read as libc::c_ulong)
            .wrapping_sub(size as libc::c_ulong) as size_t as size_t;
    }
    if close(fd) < 0 as libc::c_int {
        xfree(fileptr.data as *mut libc::c_void);
        fatal(
            dcgettext(
                0 as *const libc::c_char,
                b"Error closing input file %s\0" as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ),
            filename,
        );
    }
    if (size as size_t) < fileptr.size {
        fileptr.size = size as size_t;
    }
    if !(fileptr.name).is_null() {
        fileptr
            .name = xrealloc(
            fileptr.name as *mut libc::c_void,
            namelen.wrapping_add(1 as libc::c_int as libc::c_uint),
        ) as *mut libc::c_char;
    } else {
        fileptr
            .name = xmalloc(namelen.wrapping_add(1 as libc::c_int as libc::c_uint))
            as *mut libc::c_char;
    }
    memcpy(
        fileptr.name as *mut libc::c_void,
        filename as *const libc::c_void,
        namelen as libc::c_ulong,
    );
    *(fileptr.name).offset(namelen as isize) = '\0' as i32 as libc::c_char;
    if fileptr.size > 0 as libc::c_int as libc::c_ulong
        && *(fileptr.data)
            .offset(
                (fileptr.size).wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize,
            ) as libc::c_int != '\n' as i32
    {
        *(fileptr.data).offset(fileptr.size as isize) = '\n' as i32 as libc::c_char;
        fileptr.size = (fileptr.size).wrapping_add(1);
        fileptr.size;
    }
    *(fileptr.data).offset(fileptr.size as isize) = '\0' as i32 as libc::c_char;
    return &mut fileptr;
}
pub unsafe extern "C" fn read_stdin() -> *mut file_buffer_ty {
    static mut stdinptr: file_buffer_ty = {
        let mut init = file_buffer {
            name: 0 as *const libc::c_char as *mut libc::c_char,
            size: 0 as libc::c_int as size_t,
            data: 0 as *const libc::c_char as *mut libc::c_char,
        };
        init
    };
    let mut size: libc::c_uint = (15 as libc::c_int * 8192 as libc::c_int)
        as libc::c_uint;
    let mut ch: libc::c_int = -(1 as libc::c_int);
    let mut p: *mut libc::c_char = 0 as *mut libc::c_char;
    if !(stdinptr.data).is_null() {
        free(stdinptr.data as *mut libc::c_void);
    }
    stdinptr
        .data = xmalloc(size.wrapping_add(1 as libc::c_int as libc::c_uint))
        as *mut libc::c_char;
    stdinptr.size = 0 as libc::c_int as size_t;
    p = stdinptr.data;
    loop {
        while stdinptr.size < size as libc::c_ulong {
            ch = getc(stdin);
            if ch == -(1 as libc::c_int) {
                break;
            }
            let fresh0 = p;
            p = p.offset(1);
            *fresh0 = ch as libc::c_char;
            stdinptr.size = (stdinptr.size).wrapping_add(1);
            stdinptr.size;
        }
        if ch != -(1 as libc::c_int) {
            size = size
                .wrapping_add((2 as libc::c_int * 8192 as libc::c_int) as libc::c_uint);
            stdinptr
                .data = xrealloc(stdinptr.data as *mut libc::c_void, size)
                as *mut libc::c_char;
            p = (stdinptr.data).offset(stdinptr.size as isize);
        }
        if !(ch != -(1 as libc::c_int)) {
            break;
        }
    }
    stdinptr
        .name = b"Standard Input\0" as *const u8 as *const libc::c_char
        as *mut libc::c_char;
    *(stdinptr.data).offset(stdinptr.size as isize) = '\0' as i32 as libc::c_char;
    return &mut stdinptr;
}
pub unsafe extern "C" fn fill_buffer() {
    let mut p: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut finished_a_line: BOOLEAN = 0 as libc::c_int as BOOLEAN;
    if !bp_save.is_null() {
        buf_ptr = bp_save;
        buf_end = be_save;
        be_save = 0 as *mut libc::c_char;
        bp_save = be_save;
        if buf_ptr < buf_end {
            return;
        }
    }
    if *in_prog_pos as libc::c_int == '\0' as i32 {
        buf_ptr = in_prog_pos;
        cur_line = buf_ptr;
        had_eof = 1 as libc::c_int as BOOLEAN;
    } else {
        cur_line = in_prog_pos;
        p = cur_line;
        finished_a_line = 0 as libc::c_int as BOOLEAN;
        loop {
            p = skip_horiz_space(p);
            if is_comment_start(p) != 0 {
                p = p.offset(2 as libc::c_int as isize);
                p = skip_horiz_space(p);
                if strncmp(
                    p,
                    b"*INDENT-OFF*\0" as *const u8 as *const libc::c_char,
                    12 as libc::c_int as libc::c_ulong,
                ) == 0 as libc::c_int
                {
                    inhibit_indenting(1 as libc::c_int as BOOLEAN);
                }
            }
            while *p as libc::c_int != '\0' as i32 && *p as libc::c_int != '\n' as i32 {
                p = p.offset(1);
                p;
            }
            if *p as libc::c_int == '\n' as i32 {
                finished_a_line = 1 as libc::c_int as BOOLEAN;
                in_prog_pos = p.offset(1 as libc::c_int as isize);
            } else if (p.offset_from((*current_input).data) as libc::c_long
                as libc::c_uint as libc::c_ulong) < (*current_input).size
            {
                fatal(
                    dcgettext(
                        0 as *const libc::c_char,
                        b"File %s contains NULL-characters: cannot proceed\n\0"
                            as *const u8 as *const libc::c_char,
                        5 as libc::c_int,
                    ),
                    (*current_input).name,
                );
            } else {
                in_prog_pos = p;
                finished_a_line = 1 as libc::c_int as BOOLEAN;
            }
            if !(finished_a_line == 0) {
                break;
            }
        }
        buf_ptr = cur_line;
        buf_end = in_prog_pos;
    };
}
