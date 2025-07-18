use ::libc;
use ::c2rust_bitfields;
extern "C" {
    pub type re_dfa_t;
    pub type ldat;
    fn beep() -> libc::c_int;
    fn rpl_free(ptr: *mut libc::c_void);
    fn sprintf(_: *mut libc::c_char, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn vsnprintf(
        _: *mut libc::c_char,
        _: libc::c_ulong,
        _: *const libc::c_char,
        _: ::std::ffi::VaList,
    ) -> libc::c_int;
    fn mvwprintw(
        _: *mut WINDOW,
        _: libc::c_int,
        _: libc::c_int,
        _: *const libc::c_char,
        _: ...
    ) -> libc::c_int;
    fn free_lines(src: *mut linestruct);
    fn waddch(_: *mut WINDOW, _: chtype) -> libc::c_int;
    fn waddnstr(_: *mut WINDOW, _: *const libc::c_char, _: libc::c_int) -> libc::c_int;
    fn wattr_on(_: *mut WINDOW, _: attr_t, _: *mut libc::c_void) -> libc::c_int;
    fn wattr_off(_: *mut WINDOW, _: attr_t, _: *mut libc::c_void) -> libc::c_int;
    fn wmove(_: *mut WINDOW, _: libc::c_int, _: libc::c_int) -> libc::c_int;
    fn wnoutrefresh(_: *mut WINDOW) -> libc::c_int;
    fn wrefresh(_: *mut WINDOW) -> libc::c_int;
    static mut COLS: libc::c_int;
    fn wmouse_trafo(
        _: *const WINDOW,
        _: *mut libc::c_int,
        _: *mut libc::c_int,
        _: bool,
    ) -> bool;
    fn dcgettext(
        __domainname: *const libc::c_char,
        __msgid: *const libc::c_char,
        __category: libc::c_int,
    ) -> *mut libc::c_char;
    static mut meta_key: bool;
    static mut bracketed_paste: bool;
    static mut focusing: bool;
    static mut lastmessage: message_type;
    static mut answer: *mut libc::c_char;
    static mut flags: [libc::c_uint; 4];
    static mut footwin: *mut WINDOW;
    static mut cutbuffer: *mut linestruct;
    static mut openfile: *mut openfilestruct;
    static mut currmenu: libc::c_int;
    static mut interface_color_pair: [libc::c_int; 12];
    fn using_utf8() -> bool;
    fn is_word_char(c: *const libc::c_char, allow_punct: bool) -> bool;
    fn is_zerowidth(ch: *const libc::c_char) -> bool;
    fn char_length(pointer: *const libc::c_char) -> libc::c_int;
    fn step_left(buf: *const libc::c_char, pos: size_t) -> size_t;
    fn step_right(buf: *const libc::c_char, pos: size_t) -> size_t;
    fn do_delete();
    fn do_backspace();
    fn cut_text();
    fn copy_text();
    fn paste_text();
    fn input_tab(
        buf: *mut libc::c_char,
        place: *mut size_t,
        refresh_func: Option::<unsafe extern "C" fn() -> ()>,
        listed: *mut bool,
    ) -> *mut libc::c_char;
    fn first_sc_for(
        menu: libc::c_int,
        function: Option::<unsafe extern "C" fn() -> ()>,
    ) -> *const keystruct;
    fn get_shortcut(keycode: libc::c_int) -> *const keystruct;
    fn do_help();
    fn reset_history_pointer_for(list: *const linestruct);
    fn get_history_completion(
        h: *mut *mut linestruct,
        s: *mut libc::c_char,
        len: size_t,
    ) -> *mut libc::c_char;
    fn to_prev_word();
    fn to_next_word();
    fn do_home();
    fn do_end();
    fn do_left();
    fn do_right();
    fn make_new_node(prevnode: *mut linestruct) -> *mut linestruct;
    fn window_init();
    fn changes_something(f: functionptrtype) -> bool;
    fn nrealloc(ptr: *mut libc::c_void, howmuch: size_t) -> *mut libc::c_void;
    fn wipe_statusbar();
    fn do_enter();
    fn do_cancel();
    fn nmalloc(howmuch: size_t) -> *mut libc::c_void;
    fn do_tab();
    fn copy_of(string: *const libc::c_char) -> *mut libc::c_char;
    fn get_verbatim_kbinput(win: *mut WINDOW, count: *mut size_t) -> *mut libc::c_char;
    fn do_verbatim_input();
    fn implant(string: *const libc::c_char);
    fn do_nothing();
    fn bottombars(menu: libc::c_int);
    fn do_toggle();
    fn full_refresh();
    fn mallocstrcpy(
        dest: *mut libc::c_char,
        src: *const libc::c_char,
    ) -> *mut libc::c_char;
    fn get_newer_item();
    fn get_older_item();
    fn waiting_keycodes() -> size_t;
    fn get_kbinput(win: *mut WINDOW, showcursor: bool) -> libc::c_int;
    fn get_mouseinput(
        mouse_y: *mut libc::c_int,
        mouse_x: *mut libc::c_int,
        allow_shortcuts: bool,
    ) -> libc::c_int;
    fn breadth(text: *const libc::c_char) -> size_t;
    fn wideness(text: *const libc::c_char, maxlen: size_t) -> size_t;
    fn actual_x(text: *const libc::c_char, column: size_t) -> size_t;
    fn display_string(
        buf: *const libc::c_char,
        column: size_t,
        span: size_t,
        isdata: bool,
        isprompt: bool,
    ) -> *mut libc::c_char;
    fn edit_refresh();
    fn titlebar(path: *const libc::c_char);
    fn post_one_key(
        keystroke: *const libc::c_char,
        tag: *const libc::c_char,
        width: libc::c_int,
    );
    fn blank_bottombars();
    fn memmove(
        _: *mut libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn strncpy(
        _: *mut libc::c_char,
        _: *const libc::c_char,
        _: libc::c_ulong,
    ) -> *mut libc::c_char;
    fn strchr(_: *const libc::c_char, _: libc::c_int) -> *mut libc::c_char;
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    fn strstr(_: *const libc::c_char, _: *const libc::c_char) -> *mut libc::c_char;
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
pub type ssize_t = __ssize_t;
pub type size_t = libc::c_ulong;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct timespec {
    pub tv_sec: __time_t,
    pub tv_nsec: __syscall_slong_t,
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
pub type functionptrtype = Option::<unsafe extern "C" fn() -> ()>;
static mut prompt: *mut libc::c_char = 0 as *const libc::c_char as *mut libc::c_char;
static mut typing_x: size_t = !(0 as libc::c_int as size_t) >> 1 as libc::c_int;
pub unsafe extern "C" fn do_statusbar_home() {
    typing_x = 0 as libc::c_int as size_t;
}
pub unsafe extern "C" fn do_statusbar_end() {
    typing_x = strlen(answer);
}
pub unsafe extern "C" fn do_statusbar_prev_word() {
    let mut seen_a_word: bool = 0 as libc::c_int != 0;
    let mut step_forward: bool = 0 as libc::c_int != 0;
    while typing_x != 0 as libc::c_int as libc::c_ulong {
        typing_x = step_left(answer, typing_x);
        if is_word_char(answer.offset(typing_x as isize), 0 as libc::c_int != 0) {
            seen_a_word = 1 as libc::c_int != 0;
        } else if !is_zerowidth(answer.offset(typing_x as isize)) {
            if !seen_a_word {
                continue;
            }
            step_forward = 1 as libc::c_int != 0;
            break;
        }
    }
    if step_forward {
        typing_x = step_right(answer, typing_x);
    }
}
pub unsafe extern "C" fn do_statusbar_next_word() {
    let mut seen_space: bool = !is_word_char(
        answer.offset(typing_x as isize),
        0 as libc::c_int != 0,
    );
    let mut seen_word: bool = !seen_space;
    while *answer.offset(typing_x as isize) as libc::c_int != '\0' as i32 {
        typing_x = step_right(answer, typing_x);
        if flags[(AFTER_ENDS as libc::c_int as libc::c_ulong)
            .wrapping_div(
                (::std::mem::size_of::<libc::c_uint>() as libc::c_ulong)
                    .wrapping_mul(8 as libc::c_int as libc::c_ulong),
            ) as usize]
            & (1 as libc::c_int as libc::c_uint)
                << (AFTER_ENDS as libc::c_int as libc::c_ulong)
                    .wrapping_rem(
                        (::std::mem::size_of::<libc::c_uint>() as libc::c_ulong)
                            .wrapping_mul(8 as libc::c_int as libc::c_ulong),
                    ) != 0 as libc::c_int as libc::c_uint
        {
            if is_word_char(answer.offset(typing_x as isize), 0 as libc::c_int != 0) {
                seen_word = 1 as libc::c_int != 0;
            } else if !is_zerowidth(answer.offset(typing_x as isize)) {
                if seen_word {
                    break;
                }
            }
        } else if !is_zerowidth(answer.offset(typing_x as isize)) {
            if !is_word_char(answer.offset(typing_x as isize), 0 as libc::c_int != 0) {
                seen_space = 1 as libc::c_int != 0;
            } else if seen_space {
                break;
            }
        }
    }
}
pub unsafe extern "C" fn do_statusbar_left() {
    if typing_x > 0 as libc::c_int as libc::c_ulong {
        typing_x = step_left(answer, typing_x);
        while typing_x > 0 as libc::c_int as libc::c_ulong
            && is_zerowidth(answer.offset(typing_x as isize)) as libc::c_int != 0
        {
            typing_x = step_left(answer, typing_x);
        }
    }
}
pub unsafe extern "C" fn do_statusbar_right() {
    if *answer.offset(typing_x as isize) as libc::c_int != '\0' as i32 {
        typing_x = step_right(answer, typing_x);
        while *answer.offset(typing_x as isize) as libc::c_int != '\0' as i32
            && is_zerowidth(answer.offset(typing_x as isize)) as libc::c_int != 0
        {
            typing_x = step_right(answer, typing_x);
        }
    }
}
pub unsafe extern "C" fn do_statusbar_backspace() {
    if typing_x > 0 as libc::c_int as libc::c_ulong {
        let mut was_x: size_t = typing_x;
        typing_x = step_left(answer, typing_x);
        memmove(
            answer.offset(typing_x as isize) as *mut libc::c_void,
            answer.offset(was_x as isize) as *const libc::c_void,
            (strlen(answer))
                .wrapping_sub(was_x)
                .wrapping_add(1 as libc::c_int as libc::c_ulong),
        );
    }
}
pub unsafe extern "C" fn do_statusbar_delete() {
    if *answer.offset(typing_x as isize) as libc::c_int != '\0' as i32 {
        let mut charlen: libc::c_int = char_length(answer.offset(typing_x as isize));
        memmove(
            answer.offset(typing_x as isize) as *mut libc::c_void,
            answer.offset(typing_x as isize).offset(charlen as isize)
                as *const libc::c_void,
            (strlen(answer))
                .wrapping_sub(typing_x)
                .wrapping_sub(charlen as libc::c_ulong)
                .wrapping_add(1 as libc::c_int as libc::c_ulong),
        );
        if is_zerowidth(answer.offset(typing_x as isize)) {
            do_statusbar_delete();
        }
    }
}
pub unsafe extern "C" fn lop_the_answer() {
    if *answer.offset(typing_x as isize) as libc::c_int == '\0' as i32 {
        typing_x = 0 as libc::c_int as size_t;
    }
    *answer.offset(typing_x as isize) = '\0' as i32 as libc::c_char;
}
pub unsafe extern "C" fn copy_the_answer() {
    if *answer != 0 {
        free_lines(cutbuffer);
        cutbuffer = make_new_node(0 as *mut linestruct);
        (*cutbuffer).data = copy_of(answer);
        typing_x = 0 as libc::c_int as size_t;
    }
}
pub unsafe extern "C" fn paste_into_answer() {
    let mut pastelen: size_t = strlen((*cutbuffer).data);
    answer = nrealloc(
        answer as *mut libc::c_void,
        (strlen(answer))
            .wrapping_add(pastelen)
            .wrapping_add(1 as libc::c_int as libc::c_ulong),
    ) as *mut libc::c_char;
    memmove(
        answer.offset(typing_x as isize).offset(pastelen as isize) as *mut libc::c_void,
        answer.offset(typing_x as isize) as *const libc::c_void,
        (strlen(answer))
            .wrapping_sub(typing_x)
            .wrapping_add(1 as libc::c_int as libc::c_ulong),
    );
    strncpy(answer.offset(typing_x as isize), (*cutbuffer).data, pastelen);
    typing_x = (typing_x as libc::c_ulong).wrapping_add(pastelen) as size_t as size_t;
}
pub unsafe extern "C" fn do_statusbar_mouse() -> libc::c_int {
    let mut click_row: libc::c_int = 0;
    let mut click_col: libc::c_int = 0;
    let mut retval: libc::c_int = get_mouseinput(
        &mut click_row,
        &mut click_col,
        1 as libc::c_int != 0,
    );
    if retval == 0 as libc::c_int
        && wmouse_trafo(footwin, &mut click_row, &mut click_col, 0 as libc::c_int != 0)
            as libc::c_int != 0
    {
        let mut start_col: size_t = (breadth(prompt))
            .wrapping_add(2 as libc::c_int as libc::c_ulong);
        if click_row == 0 as libc::c_int && click_col as libc::c_ulong >= start_col {
            typing_x = actual_x(
                answer,
                (get_statusbar_page_start(
                    start_col,
                    start_col.wrapping_add(wideness(answer, typing_x)),
                ))
                    .wrapping_add(click_col as libc::c_ulong)
                    .wrapping_sub(start_col),
            );
        }
    }
    return retval;
}
pub unsafe extern "C" fn inject_into_answer(
    mut burst: *mut libc::c_char,
    mut count: size_t,
) {
    let mut index: size_t = 0 as libc::c_int as size_t;
    while index < count {
        if *burst.offset(index as isize) as libc::c_int == '\0' as i32 {
            *burst.offset(index as isize) = '\n' as i32 as libc::c_char;
        }
        index = index.wrapping_add(1);
        index;
    }
    answer = nrealloc(
        answer as *mut libc::c_void,
        (strlen(answer))
            .wrapping_add(count)
            .wrapping_add(1 as libc::c_int as libc::c_ulong),
    ) as *mut libc::c_char;
    memmove(
        answer.offset(typing_x as isize).offset(count as isize) as *mut libc::c_void,
        answer.offset(typing_x as isize) as *const libc::c_void,
        (strlen(answer))
            .wrapping_sub(typing_x)
            .wrapping_add(1 as libc::c_int as libc::c_ulong),
    );
    strncpy(answer.offset(typing_x as isize), burst, count);
    typing_x = (typing_x as libc::c_ulong).wrapping_add(count) as size_t as size_t;
}
pub unsafe extern "C" fn do_statusbar_verbatim_input() {
    let mut count: size_t = 1 as libc::c_int as size_t;
    let mut bytes: *mut libc::c_char = 0 as *mut libc::c_char;
    bytes = get_verbatim_kbinput(footwin, &mut count);
    if (0 as libc::c_int as libc::c_ulong) < count
        && count < 999 as libc::c_int as libc::c_ulong
    {
        inject_into_answer(bytes, count);
    } else if count == 0 as libc::c_int as libc::c_ulong {
        beep();
    }
    rpl_free(bytes as *mut libc::c_void);
}
pub unsafe extern "C" fn absorb_character(
    mut input: libc::c_int,
    mut function: functionptrtype,
) {
    static mut puddle: *mut libc::c_char = 0 as *const libc::c_char as *mut libc::c_char;
    static mut capacity: size_t = 8 as libc::c_int as size_t;
    static mut depth: size_t = 0 as libc::c_int as size_t;
    if function.is_none() {
        if input < 0x20 as libc::c_int || input > 0xff as libc::c_int
            || meta_key as libc::c_int != 0
        {
            beep();
        } else if !(flags[(RESTRICTED as libc::c_int as libc::c_ulong)
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
            || currmenu != (1 as libc::c_int) << 5 as libc::c_int
            || *((*openfile).filename).offset(0 as libc::c_int as isize) as libc::c_int
                == '\0' as i32
        {
            if depth.wrapping_add(1 as libc::c_int as libc::c_ulong) == capacity {
                capacity = (2 as libc::c_int as libc::c_ulong).wrapping_mul(capacity);
                puddle = nrealloc(puddle as *mut libc::c_void, capacity)
                    as *mut libc::c_char;
            } else if puddle.is_null() {
                puddle = nmalloc(capacity) as *mut libc::c_char;
            }
            let fresh0 = depth;
            depth = depth.wrapping_add(1);
            *puddle.offset(fresh0 as isize) = input as libc::c_char;
        }
    }
    if depth > 0 as libc::c_int as libc::c_ulong
        && (function.is_some()
            || waiting_keycodes() == 0 as libc::c_int as libc::c_ulong)
    {
        *puddle.offset(depth as isize) = '\0' as i32 as libc::c_char;
        inject_into_answer(puddle, depth);
        depth = 0 as libc::c_int as size_t;
    }
}
pub unsafe extern "C" fn handle_editing(mut function: functionptrtype) -> bool {
    if function == Some(do_left as unsafe extern "C" fn() -> ()) {
        do_statusbar_left();
    } else if function == Some(do_right as unsafe extern "C" fn() -> ()) {
        do_statusbar_right();
    } else if function == Some(to_prev_word as unsafe extern "C" fn() -> ()) {
        do_statusbar_prev_word();
    } else if function == Some(to_next_word as unsafe extern "C" fn() -> ()) {
        do_statusbar_next_word();
    } else if function == Some(do_home as unsafe extern "C" fn() -> ()) {
        do_statusbar_home();
    } else if function == Some(do_end as unsafe extern "C" fn() -> ()) {
        do_statusbar_end();
    } else if !(flags[(RESTRICTED as libc::c_int as libc::c_ulong)
        .wrapping_div(
            (::std::mem::size_of::<libc::c_uint>() as libc::c_ulong)
                .wrapping_mul(8 as libc::c_int as libc::c_ulong),
        ) as usize]
        & (1 as libc::c_int as libc::c_uint)
            << (RESTRICTED as libc::c_int as libc::c_ulong)
                .wrapping_rem(
                    (::std::mem::size_of::<libc::c_uint>() as libc::c_ulong)
                        .wrapping_mul(8 as libc::c_int as libc::c_ulong),
                ) != 0 as libc::c_int as libc::c_uint
        && currmenu == (1 as libc::c_int) << 5 as libc::c_int
        && *((*openfile).filename).offset(0 as libc::c_int as isize) as libc::c_int
            != '\0' as i32
        && (function == Some(do_verbatim_input as unsafe extern "C" fn() -> ())
            || function == Some(do_delete as unsafe extern "C" fn() -> ())
            || function == Some(do_backspace as unsafe extern "C" fn() -> ())
            || function == Some(cut_text as unsafe extern "C" fn() -> ())
            || function == Some(paste_text as unsafe extern "C" fn() -> ())))
    {
        if function == Some(do_verbatim_input as unsafe extern "C" fn() -> ()) {
            do_statusbar_verbatim_input();
        } else if function == Some(do_delete as unsafe extern "C" fn() -> ()) {
            do_statusbar_delete();
        } else if function == Some(do_backspace as unsafe extern "C" fn() -> ()) {
            do_statusbar_backspace();
        } else if function == Some(cut_text as unsafe extern "C" fn() -> ()) {
            lop_the_answer();
        } else if function == Some(copy_text as unsafe extern "C" fn() -> ()) {
            copy_the_answer();
        } else if function == Some(paste_text as unsafe extern "C" fn() -> ()) {
            if !cutbuffer.is_null() {
                paste_into_answer();
            }
        } else {
            return 0 as libc::c_int != 0
        }
    }
    return 1 as libc::c_int != 0;
}
pub unsafe extern "C" fn get_statusbar_page_start(
    mut base: size_t,
    mut column: size_t,
) -> size_t {
    if column == base || column < (COLS - 1 as libc::c_int) as libc::c_ulong {
        return 0 as libc::c_int as size_t
    } else if COLS as libc::c_ulong
        > base.wrapping_add(2 as libc::c_int as libc::c_ulong)
    {
        return column
            .wrapping_sub(base)
            .wrapping_sub(1 as libc::c_int as libc::c_ulong)
            .wrapping_sub(
                column
                    .wrapping_sub(base)
                    .wrapping_sub(1 as libc::c_int as libc::c_ulong)
                    .wrapping_rem(
                        (COLS as libc::c_ulong)
                            .wrapping_sub(base)
                            .wrapping_sub(2 as libc::c_int as libc::c_ulong),
                    ),
            )
    } else {
        return column.wrapping_sub(2 as libc::c_int as libc::c_ulong)
    };
}
pub unsafe extern "C" fn put_cursor_at_end_of_answer() {
    typing_x = !(0 as libc::c_int as size_t) >> 1 as libc::c_int;
}
pub unsafe extern "C" fn draw_the_promptbar() {
    let mut base: size_t = (breadth(prompt))
        .wrapping_add(2 as libc::c_int as libc::c_ulong);
    let mut column: size_t = base.wrapping_add(wideness(answer, typing_x));
    let mut the_page: size_t = 0;
    let mut end_page: size_t = 0;
    let mut expanded: *mut libc::c_char = 0 as *mut libc::c_char;
    the_page = get_statusbar_page_start(base, column);
    end_page = get_statusbar_page_start(
        base,
        base
            .wrapping_add(breadth(answer))
            .wrapping_sub(1 as libc::c_int as libc::c_ulong),
    );
    wattr_on(
        footwin,
        interface_color_pair[PROMPT_BAR as libc::c_int as usize] as attr_t,
        0 as *mut libc::c_void,
    );
    mvwprintw(
        footwin,
        0 as libc::c_int,
        0 as libc::c_int,
        b"%*s\0" as *const u8 as *const libc::c_char,
        COLS,
        b" \0" as *const u8 as *const libc::c_char,
    );
    if wmove(footwin, 0 as libc::c_int, 0 as libc::c_int) == -(1 as libc::c_int) {
        -(1 as libc::c_int);
    } else {
        waddnstr(footwin, prompt, -(1 as libc::c_int));
    };
    waddch(footwin, ':' as i32 as chtype);
    waddch(
        footwin,
        (if the_page == 0 as libc::c_int as libc::c_ulong {
            ' ' as i32
        } else {
            '<' as i32
        }) as chtype,
    );
    expanded = display_string(
        answer,
        the_page,
        (COLS as libc::c_ulong).wrapping_sub(base),
        0 as libc::c_int != 0,
        1 as libc::c_int != 0,
    );
    waddnstr(footwin, expanded, -(1 as libc::c_int));
    rpl_free(expanded as *mut libc::c_void);
    if the_page < end_page
        && base.wrapping_add(breadth(answer)).wrapping_sub(the_page)
            > COLS as libc::c_ulong
    {
        if wmove(footwin, 0 as libc::c_int, COLS - 1 as libc::c_int)
            == -(1 as libc::c_int)
        {
            -(1 as libc::c_int);
        } else {
            waddch(footwin, '>' as i32 as chtype);
        };
    }
    wattr_off(
        footwin,
        interface_color_pair[PROMPT_BAR as libc::c_int as usize] as attr_t,
        0 as *mut libc::c_void,
    );
    if flags[(NO_HELP as libc::c_int as libc::c_ulong)
        .wrapping_div(
            (::std::mem::size_of::<libc::c_uint>() as libc::c_ulong)
                .wrapping_mul(8 as libc::c_int as libc::c_ulong),
        ) as usize]
        & (1 as libc::c_int as libc::c_uint)
            << (NO_HELP as libc::c_int as libc::c_ulong)
                .wrapping_rem(
                    (::std::mem::size_of::<libc::c_uint>() as libc::c_ulong)
                        .wrapping_mul(8 as libc::c_int as libc::c_ulong),
                ) != 0 as libc::c_int as libc::c_uint
    {
        wmove(footwin, 0 as libc::c_int, 0 as libc::c_int);
        wrefresh(footwin);
    }
    wmove(footwin, 0 as libc::c_int, column.wrapping_sub(the_page) as libc::c_int);
    wnoutrefresh(footwin);
}
pub unsafe extern "C" fn add_or_remove_pipe_symbol_from_answer() {
    if *answer.offset(0 as libc::c_int as isize) as libc::c_int == '|' as i32 {
        memmove(
            answer as *mut libc::c_void,
            answer.offset(1 as libc::c_int as isize) as *const libc::c_void,
            strlen(answer),
        );
        if typing_x > 0 as libc::c_int as libc::c_ulong {
            typing_x = typing_x.wrapping_sub(1);
            typing_x;
        }
    } else {
        answer = nrealloc(
            answer as *mut libc::c_void,
            (strlen(answer)).wrapping_add(2 as libc::c_int as libc::c_ulong),
        ) as *mut libc::c_char;
        memmove(
            answer.offset(1 as libc::c_int as isize) as *mut libc::c_void,
            answer as *const libc::c_void,
            (strlen(answer)).wrapping_add(1 as libc::c_int as libc::c_ulong),
        );
        *answer.offset(0 as libc::c_int as isize) = '|' as i32 as libc::c_char;
        typing_x = typing_x.wrapping_add(1);
        typing_x;
    };
}
pub unsafe extern "C" fn acquire_an_answer(
    mut actual: *mut libc::c_int,
    mut listed: *mut bool,
    mut history_list: *mut *mut linestruct,
    mut refresh_func: Option::<unsafe extern "C" fn() -> ()>,
) -> functionptrtype {
    let mut stored_string: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut previous_was_tab: bool = 0 as libc::c_int != 0;
    let mut fragment_length: size_t = 0 as libc::c_int as size_t;
    let mut shortcut: *const keystruct = 0 as *const keystruct;
    let mut function: functionptrtype = None;
    let mut input: libc::c_int = 0;
    if typing_x > strlen(answer) {
        typing_x = strlen(answer);
    }
    loop {
        draw_the_promptbar();
        input = get_kbinput(footwin, 1 as libc::c_int != 0);
        if input == -(2 as libc::c_int) {
            refresh_func.unwrap()();
            *actual = -(2 as libc::c_int);
            rpl_free(stored_string as *mut libc::c_void);
            return None;
        }
        if input == 0o631 as libc::c_int && do_statusbar_mouse() == 1 as libc::c_int {
            input = get_kbinput(footwin, 0 as libc::c_int != 0);
        }
        if input == 0o631 as libc::c_int {
            continue;
        }
        shortcut = get_shortcut(input);
        function = if !shortcut.is_null() { (*shortcut).func } else { None };
        absorb_character(input, function);
        if function == Some(do_cancel as unsafe extern "C" fn() -> ())
            || function == Some(do_enter as unsafe extern "C" fn() -> ())
        {
            break;
        }
        if function == Some(do_tab as unsafe extern "C" fn() -> ()) {
            if !history_list.is_null() {
                if !previous_was_tab {
                    fragment_length = strlen(answer);
                }
                if fragment_length > 0 as libc::c_int as libc::c_ulong {
                    answer = get_history_completion(
                        history_list,
                        answer,
                        fragment_length,
                    );
                    typing_x = strlen(answer);
                }
            } else if currmenu
                & ((1 as libc::c_int) << 6 as libc::c_int
                    | (1 as libc::c_int) << 5 as libc::c_int
                    | (1 as libc::c_int) << 12 as libc::c_int) != 0
                && !(flags[(RESTRICTED as libc::c_int as libc::c_ulong)
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
                answer = input_tab(answer, &mut typing_x, refresh_func, listed);
            }
        } else if function == Some(get_older_item as unsafe extern "C" fn() -> ())
            && !history_list.is_null()
        {
            if stored_string.is_null() {
                reset_history_pointer_for(*history_list);
            }
            if ((**history_list).next).is_null() {
                stored_string = mallocstrcpy(stored_string, answer);
            }
            if !((**history_list).prev).is_null() {
                *history_list = (**history_list).prev;
                answer = mallocstrcpy(answer, (**history_list).data);
                typing_x = strlen(answer);
            }
        } else if function == Some(get_newer_item as unsafe extern "C" fn() -> ())
            && !history_list.is_null()
        {
            if !((**history_list).next).is_null() {
                *history_list = (**history_list).next;
                answer = mallocstrcpy(answer, (**history_list).data);
                typing_x = strlen(answer);
            }
            if ((**history_list).next).is_null() && !stored_string.is_null()
                && *answer as libc::c_int == '\0' as i32
            {
                answer = mallocstrcpy(answer, stored_string);
                typing_x = strlen(answer);
            }
        } else if function == Some(do_help as unsafe extern "C" fn() -> ())
            || function == Some(full_refresh as unsafe extern "C" fn() -> ())
        {
            function.unwrap()();
        } else if function == Some(do_toggle as unsafe extern "C" fn() -> ())
            && (*shortcut).toggle == NO_HELP as libc::c_int
        {
            flags[(NO_HELP as libc::c_int as libc::c_ulong)
                .wrapping_div(
                    (::std::mem::size_of::<libc::c_uint>() as libc::c_ulong)
                        .wrapping_mul(8 as libc::c_int as libc::c_ulong),
                ) as usize]
                ^= (1 as libc::c_int as libc::c_uint)
                    << (NO_HELP as libc::c_int as libc::c_ulong)
                        .wrapping_rem(
                            (::std::mem::size_of::<libc::c_uint>() as libc::c_ulong)
                                .wrapping_mul(8 as libc::c_int as libc::c_ulong),
                        );
            window_init();
            focusing = 0 as libc::c_int != 0;
            refresh_func.unwrap()();
            bottombars(currmenu);
        } else if !(function == Some(do_nothing as unsafe extern "C" fn() -> ())) {
            if function
                == ::std::mem::transmute::<
                    Option::<unsafe extern "C" fn(*const libc::c_char) -> ()>,
                    functionptrtype,
                >(Some(implant as unsafe extern "C" fn(*const libc::c_char) -> ()))
            {
                implant((*shortcut).expansion);
            } else if function.is_some() && !handle_editing(function) {
                if !(flags[(VIEW_MODE as libc::c_int as libc::c_ulong)
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
                    || !changes_something(function)
                {
                    function.unwrap()();
                    break;
                } else {
                    beep();
                }
            }
        }
        previous_was_tab = function == Some(do_tab as unsafe extern "C" fn() -> ());
    }
    if !stored_string.is_null() {
        reset_history_pointer_for(*history_list);
        rpl_free(stored_string as *mut libc::c_void);
    }
    *actual = input;
    return function;
}
pub unsafe extern "C" fn do_prompt(
    mut menu: libc::c_int,
    mut provided: *const libc::c_char,
    mut history_list: *mut *mut linestruct,
    mut refresh_func: Option::<unsafe extern "C" fn() -> ()>,
    mut msg: *const libc::c_char,
    mut args: ...
) -> libc::c_int {
    let mut function: functionptrtype = None;
    let mut listed: bool = 0 as libc::c_int != 0;
    let mut ap: ::std::ffi::VaListImpl;
    let mut retval: libc::c_int = 0;
    let mut was_typing_x: size_t = typing_x;
    let mut saved_prompt: *mut libc::c_char = prompt;
    bottombars(menu);
    if answer != provided as *mut libc::c_char {
        answer = mallocstrcpy(answer, provided);
    }
    loop {
        prompt = nmalloc((COLS * 4 as libc::c_int + 1 as libc::c_int) as size_t)
            as *mut libc::c_char;
        ap = args.clone();
        vsnprintf(
            prompt,
            (COLS * 4 as libc::c_int) as libc::c_ulong,
            msg,
            ap.as_va_list(),
        );
        *prompt
            .offset(
                actual_x(
                    prompt,
                    (if COLS < 5 as libc::c_int {
                        0 as libc::c_int
                    } else {
                        COLS - 5 as libc::c_int
                    }) as size_t,
                ) as isize,
            ) = '\0' as i32 as libc::c_char;
        lastmessage = VACUUM;
        function = acquire_an_answer(
            &mut retval,
            &mut listed,
            history_list,
            refresh_func,
        );
        rpl_free(prompt as *mut libc::c_void);
        if !(retval == -(2 as libc::c_int)) {
            break;
        }
    }
    prompt = saved_prompt;
    if function == Some(do_cancel as unsafe extern "C" fn() -> ())
        || function == Some(do_enter as unsafe extern "C" fn() -> ())
    {
        typing_x = was_typing_x;
    }
    if function == Some(do_cancel as unsafe extern "C" fn() -> ()) {
        retval = -(1 as libc::c_int);
    } else if function == Some(do_enter as unsafe extern "C" fn() -> ()) {
        retval = if *answer as libc::c_int == '\0' as i32 {
            -(2 as libc::c_int)
        } else {
            0 as libc::c_int
        };
    }
    if lastmessage as libc::c_uint == VACUUM as libc::c_int as libc::c_uint {
        wipe_statusbar();
    }
    if listed {
        refresh_func.unwrap()();
    }
    return retval;
}
pub unsafe extern "C" fn ask_user(
    mut withall: bool,
    mut question: *const libc::c_char,
) -> libc::c_int {
    let mut choice: libc::c_int = -(2 as libc::c_int);
    let mut width: libc::c_int = 16 as libc::c_int;
    let mut yesstr: *const libc::c_char = dcgettext(
        0 as *const libc::c_char,
        b"Yy\0" as *const u8 as *const libc::c_char,
        5 as libc::c_int,
    );
    let mut nostr: *const libc::c_char = dcgettext(
        0 as *const libc::c_char,
        b"Nn\0" as *const u8 as *const libc::c_char,
        5 as libc::c_int,
    );
    let mut allstr: *const libc::c_char = dcgettext(
        0 as *const libc::c_char,
        b"Aa\0" as *const u8 as *const libc::c_char,
        5 as libc::c_int,
    );
    let mut shortcut: *const keystruct = 0 as *const keystruct;
    let mut function: functionptrtype = None;
    while choice == -(2 as libc::c_int) {
        let mut letter: [libc::c_char; 5] = [0; 5];
        let mut index: libc::c_int = 0 as libc::c_int;
        let mut kbinput: libc::c_int = 0;
        if !(flags[(NO_HELP as libc::c_int as libc::c_ulong)
            .wrapping_div(
                (::std::mem::size_of::<libc::c_uint>() as libc::c_ulong)
                    .wrapping_mul(8 as libc::c_int as libc::c_ulong),
            ) as usize]
            & (1 as libc::c_int as libc::c_uint)
                << (NO_HELP as libc::c_int as libc::c_ulong)
                    .wrapping_rem(
                        (::std::mem::size_of::<libc::c_uint>() as libc::c_ulong)
                            .wrapping_mul(8 as libc::c_int as libc::c_ulong),
                    ) != 0 as libc::c_int as libc::c_uint)
        {
            let mut shortstr: [libc::c_char; 6] = [0; 6];
            let mut cancelshortcut: *const keystruct = first_sc_for(
                (1 as libc::c_int) << 13 as libc::c_int,
                Some(do_cancel as unsafe extern "C" fn() -> ()),
            );
            if COLS < 32 as libc::c_int {
                width = COLS / 2 as libc::c_int;
            }
            blank_bottombars();
            sprintf(
                shortstr.as_mut_ptr(),
                b" %c\0" as *const u8 as *const libc::c_char,
                *yesstr.offset(0 as libc::c_int as isize) as libc::c_int,
            );
            wmove(footwin, 1 as libc::c_int, 0 as libc::c_int);
            post_one_key(
                shortstr.as_mut_ptr(),
                dcgettext(
                    0 as *const libc::c_char,
                    b"Yes\0" as *const u8 as *const libc::c_char,
                    5 as libc::c_int,
                ),
                width,
            );
            shortstr[1 as libc::c_int
                as usize] = *nostr.offset(0 as libc::c_int as isize);
            wmove(footwin, 2 as libc::c_int, 0 as libc::c_int);
            post_one_key(
                shortstr.as_mut_ptr(),
                dcgettext(
                    0 as *const libc::c_char,
                    b"No\0" as *const u8 as *const libc::c_char,
                    5 as libc::c_int,
                ),
                width,
            );
            if withall {
                shortstr[1 as libc::c_int
                    as usize] = *allstr.offset(0 as libc::c_int as isize);
                wmove(footwin, 1 as libc::c_int, width);
                post_one_key(
                    shortstr.as_mut_ptr(),
                    dcgettext(
                        0 as *const libc::c_char,
                        b"All\0" as *const u8 as *const libc::c_char,
                        5 as libc::c_int,
                    ),
                    width,
                );
            }
            wmove(footwin, 2 as libc::c_int, width);
            post_one_key(
                (*cancelshortcut).keystr,
                dcgettext(
                    0 as *const libc::c_char,
                    b"Cancel\0" as *const u8 as *const libc::c_char,
                    5 as libc::c_int,
                ),
                width,
            );
        }
        wattr_on(
            footwin,
            interface_color_pair[PROMPT_BAR as libc::c_int as usize] as attr_t,
            0 as *mut libc::c_void,
        );
        mvwprintw(
            footwin,
            0 as libc::c_int,
            0 as libc::c_int,
            b"%*s\0" as *const u8 as *const libc::c_char,
            COLS,
            b" \0" as *const u8 as *const libc::c_char,
        );
        if wmove(footwin, 0 as libc::c_int, 0 as libc::c_int) == -(1 as libc::c_int) {
            -(1 as libc::c_int);
        } else {
            waddnstr(
                footwin,
                question,
                actual_x(question, (COLS - 1 as libc::c_int) as size_t) as libc::c_int,
            );
        };
        wattr_off(
            footwin,
            interface_color_pair[PROMPT_BAR as libc::c_int as usize] as attr_t,
            0 as *mut libc::c_void,
        );
        wnoutrefresh(footwin);
        currmenu = (1 as libc::c_int) << 13 as libc::c_int;
        kbinput = get_kbinput(footwin, !withall);
        if kbinput == -(2 as libc::c_int) {
            continue;
        }
        if bracketed_paste {
            kbinput = get_kbinput(footwin, 0 as libc::c_int != 0);
        }
        while bracketed_paste {
            get_kbinput(footwin, 0 as libc::c_int != 0);
        }
        let fresh1 = index;
        index = index + 1;
        letter[fresh1 as usize] = kbinput as libc::c_uchar as libc::c_char;
        if using_utf8() as libc::c_int != 0 && 0xc0 as libc::c_int <= kbinput
            && kbinput <= 0xf7 as libc::c_int
        {
            let mut extras: libc::c_int = kbinput / 16 as libc::c_int % 4 as libc::c_int
                + (if kbinput <= 0xcf as libc::c_int {
                    1 as libc::c_int
                } else {
                    0 as libc::c_int
                });
            while extras as libc::c_ulong <= waiting_keycodes()
                && {
                    let fresh2 = extras;
                    extras = extras - 1;
                    fresh2 > 0 as libc::c_int
                }
            {
                let fresh3 = index;
                index = index + 1;
                letter[fresh3
                    as usize] = get_kbinput(footwin, !withall) as libc::c_uchar
                    as libc::c_char;
            }
        }
        letter[index as usize] = '\0' as i32 as libc::c_char;
        if !(strstr(yesstr, letter.as_mut_ptr())).is_null() {
            choice = 1 as libc::c_int;
        } else if !(strstr(nostr, letter.as_mut_ptr())).is_null() {
            choice = 0 as libc::c_int;
        } else if withall as libc::c_int != 0
            && !(strstr(allstr, letter.as_mut_ptr())).is_null()
        {
            choice = 2 as libc::c_int;
        } else if !(strchr(b"Yy\0" as *const u8 as *const libc::c_char, kbinput))
            .is_null()
        {
            choice = 1 as libc::c_int;
        } else if !(strchr(b"Nn\0" as *const u8 as *const libc::c_char, kbinput))
            .is_null()
        {
            choice = 0 as libc::c_int;
        } else if withall as libc::c_int != 0
            && !(strchr(b"Aa\0" as *const u8 as *const libc::c_char, kbinput)).is_null()
        {
            choice = 2 as libc::c_int;
        }
        if choice != -(2 as libc::c_int) {
            break;
        }
        shortcut = get_shortcut(kbinput);
        function = if !shortcut.is_null() { (*shortcut).func } else { None };
        if function == Some(do_cancel as unsafe extern "C" fn() -> ()) {
            choice = -(1 as libc::c_int);
        } else if function == Some(full_refresh as unsafe extern "C" fn() -> ()) {
            full_refresh();
        } else if function == Some(do_toggle as unsafe extern "C" fn() -> ())
            && (*shortcut).toggle == NO_HELP as libc::c_int
        {
            flags[(NO_HELP as libc::c_int as libc::c_ulong)
                .wrapping_div(
                    (::std::mem::size_of::<libc::c_uint>() as libc::c_ulong)
                        .wrapping_mul(8 as libc::c_int as libc::c_ulong),
                ) as usize]
                ^= (1 as libc::c_int as libc::c_uint)
                    << (NO_HELP as libc::c_int as libc::c_ulong)
                        .wrapping_rem(
                            (::std::mem::size_of::<libc::c_uint>() as libc::c_ulong)
                                .wrapping_mul(8 as libc::c_int as libc::c_ulong),
                        );
            window_init();
            titlebar(0 as *const libc::c_char);
            focusing = 0 as libc::c_int != 0;
            edit_refresh();
            focusing = 1 as libc::c_int != 0;
        } else if kbinput == '\u{e}' as i32 || kbinput == '\u{11}' as i32 {
            choice = 0 as libc::c_int;
        } else if kbinput == '\u{19}' as i32 {
            choice = 1 as libc::c_int;
        } else if kbinput == 0o631 as libc::c_int {
            let mut mouse_x: libc::c_int = 0;
            let mut mouse_y: libc::c_int = 0;
            if get_mouseinput(&mut mouse_y, &mut mouse_x, 0 as libc::c_int != 0)
                == 0 as libc::c_int
                && wmouse_trafo(
                    footwin,
                    &mut mouse_y,
                    &mut mouse_x,
                    0 as libc::c_int != 0,
                ) as libc::c_int != 0 && mouse_x < width * 2 as libc::c_int
                && mouse_y > 0 as libc::c_int
            {
                let mut x: libc::c_int = mouse_x / width;
                let mut y: libc::c_int = mouse_y - 1 as libc::c_int;
                choice = -(2 as libc::c_int) * x * y + x - y + 1 as libc::c_int;
                if choice == 2 as libc::c_int && !withall {
                    choice = -(2 as libc::c_int);
                }
            }
        } else {
            beep();
        }
    }
    return choice;
}
