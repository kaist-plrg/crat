use ::libc;
use ::c2rust_bitfields;
extern "C" {
    pub type re_dfa_t;
    pub type ldat;
    fn strtol(
        _: *const libc::c_char,
        _: *mut *mut libc::c_char,
        _: libc::c_int,
    ) -> libc::c_long;
    static mut COLS: libc::c_int;
    fn to_first_file();
    fn to_last_file();
    fn using_utf8() -> bool;
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
    fn do_writeout();
    fn do_savefile();
    fn do_exit();
    fn do_enter();
    fn do_help();
    fn do_search_forward();
    fn do_page_down();
    fn do_page_up();
    fn do_findnext();
    fn do_findprevious();
    fn nmalloc(howmuch: size_t) -> *mut libc::c_void;
    fn do_spell();
    fn report_cursor_position();
    fn do_justify();
    fn to_last_line();
    fn to_first_line();
    fn to_para_end();
    fn to_para_begin();
    fn full_refresh();
    fn do_center();
    fn do_full_justify();
    fn count_lines_words_and_characters();
    fn suggest_ctrlT_ctrlZ();
    fn do_suspend();
    fn do_verbatim_input();
    fn do_scroll_down();
    fn do_scroll_up();
    fn to_next_block();
    fn to_prev_block();
    fn do_down();
    fn do_up();
    fn do_end();
    fn do_home();
    fn to_next_word();
    fn to_prev_word();
    fn do_right();
    fn do_left();
    fn do_comment();
    fn complete_a_word();
    fn to_next_anchor();
    fn to_prev_anchor();
    fn put_or_lift_anchor();
    fn do_redo();
    fn do_undo();
    fn run_macro();
    fn record_macro();
    fn do_unindent();
    fn do_indent();
    fn do_mark();
    fn do_find_bracket();
    fn do_gotolinecolumn();
    fn do_formatter();
    fn do_linter();
    fn do_replace();
    fn do_search_backward();
    fn do_tab();
    fn __ctype_tolower_loc() -> *mut *const __int32_t;
    fn strcasecmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    fn strncasecmp(
        _: *const libc::c_char,
        _: *const libc::c_char,
        _: libc::c_ulong,
    ) -> libc::c_int;
}
pub type __int32_t = libc::c_int;
pub type __dev_t = libc::c_ulong;
pub type __uid_t = libc::c_uint;
pub type __gid_t = libc::c_uint;
pub type __ino_t = libc::c_ulong;
pub type __mode_t = libc::c_uint;
pub type __nlink_t = libc::c_ulong;
pub type __off_t = libc::c_long;
pub type __time_t = libc::c_long;
pub type __blksize_t = libc::c_long;
pub type __blkcnt_t = libc::c_long;
pub type __ssize_t = libc::c_long;
pub type __syscall_slong_t = libc::c_long;
pub type __sig_atomic_t = libc::c_int;
pub type ssize_t = __ssize_t;
pub type size_t = libc::c_ulong;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct timespec {
    pub tv_sec: __time_t,
    pub tv_nsec: __syscall_slong_t,
}
pub type sig_atomic_t = __sig_atomic_t;
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
pub type regoff_t = ssize_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct regmatch_t {
    pub rm_so: regoff_t,
    pub rm_eo: regoff_t,
}
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _win_st {
    pub _cury: libc::c_short,
    pub _curx: libc::c_short,
    pub _maxy: libc::c_short,
    pub _maxx: libc::c_short,
    pub _begy: libc::c_short,
    pub _begx: libc::c_short,
    pub _flags: libc::c_short,
    pub _attrs: attr_t,
    pub _bkgd: chtype,
    pub _notimeout: bool,
    pub _clear: bool,
    pub _leaveok: bool,
    pub _scroll: bool,
    pub _idlok: bool,
    pub _idcok: bool,
    pub _immed: bool,
    pub _sync: bool,
    pub _use_keypad: bool,
    pub _delay: libc::c_int,
    pub _line: *mut ldat,
    pub _regtop: libc::c_short,
    pub _regbottom: libc::c_short,
    pub _parx: libc::c_int,
    pub _pary: libc::c_int,
    pub _parent: *mut WINDOW,
    pub _pad: pdat,
    pub _yoffset: libc::c_short,
    pub _bkgrnd: cchar_t,
    pub _color: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct cchar_t {
    pub attr: attr_t,
    pub chars: [wchar_t; 5],
    pub ext_color: libc::c_int,
}
pub type attr_t = chtype;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct pdat {
    pub _pad_y: libc::c_short,
    pub _pad_x: libc::c_short,
    pub _pad_top: libc::c_short,
    pub _pad_left: libc::c_short,
    pub _pad_bottom: libc::c_short,
    pub _pad_right: libc::c_short,
}
pub type WINDOW = _win_st;
pub type format_type = libc::c_uint;
pub const MAC_FILE: format_type = 3;
pub const DOS_FILE: format_type = 2;
pub const NIX_FILE: format_type = 1;
pub const UNSPECIFIED: format_type = 0;
pub type message_type = libc::c_uint;
pub const ALERT: message_type = 7;
pub const MILD: message_type = 6;
pub const AHEM: message_type = 5;
pub const NOTICE: message_type = 4;
pub const INFO: message_type = 3;
pub const REMARK: message_type = 2;
pub const HUSH: message_type = 1;
pub const VACUUM: message_type = 0;
pub type undo_type = libc::c_uint;
pub const OTHER: undo_type = 21;
pub const COUPLE_END: undo_type = 20;
pub const COUPLE_BEGIN: undo_type = 19;
pub const INSERT: undo_type = 18;
pub const PASTE: undo_type = 17;
pub const COPY: undo_type = 16;
pub const CUT_TO_EOF: undo_type = 15;
pub const CUT: undo_type = 14;
pub const ZAP: undo_type = 13;
pub const PREFLIGHT: undo_type = 12;
pub const UNCOMMENT: undo_type = 11;
pub const COMMENT: undo_type = 10;
pub const UNINDENT: undo_type = 9;
pub const INDENT: undo_type = 8;
pub const SPLIT_END: undo_type = 7;
pub const SPLIT_BEGIN: undo_type = 6;
pub const REPLACE: undo_type = 5;
pub const JOIN: undo_type = 4;
pub const DEL: undo_type = 3;
pub const BACK: undo_type = 2;
pub const ENTER: undo_type = 1;
pub const ADD: undo_type = 0;
pub type C2RustUnnamed = libc::c_uint;
pub const ZERO: C2RustUnnamed = 48;
pub const MINIBAR: C2RustUnnamed = 47;
pub const USE_MAGIC: C2RustUnnamed = 46;
pub const STATEFLAGS: C2RustUnnamed = 45;
pub const BOOKSTYLE: C2RustUnnamed = 44;
pub const INDICATOR: C2RustUnnamed = 43;
pub const EMPTY_LINE: C2RustUnnamed = 42;
pub const JUMPY_SCROLLING: C2RustUnnamed = 41;
pub const BREAK_LONG_LINES: C2RustUnnamed = 40;
pub const LET_THEM_ZAP: C2RustUnnamed = 39;
pub const AFTER_ENDS: C2RustUnnamed = 38;
pub const AT_BLANKS: C2RustUnnamed = 37;
pub const LINE_NUMBERS: C2RustUnnamed = 36;
pub const SHOW_CURSOR: C2RustUnnamed = 35;
pub const TRIM_BLANKS: C2RustUnnamed = 34;
pub const MAKE_IT_UNIX: C2RustUnnamed = 33;
pub const NOREAD_MODE: C2RustUnnamed = 32;
pub const LOCKING: C2RustUnnamed = 31;
pub const POSITIONLOG: C2RustUnnamed = 30;
pub const SOFTWRAP: C2RustUnnamed = 29;
pub const BOLD_TEXT: C2RustUnnamed = 28;
pub const NO_NEWLINES: C2RustUnnamed = 27;
pub const WORD_BOUNDS: C2RustUnnamed = 26;
pub const QUICK_BLANK: C2RustUnnamed = 25;
pub const TABS_TO_SPACES: C2RustUnnamed = 24;
pub const WHITESPACE_DISPLAY: C2RustUnnamed = 23;
pub const SMART_HOME: C2RustUnnamed = 22;
pub const RESTRICTED: C2RustUnnamed = 21;
pub const HISTORYLOG: C2RustUnnamed = 20;
pub const PRESERVE: C2RustUnnamed = 19;
pub const NO_SYNTAX: C2RustUnnamed = 18;
pub const INSECURE_BACKUP: C2RustUnnamed = 17;
pub const MAKE_BACKUP: C2RustUnnamed = 16;
pub const NO_CONVERT: C2RustUnnamed = 15;
pub const RAW_SEQUENCES: C2RustUnnamed = 14;
pub const REBIND_DELETE: C2RustUnnamed = 13;
pub const MULTIBUFFER: C2RustUnnamed = 12;
pub const BACKWARDS_SEARCH: C2RustUnnamed = 11;
pub const CUT_FROM_CURSOR: C2RustUnnamed = 10;
pub const SAVE_ON_EXIT: C2RustUnnamed = 9;
pub const USE_REGEXP: C2RustUnnamed = 8;
pub const USE_MOUSE: C2RustUnnamed = 7;
pub const VIEW_MODE: C2RustUnnamed = 6;
pub const AUTOINDENT: C2RustUnnamed = 5;
pub const NO_WRAP: C2RustUnnamed = 4;
pub const NO_HELP: C2RustUnnamed = 3;
pub const CONSTANT_SHOW: C2RustUnnamed = 2;
pub const CASE_SENSITIVE: C2RustUnnamed = 1;
pub const DONTUSE: C2RustUnnamed = 0;
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
pub struct groupstruct {
    pub top_line: ssize_t,
    pub bottom_line: ssize_t,
    pub indentations: *mut *mut libc::c_char,
    pub next: *mut groupstruct,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct undostruct {
    pub type_0: undo_type,
    pub xflags: libc::c_int,
    pub head_lineno: ssize_t,
    pub head_x: size_t,
    pub strdata: *mut libc::c_char,
    pub wassize: size_t,
    pub newsize: size_t,
    pub grouping: *mut groupstruct,
    pub cutbuffer: *mut linestruct,
    pub tail_lineno: ssize_t,
    pub tail_x: size_t,
    pub next: *mut undostruct,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct openfilestruct {
    pub filename: *mut libc::c_char,
    pub filetop: *mut linestruct,
    pub filebot: *mut linestruct,
    pub edittop: *mut linestruct,
    pub current: *mut linestruct,
    pub totsize: size_t,
    pub firstcolumn: size_t,
    pub current_x: size_t,
    pub placewewant: size_t,
    pub current_y: ssize_t,
    pub statinfo: *mut stat,
    pub spillage_line: *mut linestruct,
    pub mark: *mut linestruct,
    pub mark_x: size_t,
    pub softmark: bool,
    pub fmt: format_type,
    pub lock_filename: *mut libc::c_char,
    pub undotop: *mut undostruct,
    pub current_undo: *mut undostruct,
    pub last_saved: *mut undostruct,
    pub last_action: undo_type,
    pub modified: bool,
    pub syntax: *mut syntaxtype,
    pub errormessage: *mut libc::c_char,
    pub next: *mut openfilestruct,
    pub prev: *mut openfilestruct,
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
#[inline]
unsafe extern "C" fn atoi(mut __nptr: *const libc::c_char) -> libc::c_int {
    return strtol(
        __nptr,
        0 as *mut libc::c_void as *mut *mut libc::c_char,
        10 as libc::c_int,
    ) as libc::c_int;
}
#[inline]
unsafe extern "C" fn tolower(mut __c: libc::c_int) -> libc::c_int {
    return if __c >= -(128 as libc::c_int) && __c < 256 as libc::c_int {
        *(*__ctype_tolower_loc()).offset(__c as isize)
    } else {
        __c
    };
}
pub static mut the_window_resized: sig_atomic_t = 0 as libc::c_int;
pub static mut on_a_vt: bool = 0 as libc::c_int != 0;
pub static mut shifted_metas: bool = 0 as libc::c_int != 0;
pub static mut meta_key: bool = false;
pub static mut shift_held: bool = false;
pub static mut mute_modifiers: bool = 0 as libc::c_int != 0;
pub static mut bracketed_paste: bool = 0 as libc::c_int != 0;
pub static mut we_are_running: bool = 0 as libc::c_int != 0;
pub static mut more_than_one: bool = 0 as libc::c_int != 0;
pub static mut report_size: bool = 1 as libc::c_int != 0;
pub static mut ran_a_tool: bool = 0 as libc::c_int != 0;
pub static mut inhelp: bool = 0 as libc::c_int != 0;
pub static mut title: *mut libc::c_char = 0 as *const libc::c_char as *mut libc::c_char;
pub static mut refresh_needed: bool = 0 as libc::c_int != 0;
pub static mut focusing: bool = 1 as libc::c_int != 0;
pub static mut as_an_at: bool = 1 as libc::c_int != 0;
pub static mut control_C_was_pressed: bool = 0 as libc::c_int != 0;
pub static mut lastmessage: message_type = VACUUM;
pub static mut pletion_line: *mut linestruct = 0 as *const linestruct as *mut linestruct;
pub static mut also_the_last: bool = 0 as libc::c_int != 0;
pub static mut answer: *mut libc::c_char = 0 as *const libc::c_char as *mut libc::c_char;
pub static mut last_search: *mut libc::c_char = 0 as *const libc::c_char
    as *mut libc::c_char;
pub static mut didfind: libc::c_int = 0 as libc::c_int;
pub static mut present_path: *mut libc::c_char = 0 as *const libc::c_char
    as *mut libc::c_char;
pub static mut flags: [libc::c_uint; 4] = [
    0 as libc::c_int as libc::c_uint,
    0 as libc::c_int as libc::c_uint,
    0 as libc::c_int as libc::c_uint,
    0 as libc::c_int as libc::c_uint,
];
pub static mut controlleft: libc::c_int = 0;
pub static mut controlright: libc::c_int = 0;
pub static mut controlup: libc::c_int = 0;
pub static mut controldown: libc::c_int = 0;
pub static mut controlhome: libc::c_int = 0;
pub static mut controlend: libc::c_int = 0;
pub static mut controldelete: libc::c_int = 0;
pub static mut controlshiftdelete: libc::c_int = 0;
pub static mut shiftleft: libc::c_int = 0;
pub static mut shiftright: libc::c_int = 0;
pub static mut shiftup: libc::c_int = 0;
pub static mut shiftdown: libc::c_int = 0;
pub static mut shiftcontrolleft: libc::c_int = 0;
pub static mut shiftcontrolright: libc::c_int = 0;
pub static mut shiftcontrolup: libc::c_int = 0;
pub static mut shiftcontroldown: libc::c_int = 0;
pub static mut shiftcontrolhome: libc::c_int = 0;
pub static mut shiftcontrolend: libc::c_int = 0;
pub static mut altleft: libc::c_int = 0;
pub static mut altright: libc::c_int = 0;
pub static mut altup: libc::c_int = 0;
pub static mut altdown: libc::c_int = 0;
pub static mut altpageup: libc::c_int = 0;
pub static mut altpagedown: libc::c_int = 0;
pub static mut altinsert: libc::c_int = 0;
pub static mut altdelete: libc::c_int = 0;
pub static mut shiftaltleft: libc::c_int = 0;
pub static mut shiftaltright: libc::c_int = 0;
pub static mut shiftaltup: libc::c_int = 0;
pub static mut shiftaltdown: libc::c_int = 0;
pub static mut fill: ssize_t = -(8 as libc::c_int) as ssize_t;
pub static mut wrap_at: size_t = 0 as libc::c_int as size_t;
pub static mut topwin: *mut WINDOW = 0 as *const WINDOW as *mut WINDOW;
pub static mut midwin: *mut WINDOW = 0 as *const WINDOW as *mut WINDOW;
pub static mut footwin: *mut WINDOW = 0 as *const WINDOW as *mut WINDOW;
pub static mut editwinrows: libc::c_int = 0 as libc::c_int;
pub static mut editwincols: libc::c_int = -(1 as libc::c_int);
pub static mut margin: libc::c_int = 0 as libc::c_int;
pub static mut thebar: libc::c_int = 0 as libc::c_int;
pub static mut bardata: *mut libc::c_int = 0 as *const libc::c_int as *mut libc::c_int;
pub static mut stripe_column: ssize_t = 0 as libc::c_int as ssize_t;
pub static mut cutbuffer: *mut linestruct = 0 as *const linestruct as *mut linestruct;
pub static mut cutbottom: *mut linestruct = 0 as *const linestruct as *mut linestruct;
pub static mut keep_cutbuffer: bool = 0 as libc::c_int != 0;
pub static mut openfile: *mut openfilestruct = 0 as *const openfilestruct
    as *mut openfilestruct;
pub static mut startfile: *mut openfilestruct = 0 as *const openfilestruct
    as *mut openfilestruct;
pub static mut matchbrackets: *mut libc::c_char = 0 as *const libc::c_char
    as *mut libc::c_char;
pub static mut whitespace: *mut libc::c_char = 0 as *const libc::c_char
    as *mut libc::c_char;
pub static mut whitelen: [libc::c_int; 2] = [0; 2];
pub static mut punct: *mut libc::c_char = 0 as *const libc::c_char as *mut libc::c_char;
pub static mut brackets: *mut libc::c_char = 0 as *const libc::c_char
    as *mut libc::c_char;
pub static mut quotestr: *mut libc::c_char = 0 as *const libc::c_char
    as *mut libc::c_char;
pub static mut quotereg: regex_t = regex_t {
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
pub static mut word_chars: *mut libc::c_char = 0 as *const libc::c_char
    as *mut libc::c_char;
pub static mut tabsize: ssize_t = -(1 as libc::c_int) as ssize_t;
pub static mut backup_dir: *mut libc::c_char = 0 as *const libc::c_char
    as *mut libc::c_char;
pub static mut operating_dir: *mut libc::c_char = 0 as *const libc::c_char
    as *mut libc::c_char;
pub static mut alt_speller: *mut libc::c_char = 0 as *const libc::c_char
    as *mut libc::c_char;
pub static mut syntaxes: *mut syntaxtype = 0 as *const syntaxtype as *mut syntaxtype;
pub static mut syntaxstr: *mut libc::c_char = 0 as *const libc::c_char
    as *mut libc::c_char;
pub static mut have_palette: bool = 0 as libc::c_int != 0;
pub static mut rescind_colors: bool = 0 as libc::c_int != 0;
pub static mut perturbed: bool = 0 as libc::c_int != 0;
pub static mut recook: bool = 0 as libc::c_int != 0;
pub static mut currmenu: libc::c_int = (1 as libc::c_int) << 0 as libc::c_int
    | (1 as libc::c_int) << 1 as libc::c_int | (1 as libc::c_int) << 2 as libc::c_int
    | (1 as libc::c_int) << 3 as libc::c_int | (1 as libc::c_int) << 4 as libc::c_int
    | (1 as libc::c_int) << 5 as libc::c_int | (1 as libc::c_int) << 6 as libc::c_int
    | (1 as libc::c_int) << 7 as libc::c_int | (1 as libc::c_int) << 11 as libc::c_int
    | (1 as libc::c_int) << 12 as libc::c_int | (1 as libc::c_int) << 15 as libc::c_int
    | (1 as libc::c_int) << 9 as libc::c_int | (1 as libc::c_int) << 14 as libc::c_int;
pub static mut sclist: *mut keystruct = 0 as *const keystruct as *mut keystruct;
pub static mut allfuncs: *mut funcstruct = 0 as *const funcstruct as *mut funcstruct;
pub static mut tailfunc: *mut funcstruct = 0 as *const funcstruct as *mut funcstruct;
pub static mut exitfunc: *mut funcstruct = 0 as *const funcstruct as *mut funcstruct;
pub static mut search_history: *mut linestruct = 0 as *const linestruct
    as *mut linestruct;
pub static mut replace_history: *mut linestruct = 0 as *const linestruct
    as *mut linestruct;
pub static mut execute_history: *mut linestruct = 0 as *const linestruct
    as *mut linestruct;
pub static mut searchtop: *mut linestruct = 0 as *const linestruct as *mut linestruct;
pub static mut searchbot: *mut linestruct = 0 as *const linestruct as *mut linestruct;
pub static mut replacetop: *mut linestruct = 0 as *const linestruct as *mut linestruct;
pub static mut replacebot: *mut linestruct = 0 as *const linestruct as *mut linestruct;
pub static mut executetop: *mut linestruct = 0 as *const linestruct as *mut linestruct;
pub static mut executebot: *mut linestruct = 0 as *const linestruct as *mut linestruct;
pub static mut search_regexp: regex_t = regex_t {
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
pub static mut regmatches: [regmatch_t; 10] = [regmatch_t { rm_so: 0, rm_eo: 0 }; 10];
pub static mut hilite_attribute: libc::c_int = ((1 as libc::c_uint)
    << 10 as libc::c_int + 8 as libc::c_int) as libc::c_int;
pub static mut color_combo: [*mut colortype; 12] = [
    0 as *const colortype as *mut colortype,
    0 as *const colortype as *mut colortype,
    0 as *const colortype as *mut colortype,
    0 as *const colortype as *mut colortype,
    0 as *const colortype as *mut colortype,
    0 as *const colortype as *mut colortype,
    0 as *const colortype as *mut colortype,
    0 as *const colortype as *mut colortype,
    0 as *const colortype as *mut colortype,
    0 as *const colortype as *mut colortype,
    0 as *const colortype as *mut colortype,
    0 as *const colortype as *mut colortype,
];
pub static mut interface_color_pair: [libc::c_int; 12] = [
    0 as libc::c_int,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
];
pub static mut homedir: *mut libc::c_char = 0 as *const libc::c_char
    as *mut libc::c_char;
pub static mut statedir: *mut libc::c_char = 0 as *const libc::c_char
    as *mut libc::c_char;
pub static mut startup_problem: *mut libc::c_char = 0 as *const libc::c_char
    as *mut libc::c_char;
pub static mut custom_nanorc: *mut libc::c_char = 0 as *const libc::c_char
    as *mut libc::c_char;
pub static mut commandname: *mut libc::c_char = 0 as *const libc::c_char
    as *mut libc::c_char;
pub static mut planted_shortcut: *mut keystruct = 0 as *const keystruct
    as *mut keystruct;
pub static mut spotlighted: bool = 0 as libc::c_int != 0;
pub static mut light_from_col: size_t = 0 as libc::c_int as size_t;
pub static mut light_to_col: size_t = 0 as libc::c_int as size_t;
pub unsafe extern "C" fn case_sens_void() {}
pub unsafe extern "C" fn regexp_void() {}
pub unsafe extern "C" fn backwards_void() {}
pub unsafe extern "C" fn get_older_item() {}
pub unsafe extern "C" fn get_newer_item() {}
pub unsafe extern "C" fn flip_replace() {}
pub unsafe extern "C" fn flip_goto() {}
pub unsafe extern "C" fn to_files() {}
pub unsafe extern "C" fn goto_dir() {}
pub unsafe extern "C" fn do_nothing() {}
pub unsafe extern "C" fn do_toggle() {}
pub unsafe extern "C" fn dos_format() {}
pub unsafe extern "C" fn mac_format() {}
pub unsafe extern "C" fn append_it() {}
pub unsafe extern "C" fn prepend_it() {}
pub unsafe extern "C" fn back_it_up() {}
pub unsafe extern "C" fn flip_execute() {}
pub unsafe extern "C" fn flip_pipe() {}
pub unsafe extern "C" fn flip_convert() {}
pub unsafe extern "C" fn flip_newbuffer() {}
pub unsafe extern "C" fn discard_buffer() {}
pub unsafe extern "C" fn do_cancel() {}
pub unsafe extern "C" fn add_to_funcs(
    mut function: Option::<unsafe extern "C" fn() -> ()>,
    mut menus: libc::c_int,
    mut tag: *const libc::c_char,
    mut phrase: *const libc::c_char,
    mut blank_after: bool,
) {
    let mut f: *mut funcstruct = nmalloc(
        ::std::mem::size_of::<funcstruct>() as libc::c_ulong,
    ) as *mut funcstruct;
    if allfuncs.is_null() {
        allfuncs = f;
    } else {
        (*tailfunc).next = f;
    }
    tailfunc = f;
    (*f).next = 0 as *mut funcstruct;
    (*f).func = function;
    (*f).menus = menus;
    (*f).tag = tag;
    (*f).phrase = phrase;
    (*f).blank_after = blank_after;
}
pub unsafe extern "C" fn keycode_from_string(
    mut keystring: *const libc::c_char,
) -> libc::c_int {
    if *keystring.offset(0 as libc::c_int as isize) as libc::c_int == '^' as i32 {
        if *keystring.offset(2 as libc::c_int as isize) as libc::c_int == '\0' as i32 {
            if *keystring.offset(1 as libc::c_int as isize) as libc::c_int == '/' as i32
                || *keystring.offset(1 as libc::c_int as isize) as libc::c_int
                    == '-' as i32
            {
                return 31 as libc::c_int;
            }
            if *keystring.offset(1 as libc::c_int as isize) as libc::c_int <= '_' as i32
            {
                return *keystring.offset(1 as libc::c_int as isize) as libc::c_int
                    - 64 as libc::c_int;
            }
            if *keystring.offset(1 as libc::c_int as isize) as libc::c_int == '`' as i32
            {
                return 0 as libc::c_int
            } else {
                return -(1 as libc::c_int)
            }
        } else if strcasecmp(keystring, b"^Space\0" as *const u8 as *const libc::c_char)
            == 0 as libc::c_int
        {
            return 0 as libc::c_int
        } else {
            return -(1 as libc::c_int)
        }
    } else if *keystring.offset(0 as libc::c_int as isize) as libc::c_int == 'M' as i32 {
        if *keystring.offset(1 as libc::c_int as isize) as libc::c_int == '-' as i32
            && *keystring.offset(3 as libc::c_int as isize) as libc::c_int == '\0' as i32
        {
            return {
                let mut __res: libc::c_int = 0;
                if ::std::mem::size_of::<libc::c_uchar>() as libc::c_ulong
                    > 1 as libc::c_int as libc::c_ulong
                {
                    if 0 != 0 {
                        let mut __c: libc::c_int = *keystring
                            .offset(2 as libc::c_int as isize) as libc::c_uchar
                            as libc::c_int;
                        __res = if __c < -(128 as libc::c_int)
                            || __c > 255 as libc::c_int
                        {
                            __c
                        } else {
                            *(*__ctype_tolower_loc()).offset(__c as isize)
                        };
                    } else {
                        __res = tolower(
                            *keystring.offset(2 as libc::c_int as isize) as libc::c_uchar
                                as libc::c_int,
                        );
                    }
                } else {
                    __res = *(*__ctype_tolower_loc())
                        .offset(
                            *keystring.offset(2 as libc::c_int as isize) as libc::c_uchar
                                as libc::c_int as isize,
                        );
                }
                __res
            };
        }
        if strcasecmp(keystring, b"M-Space\0" as *const u8 as *const libc::c_char)
            == 0 as libc::c_int
        {
            return ' ' as i32
        } else {
            return -(1 as libc::c_int)
        }
    } else if strncasecmp(
        keystring,
        b"Sh-M-\0" as *const u8 as *const libc::c_char,
        5 as libc::c_int as libc::c_ulong,
    ) == 0 as libc::c_int
        && 'a' as i32
            <= *keystring.offset(5 as libc::c_int as isize) as libc::c_int
                | 0x20 as libc::c_int
        && *keystring.offset(5 as libc::c_int as isize) as libc::c_int
            | 0x20 as libc::c_int <= 'z' as i32
        && *keystring.offset(6 as libc::c_int as isize) as libc::c_int == '\0' as i32
    {
        shifted_metas = 1 as libc::c_int != 0;
        return *keystring.offset(5 as libc::c_int as isize) as libc::c_int
            & 0x5f as libc::c_int;
    } else if *keystring.offset(0 as libc::c_int as isize) as libc::c_int == 'F' as i32 {
        let mut fn_0: libc::c_int = atoi(&*keystring.offset(1 as libc::c_int as isize));
        if fn_0 < 1 as libc::c_int || fn_0 > 24 as libc::c_int {
            return -(1 as libc::c_int);
        }
        return 0o410 as libc::c_int + fn_0;
    } else if strcasecmp(keystring, b"Ins\0" as *const u8 as *const libc::c_char)
        == 0 as libc::c_int
    {
        return 0o513 as libc::c_int
    } else if strcasecmp(keystring, b"Del\0" as *const u8 as *const libc::c_char)
        == 0 as libc::c_int
    {
        return 0o512 as libc::c_int
    } else {
        return -(1 as libc::c_int)
    };
}
pub unsafe extern "C" fn add_to_sclist(
    mut menus: libc::c_int,
    mut scstring: *const libc::c_char,
    keycode: libc::c_int,
    mut function: Option::<unsafe extern "C" fn() -> ()>,
    mut toggle: libc::c_int,
) {
    static mut tailsc: *mut keystruct = 0 as *const keystruct as *mut keystruct;
    static mut counter: libc::c_int = 0 as libc::c_int;
    let mut sc: *mut keystruct = nmalloc(
        ::std::mem::size_of::<keystruct>() as libc::c_ulong,
    ) as *mut keystruct;
    if sclist.is_null() {
        sclist = sc;
    } else {
        (*tailsc).next = sc;
    }
    (*sc).next = 0 as *mut keystruct;
    (*sc).menus = menus;
    (*sc).func = function;
    (*sc).toggle = toggle;
    if toggle != 0 {
        (*sc)
            .ordinal = if (*tailsc).toggle == toggle {
            counter
        } else {
            counter += 1;
            counter
        };
    }
    (*sc).keystr = scstring;
    (*sc).keycode = if keycode != 0 { keycode } else { keycode_from_string(scstring) };
    tailsc = sc;
}
pub unsafe extern "C" fn first_sc_for(
    mut menu: libc::c_int,
    mut function: Option::<unsafe extern "C" fn() -> ()>,
) -> *const keystruct {
    let mut sc: *mut keystruct = sclist;
    while !sc.is_null() {
        if (*sc).menus & menu != 0 && (*sc).func == function
            && *((*sc).keystr).offset(0 as libc::c_int as isize) as libc::c_int != 0
        {
            return sc;
        }
        sc = (*sc).next;
    }
    return 0 as *const keystruct;
}
pub unsafe extern "C" fn shown_entries_for(mut menu: libc::c_int) -> size_t {
    let mut item: *mut funcstruct = allfuncs;
    let mut maximum: size_t = ((COLS + 40 as libc::c_int) / 20 as libc::c_int
        * 2 as libc::c_int) as size_t;
    let mut count: size_t = 0 as libc::c_int as size_t;
    while count < maximum && !item.is_null() {
        if (*item).menus & menu != 0 {
            count = count.wrapping_add(1);
            count;
        }
        item = (*item).next;
    }
    if menu == (1 as libc::c_int) << 5 as libc::c_int && item.is_null()
        && (first_sc_for(menu, Some(discard_buffer as unsafe extern "C" fn() -> ())))
            .is_null()
    {
        count = count.wrapping_sub(1);
        count;
    }
    return count;
}
pub unsafe extern "C" fn get_shortcut(keycode: libc::c_int) -> *const keystruct {
    if !meta_key && 0x20 as libc::c_int <= keycode && keycode <= 0xff as libc::c_int {
        return 0 as *const keystruct;
    }
    if meta_key as libc::c_int != 0 && keycode < 0x20 as libc::c_int {
        return 0 as *const keystruct;
    }
    if bracketed_paste as libc::c_int != 0 && keycode != 0x4fb as libc::c_int {
        return 0 as *const keystruct;
    }
    if keycode == 0x4ec as libc::c_int {
        return planted_shortcut;
    }
    let mut sc: *mut keystruct = sclist;
    while !sc.is_null() {
        if (*sc).menus & currmenu != 0 && keycode == (*sc).keycode {
            return sc;
        }
        sc = (*sc).next;
    }
    return 0 as *const keystruct;
}
pub unsafe extern "C" fn func_from_key(keycode: libc::c_int) -> functionptrtype {
    let mut sc: *const keystruct = get_shortcut(keycode);
    return if !sc.is_null() { (*sc).func } else { None };
}
pub unsafe extern "C" fn interpret(keycode: libc::c_int) -> functionptrtype {
    if !meta_key {
        if keycode == 'N' as i32 {
            return Some(do_findprevious as unsafe extern "C" fn() -> ());
        }
        if keycode == 'n' as i32 {
            return Some(do_findnext as unsafe extern "C" fn() -> ());
        }
        match {
            let mut __res: libc::c_int = 0;
            if ::std::mem::size_of::<libc::c_int>() as libc::c_ulong
                > 1 as libc::c_int as libc::c_ulong
            {
                if 0 != 0 {
                    let mut __c: libc::c_int = keycode;
                    __res = if __c < -(128 as libc::c_int) || __c > 255 as libc::c_int {
                        __c
                    } else {
                        *(*__ctype_tolower_loc()).offset(__c as isize)
                    };
                } else {
                    __res = tolower(keycode);
                }
            } else {
                __res = *(*__ctype_tolower_loc()).offset(keycode as isize);
            }
            __res
        } {
            98 | 45 => return Some(do_page_up as unsafe extern "C" fn() -> ()),
            32 => return Some(do_page_down as unsafe extern "C" fn() -> ()),
            119 | 47 => return Some(do_search_forward as unsafe extern "C" fn() -> ()),
            103 => return Some(goto_dir as unsafe extern "C" fn() -> ()),
            63 => return Some(do_help as unsafe extern "C" fn() -> ()),
            115 => return Some(do_enter as unsafe extern "C" fn() -> ()),
            101 | 113 | 120 => return Some(do_exit as unsafe extern "C" fn() -> ()),
            _ => {}
        }
    }
    return func_from_key(keycode);
}
pub static mut exit_tag: *const libc::c_char = b"Exit\0" as *const u8
    as *const libc::c_char;
pub static mut close_tag: *const libc::c_char = b"Close\0" as *const u8
    as *const libc::c_char;
pub unsafe extern "C" fn shortcut_init() {
    let mut cancel_gist: *const libc::c_char = b"Cancel the current function\0"
        as *const u8 as *const libc::c_char;
    let mut help_gist: *const libc::c_char = b"Display this help text\0" as *const u8
        as *const libc::c_char;
    let mut exit_gist: *const libc::c_char = b"Close the current buffer / Exit from nano\0"
        as *const u8 as *const libc::c_char;
    let mut writeout_gist: *const libc::c_char = b"Write the current buffer (or the marked region) to disk\0"
        as *const u8 as *const libc::c_char;
    let mut readfile_gist: *const libc::c_char = b"Insert another file into current buffer (or into new buffer)\0"
        as *const u8 as *const libc::c_char;
    let mut whereis_gist: *const libc::c_char = b"Search forward for a string or a regular expression\0"
        as *const u8 as *const libc::c_char;
    let mut wherewas_gist: *const libc::c_char = b"Search backward for a string or a regular expression\0"
        as *const u8 as *const libc::c_char;
    let mut cut_gist: *const libc::c_char = b"Cut current line (or marked region) and store it in cutbuffer\0"
        as *const u8 as *const libc::c_char;
    let mut paste_gist: *const libc::c_char = b"Paste the contents of cutbuffer at current cursor position\0"
        as *const u8 as *const libc::c_char;
    let mut cursorpos_gist: *const libc::c_char = b"Display the position of the cursor\0"
        as *const u8 as *const libc::c_char;
    let mut spell_gist: *const libc::c_char = b"Invoke the spell checker, if available\0"
        as *const u8 as *const libc::c_char;
    let mut replace_gist: *const libc::c_char = b"Replace a string or a regular expression\0"
        as *const u8 as *const libc::c_char;
    let mut gotoline_gist: *const libc::c_char = b"Go to line and column number\0"
        as *const u8 as *const libc::c_char;
    let mut mark_gist: *const libc::c_char = b"Mark text starting from the cursor position\0"
        as *const u8 as *const libc::c_char;
    let mut copy_gist: *const libc::c_char = b"Copy current line (or marked region) and store it in cutbuffer\0"
        as *const u8 as *const libc::c_char;
    let mut zap_gist: *const libc::c_char = b"Throw away the current line (or marked region)\0"
        as *const u8 as *const libc::c_char;
    let mut indent_gist: *const libc::c_char = b"Indent the current line (or marked lines)\0"
        as *const u8 as *const libc::c_char;
    let mut unindent_gist: *const libc::c_char = b"Unindent the current line (or marked lines)\0"
        as *const u8 as *const libc::c_char;
    let mut undo_gist: *const libc::c_char = b"Undo the last operation\0" as *const u8
        as *const libc::c_char;
    let mut redo_gist: *const libc::c_char = b"Redo the last undone operation\0"
        as *const u8 as *const libc::c_char;
    let mut back_gist: *const libc::c_char = b"Go back one character\0" as *const u8
        as *const libc::c_char;
    let mut forward_gist: *const libc::c_char = b"Go forward one character\0"
        as *const u8 as *const libc::c_char;
    let mut prevword_gist: *const libc::c_char = b"Go back one word\0" as *const u8
        as *const libc::c_char;
    let mut nextword_gist: *const libc::c_char = b"Go forward one word\0" as *const u8
        as *const libc::c_char;
    let mut prevline_gist: *const libc::c_char = b"Go to previous line\0" as *const u8
        as *const libc::c_char;
    let mut nextline_gist: *const libc::c_char = b"Go to next line\0" as *const u8
        as *const libc::c_char;
    let mut home_gist: *const libc::c_char = b"Go to beginning of current line\0"
        as *const u8 as *const libc::c_char;
    let mut end_gist: *const libc::c_char = b"Go to end of current line\0" as *const u8
        as *const libc::c_char;
    let mut prevblock_gist: *const libc::c_char = b"Go to previous block of text\0"
        as *const u8 as *const libc::c_char;
    let mut nextblock_gist: *const libc::c_char = b"Go to next block of text\0"
        as *const u8 as *const libc::c_char;
    let mut parabegin_gist: *const libc::c_char = b"Go to beginning of paragraph; then of previous paragraph\0"
        as *const u8 as *const libc::c_char;
    let mut paraend_gist: *const libc::c_char = b"Go just beyond end of paragraph; then of next paragraph\0"
        as *const u8 as *const libc::c_char;
    let mut prevpage_gist: *const libc::c_char = b"Go one screenful up\0" as *const u8
        as *const libc::c_char;
    let mut nextpage_gist: *const libc::c_char = b"Go one screenful down\0" as *const u8
        as *const libc::c_char;
    let mut firstline_gist: *const libc::c_char = b"Go to the first line of the file\0"
        as *const u8 as *const libc::c_char;
    let mut lastline_gist: *const libc::c_char = b"Go to the last line of the file\0"
        as *const u8 as *const libc::c_char;
    let mut bracket_gist: *const libc::c_char = b"Go to the matching bracket\0"
        as *const u8 as *const libc::c_char;
    let mut scrollup_gist: *const libc::c_char = b"Scroll up one line without moving the cursor textually\0"
        as *const u8 as *const libc::c_char;
    let mut scrolldown_gist: *const libc::c_char = b"Scroll down one line without moving the cursor textually\0"
        as *const u8 as *const libc::c_char;
    let mut center_gist: *const libc::c_char = b"Center the line where the cursor is\0"
        as *const u8 as *const libc::c_char;
    let mut prevfile_gist: *const libc::c_char = b"Switch to the previous file buffer\0"
        as *const u8 as *const libc::c_char;
    let mut nextfile_gist: *const libc::c_char = b"Switch to the next file buffer\0"
        as *const u8 as *const libc::c_char;
    let mut verbatim_gist: *const libc::c_char = b"Insert the next keystroke verbatim\0"
        as *const u8 as *const libc::c_char;
    let mut tab_gist: *const libc::c_char = b"Insert a tab at the cursor position (or indent marked lines)\0"
        as *const u8 as *const libc::c_char;
    let mut enter_gist: *const libc::c_char = b"Insert a newline at the cursor position\0"
        as *const u8 as *const libc::c_char;
    let mut delete_gist: *const libc::c_char = b"Delete the character under the cursor\0"
        as *const u8 as *const libc::c_char;
    let mut backspace_gist: *const libc::c_char = b"Delete the character to the left of the cursor\0"
        as *const u8 as *const libc::c_char;
    let mut chopwordleft_gist: *const libc::c_char = b"Delete backward from cursor to word start\0"
        as *const u8 as *const libc::c_char;
    let mut chopwordright_gist: *const libc::c_char = b"Delete forward from cursor to next word start\0"
        as *const u8 as *const libc::c_char;
    let mut cuttilleof_gist: *const libc::c_char = b"Cut from the cursor position to the end of the file\0"
        as *const u8 as *const libc::c_char;
    let mut justify_gist: *const libc::c_char = b"Justify the current paragraph\0"
        as *const u8 as *const libc::c_char;
    let mut fulljustify_gist: *const libc::c_char = b"Justify the entire file\0"
        as *const u8 as *const libc::c_char;
    let mut wordcount_gist: *const libc::c_char = b"Count the number of lines, words, and characters\0"
        as *const u8 as *const libc::c_char;
    let mut suspend_gist: *const libc::c_char = b"Suspend the editor (return to the shell)\0"
        as *const u8 as *const libc::c_char;
    let mut refresh_gist: *const libc::c_char = b"Refresh (redraw) the current screen\0"
        as *const u8 as *const libc::c_char;
    let mut completion_gist: *const libc::c_char = b"Try and complete the current word\0"
        as *const u8 as *const libc::c_char;
    let mut comment_gist: *const libc::c_char = b"Comment/uncomment the current line (or marked lines)\0"
        as *const u8 as *const libc::c_char;
    let mut savefile_gist: *const libc::c_char = b"Save file without prompting\0"
        as *const u8 as *const libc::c_char;
    let mut findprev_gist: *const libc::c_char = b"Search next occurrence backward\0"
        as *const u8 as *const libc::c_char;
    let mut findnext_gist: *const libc::c_char = b"Search next occurrence forward\0"
        as *const u8 as *const libc::c_char;
    let mut recordmacro_gist: *const libc::c_char = b"Start/stop recording a macro\0"
        as *const u8 as *const libc::c_char;
    let mut runmacro_gist: *const libc::c_char = b"Run the last recorded macro\0"
        as *const u8 as *const libc::c_char;
    let mut anchor_gist: *const libc::c_char = b"Place or remove an anchor at the current line\0"
        as *const u8 as *const libc::c_char;
    let mut prevanchor_gist: *const libc::c_char = b"Jump backward to the nearest anchor\0"
        as *const u8 as *const libc::c_char;
    let mut nextanchor_gist: *const libc::c_char = b"Jump forward to the nearest anchor\0"
        as *const u8 as *const libc::c_char;
    let mut case_gist: *const libc::c_char = b"Toggle the case sensitivity of the search\0"
        as *const u8 as *const libc::c_char;
    let mut reverse_gist: *const libc::c_char = b"Reverse the direction of the search\0"
        as *const u8 as *const libc::c_char;
    let mut regexp_gist: *const libc::c_char = b"Toggle the use of regular expressions\0"
        as *const u8 as *const libc::c_char;
    let mut older_gist: *const libc::c_char = b"Recall the previous search/replace string\0"
        as *const u8 as *const libc::c_char;
    let mut newer_gist: *const libc::c_char = b"Recall the next search/replace string\0"
        as *const u8 as *const libc::c_char;
    let mut dos_gist: *const libc::c_char = b"Toggle the use of DOS format\0"
        as *const u8 as *const libc::c_char;
    let mut mac_gist: *const libc::c_char = b"Toggle the use of Mac format\0"
        as *const u8 as *const libc::c_char;
    let mut append_gist: *const libc::c_char = b"Toggle appending\0" as *const u8
        as *const libc::c_char;
    let mut prepend_gist: *const libc::c_char = b"Toggle prepending\0" as *const u8
        as *const libc::c_char;
    let mut backup_gist: *const libc::c_char = b"Toggle backing up of the original file\0"
        as *const u8 as *const libc::c_char;
    let mut execute_gist: *const libc::c_char = b"Execute a function or an external command\0"
        as *const u8 as *const libc::c_char;
    let mut pipe_gist: *const libc::c_char = b"Pipe the current buffer (or marked region) to the command\0"
        as *const u8 as *const libc::c_char;
    let mut convert_gist: *const libc::c_char = b"Do not convert from DOS/Mac format\0"
        as *const u8 as *const libc::c_char;
    let mut newbuffer_gist: *const libc::c_char = b"Toggle the use of a new buffer\0"
        as *const u8 as *const libc::c_char;
    let mut discardbuffer_gist: *const libc::c_char = b"Close buffer without saving it\0"
        as *const u8 as *const libc::c_char;
    let mut tofiles_gist: *const libc::c_char = b"Go to file browser\0" as *const u8
        as *const libc::c_char;
    let mut exitbrowser_gist: *const libc::c_char = b"Exit from the file browser\0"
        as *const u8 as *const libc::c_char;
    let mut firstfile_gist: *const libc::c_char = b"Go to the first file in the list\0"
        as *const u8 as *const libc::c_char;
    let mut lastfile_gist: *const libc::c_char = b"Go to the last file in the list\0"
        as *const u8 as *const libc::c_char;
    let mut backfile_gist: *const libc::c_char = b"Go to the previous file in the list\0"
        as *const u8 as *const libc::c_char;
    let mut forwardfile_gist: *const libc::c_char = b"Go to the next file in the list\0"
        as *const u8 as *const libc::c_char;
    let mut browserlefthand_gist: *const libc::c_char = b"Go to lefthand column\0"
        as *const u8 as *const libc::c_char;
    let mut browserrighthand_gist: *const libc::c_char = b"Go to righthand column\0"
        as *const u8 as *const libc::c_char;
    let mut browsertoprow_gist: *const libc::c_char = b"Go to first row in this column\0"
        as *const u8 as *const libc::c_char;
    let mut browserbottomrow_gist: *const libc::c_char = b"Go to last row in this column\0"
        as *const u8 as *const libc::c_char;
    let mut browserwhereis_gist: *const libc::c_char = b"Search forward for a string\0"
        as *const u8 as *const libc::c_char;
    let mut browserwherewas_gist: *const libc::c_char = b"Search backward for a string\0"
        as *const u8 as *const libc::c_char;
    let mut browserrefresh_gist: *const libc::c_char = b"Refresh the file list\0"
        as *const u8 as *const libc::c_char;
    let mut gotodir_gist: *const libc::c_char = b"Go to directory\0" as *const u8
        as *const libc::c_char;
    let mut lint_gist: *const libc::c_char = b"Invoke the linter, if available\0"
        as *const u8 as *const libc::c_char;
    let mut prevlint_gist: *const libc::c_char = b"Go to previous linter msg\0"
        as *const u8 as *const libc::c_char;
    let mut nextlint_gist: *const libc::c_char = b"Go to next linter msg\0" as *const u8
        as *const libc::c_char;
    let mut formatter_gist: *const libc::c_char = b"Invoke a program to format/arrange/manipulate the buffer\0"
        as *const u8 as *const libc::c_char;
    add_to_funcs(
        Some(do_help as unsafe extern "C" fn() -> ()),
        ((1 as libc::c_int) << 0 as libc::c_int | (1 as libc::c_int) << 1 as libc::c_int
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
            | (1 as libc::c_int) << 10 as libc::c_int)
            & !((1 as libc::c_int) << 15 as libc::c_int),
        b"Help\0" as *const u8 as *const libc::c_char,
        help_gist,
        0 as libc::c_int != 0,
    );
    add_to_funcs(
        Some(do_cancel as unsafe extern "C" fn() -> ()),
        ((1 as libc::c_int) << 0 as libc::c_int | (1 as libc::c_int) << 1 as libc::c_int
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
            | (1 as libc::c_int) << 14 as libc::c_int)
            & !((1 as libc::c_int) << 0 as libc::c_int)
            | (1 as libc::c_int) << 13 as libc::c_int,
        b"Cancel\0" as *const u8 as *const libc::c_char,
        cancel_gist,
        1 as libc::c_int != 0,
    );
    add_to_funcs(
        Some(do_exit as unsafe extern "C" fn() -> ()),
        (1 as libc::c_int) << 0 as libc::c_int,
        exit_tag,
        exit_gist,
        0 as libc::c_int != 0,
    );
    exitfunc = tailfunc;
    add_to_funcs(
        Some(do_exit as unsafe extern "C" fn() -> ()),
        (1 as libc::c_int) << 10 as libc::c_int,
        close_tag,
        exitbrowser_gist,
        0 as libc::c_int != 0,
    );
    add_to_funcs(
        Some(do_writeout as unsafe extern "C" fn() -> ()),
        (1 as libc::c_int) << 0 as libc::c_int,
        b"Write Out\0" as *const u8 as *const libc::c_char,
        writeout_gist,
        0 as libc::c_int != 0,
    );
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
    {
        add_to_funcs(
            Some(do_insertfile as unsafe extern "C" fn() -> ()),
            (1 as libc::c_int) << 0 as libc::c_int,
            b"Read File\0" as *const u8 as *const libc::c_char,
            readfile_gist,
            1 as libc::c_int != 0,
        );
    } else {
        add_to_funcs(
            Some(do_justify as unsafe extern "C" fn() -> ()),
            (1 as libc::c_int) << 0 as libc::c_int,
            b"Justify\0" as *const u8 as *const libc::c_char,
            justify_gist,
            1 as libc::c_int != 0,
        );
    }
    add_to_funcs(
        Some(full_refresh as unsafe extern "C" fn() -> ()),
        (1 as libc::c_int) << 8 as libc::c_int,
        b"Refresh\0" as *const u8 as *const libc::c_char,
        b"x\0" as *const u8 as *const libc::c_char,
        0 as libc::c_int != 0,
    );
    add_to_funcs(
        Some(do_exit as unsafe extern "C" fn() -> ()),
        (1 as libc::c_int) << 8 as libc::c_int,
        close_tag,
        b"x\0" as *const u8 as *const libc::c_char,
        0 as libc::c_int != 0,
    );
    add_to_funcs(
        Some(do_search_forward as unsafe extern "C" fn() -> ()),
        (1 as libc::c_int) << 0 as libc::c_int | (1 as libc::c_int) << 8 as libc::c_int,
        b"Where Is\0" as *const u8 as *const libc::c_char,
        whereis_gist,
        0 as libc::c_int != 0,
    );
    add_to_funcs(
        Some(do_replace as unsafe extern "C" fn() -> ()),
        (1 as libc::c_int) << 0 as libc::c_int,
        b"Replace\0" as *const u8 as *const libc::c_char,
        replace_gist,
        0 as libc::c_int != 0,
    );
    add_to_funcs(
        Some(cut_text as unsafe extern "C" fn() -> ()),
        (1 as libc::c_int) << 0 as libc::c_int,
        b"Cut\0" as *const u8 as *const libc::c_char,
        cut_gist,
        0 as libc::c_int != 0,
    );
    add_to_funcs(
        Some(paste_text as unsafe extern "C" fn() -> ()),
        (1 as libc::c_int) << 0 as libc::c_int,
        b"Paste\0" as *const u8 as *const libc::c_char,
        paste_gist,
        1 as libc::c_int != 0,
    );
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
    {
        add_to_funcs(
            Some(do_execute as unsafe extern "C" fn() -> ()),
            (1 as libc::c_int) << 0 as libc::c_int,
            b"Execute\0" as *const u8 as *const libc::c_char,
            execute_gist,
            0 as libc::c_int != 0,
        );
        add_to_funcs(
            Some(do_justify as unsafe extern "C" fn() -> ()),
            (1 as libc::c_int) << 0 as libc::c_int,
            b"Justify\0" as *const u8 as *const libc::c_char,
            justify_gist,
            1 as libc::c_int != 0,
        );
    }
    add_to_funcs(
        Some(report_cursor_position as unsafe extern "C" fn() -> ()),
        (1 as libc::c_int) << 0 as libc::c_int,
        b"Location\0" as *const u8 as *const libc::c_char,
        cursorpos_gist,
        0 as libc::c_int != 0,
    );
    add_to_funcs(
        Some(do_gotolinecolumn as unsafe extern "C" fn() -> ()),
        (1 as libc::c_int) << 0 as libc::c_int,
        b"Go To Line\0" as *const u8 as *const libc::c_char,
        gotoline_gist,
        1 as libc::c_int != 0,
    );
    add_to_funcs(
        Some(do_undo as unsafe extern "C" fn() -> ()),
        (1 as libc::c_int) << 0 as libc::c_int,
        b"Undo\0" as *const u8 as *const libc::c_char,
        undo_gist,
        0 as libc::c_int != 0,
    );
    add_to_funcs(
        Some(do_redo as unsafe extern "C" fn() -> ()),
        (1 as libc::c_int) << 0 as libc::c_int,
        b"Redo\0" as *const u8 as *const libc::c_char,
        redo_gist,
        1 as libc::c_int != 0,
    );
    add_to_funcs(
        Some(do_mark as unsafe extern "C" fn() -> ()),
        (1 as libc::c_int) << 0 as libc::c_int,
        b"Set Mark\0" as *const u8 as *const libc::c_char,
        mark_gist,
        0 as libc::c_int != 0,
    );
    add_to_funcs(
        Some(copy_text as unsafe extern "C" fn() -> ()),
        (1 as libc::c_int) << 0 as libc::c_int,
        b"Copy\0" as *const u8 as *const libc::c_char,
        copy_gist,
        1 as libc::c_int != 0,
    );
    add_to_funcs(
        Some(case_sens_void as unsafe extern "C" fn() -> ()),
        (1 as libc::c_int) << 1 as libc::c_int | (1 as libc::c_int) << 2 as libc::c_int,
        b"Case Sens\0" as *const u8 as *const libc::c_char,
        case_gist,
        0 as libc::c_int != 0,
    );
    add_to_funcs(
        Some(regexp_void as unsafe extern "C" fn() -> ()),
        (1 as libc::c_int) << 1 as libc::c_int | (1 as libc::c_int) << 2 as libc::c_int,
        b"Reg.exp.\0" as *const u8 as *const libc::c_char,
        regexp_gist,
        0 as libc::c_int != 0,
    );
    add_to_funcs(
        Some(backwards_void as unsafe extern "C" fn() -> ()),
        (1 as libc::c_int) << 1 as libc::c_int | (1 as libc::c_int) << 2 as libc::c_int,
        b"Backwards\0" as *const u8 as *const libc::c_char,
        reverse_gist,
        1 as libc::c_int != 0,
    );
    add_to_funcs(
        Some(flip_replace as unsafe extern "C" fn() -> ()),
        (1 as libc::c_int) << 1 as libc::c_int,
        b"Replace\0" as *const u8 as *const libc::c_char,
        replace_gist,
        1 as libc::c_int != 0,
    );
    add_to_funcs(
        Some(flip_replace as unsafe extern "C" fn() -> ()),
        (1 as libc::c_int) << 2 as libc::c_int,
        b"No Replace\0" as *const u8 as *const libc::c_char,
        whereis_gist,
        1 as libc::c_int != 0,
    );
    add_to_funcs(
        Some(get_older_item as unsafe extern "C" fn() -> ()),
        (1 as libc::c_int) << 1 as libc::c_int | (1 as libc::c_int) << 2 as libc::c_int
            | (1 as libc::c_int) << 3 as libc::c_int
            | (1 as libc::c_int) << 11 as libc::c_int,
        b"Older\0" as *const u8 as *const libc::c_char,
        older_gist,
        0 as libc::c_int != 0,
    );
    add_to_funcs(
        Some(get_newer_item as unsafe extern "C" fn() -> ()),
        (1 as libc::c_int) << 1 as libc::c_int | (1 as libc::c_int) << 2 as libc::c_int
            | (1 as libc::c_int) << 3 as libc::c_int
            | (1 as libc::c_int) << 11 as libc::c_int,
        b"Newer\0" as *const u8 as *const libc::c_char,
        newer_gist,
        1 as libc::c_int != 0,
    );
    add_to_funcs(
        Some(goto_dir as unsafe extern "C" fn() -> ()),
        (1 as libc::c_int) << 10 as libc::c_int,
        b"Go To Dir\0" as *const u8 as *const libc::c_char,
        gotodir_gist,
        0 as libc::c_int != 0,
    );
    add_to_funcs(
        Some(full_refresh as unsafe extern "C" fn() -> ()),
        (1 as libc::c_int) << 10 as libc::c_int,
        b"Refresh\0" as *const u8 as *const libc::c_char,
        browserrefresh_gist,
        1 as libc::c_int != 0,
    );
    add_to_funcs(
        Some(do_search_forward as unsafe extern "C" fn() -> ()),
        (1 as libc::c_int) << 10 as libc::c_int,
        b"Where Is\0" as *const u8 as *const libc::c_char,
        browserwhereis_gist,
        0 as libc::c_int != 0,
    );
    add_to_funcs(
        Some(do_search_backward as unsafe extern "C" fn() -> ()),
        (1 as libc::c_int) << 10 as libc::c_int,
        b"Where Was\0" as *const u8 as *const libc::c_char,
        browserwherewas_gist,
        0 as libc::c_int != 0,
    );
    add_to_funcs(
        Some(do_findprevious as unsafe extern "C" fn() -> ()),
        (1 as libc::c_int) << 10 as libc::c_int,
        b"Previous\0" as *const u8 as *const libc::c_char,
        findprev_gist,
        0 as libc::c_int != 0,
    );
    add_to_funcs(
        Some(do_findnext as unsafe extern "C" fn() -> ()),
        (1 as libc::c_int) << 10 as libc::c_int,
        b"Next\0" as *const u8 as *const libc::c_char,
        findnext_gist,
        1 as libc::c_int != 0,
    );
    add_to_funcs(
        Some(do_find_bracket as unsafe extern "C" fn() -> ()),
        (1 as libc::c_int) << 0 as libc::c_int,
        b"To Bracket\0" as *const u8 as *const libc::c_char,
        bracket_gist,
        1 as libc::c_int != 0,
    );
    add_to_funcs(
        Some(do_search_backward as unsafe extern "C" fn() -> ()),
        (1 as libc::c_int) << 0 as libc::c_int | (1 as libc::c_int) << 8 as libc::c_int,
        b"Where Was\0" as *const u8 as *const libc::c_char,
        wherewas_gist,
        0 as libc::c_int != 0,
    );
    add_to_funcs(
        Some(do_findprevious as unsafe extern "C" fn() -> ()),
        (1 as libc::c_int) << 0 as libc::c_int | (1 as libc::c_int) << 8 as libc::c_int,
        b"Previous\0" as *const u8 as *const libc::c_char,
        findprev_gist,
        0 as libc::c_int != 0,
    );
    add_to_funcs(
        Some(do_findnext as unsafe extern "C" fn() -> ()),
        (1 as libc::c_int) << 0 as libc::c_int | (1 as libc::c_int) << 8 as libc::c_int,
        b"Next\0" as *const u8 as *const libc::c_char,
        findnext_gist,
        1 as libc::c_int != 0,
    );
    add_to_funcs(
        Some(do_left as unsafe extern "C" fn() -> ()),
        (1 as libc::c_int) << 0 as libc::c_int,
        b"Back\0" as *const u8 as *const libc::c_char,
        back_gist,
        0 as libc::c_int != 0,
    );
    add_to_funcs(
        Some(do_right as unsafe extern "C" fn() -> ()),
        (1 as libc::c_int) << 0 as libc::c_int,
        b"Forward\0" as *const u8 as *const libc::c_char,
        forward_gist,
        0 as libc::c_int != 0,
    );
    add_to_funcs(
        Some(do_left as unsafe extern "C" fn() -> ()),
        (1 as libc::c_int) << 10 as libc::c_int,
        b"Back\0" as *const u8 as *const libc::c_char,
        backfile_gist,
        0 as libc::c_int != 0,
    );
    add_to_funcs(
        Some(do_right as unsafe extern "C" fn() -> ()),
        (1 as libc::c_int) << 10 as libc::c_int,
        b"Forward\0" as *const u8 as *const libc::c_char,
        forwardfile_gist,
        0 as libc::c_int != 0,
    );
    add_to_funcs(
        Some(to_prev_word as unsafe extern "C" fn() -> ()),
        (1 as libc::c_int) << 0 as libc::c_int,
        b"Prev Word\0" as *const u8 as *const libc::c_char,
        prevword_gist,
        0 as libc::c_int != 0,
    );
    add_to_funcs(
        Some(to_next_word as unsafe extern "C" fn() -> ()),
        (1 as libc::c_int) << 0 as libc::c_int,
        b"Next Word\0" as *const u8 as *const libc::c_char,
        nextword_gist,
        0 as libc::c_int != 0,
    );
    add_to_funcs(
        Some(do_home as unsafe extern "C" fn() -> ()),
        (1 as libc::c_int) << 0 as libc::c_int,
        b"Home\0" as *const u8 as *const libc::c_char,
        home_gist,
        0 as libc::c_int != 0,
    );
    add_to_funcs(
        Some(do_end as unsafe extern "C" fn() -> ()),
        (1 as libc::c_int) << 0 as libc::c_int,
        b"End\0" as *const u8 as *const libc::c_char,
        end_gist,
        1 as libc::c_int != 0,
    );
    add_to_funcs(
        Some(do_up as unsafe extern "C" fn() -> ()),
        (1 as libc::c_int) << 0 as libc::c_int | (1 as libc::c_int) << 10 as libc::c_int
            | (1 as libc::c_int) << 8 as libc::c_int,
        b"Prev Line\0" as *const u8 as *const libc::c_char,
        prevline_gist,
        0 as libc::c_int != 0,
    );
    add_to_funcs(
        Some(do_down as unsafe extern "C" fn() -> ()),
        (1 as libc::c_int) << 0 as libc::c_int | (1 as libc::c_int) << 10 as libc::c_int
            | (1 as libc::c_int) << 8 as libc::c_int,
        b"Next Line\0" as *const u8 as *const libc::c_char,
        nextline_gist,
        0 as libc::c_int != 0,
    );
    add_to_funcs(
        Some(do_scroll_up as unsafe extern "C" fn() -> ()),
        (1 as libc::c_int) << 0 as libc::c_int,
        b"Scroll Up\0" as *const u8 as *const libc::c_char,
        scrollup_gist,
        0 as libc::c_int != 0,
    );
    add_to_funcs(
        Some(do_scroll_down as unsafe extern "C" fn() -> ()),
        (1 as libc::c_int) << 0 as libc::c_int,
        b"Scroll Down\0" as *const u8 as *const libc::c_char,
        scrolldown_gist,
        1 as libc::c_int != 0,
    );
    add_to_funcs(
        Some(to_prev_block as unsafe extern "C" fn() -> ()),
        (1 as libc::c_int) << 0 as libc::c_int,
        b"Prev Block\0" as *const u8 as *const libc::c_char,
        prevblock_gist,
        0 as libc::c_int != 0,
    );
    add_to_funcs(
        Some(to_next_block as unsafe extern "C" fn() -> ()),
        (1 as libc::c_int) << 0 as libc::c_int,
        b"Next Block\0" as *const u8 as *const libc::c_char,
        nextblock_gist,
        0 as libc::c_int != 0,
    );
    add_to_funcs(
        Some(to_para_begin as unsafe extern "C" fn() -> ()),
        (1 as libc::c_int) << 0 as libc::c_int | (1 as libc::c_int) << 4 as libc::c_int,
        b"Begin of Paragr.\0" as *const u8 as *const libc::c_char,
        parabegin_gist,
        0 as libc::c_int != 0,
    );
    add_to_funcs(
        Some(to_para_end as unsafe extern "C" fn() -> ()),
        (1 as libc::c_int) << 0 as libc::c_int | (1 as libc::c_int) << 4 as libc::c_int,
        b"End of Paragraph\0" as *const u8 as *const libc::c_char,
        paraend_gist,
        1 as libc::c_int != 0,
    );
    add_to_funcs(
        Some(do_page_up as unsafe extern "C" fn() -> ()),
        (1 as libc::c_int) << 0 as libc::c_int | (1 as libc::c_int) << 8 as libc::c_int,
        b"Prev Page\0" as *const u8 as *const libc::c_char,
        prevpage_gist,
        0 as libc::c_int != 0,
    );
    add_to_funcs(
        Some(do_page_down as unsafe extern "C" fn() -> ()),
        (1 as libc::c_int) << 0 as libc::c_int | (1 as libc::c_int) << 8 as libc::c_int,
        b"Next Page\0" as *const u8 as *const libc::c_char,
        nextpage_gist,
        0 as libc::c_int != 0,
    );
    add_to_funcs(
        Some(to_first_line as unsafe extern "C" fn() -> ()),
        (1 as libc::c_int) << 0 as libc::c_int | (1 as libc::c_int) << 8 as libc::c_int
            | (1 as libc::c_int) << 4 as libc::c_int,
        b"First Line\0" as *const u8 as *const libc::c_char,
        firstline_gist,
        0 as libc::c_int != 0,
    );
    add_to_funcs(
        Some(to_last_line as unsafe extern "C" fn() -> ()),
        (1 as libc::c_int) << 0 as libc::c_int | (1 as libc::c_int) << 8 as libc::c_int
            | (1 as libc::c_int) << 4 as libc::c_int,
        b"Last Line\0" as *const u8 as *const libc::c_char,
        lastline_gist,
        1 as libc::c_int != 0,
    );
    add_to_funcs(
        Some(switch_to_prev_buffer as unsafe extern "C" fn() -> ()),
        (1 as libc::c_int) << 0 as libc::c_int,
        b"Prev File\0" as *const u8 as *const libc::c_char,
        prevfile_gist,
        0 as libc::c_int != 0,
    );
    add_to_funcs(
        Some(switch_to_next_buffer as unsafe extern "C" fn() -> ()),
        (1 as libc::c_int) << 0 as libc::c_int,
        b"Next File\0" as *const u8 as *const libc::c_char,
        nextfile_gist,
        1 as libc::c_int != 0,
    );
    add_to_funcs(
        Some(do_tab as unsafe extern "C" fn() -> ()),
        (1 as libc::c_int) << 0 as libc::c_int,
        b"Tab\0" as *const u8 as *const libc::c_char,
        tab_gist,
        0 as libc::c_int != 0,
    );
    add_to_funcs(
        Some(do_enter as unsafe extern "C" fn() -> ()),
        (1 as libc::c_int) << 0 as libc::c_int,
        b"Enter\0" as *const u8 as *const libc::c_char,
        enter_gist,
        1 as libc::c_int != 0,
    );
    add_to_funcs(
        Some(do_backspace as unsafe extern "C" fn() -> ()),
        (1 as libc::c_int) << 0 as libc::c_int,
        b"Backspace\0" as *const u8 as *const libc::c_char,
        backspace_gist,
        0 as libc::c_int != 0,
    );
    add_to_funcs(
        Some(do_delete as unsafe extern "C" fn() -> ()),
        (1 as libc::c_int) << 0 as libc::c_int,
        b"Delete\0" as *const u8 as *const libc::c_char,
        delete_gist,
        1 as libc::c_int != 0,
    );
    add_to_funcs(
        Some(chop_previous_word as unsafe extern "C" fn() -> ()),
        (1 as libc::c_int) << 0 as libc::c_int,
        b"Chop Left\0" as *const u8 as *const libc::c_char,
        chopwordleft_gist,
        0 as libc::c_int != 0,
    );
    add_to_funcs(
        Some(chop_next_word as unsafe extern "C" fn() -> ()),
        (1 as libc::c_int) << 0 as libc::c_int,
        b"Chop Right\0" as *const u8 as *const libc::c_char,
        chopwordright_gist,
        0 as libc::c_int != 0,
    );
    add_to_funcs(
        Some(cut_till_eof as unsafe extern "C" fn() -> ()),
        (1 as libc::c_int) << 0 as libc::c_int,
        b"Cut Till End\0" as *const u8 as *const libc::c_char,
        cuttilleof_gist,
        1 as libc::c_int != 0,
    );
    add_to_funcs(
        Some(do_full_justify as unsafe extern "C" fn() -> ()),
        (1 as libc::c_int) << 0 as libc::c_int,
        b"Full Justify\0" as *const u8 as *const libc::c_char,
        fulljustify_gist,
        0 as libc::c_int != 0,
    );
    add_to_funcs(
        Some(count_lines_words_and_characters as unsafe extern "C" fn() -> ()),
        (1 as libc::c_int) << 0 as libc::c_int,
        b"Word Count\0" as *const u8 as *const libc::c_char,
        wordcount_gist,
        0 as libc::c_int != 0,
    );
    add_to_funcs(
        Some(do_verbatim_input as unsafe extern "C" fn() -> ()),
        (1 as libc::c_int) << 0 as libc::c_int,
        b"Verbatim\0" as *const u8 as *const libc::c_char,
        verbatim_gist,
        1 as libc::c_int != 0,
    );
    add_to_funcs(
        Some(do_indent as unsafe extern "C" fn() -> ()),
        (1 as libc::c_int) << 0 as libc::c_int,
        b"Indent\0" as *const u8 as *const libc::c_char,
        indent_gist,
        0 as libc::c_int != 0,
    );
    add_to_funcs(
        Some(do_unindent as unsafe extern "C" fn() -> ()),
        (1 as libc::c_int) << 0 as libc::c_int,
        b"Unindent\0" as *const u8 as *const libc::c_char,
        unindent_gist,
        1 as libc::c_int != 0,
    );
    add_to_funcs(
        Some(do_comment as unsafe extern "C" fn() -> ()),
        (1 as libc::c_int) << 0 as libc::c_int,
        b"Comment Lines\0" as *const u8 as *const libc::c_char,
        comment_gist,
        0 as libc::c_int != 0,
    );
    add_to_funcs(
        Some(complete_a_word as unsafe extern "C" fn() -> ()),
        (1 as libc::c_int) << 0 as libc::c_int,
        b"Complete\0" as *const u8 as *const libc::c_char,
        completion_gist,
        1 as libc::c_int != 0,
    );
    add_to_funcs(
        Some(record_macro as unsafe extern "C" fn() -> ()),
        (1 as libc::c_int) << 0 as libc::c_int,
        b"Record\0" as *const u8 as *const libc::c_char,
        recordmacro_gist,
        0 as libc::c_int != 0,
    );
    add_to_funcs(
        Some(run_macro as unsafe extern "C" fn() -> ()),
        (1 as libc::c_int) << 0 as libc::c_int,
        b"Run Macro\0" as *const u8 as *const libc::c_char,
        runmacro_gist,
        1 as libc::c_int != 0,
    );
    add_to_funcs(
        Some(zap_text as unsafe extern "C" fn() -> ()),
        (1 as libc::c_int) << 0 as libc::c_int,
        b"Zap\0" as *const u8 as *const libc::c_char,
        zap_gist,
        1 as libc::c_int != 0,
    );
    add_to_funcs(
        Some(put_or_lift_anchor as unsafe extern "C" fn() -> ()),
        (1 as libc::c_int) << 0 as libc::c_int,
        b"Anchor\0" as *const u8 as *const libc::c_char,
        anchor_gist,
        0 as libc::c_int != 0,
    );
    add_to_funcs(
        Some(to_prev_anchor as unsafe extern "C" fn() -> ()),
        (1 as libc::c_int) << 0 as libc::c_int,
        b"Up to anchor\0" as *const u8 as *const libc::c_char,
        prevanchor_gist,
        0 as libc::c_int != 0,
    );
    add_to_funcs(
        Some(to_next_anchor as unsafe extern "C" fn() -> ()),
        (1 as libc::c_int) << 0 as libc::c_int,
        b"Down to anchor\0" as *const u8 as *const libc::c_char,
        nextanchor_gist,
        1 as libc::c_int != 0,
    );
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
    {
        add_to_funcs(
            Some(do_spell as unsafe extern "C" fn() -> ()),
            (1 as libc::c_int) << 0 as libc::c_int,
            b"Spell Check\0" as *const u8 as *const libc::c_char,
            spell_gist,
            0 as libc::c_int != 0,
        );
        add_to_funcs(
            Some(do_linter as unsafe extern "C" fn() -> ()),
            (1 as libc::c_int) << 0 as libc::c_int,
            b"Linter\0" as *const u8 as *const libc::c_char,
            lint_gist,
            0 as libc::c_int != 0,
        );
        add_to_funcs(
            Some(do_formatter as unsafe extern "C" fn() -> ()),
            (1 as libc::c_int) << 0 as libc::c_int,
            b"Formatter\0" as *const u8 as *const libc::c_char,
            formatter_gist,
            1 as libc::c_int != 0,
        );
    }
    add_to_funcs(
        Some(do_suspend as unsafe extern "C" fn() -> ()),
        (1 as libc::c_int) << 0 as libc::c_int,
        b"Suspend\0" as *const u8 as *const libc::c_char,
        suspend_gist,
        0 as libc::c_int != 0,
    );
    add_to_funcs(
        Some(full_refresh as unsafe extern "C" fn() -> ()),
        (1 as libc::c_int) << 0 as libc::c_int,
        b"Refresh\0" as *const u8 as *const libc::c_char,
        refresh_gist,
        0 as libc::c_int != 0,
    );
    add_to_funcs(
        Some(do_center as unsafe extern "C" fn() -> ()),
        (1 as libc::c_int) << 0 as libc::c_int,
        b"Center\0" as *const u8 as *const libc::c_char,
        center_gist,
        1 as libc::c_int != 0,
    );
    add_to_funcs(
        Some(do_savefile as unsafe extern "C" fn() -> ()),
        (1 as libc::c_int) << 0 as libc::c_int,
        b"Save\0" as *const u8 as *const libc::c_char,
        savefile_gist,
        1 as libc::c_int != 0,
    );
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
        add_to_funcs(
            Some(flip_newbuffer as unsafe extern "C" fn() -> ()),
            (1 as libc::c_int) << 6 as libc::c_int
                | (1 as libc::c_int) << 7 as libc::c_int,
            b"New Buffer\0" as *const u8 as *const libc::c_char,
            newbuffer_gist,
            0 as libc::c_int != 0,
        );
    }
    add_to_funcs(
        Some(flip_pipe as unsafe extern "C" fn() -> ()),
        (1 as libc::c_int) << 7 as libc::c_int,
        b"Pipe Text\0" as *const u8 as *const libc::c_char,
        pipe_gist,
        1 as libc::c_int != 0,
    );
    add_to_funcs(
        Some(do_spell as unsafe extern "C" fn() -> ()),
        (1 as libc::c_int) << 7 as libc::c_int,
        b"Spell Check\0" as *const u8 as *const libc::c_char,
        spell_gist,
        0 as libc::c_int != 0,
    );
    add_to_funcs(
        Some(do_linter as unsafe extern "C" fn() -> ()),
        (1 as libc::c_int) << 7 as libc::c_int,
        b"Linter\0" as *const u8 as *const libc::c_char,
        lint_gist,
        1 as libc::c_int != 0,
    );
    add_to_funcs(
        Some(do_full_justify as unsafe extern "C" fn() -> ()),
        (1 as libc::c_int) << 7 as libc::c_int,
        b"Full Justify\0" as *const u8 as *const libc::c_char,
        fulljustify_gist,
        0 as libc::c_int != 0,
    );
    add_to_funcs(
        Some(do_formatter as unsafe extern "C" fn() -> ()),
        (1 as libc::c_int) << 7 as libc::c_int,
        b"Formatter\0" as *const u8 as *const libc::c_char,
        formatter_gist,
        1 as libc::c_int != 0,
    );
    add_to_funcs(
        Some(flip_goto as unsafe extern "C" fn() -> ()),
        (1 as libc::c_int) << 1 as libc::c_int,
        b"Go To Line\0" as *const u8 as *const libc::c_char,
        gotoline_gist,
        1 as libc::c_int != 0,
    );
    add_to_funcs(
        Some(flip_goto as unsafe extern "C" fn() -> ()),
        (1 as libc::c_int) << 4 as libc::c_int,
        b"Go To Text\0" as *const u8 as *const libc::c_char,
        whereis_gist,
        1 as libc::c_int != 0,
    );
    add_to_funcs(
        Some(dos_format as unsafe extern "C" fn() -> ()),
        (1 as libc::c_int) << 5 as libc::c_int,
        b"DOS Format\0" as *const u8 as *const libc::c_char,
        dos_gist,
        0 as libc::c_int != 0,
    );
    add_to_funcs(
        Some(mac_format as unsafe extern "C" fn() -> ()),
        (1 as libc::c_int) << 5 as libc::c_int,
        b"Mac Format\0" as *const u8 as *const libc::c_char,
        mac_gist,
        0 as libc::c_int != 0,
    );
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
    {
        add_to_funcs(
            Some(append_it as unsafe extern "C" fn() -> ()),
            (1 as libc::c_int) << 5 as libc::c_int,
            b"Append\0" as *const u8 as *const libc::c_char,
            append_gist,
            0 as libc::c_int != 0,
        );
        add_to_funcs(
            Some(prepend_it as unsafe extern "C" fn() -> ()),
            (1 as libc::c_int) << 5 as libc::c_int,
            b"Prepend\0" as *const u8 as *const libc::c_char,
            prepend_gist,
            0 as libc::c_int != 0,
        );
        add_to_funcs(
            Some(back_it_up as unsafe extern "C" fn() -> ()),
            (1 as libc::c_int) << 5 as libc::c_int,
            b"Backup File\0" as *const u8 as *const libc::c_char,
            backup_gist,
            1 as libc::c_int != 0,
        );
    }
    add_to_funcs(
        Some(flip_convert as unsafe extern "C" fn() -> ()),
        (1 as libc::c_int) << 6 as libc::c_int,
        b"No Conversion\0" as *const u8 as *const libc::c_char,
        convert_gist,
        1 as libc::c_int != 0,
    );
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
        add_to_funcs(
            Some(flip_execute as unsafe extern "C" fn() -> ()),
            (1 as libc::c_int) << 6 as libc::c_int,
            b"Execute Command\0" as *const u8 as *const libc::c_char,
            execute_gist,
            1 as libc::c_int != 0,
        );
        add_to_funcs(
            Some(cut_till_eof as unsafe extern "C" fn() -> ()),
            (1 as libc::c_int) << 7 as libc::c_int,
            b"Cut Till End\0" as *const u8 as *const libc::c_char,
            cuttilleof_gist,
            1 as libc::c_int != 0,
        );
        add_to_funcs(
            Some(do_suspend as unsafe extern "C" fn() -> ()),
            (1 as libc::c_int) << 7 as libc::c_int,
            b"Suspend\0" as *const u8 as *const libc::c_char,
            suspend_gist,
            1 as libc::c_int != 0,
        );
    }
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
    {
        add_to_funcs(
            Some(to_files as unsafe extern "C" fn() -> ()),
            (1 as libc::c_int) << 5 as libc::c_int
                | (1 as libc::c_int) << 6 as libc::c_int,
            b"Browse\0" as *const u8 as *const libc::c_char,
            tofiles_gist,
            1 as libc::c_int != 0,
        );
    }
    add_to_funcs(
        Some(do_page_up as unsafe extern "C" fn() -> ()),
        (1 as libc::c_int) << 10 as libc::c_int,
        b"Prev Page\0" as *const u8 as *const libc::c_char,
        prevpage_gist,
        0 as libc::c_int != 0,
    );
    add_to_funcs(
        Some(do_page_down as unsafe extern "C" fn() -> ()),
        (1 as libc::c_int) << 10 as libc::c_int,
        b"Next Page\0" as *const u8 as *const libc::c_char,
        nextpage_gist,
        0 as libc::c_int != 0,
    );
    add_to_funcs(
        Some(to_first_file as unsafe extern "C" fn() -> ()),
        (1 as libc::c_int) << 10 as libc::c_int
            | (1 as libc::c_int) << 11 as libc::c_int,
        b"First File\0" as *const u8 as *const libc::c_char,
        firstfile_gist,
        0 as libc::c_int != 0,
    );
    add_to_funcs(
        Some(to_last_file as unsafe extern "C" fn() -> ()),
        (1 as libc::c_int) << 10 as libc::c_int
            | (1 as libc::c_int) << 11 as libc::c_int,
        b"Last File\0" as *const u8 as *const libc::c_char,
        lastfile_gist,
        1 as libc::c_int != 0,
    );
    add_to_funcs(
        Some(to_prev_word as unsafe extern "C" fn() -> ()),
        (1 as libc::c_int) << 10 as libc::c_int,
        b"Left Column\0" as *const u8 as *const libc::c_char,
        browserlefthand_gist,
        0 as libc::c_int != 0,
    );
    add_to_funcs(
        Some(to_next_word as unsafe extern "C" fn() -> ()),
        (1 as libc::c_int) << 10 as libc::c_int,
        b"Right Column\0" as *const u8 as *const libc::c_char,
        browserrighthand_gist,
        0 as libc::c_int != 0,
    );
    add_to_funcs(
        Some(to_prev_block as unsafe extern "C" fn() -> ()),
        (1 as libc::c_int) << 10 as libc::c_int,
        b"Top Row\0" as *const u8 as *const libc::c_char,
        browsertoprow_gist,
        0 as libc::c_int != 0,
    );
    add_to_funcs(
        Some(to_next_block as unsafe extern "C" fn() -> ()),
        (1 as libc::c_int) << 10 as libc::c_int,
        b"Bottom Row\0" as *const u8 as *const libc::c_char,
        browserbottomrow_gist,
        1 as libc::c_int != 0,
    );
    add_to_funcs(
        Some(discard_buffer as unsafe extern "C" fn() -> ()),
        (1 as libc::c_int) << 5 as libc::c_int,
        b"Discard buffer\0" as *const u8 as *const libc::c_char,
        discardbuffer_gist,
        1 as libc::c_int != 0,
    );
    add_to_funcs(
        Some(do_page_up as unsafe extern "C" fn() -> ()),
        (1 as libc::c_int) << 14 as libc::c_int,
        b"Previous Linter message\0" as *const u8 as *const libc::c_char,
        prevlint_gist,
        0 as libc::c_int != 0,
    );
    add_to_funcs(
        Some(do_page_down as unsafe extern "C" fn() -> ()),
        (1 as libc::c_int) << 14 as libc::c_int,
        b"Next Linter message\0" as *const u8 as *const libc::c_char,
        nextlint_gist,
        0 as libc::c_int != 0,
    );
    add_to_sclist(
        (1 as libc::c_int) << 0 as libc::c_int | (1 as libc::c_int) << 1 as libc::c_int
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
            | (1 as libc::c_int) << 10 as libc::c_int,
        b"^M\0" as *const u8 as *const libc::c_char,
        '\r' as i32,
        Some(do_enter as unsafe extern "C" fn() -> ()),
        0 as libc::c_int,
    );
    add_to_sclist(
        (1 as libc::c_int) << 0 as libc::c_int | (1 as libc::c_int) << 1 as libc::c_int
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
            | (1 as libc::c_int) << 10 as libc::c_int,
        b"Enter\0" as *const u8 as *const libc::c_char,
        0o527 as libc::c_int,
        Some(do_enter as unsafe extern "C" fn() -> ()),
        0 as libc::c_int,
    );
    add_to_sclist(
        (1 as libc::c_int) << 0 as libc::c_int | (1 as libc::c_int) << 1 as libc::c_int
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
            | (1 as libc::c_int) << 14 as libc::c_int,
        b"^H\0" as *const u8 as *const libc::c_char,
        '\u{8}' as i32,
        Some(do_backspace as unsafe extern "C" fn() -> ()),
        0 as libc::c_int,
    );
    add_to_sclist(
        (1 as libc::c_int) << 0 as libc::c_int | (1 as libc::c_int) << 1 as libc::c_int
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
            | (1 as libc::c_int) << 14 as libc::c_int,
        b"Bsp\0" as *const u8 as *const libc::c_char,
        0o407 as libc::c_int,
        Some(do_backspace as unsafe extern "C" fn() -> ()),
        0 as libc::c_int,
    );
    add_to_sclist(
        (1 as libc::c_int) << 0 as libc::c_int | (1 as libc::c_int) << 1 as libc::c_int
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
            | (1 as libc::c_int) << 14 as libc::c_int,
        b"Sh-Del\0" as *const u8 as *const libc::c_char,
        0x45d as libc::c_int,
        Some(do_backspace as unsafe extern "C" fn() -> ()),
        0 as libc::c_int,
    );
    add_to_sclist(
        (1 as libc::c_int) << 0 as libc::c_int | (1 as libc::c_int) << 1 as libc::c_int
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
            | (1 as libc::c_int) << 14 as libc::c_int,
        b"^D\0" as *const u8 as *const libc::c_char,
        0 as libc::c_int,
        Some(do_delete as unsafe extern "C" fn() -> ()),
        0 as libc::c_int,
    );
    add_to_sclist(
        (1 as libc::c_int) << 0 as libc::c_int | (1 as libc::c_int) << 1 as libc::c_int
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
            | (1 as libc::c_int) << 14 as libc::c_int,
        b"Del\0" as *const u8 as *const libc::c_char,
        0o512 as libc::c_int,
        Some(do_delete as unsafe extern "C" fn() -> ()),
        0 as libc::c_int,
    );
    add_to_sclist(
        (1 as libc::c_int) << 0 as libc::c_int | (1 as libc::c_int) << 1 as libc::c_int
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
            | (1 as libc::c_int) << 14 as libc::c_int,
        b"^I\0" as *const u8 as *const libc::c_char,
        '\t' as i32,
        Some(do_tab as unsafe extern "C" fn() -> ()),
        0 as libc::c_int,
    );
    add_to_sclist(
        (1 as libc::c_int) << 0 as libc::c_int | (1 as libc::c_int) << 1 as libc::c_int
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
            | (1 as libc::c_int) << 14 as libc::c_int,
        b"Tab\0" as *const u8 as *const libc::c_char,
        '\t' as i32,
        Some(do_tab as unsafe extern "C" fn() -> ()),
        0 as libc::c_int,
    );
    add_to_sclist(
        ((1 as libc::c_int) << 0 as libc::c_int | (1 as libc::c_int) << 1 as libc::c_int
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
            | (1 as libc::c_int) << 10 as libc::c_int)
            & !((1 as libc::c_int) << 15 as libc::c_int),
        b"^G\0" as *const u8 as *const libc::c_char,
        0 as libc::c_int,
        Some(do_help as unsafe extern "C" fn() -> ()),
        0 as libc::c_int,
    );
    add_to_sclist(
        (1 as libc::c_int) << 0 as libc::c_int | (1 as libc::c_int) << 10 as libc::c_int
            | (1 as libc::c_int) << 8 as libc::c_int,
        b"^X\0" as *const u8 as *const libc::c_char,
        0 as libc::c_int,
        Some(do_exit as unsafe extern "C" fn() -> ()),
        0 as libc::c_int,
    );
    if !(flags[(PRESERVE as libc::c_int as libc::c_ulong)
        .wrapping_div(
            (::std::mem::size_of::<libc::c_uint>() as libc::c_ulong)
                .wrapping_mul(8 as libc::c_int as libc::c_ulong),
        ) as usize]
        & (1 as libc::c_int as libc::c_uint)
            << (PRESERVE as libc::c_int as libc::c_ulong)
                .wrapping_rem(
                    (::std::mem::size_of::<libc::c_uint>() as libc::c_ulong)
                        .wrapping_mul(8 as libc::c_int as libc::c_ulong),
                ) != 0 as libc::c_int as libc::c_uint)
    {
        add_to_sclist(
            (1 as libc::c_int) << 0 as libc::c_int,
            b"^S\0" as *const u8 as *const libc::c_char,
            0 as libc::c_int,
            Some(do_savefile as unsafe extern "C" fn() -> ()),
            0 as libc::c_int,
        );
    }
    add_to_sclist(
        (1 as libc::c_int) << 0 as libc::c_int,
        b"^O\0" as *const u8 as *const libc::c_char,
        0 as libc::c_int,
        Some(do_writeout as unsafe extern "C" fn() -> ()),
        0 as libc::c_int,
    );
    add_to_sclist(
        (1 as libc::c_int) << 0 as libc::c_int,
        b"^R\0" as *const u8 as *const libc::c_char,
        0 as libc::c_int,
        Some(do_insertfile as unsafe extern "C" fn() -> ()),
        0 as libc::c_int,
    );
    add_to_sclist(
        (1 as libc::c_int) << 0 as libc::c_int,
        b"Ins\0" as *const u8 as *const libc::c_char,
        0o513 as libc::c_int,
        Some(do_insertfile as unsafe extern "C" fn() -> ()),
        0 as libc::c_int,
    );
    if !(flags[(PRESERVE as libc::c_int as libc::c_ulong)
        .wrapping_div(
            (::std::mem::size_of::<libc::c_uint>() as libc::c_ulong)
                .wrapping_mul(8 as libc::c_int as libc::c_ulong),
        ) as usize]
        & (1 as libc::c_int as libc::c_uint)
            << (PRESERVE as libc::c_int as libc::c_ulong)
                .wrapping_rem(
                    (::std::mem::size_of::<libc::c_uint>() as libc::c_ulong)
                        .wrapping_mul(8 as libc::c_int as libc::c_ulong),
                ) != 0 as libc::c_int as libc::c_uint)
    {
        add_to_sclist(
            (1 as libc::c_int) << 0 as libc::c_int
                | (1 as libc::c_int) << 10 as libc::c_int
                | (1 as libc::c_int) << 8 as libc::c_int,
            b"^Q\0" as *const u8 as *const libc::c_char,
            0 as libc::c_int,
            Some(do_search_backward as unsafe extern "C" fn() -> ()),
            0 as libc::c_int,
        );
    }
    add_to_sclist(
        (1 as libc::c_int) << 0 as libc::c_int | (1 as libc::c_int) << 10 as libc::c_int
            | (1 as libc::c_int) << 8 as libc::c_int,
        b"^W\0" as *const u8 as *const libc::c_char,
        0 as libc::c_int,
        Some(do_search_forward as unsafe extern "C" fn() -> ()),
        0 as libc::c_int,
    );
    add_to_sclist(
        (1 as libc::c_int) << 0 as libc::c_int,
        b"^\\\0" as *const u8 as *const libc::c_char,
        0 as libc::c_int,
        Some(do_replace as unsafe extern "C" fn() -> ()),
        0 as libc::c_int,
    );
    add_to_sclist(
        (1 as libc::c_int) << 0 as libc::c_int,
        b"M-R\0" as *const u8 as *const libc::c_char,
        0 as libc::c_int,
        Some(do_replace as unsafe extern "C" fn() -> ()),
        0 as libc::c_int,
    );
    add_to_sclist(
        (1 as libc::c_int) << 0 as libc::c_int | (1 as libc::c_int) << 1 as libc::c_int
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
            | (1 as libc::c_int) << 14 as libc::c_int,
        b"^K\0" as *const u8 as *const libc::c_char,
        0 as libc::c_int,
        Some(cut_text as unsafe extern "C" fn() -> ()),
        0 as libc::c_int,
    );
    add_to_sclist(
        (1 as libc::c_int) << 0 as libc::c_int | (1 as libc::c_int) << 1 as libc::c_int
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
            | (1 as libc::c_int) << 14 as libc::c_int,
        b"^U\0" as *const u8 as *const libc::c_char,
        0 as libc::c_int,
        Some(paste_text as unsafe extern "C" fn() -> ()),
        0 as libc::c_int,
    );
    add_to_sclist(
        (1 as libc::c_int) << 0 as libc::c_int,
        b"^T\0" as *const u8 as *const libc::c_char,
        0 as libc::c_int,
        Some(do_execute as unsafe extern "C" fn() -> ()),
        0 as libc::c_int,
    );
    if !(flags[(PRESERVE as libc::c_int as libc::c_ulong)
        .wrapping_div(
            (::std::mem::size_of::<libc::c_uint>() as libc::c_ulong)
                .wrapping_mul(8 as libc::c_int as libc::c_ulong),
        ) as usize]
        & (1 as libc::c_int as libc::c_uint)
            << (PRESERVE as libc::c_int as libc::c_ulong)
                .wrapping_rem(
                    (::std::mem::size_of::<libc::c_uint>() as libc::c_ulong)
                        .wrapping_mul(8 as libc::c_int as libc::c_ulong),
                ) != 0 as libc::c_int as libc::c_uint)
    {
        add_to_sclist(
            (1 as libc::c_int) << 7 as libc::c_int,
            b"^S\0" as *const u8 as *const libc::c_char,
            0 as libc::c_int,
            Some(do_spell as unsafe extern "C" fn() -> ()),
            0 as libc::c_int,
        );
    }
    add_to_sclist(
        (1 as libc::c_int) << 7 as libc::c_int,
        b"^T\0" as *const u8 as *const libc::c_char,
        0 as libc::c_int,
        Some(do_spell as unsafe extern "C" fn() -> ()),
        0 as libc::c_int,
    );
    add_to_sclist(
        (1 as libc::c_int) << 0 as libc::c_int,
        b"^J\0" as *const u8 as *const libc::c_char,
        '\n' as i32,
        Some(do_justify as unsafe extern "C" fn() -> ()),
        0 as libc::c_int,
    );
    add_to_sclist(
        (1 as libc::c_int) << 0 as libc::c_int,
        b"M-B\0" as *const u8 as *const libc::c_char,
        0 as libc::c_int,
        Some(do_linter as unsafe extern "C" fn() -> ()),
        0 as libc::c_int,
    );
    add_to_sclist(
        (1 as libc::c_int) << 7 as libc::c_int,
        b"^Y\0" as *const u8 as *const libc::c_char,
        0 as libc::c_int,
        Some(do_linter as unsafe extern "C" fn() -> ()),
        0 as libc::c_int,
    );
    add_to_sclist(
        (1 as libc::c_int) << 0 as libc::c_int,
        b"M-F\0" as *const u8 as *const libc::c_char,
        0 as libc::c_int,
        Some(do_formatter as unsafe extern "C" fn() -> ()),
        0 as libc::c_int,
    );
    add_to_sclist(
        (1 as libc::c_int) << 7 as libc::c_int,
        b"^O\0" as *const u8 as *const libc::c_char,
        0 as libc::c_int,
        Some(do_formatter as unsafe extern "C" fn() -> ()),
        0 as libc::c_int,
    );
    add_to_sclist(
        (1 as libc::c_int) << 0 as libc::c_int,
        b"^C\0" as *const u8 as *const libc::c_char,
        0 as libc::c_int,
        Some(report_cursor_position as unsafe extern "C" fn() -> ()),
        0 as libc::c_int,
    );
    add_to_sclist(
        (1 as libc::c_int) << 0 as libc::c_int,
        if on_a_vt as libc::c_int != 0 {
            b"^-\0" as *const u8 as *const libc::c_char
        } else {
            b"^/\0" as *const u8 as *const libc::c_char
        },
        0 as libc::c_int,
        Some(do_gotolinecolumn as unsafe extern "C" fn() -> ()),
        0 as libc::c_int,
    );
    add_to_sclist(
        (1 as libc::c_int) << 0 as libc::c_int,
        b"M-G\0" as *const u8 as *const libc::c_char,
        0 as libc::c_int,
        Some(do_gotolinecolumn as unsafe extern "C" fn() -> ()),
        0 as libc::c_int,
    );
    add_to_sclist(
        (1 as libc::c_int) << 0 as libc::c_int,
        b"^_\0" as *const u8 as *const libc::c_char,
        0 as libc::c_int,
        Some(do_gotolinecolumn as unsafe extern "C" fn() -> ()),
        0 as libc::c_int,
    );
    add_to_sclist(
        (1 as libc::c_int) << 0 as libc::c_int | (1 as libc::c_int) << 10 as libc::c_int
            | (1 as libc::c_int) << 8 as libc::c_int
            | (1 as libc::c_int) << 14 as libc::c_int,
        b"^Y\0" as *const u8 as *const libc::c_char,
        0 as libc::c_int,
        Some(do_page_up as unsafe extern "C" fn() -> ()),
        0 as libc::c_int,
    );
    add_to_sclist(
        (1 as libc::c_int) << 0 as libc::c_int | (1 as libc::c_int) << 10 as libc::c_int
            | (1 as libc::c_int) << 8 as libc::c_int
            | (1 as libc::c_int) << 14 as libc::c_int,
        b"PgUp\0" as *const u8 as *const libc::c_char,
        0o523 as libc::c_int,
        Some(do_page_up as unsafe extern "C" fn() -> ()),
        0 as libc::c_int,
    );
    add_to_sclist(
        (1 as libc::c_int) << 0 as libc::c_int | (1 as libc::c_int) << 10 as libc::c_int
            | (1 as libc::c_int) << 8 as libc::c_int
            | (1 as libc::c_int) << 14 as libc::c_int,
        b"^V\0" as *const u8 as *const libc::c_char,
        0 as libc::c_int,
        Some(do_page_down as unsafe extern "C" fn() -> ()),
        0 as libc::c_int,
    );
    add_to_sclist(
        (1 as libc::c_int) << 0 as libc::c_int | (1 as libc::c_int) << 10 as libc::c_int
            | (1 as libc::c_int) << 8 as libc::c_int
            | (1 as libc::c_int) << 14 as libc::c_int,
        b"PgDn\0" as *const u8 as *const libc::c_char,
        0o522 as libc::c_int,
        Some(do_page_down as unsafe extern "C" fn() -> ()),
        0 as libc::c_int,
    );
    add_to_sclist(
        (1 as libc::c_int) << 10 as libc::c_int | (1 as libc::c_int) << 8 as libc::c_int,
        b"Bsp\0" as *const u8 as *const libc::c_char,
        0o407 as libc::c_int,
        Some(do_page_up as unsafe extern "C" fn() -> ()),
        0 as libc::c_int,
    );
    add_to_sclist(
        (1 as libc::c_int) << 10 as libc::c_int | (1 as libc::c_int) << 8 as libc::c_int,
        b"Sh-Del\0" as *const u8 as *const libc::c_char,
        0x45d as libc::c_int,
        Some(do_page_up as unsafe extern "C" fn() -> ()),
        0 as libc::c_int,
    );
    add_to_sclist(
        (1 as libc::c_int) << 10 as libc::c_int | (1 as libc::c_int) << 8 as libc::c_int,
        b"Space\0" as *const u8 as *const libc::c_char,
        0x20 as libc::c_int,
        Some(do_page_down as unsafe extern "C" fn() -> ()),
        0 as libc::c_int,
    );
    add_to_sclist(
        (1 as libc::c_int) << 0 as libc::c_int | (1 as libc::c_int) << 8 as libc::c_int,
        b"M-\\\0" as *const u8 as *const libc::c_char,
        0 as libc::c_int,
        Some(to_first_line as unsafe extern "C" fn() -> ()),
        0 as libc::c_int,
    );
    add_to_sclist(
        (1 as libc::c_int) << 0 as libc::c_int | (1 as libc::c_int) << 8 as libc::c_int,
        b"^Home\0" as *const u8 as *const libc::c_char,
        0x405 as libc::c_int,
        Some(to_first_line as unsafe extern "C" fn() -> ()),
        0 as libc::c_int,
    );
    add_to_sclist(
        (1 as libc::c_int) << 0 as libc::c_int | (1 as libc::c_int) << 8 as libc::c_int,
        b"M-/\0" as *const u8 as *const libc::c_char,
        0 as libc::c_int,
        Some(to_last_line as unsafe extern "C" fn() -> ()),
        0 as libc::c_int,
    );
    add_to_sclist(
        (1 as libc::c_int) << 0 as libc::c_int | (1 as libc::c_int) << 8 as libc::c_int,
        b"^End\0" as *const u8 as *const libc::c_char,
        0x406 as libc::c_int,
        Some(to_last_line as unsafe extern "C" fn() -> ()),
        0 as libc::c_int,
    );
    add_to_sclist(
        (1 as libc::c_int) << 0 as libc::c_int | (1 as libc::c_int) << 10 as libc::c_int
            | (1 as libc::c_int) << 8 as libc::c_int,
        b"M-W\0" as *const u8 as *const libc::c_char,
        0 as libc::c_int,
        Some(do_findnext as unsafe extern "C" fn() -> ()),
        0 as libc::c_int,
    );
    add_to_sclist(
        (1 as libc::c_int) << 0 as libc::c_int | (1 as libc::c_int) << 10 as libc::c_int
            | (1 as libc::c_int) << 8 as libc::c_int,
        b"M-Q\0" as *const u8 as *const libc::c_char,
        0 as libc::c_int,
        Some(do_findprevious as unsafe extern "C" fn() -> ()),
        0 as libc::c_int,
    );
    add_to_sclist(
        (1 as libc::c_int) << 0 as libc::c_int,
        b"M-]\0" as *const u8 as *const libc::c_char,
        0 as libc::c_int,
        Some(do_find_bracket as unsafe extern "C" fn() -> ()),
        0 as libc::c_int,
    );
    add_to_sclist(
        (1 as libc::c_int) << 0 as libc::c_int,
        b"M-A\0" as *const u8 as *const libc::c_char,
        0 as libc::c_int,
        Some(do_mark as unsafe extern "C" fn() -> ()),
        0 as libc::c_int,
    );
    add_to_sclist(
        (1 as libc::c_int) << 0 as libc::c_int,
        b"^6\0" as *const u8 as *const libc::c_char,
        0 as libc::c_int,
        Some(do_mark as unsafe extern "C" fn() -> ()),
        0 as libc::c_int,
    );
    add_to_sclist(
        (1 as libc::c_int) << 0 as libc::c_int,
        b"^^\0" as *const u8 as *const libc::c_char,
        0 as libc::c_int,
        Some(do_mark as unsafe extern "C" fn() -> ()),
        0 as libc::c_int,
    );
    add_to_sclist(
        (1 as libc::c_int) << 0 as libc::c_int | (1 as libc::c_int) << 1 as libc::c_int
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
            | (1 as libc::c_int) << 14 as libc::c_int,
        b"M-6\0" as *const u8 as *const libc::c_char,
        0 as libc::c_int,
        Some(copy_text as unsafe extern "C" fn() -> ()),
        0 as libc::c_int,
    );
    add_to_sclist(
        (1 as libc::c_int) << 0 as libc::c_int | (1 as libc::c_int) << 1 as libc::c_int
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
            | (1 as libc::c_int) << 14 as libc::c_int,
        b"M-^\0" as *const u8 as *const libc::c_char,
        0 as libc::c_int,
        Some(copy_text as unsafe extern "C" fn() -> ()),
        0 as libc::c_int,
    );
    add_to_sclist(
        (1 as libc::c_int) << 0 as libc::c_int,
        b"M-}\0" as *const u8 as *const libc::c_char,
        0 as libc::c_int,
        Some(do_indent as unsafe extern "C" fn() -> ()),
        0 as libc::c_int,
    );
    add_to_sclist(
        (1 as libc::c_int) << 0 as libc::c_int,
        b"\0" as *const u8 as *const libc::c_char,
        0x4f1 as libc::c_int,
        Some(do_indent as unsafe extern "C" fn() -> ()),
        0 as libc::c_int,
    );
    add_to_sclist(
        (1 as libc::c_int) << 0 as libc::c_int,
        b"M-{\0" as *const u8 as *const libc::c_char,
        0 as libc::c_int,
        Some(do_unindent as unsafe extern "C" fn() -> ()),
        0 as libc::c_int,
    );
    add_to_sclist(
        (1 as libc::c_int) << 0 as libc::c_int,
        b"Sh-Tab\0" as *const u8 as *const libc::c_char,
        0x45f as libc::c_int,
        Some(do_unindent as unsafe extern "C" fn() -> ()),
        0 as libc::c_int,
    );
    add_to_sclist(
        (1 as libc::c_int) << 0 as libc::c_int,
        b"M-:\0" as *const u8 as *const libc::c_char,
        0 as libc::c_int,
        Some(record_macro as unsafe extern "C" fn() -> ()),
        0 as libc::c_int,
    );
    add_to_sclist(
        (1 as libc::c_int) << 0 as libc::c_int,
        b"M-;\0" as *const u8 as *const libc::c_char,
        0 as libc::c_int,
        Some(run_macro as unsafe extern "C" fn() -> ()),
        0 as libc::c_int,
    );
    add_to_sclist(
        (1 as libc::c_int) << 0 as libc::c_int,
        b"M-U\0" as *const u8 as *const libc::c_char,
        0 as libc::c_int,
        Some(do_undo as unsafe extern "C" fn() -> ()),
        0 as libc::c_int,
    );
    add_to_sclist(
        (1 as libc::c_int) << 0 as libc::c_int,
        b"M-E\0" as *const u8 as *const libc::c_char,
        0 as libc::c_int,
        Some(do_redo as unsafe extern "C" fn() -> ()),
        0 as libc::c_int,
    );
    add_to_sclist(
        (1 as libc::c_int) << 0 as libc::c_int,
        b"M-Bsp\0" as *const u8 as *const libc::c_char,
        0x41d as libc::c_int,
        Some(chop_previous_word as unsafe extern "C" fn() -> ()),
        0 as libc::c_int,
    );
    add_to_sclist(
        (1 as libc::c_int) << 0 as libc::c_int,
        b"Sh-^Del\0" as *const u8 as *const libc::c_char,
        0x41d as libc::c_int,
        Some(chop_previous_word as unsafe extern "C" fn() -> ()),
        0 as libc::c_int,
    );
    add_to_sclist(
        (1 as libc::c_int) << 0 as libc::c_int,
        b"^Del\0" as *const u8 as *const libc::c_char,
        0x40d as libc::c_int,
        Some(chop_next_word as unsafe extern "C" fn() -> ()),
        0 as libc::c_int,
    );
    add_to_sclist(
        (1 as libc::c_int) << 0 as libc::c_int,
        b"M-Del\0" as *const u8 as *const libc::c_char,
        0x42d as libc::c_int,
        Some(zap_text as unsafe extern "C" fn() -> ()),
        0 as libc::c_int,
    );
    add_to_sclist(
        (1 as libc::c_int) << 0 as libc::c_int,
        b"M-Ins\0" as *const u8 as *const libc::c_char,
        0x42c as libc::c_int,
        Some(put_or_lift_anchor as unsafe extern "C" fn() -> ()),
        0 as libc::c_int,
    );
    add_to_sclist(
        (1 as libc::c_int) << 0 as libc::c_int,
        b"M-PgUp\0" as *const u8 as *const libc::c_char,
        0x427 as libc::c_int,
        Some(to_prev_anchor as unsafe extern "C" fn() -> ()),
        0 as libc::c_int,
    );
    add_to_sclist(
        (1 as libc::c_int) << 0 as libc::c_int,
        b"M-PgDn\0" as *const u8 as *const libc::c_char,
        0x428 as libc::c_int,
        Some(to_next_anchor as unsafe extern "C" fn() -> ()),
        0 as libc::c_int,
    );
    add_to_sclist(
        (1 as libc::c_int) << 0 as libc::c_int,
        b"^]\0" as *const u8 as *const libc::c_char,
        0 as libc::c_int,
        Some(complete_a_word as unsafe extern "C" fn() -> ()),
        0 as libc::c_int,
    );
    add_to_sclist(
        (1 as libc::c_int) << 0 as libc::c_int,
        b"M-3\0" as *const u8 as *const libc::c_char,
        0 as libc::c_int,
        Some(do_comment as unsafe extern "C" fn() -> ()),
        0 as libc::c_int,
    );
    add_to_sclist(
        (1 as libc::c_int) << 0 as libc::c_int | (1 as libc::c_int) << 1 as libc::c_int
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
            | (1 as libc::c_int) << 10 as libc::c_int,
        b"^B\0" as *const u8 as *const libc::c_char,
        0 as libc::c_int,
        Some(do_left as unsafe extern "C" fn() -> ()),
        0 as libc::c_int,
    );
    add_to_sclist(
        (1 as libc::c_int) << 0 as libc::c_int | (1 as libc::c_int) << 1 as libc::c_int
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
            | (1 as libc::c_int) << 10 as libc::c_int,
        b"^F\0" as *const u8 as *const libc::c_char,
        0 as libc::c_int,
        Some(do_right as unsafe extern "C" fn() -> ()),
        0 as libc::c_int,
    );
    if using_utf8() {
        add_to_sclist(
            (1 as libc::c_int) << 0 as libc::c_int
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
                | (1 as libc::c_int) << 8 as libc::c_int,
            b"\xE2\x97\x82\0" as *const u8 as *const libc::c_char,
            0o404 as libc::c_int,
            Some(do_left as unsafe extern "C" fn() -> ()),
            0 as libc::c_int,
        );
        add_to_sclist(
            (1 as libc::c_int) << 0 as libc::c_int
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
                | (1 as libc::c_int) << 8 as libc::c_int,
            b"\xE2\x96\xB8\0" as *const u8 as *const libc::c_char,
            0o405 as libc::c_int,
            Some(do_right as unsafe extern "C" fn() -> ()),
            0 as libc::c_int,
        );
        add_to_sclist(
            (1 as libc::c_int) << 0 as libc::c_int
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
                | (1 as libc::c_int) << 10 as libc::c_int,
            b"^\xE2\x97\x82\0" as *const u8 as *const libc::c_char,
            0x401 as libc::c_int,
            Some(to_prev_word as unsafe extern "C" fn() -> ()),
            0 as libc::c_int,
        );
        add_to_sclist(
            (1 as libc::c_int) << 0 as libc::c_int
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
                | (1 as libc::c_int) << 10 as libc::c_int,
            b"^\xE2\x96\xB8\0" as *const u8 as *const libc::c_char,
            0x402 as libc::c_int,
            Some(to_next_word as unsafe extern "C" fn() -> ()),
            0 as libc::c_int,
        );
        if !on_a_vt {
            add_to_sclist(
                (1 as libc::c_int) << 0 as libc::c_int,
                b"M-\xE2\x97\x82\0" as *const u8 as *const libc::c_char,
                0x421 as libc::c_int,
                Some(switch_to_prev_buffer as unsafe extern "C" fn() -> ()),
                0 as libc::c_int,
            );
            add_to_sclist(
                (1 as libc::c_int) << 0 as libc::c_int,
                b"M-\xE2\x96\xB8\0" as *const u8 as *const libc::c_char,
                0x422 as libc::c_int,
                Some(switch_to_next_buffer as unsafe extern "C" fn() -> ()),
                0 as libc::c_int,
            );
        }
    } else {
        add_to_sclist(
            (1 as libc::c_int) << 0 as libc::c_int
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
                | (1 as libc::c_int) << 8 as libc::c_int,
            b"Left\0" as *const u8 as *const libc::c_char,
            0o404 as libc::c_int,
            Some(do_left as unsafe extern "C" fn() -> ()),
            0 as libc::c_int,
        );
        add_to_sclist(
            (1 as libc::c_int) << 0 as libc::c_int
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
                | (1 as libc::c_int) << 8 as libc::c_int,
            b"Right\0" as *const u8 as *const libc::c_char,
            0o405 as libc::c_int,
            Some(do_right as unsafe extern "C" fn() -> ()),
            0 as libc::c_int,
        );
        add_to_sclist(
            (1 as libc::c_int) << 0 as libc::c_int
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
                | (1 as libc::c_int) << 10 as libc::c_int,
            b"^Left\0" as *const u8 as *const libc::c_char,
            0x401 as libc::c_int,
            Some(to_prev_word as unsafe extern "C" fn() -> ()),
            0 as libc::c_int,
        );
        add_to_sclist(
            (1 as libc::c_int) << 0 as libc::c_int
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
                | (1 as libc::c_int) << 10 as libc::c_int,
            b"^Right\0" as *const u8 as *const libc::c_char,
            0x402 as libc::c_int,
            Some(to_next_word as unsafe extern "C" fn() -> ()),
            0 as libc::c_int,
        );
        if !on_a_vt {
            add_to_sclist(
                (1 as libc::c_int) << 0 as libc::c_int,
                b"M-Left\0" as *const u8 as *const libc::c_char,
                0x421 as libc::c_int,
                Some(switch_to_prev_buffer as unsafe extern "C" fn() -> ()),
                0 as libc::c_int,
            );
            add_to_sclist(
                (1 as libc::c_int) << 0 as libc::c_int,
                b"M-Right\0" as *const u8 as *const libc::c_char,
                0x422 as libc::c_int,
                Some(switch_to_next_buffer as unsafe extern "C" fn() -> ()),
                0 as libc::c_int,
            );
        }
    }
    add_to_sclist(
        (1 as libc::c_int) << 0 as libc::c_int | (1 as libc::c_int) << 1 as libc::c_int
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
            | (1 as libc::c_int) << 14 as libc::c_int,
        b"M-Space\0" as *const u8 as *const libc::c_char,
        0 as libc::c_int,
        Some(to_prev_word as unsafe extern "C" fn() -> ()),
        0 as libc::c_int,
    );
    add_to_sclist(
        (1 as libc::c_int) << 0 as libc::c_int | (1 as libc::c_int) << 1 as libc::c_int
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
            | (1 as libc::c_int) << 14 as libc::c_int,
        b"^Space\0" as *const u8 as *const libc::c_char,
        0 as libc::c_int,
        Some(to_next_word as unsafe extern "C" fn() -> ()),
        0 as libc::c_int,
    );
    add_to_sclist(
        (1 as libc::c_int) << 0 as libc::c_int | (1 as libc::c_int) << 1 as libc::c_int
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
            | (1 as libc::c_int) << 14 as libc::c_int,
        b"^A\0" as *const u8 as *const libc::c_char,
        0 as libc::c_int,
        Some(do_home as unsafe extern "C" fn() -> ()),
        0 as libc::c_int,
    );
    add_to_sclist(
        (1 as libc::c_int) << 0 as libc::c_int | (1 as libc::c_int) << 1 as libc::c_int
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
            | (1 as libc::c_int) << 14 as libc::c_int,
        b"Home\0" as *const u8 as *const libc::c_char,
        0o406 as libc::c_int,
        Some(do_home as unsafe extern "C" fn() -> ()),
        0 as libc::c_int,
    );
    add_to_sclist(
        (1 as libc::c_int) << 0 as libc::c_int | (1 as libc::c_int) << 1 as libc::c_int
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
            | (1 as libc::c_int) << 14 as libc::c_int,
        b"^E\0" as *const u8 as *const libc::c_char,
        0 as libc::c_int,
        Some(do_end as unsafe extern "C" fn() -> ()),
        0 as libc::c_int,
    );
    add_to_sclist(
        (1 as libc::c_int) << 0 as libc::c_int | (1 as libc::c_int) << 1 as libc::c_int
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
            | (1 as libc::c_int) << 14 as libc::c_int,
        b"End\0" as *const u8 as *const libc::c_char,
        0o550 as libc::c_int,
        Some(do_end as unsafe extern "C" fn() -> ()),
        0 as libc::c_int,
    );
    add_to_sclist(
        (1 as libc::c_int) << 0 as libc::c_int | (1 as libc::c_int) << 10 as libc::c_int
            | (1 as libc::c_int) << 8 as libc::c_int,
        b"^P\0" as *const u8 as *const libc::c_char,
        0 as libc::c_int,
        Some(do_up as unsafe extern "C" fn() -> ()),
        0 as libc::c_int,
    );
    add_to_sclist(
        (1 as libc::c_int) << 0 as libc::c_int | (1 as libc::c_int) << 10 as libc::c_int
            | (1 as libc::c_int) << 8 as libc::c_int,
        b"^N\0" as *const u8 as *const libc::c_char,
        0 as libc::c_int,
        Some(do_down as unsafe extern "C" fn() -> ()),
        0 as libc::c_int,
    );
    if using_utf8() {
        add_to_sclist(
            (1 as libc::c_int) << 0 as libc::c_int
                | (1 as libc::c_int) << 10 as libc::c_int
                | (1 as libc::c_int) << 8 as libc::c_int,
            b"\xE2\x96\xB4\0" as *const u8 as *const libc::c_char,
            0o403 as libc::c_int,
            Some(do_up as unsafe extern "C" fn() -> ()),
            0 as libc::c_int,
        );
        add_to_sclist(
            (1 as libc::c_int) << 0 as libc::c_int
                | (1 as libc::c_int) << 10 as libc::c_int
                | (1 as libc::c_int) << 8 as libc::c_int,
            b"\xE2\x96\xBE\0" as *const u8 as *const libc::c_char,
            0o402 as libc::c_int,
            Some(do_down as unsafe extern "C" fn() -> ()),
            0 as libc::c_int,
        );
        add_to_sclist(
            (1 as libc::c_int) << 0 as libc::c_int
                | (1 as libc::c_int) << 10 as libc::c_int
                | (1 as libc::c_int) << 14 as libc::c_int,
            b"^\xE2\x96\xB4\0" as *const u8 as *const libc::c_char,
            0x403 as libc::c_int,
            Some(to_prev_block as unsafe extern "C" fn() -> ()),
            0 as libc::c_int,
        );
        add_to_sclist(
            (1 as libc::c_int) << 0 as libc::c_int
                | (1 as libc::c_int) << 10 as libc::c_int
                | (1 as libc::c_int) << 14 as libc::c_int,
            b"^\xE2\x96\xBE\0" as *const u8 as *const libc::c_char,
            0x404 as libc::c_int,
            Some(to_next_block as unsafe extern "C" fn() -> ()),
            0 as libc::c_int,
        );
    } else {
        add_to_sclist(
            (1 as libc::c_int) << 0 as libc::c_int
                | (1 as libc::c_int) << 10 as libc::c_int
                | (1 as libc::c_int) << 8 as libc::c_int,
            b"Up\0" as *const u8 as *const libc::c_char,
            0o403 as libc::c_int,
            Some(do_up as unsafe extern "C" fn() -> ()),
            0 as libc::c_int,
        );
        add_to_sclist(
            (1 as libc::c_int) << 0 as libc::c_int
                | (1 as libc::c_int) << 10 as libc::c_int
                | (1 as libc::c_int) << 8 as libc::c_int,
            b"Down\0" as *const u8 as *const libc::c_char,
            0o402 as libc::c_int,
            Some(do_down as unsafe extern "C" fn() -> ()),
            0 as libc::c_int,
        );
        add_to_sclist(
            (1 as libc::c_int) << 0 as libc::c_int
                | (1 as libc::c_int) << 10 as libc::c_int
                | (1 as libc::c_int) << 14 as libc::c_int,
            b"^Up\0" as *const u8 as *const libc::c_char,
            0x403 as libc::c_int,
            Some(to_prev_block as unsafe extern "C" fn() -> ()),
            0 as libc::c_int,
        );
        add_to_sclist(
            (1 as libc::c_int) << 0 as libc::c_int
                | (1 as libc::c_int) << 10 as libc::c_int
                | (1 as libc::c_int) << 14 as libc::c_int,
            b"^Down\0" as *const u8 as *const libc::c_char,
            0x404 as libc::c_int,
            Some(to_next_block as unsafe extern "C" fn() -> ()),
            0 as libc::c_int,
        );
    }
    add_to_sclist(
        (1 as libc::c_int) << 0 as libc::c_int,
        b"M-7\0" as *const u8 as *const libc::c_char,
        0 as libc::c_int,
        Some(to_prev_block as unsafe extern "C" fn() -> ()),
        0 as libc::c_int,
    );
    add_to_sclist(
        (1 as libc::c_int) << 0 as libc::c_int,
        b"M-8\0" as *const u8 as *const libc::c_char,
        0 as libc::c_int,
        Some(to_next_block as unsafe extern "C" fn() -> ()),
        0 as libc::c_int,
    );
    add_to_sclist(
        (1 as libc::c_int) << 0 as libc::c_int,
        b"M-(\0" as *const u8 as *const libc::c_char,
        0 as libc::c_int,
        Some(to_para_begin as unsafe extern "C" fn() -> ()),
        0 as libc::c_int,
    );
    add_to_sclist(
        (1 as libc::c_int) << 0 as libc::c_int,
        b"M-9\0" as *const u8 as *const libc::c_char,
        0 as libc::c_int,
        Some(to_para_begin as unsafe extern "C" fn() -> ()),
        0 as libc::c_int,
    );
    add_to_sclist(
        (1 as libc::c_int) << 0 as libc::c_int,
        b"M-)\0" as *const u8 as *const libc::c_char,
        0 as libc::c_int,
        Some(to_para_end as unsafe extern "C" fn() -> ()),
        0 as libc::c_int,
    );
    add_to_sclist(
        (1 as libc::c_int) << 0 as libc::c_int,
        b"M-0\0" as *const u8 as *const libc::c_char,
        0 as libc::c_int,
        Some(to_para_end as unsafe extern "C" fn() -> ()),
        0 as libc::c_int,
    );
    if using_utf8() {
        add_to_sclist(
            (1 as libc::c_int) << 0 as libc::c_int
                | (1 as libc::c_int) << 8 as libc::c_int,
            b"M-\xE2\x96\xB4\0" as *const u8 as *const libc::c_char,
            0x423 as libc::c_int,
            Some(do_scroll_up as unsafe extern "C" fn() -> ()),
            0 as libc::c_int,
        );
        add_to_sclist(
            (1 as libc::c_int) << 0 as libc::c_int
                | (1 as libc::c_int) << 8 as libc::c_int,
            b"M-\xE2\x96\xBE\0" as *const u8 as *const libc::c_char,
            0x424 as libc::c_int,
            Some(do_scroll_down as unsafe extern "C" fn() -> ()),
            0 as libc::c_int,
        );
    } else {
        add_to_sclist(
            (1 as libc::c_int) << 0 as libc::c_int
                | (1 as libc::c_int) << 8 as libc::c_int,
            b"M-Up\0" as *const u8 as *const libc::c_char,
            0x423 as libc::c_int,
            Some(do_scroll_up as unsafe extern "C" fn() -> ()),
            0 as libc::c_int,
        );
        add_to_sclist(
            (1 as libc::c_int) << 0 as libc::c_int
                | (1 as libc::c_int) << 8 as libc::c_int,
            b"M-Down\0" as *const u8 as *const libc::c_char,
            0x424 as libc::c_int,
            Some(do_scroll_down as unsafe extern "C" fn() -> ()),
            0 as libc::c_int,
        );
    }
    add_to_sclist(
        (1 as libc::c_int) << 0 as libc::c_int | (1 as libc::c_int) << 8 as libc::c_int,
        b"M--\0" as *const u8 as *const libc::c_char,
        0 as libc::c_int,
        Some(do_scroll_up as unsafe extern "C" fn() -> ()),
        0 as libc::c_int,
    );
    add_to_sclist(
        (1 as libc::c_int) << 0 as libc::c_int | (1 as libc::c_int) << 8 as libc::c_int,
        b"M-_\0" as *const u8 as *const libc::c_char,
        0 as libc::c_int,
        Some(do_scroll_up as unsafe extern "C" fn() -> ()),
        0 as libc::c_int,
    );
    add_to_sclist(
        (1 as libc::c_int) << 0 as libc::c_int | (1 as libc::c_int) << 8 as libc::c_int,
        b"M-+\0" as *const u8 as *const libc::c_char,
        0 as libc::c_int,
        Some(do_scroll_down as unsafe extern "C" fn() -> ()),
        0 as libc::c_int,
    );
    add_to_sclist(
        (1 as libc::c_int) << 0 as libc::c_int | (1 as libc::c_int) << 8 as libc::c_int,
        b"M-=\0" as *const u8 as *const libc::c_char,
        0 as libc::c_int,
        Some(do_scroll_down as unsafe extern "C" fn() -> ()),
        0 as libc::c_int,
    );
    add_to_sclist(
        (1 as libc::c_int) << 0 as libc::c_int,
        b"M-,\0" as *const u8 as *const libc::c_char,
        0 as libc::c_int,
        Some(switch_to_prev_buffer as unsafe extern "C" fn() -> ()),
        0 as libc::c_int,
    );
    add_to_sclist(
        (1 as libc::c_int) << 0 as libc::c_int,
        b"M-<\0" as *const u8 as *const libc::c_char,
        0 as libc::c_int,
        Some(switch_to_prev_buffer as unsafe extern "C" fn() -> ()),
        0 as libc::c_int,
    );
    add_to_sclist(
        (1 as libc::c_int) << 0 as libc::c_int,
        b"M-.\0" as *const u8 as *const libc::c_char,
        0 as libc::c_int,
        Some(switch_to_next_buffer as unsafe extern "C" fn() -> ()),
        0 as libc::c_int,
    );
    add_to_sclist(
        (1 as libc::c_int) << 0 as libc::c_int,
        b"M->\0" as *const u8 as *const libc::c_char,
        0 as libc::c_int,
        Some(switch_to_next_buffer as unsafe extern "C" fn() -> ()),
        0 as libc::c_int,
    );
    add_to_sclist(
        (1 as libc::c_int) << 0 as libc::c_int | (1 as libc::c_int) << 1 as libc::c_int
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
            | (1 as libc::c_int) << 14 as libc::c_int,
        b"M-V\0" as *const u8 as *const libc::c_char,
        0 as libc::c_int,
        Some(do_verbatim_input as unsafe extern "C" fn() -> ()),
        0 as libc::c_int,
    );
    add_to_sclist(
        (1 as libc::c_int) << 0 as libc::c_int,
        b"M-T\0" as *const u8 as *const libc::c_char,
        0 as libc::c_int,
        Some(cut_till_eof as unsafe extern "C" fn() -> ()),
        0 as libc::c_int,
    );
    add_to_sclist(
        (1 as libc::c_int) << 7 as libc::c_int,
        b"^V\0" as *const u8 as *const libc::c_char,
        0 as libc::c_int,
        Some(cut_till_eof as unsafe extern "C" fn() -> ()),
        0 as libc::c_int,
    );
    add_to_sclist(
        (1 as libc::c_int) << 7 as libc::c_int,
        b"^Z\0" as *const u8 as *const libc::c_char,
        0 as libc::c_int,
        Some(do_suspend as unsafe extern "C" fn() -> ()),
        0 as libc::c_int,
    );
    add_to_sclist(
        (1 as libc::c_int) << 0 as libc::c_int,
        b"^Z\0" as *const u8 as *const libc::c_char,
        0 as libc::c_int,
        Some(suggest_ctrlT_ctrlZ as unsafe extern "C" fn() -> ()),
        0 as libc::c_int,
    );
    add_to_sclist(
        (1 as libc::c_int) << 0 as libc::c_int,
        b"M-D\0" as *const u8 as *const libc::c_char,
        0 as libc::c_int,
        Some(count_lines_words_and_characters as unsafe extern "C" fn() -> ()),
        0 as libc::c_int,
    );
    add_to_sclist(
        (1 as libc::c_int) << 0 as libc::c_int,
        b"M-J\0" as *const u8 as *const libc::c_char,
        0 as libc::c_int,
        Some(do_full_justify as unsafe extern "C" fn() -> ()),
        0 as libc::c_int,
    );
    add_to_sclist(
        (1 as libc::c_int) << 7 as libc::c_int,
        b"^J\0" as *const u8 as *const libc::c_char,
        0 as libc::c_int,
        Some(do_full_justify as unsafe extern "C" fn() -> ()),
        0 as libc::c_int,
    );
    add_to_sclist(
        (1 as libc::c_int) << 0 as libc::c_int,
        b"^L\0" as *const u8 as *const libc::c_char,
        0 as libc::c_int,
        Some(do_center as unsafe extern "C" fn() -> ()),
        0 as libc::c_int,
    );
    if !(flags[(PRESERVE as libc::c_int as libc::c_ulong)
        .wrapping_div(
            (::std::mem::size_of::<libc::c_uint>() as libc::c_ulong)
                .wrapping_mul(8 as libc::c_int as libc::c_ulong),
        ) as usize]
        & (1 as libc::c_int as libc::c_uint)
            << (PRESERVE as libc::c_int as libc::c_ulong)
                .wrapping_rem(
                    (::std::mem::size_of::<libc::c_uint>() as libc::c_ulong)
                        .wrapping_mul(8 as libc::c_int as libc::c_ulong),
                ) != 0 as libc::c_int as libc::c_uint)
    {
        add_to_sclist(
            (1 as libc::c_int) << 0 as libc::c_int
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
                | (1 as libc::c_int) << 13 as libc::c_int,
            b"^L\0" as *const u8 as *const libc::c_char,
            0 as libc::c_int,
            Some(full_refresh as unsafe extern "C" fn() -> ()),
            0 as libc::c_int,
        );
    } else {
        add_to_sclist(
            (1 as libc::c_int) << 0 as libc::c_int
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
                | (1 as libc::c_int) << 13 as libc::c_int,
            b"^L\0" as *const u8 as *const libc::c_char,
            0 as libc::c_int,
            Some(full_refresh as unsafe extern "C" fn() -> ()),
            0 as libc::c_int,
        );
    }
    add_to_sclist(
        (1 as libc::c_int) << 0 as libc::c_int,
        b"M-Z\0" as *const u8 as *const libc::c_char,
        0 as libc::c_int,
        Some(do_toggle as unsafe extern "C" fn() -> ()),
        ZERO as libc::c_int,
    );
    add_to_sclist(
        ((1 as libc::c_int) << 0 as libc::c_int | (1 as libc::c_int) << 1 as libc::c_int
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
            & !((1 as libc::c_int) << 15 as libc::c_int),
        b"M-X\0" as *const u8 as *const libc::c_char,
        0 as libc::c_int,
        Some(do_toggle as unsafe extern "C" fn() -> ()),
        NO_HELP as libc::c_int,
    );
    add_to_sclist(
        (1 as libc::c_int) << 0 as libc::c_int,
        b"M-C\0" as *const u8 as *const libc::c_char,
        0 as libc::c_int,
        Some(do_toggle as unsafe extern "C" fn() -> ()),
        CONSTANT_SHOW as libc::c_int,
    );
    add_to_sclist(
        (1 as libc::c_int) << 0 as libc::c_int,
        b"M-S\0" as *const u8 as *const libc::c_char,
        0 as libc::c_int,
        Some(do_toggle as unsafe extern "C" fn() -> ()),
        SOFTWRAP as libc::c_int,
    );
    add_to_sclist(
        (1 as libc::c_int) << 0 as libc::c_int,
        b"M-$\0" as *const u8 as *const libc::c_char,
        0 as libc::c_int,
        Some(do_toggle as unsafe extern "C" fn() -> ()),
        SOFTWRAP as libc::c_int,
    );
    add_to_sclist(
        (1 as libc::c_int) << 0 as libc::c_int,
        b"M-N\0" as *const u8 as *const libc::c_char,
        0 as libc::c_int,
        Some(do_toggle as unsafe extern "C" fn() -> ()),
        LINE_NUMBERS as libc::c_int,
    );
    add_to_sclist(
        (1 as libc::c_int) << 0 as libc::c_int,
        b"M-#\0" as *const u8 as *const libc::c_char,
        0 as libc::c_int,
        Some(do_toggle as unsafe extern "C" fn() -> ()),
        LINE_NUMBERS as libc::c_int,
    );
    add_to_sclist(
        (1 as libc::c_int) << 0 as libc::c_int,
        b"M-P\0" as *const u8 as *const libc::c_char,
        0 as libc::c_int,
        Some(do_toggle as unsafe extern "C" fn() -> ()),
        WHITESPACE_DISPLAY as libc::c_int,
    );
    add_to_sclist(
        (1 as libc::c_int) << 0 as libc::c_int,
        b"M-Y\0" as *const u8 as *const libc::c_char,
        0 as libc::c_int,
        Some(do_toggle as unsafe extern "C" fn() -> ()),
        NO_SYNTAX as libc::c_int,
    );
    add_to_sclist(
        (1 as libc::c_int) << 0 as libc::c_int,
        b"M-H\0" as *const u8 as *const libc::c_char,
        0 as libc::c_int,
        Some(do_toggle as unsafe extern "C" fn() -> ()),
        SMART_HOME as libc::c_int,
    );
    add_to_sclist(
        (1 as libc::c_int) << 0 as libc::c_int,
        b"M-I\0" as *const u8 as *const libc::c_char,
        0 as libc::c_int,
        Some(do_toggle as unsafe extern "C" fn() -> ()),
        AUTOINDENT as libc::c_int,
    );
    add_to_sclist(
        (1 as libc::c_int) << 0 as libc::c_int,
        b"M-K\0" as *const u8 as *const libc::c_char,
        0 as libc::c_int,
        Some(do_toggle as unsafe extern "C" fn() -> ()),
        CUT_FROM_CURSOR as libc::c_int,
    );
    add_to_sclist(
        (1 as libc::c_int) << 0 as libc::c_int,
        b"M-L\0" as *const u8 as *const libc::c_char,
        0 as libc::c_int,
        Some(do_toggle as unsafe extern "C" fn() -> ()),
        BREAK_LONG_LINES as libc::c_int,
    );
    add_to_sclist(
        (1 as libc::c_int) << 0 as libc::c_int,
        b"M-O\0" as *const u8 as *const libc::c_char,
        0 as libc::c_int,
        Some(do_toggle as unsafe extern "C" fn() -> ()),
        TABS_TO_SPACES as libc::c_int,
    );
    add_to_sclist(
        (1 as libc::c_int) << 0 as libc::c_int,
        b"M-M\0" as *const u8 as *const libc::c_char,
        0 as libc::c_int,
        Some(do_toggle as unsafe extern "C" fn() -> ()),
        USE_MOUSE as libc::c_int,
    );
    add_to_sclist(
        ((1 as libc::c_int) << 0 as libc::c_int | (1 as libc::c_int) << 1 as libc::c_int
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
            | (1 as libc::c_int) << 14 as libc::c_int)
            & !((1 as libc::c_int) << 0 as libc::c_int)
            | (1 as libc::c_int) << 13 as libc::c_int,
        b"^C\0" as *const u8 as *const libc::c_char,
        0 as libc::c_int,
        Some(do_cancel as unsafe extern "C" fn() -> ()),
        0 as libc::c_int,
    );
    add_to_sclist(
        (1 as libc::c_int) << 1 as libc::c_int | (1 as libc::c_int) << 2 as libc::c_int,
        b"M-C\0" as *const u8 as *const libc::c_char,
        0 as libc::c_int,
        Some(case_sens_void as unsafe extern "C" fn() -> ()),
        0 as libc::c_int,
    );
    add_to_sclist(
        (1 as libc::c_int) << 1 as libc::c_int | (1 as libc::c_int) << 2 as libc::c_int,
        b"M-R\0" as *const u8 as *const libc::c_char,
        0 as libc::c_int,
        Some(regexp_void as unsafe extern "C" fn() -> ()),
        0 as libc::c_int,
    );
    add_to_sclist(
        (1 as libc::c_int) << 1 as libc::c_int | (1 as libc::c_int) << 2 as libc::c_int,
        b"M-B\0" as *const u8 as *const libc::c_char,
        0 as libc::c_int,
        Some(backwards_void as unsafe extern "C" fn() -> ()),
        0 as libc::c_int,
    );
    add_to_sclist(
        (1 as libc::c_int) << 1 as libc::c_int | (1 as libc::c_int) << 2 as libc::c_int,
        b"^R\0" as *const u8 as *const libc::c_char,
        0 as libc::c_int,
        Some(flip_replace as unsafe extern "C" fn() -> ()),
        0 as libc::c_int,
    );
    add_to_sclist(
        (1 as libc::c_int) << 1 as libc::c_int | (1 as libc::c_int) << 4 as libc::c_int,
        b"^T\0" as *const u8 as *const libc::c_char,
        0 as libc::c_int,
        Some(flip_goto as unsafe extern "C" fn() -> ()),
        0 as libc::c_int,
    );
    add_to_sclist(
        (1 as libc::c_int) << 1 as libc::c_int | (1 as libc::c_int) << 4 as libc::c_int,
        b"^/\0" as *const u8 as *const libc::c_char,
        0 as libc::c_int,
        Some(flip_goto as unsafe extern "C" fn() -> ()),
        0 as libc::c_int,
    );
    add_to_sclist(
        (1 as libc::c_int) << 1 as libc::c_int | (1 as libc::c_int) << 2 as libc::c_int
            | (1 as libc::c_int) << 3 as libc::c_int
            | (1 as libc::c_int) << 11 as libc::c_int
            | (1 as libc::c_int) << 15 as libc::c_int
            | (1 as libc::c_int) << 7 as libc::c_int,
        b"^P\0" as *const u8 as *const libc::c_char,
        0 as libc::c_int,
        Some(get_older_item as unsafe extern "C" fn() -> ()),
        0 as libc::c_int,
    );
    add_to_sclist(
        (1 as libc::c_int) << 1 as libc::c_int | (1 as libc::c_int) << 2 as libc::c_int
            | (1 as libc::c_int) << 3 as libc::c_int
            | (1 as libc::c_int) << 11 as libc::c_int
            | (1 as libc::c_int) << 15 as libc::c_int
            | (1 as libc::c_int) << 7 as libc::c_int,
        b"^N\0" as *const u8 as *const libc::c_char,
        0 as libc::c_int,
        Some(get_newer_item as unsafe extern "C" fn() -> ()),
        0 as libc::c_int,
    );
    if using_utf8() {
        add_to_sclist(
            (1 as libc::c_int) << 1 as libc::c_int
                | (1 as libc::c_int) << 2 as libc::c_int
                | (1 as libc::c_int) << 3 as libc::c_int
                | (1 as libc::c_int) << 11 as libc::c_int
                | (1 as libc::c_int) << 15 as libc::c_int
                | (1 as libc::c_int) << 7 as libc::c_int,
            b"\xE2\x96\xB4\0" as *const u8 as *const libc::c_char,
            0o403 as libc::c_int,
            Some(get_older_item as unsafe extern "C" fn() -> ()),
            0 as libc::c_int,
        );
        add_to_sclist(
            (1 as libc::c_int) << 1 as libc::c_int
                | (1 as libc::c_int) << 2 as libc::c_int
                | (1 as libc::c_int) << 3 as libc::c_int
                | (1 as libc::c_int) << 11 as libc::c_int
                | (1 as libc::c_int) << 15 as libc::c_int
                | (1 as libc::c_int) << 7 as libc::c_int,
            b"\xE2\x96\xBE\0" as *const u8 as *const libc::c_char,
            0o402 as libc::c_int,
            Some(get_newer_item as unsafe extern "C" fn() -> ()),
            0 as libc::c_int,
        );
    } else {
        add_to_sclist(
            (1 as libc::c_int) << 1 as libc::c_int
                | (1 as libc::c_int) << 2 as libc::c_int
                | (1 as libc::c_int) << 3 as libc::c_int
                | (1 as libc::c_int) << 11 as libc::c_int
                | (1 as libc::c_int) << 15 as libc::c_int
                | (1 as libc::c_int) << 7 as libc::c_int,
            b"Up\0" as *const u8 as *const libc::c_char,
            0o403 as libc::c_int,
            Some(get_older_item as unsafe extern "C" fn() -> ()),
            0 as libc::c_int,
        );
        add_to_sclist(
            (1 as libc::c_int) << 1 as libc::c_int
                | (1 as libc::c_int) << 2 as libc::c_int
                | (1 as libc::c_int) << 3 as libc::c_int
                | (1 as libc::c_int) << 11 as libc::c_int
                | (1 as libc::c_int) << 15 as libc::c_int
                | (1 as libc::c_int) << 7 as libc::c_int,
            b"Down\0" as *const u8 as *const libc::c_char,
            0o402 as libc::c_int,
            Some(get_newer_item as unsafe extern "C" fn() -> ()),
            0 as libc::c_int,
        );
    }
    add_to_sclist(
        (1 as libc::c_int) << 4 as libc::c_int,
        b"^W\0" as *const u8 as *const libc::c_char,
        0 as libc::c_int,
        Some(to_para_begin as unsafe extern "C" fn() -> ()),
        0 as libc::c_int,
    );
    add_to_sclist(
        (1 as libc::c_int) << 4 as libc::c_int,
        b"^O\0" as *const u8 as *const libc::c_char,
        0 as libc::c_int,
        Some(to_para_end as unsafe extern "C" fn() -> ()),
        0 as libc::c_int,
    );
    add_to_sclist(
        (1 as libc::c_int) << 4 as libc::c_int | (1 as libc::c_int) << 1 as libc::c_int
            | (1 as libc::c_int) << 15 as libc::c_int,
        b"^Y\0" as *const u8 as *const libc::c_char,
        0 as libc::c_int,
        Some(to_first_line as unsafe extern "C" fn() -> ()),
        0 as libc::c_int,
    );
    add_to_sclist(
        (1 as libc::c_int) << 4 as libc::c_int | (1 as libc::c_int) << 1 as libc::c_int
            | (1 as libc::c_int) << 15 as libc::c_int,
        b"^V\0" as *const u8 as *const libc::c_char,
        0 as libc::c_int,
        Some(to_last_line as unsafe extern "C" fn() -> ()),
        0 as libc::c_int,
    );
    add_to_sclist(
        (1 as libc::c_int) << 11 as libc::c_int,
        b"^Y\0" as *const u8 as *const libc::c_char,
        0 as libc::c_int,
        Some(to_first_file as unsafe extern "C" fn() -> ()),
        0 as libc::c_int,
    );
    add_to_sclist(
        (1 as libc::c_int) << 11 as libc::c_int,
        b"^V\0" as *const u8 as *const libc::c_char,
        0 as libc::c_int,
        Some(to_last_file as unsafe extern "C" fn() -> ()),
        0 as libc::c_int,
    );
    add_to_sclist(
        (1 as libc::c_int) << 10 as libc::c_int
            | (1 as libc::c_int) << 11 as libc::c_int,
        b"M-\\\0" as *const u8 as *const libc::c_char,
        0 as libc::c_int,
        Some(to_first_file as unsafe extern "C" fn() -> ()),
        0 as libc::c_int,
    );
    add_to_sclist(
        (1 as libc::c_int) << 10 as libc::c_int
            | (1 as libc::c_int) << 11 as libc::c_int,
        b"M-/\0" as *const u8 as *const libc::c_char,
        0 as libc::c_int,
        Some(to_last_file as unsafe extern "C" fn() -> ()),
        0 as libc::c_int,
    );
    add_to_sclist(
        (1 as libc::c_int) << 10 as libc::c_int,
        b"Home\0" as *const u8 as *const libc::c_char,
        0o406 as libc::c_int,
        Some(to_first_file as unsafe extern "C" fn() -> ()),
        0 as libc::c_int,
    );
    add_to_sclist(
        (1 as libc::c_int) << 10 as libc::c_int,
        b"End\0" as *const u8 as *const libc::c_char,
        0o550 as libc::c_int,
        Some(to_last_file as unsafe extern "C" fn() -> ()),
        0 as libc::c_int,
    );
    add_to_sclist(
        (1 as libc::c_int) << 10 as libc::c_int,
        b"^Home\0" as *const u8 as *const libc::c_char,
        0x405 as libc::c_int,
        Some(to_first_file as unsafe extern "C" fn() -> ()),
        0 as libc::c_int,
    );
    add_to_sclist(
        (1 as libc::c_int) << 10 as libc::c_int,
        b"^End\0" as *const u8 as *const libc::c_char,
        0x406 as libc::c_int,
        Some(to_last_file as unsafe extern "C" fn() -> ()),
        0 as libc::c_int,
    );
    add_to_sclist(
        (1 as libc::c_int) << 10 as libc::c_int,
        if on_a_vt as libc::c_int != 0 {
            b"^-\0" as *const u8 as *const libc::c_char
        } else {
            b"^/\0" as *const u8 as *const libc::c_char
        },
        0 as libc::c_int,
        Some(goto_dir as unsafe extern "C" fn() -> ()),
        0 as libc::c_int,
    );
    add_to_sclist(
        (1 as libc::c_int) << 10 as libc::c_int,
        b"M-G\0" as *const u8 as *const libc::c_char,
        0 as libc::c_int,
        Some(goto_dir as unsafe extern "C" fn() -> ()),
        0 as libc::c_int,
    );
    add_to_sclist(
        (1 as libc::c_int) << 10 as libc::c_int,
        b"^_\0" as *const u8 as *const libc::c_char,
        0 as libc::c_int,
        Some(goto_dir as unsafe extern "C" fn() -> ()),
        0 as libc::c_int,
    );
    if flags[(SAVE_ON_EXIT as libc::c_int as libc::c_ulong)
        .wrapping_div(
            (::std::mem::size_of::<libc::c_uint>() as libc::c_ulong)
                .wrapping_mul(8 as libc::c_int as libc::c_ulong),
        ) as usize]
        & (1 as libc::c_int as libc::c_uint)
            << (SAVE_ON_EXIT as libc::c_int as libc::c_ulong)
                .wrapping_rem(
                    (::std::mem::size_of::<libc::c_uint>() as libc::c_ulong)
                        .wrapping_mul(8 as libc::c_int as libc::c_ulong),
                ) != 0 as libc::c_int as libc::c_uint
        && !(flags[(PRESERVE as libc::c_int as libc::c_ulong)
            .wrapping_div(
                (::std::mem::size_of::<libc::c_uint>() as libc::c_ulong)
                    .wrapping_mul(8 as libc::c_int as libc::c_ulong),
            ) as usize]
            & (1 as libc::c_int as libc::c_uint)
                << (PRESERVE as libc::c_int as libc::c_ulong)
                    .wrapping_rem(
                        (::std::mem::size_of::<libc::c_uint>() as libc::c_ulong)
                            .wrapping_mul(8 as libc::c_int as libc::c_ulong),
                    ) != 0 as libc::c_int as libc::c_uint)
    {
        add_to_sclist(
            (1 as libc::c_int) << 5 as libc::c_int,
            b"^Q\0" as *const u8 as *const libc::c_char,
            0 as libc::c_int,
            Some(discard_buffer as unsafe extern "C" fn() -> ()),
            0 as libc::c_int,
        );
    }
    add_to_sclist(
        (1 as libc::c_int) << 5 as libc::c_int,
        b"M-D\0" as *const u8 as *const libc::c_char,
        0 as libc::c_int,
        Some(dos_format as unsafe extern "C" fn() -> ()),
        0 as libc::c_int,
    );
    add_to_sclist(
        (1 as libc::c_int) << 5 as libc::c_int,
        b"M-M\0" as *const u8 as *const libc::c_char,
        0 as libc::c_int,
        Some(mac_format as unsafe extern "C" fn() -> ()),
        0 as libc::c_int,
    );
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
        add_to_sclist(
            (1 as libc::c_int) << 5 as libc::c_int,
            b"M-A\0" as *const u8 as *const libc::c_char,
            0 as libc::c_int,
            Some(append_it as unsafe extern "C" fn() -> ()),
            0 as libc::c_int,
        );
        add_to_sclist(
            (1 as libc::c_int) << 5 as libc::c_int,
            b"M-P\0" as *const u8 as *const libc::c_char,
            0 as libc::c_int,
            Some(prepend_it as unsafe extern "C" fn() -> ()),
            0 as libc::c_int,
        );
        add_to_sclist(
            (1 as libc::c_int) << 5 as libc::c_int,
            b"M-B\0" as *const u8 as *const libc::c_char,
            0 as libc::c_int,
            Some(back_it_up as unsafe extern "C" fn() -> ()),
            0 as libc::c_int,
        );
        add_to_sclist(
            (1 as libc::c_int) << 6 as libc::c_int
                | (1 as libc::c_int) << 7 as libc::c_int,
            b"^X\0" as *const u8 as *const libc::c_char,
            0 as libc::c_int,
            Some(flip_execute as unsafe extern "C" fn() -> ()),
            0 as libc::c_int,
        );
    }
    add_to_sclist(
        (1 as libc::c_int) << 6 as libc::c_int,
        b"M-N\0" as *const u8 as *const libc::c_char,
        0 as libc::c_int,
        Some(flip_convert as unsafe extern "C" fn() -> ()),
        0 as libc::c_int,
    );
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
        add_to_sclist(
            (1 as libc::c_int) << 6 as libc::c_int
                | (1 as libc::c_int) << 7 as libc::c_int,
            b"M-F\0" as *const u8 as *const libc::c_char,
            0 as libc::c_int,
            Some(flip_newbuffer as unsafe extern "C" fn() -> ()),
            0 as libc::c_int,
        );
        add_to_sclist(
            (1 as libc::c_int) << 7 as libc::c_int,
            b"M-\\\0" as *const u8 as *const libc::c_char,
            0 as libc::c_int,
            Some(flip_pipe as unsafe extern "C" fn() -> ()),
            0 as libc::c_int,
        );
    }
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
    {
        add_to_sclist(
            (1 as libc::c_int) << 5 as libc::c_int
                | (1 as libc::c_int) << 6 as libc::c_int,
            b"^T\0" as *const u8 as *const libc::c_char,
            0 as libc::c_int,
            Some(to_files as unsafe extern "C" fn() -> ()),
            0 as libc::c_int,
        );
    }
    add_to_sclist(
        (1 as libc::c_int) << 10 as libc::c_int | (1 as libc::c_int) << 8 as libc::c_int,
        b"^C\0" as *const u8 as *const libc::c_char,
        0 as libc::c_int,
        Some(do_exit as unsafe extern "C" fn() -> ()),
        0 as libc::c_int,
    );
    add_to_sclist(
        (1 as libc::c_int) << 10 as libc::c_int,
        b"^T\0" as *const u8 as *const libc::c_char,
        0 as libc::c_int,
        Some(do_exit as unsafe extern "C" fn() -> ()),
        0 as libc::c_int,
    );
    add_to_sclist(
        (1 as libc::c_int) << 8 as libc::c_int,
        b"^G\0" as *const u8 as *const libc::c_char,
        0 as libc::c_int,
        Some(do_exit as unsafe extern "C" fn() -> ()),
        0 as libc::c_int,
    );
    add_to_sclist(
        (1 as libc::c_int) << 8 as libc::c_int,
        b"F1\0" as *const u8 as *const libc::c_char,
        0o410 as libc::c_int + 1 as libc::c_int,
        Some(do_exit as unsafe extern "C" fn() -> ()),
        0 as libc::c_int,
    );
    add_to_sclist(
        (1 as libc::c_int) << 8 as libc::c_int,
        b"Home\0" as *const u8 as *const libc::c_char,
        0o406 as libc::c_int,
        Some(to_first_line as unsafe extern "C" fn() -> ()),
        0 as libc::c_int,
    );
    add_to_sclist(
        (1 as libc::c_int) << 8 as libc::c_int,
        b"End\0" as *const u8 as *const libc::c_char,
        0o550 as libc::c_int,
        Some(to_last_line as unsafe extern "C" fn() -> ()),
        0 as libc::c_int,
    );
    add_to_sclist(
        (1 as libc::c_int) << 14 as libc::c_int,
        b"^X\0" as *const u8 as *const libc::c_char,
        0 as libc::c_int,
        Some(do_cancel as unsafe extern "C" fn() -> ()),
        0 as libc::c_int,
    );
    add_to_sclist(
        ((1 as libc::c_int) << 0 as libc::c_int | (1 as libc::c_int) << 1 as libc::c_int
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
            | (1 as libc::c_int) << 14 as libc::c_int)
            & !((1 as libc::c_int) << 15 as libc::c_int),
        b"F1\0" as *const u8 as *const libc::c_char,
        0o410 as libc::c_int + 1 as libc::c_int,
        Some(do_help as unsafe extern "C" fn() -> ()),
        0 as libc::c_int,
    );
    add_to_sclist(
        (1 as libc::c_int) << 0 as libc::c_int | (1 as libc::c_int) << 10 as libc::c_int
            | (1 as libc::c_int) << 8 as libc::c_int,
        b"F2\0" as *const u8 as *const libc::c_char,
        0o410 as libc::c_int + 2 as libc::c_int,
        Some(do_exit as unsafe extern "C" fn() -> ()),
        0 as libc::c_int,
    );
    add_to_sclist(
        (1 as libc::c_int) << 0 as libc::c_int,
        b"F3\0" as *const u8 as *const libc::c_char,
        0o410 as libc::c_int + 3 as libc::c_int,
        Some(do_writeout as unsafe extern "C" fn() -> ()),
        0 as libc::c_int,
    );
    add_to_sclist(
        (1 as libc::c_int) << 0 as libc::c_int,
        b"F4\0" as *const u8 as *const libc::c_char,
        0o410 as libc::c_int + 4 as libc::c_int,
        Some(do_justify as unsafe extern "C" fn() -> ()),
        0 as libc::c_int,
    );
    add_to_sclist(
        (1 as libc::c_int) << 0 as libc::c_int,
        b"F5\0" as *const u8 as *const libc::c_char,
        0o410 as libc::c_int + 5 as libc::c_int,
        Some(do_insertfile as unsafe extern "C" fn() -> ()),
        0 as libc::c_int,
    );
    add_to_sclist(
        (1 as libc::c_int) << 0 as libc::c_int | (1 as libc::c_int) << 10 as libc::c_int
            | (1 as libc::c_int) << 8 as libc::c_int,
        b"F6\0" as *const u8 as *const libc::c_char,
        0o410 as libc::c_int + 6 as libc::c_int,
        Some(do_search_forward as unsafe extern "C" fn() -> ()),
        0 as libc::c_int,
    );
    add_to_sclist(
        (1 as libc::c_int) << 0 as libc::c_int | (1 as libc::c_int) << 10 as libc::c_int
            | (1 as libc::c_int) << 8 as libc::c_int
            | (1 as libc::c_int) << 14 as libc::c_int,
        b"F7\0" as *const u8 as *const libc::c_char,
        0o410 as libc::c_int + 7 as libc::c_int,
        Some(do_page_up as unsafe extern "C" fn() -> ()),
        0 as libc::c_int,
    );
    add_to_sclist(
        (1 as libc::c_int) << 0 as libc::c_int | (1 as libc::c_int) << 10 as libc::c_int
            | (1 as libc::c_int) << 8 as libc::c_int
            | (1 as libc::c_int) << 14 as libc::c_int,
        b"F8\0" as *const u8 as *const libc::c_char,
        0o410 as libc::c_int + 8 as libc::c_int,
        Some(do_page_down as unsafe extern "C" fn() -> ()),
        0 as libc::c_int,
    );
    add_to_sclist(
        (1 as libc::c_int) << 0 as libc::c_int | (1 as libc::c_int) << 1 as libc::c_int
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
            | (1 as libc::c_int) << 14 as libc::c_int,
        b"F9\0" as *const u8 as *const libc::c_char,
        0o410 as libc::c_int + 9 as libc::c_int,
        Some(cut_text as unsafe extern "C" fn() -> ()),
        0 as libc::c_int,
    );
    add_to_sclist(
        (1 as libc::c_int) << 0 as libc::c_int | (1 as libc::c_int) << 1 as libc::c_int
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
            | (1 as libc::c_int) << 14 as libc::c_int,
        b"F10\0" as *const u8 as *const libc::c_char,
        0o410 as libc::c_int + 10 as libc::c_int,
        Some(paste_text as unsafe extern "C" fn() -> ()),
        0 as libc::c_int,
    );
    add_to_sclist(
        (1 as libc::c_int) << 0 as libc::c_int,
        b"F11\0" as *const u8 as *const libc::c_char,
        0o410 as libc::c_int + 11 as libc::c_int,
        Some(report_cursor_position as unsafe extern "C" fn() -> ()),
        0 as libc::c_int,
    );
    add_to_sclist(
        (1 as libc::c_int) << 0 as libc::c_int,
        b"F12\0" as *const u8 as *const libc::c_char,
        0o410 as libc::c_int + 12 as libc::c_int,
        Some(do_spell as unsafe extern "C" fn() -> ()),
        0 as libc::c_int,
    );
    add_to_sclist(
        ((1 as libc::c_int) << 0 as libc::c_int | (1 as libc::c_int) << 1 as libc::c_int
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
            | (1 as libc::c_int) << 14 as libc::c_int)
            & !((1 as libc::c_int) << 0 as libc::c_int)
            | (1 as libc::c_int) << 13 as libc::c_int,
        b"\0" as *const u8 as *const libc::c_char,
        0o543 as libc::c_int,
        Some(do_cancel as unsafe extern "C" fn() -> ()),
        0 as libc::c_int,
    );
    add_to_sclist(
        (1 as libc::c_int) << 0 as libc::c_int,
        b"\0" as *const u8 as *const libc::c_char,
        0o610 as libc::c_int,
        Some(do_insertfile as unsafe extern "C" fn() -> ()),
        0 as libc::c_int,
    );
    add_to_sclist(
        (1 as libc::c_int) << 0 as libc::c_int | (1 as libc::c_int) << 1 as libc::c_int
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
            | (1 as libc::c_int) << 13 as libc::c_int,
        b"\0" as *const u8 as *const libc::c_char,
        0x4fb as libc::c_int,
        Some(do_nothing as unsafe extern "C" fn() -> ()),
        0 as libc::c_int,
    );
}
pub unsafe extern "C" fn epithet_of_flag(mut flag: libc::c_int) -> *const libc::c_char {
    match flag {
        48 => return b"Hidden interface\0" as *const u8 as *const libc::c_char,
        3 => return b"Help mode\0" as *const u8 as *const libc::c_char,
        2 => {
            return b"Constant cursor position display\0" as *const u8
                as *const libc::c_char;
        }
        29 => {
            return b"Soft wrapping of overlong lines\0" as *const u8
                as *const libc::c_char;
        }
        36 => return b"Line numbering\0" as *const u8 as *const libc::c_char,
        23 => return b"Whitespace display\0" as *const u8 as *const libc::c_char,
        18 => return b"Color syntax highlighting\0" as *const u8 as *const libc::c_char,
        22 => return b"Smart home key\0" as *const u8 as *const libc::c_char,
        5 => return b"Auto indent\0" as *const u8 as *const libc::c_char,
        10 => return b"Cut to end\0" as *const u8 as *const libc::c_char,
        40 => {
            return b"Hard wrapping of overlong lines\0" as *const u8
                as *const libc::c_char;
        }
        24 => {
            return b"Conversion of typed tabs to spaces\0" as *const u8
                as *const libc::c_char;
        }
        7 => return b"Mouse support\0" as *const u8 as *const libc::c_char,
        _ => return b"Ehm...\0" as *const u8 as *const libc::c_char,
    };
}
