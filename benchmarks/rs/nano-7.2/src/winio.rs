use ::libc;
use ::c2rust_bitfields;
extern "C" {
    pub type re_dfa_t;
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    pub type ldat;
    fn beep() -> libc::c_int;
    fn curs_set(_: libc::c_int) -> libc::c_int;
    fn doupdate() -> libc::c_int;
    fn halfdelay(_: libc::c_int) -> libc::c_int;
    fn has_colors() -> bool;
    fn isendwin() -> bool;
    fn keypad(_: *mut WINDOW, _: bool) -> libc::c_int;
    fn mvwprintw(
        _: *mut WINDOW,
        _: libc::c_int,
        _: libc::c_int,
        _: *const libc::c_char,
        _: ...
    ) -> libc::c_int;
    fn napms(_: libc::c_int) -> libc::c_int;
    fn nodelay(_: *mut WINDOW, _: bool) -> libc::c_int;
    fn raw() -> libc::c_int;
    fn scrollok(_: *mut WINDOW, _: bool) -> libc::c_int;
    fn waddch(_: *mut WINDOW, _: chtype) -> libc::c_int;
    fn waddnstr(_: *mut WINDOW, _: *const libc::c_char, _: libc::c_int) -> libc::c_int;
    fn wattr_on(_: *mut WINDOW, _: attr_t, _: *mut libc::c_void) -> libc::c_int;
    fn wattr_off(_: *mut WINDOW, _: attr_t, _: *mut libc::c_void) -> libc::c_int;
    fn rpl_regexec(
        __preg: *const regex_t,
        __String: *const libc::c_char,
        __nmatch: size_t,
        __pmatch: *mut regmatch_t,
        __eflags: libc::c_int,
    ) -> libc::c_int;
    fn __ctype_get_mb_cur_max() -> size_t;
    fn wctomb(__s: *mut libc::c_char, __wchar: wchar_t) -> libc::c_int;
    fn rpl_free(ptr: *mut libc::c_void);
    static mut stdout: *mut FILE;
    static mut stderr: *mut FILE;
    fn fflush(__stream: *mut FILE) -> libc::c_int;
    fn fprintf(_: *mut FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
    fn sprintf(_: *mut libc::c_char, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn vsnprintf(
        _: *mut libc::c_char,
        _: libc::c_ulong,
        _: *const libc::c_char,
        _: ::std::ffi::VaList,
    ) -> libc::c_int;
    fn wcwidth(__c: wchar_t) -> libc::c_int;
    fn wclrtoeol(_: *mut WINDOW) -> libc::c_int;
    fn wgetch(_: *mut WINDOW) -> libc::c_int;
    fn wmove(_: *mut WINDOW, _: libc::c_int, _: libc::c_int) -> libc::c_int;
    fn wnoutrefresh(_: *mut WINDOW) -> libc::c_int;
    fn wprintw(_: *mut WINDOW, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn wredrawln(_: *mut WINDOW, _: libc::c_int, _: libc::c_int) -> libc::c_int;
    fn wrefresh(_: *mut WINDOW) -> libc::c_int;
    fn wscrl(_: *mut WINDOW, _: libc::c_int) -> libc::c_int;
    static mut curscr: *mut WINDOW;
    static mut COLS: libc::c_int;
    static mut LINES: libc::c_int;
    fn getmouse(_: *mut MEVENT) -> libc::c_int;
    fn wenclose(_: *const WINDOW, _: libc::c_int, _: libc::c_int) -> bool;
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
    fn dcngettext(
        __domainname: *const libc::c_char,
        __msgid1: *const libc::c_char,
        __msgid2: *const libc::c_char,
        __n: libc::c_ulong,
        __category: libc::c_int,
    ) -> *mut libc::c_char;
    static mut the_window_resized: sig_atomic_t;
    static mut on_a_vt: bool;
    static mut shifted_metas: bool;
    static mut meta_key: bool;
    static mut shift_held: bool;
    static mut mute_modifiers: bool;
    static mut bracketed_paste: bool;
    static mut we_are_running: bool;
    static mut more_than_one: bool;
    static mut report_size: bool;
    static mut inhelp: bool;
    static mut title: *mut libc::c_char;
    static mut focusing: bool;
    static mut as_an_at: bool;
    static mut lastmessage: message_type;
    static mut flags: [libc::c_uint; 4];
    static mut controlleft: libc::c_int;
    static mut controlright: libc::c_int;
    static mut controlup: libc::c_int;
    static mut controldown: libc::c_int;
    static mut controlhome: libc::c_int;
    static mut controlend: libc::c_int;
    static mut controldelete: libc::c_int;
    static mut controlshiftdelete: libc::c_int;
    static mut shiftup: libc::c_int;
    static mut shiftdown: libc::c_int;
    static mut shiftcontrolleft: libc::c_int;
    static mut shiftcontrolright: libc::c_int;
    static mut shiftcontrolup: libc::c_int;
    static mut shiftcontroldown: libc::c_int;
    static mut shiftcontrolhome: libc::c_int;
    static mut shiftcontrolend: libc::c_int;
    static mut altleft: libc::c_int;
    static mut altright: libc::c_int;
    static mut altup: libc::c_int;
    static mut altdown: libc::c_int;
    static mut altpageup: libc::c_int;
    static mut altpagedown: libc::c_int;
    static mut altinsert: libc::c_int;
    static mut altdelete: libc::c_int;
    static mut shiftaltleft: libc::c_int;
    static mut shiftaltright: libc::c_int;
    static mut shiftaltup: libc::c_int;
    static mut shiftaltdown: libc::c_int;
    static mut topwin: *mut WINDOW;
    static mut midwin: *mut WINDOW;
    static mut footwin: *mut WINDOW;
    static mut editwinrows: libc::c_int;
    static mut editwincols: libc::c_int;
    static mut margin: libc::c_int;
    static mut thebar: libc::c_int;
    static mut bardata: *mut libc::c_int;
    static mut stripe_column: ssize_t;
    static mut openfile: *mut openfilestruct;
    static mut startfile: *mut openfilestruct;
    static mut whitespace: *mut libc::c_char;
    static mut whitelen: [libc::c_int; 2];
    static mut tabsize: ssize_t;
    static mut have_palette: bool;
    static mut perturbed: bool;
    static mut recook: bool;
    static mut refresh_needed: bool;
    static mut currmenu: libc::c_int;
    static mut allfuncs: *mut funcstruct;
    static mut hilite_attribute: libc::c_int;
    static mut interface_color_pair: [libc::c_int; 12];
    static mut commandname: *mut libc::c_char;
    static mut planted_shortcut: *mut keystruct;
    static mut spotlighted: bool;
    static mut light_from_col: size_t;
    static mut light_to_col: size_t;
    fn using_utf8() -> bool;
    fn is_blank_char(c: *const libc::c_char) -> bool;
    fn is_cntrl_char(c: *const libc::c_char) -> bool;
    fn control_mbrep(c: *const libc::c_char, isdata: bool) -> libc::c_char;
    fn mbtowide(wc: *mut wchar_t, c: *const libc::c_char) -> libc::c_int;
    fn is_doublewidth(ch: *const libc::c_char) -> bool;
    fn is_zerowidth(ch: *const libc::c_char) -> bool;
    fn char_length(pointer: *const libc::c_char) -> libc::c_int;
    fn collect_char(
        string: *const libc::c_char,
        thechar: *mut libc::c_char,
    ) -> libc::c_int;
    fn advance_over(string: *const libc::c_char, column: *mut size_t) -> libc::c_int;
    fn step_left(buf: *const libc::c_char, pos: size_t) -> size_t;
    fn step_right(buf: *const libc::c_char, pos: size_t) -> size_t;
    fn prepare_palette();
    fn precalc_multicolorinfo();
    fn close_buffer();
    fn first_sc_for(
        menu: libc::c_int,
        function: Option::<unsafe extern "C" fn() -> ()>,
    ) -> *const keystruct;
    fn shown_entries_for(menu: libc::c_int) -> size_t;
    fn wrap_help_text_into_buffer();
    fn die(msg: *const libc::c_char, _: ...);
    fn window_init();
    fn regenerate_screen();
    fn disable_kb_interrupt();
    fn disable_flow_control();
    fn enable_flow_control();
    fn strtosc(input: *const libc::c_char) -> *mut keystruct;
    fn digits(n: ssize_t) -> libc::c_int;
    fn nmalloc(howmuch: size_t) -> *mut libc::c_void;
    fn nrealloc(ptr: *mut libc::c_void, howmuch: size_t) -> *mut libc::c_void;
    fn measured_copy(string: *const libc::c_char, count: size_t) -> *mut libc::c_char;
    fn copy_of(string: *const libc::c_char) -> *mut libc::c_char;
    fn get_page_start(column: size_t) -> size_t;
    fn xplustabs() -> size_t;
    fn actual_x(text: *const libc::c_char, column: size_t) -> size_t;
    fn wideness(text: *const libc::c_char, maxlen: size_t) -> size_t;
    fn breadth(text: *const libc::c_char) -> size_t;
    fn get_region(
        top: *mut *mut linestruct,
        top_x: *mut size_t,
        bot: *mut *mut linestruct,
        bot_x: *mut size_t,
    );
    fn number_of_characters_in(
        begin: *const linestruct,
        end: *const linestruct,
    ) -> size_t;
    fn __ctype_b_loc() -> *mut *const libc::c_ushort;
    fn __ctype_tolower_loc() -> *mut *const __int32_t;
    fn ioctl(__fd: libc::c_int, __request: libc::c_ulong, _: ...) -> libc::c_int;
    fn memmove(
        _: *mut libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn strchr(_: *const libc::c_char, _: libc::c_int) -> *mut libc::c_char;
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
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
pub type C2RustUnnamed = libc::c_int;
pub const _REG_ERPAREN: C2RustUnnamed = 16;
pub const _REG_ESIZE: C2RustUnnamed = 15;
pub const _REG_EEND: C2RustUnnamed = 14;
pub const _REG_BADRPT: C2RustUnnamed = 13;
pub const _REG_ESPACE: C2RustUnnamed = 12;
pub const _REG_ERANGE: C2RustUnnamed = 11;
pub const _REG_BADBR: C2RustUnnamed = 10;
pub const _REG_EBRACE: C2RustUnnamed = 9;
pub const _REG_EPAREN: C2RustUnnamed = 8;
pub const _REG_EBRACK: C2RustUnnamed = 7;
pub const _REG_ESUBREG: C2RustUnnamed = 6;
pub const _REG_EESCAPE: C2RustUnnamed = 5;
pub const _REG_ECTYPE: C2RustUnnamed = 4;
pub const _REG_ECOLLATE: C2RustUnnamed = 3;
pub const _REG_BADPAT: C2RustUnnamed = 2;
pub const _REG_NOMATCH: C2RustUnnamed = 1;
pub const _REG_NOERROR: C2RustUnnamed = 0;
pub const _REG_ENOSYS: C2RustUnnamed = -1;
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
pub type mmask_t = libc::c_uint;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct MEVENT {
    pub id: libc::c_short,
    pub x: libc::c_int,
    pub y: libc::c_int,
    pub z: libc::c_int,
    pub bstate: mmask_t,
}
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
pub type update_type = libc::c_uint;
pub const STATIONARY: update_type = 2;
pub const FLOWING: update_type = 1;
pub const CENTERING: update_type = 0;
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
pub type C2RustUnnamed_0 = libc::c_uint;
pub const NUMBER_OF_ELEMENTS: C2RustUnnamed_0 = 12;
pub const FUNCTION_TAG: C2RustUnnamed_0 = 11;
pub const KEY_COMBO: C2RustUnnamed_0 = 10;
pub const ERROR_MESSAGE: C2RustUnnamed_0 = 9;
pub const STATUS_BAR: C2RustUnnamed_0 = 8;
pub const PROMPT_BAR: C2RustUnnamed_0 = 7;
pub const MINI_INFOBAR: C2RustUnnamed_0 = 6;
pub const SPOTLIGHTED: C2RustUnnamed_0 = 5;
pub const SELECTED_TEXT: C2RustUnnamed_0 = 4;
pub const SCROLL_BAR: C2RustUnnamed_0 = 3;
pub const GUIDE_STRIPE: C2RustUnnamed_0 = 2;
pub const LINE_NUMBER: C2RustUnnamed_0 = 1;
pub const TITLE_BAR: C2RustUnnamed_0 = 0;
pub type C2RustUnnamed_1 = libc::c_uint;
pub const ZERO: C2RustUnnamed_1 = 48;
pub const MINIBAR: C2RustUnnamed_1 = 47;
pub const USE_MAGIC: C2RustUnnamed_1 = 46;
pub const STATEFLAGS: C2RustUnnamed_1 = 45;
pub const BOOKSTYLE: C2RustUnnamed_1 = 44;
pub const INDICATOR: C2RustUnnamed_1 = 43;
pub const EMPTY_LINE: C2RustUnnamed_1 = 42;
pub const JUMPY_SCROLLING: C2RustUnnamed_1 = 41;
pub const BREAK_LONG_LINES: C2RustUnnamed_1 = 40;
pub const LET_THEM_ZAP: C2RustUnnamed_1 = 39;
pub const AFTER_ENDS: C2RustUnnamed_1 = 38;
pub const AT_BLANKS: C2RustUnnamed_1 = 37;
pub const LINE_NUMBERS: C2RustUnnamed_1 = 36;
pub const SHOW_CURSOR: C2RustUnnamed_1 = 35;
pub const TRIM_BLANKS: C2RustUnnamed_1 = 34;
pub const MAKE_IT_UNIX: C2RustUnnamed_1 = 33;
pub const NOREAD_MODE: C2RustUnnamed_1 = 32;
pub const LOCKING: C2RustUnnamed_1 = 31;
pub const POSITIONLOG: C2RustUnnamed_1 = 30;
pub const SOFTWRAP: C2RustUnnamed_1 = 29;
pub const BOLD_TEXT: C2RustUnnamed_1 = 28;
pub const NO_NEWLINES: C2RustUnnamed_1 = 27;
pub const WORD_BOUNDS: C2RustUnnamed_1 = 26;
pub const QUICK_BLANK: C2RustUnnamed_1 = 25;
pub const TABS_TO_SPACES: C2RustUnnamed_1 = 24;
pub const WHITESPACE_DISPLAY: C2RustUnnamed_1 = 23;
pub const SMART_HOME: C2RustUnnamed_1 = 22;
pub const RESTRICTED: C2RustUnnamed_1 = 21;
pub const HISTORYLOG: C2RustUnnamed_1 = 20;
pub const PRESERVE: C2RustUnnamed_1 = 19;
pub const NO_SYNTAX: C2RustUnnamed_1 = 18;
pub const INSECURE_BACKUP: C2RustUnnamed_1 = 17;
pub const MAKE_BACKUP: C2RustUnnamed_1 = 16;
pub const NO_CONVERT: C2RustUnnamed_1 = 15;
pub const RAW_SEQUENCES: C2RustUnnamed_1 = 14;
pub const REBIND_DELETE: C2RustUnnamed_1 = 13;
pub const MULTIBUFFER: C2RustUnnamed_1 = 12;
pub const BACKWARDS_SEARCH: C2RustUnnamed_1 = 11;
pub const CUT_FROM_CURSOR: C2RustUnnamed_1 = 10;
pub const SAVE_ON_EXIT: C2RustUnnamed_1 = 9;
pub const USE_REGEXP: C2RustUnnamed_1 = 8;
pub const USE_MOUSE: C2RustUnnamed_1 = 7;
pub const VIEW_MODE: C2RustUnnamed_1 = 6;
pub const AUTOINDENT: C2RustUnnamed_1 = 5;
pub const NO_WRAP: C2RustUnnamed_1 = 4;
pub const NO_HELP: C2RustUnnamed_1 = 3;
pub const CONSTANT_SHOW: C2RustUnnamed_1 = 2;
pub const CASE_SENSITIVE: C2RustUnnamed_1 = 1;
pub const DONTUSE: C2RustUnnamed_1 = 0;
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
pub const _ISxdigit: C2RustUnnamed_2 = 4096;
pub type C2RustUnnamed_2 = libc::c_uint;
pub const _ISalnum: C2RustUnnamed_2 = 8;
pub const _ISpunct: C2RustUnnamed_2 = 4;
pub const _IScntrl: C2RustUnnamed_2 = 2;
pub const _ISblank: C2RustUnnamed_2 = 1;
pub const _ISgraph: C2RustUnnamed_2 = 32768;
pub const _ISprint: C2RustUnnamed_2 = 16384;
pub const _ISspace: C2RustUnnamed_2 = 8192;
pub const _ISdigit: C2RustUnnamed_2 = 2048;
pub const _ISalpha: C2RustUnnamed_2 = 1024;
pub const _ISlower: C2RustUnnamed_2 = 512;
pub const _ISupper: C2RustUnnamed_2 = 256;
#[inline]
unsafe extern "C" fn tolower(mut __c: libc::c_int) -> libc::c_int {
    return if __c >= -(128 as libc::c_int) && __c < 256 as libc::c_int {
        *(*__ctype_tolower_loc()).offset(__c as isize)
    } else {
        __c
    };
}
static mut key_buffer: *mut libc::c_int = 0 as *const libc::c_int as *mut libc::c_int;
static mut nextcodes: *mut libc::c_int = 0 as *const libc::c_int as *mut libc::c_int;
static mut capacity: size_t = 32 as libc::c_int as size_t;
static mut waiting_codes: size_t = 0 as libc::c_int as size_t;
static mut plants_pointer: *const libc::c_char = 0 as *const libc::c_char;
static mut digit_count: libc::c_int = 0 as libc::c_int;
static mut reveal_cursor: bool = 0 as libc::c_int != 0;
static mut linger_after_escape: bool = 0 as libc::c_int != 0;
static mut statusblank: libc::c_int = 0 as libc::c_int;
static mut from_x: size_t = 0 as libc::c_int as size_t;
static mut till_x: size_t = 0 as libc::c_int as size_t;
static mut has_more: bool = 0 as libc::c_int != 0;
static mut is_shorter: bool = 1 as libc::c_int != 0;
static mut sequel_column: size_t = 0 as libc::c_int as size_t;
static mut recording: bool = 0 as libc::c_int != 0;
static mut macro_buffer: *mut libc::c_int = 0 as *const libc::c_int as *mut libc::c_int;
static mut macro_length: size_t = 0 as libc::c_int as size_t;
pub unsafe extern "C" fn add_to_macrobuffer(mut code: libc::c_int) {
    macro_length = macro_length.wrapping_add(1);
    macro_length;
    macro_buffer = nrealloc(
        macro_buffer as *mut libc::c_void,
        macro_length.wrapping_mul(::std::mem::size_of::<libc::c_int>() as libc::c_ulong),
    ) as *mut libc::c_int;
    *macro_buffer
        .offset(
            macro_length.wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize,
        ) = code;
}
pub unsafe extern "C" fn snip_last_keystroke() {
    macro_length = macro_length.wrapping_sub(1);
    macro_length;
    while macro_length > 0 as libc::c_int as libc::c_ulong
        && *macro_buffer
            .offset(
                macro_length.wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize,
            ) == '\u{1b}' as i32
    {
        macro_length = macro_length.wrapping_sub(1);
        macro_length;
    }
}
pub unsafe extern "C" fn record_macro() {
    recording = !recording;
    if recording {
        macro_length = 0 as libc::c_int as size_t;
        statusline(
            REMARK,
            dcgettext(
                0 as *const libc::c_char,
                b"Recording a macro...\0" as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ),
        );
    } else {
        snip_last_keystroke();
        statusline(
            REMARK,
            dcgettext(
                0 as *const libc::c_char,
                b"Stopped recording\0" as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ),
        );
    }
    if flags[(STATEFLAGS as libc::c_int as libc::c_ulong)
        .wrapping_div(
            (::std::mem::size_of::<libc::c_uint>() as libc::c_ulong)
                .wrapping_mul(8 as libc::c_int as libc::c_ulong),
        ) as usize]
        & (1 as libc::c_int as libc::c_uint)
            << (STATEFLAGS as libc::c_int as libc::c_ulong)
                .wrapping_rem(
                    (::std::mem::size_of::<libc::c_uint>() as libc::c_ulong)
                        .wrapping_mul(8 as libc::c_int as libc::c_ulong),
                ) != 0 as libc::c_int as libc::c_uint
    {
        titlebar(0 as *const libc::c_char);
    }
}
pub unsafe extern "C" fn run_macro() {
    if recording {
        statusline(
            AHEM,
            dcgettext(
                0 as *const libc::c_char,
                b"Cannot run macro while recording\0" as *const u8
                    as *const libc::c_char,
                5 as libc::c_int,
            ),
        );
        snip_last_keystroke();
        return;
    }
    if macro_length == 0 as libc::c_int as libc::c_ulong {
        statusline(
            REMARK,
            dcgettext(
                0 as *const libc::c_char,
                b"Macro is empty\0" as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ),
        );
        return;
    }
    if macro_length > capacity {
        reserve_space_for(macro_length);
    }
    let mut i: size_t = 0 as libc::c_int as size_t;
    while i < macro_length {
        *key_buffer.offset(i as isize) = *macro_buffer.offset(i as isize);
        i = i.wrapping_add(1);
        i;
    }
    waiting_codes = macro_length;
    nextcodes = key_buffer;
    mute_modifiers = 1 as libc::c_int != 0;
}
pub unsafe extern "C" fn reserve_space_for(mut newsize: size_t) {
    if newsize < capacity {
        die(
            dcgettext(
                0 as *const libc::c_char,
                b"Too much input at once\n\0" as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ),
        );
    }
    key_buffer = nrealloc(
        key_buffer as *mut libc::c_void,
        newsize.wrapping_mul(::std::mem::size_of::<libc::c_int>() as libc::c_ulong),
    ) as *mut libc::c_int;
    nextcodes = key_buffer;
    capacity = newsize;
}
pub unsafe extern "C" fn read_keys_from(mut frame: *mut WINDOW) {
    let mut input: libc::c_int = -(1 as libc::c_int);
    let mut errcount: size_t = 0 as libc::c_int as size_t;
    let mut timed: bool = 0 as libc::c_int != 0;
    doupdate();
    if reveal_cursor as libc::c_int != 0
        && (!spotlighted
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
                        ) != 0 as libc::c_int as libc::c_uint
            || currmenu == (1 as libc::c_int) << 9 as libc::c_int)
        && (LINES > 1 as libc::c_int
            || lastmessage as libc::c_uint <= HUSH as libc::c_int as libc::c_uint)
    {
        curs_set(1 as libc::c_int);
    }
    if currmenu == (1 as libc::c_int) << 0 as libc::c_int
        && ((flags[(MINIBAR as libc::c_int as libc::c_ulong)
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
            || LINES == 1 as libc::c_int)
            && lastmessage as libc::c_uint > HUSH as libc::c_int as libc::c_uint
            && (lastmessage as libc::c_uint) < ALERT as libc::c_int as libc::c_uint
            && lastmessage as libc::c_uint != INFO as libc::c_int as libc::c_uint
            || spotlighted as libc::c_int != 0)
    {
        timed = 1 as libc::c_int != 0;
        halfdelay(
            if flags[(QUICK_BLANK as libc::c_int as libc::c_ulong)
                .wrapping_div(
                    (::std::mem::size_of::<libc::c_uint>() as libc::c_ulong)
                        .wrapping_mul(8 as libc::c_int as libc::c_ulong),
                ) as usize]
                & (1 as libc::c_int as libc::c_uint)
                    << (QUICK_BLANK as libc::c_int as libc::c_ulong)
                        .wrapping_rem(
                            (::std::mem::size_of::<libc::c_uint>() as libc::c_ulong)
                                .wrapping_mul(8 as libc::c_int as libc::c_ulong),
                        ) != 0 as libc::c_int as libc::c_uint
            {
                8 as libc::c_int
            } else {
                15 as libc::c_int
            },
        );
        disable_kb_interrupt();
    }
    while input == -(1 as libc::c_int) {
        input = wgetch(frame);
        if the_window_resized != 0 {
            regenerate_screen();
            input = -(2 as libc::c_int);
        }
        if timed {
            timed = 0 as libc::c_int != 0;
            raw();
            if input == -(1 as libc::c_int) {
                if spotlighted as libc::c_int != 0
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
                    || LINES == 1 as libc::c_int
                {
                    if flags[(ZERO as libc::c_int as libc::c_ulong)
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
                        && lastmessage as libc::c_uint
                            > VACUUM as libc::c_int as libc::c_uint
                    {
                        wredrawln(
                            midwin,
                            editwinrows - 1 as libc::c_int,
                            1 as libc::c_int,
                        );
                    }
                    lastmessage = VACUUM;
                    spotlighted = 0 as libc::c_int != 0;
                    update_line((*openfile).current, (*openfile).current_x);
                    wnoutrefresh(midwin);
                    curs_set(1 as libc::c_int);
                }
                if flags[(MINIBAR as libc::c_int as libc::c_ulong)
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
                    && !(flags[(ZERO as libc::c_int as libc::c_ulong)
                        .wrapping_div(
                            (::std::mem::size_of::<libc::c_uint>() as libc::c_ulong)
                                .wrapping_mul(8 as libc::c_int as libc::c_ulong),
                        ) as usize]
                        & (1 as libc::c_int as libc::c_uint)
                            << (ZERO as libc::c_int as libc::c_ulong)
                                .wrapping_rem(
                                    (::std::mem::size_of::<libc::c_uint>() as libc::c_ulong)
                                        .wrapping_mul(8 as libc::c_int as libc::c_ulong),
                                ) != 0 as libc::c_int as libc::c_uint)
                    && LINES > 1 as libc::c_int
                {
                    minibar();
                }
                as_an_at = 1 as libc::c_int != 0;
                place_the_cursor();
                doupdate();
                continue;
            }
        }
        if input == -(1 as libc::c_int)
            && {
                errcount = errcount.wrapping_add(1);
                errcount == 12345678 as libc::c_int as libc::c_ulong
            }
        {
            die(
                dcgettext(
                    0 as *const libc::c_char,
                    b"Too many errors from stdin\n\0" as *const u8
                        as *const libc::c_char,
                    5 as libc::c_int,
                ),
            );
        }
    }
    curs_set(0 as libc::c_int);
    if key_buffer.is_null() {
        reserve_space_for(capacity);
    }
    *key_buffer.offset(0 as libc::c_int as isize) = input;
    nextcodes = key_buffer;
    waiting_codes = 1 as libc::c_int as size_t;
    if currmenu == (1 as libc::c_int) << 0 as libc::c_int {
        refresh_needed = (refresh_needed as libc::c_int | spotlighted as libc::c_int) != 0;
        spotlighted = 0 as libc::c_int != 0;
    }
    if input == -(2 as libc::c_int) {
        return;
    }
    nodelay(frame, 1 as libc::c_int != 0);
    if input == 0x1b as libc::c_int
        && (linger_after_escape as libc::c_int != 0
            || flags[(RAW_SEQUENCES as libc::c_int as libc::c_ulong)
                .wrapping_div(
                    (::std::mem::size_of::<libc::c_uint>() as libc::c_ulong)
                        .wrapping_mul(8 as libc::c_int as libc::c_ulong),
                ) as usize]
                & (1 as libc::c_int as libc::c_uint)
                    << (RAW_SEQUENCES as libc::c_int as libc::c_ulong)
                        .wrapping_rem(
                            (::std::mem::size_of::<libc::c_uint>() as libc::c_ulong)
                                .wrapping_mul(8 as libc::c_int as libc::c_ulong),
                        ) != 0 as libc::c_int as libc::c_uint)
    {
        napms(20 as libc::c_int);
    }
    loop {
        if recording {
            add_to_macrobuffer(input);
        }
        input = wgetch(frame);
        if input == -(1 as libc::c_int) {
            break;
        }
        if waiting_codes == capacity {
            reserve_space_for(
                (2 as libc::c_int as libc::c_ulong).wrapping_mul(capacity),
            );
        }
        let fresh0 = waiting_codes;
        waiting_codes = waiting_codes.wrapping_add(1);
        *key_buffer.offset(fresh0 as isize) = input;
    }
    nodelay(frame, 0 as libc::c_int != 0);
}
pub unsafe extern "C" fn waiting_keycodes() -> size_t {
    return waiting_codes;
}
pub unsafe extern "C" fn put_back(mut keycode: libc::c_int) {
    if nextcodes == key_buffer {
        if waiting_codes == capacity {
            reserve_space_for(
                (2 as libc::c_int as libc::c_ulong).wrapping_mul(capacity),
            );
        }
        memmove(
            key_buffer.offset(1 as libc::c_int as isize) as *mut libc::c_void,
            key_buffer as *const libc::c_void,
            waiting_codes
                .wrapping_mul(::std::mem::size_of::<libc::c_int>() as libc::c_ulong),
        );
    } else {
        nextcodes = nextcodes.offset(-1);
        nextcodes;
    }
    *nextcodes = keycode;
    waiting_codes = waiting_codes.wrapping_add(1);
    waiting_codes;
}
pub unsafe extern "C" fn implant(mut string: *const libc::c_char) {
    plants_pointer = string;
    put_back(0x4ea as libc::c_int);
    mute_modifiers = 1 as libc::c_int != 0;
}
pub unsafe extern "C" fn get_code_from_plantation() -> libc::c_int {
    if *plants_pointer as libc::c_int == '{' as i32 {
        let mut closing: *mut libc::c_char = strchr(
            plants_pointer.offset(1 as libc::c_int as isize),
            '}' as i32,
        );
        if closing.is_null() {
            return 0x4eb as libc::c_int;
        }
        if *plants_pointer.offset(1 as libc::c_int as isize) as libc::c_int == '{' as i32
            && *plants_pointer.offset(2 as libc::c_int as isize) as libc::c_int
                == '}' as i32
        {
            plants_pointer = plants_pointer.offset(3 as libc::c_int as isize);
            if *plants_pointer as libc::c_int != '\0' as i32 {
                put_back(0x4ea as libc::c_int);
            }
            return '{' as i32;
        }
        rpl_free(commandname as *mut libc::c_void);
        rpl_free(planted_shortcut as *mut libc::c_void);
        commandname = measured_copy(
            plants_pointer.offset(1 as libc::c_int as isize),
            (closing.offset_from(plants_pointer) as libc::c_long
                - 1 as libc::c_int as libc::c_long) as size_t,
        );
        planted_shortcut = strtosc(commandname);
        if planted_shortcut.is_null() {
            return 0x4ef as libc::c_int;
        }
        plants_pointer = closing.offset(1 as libc::c_int as isize);
        if *plants_pointer as libc::c_int != '\0' as i32 {
            put_back(0x4ea as libc::c_int);
        }
        return 0x4ec as libc::c_int;
    } else {
        let mut opening: *mut libc::c_char = strchr(plants_pointer, '{' as i32);
        let mut length: libc::c_int = 0;
        if !opening.is_null() {
            length = opening.offset_from(plants_pointer) as libc::c_long as libc::c_int;
            put_back(0x4ea as libc::c_int);
        } else {
            length = strlen(plants_pointer) as libc::c_int;
        }
        let mut index: libc::c_int = length - 1 as libc::c_int;
        while index >= 0 as libc::c_int {
            put_back(
                *plants_pointer.offset(index as isize) as libc::c_uchar as libc::c_int,
            );
            index -= 1;
            index;
        }
        plants_pointer = plants_pointer.offset(length as isize);
        return -(1 as libc::c_int);
    };
}
pub unsafe extern "C" fn get_input(mut frame: *mut WINDOW) -> libc::c_int {
    if waiting_codes != 0 {
        spotlighted = 0 as libc::c_int != 0;
    } else if !frame.is_null() {
        read_keys_from(frame);
    }
    if waiting_codes > 0 as libc::c_int as libc::c_ulong {
        waiting_codes = waiting_codes.wrapping_sub(1);
        waiting_codes;
        if *nextcodes == 0x4ea as libc::c_int {
            nextcodes = nextcodes.offset(1);
            nextcodes;
            return get_code_from_plantation();
        } else {
            let fresh1 = nextcodes;
            nextcodes = nextcodes.offset(1);
            return *fresh1;
        }
    } else {
        return -(1 as libc::c_int)
    };
}
pub unsafe extern "C" fn arrow_from_ABCD(mut letter: libc::c_int) -> libc::c_int {
    if letter < 'C' as i32 {
        return if letter == 'A' as i32 {
            0o403 as libc::c_int
        } else {
            0o402 as libc::c_int
        }
    } else {
        return if letter == 'D' as i32 {
            0o404 as libc::c_int
        } else {
            0o405 as libc::c_int
        }
    };
}
pub unsafe extern "C" fn convert_SS3_sequence(
    mut seq: *const libc::c_int,
    mut length: size_t,
    mut consumed: *mut libc::c_int,
) -> libc::c_int {
    let mut current_block_48: u64;
    match *seq.offset(0 as libc::c_int as isize) {
        49 => {
            if length > 3 as libc::c_int as libc::c_ulong
                && *seq.offset(1 as libc::c_int as isize) == ';' as i32
            {
                *consumed = 4 as libc::c_int;
                match *seq.offset(2 as libc::c_int as isize) {
                    50 => {
                        if 'A' as i32 <= *seq.offset(3 as libc::c_int as isize)
                            && *seq.offset(3 as libc::c_int as isize) <= 'D' as i32
                        {
                            shift_held = 1 as libc::c_int != 0;
                            return arrow_from_ABCD(
                                *seq.offset(3 as libc::c_int as isize),
                            );
                        }
                    }
                    53 => {
                        match *seq.offset(3 as libc::c_int as isize) {
                            65 => return 0x403 as libc::c_int,
                            66 => return 0x404 as libc::c_int,
                            67 => return 0x402 as libc::c_int,
                            68 => return 0x401 as libc::c_int,
                            _ => {}
                        }
                    }
                    _ => {}
                }
            }
            current_block_48 = 14775119014532381840;
        }
        50 => {
            current_block_48 = 18228691620988496420;
        }
        51 => {
            current_block_48 = 18228691620988496420;
        }
        52 => {
            current_block_48 = 3531194215973766183;
        }
        53 => {
            current_block_48 = 1013768444847953897;
        }
        54 => {
            current_block_48 = 4522937792372956349;
        }
        55 | 56 => {
            current_block_48 = 7469266862958907235;
        }
        65 => {
            current_block_48 = 13099758733114062177;
        }
        66 => {
            current_block_48 = 13099758733114062177;
        }
        67 | 68 => {
            current_block_48 = 7053386561728980176;
        }
        70 => return 0o550 as libc::c_int,
        72 => return 0o406 as libc::c_int,
        77 => return 0o527 as libc::c_int,
        80 => {
            current_block_48 = 15531741053463456202;
        }
        81 => {
            current_block_48 = 15531741053463456202;
        }
        82 | 83 => {
            current_block_48 = 9261563926820496240;
        }
        84 => {
            current_block_48 = 12261593875952326340;
        }
        85 => {
            current_block_48 = 12261593875952326340;
        }
        86 => {
            current_block_48 = 2719350581222748826;
        }
        87 => {
            current_block_48 = 2639579738044951735;
        }
        88 | 89 => {
            current_block_48 = 8746691339416183331;
        }
        97 => return 0x403 as libc::c_int,
        98 => return 0x404 as libc::c_int,
        99 => return 0x402 as libc::c_int,
        100 => return 0x401 as libc::c_int,
        106 => return '*' as i32,
        107 => return '+' as i32,
        108 => return ',' as i32,
        109 => return '-' as i32,
        110 => return 0o512 as libc::c_int,
        111 => return '/' as i32,
        112 => return 0o513 as libc::c_int,
        113 => return 0o550 as libc::c_int,
        114 => return 0o402 as libc::c_int,
        115 => return 0o522 as libc::c_int,
        116 => return 0o404 as libc::c_int,
        118 => return 0o405 as libc::c_int,
        119 => return 0o406 as libc::c_int,
        120 => return 0o403 as libc::c_int,
        121 => return 0o523 as libc::c_int,
        _ => {
            current_block_48 = 14775119014532381840;
        }
    }
    match current_block_48 {
        18228691620988496420 => {
            current_block_48 = 3531194215973766183;
        }
        13099758733114062177 => {
            current_block_48 = 7053386561728980176;
        }
        15531741053463456202 => {
            current_block_48 = 9261563926820496240;
        }
        12261593875952326340 => {
            current_block_48 = 2719350581222748826;
        }
        _ => {}
    }
    match current_block_48 {
        3531194215973766183 => {
            current_block_48 = 1013768444847953897;
        }
        7053386561728980176 => {
            return arrow_from_ABCD(*seq.offset(0 as libc::c_int as isize));
        }
        9261563926820496240 => {
            return 0o410 as libc::c_int
                + (*seq.offset(0 as libc::c_int as isize) - 'O' as i32);
        }
        2719350581222748826 => {
            current_block_48 = 2639579738044951735;
        }
        _ => {}
    }
    match current_block_48 {
        1013768444847953897 => {
            current_block_48 = 4522937792372956349;
        }
        2639579738044951735 => {
            current_block_48 = 8746691339416183331;
        }
        _ => {}
    }
    match current_block_48 {
        4522937792372956349 => {
            current_block_48 = 7469266862958907235;
        }
        8746691339416183331 => {
            return 0o410 as libc::c_int
                + (*seq.offset(0 as libc::c_int as isize) - 'O' as i32);
        }
        _ => {}
    }
    match current_block_48 {
        7469266862958907235 => {
            if length > 1 as libc::c_int as libc::c_ulong {
                *consumed = 2 as libc::c_int;
                if *seq.offset(0 as libc::c_int as isize) == '4' as i32
                    || *seq.offset(0 as libc::c_int as isize) > '5' as i32
                {
                    return 0x4fc as libc::c_int;
                }
                match *seq.offset(1 as libc::c_int as isize) {
                    65 => return 0x403 as libc::c_int,
                    66 => return 0x404 as libc::c_int,
                    67 => return 0x402 as libc::c_int,
                    68 => return 0x401 as libc::c_int,
                    _ => {}
                }
                return *seq.offset(1 as libc::c_int as isize) - 0x40 as libc::c_int;
            }
        }
        _ => {}
    }
    return 0x4fc as libc::c_int;
}
pub unsafe extern "C" fn convert_CSI_sequence(
    mut seq: *const libc::c_int,
    mut length: size_t,
    mut consumed: *mut libc::c_int,
) -> libc::c_int {
    if *seq.offset(0 as libc::c_int as isize) < '9' as i32
        && length > 1 as libc::c_int as libc::c_ulong
    {
        *consumed = 2 as libc::c_int;
    }
    's_698: {
        let mut current_block_151: u64;
        match *seq.offset(0 as libc::c_int as isize) {
            49 => {
                if length > 1 as libc::c_int as libc::c_ulong
                    && *seq.offset(1 as libc::c_int as isize) == '~' as i32
                {
                    return 0o406 as libc::c_int
                } else if length > 2 as libc::c_int as libc::c_ulong
                    && *seq.offset(2 as libc::c_int as isize) == '~' as i32
                {
                    *consumed = 3 as libc::c_int;
                    's_42: {
                        let mut current_block_5: u64;
                        match *seq.offset(1 as libc::c_int as isize) {
                            50 => {
                                current_block_5 = 14081461556862064841;
                            }
                            51 => {
                                current_block_5 = 14081461556862064841;
                            }
                            52 => {
                                current_block_5 = 606814451225588316;
                            }
                            49 | 53 => {
                                current_block_5 = 11437189164350124282;
                            }
                            56 => {
                                current_block_5 = 15967237359003027682;
                            }
                            55 | 57 => {
                                current_block_5 = 15967237359003027682;
                            }
                            _ => {
                                break 's_42;
                            }
                        }
                        match current_block_5 {
                            14081461556862064841 => {
                                current_block_5 = 606814451225588316;
                            }
                            15967237359003027682 => {
                                return 0o410 as libc::c_int
                                    + (*seq.offset(1 as libc::c_int as isize) - '1' as i32);
                            }
                            _ => {}
                        }
                        match current_block_5 {
                            606814451225588316 => {}
                            _ => {}
                        }
                        return 0o410 as libc::c_int
                            + (*seq.offset(1 as libc::c_int as isize) - '0' as i32);
                    }
                } else if length > 3 as libc::c_int as libc::c_ulong
                    && *seq.offset(1 as libc::c_int as isize) == ';' as i32
                {
                    *consumed = 4 as libc::c_int;
                    match *seq.offset(2 as libc::c_int as isize) {
                        50 => 's_83: {
                            let mut current_block_12: u64;
                            match *seq.offset(3 as libc::c_int as isize) {
                                66 => {
                                    current_block_12 = 8994603623389184299;
                                }
                                67 => {
                                    current_block_12 = 8994603623389184299;
                                }
                                65 | 68 => {
                                    current_block_12 = 12879184554692362543;
                                }
                                70 => return 0x456 as libc::c_int,
                                72 => return 0x455 as libc::c_int,
                                _ => {
                                    break 's_83;
                                }
                            }
                            match current_block_12 {
                                8994603623389184299 => {}
                                _ => {}
                            }
                            shift_held = 1 as libc::c_int != 0;
                            return arrow_from_ABCD(
                                *seq.offset(3 as libc::c_int as isize),
                            );
                        }
                        57 | 51 => {
                            match *seq.offset(3 as libc::c_int as isize) {
                                65 => return 0x423 as libc::c_int,
                                66 => return 0x424 as libc::c_int,
                                67 => return 0x422 as libc::c_int,
                                68 => return 0x421 as libc::c_int,
                                _ => {}
                            }
                        }
                        52 => {
                            match *seq.offset(3 as libc::c_int as isize) {
                                65 => return 0x457 as libc::c_int,
                                66 => return 0x458 as libc::c_int,
                                67 => return 0x456 as libc::c_int,
                                68 => return 0x455 as libc::c_int,
                                _ => {}
                            }
                        }
                        53 => {
                            match *seq.offset(3 as libc::c_int as isize) {
                                65 => return 0x403 as libc::c_int,
                                66 => return 0x404 as libc::c_int,
                                67 => return 0x402 as libc::c_int,
                                68 => return 0x401 as libc::c_int,
                                70 => return 0x406 as libc::c_int,
                                72 => return 0x405 as libc::c_int,
                                _ => {}
                            }
                        }
                        54 => {
                            match *seq.offset(3 as libc::c_int as isize) {
                                65 => return shiftcontrolup,
                                66 => return shiftcontroldown,
                                67 => return shiftcontrolright,
                                68 => return shiftcontrolleft,
                                70 => return shiftcontrolend,
                                72 => return shiftcontrolhome,
                                _ => {}
                            }
                        }
                        _ => {}
                    }
                } else if length > 4 as libc::c_int as libc::c_ulong
                    && *seq.offset(2 as libc::c_int as isize) == ';' as i32
                    && *seq.offset(4 as libc::c_int as isize) == '~' as i32
                {
                    *consumed = 5 as libc::c_int;
                }
                current_block_151 = 9343041660989783267;
            }
            50 => {
                if length > 2 as libc::c_int as libc::c_ulong
                    && *seq.offset(2 as libc::c_int as isize) == '~' as i32
                {
                    *consumed = 3 as libc::c_int;
                    match *seq.offset(1 as libc::c_int as isize) {
                        48 => return 0o410 as libc::c_int + 9 as libc::c_int,
                        49 => return 0o410 as libc::c_int + 10 as libc::c_int,
                        51 => return 0o410 as libc::c_int + 11 as libc::c_int,
                        52 => return 0o410 as libc::c_int + 12 as libc::c_int,
                        _ => {}
                    }
                } else if length > 1 as libc::c_int as libc::c_ulong
                    && *seq.offset(1 as libc::c_int as isize) == '~' as i32
                {
                    return 0o513 as libc::c_int
                } else if length > 3 as libc::c_int as libc::c_ulong
                    && *seq.offset(1 as libc::c_int as isize) == ';' as i32
                    && *seq.offset(3 as libc::c_int as isize) == '~' as i32
                {
                    *consumed = 4 as libc::c_int;
                    if *seq.offset(2 as libc::c_int as isize) == '3' as i32 {
                        return 0x42c as libc::c_int;
                    }
                } else if length > 4 as libc::c_int as libc::c_ulong
                    && *seq.offset(2 as libc::c_int as isize) == ';' as i32
                    && *seq.offset(4 as libc::c_int as isize) == '~' as i32
                {
                    *consumed = 5 as libc::c_int;
                } else if length > 3 as libc::c_int as libc::c_ulong
                    && *seq.offset(1 as libc::c_int as isize) == '0' as i32
                    && *seq.offset(3 as libc::c_int as isize) == '~' as i32
                {
                    *consumed = 4 as libc::c_int;
                    if *seq.offset(2 as libc::c_int as isize) == '0' as i32 {
                        bracketed_paste = 1 as libc::c_int != 0;
                        return 0x4fb as libc::c_int;
                    } else if *seq.offset(2 as libc::c_int as isize) == '1' as i32 {
                        bracketed_paste = 0 as libc::c_int != 0;
                        return 0x4fb as libc::c_int;
                    }
                }
                current_block_151 = 9343041660989783267;
            }
            51 => {
                if length > 1 as libc::c_int as libc::c_ulong
                    && *seq.offset(1 as libc::c_int as isize) == '~' as i32
                {
                    return 0o512 as libc::c_int;
                }
                if length > 3 as libc::c_int as libc::c_ulong
                    && *seq.offset(1 as libc::c_int as isize) == ';' as i32
                    && *seq.offset(3 as libc::c_int as isize) == '~' as i32
                {
                    *consumed = 4 as libc::c_int;
                    if *seq.offset(2 as libc::c_int as isize) == '2' as i32 {
                        return 0x45d as libc::c_int;
                    }
                    if *seq.offset(2 as libc::c_int as isize) == '3' as i32 {
                        return 0x42d as libc::c_int;
                    }
                    if *seq.offset(2 as libc::c_int as isize) == '5' as i32 {
                        return 0x40d as libc::c_int;
                    }
                    if *seq.offset(2 as libc::c_int as isize) == '6' as i32 {
                        return controlshiftdelete;
                    }
                }
                if length > 1 as libc::c_int as libc::c_ulong
                    && *seq.offset(1 as libc::c_int as isize) == '$' as i32
                {
                    return 0x45d as libc::c_int;
                }
                if length > 1 as libc::c_int as libc::c_ulong
                    && *seq.offset(1 as libc::c_int as isize) == '^' as i32
                {
                    return 0x40d as libc::c_int;
                }
                if length > 1 as libc::c_int as libc::c_ulong
                    && *seq.offset(1 as libc::c_int as isize) == '@' as i32
                {
                    return controlshiftdelete;
                }
                if length > 2 as libc::c_int as libc::c_ulong
                    && *seq.offset(2 as libc::c_int as isize) == '~' as i32
                {
                    *consumed = 3 as libc::c_int;
                }
                current_block_151 = 9343041660989783267;
            }
            52 => {
                if length > 1 as libc::c_int as libc::c_ulong
                    && *seq.offset(1 as libc::c_int as isize) == '~' as i32
                {
                    return 0o550 as libc::c_int;
                }
                current_block_151 = 9343041660989783267;
            }
            53 => {
                if length > 1 as libc::c_int as libc::c_ulong
                    && *seq.offset(1 as libc::c_int as isize) == '~' as i32
                {
                    return 0o523 as libc::c_int
                } else if length > 3 as libc::c_int as libc::c_ulong
                    && *seq.offset(1 as libc::c_int as isize) == ';' as i32
                    && *seq.offset(3 as libc::c_int as isize) == '~' as i32
                {
                    *consumed = 4 as libc::c_int;
                    if *seq.offset(2 as libc::c_int as isize) == '2' as i32 {
                        return shiftaltup;
                    }
                    if *seq.offset(2 as libc::c_int as isize) == '3' as i32 {
                        return 0x427 as libc::c_int;
                    }
                }
                current_block_151 = 9343041660989783267;
            }
            54 => {
                if length > 1 as libc::c_int as libc::c_ulong
                    && *seq.offset(1 as libc::c_int as isize) == '~' as i32
                {
                    return 0o522 as libc::c_int
                } else if length > 3 as libc::c_int as libc::c_ulong
                    && *seq.offset(1 as libc::c_int as isize) == ';' as i32
                    && *seq.offset(3 as libc::c_int as isize) == '~' as i32
                {
                    *consumed = 4 as libc::c_int;
                    if *seq.offset(2 as libc::c_int as isize) == '2' as i32 {
                        return shiftaltdown;
                    }
                    if *seq.offset(2 as libc::c_int as isize) == '3' as i32 {
                        return 0x428 as libc::c_int;
                    }
                }
                current_block_151 = 9343041660989783267;
            }
            55 => {
                if length > 1 as libc::c_int as libc::c_ulong
                    && *seq.offset(1 as libc::c_int as isize) == '~' as i32
                {
                    return 0o406 as libc::c_int
                } else if length > 1 as libc::c_int as libc::c_ulong
                    && *seq.offset(1 as libc::c_int as isize) == '$' as i32
                {
                    return 0x455 as libc::c_int
                } else if length > 1 as libc::c_int as libc::c_ulong
                    && *seq.offset(1 as libc::c_int as isize) == '^' as i32
                {
                    return 0x405 as libc::c_int
                } else if length > 1 as libc::c_int as libc::c_ulong
                    && *seq.offset(1 as libc::c_int as isize) == '@' as i32
                {
                    return shiftcontrolhome
                }
                current_block_151 = 9343041660989783267;
            }
            56 => {
                if length > 1 as libc::c_int as libc::c_ulong
                    && *seq.offset(1 as libc::c_int as isize) == '~' as i32
                {
                    return 0o550 as libc::c_int
                } else if length > 1 as libc::c_int as libc::c_ulong
                    && *seq.offset(1 as libc::c_int as isize) == '$' as i32
                {
                    return 0x456 as libc::c_int
                } else if length > 1 as libc::c_int as libc::c_ulong
                    && *seq.offset(1 as libc::c_int as isize) == '^' as i32
                {
                    return 0x406 as libc::c_int
                } else if length > 1 as libc::c_int as libc::c_ulong
                    && *seq.offset(1 as libc::c_int as isize) == '@' as i32
                {
                    return shiftcontrolend
                }
                current_block_151 = 9343041660989783267;
            }
            57 => return 0o512 as libc::c_int,
            64 => return 0o513 as libc::c_int,
            65 => {
                current_block_151 = 13773266194424385961;
            }
            66 => {
                current_block_151 = 13773266194424385961;
            }
            67 | 68 => {
                current_block_151 = 14032048569441850207;
            }
            70 => return 0o550 as libc::c_int,
            71 => return 0o522 as libc::c_int,
            72 => return 0o406 as libc::c_int,
            73 => return 0o523 as libc::c_int,
            76 => return 0o513 as libc::c_int,
            77 => {
                current_block_151 = 14581331546066466061;
            }
            78 => {
                current_block_151 = 14581331546066466061;
            }
            79 => {
                current_block_151 = 7773802594033230171;
            }
            80 => {
                current_block_151 = 5492629648643653079;
            }
            81 => {
                current_block_151 = 12861971868104172012;
            }
            82 => {
                current_block_151 = 11433996087828366468;
            }
            83 | 84 => {
                current_block_151 = 9840048153818942413;
            }
            85 => return 0o522 as libc::c_int,
            86 => return 0o523 as libc::c_int,
            87 => return 0o410 as libc::c_int + 11 as libc::c_int,
            88 => return 0o410 as libc::c_int + 12 as libc::c_int,
            89 => return 0o550 as libc::c_int,
            90 => return 0x45f as libc::c_int,
            97 => {
                current_block_151 = 4822848011261434769;
            }
            98 => {
                current_block_151 = 4822848011261434769;
            }
            99 | 100 => {
                current_block_151 = 7726309813088044361;
            }
            91 => {
                if length > 1 as libc::c_int as libc::c_ulong {
                    *consumed = 2 as libc::c_int;
                    if ('@' as i32) < *seq.offset(1 as libc::c_int as isize)
                        && *seq.offset(1 as libc::c_int as isize) < 'F' as i32
                    {
                        return 0o410 as libc::c_int
                            + (*seq.offset(1 as libc::c_int as isize) - '@' as i32);
                    }
                }
                current_block_151 = 9343041660989783267;
            }
            _ => {
                current_block_151 = 9343041660989783267;
            }
        }
        match current_block_151 {
            13773266194424385961 => {
                current_block_151 = 14032048569441850207;
            }
            14581331546066466061 => {
                current_block_151 = 7773802594033230171;
            }
            4822848011261434769 => {
                current_block_151 = 7726309813088044361;
            }
            9343041660989783267 => {
                break 's_698;
            }
            _ => {}
        }
        match current_block_151 {
            7726309813088044361 => {
                shift_held = 1 as libc::c_int != 0;
                return arrow_from_ABCD(
                    *seq.offset(0 as libc::c_int as isize) - 0x20 as libc::c_int,
                );
            }
            14032048569441850207 => {
                return arrow_from_ABCD(*seq.offset(0 as libc::c_int as isize));
            }
            7773802594033230171 => {
                current_block_151 = 5492629648643653079;
            }
            _ => {}
        }
        match current_block_151 {
            5492629648643653079 => {
                current_block_151 = 12861971868104172012;
            }
            _ => {}
        }
        match current_block_151 {
            12861971868104172012 => {
                current_block_151 = 11433996087828366468;
            }
            _ => {}
        }
        match current_block_151 {
            11433996087828366468 => {}
            _ => {}
        }
        return 0o410 as libc::c_int
            + (*seq.offset(0 as libc::c_int as isize) - 'L' as i32);
    }
    return 0x4fc as libc::c_int;
}
pub unsafe extern "C" fn parse_escape_sequence(mut starter: libc::c_int) -> libc::c_int {
    let mut consumed: libc::c_int = 1 as libc::c_int;
    let mut keycode: libc::c_int = 0 as libc::c_int;
    if starter == 'O' as i32 {
        keycode = convert_SS3_sequence(nextcodes, waiting_codes, &mut consumed);
    } else if starter == '[' as i32 {
        keycode = convert_CSI_sequence(nextcodes, waiting_codes, &mut consumed);
    }
    waiting_codes = (waiting_codes as libc::c_ulong)
        .wrapping_sub(consumed as libc::c_ulong) as size_t as size_t;
    nextcodes = nextcodes.offset(consumed as isize);
    return keycode;
}
pub unsafe extern "C" fn assemble_byte_code(mut keycode: libc::c_int) -> libc::c_int {
    static mut byte: libc::c_int = 0 as libc::c_int;
    digit_count += 1;
    digit_count;
    if digit_count == 1 as libc::c_int {
        byte = (keycode - '0' as i32) * 100 as libc::c_int;
        return -(44 as libc::c_int);
    }
    if digit_count == 2 as libc::c_int {
        if byte < 200 as libc::c_int || keycode <= '5' as i32 {
            byte += (keycode - '0' as i32) * 10 as libc::c_int;
            return -(44 as libc::c_int);
        } else {
            return keycode
        }
    }
    if byte < 250 as libc::c_int || keycode <= '5' as i32 {
        return byte + keycode - '0' as i32
    } else {
        return keycode
    };
}
pub unsafe extern "C" fn convert_to_control(mut kbinput: libc::c_int) -> libc::c_int {
    if '@' as i32 <= kbinput && kbinput <= '_' as i32 {
        return kbinput - '@' as i32;
    }
    if '`' as i32 <= kbinput && kbinput <= '~' as i32 {
        return kbinput - '`' as i32;
    }
    if '3' as i32 <= kbinput && kbinput <= '7' as i32 {
        return kbinput - 24 as libc::c_int;
    }
    if kbinput == '?' as i32 || kbinput == '8' as i32 {
        return 0x7f as libc::c_int;
    }
    if kbinput == ' ' as i32 || kbinput == '2' as i32 {
        return 0 as libc::c_int;
    }
    if kbinput == '/' as i32 {
        return 31 as libc::c_int;
    }
    return kbinput;
}
pub unsafe extern "C" fn parse_kbinput(mut frame: *mut WINDOW) -> libc::c_int {
    static mut first_escape_was_alone: bool = 0 as libc::c_int != 0;
    static mut last_escape_was_alone: bool = 0 as libc::c_int != 0;
    static mut escapes: libc::c_int = 0 as libc::c_int;
    let mut keycode: libc::c_int = 0;
    meta_key = 0 as libc::c_int != 0;
    shift_held = 0 as libc::c_int != 0;
    keycode = get_input(frame);
    if keycode == 0x1b as libc::c_int {
        first_escape_was_alone = last_escape_was_alone;
        last_escape_was_alone = waiting_codes == 0 as libc::c_int as libc::c_ulong;
        if digit_count > 0 as libc::c_int {
            digit_count = 0 as libc::c_int;
            escapes = 1 as libc::c_int;
        } else {
            escapes += 1;
            if escapes > 2 as libc::c_int {
                escapes = if last_escape_was_alone as libc::c_int != 0 {
                    0 as libc::c_int
                } else {
                    1 as libc::c_int
                };
            }
        }
        return -(1 as libc::c_int);
    } else if keycode == -(1 as libc::c_int) {
        return -(1 as libc::c_int)
    }
    if escapes == 0 as libc::c_int {
        if keycode < 0xff as libc::c_int && keycode != '\t' as i32
            && keycode != 0x7f as libc::c_int
        {
            return keycode;
        }
    } else if escapes == 1 as libc::c_int {
        escapes = 0 as libc::c_int;
        if keycode < 0x20 as libc::c_int || (0x7e as libc::c_int) < keycode {
            if keycode == '\t' as i32 {
                return 0x45f as libc::c_int
            } else if keycode == 0o407 as libc::c_int || keycode == '\u{8}' as i32
                || keycode == 0x7f as libc::c_int
            {
                return 0x41d as libc::c_int
            } else if 0xc0 as libc::c_int <= keycode && keycode <= 0xff as libc::c_int
                && using_utf8() as libc::c_int != 0
            {
                while waiting_codes != 0
                    && 0x80 as libc::c_int
                        <= *nextcodes.offset(0 as libc::c_int as isize)
                    && *nextcodes.offset(0 as libc::c_int as isize)
                        <= 0xbf as libc::c_int
                {
                    get_input(0 as *mut WINDOW);
                }
                return 0x4fc as libc::c_int;
            } else if keycode < 0x20 as libc::c_int && !last_escape_was_alone {
                meta_key = 1 as libc::c_int != 0;
            }
        } else if waiting_codes == 0 as libc::c_int as libc::c_ulong
            || *nextcodes.offset(0 as libc::c_int as isize) == 0x1b as libc::c_int
            || keycode != 'O' as i32 && keycode != '[' as i32
        {
            if !shifted_metas {
                keycode = {
                    let mut __res: libc::c_int = 0;
                    if ::std::mem::size_of::<libc::c_int>() as libc::c_ulong
                        > 1 as libc::c_int as libc::c_ulong
                    {
                        if 0 != 0 {
                            let mut __c: libc::c_int = keycode;
                            __res = if __c < -(128 as libc::c_int)
                                || __c > 255 as libc::c_int
                            {
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
                };
            }
            meta_key = 1 as libc::c_int != 0;
        } else {
            keycode = parse_escape_sequence(keycode);
        }
    } else {
        escapes = 0 as libc::c_int;
        if keycode == '[' as i32 && waiting_codes != 0
            && ('A' as i32 <= *nextcodes.offset(0 as libc::c_int as isize)
                && *nextcodes.offset(0 as libc::c_int as isize) <= 'D' as i32
                || 'a' as i32 <= *nextcodes.offset(0 as libc::c_int as isize)
                    && *nextcodes.offset(0 as libc::c_int as isize) <= 'd' as i32)
        {
            match get_input(0 as *mut WINDOW) {
                65 => return 0o406 as libc::c_int,
                66 => return 0o550 as libc::c_int,
                67 => return 0x402 as libc::c_int,
                68 => return 0x401 as libc::c_int,
                97 => {
                    shift_held = 1 as libc::c_int != 0;
                    return 0o523 as libc::c_int;
                }
                98 => {
                    shift_held = 1 as libc::c_int != 0;
                    return 0o522 as libc::c_int;
                }
                99 => {
                    shift_held = 1 as libc::c_int != 0;
                    return 0o406 as libc::c_int;
                }
                100 => {
                    shift_held = 1 as libc::c_int != 0;
                    return 0o550 as libc::c_int;
                }
                _ => {}
            }
        } else if waiting_codes != 0
            && *nextcodes.offset(0 as libc::c_int as isize) != 0x1b as libc::c_int
            && (keycode == '[' as i32 || keycode == 'O' as i32)
        {
            keycode = parse_escape_sequence(keycode);
            meta_key = 1 as libc::c_int != 0;
        } else if '0' as i32 <= keycode
            && (keycode <= '2' as i32
                || keycode <= '9' as i32 && digit_count > 0 as libc::c_int)
        {
            let mut byte: libc::c_int = assemble_byte_code(keycode);
            if byte == -(44 as libc::c_int) {
                escapes = 2 as libc::c_int;
                return -(1 as libc::c_int);
            } else if byte > 0x7f as libc::c_int && using_utf8() as libc::c_int != 0 {
                if byte < 0xc0 as libc::c_int {
                    put_back(byte as libc::c_uchar as libc::c_int);
                    return 0xc2 as libc::c_int;
                } else {
                    put_back(
                        (byte - 0x40 as libc::c_int) as libc::c_uchar as libc::c_int,
                    );
                    return 0xc3 as libc::c_int;
                }
            } else if byte == '\t' as i32 || byte == 0x7f as libc::c_int {
                keycode = byte;
            } else {
                return byte
            }
        } else if digit_count == 0 as libc::c_int {
            if first_escape_was_alone as libc::c_int != 0 && !last_escape_was_alone {
                if !shifted_metas {
                    keycode = {
                        let mut __res: libc::c_int = 0;
                        if ::std::mem::size_of::<libc::c_int>() as libc::c_ulong
                            > 1 as libc::c_int as libc::c_ulong
                        {
                            if 0 != 0 {
                                let mut __c: libc::c_int = keycode;
                                __res = if __c < -(128 as libc::c_int)
                                    || __c > 255 as libc::c_int
                                {
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
                    };
                }
                meta_key = 1 as libc::c_int != 0;
            } else {
                keycode = convert_to_control(keycode);
            }
        }
    }
    if keycode == controlleft {
        return 0x401 as libc::c_int
    } else if keycode == controlright {
        return 0x402 as libc::c_int
    } else if keycode == controlup {
        return 0x403 as libc::c_int
    } else if keycode == controldown {
        return 0x404 as libc::c_int
    } else if keycode == controlhome {
        return 0x405 as libc::c_int
    } else if keycode == controlend {
        return 0x406 as libc::c_int
    } else if keycode == controldelete {
        return 0x40d as libc::c_int
    } else if keycode == controlshiftdelete {
        return 0x41d as libc::c_int
    } else if keycode == shiftup {
        shift_held = 1 as libc::c_int != 0;
        return 0o403 as libc::c_int;
    } else if keycode == shiftdown {
        shift_held = 1 as libc::c_int != 0;
        return 0o402 as libc::c_int;
    } else if keycode == shiftcontrolleft {
        shift_held = 1 as libc::c_int != 0;
        return 0x401 as libc::c_int;
    } else if keycode == shiftcontrolright {
        shift_held = 1 as libc::c_int != 0;
        return 0x402 as libc::c_int;
    } else if keycode == shiftcontrolup {
        shift_held = 1 as libc::c_int != 0;
        return 0x403 as libc::c_int;
    } else if keycode == shiftcontroldown {
        shift_held = 1 as libc::c_int != 0;
        return 0x404 as libc::c_int;
    } else if keycode == shiftcontrolhome {
        shift_held = 1 as libc::c_int != 0;
        return 0x405 as libc::c_int;
    } else if keycode == shiftcontrolend {
        shift_held = 1 as libc::c_int != 0;
        return 0x406 as libc::c_int;
    } else if keycode == altleft {
        return 0x421 as libc::c_int
    } else if keycode == altright {
        return 0x422 as libc::c_int
    } else if keycode == altup {
        return 0x423 as libc::c_int
    } else if keycode == altdown {
        return 0x424 as libc::c_int
    } else if keycode == altpageup {
        return 0x427 as libc::c_int
    } else if keycode == altpagedown {
        return 0x428 as libc::c_int
    } else if keycode == altinsert {
        return 0x42c as libc::c_int
    } else if keycode == altdelete {
        return 0x42d as libc::c_int
    } else if keycode == shiftaltleft {
        shift_held = 1 as libc::c_int != 0;
        return 0o406 as libc::c_int;
    } else if keycode == shiftaltright {
        shift_held = 1 as libc::c_int != 0;
        return 0o550 as libc::c_int;
    } else if keycode == shiftaltup {
        shift_held = 1 as libc::c_int != 0;
        return 0o523 as libc::c_int;
    } else if keycode == shiftaltdown {
        shift_held = 1 as libc::c_int != 0;
        return 0o522 as libc::c_int;
    } else if (0o410 as libc::c_int + 24 as libc::c_int) < keycode
        && keycode < 0o410 as libc::c_int + 64 as libc::c_int
    {
        return 0x4fc as libc::c_int
    }
    let mut modifiers: libc::c_uchar = 6 as libc::c_int as libc::c_uchar;
    if on_a_vt as libc::c_int != 0 && !mute_modifiers
        && ioctl(
            0 as libc::c_int,
            0x541c as libc::c_int as libc::c_ulong,
            &mut modifiers as *mut libc::c_uchar,
        ) >= 0 as libc::c_int
    {
        if modifiers as libc::c_int & 0x1 as libc::c_int != 0 {
            if keycode == '\t' as i32 {
                return 0x45f as libc::c_int;
            }
            if keycode == 0o512 as libc::c_int
                && modifiers as libc::c_int == 0x1 as libc::c_int
            {
                return 0x45d as libc::c_int;
            }
            if keycode == 0o512 as libc::c_int
                && modifiers as libc::c_int == 0x5 as libc::c_int
            {
                return 0x41d as libc::c_int;
            }
            if !meta_key {
                shift_held = 1 as libc::c_int != 0;
            }
        }
        if modifiers as libc::c_int == 0x8 as libc::c_int {
            match keycode {
                259 => return 0x423 as libc::c_int,
                258 => return 0x424 as libc::c_int,
                339 => return 0x427 as libc::c_int,
                338 => return 0x428 as libc::c_int,
                330 => return 0x42d as libc::c_int,
                331 => return 0x42c as libc::c_int,
                _ => {}
            }
        }
        if modifiers as libc::c_int & 0x4 as libc::c_int != 0 {
            match keycode {
                259 => return 0x403 as libc::c_int,
                258 => return 0x404 as libc::c_int,
                260 => return 0x401 as libc::c_int,
                261 => return 0x402 as libc::c_int,
                262 => return 0x405 as libc::c_int,
                360 => return 0x406 as libc::c_int,
                330 => return 0x40d as libc::c_int,
                _ => {}
            }
        }
        if modifiers as libc::c_int & 0x9 as libc::c_int == 0x9 as libc::c_int {
            match keycode {
                259 => return 0o523 as libc::c_int,
                258 => return 0o522 as libc::c_int,
                260 => return 0o406 as libc::c_int,
                261 => return 0o550 as libc::c_int,
                _ => {}
            }
        }
    }
    if keycode == '\t' as i32 && !((*openfile).mark).is_null()
        && currmenu == (1 as libc::c_int) << 0 as libc::c_int && !bracketed_paste
        && (*openfile).mark != (*openfile).current
    {
        return 0x4f1 as libc::c_int;
    }
    's_951: {
        let mut current_block_234: u64;
        match keycode {
            393 => {
                shift_held = 1 as libc::c_int != 0;
                return 0o404 as libc::c_int;
            }
            402 => {
                shift_held = 1 as libc::c_int != 0;
                return 0o405 as libc::c_int;
            }
            337 => {
                shift_held = 1 as libc::c_int != 0;
                return 0o403 as libc::c_int;
            }
            336 => {
                shift_held = 1 as libc::c_int != 0;
                return 0o402 as libc::c_int;
            }
            391 | 1109 => {
                shift_held = 1 as libc::c_int != 0;
                current_block_234 = 13501260415344254151;
            }
            348 => {
                current_block_234 = 13501260415344254151;
            }
            386 | 1110 => {
                shift_held = 1 as libc::c_int != 0;
                current_block_234 = 15464803012785988042;
            }
            351 => {
                current_block_234 = 15464803012785988042;
            }
            335 => return 0x406 as libc::c_int,
            398 | 1111 => {
                shift_held = 1 as libc::c_int != 0;
                current_block_234 = 2384955324668918263;
            }
            349 => {
                current_block_234 = 2384955324668918263;
            }
            396 | 1112 => {
                shift_held = 1 as libc::c_int != 0;
                current_block_234 = 8380916662644400154;
            }
            352 => {
                current_block_234 = 8380916662644400154;
            }
            127 | 263 => {
                return if flags[(REBIND_DELETE as libc::c_int as libc::c_ulong)
                    .wrapping_div(
                        (::std::mem::size_of::<libc::c_uint>() as libc::c_ulong)
                            .wrapping_mul(8 as libc::c_int as libc::c_ulong),
                    ) as usize]
                    & (1 as libc::c_int as libc::c_uint)
                        << (REBIND_DELETE as libc::c_int as libc::c_ulong)
                            .wrapping_rem(
                                (::std::mem::size_of::<libc::c_uint>() as libc::c_ulong)
                                    .wrapping_mul(8 as libc::c_int as libc::c_ulong),
                            ) != 0 as libc::c_int as libc::c_uint
                {
                    0o512 as libc::c_int
                } else {
                    0o407 as libc::c_int
                };
            }
            330 => {
                return if flags[(REBIND_DELETE as libc::c_int as libc::c_ulong)
                    .wrapping_div(
                        (::std::mem::size_of::<libc::c_uint>() as libc::c_ulong)
                            .wrapping_mul(8 as libc::c_int as libc::c_ulong),
                    ) as usize]
                    & (1 as libc::c_int as libc::c_uint)
                        << (REBIND_DELETE as libc::c_int as libc::c_ulong)
                            .wrapping_rem(
                                (::std::mem::size_of::<libc::c_uint>() as libc::c_ulong)
                                    .wrapping_mul(8 as libc::c_int as libc::c_ulong),
                            ) != 0 as libc::c_int as libc::c_uint
                {
                    0o407 as libc::c_int
                } else {
                    0o512 as libc::c_int
                };
            }
            383 => return 0x45d as libc::c_int,
            379 => return 0o543 as libc::c_int,
            405 | 407 => return 0x1a as libc::c_int,
            353 => return 0x45f as libc::c_int,
            378 | 354 | 350 | 410 | 1278 => return -(1 as libc::c_int),
            _ => {
                break 's_951;
            }
        }
        match current_block_234 {
            8380916662644400154 => return 0o522 as libc::c_int,
            15464803012785988042 => return 0o550 as libc::c_int,
            2384955324668918263 => return 0o523 as libc::c_int,
            _ => return 0o406 as libc::c_int,
        }
    }
    return keycode;
}
pub unsafe extern "C" fn get_kbinput(
    mut frame: *mut WINDOW,
    mut showcursor: bool,
) -> libc::c_int {
    let mut kbinput: libc::c_int = -(1 as libc::c_int);
    reveal_cursor = showcursor;
    while kbinput == -(1 as libc::c_int) {
        kbinput = parse_kbinput(frame);
    }
    if frame == midwin {
        check_statusblank();
    }
    return kbinput;
}
pub unsafe extern "C" fn assemble_unicode(mut symbol: libc::c_int) -> libc::c_long {
    static mut unicode: libc::c_long = 0 as libc::c_int as libc::c_long;
    static mut digits_0: libc::c_int = 0 as libc::c_int;
    let mut outcome: libc::c_long = -(44 as libc::c_int) as libc::c_long;
    if '0' as i32 <= symbol && symbol <= '9' as i32 {
        unicode = (unicode << 4 as libc::c_int) + symbol as libc::c_long
            - '0' as i32 as libc::c_long;
    } else if 'a' as i32 <= symbol | 0x20 as libc::c_int
        && symbol | 0x20 as libc::c_int <= 'f' as i32
    {
        unicode = (unicode << 4 as libc::c_int)
            + (symbol | 0x20 as libc::c_int) as libc::c_long - 'a' as i32 as libc::c_long
            + 10 as libc::c_int as libc::c_long;
    } else if symbol == '\r' as i32 || symbol == ' ' as i32 {
        outcome = unicode;
    } else {
        outcome = -(77 as libc::c_int) as libc::c_long;
    }
    digits_0 += 1;
    if digits_0 == 6 as libc::c_int && outcome == -(44 as libc::c_int) as libc::c_long {
        outcome = if unicode < 0x110000 as libc::c_int as libc::c_long {
            unicode
        } else {
            -(77 as libc::c_int) as libc::c_long
        };
    }
    if outcome == -(44 as libc::c_int) as libc::c_long
        && currmenu == (1 as libc::c_int) << 0 as libc::c_int
    {
        let mut partial: [libc::c_char; 7] = *::std::mem::transmute::<
            &[u8; 7],
            &mut [libc::c_char; 7],
        >(b"      \0");
        sprintf(
            partial
                .as_mut_ptr()
                .offset(6 as libc::c_int as isize)
                .offset(-(digits_0 as isize)),
            b"%0*lX\0" as *const u8 as *const libc::c_char,
            digits_0,
            unicode,
        );
        statusline(
            INFO,
            dcgettext(
                0 as *const libc::c_char,
                b"Unicode Input: %s\0" as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ),
            partial.as_mut_ptr(),
        );
    }
    if outcome != -(44 as libc::c_int) as libc::c_long {
        unicode = 0 as libc::c_int as libc::c_long;
        digits_0 = 0 as libc::c_int;
    }
    return outcome;
}
pub unsafe extern "C" fn parse_verbatim_kbinput(
    mut frame: *mut WINDOW,
    mut count: *mut size_t,
) -> *mut libc::c_int {
    let mut keycode: libc::c_int = 0;
    let mut yield_0: *mut libc::c_int = 0 as *mut libc::c_int;
    reveal_cursor = 1 as libc::c_int != 0;
    keycode = get_input(frame);
    if keycode == -(2 as libc::c_int) {
        *count = 999 as libc::c_int as size_t;
        return 0 as *mut libc::c_int;
    }
    yield_0 = nmalloc(
        (6 as libc::c_int as libc::c_ulong)
            .wrapping_mul(::std::mem::size_of::<libc::c_int>() as libc::c_ulong),
    ) as *mut libc::c_int;
    if using_utf8() as libc::c_int != 0
        && *(*__ctype_b_loc()).offset(keycode as isize) as libc::c_int
            & _ISxdigit as libc::c_int as libc::c_ushort as libc::c_int != 0
    {
        let mut unicode: libc::c_long = assemble_unicode(keycode);
        let vla = __ctype_get_mb_cur_max() as usize;
        let mut multibyte: Vec::<libc::c_char> = ::std::vec::from_elem(0, vla);
        reveal_cursor = 0 as libc::c_int != 0;
        while unicode == -(44 as libc::c_int) as libc::c_long {
            keycode = get_input(frame);
            unicode = assemble_unicode(keycode);
        }
        if keycode == -(2 as libc::c_int) {
            *count = 999 as libc::c_int as size_t;
            rpl_free(yield_0 as *mut libc::c_void);
            return 0 as *mut libc::c_int;
        }
        if unicode == -(77 as libc::c_int) as libc::c_long {
            if keycode == 0x1b as libc::c_int && waiting_codes != 0 {
                get_input(0 as *mut WINDOW);
                while waiting_codes != 0
                    && (0x1f as libc::c_int)
                        < *nextcodes.offset(0 as libc::c_int as isize)
                    && *nextcodes.offset(0 as libc::c_int as isize) < 0x40 as libc::c_int
                {
                    get_input(0 as *mut WINDOW);
                }
                if waiting_codes != 0
                    && (0x3f as libc::c_int)
                        < *nextcodes.offset(0 as libc::c_int as isize)
                    && *nextcodes.offset(0 as libc::c_int as isize) < 0x7f as libc::c_int
                {
                    get_input(0 as *mut WINDOW);
                }
            } else if 0xc0 as libc::c_int <= keycode && keycode <= 0xff as libc::c_int {
                while waiting_codes != 0
                    && (0x7f as libc::c_int)
                        < *nextcodes.offset(0 as libc::c_int as isize)
                    && *nextcodes.offset(0 as libc::c_int as isize) < 0xc0 as libc::c_int
                {
                    get_input(0 as *mut WINDOW);
                }
            }
        }
        *count = wctomb(multibyte.as_mut_ptr(), unicode as wchar_t) as size_t;
        if *count > 4 as libc::c_int as libc::c_ulong {
            *count = 0 as libc::c_int as size_t;
        }
        let mut i: size_t = 0 as libc::c_int as size_t;
        while i < *count {
            *yield_0
                .offset(
                    i as isize,
                ) = *multibyte.as_mut_ptr().offset(i as isize) as libc::c_int;
            i = i.wrapping_add(1);
            i;
        }
        return yield_0;
    }
    *yield_0.offset(0 as libc::c_int as isize) = keycode;
    if keycode == 0x1b as libc::c_int && waiting_codes != 0 {
        *yield_0.offset(1 as libc::c_int as isize) = get_input(0 as *mut WINDOW);
        *count = 2 as libc::c_int as size_t;
    }
    return yield_0;
}
pub unsafe extern "C" fn get_verbatim_kbinput(
    mut frame: *mut WINDOW,
    mut count: *mut size_t,
) -> *mut libc::c_char {
    let mut bytes: *mut libc::c_char = nmalloc(
        (4 as libc::c_int + 2 as libc::c_int) as size_t,
    ) as *mut libc::c_char;
    let mut input: *mut libc::c_int = 0 as *mut libc::c_int;
    if flags[(PRESERVE as libc::c_int as libc::c_ulong)
        .wrapping_div(
            (::std::mem::size_of::<libc::c_uint>() as libc::c_ulong)
                .wrapping_mul(8 as libc::c_int as libc::c_ulong),
        ) as usize]
        & (1 as libc::c_int as libc::c_uint)
            << (PRESERVE as libc::c_int as libc::c_ulong)
                .wrapping_rem(
                    (::std::mem::size_of::<libc::c_uint>() as libc::c_ulong)
                        .wrapping_mul(8 as libc::c_int as libc::c_ulong),
                ) != 0 as libc::c_int as libc::c_uint
    {
        disable_flow_control();
    }
    if !(flags[(RAW_SEQUENCES as libc::c_int as libc::c_ulong)
        .wrapping_div(
            (::std::mem::size_of::<libc::c_uint>() as libc::c_ulong)
                .wrapping_mul(8 as libc::c_int as libc::c_ulong),
        ) as usize]
        & (1 as libc::c_int as libc::c_uint)
            << (RAW_SEQUENCES as libc::c_int as libc::c_ulong)
                .wrapping_rem(
                    (::std::mem::size_of::<libc::c_uint>() as libc::c_ulong)
                        .wrapping_mul(8 as libc::c_int as libc::c_ulong),
                ) != 0 as libc::c_int as libc::c_uint)
    {
        keypad(frame, 0 as libc::c_int != 0);
    }
    printf(b"\x1B[?2004l\0" as *const u8 as *const libc::c_char);
    fflush(stdout);
    linger_after_escape = 1 as libc::c_int != 0;
    input = parse_verbatim_kbinput(frame, count);
    if !input.is_null() {
        if *input >= 0x80 as libc::c_int && *count == 1 as libc::c_int as libc::c_ulong {
            put_back(*input);
            *count = 999 as libc::c_int as size_t;
        } else if *input == '\n' as i32 && as_an_at as libc::c_int != 0
            || *input == '\0' as i32 && !as_an_at
        {
            *count = 0 as libc::c_int as size_t;
        }
    }
    linger_after_escape = 0 as libc::c_int != 0;
    printf(b"\x1B[?2004h\0" as *const u8 as *const libc::c_char);
    fflush(stdout);
    if flags[(PRESERVE as libc::c_int as libc::c_ulong)
        .wrapping_div(
            (::std::mem::size_of::<libc::c_uint>() as libc::c_ulong)
                .wrapping_mul(8 as libc::c_int as libc::c_ulong),
        ) as usize]
        & (1 as libc::c_int as libc::c_uint)
            << (PRESERVE as libc::c_int as libc::c_ulong)
                .wrapping_rem(
                    (::std::mem::size_of::<libc::c_uint>() as libc::c_ulong)
                        .wrapping_mul(8 as libc::c_int as libc::c_ulong),
                ) != 0 as libc::c_int as libc::c_uint
    {
        enable_flow_control();
    }
    if !(flags[(RAW_SEQUENCES as libc::c_int as libc::c_ulong)
        .wrapping_div(
            (::std::mem::size_of::<libc::c_uint>() as libc::c_ulong)
                .wrapping_mul(8 as libc::c_int as libc::c_ulong),
        ) as usize]
        & (1 as libc::c_int as libc::c_uint)
            << (RAW_SEQUENCES as libc::c_int as libc::c_ulong)
                .wrapping_rem(
                    (::std::mem::size_of::<libc::c_uint>() as libc::c_ulong)
                        .wrapping_mul(8 as libc::c_int as libc::c_ulong),
                ) != 0 as libc::c_int as libc::c_uint)
    {
        keypad(midwin, 1 as libc::c_int != 0);
        keypad(footwin, 1 as libc::c_int != 0);
    }
    if *count < 999 as libc::c_int as libc::c_ulong {
        let mut i: size_t = 0 as libc::c_int as size_t;
        while i < *count {
            *bytes.offset(i as isize) = *input.offset(i as isize) as libc::c_char;
            i = i.wrapping_add(1);
            i;
        }
        *bytes.offset(*count as isize) = '\0' as i32 as libc::c_char;
    }
    rpl_free(input as *mut libc::c_void);
    return bytes;
}
pub unsafe extern "C" fn get_mouseinput(
    mut mouse_y: *mut libc::c_int,
    mut mouse_x: *mut libc::c_int,
    mut allow_shortcuts: bool,
) -> libc::c_int {
    let mut in_middle: bool = false;
    let mut in_footer: bool = false;
    let mut event: MEVENT = MEVENT {
        id: 0,
        x: 0,
        y: 0,
        z: 0,
        bstate: 0,
    };
    if getmouse(&mut event) == -(1 as libc::c_int) {
        return -(1 as libc::c_int);
    }
    in_middle = wenclose(midwin, event.y, event.x);
    in_footer = wenclose(footwin, event.y, event.x);
    *mouse_x = event.x
        - (if in_middle as libc::c_int != 0 { margin } else { 0 as libc::c_int });
    *mouse_y = event.y;
    if event.bstate as libc::c_long
        & ((0o1 as libc::c_long)
            << (1 as libc::c_int - 1 as libc::c_int) * 5 as libc::c_int
            | (0o4 as libc::c_long)
                << (1 as libc::c_int - 1 as libc::c_int) * 5 as libc::c_int) != 0
    {
        if allow_shortcuts as libc::c_int != 0
            && !(flags[(NO_HELP as libc::c_int as libc::c_ulong)
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
            && in_footer as libc::c_int != 0
        {
            let mut width: libc::c_int = 0;
            let mut index: libc::c_int = 0;
            let mut number: size_t = 0;
            wmouse_trafo(footwin, mouse_y, mouse_x, 0 as libc::c_int != 0);
            if *mouse_y == 0 as libc::c_int {
                *mouse_x = event.x;
                *mouse_y = event.y;
                return 0 as libc::c_int;
            }
            number = shown_entries_for(currmenu);
            if number < 5 as libc::c_int as libc::c_ulong {
                width = COLS / 2 as libc::c_int;
            } else {
                width = (COLS as libc::c_ulong)
                    .wrapping_div(
                        number
                            .wrapping_add(1 as libc::c_int as libc::c_ulong)
                            .wrapping_div(2 as libc::c_int as libc::c_ulong),
                    ) as libc::c_int;
            }
            index = *mouse_x / width * 2 as libc::c_int + *mouse_y;
            if index as libc::c_ulong > number && *mouse_x % width < COLS % width {
                index -= 2 as libc::c_int;
            }
            if index as libc::c_ulong > number {
                return 2 as libc::c_int;
            }
            let mut f: *mut funcstruct = allfuncs;
            while !f.is_null() {
                if !((*f).menus & currmenu == 0 as libc::c_int) {
                    if !(first_sc_for(currmenu, (*f).func)).is_null() {
                        index -= 1;
                        if index == 0 as libc::c_int {
                            let mut shortcut: *const keystruct = first_sc_for(
                                currmenu,
                                (*f).func,
                            );
                            put_back((*shortcut).keycode);
                            if 0x20 as libc::c_int <= (*shortcut).keycode
                                && (*shortcut).keycode <= 0x7e as libc::c_int
                            {
                                put_back(0x1b as libc::c_int);
                            }
                            break;
                        }
                    }
                }
                f = (*f).next;
            }
            return 1 as libc::c_int;
        } else {
            return 0 as libc::c_int
        }
    } else if event.bstate as libc::c_long
        & ((0o2 as libc::c_long)
            << (4 as libc::c_int - 1 as libc::c_int) * 5 as libc::c_int
            | (0o2 as libc::c_long)
                << (5 as libc::c_int - 1 as libc::c_int) * 5 as libc::c_int) != 0
    {
        if in_footer {
            wmouse_trafo(footwin, mouse_y, mouse_x, 0 as libc::c_int != 0);
        }
        if in_middle as libc::c_int != 0
            || in_footer as libc::c_int != 0 && *mouse_y == 0 as libc::c_int
        {
            let mut keycode: libc::c_int = if event.bstate as libc::c_long
                & (0o2 as libc::c_long)
                    << (4 as libc::c_int - 1 as libc::c_int) * 5 as libc::c_int != 0
            {
                0o403 as libc::c_int
            } else {
                0o402 as libc::c_int
            };
            let mut count: libc::c_int = 3 as libc::c_int;
            while count > 0 as libc::c_int {
                put_back(keycode);
                count -= 1;
                count;
            }
            return 1 as libc::c_int;
        } else {
            return 2 as libc::c_int
        }
    }
    return 2 as libc::c_int;
}
pub unsafe extern "C" fn blank_row(mut window: *mut WINDOW, mut row: libc::c_int) {
    wmove(window, row, 0 as libc::c_int);
    wclrtoeol(window);
}
pub unsafe extern "C" fn blank_titlebar() {
    mvwprintw(
        topwin,
        0 as libc::c_int,
        0 as libc::c_int,
        b"%*s\0" as *const u8 as *const libc::c_char,
        COLS,
        b" \0" as *const u8 as *const libc::c_char,
    );
}
pub unsafe extern "C" fn blank_edit() {
    let mut row: libc::c_int = 0 as libc::c_int;
    while row < editwinrows {
        blank_row(midwin, row);
        row += 1;
        row;
    }
}
pub unsafe extern "C" fn blank_statusbar() {
    blank_row(footwin, 0 as libc::c_int);
}
pub unsafe extern "C" fn wipe_statusbar() {
    lastmessage = VACUUM;
    if (flags[(ZERO as libc::c_int as libc::c_ulong)
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
        || flags[(MINIBAR as libc::c_int as libc::c_ulong)
            .wrapping_div(
                (::std::mem::size_of::<libc::c_uint>() as libc::c_ulong)
                    .wrapping_mul(8 as libc::c_int as libc::c_ulong),
            ) as usize]
            & (1 as libc::c_int as libc::c_uint)
                << (MINIBAR as libc::c_int as libc::c_ulong)
                    .wrapping_rem(
                        (::std::mem::size_of::<libc::c_uint>() as libc::c_ulong)
                            .wrapping_mul(8 as libc::c_int as libc::c_ulong),
                    ) != 0 as libc::c_int as libc::c_uint || LINES == 1 as libc::c_int)
        && currmenu == (1 as libc::c_int) << 0 as libc::c_int
    {
        return;
    }
    blank_row(footwin, 0 as libc::c_int);
    wnoutrefresh(footwin);
}
pub unsafe extern "C" fn blank_bottombars() {
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
                ) != 0 as libc::c_int as libc::c_uint) && LINES > 5 as libc::c_int
    {
        blank_row(footwin, 1 as libc::c_int);
        blank_row(footwin, 2 as libc::c_int);
    }
}
pub unsafe extern "C" fn check_statusblank() {
    if statusblank == 0 as libc::c_int {
        return;
    }
    statusblank -= 1;
    if statusblank == 0 as libc::c_int {
        wipe_statusbar();
    }
    if currmenu == (1 as libc::c_int) << 0 as libc::c_int
        && (flags[(ZERO as libc::c_int as libc::c_ulong)
            .wrapping_div(
                (::std::mem::size_of::<libc::c_uint>() as libc::c_ulong)
                    .wrapping_mul(8 as libc::c_int as libc::c_ulong),
            ) as usize]
            & (1 as libc::c_int as libc::c_uint)
                << (ZERO as libc::c_int as libc::c_ulong)
                    .wrapping_rem(
                        (::std::mem::size_of::<libc::c_uint>() as libc::c_ulong)
                            .wrapping_mul(8 as libc::c_int as libc::c_ulong),
                    ) != 0 as libc::c_int as libc::c_uint || LINES == 1 as libc::c_int)
    {
        wredrawln(midwin, editwinrows - 1 as libc::c_int, 1 as libc::c_int);
        wnoutrefresh(midwin);
    }
}
pub unsafe extern "C" fn set_blankdelay_to_one() {
    statusblank = 1 as libc::c_int;
}
pub unsafe extern "C" fn display_string(
    mut text: *const libc::c_char,
    mut column: size_t,
    mut span: size_t,
    mut isdata: bool,
    mut isprompt: bool,
) -> *mut libc::c_char {
    let mut origin: *const libc::c_char = text;
    let mut start_x: size_t = actual_x(text, column);
    let mut start_col: size_t = wideness(text, start_x);
    let mut stowaways: size_t = 20 as libc::c_int as size_t;
    let mut allocsize: size_t = (COLS as libc::c_ulong)
        .wrapping_add(stowaways)
        .wrapping_mul(4 as libc::c_int as libc::c_ulong)
        .wrapping_add(1 as libc::c_int as libc::c_ulong);
    let mut converted: *mut libc::c_char = nmalloc(allocsize) as *mut libc::c_char;
    let mut index: size_t = 0 as libc::c_int as size_t;
    let mut beyond: size_t = column.wrapping_add(span);
    text = text.offset(start_x as isize);
    if span > !(0 as libc::c_int as size_t) >> 1 as libc::c_int {
        statusline(
            ALERT,
            b"Span has underflowed -- please report a bug\0" as *const u8
                as *const libc::c_char,
        );
        *converted.offset(0 as libc::c_int as isize) = '\0' as i32 as libc::c_char;
        return converted;
    }
    if (start_col < column
        || start_col > 0 as libc::c_int as libc::c_ulong && isdata as libc::c_int != 0
            && !(flags[(SOFTWRAP as libc::c_int as libc::c_ulong)
                .wrapping_div(
                    (::std::mem::size_of::<libc::c_uint>() as libc::c_ulong)
                        .wrapping_mul(8 as libc::c_int as libc::c_ulong),
                ) as usize]
                & (1 as libc::c_int as libc::c_uint)
                    << (SOFTWRAP as libc::c_int as libc::c_ulong)
                        .wrapping_rem(
                            (::std::mem::size_of::<libc::c_uint>() as libc::c_ulong)
                                .wrapping_mul(8 as libc::c_int as libc::c_ulong),
                        ) != 0 as libc::c_int as libc::c_uint))
        && *text as libc::c_int != '\0' as i32 && *text as libc::c_int != '\t' as i32
    {
        if is_cntrl_char(text) {
            if start_col < column {
                let fresh2 = index;
                index = index.wrapping_add(1);
                *converted.offset(fresh2 as isize) = control_mbrep(text, isdata);
                column = column.wrapping_add(1);
                column;
                text = text.offset(char_length(text) as isize);
            }
        } else if is_doublewidth(text) {
            if start_col == column {
                let fresh3 = index;
                index = index.wrapping_add(1);
                *converted.offset(fresh3 as isize) = ' ' as i32 as libc::c_char;
                column = column.wrapping_add(1);
                column;
            }
            let fresh4 = index;
            index = index.wrapping_add(1);
            *converted.offset(fresh4 as isize) = ']' as i32 as libc::c_char;
            column = column.wrapping_add(1);
            column;
            text = text.offset(char_length(text) as isize);
        }
    }
    while *text as libc::c_int != '\0' as i32
        && (column < beyond || is_zerowidth(text) as libc::c_int != 0)
    {
        if *text as libc::c_schar as libc::c_int > 0x20 as libc::c_int
            && *text as libc::c_int != 0x7f as libc::c_int || 0 as libc::c_int != 0
        {
            let fresh5 = text;
            text = text.offset(1);
            let fresh6 = index;
            index = index.wrapping_add(1);
            *converted.offset(fresh6 as isize) = *fresh5;
            column = column.wrapping_add(1);
            column;
        } else if *text as libc::c_int == ' ' as i32 {
            if flags[(WHITESPACE_DISPLAY as libc::c_int as libc::c_ulong)
                .wrapping_div(
                    (::std::mem::size_of::<libc::c_uint>() as libc::c_ulong)
                        .wrapping_mul(8 as libc::c_int as libc::c_ulong),
                ) as usize]
                & (1 as libc::c_int as libc::c_uint)
                    << (WHITESPACE_DISPLAY as libc::c_int as libc::c_ulong)
                        .wrapping_rem(
                            (::std::mem::size_of::<libc::c_uint>() as libc::c_ulong)
                                .wrapping_mul(8 as libc::c_int as libc::c_ulong),
                        ) != 0 as libc::c_int as libc::c_uint
            {
                let mut i: libc::c_int = whitelen[0 as libc::c_int as usize];
                while i
                    < whitelen[0 as libc::c_int as usize]
                        + whitelen[1 as libc::c_int as usize]
                {
                    let fresh7 = i;
                    i = i + 1;
                    let fresh8 = index;
                    index = index.wrapping_add(1);
                    *converted
                        .offset(fresh8 as isize) = *whitespace.offset(fresh7 as isize);
                }
            } else {
                let fresh9 = index;
                index = index.wrapping_add(1);
                *converted.offset(fresh9 as isize) = ' ' as i32 as libc::c_char;
            }
            column = column.wrapping_add(1);
            column;
            text = text.offset(1);
            text;
        } else if *text as libc::c_int == '\t' as i32 {
            if flags[(WHITESPACE_DISPLAY as libc::c_int as libc::c_ulong)
                .wrapping_div(
                    (::std::mem::size_of::<libc::c_uint>() as libc::c_ulong)
                        .wrapping_mul(8 as libc::c_int as libc::c_ulong),
                ) as usize]
                & (1 as libc::c_int as libc::c_uint)
                    << (WHITESPACE_DISPLAY as libc::c_int as libc::c_ulong)
                        .wrapping_rem(
                            (::std::mem::size_of::<libc::c_uint>() as libc::c_ulong)
                                .wrapping_mul(8 as libc::c_int as libc::c_ulong),
                        ) != 0 as libc::c_int as libc::c_uint
                && (index > 0 as libc::c_int as libc::c_ulong || !isdata
                    || !(flags[(SOFTWRAP as libc::c_int as libc::c_ulong)
                        .wrapping_div(
                            (::std::mem::size_of::<libc::c_uint>() as libc::c_ulong)
                                .wrapping_mul(8 as libc::c_int as libc::c_ulong),
                        ) as usize]
                        & (1 as libc::c_int as libc::c_uint)
                            << (SOFTWRAP as libc::c_int as libc::c_ulong)
                                .wrapping_rem(
                                    (::std::mem::size_of::<libc::c_uint>() as libc::c_ulong)
                                        .wrapping_mul(8 as libc::c_int as libc::c_ulong),
                                ) != 0 as libc::c_int as libc::c_uint)
                    || column.wrapping_rem(tabsize as libc::c_ulong)
                        == 0 as libc::c_int as libc::c_ulong || column == start_col)
            {
                let mut i_0: libc::c_int = 0 as libc::c_int;
                while i_0 < whitelen[0 as libc::c_int as usize] {
                    let fresh10 = i_0;
                    i_0 = i_0 + 1;
                    let fresh11 = index;
                    index = index.wrapping_add(1);
                    *converted
                        .offset(fresh11 as isize) = *whitespace.offset(fresh10 as isize);
                }
            } else {
                let fresh12 = index;
                index = index.wrapping_add(1);
                *converted.offset(fresh12 as isize) = ' ' as i32 as libc::c_char;
            }
            column = column.wrapping_add(1);
            column;
            while column.wrapping_rem(tabsize as libc::c_ulong)
                != 0 as libc::c_int as libc::c_ulong && column < beyond
            {
                let fresh13 = index;
                index = index.wrapping_add(1);
                *converted.offset(fresh13 as isize) = ' ' as i32 as libc::c_char;
                column = column.wrapping_add(1);
                column;
            }
            text = text.offset(1);
            text;
        } else if is_cntrl_char(text) {
            let fresh14 = index;
            index = index.wrapping_add(1);
            *converted.offset(fresh14 as isize) = '^' as i32 as libc::c_char;
            let fresh15 = index;
            index = index.wrapping_add(1);
            *converted.offset(fresh15 as isize) = control_mbrep(text, isdata);
            text = text.offset(char_length(text) as isize);
            column = (column as libc::c_ulong)
                .wrapping_add(2 as libc::c_int as libc::c_ulong) as size_t as size_t;
        } else {
            let mut charlength: libc::c_int = 0;
            let mut charwidth: libc::c_int = 0;
            let mut wc: wchar_t = 0;
            charlength = mbtowide(&mut wc, text);
            if charlength < 0 as libc::c_int {
                let fresh16 = index;
                index = index.wrapping_add(1);
                *converted.offset(fresh16 as isize) = -17i32 as libc::c_char;
                let fresh17 = index;
                index = index.wrapping_add(1);
                *converted.offset(fresh17 as isize) = -65i32 as libc::c_char;
                let fresh18 = index;
                index = index.wrapping_add(1);
                *converted.offset(fresh18 as isize) = -67i32 as libc::c_char;
                text = text.offset(1);
                text;
                column = column.wrapping_add(1);
                column;
            } else {
                charwidth = wcwidth(wc);
                if charwidth == 0 as libc::c_int
                    && {
                        stowaways = stowaways.wrapping_sub(1);
                        stowaways == 0 as libc::c_int as libc::c_ulong
                    }
                {
                    stowaways = 40 as libc::c_int as size_t;
                    allocsize = (allocsize as libc::c_ulong)
                        .wrapping_add(
                            stowaways.wrapping_mul(4 as libc::c_int as libc::c_ulong),
                        ) as size_t as size_t;
                    converted = nrealloc(converted as *mut libc::c_void, allocsize)
                        as *mut libc::c_char;
                }
                if on_a_vt as libc::c_int != 0 && charwidth == 0 as libc::c_int {
                    text = text.offset(charlength as isize);
                } else {
                    while charlength > 0 as libc::c_int {
                        let fresh19 = text;
                        text = text.offset(1);
                        let fresh20 = index;
                        index = index.wrapping_add(1);
                        *converted.offset(fresh20 as isize) = *fresh19;
                        charlength -= 1;
                        charlength;
                    }
                    column = (column as libc::c_ulong)
                        .wrapping_add(
                            (if charwidth < 0 as libc::c_int {
                                1 as libc::c_int
                            } else {
                                charwidth
                            }) as libc::c_ulong,
                        ) as size_t as size_t;
                }
            }
        }
    }
    if column > beyond
        || *text as libc::c_int != '\0' as i32
            && (isprompt as libc::c_int != 0
                || isdata as libc::c_int != 0
                    && !(flags[(SOFTWRAP as libc::c_int as libc::c_ulong)
                        .wrapping_div(
                            (::std::mem::size_of::<libc::c_uint>() as libc::c_ulong)
                                .wrapping_mul(8 as libc::c_int as libc::c_ulong),
                        ) as usize]
                        & (1 as libc::c_int as libc::c_uint)
                            << (SOFTWRAP as libc::c_int as libc::c_ulong)
                                .wrapping_rem(
                                    (::std::mem::size_of::<libc::c_uint>() as libc::c_ulong)
                                        .wrapping_mul(8 as libc::c_int as libc::c_ulong),
                                ) != 0 as libc::c_int as libc::c_uint))
    {
        loop {
            index = step_left(converted, index);
            if !is_zerowidth(converted.offset(index as isize)) {
                break;
            }
        }
        if is_doublewidth(converted.offset(index as isize)) {
            let fresh21 = index;
            index = index.wrapping_add(1);
            *converted.offset(fresh21 as isize) = '[' as i32 as libc::c_char;
        }
        has_more = 1 as libc::c_int != 0;
    } else {
        has_more = 0 as libc::c_int != 0;
    }
    is_shorter = column < beyond;
    *converted.offset(index as isize) = '\0' as i32 as libc::c_char;
    from_x = start_x;
    till_x = text.offset_from(origin) as libc::c_long as size_t;
    return converted;
}
pub unsafe extern "C" fn buffer_number(mut buffer: *mut openfilestruct) -> libc::c_int {
    let mut count: libc::c_int = 1 as libc::c_int;
    while buffer != startfile {
        buffer = (*buffer).prev;
        count += 1;
        count;
    }
    return count;
}
pub unsafe extern "C" fn show_states_at(mut window: *mut WINDOW) {
    waddnstr(
        window,
        if flags[(AUTOINDENT as libc::c_int as libc::c_ulong)
            .wrapping_div(
                (::std::mem::size_of::<libc::c_uint>() as libc::c_ulong)
                    .wrapping_mul(8 as libc::c_int as libc::c_ulong),
            ) as usize]
            & (1 as libc::c_int as libc::c_uint)
                << (AUTOINDENT as libc::c_int as libc::c_ulong)
                    .wrapping_rem(
                        (::std::mem::size_of::<libc::c_uint>() as libc::c_ulong)
                            .wrapping_mul(8 as libc::c_int as libc::c_ulong),
                    ) != 0 as libc::c_int as libc::c_uint
        {
            b"I\0" as *const u8 as *const libc::c_char
        } else {
            b" \0" as *const u8 as *const libc::c_char
        },
        -(1 as libc::c_int),
    );
    waddnstr(
        window,
        if !((*openfile).mark).is_null() {
            b"M\0" as *const u8 as *const libc::c_char
        } else {
            b" \0" as *const u8 as *const libc::c_char
        },
        -(1 as libc::c_int),
    );
    waddnstr(
        window,
        if flags[(BREAK_LONG_LINES as libc::c_int as libc::c_ulong)
            .wrapping_div(
                (::std::mem::size_of::<libc::c_uint>() as libc::c_ulong)
                    .wrapping_mul(8 as libc::c_int as libc::c_ulong),
            ) as usize]
            & (1 as libc::c_int as libc::c_uint)
                << (BREAK_LONG_LINES as libc::c_int as libc::c_ulong)
                    .wrapping_rem(
                        (::std::mem::size_of::<libc::c_uint>() as libc::c_ulong)
                            .wrapping_mul(8 as libc::c_int as libc::c_ulong),
                    ) != 0 as libc::c_int as libc::c_uint
        {
            b"L\0" as *const u8 as *const libc::c_char
        } else {
            b" \0" as *const u8 as *const libc::c_char
        },
        -(1 as libc::c_int),
    );
    waddnstr(
        window,
        if recording as libc::c_int != 0 {
            b"R\0" as *const u8 as *const libc::c_char
        } else {
            b" \0" as *const u8 as *const libc::c_char
        },
        -(1 as libc::c_int),
    );
    waddnstr(
        window,
        if flags[(SOFTWRAP as libc::c_int as libc::c_ulong)
            .wrapping_div(
                (::std::mem::size_of::<libc::c_uint>() as libc::c_ulong)
                    .wrapping_mul(8 as libc::c_int as libc::c_ulong),
            ) as usize]
            & (1 as libc::c_int as libc::c_uint)
                << (SOFTWRAP as libc::c_int as libc::c_ulong)
                    .wrapping_rem(
                        (::std::mem::size_of::<libc::c_uint>() as libc::c_ulong)
                            .wrapping_mul(8 as libc::c_int as libc::c_ulong),
                    ) != 0 as libc::c_int as libc::c_uint
        {
            b"S\0" as *const u8 as *const libc::c_char
        } else {
            b" \0" as *const u8 as *const libc::c_char
        },
        -(1 as libc::c_int),
    );
}
pub unsafe extern "C" fn titlebar(mut path: *const libc::c_char) {
    let mut verlen: size_t = 0;
    let mut prefixlen: size_t = 0;
    let mut pathlen: size_t = 0;
    let mut statelen: size_t = 0;
    let mut pluglen: size_t = 0 as libc::c_int as size_t;
    let mut offset: size_t = 0 as libc::c_int as size_t;
    let mut upperleft: *const libc::c_char = b"\0" as *const u8 as *const libc::c_char;
    let mut prefix: *const libc::c_char = b"\0" as *const u8 as *const libc::c_char;
    let mut state: *const libc::c_char = b"\0" as *const u8 as *const libc::c_char;
    let mut caption: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut ranking: *mut libc::c_char = 0 as *mut libc::c_char;
    if topwin.is_null() {
        return;
    }
    wattr_on(
        topwin,
        interface_color_pair[TITLE_BAR as libc::c_int as usize] as attr_t,
        0 as *mut libc::c_void,
    );
    blank_titlebar();
    as_an_at = 0 as libc::c_int != 0;
    if currmenu == (1 as libc::c_int) << 14 as libc::c_int {
        prefix = dcgettext(
            0 as *const libc::c_char,
            b"Linting --\0" as *const u8 as *const libc::c_char,
            5 as libc::c_int,
        );
        path = (*openfile).filename;
    } else if !inhelp && !path.is_null() {
        prefix = dcgettext(
            0 as *const libc::c_char,
            b"DIR:\0" as *const u8 as *const libc::c_char,
            5 as libc::c_int,
        );
    } else if !inhelp {
        if more_than_one {
            ranking = nmalloc(24 as libc::c_int as size_t) as *mut libc::c_char;
            sprintf(
                ranking,
                b"[%i/%i]\0" as *const u8 as *const libc::c_char,
                buffer_number(openfile),
                buffer_number((*startfile).prev),
            );
            upperleft = ranking;
        } else {
            upperleft = b"GNU nano 7.2\0" as *const u8 as *const libc::c_char;
        }
        if *((*openfile).filename).offset(0 as libc::c_int as isize) as libc::c_int
            == '\0' as i32
        {
            path = dcgettext(
                0 as *const libc::c_char,
                b"New Buffer\0" as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            );
        } else {
            path = (*openfile).filename;
        }
        if flags[(VIEW_MODE as libc::c_int as libc::c_ulong)
            .wrapping_div(
                (::std::mem::size_of::<libc::c_uint>() as libc::c_ulong)
                    .wrapping_mul(8 as libc::c_int as libc::c_ulong),
            ) as usize]
            & (1 as libc::c_int as libc::c_uint)
                << (VIEW_MODE as libc::c_int as libc::c_ulong)
                    .wrapping_rem(
                        (::std::mem::size_of::<libc::c_uint>() as libc::c_ulong)
                            .wrapping_mul(8 as libc::c_int as libc::c_ulong),
                    ) != 0 as libc::c_int as libc::c_uint
        {
            state = dcgettext(
                0 as *const libc::c_char,
                b"View\0" as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            );
        } else if flags[(STATEFLAGS as libc::c_int as libc::c_ulong)
            .wrapping_div(
                (::std::mem::size_of::<libc::c_uint>() as libc::c_ulong)
                    .wrapping_mul(8 as libc::c_int as libc::c_ulong),
            ) as usize]
            & (1 as libc::c_int as libc::c_uint)
                << (STATEFLAGS as libc::c_int as libc::c_ulong)
                    .wrapping_rem(
                        (::std::mem::size_of::<libc::c_uint>() as libc::c_ulong)
                            .wrapping_mul(8 as libc::c_int as libc::c_ulong),
                    ) != 0 as libc::c_int as libc::c_uint
        {
            state = b"+.xxxxx\0" as *const u8 as *const libc::c_char;
        } else if (*openfile).modified {
            state = dcgettext(
                0 as *const libc::c_char,
                b"Modified\0" as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            );
        } else if flags[(RESTRICTED as libc::c_int as libc::c_ulong)
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
        {
            state = dcgettext(
                0 as *const libc::c_char,
                b"Restricted\0" as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            );
        } else {
            pluglen = (breadth(
                dcgettext(
                    0 as *const libc::c_char,
                    b"Modified\0" as *const u8 as *const libc::c_char,
                    5 as libc::c_int,
                ),
            ))
                .wrapping_add(1 as libc::c_int as libc::c_ulong);
        }
    }
    verlen = (breadth(upperleft)).wrapping_add(3 as libc::c_int as libc::c_ulong);
    prefixlen = breadth(prefix);
    if prefixlen > 0 as libc::c_int as libc::c_ulong {
        prefixlen = prefixlen.wrapping_add(1);
        prefixlen;
    }
    pathlen = breadth(path);
    statelen = (breadth(state)).wrapping_add(2 as libc::c_int as libc::c_ulong);
    if statelen > 2 as libc::c_int as libc::c_ulong {
        pathlen = pathlen.wrapping_add(1);
        pathlen;
    }
    if verlen
        .wrapping_add(prefixlen)
        .wrapping_add(pathlen)
        .wrapping_add(pluglen)
        .wrapping_add(statelen) <= COLS as libc::c_ulong
    {
        if wmove(topwin, 0 as libc::c_int, 2 as libc::c_int) == -(1 as libc::c_int) {
            -(1 as libc::c_int);
        } else {
            waddnstr(topwin, upperleft, -(1 as libc::c_int));
        };
    } else {
        verlen = 2 as libc::c_int as size_t;
        if verlen
            .wrapping_add(prefixlen)
            .wrapping_add(pathlen)
            .wrapping_add(pluglen)
            .wrapping_add(statelen) > COLS as libc::c_ulong
        {
            pluglen = 0 as libc::c_int as size_t;
        }
        if verlen
            .wrapping_add(prefixlen)
            .wrapping_add(pathlen)
            .wrapping_add(pluglen)
            .wrapping_add(statelen) > COLS as libc::c_ulong
        {
            verlen = 0 as libc::c_int as size_t;
            statelen = (statelen as libc::c_ulong)
                .wrapping_sub(2 as libc::c_int as libc::c_ulong) as size_t as size_t;
        }
    }
    rpl_free(ranking as *mut libc::c_void);
    if verlen > 0 as libc::c_int as libc::c_ulong {
        offset = verlen
            .wrapping_add(
                (COLS as libc::c_ulong)
                    .wrapping_sub(verlen.wrapping_add(pluglen).wrapping_add(statelen))
                    .wrapping_sub(prefixlen.wrapping_add(pathlen))
                    .wrapping_div(2 as libc::c_int as libc::c_ulong),
            );
    }
    if verlen
        .wrapping_add(prefixlen)
        .wrapping_add(pathlen)
        .wrapping_add(pluglen)
        .wrapping_add(statelen) <= COLS as libc::c_ulong
    {
        if wmove(topwin, 0 as libc::c_int, offset as libc::c_int) == -(1 as libc::c_int)
        {
            -(1 as libc::c_int);
        } else {
            waddnstr(topwin, prefix, -(1 as libc::c_int));
        };
        if prefixlen > 0 as libc::c_int as libc::c_ulong {
            waddnstr(
                topwin,
                b" \0" as *const u8 as *const libc::c_char,
                -(1 as libc::c_int),
            );
        }
    } else {
        wmove(topwin, 0 as libc::c_int, offset as libc::c_int);
    }
    if pathlen.wrapping_add(pluglen).wrapping_add(statelen) <= COLS as libc::c_ulong {
        caption = display_string(
            path,
            0 as libc::c_int as size_t,
            pathlen,
            0 as libc::c_int != 0,
            0 as libc::c_int != 0,
        );
        waddnstr(topwin, caption, -(1 as libc::c_int));
        rpl_free(caption as *mut libc::c_void);
    } else if (5 as libc::c_int as libc::c_ulong).wrapping_add(statelen)
        <= COLS as libc::c_ulong
    {
        waddnstr(
            topwin,
            b"...\0" as *const u8 as *const libc::c_char,
            -(1 as libc::c_int),
        );
        caption = display_string(
            path,
            (3 as libc::c_int as libc::c_ulong)
                .wrapping_add(pathlen)
                .wrapping_sub(COLS as libc::c_ulong)
                .wrapping_add(statelen),
            (COLS as libc::c_ulong).wrapping_sub(statelen),
            0 as libc::c_int != 0,
            0 as libc::c_int != 0,
        );
        waddnstr(topwin, caption, -(1 as libc::c_int));
        rpl_free(caption as *mut libc::c_void);
    }
    if *state as libc::c_int != 0
        && flags[(STATEFLAGS as libc::c_int as libc::c_ulong)
            .wrapping_div(
                (::std::mem::size_of::<libc::c_uint>() as libc::c_ulong)
                    .wrapping_mul(8 as libc::c_int as libc::c_ulong),
            ) as usize]
            & (1 as libc::c_int as libc::c_uint)
                << (STATEFLAGS as libc::c_int as libc::c_ulong)
                    .wrapping_rem(
                        (::std::mem::size_of::<libc::c_uint>() as libc::c_ulong)
                            .wrapping_mul(8 as libc::c_int as libc::c_ulong),
                    ) != 0 as libc::c_int as libc::c_uint
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
        if (*openfile).modified as libc::c_int != 0 && COLS > 1 as libc::c_int {
            waddnstr(
                topwin,
                b" *\0" as *const u8 as *const libc::c_char,
                -(1 as libc::c_int),
            );
        }
        if statelen < COLS as libc::c_ulong {
            wmove(
                topwin,
                0 as libc::c_int,
                ((COLS + 2 as libc::c_int) as libc::c_ulong).wrapping_sub(statelen)
                    as libc::c_int,
            );
            show_states_at(topwin);
        }
    } else if statelen > 0 as libc::c_int as libc::c_ulong
        && statelen <= COLS as libc::c_ulong
    {
        if wmove(
            topwin,
            0 as libc::c_int,
            (COLS as libc::c_ulong).wrapping_sub(statelen) as libc::c_int,
        ) == -(1 as libc::c_int)
        {
            -(1 as libc::c_int);
        } else {
            waddnstr(topwin, state, -(1 as libc::c_int));
        };
    } else if statelen > 0 as libc::c_int as libc::c_ulong {
        if wmove(topwin, 0 as libc::c_int, 0 as libc::c_int) == -(1 as libc::c_int) {
            -(1 as libc::c_int);
        } else {
            waddnstr(topwin, state, actual_x(state, COLS as size_t) as libc::c_int);
        };
    }
    wattr_off(
        topwin,
        interface_color_pair[TITLE_BAR as libc::c_int as usize] as attr_t,
        0 as *mut libc::c_void,
    );
    wrefresh(topwin);
}
pub unsafe extern "C" fn minibar() {
    let mut thename: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut number_of_lines: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut ranking: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut location: *mut libc::c_char = nmalloc(44 as libc::c_int as size_t)
        as *mut libc::c_char;
    let mut hexadecimal: *mut libc::c_char = nmalloc(9 as libc::c_int as size_t)
        as *mut libc::c_char;
    let mut successor: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut namewidth: size_t = 0;
    let mut placewidth: size_t = 0;
    let mut tallywidth: size_t = 0 as libc::c_int as size_t;
    let mut padding: size_t = 2 as libc::c_int as size_t;
    let mut widecode: wchar_t = 0;
    wattr_on(
        footwin,
        interface_color_pair[MINI_INFOBAR as libc::c_int as usize] as attr_t,
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
    if *((*openfile).filename).offset(0 as libc::c_int as isize) as libc::c_int
        != '\0' as i32
    {
        as_an_at = 0 as libc::c_int != 0;
        thename = display_string(
            (*openfile).filename,
            0 as libc::c_int as size_t,
            COLS as size_t,
            0 as libc::c_int != 0,
            0 as libc::c_int != 0,
        );
    } else {
        thename = copy_of(
            dcgettext(
                0 as *const libc::c_char,
                b"(nameless)\0" as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ),
        );
    }
    sprintf(
        location,
        b"%zi,%zi\0" as *const u8 as *const libc::c_char,
        (*(*openfile).current).lineno,
        (xplustabs()).wrapping_add(1 as libc::c_int as libc::c_ulong),
    );
    placewidth = strlen(location);
    namewidth = breadth(thename);
    if namewidth.wrapping_add(19 as libc::c_int as libc::c_ulong) > COLS as libc::c_ulong
    {
        padding = 0 as libc::c_int as size_t;
    }
    if COLS > 4 as libc::c_int {
        if namewidth > (COLS - 2 as libc::c_int) as libc::c_ulong {
            let mut shortname: *mut libc::c_char = display_string(
                thename,
                namewidth
                    .wrapping_sub(COLS as libc::c_ulong)
                    .wrapping_add(5 as libc::c_int as libc::c_ulong),
                (COLS - 5 as libc::c_int) as size_t,
                0 as libc::c_int != 0,
                0 as libc::c_int != 0,
            );
            if wmove(footwin, 0 as libc::c_int, 0 as libc::c_int) == -(1 as libc::c_int)
            {
                -(1 as libc::c_int);
            } else {
                waddnstr(
                    footwin,
                    b"...\0" as *const u8 as *const libc::c_char,
                    -(1 as libc::c_int),
                );
            };
            waddnstr(footwin, shortname, -(1 as libc::c_int));
            rpl_free(shortname as *mut libc::c_void);
        } else {
            if wmove(footwin, 0 as libc::c_int, padding as libc::c_int)
                == -(1 as libc::c_int)
            {
                -(1 as libc::c_int);
            } else {
                waddnstr(footwin, thename, -(1 as libc::c_int));
            };
        }
        waddnstr(
            footwin,
            if (*openfile).modified as libc::c_int != 0 {
                b" *\0" as *const u8 as *const libc::c_char
            } else {
                b"  \0" as *const u8 as *const libc::c_char
            },
            -(1 as libc::c_int),
        );
    }
    if report_size as libc::c_int != 0 && COLS > 35 as libc::c_int {
        let mut count: size_t = ((*(*openfile).filebot).lineno
            - (*((*(*openfile).filebot).data).offset(0 as libc::c_int as isize)
                as libc::c_int == '\0' as i32) as libc::c_int as libc::c_long) as size_t;
        number_of_lines = nmalloc(44 as libc::c_int as size_t) as *mut libc::c_char;
        sprintf(
            number_of_lines,
            dcngettext(
                0 as *const libc::c_char,
                b" (%zu line)\0" as *const u8 as *const libc::c_char,
                b" (%zu lines)\0" as *const u8 as *const libc::c_char,
                count,
                5 as libc::c_int,
            ),
            count,
        );
        tallywidth = breadth(number_of_lines);
        if namewidth
            .wrapping_add(tallywidth)
            .wrapping_add(11 as libc::c_int as libc::c_ulong) < COLS as libc::c_ulong
        {
            waddnstr(footwin, number_of_lines, -(1 as libc::c_int));
        } else {
            tallywidth = 0 as libc::c_int as size_t;
        }
        report_size = 0 as libc::c_int != 0;
    } else if (*openfile).next != openfile && COLS > 35 as libc::c_int {
        ranking = nmalloc(24 as libc::c_int as size_t) as *mut libc::c_char;
        sprintf(
            ranking,
            b" [%i/%i]\0" as *const u8 as *const libc::c_char,
            buffer_number(openfile),
            buffer_number((*startfile).prev),
        );
        if namewidth
            .wrapping_add(placewidth)
            .wrapping_add(breadth(ranking))
            .wrapping_add(32 as libc::c_int as libc::c_ulong) < COLS as libc::c_ulong
        {
            waddnstr(footwin, ranking, -(1 as libc::c_int));
        }
    }
    if flags[(CONSTANT_SHOW as libc::c_int as libc::c_ulong)
        .wrapping_div(
            (::std::mem::size_of::<libc::c_uint>() as libc::c_ulong)
                .wrapping_mul(8 as libc::c_int as libc::c_ulong),
        ) as usize]
        & (1 as libc::c_int as libc::c_uint)
            << (CONSTANT_SHOW as libc::c_int as libc::c_ulong)
                .wrapping_rem(
                    (::std::mem::size_of::<libc::c_uint>() as libc::c_ulong)
                        .wrapping_mul(8 as libc::c_int as libc::c_ulong),
                ) != 0 as libc::c_int as libc::c_uint
        && namewidth
            .wrapping_add(tallywidth)
            .wrapping_add(placewidth)
            .wrapping_add(32 as libc::c_int as libc::c_ulong) < COLS as libc::c_ulong
    {
        if wmove(
            footwin,
            0 as libc::c_int,
            ((COLS - 27 as libc::c_int) as libc::c_ulong).wrapping_sub(placewidth)
                as libc::c_int,
        ) == -(1 as libc::c_int)
        {
            -(1 as libc::c_int);
        } else {
            waddnstr(footwin, location, -(1 as libc::c_int));
        };
    }
    if flags[(CONSTANT_SHOW as libc::c_int as libc::c_ulong)
        .wrapping_div(
            (::std::mem::size_of::<libc::c_uint>() as libc::c_ulong)
                .wrapping_mul(8 as libc::c_int as libc::c_ulong),
        ) as usize]
        & (1 as libc::c_int as libc::c_uint)
            << (CONSTANT_SHOW as libc::c_int as libc::c_ulong)
                .wrapping_rem(
                    (::std::mem::size_of::<libc::c_uint>() as libc::c_ulong)
                        .wrapping_mul(8 as libc::c_int as libc::c_ulong),
                ) != 0 as libc::c_int as libc::c_uint
        && namewidth
            .wrapping_add(tallywidth)
            .wrapping_add(28 as libc::c_int as libc::c_ulong) < COLS as libc::c_ulong
    {
        let mut this_position: *mut libc::c_char = ((*(*openfile).current).data)
            .offset((*openfile).current_x as isize);
        if *this_position as libc::c_int == '\0' as i32 {
            sprintf(
                hexadecimal,
                if !((*(*openfile).current).next).is_null() {
                    if using_utf8() as libc::c_int != 0 {
                        b"U+000A\0" as *const u8 as *const libc::c_char
                    } else {
                        b"  0x0A\0" as *const u8 as *const libc::c_char
                    }
                } else {
                    b"  ----\0" as *const u8 as *const libc::c_char
                },
            );
        } else if *this_position as libc::c_int == '\n' as i32 {
            sprintf(hexadecimal, b"  0x00\0" as *const u8 as *const libc::c_char);
        } else if (*this_position as libc::c_uchar as libc::c_int) < 0x80 as libc::c_int
            && using_utf8() as libc::c_int != 0
        {
            sprintf(
                hexadecimal,
                b"U+%04X\0" as *const u8 as *const libc::c_char,
                *this_position as libc::c_uchar as libc::c_int,
            );
        } else if using_utf8() as libc::c_int != 0
            && mbtowide(&mut widecode, this_position) > 0 as libc::c_int
        {
            sprintf(
                hexadecimal,
                b"U+%04X\0" as *const u8 as *const libc::c_char,
                widecode,
            );
        } else {
            sprintf(
                hexadecimal,
                b"  0x%02X\0" as *const u8 as *const libc::c_char,
                *this_position as libc::c_uchar as libc::c_int,
            );
        }
        if wmove(footwin, 0 as libc::c_int, COLS - 23 as libc::c_int)
            == -(1 as libc::c_int)
        {
            -(1 as libc::c_int);
        } else {
            waddnstr(footwin, hexadecimal, -(1 as libc::c_int));
        };
        successor = this_position.offset(char_length(this_position) as isize);
        if *this_position as libc::c_int != 0 && *successor as libc::c_int != 0
            && is_zerowidth(successor) as libc::c_int != 0
            && mbtowide(&mut widecode, successor) > 0 as libc::c_int
        {
            sprintf(
                hexadecimal,
                b"|%04X\0" as *const u8 as *const libc::c_char,
                widecode,
            );
            waddnstr(footwin, hexadecimal, -(1 as libc::c_int));
            successor = successor.offset(char_length(successor) as isize);
            if is_zerowidth(successor) as libc::c_int != 0
                && mbtowide(&mut widecode, successor) > 0 as libc::c_int
            {
                sprintf(
                    hexadecimal,
                    b"|%04X\0" as *const u8 as *const libc::c_char,
                    widecode,
                );
                waddnstr(footwin, hexadecimal, -(1 as libc::c_int));
            }
        } else {
            successor = 0 as *mut libc::c_char;
        }
    }
    if flags[(STATEFLAGS as libc::c_int as libc::c_ulong)
        .wrapping_div(
            (::std::mem::size_of::<libc::c_uint>() as libc::c_ulong)
                .wrapping_mul(8 as libc::c_int as libc::c_ulong),
        ) as usize]
        & (1 as libc::c_int as libc::c_uint)
            << (STATEFLAGS as libc::c_int as libc::c_ulong)
                .wrapping_rem(
                    (::std::mem::size_of::<libc::c_uint>() as libc::c_ulong)
                        .wrapping_mul(8 as libc::c_int as libc::c_ulong),
                ) != 0 as libc::c_int as libc::c_uint && successor.is_null()
        && namewidth
            .wrapping_add(tallywidth)
            .wrapping_add(14 as libc::c_int as libc::c_ulong)
            .wrapping_add((2 as libc::c_int as libc::c_ulong).wrapping_mul(padding))
            < COLS as libc::c_ulong
    {
        wmove(
            footwin,
            0 as libc::c_int,
            ((COLS - 11 as libc::c_int) as libc::c_ulong).wrapping_sub(padding)
                as libc::c_int,
        );
        show_states_at(footwin);
    }
    if namewidth.wrapping_add(6 as libc::c_int as libc::c_ulong) < COLS as libc::c_ulong
    {
        sprintf(
            location,
            b"%3zi%%\0" as *const u8 as *const libc::c_char,
            100 as libc::c_int as libc::c_long * (*(*openfile).current).lineno
                / (*(*openfile).filebot).lineno,
        );
        if wmove(
            footwin,
            0 as libc::c_int,
            ((COLS - 4 as libc::c_int) as libc::c_ulong).wrapping_sub(padding)
                as libc::c_int,
        ) == -(1 as libc::c_int)
        {
            -(1 as libc::c_int);
        } else {
            waddnstr(footwin, location, -(1 as libc::c_int));
        };
    }
    wattr_off(
        footwin,
        interface_color_pair[MINI_INFOBAR as libc::c_int as usize] as attr_t,
        0 as *mut libc::c_void,
    );
    wrefresh(footwin);
    rpl_free(number_of_lines as *mut libc::c_void);
    rpl_free(hexadecimal as *mut libc::c_void);
    rpl_free(location as *mut libc::c_void);
    rpl_free(thename as *mut libc::c_void);
    rpl_free(ranking as *mut libc::c_void);
}
pub unsafe extern "C" fn statusline(
    mut importance: message_type,
    mut msg: *const libc::c_char,
    mut args: ...
) {
    let mut showed_whitespace: bool = flags[(WHITESPACE_DISPLAY as libc::c_int
        as libc::c_ulong)
        .wrapping_div(
            (::std::mem::size_of::<libc::c_uint>() as libc::c_ulong)
                .wrapping_mul(8 as libc::c_int as libc::c_ulong),
        ) as usize]
        & (1 as libc::c_int as libc::c_uint)
            << (WHITESPACE_DISPLAY as libc::c_int as libc::c_ulong)
                .wrapping_rem(
                    (::std::mem::size_of::<libc::c_uint>() as libc::c_ulong)
                        .wrapping_mul(8 as libc::c_int as libc::c_ulong),
                ) != 0 as libc::c_int as libc::c_uint;
    static mut start_col: size_t = 0 as libc::c_int as size_t;
    let mut compound: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut message: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut bracketed: bool = false;
    let mut colorpair: libc::c_int = 0;
    let mut ap: ::std::ffi::VaListImpl;
    if (importance as libc::c_uint) < lastmessage as libc::c_uint
        && lastmessage as libc::c_uint > NOTICE as libc::c_int as libc::c_uint
    {
        return;
    }
    compound = nmalloc((4 as libc::c_int * COLS + 1 as libc::c_int) as size_t)
        as *mut libc::c_char;
    ap = args.clone();
    vsnprintf(
        compound,
        (4 as libc::c_int * COLS + 1 as libc::c_int) as libc::c_ulong,
        msg,
        ap.as_va_list(),
    );
    if isendwin() {
        fprintf(stderr, b"\n%s\n\0" as *const u8 as *const libc::c_char, compound);
        rpl_free(compound as *mut libc::c_void);
        return;
    }
    if !we_are_running
        && importance as libc::c_uint == ALERT as libc::c_int as libc::c_uint
        && !openfile.is_null() && (*openfile).fmt as u64 == 0
        && ((*openfile).errormessage).is_null() && (*openfile).next != openfile
    {
        (*openfile).errormessage = copy_of(compound);
    }
    if LINES == 1 as libc::c_int
        && (importance as libc::c_uint) < INFO as libc::c_int as libc::c_uint
    {
        wnoutrefresh(midwin);
    }
    if lastmessage as libc::c_uint == ALERT as libc::c_int as libc::c_uint {
        if start_col > 4 as libc::c_int as libc::c_ulong {
            wmove(
                footwin,
                0 as libc::c_int,
                ((COLS + 2 as libc::c_int) as libc::c_ulong).wrapping_sub(start_col)
                    as libc::c_int,
            );
            wattr_on(
                footwin,
                interface_color_pair[ERROR_MESSAGE as libc::c_int as usize] as attr_t,
                0 as *mut libc::c_void,
            );
            waddnstr(
                footwin,
                b"...\0" as *const u8 as *const libc::c_char,
                -(1 as libc::c_int),
            );
            wattr_off(
                footwin,
                interface_color_pair[ERROR_MESSAGE as libc::c_int as usize] as attr_t,
                0 as *mut libc::c_void,
            );
            wnoutrefresh(footwin);
            start_col = 0 as libc::c_int as size_t;
            napms(100 as libc::c_int);
            beep();
        }
        rpl_free(compound as *mut libc::c_void);
        return;
    }
    if importance as libc::c_uint > NOTICE as libc::c_int as libc::c_uint {
        if importance as libc::c_uint == ALERT as libc::c_int as libc::c_uint {
            beep();
        }
        colorpair = interface_color_pair[ERROR_MESSAGE as libc::c_int as usize];
    } else if importance as libc::c_uint == NOTICE as libc::c_int as libc::c_uint {
        colorpair = interface_color_pair[SELECTED_TEXT as libc::c_int as usize];
    } else {
        colorpair = interface_color_pair[STATUS_BAR as libc::c_int as usize];
    }
    lastmessage = importance;
    blank_statusbar();
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
    message = display_string(
        compound,
        0 as libc::c_int as size_t,
        COLS as size_t,
        0 as libc::c_int != 0,
        0 as libc::c_int != 0,
    );
    if showed_whitespace {
        flags[(WHITESPACE_DISPLAY as libc::c_int as libc::c_ulong)
            .wrapping_div(
                (::std::mem::size_of::<libc::c_uint>() as libc::c_ulong)
                    .wrapping_mul(8 as libc::c_int as libc::c_ulong),
            ) as usize]
            |= (1 as libc::c_int as libc::c_uint)
                << (WHITESPACE_DISPLAY as libc::c_int as libc::c_ulong)
                    .wrapping_rem(
                        (::std::mem::size_of::<libc::c_uint>() as libc::c_ulong)
                            .wrapping_mul(8 as libc::c_int as libc::c_ulong),
                    );
    }
    start_col = (COLS as libc::c_ulong)
        .wrapping_sub(breadth(message))
        .wrapping_div(2 as libc::c_int as libc::c_ulong);
    bracketed = start_col > 1 as libc::c_int as libc::c_ulong;
    wmove(
        footwin,
        0 as libc::c_int,
        (if bracketed as libc::c_int != 0 {
            start_col.wrapping_sub(2 as libc::c_int as libc::c_ulong)
        } else {
            start_col
        }) as libc::c_int,
    );
    wattr_on(footwin, colorpair as attr_t, 0 as *mut libc::c_void);
    if bracketed {
        waddnstr(
            footwin,
            b"[ \0" as *const u8 as *const libc::c_char,
            -(1 as libc::c_int),
        );
    }
    waddnstr(footwin, message, -(1 as libc::c_int));
    if bracketed {
        waddnstr(
            footwin,
            b" ]\0" as *const u8 as *const libc::c_char,
            -(1 as libc::c_int),
        );
    }
    wattr_off(footwin, colorpair as attr_t, 0 as *mut libc::c_void);
    wrefresh(footwin);
    rpl_free(compound as *mut libc::c_void);
    rpl_free(message as *mut libc::c_void);
    statusblank = if flags[(QUICK_BLANK as libc::c_int as libc::c_ulong)
        .wrapping_div(
            (::std::mem::size_of::<libc::c_uint>() as libc::c_ulong)
                .wrapping_mul(8 as libc::c_int as libc::c_ulong),
        ) as usize]
        & (1 as libc::c_int as libc::c_uint)
            << (QUICK_BLANK as libc::c_int as libc::c_ulong)
                .wrapping_rem(
                    (::std::mem::size_of::<libc::c_uint>() as libc::c_ulong)
                        .wrapping_mul(8 as libc::c_int as libc::c_ulong),
                ) != 0 as libc::c_int as libc::c_uint
    {
        1 as libc::c_int
    } else {
        20 as libc::c_int
    };
}
pub unsafe extern "C" fn statusbar(mut msg: *const libc::c_char) {
    statusline(HUSH, msg);
}
pub unsafe extern "C" fn warn_and_briefly_pause(mut msg: *const libc::c_char) {
    blank_bottombars();
    statusline(ALERT, msg);
    lastmessage = VACUUM;
    napms(1500 as libc::c_int);
}
pub unsafe extern "C" fn post_one_key(
    mut keystroke: *const libc::c_char,
    mut tag: *const libc::c_char,
    mut width: libc::c_int,
) {
    wattr_on(
        footwin,
        interface_color_pair[KEY_COMBO as libc::c_int as usize] as attr_t,
        0 as *mut libc::c_void,
    );
    waddnstr(footwin, keystroke, actual_x(keystroke, width as size_t) as libc::c_int);
    wattr_off(
        footwin,
        interface_color_pair[KEY_COMBO as libc::c_int as usize] as attr_t,
        0 as *mut libc::c_void,
    );
    width = (width as libc::c_ulong).wrapping_sub(breadth(keystroke)) as libc::c_int
        as libc::c_int;
    if width < 2 as libc::c_int {
        return;
    }
    waddch(footwin, ' ' as i32 as chtype);
    wattr_on(
        footwin,
        interface_color_pair[FUNCTION_TAG as libc::c_int as usize] as attr_t,
        0 as *mut libc::c_void,
    );
    waddnstr(
        footwin,
        tag,
        actual_x(tag, (width - 1 as libc::c_int) as size_t) as libc::c_int,
    );
    wattr_off(
        footwin,
        interface_color_pair[FUNCTION_TAG as libc::c_int as usize] as attr_t,
        0 as *mut libc::c_void,
    );
}
pub unsafe extern "C" fn bottombars(mut menu: libc::c_int) {
    let mut index: size_t = 0;
    let mut number: size_t = 0;
    let mut itemwidth: size_t = 0;
    let mut s: *const keystruct = 0 as *const keystruct;
    let mut f: *mut funcstruct = 0 as *mut funcstruct;
    currmenu = menu;
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
                ) != 0 as libc::c_int as libc::c_uint || LINES < 6 as libc::c_int
    {
        return;
    }
    number = shown_entries_for(menu);
    itemwidth = (COLS as libc::c_ulong)
        .wrapping_div(
            number
                .wrapping_add(1 as libc::c_int as libc::c_ulong)
                .wrapping_div(2 as libc::c_int as libc::c_ulong),
        );
    if itemwidth == 0 as libc::c_int as libc::c_ulong {
        return;
    }
    blank_bottombars();
    f = allfuncs;
    index = 0 as libc::c_int as size_t;
    while !f.is_null() && index < number {
        let mut thiswidth: size_t = itemwidth;
        if !((*f).menus & menu == 0 as libc::c_int) {
            s = first_sc_for(menu, (*f).func);
            if !s.is_null() {
                wmove(
                    footwin,
                    (1 as libc::c_int as libc::c_ulong)
                        .wrapping_add(
                            index.wrapping_rem(2 as libc::c_int as libc::c_ulong),
                        ) as libc::c_int,
                    index
                        .wrapping_div(2 as libc::c_int as libc::c_ulong)
                        .wrapping_mul(itemwidth) as libc::c_int,
                );
                if number.wrapping_rem(2 as libc::c_int as libc::c_ulong)
                    == 1 as libc::c_int as libc::c_ulong
                    && index.wrapping_add(2 as libc::c_int as libc::c_ulong) == number
                {
                    thiswidth = (thiswidth as libc::c_ulong).wrapping_add(itemwidth)
                        as size_t as size_t;
                }
                if index.wrapping_add(2 as libc::c_int as libc::c_ulong) >= number {
                    thiswidth = (thiswidth as libc::c_ulong)
                        .wrapping_add((COLS as libc::c_ulong).wrapping_rem(itemwidth))
                        as size_t as size_t;
                }
                post_one_key(
                    (*s).keystr,
                    dcgettext(0 as *const libc::c_char, (*f).tag, 5 as libc::c_int),
                    thiswidth as libc::c_int,
                );
                index = index.wrapping_add(1);
                index;
            }
        }
        f = (*f).next;
    }
    wrefresh(footwin);
}
pub unsafe extern "C" fn place_the_cursor() {
    let mut row: ssize_t = 0 as libc::c_int as ssize_t;
    let mut column: size_t = xplustabs();
    if flags[(SOFTWRAP as libc::c_int as libc::c_ulong)
        .wrapping_div(
            (::std::mem::size_of::<libc::c_uint>() as libc::c_ulong)
                .wrapping_mul(8 as libc::c_int as libc::c_ulong),
        ) as usize]
        & (1 as libc::c_int as libc::c_uint)
            << (SOFTWRAP as libc::c_int as libc::c_ulong)
                .wrapping_rem(
                    (::std::mem::size_of::<libc::c_uint>() as libc::c_ulong)
                        .wrapping_mul(8 as libc::c_int as libc::c_ulong),
                ) != 0 as libc::c_int as libc::c_uint
    {
        let mut line: *mut linestruct = (*openfile).edittop;
        let mut leftedge: size_t = 0;
        row = (row as libc::c_ulong)
            .wrapping_sub(chunk_for((*openfile).firstcolumn, (*openfile).edittop))
            as ssize_t as ssize_t;
        while !line.is_null() && line != (*openfile).current {
            row = (row as libc::c_ulong)
                .wrapping_add(
                    (1 as libc::c_int as libc::c_ulong)
                        .wrapping_add(extra_chunks_in(line)),
                ) as ssize_t as ssize_t;
            line = (*line).next;
        }
        row = (row as libc::c_ulong)
            .wrapping_add(get_chunk_and_edge(column, (*openfile).current, &mut leftedge))
            as ssize_t as ssize_t;
        column = (column as libc::c_ulong).wrapping_sub(leftedge) as size_t as size_t;
    } else {
        row = (*(*openfile).current).lineno - (*(*openfile).edittop).lineno;
        column = (column as libc::c_ulong).wrapping_sub(get_page_start(column)) as size_t
            as size_t;
    }
    if row < editwinrows as libc::c_long {
        wmove(
            midwin,
            row as libc::c_int,
            (margin as libc::c_ulong).wrapping_add(column) as libc::c_int,
        );
    } else {
        statusline(
            ALERT,
            b"Misplaced cursor -- please report a bug\0" as *const u8
                as *const libc::c_char,
        );
    }
    (*openfile).current_y = row;
}
pub unsafe extern "C" fn draw_row(
    mut row: libc::c_int,
    mut converted: *const libc::c_char,
    mut line: *mut linestruct,
    mut from_col: size_t,
) {
    if margin > 0 as libc::c_int {
        wattr_on(
            midwin,
            interface_color_pair[LINE_NUMBER as libc::c_int as usize] as attr_t,
            0 as *mut libc::c_void,
        );
        if flags[(SOFTWRAP as libc::c_int as libc::c_ulong)
            .wrapping_div(
                (::std::mem::size_of::<libc::c_uint>() as libc::c_ulong)
                    .wrapping_mul(8 as libc::c_int as libc::c_ulong),
            ) as usize]
            & (1 as libc::c_int as libc::c_uint)
                << (SOFTWRAP as libc::c_int as libc::c_ulong)
                    .wrapping_rem(
                        (::std::mem::size_of::<libc::c_uint>() as libc::c_ulong)
                            .wrapping_mul(8 as libc::c_int as libc::c_ulong),
                    ) != 0 as libc::c_int as libc::c_uint
            && from_col != 0 as libc::c_int as libc::c_ulong
        {
            mvwprintw(
                midwin,
                row,
                0 as libc::c_int,
                b"%*s\0" as *const u8 as *const libc::c_char,
                margin - 1 as libc::c_int,
                b" \0" as *const u8 as *const libc::c_char,
            );
        } else {
            mvwprintw(
                midwin,
                row,
                0 as libc::c_int,
                b"%*zd\0" as *const u8 as *const libc::c_char,
                margin - 1 as libc::c_int,
                (*line).lineno,
            );
        }
        wattr_off(
            midwin,
            interface_color_pair[LINE_NUMBER as libc::c_int as usize] as attr_t,
            0 as *mut libc::c_void,
        );
        if (*line).has_anchor as libc::c_int != 0
            && (from_col == 0 as libc::c_int as libc::c_ulong
                || !(flags[(SOFTWRAP as libc::c_int as libc::c_ulong)
                    .wrapping_div(
                        (::std::mem::size_of::<libc::c_uint>() as libc::c_ulong)
                            .wrapping_mul(8 as libc::c_int as libc::c_ulong),
                    ) as usize]
                    & (1 as libc::c_int as libc::c_uint)
                        << (SOFTWRAP as libc::c_int as libc::c_ulong)
                            .wrapping_rem(
                                (::std::mem::size_of::<libc::c_uint>() as libc::c_ulong)
                                    .wrapping_mul(8 as libc::c_int as libc::c_ulong),
                            ) != 0 as libc::c_int as libc::c_uint))
        {
            if using_utf8() {
                wprintw(midwin, b"\xE2\xAC\xA5\0" as *const u8 as *const libc::c_char);
            } else {
                wprintw(midwin, b"+\0" as *const u8 as *const libc::c_char);
            }
        } else {
            wprintw(midwin, b" \0" as *const u8 as *const libc::c_char);
        }
    }
    if wmove(midwin, row, margin) == -(1 as libc::c_int) {
        -(1 as libc::c_int);
    } else {
        waddnstr(midwin, converted, -(1 as libc::c_int));
    };
    if is_shorter as libc::c_int != 0
        || flags[(SOFTWRAP as libc::c_int as libc::c_ulong)
            .wrapping_div(
                (::std::mem::size_of::<libc::c_uint>() as libc::c_ulong)
                    .wrapping_mul(8 as libc::c_int as libc::c_ulong),
            ) as usize]
            & (1 as libc::c_int as libc::c_uint)
                << (SOFTWRAP as libc::c_int as libc::c_ulong)
                    .wrapping_rem(
                        (::std::mem::size_of::<libc::c_uint>() as libc::c_ulong)
                            .wrapping_mul(8 as libc::c_int as libc::c_ulong),
                    ) != 0 as libc::c_int as libc::c_uint
    {
        wclrtoeol(midwin);
    }
    if thebar != 0 {
        if wmove(midwin, row, COLS - 1 as libc::c_int) == -(1 as libc::c_int) {
            -(1 as libc::c_int);
        } else {
            waddch(midwin, *bardata.offset(row as isize) as chtype);
        };
    }
    if !((*openfile).syntax).is_null()
        && !(flags[(NO_SYNTAX as libc::c_int as libc::c_ulong)
            .wrapping_div(
                (::std::mem::size_of::<libc::c_uint>() as libc::c_ulong)
                    .wrapping_mul(8 as libc::c_int as libc::c_ulong),
            ) as usize]
            & (1 as libc::c_int as libc::c_uint)
                << (NO_SYNTAX as libc::c_int as libc::c_ulong)
                    .wrapping_rem(
                        (::std::mem::size_of::<libc::c_uint>() as libc::c_ulong)
                            .wrapping_mul(8 as libc::c_int as libc::c_ulong),
                    ) != 0 as libc::c_int as libc::c_uint)
    {
        let mut varnish: *const colortype = (*(*openfile).syntax).color;
        if (*(*openfile).syntax).nmultis as libc::c_int > 0 as libc::c_int
            && ((*line).multidata).is_null()
        {
            (*line)
                .multidata = nmalloc(
                ((*(*openfile).syntax).nmultis as libc::c_ulong)
                    .wrapping_mul(
                        ::std::mem::size_of::<libc::c_short>() as libc::c_ulong,
                    ),
            ) as *mut libc::c_short;
        }
        let mut current_block_65: u64;
        while !varnish.is_null() {
            let mut index: size_t = 0 as libc::c_int as size_t;
            let mut start_col: libc::c_int = 0 as libc::c_int;
            let mut paintlen: libc::c_int = 0 as libc::c_int;
            let mut thetext: *const libc::c_char = 0 as *const libc::c_char;
            let mut match_0: regmatch_t = regmatch_t { rm_so: 0, rm_eo: 0 };
            let mut start_line: *const linestruct = (*line).prev;
            let mut startmatch: regmatch_t = regmatch_t { rm_so: 0, rm_eo: 0 };
            let mut endmatch: regmatch_t = regmatch_t { rm_so: 0, rm_eo: 0 };
            if ((*varnish).end).is_null() {
                while index < 2000 as libc::c_int as libc::c_ulong && index < till_x {
                    if rpl_regexec(
                        (*varnish).start,
                        &mut *((*line).data).offset(index as isize),
                        1 as libc::c_int as size_t,
                        &mut match_0,
                        if index == 0 as libc::c_int as libc::c_ulong {
                            0 as libc::c_int
                        } else {
                            1 as libc::c_int
                        },
                    ) != 0 as libc::c_int
                    {
                        break;
                    }
                    match_0
                        .rm_so = (match_0.rm_so as libc::c_ulong).wrapping_add(index)
                        as regoff_t as regoff_t;
                    match_0
                        .rm_eo = (match_0.rm_eo as libc::c_ulong).wrapping_add(index)
                        as regoff_t as regoff_t;
                    index = match_0.rm_eo as size_t;
                    if match_0.rm_so as libc::c_ulong >= till_x {
                        break;
                    }
                    if match_0.rm_so == match_0.rm_eo {
                        if *((*line).data).offset(index as isize) as libc::c_int
                            == '\0' as i32
                        {
                            break;
                        }
                        index = step_right((*line).data, index);
                    } else {
                        if match_0.rm_eo as libc::c_ulong <= from_x {
                            continue;
                        }
                        if match_0.rm_so as libc::c_ulong > from_x {
                            start_col = (wideness((*line).data, match_0.rm_so as size_t))
                                .wrapping_sub(from_col) as libc::c_int;
                        }
                        thetext = converted
                            .offset(actual_x(converted, start_col as size_t) as isize);
                        paintlen = actual_x(
                            thetext,
                            (wideness((*line).data, match_0.rm_eo as size_t))
                                .wrapping_sub(from_col)
                                .wrapping_sub(start_col as libc::c_ulong),
                        ) as libc::c_int;
                        wattr_on(
                            midwin,
                            (*varnish).attributes as attr_t,
                            0 as *mut libc::c_void,
                        );
                        if wmove(midwin, row, margin + start_col) == -(1 as libc::c_int)
                        {
                            -(1 as libc::c_int);
                        } else {
                            waddnstr(midwin, thetext, paintlen);
                        };
                        wattr_off(
                            midwin,
                            (*varnish).attributes as attr_t,
                            0 as *mut libc::c_void,
                        );
                    }
                }
            } else {
                *((*line).multidata)
                    .offset(
                        (*varnish).id as isize,
                    ) = ((1 as libc::c_int) << 1 as libc::c_int) as libc::c_short;
                if !start_line.is_null() && ((*start_line).multidata).is_null() {
                    statusline(
                        ALERT,
                        b"Missing multidata -- please report a bug\0" as *const u8
                            as *const libc::c_char,
                    );
                    current_block_65 = 10150597327160359210;
                } else if !start_line.is_null()
                    && (*((*start_line).multidata).offset((*varnish).id as isize)
                        as libc::c_int == (1 as libc::c_int) << 3 as libc::c_int
                        || *((*start_line).multidata).offset((*varnish).id as isize)
                            as libc::c_int == (1 as libc::c_int) << 2 as libc::c_int)
                {
                    if rpl_regexec(
                        (*varnish).end,
                        (*line).data,
                        1 as libc::c_int as size_t,
                        &mut endmatch,
                        0 as libc::c_int,
                    ) == _REG_NOMATCH as libc::c_int
                    {
                        wattr_on(
                            midwin,
                            (*varnish).attributes as attr_t,
                            0 as *mut libc::c_void,
                        );
                        if wmove(midwin, row, margin) == -(1 as libc::c_int) {
                            -(1 as libc::c_int);
                        } else {
                            waddnstr(midwin, converted, -(1 as libc::c_int));
                        };
                        wattr_off(
                            midwin,
                            (*varnish).attributes as attr_t,
                            0 as *mut libc::c_void,
                        );
                        *((*line).multidata)
                            .offset(
                                (*varnish).id as isize,
                            ) = ((1 as libc::c_int) << 3 as libc::c_int)
                            as libc::c_short;
                        current_block_65 = 7149356873433890176;
                    } else {
                        if endmatch.rm_eo as libc::c_ulong > from_x {
                            paintlen = actual_x(
                                converted,
                                (wideness((*line).data, endmatch.rm_eo as size_t))
                                    .wrapping_sub(from_col),
                            ) as libc::c_int;
                            wattr_on(
                                midwin,
                                (*varnish).attributes as attr_t,
                                0 as *mut libc::c_void,
                            );
                            if wmove(midwin, row, margin) == -(1 as libc::c_int) {
                                -(1 as libc::c_int);
                            } else {
                                waddnstr(midwin, converted, paintlen);
                            };
                            wattr_off(
                                midwin,
                                (*varnish).attributes as attr_t,
                                0 as *mut libc::c_void,
                            );
                        }
                        *((*line).multidata)
                            .offset(
                                (*varnish).id as isize,
                            ) = ((1 as libc::c_int) << 4 as libc::c_int)
                            as libc::c_short;
                        current_block_65 = 10150597327160359210;
                    }
                } else {
                    current_block_65 = 10150597327160359210;
                }
                match current_block_65 {
                    7149356873433890176 => {}
                    _ => {
                        index = (if paintlen == 0 as libc::c_int {
                            0 as libc::c_int as libc::c_long
                        } else {
                            endmatch.rm_eo
                        }) as size_t;
                        while index < 2000 as libc::c_int as libc::c_ulong
                            && rpl_regexec(
                                (*varnish).start,
                                ((*line).data).offset(index as isize),
                                1 as libc::c_int as size_t,
                                &mut startmatch,
                                if index == 0 as libc::c_int as libc::c_ulong {
                                    0 as libc::c_int
                                } else {
                                    1 as libc::c_int
                                },
                            ) == 0 as libc::c_int
                        {
                            startmatch
                                .rm_so = (startmatch.rm_so as libc::c_ulong)
                                .wrapping_add(index) as regoff_t as regoff_t;
                            startmatch
                                .rm_eo = (startmatch.rm_eo as libc::c_ulong)
                                .wrapping_add(index) as regoff_t as regoff_t;
                            if startmatch.rm_so as libc::c_ulong > from_x {
                                start_col = (wideness(
                                    (*line).data,
                                    startmatch.rm_so as size_t,
                                ))
                                    .wrapping_sub(from_col) as libc::c_int;
                            }
                            thetext = converted
                                .offset(actual_x(converted, start_col as size_t) as isize);
                            if rpl_regexec(
                                (*varnish).end,
                                ((*line).data).offset(startmatch.rm_eo as isize),
                                1 as libc::c_int as size_t,
                                &mut endmatch,
                                if startmatch.rm_eo == 0 as libc::c_int as libc::c_long {
                                    0 as libc::c_int
                                } else {
                                    1 as libc::c_int
                                },
                            ) == 0 as libc::c_int
                            {
                                endmatch.rm_so += startmatch.rm_eo;
                                endmatch.rm_eo += startmatch.rm_eo;
                                if endmatch.rm_eo as libc::c_ulong > from_x
                                    && endmatch.rm_eo > startmatch.rm_so
                                {
                                    paintlen = actual_x(
                                        thetext,
                                        (wideness((*line).data, endmatch.rm_eo as size_t))
                                            .wrapping_sub(from_col)
                                            .wrapping_sub(start_col as libc::c_ulong),
                                    ) as libc::c_int;
                                    wattr_on(
                                        midwin,
                                        (*varnish).attributes as attr_t,
                                        0 as *mut libc::c_void,
                                    );
                                    if wmove(midwin, row, margin + start_col)
                                        == -(1 as libc::c_int)
                                    {
                                        -(1 as libc::c_int);
                                    } else {
                                        waddnstr(midwin, thetext, paintlen);
                                    };
                                    wattr_off(
                                        midwin,
                                        (*varnish).attributes as attr_t,
                                        0 as *mut libc::c_void,
                                    );
                                    *((*line).multidata)
                                        .offset(
                                            (*varnish).id as isize,
                                        ) = ((1 as libc::c_int) << 5 as libc::c_int)
                                        as libc::c_short;
                                }
                                index = endmatch.rm_eo as size_t;
                                if !(startmatch.rm_so == startmatch.rm_eo
                                    && endmatch.rm_so == endmatch.rm_eo)
                                {
                                    continue;
                                }
                                if *((*line).data).offset(index as isize) as libc::c_int
                                    == '\0' as i32
                                {
                                    break;
                                }
                                index = step_right((*line).data, index);
                            } else {
                                wattr_on(
                                    midwin,
                                    (*varnish).attributes as attr_t,
                                    0 as *mut libc::c_void,
                                );
                                if wmove(midwin, row, margin + start_col)
                                    == -(1 as libc::c_int)
                                {
                                    -(1 as libc::c_int);
                                } else {
                                    waddnstr(midwin, thetext, -(1 as libc::c_int));
                                };
                                wattr_off(
                                    midwin,
                                    (*varnish).attributes as attr_t,
                                    0 as *mut libc::c_void,
                                );
                                *((*line).multidata)
                                    .offset(
                                        (*varnish).id as isize,
                                    ) = ((1 as libc::c_int) << 2 as libc::c_int)
                                    as libc::c_short;
                                break;
                            }
                        }
                    }
                }
            }
            varnish = (*varnish).next;
        }
    }
    if stripe_column as libc::c_ulong > from_col && !inhelp
        && (sequel_column == 0 as libc::c_int as libc::c_ulong
            || stripe_column as libc::c_ulong <= sequel_column)
        && stripe_column as libc::c_ulong
            <= from_col.wrapping_add(editwincols as libc::c_ulong)
    {
        let mut target_column: ssize_t = (stripe_column as libc::c_ulong)
            .wrapping_sub(from_col)
            .wrapping_sub(1 as libc::c_int as libc::c_ulong) as ssize_t;
        let mut target_x: size_t = actual_x(converted, target_column as size_t);
        let mut striped_char: [libc::c_char; 4] = [0; 4];
        let mut charlen: size_t = 1 as libc::c_int as size_t;
        if *converted.offset(target_x as isize) as libc::c_int != '\0' as i32 {
            charlen = collect_char(
                converted.offset(target_x as isize),
                striped_char.as_mut_ptr(),
            ) as size_t;
            target_column = wideness(converted, target_x) as ssize_t;
        } else {
            striped_char[0 as libc::c_int as usize] = ' ' as i32 as libc::c_char;
        }
        wattr_on(
            midwin,
            interface_color_pair[GUIDE_STRIPE as libc::c_int as usize] as attr_t,
            0 as *mut libc::c_void,
        );
        if wmove(midwin, row, (margin as libc::c_long + target_column) as libc::c_int)
            == -(1 as libc::c_int)
        {
            -(1 as libc::c_int);
        } else {
            waddnstr(midwin, striped_char.as_mut_ptr(), charlen as libc::c_int);
        };
        wattr_off(
            midwin,
            interface_color_pair[GUIDE_STRIPE as libc::c_int as usize] as attr_t,
            0 as *mut libc::c_void,
        );
    }
    if !((*openfile).mark).is_null()
        && ((*line).lineno >= (*(*openfile).mark).lineno
            && (*line).lineno <= (*(*openfile).current).lineno
            || (*line).lineno <= (*(*openfile).mark).lineno
                && (*line).lineno >= (*(*openfile).current).lineno)
    {
        let mut top: *mut linestruct = 0 as *mut linestruct;
        let mut bot: *mut linestruct = 0 as *mut linestruct;
        let mut top_x: size_t = 0;
        let mut bot_x: size_t = 0;
        let mut start_col_0: libc::c_int = 0;
        let mut thetext_0: *const libc::c_char = 0 as *const libc::c_char;
        let mut paintlen_0: libc::c_int = -(1 as libc::c_int);
        get_region(&mut top, &mut top_x, &mut bot, &mut bot_x);
        if (*top).lineno < (*line).lineno || top_x < from_x {
            top_x = from_x;
        }
        if (*bot).lineno > (*line).lineno || bot_x > till_x {
            bot_x = till_x;
        }
        if top_x < till_x && bot_x > from_x {
            start_col_0 = (wideness((*line).data, top_x)).wrapping_sub(from_col)
                as libc::c_int;
            if start_col_0 < 0 as libc::c_int {
                start_col_0 = 0 as libc::c_int;
            }
            thetext_0 = converted
                .offset(actual_x(converted, start_col_0 as size_t) as isize);
            if bot_x < till_x {
                let mut end_col: size_t = (wideness((*line).data, bot_x))
                    .wrapping_sub(from_col);
                paintlen_0 = actual_x(
                    thetext_0,
                    end_col.wrapping_sub(start_col_0 as libc::c_ulong),
                ) as libc::c_int;
            }
            wattr_on(
                midwin,
                interface_color_pair[SELECTED_TEXT as libc::c_int as usize] as attr_t,
                0 as *mut libc::c_void,
            );
            if wmove(midwin, row, margin + start_col_0) == -(1 as libc::c_int) {
                -(1 as libc::c_int);
            } else {
                waddnstr(midwin, thetext_0, paintlen_0);
            };
            wattr_off(
                midwin,
                interface_color_pair[SELECTED_TEXT as libc::c_int as usize] as attr_t,
                0 as *mut libc::c_void,
            );
        }
    }
}
pub unsafe extern "C" fn update_line(
    mut line: *mut linestruct,
    mut index: size_t,
) -> libc::c_int {
    let mut row: libc::c_int = 0;
    let mut converted: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut from_col: size_t = 0;
    if flags[(SOFTWRAP as libc::c_int as libc::c_ulong)
        .wrapping_div(
            (::std::mem::size_of::<libc::c_uint>() as libc::c_ulong)
                .wrapping_mul(8 as libc::c_int as libc::c_ulong),
        ) as usize]
        & (1 as libc::c_int as libc::c_uint)
            << (SOFTWRAP as libc::c_int as libc::c_ulong)
                .wrapping_rem(
                    (::std::mem::size_of::<libc::c_uint>() as libc::c_ulong)
                        .wrapping_mul(8 as libc::c_int as libc::c_ulong),
                ) != 0 as libc::c_int as libc::c_uint
    {
        return update_softwrapped_line(line);
    }
    sequel_column = 0 as libc::c_int as size_t;
    row = ((*line).lineno - (*(*openfile).edittop).lineno) as libc::c_int;
    from_col = get_page_start(wideness((*line).data, index));
    converted = display_string(
        (*line).data,
        from_col,
        editwincols as size_t,
        1 as libc::c_int != 0,
        0 as libc::c_int != 0,
    );
    draw_row(row, converted, line, from_col);
    rpl_free(converted as *mut libc::c_void);
    if from_col > 0 as libc::c_int as libc::c_ulong {
        wattr_on(midwin, hilite_attribute as attr_t, 0 as *mut libc::c_void);
        if wmove(midwin, row, margin) == -(1 as libc::c_int) {
            -(1 as libc::c_int);
        } else {
            waddch(midwin, '<' as i32 as chtype);
        };
        wattr_off(midwin, hilite_attribute as attr_t, 0 as *mut libc::c_void);
    }
    if has_more {
        wattr_on(midwin, hilite_attribute as attr_t, 0 as *mut libc::c_void);
        if wmove(midwin, row, COLS - 1 as libc::c_int - thebar) == -(1 as libc::c_int) {
            -(1 as libc::c_int);
        } else {
            waddch(midwin, '>' as i32 as chtype);
        };
        wattr_off(midwin, hilite_attribute as attr_t, 0 as *mut libc::c_void);
    }
    if spotlighted as libc::c_int != 0 && line == (*openfile).current {
        spotlight(light_from_col, light_to_col);
    }
    return 1 as libc::c_int;
}
pub unsafe extern "C" fn update_softwrapped_line(
    mut line: *mut linestruct,
) -> libc::c_int {
    let mut row: libc::c_int = 0 as libc::c_int;
    let mut someline: *mut linestruct = (*openfile).edittop;
    let mut starting_row: libc::c_int = 0;
    let mut from_col: size_t = 0 as libc::c_int as size_t;
    let mut to_col: size_t = 0 as libc::c_int as size_t;
    let mut converted: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut kickoff: bool = 1 as libc::c_int != 0;
    let mut end_of_line: bool = 0 as libc::c_int != 0;
    if line == (*openfile).edittop {
        from_col = (*openfile).firstcolumn;
    } else {
        row = (row as libc::c_ulong)
            .wrapping_sub(chunk_for((*openfile).firstcolumn, (*openfile).edittop))
            as libc::c_int as libc::c_int;
    }
    while someline != line && !someline.is_null() {
        row = (row as libc::c_ulong)
            .wrapping_add(
                (1 as libc::c_int as libc::c_ulong)
                    .wrapping_add(extra_chunks_in(someline)),
            ) as libc::c_int as libc::c_int;
        someline = (*someline).next;
    }
    if row < 0 as libc::c_int || row >= editwinrows {
        statusline(
            ALERT,
            b"Badness: tried to display a chunk on row %i -- please report a bug\0"
                as *const u8 as *const libc::c_char,
            row,
        );
        return 0 as libc::c_int;
    }
    starting_row = row;
    while !end_of_line && row < editwinrows {
        to_col = get_softwrap_breakpoint(
            (*line).data,
            from_col,
            &mut kickoff,
            &mut end_of_line,
        );
        sequel_column = if end_of_line as libc::c_int != 0 {
            0 as libc::c_int as libc::c_ulong
        } else {
            to_col
        };
        converted = display_string(
            (*line).data,
            from_col,
            to_col.wrapping_sub(from_col),
            1 as libc::c_int != 0,
            0 as libc::c_int != 0,
        );
        let fresh22 = row;
        row = row + 1;
        draw_row(fresh22, converted, line, from_col);
        rpl_free(converted as *mut libc::c_void);
        from_col = to_col;
    }
    if spotlighted as libc::c_int != 0 && line == (*openfile).current {
        spotlight_softwrapped(light_from_col, light_to_col);
    }
    return row - starting_row;
}
pub unsafe extern "C" fn line_needs_update(
    old_column: size_t,
    new_column: size_t,
) -> bool {
    if !((*openfile).mark).is_null() {
        return 1 as libc::c_int != 0
    } else {
        return get_page_start(old_column) != get_page_start(new_column)
    };
}
pub unsafe extern "C" fn go_back_chunks(
    mut nrows: libc::c_int,
    mut line: *mut *mut linestruct,
    mut leftedge: *mut size_t,
) -> libc::c_int {
    let mut i: libc::c_int = 0;
    if flags[(SOFTWRAP as libc::c_int as libc::c_ulong)
        .wrapping_div(
            (::std::mem::size_of::<libc::c_uint>() as libc::c_ulong)
                .wrapping_mul(8 as libc::c_int as libc::c_ulong),
        ) as usize]
        & (1 as libc::c_int as libc::c_uint)
            << (SOFTWRAP as libc::c_int as libc::c_ulong)
                .wrapping_rem(
                    (::std::mem::size_of::<libc::c_uint>() as libc::c_ulong)
                        .wrapping_mul(8 as libc::c_int as libc::c_ulong),
                ) != 0 as libc::c_int as libc::c_uint
    {
        i = nrows;
        while i > 0 as libc::c_int {
            let mut chunk: size_t = chunk_for(*leftedge, *line);
            *leftedge = 0 as libc::c_int as size_t;
            if chunk >= i as libc::c_ulong {
                return go_forward_chunks(
                    chunk.wrapping_sub(i as libc::c_ulong) as libc::c_int,
                    line,
                    leftedge,
                );
            }
            if *line == (*openfile).filetop {
                break;
            }
            i = (i as libc::c_ulong).wrapping_sub(chunk) as libc::c_int as libc::c_int;
            *line = (**line).prev;
            *leftedge = !(0 as libc::c_int as size_t) >> 1 as libc::c_int;
            i -= 1;
            i;
        }
        if *leftedge == !(0 as libc::c_int as size_t) >> 1 as libc::c_int {
            *leftedge = leftedge_for(*leftedge, *line);
        }
    } else {
        i = nrows;
        while i > 0 as libc::c_int && !((**line).prev).is_null() {
            *line = (**line).prev;
            i -= 1;
            i;
        }
    }
    return i;
}
pub unsafe extern "C" fn go_forward_chunks(
    mut nrows: libc::c_int,
    mut line: *mut *mut linestruct,
    mut leftedge: *mut size_t,
) -> libc::c_int {
    let mut i: libc::c_int = 0;
    if flags[(SOFTWRAP as libc::c_int as libc::c_ulong)
        .wrapping_div(
            (::std::mem::size_of::<libc::c_uint>() as libc::c_ulong)
                .wrapping_mul(8 as libc::c_int as libc::c_ulong),
        ) as usize]
        & (1 as libc::c_int as libc::c_uint)
            << (SOFTWRAP as libc::c_int as libc::c_ulong)
                .wrapping_rem(
                    (::std::mem::size_of::<libc::c_uint>() as libc::c_ulong)
                        .wrapping_mul(8 as libc::c_int as libc::c_ulong),
                ) != 0 as libc::c_int as libc::c_uint
    {
        let mut current_leftedge: size_t = *leftedge;
        let mut kickoff: bool = 1 as libc::c_int != 0;
        i = nrows;
        while i > 0 as libc::c_int {
            let mut end_of_line: bool = 0 as libc::c_int != 0;
            current_leftedge = get_softwrap_breakpoint(
                (**line).data,
                current_leftedge,
                &mut kickoff,
                &mut end_of_line,
            );
            if end_of_line {
                if *line == (*openfile).filebot {
                    break;
                }
                *line = (**line).next;
                current_leftedge = 0 as libc::c_int as size_t;
                kickoff = 1 as libc::c_int != 0;
            }
            i -= 1;
            i;
        }
        if i < nrows {
            *leftedge = current_leftedge;
        }
    } else {
        i = nrows;
        while i > 0 as libc::c_int && !((**line).next).is_null() {
            *line = (**line).next;
            i -= 1;
            i;
        }
    }
    return i;
}
pub unsafe extern "C" fn less_than_a_screenful(
    mut was_lineno: size_t,
    mut was_leftedge: size_t,
) -> bool {
    if flags[(SOFTWRAP as libc::c_int as libc::c_ulong)
        .wrapping_div(
            (::std::mem::size_of::<libc::c_uint>() as libc::c_ulong)
                .wrapping_mul(8 as libc::c_int as libc::c_ulong),
        ) as usize]
        & (1 as libc::c_int as libc::c_uint)
            << (SOFTWRAP as libc::c_int as libc::c_ulong)
                .wrapping_rem(
                    (::std::mem::size_of::<libc::c_uint>() as libc::c_ulong)
                        .wrapping_mul(8 as libc::c_int as libc::c_ulong),
                ) != 0 as libc::c_int as libc::c_uint
    {
        let mut line: *mut linestruct = (*openfile).current;
        let mut leftedge: size_t = leftedge_for(xplustabs(), (*openfile).current);
        let mut rows_left: libc::c_int = go_back_chunks(
            editwinrows - 1 as libc::c_int,
            &mut line,
            &mut leftedge,
        );
        return rows_left > 0 as libc::c_int
            || ((*line).lineno as libc::c_ulong) < was_lineno
            || (*line).lineno as libc::c_ulong == was_lineno && leftedge <= was_leftedge;
    } else {
        return ((*(*openfile).current).lineno as libc::c_ulong).wrapping_sub(was_lineno)
            < editwinrows as libc::c_ulong
    };
}
pub unsafe extern "C" fn draw_scrollbar() {
    let mut fromline: libc::c_int = ((*(*openfile).edittop).lineno
        - 1 as libc::c_int as libc::c_long) as libc::c_int;
    let mut totallines: libc::c_int = (*(*openfile).filebot).lineno as libc::c_int;
    let mut coveredlines: libc::c_int = editwinrows;
    if flags[(SOFTWRAP as libc::c_int as libc::c_ulong)
        .wrapping_div(
            (::std::mem::size_of::<libc::c_uint>() as libc::c_ulong)
                .wrapping_mul(8 as libc::c_int as libc::c_ulong),
        ) as usize]
        & (1 as libc::c_int as libc::c_uint)
            << (SOFTWRAP as libc::c_int as libc::c_ulong)
                .wrapping_rem(
                    (::std::mem::size_of::<libc::c_uint>() as libc::c_ulong)
                        .wrapping_mul(8 as libc::c_int as libc::c_ulong),
                ) != 0 as libc::c_int as libc::c_uint
    {
        let mut line: *mut linestruct = (*openfile).edittop;
        let mut extras: libc::c_int = (extra_chunks_in(line))
            .wrapping_sub(chunk_for((*openfile).firstcolumn, line)) as libc::c_int;
        while ((*line).lineno + extras as libc::c_long)
            < (fromline + editwinrows) as libc::c_long && !((*line).next).is_null()
        {
            line = (*line).next;
            extras = (extras as libc::c_ulong).wrapping_add(extra_chunks_in(line))
                as libc::c_int as libc::c_int;
        }
        coveredlines = ((*line).lineno - fromline as libc::c_long) as libc::c_int;
    }
    let mut lowest: libc::c_int = fromline * editwinrows / totallines;
    let mut highest: libc::c_int = lowest + editwinrows * coveredlines / totallines;
    if editwinrows > totallines {
        highest = editwinrows;
    }
    let mut row: libc::c_int = 0 as libc::c_int;
    while row < editwinrows {
        *bardata
            .offset(
                row as isize,
            ) = ((' ' as i32 | interface_color_pair[SCROLL_BAR as libc::c_int as usize])
            as libc::c_uint
            | (if row < lowest || row > highest {
                (1 as libc::c_uint).wrapping_sub(1 as libc::c_uint)
            } else {
                (1 as libc::c_uint) << 10 as libc::c_int + 8 as libc::c_int
            })) as libc::c_int;
        if wmove(midwin, row, COLS - 1 as libc::c_int) == -(1 as libc::c_int) {
            -(1 as libc::c_int);
        } else {
            waddch(midwin, *bardata.offset(row as isize) as chtype);
        };
        row += 1;
        row;
    }
}
pub unsafe extern "C" fn edit_scroll(mut direction: bool) {
    let mut line: *mut linestruct = 0 as *mut linestruct;
    let mut leftedge: size_t = 0;
    let mut nrows: libc::c_int = 1 as libc::c_int;
    if direction as libc::c_int == 0 as libc::c_int {
        go_back_chunks(
            1 as libc::c_int,
            &mut (*openfile).edittop,
            &mut (*openfile).firstcolumn,
        );
    } else {
        go_forward_chunks(
            1 as libc::c_int,
            &mut (*openfile).edittop,
            &mut (*openfile).firstcolumn,
        );
    }
    scrollok(midwin, 1 as libc::c_int != 0);
    wscrl(
        midwin,
        if direction as libc::c_int == 0 as libc::c_int {
            -(1 as libc::c_int)
        } else {
            1 as libc::c_int
        },
    );
    scrollok(midwin, 0 as libc::c_int != 0);
    if line_needs_update((*openfile).placewewant, 0 as libc::c_int as size_t)
        as libc::c_int != 0 && nrows < editwinrows
    {
        nrows += 1;
        nrows;
    }
    line = (*openfile).edittop;
    leftedge = (*openfile).firstcolumn;
    if direction as libc::c_int == 1 as libc::c_int {
        go_forward_chunks(editwinrows - nrows, &mut line, &mut leftedge);
    }
    if thebar != 0 {
        draw_scrollbar();
    }
    if flags[(SOFTWRAP as libc::c_int as libc::c_ulong)
        .wrapping_div(
            (::std::mem::size_of::<libc::c_uint>() as libc::c_ulong)
                .wrapping_mul(8 as libc::c_int as libc::c_ulong),
        ) as usize]
        & (1 as libc::c_int as libc::c_uint)
            << (SOFTWRAP as libc::c_int as libc::c_ulong)
                .wrapping_rem(
                    (::std::mem::size_of::<libc::c_uint>() as libc::c_ulong)
                        .wrapping_mul(8 as libc::c_int as libc::c_ulong),
                ) != 0 as libc::c_int as libc::c_uint
    {
        nrows = (nrows as libc::c_ulong).wrapping_add(chunk_for(leftedge, line))
            as libc::c_int as libc::c_int;
        if line == (*openfile).edittop {
            nrows = (nrows as libc::c_ulong)
                .wrapping_sub(chunk_for((*openfile).firstcolumn, line)) as libc::c_int
                as libc::c_int;
        }
    }
    while nrows > 0 as libc::c_int && !line.is_null() {
        nrows
            -= update_line(
                line,
                if line == (*openfile).current {
                    (*openfile).current_x
                } else {
                    0 as libc::c_int as libc::c_ulong
                },
            );
        line = (*line).next;
    }
}
pub unsafe extern "C" fn get_softwrap_breakpoint(
    mut linedata: *const libc::c_char,
    mut leftedge: size_t,
    mut kickoff: *mut bool,
    mut end_of_line: *mut bool,
) -> size_t {
    static mut text: *const libc::c_char = 0 as *const libc::c_char;
    static mut column: size_t = 0;
    let mut goal_column: size_t = leftedge.wrapping_add(editwincols as libc::c_ulong);
    let mut breaking_col: size_t = goal_column;
    let mut last_blank_col: size_t = 0 as libc::c_int as size_t;
    let mut farthest_blank: *const libc::c_char = 0 as *const libc::c_char;
    if *kickoff {
        text = linedata;
        column = 0 as libc::c_int as size_t;
        *kickoff = 0 as libc::c_int != 0;
    }
    while *text as libc::c_int != '\0' as i32 && column < leftedge {
        text = text.offset(advance_over(text, &mut column) as isize);
    }
    while *text as libc::c_int != '\0' as i32 && column <= goal_column {
        if flags[(AT_BLANKS as libc::c_int as libc::c_ulong)
            .wrapping_div(
                (::std::mem::size_of::<libc::c_uint>() as libc::c_ulong)
                    .wrapping_mul(8 as libc::c_int as libc::c_ulong),
            ) as usize]
            & (1 as libc::c_int as libc::c_uint)
                << (AT_BLANKS as libc::c_int as libc::c_ulong)
                    .wrapping_rem(
                        (::std::mem::size_of::<libc::c_uint>() as libc::c_ulong)
                            .wrapping_mul(8 as libc::c_int as libc::c_ulong),
                    ) != 0 as libc::c_int as libc::c_uint
            && is_blank_char(text) as libc::c_int != 0 && column < goal_column
        {
            farthest_blank = text;
            last_blank_col = column;
        }
        breaking_col = if *text as libc::c_int == '\t' as i32 {
            goal_column
        } else {
            column
        };
        text = text.offset(advance_over(text, &mut column) as isize);
    }
    if column <= goal_column {
        *end_of_line = column < goal_column;
        return column;
    }
    if !farthest_blank.is_null() {
        advance_over(farthest_blank, &mut last_blank_col);
        if last_blank_col <= goal_column {
            return last_blank_col;
        }
        if *farthest_blank as libc::c_int == '\t' as i32 {
            breaking_col = goal_column;
        }
    }
    return if editwincols > 1 as libc::c_int {
        breaking_col
    } else {
        column.wrapping_sub(1 as libc::c_int as libc::c_ulong)
    };
}
pub unsafe extern "C" fn get_chunk_and_edge(
    mut column: size_t,
    mut line: *mut linestruct,
    mut leftedge: *mut size_t,
) -> size_t {
    let mut current_chunk: size_t = 0 as libc::c_int as size_t;
    let mut end_of_line: bool = 0 as libc::c_int != 0;
    let mut kickoff: bool = 1 as libc::c_int != 0;
    let mut start_col: size_t = 0 as libc::c_int as size_t;
    let mut end_col: size_t = 0;
    loop {
        end_col = get_softwrap_breakpoint(
            (*line).data,
            start_col,
            &mut kickoff,
            &mut end_of_line,
        );
        if end_of_line as libc::c_int != 0 || start_col <= column && column < end_col {
            if !leftedge.is_null() {
                *leftedge = start_col;
            }
            return current_chunk;
        }
        start_col = end_col;
        current_chunk = current_chunk.wrapping_add(1);
        current_chunk;
    };
}
pub unsafe extern "C" fn extra_chunks_in(mut line: *mut linestruct) -> size_t {
    return get_chunk_and_edge(-(1 as libc::c_int) as size_t, line, 0 as *mut size_t);
}
pub unsafe extern "C" fn chunk_for(
    mut column: size_t,
    mut line: *mut linestruct,
) -> size_t {
    return get_chunk_and_edge(column, line, 0 as *mut size_t);
}
pub unsafe extern "C" fn leftedge_for(
    mut column: size_t,
    mut line: *mut linestruct,
) -> size_t {
    let mut leftedge: size_t = 0;
    get_chunk_and_edge(column, line, &mut leftedge);
    return leftedge;
}
pub unsafe extern "C" fn ensure_firstcolumn_is_aligned() {
    if flags[(SOFTWRAP as libc::c_int as libc::c_ulong)
        .wrapping_div(
            (::std::mem::size_of::<libc::c_uint>() as libc::c_ulong)
                .wrapping_mul(8 as libc::c_int as libc::c_ulong),
        ) as usize]
        & (1 as libc::c_int as libc::c_uint)
            << (SOFTWRAP as libc::c_int as libc::c_ulong)
                .wrapping_rem(
                    (::std::mem::size_of::<libc::c_uint>() as libc::c_ulong)
                        .wrapping_mul(8 as libc::c_int as libc::c_ulong),
                ) != 0 as libc::c_int as libc::c_uint
    {
        (*openfile)
            .firstcolumn = leftedge_for((*openfile).firstcolumn, (*openfile).edittop);
    } else {
        (*openfile).firstcolumn = 0 as libc::c_int as size_t;
    }
    focusing = 0 as libc::c_int != 0;
}
pub unsafe extern "C" fn actual_last_column(
    mut leftedge: size_t,
    mut column: size_t,
) -> size_t {
    if flags[(SOFTWRAP as libc::c_int as libc::c_ulong)
        .wrapping_div(
            (::std::mem::size_of::<libc::c_uint>() as libc::c_ulong)
                .wrapping_mul(8 as libc::c_int as libc::c_ulong),
        ) as usize]
        & (1 as libc::c_int as libc::c_uint)
            << (SOFTWRAP as libc::c_int as libc::c_ulong)
                .wrapping_rem(
                    (::std::mem::size_of::<libc::c_uint>() as libc::c_ulong)
                        .wrapping_mul(8 as libc::c_int as libc::c_ulong),
                ) != 0 as libc::c_int as libc::c_uint
    {
        let mut kickoff: bool = 1 as libc::c_int != 0;
        let mut last_chunk: bool = 0 as libc::c_int != 0;
        let mut end_col: size_t = (get_softwrap_breakpoint(
            (*(*openfile).current).data,
            leftedge,
            &mut kickoff,
            &mut last_chunk,
        ))
            .wrapping_sub(leftedge);
        if !last_chunk {
            end_col = end_col.wrapping_sub(1);
            end_col;
        }
        if column > end_col {
            column = end_col;
        }
    }
    return leftedge.wrapping_add(column);
}
pub unsafe extern "C" fn current_is_above_screen() -> bool {
    if flags[(SOFTWRAP as libc::c_int as libc::c_ulong)
        .wrapping_div(
            (::std::mem::size_of::<libc::c_uint>() as libc::c_ulong)
                .wrapping_mul(8 as libc::c_int as libc::c_ulong),
        ) as usize]
        & (1 as libc::c_int as libc::c_uint)
            << (SOFTWRAP as libc::c_int as libc::c_ulong)
                .wrapping_rem(
                    (::std::mem::size_of::<libc::c_uint>() as libc::c_ulong)
                        .wrapping_mul(8 as libc::c_int as libc::c_ulong),
                ) != 0 as libc::c_int as libc::c_uint
    {
        return (*(*openfile).current).lineno < (*(*openfile).edittop).lineno
            || (*(*openfile).current).lineno == (*(*openfile).edittop).lineno
                && xplustabs() < (*openfile).firstcolumn
    } else {
        return (*(*openfile).current).lineno < (*(*openfile).edittop).lineno
    };
}
pub unsafe extern "C" fn current_is_below_screen() -> bool {
    if flags[(SOFTWRAP as libc::c_int as libc::c_ulong)
        .wrapping_div(
            (::std::mem::size_of::<libc::c_uint>() as libc::c_ulong)
                .wrapping_mul(8 as libc::c_int as libc::c_ulong),
        ) as usize]
        & (1 as libc::c_int as libc::c_uint)
            << (SOFTWRAP as libc::c_int as libc::c_ulong)
                .wrapping_rem(
                    (::std::mem::size_of::<libc::c_uint>() as libc::c_ulong)
                        .wrapping_mul(8 as libc::c_int as libc::c_ulong),
                ) != 0 as libc::c_int as libc::c_uint
    {
        let mut line: *mut linestruct = (*openfile).edittop;
        let mut leftedge: size_t = (*openfile).firstcolumn;
        return go_forward_chunks(
            editwinrows - 1 as libc::c_int
                - (if flags[(ZERO as libc::c_int as libc::c_ulong)
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
                    && (currmenu == (1 as libc::c_int) << 3 as libc::c_int
                        || currmenu == (1 as libc::c_int) << 13 as libc::c_int)
                {
                    1 as libc::c_int
                } else {
                    0 as libc::c_int
                }),
            &mut line,
            &mut leftedge,
        ) == 0 as libc::c_int
            && ((*line).lineno < (*(*openfile).current).lineno
                || (*line).lineno == (*(*openfile).current).lineno
                    && leftedge < leftedge_for(xplustabs(), (*openfile).current));
    } else {
        return (*(*openfile).current).lineno
            >= (*(*openfile).edittop).lineno + editwinrows as libc::c_long
                - (if flags[(ZERO as libc::c_int as libc::c_ulong)
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
                    && (currmenu == (1 as libc::c_int) << 3 as libc::c_int
                        || currmenu == (1 as libc::c_int) << 13 as libc::c_int)
                {
                    1 as libc::c_int
                } else {
                    0 as libc::c_int
                }) as libc::c_long
    };
}
pub unsafe extern "C" fn current_is_offscreen() -> bool {
    return current_is_above_screen() as libc::c_int != 0
        || current_is_below_screen() as libc::c_int != 0;
}
pub unsafe extern "C" fn edit_redraw(
    mut old_current: *mut linestruct,
    mut manner: update_type,
) {
    let mut was_pww: size_t = (*openfile).placewewant;
    (*openfile).placewewant = xplustabs();
    if current_is_offscreen() {
        adjust_viewport(
            (if flags[(JUMPY_SCROLLING as libc::c_int as libc::c_ulong)
                .wrapping_div(
                    (::std::mem::size_of::<libc::c_uint>() as libc::c_ulong)
                        .wrapping_mul(8 as libc::c_int as libc::c_ulong),
                ) as usize]
                & (1 as libc::c_int as libc::c_uint)
                    << (JUMPY_SCROLLING as libc::c_int as libc::c_ulong)
                        .wrapping_rem(
                            (::std::mem::size_of::<libc::c_uint>() as libc::c_ulong)
                                .wrapping_mul(8 as libc::c_int as libc::c_ulong),
                        ) != 0 as libc::c_int as libc::c_uint
            {
                CENTERING as libc::c_int as libc::c_uint
            } else {
                manner as libc::c_uint
            }) as update_type,
        );
        refresh_needed = 1 as libc::c_int != 0;
        return;
    }
    if !((*openfile).mark).is_null() {
        let mut line: *mut linestruct = old_current;
        while line != (*openfile).current {
            update_line(line, 0 as libc::c_int as size_t);
            line = if (*line).lineno > (*(*openfile).current).lineno {
                (*line).prev
            } else {
                (*line).next
            };
        }
    } else if old_current != (*openfile).current
        && get_page_start(was_pww) > 0 as libc::c_int as libc::c_ulong
    {
        update_line(old_current, 0 as libc::c_int as size_t);
    }
    if line_needs_update(was_pww, (*openfile).placewewant) as libc::c_int != 0
        || old_current != (*openfile).current
            && get_page_start((*openfile).placewewant)
                > 0 as libc::c_int as libc::c_ulong
    {
        update_line((*openfile).current, (*openfile).current_x);
    }
}
pub unsafe extern "C" fn edit_refresh() {
    let mut line: *mut linestruct = 0 as *mut linestruct;
    let mut row: libc::c_int = 0 as libc::c_int;
    if !((*openfile).syntax).is_null() && !have_palette
        && !(flags[(NO_SYNTAX as libc::c_int as libc::c_ulong)
            .wrapping_div(
                (::std::mem::size_of::<libc::c_uint>() as libc::c_ulong)
                    .wrapping_mul(8 as libc::c_int as libc::c_ulong),
            ) as usize]
            & (1 as libc::c_int as libc::c_uint)
                << (NO_SYNTAX as libc::c_int as libc::c_ulong)
                    .wrapping_rem(
                        (::std::mem::size_of::<libc::c_uint>() as libc::c_ulong)
                            .wrapping_mul(8 as libc::c_int as libc::c_ulong),
                    ) != 0 as libc::c_int as libc::c_uint)
        && has_colors() as libc::c_int != 0
    {
        prepare_palette();
    }
    if recook {
        precalc_multicolorinfo();
        perturbed = 0 as libc::c_int != 0;
        recook = 0 as libc::c_int != 0;
    }
    if current_is_offscreen() {
        adjust_viewport(
            (if focusing as libc::c_int != 0
                || flags[(JUMPY_SCROLLING as libc::c_int as libc::c_ulong)
                    .wrapping_div(
                        (::std::mem::size_of::<libc::c_uint>() as libc::c_ulong)
                            .wrapping_mul(8 as libc::c_int as libc::c_ulong),
                    ) as usize]
                    & (1 as libc::c_int as libc::c_uint)
                        << (JUMPY_SCROLLING as libc::c_int as libc::c_ulong)
                            .wrapping_rem(
                                (::std::mem::size_of::<libc::c_uint>() as libc::c_ulong)
                                    .wrapping_mul(8 as libc::c_int as libc::c_ulong),
                            ) != 0 as libc::c_int as libc::c_uint
            {
                CENTERING as libc::c_int
            } else {
                FLOWING as libc::c_int
            }) as update_type,
        );
    }
    if thebar != 0 {
        draw_scrollbar();
    }
    line = (*openfile).edittop;
    while row < editwinrows && !line.is_null() {
        if line == (*openfile).current {
            row += update_line(line, (*openfile).current_x);
        } else {
            row += update_line(line, 0 as libc::c_int as size_t);
        }
        line = (*line).next;
    }
    while row < editwinrows {
        blank_row(midwin, row);
        if thebar != 0 {
            if wmove(midwin, row, COLS - 1 as libc::c_int) == -(1 as libc::c_int) {
                -(1 as libc::c_int);
            } else {
                waddch(midwin, *bardata.offset(row as isize) as chtype);
            };
        }
        row += 1;
        row;
    }
    place_the_cursor();
    wnoutrefresh(midwin);
    refresh_needed = 0 as libc::c_int != 0;
}
pub unsafe extern "C" fn adjust_viewport(mut manner: update_type) {
    let mut goal: libc::c_int = 0 as libc::c_int;
    if manner as libc::c_uint == STATIONARY as libc::c_int as libc::c_uint {
        goal = (*openfile).current_y as libc::c_int;
    } else if manner as libc::c_uint == CENTERING as libc::c_int as libc::c_uint {
        goal = editwinrows / 2 as libc::c_int;
    } else if !current_is_above_screen() {
        goal = editwinrows - 1 as libc::c_int
            - (if flags[(ZERO as libc::c_int as libc::c_ulong)
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
                && (currmenu == (1 as libc::c_int) << 3 as libc::c_int
                    || currmenu == (1 as libc::c_int) << 13 as libc::c_int)
            {
                1 as libc::c_int
            } else {
                0 as libc::c_int
            });
    }
    (*openfile).edittop = (*openfile).current;
    if flags[(SOFTWRAP as libc::c_int as libc::c_ulong)
        .wrapping_div(
            (::std::mem::size_of::<libc::c_uint>() as libc::c_ulong)
                .wrapping_mul(8 as libc::c_int as libc::c_ulong),
        ) as usize]
        & (1 as libc::c_int as libc::c_uint)
            << (SOFTWRAP as libc::c_int as libc::c_ulong)
                .wrapping_rem(
                    (::std::mem::size_of::<libc::c_uint>() as libc::c_ulong)
                        .wrapping_mul(8 as libc::c_int as libc::c_ulong),
                ) != 0 as libc::c_int as libc::c_uint
    {
        (*openfile).firstcolumn = leftedge_for(xplustabs(), (*openfile).current);
    }
    go_back_chunks(goal, &mut (*openfile).edittop, &mut (*openfile).firstcolumn);
}
pub unsafe extern "C" fn full_refresh() {
    wrefresh(curscr);
}
pub unsafe extern "C" fn draw_all_subwindows() {
    if currmenu
        & !((1 as libc::c_int) << 10 as libc::c_int
            | (1 as libc::c_int) << 11 as libc::c_int
            | (1 as libc::c_int) << 12 as libc::c_int) != 0
    {
        titlebar(title);
    }
    if inhelp {
        close_buffer();
        wrap_help_text_into_buffer();
    } else if currmenu
        & !((1 as libc::c_int) << 10 as libc::c_int
            | (1 as libc::c_int) << 11 as libc::c_int
            | (1 as libc::c_int) << 12 as libc::c_int) != 0
    {
        edit_refresh();
    }
    bottombars(currmenu);
}
pub unsafe extern "C" fn report_cursor_position() {
    let mut fullwidth: size_t = (breadth((*(*openfile).current).data))
        .wrapping_add(1 as libc::c_int as libc::c_ulong);
    let mut column: size_t = (xplustabs())
        .wrapping_add(1 as libc::c_int as libc::c_ulong);
    let mut linepct: libc::c_int = 0;
    let mut colpct: libc::c_int = 0;
    let mut charpct: libc::c_int = 0;
    let mut saved_byte: libc::c_char = 0;
    let mut sum: size_t = 0;
    saved_byte = *((*(*openfile).current).data).offset((*openfile).current_x as isize);
    *((*(*openfile).current).data)
        .offset((*openfile).current_x as isize) = '\0' as i32 as libc::c_char;
    sum = number_of_characters_in((*openfile).filetop, (*openfile).current);
    *((*(*openfile).current).data).offset((*openfile).current_x as isize) = saved_byte;
    linepct = (100 as libc::c_int as libc::c_long * (*(*openfile).current).lineno
        / (*(*openfile).filebot).lineno) as libc::c_int;
    colpct = (100 as libc::c_int as libc::c_ulong)
        .wrapping_mul(column)
        .wrapping_div(fullwidth) as libc::c_int;
    charpct = (if (*openfile).totsize == 0 as libc::c_int as libc::c_ulong {
        0 as libc::c_int as libc::c_ulong
    } else {
        (100 as libc::c_int as libc::c_ulong)
            .wrapping_mul(sum)
            .wrapping_div((*openfile).totsize)
    }) as libc::c_int;
    statusline(
        INFO,
        dcgettext(
            0 as *const libc::c_char,
            b"line %*zd/%zd (%2d%%), col %2zu/%2zu (%3d%%), char %*zu/%zu (%2d%%)\0"
                as *const u8 as *const libc::c_char,
            5 as libc::c_int,
        ),
        digits((*(*openfile).filebot).lineno),
        (*(*openfile).current).lineno,
        (*(*openfile).filebot).lineno,
        linepct,
        column,
        fullwidth,
        colpct,
        digits((*openfile).totsize as ssize_t),
        sum,
        (*openfile).totsize,
        charpct,
    );
}
pub unsafe extern "C" fn spotlight(mut from_col: size_t, mut to_col: size_t) {
    let mut right_edge: size_t = (get_page_start(from_col))
        .wrapping_add(editwincols as libc::c_ulong);
    let mut overshoots: bool = to_col > right_edge;
    let mut word: *mut libc::c_char = 0 as *mut libc::c_char;
    place_the_cursor();
    if overshoots {
        to_col = right_edge;
    }
    if to_col == from_col {
        word = copy_of(b" \0" as *const u8 as *const libc::c_char);
        to_col = to_col.wrapping_add(1);
        to_col;
    } else {
        word = display_string(
            (*(*openfile).current).data,
            from_col,
            to_col.wrapping_sub(from_col),
            0 as libc::c_int != 0,
            overshoots,
        );
    }
    wattr_on(
        midwin,
        interface_color_pair[SPOTLIGHTED as libc::c_int as usize] as attr_t,
        0 as *mut libc::c_void,
    );
    waddnstr(midwin, word, actual_x(word, to_col) as libc::c_int);
    if overshoots {
        if wmove(
            midwin,
            (*openfile).current_y as libc::c_int,
            COLS - 1 as libc::c_int - thebar,
        ) == -(1 as libc::c_int)
        {
            -(1 as libc::c_int);
        } else {
            waddch(midwin, '>' as i32 as chtype);
        };
    }
    wattr_off(
        midwin,
        interface_color_pair[SPOTLIGHTED as libc::c_int as usize] as attr_t,
        0 as *mut libc::c_void,
    );
    rpl_free(word as *mut libc::c_void);
}
pub unsafe extern "C" fn spotlight_softwrapped(
    mut from_col: size_t,
    mut to_col: size_t,
) {
    let mut row: ssize_t = 0;
    let mut leftedge: size_t = leftedge_for(from_col, (*openfile).current);
    let mut break_col: size_t = 0;
    let mut end_of_line: bool = 0 as libc::c_int != 0;
    let mut kickoff: bool = 1 as libc::c_int != 0;
    let mut word: *mut libc::c_char = 0 as *mut libc::c_char;
    place_the_cursor();
    row = (*openfile).current_y;
    while row < editwinrows as libc::c_long {
        break_col = get_softwrap_breakpoint(
            (*(*openfile).current).data,
            leftedge,
            &mut kickoff,
            &mut end_of_line,
        );
        if break_col >= to_col {
            end_of_line = 1 as libc::c_int != 0;
            break_col = to_col;
        }
        if break_col == from_col {
            word = copy_of(b" \0" as *const u8 as *const libc::c_char);
            break_col = break_col.wrapping_add(1);
            break_col;
        } else {
            word = display_string(
                (*(*openfile).current).data,
                from_col,
                break_col.wrapping_sub(from_col),
                0 as libc::c_int != 0,
                0 as libc::c_int != 0,
            );
        }
        wattr_on(
            midwin,
            interface_color_pair[SPOTLIGHTED as libc::c_int as usize] as attr_t,
            0 as *mut libc::c_void,
        );
        waddnstr(midwin, word, actual_x(word, break_col) as libc::c_int);
        wattr_off(
            midwin,
            interface_color_pair[SPOTLIGHTED as libc::c_int as usize] as attr_t,
            0 as *mut libc::c_void,
        );
        rpl_free(word as *mut libc::c_void);
        if end_of_line {
            break;
        }
        row += 1;
        wmove(midwin, row as libc::c_int, margin);
        leftedge = break_col;
        from_col = break_col;
    }
}
pub unsafe extern "C" fn do_credits() {
    let mut with_interface: bool = !(flags[(ZERO as libc::c_int as libc::c_ulong)
        .wrapping_div(
            (::std::mem::size_of::<libc::c_uint>() as libc::c_ulong)
                .wrapping_mul(8 as libc::c_int as libc::c_ulong),
        ) as usize]
        & (1 as libc::c_int as libc::c_uint)
            << (ZERO as libc::c_int as libc::c_ulong)
                .wrapping_rem(
                    (::std::mem::size_of::<libc::c_uint>() as libc::c_ulong)
                        .wrapping_mul(8 as libc::c_int as libc::c_ulong),
                ) != 0 as libc::c_int as libc::c_uint);
    let mut with_help: bool = !(flags[(NO_HELP as libc::c_int as libc::c_ulong)
        .wrapping_div(
            (::std::mem::size_of::<libc::c_uint>() as libc::c_ulong)
                .wrapping_mul(8 as libc::c_int as libc::c_ulong),
        ) as usize]
        & (1 as libc::c_int as libc::c_uint)
            << (NO_HELP as libc::c_int as libc::c_ulong)
                .wrapping_rem(
                    (::std::mem::size_of::<libc::c_uint>() as libc::c_ulong)
                        .wrapping_mul(8 as libc::c_int as libc::c_ulong),
                ) != 0 as libc::c_int as libc::c_uint);
    let mut crpos: libc::c_int = 0 as libc::c_int;
    let mut xlpos: libc::c_int = 0 as libc::c_int;
    let mut credits: [*const libc::c_char; 52] = [
        0 as *const libc::c_char,
        0 as *const libc::c_char,
        b"7.2\0" as *const u8 as *const libc::c_char,
        b"\0" as *const u8 as *const libc::c_char,
        0 as *const libc::c_char,
        b"Chris Allegretta\0" as *const u8 as *const libc::c_char,
        b"Benno Schulenberg\0" as *const u8 as *const libc::c_char,
        b"David Lawrence Ramsey\0" as *const u8 as *const libc::c_char,
        b"Jordi Mallach\0" as *const u8 as *const libc::c_char,
        b"David Benbennick\0" as *const u8 as *const libc::c_char,
        b"Rocco Corsi\0" as *const u8 as *const libc::c_char,
        b"Mike Frysinger\0" as *const u8 as *const libc::c_char,
        b"Adam Rogoyski\0" as *const u8 as *const libc::c_char,
        b"Rob Siemborski\0" as *const u8 as *const libc::c_char,
        b"Mark Majeres\0" as *const u8 as *const libc::c_char,
        b"Ken Tyler\0" as *const u8 as *const libc::c_char,
        b"Sven Guckes\0" as *const u8 as *const libc::c_char,
        b"Bill Soudan\0" as *const u8 as *const libc::c_char,
        b"Christian Weisgerber\0" as *const u8 as *const libc::c_char,
        b"Erik Andersen\0" as *const u8 as *const libc::c_char,
        b"Big Gaute\0" as *const u8 as *const libc::c_char,
        b"Joshua Jensen\0" as *const u8 as *const libc::c_char,
        b"Ryan Krebs\0" as *const u8 as *const libc::c_char,
        b"Albert Chin\0" as *const u8 as *const libc::c_char,
        b"\0" as *const u8 as *const libc::c_char,
        0 as *const libc::c_char,
        b"Monique, Brielle & Joseph\0" as *const u8 as *const libc::c_char,
        b"Plattsburgh State University\0" as *const u8 as *const libc::c_char,
        b"Benet Laboratories\0" as *const u8 as *const libc::c_char,
        b"Amy Allegretta\0" as *const u8 as *const libc::c_char,
        b"Linda Young\0" as *const u8 as *const libc::c_char,
        b"Jeremy Robichaud\0" as *const u8 as *const libc::c_char,
        b"Richard Kolb II\0" as *const u8 as *const libc::c_char,
        0 as *const libc::c_char,
        b"Linus Torvalds\0" as *const u8 as *const libc::c_char,
        0 as *const libc::c_char,
        0 as *const libc::c_char,
        b"Thomas Dickey\0" as *const u8 as *const libc::c_char,
        b"Pavel Curtis\0" as *const u8 as *const libc::c_char,
        b"Zeyd Ben-Halim\0" as *const u8 as *const libc::c_char,
        b"Eric S. Raymond\0" as *const u8 as *const libc::c_char,
        0 as *const libc::c_char,
        b"\0" as *const u8 as *const libc::c_char,
        b"\0" as *const u8 as *const libc::c_char,
        0 as *const libc::c_char,
        b"\0" as *const u8 as *const libc::c_char,
        b"\0" as *const u8 as *const libc::c_char,
        b"(C) 2023\0" as *const u8 as *const libc::c_char,
        b"Free Software Foundation, Inc.\0" as *const u8 as *const libc::c_char,
        b"\0" as *const u8 as *const libc::c_char,
        b"\0" as *const u8 as *const libc::c_char,
        b"https://nano-editor.org/\0" as *const u8 as *const libc::c_char,
    ];
    let mut xlcredits: [*const libc::c_char; 9] = [
        b"The nano text editor\0" as *const u8 as *const libc::c_char,
        b"version\0" as *const u8 as *const libc::c_char,
        b"Brought to you by:\0" as *const u8 as *const libc::c_char,
        b"Special thanks to:\0" as *const u8 as *const libc::c_char,
        b"The Free Software Foundation\0" as *const u8 as *const libc::c_char,
        b"the many translators and the TP\0" as *const u8 as *const libc::c_char,
        b"For ncurses:\0" as *const u8 as *const libc::c_char,
        b"and anyone else we forgot...\0" as *const u8 as *const libc::c_char,
        b"Thank you for using nano!\0" as *const u8 as *const libc::c_char,
    ];
    if with_interface as libc::c_int != 0 || with_help as libc::c_int != 0 {
        flags[(ZERO as libc::c_int as libc::c_ulong)
            .wrapping_div(
                (::std::mem::size_of::<libc::c_uint>() as libc::c_ulong)
                    .wrapping_mul(8 as libc::c_int as libc::c_ulong),
            ) as usize]
            |= (1 as libc::c_int as libc::c_uint)
                << (ZERO as libc::c_int as libc::c_ulong)
                    .wrapping_rem(
                        (::std::mem::size_of::<libc::c_uint>() as libc::c_ulong)
                            .wrapping_mul(8 as libc::c_int as libc::c_ulong),
                    );
        flags[(NO_HELP as libc::c_int as libc::c_ulong)
            .wrapping_div(
                (::std::mem::size_of::<libc::c_uint>() as libc::c_ulong)
                    .wrapping_mul(8 as libc::c_int as libc::c_ulong),
            ) as usize]
            |= (1 as libc::c_int as libc::c_uint)
                << (NO_HELP as libc::c_int as libc::c_ulong)
                    .wrapping_rem(
                        (::std::mem::size_of::<libc::c_uint>() as libc::c_ulong)
                            .wrapping_mul(8 as libc::c_int as libc::c_ulong),
                    );
        window_init();
    }
    nodelay(midwin, 1 as libc::c_int != 0);
    scrollok(midwin, 1 as libc::c_int != 0);
    blank_edit();
    wrefresh(midwin);
    napms(600 as libc::c_int);
    crpos = 0 as libc::c_int;
    while crpos < 52 as libc::c_int + editwinrows / 2 as libc::c_int {
        if crpos < 52 as libc::c_int {
            let mut text: *const libc::c_char = credits[crpos as usize];
            if text.is_null() {
                let fresh23 = xlpos;
                xlpos = xlpos + 1;
                text = dcgettext(
                    0 as *const libc::c_char,
                    xlcredits[fresh23 as usize],
                    5 as libc::c_int,
                );
            }
            if wmove(
                midwin,
                editwinrows - 1 as libc::c_int,
                (COLS as libc::c_ulong)
                    .wrapping_sub(breadth(text))
                    .wrapping_div(2 as libc::c_int as libc::c_ulong) as libc::c_int,
            ) == -(1 as libc::c_int)
            {
                -(1 as libc::c_int);
            } else {
                waddnstr(midwin, text, -(1 as libc::c_int));
            };
            wrefresh(midwin);
        }
        if wgetch(midwin) != -(1 as libc::c_int) {
            break;
        }
        napms(600 as libc::c_int);
        wscrl(midwin, 1 as libc::c_int);
        wrefresh(midwin);
        if wgetch(midwin) != -(1 as libc::c_int) {
            break;
        }
        napms(600 as libc::c_int);
        wscrl(midwin, 1 as libc::c_int);
        wrefresh(midwin);
        crpos += 1;
        crpos;
    }
    if with_interface {
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
    }
    if with_help {
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
    }
    window_init();
    scrollok(midwin, 0 as libc::c_int != 0);
    nodelay(midwin, 0 as libc::c_int != 0);
    draw_all_subwindows();
}
