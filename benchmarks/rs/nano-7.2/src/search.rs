use ::libc;
use ::c2rust_bitfields;
extern "C" {
    pub type re_dfa_t;
    pub type ldat;
    fn napms(_: libc::c_int) -> libc::c_int;
    fn nodelay(_: *mut WINDOW, _: bool) -> libc::c_int;
    fn wgetch(_: *mut WINDOW) -> libc::c_int;
    static mut COLS: libc::c_int;
    static mut LINES: libc::c_int;
    fn sprintf(_: *mut libc::c_char, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn time(__timer: *mut time_t) -> time_t;
    fn rpl_free(ptr: *mut libc::c_void);
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
    static mut meta_key: bool;
    static mut inhelp: bool;
    static mut as_an_at: bool;
    static mut answer: *mut libc::c_char;
    static mut last_search: *mut libc::c_char;
    static mut didfind: libc::c_int;
    static mut flags: [libc::c_uint; 4];
    static mut midwin: *mut WINDOW;
    static mut editwinrows: libc::c_int;
    static mut editwincols: libc::c_int;
    static mut openfile: *mut openfilestruct;
    static mut matchbrackets: *mut libc::c_char;
    static mut perturbed: bool;
    static mut recook: bool;
    static mut refresh_needed: bool;
    static mut currmenu: libc::c_int;
    static mut search_history: *mut linestruct;
    static mut replace_history: *mut linestruct;
    static mut searchbot: *mut linestruct;
    static mut search_regexp: regex_t;
    static mut regmatches: [regmatch_t; 10];
    static mut spotlighted: bool;
    static mut light_from_col: size_t;
    static mut light_to_col: size_t;
    fn char_length(pointer: *const libc::c_char) -> libc::c_int;
    fn mbstrlen(pointer: *const libc::c_char) -> size_t;
    fn step_left(buf: *const libc::c_char, pos: size_t) -> size_t;
    fn step_right(buf: *const libc::c_char, pos: size_t) -> size_t;
    fn mbstrchr(
        string: *const libc::c_char,
        chr: *const libc::c_char,
    ) -> *mut libc::c_char;
    fn mbstrpbrk(
        string: *const libc::c_char,
        accept: *const libc::c_char,
    ) -> *mut libc::c_char;
    fn mbrevstrpbrk(
        head: *const libc::c_char,
        accept: *const libc::c_char,
        pointer: *const libc::c_char,
    ) -> *mut libc::c_char;
    fn check_the_multis(line: *mut linestruct);
    fn set_modified();
    fn func_from_key(keycode: libc::c_int) -> functionptrtype;
    fn update_history(
        item: *mut *mut linestruct,
        text: *const libc::c_char,
        avoid_duplicates: bool,
    );
    fn print_view_warning();
    fn regenerate_screen();
    fn do_prompt(
        menu: libc::c_int,
        provided: *const libc::c_char,
        history_list: *mut *mut linestruct,
        refresh_func: Option::<unsafe extern "C" fn() -> ()>,
        msg: *const libc::c_char,
        _: ...
    ) -> libc::c_int;
    fn ask_user(withall: bool, question: *const libc::c_char) -> libc::c_int;
    fn nmalloc(howmuch: size_t) -> *mut libc::c_void;
    fn statusline(importance: message_type, msg: *const libc::c_char, _: ...);
    fn wipe_statusbar();
    fn wideness(text: *const libc::c_char, maxlen: size_t) -> size_t;
    fn xplustabs() -> size_t;
    fn statusbar(msg: *const libc::c_char);
    fn get_input(win: *mut WINDOW) -> libc::c_int;
    fn do_cancel();
    fn is_separate_word(
        position: size_t,
        length: size_t,
        buf: *const libc::c_char,
    ) -> bool;
    fn strstrwrapper(
        haystack: *const libc::c_char,
        needle: *const libc::c_char,
        start: *const libc::c_char,
    ) -> *const libc::c_char;
    fn adjust_viewport(manner: update_type);
    fn leftedge_for(column: size_t, line: *mut linestruct) -> size_t;
    fn go_forward_chunks(
        nrows: libc::c_int,
        line: *mut *mut linestruct,
        leftedge: *mut size_t,
    ) -> libc::c_int;
    fn breadth(text: *const libc::c_char) -> size_t;
    fn actual_x(text: *const libc::c_char, column: size_t) -> size_t;
    fn parse_line_column(
        str: *const libc::c_char,
        line: *mut ssize_t,
        column: *mut ssize_t,
    ) -> bool;
    fn edit_refresh();
    fn flip_goto();
    fn flip_replace();
    fn regexp_void();
    fn backwards_void();
    fn case_sens_void();
    fn edit_redraw(old_current: *mut linestruct, manner: update_type);
    fn display_string(
        buf: *const libc::c_char,
        column: size_t,
        span: size_t,
        isdata: bool,
        isprompt: bool,
    ) -> *mut libc::c_char;
    fn new_magicline();
    fn mark_is_before_cursor() -> bool;
    fn add_undo(action: undo_type, message: *const libc::c_char);
    fn get_region(
        top: *mut *mut linestruct,
        top_x: *mut size_t,
        bot: *mut *mut linestruct,
        bot_x: *mut size_t,
    );
    fn mallocstrcpy(
        dest: *mut libc::c_char,
        src: *const libc::c_char,
    ) -> *mut libc::c_char;
    fn copy_of(string: *const libc::c_char) -> *mut libc::c_char;
    fn line_from_number(number: ssize_t) -> *mut linestruct;
    fn update_line(line: *mut linestruct, index: size_t) -> libc::c_int;
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
    fn strcpy(_: *mut libc::c_char, _: *const libc::c_char) -> *mut libc::c_char;
    fn strncpy(
        _: *mut libc::c_char,
        _: *const libc::c_char,
        _: libc::c_ulong,
    ) -> *mut libc::c_char;
    fn strcat(_: *mut libc::c_char, _: *const libc::c_char) -> *mut libc::c_char;
    fn strncmp(
        _: *const libc::c_char,
        _: *const libc::c_char,
        _: libc::c_ulong,
    ) -> libc::c_int;
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
pub type __sig_atomic_t = libc::c_int;
pub type ssize_t = __ssize_t;
pub type time_t = __time_t;
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
pub type functionptrtype = Option::<unsafe extern "C" fn() -> ()>;
static mut came_full_circle: bool = 0 as libc::c_int != 0;
static mut have_compiled_regexp: bool = 0 as libc::c_int != 0;
pub unsafe extern "C" fn regexp_init(mut regexp: *const libc::c_char) -> bool {
    let mut value: libc::c_int = rpl_regcomp(
        &mut search_regexp,
        regexp,
        1 as libc::c_int
            | (if flags[(CASE_SENSITIVE as libc::c_int as libc::c_ulong)
                .wrapping_div(
                    (::std::mem::size_of::<libc::c_uint>() as libc::c_ulong)
                        .wrapping_mul(8 as libc::c_int as libc::c_ulong),
                ) as usize]
                & (1 as libc::c_int as libc::c_uint)
                    << (CASE_SENSITIVE as libc::c_int as libc::c_ulong)
                        .wrapping_rem(
                            (::std::mem::size_of::<libc::c_uint>() as libc::c_ulong)
                                .wrapping_mul(8 as libc::c_int as libc::c_ulong),
                        ) != 0 as libc::c_int as libc::c_uint
            {
                0 as libc::c_int
            } else {
                (1 as libc::c_int) << 1 as libc::c_int
            }),
    );
    if value != 0 as libc::c_int {
        let mut len: size_t = rpl_regerror(
            value,
            &mut search_regexp,
            0 as *mut libc::c_char,
            0 as libc::c_int as size_t,
        );
        let mut str: *mut libc::c_char = nmalloc(len) as *mut libc::c_char;
        rpl_regerror(value, &mut search_regexp, str, len);
        statusline(
            AHEM,
            dcgettext(
                0 as *const libc::c_char,
                b"Bad regex \"%s\": %s\0" as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ),
            regexp,
            str,
        );
        rpl_free(str as *mut libc::c_void);
        return 0 as libc::c_int != 0;
    }
    have_compiled_regexp = 1 as libc::c_int != 0;
    return 1 as libc::c_int != 0;
}
pub unsafe extern "C" fn tidy_up_after_search() {
    if have_compiled_regexp {
        rpl_regfree(&mut search_regexp);
        have_compiled_regexp = 0 as libc::c_int != 0;
    }
    if !((*openfile).mark).is_null() {
        refresh_needed = 1 as libc::c_int != 0;
    }
    recook = (recook as libc::c_int | perturbed as libc::c_int) != 0;
}
pub unsafe extern "C" fn search_init(mut replacing: bool, mut retain_answer: bool) {
    let mut thedefault: *mut libc::c_char = 0 as *mut libc::c_char;
    if *last_search as libc::c_int != '\0' as i32 {
        let mut disp: *mut libc::c_char = display_string(
            last_search,
            0 as libc::c_int as size_t,
            (COLS / 3 as libc::c_int) as size_t,
            0 as libc::c_int != 0,
            0 as libc::c_int != 0,
        );
        thedefault = nmalloc(
            (strlen(disp)).wrapping_add(7 as libc::c_int as libc::c_ulong),
        ) as *mut libc::c_char;
        sprintf(
            thedefault,
            b" [%s%s]\0" as *const u8 as *const libc::c_char,
            disp,
            if breadth(last_search) > (COLS / 3 as libc::c_int) as libc::c_ulong {
                b"...\0" as *const u8 as *const libc::c_char
            } else {
                b"\0" as *const u8 as *const libc::c_char
            },
        );
        rpl_free(disp as *mut libc::c_void);
    } else {
        thedefault = copy_of(b"\0" as *const u8 as *const libc::c_char);
    }
    loop {
        let mut function: functionptrtype = None;
        let mut response: libc::c_int = do_prompt(
            if inhelp as libc::c_int != 0 {
                (1 as libc::c_int) << 15 as libc::c_int
            } else if replacing as libc::c_int != 0 {
                (1 as libc::c_int) << 2 as libc::c_int
            } else {
                (1 as libc::c_int) << 1 as libc::c_int
            },
            if retain_answer as libc::c_int != 0 {
                answer as *const libc::c_char
            } else {
                b"\0" as *const u8 as *const libc::c_char
            },
            &mut search_history as *mut *mut linestruct,
            Some(edit_refresh as unsafe extern "C" fn() -> ()),
            b"%s%s%s%s%s%s\0" as *const u8 as *const libc::c_char,
            dcgettext(
                0 as *const libc::c_char,
                b"Search\0" as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ),
            if flags[(CASE_SENSITIVE as libc::c_int as libc::c_ulong)
                .wrapping_div(
                    (::std::mem::size_of::<libc::c_uint>() as libc::c_ulong)
                        .wrapping_mul(8 as libc::c_int as libc::c_ulong),
                ) as usize]
                & (1 as libc::c_int as libc::c_uint)
                    << (CASE_SENSITIVE as libc::c_int as libc::c_ulong)
                        .wrapping_rem(
                            (::std::mem::size_of::<libc::c_uint>() as libc::c_ulong)
                                .wrapping_mul(8 as libc::c_int as libc::c_ulong),
                        ) != 0 as libc::c_int as libc::c_uint
            {
                dcgettext(
                    0 as *const libc::c_char,
                    b" [Case Sensitive]\0" as *const u8 as *const libc::c_char,
                    5 as libc::c_int,
                ) as *const libc::c_char
            } else {
                b"\0" as *const u8 as *const libc::c_char
            },
            if flags[(USE_REGEXP as libc::c_int as libc::c_ulong)
                .wrapping_div(
                    (::std::mem::size_of::<libc::c_uint>() as libc::c_ulong)
                        .wrapping_mul(8 as libc::c_int as libc::c_ulong),
                ) as usize]
                & (1 as libc::c_int as libc::c_uint)
                    << (USE_REGEXP as libc::c_int as libc::c_ulong)
                        .wrapping_rem(
                            (::std::mem::size_of::<libc::c_uint>() as libc::c_ulong)
                                .wrapping_mul(8 as libc::c_int as libc::c_ulong),
                        ) != 0 as libc::c_int as libc::c_uint
            {
                dcgettext(
                    0 as *const libc::c_char,
                    b" [Regexp]\0" as *const u8 as *const libc::c_char,
                    5 as libc::c_int,
                ) as *const libc::c_char
            } else {
                b"\0" as *const u8 as *const libc::c_char
            },
            if flags[(BACKWARDS_SEARCH as libc::c_int as libc::c_ulong)
                .wrapping_div(
                    (::std::mem::size_of::<libc::c_uint>() as libc::c_ulong)
                        .wrapping_mul(8 as libc::c_int as libc::c_ulong),
                ) as usize]
                & (1 as libc::c_int as libc::c_uint)
                    << (BACKWARDS_SEARCH as libc::c_int as libc::c_ulong)
                        .wrapping_rem(
                            (::std::mem::size_of::<libc::c_uint>() as libc::c_ulong)
                                .wrapping_mul(8 as libc::c_int as libc::c_ulong),
                        ) != 0 as libc::c_int as libc::c_uint
            {
                dcgettext(
                    0 as *const libc::c_char,
                    b" [Backwards]\0" as *const u8 as *const libc::c_char,
                    5 as libc::c_int,
                ) as *const libc::c_char
            } else {
                b"\0" as *const u8 as *const libc::c_char
            },
            if replacing as libc::c_int != 0 {
                (if !((*openfile).mark).is_null() {
                    dcgettext(
                        0 as *const libc::c_char,
                        b" (to replace) in selection\0" as *const u8
                            as *const libc::c_char,
                        5 as libc::c_int,
                    )
                } else {
                    dcgettext(
                        0 as *const libc::c_char,
                        b" (to replace)\0" as *const u8 as *const libc::c_char,
                        5 as libc::c_int,
                    )
                }) as *const libc::c_char
            } else {
                b"\0" as *const u8 as *const libc::c_char
            },
            thedefault,
        );
        if response == -(1 as libc::c_int)
            || response == -(2 as libc::c_int)
                && *last_search as libc::c_int == '\0' as i32
        {
            statusbar(
                dcgettext(
                    0 as *const libc::c_char,
                    b"Cancelled\0" as *const u8 as *const libc::c_char,
                    5 as libc::c_int,
                ),
            );
            break;
        } else if response == 0 as libc::c_int || response == -(2 as libc::c_int) {
            if *answer as libc::c_int != '\0' as i32 {
                last_search = mallocstrcpy(last_search, answer);
                update_history(&mut search_history, answer, 1 as libc::c_int != 0);
            }
            if flags[(USE_REGEXP as libc::c_int as libc::c_ulong)
                .wrapping_div(
                    (::std::mem::size_of::<libc::c_uint>() as libc::c_ulong)
                        .wrapping_mul(8 as libc::c_int as libc::c_ulong),
                ) as usize]
                & (1 as libc::c_int as libc::c_uint)
                    << (USE_REGEXP as libc::c_int as libc::c_ulong)
                        .wrapping_rem(
                            (::std::mem::size_of::<libc::c_uint>() as libc::c_ulong)
                                .wrapping_mul(8 as libc::c_int as libc::c_ulong),
                        ) != 0 as libc::c_int as libc::c_uint
                && !regexp_init(last_search)
            {
                break;
            }
            if replacing {
                ask_for_and_do_replacements();
            } else {
                go_looking();
            }
            break;
        } else {
            retain_answer = 1 as libc::c_int != 0;
            function = func_from_key(response);
            if function == Some(case_sens_void as unsafe extern "C" fn() -> ()) {
                flags[(CASE_SENSITIVE as libc::c_int as libc::c_ulong)
                    .wrapping_div(
                        (::std::mem::size_of::<libc::c_uint>() as libc::c_ulong)
                            .wrapping_mul(8 as libc::c_int as libc::c_ulong),
                    ) as usize]
                    ^= (1 as libc::c_int as libc::c_uint)
                        << (CASE_SENSITIVE as libc::c_int as libc::c_ulong)
                            .wrapping_rem(
                                (::std::mem::size_of::<libc::c_uint>() as libc::c_ulong)
                                    .wrapping_mul(8 as libc::c_int as libc::c_ulong),
                            );
            } else if function == Some(backwards_void as unsafe extern "C" fn() -> ()) {
                flags[(BACKWARDS_SEARCH as libc::c_int as libc::c_ulong)
                    .wrapping_div(
                        (::std::mem::size_of::<libc::c_uint>() as libc::c_ulong)
                            .wrapping_mul(8 as libc::c_int as libc::c_ulong),
                    ) as usize]
                    ^= (1 as libc::c_int as libc::c_uint)
                        << (BACKWARDS_SEARCH as libc::c_int as libc::c_ulong)
                            .wrapping_rem(
                                (::std::mem::size_of::<libc::c_uint>() as libc::c_ulong)
                                    .wrapping_mul(8 as libc::c_int as libc::c_ulong),
                            );
            } else if function == Some(regexp_void as unsafe extern "C" fn() -> ()) {
                flags[(USE_REGEXP as libc::c_int as libc::c_ulong)
                    .wrapping_div(
                        (::std::mem::size_of::<libc::c_uint>() as libc::c_ulong)
                            .wrapping_mul(8 as libc::c_int as libc::c_ulong),
                    ) as usize]
                    ^= (1 as libc::c_int as libc::c_uint)
                        << (USE_REGEXP as libc::c_int as libc::c_ulong)
                            .wrapping_rem(
                                (::std::mem::size_of::<libc::c_uint>() as libc::c_ulong)
                                    .wrapping_mul(8 as libc::c_int as libc::c_ulong),
                            );
            } else if function == Some(flip_replace as unsafe extern "C" fn() -> ()) {
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
                    print_view_warning();
                    napms(600 as libc::c_int);
                } else {
                    replacing = !replacing;
                }
            } else {
                if !(function == Some(flip_goto as unsafe extern "C" fn() -> ())) {
                    break;
                }
                goto_line_and_column(
                    (*(*openfile).current).lineno,
                    ((*openfile).placewewant)
                        .wrapping_add(1 as libc::c_int as libc::c_ulong) as ssize_t,
                    1 as libc::c_int != 0,
                    1 as libc::c_int != 0,
                );
                break;
            }
        }
    }
    tidy_up_after_search();
    rpl_free(thedefault as *mut libc::c_void);
}
pub unsafe extern "C" fn findnextstr(
    mut needle: *const libc::c_char,
    mut whole_word_only: bool,
    mut modus: libc::c_int,
    mut match_len: *mut size_t,
    mut skipone: bool,
    mut begin: *const linestruct,
    mut begin_x: size_t,
) -> libc::c_int {
    let mut found_len: size_t = strlen(needle);
    let mut feedback: libc::c_int = 0 as libc::c_int;
    let mut line: *mut linestruct = (*openfile).current;
    let mut from: *const libc::c_char = ((*line).data)
        .offset((*openfile).current_x as isize);
    let mut found: *const libc::c_char = 0 as *const libc::c_char;
    let mut found_x: size_t = 0;
    let mut lastkbcheck: time_t = time(0 as *mut time_t);
    nodelay(midwin, 1 as libc::c_int != 0);
    if begin.is_null() {
        came_full_circle = 0 as libc::c_int != 0;
    }
    loop {
        if skipone {
            skipone = 0 as libc::c_int != 0;
            if flags[(BACKWARDS_SEARCH as libc::c_int as libc::c_ulong)
                .wrapping_div(
                    (::std::mem::size_of::<libc::c_uint>() as libc::c_ulong)
                        .wrapping_mul(8 as libc::c_int as libc::c_ulong),
                ) as usize]
                & (1 as libc::c_int as libc::c_uint)
                    << (BACKWARDS_SEARCH as libc::c_int as libc::c_ulong)
                        .wrapping_rem(
                            (::std::mem::size_of::<libc::c_uint>() as libc::c_ulong)
                                .wrapping_mul(8 as libc::c_int as libc::c_ulong),
                        ) != 0 as libc::c_int as libc::c_uint
                && from != (*line).data as *const libc::c_char
            {
                from = ((*line).data)
                    .offset(
                        step_left(
                            (*line).data,
                            from.offset_from((*line).data) as libc::c_long as size_t,
                        ) as isize,
                    );
                found = strstrwrapper((*line).data, needle, from);
            } else if !(flags[(BACKWARDS_SEARCH as libc::c_int as libc::c_ulong)
                .wrapping_div(
                    (::std::mem::size_of::<libc::c_uint>() as libc::c_ulong)
                        .wrapping_mul(8 as libc::c_int as libc::c_ulong),
                ) as usize]
                & (1 as libc::c_int as libc::c_uint)
                    << (BACKWARDS_SEARCH as libc::c_int as libc::c_ulong)
                        .wrapping_rem(
                            (::std::mem::size_of::<libc::c_uint>() as libc::c_ulong)
                                .wrapping_mul(8 as libc::c_int as libc::c_ulong),
                        ) != 0 as libc::c_int as libc::c_uint)
                && *from as libc::c_int != '\0' as i32
            {
                from = from.offset(char_length(from) as isize);
                found = strstrwrapper((*line).data, needle, from);
            }
        } else {
            found = strstrwrapper((*line).data, needle, from);
        }
        if !found.is_null() {
            if flags[(USE_REGEXP as libc::c_int as libc::c_ulong)
                .wrapping_div(
                    (::std::mem::size_of::<libc::c_uint>() as libc::c_ulong)
                        .wrapping_mul(8 as libc::c_int as libc::c_ulong),
                ) as usize]
                & (1 as libc::c_int as libc::c_uint)
                    << (USE_REGEXP as libc::c_int as libc::c_ulong)
                        .wrapping_rem(
                            (::std::mem::size_of::<libc::c_uint>() as libc::c_ulong)
                                .wrapping_mul(8 as libc::c_int as libc::c_ulong),
                        ) != 0 as libc::c_int as libc::c_uint
            {
                found_len = (regmatches[0 as libc::c_int as usize].rm_eo
                    - regmatches[0 as libc::c_int as usize].rm_so) as size_t;
            }
            if whole_word_only as libc::c_int != 0
                && !is_separate_word(
                    found.offset_from((*line).data) as libc::c_long as size_t,
                    found_len,
                    (*line).data,
                )
            {
                from = found.offset(char_length(found) as isize);
                continue;
            } else if !((*line).next).is_null()
                || *((*line).data).offset(0 as libc::c_int as isize) as libc::c_int != 0
            {
                break;
            }
        }
        if the_window_resized != 0 {
            regenerate_screen();
            nodelay(midwin, 1 as libc::c_int != 0);
            statusbar(
                dcgettext(
                    0 as *const libc::c_char,
                    b"Searching...\0" as *const u8 as *const libc::c_char,
                    5 as libc::c_int,
                ),
            );
            feedback = 1 as libc::c_int;
        }
        if came_full_circle {
            nodelay(midwin, 0 as libc::c_int != 0);
            return 0 as libc::c_int;
        }
        line = if flags[(BACKWARDS_SEARCH as libc::c_int as libc::c_ulong)
            .wrapping_div(
                (::std::mem::size_of::<libc::c_uint>() as libc::c_ulong)
                    .wrapping_mul(8 as libc::c_int as libc::c_ulong),
            ) as usize]
            & (1 as libc::c_int as libc::c_uint)
                << (BACKWARDS_SEARCH as libc::c_int as libc::c_ulong)
                    .wrapping_rem(
                        (::std::mem::size_of::<libc::c_uint>() as libc::c_ulong)
                            .wrapping_mul(8 as libc::c_int as libc::c_ulong),
                    ) != 0 as libc::c_int as libc::c_uint
        {
            (*line).prev
        } else {
            (*line).next
        };
        if line.is_null() {
            if whole_word_only as libc::c_int != 0 || modus == 2 as libc::c_int {
                nodelay(midwin, 0 as libc::c_int != 0);
                return 0 as libc::c_int;
            }
            line = if flags[(BACKWARDS_SEARCH as libc::c_int as libc::c_ulong)
                .wrapping_div(
                    (::std::mem::size_of::<libc::c_uint>() as libc::c_ulong)
                        .wrapping_mul(8 as libc::c_int as libc::c_ulong),
                ) as usize]
                & (1 as libc::c_int as libc::c_uint)
                    << (BACKWARDS_SEARCH as libc::c_int as libc::c_ulong)
                        .wrapping_rem(
                            (::std::mem::size_of::<libc::c_uint>() as libc::c_ulong)
                                .wrapping_mul(8 as libc::c_int as libc::c_ulong),
                        ) != 0 as libc::c_int as libc::c_uint
            {
                (*openfile).filebot
            } else {
                (*openfile).filetop
            };
            if modus == 0 as libc::c_int {
                statusline(
                    REMARK,
                    dcgettext(
                        0 as *const libc::c_char,
                        b"Search Wrapped\0" as *const u8 as *const libc::c_char,
                        5 as libc::c_int,
                    ),
                );
                feedback = -(2 as libc::c_int);
            }
        }
        if line == begin as *mut linestruct {
            came_full_circle = 1 as libc::c_int != 0;
        }
        from = (*line).data;
        if flags[(BACKWARDS_SEARCH as libc::c_int as libc::c_ulong)
            .wrapping_div(
                (::std::mem::size_of::<libc::c_uint>() as libc::c_ulong)
                    .wrapping_mul(8 as libc::c_int as libc::c_ulong),
            ) as usize]
            & (1 as libc::c_int as libc::c_uint)
                << (BACKWARDS_SEARCH as libc::c_int as libc::c_ulong)
                    .wrapping_rem(
                        (::std::mem::size_of::<libc::c_uint>() as libc::c_ulong)
                            .wrapping_mul(8 as libc::c_int as libc::c_ulong),
                    ) != 0 as libc::c_int as libc::c_uint
        {
            from = from.offset(strlen((*line).data) as isize);
        }
        if time(0 as *mut time_t) - lastkbcheck > 0 as libc::c_int as libc::c_long {
            let mut input: libc::c_int = wgetch(midwin);
            lastkbcheck = time(0 as *mut time_t);
            while input != -(1 as libc::c_int) {
                if input == 0x1b as libc::c_int {
                    napms(20 as libc::c_int);
                    input = wgetch(midwin);
                    meta_key = 1 as libc::c_int != 0;
                } else {
                    meta_key = 0 as libc::c_int != 0;
                }
                if func_from_key(input)
                    == Some(do_cancel as unsafe extern "C" fn() -> ())
                {
                    if the_window_resized != 0 {
                        regenerate_screen();
                    }
                    statusbar(
                        dcgettext(
                            0 as *const libc::c_char,
                            b"Cancelled\0" as *const u8 as *const libc::c_char,
                            5 as libc::c_int,
                        ),
                    );
                    while input != -(1 as libc::c_int) {
                        input = get_input(0 as *mut WINDOW);
                    }
                    nodelay(midwin, 0 as libc::c_int != 0);
                    return -(2 as libc::c_int);
                }
                input = wgetch(midwin);
            }
            feedback += 1;
            if feedback > 0 as libc::c_int {
                statusbar(
                    dcgettext(
                        0 as *const libc::c_char,
                        b"Searching...\0" as *const u8 as *const libc::c_char,
                        5 as libc::c_int,
                    ),
                );
            }
        }
    }
    found_x = found.offset_from((*line).data) as libc::c_long as size_t;
    nodelay(midwin, 0 as libc::c_int != 0);
    if came_full_circle as libc::c_int != 0
        && (!(flags[(BACKWARDS_SEARCH as libc::c_int as libc::c_ulong)
            .wrapping_div(
                (::std::mem::size_of::<libc::c_uint>() as libc::c_ulong)
                    .wrapping_mul(8 as libc::c_int as libc::c_ulong),
            ) as usize]
            & (1 as libc::c_int as libc::c_uint)
                << (BACKWARDS_SEARCH as libc::c_int as libc::c_ulong)
                    .wrapping_rem(
                        (::std::mem::size_of::<libc::c_uint>() as libc::c_ulong)
                            .wrapping_mul(8 as libc::c_int as libc::c_ulong),
                    ) != 0 as libc::c_int as libc::c_uint)
            && (found_x > begin_x || modus == 1 as libc::c_int && found_x == begin_x)
            || flags[(BACKWARDS_SEARCH as libc::c_int as libc::c_ulong)
                .wrapping_div(
                    (::std::mem::size_of::<libc::c_uint>() as libc::c_ulong)
                        .wrapping_mul(8 as libc::c_int as libc::c_ulong),
                ) as usize]
                & (1 as libc::c_int as libc::c_uint)
                    << (BACKWARDS_SEARCH as libc::c_int as libc::c_ulong)
                        .wrapping_rem(
                            (::std::mem::size_of::<libc::c_uint>() as libc::c_ulong)
                                .wrapping_mul(8 as libc::c_int as libc::c_ulong),
                        ) != 0 as libc::c_int as libc::c_uint && found_x < begin_x)
    {
        return 0 as libc::c_int;
    }
    (*openfile).current = line;
    (*openfile).current_x = found_x;
    if !match_len.is_null() {
        *match_len = found_len;
    }
    if modus == 0 as libc::c_int
        && (((*openfile).mark).is_null() || (*openfile).softmark as libc::c_int != 0)
    {
        spotlighted = 1 as libc::c_int != 0;
        light_from_col = xplustabs();
        light_to_col = wideness((*line).data, found_x.wrapping_add(found_len));
        refresh_needed = 1 as libc::c_int != 0;
    }
    if feedback > 0 as libc::c_int {
        wipe_statusbar();
    }
    return 1 as libc::c_int;
}
pub unsafe extern "C" fn do_search_forward() {
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
    search_init(0 as libc::c_int != 0, 0 as libc::c_int != 0);
}
pub unsafe extern "C" fn do_search_backward() {
    flags[(BACKWARDS_SEARCH as libc::c_int as libc::c_ulong)
        .wrapping_div(
            (::std::mem::size_of::<libc::c_uint>() as libc::c_ulong)
                .wrapping_mul(8 as libc::c_int as libc::c_ulong),
        ) as usize]
        |= (1 as libc::c_int as libc::c_uint)
            << (BACKWARDS_SEARCH as libc::c_int as libc::c_ulong)
                .wrapping_rem(
                    (::std::mem::size_of::<libc::c_uint>() as libc::c_ulong)
                        .wrapping_mul(8 as libc::c_int as libc::c_ulong),
                );
    search_init(0 as libc::c_int != 0, 0 as libc::c_int != 0);
}
pub unsafe extern "C" fn do_research() {
    if *last_search as libc::c_int == '\0' as i32 && !((*searchbot).prev).is_null() {
        last_search = mallocstrcpy(last_search, (*(*searchbot).prev).data);
    }
    if *last_search as libc::c_int == '\0' as i32 {
        statusline(
            AHEM,
            dcgettext(
                0 as *const libc::c_char,
                b"No current search pattern\0" as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ),
        );
        return;
    }
    if flags[(USE_REGEXP as libc::c_int as libc::c_ulong)
        .wrapping_div(
            (::std::mem::size_of::<libc::c_uint>() as libc::c_ulong)
                .wrapping_mul(8 as libc::c_int as libc::c_ulong),
        ) as usize]
        & (1 as libc::c_int as libc::c_uint)
            << (USE_REGEXP as libc::c_int as libc::c_ulong)
                .wrapping_rem(
                    (::std::mem::size_of::<libc::c_uint>() as libc::c_ulong)
                        .wrapping_mul(8 as libc::c_int as libc::c_ulong),
                ) != 0 as libc::c_int as libc::c_uint && !regexp_init(last_search)
    {
        return;
    }
    currmenu = (1 as libc::c_int) << 1 as libc::c_int;
    if LINES > 1 as libc::c_int {
        wipe_statusbar();
    }
    go_looking();
    tidy_up_after_search();
}
pub unsafe extern "C" fn do_findprevious() {
    flags[(BACKWARDS_SEARCH as libc::c_int as libc::c_ulong)
        .wrapping_div(
            (::std::mem::size_of::<libc::c_uint>() as libc::c_ulong)
                .wrapping_mul(8 as libc::c_int as libc::c_ulong),
        ) as usize]
        |= (1 as libc::c_int as libc::c_uint)
            << (BACKWARDS_SEARCH as libc::c_int as libc::c_ulong)
                .wrapping_rem(
                    (::std::mem::size_of::<libc::c_uint>() as libc::c_ulong)
                        .wrapping_mul(8 as libc::c_int as libc::c_ulong),
                );
    do_research();
}
pub unsafe extern "C" fn do_findnext() {
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
    do_research();
}
pub unsafe extern "C" fn not_found_msg(mut str: *const libc::c_char) {
    let mut disp: *mut libc::c_char = display_string(
        str,
        0 as libc::c_int as size_t,
        (COLS / 2 as libc::c_int + 1 as libc::c_int) as size_t,
        0 as libc::c_int != 0,
        0 as libc::c_int != 0,
    );
    let mut numchars: size_t = actual_x(
        disp,
        wideness(disp, (COLS / 2 as libc::c_int) as size_t),
    );
    statusline(
        AHEM,
        dcgettext(
            0 as *const libc::c_char,
            b"\"%.*s%s\" not found\0" as *const u8 as *const libc::c_char,
            5 as libc::c_int,
        ),
        numchars,
        disp,
        if *disp.offset(numchars as isize) as libc::c_int == '\0' as i32 {
            b"\0" as *const u8 as *const libc::c_char
        } else {
            b"...\0" as *const u8 as *const libc::c_char
        },
    );
    rpl_free(disp as *mut libc::c_void);
}
pub unsafe extern "C" fn go_looking() {
    let mut was_current: *mut linestruct = (*openfile).current;
    let mut was_current_x: size_t = (*openfile).current_x;
    came_full_circle = 0 as libc::c_int != 0;
    didfind = findnextstr(
        last_search,
        0 as libc::c_int != 0,
        0 as libc::c_int,
        0 as *mut size_t,
        1 as libc::c_int != 0,
        (*openfile).current,
        (*openfile).current_x,
    );
    if didfind == 1 as libc::c_int && (*openfile).current == was_current
        && (*openfile).current_x == was_current_x
    {
        statusline(
            REMARK,
            dcgettext(
                0 as *const libc::c_char,
                b"This is the only occurrence\0" as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ),
        );
    } else if didfind == 0 as libc::c_int {
        not_found_msg(last_search);
    }
    edit_redraw(was_current, CENTERING);
}
pub unsafe extern "C" fn replace_regexp(
    mut string: *mut libc::c_char,
    mut create: bool,
) -> libc::c_int {
    let mut replacement_size: size_t = 0 as libc::c_int as size_t;
    let mut c: *const libc::c_char = answer;
    while *c as libc::c_int != '\0' as i32 {
        let mut num: libc::c_int = *c.offset(1 as libc::c_int as isize) as libc::c_int
            - '0' as i32;
        if *c as libc::c_int != '\\' as i32 || num < 1 as libc::c_int
            || num > 9 as libc::c_int || num as libc::c_ulong > search_regexp.re_nsub
        {
            if create {
                let fresh0 = string;
                string = string.offset(1);
                *fresh0 = *c;
            }
            c = c.offset(1);
            c;
            replacement_size = replacement_size.wrapping_add(1);
            replacement_size;
        } else {
            let mut i: size_t = (regmatches[num as usize].rm_eo
                - regmatches[num as usize].rm_so) as size_t;
            c = c.offset(2 as libc::c_int as isize);
            replacement_size = (replacement_size as libc::c_ulong).wrapping_add(i)
                as size_t as size_t;
            if create {
                strncpy(
                    string,
                    ((*(*openfile).current).data)
                        .offset(regmatches[num as usize].rm_so as isize),
                    i,
                );
                string = string.offset(i as isize);
            }
        }
    }
    if create {
        *string = '\0' as i32 as libc::c_char;
    }
    return replacement_size as libc::c_int;
}
pub unsafe extern "C" fn replace_line(
    mut needle: *const libc::c_char,
) -> *mut libc::c_char {
    let mut new_size: size_t = (strlen((*(*openfile).current).data))
        .wrapping_add(1 as libc::c_int as libc::c_ulong);
    let mut match_len: size_t = 0;
    let mut copy: *mut libc::c_char = 0 as *mut libc::c_char;
    if flags[(USE_REGEXP as libc::c_int as libc::c_ulong)
        .wrapping_div(
            (::std::mem::size_of::<libc::c_uint>() as libc::c_ulong)
                .wrapping_mul(8 as libc::c_int as libc::c_ulong),
        ) as usize]
        & (1 as libc::c_int as libc::c_uint)
            << (USE_REGEXP as libc::c_int as libc::c_ulong)
                .wrapping_rem(
                    (::std::mem::size_of::<libc::c_uint>() as libc::c_ulong)
                        .wrapping_mul(8 as libc::c_int as libc::c_ulong),
                ) != 0 as libc::c_int as libc::c_uint
    {
        match_len = (regmatches[0 as libc::c_int as usize].rm_eo
            - regmatches[0 as libc::c_int as usize].rm_so) as size_t;
        new_size = (new_size as libc::c_ulong)
            .wrapping_add(
                (replace_regexp(0 as *mut libc::c_char, 0 as libc::c_int != 0)
                    as libc::c_ulong)
                    .wrapping_sub(match_len),
            ) as size_t as size_t;
    } else {
        match_len = strlen(needle);
        new_size = (new_size as libc::c_ulong)
            .wrapping_add((strlen(answer)).wrapping_sub(match_len)) as size_t as size_t;
    }
    copy = nmalloc(new_size) as *mut libc::c_char;
    strncpy(copy, (*(*openfile).current).data, (*openfile).current_x);
    if flags[(USE_REGEXP as libc::c_int as libc::c_ulong)
        .wrapping_div(
            (::std::mem::size_of::<libc::c_uint>() as libc::c_ulong)
                .wrapping_mul(8 as libc::c_int as libc::c_ulong),
        ) as usize]
        & (1 as libc::c_int as libc::c_uint)
            << (USE_REGEXP as libc::c_int as libc::c_ulong)
                .wrapping_rem(
                    (::std::mem::size_of::<libc::c_uint>() as libc::c_ulong)
                        .wrapping_mul(8 as libc::c_int as libc::c_ulong),
                ) != 0 as libc::c_int as libc::c_uint
    {
        replace_regexp(
            copy.offset((*openfile).current_x as isize),
            1 as libc::c_int != 0,
        );
    } else {
        strcpy(copy.offset((*openfile).current_x as isize), answer);
    }
    strcat(
        copy,
        ((*(*openfile).current).data)
            .offset((*openfile).current_x as isize)
            .offset(match_len as isize),
    );
    return copy;
}
pub unsafe extern "C" fn do_replace_loop(
    mut needle: *const libc::c_char,
    mut whole_word_only: bool,
    mut real_current: *const linestruct,
    mut real_current_x: *mut size_t,
) -> ssize_t {
    let mut skipone: bool = flags[(BACKWARDS_SEARCH as libc::c_int as libc::c_ulong)
        .wrapping_div(
            (::std::mem::size_of::<libc::c_uint>() as libc::c_ulong)
                .wrapping_mul(8 as libc::c_int as libc::c_ulong),
        ) as usize]
        & (1 as libc::c_int as libc::c_uint)
            << (BACKWARDS_SEARCH as libc::c_int as libc::c_ulong)
                .wrapping_rem(
                    (::std::mem::size_of::<libc::c_uint>() as libc::c_ulong)
                        .wrapping_mul(8 as libc::c_int as libc::c_ulong),
                ) != 0 as libc::c_int as libc::c_uint;
    let mut replaceall: bool = 0 as libc::c_int != 0;
    let mut modus: libc::c_int = 1 as libc::c_int;
    let mut numreplaced: ssize_t = -(1 as libc::c_int) as ssize_t;
    let mut match_len: size_t = 0;
    let mut was_mark: *mut linestruct = (*openfile).mark;
    let mut top: *mut linestruct = 0 as *mut linestruct;
    let mut bot: *mut linestruct = 0 as *mut linestruct;
    let mut top_x: size_t = 0;
    let mut bot_x: size_t = 0;
    let mut right_side_up: bool = !((*openfile).mark).is_null()
        && mark_is_before_cursor() as libc::c_int != 0;
    if !((*openfile).mark).is_null() {
        get_region(&mut top, &mut top_x, &mut bot, &mut bot_x);
        (*openfile).mark = 0 as *mut linestruct;
        modus = 2 as libc::c_int;
        if !(flags[(BACKWARDS_SEARCH as libc::c_int as libc::c_ulong)
            .wrapping_div(
                (::std::mem::size_of::<libc::c_uint>() as libc::c_ulong)
                    .wrapping_mul(8 as libc::c_int as libc::c_ulong),
            ) as usize]
            & (1 as libc::c_int as libc::c_uint)
                << (BACKWARDS_SEARCH as libc::c_int as libc::c_ulong)
                    .wrapping_rem(
                        (::std::mem::size_of::<libc::c_uint>() as libc::c_ulong)
                            .wrapping_mul(8 as libc::c_int as libc::c_ulong),
                    ) != 0 as libc::c_int as libc::c_uint)
        {
            (*openfile).current = top;
            (*openfile).current_x = top_x;
        } else {
            (*openfile).current = bot;
            (*openfile).current_x = bot_x;
        }
    }
    came_full_circle = 0 as libc::c_int != 0;
    loop {
        let mut choice: libc::c_int = 0 as libc::c_int;
        let mut result: libc::c_int = findnextstr(
            needle,
            whole_word_only,
            modus,
            &mut match_len,
            skipone,
            real_current,
            *real_current_x,
        );
        if result < 1 as libc::c_int {
            if result < 0 as libc::c_int {
                numreplaced = -(2 as libc::c_int) as ssize_t;
            }
            break;
        } else {
            if !was_mark.is_null()
                && ((*(*openfile).current).lineno > (*bot).lineno
                    || (*(*openfile).current).lineno < (*top).lineno
                    || (*openfile).current == bot
                        && ((*openfile).current_x).wrapping_add(match_len) > bot_x
                    || (*openfile).current == top && (*openfile).current_x < top_x)
            {
                break;
            }
            if numreplaced == -(1 as libc::c_int) as libc::c_long {
                numreplaced = 0 as libc::c_int as ssize_t;
            }
            if !replaceall {
                spotlighted = 1 as libc::c_int != 0;
                light_from_col = xplustabs();
                light_to_col = wideness(
                    (*(*openfile).current).data,
                    ((*openfile).current_x).wrapping_add(match_len),
                );
                edit_refresh();
                choice = ask_user(
                    1 as libc::c_int != 0,
                    dcgettext(
                        0 as *const libc::c_char,
                        b"Replace this instance?\0" as *const u8 as *const libc::c_char,
                        5 as libc::c_int,
                    ),
                );
                spotlighted = 0 as libc::c_int != 0;
                if choice == -(1 as libc::c_int) {
                    break;
                }
                replaceall = choice == 2 as libc::c_int;
                skipone = choice == 0 as libc::c_int
                    || flags[(BACKWARDS_SEARCH as libc::c_int as libc::c_ulong)
                        .wrapping_div(
                            (::std::mem::size_of::<libc::c_uint>() as libc::c_ulong)
                                .wrapping_mul(8 as libc::c_int as libc::c_ulong),
                        ) as usize]
                        & (1 as libc::c_int as libc::c_uint)
                            << (BACKWARDS_SEARCH as libc::c_int as libc::c_ulong)
                                .wrapping_rem(
                                    (::std::mem::size_of::<libc::c_uint>() as libc::c_ulong)
                                        .wrapping_mul(8 as libc::c_int as libc::c_ulong),
                                ) != 0 as libc::c_int as libc::c_uint;
            }
            if choice == 1 as libc::c_int || replaceall as libc::c_int != 0 {
                let mut length_change: size_t = 0;
                let mut altered: *mut libc::c_char = 0 as *mut libc::c_char;
                altered = replace_line(needle);
                length_change = (strlen(altered))
                    .wrapping_sub(strlen((*(*openfile).current).data));
                add_undo(REPLACE, 0 as *const libc::c_char);
                if !was_mark.is_null() && !right_side_up {
                    if (*openfile).current == was_mark
                        && (*openfile).mark_x > (*openfile).current_x
                    {
                        if (*openfile).mark_x
                            < ((*openfile).current_x).wrapping_add(match_len)
                        {
                            (*openfile).mark_x = (*openfile).current_x;
                        } else {
                            (*openfile)
                                .mark_x = ((*openfile).mark_x as libc::c_ulong)
                                .wrapping_add(length_change) as size_t as size_t;
                        }
                        bot_x = (*openfile).mark_x;
                    }
                }
                if was_mark.is_null() || right_side_up as libc::c_int != 0 {
                    if (*openfile).current == real_current as *mut linestruct
                        && (*openfile).current_x < *real_current_x
                    {
                        if *real_current_x
                            < ((*openfile).current_x).wrapping_add(match_len)
                        {
                            *real_current_x = ((*openfile).current_x)
                                .wrapping_add(match_len);
                        }
                        *real_current_x = (*real_current_x as libc::c_ulong)
                            .wrapping_add(length_change) as size_t as size_t;
                        bot_x = *real_current_x;
                    }
                }
                if match_len == 0 as libc::c_int as libc::c_ulong
                    || *needle as libc::c_int == '^' as i32
                        && flags[(USE_REGEXP as libc::c_int as libc::c_ulong)
                            .wrapping_div(
                                (::std::mem::size_of::<libc::c_uint>() as libc::c_ulong)
                                    .wrapping_mul(8 as libc::c_int as libc::c_ulong),
                            ) as usize]
                            & (1 as libc::c_int as libc::c_uint)
                                << (USE_REGEXP as libc::c_int as libc::c_ulong)
                                    .wrapping_rem(
                                        (::std::mem::size_of::<libc::c_uint>() as libc::c_ulong)
                                            .wrapping_mul(8 as libc::c_int as libc::c_ulong),
                                    ) != 0 as libc::c_int as libc::c_uint
                {
                    skipone = 1 as libc::c_int != 0;
                }
                if !(flags[(BACKWARDS_SEARCH as libc::c_int as libc::c_ulong)
                    .wrapping_div(
                        (::std::mem::size_of::<libc::c_uint>() as libc::c_ulong)
                            .wrapping_mul(8 as libc::c_int as libc::c_ulong),
                    ) as usize]
                    & (1 as libc::c_int as libc::c_uint)
                        << (BACKWARDS_SEARCH as libc::c_int as libc::c_ulong)
                            .wrapping_rem(
                                (::std::mem::size_of::<libc::c_uint>() as libc::c_ulong)
                                    .wrapping_mul(8 as libc::c_int as libc::c_ulong),
                            ) != 0 as libc::c_int as libc::c_uint)
                {
                    (*openfile)
                        .current_x = ((*openfile).current_x as libc::c_ulong)
                        .wrapping_add(match_len.wrapping_add(length_change)) as size_t
                        as size_t;
                }
                (*openfile)
                    .totsize = ((*openfile).totsize as libc::c_ulong)
                    .wrapping_add(
                        (mbstrlen(altered))
                            .wrapping_sub(mbstrlen((*(*openfile).current).data)),
                    ) as size_t as size_t;
                rpl_free((*(*openfile).current).data as *mut libc::c_void);
                (*(*openfile).current).data = altered;
                check_the_multis((*openfile).current);
                refresh_needed = 0 as libc::c_int != 0;
                set_modified();
                as_an_at = 1 as libc::c_int != 0;
                numreplaced += 1;
                numreplaced;
            }
        }
    }
    if numreplaced == -(1 as libc::c_int) as libc::c_long {
        not_found_msg(needle);
    }
    (*openfile).mark = was_mark;
    if !(flags[(NO_NEWLINES as libc::c_int as libc::c_ulong)
        .wrapping_div(
            (::std::mem::size_of::<libc::c_uint>() as libc::c_ulong)
                .wrapping_mul(8 as libc::c_int as libc::c_ulong),
        ) as usize]
        & (1 as libc::c_int as libc::c_uint)
            << (NO_NEWLINES as libc::c_int as libc::c_ulong)
                .wrapping_rem(
                    (::std::mem::size_of::<libc::c_uint>() as libc::c_ulong)
                        .wrapping_mul(8 as libc::c_int as libc::c_ulong),
                ) != 0 as libc::c_int as libc::c_uint)
        && *((*(*openfile).filebot).data).offset(0 as libc::c_int as isize)
            as libc::c_int != '\0' as i32
    {
        new_magicline();
    }
    return numreplaced;
}
pub unsafe extern "C" fn do_replace() {
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
        print_view_warning();
    } else {
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
        search_init(1 as libc::c_int != 0, 0 as libc::c_int != 0);
    };
}
pub unsafe extern "C" fn ask_for_and_do_replacements() {
    let mut was_edittop: *mut linestruct = (*openfile).edittop;
    let mut was_firstcolumn: size_t = (*openfile).firstcolumn;
    let mut beginline: *mut linestruct = (*openfile).current;
    let mut begin_x: size_t = (*openfile).current_x;
    let mut numreplaced: ssize_t = 0;
    let mut response: libc::c_int = do_prompt(
        (1 as libc::c_int) << 3 as libc::c_int,
        b"\0" as *const u8 as *const libc::c_char,
        &mut replace_history as *mut *mut linestruct,
        Some(edit_refresh as unsafe extern "C" fn() -> ()),
        dcgettext(
            0 as *const libc::c_char,
            b"Replace with\0" as *const u8 as *const libc::c_char,
            5 as libc::c_int,
        ),
    );
    if response == 0 as libc::c_int {
        update_history(&mut replace_history, answer, 1 as libc::c_int != 0);
    }
    if response == -(1 as libc::c_int) {
        statusbar(
            dcgettext(
                0 as *const libc::c_char,
                b"Cancelled\0" as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ),
        );
        return;
    } else if response > 0 as libc::c_int {
        return
    }
    numreplaced = do_replace_loop(
        last_search,
        0 as libc::c_int != 0,
        beginline,
        &mut begin_x,
    );
    (*openfile).edittop = was_edittop;
    (*openfile).firstcolumn = was_firstcolumn;
    (*openfile).current = beginline;
    (*openfile).current_x = begin_x;
    refresh_needed = 1 as libc::c_int != 0;
    if numreplaced >= 0 as libc::c_int as libc::c_long {
        statusline(
            REMARK,
            dcngettext(
                0 as *const libc::c_char,
                b"Replaced %zd occurrence\0" as *const u8 as *const libc::c_char,
                b"Replaced %zd occurrences\0" as *const u8 as *const libc::c_char,
                numreplaced as libc::c_ulong,
                5 as libc::c_int,
            ),
            numreplaced,
        );
    }
}
pub unsafe extern "C" fn goto_line_posx(mut linenumber: ssize_t, mut pos_x: size_t) {
    if linenumber > (*(*openfile).edittop).lineno + editwinrows as libc::c_long
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
            && linenumber > (*(*openfile).current).lineno
    {
        recook = (recook as libc::c_int | perturbed as libc::c_int) != 0;
    }
    if linenumber < (*(*openfile).filebot).lineno {
        (*openfile).current = line_from_number(linenumber);
    } else {
        (*openfile).current = (*openfile).filebot;
    }
    (*openfile).current_x = pos_x;
    (*openfile).placewewant = xplustabs();
    refresh_needed = 1 as libc::c_int != 0;
}
pub unsafe extern "C" fn goto_line_and_column(
    mut line: ssize_t,
    mut column: ssize_t,
    mut retain_answer: bool,
    mut interactive: bool,
) {
    if interactive {
        let mut response: libc::c_int = do_prompt(
            (1 as libc::c_int) << 4 as libc::c_int,
            if retain_answer as libc::c_int != 0 {
                answer as *const libc::c_char
            } else {
                b"\0" as *const u8 as *const libc::c_char
            },
            0 as *mut *mut linestruct,
            Some(edit_refresh as unsafe extern "C" fn() -> ()),
            dcgettext(
                0 as *const libc::c_char,
                b"Enter line number, column number\0" as *const u8
                    as *const libc::c_char,
                5 as libc::c_int,
            ),
        );
        if response < 0 as libc::c_int {
            statusbar(
                dcgettext(
                    0 as *const libc::c_char,
                    b"Cancelled\0" as *const u8 as *const libc::c_char,
                    5 as libc::c_int,
                ),
            );
            return;
        }
        if func_from_key(response) == Some(flip_goto as unsafe extern "C" fn() -> ()) {
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
            search_init(0 as libc::c_int != 0, 1 as libc::c_int != 0);
            return;
        }
        if response > 0 as libc::c_int {
            return;
        }
        if !parse_line_column(answer, &mut line, &mut column) {
            statusline(
                AHEM,
                dcgettext(
                    0 as *const libc::c_char,
                    b"Invalid line or column number\0" as *const u8
                        as *const libc::c_char,
                    5 as libc::c_int,
                ),
            );
            return;
        }
    } else {
        if line == 0 as libc::c_int as libc::c_long {
            line = (*(*openfile).current).lineno;
        }
        if column == 0 as libc::c_int as libc::c_long {
            column = ((*openfile).placewewant)
                .wrapping_add(1 as libc::c_int as libc::c_ulong) as ssize_t;
        }
    }
    if line < 0 as libc::c_int as libc::c_long {
        line = (*(*openfile).filebot).lineno + line + 1 as libc::c_int as libc::c_long;
    }
    if line < 1 as libc::c_int as libc::c_long {
        line = 1 as libc::c_int as ssize_t;
    }
    if line > (*(*openfile).edittop).lineno + editwinrows as libc::c_long
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
            && line > (*(*openfile).current).lineno
    {
        recook = (recook as libc::c_int | perturbed as libc::c_int) != 0;
    }
    (*openfile).current = (*openfile).filetop;
    while line > 1 as libc::c_int as libc::c_long
        && (*openfile).current != (*openfile).filebot
    {
        (*openfile).current = (*(*openfile).current).next;
        line -= 1;
        line;
    }
    if column < 0 as libc::c_int as libc::c_long {
        column = (breadth((*(*openfile).current).data))
            .wrapping_add(column as libc::c_ulong)
            .wrapping_add(2 as libc::c_int as libc::c_ulong) as ssize_t;
    }
    if column < 1 as libc::c_int as libc::c_long {
        column = 1 as libc::c_int as ssize_t;
    }
    (*openfile)
        .current_x = actual_x(
        (*(*openfile).current).data,
        (column - 1 as libc::c_int as libc::c_long) as size_t,
    );
    (*openfile).placewewant = (column - 1 as libc::c_int as libc::c_long) as size_t;
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
        && ((*openfile).placewewant).wrapping_div(editwincols as libc::c_ulong)
            > (breadth((*(*openfile).current).data))
                .wrapping_div(editwincols as libc::c_ulong)
    {
        (*openfile).placewewant = breadth((*(*openfile).current).data);
    }
    if interactive {
        adjust_viewport(
            (if *answer as libc::c_int == ',' as i32 {
                STATIONARY as libc::c_int
            } else {
                CENTERING as libc::c_int
            }) as update_type,
        );
        refresh_needed = 1 as libc::c_int != 0;
    } else {
        let mut rows_from_tail: libc::c_int = 0;
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
            let mut currentline: *mut linestruct = (*openfile).current;
            let mut leftedge: size_t = leftedge_for(xplustabs(), (*openfile).current);
            rows_from_tail = editwinrows / 2 as libc::c_int
                - go_forward_chunks(
                    editwinrows / 2 as libc::c_int,
                    &mut currentline,
                    &mut leftedge,
                );
        } else {
            rows_from_tail = ((*(*openfile).filebot).lineno
                - (*(*openfile).current).lineno) as libc::c_int;
        }
        if rows_from_tail < editwinrows / 2 as libc::c_int
            && !(flags[(JUMPY_SCROLLING as libc::c_int as libc::c_ulong)
                .wrapping_div(
                    (::std::mem::size_of::<libc::c_uint>() as libc::c_ulong)
                        .wrapping_mul(8 as libc::c_int as libc::c_ulong),
                ) as usize]
                & (1 as libc::c_int as libc::c_uint)
                    << (JUMPY_SCROLLING as libc::c_int as libc::c_ulong)
                        .wrapping_rem(
                            (::std::mem::size_of::<libc::c_uint>() as libc::c_ulong)
                                .wrapping_mul(8 as libc::c_int as libc::c_ulong),
                        ) != 0 as libc::c_int as libc::c_uint)
        {
            (*openfile)
                .current_y = (editwinrows - 1 as libc::c_int - rows_from_tail)
                as ssize_t;
            adjust_viewport(STATIONARY);
        } else {
            adjust_viewport(CENTERING);
        }
    };
}
pub unsafe extern "C" fn do_gotolinecolumn() {
    goto_line_and_column(
        (*(*openfile).current).lineno,
        ((*openfile).placewewant).wrapping_add(1 as libc::c_int as libc::c_ulong)
            as ssize_t,
        0 as libc::c_int != 0,
        1 as libc::c_int != 0,
    );
}
pub unsafe extern "C" fn find_a_bracket(
    mut reverse: bool,
    mut bracket_pair: *const libc::c_char,
) -> bool {
    let mut line: *mut linestruct = (*openfile).current;
    let mut pointer: *const libc::c_char = 0 as *const libc::c_char;
    let mut found: *const libc::c_char = 0 as *const libc::c_char;
    if reverse {
        if (*openfile).current_x == 0 as libc::c_int as libc::c_ulong {
            line = (*line).prev;
            if line.is_null() {
                return 0 as libc::c_int != 0;
            }
            pointer = ((*line).data).offset(strlen((*line).data) as isize);
        } else {
            pointer = ((*line).data)
                .offset(step_left((*line).data, (*openfile).current_x) as isize);
        }
        loop {
            found = mbrevstrpbrk((*line).data, bracket_pair, pointer);
            if !found.is_null() {
                break;
            }
            line = (*line).prev;
            if line.is_null() {
                return 0 as libc::c_int != 0;
            }
            pointer = ((*line).data).offset(strlen((*line).data) as isize);
        }
    } else {
        pointer = ((*line).data)
            .offset(step_right((*line).data, (*openfile).current_x) as isize);
        loop {
            found = mbstrpbrk(pointer, bracket_pair);
            if !found.is_null() {
                break;
            }
            line = (*line).next;
            if line.is_null() {
                return 0 as libc::c_int != 0;
            }
            pointer = (*line).data;
        }
    }
    (*openfile).current = line;
    (*openfile).current_x = found.offset_from((*line).data) as libc::c_long as size_t;
    return 1 as libc::c_int != 0;
}
pub unsafe extern "C" fn do_find_bracket() {
    let mut was_current: *mut linestruct = (*openfile).current;
    let mut was_current_x: size_t = (*openfile).current_x;
    let mut ch: *const libc::c_char = 0 as *const libc::c_char;
    let mut ch_len: libc::c_int = 0;
    let mut wanted_ch: *const libc::c_char = 0 as *const libc::c_char;
    let mut wanted_ch_len: libc::c_int = 0;
    let mut bracket_pair: [libc::c_char; 9] = [0; 9];
    let mut halfway: size_t = 0 as libc::c_int as size_t;
    let mut charcount: size_t = (mbstrlen(matchbrackets))
        .wrapping_div(2 as libc::c_int as libc::c_ulong);
    let mut balance: size_t = 1 as libc::c_int as size_t;
    let mut reverse: bool = false;
    ch = mbstrchr(
        matchbrackets,
        ((*(*openfile).current).data).offset((*openfile).current_x as isize),
    );
    if ch.is_null() {
        statusline(
            AHEM,
            dcgettext(
                0 as *const libc::c_char,
                b"Not a bracket\0" as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ),
        );
        return;
    }
    let mut i: size_t = 0 as libc::c_int as size_t;
    while i < charcount {
        halfway = (halfway as libc::c_ulong)
            .wrapping_add(
                char_length(matchbrackets.offset(halfway as isize)) as libc::c_ulong,
            ) as size_t as size_t;
        i = i.wrapping_add(1);
        i;
    }
    reverse = ch >= matchbrackets.offset(halfway as isize) as *const libc::c_char;
    wanted_ch = ch;
    loop {
        let fresh1 = charcount;
        charcount = charcount.wrapping_sub(1);
        if !(fresh1 > 0 as libc::c_int as libc::c_ulong) {
            break;
        }
        if reverse {
            wanted_ch = matchbrackets
                .offset(
                    step_left(
                        matchbrackets,
                        wanted_ch.offset_from(matchbrackets) as libc::c_long as size_t,
                    ) as isize,
                );
        } else {
            wanted_ch = wanted_ch.offset(char_length(wanted_ch) as isize);
        }
    }
    ch_len = char_length(ch);
    wanted_ch_len = char_length(wanted_ch);
    strncpy(bracket_pair.as_mut_ptr(), ch, ch_len as libc::c_ulong);
    strncpy(
        bracket_pair.as_mut_ptr().offset(ch_len as isize),
        wanted_ch,
        wanted_ch_len as libc::c_ulong,
    );
    bracket_pair[(ch_len + wanted_ch_len) as usize] = '\0' as i32 as libc::c_char;
    while find_a_bracket(reverse, bracket_pair.as_mut_ptr()) {
        balance = (balance as libc::c_ulong)
            .wrapping_add(
                (if strncmp(
                    ((*(*openfile).current).data).offset((*openfile).current_x as isize),
                    ch,
                    ch_len as libc::c_ulong,
                ) == 0 as libc::c_int
                {
                    1 as libc::c_int
                } else {
                    -(1 as libc::c_int)
                }) as libc::c_ulong,
            ) as size_t as size_t;
        if balance == 0 as libc::c_int as libc::c_ulong {
            edit_redraw(was_current, FLOWING);
            return;
        }
    }
    statusline(
        AHEM,
        dcgettext(
            0 as *const libc::c_char,
            b"No matching bracket\0" as *const u8 as *const libc::c_char,
            5 as libc::c_int,
        ),
    );
    (*openfile).current = was_current;
    (*openfile).current_x = was_current_x;
}
pub unsafe extern "C" fn put_or_lift_anchor() {
    (*(*openfile).current).has_anchor = !(*(*openfile).current).has_anchor;
    update_line((*openfile).current, (*openfile).current_x);
    if (*(*openfile).current).has_anchor {
        statusline(
            REMARK,
            dcgettext(
                0 as *const libc::c_char,
                b"Placed anchor\0" as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ),
        );
    } else {
        statusline(
            REMARK,
            dcgettext(
                0 as *const libc::c_char,
                b"Removed anchor\0" as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ),
        );
    };
}
pub unsafe extern "C" fn go_to_and_confirm(mut line: *mut linestruct) {
    let mut was_current: *mut linestruct = (*openfile).current;
    if line != (*openfile).current {
        (*openfile).current = line;
        (*openfile).current_x = 0 as libc::c_int as size_t;
        if (*line).lineno > (*(*openfile).edittop).lineno + editwinrows as libc::c_long
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
                && (*line).lineno > (*was_current).lineno
        {
            recook = (recook as libc::c_int | perturbed as libc::c_int) != 0;
        }
        edit_redraw(was_current, CENTERING);
        statusbar(
            dcgettext(
                0 as *const libc::c_char,
                b"Jumped to anchor\0" as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ),
        );
    } else if (*(*openfile).current).has_anchor {
        statusline(
            REMARK,
            dcgettext(
                0 as *const libc::c_char,
                b"This is the only anchor\0" as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ),
        );
    } else {
        statusline(
            AHEM,
            dcgettext(
                0 as *const libc::c_char,
                b"There are no anchors\0" as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ),
        );
    };
}
pub unsafe extern "C" fn to_prev_anchor() {
    let mut line: *mut linestruct = (*openfile).current;
    loop {
        line = if !((*line).prev).is_null() {
            (*line).prev
        } else {
            (*openfile).filebot
        };
        if !(!(*line).has_anchor && line != (*openfile).current) {
            break;
        }
    }
    go_to_and_confirm(line);
}
pub unsafe extern "C" fn to_next_anchor() {
    let mut line: *mut linestruct = (*openfile).current;
    loop {
        line = if !((*line).next).is_null() {
            (*line).next
        } else {
            (*openfile).filetop
        };
        if !(!(*line).has_anchor && line != (*openfile).current) {
            break;
        }
    }
    go_to_and_confirm(line);
}
