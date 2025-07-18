use ::libc;
use ::c2rust_bitfields;
extern "C" {
    pub type re_dfa_t;
    pub type ldat;
    fn beep() -> libc::c_int;
    fn curs_set(_: libc::c_int) -> libc::c_int;
    fn rpl_free(ptr: *mut libc::c_void);
    fn sprintf(_: *mut libc::c_char, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn snprintf(
        _: *mut libc::c_char,
        _: libc::c_ulong,
        _: *const libc::c_char,
        _: ...
    ) -> libc::c_int;
    fn full_refresh();
    static mut COLS: libc::c_int;
    static mut LINES: libc::c_int;
    fn dcgettext(
        __domainname: *const libc::c_char,
        __msgid: *const libc::c_char,
        __category: libc::c_int,
    ) -> *mut libc::c_char;
    static mut bracketed_paste: bool;
    static mut inhelp: bool;
    static mut title: *mut libc::c_char;
    static mut focusing: bool;
    static mut lastmessage: message_type;
    static mut answer: *mut libc::c_char;
    static mut didfind: libc::c_int;
    static mut flags: [libc::c_uint; 4];
    static mut midwin: *mut WINDOW;
    static mut editwinrows: libc::c_int;
    static mut editwincols: libc::c_int;
    static mut margin: libc::c_int;
    static mut thebar: libc::c_int;
    static mut openfile: *mut openfilestruct;
    static mut tabsize: ssize_t;
    static mut syntaxstr: *mut libc::c_char;
    static mut have_palette: bool;
    static mut currmenu: libc::c_int;
    static mut sclist: *mut keystruct;
    static mut allfuncs: *mut funcstruct;
    static mut spotlighted: bool;
    fn browser_refresh();
    fn find_and_prime_applicable_syntax();
    fn make_new_buffer();
    fn prepare_for_display();
    fn close_buffer();
    fn first_sc_for(
        menu: libc::c_int,
        function: Option::<unsafe extern "C" fn() -> ()>,
    ) -> *const keystruct;
    fn interpret(keycode: libc::c_int) -> functionptrtype;
    fn epithet_of_flag(flag: libc::c_int) -> *const libc::c_char;
    fn remove_magicline();
    fn copy_of(string: *const libc::c_char) -> *mut libc::c_char;
    fn make_new_node(prevnode: *mut linestruct) -> *mut linestruct;
    fn nmalloc(howmuch: size_t) -> *mut libc::c_void;
    fn break_line(
        textstart: *const libc::c_char,
        goal: ssize_t,
        snap_at_nl: bool,
    ) -> ssize_t;
    fn mallocstrcpy(
        dest: *mut libc::c_char,
        src: *const libc::c_char,
    ) -> *mut libc::c_char;
    fn edit_refresh();
    fn titlebar(path: *const libc::c_char);
    fn bottombars(menu: libc::c_int);
    fn blank_statusbar();
    fn window_init();
    fn unbound_key(code: libc::c_int);
    fn do_exit();
    fn get_mouseinput(
        mouse_y: *mut libc::c_int,
        mouse_x: *mut libc::c_int,
        allow_shortcuts: bool,
    ) -> libc::c_int;
    fn implant(string: *const libc::c_char);
    fn do_findnext();
    fn do_findprevious();
    fn do_search_forward();
    fn do_search_backward();
    fn to_last_line();
    fn to_first_line();
    fn do_page_down();
    fn do_page_up();
    fn do_scroll_down();
    fn do_down();
    fn do_scroll_up();
    fn do_up();
    fn do_right();
    fn do_left();
    fn get_kbinput(win: *mut WINDOW, showcursor: bool) -> libc::c_int;
    fn measured_copy(string: *const libc::c_char, count: size_t) -> *mut libc::c_char;
    fn do_toggle();
    fn memcpy(
        _: *mut libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn strcpy(_: *mut libc::c_char, _: *const libc::c_char) -> *mut libc::c_char;
    fn strcat(_: *mut libc::c_char, _: *const libc::c_char) -> *mut libc::c_char;
    fn strstr(_: *const libc::c_char, _: *const libc::c_char) -> *mut libc::c_char;
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
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
static mut help_text: *mut libc::c_char = 0 as *const libc::c_char as *mut libc::c_char;
static mut start_of_body: *const libc::c_char = 0 as *const libc::c_char;
static mut end_of_intro: *mut libc::c_char = 0 as *const libc::c_char
    as *mut libc::c_char;
static mut location: size_t = 0;
pub unsafe extern "C" fn help_init() {
    let mut allocsize: size_t = 0 as libc::c_int as size_t;
    let mut htx: [*const libc::c_char; 3] = [0 as *const libc::c_char; 3];
    let mut f: *const funcstruct = 0 as *const funcstruct;
    let mut s: *const keystruct = 0 as *const keystruct;
    let mut ptr: *mut libc::c_char = 0 as *mut libc::c_char;
    if currmenu
        & ((1 as libc::c_int) << 1 as libc::c_int
            | (1 as libc::c_int) << 2 as libc::c_int
            | (1 as libc::c_int) << 3 as libc::c_int) != 0
    {
        htx[0 as libc::c_int
            as usize] = b"Search Command Help Text\n\n Enter the words or characters you would like to search for, and then press Enter.  If there is a match for the text you entered, the screen will be updated to the location of the nearest match for the search string.\n\n The previous search string will be shown in brackets after the search prompt.  Hitting Enter without entering any text will perform the previous search.  \0"
            as *const u8 as *const libc::c_char;
        htx[1 as libc::c_int
            as usize] = b"If you have selected text with the mark and then search to replace, only matches in the selected text will be replaced.\n\n The following function keys are available in Search mode:\n\n\0"
            as *const u8 as *const libc::c_char;
        htx[2 as libc::c_int as usize] = 0 as *const libc::c_char;
    } else if currmenu == (1 as libc::c_int) << 4 as libc::c_int {
        htx[0 as libc::c_int
            as usize] = b"Go To Line Help Text\n\n Enter the line number that you wish to go to and hit Enter.  If there are fewer lines of text than the number you entered, you will be brought to the last line of the file.\n\n The following function keys are available in Go To Line mode:\n\n\0"
            as *const u8 as *const libc::c_char;
        htx[1 as libc::c_int as usize] = 0 as *const libc::c_char;
        htx[2 as libc::c_int as usize] = 0 as *const libc::c_char;
    } else if currmenu == (1 as libc::c_int) << 6 as libc::c_int {
        htx[0 as libc::c_int
            as usize] = b"Insert File Help Text\n\n Type in the name of a file to be inserted into the current file buffer at the current cursor location.\n\n If you have compiled nano with multiple file buffer support, and enable multiple file buffers with the -F or --multibuffer command line flags, the Meta-F toggle, or a nanorc file, inserting a file will cause it to be loaded into a separate buffer (use Meta-< and > to switch between file buffers).  \0"
            as *const u8 as *const libc::c_char;
        htx[1 as libc::c_int
            as usize] = b"If you need another blank buffer, do not enter any filename, or type in a nonexistent filename at the prompt and press Enter.\n\n The following function keys are available in Insert File mode:\n\n\0"
            as *const u8 as *const libc::c_char;
        htx[2 as libc::c_int as usize] = 0 as *const libc::c_char;
    } else if currmenu == (1 as libc::c_int) << 5 as libc::c_int {
        htx[0 as libc::c_int
            as usize] = b"Write File Help Text\n\n Type the name that you wish to save the current file as and press Enter to save the file.\n\n If you have selected text with the mark, you will be prompted to save only the selected portion to a separate file.  To reduce the chance of overwriting the current file with just a portion of it, the current filename is not the default in this mode.\n\n The following function keys are available in Write File mode:\n\n\0"
            as *const u8 as *const libc::c_char;
        htx[1 as libc::c_int as usize] = 0 as *const libc::c_char;
        htx[2 as libc::c_int as usize] = 0 as *const libc::c_char;
    } else if currmenu == (1 as libc::c_int) << 10 as libc::c_int {
        htx[0 as libc::c_int
            as usize] = b"File Browser Help Text\n\n The file browser is used to visually browse the directory structure to select a file for reading or writing.  You may use the arrow keys or Page Up/Down to browse through the files, and S or Enter to choose the selected file or enter the selected directory.  To move up one level, select the directory called \"..\" at the top of the file list.\n\n The following function keys are available in the file browser:\n\n\0"
            as *const u8 as *const libc::c_char;
        htx[1 as libc::c_int as usize] = 0 as *const libc::c_char;
        htx[2 as libc::c_int as usize] = 0 as *const libc::c_char;
    } else if currmenu == (1 as libc::c_int) << 11 as libc::c_int {
        htx[0 as libc::c_int
            as usize] = b"Browser Search Command Help Text\n\n Enter the words or characters you would like to search for, and then press Enter.  If there is a match for the text you entered, the screen will be updated to the location of the nearest match for the search string.\n\n The previous search string will be shown in brackets after the search prompt.  Hitting Enter without entering any text will perform the previous search.\n\n\0"
            as *const u8 as *const libc::c_char;
        htx[1 as libc::c_int
            as usize] = b" The following function keys are available in Browser Search mode:\n\n\0"
            as *const u8 as *const libc::c_char;
        htx[2 as libc::c_int as usize] = 0 as *const libc::c_char;
    } else if currmenu == (1 as libc::c_int) << 12 as libc::c_int {
        htx[0 as libc::c_int
            as usize] = b"Browser Go To Directory Help Text\n\n Enter the name of the directory you would like to browse to.\n\n If tab completion has not been disabled, you can use the Tab key to (attempt to) automatically complete the directory name.\n\n The following function keys are available in Browser Go To Directory mode:\n\n\0"
            as *const u8 as *const libc::c_char;
        htx[1 as libc::c_int as usize] = 0 as *const libc::c_char;
        htx[2 as libc::c_int as usize] = 0 as *const libc::c_char;
    } else if currmenu == (1 as libc::c_int) << 9 as libc::c_int {
        htx[0 as libc::c_int
            as usize] = b"Spell Check Help Text\n\n The spell checker checks the spelling of all text in the current file.  When an unknown word is encountered, it is highlighted and a replacement can be edited.  It will then prompt to replace every instance of the given misspelled word in the current file, or, if you have selected text with the mark, in the selected text.\n\n The following function keys are available in Spell Check mode:\n\n\0"
            as *const u8 as *const libc::c_char;
        htx[1 as libc::c_int as usize] = 0 as *const libc::c_char;
        htx[2 as libc::c_int as usize] = 0 as *const libc::c_char;
    } else if currmenu == (1 as libc::c_int) << 7 as libc::c_int {
        htx[0 as libc::c_int
            as usize] = b"Execute Command Help Text\n\n This mode allows you to insert the output of a command run by the shell into the current buffer (or into a new buffer).  If the command is preceded by '|' (the pipe symbol), the current contents of the buffer (or marked region) will be piped to the command.  \0"
            as *const u8 as *const libc::c_char;
        htx[1 as libc::c_int
            as usize] = b"If you just need another blank buffer, do not enter any command.\n\n You can also pick one of four tools, or cut a large piece of the buffer, or put the editor to sleep.\n\n\0"
            as *const u8 as *const libc::c_char;
        htx[2 as libc::c_int
            as usize] = b" The following function keys are available in Execute Command mode:\n\n\0"
            as *const u8 as *const libc::c_char;
    } else if currmenu == (1 as libc::c_int) << 14 as libc::c_int {
        htx[0 as libc::c_int
            as usize] = b"=== Linter ===\n\n In this mode, the status bar shows an error message or warning, and the cursor is put at the corresponding position in the file.  With PageUp and PageDown you can switch to earlier and later messages.\n\n\0"
            as *const u8 as *const libc::c_char;
        htx[1 as libc::c_int
            as usize] = b" The following function keys are available in Linter mode:\n\n\0"
            as *const u8 as *const libc::c_char;
        htx[2 as libc::c_int as usize] = 0 as *const libc::c_char;
    } else {
        htx[0 as libc::c_int
            as usize] = b"Main nano help text\n\n The nano editor is designed to emulate the functionality and ease-of-use of the UW Pico text editor.  There are four main sections of the editor.  The top line shows the program version, the current filename being edited, and whether or not the file has been modified.  Next is the main editor window showing the file being edited.  The status line is the third line from the bottom and shows important messages.  \0"
            as *const u8 as *const libc::c_char;
        htx[1 as libc::c_int
            as usize] = b"The bottom two lines show the most commonly used shortcuts in the editor.\n\n Shortcuts are written as follows: Control-key sequences are notated with a '^' and can be entered either by using the Ctrl key or pressing the Esc key twice.  Meta-key sequences are notated with 'M-' and can be entered using either the Alt, Cmd, or Esc key, depending on your keyboard setup.  \0"
            as *const u8 as *const libc::c_char;
        htx[2 as libc::c_int
            as usize] = b"Also, pressing Esc twice and then typing a three-digit decimal number from 000 to 255 will enter the character with the corresponding value.  The following keystrokes are available in the main editor window.  Alternative keys are shown in parentheses:\n\n\0"
            as *const u8 as *const libc::c_char;
    }
    htx[0 as libc::c_int
        as usize] = dcgettext(
        0 as *const libc::c_char,
        htx[0 as libc::c_int as usize],
        5 as libc::c_int,
    );
    if !(htx[1 as libc::c_int as usize]).is_null() {
        htx[1 as libc::c_int
            as usize] = dcgettext(
            0 as *const libc::c_char,
            htx[1 as libc::c_int as usize],
            5 as libc::c_int,
        );
    }
    if !(htx[2 as libc::c_int as usize]).is_null() {
        htx[2 as libc::c_int
            as usize] = dcgettext(
            0 as *const libc::c_char,
            htx[2 as libc::c_int as usize],
            5 as libc::c_int,
        );
    }
    allocsize = (allocsize as libc::c_ulong)
        .wrapping_add(strlen(htx[0 as libc::c_int as usize])) as size_t as size_t;
    if !(htx[1 as libc::c_int as usize]).is_null() {
        allocsize = (allocsize as libc::c_ulong)
            .wrapping_add(strlen(htx[1 as libc::c_int as usize])) as size_t as size_t;
    }
    if !(htx[2 as libc::c_int as usize]).is_null() {
        allocsize = (allocsize as libc::c_ulong)
            .wrapping_add(strlen(htx[2 as libc::c_int as usize])) as size_t as size_t;
    }
    f = allfuncs;
    while !f.is_null() {
        if (*f).menus & currmenu != 0 {
            allocsize = (allocsize as libc::c_ulong)
                .wrapping_add(
                    (strlen(
                        dcgettext(
                            0 as *const libc::c_char,
                            (*f).phrase,
                            5 as libc::c_int,
                        ),
                    ))
                        .wrapping_add(21 as libc::c_int as libc::c_ulong),
                ) as size_t as size_t;
        }
        f = (*f).next;
    }
    if currmenu == (1 as libc::c_int) << 0 as libc::c_int {
        let mut onoff_len: size_t = strlen(
            dcgettext(
                0 as *const libc::c_char,
                b"enable/disable\0" as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ),
        );
        s = sclist;
        while !s.is_null() {
            if (*s).func == Some(do_toggle as unsafe extern "C" fn() -> ()) {
                allocsize = (allocsize as libc::c_ulong)
                    .wrapping_add(
                        (strlen(
                            dcgettext(
                                0 as *const libc::c_char,
                                epithet_of_flag((*s).toggle),
                                5 as libc::c_int,
                            ),
                        ))
                            .wrapping_add(onoff_len)
                            .wrapping_add(9 as libc::c_int as libc::c_ulong),
                    ) as size_t as size_t;
            }
            s = (*s).next;
        }
    }
    help_text = nmalloc(allocsize.wrapping_add(1 as libc::c_int as libc::c_ulong))
        as *mut libc::c_char;
    strcpy(help_text, htx[0 as libc::c_int as usize]);
    if !(htx[1 as libc::c_int as usize]).is_null() {
        strcat(help_text, htx[1 as libc::c_int as usize]);
    }
    if !(htx[2 as libc::c_int as usize]).is_null() {
        strcat(help_text, htx[2 as libc::c_int as usize]);
    }
    end_of_intro = help_text.offset(strlen(help_text) as isize);
    ptr = end_of_intro;
    f = allfuncs;
    while !f.is_null() {
        let mut tally: libc::c_int = 0 as libc::c_int;
        if !((*f).menus & currmenu == 0 as libc::c_int) {
            s = sclist;
            while !s.is_null() {
                if (*s).menus & currmenu != 0 && (*s).func == (*f).func
                    && *((*s).keystr).offset(0 as libc::c_int as isize) as libc::c_int
                        != 0
                {
                    tally += 1;
                    if tally == 1 as libc::c_int {
                        sprintf(
                            ptr,
                            b"%s                \0" as *const u8 as *const libc::c_char,
                            (*s).keystr,
                        );
                        ptr = ptr
                            .offset(
                                (if !(strstr(
                                    (*s).keystr,
                                    b"\xE2\0" as *const u8 as *const libc::c_char,
                                ))
                                    .is_null()
                                {
                                    9 as libc::c_int
                                } else {
                                    7 as libc::c_int
                                }) as isize,
                            );
                    } else {
                        sprintf(
                            ptr,
                            b"(%s)       \0" as *const u8 as *const libc::c_char,
                            (*s).keystr,
                        );
                        ptr = ptr
                            .offset(
                                (if !(strstr(
                                    (*s).keystr,
                                    b"\xE2\0" as *const u8 as *const libc::c_char,
                                ))
                                    .is_null()
                                {
                                    12 as libc::c_int
                                } else {
                                    10 as libc::c_int
                                }) as isize,
                            );
                        break;
                    }
                }
                s = (*s).next;
            }
            if tally == 0 as libc::c_int {
                ptr = ptr
                    .offset(
                        sprintf(ptr, b"\t\t \0" as *const u8 as *const libc::c_char)
                            as isize,
                    );
            } else if tally == 1 as libc::c_int {
                ptr = ptr.offset(10 as libc::c_int as isize);
            }
            ptr = ptr
                .offset(
                    sprintf(
                        ptr,
                        b"%s\n\0" as *const u8 as *const libc::c_char,
                        dcgettext(
                            0 as *const libc::c_char,
                            (*f).phrase,
                            5 as libc::c_int,
                        ),
                    ) as isize,
                );
            if (*f).blank_after {
                ptr = ptr
                    .offset(
                        sprintf(ptr, b"\n\0" as *const u8 as *const libc::c_char)
                            as isize,
                    );
            }
        }
        f = (*f).next;
    }
    if currmenu == (1 as libc::c_int) << 0 as libc::c_int {
        let mut maximum: libc::c_int = 0 as libc::c_int;
        let mut counter: libc::c_int = 0 as libc::c_int;
        s = sclist;
        while !s.is_null() {
            maximum = if (*s).toggle != 0 && (*s).ordinal > maximum {
                (*s).ordinal
            } else {
                maximum
            };
            s = (*s).next;
        }
        while counter < maximum {
            counter += 1;
            counter;
            s = sclist;
            while !s.is_null() {
                if (*s).toggle != 0 && (*s).ordinal == counter {
                    ptr = ptr
                        .offset(
                            sprintf(
                                ptr,
                                b"%s\t\t %s %s\n\0" as *const u8 as *const libc::c_char,
                                if (*s).menus & (1 as libc::c_int) << 0 as libc::c_int != 0
                                {
                                    (*s).keystr
                                } else {
                                    b"\0" as *const u8 as *const libc::c_char
                                },
                                dcgettext(
                                    0 as *const libc::c_char,
                                    epithet_of_flag((*s).toggle),
                                    5 as libc::c_int,
                                ),
                                dcgettext(
                                    0 as *const libc::c_char,
                                    b"enable/disable\0" as *const u8 as *const libc::c_char,
                                    5 as libc::c_int,
                                ),
                            ) as isize,
                        );
                    if (*s).toggle == NO_SYNTAX as libc::c_int {
                        ptr = ptr
                            .offset(
                                sprintf(ptr, b"\n\0" as *const u8 as *const libc::c_char)
                                    as isize,
                            );
                    }
                    break;
                } else {
                    s = (*s).next;
                }
            }
        }
    }
}
pub unsafe extern "C" fn wrap_help_text_into_buffer() {
    let mut wrapping_point: size_t = ((if COLS < 40 as libc::c_int {
        40 as libc::c_int
    } else {
        if COLS > 74 as libc::c_int { 74 as libc::c_int } else { COLS }
    }) - thebar) as size_t;
    let mut ptr: *const libc::c_char = start_of_body;
    let mut sum: size_t = 0 as libc::c_int as size_t;
    make_new_buffer();
    if (flags[(MINIBAR as libc::c_int as libc::c_ulong)
        .wrapping_div(
            (::std::mem::size_of::<libc::c_uint>() as libc::c_ulong)
                .wrapping_mul(8 as libc::c_int as libc::c_ulong),
        ) as usize]
        & (1 as libc::c_int as libc::c_uint)
            << (MINIBAR as libc::c_int as libc::c_ulong)
                .wrapping_rem(
                    (::std::mem::size_of::<libc::c_uint>() as libc::c_ulong)
                        .wrapping_mul(8 as libc::c_int as libc::c_ulong),
                ) != 0 as libc::c_int as libc::c_uint
        || !(flags[(EMPTY_LINE as libc::c_int as libc::c_ulong)
            .wrapping_div(
                (::std::mem::size_of::<libc::c_uint>() as libc::c_ulong)
                    .wrapping_mul(8 as libc::c_int as libc::c_ulong),
            ) as usize]
            & (1 as libc::c_int as libc::c_uint)
                << (EMPTY_LINE as libc::c_int as libc::c_ulong)
                    .wrapping_rem(
                        (::std::mem::size_of::<libc::c_uint>() as libc::c_ulong)
                            .wrapping_mul(8 as libc::c_int as libc::c_ulong),
                    ) != 0 as libc::c_int as libc::c_uint)) && LINES > 6 as libc::c_int
    {
        (*(*openfile).current)
            .data = mallocstrcpy(
            (*(*openfile).current).data,
            b" \0" as *const u8 as *const libc::c_char,
        );
        (*(*openfile).current).next = make_new_node((*openfile).current);
        (*openfile).current = (*(*openfile).current).next;
    }
    while *ptr as libc::c_int != '\0' as i32 {
        let mut length: libc::c_int = 0;
        let mut shim: libc::c_int = 0;
        let mut oneline: *mut libc::c_char = 0 as *mut libc::c_char;
        if ptr == end_of_intro as *const libc::c_char {
            wrapping_point = ((if COLS < 40 as libc::c_int {
                40 as libc::c_int
            } else {
                COLS
            }) - thebar) as size_t;
        }
        if ptr < end_of_intro as *const libc::c_char
            || *ptr.offset(-(1 as libc::c_int as isize)) as libc::c_int == '\n' as i32
        {
            length = break_line(ptr, wrapping_point as ssize_t, 1 as libc::c_int != 0)
                as libc::c_int;
            oneline = nmalloc((length + 1 as libc::c_int) as size_t)
                as *mut libc::c_char;
            shim = if *ptr.offset(length as isize).offset(-(1 as libc::c_int as isize))
                as libc::c_int == ' ' as i32
            {
                0 as libc::c_int
            } else {
                1 as libc::c_int
            };
            snprintf(
                oneline,
                (length + shim) as libc::c_ulong,
                b"%s\0" as *const u8 as *const libc::c_char,
                ptr,
            );
        } else {
            length = break_line(
                ptr,
                ((if COLS < 40 as libc::c_int {
                    22 as libc::c_int
                } else {
                    COLS - 18 as libc::c_int
                }) - thebar) as ssize_t,
                1 as libc::c_int != 0,
            ) as libc::c_int;
            oneline = nmalloc((length + 5 as libc::c_int) as size_t)
                as *mut libc::c_char;
            snprintf(
                oneline,
                (length + 5 as libc::c_int) as libc::c_ulong,
                b"\t\t  %s\0" as *const u8 as *const libc::c_char,
                ptr,
            );
        }
        rpl_free((*(*openfile).current).data as *mut libc::c_void);
        (*(*openfile).current).data = oneline;
        ptr = ptr.offset(length as isize);
        if *ptr as libc::c_int != '\n' as i32 {
            ptr = ptr.offset(-1);
            ptr;
        }
        loop {
            (*(*openfile).current).next = make_new_node((*openfile).current);
            (*openfile).current = (*(*openfile).current).next;
            (*(*openfile).current)
                .data = copy_of(b"\0" as *const u8 as *const libc::c_char);
            ptr = ptr.offset(1);
            if !(*ptr as libc::c_int == '\n' as i32) {
                break;
            }
        }
    }
    (*openfile).filebot = (*openfile).current;
    (*openfile).current = (*openfile).filetop;
    remove_magicline();
    find_and_prime_applicable_syntax();
    prepare_for_display();
    loop {
        sum = (sum as libc::c_ulong).wrapping_add(strlen((*(*openfile).current).data))
            as size_t as size_t;
        if sum > location {
            break;
        }
        (*openfile).current = (*(*openfile).current).next;
    }
    (*openfile).edittop = (*openfile).current;
}
pub unsafe extern "C" fn show_help() {
    let mut kbinput: libc::c_int = -(1 as libc::c_int);
    let mut function: functionptrtype = None;
    let mut oldmenu: libc::c_int = currmenu;
    let mut was_margin: libc::c_int = margin;
    let mut was_tabsize: ssize_t = tabsize;
    let mut was_syntax: *mut libc::c_char = syntaxstr;
    let mut saved_answer: *mut libc::c_char = if !answer.is_null() {
        copy_of(answer)
    } else {
        0 as *mut libc::c_char
    };
    let mut stash: [libc::c_uint; 4] = [0; 4];
    let mut line: *mut linestruct = 0 as *mut linestruct;
    let mut length: libc::c_int = 0;
    memcpy(
        stash.as_mut_ptr() as *mut libc::c_void,
        flags.as_mut_ptr() as *const libc::c_void,
        ::std::mem::size_of::<[libc::c_uint; 4]>() as libc::c_ulong,
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
        || flags[(ZERO as libc::c_int as libc::c_ulong)
            .wrapping_div(
                (::std::mem::size_of::<libc::c_uint>() as libc::c_ulong)
                    .wrapping_mul(8 as libc::c_int as libc::c_ulong),
            ) as usize]
            & (1 as libc::c_int as libc::c_uint)
                << (ZERO as libc::c_int as libc::c_ulong)
                    .wrapping_rem(
                        (::std::mem::size_of::<libc::c_uint>() as libc::c_ulong)
                            .wrapping_mul(8 as libc::c_int as libc::c_ulong),
                    ) != 0 as libc::c_int as libc::c_uint
    {
        flags[(NO_HELP as libc::c_int as libc::c_ulong)
            .wrapping_div(
                (::std::mem::size_of::<libc::c_uint>() as libc::c_ulong)
                    .wrapping_mul(8 as libc::c_int as libc::c_ulong),
            ) as usize]
            &= !((1 as libc::c_int as libc::c_uint)
                << (NO_HELP as libc::c_int as libc::c_ulong)
                    .wrapping_rem(
                        (::std::mem::size_of::<libc::c_uint>() as libc::c_ulong)
                            .wrapping_mul(8 as libc::c_int as libc::c_ulong),
                    ));
        flags[(ZERO as libc::c_int as libc::c_ulong)
            .wrapping_div(
                (::std::mem::size_of::<libc::c_uint>() as libc::c_ulong)
                    .wrapping_mul(8 as libc::c_int as libc::c_ulong),
            ) as usize]
            &= !((1 as libc::c_int as libc::c_uint)
                << (ZERO as libc::c_int as libc::c_ulong)
                    .wrapping_rem(
                        (::std::mem::size_of::<libc::c_uint>() as libc::c_ulong)
                            .wrapping_mul(8 as libc::c_int as libc::c_ulong),
                    ));
        window_init();
    } else {
        blank_statusbar();
    }
    flags[(BACKWARDS_SEARCH as libc::c_int as libc::c_ulong)
        .wrapping_div(
            (::std::mem::size_of::<libc::c_uint>() as libc::c_ulong)
                .wrapping_mul(8 as libc::c_int as libc::c_ulong),
        ) as usize]
        &= !((1 as libc::c_int as libc::c_uint)
            << (BACKWARDS_SEARCH as libc::c_int as libc::c_ulong)
                .wrapping_rem(
                    (::std::mem::size_of::<libc::c_uint>() as libc::c_ulong)
                        .wrapping_mul(8 as libc::c_int as libc::c_ulong),
                ));
    flags[(CASE_SENSITIVE as libc::c_int as libc::c_ulong)
        .wrapping_div(
            (::std::mem::size_of::<libc::c_uint>() as libc::c_ulong)
                .wrapping_mul(8 as libc::c_int as libc::c_ulong),
        ) as usize]
        &= !((1 as libc::c_int as libc::c_uint)
            << (CASE_SENSITIVE as libc::c_int as libc::c_ulong)
                .wrapping_rem(
                    (::std::mem::size_of::<libc::c_uint>() as libc::c_ulong)
                        .wrapping_mul(8 as libc::c_int as libc::c_ulong),
                ));
    flags[(USE_REGEXP as libc::c_int as libc::c_ulong)
        .wrapping_div(
            (::std::mem::size_of::<libc::c_uint>() as libc::c_ulong)
                .wrapping_mul(8 as libc::c_int as libc::c_ulong),
        ) as usize]
        &= !((1 as libc::c_int as libc::c_uint)
            << (USE_REGEXP as libc::c_int as libc::c_ulong)
                .wrapping_rem(
                    (::std::mem::size_of::<libc::c_uint>() as libc::c_ulong)
                        .wrapping_mul(8 as libc::c_int as libc::c_ulong),
                ));
    flags[(WHITESPACE_DISPLAY as libc::c_int as libc::c_ulong)
        .wrapping_div(
            (::std::mem::size_of::<libc::c_uint>() as libc::c_ulong)
                .wrapping_mul(8 as libc::c_int as libc::c_ulong),
        ) as usize]
        &= !((1 as libc::c_int as libc::c_uint)
            << (WHITESPACE_DISPLAY as libc::c_int as libc::c_ulong)
                .wrapping_rem(
                    (::std::mem::size_of::<libc::c_uint>() as libc::c_ulong)
                        .wrapping_mul(8 as libc::c_int as libc::c_ulong),
                ));
    editwincols = COLS - thebar;
    margin = 0 as libc::c_int;
    tabsize = 8 as libc::c_int as ssize_t;
    syntaxstr = b"nanohelp\0" as *const u8 as *const libc::c_char as *mut libc::c_char;
    curs_set(0 as libc::c_int);
    help_init();
    inhelp = 1 as libc::c_int != 0;
    location = 0 as libc::c_int as size_t;
    didfind = 0 as libc::c_int;
    bottombars((1 as libc::c_int) << 8 as libc::c_int);
    length = break_line(
        help_text,
        (!(0 as libc::c_int as size_t) >> 1 as libc::c_int) as ssize_t,
        1 as libc::c_int != 0,
    ) as libc::c_int;
    title = measured_copy(help_text, length as size_t);
    titlebar(title);
    start_of_body = help_text.offset(length as isize);
    while *start_of_body as libc::c_int == '\n' as i32 {
        start_of_body = start_of_body.offset(1);
        start_of_body;
    }
    wrap_help_text_into_buffer();
    edit_refresh();
    loop {
        lastmessage = VACUUM;
        focusing = 1 as libc::c_int != 0;
        kbinput = get_kbinput(
            midwin,
            didfind == 1 as libc::c_int
                || flags[(SHOW_CURSOR as libc::c_int as libc::c_ulong)
                    .wrapping_div(
                        (::std::mem::size_of::<libc::c_uint>() as libc::c_ulong)
                            .wrapping_mul(8 as libc::c_int as libc::c_ulong),
                    ) as usize]
                    & (1 as libc::c_int as libc::c_uint)
                        << (SHOW_CURSOR as libc::c_int as libc::c_ulong)
                            .wrapping_rem(
                                (::std::mem::size_of::<libc::c_uint>() as libc::c_ulong)
                                    .wrapping_mul(8 as libc::c_int as libc::c_ulong),
                            ) != 0 as libc::c_int as libc::c_uint,
        );
        didfind = 0 as libc::c_int;
        spotlighted = 0 as libc::c_int != 0;
        if bracketed_paste as libc::c_int != 0 || kbinput == 0x4fb as libc::c_int {
            beep();
        } else {
            function = interpret(kbinput);
            if function == Some(full_refresh as unsafe extern "C" fn() -> ()) {
                full_refresh();
            } else if flags[(SHOW_CURSOR as libc::c_int as libc::c_ulong)
                .wrapping_div(
                    (::std::mem::size_of::<libc::c_uint>() as libc::c_ulong)
                        .wrapping_mul(8 as libc::c_int as libc::c_ulong),
                ) as usize]
                & (1 as libc::c_int as libc::c_uint)
                    << (SHOW_CURSOR as libc::c_int as libc::c_ulong)
                        .wrapping_rem(
                            (::std::mem::size_of::<libc::c_uint>() as libc::c_ulong)
                                .wrapping_mul(8 as libc::c_int as libc::c_ulong),
                        ) != 0 as libc::c_int as libc::c_uint
                && (function == Some(do_left as unsafe extern "C" fn() -> ())
                    || function == Some(do_right as unsafe extern "C" fn() -> ())
                    || function == Some(do_up as unsafe extern "C" fn() -> ())
                    || function == Some(do_down as unsafe extern "C" fn() -> ()))
            {
                function.unwrap()();
            } else if function == Some(do_up as unsafe extern "C" fn() -> ())
                || function == Some(do_scroll_up as unsafe extern "C" fn() -> ())
            {
                do_scroll_up();
            } else if function == Some(do_down as unsafe extern "C" fn() -> ())
                || function == Some(do_scroll_down as unsafe extern "C" fn() -> ())
            {
                if ((*(*openfile).edittop).lineno + editwinrows as libc::c_long
                    - 1 as libc::c_int as libc::c_long) < (*(*openfile).filebot).lineno
                {
                    do_scroll_down();
                }
            } else if function == Some(do_page_up as unsafe extern "C" fn() -> ())
                || function == Some(do_page_down as unsafe extern "C" fn() -> ())
                || function == Some(to_first_line as unsafe extern "C" fn() -> ())
                || function == Some(to_last_line as unsafe extern "C" fn() -> ())
            {
                function.unwrap()();
            } else if function
                == Some(do_search_backward as unsafe extern "C" fn() -> ())
                || function == Some(do_search_forward as unsafe extern "C" fn() -> ())
                || function == Some(do_findprevious as unsafe extern "C" fn() -> ())
                || function == Some(do_findnext as unsafe extern "C" fn() -> ())
            {
                function.unwrap()();
                bottombars((1 as libc::c_int) << 8 as libc::c_int);
            } else if function
                == ::std::mem::transmute::<
                    Option::<unsafe extern "C" fn(*const libc::c_char) -> ()>,
                    functionptrtype,
                >(Some(implant as unsafe extern "C" fn(*const libc::c_char) -> ()))
            {
                implant(
                    (*first_sc_for((1 as libc::c_int) << 8 as libc::c_int, function))
                        .expansion,
                );
            } else if kbinput == 0o631 as libc::c_int {
                let mut dummy_row: libc::c_int = 0;
                let mut dummy_col: libc::c_int = 0;
                get_mouseinput(&mut dummy_row, &mut dummy_col, 1 as libc::c_int != 0);
            } else if !(kbinput == -(2 as libc::c_int)) {
                if function == Some(do_exit as unsafe extern "C" fn() -> ()) {
                    break;
                }
                unbound_key(kbinput);
            }
            edit_refresh();
            location = 0 as libc::c_int as size_t;
            line = (*openfile).filetop;
            while line != (*openfile).edittop {
                location = (location as libc::c_ulong).wrapping_add(strlen((*line).data))
                    as size_t as size_t;
                line = (*line).next;
            }
        }
    }
    close_buffer();
    memcpy(
        flags.as_mut_ptr() as *mut libc::c_void,
        stash.as_mut_ptr() as *const libc::c_void,
        ::std::mem::size_of::<[libc::c_uint; 4]>() as libc::c_ulong,
    );
    margin = was_margin;
    editwincols = COLS - margin - thebar;
    tabsize = was_tabsize;
    syntaxstr = was_syntax;
    have_palette = 0 as libc::c_int != 0;
    rpl_free(title as *mut libc::c_void);
    title = 0 as *mut libc::c_char;
    rpl_free(answer as *mut libc::c_void);
    answer = saved_answer;
    rpl_free(help_text as *mut libc::c_void);
    inhelp = 0 as libc::c_int != 0;
    curs_set(0 as libc::c_int);
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
        || flags[(ZERO as libc::c_int as libc::c_ulong)
            .wrapping_div(
                (::std::mem::size_of::<libc::c_uint>() as libc::c_ulong)
                    .wrapping_mul(8 as libc::c_int as libc::c_ulong),
            ) as usize]
            & (1 as libc::c_int as libc::c_uint)
                << (ZERO as libc::c_int as libc::c_ulong)
                    .wrapping_rem(
                        (::std::mem::size_of::<libc::c_uint>() as libc::c_ulong)
                            .wrapping_mul(8 as libc::c_int as libc::c_ulong),
                    ) != 0 as libc::c_int as libc::c_uint
    {
        window_init();
    } else {
        blank_statusbar();
    }
    bottombars(oldmenu);
    if oldmenu
        & ((1 as libc::c_int) << 10 as libc::c_int
            | (1 as libc::c_int) << 11 as libc::c_int
            | (1 as libc::c_int) << 12 as libc::c_int) != 0
    {
        browser_refresh();
    } else {
        titlebar(0 as *const libc::c_char);
        edit_refresh();
    };
}
pub unsafe extern "C" fn do_help() {
    show_help();
}
