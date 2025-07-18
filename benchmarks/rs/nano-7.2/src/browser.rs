use ::libc;
extern "C" {
    pub type __dirstream;
    pub type ldat;
    fn beep() -> libc::c_int;
    fn curs_set(_: libc::c_int) -> libc::c_int;
    fn mvwprintw(
        _: *mut WINDOW,
        _: libc::c_int,
        _: libc::c_int,
        _: *const libc::c_char,
        _: ...
    ) -> libc::c_int;
    fn napms(_: libc::c_int) -> libc::c_int;
    fn opendir(__name: *const libc::c_char) -> *mut DIR;
    fn closedir(__dirp: *mut DIR) -> libc::c_int;
    fn readdir(__dirp: *mut DIR) -> *mut dirent;
    fn rewinddir(__dirp: *mut DIR);
    fn rpl_free(ptr: *mut libc::c_void);
    fn __xstat(
        __ver: libc::c_int,
        __filename: *const libc::c_char,
        __stat_buf: *mut stat,
    ) -> libc::c_int;
    fn __lxstat(
        __ver: libc::c_int,
        __filename: *const libc::c_char,
        __stat_buf: *mut stat,
    ) -> libc::c_int;
    fn sprintf(_: *mut libc::c_char, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn waddnstr(_: *mut WINDOW, _: *const libc::c_char, _: libc::c_int) -> libc::c_int;
    fn wattr_on(_: *mut WINDOW, _: attr_t, _: *mut libc::c_void) -> libc::c_int;
    fn wattr_off(_: *mut WINDOW, _: attr_t, _: *mut libc::c_void) -> libc::c_int;
    fn wmove(_: *mut WINDOW, _: libc::c_int, _: libc::c_int) -> libc::c_int;
    fn wnoutrefresh(_: *mut WINDOW) -> libc::c_int;
    static mut last_search: *mut libc::c_char;
    static mut COLS: libc::c_int;
    static mut LINES: libc::c_int;
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
    static mut bracketed_paste: bool;
    static mut lastmessage: message_type;
    static mut answer: *mut libc::c_char;
    static mut present_path: *mut libc::c_char;
    static mut flags: [libc::c_uint; 4];
    static mut midwin: *mut WINDOW;
    static mut editwinrows: libc::c_int;
    static mut operating_dir: *mut libc::c_char;
    static mut search_history: *mut linestruct;
    static mut searchbot: *mut linestruct;
    static mut interface_color_pair: [libc::c_int; 12];
    fn tail(path: *const libc::c_char) -> *const libc::c_char;
    fn breadth(text: *const libc::c_char) -> size_t;
    fn display_string(
        buf: *const libc::c_char,
        column: size_t,
        span: size_t,
        isdata: bool,
        isprompt: bool,
    ) -> *mut libc::c_char;
    fn actual_x(text: *const libc::c_char, column: size_t) -> size_t;
    fn mallocstrcpy(
        dest: *mut libc::c_char,
        src: *const libc::c_char,
    ) -> *mut libc::c_char;
    fn nmalloc(howmuch: size_t) -> *mut libc::c_void;
    fn copy_of(string: *const libc::c_char) -> *mut libc::c_char;
    fn blank_edit();
    fn titlebar(path: *const libc::c_char);
    fn real_dir_from_tilde(path: *const libc::c_char) -> *mut libc::c_char;
    fn free_chararray(array: *mut *mut libc::c_char, len: size_t);
    fn edit_refresh();
    fn get_full_path(origpath: *const libc::c_char) -> *mut libc::c_char;
    fn free_and_assign(
        dest: *mut libc::c_char,
        src: *mut libc::c_char,
    ) -> *mut libc::c_char;
    fn unbound_key(code: libc::c_int);
    fn do_exit();
    fn first_sc_for(
        menu: libc::c_int,
        function: Option::<unsafe extern "C" fn() -> ()>,
    ) -> *const keystruct;
    fn implant(string: *const libc::c_char);
    fn statusline(importance: message_type, msg: *const libc::c_char, _: ...);
    fn outside_of_confinement(
        currpath: *const libc::c_char,
        allow_tabcomp: bool,
    ) -> bool;
    fn do_enter();
    fn nrealloc(ptr: *mut libc::c_void, howmuch: size_t) -> *mut libc::c_void;
    fn statusbar(msg: *const libc::c_char);
    fn do_prompt(
        menu: libc::c_int,
        provided: *const libc::c_char,
        history_list: *mut *mut linestruct,
        refresh_func: Option::<unsafe extern "C" fn() -> ()>,
        msg: *const libc::c_char,
        _: ...
    ) -> libc::c_int;
    fn goto_dir();
    fn do_page_down();
    fn do_page_up();
    fn to_next_block();
    fn to_prev_block();
    fn do_down();
    fn do_up();
    fn to_next_word();
    fn to_prev_word();
    fn do_right();
    fn do_left();
    fn not_found_msg(str: *const libc::c_char);
    fn mbstrcasestr(
        haystack: *const libc::c_char,
        needle: *const libc::c_char,
    ) -> *mut libc::c_char;
    fn wipe_statusbar();
    fn do_findnext();
    fn do_findprevious();
    fn update_history(
        item: *mut *mut linestruct,
        text: *const libc::c_char,
        avoid_duplicates: bool,
    );
    fn do_search_forward();
    fn do_search_backward();
    fn window_init();
    fn get_shortcut(keycode: libc::c_int) -> *const keystruct;
    fn do_toggle();
    fn do_help();
    fn full_refresh();
    fn interpret(keycode: libc::c_int) -> functionptrtype;
    fn get_kbinput(win: *mut WINDOW, showcursor: bool) -> libc::c_int;
    fn get_mouseinput(
        mouse_y: *mut libc::c_int,
        mouse_x: *mut libc::c_int,
        allow_shortcuts: bool,
    ) -> libc::c_int;
    fn bottombars(menu: libc::c_int);
    fn diralphasort(va: *const libc::c_void, vb: *const libc::c_void) -> libc::c_int;
    fn realpath(
        __name: *const libc::c_char,
        __resolved: *mut libc::c_char,
    ) -> *mut libc::c_char;
    fn qsort(
        __base: *mut libc::c_void,
        __nmemb: size_t,
        __size: size_t,
        __compar: __compar_fn_t,
    );
    fn __errno_location() -> *mut libc::c_int;
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    fn strrchr(_: *const libc::c_char, _: libc::c_int) -> *mut libc::c_char;
    fn strerror(_: libc::c_int) -> *mut libc::c_char;
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
}
pub type __intmax_t = libc::c_long;
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
pub type off_t = __off_t;
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
pub type DIR = __dirstream;
pub type wchar_t = libc::c_int;
pub type __compar_fn_t = Option::<
    unsafe extern "C" fn(*const libc::c_void, *const libc::c_void) -> libc::c_int,
>;
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
pub type intmax_t = __intmax_t;
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
pub type message_type = libc::c_uint;
pub const ALERT: message_type = 7;
pub const MILD: message_type = 6;
pub const AHEM: message_type = 5;
pub const NOTICE: message_type = 4;
pub const INFO: message_type = 3;
pub const REMARK: message_type = 2;
pub const HUSH: message_type = 1;
pub const VACUUM: message_type = 0;
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
#[inline]
unsafe extern "C" fn stat(
    mut __path: *const libc::c_char,
    mut __statbuf: *mut stat,
) -> libc::c_int {
    return __xstat(1 as libc::c_int, __path, __statbuf);
}
#[inline]
unsafe extern "C" fn lstat(
    mut __path: *const libc::c_char,
    mut __statbuf: *mut stat,
) -> libc::c_int {
    return __lxstat(1 as libc::c_int, __path, __statbuf);
}
static mut filelist: *mut *mut libc::c_char = 0 as *const *mut libc::c_char
    as *mut *mut libc::c_char;
static mut list_length: size_t = 0 as libc::c_int as size_t;
static mut usable_rows: size_t = 0 as libc::c_int as size_t;
static mut piles: libc::c_int = 0 as libc::c_int;
static mut gauge: libc::c_int = 0 as libc::c_int;
static mut selected: size_t = 0 as libc::c_int as size_t;
pub unsafe extern "C" fn read_the_list(
    mut path: *const libc::c_char,
    mut dir: *mut DIR,
) {
    let mut path_len: size_t = strlen(path);
    let mut entry: *const dirent = 0 as *const dirent;
    let mut widest: size_t = 0 as libc::c_int as size_t;
    let mut index: size_t = 0 as libc::c_int as size_t;
    loop {
        entry = readdir(dir);
        if entry.is_null() {
            break;
        }
        let mut span: size_t = breadth(((*entry).d_name).as_ptr());
        if span > widest {
            widest = span;
        }
        index = index.wrapping_add(1);
        index;
    }
    gauge = widest.wrapping_add(10 as libc::c_int as libc::c_ulong) as libc::c_int;
    if gauge < 15 as libc::c_int {
        gauge = 15 as libc::c_int;
    }
    if gauge > COLS {
        gauge = COLS;
    }
    rewinddir(dir);
    free_chararray(filelist, list_length);
    list_length = index;
    index = 0 as libc::c_int as size_t;
    filelist = nmalloc(
        list_length
            .wrapping_mul(::std::mem::size_of::<*mut libc::c_char>() as libc::c_ulong),
    ) as *mut *mut libc::c_char;
    loop {
        entry = readdir(dir);
        if !(!entry.is_null() && index < list_length) {
            break;
        }
        if strcmp(((*entry).d_name).as_ptr(), b".\0" as *const u8 as *const libc::c_char)
            == 0 as libc::c_int
        {
            continue;
        }
        let ref mut fresh0 = *filelist.offset(index as isize);
        *fresh0 = nmalloc(
            path_len
                .wrapping_add(strlen(((*entry).d_name).as_ptr()))
                .wrapping_add(1 as libc::c_int as libc::c_ulong),
        ) as *mut libc::c_char;
        sprintf(
            *filelist.offset(index as isize),
            b"%s%s\0" as *const u8 as *const libc::c_char,
            path,
            ((*entry).d_name).as_ptr(),
        );
        index = index.wrapping_add(1);
        index;
    }
    list_length = index;
    qsort(
        filelist as *mut libc::c_void,
        list_length,
        ::std::mem::size_of::<*mut libc::c_char>() as libc::c_ulong,
        Some(
            diralphasort
                as unsafe extern "C" fn(
                    *const libc::c_void,
                    *const libc::c_void,
                ) -> libc::c_int,
        ),
    );
    piles = (COLS + 2 as libc::c_int) / (gauge + 2 as libc::c_int);
    usable_rows = (editwinrows
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
                    ) != 0 as libc::c_int as libc::c_uint && LINES > 1 as libc::c_int
        {
            1 as libc::c_int
        } else {
            0 as libc::c_int
        })) as size_t;
}
pub unsafe extern "C" fn reselect(mut name: *const libc::c_char) {
    let mut looking_at: size_t = 0 as libc::c_int as size_t;
    while looking_at < list_length
        && strcmp(*filelist.offset(looking_at as isize), name) != 0 as libc::c_int
    {
        looking_at = looking_at.wrapping_add(1);
        looking_at;
    }
    if looking_at < list_length {
        selected = looking_at;
    } else if selected > list_length {
        selected = list_length.wrapping_sub(1 as libc::c_int as libc::c_ulong);
    } else {
        selected = selected.wrapping_sub(1);
        selected;
    };
}
pub unsafe extern "C" fn browser_refresh() {
    let mut row: libc::c_int = 0 as libc::c_int;
    let mut col: libc::c_int = 0 as libc::c_int;
    let mut the_row: libc::c_int = 0 as libc::c_int;
    let mut the_column: libc::c_int = 0 as libc::c_int;
    let mut info: *mut libc::c_char = 0 as *mut libc::c_char;
    titlebar(present_path);
    blank_edit();
    let mut index: size_t = selected
        .wrapping_sub(
            selected.wrapping_rem(usable_rows.wrapping_mul(piles as libc::c_ulong)),
        );
    while index < list_length && (row as libc::c_ulong) < usable_rows {
        let mut thename: *const libc::c_char = tail(*filelist.offset(index as isize));
        let mut namelen: size_t = breadth(thename);
        let mut infolen: size_t = 0;
        let mut infomaxlen: size_t = 7 as libc::c_int as size_t;
        let mut dots: bool = COLS >= 15 as libc::c_int
            && namelen >= (gauge as libc::c_ulong).wrapping_sub(infomaxlen);
        let mut disp: *mut libc::c_char = display_string(
            thename,
            if dots as libc::c_int != 0 {
                namelen
                    .wrapping_add(infomaxlen)
                    .wrapping_add(4 as libc::c_int as libc::c_ulong)
                    .wrapping_sub(gauge as libc::c_ulong)
            } else {
                0 as libc::c_int as libc::c_ulong
            },
            gauge as size_t,
            0 as libc::c_int != 0,
            0 as libc::c_int != 0,
        );
        let mut state: stat = stat {
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
        if index == selected {
            wattr_on(
                midwin,
                interface_color_pair[SELECTED_TEXT as libc::c_int as usize] as attr_t,
                0 as *mut libc::c_void,
            );
            mvwprintw(
                midwin,
                row,
                col,
                b"%*s\0" as *const u8 as *const libc::c_char,
                gauge,
                b" \0" as *const u8 as *const libc::c_char,
            );
            the_row = row;
            the_column = col;
        }
        if dots {
            if wmove(midwin, row, col) == -(1 as libc::c_int) {
                -(1 as libc::c_int);
            } else {
                waddnstr(
                    midwin,
                    b"...\0" as *const u8 as *const libc::c_char,
                    -(1 as libc::c_int),
                );
            };
        }
        if wmove(
            midwin,
            row,
            if dots as libc::c_int != 0 { col + 3 as libc::c_int } else { col },
        ) == -(1 as libc::c_int)
        {
            -(1 as libc::c_int);
        } else {
            waddnstr(midwin, disp, -(1 as libc::c_int));
        };
        col += gauge;
        if lstat(*filelist.offset(index as isize), &mut state) == -(1 as libc::c_int)
            || state.st_mode & 0o170000 as libc::c_int as libc::c_uint
                == 0o120000 as libc::c_int as libc::c_uint
        {
            if stat(*filelist.offset(index as isize), &mut state) == -(1 as libc::c_int)
                || !(state.st_mode & 0o170000 as libc::c_int as libc::c_uint
                    == 0o40000 as libc::c_int as libc::c_uint)
            {
                info = copy_of(b"--\0" as *const u8 as *const libc::c_char);
            } else {
                info = copy_of(
                    dcgettext(
                        0 as *const libc::c_char,
                        b"(dir)\0" as *const u8 as *const libc::c_char,
                        5 as libc::c_int,
                    ),
                );
            }
        } else if state.st_mode & 0o170000 as libc::c_int as libc::c_uint
            == 0o40000 as libc::c_int as libc::c_uint
        {
            if strcmp(thename, b"..\0" as *const u8 as *const libc::c_char)
                == 0 as libc::c_int
            {
                info = copy_of(
                    dcgettext(
                        0 as *const libc::c_char,
                        b"(parent dir)\0" as *const u8 as *const libc::c_char,
                        5 as libc::c_int,
                    ),
                );
                infomaxlen = 12 as libc::c_int as size_t;
            } else {
                info = copy_of(
                    dcgettext(
                        0 as *const libc::c_char,
                        b"(dir)\0" as *const u8 as *const libc::c_char,
                        5 as libc::c_int,
                    ),
                );
            }
        } else {
            let mut result: off_t = state.st_size;
            let mut modifier: libc::c_char = 0;
            info = nmalloc(infomaxlen.wrapping_add(1 as libc::c_int as libc::c_ulong))
                as *mut libc::c_char;
            if state.st_size < ((1 as libc::c_int) << 10 as libc::c_int) as libc::c_long
            {
                modifier = ' ' as i32 as libc::c_char;
            } else if state.st_size
                < ((1 as libc::c_int) << 20 as libc::c_int) as libc::c_long
            {
                result >>= 10 as libc::c_int;
                modifier = 'K' as i32 as libc::c_char;
            } else if state.st_size
                < ((1 as libc::c_int) << 30 as libc::c_int) as libc::c_long
            {
                result >>= 20 as libc::c_int;
                modifier = 'M' as i32 as libc::c_char;
            } else {
                result >>= 30 as libc::c_int;
                modifier = 'G' as i32 as libc::c_char;
            }
            if result < ((1 as libc::c_int) << 10 as libc::c_int) as libc::c_long {
                sprintf(
                    info,
                    b"%4ju %cB\0" as *const u8 as *const libc::c_char,
                    result,
                    modifier as libc::c_int,
                );
            } else {
                info = mallocstrcpy(
                    info,
                    dcgettext(
                        0 as *const libc::c_char,
                        b"(huge)\0" as *const u8 as *const libc::c_char,
                        5 as libc::c_int,
                    ),
                );
            }
        }
        infolen = breadth(info);
        if infolen > infomaxlen {
            *info
                .offset(
                    actual_x(info, infomaxlen) as isize,
                ) = '\0' as i32 as libc::c_char;
            infolen = infomaxlen;
        }
        if wmove(
            midwin,
            row,
            (col as libc::c_ulong).wrapping_sub(infolen) as libc::c_int,
        ) == -(1 as libc::c_int)
        {
            -(1 as libc::c_int);
        } else {
            waddnstr(midwin, info, -(1 as libc::c_int));
        };
        if index == selected {
            wattr_off(
                midwin,
                interface_color_pair[SELECTED_TEXT as libc::c_int as usize] as attr_t,
                0 as *mut libc::c_void,
            );
        }
        rpl_free(disp as *mut libc::c_void);
        rpl_free(info as *mut libc::c_void);
        col += 2 as libc::c_int;
        if col > COLS - gauge {
            row += 1;
            row;
            col = 0 as libc::c_int;
        }
        index = index.wrapping_add(1);
        index;
    }
    if flags[(SHOW_CURSOR as libc::c_int as libc::c_ulong)
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
    {
        wmove(midwin, the_row, the_column);
        curs_set(1 as libc::c_int);
    }
    wnoutrefresh(midwin);
}
pub unsafe extern "C" fn findfile(mut needle: *const libc::c_char, mut forwards: bool) {
    let mut began_at: size_t = selected;
    loop {
        if forwards {
            let fresh1 = selected;
            selected = selected.wrapping_add(1);
            if fresh1 == list_length.wrapping_sub(1 as libc::c_int as libc::c_ulong) {
                selected = 0 as libc::c_int as size_t;
                statusbar(
                    dcgettext(
                        0 as *const libc::c_char,
                        b"Search Wrapped\0" as *const u8 as *const libc::c_char,
                        5 as libc::c_int,
                    ),
                );
            }
        } else {
            let fresh2 = selected;
            selected = selected.wrapping_sub(1);
            if fresh2 == 0 as libc::c_int as libc::c_ulong {
                selected = list_length.wrapping_sub(1 as libc::c_int as libc::c_ulong);
                statusbar(
                    dcgettext(
                        0 as *const libc::c_char,
                        b"Search Wrapped\0" as *const u8 as *const libc::c_char,
                        5 as libc::c_int,
                    ),
                );
            }
        }
        if !(mbstrcasestr(tail(*filelist.offset(selected as isize)), needle)).is_null() {
            if selected == began_at {
                statusbar(
                    dcgettext(
                        0 as *const libc::c_char,
                        b"This is the only occurrence\0" as *const u8
                            as *const libc::c_char,
                        5 as libc::c_int,
                    ),
                );
            }
            return;
        }
        if selected == began_at {
            not_found_msg(needle);
            return;
        }
    };
}
pub unsafe extern "C" fn search_filename(mut forwards: bool) {
    let mut thedefault: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut response: libc::c_int = 0;
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
    response = do_prompt(
        (1 as libc::c_int) << 11 as libc::c_int,
        b"\0" as *const u8 as *const libc::c_char,
        &mut search_history as *mut *mut linestruct,
        Some(browser_refresh as unsafe extern "C" fn() -> ()),
        b"%s%s%s\0" as *const u8 as *const libc::c_char,
        dcgettext(
            0 as *const libc::c_char,
            b"Search\0" as *const u8 as *const libc::c_char,
            5 as libc::c_int,
        ),
        if !forwards {
            dcgettext(
                0 as *const libc::c_char,
                b" [Backwards]\0" as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ) as *const libc::c_char
        } else {
            b"\0" as *const u8 as *const libc::c_char
        },
        thedefault,
    );
    rpl_free(thedefault as *mut libc::c_void);
    if response == -(1 as libc::c_int)
        || response == -(2 as libc::c_int) && *last_search as libc::c_int == '\0' as i32
    {
        statusbar(
            dcgettext(
                0 as *const libc::c_char,
                b"Cancelled\0" as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ),
        );
        return;
    }
    if *answer as libc::c_int != '\0' as i32 {
        last_search = mallocstrcpy(last_search, answer);
        update_history(&mut search_history, answer, 1 as libc::c_int != 0);
    }
    if response == 0 as libc::c_int || response == -(2 as libc::c_int) {
        findfile(last_search, forwards);
    }
}
pub unsafe extern "C" fn research_filename(mut forwards: bool) {
    if *last_search as libc::c_int == '\0' as i32 && !((*searchbot).prev).is_null() {
        last_search = mallocstrcpy(last_search, (*(*searchbot).prev).data);
    }
    if *last_search as libc::c_int == '\0' as i32 {
        statusbar(
            dcgettext(
                0 as *const libc::c_char,
                b"No current search pattern\0" as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ),
        );
    } else {
        wipe_statusbar();
        findfile(last_search, forwards);
    };
}
pub unsafe extern "C" fn to_first_file() {
    selected = 0 as libc::c_int as size_t;
}
pub unsafe extern "C" fn to_last_file() {
    selected = list_length.wrapping_sub(1 as libc::c_int as libc::c_ulong);
}
pub unsafe extern "C" fn strip_last_component(
    mut path: *const libc::c_char,
) -> *mut libc::c_char {
    let mut copy: *mut libc::c_char = copy_of(path);
    let mut last_slash: *mut libc::c_char = strrchr(copy, '/' as i32);
    if !last_slash.is_null() {
        *last_slash = '\0' as i32 as libc::c_char;
    }
    return copy;
}
pub unsafe extern "C" fn browse(mut path: *mut libc::c_char) -> *mut libc::c_char {
    let mut present_name: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut old_selected: size_t = 0;
    let mut dir: *mut DIR = 0 as *mut DIR;
    let mut chosen: *mut libc::c_char = 0 as *mut libc::c_char;
    '_read_directory_contents: loop {
        path = free_and_assign(path, get_full_path(path));
        if !path.is_null() {
            dir = opendir(path);
        }
        if path.is_null() || dir.is_null() {
            statusline(
                ALERT,
                dcgettext(
                    0 as *const libc::c_char,
                    b"Cannot open directory: %s\0" as *const u8 as *const libc::c_char,
                    5 as libc::c_int,
                ),
                strerror(*__errno_location()),
            );
            if filelist.is_null() {
                lastmessage = VACUUM;
                rpl_free(present_name as *mut libc::c_void);
                rpl_free(path as *mut libc::c_void);
                napms(1200 as libc::c_int);
                return 0 as *mut libc::c_char;
            }
            path = mallocstrcpy(path, present_path);
            present_name = mallocstrcpy(
                present_name,
                *filelist.offset(selected as isize),
            );
        }
        if !dir.is_null() {
            read_the_list(path, dir);
            closedir(dir);
            dir = 0 as *mut DIR;
        }
        if !present_name.is_null() {
            reselect(present_name);
            rpl_free(present_name as *mut libc::c_void);
            present_name = 0 as *mut libc::c_char;
        } else {
            selected = 0 as libc::c_int as size_t;
        }
        old_selected = -(1 as libc::c_int) as size_t;
        present_path = mallocstrcpy(present_path, path);
        titlebar(path);
        loop {
            let mut function: functionptrtype = None;
            let mut kbinput: libc::c_int = 0;
            lastmessage = VACUUM;
            bottombars((1 as libc::c_int) << 10 as libc::c_int);
            if old_selected != selected
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
            {
                browser_refresh();
            }
            old_selected = selected;
            kbinput = get_kbinput(
                midwin,
                flags[(SHOW_CURSOR as libc::c_int as libc::c_ulong)
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
            if kbinput == 0o631 as libc::c_int {
                let mut mouse_x: libc::c_int = 0;
                let mut mouse_y: libc::c_int = 0;
                if get_mouseinput(&mut mouse_y, &mut mouse_x, 1 as libc::c_int != 0)
                    == 0 as libc::c_int
                    && wmouse_trafo(
                        midwin,
                        &mut mouse_y,
                        &mut mouse_x,
                        0 as libc::c_int != 0,
                    ) as libc::c_int != 0
                {
                    selected = selected
                        .wrapping_sub(
                            selected
                                .wrapping_rem(
                                    usable_rows.wrapping_mul(piles as libc::c_ulong),
                                ),
                        )
                        .wrapping_add((mouse_y * piles) as libc::c_ulong)
                        .wrapping_add(
                            (mouse_x / (gauge + 2 as libc::c_int)) as libc::c_ulong,
                        );
                    if mouse_x > piles * (gauge + 2 as libc::c_int) {
                        selected = selected.wrapping_sub(1);
                        selected;
                    }
                    if selected
                        > list_length.wrapping_sub(1 as libc::c_int as libc::c_ulong)
                    {
                        selected = list_length
                            .wrapping_sub(1 as libc::c_int as libc::c_ulong);
                    }
                    if old_selected == selected {
                        kbinput = 0o527 as libc::c_int;
                    }
                }
                if kbinput == 0o631 as libc::c_int {
                    continue;
                }
            }
            while bracketed_paste {
                kbinput = get_kbinput(midwin, 0 as libc::c_int != 0);
            }
            if kbinput == 0x4fb as libc::c_int {
                beep();
            } else {
                function = interpret(kbinput);
                if function == Some(full_refresh as unsafe extern "C" fn() -> ())
                    || function == Some(do_help as unsafe extern "C" fn() -> ())
                {
                    function.unwrap()();
                    kbinput = -(2 as libc::c_int);
                } else if function == Some(do_toggle as unsafe extern "C" fn() -> ())
                    && (*get_shortcut(kbinput)).toggle == NO_HELP as libc::c_int
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
                    kbinput = -(2 as libc::c_int);
                } else if function
                    == Some(do_search_backward as unsafe extern "C" fn() -> ())
                {
                    search_filename(0 as libc::c_int != 0);
                } else if function
                    == Some(do_search_forward as unsafe extern "C" fn() -> ())
                {
                    search_filename(1 as libc::c_int != 0);
                } else if function
                    == Some(do_findprevious as unsafe extern "C" fn() -> ())
                {
                    research_filename(0 as libc::c_int != 0);
                } else if function == Some(do_findnext as unsafe extern "C" fn() -> ()) {
                    research_filename(1 as libc::c_int != 0);
                } else if function == Some(do_left as unsafe extern "C" fn() -> ()) {
                    if selected > 0 as libc::c_int as libc::c_ulong {
                        selected = selected.wrapping_sub(1);
                        selected;
                    }
                } else if function == Some(do_right as unsafe extern "C" fn() -> ()) {
                    if selected
                        < list_length.wrapping_sub(1 as libc::c_int as libc::c_ulong)
                    {
                        selected = selected.wrapping_add(1);
                        selected;
                    }
                } else if function == Some(to_prev_word as unsafe extern "C" fn() -> ())
                {
                    selected = (selected as libc::c_ulong)
                        .wrapping_sub(selected.wrapping_rem(piles as libc::c_ulong))
                        as size_t as size_t;
                } else if function == Some(to_next_word as unsafe extern "C" fn() -> ())
                {
                    selected = (selected as libc::c_ulong)
                        .wrapping_add(
                            ((piles - 1 as libc::c_int) as libc::c_ulong)
                                .wrapping_sub(selected.wrapping_rem(piles as libc::c_ulong)),
                        ) as size_t as size_t;
                    if selected >= list_length {
                        selected = list_length
                            .wrapping_sub(1 as libc::c_int as libc::c_ulong);
                    }
                } else if function == Some(do_up as unsafe extern "C" fn() -> ()) {
                    if selected >= piles as libc::c_ulong {
                        selected = (selected as libc::c_ulong)
                            .wrapping_sub(piles as libc::c_ulong) as size_t as size_t;
                    }
                } else if function == Some(do_down as unsafe extern "C" fn() -> ()) {
                    if selected.wrapping_add(piles as libc::c_ulong)
                        <= list_length.wrapping_sub(1 as libc::c_int as libc::c_ulong)
                    {
                        selected = (selected as libc::c_ulong)
                            .wrapping_add(piles as libc::c_ulong) as size_t as size_t;
                    }
                } else if function == Some(to_prev_block as unsafe extern "C" fn() -> ())
                {
                    selected = selected
                        .wrapping_div(usable_rows.wrapping_mul(piles as libc::c_ulong))
                        .wrapping_mul(usable_rows)
                        .wrapping_mul(piles as libc::c_ulong)
                        .wrapping_add(selected.wrapping_rem(piles as libc::c_ulong));
                } else if function == Some(to_next_block as unsafe extern "C" fn() -> ())
                {
                    selected = selected
                        .wrapping_div(usable_rows.wrapping_mul(piles as libc::c_ulong))
                        .wrapping_mul(usable_rows)
                        .wrapping_mul(piles as libc::c_ulong)
                        .wrapping_add(selected.wrapping_rem(piles as libc::c_ulong))
                        .wrapping_add(usable_rows.wrapping_mul(piles as libc::c_ulong))
                        .wrapping_sub(piles as libc::c_ulong);
                    if selected >= list_length {
                        selected = list_length
                            .wrapping_div(piles as libc::c_ulong)
                            .wrapping_mul(piles as libc::c_ulong)
                            .wrapping_add(selected.wrapping_rem(piles as libc::c_ulong));
                    }
                    if selected >= list_length {
                        selected = (selected as libc::c_ulong)
                            .wrapping_sub(piles as libc::c_ulong) as size_t as size_t;
                    }
                } else if function == Some(do_page_up as unsafe extern "C" fn() -> ()) {
                    if selected < piles as libc::c_ulong {
                        selected = 0 as libc::c_int as size_t;
                    } else if selected < usable_rows.wrapping_mul(piles as libc::c_ulong)
                    {
                        selected = selected.wrapping_rem(piles as libc::c_ulong);
                    } else {
                        selected = (selected as libc::c_ulong)
                            .wrapping_sub(
                                usable_rows.wrapping_mul(piles as libc::c_ulong),
                            ) as size_t as size_t;
                    }
                } else if function == Some(do_page_down as unsafe extern "C" fn() -> ())
                {
                    if selected.wrapping_add(piles as libc::c_ulong)
                        >= list_length.wrapping_sub(1 as libc::c_int as libc::c_ulong)
                    {
                        selected = list_length
                            .wrapping_sub(1 as libc::c_int as libc::c_ulong);
                    } else if selected
                        .wrapping_add(usable_rows.wrapping_mul(piles as libc::c_ulong))
                        >= list_length
                    {
                        selected = selected
                            .wrapping_add(
                                usable_rows.wrapping_mul(piles as libc::c_ulong),
                            )
                            .wrapping_sub(list_length)
                            .wrapping_rem(piles as libc::c_ulong)
                            .wrapping_add(list_length)
                            .wrapping_sub(piles as libc::c_ulong);
                    } else {
                        selected = (selected as libc::c_ulong)
                            .wrapping_add(
                                usable_rows.wrapping_mul(piles as libc::c_ulong),
                            ) as size_t as size_t;
                    }
                } else if function == Some(to_first_file as unsafe extern "C" fn() -> ())
                {
                    selected = 0 as libc::c_int as size_t;
                } else if function == Some(to_last_file as unsafe extern "C" fn() -> ())
                {
                    selected = list_length
                        .wrapping_sub(1 as libc::c_int as libc::c_ulong);
                } else if function == Some(goto_dir as unsafe extern "C" fn() -> ()) {
                    if do_prompt(
                        (1 as libc::c_int) << 12 as libc::c_int,
                        b"\0" as *const u8 as *const libc::c_char,
                        0 as *mut *mut linestruct,
                        Some(browser_refresh as unsafe extern "C" fn() -> ()),
                        dcgettext(
                            0 as *const libc::c_char,
                            b"Go To Directory\0" as *const u8 as *const libc::c_char,
                            5 as libc::c_int,
                        ),
                    ) < 0 as libc::c_int
                    {
                        statusbar(
                            dcgettext(
                                0 as *const libc::c_char,
                                b"Cancelled\0" as *const u8 as *const libc::c_char,
                                5 as libc::c_int,
                            ),
                        );
                        continue;
                    } else {
                        path = free_and_assign(path, real_dir_from_tilde(answer));
                        if *path as libc::c_int != '/' as i32 {
                            path = nrealloc(
                                path as *mut libc::c_void,
                                (strlen(present_path))
                                    .wrapping_add(strlen(answer))
                                    .wrapping_add(1 as libc::c_int as libc::c_ulong),
                            ) as *mut libc::c_char;
                            sprintf(
                                path,
                                b"%s%s\0" as *const u8 as *const libc::c_char,
                                present_path,
                                answer,
                            );
                        }
                        if outside_of_confinement(path, 0 as libc::c_int != 0) {
                            statusline(
                                ALERT,
                                dcgettext(
                                    0 as *const libc::c_char,
                                    b"Can't go outside of %s\0" as *const u8
                                        as *const libc::c_char,
                                    5 as libc::c_int,
                                ),
                                operating_dir,
                            );
                            path = mallocstrcpy(path, present_path);
                            continue;
                        } else {
                            while strlen(path) > 1 as libc::c_int as libc::c_ulong
                                && *path
                                    .offset(
                                        (strlen(path))
                                            .wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize,
                                    ) as libc::c_int == '/' as i32
                            {
                                *path
                                    .offset(
                                        (strlen(path))
                                            .wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize,
                                    ) = '\0' as i32 as libc::c_char;
                            }
                            let mut j: size_t = 0 as libc::c_int as size_t;
                            while j < list_length {
                                if strcmp(*filelist.offset(j as isize), path)
                                    == 0 as libc::c_int
                                {
                                    selected = j;
                                }
                                j = j.wrapping_add(1);
                                j;
                            }
                            break;
                        }
                    }
                } else if function == Some(do_enter as unsafe extern "C" fn() -> ()) {
                    let mut st: stat = stat {
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
                    if strcmp(
                        *filelist.offset(selected as isize),
                        b"/..\0" as *const u8 as *const libc::c_char,
                    ) == 0 as libc::c_int
                    {
                        statusline(
                            ALERT,
                            dcgettext(
                                0 as *const libc::c_char,
                                b"Can't move up a directory\0" as *const u8
                                    as *const libc::c_char,
                                5 as libc::c_int,
                            ),
                        );
                        continue;
                    } else if outside_of_confinement(
                        *filelist.offset(selected as isize),
                        0 as libc::c_int != 0,
                    ) {
                        statusline(
                            ALERT,
                            dcgettext(
                                0 as *const libc::c_char,
                                b"Can't go outside of %s\0" as *const u8
                                    as *const libc::c_char,
                                5 as libc::c_int,
                            ),
                            operating_dir,
                        );
                        continue;
                    } else if stat(*filelist.offset(selected as isize), &mut st)
                        == -(1 as libc::c_int)
                    {
                        statusline(
                            ALERT,
                            dcgettext(
                                0 as *const libc::c_char,
                                b"Error reading %s: %s\0" as *const u8
                                    as *const libc::c_char,
                                5 as libc::c_int,
                            ),
                            *filelist.offset(selected as isize),
                            strerror(*__errno_location()),
                        );
                        continue;
                    } else if !(st.st_mode & 0o170000 as libc::c_int as libc::c_uint
                        == 0o40000 as libc::c_int as libc::c_uint)
                    {
                        chosen = copy_of(*filelist.offset(selected as isize));
                        break '_read_directory_contents;
                    } else {
                        if strcmp(
                            tail(*filelist.offset(selected as isize)),
                            b"..\0" as *const u8 as *const libc::c_char,
                        ) == 0 as libc::c_int
                        {
                            present_name = strip_last_component(
                                *filelist.offset(selected as isize),
                            );
                        }
                        path = mallocstrcpy(path, *filelist.offset(selected as isize));
                        break;
                    }
                } else if function
                    == ::std::mem::transmute::<
                        Option::<unsafe extern "C" fn(*const libc::c_char) -> ()>,
                        functionptrtype,
                    >(Some(implant as unsafe extern "C" fn(*const libc::c_char) -> ()))
                {
                    implant(
                        (*first_sc_for(
                            (1 as libc::c_int) << 10 as libc::c_int,
                            function,
                        ))
                            .expansion,
                    );
                } else if !(kbinput == -(2 as libc::c_int)) {
                    if function == Some(do_exit as unsafe extern "C" fn() -> ()) {
                        break '_read_directory_contents;
                    }
                    unbound_key(kbinput);
                }
                if !(kbinput == -(2 as libc::c_int)) {
                    continue;
                }
                present_name = copy_of(*filelist.offset(selected as isize));
                break;
            }
        }
    }
    titlebar(0 as *const libc::c_char);
    edit_refresh();
    rpl_free(path as *mut libc::c_void);
    free_chararray(filelist, list_length);
    filelist = 0 as *mut *mut libc::c_char;
    list_length = 0 as libc::c_int as size_t;
    return chosen;
}
pub unsafe extern "C" fn browse_in(
    mut inpath: *const libc::c_char,
) -> *mut libc::c_char {
    let mut path: *mut libc::c_char = real_dir_from_tilde(inpath);
    let mut fileinfo: stat = stat {
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
    if stat(path, &mut fileinfo) == -(1 as libc::c_int)
        || !(fileinfo.st_mode & 0o170000 as libc::c_int as libc::c_uint
            == 0o40000 as libc::c_int as libc::c_uint)
    {
        path = free_and_assign(path, strip_last_component(path));
        if stat(path, &mut fileinfo) == -(1 as libc::c_int)
            || !(fileinfo.st_mode & 0o170000 as libc::c_int as libc::c_uint
                == 0o40000 as libc::c_int as libc::c_uint)
        {
            path = free_and_assign(
                path,
                realpath(
                    b".\0" as *const u8 as *const libc::c_char,
                    0 as *mut libc::c_char,
                ),
            );
            if path.is_null() {
                statusline(
                    ALERT,
                    dcgettext(
                        0 as *const libc::c_char,
                        b"The working directory has disappeared\0" as *const u8
                            as *const libc::c_char,
                        5 as libc::c_int,
                    ),
                );
                napms(1200 as libc::c_int);
                return 0 as *mut libc::c_char;
            }
        }
    }
    if outside_of_confinement(path, 0 as libc::c_int != 0) {
        path = mallocstrcpy(path, operating_dir);
    }
    return browse(path);
}
