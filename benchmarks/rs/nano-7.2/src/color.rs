use ::libc;
use ::c2rust_bitfields;
extern "C" {
    pub type re_dfa_t;
    fn init_pair(_: libc::c_short, _: libc::c_short, _: libc::c_short) -> libc::c_int;
    fn use_default_colors() -> libc::c_int;
    static mut COLORS: libc::c_int;
    fn rpl_free(ptr: *mut libc::c_void);
    fn rpl_regexec(
        __preg: *const regex_t,
        __String: *const libc::c_char,
        __nmatch: size_t,
        __pmatch: *mut regmatch_t,
        __eflags: libc::c_int,
    ) -> libc::c_int;
    fn dcgettext(
        __domainname: *const libc::c_char,
        __msgid: *const libc::c_char,
        __category: libc::c_int,
    ) -> *mut libc::c_char;
    static mut inhelp: bool;
    static mut flags: [libc::c_uint; 4];
    static mut openfile: *mut openfilestruct;
    static mut syntaxes: *mut syntaxtype;
    static mut syntaxstr: *mut libc::c_char;
    static mut have_palette: bool;
    static mut rescind_colors: bool;
    static mut perturbed: bool;
    static mut refresh_needed: bool;
    static mut hilite_attribute: libc::c_int;
    static mut color_combo: [*mut colortype; 12];
    static mut interface_color_pair: [libc::c_int; 12];
    fn step_right(buf: *const libc::c_char, pos: size_t) -> size_t;
    fn parse_one_include(file: *mut libc::c_char, syntax: *mut syntaxtype);
    fn get_full_path(origpath: *const libc::c_char) -> *mut libc::c_char;
    fn mallocstrcpy(
        dest: *mut libc::c_char,
        src: *const libc::c_char,
    ) -> *mut libc::c_char;
    fn statusline(importance: message_type, msg: *const libc::c_char, _: ...);
    fn nmalloc(howmuch: size_t) -> *mut libc::c_void;
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
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
pub type regoff_t = ssize_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct regmatch_t {
    pub rm_so: regoff_t,
    pub rm_eo: regoff_t,
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
pub type chtype = libc::c_uint;
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
static mut defaults_allowed: bool = 0 as libc::c_int != 0;
pub unsafe extern "C" fn set_interface_colorpairs() {
    defaults_allowed = use_default_colors() == 0 as libc::c_int;
    let mut index: size_t = 0 as libc::c_int as size_t;
    while index < NUMBER_OF_ELEMENTS as libc::c_int as libc::c_ulong {
        let mut combo: *mut colortype = color_combo[index as usize];
        if !combo.is_null() {
            if !defaults_allowed {
                if (*combo).fg as libc::c_int == -(1 as libc::c_int) {
                    (*combo).fg = 7 as libc::c_int as libc::c_short;
                }
                if (*combo).bg as libc::c_int == -(1 as libc::c_int) {
                    (*combo).bg = 0 as libc::c_int as libc::c_short;
                }
            }
            init_pair(
                index.wrapping_add(1 as libc::c_int as libc::c_ulong) as libc::c_short,
                (*combo).fg,
                (*combo).bg,
            );
            interface_color_pair[index
                as usize] = ((index.wrapping_add(1 as libc::c_int as libc::c_ulong)
                as chtype) << 0 as libc::c_int + 8 as libc::c_int
                & ((1 as libc::c_uint) << 8 as libc::c_int)
                    .wrapping_sub(1 as libc::c_uint)
                    << 0 as libc::c_int + 8 as libc::c_int
                | (*combo).attributes as libc::c_uint) as libc::c_int;
            rescind_colors = 0 as libc::c_int != 0;
        } else if index == FUNCTION_TAG as libc::c_int as libc::c_ulong
            || index == SCROLL_BAR as libc::c_int as libc::c_ulong
        {
            interface_color_pair[index
                as usize] = (1 as libc::c_uint).wrapping_sub(1 as libc::c_uint)
                as libc::c_int;
        } else if index == GUIDE_STRIPE as libc::c_int as libc::c_ulong {
            interface_color_pair[index
                as usize] = ((1 as libc::c_uint) << 10 as libc::c_int + 8 as libc::c_int)
                as libc::c_int;
        } else if index == SPOTLIGHTED as libc::c_int as libc::c_ulong {
            init_pair(
                index.wrapping_add(1 as libc::c_int as libc::c_ulong) as libc::c_short,
                0 as libc::c_int as libc::c_short,
                (3 as libc::c_int
                    + (if COLORS > 15 as libc::c_int {
                        8 as libc::c_int
                    } else {
                        0 as libc::c_int
                    })) as libc::c_short,
            );
            interface_color_pair[index
                as usize] = ((index.wrapping_add(1 as libc::c_int as libc::c_ulong)
                as chtype) << 0 as libc::c_int + 8 as libc::c_int
                & ((1 as libc::c_uint) << 8 as libc::c_int)
                    .wrapping_sub(1 as libc::c_uint)
                    << 0 as libc::c_int + 8 as libc::c_int) as libc::c_int;
        } else if index == MINI_INFOBAR as libc::c_int as libc::c_ulong
            || index == PROMPT_BAR as libc::c_int as libc::c_ulong
        {
            interface_color_pair[index
                as usize] = interface_color_pair[TITLE_BAR as libc::c_int as usize];
        } else if index == ERROR_MESSAGE as libc::c_int as libc::c_ulong {
            init_pair(
                index.wrapping_add(1 as libc::c_int as libc::c_ulong) as libc::c_short,
                7 as libc::c_int as libc::c_short,
                1 as libc::c_int as libc::c_short,
            );
            interface_color_pair[index
                as usize] = ((index.wrapping_add(1 as libc::c_int as libc::c_ulong)
                as chtype) << 0 as libc::c_int + 8 as libc::c_int
                & ((1 as libc::c_uint) << 8 as libc::c_int)
                    .wrapping_sub(1 as libc::c_uint)
                    << 0 as libc::c_int + 8 as libc::c_int
                | (1 as libc::c_uint) << 13 as libc::c_int + 8 as libc::c_int)
                as libc::c_int;
        } else {
            interface_color_pair[index as usize] = hilite_attribute;
        }
        rpl_free(color_combo[index as usize] as *mut libc::c_void);
        index = index.wrapping_add(1);
        index;
    }
    if rescind_colors {
        interface_color_pair[SPOTLIGHTED as libc::c_int
            as usize] = ((1 as libc::c_uint) << 10 as libc::c_int + 8 as libc::c_int)
            as libc::c_int;
        interface_color_pair[ERROR_MESSAGE as libc::c_int
            as usize] = ((1 as libc::c_uint) << 10 as libc::c_int + 8 as libc::c_int)
            as libc::c_int;
    }
}
pub unsafe extern "C" fn set_syntax_colorpairs(mut sntx: *mut syntaxtype) {
    let mut number: libc::c_short = NUMBER_OF_ELEMENTS as libc::c_int as libc::c_short;
    let mut older: *mut colortype = 0 as *mut colortype;
    let mut ink: *mut colortype = (*sntx).color;
    while !ink.is_null() {
        if !defaults_allowed {
            if (*ink).fg as libc::c_int == -(1 as libc::c_int) {
                (*ink).fg = 7 as libc::c_int as libc::c_short;
            }
            if (*ink).bg as libc::c_int == -(1 as libc::c_int) {
                (*ink).bg = 0 as libc::c_int as libc::c_short;
            }
        }
        older = (*sntx).color;
        while older != ink
            && ((*older).fg as libc::c_int != (*ink).fg as libc::c_int
                || (*older).bg as libc::c_int != (*ink).bg as libc::c_int)
        {
            older = (*older).next;
        }
        (*ink)
            .pairnum = (if older != ink {
            (*older).pairnum as libc::c_int
        } else {
            number += 1;
            number as libc::c_int
        }) as libc::c_short;
        (*ink)
            .attributes = ((*ink).attributes as libc::c_uint
            | ((*ink).pairnum as chtype) << 0 as libc::c_int + 8 as libc::c_int
                & ((1 as libc::c_uint) << 8 as libc::c_int)
                    .wrapping_sub(1 as libc::c_uint)
                    << 0 as libc::c_int + 8 as libc::c_int) as libc::c_int;
        ink = (*ink).next;
    }
}
pub unsafe extern "C" fn prepare_palette() {
    let mut number: libc::c_short = NUMBER_OF_ELEMENTS as libc::c_int as libc::c_short;
    let mut ink: *mut colortype = (*(*openfile).syntax).color;
    while !ink.is_null() {
        if (*ink).pairnum as libc::c_int > number as libc::c_int {
            init_pair((*ink).pairnum, (*ink).fg, (*ink).bg);
            number = (*ink).pairnum;
        }
        ink = (*ink).next;
    }
    have_palette = 1 as libc::c_int != 0;
}
pub unsafe extern "C" fn found_in_list(
    mut head: *mut regexlisttype,
    mut shibboleth: *const libc::c_char,
) -> bool {
    let mut item: *mut regexlisttype = head;
    while !item.is_null() {
        if rpl_regexec(
            (*item).one_rgx,
            shibboleth,
            0 as libc::c_int as size_t,
            0 as *mut regmatch_t,
            0 as libc::c_int,
        ) == 0 as libc::c_int
        {
            return 1 as libc::c_int != 0;
        }
        item = (*item).next;
    }
    return 0 as libc::c_int != 0;
}
pub unsafe extern "C" fn find_and_prime_applicable_syntax() {
    let mut sntx: *mut syntaxtype = 0 as *mut syntaxtype;
    if syntaxes.is_null() {
        return;
    }
    if !syntaxstr.is_null() {
        if strcmp(syntaxstr, b"none\0" as *const u8 as *const libc::c_char)
            == 0 as libc::c_int
        {
            return;
        }
        sntx = syntaxes;
        while !sntx.is_null() {
            if strcmp((*sntx).name, syntaxstr) == 0 as libc::c_int {
                break;
            }
            sntx = (*sntx).next;
        }
        if sntx.is_null() && !inhelp {
            statusline(
                ALERT,
                dcgettext(
                    0 as *const libc::c_char,
                    b"Unknown syntax name: %s\0" as *const u8 as *const libc::c_char,
                    5 as libc::c_int,
                ),
                syntaxstr,
            );
        }
    }
    if sntx.is_null() && !inhelp {
        let mut fullname: *mut libc::c_char = get_full_path((*openfile).filename);
        if fullname.is_null() {
            fullname = mallocstrcpy(fullname, (*openfile).filename);
        }
        sntx = syntaxes;
        while !sntx.is_null() {
            if found_in_list((*sntx).extensions, fullname) {
                break;
            }
            sntx = (*sntx).next;
        }
        rpl_free(fullname as *mut libc::c_void);
    }
    if sntx.is_null() && !inhelp {
        sntx = syntaxes;
        while !sntx.is_null() {
            if found_in_list((*sntx).headers, (*(*openfile).filetop).data) {
                break;
            }
            sntx = (*sntx).next;
        }
    }
    if sntx.is_null() && !inhelp {
        sntx = syntaxes;
        while !sntx.is_null() {
            if strcmp((*sntx).name, b"default\0" as *const u8 as *const libc::c_char)
                == 0 as libc::c_int
            {
                break;
            }
            sntx = (*sntx).next;
        }
    }
    if !sntx.is_null() && !((*sntx).filename).is_null() {
        parse_one_include((*sntx).filename, sntx);
        set_syntax_colorpairs(sntx);
    }
    (*openfile).syntax = sntx;
}
pub unsafe extern "C" fn check_the_multis(mut line: *mut linestruct) {
    let mut ink: *const colortype = 0 as *const colortype;
    let mut astart: bool = false;
    let mut anend: bool = false;
    let mut startmatch: regmatch_t = regmatch_t { rm_so: 0, rm_eo: 0 };
    let mut endmatch: regmatch_t = regmatch_t { rm_so: 0, rm_eo: 0 };
    let mut afterstart: *mut libc::c_char = 0 as *mut libc::c_char;
    if ((*openfile).syntax).is_null()
        || (*(*openfile).syntax).nmultis as libc::c_int == 0 as libc::c_int
    {
        return;
    }
    if ((*line).multidata).is_null() {
        refresh_needed = 1 as libc::c_int != 0;
        return;
    }
    let mut current_block_12: u64;
    ink = (*(*openfile).syntax).color;
    while !ink.is_null() {
        if !((*ink).end).is_null() {
            astart = rpl_regexec(
                (*ink).start,
                (*line).data,
                1 as libc::c_int as size_t,
                &mut startmatch,
                0 as libc::c_int,
            ) == 0 as libc::c_int;
            afterstart = ((*line).data)
                .offset(
                    (if astart as libc::c_int != 0 {
                        startmatch.rm_eo
                    } else {
                        0 as libc::c_int as libc::c_long
                    }) as isize,
                );
            anend = rpl_regexec(
                (*ink).end,
                afterstart,
                1 as libc::c_int as size_t,
                &mut endmatch,
                0 as libc::c_int,
            ) == 0 as libc::c_int;
            if *((*line).multidata).offset((*ink).id as isize) as libc::c_int
                == (1 as libc::c_int) << 1 as libc::c_int
            {
                if !astart {
                    current_block_12 = 15619007995458559411;
                } else {
                    current_block_12 = 1109700713171191020;
                }
            } else if *((*line).multidata).offset((*ink).id as isize) as libc::c_int
                == (1 as libc::c_int) << 3 as libc::c_int
            {
                if !anend
                    && (!astart
                        || rpl_regexec(
                            (*ink).end,
                            (*line).data,
                            1 as libc::c_int as size_t,
                            &mut endmatch,
                            0 as libc::c_int,
                        ) != 0 as libc::c_int)
                {
                    current_block_12 = 15619007995458559411;
                } else {
                    current_block_12 = 1109700713171191020;
                }
            } else if *((*line).multidata).offset((*ink).id as isize) as libc::c_int
                == (1 as libc::c_int) << 5 as libc::c_int
            {
                if astart as libc::c_int != 0 && anend as libc::c_int != 0
                    && rpl_regexec(
                        (*ink).start,
                        ((*line).data)
                            .offset(startmatch.rm_eo as isize)
                            .offset(endmatch.rm_eo as isize),
                        1 as libc::c_int as size_t,
                        &mut startmatch,
                        0 as libc::c_int,
                    ) != 0 as libc::c_int
                {
                    current_block_12 = 15619007995458559411;
                } else {
                    current_block_12 = 1109700713171191020;
                }
            } else if *((*line).multidata).offset((*ink).id as isize) as libc::c_int
                == (1 as libc::c_int) << 2 as libc::c_int
            {
                if astart as libc::c_int != 0 && !anend {
                    current_block_12 = 15619007995458559411;
                } else {
                    current_block_12 = 1109700713171191020;
                }
            } else if *((*line).multidata).offset((*ink).id as isize) as libc::c_int
                == (1 as libc::c_int) << 4 as libc::c_int
            {
                if !astart && anend as libc::c_int != 0 {
                    current_block_12 = 15619007995458559411;
                } else {
                    current_block_12 = 1109700713171191020;
                }
            } else {
                current_block_12 = 1109700713171191020;
            }
            match current_block_12 {
                15619007995458559411 => {}
                _ => {
                    refresh_needed = 1 as libc::c_int != 0;
                    perturbed = 1 as libc::c_int != 0;
                    return;
                }
            }
        }
        ink = (*ink).next;
    }
}
pub unsafe extern "C" fn precalc_multicolorinfo() {
    let mut ink: *const colortype = 0 as *const colortype;
    let mut startmatch: regmatch_t = regmatch_t { rm_so: 0, rm_eo: 0 };
    let mut endmatch: regmatch_t = regmatch_t { rm_so: 0, rm_eo: 0 };
    let mut line: *mut linestruct = 0 as *mut linestruct;
    let mut tailline: *mut linestruct = 0 as *mut linestruct;
    if ((*openfile).syntax).is_null()
        || (*(*openfile).syntax).nmultis as libc::c_int == 0 as libc::c_int
        || flags[(NO_SYNTAX as libc::c_int as libc::c_ulong)
            .wrapping_div(
                (::std::mem::size_of::<libc::c_uint>() as libc::c_ulong)
                    .wrapping_mul(8 as libc::c_int as libc::c_ulong),
            ) as usize]
            & (1 as libc::c_int as libc::c_uint)
                << (NO_SYNTAX as libc::c_int as libc::c_ulong)
                    .wrapping_rem(
                        (::std::mem::size_of::<libc::c_uint>() as libc::c_ulong)
                            .wrapping_mul(8 as libc::c_int as libc::c_ulong),
                    ) != 0 as libc::c_int as libc::c_uint
    {
        return;
    }
    line = (*openfile).filetop;
    while !line.is_null() {
        if ((*line).multidata).is_null() {
            (*line)
                .multidata = nmalloc(
                ((*(*openfile).syntax).nmultis as libc::c_ulong)
                    .wrapping_mul(
                        ::std::mem::size_of::<libc::c_short>() as libc::c_ulong,
                    ),
            ) as *mut libc::c_short;
        }
        line = (*line).next;
    }
    ink = (*(*openfile).syntax).color;
    while !ink.is_null() {
        if !((*ink).end).is_null() {
            line = (*openfile).filetop;
            while !line.is_null() {
                let mut index: libc::c_int = 0 as libc::c_int;
                *((*line).multidata)
                    .offset(
                        (*ink).id as isize,
                    ) = ((1 as libc::c_int) << 1 as libc::c_int) as libc::c_short;
                while rpl_regexec(
                    (*ink).start,
                    ((*line).data).offset(index as isize),
                    1 as libc::c_int as size_t,
                    &mut startmatch,
                    if index == 0 as libc::c_int {
                        0 as libc::c_int
                    } else {
                        1 as libc::c_int
                    },
                ) == 0 as libc::c_int
                {
                    index = (index as libc::c_long + startmatch.rm_eo) as libc::c_int;
                    if rpl_regexec(
                        (*ink).end,
                        ((*line).data).offset(index as isize),
                        1 as libc::c_int as size_t,
                        &mut endmatch,
                        if index == 0 as libc::c_int {
                            0 as libc::c_int
                        } else {
                            1 as libc::c_int
                        },
                    ) == 0 as libc::c_int
                    {
                        *((*line).multidata)
                            .offset(
                                (*ink).id as isize,
                            ) = ((1 as libc::c_int) << 5 as libc::c_int)
                            as libc::c_short;
                        index = (index as libc::c_long + endmatch.rm_eo) as libc::c_int;
                        if !(startmatch.rm_eo - startmatch.rm_so + endmatch.rm_eo
                            == 0 as libc::c_int as libc::c_long)
                        {
                            continue;
                        }
                        if *((*line).data).offset(index as isize) as libc::c_int
                            == '\0' as i32
                        {
                            break;
                        }
                        index = step_right((*line).data, index as size_t) as libc::c_int;
                    } else {
                        tailline = (*line).next;
                        while !tailline.is_null()
                            && rpl_regexec(
                                (*ink).end,
                                (*tailline).data,
                                1 as libc::c_int as size_t,
                                &mut endmatch,
                                0 as libc::c_int,
                            ) != 0 as libc::c_int
                        {
                            tailline = (*tailline).next;
                        }
                        *((*line).multidata)
                            .offset(
                                (*ink).id as isize,
                            ) = ((1 as libc::c_int) << 2 as libc::c_int)
                            as libc::c_short;
                        line = (*line).next;
                        while line != tailline {
                            *((*line).multidata)
                                .offset(
                                    (*ink).id as isize,
                                ) = ((1 as libc::c_int) << 3 as libc::c_int)
                                as libc::c_short;
                            line = (*line).next;
                        }
                        if tailline.is_null() {
                            line = (*openfile).filebot;
                            break;
                        } else {
                            *((*tailline).multidata)
                                .offset(
                                    (*ink).id as isize,
                                ) = ((1 as libc::c_int) << 4 as libc::c_int)
                                as libc::c_short;
                            index = endmatch.rm_eo as libc::c_int;
                        }
                    }
                }
                line = (*line).next;
            }
        }
        ink = (*ink).next;
    }
}
