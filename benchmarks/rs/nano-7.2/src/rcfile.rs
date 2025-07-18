use ::libc;
use ::c2rust_bitfields;
extern "C" {
    pub type re_dfa_t;
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    fn rpl_regcomp(
        __preg: *mut regex_t,
        __pattern: *const libc::c_char,
        __cflags: libc::c_int,
    ) -> libc::c_int;
    fn rpl_regerror(
        __errcode: libc::c_int,
        __preg: *const regex_t,
        __errbuf: *mut libc::c_char,
        __errbuf_size: size_t,
    ) -> size_t;
    fn rpl_regfree(__preg: *mut regex_t);
    fn getenv(__name: *const libc::c_char) -> *mut libc::c_char;
    fn mbstowcs(__pwcs: *mut wchar_t, __s: *const libc::c_char, __n: size_t) -> size_t;
    fn rpl_free(ptr: *mut libc::c_void);
    static mut stderr: *mut FILE;
    fn fclose(__stream: *mut FILE) -> libc::c_int;
    fn fopen(_: *const libc::c_char, _: *const libc::c_char) -> *mut FILE;
    fn fprintf(_: *mut FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn sprintf(_: *mut libc::c_char, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn snprintf(
        _: *mut libc::c_char,
        _: libc::c_ulong,
        _: *const libc::c_char,
        _: ...
    ) -> libc::c_int;
    fn vsnprintf(
        _: *mut libc::c_char,
        _: libc::c_ulong,
        _: *const libc::c_char,
        _: ::std::ffi::VaList,
    ) -> libc::c_int;
    fn sscanf(_: *const libc::c_char, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn __getdelim(
        __lineptr: *mut *mut libc::c_char,
        __n: *mut size_t,
        __delimiter: libc::c_int,
        __stream: *mut FILE,
    ) -> __ssize_t;
    static mut COLORS: libc::c_int;
    fn dcgettext(
        __domainname: *const libc::c_char,
        __msgid: *const libc::c_char,
        __category: libc::c_int,
    ) -> *mut libc::c_char;
    static mut flags: [libc::c_uint; 4];
    static mut fill: ssize_t;
    static mut stripe_column: ssize_t;
    static mut matchbrackets: *mut libc::c_char;
    static mut whitespace: *mut libc::c_char;
    static mut whitelen: [libc::c_int; 2];
    static mut punct: *mut libc::c_char;
    static mut brackets: *mut libc::c_char;
    static mut quotestr: *mut libc::c_char;
    static mut word_chars: *mut libc::c_char;
    static mut tabsize: ssize_t;
    static mut backup_dir: *mut libc::c_char;
    static mut operating_dir: *mut libc::c_char;
    static mut alt_speller: *mut libc::c_char;
    static mut syntaxes: *mut syntaxtype;
    static mut sclist: *mut keystruct;
    static mut allfuncs: *mut funcstruct;
    static mut color_combo: [*mut colortype; 12];
    static mut homedir: *mut libc::c_char;
    static mut startup_problem: *mut libc::c_char;
    static mut custom_nanorc: *mut libc::c_char;
    fn to_first_file();
    fn to_last_file();
    fn using_utf8() -> bool;
    fn char_length(pointer: *const libc::c_char) -> libc::c_int;
    fn mbstrlen(pointer: *const libc::c_char) -> size_t;
    fn has_blank_char(string: *const libc::c_char) -> bool;
    fn do_delete();
    fn do_backspace();
    fn chop_previous_word();
    fn chop_next_word();
    fn cut_text();
    fn cut_till_eof();
    fn zap_text();
    fn copy_text();
    fn paste_text();
    fn switch_to_prev_buffer();
    fn switch_to_next_buffer();
    fn do_insertfile();
    fn do_execute();
    fn get_full_path(origpath: *const libc::c_char) -> *mut libc::c_char;
    fn do_writeout();
    fn do_savefile();
    fn real_dir_from_tilde(path: *const libc::c_char) -> *mut libc::c_char;
    fn first_sc_for(
        menu: libc::c_int,
        function: Option::<unsafe extern "C" fn() -> ()>,
    ) -> *const keystruct;
    fn keycode_from_string(keystring: *const libc::c_char) -> libc::c_int;
    fn do_help();
    fn to_first_line();
    fn to_last_line();
    fn do_page_up();
    fn do_page_down();
    fn to_para_begin();
    fn to_para_end();
    fn to_prev_block();
    fn to_next_block();
    fn to_prev_word();
    fn to_next_word();
    fn do_home();
    fn do_end();
    fn do_up();
    fn do_down();
    fn do_scroll_up();
    fn do_scroll_down();
    fn do_center();
    fn do_left();
    fn do_right();
    fn make_new_node(prevnode: *mut linestruct) -> *mut linestruct;
    fn do_exit();
    fn die(msg: *const libc::c_char, _: ...);
    fn do_suspend();
    fn nmalloc(howmuch: size_t) -> *mut libc::c_void;
    fn copy_of(string: *const libc::c_char) -> *mut libc::c_char;
    fn do_toggle();
    fn goto_dir();
    fn to_files();
    fn flip_newbuffer();
    fn flip_convert();
    fn flip_pipe();
    fn flip_execute();
    fn back_it_up();
    fn prepend_it();
    fn append_it();
    fn mac_format();
    fn dos_format();
    fn get_newer_item();
    fn get_older_item();
    fn flip_goto();
    fn flip_replace();
    fn backwards_void();
    fn regexp_void();
    fn case_sens_void();
    fn full_refresh();
    fn do_enter();
    fn do_tab();
    fn do_verbatim_input();
    fn do_redo();
    fn do_undo();
    fn to_next_anchor();
    fn to_prev_anchor();
    fn put_or_lift_anchor();
    fn run_macro();
    fn record_macro();
    fn count_lines_words_and_characters();
    fn do_find_bracket();
    fn do_unindent();
    fn do_indent();
    fn complete_a_word();
    fn do_comment();
    fn do_full_justify();
    fn do_justify();
    fn do_gotolinecolumn();
    fn report_cursor_position();
    fn do_formatter();
    fn do_linter();
    fn do_spell();
    fn do_mark();
    fn do_replace();
    fn do_findnext();
    fn do_findprevious();
    fn do_search_backward();
    fn do_search_forward();
    fn discard_buffer();
    fn do_cancel();
    fn mallocstrcpy(
        dest: *mut libc::c_char,
        src: *const libc::c_char,
    ) -> *mut libc::c_char;
    fn parse_num(str: *const libc::c_char, result: *mut ssize_t) -> bool;
    fn breadth(text: *const libc::c_char) -> size_t;
    fn implant(string: *const libc::c_char);
    fn get_homedir();
    fn concatenate(
        path: *const libc::c_char,
        name: *const libc::c_char,
    ) -> *mut libc::c_char;
    fn __xstat(
        __ver: libc::c_int,
        __filename: *const libc::c_char,
        __stat_buf: *mut stat,
    ) -> libc::c_int;
    fn __ctype_b_loc() -> *mut *const libc::c_ushort;
    fn __ctype_toupper_loc() -> *mut *const __int32_t;
    fn __errno_location() -> *mut libc::c_int;
    fn rpl_glob(
        __pattern: *const libc::c_char,
        __flags: libc::c_int,
        __errfunc: Option::<
            unsafe extern "C" fn(*const libc::c_char, libc::c_int) -> libc::c_int,
        >,
        __pglob: *mut glob_t,
    ) -> libc::c_int;
    fn rpl_globfree(__pglob: *mut glob_t);
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    fn strncmp(
        _: *const libc::c_char,
        _: *const libc::c_char,
        _: libc::c_ulong,
    ) -> libc::c_int;
    fn strchr(_: *const libc::c_char, _: libc::c_int) -> *mut libc::c_char;
    fn strerror(_: libc::c_int) -> *mut libc::c_char;
    fn strstr(_: *const libc::c_char, _: *const libc::c_char) -> *mut libc::c_char;
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    fn access(__name: *const libc::c_char, __type: libc::c_int) -> libc::c_int;
}
pub type __builtin_va_list = [__va_list_tag; 1];
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __va_list_tag {
    pub gp_offset: libc::c_uint,
    pub fp_offset: libc::c_uint,
    pub overflow_arg_area: *mut libc::c_void,
    pub reg_save_area: *mut libc::c_void,
}
pub type __int32_t = libc::c_int;
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
pub type ssize_t = __ssize_t;
pub type size_t = libc::c_ulong;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct timespec {
    pub tv_sec: __time_t,
    pub tv_nsec: __syscall_slong_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct dirent {
    pub d_ino: __ino_t,
    pub d_off: __off_t,
    pub d_reclen: libc::c_ushort,
    pub d_type: libc::c_uchar,
    pub d_name: [libc::c_char; 256],
}
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
pub type regex_t = re_pattern_buffer;
pub type wchar_t = libc::c_int;
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
pub type chtype = libc::c_uint;
pub type va_list = __builtin_va_list;
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
pub type C2RustUnnamed = libc::c_uint;
pub const NUMBER_OF_ELEMENTS: C2RustUnnamed = 12;
pub const FUNCTION_TAG: C2RustUnnamed = 11;
pub const KEY_COMBO: C2RustUnnamed = 10;
pub const ERROR_MESSAGE: C2RustUnnamed = 9;
pub const STATUS_BAR: C2RustUnnamed = 8;
pub const PROMPT_BAR: C2RustUnnamed = 7;
pub const MINI_INFOBAR: C2RustUnnamed = 6;
pub const SPOTLIGHTED: C2RustUnnamed = 5;
pub const SELECTED_TEXT: C2RustUnnamed = 4;
pub const SCROLL_BAR: C2RustUnnamed = 3;
pub const GUIDE_STRIPE: C2RustUnnamed = 2;
pub const LINE_NUMBER: C2RustUnnamed = 1;
pub const TITLE_BAR: C2RustUnnamed = 0;
pub type C2RustUnnamed_0 = libc::c_uint;
pub const ZERO: C2RustUnnamed_0 = 48;
pub const MINIBAR: C2RustUnnamed_0 = 47;
pub const USE_MAGIC: C2RustUnnamed_0 = 46;
pub const STATEFLAGS: C2RustUnnamed_0 = 45;
pub const BOOKSTYLE: C2RustUnnamed_0 = 44;
pub const INDICATOR: C2RustUnnamed_0 = 43;
pub const EMPTY_LINE: C2RustUnnamed_0 = 42;
pub const JUMPY_SCROLLING: C2RustUnnamed_0 = 41;
pub const BREAK_LONG_LINES: C2RustUnnamed_0 = 40;
pub const LET_THEM_ZAP: C2RustUnnamed_0 = 39;
pub const AFTER_ENDS: C2RustUnnamed_0 = 38;
pub const AT_BLANKS: C2RustUnnamed_0 = 37;
pub const LINE_NUMBERS: C2RustUnnamed_0 = 36;
pub const SHOW_CURSOR: C2RustUnnamed_0 = 35;
pub const TRIM_BLANKS: C2RustUnnamed_0 = 34;
pub const MAKE_IT_UNIX: C2RustUnnamed_0 = 33;
pub const NOREAD_MODE: C2RustUnnamed_0 = 32;
pub const LOCKING: C2RustUnnamed_0 = 31;
pub const POSITIONLOG: C2RustUnnamed_0 = 30;
pub const SOFTWRAP: C2RustUnnamed_0 = 29;
pub const BOLD_TEXT: C2RustUnnamed_0 = 28;
pub const NO_NEWLINES: C2RustUnnamed_0 = 27;
pub const WORD_BOUNDS: C2RustUnnamed_0 = 26;
pub const QUICK_BLANK: C2RustUnnamed_0 = 25;
pub const TABS_TO_SPACES: C2RustUnnamed_0 = 24;
pub const WHITESPACE_DISPLAY: C2RustUnnamed_0 = 23;
pub const SMART_HOME: C2RustUnnamed_0 = 22;
pub const RESTRICTED: C2RustUnnamed_0 = 21;
pub const HISTORYLOG: C2RustUnnamed_0 = 20;
pub const PRESERVE: C2RustUnnamed_0 = 19;
pub const NO_SYNTAX: C2RustUnnamed_0 = 18;
pub const INSECURE_BACKUP: C2RustUnnamed_0 = 17;
pub const MAKE_BACKUP: C2RustUnnamed_0 = 16;
pub const NO_CONVERT: C2RustUnnamed_0 = 15;
pub const RAW_SEQUENCES: C2RustUnnamed_0 = 14;
pub const REBIND_DELETE: C2RustUnnamed_0 = 13;
pub const MULTIBUFFER: C2RustUnnamed_0 = 12;
pub const BACKWARDS_SEARCH: C2RustUnnamed_0 = 11;
pub const CUT_FROM_CURSOR: C2RustUnnamed_0 = 10;
pub const SAVE_ON_EXIT: C2RustUnnamed_0 = 9;
pub const USE_REGEXP: C2RustUnnamed_0 = 8;
pub const USE_MOUSE: C2RustUnnamed_0 = 7;
pub const VIEW_MODE: C2RustUnnamed_0 = 6;
pub const AUTOINDENT: C2RustUnnamed_0 = 5;
pub const NO_WRAP: C2RustUnnamed_0 = 4;
pub const NO_HELP: C2RustUnnamed_0 = 3;
pub const CONSTANT_SHOW: C2RustUnnamed_0 = 2;
pub const CASE_SENSITIVE: C2RustUnnamed_0 = 1;
pub const DONTUSE: C2RustUnnamed_0 = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct colortype {
    pub id: libc::c_short,
    pub fg: libc::c_short,
    pub bg: libc::c_short,
    pub pairnum: libc::c_short,
    pub attributes: libc::c_int,
    pub start: *mut regex_t,
    pub end: *mut regex_t,
    pub next: *mut colortype,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct regexlisttype {
    pub one_rgx: *mut regex_t,
    pub next: *mut regexlisttype,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct augmentstruct {
    pub filename: *mut libc::c_char,
    pub lineno: ssize_t,
    pub data: *mut libc::c_char,
    pub next: *mut augmentstruct,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct syntaxtype {
    pub name: *mut libc::c_char,
    pub filename: *mut libc::c_char,
    pub lineno: size_t,
    pub augmentations: *mut augmentstruct,
    pub extensions: *mut regexlisttype,
    pub headers: *mut regexlisttype,
    pub magics: *mut regexlisttype,
    pub linter: *mut libc::c_char,
    pub formatter: *mut libc::c_char,
    pub tab: *mut libc::c_char,
    pub comment: *mut libc::c_char,
    pub color: *mut colortype,
    pub nmultis: libc::c_short,
    pub next: *mut syntaxtype,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct linestruct {
    pub data: *mut libc::c_char,
    pub lineno: ssize_t,
    pub next: *mut linestruct,
    pub prev: *mut linestruct,
    pub multidata: *mut libc::c_short,
    pub has_anchor: bool,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct rcoption {
    pub name: *const libc::c_char,
    pub flag: libc::c_long,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct keystruct {
    pub keystr: *const libc::c_char,
    pub keycode: libc::c_int,
    pub menus: libc::c_int,
    pub func: Option::<unsafe extern "C" fn() -> ()>,
    pub toggle: libc::c_int,
    pub ordinal: libc::c_int,
    pub expansion: *mut libc::c_char,
    pub next: *mut keystruct,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct funcstruct {
    pub func: Option::<unsafe extern "C" fn() -> ()>,
    pub tag: *const libc::c_char,
    pub phrase: *const libc::c_char,
    pub blank_after: bool,
    pub menus: libc::c_int,
    pub next: *mut funcstruct,
}
pub type functionptrtype = Option::<unsafe extern "C" fn() -> ()>;
pub const _ISblank: C2RustUnnamed_1 = 1;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct glob_t {
    pub gl_pathc: size_t,
    pub gl_pathv: *mut *mut libc::c_char,
    pub gl_offs: size_t,
    pub gl_flags: libc::c_int,
    pub gl_closedir: Option::<unsafe extern "C" fn(*mut libc::c_void) -> ()>,
    pub gl_readdir: Option::<unsafe extern "C" fn(*mut libc::c_void) -> *mut dirent>,
    pub gl_opendir: Option::<
        unsafe extern "C" fn(*const libc::c_char) -> *mut libc::c_void,
    >,
    pub gl_lstat: Option::<
        unsafe extern "C" fn(*const libc::c_char, *mut stat) -> libc::c_int,
    >,
    pub gl_stat: Option::<
        unsafe extern "C" fn(*const libc::c_char, *mut stat) -> libc::c_int,
    >,
}
pub type C2RustUnnamed_1 = libc::c_uint;
pub const _ISalnum: C2RustUnnamed_1 = 8;
pub const _ISpunct: C2RustUnnamed_1 = 4;
pub const _IScntrl: C2RustUnnamed_1 = 2;
pub const _ISgraph: C2RustUnnamed_1 = 32768;
pub const _ISprint: C2RustUnnamed_1 = 16384;
pub const _ISspace: C2RustUnnamed_1 = 8192;
pub const _ISxdigit: C2RustUnnamed_1 = 4096;
pub const _ISdigit: C2RustUnnamed_1 = 2048;
pub const _ISalpha: C2RustUnnamed_1 = 1024;
pub const _ISlower: C2RustUnnamed_1 = 512;
pub const _ISupper: C2RustUnnamed_1 = 256;
#[inline]
unsafe extern "C" fn getline(
    mut __lineptr: *mut *mut libc::c_char,
    mut __n: *mut size_t,
    mut __stream: *mut FILE,
) -> __ssize_t {
    return __getdelim(__lineptr, __n, '\n' as i32, __stream);
}
#[inline]
unsafe extern "C" fn stat(
    mut __path: *const libc::c_char,
    mut __statbuf: *mut stat,
) -> libc::c_int {
    return __xstat(1 as libc::c_int, __path, __statbuf);
}
#[inline]
unsafe extern "C" fn toupper(mut __c: libc::c_int) -> libc::c_int {
    return if __c >= -(128 as libc::c_int) && __c < 256 as libc::c_int {
        *(*__ctype_toupper_loc()).offset(__c as isize)
    } else {
        __c
    };
}
static mut rcopts: [rcoption; 66] = [
    {
        let mut init = rcoption {
            name: b"boldtext\0" as *const u8 as *const libc::c_char,
            flag: BOLD_TEXT as libc::c_int as libc::c_long,
        };
        init
    },
    {
        let mut init = rcoption {
            name: b"brackets\0" as *const u8 as *const libc::c_char,
            flag: 0 as libc::c_int as libc::c_long,
        };
        init
    },
    {
        let mut init = rcoption {
            name: b"breaklonglines\0" as *const u8 as *const libc::c_char,
            flag: BREAK_LONG_LINES as libc::c_int as libc::c_long,
        };
        init
    },
    {
        let mut init = rcoption {
            name: b"casesensitive\0" as *const u8 as *const libc::c_char,
            flag: CASE_SENSITIVE as libc::c_int as libc::c_long,
        };
        init
    },
    {
        let mut init = rcoption {
            name: b"constantshow\0" as *const u8 as *const libc::c_char,
            flag: CONSTANT_SHOW as libc::c_int as libc::c_long,
        };
        init
    },
    {
        let mut init = rcoption {
            name: b"fill\0" as *const u8 as *const libc::c_char,
            flag: 0 as libc::c_int as libc::c_long,
        };
        init
    },
    {
        let mut init = rcoption {
            name: b"historylog\0" as *const u8 as *const libc::c_char,
            flag: HISTORYLOG as libc::c_int as libc::c_long,
        };
        init
    },
    {
        let mut init = rcoption {
            name: b"linenumbers\0" as *const u8 as *const libc::c_char,
            flag: LINE_NUMBERS as libc::c_int as libc::c_long,
        };
        init
    },
    {
        let mut init = rcoption {
            name: b"mouse\0" as *const u8 as *const libc::c_char,
            flag: USE_MOUSE as libc::c_int as libc::c_long,
        };
        init
    },
    {
        let mut init = rcoption {
            name: b"multibuffer\0" as *const u8 as *const libc::c_char,
            flag: MULTIBUFFER as libc::c_int as libc::c_long,
        };
        init
    },
    {
        let mut init = rcoption {
            name: b"nohelp\0" as *const u8 as *const libc::c_char,
            flag: NO_HELP as libc::c_int as libc::c_long,
        };
        init
    },
    {
        let mut init = rcoption {
            name: b"nonewlines\0" as *const u8 as *const libc::c_char,
            flag: NO_NEWLINES as libc::c_int as libc::c_long,
        };
        init
    },
    {
        let mut init = rcoption {
            name: b"nowrap\0" as *const u8 as *const libc::c_char,
            flag: NO_WRAP as libc::c_int as libc::c_long,
        };
        init
    },
    {
        let mut init = rcoption {
            name: b"operatingdir\0" as *const u8 as *const libc::c_char,
            flag: 0 as libc::c_int as libc::c_long,
        };
        init
    },
    {
        let mut init = rcoption {
            name: b"positionlog\0" as *const u8 as *const libc::c_char,
            flag: POSITIONLOG as libc::c_int as libc::c_long,
        };
        init
    },
    {
        let mut init = rcoption {
            name: b"preserve\0" as *const u8 as *const libc::c_char,
            flag: PRESERVE as libc::c_int as libc::c_long,
        };
        init
    },
    {
        let mut init = rcoption {
            name: b"punct\0" as *const u8 as *const libc::c_char,
            flag: 0 as libc::c_int as libc::c_long,
        };
        init
    },
    {
        let mut init = rcoption {
            name: b"quotestr\0" as *const u8 as *const libc::c_char,
            flag: 0 as libc::c_int as libc::c_long,
        };
        init
    },
    {
        let mut init = rcoption {
            name: b"quickblank\0" as *const u8 as *const libc::c_char,
            flag: QUICK_BLANK as libc::c_int as libc::c_long,
        };
        init
    },
    {
        let mut init = rcoption {
            name: b"rawsequences\0" as *const u8 as *const libc::c_char,
            flag: RAW_SEQUENCES as libc::c_int as libc::c_long,
        };
        init
    },
    {
        let mut init = rcoption {
            name: b"rebinddelete\0" as *const u8 as *const libc::c_char,
            flag: REBIND_DELETE as libc::c_int as libc::c_long,
        };
        init
    },
    {
        let mut init = rcoption {
            name: b"regexp\0" as *const u8 as *const libc::c_char,
            flag: USE_REGEXP as libc::c_int as libc::c_long,
        };
        init
    },
    {
        let mut init = rcoption {
            name: b"saveonexit\0" as *const u8 as *const libc::c_char,
            flag: SAVE_ON_EXIT as libc::c_int as libc::c_long,
        };
        init
    },
    {
        let mut init = rcoption {
            name: b"speller\0" as *const u8 as *const libc::c_char,
            flag: 0 as libc::c_int as libc::c_long,
        };
        init
    },
    {
        let mut init = rcoption {
            name: b"afterends\0" as *const u8 as *const libc::c_char,
            flag: AFTER_ENDS as libc::c_int as libc::c_long,
        };
        init
    },
    {
        let mut init = rcoption {
            name: b"allow_insecure_backup\0" as *const u8 as *const libc::c_char,
            flag: INSECURE_BACKUP as libc::c_int as libc::c_long,
        };
        init
    },
    {
        let mut init = rcoption {
            name: b"atblanks\0" as *const u8 as *const libc::c_char,
            flag: AT_BLANKS as libc::c_int as libc::c_long,
        };
        init
    },
    {
        let mut init = rcoption {
            name: b"autoindent\0" as *const u8 as *const libc::c_char,
            flag: AUTOINDENT as libc::c_int as libc::c_long,
        };
        init
    },
    {
        let mut init = rcoption {
            name: b"backup\0" as *const u8 as *const libc::c_char,
            flag: MAKE_BACKUP as libc::c_int as libc::c_long,
        };
        init
    },
    {
        let mut init = rcoption {
            name: b"backupdir\0" as *const u8 as *const libc::c_char,
            flag: 0 as libc::c_int as libc::c_long,
        };
        init
    },
    {
        let mut init = rcoption {
            name: b"bookstyle\0" as *const u8 as *const libc::c_char,
            flag: BOOKSTYLE as libc::c_int as libc::c_long,
        };
        init
    },
    {
        let mut init = rcoption {
            name: b"cutfromcursor\0" as *const u8 as *const libc::c_char,
            flag: CUT_FROM_CURSOR as libc::c_int as libc::c_long,
        };
        init
    },
    {
        let mut init = rcoption {
            name: b"emptyline\0" as *const u8 as *const libc::c_char,
            flag: EMPTY_LINE as libc::c_int as libc::c_long,
        };
        init
    },
    {
        let mut init = rcoption {
            name: b"guidestripe\0" as *const u8 as *const libc::c_char,
            flag: 0 as libc::c_int as libc::c_long,
        };
        init
    },
    {
        let mut init = rcoption {
            name: b"indicator\0" as *const u8 as *const libc::c_char,
            flag: INDICATOR as libc::c_int as libc::c_long,
        };
        init
    },
    {
        let mut init = rcoption {
            name: b"jumpyscrolling\0" as *const u8 as *const libc::c_char,
            flag: JUMPY_SCROLLING as libc::c_int as libc::c_long,
        };
        init
    },
    {
        let mut init = rcoption {
            name: b"locking\0" as *const u8 as *const libc::c_char,
            flag: LOCKING as libc::c_int as libc::c_long,
        };
        init
    },
    {
        let mut init = rcoption {
            name: b"matchbrackets\0" as *const u8 as *const libc::c_char,
            flag: 0 as libc::c_int as libc::c_long,
        };
        init
    },
    {
        let mut init = rcoption {
            name: b"minibar\0" as *const u8 as *const libc::c_char,
            flag: MINIBAR as libc::c_int as libc::c_long,
        };
        init
    },
    {
        let mut init = rcoption {
            name: b"noconvert\0" as *const u8 as *const libc::c_char,
            flag: NO_CONVERT as libc::c_int as libc::c_long,
        };
        init
    },
    {
        let mut init = rcoption {
            name: b"showcursor\0" as *const u8 as *const libc::c_char,
            flag: SHOW_CURSOR as libc::c_int as libc::c_long,
        };
        init
    },
    {
        let mut init = rcoption {
            name: b"smarthome\0" as *const u8 as *const libc::c_char,
            flag: SMART_HOME as libc::c_int as libc::c_long,
        };
        init
    },
    {
        let mut init = rcoption {
            name: b"softwrap\0" as *const u8 as *const libc::c_char,
            flag: SOFTWRAP as libc::c_int as libc::c_long,
        };
        init
    },
    {
        let mut init = rcoption {
            name: b"stateflags\0" as *const u8 as *const libc::c_char,
            flag: STATEFLAGS as libc::c_int as libc::c_long,
        };
        init
    },
    {
        let mut init = rcoption {
            name: b"tabsize\0" as *const u8 as *const libc::c_char,
            flag: 0 as libc::c_int as libc::c_long,
        };
        init
    },
    {
        let mut init = rcoption {
            name: b"tabstospaces\0" as *const u8 as *const libc::c_char,
            flag: TABS_TO_SPACES as libc::c_int as libc::c_long,
        };
        init
    },
    {
        let mut init = rcoption {
            name: b"trimblanks\0" as *const u8 as *const libc::c_char,
            flag: TRIM_BLANKS as libc::c_int as libc::c_long,
        };
        init
    },
    {
        let mut init = rcoption {
            name: b"unix\0" as *const u8 as *const libc::c_char,
            flag: MAKE_IT_UNIX as libc::c_int as libc::c_long,
        };
        init
    },
    {
        let mut init = rcoption {
            name: b"whitespace\0" as *const u8 as *const libc::c_char,
            flag: 0 as libc::c_int as libc::c_long,
        };
        init
    },
    {
        let mut init = rcoption {
            name: b"wordbounds\0" as *const u8 as *const libc::c_char,
            flag: WORD_BOUNDS as libc::c_int as libc::c_long,
        };
        init
    },
    {
        let mut init = rcoption {
            name: b"wordchars\0" as *const u8 as *const libc::c_char,
            flag: 0 as libc::c_int as libc::c_long,
        };
        init
    },
    {
        let mut init = rcoption {
            name: b"zap\0" as *const u8 as *const libc::c_char,
            flag: LET_THEM_ZAP as libc::c_int as libc::c_long,
        };
        init
    },
    {
        let mut init = rcoption {
            name: b"zero\0" as *const u8 as *const libc::c_char,
            flag: ZERO as libc::c_int as libc::c_long,
        };
        init
    },
    {
        let mut init = rcoption {
            name: b"titlecolor\0" as *const u8 as *const libc::c_char,
            flag: 0 as libc::c_int as libc::c_long,
        };
        init
    },
    {
        let mut init = rcoption {
            name: b"numbercolor\0" as *const u8 as *const libc::c_char,
            flag: 0 as libc::c_int as libc::c_long,
        };
        init
    },
    {
        let mut init = rcoption {
            name: b"stripecolor\0" as *const u8 as *const libc::c_char,
            flag: 0 as libc::c_int as libc::c_long,
        };
        init
    },
    {
        let mut init = rcoption {
            name: b"scrollercolor\0" as *const u8 as *const libc::c_char,
            flag: 0 as libc::c_int as libc::c_long,
        };
        init
    },
    {
        let mut init = rcoption {
            name: b"selectedcolor\0" as *const u8 as *const libc::c_char,
            flag: 0 as libc::c_int as libc::c_long,
        };
        init
    },
    {
        let mut init = rcoption {
            name: b"spotlightcolor\0" as *const u8 as *const libc::c_char,
            flag: 0 as libc::c_int as libc::c_long,
        };
        init
    },
    {
        let mut init = rcoption {
            name: b"minicolor\0" as *const u8 as *const libc::c_char,
            flag: 0 as libc::c_int as libc::c_long,
        };
        init
    },
    {
        let mut init = rcoption {
            name: b"promptcolor\0" as *const u8 as *const libc::c_char,
            flag: 0 as libc::c_int as libc::c_long,
        };
        init
    },
    {
        let mut init = rcoption {
            name: b"statuscolor\0" as *const u8 as *const libc::c_char,
            flag: 0 as libc::c_int as libc::c_long,
        };
        init
    },
    {
        let mut init = rcoption {
            name: b"errorcolor\0" as *const u8 as *const libc::c_char,
            flag: 0 as libc::c_int as libc::c_long,
        };
        init
    },
    {
        let mut init = rcoption {
            name: b"keycolor\0" as *const u8 as *const libc::c_char,
            flag: 0 as libc::c_int as libc::c_long,
        };
        init
    },
    {
        let mut init = rcoption {
            name: b"functioncolor\0" as *const u8 as *const libc::c_char,
            flag: 0 as libc::c_int as libc::c_long,
        };
        init
    },
    {
        let mut init = rcoption {
            name: 0 as *const libc::c_char,
            flag: 0 as libc::c_int as libc::c_long,
        };
        init
    },
];
static mut lineno: size_t = 0 as libc::c_int as size_t;
static mut nanorc: *mut libc::c_char = 0 as *const libc::c_char as *mut libc::c_char;
static mut opensyntax: bool = 0 as libc::c_int != 0;
static mut live_syntax: *mut syntaxtype = 0 as *const syntaxtype as *mut syntaxtype;
static mut seen_color_command: bool = 0 as libc::c_int != 0;
static mut lastcolor: *mut colortype = 0 as *const colortype as *mut colortype;
static mut errors_head: *mut linestruct = 0 as *const linestruct as *mut linestruct;
static mut errors_tail: *mut linestruct = 0 as *const linestruct as *mut linestruct;
pub unsafe extern "C" fn display_rcfile_errors() {
    let mut error: *mut linestruct = errors_head;
    while !error.is_null() {
        fprintf(stderr, b"%s\n\0" as *const u8 as *const libc::c_char, (*error).data);
        error = (*error).next;
    }
}
pub unsafe extern "C" fn jot_error(mut msg: *const libc::c_char, mut args: ...) {
    let mut error: *mut linestruct = make_new_node(errors_tail);
    let mut textbuf: [libc::c_char; 4296] = [0; 4296];
    let mut length: libc::c_int = 0 as libc::c_int;
    let mut ap: ::std::ffi::VaListImpl;
    if errors_head.is_null() {
        errors_head = error;
    } else {
        (*errors_tail).next = error;
    }
    errors_tail = error;
    if startup_problem.is_null() {
        if !nanorc.is_null() {
            snprintf(
                textbuf.as_mut_ptr(),
                (4096 as libc::c_int + 200 as libc::c_int) as libc::c_ulong,
                dcgettext(
                    0 as *const libc::c_char,
                    b"Mistakes in '%s'\0" as *const u8 as *const libc::c_char,
                    5 as libc::c_int,
                ),
                nanorc,
            );
            startup_problem = copy_of(textbuf.as_mut_ptr());
        } else {
            startup_problem = copy_of(
                dcgettext(
                    0 as *const libc::c_char,
                    b"Problems with history file\0" as *const u8 as *const libc::c_char,
                    5 as libc::c_int,
                ),
            );
        }
    }
    if lineno > 0 as libc::c_int as libc::c_ulong {
        length = snprintf(
            textbuf.as_mut_ptr(),
            (4096 as libc::c_int + 200 as libc::c_int) as libc::c_ulong,
            dcgettext(
                0 as *const libc::c_char,
                b"Error in %s on line %zu: \0" as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ),
            nanorc,
            lineno,
        );
    }
    ap = args.clone();
    length
        += vsnprintf(
            textbuf.as_mut_ptr().offset(length as isize),
            (4096 as libc::c_int + 200 as libc::c_int - length) as libc::c_ulong,
            dcgettext(0 as *const libc::c_char, msg, 5 as libc::c_int),
            ap.as_va_list(),
        );
    (*error).data = nmalloc((length + 1 as libc::c_int) as size_t) as *mut libc::c_char;
    sprintf(
        (*error).data,
        b"%s\0" as *const u8 as *const libc::c_char,
        textbuf.as_mut_ptr(),
    );
}
pub unsafe extern "C" fn strtosc(mut input: *const libc::c_char) -> *mut keystruct {
    let mut s: *mut keystruct = nmalloc(
        ::std::mem::size_of::<keystruct>() as libc::c_ulong,
    ) as *mut keystruct;
    (*s).toggle = 0 as libc::c_int;
    if strcmp(input, b"cancel\0" as *const u8 as *const libc::c_char) == 0 {
        (*s).func = Some(do_cancel as unsafe extern "C" fn() -> ());
    } else if strcmp(input, b"help\0" as *const u8 as *const libc::c_char) == 0 {
        (*s).func = Some(do_help as unsafe extern "C" fn() -> ());
    } else if strcmp(input, b"exit\0" as *const u8 as *const libc::c_char) == 0 {
        (*s).func = Some(do_exit as unsafe extern "C" fn() -> ());
    } else if strcmp(input, b"discardbuffer\0" as *const u8 as *const libc::c_char) == 0
    {
        (*s).func = Some(discard_buffer as unsafe extern "C" fn() -> ());
    } else if strcmp(input, b"writeout\0" as *const u8 as *const libc::c_char) == 0 {
        (*s).func = Some(do_writeout as unsafe extern "C" fn() -> ());
    } else if strcmp(input, b"savefile\0" as *const u8 as *const libc::c_char) == 0 {
        (*s).func = Some(do_savefile as unsafe extern "C" fn() -> ());
    } else if strcmp(input, b"insert\0" as *const u8 as *const libc::c_char) == 0 {
        (*s).func = Some(do_insertfile as unsafe extern "C" fn() -> ());
    } else if strcmp(input, b"whereis\0" as *const u8 as *const libc::c_char) == 0 {
        (*s).func = Some(do_search_forward as unsafe extern "C" fn() -> ());
    } else if strcmp(input, b"wherewas\0" as *const u8 as *const libc::c_char) == 0 {
        (*s).func = Some(do_search_backward as unsafe extern "C" fn() -> ());
    } else if strcmp(input, b"findprevious\0" as *const u8 as *const libc::c_char) == 0 {
        (*s).func = Some(do_findprevious as unsafe extern "C" fn() -> ());
    } else if strcmp(input, b"findnext\0" as *const u8 as *const libc::c_char) == 0 {
        (*s).func = Some(do_findnext as unsafe extern "C" fn() -> ());
    } else if strcmp(input, b"replace\0" as *const u8 as *const libc::c_char) == 0 {
        (*s).func = Some(do_replace as unsafe extern "C" fn() -> ());
    } else if strcmp(input, b"cut\0" as *const u8 as *const libc::c_char) == 0 {
        (*s).func = Some(cut_text as unsafe extern "C" fn() -> ());
    } else if strcmp(input, b"paste\0" as *const u8 as *const libc::c_char) == 0 {
        (*s).func = Some(paste_text as unsafe extern "C" fn() -> ());
    } else if strcmp(input, b"execute\0" as *const u8 as *const libc::c_char) == 0 {
        (*s).func = Some(do_execute as unsafe extern "C" fn() -> ());
    } else if strcmp(input, b"cutrestoffile\0" as *const u8 as *const libc::c_char) == 0
    {
        (*s).func = Some(cut_till_eof as unsafe extern "C" fn() -> ());
    } else if strcmp(input, b"copy\0" as *const u8 as *const libc::c_char) == 0 {
        (*s).func = Some(copy_text as unsafe extern "C" fn() -> ());
    } else if strcmp(input, b"zap\0" as *const u8 as *const libc::c_char) == 0 {
        (*s).func = Some(zap_text as unsafe extern "C" fn() -> ());
    } else if strcmp(input, b"mark\0" as *const u8 as *const libc::c_char) == 0 {
        (*s).func = Some(do_mark as unsafe extern "C" fn() -> ());
    } else if strcmp(input, b"tospell\0" as *const u8 as *const libc::c_char) == 0
        || strcmp(input, b"speller\0" as *const u8 as *const libc::c_char) == 0
    {
        (*s).func = Some(do_spell as unsafe extern "C" fn() -> ());
    } else if strcmp(input, b"linter\0" as *const u8 as *const libc::c_char) == 0 {
        (*s).func = Some(do_linter as unsafe extern "C" fn() -> ());
    } else if strcmp(input, b"formatter\0" as *const u8 as *const libc::c_char) == 0 {
        (*s).func = Some(do_formatter as unsafe extern "C" fn() -> ());
    } else if strcmp(input, b"location\0" as *const u8 as *const libc::c_char) == 0 {
        (*s).func = Some(report_cursor_position as unsafe extern "C" fn() -> ());
    } else if strcmp(input, b"gotoline\0" as *const u8 as *const libc::c_char) == 0 {
        (*s).func = Some(do_gotolinecolumn as unsafe extern "C" fn() -> ());
    } else if strcmp(input, b"justify\0" as *const u8 as *const libc::c_char) == 0 {
        (*s).func = Some(do_justify as unsafe extern "C" fn() -> ());
    } else if strcmp(input, b"fulljustify\0" as *const u8 as *const libc::c_char) == 0 {
        (*s).func = Some(do_full_justify as unsafe extern "C" fn() -> ());
    } else if strcmp(input, b"beginpara\0" as *const u8 as *const libc::c_char) == 0 {
        (*s).func = Some(to_para_begin as unsafe extern "C" fn() -> ());
    } else if strcmp(input, b"endpara\0" as *const u8 as *const libc::c_char) == 0 {
        (*s).func = Some(to_para_end as unsafe extern "C" fn() -> ());
    } else if strcmp(input, b"comment\0" as *const u8 as *const libc::c_char) == 0 {
        (*s).func = Some(do_comment as unsafe extern "C" fn() -> ());
    } else if strcmp(input, b"complete\0" as *const u8 as *const libc::c_char) == 0 {
        (*s).func = Some(complete_a_word as unsafe extern "C" fn() -> ());
    } else if strcmp(input, b"indent\0" as *const u8 as *const libc::c_char) == 0 {
        (*s).func = Some(do_indent as unsafe extern "C" fn() -> ());
    } else if strcmp(input, b"unindent\0" as *const u8 as *const libc::c_char) == 0 {
        (*s).func = Some(do_unindent as unsafe extern "C" fn() -> ());
    } else if strcmp(input, b"chopwordleft\0" as *const u8 as *const libc::c_char) == 0 {
        (*s).func = Some(chop_previous_word as unsafe extern "C" fn() -> ());
    } else if strcmp(input, b"chopwordright\0" as *const u8 as *const libc::c_char) == 0
    {
        (*s).func = Some(chop_next_word as unsafe extern "C" fn() -> ());
    } else if strcmp(input, b"findbracket\0" as *const u8 as *const libc::c_char) == 0 {
        (*s).func = Some(do_find_bracket as unsafe extern "C" fn() -> ());
    } else if strcmp(input, b"wordcount\0" as *const u8 as *const libc::c_char) == 0 {
        (*s)
            .func = Some(
            count_lines_words_and_characters as unsafe extern "C" fn() -> (),
        );
    } else if strcmp(input, b"recordmacro\0" as *const u8 as *const libc::c_char) == 0 {
        (*s).func = Some(record_macro as unsafe extern "C" fn() -> ());
    } else if strcmp(input, b"runmacro\0" as *const u8 as *const libc::c_char) == 0 {
        (*s).func = Some(run_macro as unsafe extern "C" fn() -> ());
    } else if strcmp(input, b"anchor\0" as *const u8 as *const libc::c_char) == 0 {
        (*s).func = Some(put_or_lift_anchor as unsafe extern "C" fn() -> ());
    } else if strcmp(input, b"prevanchor\0" as *const u8 as *const libc::c_char) == 0 {
        (*s).func = Some(to_prev_anchor as unsafe extern "C" fn() -> ());
    } else if strcmp(input, b"nextanchor\0" as *const u8 as *const libc::c_char) == 0 {
        (*s).func = Some(to_next_anchor as unsafe extern "C" fn() -> ());
    } else if strcmp(input, b"undo\0" as *const u8 as *const libc::c_char) == 0 {
        (*s).func = Some(do_undo as unsafe extern "C" fn() -> ());
    } else if strcmp(input, b"redo\0" as *const u8 as *const libc::c_char) == 0 {
        (*s).func = Some(do_redo as unsafe extern "C" fn() -> ());
    } else if strcmp(input, b"left\0" as *const u8 as *const libc::c_char) == 0
        || strcmp(input, b"back\0" as *const u8 as *const libc::c_char) == 0
    {
        (*s).func = Some(do_left as unsafe extern "C" fn() -> ());
    } else if strcmp(input, b"right\0" as *const u8 as *const libc::c_char) == 0
        || strcmp(input, b"forward\0" as *const u8 as *const libc::c_char) == 0
    {
        (*s).func = Some(do_right as unsafe extern "C" fn() -> ());
    } else if strcmp(input, b"up\0" as *const u8 as *const libc::c_char) == 0
        || strcmp(input, b"prevline\0" as *const u8 as *const libc::c_char) == 0
    {
        (*s).func = Some(do_up as unsafe extern "C" fn() -> ());
    } else if strcmp(input, b"down\0" as *const u8 as *const libc::c_char) == 0
        || strcmp(input, b"nextline\0" as *const u8 as *const libc::c_char) == 0
    {
        (*s).func = Some(do_down as unsafe extern "C" fn() -> ());
    } else if strcmp(input, b"scrollup\0" as *const u8 as *const libc::c_char) == 0 {
        (*s).func = Some(do_scroll_up as unsafe extern "C" fn() -> ());
    } else if strcmp(input, b"scrolldown\0" as *const u8 as *const libc::c_char) == 0 {
        (*s).func = Some(do_scroll_down as unsafe extern "C" fn() -> ());
    } else if strcmp(input, b"center\0" as *const u8 as *const libc::c_char) == 0 {
        (*s).func = Some(do_center as unsafe extern "C" fn() -> ());
    } else if strcmp(input, b"prevword\0" as *const u8 as *const libc::c_char) == 0 {
        (*s).func = Some(to_prev_word as unsafe extern "C" fn() -> ());
    } else if strcmp(input, b"nextword\0" as *const u8 as *const libc::c_char) == 0 {
        (*s).func = Some(to_next_word as unsafe extern "C" fn() -> ());
    } else if strcmp(input, b"home\0" as *const u8 as *const libc::c_char) == 0 {
        (*s).func = Some(do_home as unsafe extern "C" fn() -> ());
    } else if strcmp(input, b"end\0" as *const u8 as *const libc::c_char) == 0 {
        (*s).func = Some(do_end as unsafe extern "C" fn() -> ());
    } else if strcmp(input, b"prevblock\0" as *const u8 as *const libc::c_char) == 0 {
        (*s).func = Some(to_prev_block as unsafe extern "C" fn() -> ());
    } else if strcmp(input, b"nextblock\0" as *const u8 as *const libc::c_char) == 0 {
        (*s).func = Some(to_next_block as unsafe extern "C" fn() -> ());
    } else if strcmp(input, b"pageup\0" as *const u8 as *const libc::c_char) == 0
        || strcmp(input, b"prevpage\0" as *const u8 as *const libc::c_char) == 0
    {
        (*s).func = Some(do_page_up as unsafe extern "C" fn() -> ());
    } else if strcmp(input, b"pagedown\0" as *const u8 as *const libc::c_char) == 0
        || strcmp(input, b"nextpage\0" as *const u8 as *const libc::c_char) == 0
    {
        (*s).func = Some(do_page_down as unsafe extern "C" fn() -> ());
    } else if strcmp(input, b"firstline\0" as *const u8 as *const libc::c_char) == 0 {
        (*s).func = Some(to_first_line as unsafe extern "C" fn() -> ());
    } else if strcmp(input, b"lastline\0" as *const u8 as *const libc::c_char) == 0 {
        (*s).func = Some(to_last_line as unsafe extern "C" fn() -> ());
    } else if strcmp(input, b"prevbuf\0" as *const u8 as *const libc::c_char) == 0 {
        (*s).func = Some(switch_to_prev_buffer as unsafe extern "C" fn() -> ());
    } else if strcmp(input, b"nextbuf\0" as *const u8 as *const libc::c_char) == 0 {
        (*s).func = Some(switch_to_next_buffer as unsafe extern "C" fn() -> ());
    } else if strcmp(input, b"verbatim\0" as *const u8 as *const libc::c_char) == 0 {
        (*s).func = Some(do_verbatim_input as unsafe extern "C" fn() -> ());
    } else if strcmp(input, b"tab\0" as *const u8 as *const libc::c_char) == 0 {
        (*s).func = Some(do_tab as unsafe extern "C" fn() -> ());
    } else if strcmp(input, b"enter\0" as *const u8 as *const libc::c_char) == 0 {
        (*s).func = Some(do_enter as unsafe extern "C" fn() -> ());
    } else if strcmp(input, b"delete\0" as *const u8 as *const libc::c_char) == 0 {
        (*s).func = Some(do_delete as unsafe extern "C" fn() -> ());
    } else if strcmp(input, b"backspace\0" as *const u8 as *const libc::c_char) == 0 {
        (*s).func = Some(do_backspace as unsafe extern "C" fn() -> ());
    } else if strcmp(input, b"refresh\0" as *const u8 as *const libc::c_char) == 0 {
        (*s).func = Some(full_refresh as unsafe extern "C" fn() -> ());
    } else if strcmp(input, b"suspend\0" as *const u8 as *const libc::c_char) == 0 {
        (*s).func = Some(do_suspend as unsafe extern "C" fn() -> ());
    } else if strcmp(input, b"casesens\0" as *const u8 as *const libc::c_char) == 0 {
        (*s).func = Some(case_sens_void as unsafe extern "C" fn() -> ());
    } else if strcmp(input, b"regexp\0" as *const u8 as *const libc::c_char) == 0 {
        (*s).func = Some(regexp_void as unsafe extern "C" fn() -> ());
    } else if strcmp(input, b"backwards\0" as *const u8 as *const libc::c_char) == 0 {
        (*s).func = Some(backwards_void as unsafe extern "C" fn() -> ());
    } else if strcmp(input, b"flipreplace\0" as *const u8 as *const libc::c_char) == 0 {
        (*s).func = Some(flip_replace as unsafe extern "C" fn() -> ());
    } else if strcmp(input, b"flipgoto\0" as *const u8 as *const libc::c_char) == 0 {
        (*s).func = Some(flip_goto as unsafe extern "C" fn() -> ());
    } else if strcmp(input, b"older\0" as *const u8 as *const libc::c_char) == 0 {
        (*s).func = Some(get_older_item as unsafe extern "C" fn() -> ());
    } else if strcmp(input, b"newer\0" as *const u8 as *const libc::c_char) == 0 {
        (*s).func = Some(get_newer_item as unsafe extern "C" fn() -> ());
    } else if strcmp(input, b"dosformat\0" as *const u8 as *const libc::c_char) == 0 {
        (*s).func = Some(dos_format as unsafe extern "C" fn() -> ());
    } else if strcmp(input, b"macformat\0" as *const u8 as *const libc::c_char) == 0 {
        (*s).func = Some(mac_format as unsafe extern "C" fn() -> ());
    } else if strcmp(input, b"append\0" as *const u8 as *const libc::c_char) == 0 {
        (*s).func = Some(append_it as unsafe extern "C" fn() -> ());
    } else if strcmp(input, b"prepend\0" as *const u8 as *const libc::c_char) == 0 {
        (*s).func = Some(prepend_it as unsafe extern "C" fn() -> ());
    } else if strcmp(input, b"backup\0" as *const u8 as *const libc::c_char) == 0 {
        (*s).func = Some(back_it_up as unsafe extern "C" fn() -> ());
    } else if strcmp(input, b"flipexecute\0" as *const u8 as *const libc::c_char) == 0 {
        (*s).func = Some(flip_execute as unsafe extern "C" fn() -> ());
    } else if strcmp(input, b"flippipe\0" as *const u8 as *const libc::c_char) == 0 {
        (*s).func = Some(flip_pipe as unsafe extern "C" fn() -> ());
    } else if strcmp(input, b"flipconvert\0" as *const u8 as *const libc::c_char) == 0 {
        (*s).func = Some(flip_convert as unsafe extern "C" fn() -> ());
    } else if strcmp(input, b"flipnewbuffer\0" as *const u8 as *const libc::c_char) == 0
    {
        (*s).func = Some(flip_newbuffer as unsafe extern "C" fn() -> ());
    } else if strcmp(input, b"tofiles\0" as *const u8 as *const libc::c_char) == 0
        || strcmp(input, b"browser\0" as *const u8 as *const libc::c_char) == 0
    {
        (*s).func = Some(to_files as unsafe extern "C" fn() -> ());
    } else if strcmp(input, b"gotodir\0" as *const u8 as *const libc::c_char) == 0 {
        (*s).func = Some(goto_dir as unsafe extern "C" fn() -> ());
    } else if strcmp(input, b"firstfile\0" as *const u8 as *const libc::c_char) == 0 {
        (*s).func = Some(to_first_file as unsafe extern "C" fn() -> ());
    } else if strcmp(input, b"lastfile\0" as *const u8 as *const libc::c_char) == 0 {
        (*s).func = Some(to_last_file as unsafe extern "C" fn() -> ());
    } else {
        (*s).func = Some(do_toggle as unsafe extern "C" fn() -> ());
        if strcmp(input, b"nohelp\0" as *const u8 as *const libc::c_char) == 0 {
            (*s).toggle = NO_HELP as libc::c_int;
        } else if strcmp(input, b"zero\0" as *const u8 as *const libc::c_char) == 0 {
            (*s).toggle = ZERO as libc::c_int;
        } else if strcmp(input, b"constantshow\0" as *const u8 as *const libc::c_char)
            == 0
        {
            (*s).toggle = CONSTANT_SHOW as libc::c_int;
        } else if strcmp(input, b"softwrap\0" as *const u8 as *const libc::c_char) == 0 {
            (*s).toggle = SOFTWRAP as libc::c_int;
        } else if strcmp(input, b"linenumbers\0" as *const u8 as *const libc::c_char)
            == 0
        {
            (*s).toggle = LINE_NUMBERS as libc::c_int;
        } else if strcmp(
            input,
            b"whitespacedisplay\0" as *const u8 as *const libc::c_char,
        ) == 0
        {
            (*s).toggle = WHITESPACE_DISPLAY as libc::c_int;
        } else if strcmp(input, b"nosyntax\0" as *const u8 as *const libc::c_char) == 0 {
            (*s).toggle = NO_SYNTAX as libc::c_int;
        } else if strcmp(input, b"smarthome\0" as *const u8 as *const libc::c_char) == 0
        {
            (*s).toggle = SMART_HOME as libc::c_int;
        } else if strcmp(input, b"autoindent\0" as *const u8 as *const libc::c_char) == 0
        {
            (*s).toggle = AUTOINDENT as libc::c_int;
        } else if strcmp(input, b"cutfromcursor\0" as *const u8 as *const libc::c_char)
            == 0
        {
            (*s).toggle = CUT_FROM_CURSOR as libc::c_int;
        } else if strcmp(input, b"breaklonglines\0" as *const u8 as *const libc::c_char)
            == 0 || strcmp(input, b"nowrap\0" as *const u8 as *const libc::c_char) == 0
        {
            (*s).toggle = BREAK_LONG_LINES as libc::c_int;
        } else if strcmp(input, b"tabstospaces\0" as *const u8 as *const libc::c_char)
            == 0
        {
            (*s).toggle = TABS_TO_SPACES as libc::c_int;
        } else if strcmp(input, b"mouse\0" as *const u8 as *const libc::c_char) == 0 {
            (*s).toggle = USE_MOUSE as libc::c_int;
        } else {
            rpl_free(s as *mut libc::c_void);
            return 0 as *mut keystruct;
        }
    }
    return s;
}
pub static mut menunames: [*mut libc::c_char; 16] = [
    b"main\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"search\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"replace\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"replacewith\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"yesno\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"gotoline\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"writeout\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"insert\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"execute\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"help\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"spell\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"linter\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"browser\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"whereisfile\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"gotodir\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"all\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
];
pub static mut menusymbols: [libc::c_int; 16] = [
    (1 as libc::c_int) << 0 as libc::c_int,
    (1 as libc::c_int) << 1 as libc::c_int,
    (1 as libc::c_int) << 2 as libc::c_int,
    (1 as libc::c_int) << 3 as libc::c_int,
    (1 as libc::c_int) << 13 as libc::c_int,
    (1 as libc::c_int) << 4 as libc::c_int,
    (1 as libc::c_int) << 5 as libc::c_int,
    (1 as libc::c_int) << 6 as libc::c_int,
    (1 as libc::c_int) << 7 as libc::c_int,
    (1 as libc::c_int) << 8 as libc::c_int,
    (1 as libc::c_int) << 9 as libc::c_int,
    (1 as libc::c_int) << 14 as libc::c_int,
    (1 as libc::c_int) << 10 as libc::c_int,
    (1 as libc::c_int) << 11 as libc::c_int,
    (1 as libc::c_int) << 12 as libc::c_int,
    (1 as libc::c_int) << 0 as libc::c_int | (1 as libc::c_int) << 1 as libc::c_int
        | (1 as libc::c_int) << 2 as libc::c_int | (1 as libc::c_int) << 3 as libc::c_int
        | (1 as libc::c_int) << 4 as libc::c_int | (1 as libc::c_int) << 5 as libc::c_int
        | (1 as libc::c_int) << 6 as libc::c_int | (1 as libc::c_int) << 7 as libc::c_int
        | (1 as libc::c_int) << 11 as libc::c_int
        | (1 as libc::c_int) << 12 as libc::c_int
        | (1 as libc::c_int) << 15 as libc::c_int
        | (1 as libc::c_int) << 9 as libc::c_int
        | (1 as libc::c_int) << 14 as libc::c_int
        | (1 as libc::c_int) << 10 as libc::c_int
        | (1 as libc::c_int) << 8 as libc::c_int
        | (1 as libc::c_int) << 13 as libc::c_int,
];
pub unsafe extern "C" fn name_to_menu(mut name: *const libc::c_char) -> libc::c_int {
    let mut index: libc::c_int = -(1 as libc::c_int);
    loop {
        index += 1;
        if !(index < 16 as libc::c_int) {
            break;
        }
        if strcmp(name, menunames[index as usize]) == 0 as libc::c_int {
            return menusymbols[index as usize];
        }
    }
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn menu_to_name(mut menu: libc::c_int) -> *mut libc::c_char {
    let mut index: libc::c_int = -(1 as libc::c_int);
    loop {
        index += 1;
        if !(index < 16 as libc::c_int) {
            break;
        }
        if menusymbols[index as usize] == menu {
            return menunames[index as usize];
        }
    }
    return b"boooo\0" as *const u8 as *const libc::c_char as *mut libc::c_char;
}
pub unsafe extern "C" fn parse_next_word(
    mut ptr: *mut libc::c_char,
) -> *mut libc::c_char {
    while *(*__ctype_b_loc()).offset(*ptr as libc::c_uchar as libc::c_int as isize)
        as libc::c_int & _ISblank as libc::c_int as libc::c_ushort as libc::c_int == 0
        && *ptr as libc::c_int != '\0' as i32
    {
        ptr = ptr.offset(1);
        ptr;
    }
    if *ptr as libc::c_int == '\0' as i32 {
        return ptr;
    }
    let fresh0 = ptr;
    ptr = ptr.offset(1);
    *fresh0 = '\0' as i32 as libc::c_char;
    while *(*__ctype_b_loc()).offset(*ptr as libc::c_uchar as libc::c_int as isize)
        as libc::c_int & _ISblank as libc::c_int as libc::c_ushort as libc::c_int != 0
    {
        ptr = ptr.offset(1);
        ptr;
    }
    return ptr;
}
pub unsafe extern "C" fn parse_argument(
    mut ptr: *mut libc::c_char,
) -> *mut libc::c_char {
    let mut ptr_save: *const libc::c_char = ptr;
    let mut last_quote: *mut libc::c_char = 0 as *mut libc::c_char;
    if *ptr as libc::c_int != '"' as i32 {
        return parse_next_word(ptr);
    }
    while *ptr as libc::c_int != '\0' as i32 {
        ptr = ptr.offset(1);
        if *ptr as libc::c_int == '"' as i32 {
            last_quote = ptr;
        }
    }
    if last_quote.is_null() {
        jot_error(
            b"Argument '%s' has an unterminated \"\0" as *const u8
                as *const libc::c_char,
            ptr_save,
        );
        return 0 as *mut libc::c_char;
    }
    *last_quote = '\0' as i32 as libc::c_char;
    ptr = last_quote.offset(1 as libc::c_int as isize);
    while *(*__ctype_b_loc()).offset(*ptr as libc::c_uchar as libc::c_int as isize)
        as libc::c_int & _ISblank as libc::c_int as libc::c_ushort as libc::c_int != 0
    {
        ptr = ptr.offset(1);
        ptr;
    }
    return ptr;
}
pub unsafe extern "C" fn parse_next_regex(
    mut ptr: *mut libc::c_char,
) -> *mut libc::c_char {
    let mut starting_point: *mut libc::c_char = ptr;
    if *ptr.offset(-(1 as libc::c_int as isize)) as libc::c_int != '"' as i32 {
        jot_error(
            b"Regex strings must begin and end with a \" character\0" as *const u8
                as *const libc::c_char,
        );
        return 0 as *mut libc::c_char;
    }
    while *ptr as libc::c_int != '\0' as i32
        && (*ptr as libc::c_int != '"' as i32
            || *ptr.offset(1 as libc::c_int as isize) as libc::c_int != '\0' as i32
                && *(*__ctype_b_loc())
                    .offset(
                        *ptr.offset(1 as libc::c_int as isize) as libc::c_uchar
                            as libc::c_int as isize,
                    ) as libc::c_int
                    & _ISblank as libc::c_int as libc::c_ushort as libc::c_int == 0)
    {
        ptr = ptr.offset(1);
        ptr;
    }
    if *ptr as libc::c_int == '\0' as i32 {
        jot_error(
            b"Regex strings must begin and end with a \" character\0" as *const u8
                as *const libc::c_char,
        );
        return 0 as *mut libc::c_char;
    }
    if ptr == starting_point {
        jot_error(b"Empty regex string\0" as *const u8 as *const libc::c_char);
        return 0 as *mut libc::c_char;
    }
    let fresh1 = ptr;
    ptr = ptr.offset(1);
    *fresh1 = '\0' as i32 as libc::c_char;
    while *(*__ctype_b_loc()).offset(*ptr as libc::c_uchar as libc::c_int as isize)
        as libc::c_int & _ISblank as libc::c_int as libc::c_ushort as libc::c_int != 0
    {
        ptr = ptr.offset(1);
        ptr;
    }
    return ptr;
}
pub unsafe extern "C" fn compile(
    mut expression: *const libc::c_char,
    mut rex_flags: libc::c_int,
    mut packed: *mut *mut regex_t,
) -> bool {
    let mut compiled: *mut regex_t = nmalloc(
        ::std::mem::size_of::<regex_t>() as libc::c_ulong,
    ) as *mut regex_t;
    let mut outcome: libc::c_int = rpl_regcomp(compiled, expression, rex_flags);
    if outcome != 0 as libc::c_int {
        let mut length: size_t = rpl_regerror(
            outcome,
            compiled,
            0 as *mut libc::c_char,
            0 as libc::c_int as size_t,
        );
        let mut message: *mut libc::c_char = nmalloc(length) as *mut libc::c_char;
        rpl_regerror(outcome, compiled, message, length);
        jot_error(
            b"Bad regex \"%s\": %s\0" as *const u8 as *const libc::c_char,
            expression,
            message,
        );
        rpl_free(message as *mut libc::c_void);
        rpl_regfree(compiled);
        rpl_free(compiled as *mut libc::c_void);
    } else {
        *packed = compiled;
    }
    return outcome == 0 as libc::c_int;
}
pub unsafe extern "C" fn begin_new_syntax(mut ptr: *mut libc::c_char) {
    let mut nameptr: *mut libc::c_char = ptr;
    if *ptr as libc::c_int == '\0' as i32
        || *ptr as libc::c_int == '"' as i32
            && (*ptr.offset(1 as libc::c_int as isize) as libc::c_int == '\0' as i32
                || *ptr.offset(1 as libc::c_int as isize) as libc::c_int == '"' as i32)
    {
        jot_error(b"Missing syntax name\0" as *const u8 as *const libc::c_char);
        return;
    }
    ptr = parse_next_word(ptr);
    if (*nameptr as libc::c_int == '"' as i32) as libc::c_int
        ^ (*nameptr
            .offset(
                (strlen(nameptr)).wrapping_sub(1 as libc::c_int as libc::c_ulong)
                    as isize,
            ) as libc::c_int == '"' as i32) as libc::c_int != 0
    {
        jot_error(
            b"Unpaired quote in syntax name\0" as *const u8 as *const libc::c_char,
        );
        return;
    }
    if *nameptr as libc::c_int == '"' as i32 {
        nameptr = nameptr.offset(1);
        nameptr;
        *nameptr
            .offset(
                (strlen(nameptr)).wrapping_sub(1 as libc::c_int as libc::c_ulong)
                    as isize,
            ) = '\0' as i32 as libc::c_char;
    }
    if strcmp(nameptr, b"none\0" as *const u8 as *const libc::c_char) == 0 as libc::c_int
    {
        jot_error(
            b"The \"none\" syntax is reserved\0" as *const u8 as *const libc::c_char,
        );
        return;
    }
    live_syntax = nmalloc(::std::mem::size_of::<syntaxtype>() as libc::c_ulong)
        as *mut syntaxtype;
    (*live_syntax).name = copy_of(nameptr);
    (*live_syntax).filename = copy_of(nanorc);
    (*live_syntax).lineno = lineno;
    (*live_syntax).augmentations = 0 as *mut augmentstruct;
    (*live_syntax).extensions = 0 as *mut regexlisttype;
    (*live_syntax).headers = 0 as *mut regexlisttype;
    (*live_syntax).magics = 0 as *mut regexlisttype;
    (*live_syntax).linter = 0 as *mut libc::c_char;
    (*live_syntax).formatter = 0 as *mut libc::c_char;
    (*live_syntax).tab = 0 as *mut libc::c_char;
    (*live_syntax).comment = copy_of(b"#\0" as *const u8 as *const libc::c_char);
    (*live_syntax).color = 0 as *mut colortype;
    (*live_syntax).nmultis = 0 as libc::c_int as libc::c_short;
    (*live_syntax).next = syntaxes;
    syntaxes = live_syntax;
    opensyntax = 1 as libc::c_int != 0;
    seen_color_command = 0 as libc::c_int != 0;
    if strcmp((*live_syntax).name, b"default\0" as *const u8 as *const libc::c_char)
        == 0 as libc::c_int && *ptr as libc::c_int != '\0' as i32
    {
        jot_error(
            b"The \"default\" syntax does not accept extensions\0" as *const u8
                as *const libc::c_char,
        );
        return;
    }
    if *ptr as libc::c_int != '\0' as i32 {
        grab_and_store(
            b"extension\0" as *const u8 as *const libc::c_char,
            ptr,
            &mut (*live_syntax).extensions,
        );
    }
}
pub unsafe extern "C" fn check_for_nonempty_syntax() {
    if opensyntax as libc::c_int != 0 && !seen_color_command {
        let mut current_lineno: size_t = lineno;
        lineno = (*live_syntax).lineno;
        jot_error(
            b"Syntax \"%s\" has no color commands\0" as *const u8 as *const libc::c_char,
            (*live_syntax).name,
        );
        lineno = current_lineno;
    }
    opensyntax = 0 as libc::c_int != 0;
}
pub unsafe extern "C" fn is_universal(
    mut func: Option::<unsafe extern "C" fn() -> ()>,
) -> bool {
    return func == Some(do_left as unsafe extern "C" fn() -> ())
        || func == Some(do_right as unsafe extern "C" fn() -> ())
        || func == Some(do_home as unsafe extern "C" fn() -> ())
        || func == Some(do_end as unsafe extern "C" fn() -> ())
        || func == Some(to_prev_word as unsafe extern "C" fn() -> ())
        || func == Some(to_next_word as unsafe extern "C" fn() -> ())
        || func == Some(do_delete as unsafe extern "C" fn() -> ())
        || func == Some(do_backspace as unsafe extern "C" fn() -> ())
        || func == Some(cut_text as unsafe extern "C" fn() -> ())
        || func == Some(paste_text as unsafe extern "C" fn() -> ())
        || func == Some(do_tab as unsafe extern "C" fn() -> ())
        || func == Some(do_enter as unsafe extern "C" fn() -> ())
        || func == Some(do_verbatim_input as unsafe extern "C" fn() -> ());
}
pub unsafe extern "C" fn parse_binding(mut ptr: *mut libc::c_char, mut dobind: bool) {
    let mut current_block: u64;
    let mut keyptr: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut keycopy: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut funcptr: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut menuptr: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut keycode: libc::c_int = 0;
    let mut menu: libc::c_int = 0;
    let mut mask: libc::c_int = 0 as libc::c_int;
    let mut newsc: *mut keystruct = 0 as *mut keystruct;
    check_for_nonempty_syntax();
    if *ptr as libc::c_int == '\0' as i32 {
        jot_error(b"Missing key name\0" as *const u8 as *const libc::c_char);
        return;
    }
    keyptr = ptr;
    ptr = parse_next_word(ptr);
    keycopy = copy_of(keyptr);
    if *keycopy.offset(0 as libc::c_int as isize) as libc::c_int == '^' as i32 {
        *keycopy
            .offset(
                1 as libc::c_int as isize,
            ) = ({
            let mut __res: libc::c_int = 0;
            if ::std::mem::size_of::<libc::c_uchar>() as libc::c_ulong
                > 1 as libc::c_int as libc::c_ulong
            {
                if 0 != 0 {
                    let mut __c: libc::c_int = *keycopy.offset(1 as libc::c_int as isize)
                        as libc::c_uchar as libc::c_int;
                    __res = if __c < -(128 as libc::c_int) || __c > 255 as libc::c_int {
                        __c
                    } else {
                        *(*__ctype_toupper_loc()).offset(__c as isize)
                    };
                } else {
                    __res = toupper(
                        *keycopy.offset(1 as libc::c_int as isize) as libc::c_uchar
                            as libc::c_int,
                    );
                }
            } else {
                __res = *(*__ctype_toupper_loc())
                    .offset(
                        *keycopy.offset(1 as libc::c_int as isize) as libc::c_uchar
                            as libc::c_int as isize,
                    );
            }
            __res
        }) as libc::c_char;
    } else {
        *keycopy
            .offset(
                0 as libc::c_int as isize,
            ) = ({
            let mut __res: libc::c_int = 0;
            if ::std::mem::size_of::<libc::c_uchar>() as libc::c_ulong
                > 1 as libc::c_int as libc::c_ulong
            {
                if 0 != 0 {
                    let mut __c: libc::c_int = *keycopy.offset(0 as libc::c_int as isize)
                        as libc::c_uchar as libc::c_int;
                    __res = if __c < -(128 as libc::c_int) || __c > 255 as libc::c_int {
                        __c
                    } else {
                        *(*__ctype_toupper_loc()).offset(__c as isize)
                    };
                } else {
                    __res = toupper(
                        *keycopy.offset(0 as libc::c_int as isize) as libc::c_uchar
                            as libc::c_int,
                    );
                }
            } else {
                __res = *(*__ctype_toupper_loc())
                    .offset(
                        *keycopy.offset(0 as libc::c_int as isize) as libc::c_uchar
                            as libc::c_int as isize,
                    );
            }
            __res
        }) as libc::c_char;
    }
    if *keycopy.offset(1 as libc::c_int as isize) as libc::c_int == '\0' as i32
        || *keycopy.offset(0 as libc::c_int as isize) as libc::c_int == 'M' as i32
            && *keycopy.offset(2 as libc::c_int as isize) as libc::c_int == '\0' as i32
    {
        jot_error(
            b"Key name %s is invalid\0" as *const u8 as *const libc::c_char,
            keycopy,
        );
    } else {
        keycode = keycode_from_string(keycopy);
        if keycode < 0 as libc::c_int {
            jot_error(
                b"Key name %s is invalid\0" as *const u8 as *const libc::c_char,
                keycopy,
            );
        } else {
            if dobind {
                funcptr = ptr;
                ptr = parse_argument(ptr);
                if *funcptr.offset(0 as libc::c_int as isize) as libc::c_int
                    == '\0' as i32
                {
                    jot_error(
                        b"Must specify a function to bind the key to\0" as *const u8
                            as *const libc::c_char,
                    );
                    current_block = 420721533736882009;
                } else if ptr.is_null() {
                    current_block = 420721533736882009;
                } else {
                    current_block = 6057473163062296781;
                }
            } else {
                current_block = 6057473163062296781;
            }
            match current_block {
                420721533736882009 => {}
                _ => {
                    menuptr = ptr;
                    ptr = parse_next_word(ptr);
                    if *menuptr.offset(0 as libc::c_int as isize) as libc::c_int
                        == '\0' as i32
                    {
                        jot_error(
                            b"Must specify a menu (or \"all\") in which to bind/unbind the key\0"
                                as *const u8 as *const libc::c_char,
                        );
                    } else {
                        menu = name_to_menu(menuptr);
                        if menu == 0 as libc::c_int {
                            jot_error(
                                b"Unknown menu: %s\0" as *const u8 as *const libc::c_char,
                                menuptr,
                            );
                        } else {
                            if dobind {
                                if *funcptr as libc::c_int == '"' as i32 {
                                    newsc = nmalloc(
                                        ::std::mem::size_of::<keystruct>() as libc::c_ulong,
                                    ) as *mut keystruct;
                                    (*newsc)
                                        .func = ::std::mem::transmute::<
                                        Option::<unsafe extern "C" fn(*const libc::c_char) -> ()>,
                                        functionptrtype,
                                    >(
                                        Some(
                                            implant as unsafe extern "C" fn(*const libc::c_char) -> (),
                                        ),
                                    );
                                    (*newsc)
                                        .expansion = copy_of(
                                        funcptr.offset(1 as libc::c_int as isize),
                                    );
                                    (*newsc).toggle = 0 as libc::c_int;
                                } else {
                                    newsc = strtosc(funcptr);
                                }
                                if newsc.is_null() {
                                    jot_error(
                                        b"Unknown function: %s\0" as *const u8
                                            as *const libc::c_char,
                                        funcptr,
                                    );
                                    current_block = 420721533736882009;
                                } else {
                                    current_block = 11913429853522160501;
                                }
                            } else {
                                current_block = 11913429853522160501;
                            }
                            match current_block {
                                420721533736882009 => {}
                                _ => {
                                    let mut s: *mut keystruct = sclist;
                                    while !s.is_null() {
                                        if (*s).menus & menu != 0 && (*s).keycode == keycode {
                                            (*s).menus &= !menu;
                                        }
                                        s = (*s).next;
                                    }
                                    if dobind {
                                        if is_universal((*newsc).func) {
                                            menu
                                                &= (1 as libc::c_int) << 0 as libc::c_int
                                                    | (1 as libc::c_int) << 1 as libc::c_int
                                                    | (1 as libc::c_int) << 2 as libc::c_int
                                                    | (1 as libc::c_int) << 3 as libc::c_int
                                                    | (1 as libc::c_int) << 4 as libc::c_int
                                                    | (1 as libc::c_int) << 5 as libc::c_int
                                                    | (1 as libc::c_int) << 6 as libc::c_int
                                                    | (1 as libc::c_int) << 7 as libc::c_int
                                                    | (1 as libc::c_int) << 11 as libc::c_int
                                                    | (1 as libc::c_int) << 12 as libc::c_int
                                                    | (1 as libc::c_int) << 15 as libc::c_int
                                                    | (1 as libc::c_int) << 9 as libc::c_int
                                                    | (1 as libc::c_int) << 14 as libc::c_int
                                                    | (1 as libc::c_int) << 10 as libc::c_int;
                                        } else if (*newsc).func
                                            == Some(do_toggle as unsafe extern "C" fn() -> ())
                                            && (*newsc).toggle == NO_HELP as libc::c_int
                                        {
                                            menu
                                                &= ((1 as libc::c_int) << 0 as libc::c_int
                                                    | (1 as libc::c_int) << 1 as libc::c_int
                                                    | (1 as libc::c_int) << 2 as libc::c_int
                                                    | (1 as libc::c_int) << 3 as libc::c_int
                                                    | (1 as libc::c_int) << 4 as libc::c_int
                                                    | (1 as libc::c_int) << 5 as libc::c_int
                                                    | (1 as libc::c_int) << 6 as libc::c_int
                                                    | (1 as libc::c_int) << 7 as libc::c_int
                                                    | (1 as libc::c_int) << 11 as libc::c_int
                                                    | (1 as libc::c_int) << 12 as libc::c_int
                                                    | (1 as libc::c_int) << 15 as libc::c_int
                                                    | (1 as libc::c_int) << 9 as libc::c_int
                                                    | (1 as libc::c_int) << 14 as libc::c_int
                                                    | (1 as libc::c_int) << 10 as libc::c_int
                                                    | (1 as libc::c_int) << 13 as libc::c_int)
                                                    & !((1 as libc::c_int) << 15 as libc::c_int);
                                        } else if (*newsc).func
                                            == Some(do_toggle as unsafe extern "C" fn() -> ())
                                        {
                                            menu &= (1 as libc::c_int) << 0 as libc::c_int;
                                        } else if (*newsc).func
                                            == Some(full_refresh as unsafe extern "C" fn() -> ())
                                        {
                                            menu
                                                &= (1 as libc::c_int) << 0 as libc::c_int
                                                    | (1 as libc::c_int) << 1 as libc::c_int
                                                    | (1 as libc::c_int) << 2 as libc::c_int
                                                    | (1 as libc::c_int) << 3 as libc::c_int
                                                    | (1 as libc::c_int) << 4 as libc::c_int
                                                    | (1 as libc::c_int) << 5 as libc::c_int
                                                    | (1 as libc::c_int) << 6 as libc::c_int
                                                    | (1 as libc::c_int) << 7 as libc::c_int
                                                    | (1 as libc::c_int) << 11 as libc::c_int
                                                    | (1 as libc::c_int) << 12 as libc::c_int
                                                    | (1 as libc::c_int) << 15 as libc::c_int
                                                    | (1 as libc::c_int) << 9 as libc::c_int
                                                    | (1 as libc::c_int) << 14 as libc::c_int
                                                    | (1 as libc::c_int) << 10 as libc::c_int
                                                    | (1 as libc::c_int) << 8 as libc::c_int
                                                    | (1 as libc::c_int) << 13 as libc::c_int;
                                        } else if (*newsc).func
                                            == ::std::mem::transmute::<
                                                Option::<unsafe extern "C" fn(*const libc::c_char) -> ()>,
                                                functionptrtype,
                                            >(
                                                Some(
                                                    implant as unsafe extern "C" fn(*const libc::c_char) -> (),
                                                ),
                                            )
                                        {
                                            menu
                                                &= (1 as libc::c_int) << 0 as libc::c_int
                                                    | (1 as libc::c_int) << 1 as libc::c_int
                                                    | (1 as libc::c_int) << 2 as libc::c_int
                                                    | (1 as libc::c_int) << 3 as libc::c_int
                                                    | (1 as libc::c_int) << 4 as libc::c_int
                                                    | (1 as libc::c_int) << 5 as libc::c_int
                                                    | (1 as libc::c_int) << 6 as libc::c_int
                                                    | (1 as libc::c_int) << 7 as libc::c_int
                                                    | (1 as libc::c_int) << 11 as libc::c_int
                                                    | (1 as libc::c_int) << 12 as libc::c_int
                                                    | (1 as libc::c_int) << 15 as libc::c_int
                                                    | (1 as libc::c_int) << 9 as libc::c_int
                                                    | (1 as libc::c_int) << 14 as libc::c_int
                                                    | (1 as libc::c_int) << 10 as libc::c_int
                                                    | (1 as libc::c_int) << 8 as libc::c_int;
                                        } else {
                                            let mut f: *mut funcstruct = allfuncs;
                                            while !f.is_null() {
                                                if (*f).func == (*newsc).func {
                                                    mask |= (*f).menus;
                                                }
                                                f = (*f).next;
                                            }
                                            menu &= mask;
                                        }
                                        if menu == 0 {
                                            if !(flags[(RESTRICTED as libc::c_int as libc::c_ulong)
                                                .wrapping_div(
                                                    (::std::mem::size_of::<libc::c_uint>() as libc::c_ulong)
                                                        .wrapping_mul(8 as libc::c_int as libc::c_ulong),
                                                ) as usize]
                                                & (1 as libc::c_int as libc::c_uint)
                                                    << (RESTRICTED as libc::c_int as libc::c_ulong)
                                                        .wrapping_rem(
                                                            (::std::mem::size_of::<libc::c_uint>() as libc::c_ulong)
                                                                .wrapping_mul(8 as libc::c_int as libc::c_ulong),
                                                        ) != 0 as libc::c_int as libc::c_uint)
                                                && !(flags[(VIEW_MODE as libc::c_int as libc::c_ulong)
                                                    .wrapping_div(
                                                        (::std::mem::size_of::<libc::c_uint>() as libc::c_ulong)
                                                            .wrapping_mul(8 as libc::c_int as libc::c_ulong),
                                                    ) as usize]
                                                    & (1 as libc::c_int as libc::c_uint)
                                                        << (VIEW_MODE as libc::c_int as libc::c_ulong)
                                                            .wrapping_rem(
                                                                (::std::mem::size_of::<libc::c_uint>() as libc::c_ulong)
                                                                    .wrapping_mul(8 as libc::c_int as libc::c_ulong),
                                                            ) != 0 as libc::c_int as libc::c_uint)
                                            {
                                                jot_error(
                                                    b"Function '%s' does not exist in menu '%s'\0" as *const u8
                                                        as *const libc::c_char,
                                                    funcptr,
                                                    menuptr,
                                                );
                                            }
                                        } else {
                                            (*newsc).menus = menu;
                                            (*newsc).keystr = keycopy;
                                            (*newsc).keycode = keycode;
                                            if (*newsc).keycode == 0x1b as libc::c_int {
                                                jot_error(
                                                    b"Keystroke %s may not be rebound\0" as *const u8
                                                        as *const libc::c_char,
                                                    keycopy,
                                                );
                                            } else {
                                                if (*newsc).func
                                                    == Some(do_toggle as unsafe extern "C" fn() -> ())
                                                {
                                                    let mut s_0: *mut keystruct = sclist;
                                                    while !s_0.is_null() {
                                                        if (*s_0).func
                                                            == Some(do_toggle as unsafe extern "C" fn() -> ())
                                                            && (*s_0).toggle == (*newsc).toggle
                                                        {
                                                            (*newsc).ordinal = (*s_0).ordinal;
                                                        }
                                                        s_0 = (*s_0).next;
                                                    }
                                                } else {
                                                    (*newsc).ordinal = 0 as libc::c_int;
                                                }
                                                (*newsc).next = sclist;
                                                sclist = newsc;
                                                return;
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
    rpl_free(keycopy as *mut libc::c_void);
    rpl_free(newsc as *mut libc::c_void);
}
pub unsafe extern "C" fn is_good_file(mut file: *mut libc::c_char) -> bool {
    let mut rcinfo: stat = stat {
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
    if access(file, 4 as libc::c_int) != 0 as libc::c_int {
        return 0 as libc::c_int != 0;
    }
    if stat(file, &mut rcinfo) != -(1 as libc::c_int)
        && (rcinfo.st_mode & 0o170000 as libc::c_int as libc::c_uint
            == 0o40000 as libc::c_int as libc::c_uint
            || rcinfo.st_mode & 0o170000 as libc::c_int as libc::c_uint
                == 0o20000 as libc::c_int as libc::c_uint
            || rcinfo.st_mode & 0o170000 as libc::c_int as libc::c_uint
                == 0o60000 as libc::c_int as libc::c_uint)
    {
        jot_error(
            if rcinfo.st_mode & 0o170000 as libc::c_int as libc::c_uint
                == 0o40000 as libc::c_int as libc::c_uint
            {
                b"\"%s\" is a directory\0" as *const u8 as *const libc::c_char
            } else {
                b"\"%s\" is a device file\0" as *const u8 as *const libc::c_char
            },
            file,
        );
        return 0 as libc::c_int != 0;
    } else {
        return 1 as libc::c_int != 0
    };
}
pub unsafe extern "C" fn parse_one_include(
    mut file: *mut libc::c_char,
    mut syntax: *mut syntaxtype,
) {
    let mut was_nanorc: *mut libc::c_char = nanorc;
    let mut was_lineno: size_t = lineno;
    let mut extra: *mut augmentstruct = 0 as *mut augmentstruct;
    let mut rcstream: *mut FILE = 0 as *mut FILE;
    if access(file, 4 as libc::c_int) == 0 as libc::c_int && !is_good_file(file) {
        return;
    }
    rcstream = fopen(file, b"rb\0" as *const u8 as *const libc::c_char);
    if rcstream.is_null() {
        jot_error(
            b"Error reading %s: %s\0" as *const u8 as *const libc::c_char,
            file,
            strerror(*__errno_location()),
        );
        return;
    }
    nanorc = file;
    lineno = 0 as libc::c_int as size_t;
    if syntax.is_null() {
        parse_rcfile(rcstream, 1 as libc::c_int != 0, 1 as libc::c_int != 0);
        nanorc = was_nanorc;
        lineno = was_lineno;
        return;
    }
    live_syntax = syntax;
    lastcolor = 0 as *mut colortype;
    parse_rcfile(rcstream, 1 as libc::c_int != 0, 0 as libc::c_int != 0);
    extra = (*syntax).augmentations;
    while !extra.is_null() {
        let mut keyword: *mut libc::c_char = (*extra).data;
        let mut therest: *mut libc::c_char = parse_next_word((*extra).data);
        nanorc = (*extra).filename;
        lineno = (*extra).lineno as size_t;
        if !parse_syntax_commands(keyword, therest) {
            jot_error(
                b"Command \"%s\" not understood\0" as *const u8 as *const libc::c_char,
                keyword,
            );
        }
        extra = (*extra).next;
    }
    rpl_free((*syntax).filename as *mut libc::c_void);
    (*syntax).filename = 0 as *mut libc::c_char;
    nanorc = was_nanorc;
    lineno = was_lineno;
}
pub unsafe extern "C" fn parse_includes(mut ptr: *mut libc::c_char) {
    let mut pattern: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut expanded: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut files: glob_t = glob_t {
        gl_pathc: 0,
        gl_pathv: 0 as *mut *mut libc::c_char,
        gl_offs: 0,
        gl_flags: 0,
        gl_closedir: None,
        gl_readdir: None,
        gl_opendir: None,
        gl_lstat: None,
        gl_stat: None,
    };
    let mut result: libc::c_int = 0;
    check_for_nonempty_syntax();
    pattern = ptr;
    if *pattern as libc::c_int == '"' as i32 {
        pattern = pattern.offset(1);
        pattern;
    }
    ptr = parse_argument(ptr);
    expanded = real_dir_from_tilde(pattern);
    result = rpl_glob(
        expanded,
        (1 as libc::c_int) << 0 as libc::c_int | (1 as libc::c_int) << 4 as libc::c_int,
        None,
        &mut files,
    );
    if result == 0 as libc::c_int {
        let mut i: size_t = 0 as libc::c_int as size_t;
        while i < files.gl_pathc {
            parse_one_include(
                *(files.gl_pathv).offset(i as isize),
                0 as *mut syntaxtype,
            );
            i = i.wrapping_add(1);
            i;
        }
    } else if result != 3 as libc::c_int {
        jot_error(
            b"Error expanding %s: %s\0" as *const u8 as *const libc::c_char,
            pattern,
            strerror(*__errno_location()),
        );
    }
    rpl_globfree(&mut files);
    rpl_free(expanded as *mut libc::c_void);
}
pub unsafe extern "C" fn closest_index_color(
    mut red: libc::c_short,
    mut green: libc::c_short,
    mut blue: libc::c_short,
) -> libc::c_short {
    static mut level: [libc::c_short; 16] = [
        0 as libc::c_int as libc::c_short,
        0 as libc::c_int as libc::c_short,
        0 as libc::c_int as libc::c_short,
        0 as libc::c_int as libc::c_short,
        1 as libc::c_int as libc::c_short,
        1 as libc::c_int as libc::c_short,
        1 as libc::c_int as libc::c_short,
        1 as libc::c_int as libc::c_short,
        2 as libc::c_int as libc::c_short,
        2 as libc::c_int as libc::c_short,
        3 as libc::c_int as libc::c_short,
        3 as libc::c_int as libc::c_short,
        4 as libc::c_int as libc::c_short,
        4 as libc::c_int as libc::c_short,
        5 as libc::c_int as libc::c_short,
        5 as libc::c_int as libc::c_short,
    ];
    if COLORS == 256 as libc::c_int {
        return (36 as libc::c_int * level[red as usize] as libc::c_int
            + 6 as libc::c_int * level[green as usize] as libc::c_int
            + level[blue as usize] as libc::c_int + 16 as libc::c_int) as libc::c_short
    } else {
        return -(1 as libc::c_int) as libc::c_short
    };
}
pub static mut hues: [[libc::c_char; 8]; 34] = unsafe {
    [
        *::std::mem::transmute::<&[u8; 8], &[libc::c_char; 8]>(b"red\0\0\0\0\0"),
        *::std::mem::transmute::<&[u8; 8], &[libc::c_char; 8]>(b"green\0\0\0"),
        *::std::mem::transmute::<&[u8; 8], &[libc::c_char; 8]>(b"blue\0\0\0\0"),
        *::std::mem::transmute::<&[u8; 8], &[libc::c_char; 8]>(b"yellow\0\0"),
        *::std::mem::transmute::<&[u8; 8], &[libc::c_char; 8]>(b"cyan\0\0\0\0"),
        *::std::mem::transmute::<&[u8; 8], &[libc::c_char; 8]>(b"magenta\0"),
        *::std::mem::transmute::<&[u8; 8], &[libc::c_char; 8]>(b"white\0\0\0"),
        *::std::mem::transmute::<&[u8; 8], &[libc::c_char; 8]>(b"black\0\0\0"),
        *::std::mem::transmute::<&[u8; 8], &[libc::c_char; 8]>(b"normal\0\0"),
        *::std::mem::transmute::<&[u8; 8], &[libc::c_char; 8]>(b"pink\0\0\0\0"),
        *::std::mem::transmute::<&[u8; 8], &[libc::c_char; 8]>(b"purple\0\0"),
        *::std::mem::transmute::<&[u8; 8], &[libc::c_char; 8]>(b"mauve\0\0\0"),
        *::std::mem::transmute::<&[u8; 8], &[libc::c_char; 8]>(b"lagoon\0\0"),
        *::std::mem::transmute::<&[u8; 8], &[libc::c_char; 8]>(b"mint\0\0\0\0"),
        *::std::mem::transmute::<&[u8; 8], &[libc::c_char; 8]>(b"lime\0\0\0\0"),
        *::std::mem::transmute::<&[u8; 8], &[libc::c_char; 8]>(b"peach\0\0\0"),
        *::std::mem::transmute::<&[u8; 8], &[libc::c_char; 8]>(b"orange\0\0"),
        *::std::mem::transmute::<&[u8; 8], &[libc::c_char; 8]>(b"latte\0\0\0"),
        *::std::mem::transmute::<&[u8; 8], &[libc::c_char; 8]>(b"rosy\0\0\0\0"),
        *::std::mem::transmute::<&[u8; 8], &[libc::c_char; 8]>(b"beet\0\0\0\0"),
        *::std::mem::transmute::<&[u8; 8], &[libc::c_char; 8]>(b"plum\0\0\0\0"),
        *::std::mem::transmute::<&[u8; 8], &[libc::c_char; 8]>(b"sea\0\0\0\0\0"),
        *::std::mem::transmute::<&[u8; 8], &[libc::c_char; 8]>(b"sky\0\0\0\0\0"),
        *::std::mem::transmute::<&[u8; 8], &[libc::c_char; 8]>(b"slate\0\0\0"),
        *::std::mem::transmute::<&[u8; 8], &[libc::c_char; 8]>(b"teal\0\0\0\0"),
        *::std::mem::transmute::<&[u8; 8], &[libc::c_char; 8]>(b"sage\0\0\0\0"),
        *::std::mem::transmute::<&[u8; 8], &[libc::c_char; 8]>(b"brown\0\0\0"),
        *::std::mem::transmute::<&[u8; 8], &[libc::c_char; 8]>(b"ocher\0\0\0"),
        *::std::mem::transmute::<&[u8; 8], &[libc::c_char; 8]>(b"sand\0\0\0\0"),
        *::std::mem::transmute::<&[u8; 8], &[libc::c_char; 8]>(b"tawny\0\0\0"),
        *::std::mem::transmute::<&[u8; 8], &[libc::c_char; 8]>(b"brick\0\0\0"),
        *::std::mem::transmute::<&[u8; 8], &[libc::c_char; 8]>(b"crimson\0"),
        *::std::mem::transmute::<&[u8; 8], &[libc::c_char; 8]>(b"grey\0\0\0\0"),
        *::std::mem::transmute::<&[u8; 8], &[libc::c_char; 8]>(b"gray\0\0\0\0"),
    ]
};
pub static mut indices: [libc::c_short; 34] = [
    1 as libc::c_int as libc::c_short,
    2 as libc::c_int as libc::c_short,
    4 as libc::c_int as libc::c_short,
    3 as libc::c_int as libc::c_short,
    6 as libc::c_int as libc::c_short,
    5 as libc::c_int as libc::c_short,
    7 as libc::c_int as libc::c_short,
    0 as libc::c_int as libc::c_short,
    -(1 as libc::c_int) as libc::c_short,
    204 as libc::c_int as libc::c_short,
    163 as libc::c_int as libc::c_short,
    134 as libc::c_int as libc::c_short,
    38 as libc::c_int as libc::c_short,
    48 as libc::c_int as libc::c_short,
    148 as libc::c_int as libc::c_short,
    215 as libc::c_int as libc::c_short,
    208 as libc::c_int as libc::c_short,
    137 as libc::c_int as libc::c_short,
    175 as libc::c_int as libc::c_short,
    127 as libc::c_int as libc::c_short,
    98 as libc::c_int as libc::c_short,
    32 as libc::c_int as libc::c_short,
    111 as libc::c_int as libc::c_short,
    66 as libc::c_int as libc::c_short,
    35 as libc::c_int as libc::c_short,
    107 as libc::c_int as libc::c_short,
    100 as libc::c_int as libc::c_short,
    142 as libc::c_int as libc::c_short,
    186 as libc::c_int as libc::c_short,
    136 as libc::c_int as libc::c_short,
    166 as libc::c_int as libc::c_short,
    161 as libc::c_int as libc::c_short,
    (0 as libc::c_int + 8 as libc::c_int) as libc::c_short,
    (0 as libc::c_int + 8 as libc::c_int) as libc::c_short,
];
pub unsafe extern "C" fn color_to_short(
    mut colorname: *const libc::c_char,
    mut vivid: *mut bool,
    mut thick: *mut bool,
) -> libc::c_short {
    if strncmp(
        colorname,
        b"bright\0" as *const u8 as *const libc::c_char,
        6 as libc::c_int as libc::c_ulong,
    ) == 0 as libc::c_int
        && *colorname.offset(6 as libc::c_int as isize) as libc::c_int != '\0' as i32
    {
        *vivid = 1 as libc::c_int != 0;
        *thick = 1 as libc::c_int != 0;
        colorname = colorname.offset(6 as libc::c_int as isize);
    } else if strncmp(
        colorname,
        b"light\0" as *const u8 as *const libc::c_char,
        5 as libc::c_int as libc::c_ulong,
    ) == 0 as libc::c_int
        && *colorname.offset(5 as libc::c_int as isize) as libc::c_int != '\0' as i32
    {
        *vivid = 1 as libc::c_int != 0;
        *thick = 0 as libc::c_int != 0;
        colorname = colorname.offset(5 as libc::c_int as isize);
    } else {
        *vivid = 0 as libc::c_int != 0;
        *thick = 0 as libc::c_int != 0;
    }
    if *colorname.offset(0 as libc::c_int as isize) as libc::c_int == '#' as i32
        && strlen(colorname) == 4 as libc::c_int as libc::c_ulong
    {
        let mut r: libc::c_ushort = 0;
        let mut g: libc::c_ushort = 0;
        let mut b: libc::c_ushort = 0;
        if *vivid {
            jot_error(
                b"Color '%s' takes no prefix\0" as *const u8 as *const libc::c_char,
                colorname,
            );
            return -(2 as libc::c_int) as libc::c_short;
        }
        if sscanf(
            colorname,
            b"#%1hX%1hX%1hX\0" as *const u8 as *const libc::c_char,
            &mut r as *mut libc::c_ushort,
            &mut g as *mut libc::c_ushort,
            &mut b as *mut libc::c_ushort,
        ) == 3 as libc::c_int
        {
            return closest_index_color(
                r as libc::c_short,
                g as libc::c_short,
                b as libc::c_short,
            );
        }
    }
    let mut index: libc::c_int = 0 as libc::c_int;
    while index < 34 as libc::c_int {
        if strcmp(colorname, (hues[index as usize]).as_ptr()) == 0 as libc::c_int {
            if index > 7 as libc::c_int && *vivid as libc::c_int != 0 {
                jot_error(
                    b"Color '%s' takes no prefix\0" as *const u8 as *const libc::c_char,
                    colorname,
                );
                return -(2 as libc::c_int) as libc::c_short;
            } else if index > 8 as libc::c_int && COLORS < 255 as libc::c_int {
                return -(1 as libc::c_int) as libc::c_short
            } else {
                return indices[index as usize]
            }
        }
        index += 1;
        index;
    }
    jot_error(
        b"Color \"%s\" not understood\0" as *const u8 as *const libc::c_char,
        colorname,
    );
    return -(2 as libc::c_int) as libc::c_short;
}
pub unsafe extern "C" fn parse_combination(
    mut combostr: *mut libc::c_char,
    mut fg: *mut libc::c_short,
    mut bg: *mut libc::c_short,
    mut attributes: *mut libc::c_int,
) -> bool {
    let mut vivid: bool = false;
    let mut thick: bool = false;
    let mut comma: *mut libc::c_char = 0 as *mut libc::c_char;
    *attributes = (1 as libc::c_uint).wrapping_sub(1 as libc::c_uint) as libc::c_int;
    if strncmp(
        combostr,
        b"bold\0" as *const u8 as *const libc::c_char,
        4 as libc::c_int as libc::c_ulong,
    ) == 0 as libc::c_int
    {
        *attributes = (*attributes as libc::c_uint
            | (1 as libc::c_uint) << 13 as libc::c_int + 8 as libc::c_int)
            as libc::c_int;
        if *combostr.offset(4 as libc::c_int as isize) as libc::c_int != ',' as i32 {
            jot_error(
                b"An attribute requires a subsequent comma\0" as *const u8
                    as *const libc::c_char,
            );
            return 0 as libc::c_int != 0;
        }
        combostr = combostr.offset(5 as libc::c_int as isize);
    }
    if strncmp(
        combostr,
        b"italic\0" as *const u8 as *const libc::c_char,
        6 as libc::c_int as libc::c_ulong,
    ) == 0 as libc::c_int
    {
        *attributes = (*attributes as libc::c_uint
            | (1 as libc::c_uint) << 23 as libc::c_int + 8 as libc::c_int)
            as libc::c_int;
        if *combostr.offset(6 as libc::c_int as isize) as libc::c_int != ',' as i32 {
            jot_error(
                b"An attribute requires a subsequent comma\0" as *const u8
                    as *const libc::c_char,
            );
            return 0 as libc::c_int != 0;
        }
        combostr = combostr.offset(7 as libc::c_int as isize);
    }
    comma = strchr(combostr, ',' as i32);
    if !comma.is_null() {
        *comma = '\0' as i32 as libc::c_char;
    }
    if comma.is_null() || comma > combostr {
        *fg = color_to_short(combostr, &mut vivid, &mut thick);
        if *fg as libc::c_int == -(2 as libc::c_int) {
            return 0 as libc::c_int != 0;
        }
        if vivid as libc::c_int != 0 && !thick && COLORS > 8 as libc::c_int {
            *fg = (*fg as libc::c_int + 8 as libc::c_int) as libc::c_short;
        } else if vivid {
            *attributes = (*attributes as libc::c_uint
                | (1 as libc::c_uint) << 13 as libc::c_int + 8 as libc::c_int)
                as libc::c_int;
        }
    } else {
        *fg = -(1 as libc::c_int) as libc::c_short;
    }
    if !comma.is_null() {
        *bg = color_to_short(
            comma.offset(1 as libc::c_int as isize),
            &mut vivid,
            &mut thick,
        );
        if *bg as libc::c_int == -(2 as libc::c_int) {
            return 0 as libc::c_int != 0;
        }
        if vivid as libc::c_int != 0 && COLORS > 8 as libc::c_int {
            *bg = (*bg as libc::c_int + 8 as libc::c_int) as libc::c_short;
        }
    } else {
        *bg = -(1 as libc::c_int) as libc::c_short;
    }
    return 1 as libc::c_int != 0;
}
pub unsafe extern "C" fn parse_rule(
    mut ptr: *mut libc::c_char,
    mut rex_flags: libc::c_int,
) {
    let mut names: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut regexstring: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut fg: libc::c_short = 0;
    let mut bg: libc::c_short = 0;
    let mut attributes: libc::c_int = 0;
    if *ptr as libc::c_int == '\0' as i32 {
        jot_error(b"Missing color name\0" as *const u8 as *const libc::c_char);
        return;
    }
    names = ptr;
    ptr = parse_next_word(ptr);
    if !parse_combination(names, &mut fg, &mut bg, &mut attributes) {
        return;
    }
    if *ptr as libc::c_int == '\0' as i32 {
        jot_error(
            b"Missing regex string after '%s' command\0" as *const u8
                as *const libc::c_char,
            b"color\0" as *const u8 as *const libc::c_char,
        );
        return;
    }
    while *ptr as libc::c_int != '\0' as i32 {
        let mut start_rgx: *mut regex_t = 0 as *mut regex_t;
        let mut end_rgx: *mut regex_t = 0 as *mut regex_t;
        let mut newcolor: *mut colortype = 0 as *mut colortype;
        let mut expectend: bool = 0 as libc::c_int != 0;
        if strncmp(
            ptr,
            b"start=\0" as *const u8 as *const libc::c_char,
            6 as libc::c_int as libc::c_ulong,
        ) == 0 as libc::c_int
        {
            ptr = ptr.offset(6 as libc::c_int as isize);
            expectend = 1 as libc::c_int != 0;
        }
        ptr = ptr.offset(1);
        regexstring = ptr;
        ptr = parse_next_regex(ptr);
        if ptr.is_null() || !compile(regexstring, rex_flags, &mut start_rgx) {
            return;
        }
        if expectend {
            if strncmp(
                ptr,
                b"end=\0" as *const u8 as *const libc::c_char,
                4 as libc::c_int as libc::c_ulong,
            ) != 0 as libc::c_int
            {
                jot_error(
                    b"\"start=\" requires a corresponding \"end=\"\0" as *const u8
                        as *const libc::c_char,
                );
                rpl_regfree(start_rgx);
                rpl_free(start_rgx as *mut libc::c_void);
                return;
            }
            regexstring = ptr.offset(5 as libc::c_int as isize);
            ptr = parse_next_regex(ptr.offset(5 as libc::c_int as isize));
            if ptr.is_null() || !compile(regexstring, rex_flags, &mut end_rgx) {
                rpl_regfree(start_rgx);
                rpl_free(start_rgx as *mut libc::c_void);
                return;
            }
        }
        newcolor = nmalloc(::std::mem::size_of::<colortype>() as libc::c_ulong)
            as *mut colortype;
        (*newcolor).start = start_rgx;
        (*newcolor).end = end_rgx;
        (*newcolor).fg = fg;
        (*newcolor).bg = bg;
        (*newcolor).attributes = attributes;
        if lastcolor.is_null() {
            (*live_syntax).color = newcolor;
        } else {
            (*lastcolor).next = newcolor;
        }
        (*newcolor).next = 0 as *mut colortype;
        lastcolor = newcolor;
        if expectend {
            (*newcolor).id = (*live_syntax).nmultis;
            (*live_syntax).nmultis += 1;
            (*live_syntax).nmultis;
        }
    }
}
pub unsafe extern "C" fn parse_interface_color(
    mut combostr: *mut libc::c_char,
) -> *mut colortype {
    let mut trio: *mut colortype = nmalloc(
        ::std::mem::size_of::<colortype>() as libc::c_ulong,
    ) as *mut colortype;
    if !parse_combination(
        combostr,
        &mut (*trio).fg,
        &mut (*trio).bg,
        &mut (*trio).attributes,
    ) {
        rpl_free(trio as *mut libc::c_void);
        return 0 as *mut colortype;
    } else {
        return trio
    };
}
pub unsafe extern "C" fn grab_and_store(
    mut kind: *const libc::c_char,
    mut ptr: *mut libc::c_char,
    mut storage: *mut *mut regexlisttype,
) {
    let mut lastthing: *mut regexlisttype = 0 as *mut regexlisttype;
    let mut newthing: *mut regexlisttype = 0 as *mut regexlisttype;
    let mut regexstring: *const libc::c_char = 0 as *const libc::c_char;
    if !opensyntax {
        jot_error(
            b"A '%s' command requires a preceding 'syntax' command\0" as *const u8
                as *const libc::c_char,
            kind,
        );
        return;
    }
    if strcmp((*live_syntax).name, b"default\0" as *const u8 as *const libc::c_char)
        == 0 as libc::c_int && *ptr as libc::c_int != '\0' as i32
    {
        jot_error(
            b"The \"default\" syntax does not accept '%s' regexes\0" as *const u8
                as *const libc::c_char,
            kind,
        );
        return;
    }
    if *ptr as libc::c_int == '\0' as i32 {
        jot_error(
            b"Missing regex string after '%s' command\0" as *const u8
                as *const libc::c_char,
            kind,
        );
        return;
    }
    lastthing = *storage;
    while !lastthing.is_null() && !((*lastthing).next).is_null() {
        lastthing = (*lastthing).next;
    }
    while *ptr as libc::c_int != '\0' as i32 {
        let mut packed_rgx: *mut regex_t = 0 as *mut regex_t;
        ptr = ptr.offset(1);
        regexstring = ptr;
        ptr = parse_next_regex(ptr);
        if ptr.is_null() {
            return;
        }
        if !compile(
            regexstring,
            1 as libc::c_int | (1 as libc::c_int) << 3 as libc::c_int,
            &mut packed_rgx,
        ) {
            continue;
        }
        newthing = nmalloc(::std::mem::size_of::<regexlisttype>() as libc::c_ulong)
            as *mut regexlisttype;
        (*newthing).one_rgx = packed_rgx;
        (*newthing).next = 0 as *mut regexlisttype;
        if lastthing.is_null() {
            *storage = newthing;
        } else {
            (*lastthing).next = newthing;
        }
        lastthing = newthing;
    }
}
pub unsafe extern "C" fn pick_up_name(
    mut kind: *const libc::c_char,
    mut ptr: *mut libc::c_char,
    mut storage: *mut *mut libc::c_char,
) {
    if *ptr as libc::c_int == '\0' as i32 {
        jot_error(
            b"Missing argument after '%s'\0" as *const u8 as *const libc::c_char,
            kind,
        );
        return;
    }
    if *ptr as libc::c_int == '"' as i32 {
        let mut look: *mut libc::c_char = ptr.offset(strlen(ptr) as isize);
        while *look as libc::c_int != '"' as i32 {
            look = look.offset(-1);
            if look == ptr {
                jot_error(
                    b"Argument of '%s' lacks closing \"\0" as *const u8
                        as *const libc::c_char,
                    kind,
                );
                return;
            }
        }
        *look = '\0' as i32 as libc::c_char;
        ptr = ptr.offset(1);
        ptr;
    }
    *storage = mallocstrcpy(*storage, ptr);
}
pub unsafe extern "C" fn parse_syntax_commands(
    mut keyword: *mut libc::c_char,
    mut ptr: *mut libc::c_char,
) -> bool {
    if strcmp(keyword, b"color\0" as *const u8 as *const libc::c_char)
        == 0 as libc::c_int
    {
        parse_rule(ptr, 1 as libc::c_int);
    } else if strcmp(keyword, b"icolor\0" as *const u8 as *const libc::c_char)
        == 0 as libc::c_int
    {
        parse_rule(ptr, 1 as libc::c_int | (1 as libc::c_int) << 1 as libc::c_int);
    } else if strcmp(keyword, b"comment\0" as *const u8 as *const libc::c_char)
        == 0 as libc::c_int
    {
        pick_up_name(
            b"comment\0" as *const u8 as *const libc::c_char,
            ptr,
            &mut (*live_syntax).comment,
        );
    } else if strcmp(keyword, b"tabgives\0" as *const u8 as *const libc::c_char)
        == 0 as libc::c_int
    {
        pick_up_name(
            b"tabgives\0" as *const u8 as *const libc::c_char,
            ptr,
            &mut (*live_syntax).tab,
        );
    } else if strcmp(keyword, b"linter\0" as *const u8 as *const libc::c_char)
        == 0 as libc::c_int
    {
        pick_up_name(
            b"linter\0" as *const u8 as *const libc::c_char,
            ptr,
            &mut (*live_syntax).linter,
        );
    } else if strcmp(keyword, b"formatter\0" as *const u8 as *const libc::c_char)
        == 0 as libc::c_int
    {
        pick_up_name(
            b"formatter\0" as *const u8 as *const libc::c_char,
            ptr,
            &mut (*live_syntax).formatter,
        );
    } else {
        return 0 as libc::c_int != 0
    }
    return 1 as libc::c_int != 0;
}
unsafe extern "C" fn check_vitals_mapped() {
    let mut vitals: [Option::<unsafe extern "C" fn() -> ()>; 4] = [
        Some(do_exit as unsafe extern "C" fn() -> ()),
        Some(do_exit as unsafe extern "C" fn() -> ()),
        Some(do_exit as unsafe extern "C" fn() -> ()),
        Some(do_cancel as unsafe extern "C" fn() -> ()),
    ];
    let mut inmenus: [libc::c_int; 4] = [
        (1 as libc::c_int) << 0 as libc::c_int,
        (1 as libc::c_int) << 10 as libc::c_int,
        (1 as libc::c_int) << 8 as libc::c_int,
        (1 as libc::c_int) << 13 as libc::c_int,
    ];
    let mut v: libc::c_int = 0 as libc::c_int;
    while v < 4 as libc::c_int {
        let mut f: *mut funcstruct = allfuncs;
        while !f.is_null() {
            if (*f).func == vitals[v as usize] && (*f).menus & inmenus[v as usize] != 0 {
                if !(first_sc_for(inmenus[v as usize], (*f).func)).is_null() {
                    break;
                }
                jot_error(
                    b"No key is bound to function '%s' in menu '%s'.  Exiting.\n\0"
                        as *const u8 as *const libc::c_char,
                    (*f).tag,
                    menu_to_name(inmenus[v as usize]),
                );
                die(
                    dcgettext(
                        0 as *const libc::c_char,
                        b"If needed, use nano with the -I option to adjust your nanorc settings.\n\0"
                            as *const u8 as *const libc::c_char,
                        5 as libc::c_int,
                    ),
                );
            }
            f = (*f).next;
        }
        v += 1;
        v;
    }
}
pub unsafe extern "C" fn parse_rcfile(
    mut rcstream: *mut FILE,
    mut just_syntax: bool,
    mut intros_only: bool,
) {
    let mut buffer: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut size: size_t = 0 as libc::c_int as size_t;
    let mut length: ssize_t = 0;
    loop {
        length = getline(&mut buffer, &mut size, rcstream);
        if !(length > 0 as libc::c_int as libc::c_long) {
            break;
        }
        let mut ptr: *mut libc::c_char = 0 as *mut libc::c_char;
        let mut keyword: *mut libc::c_char = 0 as *mut libc::c_char;
        let mut option: *mut libc::c_char = 0 as *mut libc::c_char;
        let mut argument: *mut libc::c_char = 0 as *mut libc::c_char;
        let mut drop_open: bool = 0 as libc::c_int != 0;
        let mut set: libc::c_int = 0 as libc::c_int;
        let mut i: size_t = 0;
        lineno = lineno.wrapping_add(1);
        lineno;
        if just_syntax as libc::c_int != 0 && !intros_only
            && lineno <= (*live_syntax).lineno
        {
            continue;
        }
        if *buffer.offset((length - 1 as libc::c_int as libc::c_long) as isize)
            as libc::c_int == '\n' as i32
        {
            length -= 1;
            *buffer.offset(length as isize) = '\0' as i32 as libc::c_char;
        }
        if length > 0 as libc::c_int as libc::c_long
            && *buffer.offset((length - 1 as libc::c_int as libc::c_long) as isize)
                as libc::c_int == '\r' as i32
        {
            length -= 1;
            *buffer.offset(length as isize) = '\0' as i32 as libc::c_char;
        }
        ptr = buffer;
        while *(*__ctype_b_loc()).offset(*ptr as libc::c_uchar as libc::c_int as isize)
            as libc::c_int & _ISblank as libc::c_int as libc::c_ushort as libc::c_int
            != 0
        {
            ptr = ptr.offset(1);
            ptr;
        }
        if *ptr as libc::c_int == '\0' as i32 || *ptr as libc::c_int == '#' as i32 {
            continue;
        }
        keyword = ptr;
        ptr = parse_next_word(ptr);
        if !just_syntax
            && strcmp(keyword, b"extendsyntax\0" as *const u8 as *const libc::c_char)
                == 0 as libc::c_int
        {
            let mut newitem: *mut augmentstruct = 0 as *mut augmentstruct;
            let mut extra: *mut augmentstruct = 0 as *mut augmentstruct;
            let mut syntaxname: *mut libc::c_char = ptr;
            let mut sntx: *mut syntaxtype = 0 as *mut syntaxtype;
            check_for_nonempty_syntax();
            ptr = parse_next_word(ptr);
            sntx = syntaxes;
            while !sntx.is_null() {
                if strcmp((*sntx).name, syntaxname) == 0 {
                    break;
                }
                sntx = (*sntx).next;
            }
            if sntx.is_null() {
                jot_error(
                    b"Could not find syntax \"%s\" to extend\0" as *const u8
                        as *const libc::c_char,
                    syntaxname,
                );
                continue;
            } else {
                keyword = ptr;
                argument = copy_of(ptr);
                ptr = parse_next_word(ptr);
                if strcmp(keyword, b"header\0" as *const u8 as *const libc::c_char)
                    == 0 as libc::c_int
                    || strcmp(keyword, b"magic\0" as *const u8 as *const libc::c_char)
                        == 0 as libc::c_int
                {
                    rpl_free(argument as *mut libc::c_void);
                    live_syntax = sntx;
                    opensyntax = 1 as libc::c_int != 0;
                    drop_open = 1 as libc::c_int != 0;
                } else {
                    newitem = nmalloc(
                        ::std::mem::size_of::<augmentstruct>() as libc::c_ulong,
                    ) as *mut augmentstruct;
                    (*newitem).filename = copy_of(nanorc);
                    (*newitem).lineno = lineno as ssize_t;
                    (*newitem).data = argument;
                    (*newitem).next = 0 as *mut augmentstruct;
                    if !((*sntx).augmentations).is_null() {
                        extra = (*sntx).augmentations;
                        while !((*extra).next).is_null() {
                            extra = (*extra).next;
                        }
                        (*extra).next = newitem;
                    } else {
                        (*sntx).augmentations = newitem;
                    }
                    continue;
                }
            }
        }
        if strcmp(keyword, b"syntax\0" as *const u8 as *const libc::c_char)
            == 0 as libc::c_int
        {
            if !intros_only {
                break;
            }
            check_for_nonempty_syntax();
            begin_new_syntax(ptr);
        } else if strcmp(keyword, b"header\0" as *const u8 as *const libc::c_char)
            == 0 as libc::c_int
        {
            if intros_only {
                grab_and_store(
                    b"header\0" as *const u8 as *const libc::c_char,
                    ptr,
                    &mut (*live_syntax).headers,
                );
            }
        } else if !(strcmp(keyword, b"magic\0" as *const u8 as *const libc::c_char)
            == 0 as libc::c_int)
        {
            if just_syntax as libc::c_int != 0
                && (strcmp(keyword, b"set\0" as *const u8 as *const libc::c_char)
                    == 0 as libc::c_int
                    || strcmp(keyword, b"unset\0" as *const u8 as *const libc::c_char)
                        == 0 as libc::c_int
                    || strcmp(keyword, b"bind\0" as *const u8 as *const libc::c_char)
                        == 0 as libc::c_int
                    || strcmp(keyword, b"unbind\0" as *const u8 as *const libc::c_char)
                        == 0 as libc::c_int
                    || strcmp(keyword, b"include\0" as *const u8 as *const libc::c_char)
                        == 0 as libc::c_int
                    || strcmp(
                        keyword,
                        b"extendsyntax\0" as *const u8 as *const libc::c_char,
                    ) == 0 as libc::c_int)
            {
                if !intros_only {
                    break;
                }
                jot_error(
                    b"Command \"%s\" not allowed in included file\0" as *const u8
                        as *const libc::c_char,
                    keyword,
                );
            } else if intros_only as libc::c_int != 0
                && (strcmp(keyword, b"color\0" as *const u8 as *const libc::c_char)
                    == 0 as libc::c_int
                    || strcmp(keyword, b"icolor\0" as *const u8 as *const libc::c_char)
                        == 0 as libc::c_int
                    || strcmp(keyword, b"comment\0" as *const u8 as *const libc::c_char)
                        == 0 as libc::c_int
                    || strcmp(keyword, b"tabgives\0" as *const u8 as *const libc::c_char)
                        == 0 as libc::c_int
                    || strcmp(keyword, b"linter\0" as *const u8 as *const libc::c_char)
                        == 0 as libc::c_int
                    || strcmp(
                        keyword,
                        b"formatter\0" as *const u8 as *const libc::c_char,
                    ) == 0 as libc::c_int)
            {
                if !opensyntax {
                    jot_error(
                        b"A '%s' command requires a preceding 'syntax' command\0"
                            as *const u8 as *const libc::c_char,
                        keyword,
                    );
                }
                if !(strstr(b"icolor\0" as *const u8 as *const libc::c_char, keyword))
                    .is_null()
                {
                    seen_color_command = 1 as libc::c_int != 0;
                }
                continue;
            } else if !parse_syntax_commands(keyword, ptr) {
                if strcmp(keyword, b"include\0" as *const u8 as *const libc::c_char)
                    == 0 as libc::c_int
                {
                    parse_includes(ptr);
                } else if strcmp(keyword, b"set\0" as *const u8 as *const libc::c_char)
                    == 0 as libc::c_int
                {
                    set = 1 as libc::c_int;
                } else if strcmp(keyword, b"unset\0" as *const u8 as *const libc::c_char)
                    == 0 as libc::c_int
                {
                    set = -(1 as libc::c_int);
                } else if strcmp(keyword, b"bind\0" as *const u8 as *const libc::c_char)
                    == 0 as libc::c_int
                {
                    parse_binding(ptr, 1 as libc::c_int != 0);
                } else if strcmp(
                    keyword,
                    b"unbind\0" as *const u8 as *const libc::c_char,
                ) == 0 as libc::c_int
                {
                    parse_binding(ptr, 0 as libc::c_int != 0);
                } else if intros_only {
                    jot_error(
                        b"Command \"%s\" not understood\0" as *const u8
                            as *const libc::c_char,
                        keyword,
                    );
                }
            }
        }
        if drop_open {
            opensyntax = 0 as libc::c_int != 0;
        }
        if set == 0 as libc::c_int {
            continue;
        }
        check_for_nonempty_syntax();
        if *ptr as libc::c_int == '\0' as i32 {
            jot_error(b"Missing option\0" as *const u8 as *const libc::c_char);
        } else {
            option = ptr;
            ptr = parse_next_word(ptr);
            i = 0 as libc::c_int as size_t;
            while !(rcopts[i as usize].name).is_null() {
                if strcmp(option, rcopts[i as usize].name) == 0 as libc::c_int {
                    break;
                }
                i = i.wrapping_add(1);
                i;
            }
            if (rcopts[i as usize].name).is_null() {
                jot_error(
                    b"Unknown option: %s\0" as *const u8 as *const libc::c_char,
                    option,
                );
            } else if rcopts[i as usize].flag != 0 {
                if set == 1 as libc::c_int {
                    flags[(rcopts[i as usize].flag as libc::c_ulong)
                        .wrapping_div(
                            (::std::mem::size_of::<libc::c_uint>() as libc::c_ulong)
                                .wrapping_mul(8 as libc::c_int as libc::c_ulong),
                        ) as usize]
                        |= (1 as libc::c_int as libc::c_uint)
                            << (rcopts[i as usize].flag as libc::c_ulong)
                                .wrapping_rem(
                                    (::std::mem::size_of::<libc::c_uint>() as libc::c_ulong)
                                        .wrapping_mul(8 as libc::c_int as libc::c_ulong),
                                );
                } else {
                    flags[(rcopts[i as usize].flag as libc::c_ulong)
                        .wrapping_div(
                            (::std::mem::size_of::<libc::c_uint>() as libc::c_ulong)
                                .wrapping_mul(8 as libc::c_int as libc::c_ulong),
                        ) as usize]
                        &= !((1 as libc::c_int as libc::c_uint)
                            << (rcopts[i as usize].flag as libc::c_ulong)
                                .wrapping_rem(
                                    (::std::mem::size_of::<libc::c_uint>() as libc::c_ulong)
                                        .wrapping_mul(8 as libc::c_int as libc::c_ulong),
                                ));
                }
            } else if set == -(1 as libc::c_int) {
                jot_error(
                    b"Cannot unset option \"%s\"\0" as *const u8 as *const libc::c_char,
                    option,
                );
            } else if *ptr as libc::c_int == '\0' as i32 {
                jot_error(
                    b"Option \"%s\" requires an argument\0" as *const u8
                        as *const libc::c_char,
                    option,
                );
            } else {
                argument = ptr;
                if *argument as libc::c_int == '"' as i32 {
                    argument = argument.offset(1);
                    argument;
                }
                ptr = parse_argument(ptr);
                if using_utf8() as libc::c_int != 0
                    && mbstowcs(0 as *mut wchar_t, argument, 0 as libc::c_int as size_t)
                        == -(1 as libc::c_int) as size_t
                {
                    jot_error(
                        b"Argument is not a valid multibyte string\0" as *const u8
                            as *const libc::c_char,
                    );
                } else if strcmp(
                    option,
                    b"titlecolor\0" as *const u8 as *const libc::c_char,
                ) == 0 as libc::c_int
                {
                    color_combo[TITLE_BAR as libc::c_int
                        as usize] = parse_interface_color(argument);
                } else if strcmp(
                    option,
                    b"numbercolor\0" as *const u8 as *const libc::c_char,
                ) == 0 as libc::c_int
                {
                    color_combo[LINE_NUMBER as libc::c_int
                        as usize] = parse_interface_color(argument);
                } else if strcmp(
                    option,
                    b"stripecolor\0" as *const u8 as *const libc::c_char,
                ) == 0 as libc::c_int
                {
                    color_combo[GUIDE_STRIPE as libc::c_int
                        as usize] = parse_interface_color(argument);
                } else if strcmp(
                    option,
                    b"scrollercolor\0" as *const u8 as *const libc::c_char,
                ) == 0 as libc::c_int
                {
                    color_combo[SCROLL_BAR as libc::c_int
                        as usize] = parse_interface_color(argument);
                } else if strcmp(
                    option,
                    b"selectedcolor\0" as *const u8 as *const libc::c_char,
                ) == 0 as libc::c_int
                {
                    color_combo[SELECTED_TEXT as libc::c_int
                        as usize] = parse_interface_color(argument);
                } else if strcmp(
                    option,
                    b"spotlightcolor\0" as *const u8 as *const libc::c_char,
                ) == 0 as libc::c_int
                {
                    color_combo[SPOTLIGHTED as libc::c_int
                        as usize] = parse_interface_color(argument);
                } else if strcmp(
                    option,
                    b"minicolor\0" as *const u8 as *const libc::c_char,
                ) == 0 as libc::c_int
                {
                    color_combo[MINI_INFOBAR as libc::c_int
                        as usize] = parse_interface_color(argument);
                } else if strcmp(
                    option,
                    b"promptcolor\0" as *const u8 as *const libc::c_char,
                ) == 0 as libc::c_int
                {
                    color_combo[PROMPT_BAR as libc::c_int
                        as usize] = parse_interface_color(argument);
                } else if strcmp(
                    option,
                    b"statuscolor\0" as *const u8 as *const libc::c_char,
                ) == 0 as libc::c_int
                {
                    color_combo[STATUS_BAR as libc::c_int
                        as usize] = parse_interface_color(argument);
                } else if strcmp(
                    option,
                    b"errorcolor\0" as *const u8 as *const libc::c_char,
                ) == 0 as libc::c_int
                {
                    color_combo[ERROR_MESSAGE as libc::c_int
                        as usize] = parse_interface_color(argument);
                } else if strcmp(
                    option,
                    b"keycolor\0" as *const u8 as *const libc::c_char,
                ) == 0 as libc::c_int
                {
                    color_combo[KEY_COMBO as libc::c_int
                        as usize] = parse_interface_color(argument);
                } else if strcmp(
                    option,
                    b"functioncolor\0" as *const u8 as *const libc::c_char,
                ) == 0 as libc::c_int
                {
                    color_combo[FUNCTION_TAG as libc::c_int
                        as usize] = parse_interface_color(argument);
                } else if strcmp(
                    option,
                    b"operatingdir\0" as *const u8 as *const libc::c_char,
                ) == 0 as libc::c_int
                {
                    operating_dir = mallocstrcpy(operating_dir, argument);
                } else if strcmp(option, b"fill\0" as *const u8 as *const libc::c_char)
                    == 0 as libc::c_int
                {
                    if !parse_num(argument, &mut fill) {
                        jot_error(
                            b"Requested fill size \"%s\" is invalid\0" as *const u8
                                as *const libc::c_char,
                            argument,
                        );
                        fill = -(8 as libc::c_int) as ssize_t;
                    }
                } else if strcmp(
                    option,
                    b"matchbrackets\0" as *const u8 as *const libc::c_char,
                ) == 0 as libc::c_int
                {
                    if has_blank_char(argument) {
                        jot_error(
                            b"Non-blank characters required\0" as *const u8
                                as *const libc::c_char,
                        );
                    } else if (mbstrlen(argument))
                        .wrapping_rem(2 as libc::c_int as libc::c_ulong)
                        != 0 as libc::c_int as libc::c_ulong
                    {
                        jot_error(
                            b"Even number of characters required\0" as *const u8
                                as *const libc::c_char,
                        );
                    } else {
                        matchbrackets = mallocstrcpy(matchbrackets, argument);
                    }
                } else if strcmp(
                    option,
                    b"whitespace\0" as *const u8 as *const libc::c_char,
                ) == 0 as libc::c_int
                {
                    if mbstrlen(argument) != 2 as libc::c_int as libc::c_ulong
                        || breadth(argument) != 2 as libc::c_int as libc::c_ulong
                    {
                        jot_error(
                            b"Two single-column characters required\0" as *const u8
                                as *const libc::c_char,
                        );
                    } else {
                        whitespace = mallocstrcpy(whitespace, argument);
                        whitelen[0 as libc::c_int as usize] = char_length(whitespace);
                        whitelen[1 as libc::c_int
                            as usize] = char_length(
                            whitespace
                                .offset(whitelen[0 as libc::c_int as usize] as isize),
                        );
                    }
                } else if strcmp(option, b"punct\0" as *const u8 as *const libc::c_char)
                    == 0 as libc::c_int
                {
                    if has_blank_char(argument) {
                        jot_error(
                            b"Non-blank characters required\0" as *const u8
                                as *const libc::c_char,
                        );
                    } else {
                        punct = mallocstrcpy(punct, argument);
                    }
                } else if strcmp(
                    option,
                    b"brackets\0" as *const u8 as *const libc::c_char,
                ) == 0 as libc::c_int
                {
                    if has_blank_char(argument) {
                        jot_error(
                            b"Non-blank characters required\0" as *const u8
                                as *const libc::c_char,
                        );
                    } else {
                        brackets = mallocstrcpy(brackets, argument);
                    }
                } else if strcmp(
                    option,
                    b"quotestr\0" as *const u8 as *const libc::c_char,
                ) == 0 as libc::c_int
                {
                    quotestr = mallocstrcpy(quotestr, argument);
                } else if strcmp(
                    option,
                    b"speller\0" as *const u8 as *const libc::c_char,
                ) == 0 as libc::c_int
                {
                    alt_speller = mallocstrcpy(alt_speller, argument);
                } else if strcmp(
                    option,
                    b"backupdir\0" as *const u8 as *const libc::c_char,
                ) == 0 as libc::c_int
                {
                    backup_dir = mallocstrcpy(backup_dir, argument);
                } else if strcmp(
                    option,
                    b"wordchars\0" as *const u8 as *const libc::c_char,
                ) == 0 as libc::c_int
                {
                    word_chars = mallocstrcpy(word_chars, argument);
                } else if strcmp(
                    option,
                    b"guidestripe\0" as *const u8 as *const libc::c_char,
                ) == 0 as libc::c_int
                {
                    if !parse_num(argument, &mut stripe_column)
                        || stripe_column <= 0 as libc::c_int as libc::c_long
                    {
                        jot_error(
                            b"Guide column \"%s\" is invalid\0" as *const u8
                                as *const libc::c_char,
                            argument,
                        );
                        stripe_column = 0 as libc::c_int as ssize_t;
                    }
                } else if strcmp(
                    option,
                    b"tabsize\0" as *const u8 as *const libc::c_char,
                ) == 0 as libc::c_int
                {
                    if !parse_num(argument, &mut tabsize)
                        || tabsize <= 0 as libc::c_int as libc::c_long
                    {
                        jot_error(
                            b"Requested tab size \"%s\" is invalid\0" as *const u8
                                as *const libc::c_char,
                            argument,
                        );
                        tabsize = -(1 as libc::c_int) as ssize_t;
                    }
                }
            }
        }
    }
    if intros_only {
        check_for_nonempty_syntax();
    }
    fclose(rcstream);
    rpl_free(buffer as *mut libc::c_void);
    lineno = 0 as libc::c_int as size_t;
}
pub unsafe extern "C" fn parse_one_nanorc() {
    let mut rcstream: *mut FILE = fopen(
        nanorc,
        b"rb\0" as *const u8 as *const libc::c_char,
    );
    if !rcstream.is_null() {
        parse_rcfile(rcstream, 0 as libc::c_int != 0, 1 as libc::c_int != 0);
    } else if *__errno_location() != 2 as libc::c_int {
        jot_error(
            b"Error reading %s: %s\0" as *const u8 as *const libc::c_char,
            nanorc,
            strerror(*__errno_location()),
        );
    }
}
pub unsafe extern "C" fn have_nanorc(
    mut path: *const libc::c_char,
    mut name: *const libc::c_char,
) -> bool {
    if path.is_null() {
        return 0 as libc::c_int != 0;
    }
    rpl_free(nanorc as *mut libc::c_void);
    nanorc = concatenate(path, name);
    return is_good_file(nanorc);
}
pub unsafe extern "C" fn do_rcfiles() {
    if !custom_nanorc.is_null() {
        nanorc = get_full_path(custom_nanorc);
        if nanorc.is_null() || access(nanorc, 0 as libc::c_int) != 0 as libc::c_int {
            die(
                dcgettext(
                    0 as *const libc::c_char,
                    b"Specified rcfile does not exist\n\0" as *const u8
                        as *const libc::c_char,
                    5 as libc::c_int,
                ),
            );
        }
    } else {
        nanorc = mallocstrcpy(
            nanorc,
            b"/usr/local/etc/nanorc\0" as *const u8 as *const libc::c_char,
        );
    }
    if is_good_file(nanorc) {
        parse_one_nanorc();
    }
    if custom_nanorc.is_null() {
        let mut xdgconfdir: *const libc::c_char = getenv(
            b"XDG_CONFIG_HOME\0" as *const u8 as *const libc::c_char,
        );
        get_homedir();
        if have_nanorc(homedir, b"/.nanorc\0" as *const u8 as *const libc::c_char)
            as libc::c_int != 0
            || have_nanorc(
                xdgconfdir,
                b"/nano/nanorc\0" as *const u8 as *const libc::c_char,
            ) as libc::c_int != 0
            || have_nanorc(
                homedir,
                b"/.config/nano/nanorc\0" as *const u8 as *const libc::c_char,
            ) as libc::c_int != 0
        {
            parse_one_nanorc();
        } else if homedir.is_null() && xdgconfdir.is_null() {
            jot_error(
                b"I can't find my home directory!  Wah!\0" as *const u8
                    as *const libc::c_char,
            );
        }
    }
    check_vitals_mapped();
    rpl_free(nanorc as *mut libc::c_void);
    nanorc = 0 as *mut libc::c_char;
}
