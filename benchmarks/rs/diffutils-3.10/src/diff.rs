use ::libc;
use ::c2rust_bitfields;
extern "C" {
    pub type re_dfa_t;
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    pub type exclude;
    fn getenv(__name: *const libc::c_char) -> *mut libc::c_char;
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
    fn lseek(__fd: libc::c_int, __offset: __off_t, __whence: libc::c_int) -> __off_t;
    fn close(__fd: libc::c_int) -> libc::c_int;
    fn isatty(__fd: libc::c_int) -> libc::c_int;
    static mut optarg: *mut libc::c_char;
    static mut optind: libc::c_int;
    fn open(__file: *const libc::c_char, __oflag: libc::c_int, _: ...) -> libc::c_int;
    fn signal(__sig: libc::c_int, __handler: __sighandler_t) -> __sighandler_t;
    fn strcpy(_: *mut libc::c_char, _: *const libc::c_char) -> *mut libc::c_char;
    fn rpl_free(ptr: *mut libc::c_void);
    fn setlocale(
        __category: libc::c_int,
        __locale: *const libc::c_char,
    ) -> *mut libc::c_char;
    fn __strtol_internal(
        __nptr: *const libc::c_char,
        __endptr: *mut *mut libc::c_char,
        __base: libc::c_int,
        __group: libc::c_int,
    ) -> libc::c_long;
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
    fn strchr(_: *const libc::c_char, _: libc::c_int) -> *mut libc::c_char;
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    fn stpcpy(_: *mut libc::c_char, _: *const libc::c_char) -> *mut libc::c_char;
    fn dcgettext(
        __domainname: *const libc::c_char,
        __msgid: *const libc::c_char,
        __category: libc::c_int,
    ) -> *mut libc::c_char;
    fn textdomain(__domainname: *const libc::c_char) -> *mut libc::c_char;
    fn bindtextdomain(
        __domainname: *const libc::c_char,
        __dirname: *const libc::c_char,
    ) -> *mut libc::c_char;
    fn __errno_location() -> *mut libc::c_int;
    fn proper_name(name: *const libc::c_char) -> *const libc::c_char;
    static mut Version: *const libc::c_char;
    fn rpl_re_set_syntax(__syntax: reg_syntax_t) -> reg_syntax_t;
    fn rpl_re_compile_pattern(
        __pattern: *const libc::c_char,
        __length: size_t,
        __buffer: *mut re_pattern_buffer,
    ) -> *const libc::c_char;
    static mut stdout: *mut FILE;
    fn fclose(__stream: *mut FILE) -> libc::c_int;
    fn fflush_unlocked(__stream: *mut FILE) -> libc::c_int;
    fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
    fn __overflow(_: *mut FILE, _: libc::c_int) -> libc::c_int;
    fn fputs_unlocked(__s: *const libc::c_char, __stream: *mut FILE) -> libc::c_int;
    fn diff_2_files(_: *mut comparison) -> libc::c_int;
    fn diff_dirs(
        _: *const comparison,
        _: Option::<
            unsafe extern "C" fn(
                *const comparison,
                *const libc::c_char,
                *const libc::c_char,
            ) -> libc::c_int,
        >,
    ) -> libc::c_int;
    fn find_dir_file_pathname(
        _: *const libc::c_char,
        _: *const libc::c_char,
    ) -> *mut libc::c_char;
    static pr_program: [libc::c_char; 0];
    fn cleanup_signal_handlers();
    fn fatal(_: *const libc::c_char);
    fn message(_: *const libc::c_char, _: ...);
    fn perror_with_name(_: *const libc::c_char);
    fn pfatal_with_name(_: *const libc::c_char);
    fn print_message_queue();
    fn set_color_palette(palette: *const libc::c_char);
    fn __assert_fail(
        __assertion: *const libc::c_char,
        __file: *const libc::c_char,
        __line: libc::c_uint,
        __function: *const libc::c_char,
    ) -> !;
    fn error(
        __status: libc::c_int,
        __errnum: libc::c_int,
        __format: *const libc::c_char,
        _: ...
    );
    fn c_stack_action(
        _: Option::<unsafe extern "C" fn(libc::c_int) -> ()>,
    ) -> libc::c_int;
    fn last_component(filename: *const libc::c_char) -> *mut libc::c_char;
    fn new_exclude() -> *mut exclude;
    fn add_exclude(_: *mut exclude, _: *const libc::c_char, _: libc::c_int);
    fn add_exclude_file(
        _: Option::<
            unsafe extern "C" fn(*mut exclude, *const libc::c_char, libc::c_int) -> (),
        >,
        _: *mut exclude,
        _: *const libc::c_char,
        _: libc::c_int,
        _: libc::c_char,
    ) -> libc::c_int;
    static mut exit_failure: libc::c_int;
    fn file_name_concat(
        dir: *const libc::c_char,
        base: *const libc::c_char,
        base_in_result: *mut *mut libc::c_char,
    ) -> *mut libc::c_char;
    fn file_type(_: *const stat) -> *const libc::c_char;
    fn getopt_long(
        ___argc: libc::c_int,
        ___argv: *const *mut libc::c_char,
        __shortopts: *const libc::c_char,
        __longopts: *const option,
        __longind: *mut libc::c_int,
    ) -> libc::c_int;
    fn hard_locale(category: libc::c_int) -> bool;
    static mut program_name: *const libc::c_char;
    fn set_program_name(argv0: *const libc::c_char);
    fn shell_quote_length(string: *const libc::c_char) -> size_t;
    fn shell_quote_copy(
        p: *mut libc::c_char,
        string: *const libc::c_char,
    ) -> *mut libc::c_char;
    fn gettime(_: *mut timespec);
    fn version_etc(
        stream: *mut FILE,
        command_name: *const libc::c_char,
        package: *const libc::c_char,
        version: *const libc::c_char,
        _: ...
    );
    fn emit_bug_reporting_address();
    fn xalloc_die();
    fn xmalloc(s: size_t) -> *mut libc::c_void;
    fn xrealloc(p: *mut libc::c_void, s: size_t) -> *mut libc::c_void;
    fn xreadlink(filename: *const libc::c_char) -> *mut libc::c_char;
    fn xstdopen();
}
pub type __intmax_t = libc::c_long;
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
pub type off_t = __off_t;
pub type size_t = libc::c_ulong;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct timespec {
    pub tv_sec: __time_t,
    pub tv_nsec: __syscall_slong_t,
}
pub type ptrdiff_t = libc::c_long;
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
pub type __sighandler_t = Option::<unsafe extern "C" fn(libc::c_int) -> ()>;
pub type intmax_t = __intmax_t;
pub type lin = ptrdiff_t;
pub type __re_long_size_t = size_t;
pub type reg_syntax_t = libc::c_ulong;
#[derive(Copy, Clone, BitfieldStruct)]
#[repr(C)]
pub struct re_pattern_buffer {
    pub buffer: *mut re_dfa_t,
    pub allocated: __re_long_size_t,
    pub used: __re_long_size_t,
    pub syntax: reg_syntax_t,
    pub fastmap: *mut libc::c_char,
    pub translate: *mut libc::c_uchar,
    pub re_nsub: size_t,
    #[bitfield(name = "can_be_null", ty = "libc::c_uint", bits = "0..=0")]
    #[bitfield(name = "regs_allocated", ty = "libc::c_uint", bits = "1..=2")]
    #[bitfield(name = "fastmap_accurate", ty = "libc::c_uint", bits = "3..=3")]
    #[bitfield(name = "no_sub", ty = "libc::c_uint", bits = "4..=4")]
    #[bitfield(name = "not_bol", ty = "libc::c_uint", bits = "5..=5")]
    #[bitfield(name = "not_eol", ty = "libc::c_uint", bits = "6..=6")]
    #[bitfield(name = "newline_anchor", ty = "libc::c_uint", bits = "7..=7")]
    pub can_be_null_regs_allocated_fastmap_accurate_no_sub_not_bol_not_eol_newline_anchor: [u8; 1],
    #[bitfield(padding)]
    pub c2rust_padding: [u8; 7],
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
pub type changes = libc::c_uint;
pub const CHANGED: changes = 3;
pub const NEW: changes = 2;
pub const OLD: changes = 1;
pub const UNCHANGED: changes = 0;
pub type colors_style = libc::c_uint;
pub const ALWAYS: colors_style = 2;
pub const AUTO: colors_style = 1;
pub const NEVER: colors_style = 0;
pub type output_style = libc::c_uint;
pub const OUTPUT_SDIFF: output_style = 8;
pub const OUTPUT_IFDEF: output_style = 7;
pub const OUTPUT_RCS: output_style = 6;
pub const OUTPUT_FORWARD_ED: output_style = 5;
pub const OUTPUT_ED: output_style = 4;
pub const OUTPUT_UNIFIED: output_style = 3;
pub const OUTPUT_CONTEXT: output_style = 2;
pub const OUTPUT_NORMAL: output_style = 1;
pub const OUTPUT_UNSPECIFIED: output_style = 0;
pub type DIFF_white_space = libc::c_uint;
pub const IGNORE_ALL_SPACE: DIFF_white_space = 5;
pub const IGNORE_SPACE_CHANGE: DIFF_white_space = 4;
pub const IGNORE_TAB_EXPANSION_AND_TRAILING_SPACE: DIFF_white_space = 3;
pub const IGNORE_TRAILING_SPACE: DIFF_white_space = 2;
pub const IGNORE_TAB_EXPANSION: DIFF_white_space = 1;
pub const IGNORE_NO_WHITE_SPACE: DIFF_white_space = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct file_data {
    pub desc: libc::c_int,
    pub name: *const libc::c_char,
    pub stat: stat,
    pub buffer: *mut size_t,
    pub bufsize: size_t,
    pub buffered: size_t,
    pub linbuf: *mut *const libc::c_char,
    pub linbuf_base: lin,
    pub buffered_lines: lin,
    pub valid_lines: lin,
    pub alloc_lines: lin,
    pub prefix_end: *const libc::c_char,
    pub prefix_lines: lin,
    pub suffix_begin: *const libc::c_char,
    pub equivs: *mut lin,
    pub undiscarded: *mut lin,
    pub realindexes: *mut lin,
    pub nondiscarded_lines: lin,
    pub changed: *mut libc::c_char,
    pub missing_newline: bool,
    pub eof: bool,
    pub equiv_max: lin,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct comparison {
    pub file: [file_data; 2],
    pub parent: *const comparison,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct option {
    pub name: *const libc::c_char,
    pub has_arg: libc::c_int,
    pub flag: *mut libc::c_int,
    pub val: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct regexp_list {
    pub regexps: *mut libc::c_char,
    pub len: size_t,
    pub size: size_t,
    pub multiple_regexps: bool,
    pub buf: *mut re_pattern_buffer,
}
pub const binary: C2RustUnnamed_1 = 1;
pub const UNOPENED: C2RustUnnamed_4 = -2;
pub const NONEXISTENT: C2RustUnnamed_3 = -1;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed {
    pub _gl_dummy: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_0 {
    pub _gl_dummy: libc::c_int,
}
pub type C2RustUnnamed_1 = libc::c_uint;
pub type C2RustUnnamed_2 = libc::c_uint;
pub const PRESUME_OUTPUT_TTY_OPTION: C2RustUnnamed_2 = 155;
pub const NO_DIRECTORY_OPTION: C2RustUnnamed_2 = 154;
pub const COLOR_PALETTE_OPTION: C2RustUnnamed_2 = 153;
pub const COLOR_OPTION: C2RustUnnamed_2 = 152;
pub const CHANGED_GROUP_FORMAT_OPTION: C2RustUnnamed_2 = 151;
pub const NEW_GROUP_FORMAT_OPTION: C2RustUnnamed_2 = 150;
pub const OLD_GROUP_FORMAT_OPTION: C2RustUnnamed_2 = 149;
pub const UNCHANGED_GROUP_FORMAT_OPTION: C2RustUnnamed_2 = 148;
pub const NEW_LINE_FORMAT_OPTION: C2RustUnnamed_2 = 147;
pub const OLD_LINE_FORMAT_OPTION: C2RustUnnamed_2 = 146;
pub const UNCHANGED_LINE_FORMAT_OPTION: C2RustUnnamed_2 = 145;
pub const TO_FILE_OPTION: C2RustUnnamed_2 = 144;
pub const TABSIZE_OPTION: C2RustUnnamed_2 = 143;
pub const SUPPRESS_COMMON_LINES_OPTION: C2RustUnnamed_2 = 142;
pub const SUPPRESS_BLANK_EMPTY_OPTION: C2RustUnnamed_2 = 141;
pub const STRIP_TRAILING_CR_OPTION: C2RustUnnamed_2 = 140;
pub const SDIFF_MERGE_ASSIST_OPTION: C2RustUnnamed_2 = 139;
pub const NORMAL_OPTION: C2RustUnnamed_2 = 138;
pub const NO_IGNORE_FILE_NAME_CASE_OPTION: C2RustUnnamed_2 = 137;
pub const NO_DEREFERENCE_OPTION: C2RustUnnamed_2 = 136;
pub const LINE_FORMAT_OPTION: C2RustUnnamed_2 = 135;
pub const LEFT_COLUMN_OPTION: C2RustUnnamed_2 = 134;
pub const INHIBIT_HUNK_MERGE_OPTION: C2RustUnnamed_2 = 133;
pub const IGNORE_FILE_NAME_CASE_OPTION: C2RustUnnamed_2 = 132;
pub const HORIZON_LINES_OPTION: C2RustUnnamed_2 = 131;
pub const HELP_OPTION: C2RustUnnamed_2 = 130;
pub const FROM_FILE_OPTION: C2RustUnnamed_2 = 129;
pub const BINARY_OPTION: C2RustUnnamed_2 = 128;
pub type C2RustUnnamed_3 = libc::c_int;
pub type C2RustUnnamed_4 = libc::c_int;
#[inline]
unsafe extern "C" fn stat(
    mut __path: *const libc::c_char,
    mut __statbuf: *mut stat,
) -> libc::c_int {
    return __xstat(1 as libc::c_int, __path, __statbuf);
}
#[inline]
unsafe extern "C" fn fstat(
    mut __fd: libc::c_int,
    mut __statbuf: *mut stat,
) -> libc::c_int {
    return __fxstat(1 as libc::c_int, __fd, __statbuf);
}
#[inline]
unsafe extern "C" fn lstat(
    mut __path: *const libc::c_char,
    mut __statbuf: *mut stat,
) -> libc::c_int {
    return __lxstat(1 as libc::c_int, __path, __statbuf);
}
#[inline]
unsafe extern "C" fn strtoimax(
    mut nptr: *const libc::c_char,
    mut endptr: *mut *mut libc::c_char,
    mut base: libc::c_int,
) -> intmax_t {
    return __strtol_internal(nptr, endptr, base, 0 as libc::c_int);
}
#[inline]
unsafe extern "C" fn putchar_unlocked(mut __c: libc::c_int) -> libc::c_int {
    return if ((*stdout)._IO_write_ptr >= (*stdout)._IO_write_end) as libc::c_int
        as libc::c_long != 0
    {
        __overflow(stdout, __c as libc::c_uchar as libc::c_int)
    } else {
        let fresh0 = (*stdout)._IO_write_ptr;
        (*stdout)._IO_write_ptr = ((*stdout)._IO_write_ptr).offset(1);
        *fresh0 = __c as libc::c_char;
        *fresh0 as libc::c_uchar as libc::c_int
    };
}
#[inline]
unsafe extern "C" fn ferror_unlocked(mut __stream: *mut FILE) -> libc::c_int {
    return ((*__stream)._flags & 0x20 as libc::c_int != 0 as libc::c_int) as libc::c_int;
}
#[inline]
#[linkage = "external"]
pub unsafe extern "C" fn robust_output_style(mut s: output_style) -> bool {
    return s as libc::c_uint != OUTPUT_ED as libc::c_int as libc::c_uint
        && s as libc::c_uint != OUTPUT_FORWARD_ED as libc::c_int as libc::c_uint;
}
pub static mut output_style: output_style = OUTPUT_UNSPECIFIED;
pub static mut colors_style: colors_style = NEVER;
pub static mut no_diff_means_no_output: bool = false;
pub static mut context: lin = 0;
pub static mut text: bool = false;
pub static mut horizon_lines: lin = 0;
pub static mut ignore_white_space: DIFF_white_space = IGNORE_NO_WHITE_SPACE;
pub static mut ignore_blank_lines: bool = false;
pub static mut files_can_be_treated_as_binary: bool = false;
pub static mut ignore_case: bool = false;
pub static mut ignore_file_name_case: bool = false;
pub static mut no_dereference_symlinks: bool = false;
pub static mut file_label: [*mut libc::c_char; 2] = [0 as *const libc::c_char
    as *mut libc::c_char; 2];
pub static mut function_regexp: re_pattern_buffer = re_pattern_buffer {
    buffer: 0 as *const re_dfa_t as *mut re_dfa_t,
    allocated: 0,
    used: 0,
    syntax: 0,
    fastmap: 0 as *const libc::c_char as *mut libc::c_char,
    translate: 0 as *const libc::c_uchar as *mut libc::c_uchar,
    re_nsub: 0,
    can_be_null_regs_allocated_fastmap_accurate_no_sub_not_bol_not_eol_newline_anchor: [0; 1],
    c2rust_padding: [0; 7],
};
pub static mut ignore_regexp: re_pattern_buffer = re_pattern_buffer {
    buffer: 0 as *const re_dfa_t as *mut re_dfa_t,
    allocated: 0,
    used: 0,
    syntax: 0,
    fastmap: 0 as *const libc::c_char as *mut libc::c_char,
    translate: 0 as *const libc::c_uchar as *mut libc::c_uchar,
    re_nsub: 0,
    can_be_null_regs_allocated_fastmap_accurate_no_sub_not_bol_not_eol_newline_anchor: [0; 1],
    c2rust_padding: [0; 7],
};
pub static mut brief: bool = false;
pub static mut expand_tabs: bool = false;
pub static mut tabsize: size_t = 0;
pub static mut initial_tab: bool = false;
pub static mut suppress_blank_empty: bool = false;
pub static mut strip_trailing_cr: bool = false;
pub static mut starting_file: *const libc::c_char = 0 as *const libc::c_char;
pub static mut paginate: bool = false;
pub static mut group_format: [*const libc::c_char; 4] = [0 as *const libc::c_char; 4];
pub static mut line_format: [*const libc::c_char; 3] = [0 as *const libc::c_char; 3];
pub static mut sdiff_merge_assist: bool = false;
pub static mut left_column: bool = false;
pub static mut suppress_common_lines: bool = false;
pub static mut sdiff_half_width: size_t = 0;
pub static mut sdiff_column2_offset: size_t = 0;
pub static mut switch_string: *mut libc::c_char = 0 as *const libc::c_char
    as *mut libc::c_char;
pub static mut speed_large_files: bool = false;
pub static mut excluded: *mut exclude = 0 as *const exclude as *mut exclude;
pub static mut minimal: bool = false;
pub static mut time_format: *const libc::c_char = 0 as *const libc::c_char;
pub static mut files: [file_data; 2] = [file_data {
    desc: 0,
    name: 0 as *const libc::c_char,
    stat: stat {
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
    },
    buffer: 0 as *const size_t as *mut size_t,
    bufsize: 0,
    buffered: 0,
    linbuf: 0 as *const *const libc::c_char as *mut *const libc::c_char,
    linbuf_base: 0,
    buffered_lines: 0,
    valid_lines: 0,
    alloc_lines: 0,
    prefix_end: 0 as *const libc::c_char,
    prefix_lines: 0,
    suffix_begin: 0 as *const libc::c_char,
    equivs: 0 as *const lin as *mut lin,
    undiscarded: 0 as *const lin as *mut lin,
    realindexes: 0 as *const lin as *mut lin,
    nondiscarded_lines: 0,
    changed: 0 as *const libc::c_char as *mut libc::c_char,
    missing_newline: false,
    eof: false,
    equiv_max: 0,
}; 2];
pub static mut outfile: *mut FILE = 0 as *const FILE as *mut FILE;
pub static mut presume_output_tty: bool = false;
#[inline]
unsafe extern "C" fn __gl_setmode(
    mut fd: libc::c_int,
    mut mode: libc::c_int,
) -> libc::c_int {
    return 0 as libc::c_int;
}
#[inline]
unsafe extern "C" fn set_binary_mode(
    mut fd: libc::c_int,
    mut mode: libc::c_int,
) -> libc::c_int {
    return __gl_setmode(fd, mode);
}
static mut PROGRAM_NAME: [libc::c_char; 5] = unsafe {
    *::std::mem::transmute::<&[u8; 5], &[libc::c_char; 5]>(b"diff\0")
};
static mut recursive: bool = false;
static mut function_regexp_list: regexp_list = regexp_list {
    regexps: 0 as *const libc::c_char as *mut libc::c_char,
    len: 0,
    size: 0,
    multiple_regexps: false,
    buf: 0 as *const re_pattern_buffer as *mut re_pattern_buffer,
};
static mut ignore_regexp_list: regexp_list = regexp_list {
    regexps: 0 as *const libc::c_char as *mut libc::c_char,
    len: 0,
    size: 0,
    multiple_regexps: false,
    buf: 0 as *const re_pattern_buffer as *mut re_pattern_buffer,
};
static mut new_file: bool = false;
static mut unidirectional_new_file: bool = false;
static mut report_identical_files: bool = false;
static mut no_directory: bool = false;
static mut shortopts: [libc::c_char; 57] = unsafe {
    *::std::mem::transmute::<
        &[u8; 57],
        &[libc::c_char; 57],
    >(b"0123456789abBcC:dD:eEfF:hHiI:lL:nNpPqrsS:tTuU:vwW:x:X:yZ\0")
};
static mut group_format_option: [[libc::c_char; 25]; 4] = unsafe {
    [
        *::std::mem::transmute::<
            &[u8; 25],
            &[libc::c_char; 25],
        >(b"--unchanged-group-format\0"),
        *::std::mem::transmute::<
            &[u8; 25],
            &[libc::c_char; 25],
        >(b"--old-group-format\0\0\0\0\0\0\0"),
        *::std::mem::transmute::<
            &[u8; 25],
            &[libc::c_char; 25],
        >(b"--new-group-format\0\0\0\0\0\0\0"),
        *::std::mem::transmute::<
            &[u8; 25],
            &[libc::c_char; 25],
        >(b"--changed-group-format\0\0\0"),
    ]
};
static mut line_format_option: [[libc::c_char; 24]; 3] = unsafe {
    [
        *::std::mem::transmute::<
            &[u8; 24],
            &[libc::c_char; 24],
        >(b"--unchanged-line-format\0"),
        *::std::mem::transmute::<
            &[u8; 24],
            &[libc::c_char; 24],
        >(b"--old-line-format\0\0\0\0\0\0\0"),
        *::std::mem::transmute::<
            &[u8; 24],
            &[libc::c_char; 24],
        >(b"--new-line-format\0\0\0\0\0\0\0"),
    ]
};
static mut longopts: [option; 62] = [
    {
        let mut init = option {
            name: b"binary\0" as *const u8 as *const libc::c_char,
            has_arg: 0 as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: BINARY_OPTION as libc::c_int,
        };
        init
    },
    {
        let mut init = option {
            name: b"brief\0" as *const u8 as *const libc::c_char,
            has_arg: 0 as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: 'q' as i32,
        };
        init
    },
    {
        let mut init = option {
            name: b"changed-group-format\0" as *const u8 as *const libc::c_char,
            has_arg: 1 as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: CHANGED_GROUP_FORMAT_OPTION as libc::c_int,
        };
        init
    },
    {
        let mut init = option {
            name: b"color\0" as *const u8 as *const libc::c_char,
            has_arg: 2 as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: COLOR_OPTION as libc::c_int,
        };
        init
    },
    {
        let mut init = option {
            name: b"context\0" as *const u8 as *const libc::c_char,
            has_arg: 2 as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: 'C' as i32,
        };
        init
    },
    {
        let mut init = option {
            name: b"ed\0" as *const u8 as *const libc::c_char,
            has_arg: 0 as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: 'e' as i32,
        };
        init
    },
    {
        let mut init = option {
            name: b"exclude\0" as *const u8 as *const libc::c_char,
            has_arg: 1 as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: 'x' as i32,
        };
        init
    },
    {
        let mut init = option {
            name: b"exclude-from\0" as *const u8 as *const libc::c_char,
            has_arg: 1 as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: 'X' as i32,
        };
        init
    },
    {
        let mut init = option {
            name: b"expand-tabs\0" as *const u8 as *const libc::c_char,
            has_arg: 0 as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: 't' as i32,
        };
        init
    },
    {
        let mut init = option {
            name: b"forward-ed\0" as *const u8 as *const libc::c_char,
            has_arg: 0 as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: 'f' as i32,
        };
        init
    },
    {
        let mut init = option {
            name: b"from-file\0" as *const u8 as *const libc::c_char,
            has_arg: 1 as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: FROM_FILE_OPTION as libc::c_int,
        };
        init
    },
    {
        let mut init = option {
            name: b"help\0" as *const u8 as *const libc::c_char,
            has_arg: 0 as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: HELP_OPTION as libc::c_int,
        };
        init
    },
    {
        let mut init = option {
            name: b"horizon-lines\0" as *const u8 as *const libc::c_char,
            has_arg: 1 as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: HORIZON_LINES_OPTION as libc::c_int,
        };
        init
    },
    {
        let mut init = option {
            name: b"ifdef\0" as *const u8 as *const libc::c_char,
            has_arg: 1 as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: 'D' as i32,
        };
        init
    },
    {
        let mut init = option {
            name: b"ignore-all-space\0" as *const u8 as *const libc::c_char,
            has_arg: 0 as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: 'w' as i32,
        };
        init
    },
    {
        let mut init = option {
            name: b"ignore-blank-lines\0" as *const u8 as *const libc::c_char,
            has_arg: 0 as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: 'B' as i32,
        };
        init
    },
    {
        let mut init = option {
            name: b"ignore-case\0" as *const u8 as *const libc::c_char,
            has_arg: 0 as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: 'i' as i32,
        };
        init
    },
    {
        let mut init = option {
            name: b"ignore-file-name-case\0" as *const u8 as *const libc::c_char,
            has_arg: 0 as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: IGNORE_FILE_NAME_CASE_OPTION as libc::c_int,
        };
        init
    },
    {
        let mut init = option {
            name: b"ignore-matching-lines\0" as *const u8 as *const libc::c_char,
            has_arg: 1 as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: 'I' as i32,
        };
        init
    },
    {
        let mut init = option {
            name: b"ignore-space-change\0" as *const u8 as *const libc::c_char,
            has_arg: 0 as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: 'b' as i32,
        };
        init
    },
    {
        let mut init = option {
            name: b"ignore-tab-expansion\0" as *const u8 as *const libc::c_char,
            has_arg: 0 as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: 'E' as i32,
        };
        init
    },
    {
        let mut init = option {
            name: b"ignore-trailing-space\0" as *const u8 as *const libc::c_char,
            has_arg: 0 as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: 'Z' as i32,
        };
        init
    },
    {
        let mut init = option {
            name: b"inhibit-hunk-merge\0" as *const u8 as *const libc::c_char,
            has_arg: 0 as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: INHIBIT_HUNK_MERGE_OPTION as libc::c_int,
        };
        init
    },
    {
        let mut init = option {
            name: b"initial-tab\0" as *const u8 as *const libc::c_char,
            has_arg: 0 as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: 'T' as i32,
        };
        init
    },
    {
        let mut init = option {
            name: b"label\0" as *const u8 as *const libc::c_char,
            has_arg: 1 as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: 'L' as i32,
        };
        init
    },
    {
        let mut init = option {
            name: b"left-column\0" as *const u8 as *const libc::c_char,
            has_arg: 0 as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: LEFT_COLUMN_OPTION as libc::c_int,
        };
        init
    },
    {
        let mut init = option {
            name: b"line-format\0" as *const u8 as *const libc::c_char,
            has_arg: 1 as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: LINE_FORMAT_OPTION as libc::c_int,
        };
        init
    },
    {
        let mut init = option {
            name: b"minimal\0" as *const u8 as *const libc::c_char,
            has_arg: 0 as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: 'd' as i32,
        };
        init
    },
    {
        let mut init = option {
            name: b"new-file\0" as *const u8 as *const libc::c_char,
            has_arg: 0 as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: 'N' as i32,
        };
        init
    },
    {
        let mut init = option {
            name: b"new-group-format\0" as *const u8 as *const libc::c_char,
            has_arg: 1 as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: NEW_GROUP_FORMAT_OPTION as libc::c_int,
        };
        init
    },
    {
        let mut init = option {
            name: b"new-line-format\0" as *const u8 as *const libc::c_char,
            has_arg: 1 as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: NEW_LINE_FORMAT_OPTION as libc::c_int,
        };
        init
    },
    {
        let mut init = option {
            name: b"no-dereference\0" as *const u8 as *const libc::c_char,
            has_arg: 0 as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: NO_DEREFERENCE_OPTION as libc::c_int,
        };
        init
    },
    {
        let mut init = option {
            name: b"no-ignore-file-name-case\0" as *const u8 as *const libc::c_char,
            has_arg: 0 as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: NO_IGNORE_FILE_NAME_CASE_OPTION as libc::c_int,
        };
        init
    },
    {
        let mut init = option {
            name: b"normal\0" as *const u8 as *const libc::c_char,
            has_arg: 0 as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: NORMAL_OPTION as libc::c_int,
        };
        init
    },
    {
        let mut init = option {
            name: b"old-group-format\0" as *const u8 as *const libc::c_char,
            has_arg: 1 as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: OLD_GROUP_FORMAT_OPTION as libc::c_int,
        };
        init
    },
    {
        let mut init = option {
            name: b"old-line-format\0" as *const u8 as *const libc::c_char,
            has_arg: 1 as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: OLD_LINE_FORMAT_OPTION as libc::c_int,
        };
        init
    },
    {
        let mut init = option {
            name: b"paginate\0" as *const u8 as *const libc::c_char,
            has_arg: 0 as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: 'l' as i32,
        };
        init
    },
    {
        let mut init = option {
            name: b"palette\0" as *const u8 as *const libc::c_char,
            has_arg: 1 as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: COLOR_PALETTE_OPTION as libc::c_int,
        };
        init
    },
    {
        let mut init = option {
            name: b"rcs\0" as *const u8 as *const libc::c_char,
            has_arg: 0 as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: 'n' as i32,
        };
        init
    },
    {
        let mut init = option {
            name: b"recursive\0" as *const u8 as *const libc::c_char,
            has_arg: 0 as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: 'r' as i32,
        };
        init
    },
    {
        let mut init = option {
            name: b"report-identical-files\0" as *const u8 as *const libc::c_char,
            has_arg: 0 as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: 's' as i32,
        };
        init
    },
    {
        let mut init = option {
            name: b"sdiff-merge-assist\0" as *const u8 as *const libc::c_char,
            has_arg: 0 as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: SDIFF_MERGE_ASSIST_OPTION as libc::c_int,
        };
        init
    },
    {
        let mut init = option {
            name: b"show-c-function\0" as *const u8 as *const libc::c_char,
            has_arg: 0 as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: 'p' as i32,
        };
        init
    },
    {
        let mut init = option {
            name: b"show-function-line\0" as *const u8 as *const libc::c_char,
            has_arg: 1 as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: 'F' as i32,
        };
        init
    },
    {
        let mut init = option {
            name: b"side-by-side\0" as *const u8 as *const libc::c_char,
            has_arg: 0 as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: 'y' as i32,
        };
        init
    },
    {
        let mut init = option {
            name: b"speed-large-files\0" as *const u8 as *const libc::c_char,
            has_arg: 0 as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: 'H' as i32,
        };
        init
    },
    {
        let mut init = option {
            name: b"starting-file\0" as *const u8 as *const libc::c_char,
            has_arg: 1 as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: 'S' as i32,
        };
        init
    },
    {
        let mut init = option {
            name: b"strip-trailing-cr\0" as *const u8 as *const libc::c_char,
            has_arg: 0 as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: STRIP_TRAILING_CR_OPTION as libc::c_int,
        };
        init
    },
    {
        let mut init = option {
            name: b"suppress-blank-empty\0" as *const u8 as *const libc::c_char,
            has_arg: 0 as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: SUPPRESS_BLANK_EMPTY_OPTION as libc::c_int,
        };
        init
    },
    {
        let mut init = option {
            name: b"suppress-common-lines\0" as *const u8 as *const libc::c_char,
            has_arg: 0 as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: SUPPRESS_COMMON_LINES_OPTION as libc::c_int,
        };
        init
    },
    {
        let mut init = option {
            name: b"tabsize\0" as *const u8 as *const libc::c_char,
            has_arg: 1 as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: TABSIZE_OPTION as libc::c_int,
        };
        init
    },
    {
        let mut init = option {
            name: b"text\0" as *const u8 as *const libc::c_char,
            has_arg: 0 as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: 'a' as i32,
        };
        init
    },
    {
        let mut init = option {
            name: b"to-file\0" as *const u8 as *const libc::c_char,
            has_arg: 1 as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: TO_FILE_OPTION as libc::c_int,
        };
        init
    },
    {
        let mut init = option {
            name: b"unchanged-group-format\0" as *const u8 as *const libc::c_char,
            has_arg: 1 as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: UNCHANGED_GROUP_FORMAT_OPTION as libc::c_int,
        };
        init
    },
    {
        let mut init = option {
            name: b"unchanged-line-format\0" as *const u8 as *const libc::c_char,
            has_arg: 1 as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: UNCHANGED_LINE_FORMAT_OPTION as libc::c_int,
        };
        init
    },
    {
        let mut init = option {
            name: b"unidirectional-new-file\0" as *const u8 as *const libc::c_char,
            has_arg: 0 as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: 'P' as i32,
        };
        init
    },
    {
        let mut init = option {
            name: b"unified\0" as *const u8 as *const libc::c_char,
            has_arg: 2 as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: 'U' as i32,
        };
        init
    },
    {
        let mut init = option {
            name: b"version\0" as *const u8 as *const libc::c_char,
            has_arg: 0 as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: 'v' as i32,
        };
        init
    },
    {
        let mut init = option {
            name: b"width\0" as *const u8 as *const libc::c_char,
            has_arg: 1 as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: 'W' as i32,
        };
        init
    },
    {
        let mut init = option {
            name: b"-no-directory\0" as *const u8 as *const libc::c_char,
            has_arg: 0 as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: NO_DIRECTORY_OPTION as libc::c_int,
        };
        init
    },
    {
        let mut init = option {
            name: b"-presume-output-tty\0" as *const u8 as *const libc::c_char,
            has_arg: 0 as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: PRESUME_OUTPUT_TTY_OPTION as libc::c_int,
        };
        init
    },
    {
        let mut init = option {
            name: 0 as *const libc::c_char,
            has_arg: 0 as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: 0 as libc::c_int,
        };
        init
    },
];
unsafe extern "C" fn option_list(
    mut optionvec: *mut *mut libc::c_char,
    mut count: libc::c_int,
) -> *mut libc::c_char {
    let mut i: libc::c_int = 0;
    let mut size: size_t = 1 as libc::c_int as size_t;
    let mut result: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut p: *mut libc::c_char = 0 as *mut libc::c_char;
    i = 0 as libc::c_int;
    while i < count {
        let mut optsize: size_t = (1 as libc::c_int as libc::c_ulong)
            .wrapping_add(shell_quote_length(*optionvec.offset(i as isize)));
        let (fresh1, fresh2) = optsize.overflowing_add(size);
        *(&mut size as *mut size_t) = fresh1;
        if fresh2 {
            xalloc_die();
        }
        i += 1;
        i;
    }
    result = xmalloc(size) as *mut libc::c_char;
    p = result;
    i = 0 as libc::c_int;
    while i < count {
        let fresh3 = p;
        p = p.offset(1);
        *fresh3 = ' ' as i32 as libc::c_char;
        p = shell_quote_copy(p, *optionvec.offset(i as isize));
        i += 1;
        i;
    }
    *p = '\0' as i32 as libc::c_char;
    return result;
}
unsafe extern "C" fn exclude_options() -> libc::c_int {
    return (1 as libc::c_int) << 28 as libc::c_int
        | (if ignore_file_name_case as libc::c_int != 0 {
            (1 as libc::c_int) << 4 as libc::c_int
        } else {
            0 as libc::c_int
        });
}
unsafe fn main_0(
    mut argc: libc::c_int,
    mut argv: *mut *mut libc::c_char,
) -> libc::c_int {
    let mut exit_status: libc::c_int = 0 as libc::c_int;
    let mut c: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    let mut prev: libc::c_int = -(1 as libc::c_int);
    let mut ocontext: lin = -(1 as libc::c_int) as lin;
    let mut explicit_context: bool = 0 as libc::c_int != 0;
    let mut width: size_t = 0 as libc::c_int as size_t;
    let mut show_c_function: bool = 0 as libc::c_int != 0;
    let mut from_file: *const libc::c_char = 0 as *const libc::c_char;
    let mut to_file: *const libc::c_char = 0 as *const libc::c_char;
    let mut numval: intmax_t = 0;
    let mut numend: *mut libc::c_char = 0 as *mut libc::c_char;
    ::std::ptr::write_volatile(&mut exit_failure as *mut libc::c_int, 2 as libc::c_int);
    set_program_name(*argv.offset(0 as libc::c_int as isize));
    setlocale(6 as libc::c_int, b"\0" as *const u8 as *const libc::c_char);
    bindtextdomain(
        b"diffutils\0" as *const u8 as *const libc::c_char,
        b"/usr/local/share/locale\0" as *const u8 as *const libc::c_char,
    );
    textdomain(b"diffutils\0" as *const u8 as *const libc::c_char);
    c_stack_action(None);
    function_regexp_list.buf = &mut function_regexp;
    ignore_regexp_list.buf = &mut ignore_regexp;
    rpl_re_set_syntax(
        (((1 as libc::c_int as libc::c_ulong) << 1 as libc::c_int) << 1 as libc::c_int
            | ((((((1 as libc::c_int as libc::c_ulong) << 1 as libc::c_int)
                << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
                << 1 as libc::c_int) << 1 as libc::c_int
            | (((((((1 as libc::c_int as libc::c_ulong) << 1 as libc::c_int)
                << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
                << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int
            | (((((((((1 as libc::c_int as libc::c_ulong) << 1 as libc::c_int)
                << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
                << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
                << 1 as libc::c_int) << 1 as libc::c_int
            | ((((((((((((((((1 as libc::c_int as libc::c_ulong) << 1 as libc::c_int)
                << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
                << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
                << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
                << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
                << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int
            | (1 as libc::c_int as libc::c_ulong) << 1 as libc::c_int
            | ((((((((((((((((((((((((1 as libc::c_int as libc::c_ulong)
                << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
                << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
                << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
                << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
                << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
                << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
                << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
                << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int
            | (((((((((((1 as libc::c_int as libc::c_ulong) << 1 as libc::c_int)
                << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
                << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
                << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
                << 1 as libc::c_int)
            & !(((((((((((((((((((((((((1 as libc::c_int as libc::c_ulong)
                << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
                << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
                << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
                << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
                << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
                << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
                << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
                << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int
                | (((((((1 as libc::c_int as libc::c_ulong) << 1 as libc::c_int)
                    << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
                    << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
            | ((((((((((((((((((1 as libc::c_int as libc::c_ulong) << 1 as libc::c_int)
                << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
                << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
                << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
                << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
                << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
                << 1 as libc::c_int) << 1 as libc::c_int,
    );
    excluded = new_exclude();
    presume_output_tty = 0 as libc::c_int != 0;
    xstdopen();
    loop {
        c = getopt_long(
            argc,
            argv,
            shortopts.as_ptr(),
            longopts.as_ptr(),
            0 as *mut libc::c_int,
        );
        if !(c != -(1 as libc::c_int)) {
            break;
        }
        match c {
            48 | 49 | 50 | 51 | 52 | 53 | 54 | 55 | 56 | 57 => {
                ocontext = if !((prev as libc::c_uint)
                    .wrapping_sub('0' as i32 as libc::c_uint)
                    <= 9 as libc::c_int as libc::c_uint)
                {
                    (c - '0' as i32) as libc::c_long
                } else if (ocontext
                    - ((c - '0' as i32) as libc::c_long
                        <= (9223372036854775807 as libc::c_long
                            - 1 as libc::c_int as libc::c_long)
                            / 2 as libc::c_int as libc::c_long
                            % 10 as libc::c_int as libc::c_long) as libc::c_int
                        as libc::c_long)
                    < (9223372036854775807 as libc::c_long
                        - 1 as libc::c_int as libc::c_long)
                        / 2 as libc::c_int as libc::c_long
                        / 10 as libc::c_int as libc::c_long
                {
                    10 as libc::c_int as libc::c_long * ocontext
                        + (c - '0' as i32) as libc::c_long
                } else {
                    (9223372036854775807 as libc::c_long
                        - 1 as libc::c_int as libc::c_long)
                        / 2 as libc::c_int as libc::c_long
                };
            }
            97 => {
                text = 1 as libc::c_int != 0;
            }
            98 => {
                if (ignore_white_space as libc::c_uint)
                    < IGNORE_SPACE_CHANGE as libc::c_int as libc::c_uint
                {
                    ignore_white_space = IGNORE_SPACE_CHANGE;
                }
            }
            90 => {
                if (ignore_white_space as libc::c_uint)
                    < IGNORE_SPACE_CHANGE as libc::c_int as libc::c_uint
                {
                    ignore_white_space = ::std::mem::transmute::<
                        libc::c_uint,
                        DIFF_white_space,
                    >(
                        ignore_white_space as libc::c_uint
                            | IGNORE_TRAILING_SPACE as libc::c_int as libc::c_uint,
                    );
                }
            }
            66 => {
                ignore_blank_lines = 1 as libc::c_int != 0;
            }
            67 | 85 => {
                if !optarg.is_null() {
                    numval = strtoimax(optarg, &mut numend, 10 as libc::c_int);
                    if *numend as libc::c_int != 0
                        || numval < 0 as libc::c_int as libc::c_long
                    {
                        try_help(
                            b"invalid context length '%s'\0" as *const u8
                                as *const libc::c_char,
                            optarg,
                        );
                    }
                    if ((9223372036854775807 as libc::c_long
                        - 1 as libc::c_int as libc::c_long)
                        / 2 as libc::c_int as libc::c_long) < numval
                    {
                        numval = (9223372036854775807 as libc::c_long
                            - 1 as libc::c_int as libc::c_long)
                            / 2 as libc::c_int as libc::c_long;
                    }
                } else {
                    numval = 3 as libc::c_int as intmax_t;
                }
                specify_style(
                    (if c == 'U' as i32 {
                        OUTPUT_UNIFIED as libc::c_int
                    } else {
                        OUTPUT_CONTEXT as libc::c_int
                    }) as output_style,
                );
                if context < numval {
                    context = numval;
                }
                explicit_context = 1 as libc::c_int != 0;
            }
            99 => {
                specify_style(OUTPUT_CONTEXT);
                if context < 3 as libc::c_int as libc::c_long {
                    context = 3 as libc::c_int as lin;
                }
            }
            100 => {
                minimal = 1 as libc::c_int != 0;
            }
            68 => {
                specify_style(OUTPUT_IFDEF);
                static mut C_ifdef_group_formats: [libc::c_char; 104] = unsafe {
                    *::std::mem::transmute::<
                        &[u8; 104],
                        &[libc::c_char; 104],
                    >(
                        b"%=\0#ifndef @\n%<#endif /* ! @ */\n\0#ifdef @\n%>#endif /* @ */\n\0#ifndef @\n%<#else /* @ */\n%>#endif /* @ */\n\0",
                    )
                };
                let mut alloc: size_t = strlen(optarg);
                if (if ::std::mem::size_of::<size_t>() as libc::c_ulong
                    == ::std::mem::size_of::<libc::c_schar>() as libc::c_ulong
                {
                    if !((0 as libc::c_int as size_t) < -(1 as libc::c_int) as size_t) {
                        if (if (7 as libc::c_int) < 0 as libc::c_int {
                            if alloc < 0 as libc::c_int as libc::c_ulong {
                                if ((if 1 as libc::c_int != 0 {
                                    0 as libc::c_int
                                } else {
                                    (if 1 as libc::c_int != 0 {
                                        0 as libc::c_int
                                    } else {
                                        127 as libc::c_int
                                    }) + 7 as libc::c_int
                                }) - 1 as libc::c_int) < 0 as libc::c_int
                                {
                                    (alloc
                                        < (127 as libc::c_int / 7 as libc::c_int) as libc::c_ulong)
                                        as libc::c_int
                                } else {
                                    ((if (if (if ((if 1 as libc::c_int != 0 {
                                        0 as libc::c_int
                                    } else {
                                        7 as libc::c_int
                                    }) - 1 as libc::c_int) < 0 as libc::c_int
                                    {
                                        !(((((if 1 as libc::c_int != 0 {
                                            0 as libc::c_int
                                        } else {
                                            7 as libc::c_int
                                        }) + 1 as libc::c_int)
                                            << (::std::mem::size_of::<libc::c_int>() as libc::c_ulong)
                                                .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                                                .wrapping_sub(2 as libc::c_int as libc::c_ulong))
                                            - 1 as libc::c_int) * 2 as libc::c_int + 1 as libc::c_int)
                                    } else {
                                        (if 1 as libc::c_int != 0 {
                                            0 as libc::c_int
                                        } else {
                                            7 as libc::c_int
                                        }) + 0 as libc::c_int
                                    }) < 0 as libc::c_int
                                    {
                                        ((7 as libc::c_int)
                                            < -(if ((if 1 as libc::c_int != 0 {
                                                0 as libc::c_int
                                            } else {
                                                7 as libc::c_int
                                            }) - 1 as libc::c_int) < 0 as libc::c_int
                                            {
                                                ((((if 1 as libc::c_int != 0 {
                                                    0 as libc::c_int
                                                } else {
                                                    7 as libc::c_int
                                                }) + 1 as libc::c_int)
                                                    << (::std::mem::size_of::<libc::c_int>() as libc::c_ulong)
                                                        .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                                                        .wrapping_sub(2 as libc::c_int as libc::c_ulong))
                                                    - 1 as libc::c_int) * 2 as libc::c_int + 1 as libc::c_int
                                            } else {
                                                (if 1 as libc::c_int != 0 {
                                                    0 as libc::c_int
                                                } else {
                                                    7 as libc::c_int
                                                }) - 1 as libc::c_int
                                            })) as libc::c_int
                                    } else {
                                        ((0 as libc::c_int) < 7 as libc::c_int) as libc::c_int
                                    }) != 0
                                    {
                                        (if 1 as libc::c_int != 0 {
                                            0 as libc::c_int
                                        } else {
                                            7 as libc::c_int
                                        }) + 127 as libc::c_int
                                            >> (::std::mem::size_of::<libc::c_int>() as libc::c_ulong)
                                                .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                                                .wrapping_sub(1 as libc::c_int as libc::c_ulong)
                                    } else {
                                        127 as libc::c_int / -(7 as libc::c_int)
                                    }) as libc::c_ulong
                                        <= (-(1 as libc::c_int) as libc::c_ulong)
                                            .wrapping_sub(alloc)) as libc::c_int
                                }
                            } else {
                                if (if (if ((if 1 as libc::c_int != 0 {
                                    0 as libc::c_int
                                } else {
                                    (if 1 as libc::c_int != 0 {
                                        0 as libc::c_int
                                    } else {
                                        7 as libc::c_int
                                    }) + (-(127 as libc::c_int) - 1 as libc::c_int)
                                }) - 1 as libc::c_int) < 0 as libc::c_int
                                {
                                    !(((((if 1 as libc::c_int != 0 {
                                        0 as libc::c_int
                                    } else {
                                        (if 1 as libc::c_int != 0 {
                                            0 as libc::c_int
                                        } else {
                                            7 as libc::c_int
                                        }) + (-(127 as libc::c_int) - 1 as libc::c_int)
                                    }) + 1 as libc::c_int)
                                        << (::std::mem::size_of::<libc::c_int>() as libc::c_ulong)
                                            .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                                            .wrapping_sub(2 as libc::c_int as libc::c_ulong))
                                        - 1 as libc::c_int) * 2 as libc::c_int + 1 as libc::c_int)
                                } else {
                                    (if 1 as libc::c_int != 0 {
                                        0 as libc::c_int
                                    } else {
                                        (if 1 as libc::c_int != 0 {
                                            0 as libc::c_int
                                        } else {
                                            7 as libc::c_int
                                        }) + (-(127 as libc::c_int) - 1 as libc::c_int)
                                    }) + 0 as libc::c_int
                                }) < 0 as libc::c_int
                                {
                                    ((if 1 as libc::c_int != 0 {
                                        0 as libc::c_int
                                    } else {
                                        7 as libc::c_int
                                    }) + (-(127 as libc::c_int) - 1 as libc::c_int)
                                        < -(if ((if 1 as libc::c_int != 0 {
                                            0 as libc::c_int
                                        } else {
                                            (if 1 as libc::c_int != 0 {
                                                0 as libc::c_int
                                            } else {
                                                7 as libc::c_int
                                            }) + (-(127 as libc::c_int) - 1 as libc::c_int)
                                        }) - 1 as libc::c_int) < 0 as libc::c_int
                                        {
                                            ((((if 1 as libc::c_int != 0 {
                                                0 as libc::c_int
                                            } else {
                                                (if 1 as libc::c_int != 0 {
                                                    0 as libc::c_int
                                                } else {
                                                    7 as libc::c_int
                                                }) + (-(127 as libc::c_int) - 1 as libc::c_int)
                                            }) + 1 as libc::c_int)
                                                << (::std::mem::size_of::<libc::c_int>() as libc::c_ulong)
                                                    .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                                                    .wrapping_sub(2 as libc::c_int as libc::c_ulong))
                                                - 1 as libc::c_int) * 2 as libc::c_int + 1 as libc::c_int
                                        } else {
                                            (if 1 as libc::c_int != 0 {
                                                0 as libc::c_int
                                            } else {
                                                (if 1 as libc::c_int != 0 {
                                                    0 as libc::c_int
                                                } else {
                                                    7 as libc::c_int
                                                }) + (-(127 as libc::c_int) - 1 as libc::c_int)
                                            }) - 1 as libc::c_int
                                        })) as libc::c_int
                                } else {
                                    ((0 as libc::c_int)
                                        < (if 1 as libc::c_int != 0 {
                                            0 as libc::c_int
                                        } else {
                                            7 as libc::c_int
                                        }) + (-(127 as libc::c_int) - 1 as libc::c_int))
                                        as libc::c_int
                                }) != 0 && 7 as libc::c_int == -(1 as libc::c_int)
                                {
                                    if (if 1 as libc::c_int != 0 {
                                        0 as libc::c_int as libc::c_ulong
                                    } else {
                                        alloc
                                    })
                                        .wrapping_sub(1 as libc::c_int as libc::c_ulong)
                                        < 0 as libc::c_int as libc::c_ulong
                                    {
                                        ((0 as libc::c_int as libc::c_ulong)
                                            < alloc
                                                .wrapping_add(
                                                    (-(127 as libc::c_int) - 1 as libc::c_int) as libc::c_ulong,
                                                )) as libc::c_int
                                    } else {
                                        ((0 as libc::c_int as libc::c_ulong) < alloc
                                            && ((-(1 as libc::c_int)
                                                - (-(127 as libc::c_int) - 1 as libc::c_int))
                                                as libc::c_ulong)
                                                < alloc.wrapping_sub(1 as libc::c_int as libc::c_ulong))
                                            as libc::c_int
                                    }
                                } else {
                                    ((((-(127 as libc::c_int) - 1 as libc::c_int)
                                        / 7 as libc::c_int) as libc::c_ulong) < alloc)
                                        as libc::c_int
                                }
                            }
                        } else {
                            if 7 as libc::c_int == 0 as libc::c_int {
                                0 as libc::c_int
                            } else {
                                if alloc < 0 as libc::c_int as libc::c_ulong {
                                    if (if (if (if 1 as libc::c_int != 0 {
                                        0 as libc::c_int as libc::c_ulong
                                    } else {
                                        (if 1 as libc::c_int != 0 {
                                            0 as libc::c_int as libc::c_ulong
                                        } else {
                                            alloc
                                        })
                                            .wrapping_add(
                                                (-(127 as libc::c_int) - 1 as libc::c_int) as libc::c_ulong,
                                            )
                                    })
                                        .wrapping_sub(1 as libc::c_int as libc::c_ulong)
                                        < 0 as libc::c_int as libc::c_ulong
                                    {
                                        !((if 1 as libc::c_int != 0 {
                                            0 as libc::c_int as libc::c_ulong
                                        } else {
                                            (if 1 as libc::c_int != 0 {
                                                0 as libc::c_int as libc::c_ulong
                                            } else {
                                                alloc
                                            })
                                                .wrapping_add(
                                                    (-(127 as libc::c_int) - 1 as libc::c_int) as libc::c_ulong,
                                                )
                                        })
                                            .wrapping_add(1 as libc::c_int as libc::c_ulong)
                                            << (::std::mem::size_of::<libc::c_ulong>() as libc::c_ulong)
                                                .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                                                .wrapping_sub(2 as libc::c_int as libc::c_ulong))
                                            .wrapping_sub(1 as libc::c_int as libc::c_ulong)
                                            .wrapping_mul(2 as libc::c_int as libc::c_ulong)
                                            .wrapping_add(1 as libc::c_int as libc::c_ulong)
                                    } else {
                                        (if 1 as libc::c_int != 0 {
                                            0 as libc::c_int as libc::c_ulong
                                        } else {
                                            (if 1 as libc::c_int != 0 {
                                                0 as libc::c_int as libc::c_ulong
                                            } else {
                                                alloc
                                            })
                                                .wrapping_add(
                                                    (-(127 as libc::c_int) - 1 as libc::c_int) as libc::c_ulong,
                                                )
                                        })
                                            .wrapping_add(0 as libc::c_int as libc::c_ulong)
                                    }) < 0 as libc::c_int as libc::c_ulong
                                    {
                                        ((if 1 as libc::c_int != 0 {
                                            0 as libc::c_int as libc::c_ulong
                                        } else {
                                            alloc
                                        })
                                            .wrapping_add(
                                                (-(127 as libc::c_int) - 1 as libc::c_int) as libc::c_ulong,
                                            )
                                            < (if (if 1 as libc::c_int != 0 {
                                                0 as libc::c_int as libc::c_ulong
                                            } else {
                                                (if 1 as libc::c_int != 0 {
                                                    0 as libc::c_int as libc::c_ulong
                                                } else {
                                                    alloc
                                                })
                                                    .wrapping_add(
                                                        (-(127 as libc::c_int) - 1 as libc::c_int) as libc::c_ulong,
                                                    )
                                            })
                                                .wrapping_sub(1 as libc::c_int as libc::c_ulong)
                                                < 0 as libc::c_int as libc::c_ulong
                                            {
                                                ((if 1 as libc::c_int != 0 {
                                                    0 as libc::c_int as libc::c_ulong
                                                } else {
                                                    (if 1 as libc::c_int != 0 {
                                                        0 as libc::c_int as libc::c_ulong
                                                    } else {
                                                        alloc
                                                    })
                                                        .wrapping_add(
                                                            (-(127 as libc::c_int) - 1 as libc::c_int) as libc::c_ulong,
                                                        )
                                                })
                                                    .wrapping_add(1 as libc::c_int as libc::c_ulong)
                                                    << (::std::mem::size_of::<libc::c_ulong>() as libc::c_ulong)
                                                        .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                                                        .wrapping_sub(2 as libc::c_int as libc::c_ulong))
                                                    .wrapping_sub(1 as libc::c_int as libc::c_ulong)
                                                    .wrapping_mul(2 as libc::c_int as libc::c_ulong)
                                                    .wrapping_add(1 as libc::c_int as libc::c_ulong)
                                            } else {
                                                (if 1 as libc::c_int != 0 {
                                                    0 as libc::c_int as libc::c_ulong
                                                } else {
                                                    (if 1 as libc::c_int != 0 {
                                                        0 as libc::c_int as libc::c_ulong
                                                    } else {
                                                        alloc
                                                    })
                                                        .wrapping_add(
                                                            (-(127 as libc::c_int) - 1 as libc::c_int) as libc::c_ulong,
                                                        )
                                                })
                                                    .wrapping_sub(1 as libc::c_int as libc::c_ulong)
                                            })
                                                .wrapping_neg()) as libc::c_int
                                    } else {
                                        ((0 as libc::c_int as libc::c_ulong)
                                            < (if 1 as libc::c_int != 0 {
                                                0 as libc::c_int as libc::c_ulong
                                            } else {
                                                alloc
                                            })
                                                .wrapping_add(
                                                    (-(127 as libc::c_int) - 1 as libc::c_int) as libc::c_ulong,
                                                )) as libc::c_int
                                    }) != 0 && alloc == -(1 as libc::c_int) as libc::c_ulong
                                    {
                                        if ((if 1 as libc::c_int != 0 {
                                            0 as libc::c_int
                                        } else {
                                            7 as libc::c_int
                                        }) - 1 as libc::c_int) < 0 as libc::c_int
                                        {
                                            ((0 as libc::c_int)
                                                < 7 as libc::c_int
                                                    + (-(127 as libc::c_int) - 1 as libc::c_int)) as libc::c_int
                                        } else {
                                            (-(1 as libc::c_int)
                                                - (-(127 as libc::c_int) - 1 as libc::c_int)
                                                < 7 as libc::c_int - 1 as libc::c_int) as libc::c_int
                                        }
                                    } else {
                                        (((-(127 as libc::c_int) - 1 as libc::c_int)
                                            as libc::c_ulong)
                                            .wrapping_div(alloc) < 7 as libc::c_int as libc::c_ulong)
                                            as libc::c_int
                                    }
                                } else {
                                    (((127 as libc::c_int / 7 as libc::c_int) as libc::c_ulong)
                                        < alloc) as libc::c_int
                                }
                            }
                        }) != 0
                        {
                            alloc = (alloc as libc::c_uint)
                                .wrapping_mul(7 as libc::c_int as libc::c_uint)
                                as libc::c_schar as size_t;
                            1 as libc::c_int
                        } else {
                            alloc = (alloc as libc::c_uint)
                                .wrapping_mul(7 as libc::c_int as libc::c_uint)
                                as libc::c_schar as size_t;
                            0 as libc::c_int
                        }
                    } else {
                        if (if (7 as libc::c_int) < 0 as libc::c_int {
                            if alloc < 0 as libc::c_int as libc::c_ulong {
                                if ((if 1 as libc::c_int != 0 {
                                    0 as libc::c_int
                                } else {
                                    (if 1 as libc::c_int != 0 {
                                        0 as libc::c_int
                                    } else {
                                        127 as libc::c_int * 2 as libc::c_int + 1 as libc::c_int
                                    }) + 7 as libc::c_int
                                }) - 1 as libc::c_int) < 0 as libc::c_int
                                {
                                    (alloc
                                        < ((127 as libc::c_int * 2 as libc::c_int
                                            + 1 as libc::c_int) / 7 as libc::c_int) as libc::c_ulong)
                                        as libc::c_int
                                } else {
                                    ((if (if (if ((if 1 as libc::c_int != 0 {
                                        0 as libc::c_int
                                    } else {
                                        7 as libc::c_int
                                    }) - 1 as libc::c_int) < 0 as libc::c_int
                                    {
                                        !(((((if 1 as libc::c_int != 0 {
                                            0 as libc::c_int
                                        } else {
                                            7 as libc::c_int
                                        }) + 1 as libc::c_int)
                                            << (::std::mem::size_of::<libc::c_int>() as libc::c_ulong)
                                                .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                                                .wrapping_sub(2 as libc::c_int as libc::c_ulong))
                                            - 1 as libc::c_int) * 2 as libc::c_int + 1 as libc::c_int)
                                    } else {
                                        (if 1 as libc::c_int != 0 {
                                            0 as libc::c_int
                                        } else {
                                            7 as libc::c_int
                                        }) + 0 as libc::c_int
                                    }) < 0 as libc::c_int
                                    {
                                        ((7 as libc::c_int)
                                            < -(if ((if 1 as libc::c_int != 0 {
                                                0 as libc::c_int
                                            } else {
                                                7 as libc::c_int
                                            }) - 1 as libc::c_int) < 0 as libc::c_int
                                            {
                                                ((((if 1 as libc::c_int != 0 {
                                                    0 as libc::c_int
                                                } else {
                                                    7 as libc::c_int
                                                }) + 1 as libc::c_int)
                                                    << (::std::mem::size_of::<libc::c_int>() as libc::c_ulong)
                                                        .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                                                        .wrapping_sub(2 as libc::c_int as libc::c_ulong))
                                                    - 1 as libc::c_int) * 2 as libc::c_int + 1 as libc::c_int
                                            } else {
                                                (if 1 as libc::c_int != 0 {
                                                    0 as libc::c_int
                                                } else {
                                                    7 as libc::c_int
                                                }) - 1 as libc::c_int
                                            })) as libc::c_int
                                    } else {
                                        ((0 as libc::c_int) < 7 as libc::c_int) as libc::c_int
                                    }) != 0
                                    {
                                        (if 1 as libc::c_int != 0 {
                                            0 as libc::c_int
                                        } else {
                                            7 as libc::c_int
                                        })
                                            + (127 as libc::c_int * 2 as libc::c_int + 1 as libc::c_int)
                                            >> (::std::mem::size_of::<libc::c_int>() as libc::c_ulong)
                                                .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                                                .wrapping_sub(1 as libc::c_int as libc::c_ulong)
                                    } else {
                                        (127 as libc::c_int * 2 as libc::c_int + 1 as libc::c_int)
                                            / -(7 as libc::c_int)
                                    }) as libc::c_ulong
                                        <= (-(1 as libc::c_int) as libc::c_ulong)
                                            .wrapping_sub(alloc)) as libc::c_int
                                }
                            } else {
                                if (if (if ((if 1 as libc::c_int != 0 {
                                    0 as libc::c_int
                                } else {
                                    (if 1 as libc::c_int != 0 {
                                        0 as libc::c_int
                                    } else {
                                        7 as libc::c_int
                                    }) + 0 as libc::c_int
                                }) - 1 as libc::c_int) < 0 as libc::c_int
                                {
                                    !(((((if 1 as libc::c_int != 0 {
                                        0 as libc::c_int
                                    } else {
                                        (if 1 as libc::c_int != 0 {
                                            0 as libc::c_int
                                        } else {
                                            7 as libc::c_int
                                        }) + 0 as libc::c_int
                                    }) + 1 as libc::c_int)
                                        << (::std::mem::size_of::<libc::c_int>() as libc::c_ulong)
                                            .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                                            .wrapping_sub(2 as libc::c_int as libc::c_ulong))
                                        - 1 as libc::c_int) * 2 as libc::c_int + 1 as libc::c_int)
                                } else {
                                    (if 1 as libc::c_int != 0 {
                                        0 as libc::c_int
                                    } else {
                                        (if 1 as libc::c_int != 0 {
                                            0 as libc::c_int
                                        } else {
                                            7 as libc::c_int
                                        }) + 0 as libc::c_int
                                    }) + 0 as libc::c_int
                                }) < 0 as libc::c_int
                                {
                                    (((if 1 as libc::c_int != 0 {
                                        0 as libc::c_int
                                    } else {
                                        7 as libc::c_int
                                    }) + 0 as libc::c_int)
                                        < -(if ((if 1 as libc::c_int != 0 {
                                            0 as libc::c_int
                                        } else {
                                            (if 1 as libc::c_int != 0 {
                                                0 as libc::c_int
                                            } else {
                                                7 as libc::c_int
                                            }) + 0 as libc::c_int
                                        }) - 1 as libc::c_int) < 0 as libc::c_int
                                        {
                                            ((((if 1 as libc::c_int != 0 {
                                                0 as libc::c_int
                                            } else {
                                                (if 1 as libc::c_int != 0 {
                                                    0 as libc::c_int
                                                } else {
                                                    7 as libc::c_int
                                                }) + 0 as libc::c_int
                                            }) + 1 as libc::c_int)
                                                << (::std::mem::size_of::<libc::c_int>() as libc::c_ulong)
                                                    .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                                                    .wrapping_sub(2 as libc::c_int as libc::c_ulong))
                                                - 1 as libc::c_int) * 2 as libc::c_int + 1 as libc::c_int
                                        } else {
                                            (if 1 as libc::c_int != 0 {
                                                0 as libc::c_int
                                            } else {
                                                (if 1 as libc::c_int != 0 {
                                                    0 as libc::c_int
                                                } else {
                                                    7 as libc::c_int
                                                }) + 0 as libc::c_int
                                            }) - 1 as libc::c_int
                                        })) as libc::c_int
                                } else {
                                    ((0 as libc::c_int)
                                        < (if 1 as libc::c_int != 0 {
                                            0 as libc::c_int
                                        } else {
                                            7 as libc::c_int
                                        }) + 0 as libc::c_int) as libc::c_int
                                }) != 0 && 7 as libc::c_int == -(1 as libc::c_int)
                                {
                                    if (if 1 as libc::c_int != 0 {
                                        0 as libc::c_int as libc::c_ulong
                                    } else {
                                        alloc
                                    })
                                        .wrapping_sub(1 as libc::c_int as libc::c_ulong)
                                        < 0 as libc::c_int as libc::c_ulong
                                    {
                                        ((0 as libc::c_int as libc::c_ulong)
                                            < alloc.wrapping_add(0 as libc::c_int as libc::c_ulong))
                                            as libc::c_int
                                    } else {
                                        ((0 as libc::c_int as libc::c_ulong) < alloc
                                            && ((-(1 as libc::c_int) - 0 as libc::c_int)
                                                as libc::c_ulong)
                                                < alloc.wrapping_sub(1 as libc::c_int as libc::c_ulong))
                                            as libc::c_int
                                    }
                                } else {
                                    (((0 as libc::c_int / 7 as libc::c_int) as libc::c_ulong)
                                        < alloc) as libc::c_int
                                }
                            }
                        } else {
                            if 7 as libc::c_int == 0 as libc::c_int {
                                0 as libc::c_int
                            } else {
                                if alloc < 0 as libc::c_int as libc::c_ulong {
                                    if (if (if (if 1 as libc::c_int != 0 {
                                        0 as libc::c_int as libc::c_ulong
                                    } else {
                                        (if 1 as libc::c_int != 0 {
                                            0 as libc::c_int as libc::c_ulong
                                        } else {
                                            alloc
                                        })
                                            .wrapping_add(0 as libc::c_int as libc::c_ulong)
                                    })
                                        .wrapping_sub(1 as libc::c_int as libc::c_ulong)
                                        < 0 as libc::c_int as libc::c_ulong
                                    {
                                        !((if 1 as libc::c_int != 0 {
                                            0 as libc::c_int as libc::c_ulong
                                        } else {
                                            (if 1 as libc::c_int != 0 {
                                                0 as libc::c_int as libc::c_ulong
                                            } else {
                                                alloc
                                            })
                                                .wrapping_add(0 as libc::c_int as libc::c_ulong)
                                        })
                                            .wrapping_add(1 as libc::c_int as libc::c_ulong)
                                            << (::std::mem::size_of::<libc::c_ulong>() as libc::c_ulong)
                                                .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                                                .wrapping_sub(2 as libc::c_int as libc::c_ulong))
                                            .wrapping_sub(1 as libc::c_int as libc::c_ulong)
                                            .wrapping_mul(2 as libc::c_int as libc::c_ulong)
                                            .wrapping_add(1 as libc::c_int as libc::c_ulong)
                                    } else {
                                        (if 1 as libc::c_int != 0 {
                                            0 as libc::c_int as libc::c_ulong
                                        } else {
                                            (if 1 as libc::c_int != 0 {
                                                0 as libc::c_int as libc::c_ulong
                                            } else {
                                                alloc
                                            })
                                                .wrapping_add(0 as libc::c_int as libc::c_ulong)
                                        })
                                            .wrapping_add(0 as libc::c_int as libc::c_ulong)
                                    }) < 0 as libc::c_int as libc::c_ulong
                                    {
                                        ((if 1 as libc::c_int != 0 {
                                            0 as libc::c_int as libc::c_ulong
                                        } else {
                                            alloc
                                        })
                                            .wrapping_add(0 as libc::c_int as libc::c_ulong)
                                            < (if (if 1 as libc::c_int != 0 {
                                                0 as libc::c_int as libc::c_ulong
                                            } else {
                                                (if 1 as libc::c_int != 0 {
                                                    0 as libc::c_int as libc::c_ulong
                                                } else {
                                                    alloc
                                                })
                                                    .wrapping_add(0 as libc::c_int as libc::c_ulong)
                                            })
                                                .wrapping_sub(1 as libc::c_int as libc::c_ulong)
                                                < 0 as libc::c_int as libc::c_ulong
                                            {
                                                ((if 1 as libc::c_int != 0 {
                                                    0 as libc::c_int as libc::c_ulong
                                                } else {
                                                    (if 1 as libc::c_int != 0 {
                                                        0 as libc::c_int as libc::c_ulong
                                                    } else {
                                                        alloc
                                                    })
                                                        .wrapping_add(0 as libc::c_int as libc::c_ulong)
                                                })
                                                    .wrapping_add(1 as libc::c_int as libc::c_ulong)
                                                    << (::std::mem::size_of::<libc::c_ulong>() as libc::c_ulong)
                                                        .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                                                        .wrapping_sub(2 as libc::c_int as libc::c_ulong))
                                                    .wrapping_sub(1 as libc::c_int as libc::c_ulong)
                                                    .wrapping_mul(2 as libc::c_int as libc::c_ulong)
                                                    .wrapping_add(1 as libc::c_int as libc::c_ulong)
                                            } else {
                                                (if 1 as libc::c_int != 0 {
                                                    0 as libc::c_int as libc::c_ulong
                                                } else {
                                                    (if 1 as libc::c_int != 0 {
                                                        0 as libc::c_int as libc::c_ulong
                                                    } else {
                                                        alloc
                                                    })
                                                        .wrapping_add(0 as libc::c_int as libc::c_ulong)
                                                })
                                                    .wrapping_sub(1 as libc::c_int as libc::c_ulong)
                                            })
                                                .wrapping_neg()) as libc::c_int
                                    } else {
                                        ((0 as libc::c_int as libc::c_ulong)
                                            < (if 1 as libc::c_int != 0 {
                                                0 as libc::c_int as libc::c_ulong
                                            } else {
                                                alloc
                                            })
                                                .wrapping_add(0 as libc::c_int as libc::c_ulong))
                                            as libc::c_int
                                    }) != 0 && alloc == -(1 as libc::c_int) as libc::c_ulong
                                    {
                                        if ((if 1 as libc::c_int != 0 {
                                            0 as libc::c_int
                                        } else {
                                            7 as libc::c_int
                                        }) - 1 as libc::c_int) < 0 as libc::c_int
                                        {
                                            ((0 as libc::c_int) < 7 as libc::c_int + 0 as libc::c_int)
                                                as libc::c_int
                                        } else {
                                            ((-(1 as libc::c_int) - 0 as libc::c_int)
                                                < 7 as libc::c_int - 1 as libc::c_int) as libc::c_int
                                        }
                                    } else {
                                        ((0 as libc::c_int as libc::c_ulong).wrapping_div(alloc)
                                            < 7 as libc::c_int as libc::c_ulong) as libc::c_int
                                    }
                                } else {
                                    ((((127 as libc::c_int * 2 as libc::c_int
                                        + 1 as libc::c_int) / 7 as libc::c_int) as libc::c_ulong)
                                        < alloc) as libc::c_int
                                }
                            }
                        }) != 0
                        {
                            alloc = (alloc as libc::c_uint)
                                .wrapping_mul(7 as libc::c_int as libc::c_uint)
                                as libc::c_uchar as size_t;
                            1 as libc::c_int
                        } else {
                            alloc = (alloc as libc::c_uint)
                                .wrapping_mul(7 as libc::c_int as libc::c_uint)
                                as libc::c_uchar as size_t;
                            0 as libc::c_int
                        }
                    }
                } else {
                    if ::std::mem::size_of::<size_t>() as libc::c_ulong
                        == ::std::mem::size_of::<libc::c_short>() as libc::c_ulong
                    {
                        if !((0 as libc::c_int as size_t)
                            < -(1 as libc::c_int) as size_t)
                        {
                            if (if (7 as libc::c_int) < 0 as libc::c_int {
                                if alloc < 0 as libc::c_int as libc::c_ulong {
                                    if ((if 1 as libc::c_int != 0 {
                                        0 as libc::c_int
                                    } else {
                                        (if 1 as libc::c_int != 0 {
                                            0 as libc::c_int
                                        } else {
                                            32767 as libc::c_int
                                        }) + 7 as libc::c_int
                                    }) - 1 as libc::c_int) < 0 as libc::c_int
                                    {
                                        (alloc
                                            < (32767 as libc::c_int / 7 as libc::c_int)
                                                as libc::c_ulong) as libc::c_int
                                    } else {
                                        ((if (if (if ((if 1 as libc::c_int != 0 {
                                            0 as libc::c_int
                                        } else {
                                            7 as libc::c_int
                                        }) - 1 as libc::c_int) < 0 as libc::c_int
                                        {
                                            !(((((if 1 as libc::c_int != 0 {
                                                0 as libc::c_int
                                            } else {
                                                7 as libc::c_int
                                            }) + 1 as libc::c_int)
                                                << (::std::mem::size_of::<libc::c_int>() as libc::c_ulong)
                                                    .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                                                    .wrapping_sub(2 as libc::c_int as libc::c_ulong))
                                                - 1 as libc::c_int) * 2 as libc::c_int + 1 as libc::c_int)
                                        } else {
                                            (if 1 as libc::c_int != 0 {
                                                0 as libc::c_int
                                            } else {
                                                7 as libc::c_int
                                            }) + 0 as libc::c_int
                                        }) < 0 as libc::c_int
                                        {
                                            ((7 as libc::c_int)
                                                < -(if ((if 1 as libc::c_int != 0 {
                                                    0 as libc::c_int
                                                } else {
                                                    7 as libc::c_int
                                                }) - 1 as libc::c_int) < 0 as libc::c_int
                                                {
                                                    ((((if 1 as libc::c_int != 0 {
                                                        0 as libc::c_int
                                                    } else {
                                                        7 as libc::c_int
                                                    }) + 1 as libc::c_int)
                                                        << (::std::mem::size_of::<libc::c_int>() as libc::c_ulong)
                                                            .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                                                            .wrapping_sub(2 as libc::c_int as libc::c_ulong))
                                                        - 1 as libc::c_int) * 2 as libc::c_int + 1 as libc::c_int
                                                } else {
                                                    (if 1 as libc::c_int != 0 {
                                                        0 as libc::c_int
                                                    } else {
                                                        7 as libc::c_int
                                                    }) - 1 as libc::c_int
                                                })) as libc::c_int
                                        } else {
                                            ((0 as libc::c_int) < 7 as libc::c_int) as libc::c_int
                                        }) != 0
                                        {
                                            (if 1 as libc::c_int != 0 {
                                                0 as libc::c_int
                                            } else {
                                                7 as libc::c_int
                                            }) + 32767 as libc::c_int
                                                >> (::std::mem::size_of::<libc::c_int>() as libc::c_ulong)
                                                    .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                                                    .wrapping_sub(1 as libc::c_int as libc::c_ulong)
                                        } else {
                                            32767 as libc::c_int / -(7 as libc::c_int)
                                        }) as libc::c_ulong
                                            <= (-(1 as libc::c_int) as libc::c_ulong)
                                                .wrapping_sub(alloc)) as libc::c_int
                                    }
                                } else {
                                    if (if (if ((if 1 as libc::c_int != 0 {
                                        0 as libc::c_int
                                    } else {
                                        (if 1 as libc::c_int != 0 {
                                            0 as libc::c_int
                                        } else {
                                            7 as libc::c_int
                                        }) + (-(32767 as libc::c_int) - 1 as libc::c_int)
                                    }) - 1 as libc::c_int) < 0 as libc::c_int
                                    {
                                        !(((((if 1 as libc::c_int != 0 {
                                            0 as libc::c_int
                                        } else {
                                            (if 1 as libc::c_int != 0 {
                                                0 as libc::c_int
                                            } else {
                                                7 as libc::c_int
                                            }) + (-(32767 as libc::c_int) - 1 as libc::c_int)
                                        }) + 1 as libc::c_int)
                                            << (::std::mem::size_of::<libc::c_int>() as libc::c_ulong)
                                                .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                                                .wrapping_sub(2 as libc::c_int as libc::c_ulong))
                                            - 1 as libc::c_int) * 2 as libc::c_int + 1 as libc::c_int)
                                    } else {
                                        (if 1 as libc::c_int != 0 {
                                            0 as libc::c_int
                                        } else {
                                            (if 1 as libc::c_int != 0 {
                                                0 as libc::c_int
                                            } else {
                                                7 as libc::c_int
                                            }) + (-(32767 as libc::c_int) - 1 as libc::c_int)
                                        }) + 0 as libc::c_int
                                    }) < 0 as libc::c_int
                                    {
                                        ((if 1 as libc::c_int != 0 {
                                            0 as libc::c_int
                                        } else {
                                            7 as libc::c_int
                                        }) + (-(32767 as libc::c_int) - 1 as libc::c_int)
                                            < -(if ((if 1 as libc::c_int != 0 {
                                                0 as libc::c_int
                                            } else {
                                                (if 1 as libc::c_int != 0 {
                                                    0 as libc::c_int
                                                } else {
                                                    7 as libc::c_int
                                                }) + (-(32767 as libc::c_int) - 1 as libc::c_int)
                                            }) - 1 as libc::c_int) < 0 as libc::c_int
                                            {
                                                ((((if 1 as libc::c_int != 0 {
                                                    0 as libc::c_int
                                                } else {
                                                    (if 1 as libc::c_int != 0 {
                                                        0 as libc::c_int
                                                    } else {
                                                        7 as libc::c_int
                                                    }) + (-(32767 as libc::c_int) - 1 as libc::c_int)
                                                }) + 1 as libc::c_int)
                                                    << (::std::mem::size_of::<libc::c_int>() as libc::c_ulong)
                                                        .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                                                        .wrapping_sub(2 as libc::c_int as libc::c_ulong))
                                                    - 1 as libc::c_int) * 2 as libc::c_int + 1 as libc::c_int
                                            } else {
                                                (if 1 as libc::c_int != 0 {
                                                    0 as libc::c_int
                                                } else {
                                                    (if 1 as libc::c_int != 0 {
                                                        0 as libc::c_int
                                                    } else {
                                                        7 as libc::c_int
                                                    }) + (-(32767 as libc::c_int) - 1 as libc::c_int)
                                                }) - 1 as libc::c_int
                                            })) as libc::c_int
                                    } else {
                                        ((0 as libc::c_int)
                                            < (if 1 as libc::c_int != 0 {
                                                0 as libc::c_int
                                            } else {
                                                7 as libc::c_int
                                            }) + (-(32767 as libc::c_int) - 1 as libc::c_int))
                                            as libc::c_int
                                    }) != 0 && 7 as libc::c_int == -(1 as libc::c_int)
                                    {
                                        if (if 1 as libc::c_int != 0 {
                                            0 as libc::c_int as libc::c_ulong
                                        } else {
                                            alloc
                                        })
                                            .wrapping_sub(1 as libc::c_int as libc::c_ulong)
                                            < 0 as libc::c_int as libc::c_ulong
                                        {
                                            ((0 as libc::c_int as libc::c_ulong)
                                                < alloc
                                                    .wrapping_add(
                                                        (-(32767 as libc::c_int) - 1 as libc::c_int)
                                                            as libc::c_ulong,
                                                    )) as libc::c_int
                                        } else {
                                            ((0 as libc::c_int as libc::c_ulong) < alloc
                                                && ((-(1 as libc::c_int)
                                                    - (-(32767 as libc::c_int) - 1 as libc::c_int))
                                                    as libc::c_ulong)
                                                    < alloc.wrapping_sub(1 as libc::c_int as libc::c_ulong))
                                                as libc::c_int
                                        }
                                    } else {
                                        ((((-(32767 as libc::c_int) - 1 as libc::c_int)
                                            / 7 as libc::c_int) as libc::c_ulong) < alloc)
                                            as libc::c_int
                                    }
                                }
                            } else {
                                if 7 as libc::c_int == 0 as libc::c_int {
                                    0 as libc::c_int
                                } else {
                                    if alloc < 0 as libc::c_int as libc::c_ulong {
                                        if (if (if (if 1 as libc::c_int != 0 {
                                            0 as libc::c_int as libc::c_ulong
                                        } else {
                                            (if 1 as libc::c_int != 0 {
                                                0 as libc::c_int as libc::c_ulong
                                            } else {
                                                alloc
                                            })
                                                .wrapping_add(
                                                    (-(32767 as libc::c_int) - 1 as libc::c_int)
                                                        as libc::c_ulong,
                                                )
                                        })
                                            .wrapping_sub(1 as libc::c_int as libc::c_ulong)
                                            < 0 as libc::c_int as libc::c_ulong
                                        {
                                            !((if 1 as libc::c_int != 0 {
                                                0 as libc::c_int as libc::c_ulong
                                            } else {
                                                (if 1 as libc::c_int != 0 {
                                                    0 as libc::c_int as libc::c_ulong
                                                } else {
                                                    alloc
                                                })
                                                    .wrapping_add(
                                                        (-(32767 as libc::c_int) - 1 as libc::c_int)
                                                            as libc::c_ulong,
                                                    )
                                            })
                                                .wrapping_add(1 as libc::c_int as libc::c_ulong)
                                                << (::std::mem::size_of::<libc::c_ulong>() as libc::c_ulong)
                                                    .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                                                    .wrapping_sub(2 as libc::c_int as libc::c_ulong))
                                                .wrapping_sub(1 as libc::c_int as libc::c_ulong)
                                                .wrapping_mul(2 as libc::c_int as libc::c_ulong)
                                                .wrapping_add(1 as libc::c_int as libc::c_ulong)
                                        } else {
                                            (if 1 as libc::c_int != 0 {
                                                0 as libc::c_int as libc::c_ulong
                                            } else {
                                                (if 1 as libc::c_int != 0 {
                                                    0 as libc::c_int as libc::c_ulong
                                                } else {
                                                    alloc
                                                })
                                                    .wrapping_add(
                                                        (-(32767 as libc::c_int) - 1 as libc::c_int)
                                                            as libc::c_ulong,
                                                    )
                                            })
                                                .wrapping_add(0 as libc::c_int as libc::c_ulong)
                                        }) < 0 as libc::c_int as libc::c_ulong
                                        {
                                            ((if 1 as libc::c_int != 0 {
                                                0 as libc::c_int as libc::c_ulong
                                            } else {
                                                alloc
                                            })
                                                .wrapping_add(
                                                    (-(32767 as libc::c_int) - 1 as libc::c_int)
                                                        as libc::c_ulong,
                                                )
                                                < (if (if 1 as libc::c_int != 0 {
                                                    0 as libc::c_int as libc::c_ulong
                                                } else {
                                                    (if 1 as libc::c_int != 0 {
                                                        0 as libc::c_int as libc::c_ulong
                                                    } else {
                                                        alloc
                                                    })
                                                        .wrapping_add(
                                                            (-(32767 as libc::c_int) - 1 as libc::c_int)
                                                                as libc::c_ulong,
                                                        )
                                                })
                                                    .wrapping_sub(1 as libc::c_int as libc::c_ulong)
                                                    < 0 as libc::c_int as libc::c_ulong
                                                {
                                                    ((if 1 as libc::c_int != 0 {
                                                        0 as libc::c_int as libc::c_ulong
                                                    } else {
                                                        (if 1 as libc::c_int != 0 {
                                                            0 as libc::c_int as libc::c_ulong
                                                        } else {
                                                            alloc
                                                        })
                                                            .wrapping_add(
                                                                (-(32767 as libc::c_int) - 1 as libc::c_int)
                                                                    as libc::c_ulong,
                                                            )
                                                    })
                                                        .wrapping_add(1 as libc::c_int as libc::c_ulong)
                                                        << (::std::mem::size_of::<libc::c_ulong>() as libc::c_ulong)
                                                            .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                                                            .wrapping_sub(2 as libc::c_int as libc::c_ulong))
                                                        .wrapping_sub(1 as libc::c_int as libc::c_ulong)
                                                        .wrapping_mul(2 as libc::c_int as libc::c_ulong)
                                                        .wrapping_add(1 as libc::c_int as libc::c_ulong)
                                                } else {
                                                    (if 1 as libc::c_int != 0 {
                                                        0 as libc::c_int as libc::c_ulong
                                                    } else {
                                                        (if 1 as libc::c_int != 0 {
                                                            0 as libc::c_int as libc::c_ulong
                                                        } else {
                                                            alloc
                                                        })
                                                            .wrapping_add(
                                                                (-(32767 as libc::c_int) - 1 as libc::c_int)
                                                                    as libc::c_ulong,
                                                            )
                                                    })
                                                        .wrapping_sub(1 as libc::c_int as libc::c_ulong)
                                                })
                                                    .wrapping_neg()) as libc::c_int
                                        } else {
                                            ((0 as libc::c_int as libc::c_ulong)
                                                < (if 1 as libc::c_int != 0 {
                                                    0 as libc::c_int as libc::c_ulong
                                                } else {
                                                    alloc
                                                })
                                                    .wrapping_add(
                                                        (-(32767 as libc::c_int) - 1 as libc::c_int)
                                                            as libc::c_ulong,
                                                    )) as libc::c_int
                                        }) != 0 && alloc == -(1 as libc::c_int) as libc::c_ulong
                                        {
                                            if ((if 1 as libc::c_int != 0 {
                                                0 as libc::c_int
                                            } else {
                                                7 as libc::c_int
                                            }) - 1 as libc::c_int) < 0 as libc::c_int
                                            {
                                                ((0 as libc::c_int)
                                                    < 7 as libc::c_int
                                                        + (-(32767 as libc::c_int) - 1 as libc::c_int))
                                                    as libc::c_int
                                            } else {
                                                (-(1 as libc::c_int)
                                                    - (-(32767 as libc::c_int) - 1 as libc::c_int)
                                                    < 7 as libc::c_int - 1 as libc::c_int) as libc::c_int
                                            }
                                        } else {
                                            (((-(32767 as libc::c_int) - 1 as libc::c_int)
                                                as libc::c_ulong)
                                                .wrapping_div(alloc) < 7 as libc::c_int as libc::c_ulong)
                                                as libc::c_int
                                        }
                                    } else {
                                        (((32767 as libc::c_int / 7 as libc::c_int)
                                            as libc::c_ulong) < alloc) as libc::c_int
                                    }
                                }
                            }) != 0
                            {
                                alloc = (alloc as libc::c_uint)
                                    .wrapping_mul(7 as libc::c_int as libc::c_uint)
                                    as libc::c_short as size_t;
                                1 as libc::c_int
                            } else {
                                alloc = (alloc as libc::c_uint)
                                    .wrapping_mul(7 as libc::c_int as libc::c_uint)
                                    as libc::c_short as size_t;
                                0 as libc::c_int
                            }
                        } else {
                            if (if (7 as libc::c_int) < 0 as libc::c_int {
                                if alloc < 0 as libc::c_int as libc::c_ulong {
                                    if ((if 1 as libc::c_int != 0 {
                                        0 as libc::c_int
                                    } else {
                                        (if 1 as libc::c_int != 0 {
                                            0 as libc::c_int
                                        } else {
                                            32767 as libc::c_int * 2 as libc::c_int + 1 as libc::c_int
                                        }) + 7 as libc::c_int
                                    }) - 1 as libc::c_int) < 0 as libc::c_int
                                    {
                                        (alloc
                                            < ((32767 as libc::c_int * 2 as libc::c_int
                                                + 1 as libc::c_int) / 7 as libc::c_int) as libc::c_ulong)
                                            as libc::c_int
                                    } else {
                                        ((if (if (if ((if 1 as libc::c_int != 0 {
                                            0 as libc::c_int
                                        } else {
                                            7 as libc::c_int
                                        }) - 1 as libc::c_int) < 0 as libc::c_int
                                        {
                                            !(((((if 1 as libc::c_int != 0 {
                                                0 as libc::c_int
                                            } else {
                                                7 as libc::c_int
                                            }) + 1 as libc::c_int)
                                                << (::std::mem::size_of::<libc::c_int>() as libc::c_ulong)
                                                    .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                                                    .wrapping_sub(2 as libc::c_int as libc::c_ulong))
                                                - 1 as libc::c_int) * 2 as libc::c_int + 1 as libc::c_int)
                                        } else {
                                            (if 1 as libc::c_int != 0 {
                                                0 as libc::c_int
                                            } else {
                                                7 as libc::c_int
                                            }) + 0 as libc::c_int
                                        }) < 0 as libc::c_int
                                        {
                                            ((7 as libc::c_int)
                                                < -(if ((if 1 as libc::c_int != 0 {
                                                    0 as libc::c_int
                                                } else {
                                                    7 as libc::c_int
                                                }) - 1 as libc::c_int) < 0 as libc::c_int
                                                {
                                                    ((((if 1 as libc::c_int != 0 {
                                                        0 as libc::c_int
                                                    } else {
                                                        7 as libc::c_int
                                                    }) + 1 as libc::c_int)
                                                        << (::std::mem::size_of::<libc::c_int>() as libc::c_ulong)
                                                            .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                                                            .wrapping_sub(2 as libc::c_int as libc::c_ulong))
                                                        - 1 as libc::c_int) * 2 as libc::c_int + 1 as libc::c_int
                                                } else {
                                                    (if 1 as libc::c_int != 0 {
                                                        0 as libc::c_int
                                                    } else {
                                                        7 as libc::c_int
                                                    }) - 1 as libc::c_int
                                                })) as libc::c_int
                                        } else {
                                            ((0 as libc::c_int) < 7 as libc::c_int) as libc::c_int
                                        }) != 0
                                        {
                                            (if 1 as libc::c_int != 0 {
                                                0 as libc::c_int
                                            } else {
                                                7 as libc::c_int
                                            })
                                                + (32767 as libc::c_int * 2 as libc::c_int
                                                    + 1 as libc::c_int)
                                                >> (::std::mem::size_of::<libc::c_int>() as libc::c_ulong)
                                                    .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                                                    .wrapping_sub(1 as libc::c_int as libc::c_ulong)
                                        } else {
                                            (32767 as libc::c_int * 2 as libc::c_int + 1 as libc::c_int)
                                                / -(7 as libc::c_int)
                                        }) as libc::c_ulong
                                            <= (-(1 as libc::c_int) as libc::c_ulong)
                                                .wrapping_sub(alloc)) as libc::c_int
                                    }
                                } else {
                                    if (if (if ((if 1 as libc::c_int != 0 {
                                        0 as libc::c_int
                                    } else {
                                        (if 1 as libc::c_int != 0 {
                                            0 as libc::c_int
                                        } else {
                                            7 as libc::c_int
                                        }) + 0 as libc::c_int
                                    }) - 1 as libc::c_int) < 0 as libc::c_int
                                    {
                                        !(((((if 1 as libc::c_int != 0 {
                                            0 as libc::c_int
                                        } else {
                                            (if 1 as libc::c_int != 0 {
                                                0 as libc::c_int
                                            } else {
                                                7 as libc::c_int
                                            }) + 0 as libc::c_int
                                        }) + 1 as libc::c_int)
                                            << (::std::mem::size_of::<libc::c_int>() as libc::c_ulong)
                                                .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                                                .wrapping_sub(2 as libc::c_int as libc::c_ulong))
                                            - 1 as libc::c_int) * 2 as libc::c_int + 1 as libc::c_int)
                                    } else {
                                        (if 1 as libc::c_int != 0 {
                                            0 as libc::c_int
                                        } else {
                                            (if 1 as libc::c_int != 0 {
                                                0 as libc::c_int
                                            } else {
                                                7 as libc::c_int
                                            }) + 0 as libc::c_int
                                        }) + 0 as libc::c_int
                                    }) < 0 as libc::c_int
                                    {
                                        (((if 1 as libc::c_int != 0 {
                                            0 as libc::c_int
                                        } else {
                                            7 as libc::c_int
                                        }) + 0 as libc::c_int)
                                            < -(if ((if 1 as libc::c_int != 0 {
                                                0 as libc::c_int
                                            } else {
                                                (if 1 as libc::c_int != 0 {
                                                    0 as libc::c_int
                                                } else {
                                                    7 as libc::c_int
                                                }) + 0 as libc::c_int
                                            }) - 1 as libc::c_int) < 0 as libc::c_int
                                            {
                                                ((((if 1 as libc::c_int != 0 {
                                                    0 as libc::c_int
                                                } else {
                                                    (if 1 as libc::c_int != 0 {
                                                        0 as libc::c_int
                                                    } else {
                                                        7 as libc::c_int
                                                    }) + 0 as libc::c_int
                                                }) + 1 as libc::c_int)
                                                    << (::std::mem::size_of::<libc::c_int>() as libc::c_ulong)
                                                        .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                                                        .wrapping_sub(2 as libc::c_int as libc::c_ulong))
                                                    - 1 as libc::c_int) * 2 as libc::c_int + 1 as libc::c_int
                                            } else {
                                                (if 1 as libc::c_int != 0 {
                                                    0 as libc::c_int
                                                } else {
                                                    (if 1 as libc::c_int != 0 {
                                                        0 as libc::c_int
                                                    } else {
                                                        7 as libc::c_int
                                                    }) + 0 as libc::c_int
                                                }) - 1 as libc::c_int
                                            })) as libc::c_int
                                    } else {
                                        ((0 as libc::c_int)
                                            < (if 1 as libc::c_int != 0 {
                                                0 as libc::c_int
                                            } else {
                                                7 as libc::c_int
                                            }) + 0 as libc::c_int) as libc::c_int
                                    }) != 0 && 7 as libc::c_int == -(1 as libc::c_int)
                                    {
                                        if (if 1 as libc::c_int != 0 {
                                            0 as libc::c_int as libc::c_ulong
                                        } else {
                                            alloc
                                        })
                                            .wrapping_sub(1 as libc::c_int as libc::c_ulong)
                                            < 0 as libc::c_int as libc::c_ulong
                                        {
                                            ((0 as libc::c_int as libc::c_ulong)
                                                < alloc.wrapping_add(0 as libc::c_int as libc::c_ulong))
                                                as libc::c_int
                                        } else {
                                            ((0 as libc::c_int as libc::c_ulong) < alloc
                                                && ((-(1 as libc::c_int) - 0 as libc::c_int)
                                                    as libc::c_ulong)
                                                    < alloc.wrapping_sub(1 as libc::c_int as libc::c_ulong))
                                                as libc::c_int
                                        }
                                    } else {
                                        (((0 as libc::c_int / 7 as libc::c_int) as libc::c_ulong)
                                            < alloc) as libc::c_int
                                    }
                                }
                            } else {
                                if 7 as libc::c_int == 0 as libc::c_int {
                                    0 as libc::c_int
                                } else {
                                    if alloc < 0 as libc::c_int as libc::c_ulong {
                                        if (if (if (if 1 as libc::c_int != 0 {
                                            0 as libc::c_int as libc::c_ulong
                                        } else {
                                            (if 1 as libc::c_int != 0 {
                                                0 as libc::c_int as libc::c_ulong
                                            } else {
                                                alloc
                                            })
                                                .wrapping_add(0 as libc::c_int as libc::c_ulong)
                                        })
                                            .wrapping_sub(1 as libc::c_int as libc::c_ulong)
                                            < 0 as libc::c_int as libc::c_ulong
                                        {
                                            !((if 1 as libc::c_int != 0 {
                                                0 as libc::c_int as libc::c_ulong
                                            } else {
                                                (if 1 as libc::c_int != 0 {
                                                    0 as libc::c_int as libc::c_ulong
                                                } else {
                                                    alloc
                                                })
                                                    .wrapping_add(0 as libc::c_int as libc::c_ulong)
                                            })
                                                .wrapping_add(1 as libc::c_int as libc::c_ulong)
                                                << (::std::mem::size_of::<libc::c_ulong>() as libc::c_ulong)
                                                    .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                                                    .wrapping_sub(2 as libc::c_int as libc::c_ulong))
                                                .wrapping_sub(1 as libc::c_int as libc::c_ulong)
                                                .wrapping_mul(2 as libc::c_int as libc::c_ulong)
                                                .wrapping_add(1 as libc::c_int as libc::c_ulong)
                                        } else {
                                            (if 1 as libc::c_int != 0 {
                                                0 as libc::c_int as libc::c_ulong
                                            } else {
                                                (if 1 as libc::c_int != 0 {
                                                    0 as libc::c_int as libc::c_ulong
                                                } else {
                                                    alloc
                                                })
                                                    .wrapping_add(0 as libc::c_int as libc::c_ulong)
                                            })
                                                .wrapping_add(0 as libc::c_int as libc::c_ulong)
                                        }) < 0 as libc::c_int as libc::c_ulong
                                        {
                                            ((if 1 as libc::c_int != 0 {
                                                0 as libc::c_int as libc::c_ulong
                                            } else {
                                                alloc
                                            })
                                                .wrapping_add(0 as libc::c_int as libc::c_ulong)
                                                < (if (if 1 as libc::c_int != 0 {
                                                    0 as libc::c_int as libc::c_ulong
                                                } else {
                                                    (if 1 as libc::c_int != 0 {
                                                        0 as libc::c_int as libc::c_ulong
                                                    } else {
                                                        alloc
                                                    })
                                                        .wrapping_add(0 as libc::c_int as libc::c_ulong)
                                                })
                                                    .wrapping_sub(1 as libc::c_int as libc::c_ulong)
                                                    < 0 as libc::c_int as libc::c_ulong
                                                {
                                                    ((if 1 as libc::c_int != 0 {
                                                        0 as libc::c_int as libc::c_ulong
                                                    } else {
                                                        (if 1 as libc::c_int != 0 {
                                                            0 as libc::c_int as libc::c_ulong
                                                        } else {
                                                            alloc
                                                        })
                                                            .wrapping_add(0 as libc::c_int as libc::c_ulong)
                                                    })
                                                        .wrapping_add(1 as libc::c_int as libc::c_ulong)
                                                        << (::std::mem::size_of::<libc::c_ulong>() as libc::c_ulong)
                                                            .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                                                            .wrapping_sub(2 as libc::c_int as libc::c_ulong))
                                                        .wrapping_sub(1 as libc::c_int as libc::c_ulong)
                                                        .wrapping_mul(2 as libc::c_int as libc::c_ulong)
                                                        .wrapping_add(1 as libc::c_int as libc::c_ulong)
                                                } else {
                                                    (if 1 as libc::c_int != 0 {
                                                        0 as libc::c_int as libc::c_ulong
                                                    } else {
                                                        (if 1 as libc::c_int != 0 {
                                                            0 as libc::c_int as libc::c_ulong
                                                        } else {
                                                            alloc
                                                        })
                                                            .wrapping_add(0 as libc::c_int as libc::c_ulong)
                                                    })
                                                        .wrapping_sub(1 as libc::c_int as libc::c_ulong)
                                                })
                                                    .wrapping_neg()) as libc::c_int
                                        } else {
                                            ((0 as libc::c_int as libc::c_ulong)
                                                < (if 1 as libc::c_int != 0 {
                                                    0 as libc::c_int as libc::c_ulong
                                                } else {
                                                    alloc
                                                })
                                                    .wrapping_add(0 as libc::c_int as libc::c_ulong))
                                                as libc::c_int
                                        }) != 0 && alloc == -(1 as libc::c_int) as libc::c_ulong
                                        {
                                            if ((if 1 as libc::c_int != 0 {
                                                0 as libc::c_int
                                            } else {
                                                7 as libc::c_int
                                            }) - 1 as libc::c_int) < 0 as libc::c_int
                                            {
                                                ((0 as libc::c_int) < 7 as libc::c_int + 0 as libc::c_int)
                                                    as libc::c_int
                                            } else {
                                                ((-(1 as libc::c_int) - 0 as libc::c_int)
                                                    < 7 as libc::c_int - 1 as libc::c_int) as libc::c_int
                                            }
                                        } else {
                                            ((0 as libc::c_int as libc::c_ulong).wrapping_div(alloc)
                                                < 7 as libc::c_int as libc::c_ulong) as libc::c_int
                                        }
                                    } else {
                                        ((((32767 as libc::c_int * 2 as libc::c_int
                                            + 1 as libc::c_int) / 7 as libc::c_int) as libc::c_ulong)
                                            < alloc) as libc::c_int
                                    }
                                }
                            }) != 0
                            {
                                alloc = (alloc as libc::c_uint)
                                    .wrapping_mul(7 as libc::c_int as libc::c_uint)
                                    as libc::c_ushort as size_t;
                                1 as libc::c_int
                            } else {
                                alloc = (alloc as libc::c_uint)
                                    .wrapping_mul(7 as libc::c_int as libc::c_uint)
                                    as libc::c_ushort as size_t;
                                0 as libc::c_int
                            }
                        }
                    } else {
                        if ::std::mem::size_of::<size_t>() as libc::c_ulong
                            == ::std::mem::size_of::<libc::c_int>() as libc::c_ulong
                        {
                            if (if 1 as libc::c_int != 0 {
                                0 as libc::c_int as libc::c_ulong
                            } else {
                                alloc
                            })
                                .wrapping_sub(1 as libc::c_int as libc::c_ulong)
                                < 0 as libc::c_int as libc::c_ulong
                            {
                                if (if (7 as libc::c_int) < 0 as libc::c_int {
                                    if alloc < 0 as libc::c_int as libc::c_ulong {
                                        if ((if 1 as libc::c_int != 0 {
                                            0 as libc::c_int
                                        } else {
                                            (if 1 as libc::c_int != 0 {
                                                0 as libc::c_int
                                            } else {
                                                2147483647 as libc::c_int
                                            }) + 7 as libc::c_int
                                        }) - 1 as libc::c_int) < 0 as libc::c_int
                                        {
                                            (alloc
                                                < (2147483647 as libc::c_int / 7 as libc::c_int)
                                                    as libc::c_ulong) as libc::c_int
                                        } else {
                                            ((if (if (if ((if 1 as libc::c_int != 0 {
                                                0 as libc::c_int
                                            } else {
                                                7 as libc::c_int
                                            }) - 1 as libc::c_int) < 0 as libc::c_int
                                            {
                                                !(((((if 1 as libc::c_int != 0 {
                                                    0 as libc::c_int
                                                } else {
                                                    7 as libc::c_int
                                                }) + 1 as libc::c_int)
                                                    << (::std::mem::size_of::<libc::c_int>() as libc::c_ulong)
                                                        .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                                                        .wrapping_sub(2 as libc::c_int as libc::c_ulong))
                                                    - 1 as libc::c_int) * 2 as libc::c_int + 1 as libc::c_int)
                                            } else {
                                                (if 1 as libc::c_int != 0 {
                                                    0 as libc::c_int
                                                } else {
                                                    7 as libc::c_int
                                                }) + 0 as libc::c_int
                                            }) < 0 as libc::c_int
                                            {
                                                ((7 as libc::c_int)
                                                    < -(if ((if 1 as libc::c_int != 0 {
                                                        0 as libc::c_int
                                                    } else {
                                                        7 as libc::c_int
                                                    }) - 1 as libc::c_int) < 0 as libc::c_int
                                                    {
                                                        ((((if 1 as libc::c_int != 0 {
                                                            0 as libc::c_int
                                                        } else {
                                                            7 as libc::c_int
                                                        }) + 1 as libc::c_int)
                                                            << (::std::mem::size_of::<libc::c_int>() as libc::c_ulong)
                                                                .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                                                                .wrapping_sub(2 as libc::c_int as libc::c_ulong))
                                                            - 1 as libc::c_int) * 2 as libc::c_int + 1 as libc::c_int
                                                    } else {
                                                        (if 1 as libc::c_int != 0 {
                                                            0 as libc::c_int
                                                        } else {
                                                            7 as libc::c_int
                                                        }) - 1 as libc::c_int
                                                    })) as libc::c_int
                                            } else {
                                                ((0 as libc::c_int) < 7 as libc::c_int) as libc::c_int
                                            }) != 0
                                            {
                                                (if 1 as libc::c_int != 0 {
                                                    0 as libc::c_int
                                                } else {
                                                    7 as libc::c_int
                                                }) + 2147483647 as libc::c_int
                                                    >> (::std::mem::size_of::<libc::c_int>() as libc::c_ulong)
                                                        .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                                                        .wrapping_sub(1 as libc::c_int as libc::c_ulong)
                                            } else {
                                                2147483647 as libc::c_int / -(7 as libc::c_int)
                                            }) as libc::c_ulong
                                                <= (-(1 as libc::c_int) as libc::c_ulong)
                                                    .wrapping_sub(alloc)) as libc::c_int
                                        }
                                    } else {
                                        if (if (if ((if 1 as libc::c_int != 0 {
                                            0 as libc::c_int
                                        } else {
                                            (if 1 as libc::c_int != 0 {
                                                0 as libc::c_int
                                            } else {
                                                7 as libc::c_int
                                            }) + (-(2147483647 as libc::c_int) - 1 as libc::c_int)
                                        }) - 1 as libc::c_int) < 0 as libc::c_int
                                        {
                                            !(((((if 1 as libc::c_int != 0 {
                                                0 as libc::c_int
                                            } else {
                                                (if 1 as libc::c_int != 0 {
                                                    0 as libc::c_int
                                                } else {
                                                    7 as libc::c_int
                                                }) + (-(2147483647 as libc::c_int) - 1 as libc::c_int)
                                            }) + 1 as libc::c_int)
                                                << (::std::mem::size_of::<libc::c_int>() as libc::c_ulong)
                                                    .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                                                    .wrapping_sub(2 as libc::c_int as libc::c_ulong))
                                                - 1 as libc::c_int) * 2 as libc::c_int + 1 as libc::c_int)
                                        } else {
                                            (if 1 as libc::c_int != 0 {
                                                0 as libc::c_int
                                            } else {
                                                (if 1 as libc::c_int != 0 {
                                                    0 as libc::c_int
                                                } else {
                                                    7 as libc::c_int
                                                }) + (-(2147483647 as libc::c_int) - 1 as libc::c_int)
                                            }) + 0 as libc::c_int
                                        }) < 0 as libc::c_int
                                        {
                                            ((if 1 as libc::c_int != 0 {
                                                0 as libc::c_int
                                            } else {
                                                7 as libc::c_int
                                            }) + (-(2147483647 as libc::c_int) - 1 as libc::c_int)
                                                < -(if ((if 1 as libc::c_int != 0 {
                                                    0 as libc::c_int
                                                } else {
                                                    (if 1 as libc::c_int != 0 {
                                                        0 as libc::c_int
                                                    } else {
                                                        7 as libc::c_int
                                                    }) + (-(2147483647 as libc::c_int) - 1 as libc::c_int)
                                                }) - 1 as libc::c_int) < 0 as libc::c_int
                                                {
                                                    ((((if 1 as libc::c_int != 0 {
                                                        0 as libc::c_int
                                                    } else {
                                                        (if 1 as libc::c_int != 0 {
                                                            0 as libc::c_int
                                                        } else {
                                                            7 as libc::c_int
                                                        }) + (-(2147483647 as libc::c_int) - 1 as libc::c_int)
                                                    }) + 1 as libc::c_int)
                                                        << (::std::mem::size_of::<libc::c_int>() as libc::c_ulong)
                                                            .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                                                            .wrapping_sub(2 as libc::c_int as libc::c_ulong))
                                                        - 1 as libc::c_int) * 2 as libc::c_int + 1 as libc::c_int
                                                } else {
                                                    (if 1 as libc::c_int != 0 {
                                                        0 as libc::c_int
                                                    } else {
                                                        (if 1 as libc::c_int != 0 {
                                                            0 as libc::c_int
                                                        } else {
                                                            7 as libc::c_int
                                                        }) + (-(2147483647 as libc::c_int) - 1 as libc::c_int)
                                                    }) - 1 as libc::c_int
                                                })) as libc::c_int
                                        } else {
                                            ((0 as libc::c_int)
                                                < (if 1 as libc::c_int != 0 {
                                                    0 as libc::c_int
                                                } else {
                                                    7 as libc::c_int
                                                }) + (-(2147483647 as libc::c_int) - 1 as libc::c_int))
                                                as libc::c_int
                                        }) != 0 && 7 as libc::c_int == -(1 as libc::c_int)
                                        {
                                            if (if 1 as libc::c_int != 0 {
                                                0 as libc::c_int as libc::c_ulong
                                            } else {
                                                alloc
                                            })
                                                .wrapping_sub(1 as libc::c_int as libc::c_ulong)
                                                < 0 as libc::c_int as libc::c_ulong
                                            {
                                                ((0 as libc::c_int as libc::c_ulong)
                                                    < alloc
                                                        .wrapping_add(
                                                            (-(2147483647 as libc::c_int) - 1 as libc::c_int)
                                                                as libc::c_ulong,
                                                        )) as libc::c_int
                                            } else {
                                                ((0 as libc::c_int as libc::c_ulong) < alloc
                                                    && ((-(1 as libc::c_int)
                                                        - (-(2147483647 as libc::c_int) - 1 as libc::c_int))
                                                        as libc::c_ulong)
                                                        < alloc.wrapping_sub(1 as libc::c_int as libc::c_ulong))
                                                    as libc::c_int
                                            }
                                        } else {
                                            ((((-(2147483647 as libc::c_int) - 1 as libc::c_int)
                                                / 7 as libc::c_int) as libc::c_ulong) < alloc)
                                                as libc::c_int
                                        }
                                    }
                                } else {
                                    if 7 as libc::c_int == 0 as libc::c_int {
                                        0 as libc::c_int
                                    } else {
                                        if alloc < 0 as libc::c_int as libc::c_ulong {
                                            if (if (if (if 1 as libc::c_int != 0 {
                                                0 as libc::c_int as libc::c_ulong
                                            } else {
                                                (if 1 as libc::c_int != 0 {
                                                    0 as libc::c_int as libc::c_ulong
                                                } else {
                                                    alloc
                                                })
                                                    .wrapping_add(
                                                        (-(2147483647 as libc::c_int) - 1 as libc::c_int)
                                                            as libc::c_ulong,
                                                    )
                                            })
                                                .wrapping_sub(1 as libc::c_int as libc::c_ulong)
                                                < 0 as libc::c_int as libc::c_ulong
                                            {
                                                !((if 1 as libc::c_int != 0 {
                                                    0 as libc::c_int as libc::c_ulong
                                                } else {
                                                    (if 1 as libc::c_int != 0 {
                                                        0 as libc::c_int as libc::c_ulong
                                                    } else {
                                                        alloc
                                                    })
                                                        .wrapping_add(
                                                            (-(2147483647 as libc::c_int) - 1 as libc::c_int)
                                                                as libc::c_ulong,
                                                        )
                                                })
                                                    .wrapping_add(1 as libc::c_int as libc::c_ulong)
                                                    << (::std::mem::size_of::<libc::c_ulong>() as libc::c_ulong)
                                                        .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                                                        .wrapping_sub(2 as libc::c_int as libc::c_ulong))
                                                    .wrapping_sub(1 as libc::c_int as libc::c_ulong)
                                                    .wrapping_mul(2 as libc::c_int as libc::c_ulong)
                                                    .wrapping_add(1 as libc::c_int as libc::c_ulong)
                                            } else {
                                                (if 1 as libc::c_int != 0 {
                                                    0 as libc::c_int as libc::c_ulong
                                                } else {
                                                    (if 1 as libc::c_int != 0 {
                                                        0 as libc::c_int as libc::c_ulong
                                                    } else {
                                                        alloc
                                                    })
                                                        .wrapping_add(
                                                            (-(2147483647 as libc::c_int) - 1 as libc::c_int)
                                                                as libc::c_ulong,
                                                        )
                                                })
                                                    .wrapping_add(0 as libc::c_int as libc::c_ulong)
                                            }) < 0 as libc::c_int as libc::c_ulong
                                            {
                                                ((if 1 as libc::c_int != 0 {
                                                    0 as libc::c_int as libc::c_ulong
                                                } else {
                                                    alloc
                                                })
                                                    .wrapping_add(
                                                        (-(2147483647 as libc::c_int) - 1 as libc::c_int)
                                                            as libc::c_ulong,
                                                    )
                                                    < (if (if 1 as libc::c_int != 0 {
                                                        0 as libc::c_int as libc::c_ulong
                                                    } else {
                                                        (if 1 as libc::c_int != 0 {
                                                            0 as libc::c_int as libc::c_ulong
                                                        } else {
                                                            alloc
                                                        })
                                                            .wrapping_add(
                                                                (-(2147483647 as libc::c_int) - 1 as libc::c_int)
                                                                    as libc::c_ulong,
                                                            )
                                                    })
                                                        .wrapping_sub(1 as libc::c_int as libc::c_ulong)
                                                        < 0 as libc::c_int as libc::c_ulong
                                                    {
                                                        ((if 1 as libc::c_int != 0 {
                                                            0 as libc::c_int as libc::c_ulong
                                                        } else {
                                                            (if 1 as libc::c_int != 0 {
                                                                0 as libc::c_int as libc::c_ulong
                                                            } else {
                                                                alloc
                                                            })
                                                                .wrapping_add(
                                                                    (-(2147483647 as libc::c_int) - 1 as libc::c_int)
                                                                        as libc::c_ulong,
                                                                )
                                                        })
                                                            .wrapping_add(1 as libc::c_int as libc::c_ulong)
                                                            << (::std::mem::size_of::<libc::c_ulong>() as libc::c_ulong)
                                                                .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                                                                .wrapping_sub(2 as libc::c_int as libc::c_ulong))
                                                            .wrapping_sub(1 as libc::c_int as libc::c_ulong)
                                                            .wrapping_mul(2 as libc::c_int as libc::c_ulong)
                                                            .wrapping_add(1 as libc::c_int as libc::c_ulong)
                                                    } else {
                                                        (if 1 as libc::c_int != 0 {
                                                            0 as libc::c_int as libc::c_ulong
                                                        } else {
                                                            (if 1 as libc::c_int != 0 {
                                                                0 as libc::c_int as libc::c_ulong
                                                            } else {
                                                                alloc
                                                            })
                                                                .wrapping_add(
                                                                    (-(2147483647 as libc::c_int) - 1 as libc::c_int)
                                                                        as libc::c_ulong,
                                                                )
                                                        })
                                                            .wrapping_sub(1 as libc::c_int as libc::c_ulong)
                                                    })
                                                        .wrapping_neg()) as libc::c_int
                                            } else {
                                                ((0 as libc::c_int as libc::c_ulong)
                                                    < (if 1 as libc::c_int != 0 {
                                                        0 as libc::c_int as libc::c_ulong
                                                    } else {
                                                        alloc
                                                    })
                                                        .wrapping_add(
                                                            (-(2147483647 as libc::c_int) - 1 as libc::c_int)
                                                                as libc::c_ulong,
                                                        )) as libc::c_int
                                            }) != 0 && alloc == -(1 as libc::c_int) as libc::c_ulong
                                            {
                                                if ((if 1 as libc::c_int != 0 {
                                                    0 as libc::c_int
                                                } else {
                                                    7 as libc::c_int
                                                }) - 1 as libc::c_int) < 0 as libc::c_int
                                                {
                                                    ((0 as libc::c_int)
                                                        < 7 as libc::c_int
                                                            + (-(2147483647 as libc::c_int) - 1 as libc::c_int))
                                                        as libc::c_int
                                                } else {
                                                    (-(1 as libc::c_int)
                                                        - (-(2147483647 as libc::c_int) - 1 as libc::c_int)
                                                        < 7 as libc::c_int - 1 as libc::c_int) as libc::c_int
                                                }
                                            } else {
                                                (((-(2147483647 as libc::c_int) - 1 as libc::c_int)
                                                    as libc::c_ulong)
                                                    .wrapping_div(alloc) < 7 as libc::c_int as libc::c_ulong)
                                                    as libc::c_int
                                            }
                                        } else {
                                            (((2147483647 as libc::c_int / 7 as libc::c_int)
                                                as libc::c_ulong) < alloc) as libc::c_int
                                        }
                                    }
                                }) != 0
                                {
                                    alloc = (alloc as libc::c_uint)
                                        .wrapping_mul(7 as libc::c_int as libc::c_uint)
                                        as libc::c_int as size_t;
                                    1 as libc::c_int
                                } else {
                                    alloc = (alloc as libc::c_uint)
                                        .wrapping_mul(7 as libc::c_int as libc::c_uint)
                                        as libc::c_int as size_t;
                                    0 as libc::c_int
                                }
                            } else {
                                if (if (7 as libc::c_int) < 0 as libc::c_int {
                                    if alloc < 0 as libc::c_int as libc::c_ulong {
                                        if (if 1 as libc::c_int != 0 {
                                            0 as libc::c_int as libc::c_uint
                                        } else {
                                            (if 1 as libc::c_int != 0 {
                                                0 as libc::c_int as libc::c_uint
                                            } else {
                                                (2147483647 as libc::c_int as libc::c_uint)
                                                    .wrapping_mul(2 as libc::c_uint)
                                                    .wrapping_add(1 as libc::c_uint)
                                            })
                                                .wrapping_add(7 as libc::c_int as libc::c_uint)
                                        })
                                            .wrapping_sub(1 as libc::c_int as libc::c_uint)
                                            < 0 as libc::c_int as libc::c_uint
                                        {
                                            (alloc
                                                < (2147483647 as libc::c_int as libc::c_uint)
                                                    .wrapping_mul(2 as libc::c_uint)
                                                    .wrapping_add(1 as libc::c_uint)
                                                    .wrapping_div(7 as libc::c_int as libc::c_uint)
                                                    as libc::c_ulong) as libc::c_int
                                        } else {
                                            ((if (if (if ((if 1 as libc::c_int != 0 {
                                                0 as libc::c_int
                                            } else {
                                                7 as libc::c_int
                                            }) - 1 as libc::c_int) < 0 as libc::c_int
                                            {
                                                !(((((if 1 as libc::c_int != 0 {
                                                    0 as libc::c_int
                                                } else {
                                                    7 as libc::c_int
                                                }) + 1 as libc::c_int)
                                                    << (::std::mem::size_of::<libc::c_int>() as libc::c_ulong)
                                                        .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                                                        .wrapping_sub(2 as libc::c_int as libc::c_ulong))
                                                    - 1 as libc::c_int) * 2 as libc::c_int + 1 as libc::c_int)
                                            } else {
                                                (if 1 as libc::c_int != 0 {
                                                    0 as libc::c_int
                                                } else {
                                                    7 as libc::c_int
                                                }) + 0 as libc::c_int
                                            }) < 0 as libc::c_int
                                            {
                                                ((7 as libc::c_int)
                                                    < -(if ((if 1 as libc::c_int != 0 {
                                                        0 as libc::c_int
                                                    } else {
                                                        7 as libc::c_int
                                                    }) - 1 as libc::c_int) < 0 as libc::c_int
                                                    {
                                                        ((((if 1 as libc::c_int != 0 {
                                                            0 as libc::c_int
                                                        } else {
                                                            7 as libc::c_int
                                                        }) + 1 as libc::c_int)
                                                            << (::std::mem::size_of::<libc::c_int>() as libc::c_ulong)
                                                                .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                                                                .wrapping_sub(2 as libc::c_int as libc::c_ulong))
                                                            - 1 as libc::c_int) * 2 as libc::c_int + 1 as libc::c_int
                                                    } else {
                                                        (if 1 as libc::c_int != 0 {
                                                            0 as libc::c_int
                                                        } else {
                                                            7 as libc::c_int
                                                        }) - 1 as libc::c_int
                                                    })) as libc::c_int
                                            } else {
                                                ((0 as libc::c_int) < 7 as libc::c_int) as libc::c_int
                                            }) != 0
                                            {
                                                ((if 1 as libc::c_int != 0 {
                                                    0 as libc::c_int
                                                } else {
                                                    7 as libc::c_int
                                                }) as libc::c_uint)
                                                    .wrapping_add(
                                                        (2147483647 as libc::c_int as libc::c_uint)
                                                            .wrapping_mul(2 as libc::c_uint)
                                                            .wrapping_add(1 as libc::c_uint),
                                                    )
                                                    >> (::std::mem::size_of::<libc::c_int>() as libc::c_ulong)
                                                        .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                                                        .wrapping_sub(1 as libc::c_int as libc::c_ulong)
                                            } else {
                                                (2147483647 as libc::c_int as libc::c_uint)
                                                    .wrapping_mul(2 as libc::c_uint)
                                                    .wrapping_add(1 as libc::c_uint)
                                                    .wrapping_div(-(7 as libc::c_int) as libc::c_uint)
                                            }) as libc::c_ulong
                                                <= (-(1 as libc::c_int) as libc::c_ulong)
                                                    .wrapping_sub(alloc)) as libc::c_int
                                        }
                                    } else {
                                        if (if (if ((if 1 as libc::c_int != 0 {
                                            0 as libc::c_int
                                        } else {
                                            (if 1 as libc::c_int != 0 {
                                                0 as libc::c_int
                                            } else {
                                                7 as libc::c_int
                                            }) + 0 as libc::c_int
                                        }) - 1 as libc::c_int) < 0 as libc::c_int
                                        {
                                            !(((((if 1 as libc::c_int != 0 {
                                                0 as libc::c_int
                                            } else {
                                                (if 1 as libc::c_int != 0 {
                                                    0 as libc::c_int
                                                } else {
                                                    7 as libc::c_int
                                                }) + 0 as libc::c_int
                                            }) + 1 as libc::c_int)
                                                << (::std::mem::size_of::<libc::c_int>() as libc::c_ulong)
                                                    .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                                                    .wrapping_sub(2 as libc::c_int as libc::c_ulong))
                                                - 1 as libc::c_int) * 2 as libc::c_int + 1 as libc::c_int)
                                        } else {
                                            (if 1 as libc::c_int != 0 {
                                                0 as libc::c_int
                                            } else {
                                                (if 1 as libc::c_int != 0 {
                                                    0 as libc::c_int
                                                } else {
                                                    7 as libc::c_int
                                                }) + 0 as libc::c_int
                                            }) + 0 as libc::c_int
                                        }) < 0 as libc::c_int
                                        {
                                            (((if 1 as libc::c_int != 0 {
                                                0 as libc::c_int
                                            } else {
                                                7 as libc::c_int
                                            }) + 0 as libc::c_int)
                                                < -(if ((if 1 as libc::c_int != 0 {
                                                    0 as libc::c_int
                                                } else {
                                                    (if 1 as libc::c_int != 0 {
                                                        0 as libc::c_int
                                                    } else {
                                                        7 as libc::c_int
                                                    }) + 0 as libc::c_int
                                                }) - 1 as libc::c_int) < 0 as libc::c_int
                                                {
                                                    ((((if 1 as libc::c_int != 0 {
                                                        0 as libc::c_int
                                                    } else {
                                                        (if 1 as libc::c_int != 0 {
                                                            0 as libc::c_int
                                                        } else {
                                                            7 as libc::c_int
                                                        }) + 0 as libc::c_int
                                                    }) + 1 as libc::c_int)
                                                        << (::std::mem::size_of::<libc::c_int>() as libc::c_ulong)
                                                            .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                                                            .wrapping_sub(2 as libc::c_int as libc::c_ulong))
                                                        - 1 as libc::c_int) * 2 as libc::c_int + 1 as libc::c_int
                                                } else {
                                                    (if 1 as libc::c_int != 0 {
                                                        0 as libc::c_int
                                                    } else {
                                                        (if 1 as libc::c_int != 0 {
                                                            0 as libc::c_int
                                                        } else {
                                                            7 as libc::c_int
                                                        }) + 0 as libc::c_int
                                                    }) - 1 as libc::c_int
                                                })) as libc::c_int
                                        } else {
                                            ((0 as libc::c_int)
                                                < (if 1 as libc::c_int != 0 {
                                                    0 as libc::c_int
                                                } else {
                                                    7 as libc::c_int
                                                }) + 0 as libc::c_int) as libc::c_int
                                        }) != 0 && 7 as libc::c_int == -(1 as libc::c_int)
                                        {
                                            if (if 1 as libc::c_int != 0 {
                                                0 as libc::c_int as libc::c_ulong
                                            } else {
                                                alloc
                                            })
                                                .wrapping_sub(1 as libc::c_int as libc::c_ulong)
                                                < 0 as libc::c_int as libc::c_ulong
                                            {
                                                ((0 as libc::c_int as libc::c_ulong)
                                                    < alloc.wrapping_add(0 as libc::c_int as libc::c_ulong))
                                                    as libc::c_int
                                            } else {
                                                ((0 as libc::c_int as libc::c_ulong) < alloc
                                                    && ((-(1 as libc::c_int) - 0 as libc::c_int)
                                                        as libc::c_ulong)
                                                        < alloc.wrapping_sub(1 as libc::c_int as libc::c_ulong))
                                                    as libc::c_int
                                            }
                                        } else {
                                            (((0 as libc::c_int / 7 as libc::c_int) as libc::c_ulong)
                                                < alloc) as libc::c_int
                                        }
                                    }
                                } else {
                                    if 7 as libc::c_int == 0 as libc::c_int {
                                        0 as libc::c_int
                                    } else {
                                        if alloc < 0 as libc::c_int as libc::c_ulong {
                                            if (if (if (if 1 as libc::c_int != 0 {
                                                0 as libc::c_int as libc::c_ulong
                                            } else {
                                                (if 1 as libc::c_int != 0 {
                                                    0 as libc::c_int as libc::c_ulong
                                                } else {
                                                    alloc
                                                })
                                                    .wrapping_add(0 as libc::c_int as libc::c_ulong)
                                            })
                                                .wrapping_sub(1 as libc::c_int as libc::c_ulong)
                                                < 0 as libc::c_int as libc::c_ulong
                                            {
                                                !((if 1 as libc::c_int != 0 {
                                                    0 as libc::c_int as libc::c_ulong
                                                } else {
                                                    (if 1 as libc::c_int != 0 {
                                                        0 as libc::c_int as libc::c_ulong
                                                    } else {
                                                        alloc
                                                    })
                                                        .wrapping_add(0 as libc::c_int as libc::c_ulong)
                                                })
                                                    .wrapping_add(1 as libc::c_int as libc::c_ulong)
                                                    << (::std::mem::size_of::<libc::c_ulong>() as libc::c_ulong)
                                                        .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                                                        .wrapping_sub(2 as libc::c_int as libc::c_ulong))
                                                    .wrapping_sub(1 as libc::c_int as libc::c_ulong)
                                                    .wrapping_mul(2 as libc::c_int as libc::c_ulong)
                                                    .wrapping_add(1 as libc::c_int as libc::c_ulong)
                                            } else {
                                                (if 1 as libc::c_int != 0 {
                                                    0 as libc::c_int as libc::c_ulong
                                                } else {
                                                    (if 1 as libc::c_int != 0 {
                                                        0 as libc::c_int as libc::c_ulong
                                                    } else {
                                                        alloc
                                                    })
                                                        .wrapping_add(0 as libc::c_int as libc::c_ulong)
                                                })
                                                    .wrapping_add(0 as libc::c_int as libc::c_ulong)
                                            }) < 0 as libc::c_int as libc::c_ulong
                                            {
                                                ((if 1 as libc::c_int != 0 {
                                                    0 as libc::c_int as libc::c_ulong
                                                } else {
                                                    alloc
                                                })
                                                    .wrapping_add(0 as libc::c_int as libc::c_ulong)
                                                    < (if (if 1 as libc::c_int != 0 {
                                                        0 as libc::c_int as libc::c_ulong
                                                    } else {
                                                        (if 1 as libc::c_int != 0 {
                                                            0 as libc::c_int as libc::c_ulong
                                                        } else {
                                                            alloc
                                                        })
                                                            .wrapping_add(0 as libc::c_int as libc::c_ulong)
                                                    })
                                                        .wrapping_sub(1 as libc::c_int as libc::c_ulong)
                                                        < 0 as libc::c_int as libc::c_ulong
                                                    {
                                                        ((if 1 as libc::c_int != 0 {
                                                            0 as libc::c_int as libc::c_ulong
                                                        } else {
                                                            (if 1 as libc::c_int != 0 {
                                                                0 as libc::c_int as libc::c_ulong
                                                            } else {
                                                                alloc
                                                            })
                                                                .wrapping_add(0 as libc::c_int as libc::c_ulong)
                                                        })
                                                            .wrapping_add(1 as libc::c_int as libc::c_ulong)
                                                            << (::std::mem::size_of::<libc::c_ulong>() as libc::c_ulong)
                                                                .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                                                                .wrapping_sub(2 as libc::c_int as libc::c_ulong))
                                                            .wrapping_sub(1 as libc::c_int as libc::c_ulong)
                                                            .wrapping_mul(2 as libc::c_int as libc::c_ulong)
                                                            .wrapping_add(1 as libc::c_int as libc::c_ulong)
                                                    } else {
                                                        (if 1 as libc::c_int != 0 {
                                                            0 as libc::c_int as libc::c_ulong
                                                        } else {
                                                            (if 1 as libc::c_int != 0 {
                                                                0 as libc::c_int as libc::c_ulong
                                                            } else {
                                                                alloc
                                                            })
                                                                .wrapping_add(0 as libc::c_int as libc::c_ulong)
                                                        })
                                                            .wrapping_sub(1 as libc::c_int as libc::c_ulong)
                                                    })
                                                        .wrapping_neg()) as libc::c_int
                                            } else {
                                                ((0 as libc::c_int as libc::c_ulong)
                                                    < (if 1 as libc::c_int != 0 {
                                                        0 as libc::c_int as libc::c_ulong
                                                    } else {
                                                        alloc
                                                    })
                                                        .wrapping_add(0 as libc::c_int as libc::c_ulong))
                                                    as libc::c_int
                                            }) != 0 && alloc == -(1 as libc::c_int) as libc::c_ulong
                                            {
                                                if ((if 1 as libc::c_int != 0 {
                                                    0 as libc::c_int
                                                } else {
                                                    7 as libc::c_int
                                                }) - 1 as libc::c_int) < 0 as libc::c_int
                                                {
                                                    ((0 as libc::c_int) < 7 as libc::c_int + 0 as libc::c_int)
                                                        as libc::c_int
                                                } else {
                                                    ((-(1 as libc::c_int) - 0 as libc::c_int)
                                                        < 7 as libc::c_int - 1 as libc::c_int) as libc::c_int
                                                }
                                            } else {
                                                ((0 as libc::c_int as libc::c_ulong).wrapping_div(alloc)
                                                    < 7 as libc::c_int as libc::c_ulong) as libc::c_int
                                            }
                                        } else {
                                            (((2147483647 as libc::c_int as libc::c_uint)
                                                .wrapping_mul(2 as libc::c_uint)
                                                .wrapping_add(1 as libc::c_uint)
                                                .wrapping_div(7 as libc::c_int as libc::c_uint)
                                                as libc::c_ulong) < alloc) as libc::c_int
                                        }
                                    }
                                }) != 0
                                {
                                    alloc = (alloc as libc::c_uint)
                                        .wrapping_mul(7 as libc::c_int as libc::c_uint) as size_t;
                                    1 as libc::c_int
                                } else {
                                    alloc = (alloc as libc::c_uint)
                                        .wrapping_mul(7 as libc::c_int as libc::c_uint) as size_t;
                                    0 as libc::c_int
                                }
                            }
                        } else {
                            if ::std::mem::size_of::<size_t>() as libc::c_ulong
                                == ::std::mem::size_of::<libc::c_long>() as libc::c_ulong
                            {
                                if (if 1 as libc::c_int != 0 {
                                    0 as libc::c_int as libc::c_ulong
                                } else {
                                    alloc
                                })
                                    .wrapping_sub(1 as libc::c_int as libc::c_ulong)
                                    < 0 as libc::c_int as libc::c_ulong
                                {
                                    if (if (7 as libc::c_int) < 0 as libc::c_int {
                                        if alloc < 0 as libc::c_int as libc::c_ulong {
                                            if ((if 1 as libc::c_int != 0 {
                                                0 as libc::c_int as libc::c_long
                                            } else {
                                                (if 1 as libc::c_int != 0 {
                                                    0 as libc::c_int as libc::c_long
                                                } else {
                                                    9223372036854775807 as libc::c_long
                                                }) + 7 as libc::c_int as libc::c_long
                                            }) - 1 as libc::c_int as libc::c_long)
                                                < 0 as libc::c_int as libc::c_long
                                            {
                                                (alloc
                                                    < (9223372036854775807 as libc::c_long
                                                        / 7 as libc::c_int as libc::c_long) as libc::c_ulong)
                                                    as libc::c_int
                                            } else {
                                                ((if (if (if ((if 1 as libc::c_int != 0 {
                                                    0 as libc::c_int
                                                } else {
                                                    7 as libc::c_int
                                                }) - 1 as libc::c_int) < 0 as libc::c_int
                                                {
                                                    !(((((if 1 as libc::c_int != 0 {
                                                        0 as libc::c_int
                                                    } else {
                                                        7 as libc::c_int
                                                    }) + 1 as libc::c_int)
                                                        << (::std::mem::size_of::<libc::c_int>() as libc::c_ulong)
                                                            .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                                                            .wrapping_sub(2 as libc::c_int as libc::c_ulong))
                                                        - 1 as libc::c_int) * 2 as libc::c_int + 1 as libc::c_int)
                                                } else {
                                                    (if 1 as libc::c_int != 0 {
                                                        0 as libc::c_int
                                                    } else {
                                                        7 as libc::c_int
                                                    }) + 0 as libc::c_int
                                                }) < 0 as libc::c_int
                                                {
                                                    ((7 as libc::c_int)
                                                        < -(if ((if 1 as libc::c_int != 0 {
                                                            0 as libc::c_int
                                                        } else {
                                                            7 as libc::c_int
                                                        }) - 1 as libc::c_int) < 0 as libc::c_int
                                                        {
                                                            ((((if 1 as libc::c_int != 0 {
                                                                0 as libc::c_int
                                                            } else {
                                                                7 as libc::c_int
                                                            }) + 1 as libc::c_int)
                                                                << (::std::mem::size_of::<libc::c_int>() as libc::c_ulong)
                                                                    .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                                                                    .wrapping_sub(2 as libc::c_int as libc::c_ulong))
                                                                - 1 as libc::c_int) * 2 as libc::c_int + 1 as libc::c_int
                                                        } else {
                                                            (if 1 as libc::c_int != 0 {
                                                                0 as libc::c_int
                                                            } else {
                                                                7 as libc::c_int
                                                            }) - 1 as libc::c_int
                                                        })) as libc::c_int
                                                } else {
                                                    ((0 as libc::c_int) < 7 as libc::c_int) as libc::c_int
                                                }) != 0
                                                {
                                                    (if 1 as libc::c_int != 0 {
                                                        0 as libc::c_int
                                                    } else {
                                                        7 as libc::c_int
                                                    }) as libc::c_long + 9223372036854775807 as libc::c_long
                                                        >> (::std::mem::size_of::<libc::c_int>() as libc::c_ulong)
                                                            .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                                                            .wrapping_sub(1 as libc::c_int as libc::c_ulong)
                                                } else {
                                                    9223372036854775807 as libc::c_long
                                                        / -(7 as libc::c_int) as libc::c_long
                                                }) as libc::c_ulong
                                                    <= (-(1 as libc::c_int) as libc::c_ulong)
                                                        .wrapping_sub(alloc)) as libc::c_int
                                            }
                                        } else {
                                            if (if (if ((if 1 as libc::c_int != 0 {
                                                0 as libc::c_int as libc::c_long
                                            } else {
                                                (if 1 as libc::c_int != 0 {
                                                    0 as libc::c_int
                                                } else {
                                                    7 as libc::c_int
                                                }) as libc::c_long
                                                    + (-(9223372036854775807 as libc::c_long)
                                                        - 1 as libc::c_long)
                                            }) - 1 as libc::c_int as libc::c_long)
                                                < 0 as libc::c_int as libc::c_long
                                            {
                                                !(((((if 1 as libc::c_int != 0 {
                                                    0 as libc::c_int as libc::c_long
                                                } else {
                                                    (if 1 as libc::c_int != 0 {
                                                        0 as libc::c_int
                                                    } else {
                                                        7 as libc::c_int
                                                    }) as libc::c_long
                                                        + (-(9223372036854775807 as libc::c_long)
                                                            - 1 as libc::c_long)
                                                }) + 1 as libc::c_int as libc::c_long)
                                                    << (::std::mem::size_of::<libc::c_long>() as libc::c_ulong)
                                                        .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                                                        .wrapping_sub(2 as libc::c_int as libc::c_ulong))
                                                    - 1 as libc::c_int as libc::c_long)
                                                    * 2 as libc::c_int as libc::c_long
                                                    + 1 as libc::c_int as libc::c_long)
                                            } else {
                                                (if 1 as libc::c_int != 0 {
                                                    0 as libc::c_int as libc::c_long
                                                } else {
                                                    (if 1 as libc::c_int != 0 {
                                                        0 as libc::c_int
                                                    } else {
                                                        7 as libc::c_int
                                                    }) as libc::c_long
                                                        + (-(9223372036854775807 as libc::c_long)
                                                            - 1 as libc::c_long)
                                                }) + 0 as libc::c_int as libc::c_long
                                            }) < 0 as libc::c_int as libc::c_long
                                            {
                                                ((if 1 as libc::c_int != 0 {
                                                    0 as libc::c_int
                                                } else {
                                                    7 as libc::c_int
                                                }) as libc::c_long
                                                    + (-(9223372036854775807 as libc::c_long)
                                                        - 1 as libc::c_long)
                                                    < -(if ((if 1 as libc::c_int != 0 {
                                                        0 as libc::c_int as libc::c_long
                                                    } else {
                                                        (if 1 as libc::c_int != 0 {
                                                            0 as libc::c_int
                                                        } else {
                                                            7 as libc::c_int
                                                        }) as libc::c_long
                                                            + (-(9223372036854775807 as libc::c_long)
                                                                - 1 as libc::c_long)
                                                    }) - 1 as libc::c_int as libc::c_long)
                                                        < 0 as libc::c_int as libc::c_long
                                                    {
                                                        ((((if 1 as libc::c_int != 0 {
                                                            0 as libc::c_int as libc::c_long
                                                        } else {
                                                            (if 1 as libc::c_int != 0 {
                                                                0 as libc::c_int
                                                            } else {
                                                                7 as libc::c_int
                                                            }) as libc::c_long
                                                                + (-(9223372036854775807 as libc::c_long)
                                                                    - 1 as libc::c_long)
                                                        }) + 1 as libc::c_int as libc::c_long)
                                                            << (::std::mem::size_of::<libc::c_long>() as libc::c_ulong)
                                                                .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                                                                .wrapping_sub(2 as libc::c_int as libc::c_ulong))
                                                            - 1 as libc::c_int as libc::c_long)
                                                            * 2 as libc::c_int as libc::c_long
                                                            + 1 as libc::c_int as libc::c_long
                                                    } else {
                                                        (if 1 as libc::c_int != 0 {
                                                            0 as libc::c_int as libc::c_long
                                                        } else {
                                                            (if 1 as libc::c_int != 0 {
                                                                0 as libc::c_int
                                                            } else {
                                                                7 as libc::c_int
                                                            }) as libc::c_long
                                                                + (-(9223372036854775807 as libc::c_long)
                                                                    - 1 as libc::c_long)
                                                        }) - 1 as libc::c_int as libc::c_long
                                                    })) as libc::c_int
                                            } else {
                                                ((0 as libc::c_int as libc::c_long)
                                                    < (if 1 as libc::c_int != 0 {
                                                        0 as libc::c_int
                                                    } else {
                                                        7 as libc::c_int
                                                    }) as libc::c_long
                                                        + (-(9223372036854775807 as libc::c_long)
                                                            - 1 as libc::c_long)) as libc::c_int
                                            }) != 0 && 7 as libc::c_int == -(1 as libc::c_int)
                                            {
                                                if (if 1 as libc::c_int != 0 {
                                                    0 as libc::c_int as libc::c_ulong
                                                } else {
                                                    alloc
                                                })
                                                    .wrapping_sub(1 as libc::c_int as libc::c_ulong)
                                                    < 0 as libc::c_int as libc::c_ulong
                                                {
                                                    ((0 as libc::c_int as libc::c_ulong)
                                                        < alloc
                                                            .wrapping_add(
                                                                (-(9223372036854775807 as libc::c_long) - 1 as libc::c_long)
                                                                    as libc::c_ulong,
                                                            )) as libc::c_int
                                                } else {
                                                    ((0 as libc::c_int as libc::c_ulong) < alloc
                                                        && ((-(1 as libc::c_int) as libc::c_long
                                                            - (-(9223372036854775807 as libc::c_long)
                                                                - 1 as libc::c_long)) as libc::c_ulong)
                                                            < alloc.wrapping_sub(1 as libc::c_int as libc::c_ulong))
                                                        as libc::c_int
                                                }
                                            } else {
                                                ((((-(9223372036854775807 as libc::c_long)
                                                    - 1 as libc::c_long) / 7 as libc::c_int as libc::c_long)
                                                    as libc::c_ulong) < alloc) as libc::c_int
                                            }
                                        }
                                    } else {
                                        if 7 as libc::c_int == 0 as libc::c_int {
                                            0 as libc::c_int
                                        } else {
                                            if alloc < 0 as libc::c_int as libc::c_ulong {
                                                if (if (if (if 1 as libc::c_int != 0 {
                                                    0 as libc::c_int as libc::c_ulong
                                                } else {
                                                    (if 1 as libc::c_int != 0 {
                                                        0 as libc::c_int as libc::c_ulong
                                                    } else {
                                                        alloc
                                                    })
                                                        .wrapping_add(
                                                            (-(9223372036854775807 as libc::c_long) - 1 as libc::c_long)
                                                                as libc::c_ulong,
                                                        )
                                                })
                                                    .wrapping_sub(1 as libc::c_int as libc::c_ulong)
                                                    < 0 as libc::c_int as libc::c_ulong
                                                {
                                                    !((if 1 as libc::c_int != 0 {
                                                        0 as libc::c_int as libc::c_ulong
                                                    } else {
                                                        (if 1 as libc::c_int != 0 {
                                                            0 as libc::c_int as libc::c_ulong
                                                        } else {
                                                            alloc
                                                        })
                                                            .wrapping_add(
                                                                (-(9223372036854775807 as libc::c_long) - 1 as libc::c_long)
                                                                    as libc::c_ulong,
                                                            )
                                                    })
                                                        .wrapping_add(1 as libc::c_int as libc::c_ulong)
                                                        << (::std::mem::size_of::<libc::c_ulong>() as libc::c_ulong)
                                                            .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                                                            .wrapping_sub(2 as libc::c_int as libc::c_ulong))
                                                        .wrapping_sub(1 as libc::c_int as libc::c_ulong)
                                                        .wrapping_mul(2 as libc::c_int as libc::c_ulong)
                                                        .wrapping_add(1 as libc::c_int as libc::c_ulong)
                                                } else {
                                                    (if 1 as libc::c_int != 0 {
                                                        0 as libc::c_int as libc::c_ulong
                                                    } else {
                                                        (if 1 as libc::c_int != 0 {
                                                            0 as libc::c_int as libc::c_ulong
                                                        } else {
                                                            alloc
                                                        })
                                                            .wrapping_add(
                                                                (-(9223372036854775807 as libc::c_long) - 1 as libc::c_long)
                                                                    as libc::c_ulong,
                                                            )
                                                    })
                                                        .wrapping_add(0 as libc::c_int as libc::c_ulong)
                                                }) < 0 as libc::c_int as libc::c_ulong
                                                {
                                                    ((if 1 as libc::c_int != 0 {
                                                        0 as libc::c_int as libc::c_ulong
                                                    } else {
                                                        alloc
                                                    })
                                                        .wrapping_add(
                                                            (-(9223372036854775807 as libc::c_long) - 1 as libc::c_long)
                                                                as libc::c_ulong,
                                                        )
                                                        < (if (if 1 as libc::c_int != 0 {
                                                            0 as libc::c_int as libc::c_ulong
                                                        } else {
                                                            (if 1 as libc::c_int != 0 {
                                                                0 as libc::c_int as libc::c_ulong
                                                            } else {
                                                                alloc
                                                            })
                                                                .wrapping_add(
                                                                    (-(9223372036854775807 as libc::c_long) - 1 as libc::c_long)
                                                                        as libc::c_ulong,
                                                                )
                                                        })
                                                            .wrapping_sub(1 as libc::c_int as libc::c_ulong)
                                                            < 0 as libc::c_int as libc::c_ulong
                                                        {
                                                            ((if 1 as libc::c_int != 0 {
                                                                0 as libc::c_int as libc::c_ulong
                                                            } else {
                                                                (if 1 as libc::c_int != 0 {
                                                                    0 as libc::c_int as libc::c_ulong
                                                                } else {
                                                                    alloc
                                                                })
                                                                    .wrapping_add(
                                                                        (-(9223372036854775807 as libc::c_long) - 1 as libc::c_long)
                                                                            as libc::c_ulong,
                                                                    )
                                                            })
                                                                .wrapping_add(1 as libc::c_int as libc::c_ulong)
                                                                << (::std::mem::size_of::<libc::c_ulong>() as libc::c_ulong)
                                                                    .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                                                                    .wrapping_sub(2 as libc::c_int as libc::c_ulong))
                                                                .wrapping_sub(1 as libc::c_int as libc::c_ulong)
                                                                .wrapping_mul(2 as libc::c_int as libc::c_ulong)
                                                                .wrapping_add(1 as libc::c_int as libc::c_ulong)
                                                        } else {
                                                            (if 1 as libc::c_int != 0 {
                                                                0 as libc::c_int as libc::c_ulong
                                                            } else {
                                                                (if 1 as libc::c_int != 0 {
                                                                    0 as libc::c_int as libc::c_ulong
                                                                } else {
                                                                    alloc
                                                                })
                                                                    .wrapping_add(
                                                                        (-(9223372036854775807 as libc::c_long) - 1 as libc::c_long)
                                                                            as libc::c_ulong,
                                                                    )
                                                            })
                                                                .wrapping_sub(1 as libc::c_int as libc::c_ulong)
                                                        })
                                                            .wrapping_neg()) as libc::c_int
                                                } else {
                                                    ((0 as libc::c_int as libc::c_ulong)
                                                        < (if 1 as libc::c_int != 0 {
                                                            0 as libc::c_int as libc::c_ulong
                                                        } else {
                                                            alloc
                                                        })
                                                            .wrapping_add(
                                                                (-(9223372036854775807 as libc::c_long) - 1 as libc::c_long)
                                                                    as libc::c_ulong,
                                                            )) as libc::c_int
                                                }) != 0 && alloc == -(1 as libc::c_int) as libc::c_ulong
                                                {
                                                    if ((if 1 as libc::c_int != 0 {
                                                        0 as libc::c_int
                                                    } else {
                                                        7 as libc::c_int
                                                    }) - 1 as libc::c_int) < 0 as libc::c_int
                                                    {
                                                        ((0 as libc::c_int as libc::c_long)
                                                            < 7 as libc::c_int as libc::c_long
                                                                + (-(9223372036854775807 as libc::c_long)
                                                                    - 1 as libc::c_long)) as libc::c_int
                                                    } else {
                                                        (-(1 as libc::c_int) as libc::c_long
                                                            - (-(9223372036854775807 as libc::c_long)
                                                                - 1 as libc::c_long)
                                                            < (7 as libc::c_int - 1 as libc::c_int) as libc::c_long)
                                                            as libc::c_int
                                                    }
                                                } else {
                                                    (((-(9223372036854775807 as libc::c_long)
                                                        - 1 as libc::c_long) as libc::c_ulong)
                                                        .wrapping_div(alloc) < 7 as libc::c_int as libc::c_ulong)
                                                        as libc::c_int
                                                }
                                            } else {
                                                (((9223372036854775807 as libc::c_long
                                                    / 7 as libc::c_int as libc::c_long) as libc::c_ulong)
                                                    < alloc) as libc::c_int
                                            }
                                        }
                                    }) != 0
                                    {
                                        alloc = alloc
                                            .wrapping_mul(7 as libc::c_int as libc::c_ulong)
                                            as libc::c_long as size_t;
                                        1 as libc::c_int
                                    } else {
                                        alloc = alloc
                                            .wrapping_mul(7 as libc::c_int as libc::c_ulong)
                                            as libc::c_long as size_t;
                                        0 as libc::c_int
                                    }
                                } else {
                                    if (if (7 as libc::c_int) < 0 as libc::c_int {
                                        if alloc < 0 as libc::c_int as libc::c_ulong {
                                            if (if 1 as libc::c_int != 0 {
                                                0 as libc::c_int as libc::c_ulong
                                            } else {
                                                (if 1 as libc::c_int != 0 {
                                                    0 as libc::c_int as libc::c_ulong
                                                } else {
                                                    (9223372036854775807 as libc::c_long as libc::c_ulong)
                                                        .wrapping_mul(2 as libc::c_ulong)
                                                        .wrapping_add(1 as libc::c_ulong)
                                                })
                                                    .wrapping_add(7 as libc::c_int as libc::c_ulong)
                                            })
                                                .wrapping_sub(1 as libc::c_int as libc::c_ulong)
                                                < 0 as libc::c_int as libc::c_ulong
                                            {
                                                (alloc
                                                    < (9223372036854775807 as libc::c_long as libc::c_ulong)
                                                        .wrapping_mul(2 as libc::c_ulong)
                                                        .wrapping_add(1 as libc::c_ulong)
                                                        .wrapping_div(7 as libc::c_int as libc::c_ulong))
                                                    as libc::c_int
                                            } else {
                                                ((if (if (if ((if 1 as libc::c_int != 0 {
                                                    0 as libc::c_int
                                                } else {
                                                    7 as libc::c_int
                                                }) - 1 as libc::c_int) < 0 as libc::c_int
                                                {
                                                    !(((((if 1 as libc::c_int != 0 {
                                                        0 as libc::c_int
                                                    } else {
                                                        7 as libc::c_int
                                                    }) + 1 as libc::c_int)
                                                        << (::std::mem::size_of::<libc::c_int>() as libc::c_ulong)
                                                            .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                                                            .wrapping_sub(2 as libc::c_int as libc::c_ulong))
                                                        - 1 as libc::c_int) * 2 as libc::c_int + 1 as libc::c_int)
                                                } else {
                                                    (if 1 as libc::c_int != 0 {
                                                        0 as libc::c_int
                                                    } else {
                                                        7 as libc::c_int
                                                    }) + 0 as libc::c_int
                                                }) < 0 as libc::c_int
                                                {
                                                    ((7 as libc::c_int)
                                                        < -(if ((if 1 as libc::c_int != 0 {
                                                            0 as libc::c_int
                                                        } else {
                                                            7 as libc::c_int
                                                        }) - 1 as libc::c_int) < 0 as libc::c_int
                                                        {
                                                            ((((if 1 as libc::c_int != 0 {
                                                                0 as libc::c_int
                                                            } else {
                                                                7 as libc::c_int
                                                            }) + 1 as libc::c_int)
                                                                << (::std::mem::size_of::<libc::c_int>() as libc::c_ulong)
                                                                    .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                                                                    .wrapping_sub(2 as libc::c_int as libc::c_ulong))
                                                                - 1 as libc::c_int) * 2 as libc::c_int + 1 as libc::c_int
                                                        } else {
                                                            (if 1 as libc::c_int != 0 {
                                                                0 as libc::c_int
                                                            } else {
                                                                7 as libc::c_int
                                                            }) - 1 as libc::c_int
                                                        })) as libc::c_int
                                                } else {
                                                    ((0 as libc::c_int) < 7 as libc::c_int) as libc::c_int
                                                }) != 0
                                                {
                                                    ((if 1 as libc::c_int != 0 {
                                                        0 as libc::c_int
                                                    } else {
                                                        7 as libc::c_int
                                                    }) as libc::c_ulong)
                                                        .wrapping_add(
                                                            (9223372036854775807 as libc::c_long as libc::c_ulong)
                                                                .wrapping_mul(2 as libc::c_ulong)
                                                                .wrapping_add(1 as libc::c_ulong),
                                                        )
                                                        >> (::std::mem::size_of::<libc::c_int>() as libc::c_ulong)
                                                            .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                                                            .wrapping_sub(1 as libc::c_int as libc::c_ulong)
                                                } else {
                                                    (9223372036854775807 as libc::c_long as libc::c_ulong)
                                                        .wrapping_mul(2 as libc::c_ulong)
                                                        .wrapping_add(1 as libc::c_ulong)
                                                        .wrapping_div(-(7 as libc::c_int) as libc::c_ulong)
                                                })
                                                    <= (-(1 as libc::c_int) as libc::c_ulong)
                                                        .wrapping_sub(alloc)) as libc::c_int
                                            }
                                        } else {
                                            if (if (if ((if 1 as libc::c_int != 0 {
                                                0 as libc::c_int
                                            } else {
                                                (if 1 as libc::c_int != 0 {
                                                    0 as libc::c_int
                                                } else {
                                                    7 as libc::c_int
                                                }) + 0 as libc::c_int
                                            }) - 1 as libc::c_int) < 0 as libc::c_int
                                            {
                                                !(((((if 1 as libc::c_int != 0 {
                                                    0 as libc::c_int
                                                } else {
                                                    (if 1 as libc::c_int != 0 {
                                                        0 as libc::c_int
                                                    } else {
                                                        7 as libc::c_int
                                                    }) + 0 as libc::c_int
                                                }) + 1 as libc::c_int)
                                                    << (::std::mem::size_of::<libc::c_int>() as libc::c_ulong)
                                                        .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                                                        .wrapping_sub(2 as libc::c_int as libc::c_ulong))
                                                    - 1 as libc::c_int) * 2 as libc::c_int + 1 as libc::c_int)
                                            } else {
                                                (if 1 as libc::c_int != 0 {
                                                    0 as libc::c_int
                                                } else {
                                                    (if 1 as libc::c_int != 0 {
                                                        0 as libc::c_int
                                                    } else {
                                                        7 as libc::c_int
                                                    }) + 0 as libc::c_int
                                                }) + 0 as libc::c_int
                                            }) < 0 as libc::c_int
                                            {
                                                (((if 1 as libc::c_int != 0 {
                                                    0 as libc::c_int
                                                } else {
                                                    7 as libc::c_int
                                                }) + 0 as libc::c_int)
                                                    < -(if ((if 1 as libc::c_int != 0 {
                                                        0 as libc::c_int
                                                    } else {
                                                        (if 1 as libc::c_int != 0 {
                                                            0 as libc::c_int
                                                        } else {
                                                            7 as libc::c_int
                                                        }) + 0 as libc::c_int
                                                    }) - 1 as libc::c_int) < 0 as libc::c_int
                                                    {
                                                        ((((if 1 as libc::c_int != 0 {
                                                            0 as libc::c_int
                                                        } else {
                                                            (if 1 as libc::c_int != 0 {
                                                                0 as libc::c_int
                                                            } else {
                                                                7 as libc::c_int
                                                            }) + 0 as libc::c_int
                                                        }) + 1 as libc::c_int)
                                                            << (::std::mem::size_of::<libc::c_int>() as libc::c_ulong)
                                                                .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                                                                .wrapping_sub(2 as libc::c_int as libc::c_ulong))
                                                            - 1 as libc::c_int) * 2 as libc::c_int + 1 as libc::c_int
                                                    } else {
                                                        (if 1 as libc::c_int != 0 {
                                                            0 as libc::c_int
                                                        } else {
                                                            (if 1 as libc::c_int != 0 {
                                                                0 as libc::c_int
                                                            } else {
                                                                7 as libc::c_int
                                                            }) + 0 as libc::c_int
                                                        }) - 1 as libc::c_int
                                                    })) as libc::c_int
                                            } else {
                                                ((0 as libc::c_int)
                                                    < (if 1 as libc::c_int != 0 {
                                                        0 as libc::c_int
                                                    } else {
                                                        7 as libc::c_int
                                                    }) + 0 as libc::c_int) as libc::c_int
                                            }) != 0 && 7 as libc::c_int == -(1 as libc::c_int)
                                            {
                                                if (if 1 as libc::c_int != 0 {
                                                    0 as libc::c_int as libc::c_ulong
                                                } else {
                                                    alloc
                                                })
                                                    .wrapping_sub(1 as libc::c_int as libc::c_ulong)
                                                    < 0 as libc::c_int as libc::c_ulong
                                                {
                                                    ((0 as libc::c_int as libc::c_ulong)
                                                        < alloc.wrapping_add(0 as libc::c_int as libc::c_ulong))
                                                        as libc::c_int
                                                } else {
                                                    ((0 as libc::c_int as libc::c_ulong) < alloc
                                                        && ((-(1 as libc::c_int) - 0 as libc::c_int)
                                                            as libc::c_ulong)
                                                            < alloc.wrapping_sub(1 as libc::c_int as libc::c_ulong))
                                                        as libc::c_int
                                                }
                                            } else {
                                                (((0 as libc::c_int / 7 as libc::c_int) as libc::c_ulong)
                                                    < alloc) as libc::c_int
                                            }
                                        }
                                    } else {
                                        if 7 as libc::c_int == 0 as libc::c_int {
                                            0 as libc::c_int
                                        } else {
                                            if alloc < 0 as libc::c_int as libc::c_ulong {
                                                if (if (if (if 1 as libc::c_int != 0 {
                                                    0 as libc::c_int as libc::c_ulong
                                                } else {
                                                    (if 1 as libc::c_int != 0 {
                                                        0 as libc::c_int as libc::c_ulong
                                                    } else {
                                                        alloc
                                                    })
                                                        .wrapping_add(0 as libc::c_int as libc::c_ulong)
                                                })
                                                    .wrapping_sub(1 as libc::c_int as libc::c_ulong)
                                                    < 0 as libc::c_int as libc::c_ulong
                                                {
                                                    !((if 1 as libc::c_int != 0 {
                                                        0 as libc::c_int as libc::c_ulong
                                                    } else {
                                                        (if 1 as libc::c_int != 0 {
                                                            0 as libc::c_int as libc::c_ulong
                                                        } else {
                                                            alloc
                                                        })
                                                            .wrapping_add(0 as libc::c_int as libc::c_ulong)
                                                    })
                                                        .wrapping_add(1 as libc::c_int as libc::c_ulong)
                                                        << (::std::mem::size_of::<libc::c_ulong>() as libc::c_ulong)
                                                            .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                                                            .wrapping_sub(2 as libc::c_int as libc::c_ulong))
                                                        .wrapping_sub(1 as libc::c_int as libc::c_ulong)
                                                        .wrapping_mul(2 as libc::c_int as libc::c_ulong)
                                                        .wrapping_add(1 as libc::c_int as libc::c_ulong)
                                                } else {
                                                    (if 1 as libc::c_int != 0 {
                                                        0 as libc::c_int as libc::c_ulong
                                                    } else {
                                                        (if 1 as libc::c_int != 0 {
                                                            0 as libc::c_int as libc::c_ulong
                                                        } else {
                                                            alloc
                                                        })
                                                            .wrapping_add(0 as libc::c_int as libc::c_ulong)
                                                    })
                                                        .wrapping_add(0 as libc::c_int as libc::c_ulong)
                                                }) < 0 as libc::c_int as libc::c_ulong
                                                {
                                                    ((if 1 as libc::c_int != 0 {
                                                        0 as libc::c_int as libc::c_ulong
                                                    } else {
                                                        alloc
                                                    })
                                                        .wrapping_add(0 as libc::c_int as libc::c_ulong)
                                                        < (if (if 1 as libc::c_int != 0 {
                                                            0 as libc::c_int as libc::c_ulong
                                                        } else {
                                                            (if 1 as libc::c_int != 0 {
                                                                0 as libc::c_int as libc::c_ulong
                                                            } else {
                                                                alloc
                                                            })
                                                                .wrapping_add(0 as libc::c_int as libc::c_ulong)
                                                        })
                                                            .wrapping_sub(1 as libc::c_int as libc::c_ulong)
                                                            < 0 as libc::c_int as libc::c_ulong
                                                        {
                                                            ((if 1 as libc::c_int != 0 {
                                                                0 as libc::c_int as libc::c_ulong
                                                            } else {
                                                                (if 1 as libc::c_int != 0 {
                                                                    0 as libc::c_int as libc::c_ulong
                                                                } else {
                                                                    alloc
                                                                })
                                                                    .wrapping_add(0 as libc::c_int as libc::c_ulong)
                                                            })
                                                                .wrapping_add(1 as libc::c_int as libc::c_ulong)
                                                                << (::std::mem::size_of::<libc::c_ulong>() as libc::c_ulong)
                                                                    .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                                                                    .wrapping_sub(2 as libc::c_int as libc::c_ulong))
                                                                .wrapping_sub(1 as libc::c_int as libc::c_ulong)
                                                                .wrapping_mul(2 as libc::c_int as libc::c_ulong)
                                                                .wrapping_add(1 as libc::c_int as libc::c_ulong)
                                                        } else {
                                                            (if 1 as libc::c_int != 0 {
                                                                0 as libc::c_int as libc::c_ulong
                                                            } else {
                                                                (if 1 as libc::c_int != 0 {
                                                                    0 as libc::c_int as libc::c_ulong
                                                                } else {
                                                                    alloc
                                                                })
                                                                    .wrapping_add(0 as libc::c_int as libc::c_ulong)
                                                            })
                                                                .wrapping_sub(1 as libc::c_int as libc::c_ulong)
                                                        })
                                                            .wrapping_neg()) as libc::c_int
                                                } else {
                                                    ((0 as libc::c_int as libc::c_ulong)
                                                        < (if 1 as libc::c_int != 0 {
                                                            0 as libc::c_int as libc::c_ulong
                                                        } else {
                                                            alloc
                                                        })
                                                            .wrapping_add(0 as libc::c_int as libc::c_ulong))
                                                        as libc::c_int
                                                }) != 0 && alloc == -(1 as libc::c_int) as libc::c_ulong
                                                {
                                                    if ((if 1 as libc::c_int != 0 {
                                                        0 as libc::c_int
                                                    } else {
                                                        7 as libc::c_int
                                                    }) - 1 as libc::c_int) < 0 as libc::c_int
                                                    {
                                                        ((0 as libc::c_int) < 7 as libc::c_int + 0 as libc::c_int)
                                                            as libc::c_int
                                                    } else {
                                                        ((-(1 as libc::c_int) - 0 as libc::c_int)
                                                            < 7 as libc::c_int - 1 as libc::c_int) as libc::c_int
                                                    }
                                                } else {
                                                    ((0 as libc::c_int as libc::c_ulong).wrapping_div(alloc)
                                                        < 7 as libc::c_int as libc::c_ulong) as libc::c_int
                                                }
                                            } else {
                                                ((9223372036854775807 as libc::c_long as libc::c_ulong)
                                                    .wrapping_mul(2 as libc::c_ulong)
                                                    .wrapping_add(1 as libc::c_ulong)
                                                    .wrapping_div(7 as libc::c_int as libc::c_ulong) < alloc)
                                                    as libc::c_int
                                            }
                                        }
                                    }) != 0
                                    {
                                        alloc = alloc
                                            .wrapping_mul(7 as libc::c_int as libc::c_ulong);
                                        1 as libc::c_int
                                    } else {
                                        alloc = alloc
                                            .wrapping_mul(7 as libc::c_int as libc::c_ulong);
                                        0 as libc::c_int
                                    }
                                }
                            } else {
                                if (if 1 as libc::c_int != 0 {
                                    0 as libc::c_int as libc::c_ulong
                                } else {
                                    alloc
                                })
                                    .wrapping_sub(1 as libc::c_int as libc::c_ulong)
                                    < 0 as libc::c_int as libc::c_ulong
                                {
                                    if (if (7 as libc::c_int) < 0 as libc::c_int {
                                        if alloc < 0 as libc::c_int as libc::c_ulong {
                                            if ((if 1 as libc::c_int != 0 {
                                                0 as libc::c_int as libc::c_longlong
                                            } else {
                                                (if 1 as libc::c_int != 0 {
                                                    0 as libc::c_int as libc::c_longlong
                                                } else {
                                                    9223372036854775807 as libc::c_longlong
                                                }) + 7 as libc::c_int as libc::c_longlong
                                            }) - 1 as libc::c_int as libc::c_longlong)
                                                < 0 as libc::c_int as libc::c_longlong
                                            {
                                                ((alloc as libc::c_ulonglong)
                                                    < (9223372036854775807 as libc::c_longlong
                                                        / 7 as libc::c_int as libc::c_longlong)
                                                        as libc::c_ulonglong) as libc::c_int
                                            } else {
                                                ((if (if (if ((if 1 as libc::c_int != 0 {
                                                    0 as libc::c_int
                                                } else {
                                                    7 as libc::c_int
                                                }) - 1 as libc::c_int) < 0 as libc::c_int
                                                {
                                                    !(((((if 1 as libc::c_int != 0 {
                                                        0 as libc::c_int
                                                    } else {
                                                        7 as libc::c_int
                                                    }) + 1 as libc::c_int)
                                                        << (::std::mem::size_of::<libc::c_int>() as libc::c_ulong)
                                                            .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                                                            .wrapping_sub(2 as libc::c_int as libc::c_ulong))
                                                        - 1 as libc::c_int) * 2 as libc::c_int + 1 as libc::c_int)
                                                } else {
                                                    (if 1 as libc::c_int != 0 {
                                                        0 as libc::c_int
                                                    } else {
                                                        7 as libc::c_int
                                                    }) + 0 as libc::c_int
                                                }) < 0 as libc::c_int
                                                {
                                                    ((7 as libc::c_int)
                                                        < -(if ((if 1 as libc::c_int != 0 {
                                                            0 as libc::c_int
                                                        } else {
                                                            7 as libc::c_int
                                                        }) - 1 as libc::c_int) < 0 as libc::c_int
                                                        {
                                                            ((((if 1 as libc::c_int != 0 {
                                                                0 as libc::c_int
                                                            } else {
                                                                7 as libc::c_int
                                                            }) + 1 as libc::c_int)
                                                                << (::std::mem::size_of::<libc::c_int>() as libc::c_ulong)
                                                                    .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                                                                    .wrapping_sub(2 as libc::c_int as libc::c_ulong))
                                                                - 1 as libc::c_int) * 2 as libc::c_int + 1 as libc::c_int
                                                        } else {
                                                            (if 1 as libc::c_int != 0 {
                                                                0 as libc::c_int
                                                            } else {
                                                                7 as libc::c_int
                                                            }) - 1 as libc::c_int
                                                        })) as libc::c_int
                                                } else {
                                                    ((0 as libc::c_int) < 7 as libc::c_int) as libc::c_int
                                                }) != 0
                                                {
                                                    (if 1 as libc::c_int != 0 {
                                                        0 as libc::c_int
                                                    } else {
                                                        7 as libc::c_int
                                                    }) as libc::c_longlong
                                                        + 9223372036854775807 as libc::c_longlong
                                                        >> (::std::mem::size_of::<libc::c_int>() as libc::c_ulong)
                                                            .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                                                            .wrapping_sub(1 as libc::c_int as libc::c_ulong)
                                                } else {
                                                    9223372036854775807 as libc::c_longlong
                                                        / -(7 as libc::c_int) as libc::c_longlong
                                                }) as libc::c_ulonglong
                                                    <= (-(1 as libc::c_int) as libc::c_ulong)
                                                        .wrapping_sub(alloc) as libc::c_ulonglong) as libc::c_int
                                            }
                                        } else {
                                            if (if (if ((if 1 as libc::c_int != 0 {
                                                0 as libc::c_int as libc::c_longlong
                                            } else {
                                                (if 1 as libc::c_int != 0 {
                                                    0 as libc::c_int
                                                } else {
                                                    7 as libc::c_int
                                                }) as libc::c_longlong
                                                    + (-(9223372036854775807 as libc::c_longlong)
                                                        - 1 as libc::c_longlong)
                                            }) - 1 as libc::c_int as libc::c_longlong)
                                                < 0 as libc::c_int as libc::c_longlong
                                            {
                                                !(((((if 1 as libc::c_int != 0 {
                                                    0 as libc::c_int as libc::c_longlong
                                                } else {
                                                    (if 1 as libc::c_int != 0 {
                                                        0 as libc::c_int
                                                    } else {
                                                        7 as libc::c_int
                                                    }) as libc::c_longlong
                                                        + (-(9223372036854775807 as libc::c_longlong)
                                                            - 1 as libc::c_longlong)
                                                }) + 1 as libc::c_int as libc::c_longlong)
                                                    << (::std::mem::size_of::<libc::c_longlong>()
                                                        as libc::c_ulong)
                                                        .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                                                        .wrapping_sub(2 as libc::c_int as libc::c_ulong))
                                                    - 1 as libc::c_int as libc::c_longlong)
                                                    * 2 as libc::c_int as libc::c_longlong
                                                    + 1 as libc::c_int as libc::c_longlong)
                                            } else {
                                                (if 1 as libc::c_int != 0 {
                                                    0 as libc::c_int as libc::c_longlong
                                                } else {
                                                    (if 1 as libc::c_int != 0 {
                                                        0 as libc::c_int
                                                    } else {
                                                        7 as libc::c_int
                                                    }) as libc::c_longlong
                                                        + (-(9223372036854775807 as libc::c_longlong)
                                                            - 1 as libc::c_longlong)
                                                }) + 0 as libc::c_int as libc::c_longlong
                                            }) < 0 as libc::c_int as libc::c_longlong
                                            {
                                                ((if 1 as libc::c_int != 0 {
                                                    0 as libc::c_int
                                                } else {
                                                    7 as libc::c_int
                                                }) as libc::c_longlong
                                                    + (-(9223372036854775807 as libc::c_longlong)
                                                        - 1 as libc::c_longlong)
                                                    < -(if ((if 1 as libc::c_int != 0 {
                                                        0 as libc::c_int as libc::c_longlong
                                                    } else {
                                                        (if 1 as libc::c_int != 0 {
                                                            0 as libc::c_int
                                                        } else {
                                                            7 as libc::c_int
                                                        }) as libc::c_longlong
                                                            + (-(9223372036854775807 as libc::c_longlong)
                                                                - 1 as libc::c_longlong)
                                                    }) - 1 as libc::c_int as libc::c_longlong)
                                                        < 0 as libc::c_int as libc::c_longlong
                                                    {
                                                        ((((if 1 as libc::c_int != 0 {
                                                            0 as libc::c_int as libc::c_longlong
                                                        } else {
                                                            (if 1 as libc::c_int != 0 {
                                                                0 as libc::c_int
                                                            } else {
                                                                7 as libc::c_int
                                                            }) as libc::c_longlong
                                                                + (-(9223372036854775807 as libc::c_longlong)
                                                                    - 1 as libc::c_longlong)
                                                        }) + 1 as libc::c_int as libc::c_longlong)
                                                            << (::std::mem::size_of::<libc::c_longlong>()
                                                                as libc::c_ulong)
                                                                .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                                                                .wrapping_sub(2 as libc::c_int as libc::c_ulong))
                                                            - 1 as libc::c_int as libc::c_longlong)
                                                            * 2 as libc::c_int as libc::c_longlong
                                                            + 1 as libc::c_int as libc::c_longlong
                                                    } else {
                                                        (if 1 as libc::c_int != 0 {
                                                            0 as libc::c_int as libc::c_longlong
                                                        } else {
                                                            (if 1 as libc::c_int != 0 {
                                                                0 as libc::c_int
                                                            } else {
                                                                7 as libc::c_int
                                                            }) as libc::c_longlong
                                                                + (-(9223372036854775807 as libc::c_longlong)
                                                                    - 1 as libc::c_longlong)
                                                        }) - 1 as libc::c_int as libc::c_longlong
                                                    })) as libc::c_int
                                            } else {
                                                ((0 as libc::c_int as libc::c_longlong)
                                                    < (if 1 as libc::c_int != 0 {
                                                        0 as libc::c_int
                                                    } else {
                                                        7 as libc::c_int
                                                    }) as libc::c_longlong
                                                        + (-(9223372036854775807 as libc::c_longlong)
                                                            - 1 as libc::c_longlong)) as libc::c_int
                                            }) != 0 && 7 as libc::c_int == -(1 as libc::c_int)
                                            {
                                                if (if 1 as libc::c_int != 0 {
                                                    0 as libc::c_int as libc::c_ulong
                                                } else {
                                                    alloc
                                                })
                                                    .wrapping_sub(1 as libc::c_int as libc::c_ulong)
                                                    < 0 as libc::c_int as libc::c_ulong
                                                {
                                                    ((0 as libc::c_int as libc::c_ulonglong)
                                                        < (alloc as libc::c_ulonglong)
                                                            .wrapping_add(
                                                                (-(9223372036854775807 as libc::c_longlong)
                                                                    - 1 as libc::c_longlong) as libc::c_ulonglong,
                                                            )) as libc::c_int
                                                } else {
                                                    ((0 as libc::c_int as libc::c_ulong) < alloc
                                                        && ((-(1 as libc::c_int) as libc::c_longlong
                                                            - (-(9223372036854775807 as libc::c_longlong)
                                                                - 1 as libc::c_longlong)) as libc::c_ulonglong)
                                                            < alloc.wrapping_sub(1 as libc::c_int as libc::c_ulong)
                                                                as libc::c_ulonglong) as libc::c_int
                                                }
                                            } else {
                                                ((((-(9223372036854775807 as libc::c_longlong)
                                                    - 1 as libc::c_longlong)
                                                    / 7 as libc::c_int as libc::c_longlong)
                                                    as libc::c_ulonglong) < alloc as libc::c_ulonglong)
                                                    as libc::c_int
                                            }
                                        }
                                    } else {
                                        if 7 as libc::c_int == 0 as libc::c_int {
                                            0 as libc::c_int
                                        } else {
                                            if alloc < 0 as libc::c_int as libc::c_ulong {
                                                if (if (if (if 1 as libc::c_int != 0 {
                                                    0 as libc::c_int as libc::c_ulonglong
                                                } else {
                                                    ((if 1 as libc::c_int != 0 {
                                                        0 as libc::c_int as libc::c_ulong
                                                    } else {
                                                        alloc
                                                    }) as libc::c_ulonglong)
                                                        .wrapping_add(
                                                            (-(9223372036854775807 as libc::c_longlong)
                                                                - 1 as libc::c_longlong) as libc::c_ulonglong,
                                                        )
                                                })
                                                    .wrapping_sub(1 as libc::c_int as libc::c_ulonglong)
                                                    < 0 as libc::c_int as libc::c_ulonglong
                                                {
                                                    !((if 1 as libc::c_int != 0 {
                                                        0 as libc::c_int as libc::c_ulonglong
                                                    } else {
                                                        ((if 1 as libc::c_int != 0 {
                                                            0 as libc::c_int as libc::c_ulong
                                                        } else {
                                                            alloc
                                                        }) as libc::c_ulonglong)
                                                            .wrapping_add(
                                                                (-(9223372036854775807 as libc::c_longlong)
                                                                    - 1 as libc::c_longlong) as libc::c_ulonglong,
                                                            )
                                                    })
                                                        .wrapping_add(1 as libc::c_int as libc::c_ulonglong)
                                                        << (::std::mem::size_of::<libc::c_ulonglong>()
                                                            as libc::c_ulong)
                                                            .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                                                            .wrapping_sub(2 as libc::c_int as libc::c_ulong))
                                                        .wrapping_sub(1 as libc::c_int as libc::c_ulonglong)
                                                        .wrapping_mul(2 as libc::c_int as libc::c_ulonglong)
                                                        .wrapping_add(1 as libc::c_int as libc::c_ulonglong)
                                                } else {
                                                    (if 1 as libc::c_int != 0 {
                                                        0 as libc::c_int as libc::c_ulonglong
                                                    } else {
                                                        ((if 1 as libc::c_int != 0 {
                                                            0 as libc::c_int as libc::c_ulong
                                                        } else {
                                                            alloc
                                                        }) as libc::c_ulonglong)
                                                            .wrapping_add(
                                                                (-(9223372036854775807 as libc::c_longlong)
                                                                    - 1 as libc::c_longlong) as libc::c_ulonglong,
                                                            )
                                                    })
                                                        .wrapping_add(0 as libc::c_int as libc::c_ulonglong)
                                                }) < 0 as libc::c_int as libc::c_ulonglong
                                                {
                                                    (((if 1 as libc::c_int != 0 {
                                                        0 as libc::c_int as libc::c_ulong
                                                    } else {
                                                        alloc
                                                    }) as libc::c_ulonglong)
                                                        .wrapping_add(
                                                            (-(9223372036854775807 as libc::c_longlong)
                                                                - 1 as libc::c_longlong) as libc::c_ulonglong,
                                                        )
                                                        < (if (if 1 as libc::c_int != 0 {
                                                            0 as libc::c_int as libc::c_ulonglong
                                                        } else {
                                                            ((if 1 as libc::c_int != 0 {
                                                                0 as libc::c_int as libc::c_ulong
                                                            } else {
                                                                alloc
                                                            }) as libc::c_ulonglong)
                                                                .wrapping_add(
                                                                    (-(9223372036854775807 as libc::c_longlong)
                                                                        - 1 as libc::c_longlong) as libc::c_ulonglong,
                                                                )
                                                        })
                                                            .wrapping_sub(1 as libc::c_int as libc::c_ulonglong)
                                                            < 0 as libc::c_int as libc::c_ulonglong
                                                        {
                                                            ((if 1 as libc::c_int != 0 {
                                                                0 as libc::c_int as libc::c_ulonglong
                                                            } else {
                                                                ((if 1 as libc::c_int != 0 {
                                                                    0 as libc::c_int as libc::c_ulong
                                                                } else {
                                                                    alloc
                                                                }) as libc::c_ulonglong)
                                                                    .wrapping_add(
                                                                        (-(9223372036854775807 as libc::c_longlong)
                                                                            - 1 as libc::c_longlong) as libc::c_ulonglong,
                                                                    )
                                                            })
                                                                .wrapping_add(1 as libc::c_int as libc::c_ulonglong)
                                                                << (::std::mem::size_of::<libc::c_ulonglong>()
                                                                    as libc::c_ulong)
                                                                    .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                                                                    .wrapping_sub(2 as libc::c_int as libc::c_ulong))
                                                                .wrapping_sub(1 as libc::c_int as libc::c_ulonglong)
                                                                .wrapping_mul(2 as libc::c_int as libc::c_ulonglong)
                                                                .wrapping_add(1 as libc::c_int as libc::c_ulonglong)
                                                        } else {
                                                            (if 1 as libc::c_int != 0 {
                                                                0 as libc::c_int as libc::c_ulonglong
                                                            } else {
                                                                ((if 1 as libc::c_int != 0 {
                                                                    0 as libc::c_int as libc::c_ulong
                                                                } else {
                                                                    alloc
                                                                }) as libc::c_ulonglong)
                                                                    .wrapping_add(
                                                                        (-(9223372036854775807 as libc::c_longlong)
                                                                            - 1 as libc::c_longlong) as libc::c_ulonglong,
                                                                    )
                                                            })
                                                                .wrapping_sub(1 as libc::c_int as libc::c_ulonglong)
                                                        })
                                                            .wrapping_neg()) as libc::c_int
                                                } else {
                                                    ((0 as libc::c_int as libc::c_ulonglong)
                                                        < ((if 1 as libc::c_int != 0 {
                                                            0 as libc::c_int as libc::c_ulong
                                                        } else {
                                                            alloc
                                                        }) as libc::c_ulonglong)
                                                            .wrapping_add(
                                                                (-(9223372036854775807 as libc::c_longlong)
                                                                    - 1 as libc::c_longlong) as libc::c_ulonglong,
                                                            )) as libc::c_int
                                                }) != 0 && alloc == -(1 as libc::c_int) as libc::c_ulong
                                                {
                                                    if ((if 1 as libc::c_int != 0 {
                                                        0 as libc::c_int
                                                    } else {
                                                        7 as libc::c_int
                                                    }) - 1 as libc::c_int) < 0 as libc::c_int
                                                    {
                                                        ((0 as libc::c_int as libc::c_longlong)
                                                            < 7 as libc::c_int as libc::c_longlong
                                                                + (-(9223372036854775807 as libc::c_longlong)
                                                                    - 1 as libc::c_longlong)) as libc::c_int
                                                    } else {
                                                        (-(1 as libc::c_int) as libc::c_longlong
                                                            - (-(9223372036854775807 as libc::c_longlong)
                                                                - 1 as libc::c_longlong)
                                                            < (7 as libc::c_int - 1 as libc::c_int) as libc::c_longlong)
                                                            as libc::c_int
                                                    }
                                                } else {
                                                    (((-(9223372036854775807 as libc::c_longlong)
                                                        - 1 as libc::c_longlong) as libc::c_ulonglong)
                                                        .wrapping_div(alloc as libc::c_ulonglong)
                                                        < 7 as libc::c_int as libc::c_ulonglong) as libc::c_int
                                                }
                                            } else {
                                                (((9223372036854775807 as libc::c_longlong
                                                    / 7 as libc::c_int as libc::c_longlong)
                                                    as libc::c_ulonglong) < alloc as libc::c_ulonglong)
                                                    as libc::c_int
                                            }
                                        }
                                    }) != 0
                                    {
                                        alloc = (alloc as libc::c_ulonglong)
                                            .wrapping_mul(7 as libc::c_int as libc::c_ulonglong)
                                            as libc::c_longlong as size_t;
                                        1 as libc::c_int
                                    } else {
                                        alloc = (alloc as libc::c_ulonglong)
                                            .wrapping_mul(7 as libc::c_int as libc::c_ulonglong)
                                            as libc::c_longlong as size_t;
                                        0 as libc::c_int
                                    }
                                } else {
                                    if (if (7 as libc::c_int) < 0 as libc::c_int {
                                        if alloc < 0 as libc::c_int as libc::c_ulong {
                                            if (if 1 as libc::c_int != 0 {
                                                0 as libc::c_int as libc::c_ulonglong
                                            } else {
                                                (if 1 as libc::c_int != 0 {
                                                    0 as libc::c_int as libc::c_ulonglong
                                                } else {
                                                    (9223372036854775807 as libc::c_longlong
                                                        as libc::c_ulonglong)
                                                        .wrapping_mul(2 as libc::c_ulonglong)
                                                        .wrapping_add(1 as libc::c_ulonglong)
                                                })
                                                    .wrapping_add(7 as libc::c_int as libc::c_ulonglong)
                                            })
                                                .wrapping_sub(1 as libc::c_int as libc::c_ulonglong)
                                                < 0 as libc::c_int as libc::c_ulonglong
                                            {
                                                ((alloc as libc::c_ulonglong)
                                                    < (9223372036854775807 as libc::c_longlong
                                                        as libc::c_ulonglong)
                                                        .wrapping_mul(2 as libc::c_ulonglong)
                                                        .wrapping_add(1 as libc::c_ulonglong)
                                                        .wrapping_div(7 as libc::c_int as libc::c_ulonglong))
                                                    as libc::c_int
                                            } else {
                                                ((if (if (if ((if 1 as libc::c_int != 0 {
                                                    0 as libc::c_int
                                                } else {
                                                    7 as libc::c_int
                                                }) - 1 as libc::c_int) < 0 as libc::c_int
                                                {
                                                    !(((((if 1 as libc::c_int != 0 {
                                                        0 as libc::c_int
                                                    } else {
                                                        7 as libc::c_int
                                                    }) + 1 as libc::c_int)
                                                        << (::std::mem::size_of::<libc::c_int>() as libc::c_ulong)
                                                            .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                                                            .wrapping_sub(2 as libc::c_int as libc::c_ulong))
                                                        - 1 as libc::c_int) * 2 as libc::c_int + 1 as libc::c_int)
                                                } else {
                                                    (if 1 as libc::c_int != 0 {
                                                        0 as libc::c_int
                                                    } else {
                                                        7 as libc::c_int
                                                    }) + 0 as libc::c_int
                                                }) < 0 as libc::c_int
                                                {
                                                    ((7 as libc::c_int)
                                                        < -(if ((if 1 as libc::c_int != 0 {
                                                            0 as libc::c_int
                                                        } else {
                                                            7 as libc::c_int
                                                        }) - 1 as libc::c_int) < 0 as libc::c_int
                                                        {
                                                            ((((if 1 as libc::c_int != 0 {
                                                                0 as libc::c_int
                                                            } else {
                                                                7 as libc::c_int
                                                            }) + 1 as libc::c_int)
                                                                << (::std::mem::size_of::<libc::c_int>() as libc::c_ulong)
                                                                    .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                                                                    .wrapping_sub(2 as libc::c_int as libc::c_ulong))
                                                                - 1 as libc::c_int) * 2 as libc::c_int + 1 as libc::c_int
                                                        } else {
                                                            (if 1 as libc::c_int != 0 {
                                                                0 as libc::c_int
                                                            } else {
                                                                7 as libc::c_int
                                                            }) - 1 as libc::c_int
                                                        })) as libc::c_int
                                                } else {
                                                    ((0 as libc::c_int) < 7 as libc::c_int) as libc::c_int
                                                }) != 0
                                                {
                                                    ((if 1 as libc::c_int != 0 {
                                                        0 as libc::c_int
                                                    } else {
                                                        7 as libc::c_int
                                                    }) as libc::c_ulonglong)
                                                        .wrapping_add(
                                                            (9223372036854775807 as libc::c_longlong
                                                                as libc::c_ulonglong)
                                                                .wrapping_mul(2 as libc::c_ulonglong)
                                                                .wrapping_add(1 as libc::c_ulonglong),
                                                        )
                                                        >> (::std::mem::size_of::<libc::c_int>() as libc::c_ulong)
                                                            .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                                                            .wrapping_sub(1 as libc::c_int as libc::c_ulong)
                                                } else {
                                                    (9223372036854775807 as libc::c_longlong
                                                        as libc::c_ulonglong)
                                                        .wrapping_mul(2 as libc::c_ulonglong)
                                                        .wrapping_add(1 as libc::c_ulonglong)
                                                        .wrapping_div(-(7 as libc::c_int) as libc::c_ulonglong)
                                                })
                                                    <= (-(1 as libc::c_int) as libc::c_ulong)
                                                        .wrapping_sub(alloc) as libc::c_ulonglong) as libc::c_int
                                            }
                                        } else {
                                            if (if (if ((if 1 as libc::c_int != 0 {
                                                0 as libc::c_int
                                            } else {
                                                (if 1 as libc::c_int != 0 {
                                                    0 as libc::c_int
                                                } else {
                                                    7 as libc::c_int
                                                }) + 0 as libc::c_int
                                            }) - 1 as libc::c_int) < 0 as libc::c_int
                                            {
                                                !(((((if 1 as libc::c_int != 0 {
                                                    0 as libc::c_int
                                                } else {
                                                    (if 1 as libc::c_int != 0 {
                                                        0 as libc::c_int
                                                    } else {
                                                        7 as libc::c_int
                                                    }) + 0 as libc::c_int
                                                }) + 1 as libc::c_int)
                                                    << (::std::mem::size_of::<libc::c_int>() as libc::c_ulong)
                                                        .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                                                        .wrapping_sub(2 as libc::c_int as libc::c_ulong))
                                                    - 1 as libc::c_int) * 2 as libc::c_int + 1 as libc::c_int)
                                            } else {
                                                (if 1 as libc::c_int != 0 {
                                                    0 as libc::c_int
                                                } else {
                                                    (if 1 as libc::c_int != 0 {
                                                        0 as libc::c_int
                                                    } else {
                                                        7 as libc::c_int
                                                    }) + 0 as libc::c_int
                                                }) + 0 as libc::c_int
                                            }) < 0 as libc::c_int
                                            {
                                                (((if 1 as libc::c_int != 0 {
                                                    0 as libc::c_int
                                                } else {
                                                    7 as libc::c_int
                                                }) + 0 as libc::c_int)
                                                    < -(if ((if 1 as libc::c_int != 0 {
                                                        0 as libc::c_int
                                                    } else {
                                                        (if 1 as libc::c_int != 0 {
                                                            0 as libc::c_int
                                                        } else {
                                                            7 as libc::c_int
                                                        }) + 0 as libc::c_int
                                                    }) - 1 as libc::c_int) < 0 as libc::c_int
                                                    {
                                                        ((((if 1 as libc::c_int != 0 {
                                                            0 as libc::c_int
                                                        } else {
                                                            (if 1 as libc::c_int != 0 {
                                                                0 as libc::c_int
                                                            } else {
                                                                7 as libc::c_int
                                                            }) + 0 as libc::c_int
                                                        }) + 1 as libc::c_int)
                                                            << (::std::mem::size_of::<libc::c_int>() as libc::c_ulong)
                                                                .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                                                                .wrapping_sub(2 as libc::c_int as libc::c_ulong))
                                                            - 1 as libc::c_int) * 2 as libc::c_int + 1 as libc::c_int
                                                    } else {
                                                        (if 1 as libc::c_int != 0 {
                                                            0 as libc::c_int
                                                        } else {
                                                            (if 1 as libc::c_int != 0 {
                                                                0 as libc::c_int
                                                            } else {
                                                                7 as libc::c_int
                                                            }) + 0 as libc::c_int
                                                        }) - 1 as libc::c_int
                                                    })) as libc::c_int
                                            } else {
                                                ((0 as libc::c_int)
                                                    < (if 1 as libc::c_int != 0 {
                                                        0 as libc::c_int
                                                    } else {
                                                        7 as libc::c_int
                                                    }) + 0 as libc::c_int) as libc::c_int
                                            }) != 0 && 7 as libc::c_int == -(1 as libc::c_int)
                                            {
                                                if (if 1 as libc::c_int != 0 {
                                                    0 as libc::c_int as libc::c_ulong
                                                } else {
                                                    alloc
                                                })
                                                    .wrapping_sub(1 as libc::c_int as libc::c_ulong)
                                                    < 0 as libc::c_int as libc::c_ulong
                                                {
                                                    ((0 as libc::c_int as libc::c_ulong)
                                                        < alloc.wrapping_add(0 as libc::c_int as libc::c_ulong))
                                                        as libc::c_int
                                                } else {
                                                    ((0 as libc::c_int as libc::c_ulong) < alloc
                                                        && ((-(1 as libc::c_int) - 0 as libc::c_int)
                                                            as libc::c_ulong)
                                                            < alloc.wrapping_sub(1 as libc::c_int as libc::c_ulong))
                                                        as libc::c_int
                                                }
                                            } else {
                                                (((0 as libc::c_int / 7 as libc::c_int) as libc::c_ulong)
                                                    < alloc) as libc::c_int
                                            }
                                        }
                                    } else {
                                        if 7 as libc::c_int == 0 as libc::c_int {
                                            0 as libc::c_int
                                        } else {
                                            if alloc < 0 as libc::c_int as libc::c_ulong {
                                                if (if (if (if 1 as libc::c_int != 0 {
                                                    0 as libc::c_int as libc::c_ulong
                                                } else {
                                                    (if 1 as libc::c_int != 0 {
                                                        0 as libc::c_int as libc::c_ulong
                                                    } else {
                                                        alloc
                                                    })
                                                        .wrapping_add(0 as libc::c_int as libc::c_ulong)
                                                })
                                                    .wrapping_sub(1 as libc::c_int as libc::c_ulong)
                                                    < 0 as libc::c_int as libc::c_ulong
                                                {
                                                    !((if 1 as libc::c_int != 0 {
                                                        0 as libc::c_int as libc::c_ulong
                                                    } else {
                                                        (if 1 as libc::c_int != 0 {
                                                            0 as libc::c_int as libc::c_ulong
                                                        } else {
                                                            alloc
                                                        })
                                                            .wrapping_add(0 as libc::c_int as libc::c_ulong)
                                                    })
                                                        .wrapping_add(1 as libc::c_int as libc::c_ulong)
                                                        << (::std::mem::size_of::<libc::c_ulong>() as libc::c_ulong)
                                                            .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                                                            .wrapping_sub(2 as libc::c_int as libc::c_ulong))
                                                        .wrapping_sub(1 as libc::c_int as libc::c_ulong)
                                                        .wrapping_mul(2 as libc::c_int as libc::c_ulong)
                                                        .wrapping_add(1 as libc::c_int as libc::c_ulong)
                                                } else {
                                                    (if 1 as libc::c_int != 0 {
                                                        0 as libc::c_int as libc::c_ulong
                                                    } else {
                                                        (if 1 as libc::c_int != 0 {
                                                            0 as libc::c_int as libc::c_ulong
                                                        } else {
                                                            alloc
                                                        })
                                                            .wrapping_add(0 as libc::c_int as libc::c_ulong)
                                                    })
                                                        .wrapping_add(0 as libc::c_int as libc::c_ulong)
                                                }) < 0 as libc::c_int as libc::c_ulong
                                                {
                                                    ((if 1 as libc::c_int != 0 {
                                                        0 as libc::c_int as libc::c_ulong
                                                    } else {
                                                        alloc
                                                    })
                                                        .wrapping_add(0 as libc::c_int as libc::c_ulong)
                                                        < (if (if 1 as libc::c_int != 0 {
                                                            0 as libc::c_int as libc::c_ulong
                                                        } else {
                                                            (if 1 as libc::c_int != 0 {
                                                                0 as libc::c_int as libc::c_ulong
                                                            } else {
                                                                alloc
                                                            })
                                                                .wrapping_add(0 as libc::c_int as libc::c_ulong)
                                                        })
                                                            .wrapping_sub(1 as libc::c_int as libc::c_ulong)
                                                            < 0 as libc::c_int as libc::c_ulong
                                                        {
                                                            ((if 1 as libc::c_int != 0 {
                                                                0 as libc::c_int as libc::c_ulong
                                                            } else {
                                                                (if 1 as libc::c_int != 0 {
                                                                    0 as libc::c_int as libc::c_ulong
                                                                } else {
                                                                    alloc
                                                                })
                                                                    .wrapping_add(0 as libc::c_int as libc::c_ulong)
                                                            })
                                                                .wrapping_add(1 as libc::c_int as libc::c_ulong)
                                                                << (::std::mem::size_of::<libc::c_ulong>() as libc::c_ulong)
                                                                    .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                                                                    .wrapping_sub(2 as libc::c_int as libc::c_ulong))
                                                                .wrapping_sub(1 as libc::c_int as libc::c_ulong)
                                                                .wrapping_mul(2 as libc::c_int as libc::c_ulong)
                                                                .wrapping_add(1 as libc::c_int as libc::c_ulong)
                                                        } else {
                                                            (if 1 as libc::c_int != 0 {
                                                                0 as libc::c_int as libc::c_ulong
                                                            } else {
                                                                (if 1 as libc::c_int != 0 {
                                                                    0 as libc::c_int as libc::c_ulong
                                                                } else {
                                                                    alloc
                                                                })
                                                                    .wrapping_add(0 as libc::c_int as libc::c_ulong)
                                                            })
                                                                .wrapping_sub(1 as libc::c_int as libc::c_ulong)
                                                        })
                                                            .wrapping_neg()) as libc::c_int
                                                } else {
                                                    ((0 as libc::c_int as libc::c_ulong)
                                                        < (if 1 as libc::c_int != 0 {
                                                            0 as libc::c_int as libc::c_ulong
                                                        } else {
                                                            alloc
                                                        })
                                                            .wrapping_add(0 as libc::c_int as libc::c_ulong))
                                                        as libc::c_int
                                                }) != 0 && alloc == -(1 as libc::c_int) as libc::c_ulong
                                                {
                                                    if ((if 1 as libc::c_int != 0 {
                                                        0 as libc::c_int
                                                    } else {
                                                        7 as libc::c_int
                                                    }) - 1 as libc::c_int) < 0 as libc::c_int
                                                    {
                                                        ((0 as libc::c_int) < 7 as libc::c_int + 0 as libc::c_int)
                                                            as libc::c_int
                                                    } else {
                                                        ((-(1 as libc::c_int) - 0 as libc::c_int)
                                                            < 7 as libc::c_int - 1 as libc::c_int) as libc::c_int
                                                    }
                                                } else {
                                                    ((0 as libc::c_int as libc::c_ulong).wrapping_div(alloc)
                                                        < 7 as libc::c_int as libc::c_ulong) as libc::c_int
                                                }
                                            } else {
                                                ((9223372036854775807 as libc::c_longlong
                                                    as libc::c_ulonglong)
                                                    .wrapping_mul(2 as libc::c_ulonglong)
                                                    .wrapping_add(1 as libc::c_ulonglong)
                                                    .wrapping_div(7 as libc::c_int as libc::c_ulonglong)
                                                    < alloc as libc::c_ulonglong) as libc::c_int
                                            }
                                        }
                                    }) != 0
                                    {
                                        alloc = (alloc as libc::c_ulonglong)
                                            .wrapping_mul(7 as libc::c_int as libc::c_ulonglong)
                                            as size_t;
                                        1 as libc::c_int
                                    } else {
                                        alloc = (alloc as libc::c_ulonglong)
                                            .wrapping_mul(7 as libc::c_int as libc::c_ulonglong)
                                            as size_t;
                                        0 as libc::c_int
                                    }
                                }
                            }
                        }
                    }
                }) != 0
                    || {
                        let (fresh4, fresh5) = alloc
                            .overflowing_add(
                                (::std::mem::size_of::<[libc::c_char; 104]>()
                                    as libc::c_ulong)
                                    .wrapping_sub(7 as libc::c_int as libc::c_ulong),
                            );
                        *(&mut alloc as *mut size_t) = fresh4;
                        fresh5 as libc::c_int != 0
                    }
                {
                    xalloc_die();
                }
                let mut b: *mut libc::c_char = xmalloc(alloc) as *mut libc::c_char;
                let mut base: *mut libc::c_char = b;
                let mut changes: libc::c_int = 0 as libc::c_int;
                i = 0 as libc::c_int;
                while (i as libc::c_ulong)
                    < ::std::mem::size_of::<[libc::c_char; 104]>() as libc::c_ulong
                {
                    let mut ch: libc::c_char = C_ifdef_group_formats[i as usize];
                    match ch as libc::c_int {
                        64 => {
                            b = stpcpy(b, optarg);
                        }
                        0 => {
                            let fresh7 = b;
                            b = b.offset(1);
                            *fresh7 = ch;
                            let fresh8 = changes;
                            changes = changes + 1;
                            specify_value(
                                &mut *group_format.as_mut_ptr().offset(fresh8 as isize),
                                base,
                                b"-D\0" as *const u8 as *const libc::c_char,
                            );
                            base = b;
                        }
                        _ => {
                            let fresh6 = b;
                            b = b.offset(1);
                            *fresh6 = ch;
                        }
                    }
                    i += 1;
                    i;
                }
            }
            101 => {
                specify_style(OUTPUT_ED);
            }
            69 => {
                if (ignore_white_space as libc::c_uint)
                    < IGNORE_SPACE_CHANGE as libc::c_int as libc::c_uint
                {
                    ignore_white_space = ::std::mem::transmute::<
                        libc::c_uint,
                        DIFF_white_space,
                    >(
                        ignore_white_space as libc::c_uint
                            | IGNORE_TAB_EXPANSION as libc::c_int as libc::c_uint,
                    );
                }
            }
            102 => {
                specify_style(OUTPUT_FORWARD_ED);
            }
            70 => {
                add_regexp(&mut function_regexp_list, optarg);
            }
            104 => {}
            72 => {
                speed_large_files = 1 as libc::c_int != 0;
            }
            105 => {
                ignore_case = 1 as libc::c_int != 0;
            }
            73 => {
                add_regexp(&mut ignore_regexp_list, optarg);
            }
            108 => {
                if *pr_program.as_ptr().offset(0 as libc::c_int as isize) == 0 {
                    try_help(
                        b"pagination not supported on this host\0" as *const u8
                            as *const libc::c_char,
                        0 as *const libc::c_char,
                    );
                }
                paginate = 1 as libc::c_int != 0;
                signal(17 as libc::c_int, None);
            }
            76 => {
                if (file_label[0 as libc::c_int as usize]).is_null() {
                    file_label[0 as libc::c_int as usize] = optarg;
                } else if (file_label[1 as libc::c_int as usize]).is_null() {
                    file_label[1 as libc::c_int as usize] = optarg;
                } else {
                    fatal(
                        b"too many file label options\0" as *const u8
                            as *const libc::c_char,
                    );
                }
            }
            110 => {
                specify_style(OUTPUT_RCS);
            }
            78 => {
                new_file = 1 as libc::c_int != 0;
            }
            112 => {
                show_c_function = 1 as libc::c_int != 0;
                add_regexp(
                    &mut function_regexp_list,
                    b"^[[:alpha:]$_]\0" as *const u8 as *const libc::c_char,
                );
            }
            80 => {
                unidirectional_new_file = 1 as libc::c_int != 0;
            }
            113 => {
                brief = 1 as libc::c_int != 0;
            }
            114 => {
                recursive = 1 as libc::c_int != 0;
            }
            115 => {
                report_identical_files = 1 as libc::c_int != 0;
            }
            83 => {
                specify_value(
                    &mut starting_file,
                    optarg,
                    b"-S\0" as *const u8 as *const libc::c_char,
                );
            }
            116 => {
                expand_tabs = 1 as libc::c_int != 0;
            }
            84 => {
                initial_tab = 1 as libc::c_int != 0;
            }
            117 => {
                specify_style(OUTPUT_UNIFIED);
                if context < 3 as libc::c_int as libc::c_long {
                    context = 3 as libc::c_int as lin;
                }
            }
            118 => {
                version_etc(
                    stdout,
                    PROGRAM_NAME.as_ptr(),
                    b"GNU diffutils\0" as *const u8 as *const libc::c_char,
                    Version,
                    proper_name(b"Paul Eggert\0" as *const u8 as *const libc::c_char),
                    proper_name(b"Mike Haertel\0" as *const u8 as *const libc::c_char),
                    proper_name(b"David Hayes\0" as *const u8 as *const libc::c_char),
                    proper_name(
                        b"Richard Stallman\0" as *const u8 as *const libc::c_char,
                    ),
                    proper_name(b"Len Tower\0" as *const u8 as *const libc::c_char),
                    0 as *mut libc::c_void,
                );
                check_stdout();
                return 0 as libc::c_int;
            }
            119 => {
                ignore_white_space = IGNORE_ALL_SPACE;
            }
            120 => {
                add_exclude(excluded, optarg, exclude_options());
            }
            88 => {
                if add_exclude_file(
                    Some(
                        add_exclude
                            as unsafe extern "C" fn(
                                *mut exclude,
                                *const libc::c_char,
                                libc::c_int,
                            ) -> (),
                    ),
                    excluded,
                    optarg,
                    exclude_options(),
                    '\n' as i32 as libc::c_char,
                ) != 0
                {
                    pfatal_with_name(optarg);
                }
            }
            121 => {
                specify_style(OUTPUT_SDIFF);
            }
            87 => {
                numval = strtoimax(optarg, &mut numend, 10 as libc::c_int);
                if !((0 as libc::c_int as libc::c_long) < numval
                    && numval as libc::c_ulong <= 18446744073709551615 as libc::c_ulong)
                    || *numend as libc::c_int != 0
                {
                    try_help(
                        b"invalid width '%s'\0" as *const u8 as *const libc::c_char,
                        optarg,
                    );
                }
                if width != numval as libc::c_ulong {
                    if width != 0 {
                        fatal(
                            b"conflicting width options\0" as *const u8
                                as *const libc::c_char,
                        );
                    }
                    width = numval as size_t;
                }
            }
            129 => {
                specify_value(
                    &mut from_file,
                    optarg,
                    b"--from-file\0" as *const u8 as *const libc::c_char,
                );
            }
            130 => {
                usage();
                check_stdout();
                return 0 as libc::c_int;
            }
            131 => {
                numval = strtoimax(optarg, &mut numend, 10 as libc::c_int);
                if *numend as libc::c_int != 0
                    || numval < 0 as libc::c_int as libc::c_long
                {
                    try_help(
                        b"invalid horizon length '%s'\0" as *const u8
                            as *const libc::c_char,
                        optarg,
                    );
                }
                horizon_lines = if horizon_lines
                    >= (if numval <= 9223372036854775807 as libc::c_long {
                        numval
                    } else {
                        9223372036854775807 as libc::c_long
                    })
                {
                    horizon_lines
                } else if numval <= 9223372036854775807 as libc::c_long {
                    numval
                } else {
                    9223372036854775807 as libc::c_long
                };
            }
            132 => {
                ignore_file_name_case = 1 as libc::c_int != 0;
            }
            0 | 128 | 133 => {}
            134 => {
                left_column = 1 as libc::c_int != 0;
            }
            135 => {
                specify_style(OUTPUT_IFDEF);
                i = 0 as libc::c_int;
                while (i as libc::c_ulong)
                    < (::std::mem::size_of::<[*const libc::c_char; 3]>()
                        as libc::c_ulong)
                        .wrapping_div(
                            ::std::mem::size_of::<*const libc::c_char>() as libc::c_ulong,
                        )
                {
                    specify_value(
                        &mut *line_format.as_mut_ptr().offset(i as isize),
                        optarg,
                        b"--line-format\0" as *const u8 as *const libc::c_char,
                    );
                    i += 1;
                    i;
                }
            }
            136 => {
                no_dereference_symlinks = 1 as libc::c_int != 0;
            }
            137 => {
                ignore_file_name_case = 0 as libc::c_int != 0;
            }
            138 => {
                specify_style(OUTPUT_NORMAL);
            }
            139 => {
                specify_style(OUTPUT_SDIFF);
                sdiff_merge_assist = 1 as libc::c_int != 0;
            }
            140 => {
                strip_trailing_cr = 1 as libc::c_int != 0;
            }
            141 => {
                suppress_blank_empty = 1 as libc::c_int != 0;
            }
            142 => {
                suppress_common_lines = 1 as libc::c_int != 0;
            }
            143 => {
                numval = strtoimax(optarg, &mut numend, 10 as libc::c_int);
                if !((0 as libc::c_int as libc::c_long) < numval
                    && numval as libc::c_ulong
                        <= (18446744073709551615 as libc::c_ulong)
                            .wrapping_sub(3 as libc::c_int as libc::c_ulong))
                    || *numend as libc::c_int != 0
                {
                    try_help(
                        b"invalid tabsize '%s'\0" as *const u8 as *const libc::c_char,
                        optarg,
                    );
                }
                if tabsize != numval as libc::c_ulong {
                    if tabsize != 0 {
                        fatal(
                            b"conflicting tabsize options\0" as *const u8
                                as *const libc::c_char,
                        );
                    }
                    tabsize = numval as size_t;
                }
            }
            144 => {
                specify_value(
                    &mut to_file,
                    optarg,
                    b"--to-file\0" as *const u8 as *const libc::c_char,
                );
            }
            145 | 146 | 147 => {
                specify_style(OUTPUT_IFDEF);
                c -= UNCHANGED_LINE_FORMAT_OPTION as libc::c_int;
                specify_value(
                    &mut *line_format.as_mut_ptr().offset(c as isize),
                    optarg,
                    (line_format_option[c as usize]).as_ptr(),
                );
            }
            148 | 149 | 150 | 151 => {
                specify_style(OUTPUT_IFDEF);
                c -= UNCHANGED_GROUP_FORMAT_OPTION as libc::c_int;
                specify_value(
                    &mut *group_format.as_mut_ptr().offset(c as isize),
                    optarg,
                    (group_format_option[c as usize]).as_ptr(),
                );
            }
            152 => {
                specify_colors_style(optarg);
            }
            153 => {
                set_color_palette(optarg);
            }
            154 => {
                no_directory = 1 as libc::c_int != 0;
            }
            155 => {
                presume_output_tty = 1 as libc::c_int != 0;
            }
            _ => {
                try_help(0 as *const libc::c_char, 0 as *const libc::c_char);
            }
        }
        prev = c;
    }
    if colors_style as libc::c_uint == AUTO as libc::c_int as libc::c_uint {
        let mut t: *const libc::c_char = getenv(
            b"TERM\0" as *const u8 as *const libc::c_char,
        );
        if !t.is_null()
            && strcmp(t, b"dumb\0" as *const u8 as *const libc::c_char)
                == 0 as libc::c_int
        {
            colors_style = NEVER;
        }
    }
    if output_style as libc::c_uint == OUTPUT_UNSPECIFIED as libc::c_int as libc::c_uint
    {
        if show_c_function {
            specify_style(OUTPUT_CONTEXT);
            if ocontext < 0 as libc::c_int as libc::c_long {
                context = 3 as libc::c_int as lin;
            }
        } else {
            specify_style(OUTPUT_NORMAL);
        }
    }
    if output_style as libc::c_uint != OUTPUT_CONTEXT as libc::c_int as libc::c_uint
        || hard_locale(2 as libc::c_int) as libc::c_int != 0
    {
        time_format = b"%Y-%m-%d %H:%M:%S.%N %z\0" as *const u8 as *const libc::c_char;
    } else {
        time_format = b"%a %b %e %T %Y\0" as *const u8 as *const libc::c_char;
    }
    if 0 as libc::c_int as libc::c_long <= ocontext
        && (output_style as libc::c_uint == OUTPUT_CONTEXT as libc::c_int as libc::c_uint
            || output_style as libc::c_uint
                == OUTPUT_UNIFIED as libc::c_int as libc::c_uint)
        && (context < ocontext || ocontext < context && !explicit_context)
    {
        context = ocontext;
    }
    if tabsize == 0 {
        tabsize = 8 as libc::c_int as size_t;
    }
    if width == 0 {
        width = 130 as libc::c_int as size_t;
    }
    let mut t_0: size_t = if expand_tabs as libc::c_int != 0 {
        1 as libc::c_int as libc::c_ulong
    } else {
        tabsize
    };
    let mut w: size_t = width;
    let mut t_plus_g: size_t = t_0.wrapping_add(3 as libc::c_int as libc::c_ulong);
    let mut unaligned_off: size_t = (w >> 1 as libc::c_int)
        .wrapping_add(t_plus_g >> 1 as libc::c_int)
        .wrapping_add(w & t_plus_g & 1 as libc::c_int as libc::c_ulong);
    let mut off: size_t = unaligned_off.wrapping_sub(unaligned_off.wrapping_rem(t_0));
    sdiff_half_width = if off <= 3 as libc::c_int as libc::c_ulong || w <= off {
        0 as libc::c_int as libc::c_ulong
    } else if off.wrapping_sub(3 as libc::c_int as libc::c_ulong) <= w.wrapping_sub(off)
    {
        off.wrapping_sub(3 as libc::c_int as libc::c_ulong)
    } else {
        w.wrapping_sub(off)
    };
    sdiff_column2_offset = if sdiff_half_width != 0 { off } else { w };
    if horizon_lines < context {
        horizon_lines = context;
    }
    summarize_regexp_list(&mut function_regexp_list);
    summarize_regexp_list(&mut ignore_regexp_list);
    if output_style as libc::c_uint == OUTPUT_IFDEF as libc::c_int as libc::c_uint {
        i = 0 as libc::c_int;
        while (i as libc::c_ulong)
            < (::std::mem::size_of::<[*const libc::c_char; 3]>() as libc::c_ulong)
                .wrapping_div(
                    ::std::mem::size_of::<*const libc::c_char>() as libc::c_ulong,
                )
        {
            if (line_format[i as usize]).is_null() {
                line_format[i as usize] = b"%l\n\0" as *const u8 as *const libc::c_char;
            }
            i += 1;
            i;
        }
        if (group_format[OLD as libc::c_int as usize]).is_null() {
            group_format[OLD as libc::c_int
                as usize] = if !(group_format[CHANGED as libc::c_int as usize]).is_null()
            {
                group_format[CHANGED as libc::c_int as usize]
            } else {
                b"%<\0" as *const u8 as *const libc::c_char
            };
        }
        if (group_format[NEW as libc::c_int as usize]).is_null() {
            group_format[NEW as libc::c_int
                as usize] = if !(group_format[CHANGED as libc::c_int as usize]).is_null()
            {
                group_format[CHANGED as libc::c_int as usize]
            } else {
                b"%>\0" as *const u8 as *const libc::c_char
            };
        }
        if (group_format[UNCHANGED as libc::c_int as usize]).is_null() {
            group_format[UNCHANGED as libc::c_int
                as usize] = b"%=\0" as *const u8 as *const libc::c_char;
        }
        if (group_format[CHANGED as libc::c_int as usize]).is_null() {
            let mut p: *mut libc::c_char = xmalloc(
                (strlen(group_format[OLD as libc::c_int as usize]))
                    .wrapping_add(strlen(group_format[NEW as libc::c_int as usize]))
                    .wrapping_add(1 as libc::c_int as libc::c_ulong),
            ) as *mut libc::c_char;
            group_format[CHANGED as libc::c_int as usize] = p;
            strcpy(
                stpcpy(p, group_format[OLD as libc::c_int as usize]),
                group_format[NEW as libc::c_int as usize],
            );
        }
    }
    no_diff_means_no_output = if output_style as libc::c_uint
        == OUTPUT_IFDEF as libc::c_int as libc::c_uint
    {
        (*group_format[UNCHANGED as libc::c_int as usize] == 0
            || strcmp(
                group_format[UNCHANGED as libc::c_int as usize],
                b"%=\0" as *const u8 as *const libc::c_char,
            ) == 0 as libc::c_int
                && *line_format[UNCHANGED as libc::c_int as usize] == 0) as libc::c_int
    } else {
        (output_style as libc::c_uint != OUTPUT_SDIFF as libc::c_int as libc::c_uint)
            as libc::c_int | suppress_common_lines as libc::c_int
    } != 0;
    files_can_be_treated_as_binary = brief as libc::c_int & binary as libc::c_int
        & !(ignore_blank_lines as libc::c_int | ignore_case as libc::c_int
            | strip_trailing_cr as libc::c_int
            | (!(ignore_regexp_list.regexps).is_null()
                || ignore_white_space as libc::c_uint != 0) as libc::c_int) != 0;
    switch_string = option_list(
        argv.offset(1 as libc::c_int as isize),
        optind - 1 as libc::c_int,
    );
    if !from_file.is_null() {
        if !to_file.is_null() {
            fatal(
                b"--from-file and --to-file both specified\0" as *const u8
                    as *const libc::c_char,
            );
        } else {
            while optind < argc {
                let mut status: libc::c_int = compare_files(
                    0 as *const comparison,
                    from_file,
                    *argv.offset(optind as isize),
                );
                if exit_status < status {
                    exit_status = status;
                }
                optind += 1;
                optind;
            }
        }
    } else if !to_file.is_null() {
        while optind < argc {
            let mut status_0: libc::c_int = compare_files(
                0 as *const comparison,
                *argv.offset(optind as isize),
                to_file,
            );
            if exit_status < status_0 {
                exit_status = status_0;
            }
            optind += 1;
            optind;
        }
    } else {
        if argc - optind != 2 as libc::c_int {
            if argc - optind < 2 as libc::c_int {
                try_help(
                    b"missing operand after '%s'\0" as *const u8 as *const libc::c_char,
                    *argv.offset((argc - 1 as libc::c_int) as isize),
                );
            } else {
                try_help(
                    b"extra operand '%s'\0" as *const u8 as *const libc::c_char,
                    *argv.offset((optind + 2 as libc::c_int) as isize),
                );
            }
        }
        exit_status = compare_files(
            0 as *const comparison,
            *argv.offset(optind as isize),
            *argv.offset((optind + 1 as libc::c_int) as isize),
        );
    }
    print_message_queue();
    check_stdout();
    cleanup_signal_handlers();
    return exit_status;
}
unsafe extern "C" fn add_regexp(
    mut reglist: *mut regexp_list,
    mut pattern: *const libc::c_char,
) {
    let mut patlen: size_t = strlen(pattern);
    let mut m: *const libc::c_char = rpl_re_compile_pattern(
        pattern,
        patlen,
        (*reglist).buf,
    );
    if !m.is_null() {
        error(
            2 as libc::c_int,
            0 as libc::c_int,
            b"%s: %s\0" as *const u8 as *const libc::c_char,
            pattern,
            m,
        );
    } else {
        let mut regexps: *mut libc::c_char = (*reglist).regexps;
        let mut len: size_t = (*reglist).len;
        (*reglist).multiple_regexps = !regexps.is_null();
        let mut multiple_regexps: bool = (*reglist).multiple_regexps;
        (*reglist)
            .len = len
            .wrapping_add(
                (2 as libc::c_int * multiple_regexps as libc::c_int) as libc::c_ulong,
            )
            .wrapping_add(patlen);
        let mut newlen: size_t = (*reglist).len;
        let mut size: size_t = (*reglist).size;
        if size <= newlen {
            if size == 0 {
                size = 1 as libc::c_int as size_t;
            }
            loop {
                size = (size as libc::c_ulong)
                    .wrapping_mul(2 as libc::c_int as libc::c_ulong) as size_t as size_t;
                if !(size <= newlen) {
                    break;
                }
            }
            (*reglist).size = size;
            regexps = xrealloc(regexps as *mut libc::c_void, size) as *mut libc::c_char;
            (*reglist).regexps = regexps;
        }
        if multiple_regexps {
            let fresh9 = len;
            len = len.wrapping_add(1);
            *regexps.offset(fresh9 as isize) = '\\' as i32 as libc::c_char;
            let fresh10 = len;
            len = len.wrapping_add(1);
            *regexps.offset(fresh10 as isize) = '|' as i32 as libc::c_char;
        }
        memcpy(
            regexps.offset(len as isize) as *mut libc::c_void,
            pattern as *const libc::c_void,
            patlen.wrapping_add(1 as libc::c_int as libc::c_ulong),
        );
    };
}
unsafe extern "C" fn summarize_regexp_list(mut reglist: *mut regexp_list) {
    if !((*reglist).regexps).is_null() {
        (*(*reglist).buf)
            .fastmap = xmalloc(((1 as libc::c_int) << 8 as libc::c_int) as size_t)
            as *mut libc::c_char;
        if (*reglist).multiple_regexps {
            let mut m: *const libc::c_char = rpl_re_compile_pattern(
                (*reglist).regexps,
                (*reglist).len,
                (*reglist).buf,
            );
            if !m.is_null() {
                if ::std::mem::size_of::<C2RustUnnamed>() as libc::c_ulong != 0 {
                    error(
                        2 as libc::c_int,
                        0 as libc::c_int,
                        b"%s: %s\0" as *const u8 as *const libc::c_char,
                        (*reglist).regexps,
                        m,
                    );
                    if 0 as libc::c_int != 0 {} else {
                        unreachable!();
                    };
                } else {
                    error(
                        2 as libc::c_int,
                        0 as libc::c_int,
                        b"%s: %s\0" as *const u8 as *const libc::c_char,
                        (*reglist).regexps,
                        m,
                    );
                    if 0 as libc::c_int != 0 {} else {
                        unreachable!();
                    };
                };
            }
        }
    }
}
unsafe extern "C" fn try_help(
    mut reason_msgid: *const libc::c_char,
    mut operand: *const libc::c_char,
) {
    if !reason_msgid.is_null() {
        error(
            0 as libc::c_int,
            0 as libc::c_int,
            dcgettext(0 as *const libc::c_char, reason_msgid, 5 as libc::c_int),
            operand,
        );
    }
    if ::std::mem::size_of::<C2RustUnnamed_0>() as libc::c_ulong != 0 {
        error(
            2 as libc::c_int,
            0 as libc::c_int,
            dcgettext(
                0 as *const libc::c_char,
                b"Try '%s --help' for more information.\0" as *const u8
                    as *const libc::c_char,
                5 as libc::c_int,
            ),
            program_name,
        );
        if 0 as libc::c_int != 0 {} else {
            unreachable!();
        };
    } else {
        error(
            2 as libc::c_int,
            0 as libc::c_int,
            dcgettext(
                0 as *const libc::c_char,
                b"Try '%s --help' for more information.\0" as *const u8
                    as *const libc::c_char,
                5 as libc::c_int,
            ),
            program_name,
        );
        if 0 as libc::c_int != 0 {} else {
            unreachable!();
        };
    };
}
unsafe extern "C" fn check_stdout() {
    if ferror_unlocked(stdout) != 0 {
        fatal(b"write failed\0" as *const u8 as *const libc::c_char);
    } else if fclose(stdout) != 0 as libc::c_int {
        pfatal_with_name(
            dcgettext(
                0 as *const libc::c_char,
                b"standard output\0" as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ),
        );
    }
}
static mut option_help_msgid: [*const libc::c_char; 69] = [
    b"    --normal                  output a normal diff (the default)\0" as *const u8
        as *const libc::c_char,
    b"-q, --brief                   report only when files differ\0" as *const u8
        as *const libc::c_char,
    b"-s, --report-identical-files  report when two files are the same\0" as *const u8
        as *const libc::c_char,
    b"-c, -C NUM, --context[=NUM]   output NUM (default 3) lines of copied context\0"
        as *const u8 as *const libc::c_char,
    b"-u, -U NUM, --unified[=NUM]   output NUM (default 3) lines of unified context\0"
        as *const u8 as *const libc::c_char,
    b"-e, --ed                      output an ed script\0" as *const u8
        as *const libc::c_char,
    b"-n, --rcs                     output an RCS format diff\0" as *const u8
        as *const libc::c_char,
    b"-y, --side-by-side            output in two columns\0" as *const u8
        as *const libc::c_char,
    b"-W, --width=NUM               output at most NUM (default 130) print columns\0"
        as *const u8 as *const libc::c_char,
    b"    --left-column             output only the left column of common lines\0"
        as *const u8 as *const libc::c_char,
    b"    --suppress-common-lines   do not output common lines\0" as *const u8
        as *const libc::c_char,
    b"\0" as *const u8 as *const libc::c_char,
    b"-p, --show-c-function         show which C function each change is in\0"
        as *const u8 as *const libc::c_char,
    b"-F, --show-function-line=RE   show the most recent line matching RE\0" as *const u8
        as *const libc::c_char,
    b"    --label LABEL             use LABEL instead of file name and timestamp\n                                (can be repeated)\0"
        as *const u8 as *const libc::c_char,
    b"\0" as *const u8 as *const libc::c_char,
    b"-t, --expand-tabs             expand tabs to spaces in output\0" as *const u8
        as *const libc::c_char,
    b"-T, --initial-tab             make tabs line up by prepending a tab\0" as *const u8
        as *const libc::c_char,
    b"    --tabsize=NUM             tab stops every NUM (default 8) print columns\0"
        as *const u8 as *const libc::c_char,
    b"    --suppress-blank-empty    suppress space or tab before empty output lines\0"
        as *const u8 as *const libc::c_char,
    b"-l, --paginate                pass output through 'pr' to paginate it\0"
        as *const u8 as *const libc::c_char,
    b"\0" as *const u8 as *const libc::c_char,
    b"-r, --recursive                 recursively compare any subdirectories found\0"
        as *const u8 as *const libc::c_char,
    b"    --no-dereference            don't follow symbolic links\0" as *const u8
        as *const libc::c_char,
    b"-N, --new-file                  treat absent files as empty\0" as *const u8
        as *const libc::c_char,
    b"    --unidirectional-new-file   treat absent first files as empty\0" as *const u8
        as *const libc::c_char,
    b"    --ignore-file-name-case     ignore case when comparing file names\0"
        as *const u8 as *const libc::c_char,
    b"    --no-ignore-file-name-case  consider case when comparing file names\0"
        as *const u8 as *const libc::c_char,
    b"-x, --exclude=PAT               exclude files that match PAT\0" as *const u8
        as *const libc::c_char,
    b"-X, --exclude-from=FILE         exclude files that match any pattern in FILE\0"
        as *const u8 as *const libc::c_char,
    b"-S, --starting-file=FILE        start with FILE when comparing directories\0"
        as *const u8 as *const libc::c_char,
    b"    --from-file=FILE1           compare FILE1 to all operands;\n                                  FILE1 can be a directory\0"
        as *const u8 as *const libc::c_char,
    b"    --to-file=FILE2             compare all operands to FILE2;\n                                  FILE2 can be a directory\0"
        as *const u8 as *const libc::c_char,
    b"\0" as *const u8 as *const libc::c_char,
    b"-i, --ignore-case               ignore case differences in file contents\0"
        as *const u8 as *const libc::c_char,
    b"-E, --ignore-tab-expansion      ignore changes due to tab expansion\0" as *const u8
        as *const libc::c_char,
    b"-Z, --ignore-trailing-space     ignore white space at line end\0" as *const u8
        as *const libc::c_char,
    b"-b, --ignore-space-change       ignore changes in the amount of white space\0"
        as *const u8 as *const libc::c_char,
    b"-w, --ignore-all-space          ignore all white space\0" as *const u8
        as *const libc::c_char,
    b"-B, --ignore-blank-lines        ignore changes where lines are all blank\0"
        as *const u8 as *const libc::c_char,
    b"-I, --ignore-matching-lines=RE  ignore changes where all lines match RE\0"
        as *const u8 as *const libc::c_char,
    b"\0" as *const u8 as *const libc::c_char,
    b"-a, --text                      treat all files as text\0" as *const u8
        as *const libc::c_char,
    b"    --strip-trailing-cr         strip trailing carriage return on input\0"
        as *const u8 as *const libc::c_char,
    b"\0" as *const u8 as *const libc::c_char,
    b"-D, --ifdef=NAME                output merged file with '#ifdef NAME' diffs\0"
        as *const u8 as *const libc::c_char,
    b"    --GTYPE-group-format=GFMT   format GTYPE input groups with GFMT\0" as *const u8
        as *const libc::c_char,
    b"    --line-format=LFMT          format all input lines with LFMT\0" as *const u8
        as *const libc::c_char,
    b"    --LTYPE-line-format=LFMT    format LTYPE input lines with LFMT\0" as *const u8
        as *const libc::c_char,
    b"  These format options provide fine-grained control over the output\n    of diff, generalizing -D/--ifdef.\0"
        as *const u8 as *const libc::c_char,
    b"  LTYPE is 'old', 'new', or 'unchanged'.  GTYPE is LTYPE or 'changed'.\0"
        as *const u8 as *const libc::c_char,
    b"  GFMT (only) may contain:\n    %<  lines from FILE1\n    %>  lines from FILE2\n    %=  lines common to FILE1 and FILE2\n    %[-][WIDTH][.[PREC]]{doxX}LETTER  printf-style spec for LETTER\n      LETTERs are as follows for new group, lower case for old group:\n        F  first line number\n        L  last line number\n        N  number of lines = L-F+1\n        E  F-1\n        M  L+1\n    %(A=B?T:E)  if A equals B then T else E\0"
        as *const u8 as *const libc::c_char,
    b"  LFMT (only) may contain:\n    %L  contents of line\n    %l  contents of line, excluding any trailing newline\n    %[-][WIDTH][.[PREC]]{doxX}n  printf-style spec for input line number\0"
        as *const u8 as *const libc::c_char,
    b"  Both GFMT and LFMT may contain:\n    %%  %\n    %c'C'  the single character C\n    %c'\\OOO'  the character with octal code OOO\n    C    the character C (other characters represent themselves)\0"
        as *const u8 as *const libc::c_char,
    b"\0" as *const u8 as *const libc::c_char,
    b"-d, --minimal            try hard to find a smaller set of changes\0" as *const u8
        as *const libc::c_char,
    b"    --horizon-lines=NUM  keep NUM lines of the common prefix and suffix\0"
        as *const u8 as *const libc::c_char,
    b"    --speed-large-files  assume large files and many scattered small changes\0"
        as *const u8 as *const libc::c_char,
    b"    --color[=WHEN]       color output; WHEN is 'never', 'always', or 'auto';\n                           plain --color means --color='auto'\0"
        as *const u8 as *const libc::c_char,
    b"    --palette=PALETTE    the colors to use when --color is active; PALETTE is\n                           a colon-separated list of terminfo capabilities\0"
        as *const u8 as *const libc::c_char,
    b"\0" as *const u8 as *const libc::c_char,
    b"    --help               display this help and exit\0" as *const u8
        as *const libc::c_char,
    b"-v, --version            output version information and exit\0" as *const u8
        as *const libc::c_char,
    b"\0" as *const u8 as *const libc::c_char,
    b"FILES are 'FILE1 FILE2' or 'DIR1 DIR2' or 'DIR FILE' or 'FILE DIR'.\0" as *const u8
        as *const libc::c_char,
    b"If --from-file or --to-file is given, there are no restrictions on FILE(s).\0"
        as *const u8 as *const libc::c_char,
    b"If a FILE is '-', read standard input.\0" as *const u8 as *const libc::c_char,
    b"Exit status is 0 if inputs are the same, 1 if different, 2 if trouble.\0"
        as *const u8 as *const libc::c_char,
    0 as *const libc::c_char,
];
unsafe extern "C" fn usage() {
    let mut p: *const *const libc::c_char = 0 as *const *const libc::c_char;
    printf(
        dcgettext(
            0 as *const libc::c_char,
            b"Usage: %s [OPTION]... FILES\n\0" as *const u8 as *const libc::c_char,
            5 as libc::c_int,
        ),
        program_name,
    );
    printf(
        b"%s\n\n\0" as *const u8 as *const libc::c_char,
        dcgettext(
            0 as *const libc::c_char,
            b"Compare FILES line by line.\0" as *const u8 as *const libc::c_char,
            5 as libc::c_int,
        ),
    );
    fputs_unlocked(
        dcgettext(
            0 as *const libc::c_char,
            b"Mandatory arguments to long options are mandatory for short options too.\n\0"
                as *const u8 as *const libc::c_char,
            5 as libc::c_int,
        ),
        stdout,
    );
    p = option_help_msgid.as_ptr();
    while !(*p).is_null() {
        if **p == 0 {
            putchar_unlocked('\n' as i32);
        } else {
            let mut msg: *const libc::c_char = dcgettext(
                0 as *const libc::c_char,
                *p,
                5 as libc::c_int,
            );
            let mut nl: *const libc::c_char = 0 as *const libc::c_char;
            loop {
                nl = strchr(msg, '\n' as i32);
                if nl.is_null() {
                    break;
                }
                let mut msglen: libc::c_int = nl
                    .offset(1 as libc::c_int as isize)
                    .offset_from(msg) as libc::c_long as libc::c_int;
                if msglen < 4096 as libc::c_int {} else {
                    __assert_fail(
                        b"msglen < 4096\0" as *const u8 as *const libc::c_char,
                        b"diff.c\0" as *const u8 as *const libc::c_char,
                        1076 as libc::c_int as libc::c_uint,
                        (*::std::mem::transmute::<
                            &[u8; 17],
                            &[libc::c_char; 17],
                        >(b"void usage(void)\0"))
                            .as_ptr(),
                    );
                }
                'c_10267: {
                    if msglen < 4096 as libc::c_int {} else {
                        __assert_fail(
                            b"msglen < 4096\0" as *const u8 as *const libc::c_char,
                            b"diff.c\0" as *const u8 as *const libc::c_char,
                            1076 as libc::c_int as libc::c_uint,
                            (*::std::mem::transmute::<
                                &[u8; 17],
                                &[libc::c_char; 17],
                            >(b"void usage(void)\0"))
                                .as_ptr(),
                        );
                    }
                };
                printf(b"  %.*s\0" as *const u8 as *const libc::c_char, msglen, msg);
                msg = nl.offset(1 as libc::c_int as isize);
            }
            printf(
                &*(b"  %s\n\0" as *const u8 as *const libc::c_char)
                    .offset(
                        (2 as libc::c_int
                            * (*msg as libc::c_int != ' ' as i32
                                && *msg as libc::c_int != '-' as i32) as libc::c_int)
                            as isize,
                    ) as *const libc::c_char,
                msg,
            );
        }
        p = p.offset(1);
        p;
    }
    emit_bug_reporting_address();
}
unsafe extern "C" fn specify_value(
    mut var: *mut *const libc::c_char,
    mut value: *const libc::c_char,
    mut option: *const libc::c_char,
) {
    if !(*var).is_null() && !(strcmp(*var, value) == 0 as libc::c_int) {
        error(
            0 as libc::c_int,
            0 as libc::c_int,
            dcgettext(
                0 as *const libc::c_char,
                b"conflicting %s option value '%s'\0" as *const u8
                    as *const libc::c_char,
                5 as libc::c_int,
            ),
            option,
            value,
        );
        try_help(0 as *const libc::c_char, 0 as *const libc::c_char);
    }
    *var = value;
}
unsafe extern "C" fn specify_style(mut style: output_style) {
    if output_style as libc::c_uint != style as libc::c_uint {
        if output_style as libc::c_uint
            != OUTPUT_UNSPECIFIED as libc::c_int as libc::c_uint
        {
            try_help(
                b"conflicting output style options\0" as *const u8
                    as *const libc::c_char,
                0 as *const libc::c_char,
            );
        }
        output_style = style;
    }
}
unsafe extern "C" fn specify_colors_style(mut value: *const libc::c_char) {
    if value.is_null()
        || strcmp(value, b"auto\0" as *const u8 as *const libc::c_char)
            == 0 as libc::c_int
    {
        colors_style = AUTO;
    } else if strcmp(value, b"always\0" as *const u8 as *const libc::c_char)
        == 0 as libc::c_int
    {
        colors_style = ALWAYS;
    } else if strcmp(value, b"never\0" as *const u8 as *const libc::c_char)
        == 0 as libc::c_int
    {
        colors_style = NEVER;
    } else {
        try_help(b"invalid color '%s'\0" as *const u8 as *const libc::c_char, value);
    };
}
unsafe extern "C" fn set_mtime_to_now(mut st: *mut stat) {
    gettime(&mut (*st).st_mtim);
}
unsafe extern "C" fn errno_encode(mut err: libc::c_int) -> libc::c_int {
    return -(3 as libc::c_int) - err;
}
unsafe extern "C" fn errno_decode(mut desc: libc::c_int) -> libc::c_int {
    return -(3 as libc::c_int) - desc;
}
unsafe extern "C" fn compare_files(
    mut parent: *const comparison,
    mut name0: *const libc::c_char,
    mut name1: *const libc::c_char,
) -> libc::c_int {
    let mut cmp: comparison = comparison {
        file: [file_data {
            desc: 0,
            name: 0 as *const libc::c_char,
            stat: stat {
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
            },
            buffer: 0 as *const size_t as *mut size_t,
            bufsize: 0,
            buffered: 0,
            linbuf: 0 as *const *const libc::c_char as *mut *const libc::c_char,
            linbuf_base: 0,
            buffered_lines: 0,
            valid_lines: 0,
            alloc_lines: 0,
            prefix_end: 0 as *const libc::c_char,
            prefix_lines: 0,
            suffix_begin: 0 as *const libc::c_char,
            equivs: 0 as *const lin as *mut lin,
            undiscarded: 0 as *const lin as *mut lin,
            realindexes: 0 as *const lin as *mut lin,
            nondiscarded_lines: 0,
            changed: 0 as *const libc::c_char as *mut libc::c_char,
            missing_newline: false,
            eof: false,
            equiv_max: 0,
        }; 2],
        parent: 0 as *const comparison,
    };
    let mut f: libc::c_int = 0;
    let mut status: libc::c_int = 0 as libc::c_int;
    let mut same_files: bool = false;
    let mut free0: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut free1: *mut libc::c_char = 0 as *mut libc::c_char;
    if !(!name0.is_null() && !name1.is_null()
        || unidirectional_new_file as libc::c_int != 0 && !name1.is_null()
        || new_file as libc::c_int != 0)
    {
        let mut name: *const libc::c_char = if !name0.is_null() { name0 } else { name1 };
        let mut dir: *const libc::c_char = (*parent)
            .file[name0.is_null() as libc::c_int as usize]
            .name;
        message(b"Only in %s: %s\n\0" as *const u8 as *const libc::c_char, dir, name);
        return 1 as libc::c_int;
    }
    memset(
        (cmp.file).as_mut_ptr() as *mut libc::c_void,
        0 as libc::c_int,
        ::std::mem::size_of::<[file_data; 2]>() as libc::c_ulong,
    );
    cmp.parent = parent;
    cmp
        .file[0 as libc::c_int as usize]
        .desc = if !name0.is_null() {
        UNOPENED as libc::c_int
    } else {
        NONEXISTENT as libc::c_int
    };
    cmp
        .file[1 as libc::c_int as usize]
        .desc = if !name1.is_null() {
        UNOPENED as libc::c_int
    } else {
        NONEXISTENT as libc::c_int
    };
    if name0.is_null() {
        name0 = name1;
    }
    if name1.is_null() {
        name1 = name0;
    }
    if parent.is_null() {
        free0 = 0 as *mut libc::c_char;
        free1 = 0 as *mut libc::c_char;
        cmp.file[0 as libc::c_int as usize].name = name0;
        cmp.file[1 as libc::c_int as usize].name = name1;
    } else {
        free0 = file_name_concat(
            (*parent).file[0 as libc::c_int as usize].name,
            name0,
            0 as *mut *mut libc::c_char,
        );
        cmp.file[0 as libc::c_int as usize].name = free0;
        free1 = file_name_concat(
            (*parent).file[1 as libc::c_int as usize].name,
            name1,
            0 as *mut *mut libc::c_char,
        );
        cmp.file[1 as libc::c_int as usize].name = free1;
    }
    f = 0 as libc::c_int;
    while f < 2 as libc::c_int {
        if cmp.file[f as usize].desc != NONEXISTENT as libc::c_int {
            if f != 0
                && strcmp(
                    cmp.file[f as usize].name,
                    cmp.file[0 as libc::c_int as usize].name,
                ) == 0 as libc::c_int
            {
                cmp.file[f as usize].desc = cmp.file[0 as libc::c_int as usize].desc;
                cmp.file[f as usize].stat = cmp.file[0 as libc::c_int as usize].stat;
            } else if strcmp(
                cmp.file[f as usize].name,
                b"-\0" as *const u8 as *const libc::c_char,
            ) == 0 as libc::c_int
            {
                cmp.file[f as usize].desc = 0 as libc::c_int;
                if binary as libc::c_int != 0 && isatty(0 as libc::c_int) == 0 {
                    set_binary_mode(0 as libc::c_int, 0 as libc::c_int);
                }
                if fstat(
                    0 as libc::c_int,
                    &mut (*(cmp.file).as_mut_ptr().offset(f as isize)).stat,
                ) != 0 as libc::c_int
                {
                    cmp.file[f as usize].desc = errno_encode(*__errno_location());
                } else {
                    if cmp.file[f as usize].stat.st_mode
                        & 0o170000 as libc::c_int as libc::c_uint
                        == 0o100000 as libc::c_int as libc::c_uint
                    {
                        let mut pos: off_t = lseek(
                            0 as libc::c_int,
                            0 as libc::c_int as __off_t,
                            1 as libc::c_int,
                        );
                        if pos < 0 as libc::c_int as libc::c_long {
                            cmp
                                .file[f as usize]
                                .desc = errno_encode(*__errno_location());
                        } else {
                            cmp
                                .file[f as usize]
                                .stat
                                .st_size = if 0 as libc::c_int as libc::c_long
                                >= cmp.file[f as usize].stat.st_size - pos
                            {
                                0 as libc::c_int as libc::c_long
                            } else {
                                cmp.file[f as usize].stat.st_size - pos
                            };
                        }
                    }
                    set_mtime_to_now(
                        &mut (*(cmp.file).as_mut_ptr().offset(f as isize)).stat,
                    );
                }
            } else if (if no_dereference_symlinks as libc::c_int != 0 {
                lstat(
                    cmp.file[f as usize].name,
                    &mut (*(cmp.file).as_mut_ptr().offset(f as isize)).stat,
                )
            } else {
                stat(
                    cmp.file[f as usize].name,
                    &mut (*(cmp.file).as_mut_ptr().offset(f as isize)).stat,
                )
            }) != 0 as libc::c_int
            {
                cmp.file[f as usize].desc = errno_encode(*__errno_location());
            }
        }
        f += 1;
        f;
    }
    f = 0 as libc::c_int;
    while f < 2 as libc::c_int {
        if (new_file as libc::c_int != 0
            || f == 0 as libc::c_int && unidirectional_new_file as libc::c_int != 0)
            && (if cmp.file[f as usize].desc == UNOPENED as libc::c_int {
                (cmp.file[f as usize].stat.st_mode
                    & 0o170000 as libc::c_int as libc::c_uint
                    == 0o100000 as libc::c_int as libc::c_uint
                    && cmp.file[f as usize].stat.st_mode
                        & (0o400 as libc::c_int | 0o200 as libc::c_int
                            | 0o100 as libc::c_int
                            | (0o400 as libc::c_int | 0o200 as libc::c_int
                                | 0o100 as libc::c_int) >> 3 as libc::c_int
                            | (0o400 as libc::c_int | 0o200 as libc::c_int
                                | 0o100 as libc::c_int) >> 3 as libc::c_int
                                >> 3 as libc::c_int) as libc::c_uint == 0
                    && cmp.file[f as usize].stat.st_size
                        == 0 as libc::c_int as libc::c_long) as libc::c_int
            } else {
                ((cmp.file[f as usize].desc == errno_encode(2 as libc::c_int)
                    || cmp.file[f as usize].desc == errno_encode(9 as libc::c_int))
                    && parent.is_null()
                    && (cmp.file[(1 as libc::c_int - f) as usize].desc
                        == UNOPENED as libc::c_int
                        || cmp.file[(1 as libc::c_int - f) as usize].desc
                            == 0 as libc::c_int)) as libc::c_int
            }) != 0
        {
            cmp.file[f as usize].desc = NONEXISTENT as libc::c_int;
        }
        f += 1;
        f;
    }
    f = 0 as libc::c_int;
    while f < 2 as libc::c_int {
        if cmp.file[f as usize].desc == NONEXISTENT as libc::c_int {
            memset(
                &mut (*(cmp.file).as_mut_ptr().offset(f as isize)).stat as *mut stat
                    as *mut libc::c_void,
                0 as libc::c_int,
                ::std::mem::size_of::<stat>() as libc::c_ulong,
            );
            cmp
                .file[f as usize]
                .stat
                .st_mode = cmp.file[(1 as libc::c_int - f) as usize].stat.st_mode;
        }
        f += 1;
        f;
    }
    f = 0 as libc::c_int;
    while f < 2 as libc::c_int {
        let mut e: libc::c_int = errno_decode(cmp.file[f as usize].desc);
        if 0 as libc::c_int <= e {
            *__errno_location() = e;
            perror_with_name(cmp.file[f as usize].name);
            status = 2 as libc::c_int;
        }
        f += 1;
        f;
    }
    if status == 0 as libc::c_int && parent.is_null() && !no_directory
        && ((cmp.file[0 as libc::c_int as usize].stat.st_mode
            & 0o170000 as libc::c_int as libc::c_uint
            == 0o40000 as libc::c_int as libc::c_uint) as libc::c_int
            != 0 as libc::c_int) as libc::c_int
            != ((cmp.file[1 as libc::c_int as usize].stat.st_mode
                & 0o170000 as libc::c_int as libc::c_uint
                == 0o40000 as libc::c_int as libc::c_uint) as libc::c_int
                != 0 as libc::c_int) as libc::c_int
    {
        let mut fnm_arg: libc::c_int = ((cmp.file[0 as libc::c_int as usize].stat.st_mode
            & 0o170000 as libc::c_int as libc::c_uint
            == 0o40000 as libc::c_int as libc::c_uint) as libc::c_int
            != 0 as libc::c_int) as libc::c_int;
        let mut dir_arg: libc::c_int = 1 as libc::c_int - fnm_arg;
        let mut fnm: *const libc::c_char = cmp.file[fnm_arg as usize].name;
        let mut dir_0: *const libc::c_char = cmp.file[dir_arg as usize].name;
        free0 = find_dir_file_pathname(dir_0, last_component(fnm));
        cmp.file[dir_arg as usize].name = free0;
        let mut filename: *const libc::c_char = cmp.file[dir_arg as usize].name;
        if strcmp(fnm, b"-\0" as *const u8 as *const libc::c_char) == 0 as libc::c_int {
            fatal(
                b"cannot compare '-' to a directory\0" as *const u8
                    as *const libc::c_char,
            );
        }
        if (if no_dereference_symlinks as libc::c_int != 0 {
            lstat(
                filename,
                &mut (*(cmp.file).as_mut_ptr().offset(dir_arg as isize)).stat,
            )
        } else {
            stat(filename, &mut (*(cmp.file).as_mut_ptr().offset(dir_arg as isize)).stat)
        }) != 0 as libc::c_int
        {
            perror_with_name(filename);
            status = 2 as libc::c_int;
        }
    }
    if !(status != 0 as libc::c_int) {
        if !(cmp.file[0 as libc::c_int as usize].desc == NONEXISTENT as libc::c_int
            && cmp.file[1 as libc::c_int as usize].desc == NONEXISTENT as libc::c_int)
        {
            same_files = cmp.file[0 as libc::c_int as usize].desc
                != NONEXISTENT as libc::c_int
                && cmp.file[1 as libc::c_int as usize].desc != NONEXISTENT as libc::c_int
                && (0 as libc::c_int)
                    < (cmp.file[0 as libc::c_int as usize].stat.st_ino
                        == cmp.file[1 as libc::c_int as usize].stat.st_ino
                        && cmp.file[0 as libc::c_int as usize].stat.st_dev
                            == cmp.file[1 as libc::c_int as usize].stat.st_dev
                        || (cmp.file[0 as libc::c_int as usize].stat.st_mode
                            & 0o170000 as libc::c_int as libc::c_uint
                            == 0o60000 as libc::c_int as libc::c_uint
                            && cmp.file[1 as libc::c_int as usize].stat.st_mode
                                & 0o170000 as libc::c_int as libc::c_uint
                                == 0o60000 as libc::c_int as libc::c_uint
                            || cmp.file[0 as libc::c_int as usize].stat.st_mode
                                & 0o170000 as libc::c_int as libc::c_uint
                                == 0o20000 as libc::c_int as libc::c_uint
                                && cmp.file[1 as libc::c_int as usize].stat.st_mode
                                    & 0o170000 as libc::c_int as libc::c_uint
                                    == 0o20000 as libc::c_int as libc::c_uint)
                            && cmp.file[0 as libc::c_int as usize].stat.st_rdev
                                == cmp.file[1 as libc::c_int as usize].stat.st_rdev)
                        as libc::c_int
                && (cmp.file[0 as libc::c_int as usize].stat.st_mode
                    == cmp.file[1 as libc::c_int as usize].stat.st_mode
                    && cmp.file[0 as libc::c_int as usize].stat.st_nlink
                        == cmp.file[1 as libc::c_int as usize].stat.st_nlink
                    && cmp.file[0 as libc::c_int as usize].stat.st_uid
                        == cmp.file[1 as libc::c_int as usize].stat.st_uid
                    && cmp.file[0 as libc::c_int as usize].stat.st_gid
                        == cmp.file[1 as libc::c_int as usize].stat.st_gid
                    && cmp.file[0 as libc::c_int as usize].stat.st_size
                        == cmp.file[1 as libc::c_int as usize].stat.st_size
                    && cmp.file[0 as libc::c_int as usize].stat.st_mtim.tv_sec
                        == cmp.file[1 as libc::c_int as usize].stat.st_mtim.tv_sec
                    && cmp.file[0 as libc::c_int as usize].stat.st_ctim.tv_sec
                        == cmp.file[1 as libc::c_int as usize].stat.st_ctim.tv_sec);
            if !(same_files as libc::c_int != 0
                && no_diff_means_no_output as libc::c_int != 0)
            {
                if ((cmp.file[0 as libc::c_int as usize].stat.st_mode
                    & 0o170000 as libc::c_int as libc::c_uint
                    == 0o40000 as libc::c_int as libc::c_uint) as libc::c_int
                    != 0 as libc::c_int) as libc::c_int
                    & ((cmp.file[1 as libc::c_int as usize].stat.st_mode
                        & 0o170000 as libc::c_int as libc::c_uint
                        == 0o40000 as libc::c_int as libc::c_uint) as libc::c_int
                        != 0 as libc::c_int) as libc::c_int != 0
                {
                    if output_style as libc::c_uint
                        == OUTPUT_IFDEF as libc::c_int as libc::c_uint
                    {
                        fatal(
                            b"-D option not supported with directories\0" as *const u8
                                as *const libc::c_char,
                        );
                    }
                    if !parent.is_null() && !recursive {
                        message(
                            b"Common subdirectories: %s and %s\n\0" as *const u8
                                as *const libc::c_char,
                            cmp.file[0 as libc::c_int as usize].name,
                            cmp.file[1 as libc::c_int as usize].name,
                        );
                    } else {
                        status = diff_dirs(
                            &mut cmp,
                            Some(
                                compare_files
                                    as unsafe extern "C" fn(
                                        *const comparison,
                                        *const libc::c_char,
                                        *const libc::c_char,
                                    ) -> libc::c_int,
                            ),
                        );
                    }
                } else if ((cmp.file[0 as libc::c_int as usize].stat.st_mode
                    & 0o170000 as libc::c_int as libc::c_uint
                    == 0o40000 as libc::c_int as libc::c_uint) as libc::c_int
                    != 0 as libc::c_int) as libc::c_int
                    | ((cmp.file[1 as libc::c_int as usize].stat.st_mode
                        & 0o170000 as libc::c_int as libc::c_uint
                        == 0o40000 as libc::c_int as libc::c_uint) as libc::c_int
                        != 0 as libc::c_int) as libc::c_int != 0
                    || !parent.is_null()
                        && !((cmp.file[0 as libc::c_int as usize].stat.st_mode
                            & 0o170000 as libc::c_int as libc::c_uint
                            == 0o100000 as libc::c_int as libc::c_uint
                            || cmp.file[0 as libc::c_int as usize].stat.st_mode
                                & 0o170000 as libc::c_int as libc::c_uint
                                == 0o120000 as libc::c_int as libc::c_uint)
                            && (cmp.file[1 as libc::c_int as usize].stat.st_mode
                                & 0o170000 as libc::c_int as libc::c_uint
                                == 0o100000 as libc::c_int as libc::c_uint
                                || cmp.file[1 as libc::c_int as usize].stat.st_mode
                                    & 0o170000 as libc::c_int as libc::c_uint
                                    == 0o120000 as libc::c_int as libc::c_uint))
                {
                    if cmp.file[0 as libc::c_int as usize].desc
                        == NONEXISTENT as libc::c_int
                        || cmp.file[1 as libc::c_int as usize].desc
                            == NONEXISTENT as libc::c_int
                    {
                        if ((cmp.file[0 as libc::c_int as usize].stat.st_mode
                            & 0o170000 as libc::c_int as libc::c_uint
                            == 0o40000 as libc::c_int as libc::c_uint) as libc::c_int
                            != 0 as libc::c_int) as libc::c_int
                            | ((cmp.file[1 as libc::c_int as usize].stat.st_mode
                                & 0o170000 as libc::c_int as libc::c_uint
                                == 0o40000 as libc::c_int as libc::c_uint) as libc::c_int
                                != 0 as libc::c_int) as libc::c_int != 0
                            && recursive as libc::c_int != 0
                            && (new_file as libc::c_int != 0
                                || unidirectional_new_file as libc::c_int != 0
                                    && cmp.file[0 as libc::c_int as usize].desc
                                        == NONEXISTENT as libc::c_int)
                        {
                            status = diff_dirs(
                                &mut cmp,
                                Some(
                                    compare_files
                                        as unsafe extern "C" fn(
                                            *const comparison,
                                            *const libc::c_char,
                                            *const libc::c_char,
                                        ) -> libc::c_int,
                                ),
                            );
                        } else {
                            let mut dir_1: *const libc::c_char = 0
                                as *const libc::c_char;
                            if !parent.is_null() {} else {
                                __assert_fail(
                                    b"parent\0" as *const u8 as *const libc::c_char,
                                    b"diff.c\0" as *const u8 as *const libc::c_char,
                                    1397 as libc::c_int as libc::c_uint,
                                    (*::std::mem::transmute::<
                                        &[u8; 73],
                                        &[libc::c_char; 73],
                                    >(
                                        b"int compare_files(const struct comparison *, const char *, const char *)\0",
                                    ))
                                        .as_ptr(),
                                );
                            }
                            'c_7716: {
                                if !parent.is_null() {} else {
                                    __assert_fail(
                                        b"parent\0" as *const u8 as *const libc::c_char,
                                        b"diff.c\0" as *const u8 as *const libc::c_char,
                                        1397 as libc::c_int as libc::c_uint,
                                        (*::std::mem::transmute::<
                                            &[u8; 73],
                                            &[libc::c_char; 73],
                                        >(
                                            b"int compare_files(const struct comparison *, const char *, const char *)\0",
                                        ))
                                            .as_ptr(),
                                    );
                                }
                            };
                            dir_1 = (*parent)
                                .file[(cmp.file[0 as libc::c_int as usize].desc
                                    == NONEXISTENT as libc::c_int) as libc::c_int as usize]
                                .name;
                            message(
                                b"Only in %s: %s\n\0" as *const u8 as *const libc::c_char,
                                dir_1,
                                name0,
                            );
                            status = 1 as libc::c_int;
                        }
                    } else {
                        message(
                            b"File %s is a %s while file %s is a %s\n\0" as *const u8
                                as *const libc::c_char,
                            if !(file_label[0 as libc::c_int as usize]).is_null() {
                                file_label[0 as libc::c_int as usize] as *const libc::c_char
                            } else {
                                cmp.file[0 as libc::c_int as usize].name
                            },
                            file_type(
                                &mut (*(cmp.file)
                                    .as_mut_ptr()
                                    .offset(0 as libc::c_int as isize))
                                    .stat,
                            ),
                            if !(file_label[1 as libc::c_int as usize]).is_null() {
                                file_label[1 as libc::c_int as usize] as *const libc::c_char
                            } else {
                                cmp.file[1 as libc::c_int as usize].name
                            },
                            file_type(
                                &mut (*(cmp.file)
                                    .as_mut_ptr()
                                    .offset(1 as libc::c_int as isize))
                                    .stat,
                            ),
                        );
                        status = 1 as libc::c_int;
                    }
                } else if cmp.file[0 as libc::c_int as usize].stat.st_mode
                    & 0o170000 as libc::c_int as libc::c_uint
                    == 0o120000 as libc::c_int as libc::c_uint
                    || cmp.file[1 as libc::c_int as usize].stat.st_mode
                        & 0o170000 as libc::c_int as libc::c_uint
                        == 0o120000 as libc::c_int as libc::c_uint
                {
                    if no_dereference_symlinks {} else {
                        __assert_fail(
                            b"no_dereference_symlinks\0" as *const u8
                                as *const libc::c_char,
                            b"diff.c\0" as *const u8 as *const libc::c_char,
                            1425 as libc::c_int as libc::c_uint,
                            (*::std::mem::transmute::<
                                &[u8; 73],
                                &[libc::c_char; 73],
                            >(
                                b"int compare_files(const struct comparison *, const char *, const char *)\0",
                            ))
                                .as_ptr(),
                        );
                    }
                    'c_7526: {
                        if no_dereference_symlinks {} else {
                            __assert_fail(
                                b"no_dereference_symlinks\0" as *const u8
                                    as *const libc::c_char,
                                b"diff.c\0" as *const u8 as *const libc::c_char,
                                1425 as libc::c_int as libc::c_uint,
                                (*::std::mem::transmute::<
                                    &[u8; 73],
                                    &[libc::c_char; 73],
                                >(
                                    b"int compare_files(const struct comparison *, const char *, const char *)\0",
                                ))
                                    .as_ptr(),
                            );
                        }
                    };
                    if cmp.file[0 as libc::c_int as usize].stat.st_mode
                        & 0o170000 as libc::c_int as libc::c_uint
                        == 0o120000 as libc::c_int as libc::c_uint
                        && cmp.file[1 as libc::c_int as usize].stat.st_mode
                            & 0o170000 as libc::c_int as libc::c_uint
                            == 0o120000 as libc::c_int as libc::c_uint
                    {
                        let mut link_value: [*mut libc::c_char; 2] = [
                            0 as *mut libc::c_char,
                            0 as *mut libc::c_char,
                        ];
                        f = 0 as libc::c_int;
                        while f < 2 as libc::c_int {
                            link_value[f
                                as usize] = xreadlink(cmp.file[f as usize].name);
                            if (link_value[f as usize]).is_null() {
                                perror_with_name(cmp.file[f as usize].name);
                                status = 2 as libc::c_int;
                                break;
                            } else {
                                f += 1;
                                f;
                            }
                        }
                        if status == 0 as libc::c_int {
                            if !(strcmp(
                                link_value[0 as libc::c_int as usize],
                                link_value[1 as libc::c_int as usize],
                            ) == 0 as libc::c_int)
                            {
                                message(
                                    b"Symbolic links %s and %s differ\n\0" as *const u8
                                        as *const libc::c_char,
                                    cmp.file[0 as libc::c_int as usize].name,
                                    cmp.file[1 as libc::c_int as usize].name,
                                );
                                status = 1 as libc::c_int;
                            }
                        }
                        f = 0 as libc::c_int;
                        while f < 2 as libc::c_int {
                            rpl_free(link_value[f as usize] as *mut libc::c_void);
                            f += 1;
                            f;
                        }
                    } else {
                        message(
                            b"File %s is a %s while file %s is a %s\n\0" as *const u8
                                as *const libc::c_char,
                            if !(file_label[0 as libc::c_int as usize]).is_null() {
                                file_label[0 as libc::c_int as usize] as *const libc::c_char
                            } else {
                                cmp.file[0 as libc::c_int as usize].name
                            },
                            file_type(
                                &mut (*(cmp.file)
                                    .as_mut_ptr()
                                    .offset(0 as libc::c_int as isize))
                                    .stat,
                            ),
                            if !(file_label[1 as libc::c_int as usize]).is_null() {
                                file_label[1 as libc::c_int as usize] as *const libc::c_char
                            } else {
                                cmp.file[1 as libc::c_int as usize].name
                            },
                            file_type(
                                &mut (*(cmp.file)
                                    .as_mut_ptr()
                                    .offset(1 as libc::c_int as isize))
                                    .stat,
                            ),
                        );
                        status = 1 as libc::c_int;
                    }
                } else if files_can_be_treated_as_binary as libc::c_int != 0
                    && cmp.file[0 as libc::c_int as usize].stat.st_mode
                        & 0o170000 as libc::c_int as libc::c_uint
                        == 0o100000 as libc::c_int as libc::c_uint
                    && cmp.file[1 as libc::c_int as usize].stat.st_mode
                        & 0o170000 as libc::c_int as libc::c_uint
                        == 0o100000 as libc::c_int as libc::c_uint
                    && cmp.file[0 as libc::c_int as usize].stat.st_size
                        != cmp.file[1 as libc::c_int as usize].stat.st_size
                    && (0 as libc::c_int as libc::c_long)
                        < cmp.file[0 as libc::c_int as usize].stat.st_size
                    && (0 as libc::c_int as libc::c_long)
                        < cmp.file[1 as libc::c_int as usize].stat.st_size
                {
                    message(
                        b"Files %s and %s differ\n\0" as *const u8
                            as *const libc::c_char,
                        if !(file_label[0 as libc::c_int as usize]).is_null() {
                            file_label[0 as libc::c_int as usize] as *const libc::c_char
                        } else {
                            cmp.file[0 as libc::c_int as usize].name
                        },
                        if !(file_label[1 as libc::c_int as usize]).is_null() {
                            file_label[1 as libc::c_int as usize] as *const libc::c_char
                        } else {
                            cmp.file[1 as libc::c_int as usize].name
                        },
                    );
                    status = 1 as libc::c_int;
                } else {
                    let mut oflags: libc::c_int = 0 as libc::c_int
                        | (if binary as libc::c_int != 0 {
                            0 as libc::c_int
                        } else {
                            0 as libc::c_int
                        });
                    if cmp.file[0 as libc::c_int as usize].desc
                        == UNOPENED as libc::c_int
                    {
                        cmp
                            .file[0 as libc::c_int as usize]
                            .desc = open(
                            cmp.file[0 as libc::c_int as usize].name,
                            oflags,
                            0 as libc::c_int,
                        );
                        if cmp.file[0 as libc::c_int as usize].desc < 0 as libc::c_int {
                            perror_with_name(cmp.file[0 as libc::c_int as usize].name);
                            status = 2 as libc::c_int;
                        }
                    }
                    if cmp.file[1 as libc::c_int as usize].desc
                        == UNOPENED as libc::c_int
                    {
                        if same_files {
                            cmp
                                .file[1 as libc::c_int as usize]
                                .desc = cmp.file[0 as libc::c_int as usize].desc;
                        } else {
                            cmp
                                .file[1 as libc::c_int as usize]
                                .desc = open(
                                cmp.file[1 as libc::c_int as usize].name,
                                oflags,
                                0 as libc::c_int,
                            );
                            if cmp.file[1 as libc::c_int as usize].desc
                                < 0 as libc::c_int
                            {
                                perror_with_name(cmp.file[1 as libc::c_int as usize].name);
                                status = 2 as libc::c_int;
                            }
                        }
                    }
                    if status == 0 as libc::c_int {
                        status = diff_2_files(&mut cmp);
                    }
                    if 0 as libc::c_int <= cmp.file[0 as libc::c_int as usize].desc
                        && close(cmp.file[0 as libc::c_int as usize].desc)
                            != 0 as libc::c_int
                    {
                        perror_with_name(cmp.file[0 as libc::c_int as usize].name);
                        status = 2 as libc::c_int;
                    }
                    if 0 as libc::c_int <= cmp.file[1 as libc::c_int as usize].desc
                        && cmp.file[0 as libc::c_int as usize].desc
                            != cmp.file[1 as libc::c_int as usize].desc
                        && close(cmp.file[1 as libc::c_int as usize].desc)
                            != 0 as libc::c_int
                    {
                        perror_with_name(cmp.file[1 as libc::c_int as usize].name);
                        status = 2 as libc::c_int;
                    }
                }
            }
        }
    }
    if status == 0 as libc::c_int {
        if report_identical_files as libc::c_int != 0
            && !((cmp.file[0 as libc::c_int as usize].stat.st_mode
                & 0o170000 as libc::c_int as libc::c_uint
                == 0o40000 as libc::c_int as libc::c_uint) as libc::c_int
                != 0 as libc::c_int)
        {
            message(
                b"Files %s and %s are identical\n\0" as *const u8 as *const libc::c_char,
                if !(file_label[0 as libc::c_int as usize]).is_null() {
                    file_label[0 as libc::c_int as usize] as *const libc::c_char
                } else {
                    cmp.file[0 as libc::c_int as usize].name
                },
                if !(file_label[1 as libc::c_int as usize]).is_null() {
                    file_label[1 as libc::c_int as usize] as *const libc::c_char
                } else {
                    cmp.file[1 as libc::c_int as usize].name
                },
            );
        }
    } else if fflush_unlocked(stdout) != 0 as libc::c_int {
        pfatal_with_name(
            dcgettext(
                0 as *const libc::c_char,
                b"standard output\0" as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ),
        );
    }
    rpl_free(free0 as *mut libc::c_void);
    rpl_free(free1 as *mut libc::c_void);
    return status;
}
pub fn main() {
    let mut args: Vec::<*mut libc::c_char> = Vec::new();
    for arg in ::std::env::args_os() {
        args.push(
            (::std::ffi::CString::new(
                ::std::os::unix::ffi::OsStrExt::as_bytes(arg.as_os_str()),
            ))
                .unwrap()
                .into_raw(),
        );
    }
    args.push(::std::ptr::null_mut());
    unsafe {
        ::std::process::exit(
            main_0(
                (args.len() - 1) as libc::c_int,
                args.as_mut_ptr() as *mut *mut libc::c_char,
            ) as i32,
        )
    }
}
