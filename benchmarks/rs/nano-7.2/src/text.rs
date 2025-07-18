use ::libc;
use ::c2rust_bitfields;
extern "C" {
    pub type re_dfa_t;
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    pub type ldat;
    fn beep() -> libc::c_int;
    fn doupdate() -> libc::c_int;
    fn endwin() -> libc::c_int;
    fn napms(_: libc::c_int) -> libc::c_int;
    fn rpl_regexec(
        __preg: *const regex_t,
        __String: *const libc::c_char,
        __nmatch: size_t,
        __pmatch: *mut regmatch_t,
        __eflags: libc::c_int,
    ) -> libc::c_int;
    fn strtol(
        _: *const libc::c_char,
        _: *mut *mut libc::c_char,
        _: libc::c_int,
    ) -> libc::c_long;
    fn exit(_: libc::c_int) -> !;
    fn rpl_free(ptr: *mut libc::c_void);
    fn time(__timer: *mut time_t) -> time_t;
    fn __xstat(
        __ver: libc::c_int,
        __filename: *const libc::c_char,
        __stat_buf: *mut stat,
    ) -> libc::c_int;
    fn sprintf(_: *mut libc::c_char, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn wnoutrefresh(_: *mut WINDOW) -> libc::c_int;
    fn wredrawln(_: *mut WINDOW, _: libc::c_int, _: libc::c_int) -> libc::c_int;
    static mut LINES: libc::c_int;
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
    static mut shift_held: bool;
    static mut ran_a_tool: bool;
    static mut inhelp: bool;
    static mut focusing: bool;
    static mut lastmessage: message_type;
    static mut pletion_line: *mut linestruct;
    static mut answer: *mut libc::c_char;
    static mut flags: [libc::c_uint; 4];
    static mut wrap_at: size_t;
    static mut midwin: *mut WINDOW;
    static mut footwin: *mut WINDOW;
    static mut editwinrows: libc::c_int;
    static mut cutbuffer: *mut linestruct;
    static mut openfile: *mut openfilestruct;
    static mut punct: *mut libc::c_char;
    static mut brackets: *mut libc::c_char;
    static mut quotereg: regex_t;
    static mut tabsize: ssize_t;
    static mut alt_speller: *mut libc::c_char;
    static mut perturbed: bool;
    static mut recook: bool;
    static mut refresh_needed: bool;
    static mut currmenu: libc::c_int;
    static mut spotlighted: bool;
    static mut light_from_col: size_t;
    static mut light_to_col: size_t;
    fn is_blank_char(c: *const libc::c_char) -> bool;
    fn is_word_char(c: *const libc::c_char, allow_punct: bool) -> bool;
    fn char_length(pointer: *const libc::c_char) -> libc::c_int;
    fn mbstrlen(pointer: *const libc::c_char) -> size_t;
    fn advance_over(string: *const libc::c_char, column: *mut size_t) -> libc::c_int;
    fn step_left(buf: *const libc::c_char, pos: size_t) -> size_t;
    fn step_right(buf: *const libc::c_char, pos: size_t) -> size_t;
    fn mbstrchr(
        string: *const libc::c_char,
        chr: *const libc::c_char,
    ) -> *mut libc::c_char;
    fn white_string(string: *const libc::c_char) -> bool;
    fn check_the_multis(line: *mut linestruct);
    fn do_delete();
    fn extract_segment(
        top: *mut linestruct,
        top_x: size_t,
        bot: *mut linestruct,
        bot_x: size_t,
    );
    fn ingraft_buffer(topline: *mut linestruct);
    fn copy_from_buffer(somebuffer: *mut linestruct);
    fn cut_marked_region();
    fn do_snip(marked: bool, until_eof: bool, append: bool);
    fn open_buffer(filename: *const libc::c_char, new_one: bool) -> bool;
    fn set_modified();
    fn read_file(
        f: *mut FILE,
        fd: libc::c_int,
        filename: *const libc::c_char,
        undoable: bool,
    );
    fn open_file(
        filename: *const libc::c_char,
        new_one: bool,
        f: *mut *mut FILE,
    ) -> libc::c_int;
    fn safe_tempfile(stream: *mut *mut FILE) -> *mut libc::c_char;
    fn write_file(
        name: *const libc::c_char,
        thefile: *mut FILE,
        normal: bool,
        method: kind_of_writing_type,
        annotate: bool,
    ) -> bool;
    fn write_region_to_file(
        name: *const libc::c_char,
        stream: *mut FILE,
        normal: bool,
        method: kind_of_writing_type,
    ) -> bool;
    fn write_it_out(exiting: bool, withprompt: bool) -> libc::c_int;
    fn func_from_key(keycode: libc::c_int) -> functionptrtype;
    fn do_help();
    fn do_page_up();
    fn do_page_down();
    fn do_para_begin(line: *mut *mut linestruct);
    fn do_para_end(line: *mut *mut linestruct);
    fn to_prev_block();
    fn to_next_block();
    fn do_next_word(after_ends: bool) -> bool;
    fn make_new_node(prevnode: *mut linestruct) -> *mut linestruct;
    fn splice_node(afterthis: *mut linestruct, newnode: *mut linestruct);
    fn unlink_node(line: *mut linestruct);
    fn copy_buffer(src: *const linestruct) -> *mut linestruct;
    fn free_lines(src: *mut linestruct);
    fn renumber_from(line: *mut linestruct);
    fn in_restricted_mode() -> bool;
    fn die(msg: *const libc::c_char, _: ...);
    fn window_init();
    fn block_sigwinch(blockit: bool);
    fn terminal_init();
    fn confirm_margin();
    fn inject(burst: *mut libc::c_char, count: size_t);
    fn put_cursor_at_end_of_answer();
    fn do_prompt(
        menu: libc::c_int,
        provided: *const libc::c_char,
        history_list: *mut *mut linestruct,
        refresh_func: Option::<unsafe extern "C" fn() -> ()>,
        msg: *const libc::c_char,
        _: ...
    ) -> libc::c_int;
    fn ask_user(withall: bool, question: *const libc::c_char) -> libc::c_int;
    fn findnextstr(
        needle: *const libc::c_char,
        whole_word_only: bool,
        modus: libc::c_int,
        match_len: *mut size_t,
        skipone: bool,
        begin: *const linestruct,
        begin_x: size_t,
    ) -> libc::c_int;
    fn do_replace_loop(
        needle: *const libc::c_char,
        whole_word_only: bool,
        real_current: *const linestruct,
        real_current_x: *mut size_t,
    ) -> ssize_t;
    fn goto_line_posx(line: ssize_t, pos_x: size_t);
    fn statusbar(msg: *const libc::c_char);
    fn nmalloc(howmuch: size_t) -> *mut libc::c_void;
    fn xplustabs() -> size_t;
    fn ensure_firstcolumn_is_aligned();
    fn copy_of(string: *const libc::c_char) -> *mut libc::c_char;
    fn nrealloc(ptr: *mut libc::c_void, howmuch: size_t) -> *mut libc::c_void;
    fn mark_is_before_cursor() -> bool;
    fn measured_copy(string: *const libc::c_char, count: size_t) -> *mut libc::c_char;
    fn free_chararray(array: *mut *mut libc::c_char, len: size_t);
    fn mallocstrcpy(
        dest: *mut libc::c_char,
        src: *const libc::c_char,
    ) -> *mut libc::c_char;
    fn get_range(top: *mut *mut linestruct, bot: *mut *mut linestruct);
    fn statusline(importance: message_type, msg: *const libc::c_char, _: ...);
    fn titlebar(path: *const libc::c_char);
    fn line_from_number(number: ssize_t) -> *mut linestruct;
    fn adjust_viewport(manner: update_type);
    fn remove_magicline();
    fn new_magicline();
    fn wideness(text: *const libc::c_char, maxlen: size_t) -> size_t;
    fn breadth(text: *const libc::c_char) -> size_t;
    fn get_region(
        top: *mut *mut linestruct,
        top_x: *mut size_t,
        bot: *mut *mut linestruct,
        bot_x: *mut size_t,
    );
    fn edit_refresh();
    fn full_refresh();
    fn blank_bottombars();
    fn wipe_statusbar();
    fn do_cancel();
    fn get_kbinput(win: *mut WINDOW, showcursor: bool) -> libc::c_int;
    fn place_the_cursor();
    fn bottombars(menu: libc::c_int);
    fn actual_x(text: *const libc::c_char, column: size_t) -> size_t;
    fn number_of_characters_in(
        begin: *const linestruct,
        end: *const linestruct,
    ) -> size_t;
    fn get_verbatim_kbinput(win: *mut WINDOW, count: *mut size_t) -> *mut libc::c_char;
    fn edit_scroll(direction: bool);
    fn __errno_location() -> *mut libc::c_int;
    fn open(__file: *const libc::c_char, __oflag: libc::c_int, _: ...) -> libc::c_int;
    fn memcpy(
        _: *mut libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn memmove(
        _: *mut libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn memset(
        _: *mut libc::c_void,
        _: libc::c_int,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn strcpy(_: *mut libc::c_char, _: *const libc::c_char) -> *mut libc::c_char;
    fn strncpy(
        _: *mut libc::c_char,
        _: *const libc::c_char,
        _: libc::c_ulong,
    ) -> *mut libc::c_char;
    fn strcat(_: *mut libc::c_char, _: *const libc::c_char) -> *mut libc::c_char;
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    fn strncmp(
        _: *const libc::c_char,
        _: *const libc::c_char,
        _: libc::c_ulong,
    ) -> libc::c_int;
    fn strchr(_: *const libc::c_char, _: libc::c_int) -> *mut libc::c_char;
    fn strstr(_: *const libc::c_char, _: *const libc::c_char) -> *mut libc::c_char;
    fn strtok(_: *mut libc::c_char, _: *const libc::c_char) -> *mut libc::c_char;
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    fn strerror(_: libc::c_int) -> *mut libc::c_char;
    fn close(__fd: libc::c_int) -> libc::c_int;
    fn read(__fd: libc::c_int, __buf: *mut libc::c_void, __nbytes: size_t) -> ssize_t;
    fn pipe(__pipedes: *mut libc::c_int) -> libc::c_int;
    fn dup2(__fd: libc::c_int, __fd2: libc::c_int) -> libc::c_int;
    fn execvp(
        __file: *const libc::c_char,
        __argv: *const *mut libc::c_char,
    ) -> libc::c_int;
    fn execlp(
        __file: *const libc::c_char,
        __arg: *const libc::c_char,
        _: ...
    ) -> libc::c_int;
    fn fpathconf(__fd: libc::c_int, __name: libc::c_int) -> libc::c_long;
    fn fork() -> __pid_t;
    fn unlink(__name: *const libc::c_char) -> libc::c_int;
    fn wait(__stat_loc: *mut libc::c_int) -> __pid_t;
    fn waitpid(
        __pid: __pid_t,
        __stat_loc: *mut libc::c_int,
        __options: libc::c_int,
    ) -> __pid_t;
}
pub type __dev_t = libc::c_ulong;
pub type __uid_t = libc::c_uint;
pub type __gid_t = libc::c_uint;
pub type __ino_t = libc::c_ulong;
pub type __mode_t = libc::c_uint;
pub type __nlink_t = libc::c_ulong;
pub type __off_t = libc::c_long;
pub type __off64_t = libc::c_long;
pub type __pid_t = libc::c_int;
pub type __time_t = libc::c_long;
pub type __blksize_t = libc::c_long;
pub type __blkcnt_t = libc::c_long;
pub type __ssize_t = libc::c_long;
pub type __syscall_slong_t = libc::c_long;
pub type pid_t = __pid_t;
pub type ssize_t = __ssize_t;
pub type time_t = __time_t;
pub type size_t = libc::c_ulong;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct timespec {
    pub tv_sec: __time_t,
    pub tv_nsec: __syscall_slong_t,
}
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
pub type kind_of_writing_type = libc::c_uint;
pub const PREPEND: kind_of_writing_type = 2;
pub const APPEND: kind_of_writing_type = 1;
pub const OVERWRITE: kind_of_writing_type = 0;
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
pub struct lintstruct {
    pub lineno: ssize_t,
    pub colno: ssize_t,
    pub msg: *mut libc::c_char,
    pub filename: *mut libc::c_char,
    pub next: *mut lintstruct,
    pub prev: *mut lintstruct,
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
pub struct completionstruct {
    pub word: *mut libc::c_char,
    pub next: *mut completionstruct,
}
pub type functionptrtype = Option::<unsafe extern "C" fn() -> ()>;
pub const _PC_PIPE_BUF: C2RustUnnamed_1 = 5;
pub type C2RustUnnamed_1 = libc::c_uint;
pub const _PC_2_SYMLINKS: C2RustUnnamed_1 = 20;
pub const _PC_SYMLINK_MAX: C2RustUnnamed_1 = 19;
pub const _PC_ALLOC_SIZE_MIN: C2RustUnnamed_1 = 18;
pub const _PC_REC_XFER_ALIGN: C2RustUnnamed_1 = 17;
pub const _PC_REC_MIN_XFER_SIZE: C2RustUnnamed_1 = 16;
pub const _PC_REC_MAX_XFER_SIZE: C2RustUnnamed_1 = 15;
pub const _PC_REC_INCR_XFER_SIZE: C2RustUnnamed_1 = 14;
pub const _PC_FILESIZEBITS: C2RustUnnamed_1 = 13;
pub const _PC_SOCK_MAXBUF: C2RustUnnamed_1 = 12;
pub const _PC_PRIO_IO: C2RustUnnamed_1 = 11;
pub const _PC_ASYNC_IO: C2RustUnnamed_1 = 10;
pub const _PC_SYNC_IO: C2RustUnnamed_1 = 9;
pub const _PC_VDISABLE: C2RustUnnamed_1 = 8;
pub const _PC_NO_TRUNC: C2RustUnnamed_1 = 7;
pub const _PC_CHOWN_RESTRICTED: C2RustUnnamed_1 = 6;
pub const _PC_PATH_MAX: C2RustUnnamed_1 = 4;
pub const _PC_NAME_MAX: C2RustUnnamed_1 = 3;
pub const _PC_MAX_INPUT: C2RustUnnamed_1 = 2;
pub const _PC_MAX_CANON: C2RustUnnamed_1 = 1;
pub const _PC_LINK_MAX: C2RustUnnamed_1 = 0;
#[inline]
unsafe extern "C" fn stat(
    mut __path: *const libc::c_char,
    mut __statbuf: *mut stat,
) -> libc::c_int {
    return __xstat(1 as libc::c_int, __path, __statbuf);
}
static mut pletion_x: libc::c_int = 0 as libc::c_int;
static mut list_of_completions: *mut completionstruct = 0 as *const completionstruct
    as *mut completionstruct;
pub unsafe extern "C" fn do_mark() {
    if ((*openfile).mark).is_null() {
        (*openfile).mark = (*openfile).current;
        (*openfile).mark_x = (*openfile).current_x;
        (*openfile).softmark = 0 as libc::c_int != 0;
        statusbar(
            dcgettext(
                0 as *const libc::c_char,
                b"Mark Set\0" as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ),
        );
    } else {
        (*openfile).mark = 0 as *mut linestruct;
        statusbar(
            dcgettext(
                0 as *const libc::c_char,
                b"Mark Unset\0" as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ),
        );
        refresh_needed = 1 as libc::c_int != 0;
    };
}
pub unsafe extern "C" fn do_tab() {
    if !((*openfile).syntax).is_null() && !((*(*openfile).syntax).tab).is_null() {
        inject((*(*openfile).syntax).tab, strlen((*(*openfile).syntax).tab));
    } else if flags[(TABS_TO_SPACES as libc::c_int as libc::c_ulong)
        .wrapping_div(
            (::std::mem::size_of::<libc::c_uint>() as libc::c_ulong)
                .wrapping_mul(8 as libc::c_int as libc::c_ulong),
        ) as usize]
        & (1 as libc::c_int as libc::c_uint)
            << (TABS_TO_SPACES as libc::c_int as libc::c_ulong)
                .wrapping_rem(
                    (::std::mem::size_of::<libc::c_uint>() as libc::c_ulong)
                        .wrapping_mul(8 as libc::c_int as libc::c_ulong),
                ) != 0 as libc::c_int as libc::c_uint
    {
        let mut spaces: *mut libc::c_char = nmalloc(
            (tabsize + 1 as libc::c_int as libc::c_long) as size_t,
        ) as *mut libc::c_char;
        let mut length: size_t = (tabsize as libc::c_ulong)
            .wrapping_sub((xplustabs()).wrapping_rem(tabsize as libc::c_ulong));
        memset(spaces as *mut libc::c_void, ' ' as i32, length);
        *spaces.offset(length as isize) = '\0' as i32 as libc::c_char;
        inject(spaces, length);
        rpl_free(spaces as *mut libc::c_void);
    } else {
        inject(
            b"\t\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            1 as libc::c_int as size_t,
        );
    };
}
pub unsafe extern "C" fn indent_a_line(
    mut line: *mut linestruct,
    mut indentation: *mut libc::c_char,
) {
    let mut length: size_t = strlen((*line).data);
    let mut indent_len: size_t = strlen(indentation);
    if indent_len == 0 as libc::c_int as libc::c_ulong {
        return;
    }
    (*line)
        .data = nrealloc(
        (*line).data as *mut libc::c_void,
        length.wrapping_add(indent_len).wrapping_add(1 as libc::c_int as libc::c_ulong),
    ) as *mut libc::c_char;
    memmove(
        ((*line).data).offset(indent_len as isize) as *mut libc::c_void,
        (*line).data as *const libc::c_void,
        length.wrapping_add(1 as libc::c_int as libc::c_ulong),
    );
    memcpy(
        (*line).data as *mut libc::c_void,
        indentation as *const libc::c_void,
        indent_len,
    );
    (*openfile)
        .totsize = ((*openfile).totsize as libc::c_ulong).wrapping_add(indent_len)
        as size_t as size_t;
    if line == (*openfile).mark && (*openfile).mark_x > 0 as libc::c_int as libc::c_ulong
    {
        (*openfile)
            .mark_x = ((*openfile).mark_x as libc::c_ulong).wrapping_add(indent_len)
            as size_t as size_t;
    }
    if line == (*openfile).current
        && (*openfile).current_x > 0 as libc::c_int as libc::c_ulong
    {
        (*openfile)
            .current_x = ((*openfile).current_x as libc::c_ulong)
            .wrapping_add(indent_len) as size_t as size_t;
        (*openfile).placewewant = xplustabs();
    }
}
pub unsafe extern "C" fn do_indent() {
    let mut top: *mut linestruct = 0 as *mut linestruct;
    let mut bot: *mut linestruct = 0 as *mut linestruct;
    let mut line: *mut linestruct = 0 as *mut linestruct;
    let mut indentation: *mut libc::c_char = 0 as *mut libc::c_char;
    get_range(&mut top, &mut bot);
    while top != (*bot).next
        && *((*top).data).offset(0 as libc::c_int as isize) as libc::c_int == '\0' as i32
    {
        top = (*top).next;
    }
    if top == (*bot).next {
        return;
    }
    indentation = nmalloc((tabsize + 1 as libc::c_int as libc::c_long) as size_t)
        as *mut libc::c_char;
    if !((*openfile).syntax).is_null() && !((*(*openfile).syntax).tab).is_null() {
        indentation = mallocstrcpy(indentation, (*(*openfile).syntax).tab);
    } else if flags[(TABS_TO_SPACES as libc::c_int as libc::c_ulong)
        .wrapping_div(
            (::std::mem::size_of::<libc::c_uint>() as libc::c_ulong)
                .wrapping_mul(8 as libc::c_int as libc::c_ulong),
        ) as usize]
        & (1 as libc::c_int as libc::c_uint)
            << (TABS_TO_SPACES as libc::c_int as libc::c_ulong)
                .wrapping_rem(
                    (::std::mem::size_of::<libc::c_uint>() as libc::c_ulong)
                        .wrapping_mul(8 as libc::c_int as libc::c_ulong),
                ) != 0 as libc::c_int as libc::c_uint
    {
        memset(indentation as *mut libc::c_void, ' ' as i32, tabsize as libc::c_ulong);
        *indentation.offset(tabsize as isize) = '\0' as i32 as libc::c_char;
    } else {
        *indentation.offset(0 as libc::c_int as isize) = '\t' as i32 as libc::c_char;
        *indentation.offset(1 as libc::c_int as isize) = '\0' as i32 as libc::c_char;
    }
    add_undo(INDENT, 0 as *const libc::c_char);
    line = top;
    while line != (*bot).next {
        let mut real_indent: *mut libc::c_char = (if *((*line).data)
            .offset(0 as libc::c_int as isize) as libc::c_int == '\0' as i32
        {
            b"\0" as *const u8 as *const libc::c_char
        } else {
            indentation as *const libc::c_char
        }) as *mut libc::c_char;
        indent_a_line(line, real_indent);
        update_multiline_undo((*line).lineno, real_indent);
        line = (*line).next;
    }
    rpl_free(indentation as *mut libc::c_void);
    set_modified();
    ensure_firstcolumn_is_aligned();
    refresh_needed = 1 as libc::c_int != 0;
    shift_held = 1 as libc::c_int != 0;
}
pub unsafe extern "C" fn length_of_white(mut text: *const libc::c_char) -> size_t {
    let mut white_count: size_t = 0 as libc::c_int as size_t;
    if !((*openfile).syntax).is_null() && !((*(*openfile).syntax).tab).is_null() {
        let mut thelength: size_t = strlen((*(*openfile).syntax).tab);
        while *text.offset(white_count as isize) as libc::c_int
            == *((*(*openfile).syntax).tab).offset(white_count as isize) as libc::c_int
        {
            white_count = white_count.wrapping_add(1);
            if white_count == thelength {
                return thelength;
            }
        }
        white_count = 0 as libc::c_int as size_t;
    }
    loop {
        if *text as libc::c_int == '\t' as i32 {
            return white_count.wrapping_add(1 as libc::c_int as libc::c_ulong);
        }
        if *text as libc::c_int != ' ' as i32 {
            return white_count;
        }
        white_count = white_count.wrapping_add(1);
        if white_count == tabsize as libc::c_ulong {
            return tabsize as size_t;
        }
        text = text.offset(1);
        text;
    };
}
pub unsafe extern "C" fn compensate_leftward(
    mut line: *mut linestruct,
    mut leftshift: size_t,
) {
    if line == (*openfile).mark {
        if (*openfile).mark_x < leftshift {
            (*openfile).mark_x = 0 as libc::c_int as size_t;
        } else {
            (*openfile)
                .mark_x = ((*openfile).mark_x as libc::c_ulong).wrapping_sub(leftshift)
                as size_t as size_t;
        }
    }
    if line == (*openfile).current {
        if (*openfile).current_x < leftshift {
            (*openfile).current_x = 0 as libc::c_int as size_t;
        } else {
            (*openfile)
                .current_x = ((*openfile).current_x as libc::c_ulong)
                .wrapping_sub(leftshift) as size_t as size_t;
        }
        (*openfile).placewewant = xplustabs();
    }
}
pub unsafe extern "C" fn unindent_a_line(
    mut line: *mut linestruct,
    mut indent_len: size_t,
) {
    let mut length: size_t = strlen((*line).data);
    if indent_len == 0 as libc::c_int as libc::c_ulong {
        return;
    }
    memmove(
        (*line).data as *mut libc::c_void,
        ((*line).data).offset(indent_len as isize) as *const libc::c_void,
        length.wrapping_sub(indent_len).wrapping_add(1 as libc::c_int as libc::c_ulong),
    );
    (*openfile)
        .totsize = ((*openfile).totsize as libc::c_ulong).wrapping_sub(indent_len)
        as size_t as size_t;
    compensate_leftward(line, indent_len);
}
pub unsafe extern "C" fn do_unindent() {
    let mut top: *mut linestruct = 0 as *mut linestruct;
    let mut bot: *mut linestruct = 0 as *mut linestruct;
    let mut line: *mut linestruct = 0 as *mut linestruct;
    get_range(&mut top, &mut bot);
    while top != (*bot).next
        && length_of_white((*top).data) == 0 as libc::c_int as libc::c_ulong
    {
        top = (*top).next;
    }
    if top == (*bot).next {
        return;
    }
    add_undo(UNINDENT, 0 as *const libc::c_char);
    line = top;
    while line != (*bot).next {
        let mut indent_len: size_t = length_of_white((*line).data);
        let mut indentation: *mut libc::c_char = measured_copy((*line).data, indent_len);
        unindent_a_line(line, indent_len);
        update_multiline_undo((*line).lineno, indentation);
        rpl_free(indentation as *mut libc::c_void);
        line = (*line).next;
    }
    set_modified();
    ensure_firstcolumn_is_aligned();
    refresh_needed = 1 as libc::c_int != 0;
    shift_held = 1 as libc::c_int != 0;
}
pub unsafe extern "C" fn handle_indent_action(
    mut u: *mut undostruct,
    mut undoing: bool,
    mut add_indent: bool,
) {
    let mut group: *mut groupstruct = (*u).grouping;
    let mut line: *mut linestruct = line_from_number((*group).top_line);
    if !undoing {
        goto_line_posx((*u).head_lineno, (*u).head_x);
    }
    while !line.is_null() && (*line).lineno <= (*group).bottom_line {
        let mut blanks: *mut libc::c_char = *((*group).indentations)
            .offset(((*line).lineno - (*group).top_line) as isize);
        if undoing as libc::c_int ^ add_indent as libc::c_int != 0 {
            indent_a_line(line, blanks);
        } else {
            unindent_a_line(line, strlen(blanks));
        }
        line = (*line).next;
    }
    if undoing {
        goto_line_posx((*u).head_lineno, (*u).head_x);
    }
    refresh_needed = 1 as libc::c_int != 0;
}
pub unsafe extern "C" fn comment_line(
    mut action: undo_type,
    mut line: *mut linestruct,
    mut comment_seq: *const libc::c_char,
) -> bool {
    let mut comment_seq_len: size_t = strlen(comment_seq);
    let mut post_seq: *const libc::c_char = strchr(comment_seq, '|' as i32);
    let mut pre_len: size_t = if !post_seq.is_null() {
        let fresh0 = post_seq;
        post_seq = post_seq.offset(1);
        fresh0.offset_from(comment_seq) as libc::c_long as libc::c_ulong
    } else {
        comment_seq_len
    };
    let mut post_len: size_t = if !post_seq.is_null() {
        comment_seq_len
            .wrapping_sub(pre_len)
            .wrapping_sub(1 as libc::c_int as libc::c_ulong)
    } else {
        0 as libc::c_int as libc::c_ulong
    };
    let mut line_len: size_t = strlen((*line).data);
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
                ) != 0 as libc::c_int as libc::c_uint) && line == (*openfile).filebot
    {
        return 0 as libc::c_int != 0;
    }
    if action as libc::c_uint == COMMENT as libc::c_int as libc::c_uint {
        (*line)
            .data = nrealloc(
            (*line).data as *mut libc::c_void,
            line_len
                .wrapping_add(pre_len)
                .wrapping_add(post_len)
                .wrapping_add(1 as libc::c_int as libc::c_ulong),
        ) as *mut libc::c_char;
        memmove(
            ((*line).data).offset(pre_len as isize) as *mut libc::c_void,
            (*line).data as *const libc::c_void,
            line_len.wrapping_add(1 as libc::c_int as libc::c_ulong),
        );
        memmove(
            (*line).data as *mut libc::c_void,
            comment_seq as *const libc::c_void,
            pre_len,
        );
        if post_len > 0 as libc::c_int as libc::c_ulong {
            memmove(
                ((*line).data).offset(pre_len as isize).offset(line_len as isize)
                    as *mut libc::c_void,
                post_seq as *const libc::c_void,
                post_len.wrapping_add(1 as libc::c_int as libc::c_ulong),
            );
        }
        (*openfile)
            .totsize = ((*openfile).totsize as libc::c_ulong)
            .wrapping_add(pre_len.wrapping_add(post_len)) as size_t as size_t;
        if line == (*openfile).mark
            && (*openfile).mark_x > 0 as libc::c_int as libc::c_ulong
        {
            (*openfile)
                .mark_x = ((*openfile).mark_x as libc::c_ulong).wrapping_add(pre_len)
                as size_t as size_t;
        }
        if line == (*openfile).current
            && (*openfile).current_x > 0 as libc::c_int as libc::c_ulong
        {
            (*openfile)
                .current_x = ((*openfile).current_x as libc::c_ulong)
                .wrapping_add(pre_len) as size_t as size_t;
            (*openfile).placewewant = xplustabs();
        }
        return 1 as libc::c_int != 0;
    }
    if strncmp((*line).data, comment_seq, pre_len) == 0 as libc::c_int
        && (post_len == 0 as libc::c_int as libc::c_ulong
            || strcmp(
                ((*line).data).offset(line_len as isize).offset(-(post_len as isize)),
                post_seq,
            ) == 0 as libc::c_int)
    {
        if action as libc::c_uint == PREFLIGHT as libc::c_int as libc::c_uint {
            return 1 as libc::c_int != 0;
        }
        memmove(
            (*line).data as *mut libc::c_void,
            ((*line).data).offset(pre_len as isize) as *const libc::c_void,
            line_len.wrapping_sub(pre_len),
        );
        *((*line).data)
            .offset(
                line_len.wrapping_sub(pre_len).wrapping_sub(post_len) as isize,
            ) = '\0' as i32 as libc::c_char;
        (*openfile)
            .totsize = ((*openfile).totsize as libc::c_ulong)
            .wrapping_sub(pre_len.wrapping_add(post_len)) as size_t as size_t;
        compensate_leftward(line, pre_len);
        return 1 as libc::c_int != 0;
    }
    return 0 as libc::c_int != 0;
}
pub unsafe extern "C" fn do_comment() {
    let mut comment_seq: *const libc::c_char = b"#\0" as *const u8
        as *const libc::c_char;
    let mut action: undo_type = UNCOMMENT;
    let mut top: *mut linestruct = 0 as *mut linestruct;
    let mut bot: *mut linestruct = 0 as *mut linestruct;
    let mut line: *mut linestruct = 0 as *mut linestruct;
    let mut empty: bool = false;
    let mut all_empty: bool = 1 as libc::c_int != 0;
    if !((*openfile).syntax).is_null() {
        comment_seq = (*(*openfile).syntax).comment;
    }
    if *comment_seq as libc::c_int == '\0' as i32 {
        statusline(
            AHEM,
            dcgettext(
                0 as *const libc::c_char,
                b"Commenting is not supported for this file type\0" as *const u8
                    as *const libc::c_char,
                5 as libc::c_int,
            ),
        );
        return;
    }
    get_range(&mut top, &mut bot);
    if top == bot && bot == (*openfile).filebot
        && !(flags[(NO_NEWLINES as libc::c_int as libc::c_ulong)
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
    {
        statusline(
            AHEM,
            dcgettext(
                0 as *const libc::c_char,
                b"Cannot comment past end of file\0" as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ),
        );
        return;
    }
    line = top;
    while line != (*bot).next {
        empty = white_string((*line).data);
        if !empty && !comment_line(PREFLIGHT, line, comment_seq) {
            action = COMMENT;
            break;
        } else {
            all_empty = all_empty as libc::c_int != 0 && empty as libc::c_int != 0;
            line = (*line).next;
        }
    }
    action = (if all_empty as libc::c_int != 0 {
        COMMENT as libc::c_int as libc::c_uint
    } else {
        action as libc::c_uint
    }) as undo_type;
    add_undo(action, 0 as *const libc::c_char);
    (*(*openfile).current_undo).strdata = copy_of(comment_seq);
    line = top;
    while line != (*bot).next {
        if comment_line(action, line, comment_seq) {
            update_multiline_undo(
                (*line).lineno,
                b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            );
        }
        line = (*line).next;
    }
    set_modified();
    ensure_firstcolumn_is_aligned();
    refresh_needed = 1 as libc::c_int != 0;
    shift_held = 1 as libc::c_int != 0;
}
pub unsafe extern "C" fn handle_comment_action(
    mut u: *mut undostruct,
    mut undoing: bool,
    mut add_comment: bool,
) {
    let mut group: *mut groupstruct = (*u).grouping;
    if !undoing {
        goto_line_posx((*u).head_lineno, (*u).head_x);
    }
    while !group.is_null() {
        let mut line: *mut linestruct = line_from_number((*group).top_line);
        while !line.is_null() && (*line).lineno <= (*group).bottom_line {
            comment_line(
                (if undoing as libc::c_int ^ add_comment as libc::c_int != 0 {
                    COMMENT as libc::c_int
                } else {
                    UNCOMMENT as libc::c_int
                }) as undo_type,
                line,
                (*u).strdata,
            );
            line = (*line).next;
        }
        group = (*group).next;
    }
    if undoing {
        goto_line_posx((*u).head_lineno, (*u).head_x);
    }
    refresh_needed = 1 as libc::c_int != 0;
}
pub unsafe extern "C" fn undo_cut(mut u: *mut undostruct) {
    goto_line_posx(
        (*u).head_lineno,
        if (*u).xflags & (1 as libc::c_int) << 2 as libc::c_int != 0 {
            0 as libc::c_int as libc::c_ulong
        } else {
            (*u).head_x
        },
    );
    if (*u).xflags & (1 as libc::c_int) << 6 as libc::c_int == 0 {
        (*(*openfile).current).has_anchor = 0 as libc::c_int != 0;
    }
    if !((*u).cutbuffer).is_null() {
        copy_from_buffer((*u).cutbuffer);
    }
    if (*u).xflags & (1 as libc::c_int) << 3 as libc::c_int != 0
        && !(flags[(NO_NEWLINES as libc::c_int as libc::c_ulong)
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
        && (*openfile).filebot != (*openfile).current
        && *((*(*(*openfile).filebot).prev).data).offset(0 as libc::c_int as isize)
            as libc::c_int == '\0' as i32
    {
        remove_magicline();
    }
    if (*u).xflags & (1 as libc::c_int) << 5 as libc::c_int != 0 {
        goto_line_posx((*u).head_lineno, (*u).head_x);
    }
}
pub unsafe extern "C" fn redo_cut(mut u: *mut undostruct) {
    let mut oldcutbuffer: *mut linestruct = cutbuffer;
    cutbuffer = 0 as *mut linestruct;
    (*openfile).mark = line_from_number((*u).head_lineno);
    (*openfile)
        .mark_x = if (*u).xflags & (1 as libc::c_int) << 2 as libc::c_int != 0 {
        0 as libc::c_int as libc::c_ulong
    } else {
        (*u).head_x
    };
    goto_line_posx((*u).tail_lineno, (*u).tail_x);
    do_snip(
        1 as libc::c_int != 0,
        0 as libc::c_int != 0,
        (*u).type_0 as libc::c_uint == ZAP as libc::c_int as libc::c_uint,
    );
    free_lines(cutbuffer);
    cutbuffer = oldcutbuffer;
}
pub unsafe extern "C" fn do_undo() {
    let mut u: *mut undostruct = (*openfile).current_undo;
    let mut oldcutbuffer: *mut linestruct = 0 as *mut linestruct;
    let mut intruder: *mut linestruct = 0 as *mut linestruct;
    let mut line: *mut linestruct = 0 as *mut linestruct;
    let mut original_x: size_t = 0;
    let mut regain_from_x: size_t = 0;
    let mut undidmsg: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut data: *mut libc::c_char = 0 as *mut libc::c_char;
    if u.is_null() {
        statusline(
            AHEM,
            dcgettext(
                0 as *const libc::c_char,
                b"Nothing to undo\0" as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ),
        );
        return;
    }
    if (*u).type_0 as libc::c_uint <= REPLACE as libc::c_int as libc::c_uint {
        line = line_from_number((*u).tail_lineno);
    }
    match (*u).type_0 as libc::c_uint {
        0 => {
            undidmsg = dcgettext(
                0 as *const libc::c_char,
                b"addition\0" as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            );
            if (*u).xflags & (1 as libc::c_int) << 3 as libc::c_int != 0
                && !(flags[(NO_NEWLINES as libc::c_int as libc::c_ulong)
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
            {
                remove_magicline();
            }
            memmove(
                ((*line).data).offset((*u).head_x as isize) as *mut libc::c_void,
                ((*line).data)
                    .offset((*u).head_x as isize)
                    .offset(strlen((*u).strdata) as isize) as *const libc::c_void,
                (strlen(((*line).data).offset((*u).head_x as isize)))
                    .wrapping_sub(strlen((*u).strdata))
                    .wrapping_add(1 as libc::c_int as libc::c_ulong),
            );
            goto_line_posx((*u).head_lineno, (*u).head_x);
        }
        1 => {
            undidmsg = dcgettext(
                0 as *const libc::c_char,
                b"line break\0" as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            );
            original_x = if (*u).head_x == 0 as libc::c_int as libc::c_ulong {
                (*u).tail_x
            } else {
                (*u).head_x
            };
            regain_from_x = if (*u).head_x == 0 as libc::c_int as libc::c_ulong {
                0 as libc::c_int as libc::c_ulong
            } else {
                (*u).tail_x
            };
            (*line)
                .data = nrealloc(
                (*line).data as *mut libc::c_void,
                (strlen((*line).data))
                    .wrapping_add(
                        strlen(&mut *((*u).strdata).offset(regain_from_x as isize)),
                    )
                    .wrapping_add(1 as libc::c_int as libc::c_ulong),
            ) as *mut libc::c_char;
            strcat((*line).data, &mut *((*u).strdata).offset(regain_from_x as isize));
            (*line)
                .has_anchor = ((*line).has_anchor as libc::c_int
                | (*(*line).next).has_anchor as libc::c_int) != 0;
            unlink_node((*line).next);
            renumber_from(line);
            (*openfile).current = line;
            goto_line_posx((*u).head_lineno, original_x);
        }
        2 | 3 => {
            undidmsg = dcgettext(
                0 as *const libc::c_char,
                b"deletion\0" as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            );
            data = nmalloc(
                (strlen((*line).data))
                    .wrapping_add(strlen((*u).strdata))
                    .wrapping_add(1 as libc::c_int as libc::c_ulong),
            ) as *mut libc::c_char;
            strncpy(data, (*line).data, (*u).head_x);
            strcpy(&mut *data.offset((*u).head_x as isize), (*u).strdata);
            strcpy(
                &mut *data
                    .offset(
                        ((*u).head_x)
                            .wrapping_add(
                                (strlen
                                    as unsafe extern "C" fn(
                                        *const libc::c_char,
                                    ) -> libc::c_ulong)((*u).strdata),
                            ) as isize,
                    ),
                &mut *((*line).data).offset((*u).head_x as isize),
            );
            rpl_free((*line).data as *mut libc::c_void);
            (*line).data = data;
            goto_line_posx((*u).tail_lineno, (*u).tail_x);
        }
        4 => {
            undidmsg = dcgettext(
                0 as *const libc::c_char,
                b"line join\0" as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            );
            if (*u).xflags & (1 as libc::c_int) << 1 as libc::c_int != 0
                && !(flags[(NO_NEWLINES as libc::c_int as libc::c_ulong)
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
            {
                (*openfile).current = (*openfile).filebot;
                (*openfile).current_x = 0 as libc::c_int as size_t;
            } else {
                *((*line).data)
                    .offset((*u).tail_x as isize) = '\0' as i32 as libc::c_char;
                intruder = make_new_node(line);
                (*intruder).data = copy_of((*u).strdata);
                splice_node(line, intruder);
                renumber_from(intruder);
                goto_line_posx((*u).head_lineno, (*u).head_x);
            }
        }
        5 => {
            undidmsg = dcgettext(
                0 as *const libc::c_char,
                b"replacement\0" as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            );
            if (*u).xflags & (1 as libc::c_int) << 3 as libc::c_int != 0
                && !(flags[(NO_NEWLINES as libc::c_int as libc::c_ulong)
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
            {
                remove_magicline();
            }
            data = (*u).strdata;
            (*u).strdata = (*line).data;
            (*line).data = data;
            goto_line_posx((*u).head_lineno, (*u).head_x);
        }
        6 => {
            undidmsg = dcgettext(
                0 as *const libc::c_char,
                b"addition\0" as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            );
        }
        7 => {
            (*openfile).current_undo = (*(*openfile).current_undo).next;
            while (*(*openfile).current_undo).type_0 as libc::c_uint
                != SPLIT_BEGIN as libc::c_int as libc::c_uint
            {
                do_undo();
            }
            u = (*openfile).current_undo;
        }
        13 => {
            undidmsg = dcgettext(
                0 as *const libc::c_char,
                b"erasure\0" as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            );
            undo_cut(u);
        }
        15 | 14 => {
            undidmsg = dcgettext(
                0 as *const libc::c_char,
                b"cut\0" as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            );
            undo_cut(u);
        }
        17 => {
            undidmsg = dcgettext(
                0 as *const libc::c_char,
                b"paste\0" as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            );
            redo_cut(u);
            if (*u).xflags & (1 as libc::c_int) << 3 as libc::c_int != 0
                && !(flags[(NO_NEWLINES as libc::c_int as libc::c_ulong)
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
                && (*openfile).filebot != (*openfile).current
            {
                remove_magicline();
            }
        }
        18 => {
            undidmsg = dcgettext(
                0 as *const libc::c_char,
                b"insertion\0" as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            );
            oldcutbuffer = cutbuffer;
            cutbuffer = 0 as *mut linestruct;
            goto_line_posx((*u).head_lineno, (*u).head_x);
            (*openfile).mark = line_from_number((*u).tail_lineno);
            (*openfile).mark_x = (*u).tail_x;
            cut_marked_region();
            (*u).cutbuffer = cutbuffer;
            cutbuffer = oldcutbuffer;
            if (*u).xflags & (1 as libc::c_int) << 3 as libc::c_int != 0
                && !(flags[(NO_NEWLINES as libc::c_int as libc::c_ulong)
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
                && (*openfile).filebot != (*openfile).current
            {
                remove_magicline();
            }
        }
        19 => {
            undidmsg = (*u).strdata;
            goto_line_posx((*u).head_lineno, (*u).head_x);
            (*openfile).current_y = (*u).tail_lineno;
            adjust_viewport(STATIONARY);
        }
        20 => {
            (*(*openfile).current_undo).head_lineno = (*openfile).current_y;
            (*openfile).current_undo = (*(*openfile).current_undo).next;
            do_undo();
            do_undo();
            do_undo();
            return;
        }
        8 => {
            handle_indent_action(u, 1 as libc::c_int != 0, 1 as libc::c_int != 0);
            undidmsg = dcgettext(
                0 as *const libc::c_char,
                b"indent\0" as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            );
        }
        9 => {
            handle_indent_action(u, 1 as libc::c_int != 0, 0 as libc::c_int != 0);
            undidmsg = dcgettext(
                0 as *const libc::c_char,
                b"unindent\0" as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            );
        }
        10 => {
            handle_comment_action(u, 1 as libc::c_int != 0, 1 as libc::c_int != 0);
            undidmsg = dcgettext(
                0 as *const libc::c_char,
                b"comment\0" as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            );
        }
        11 => {
            handle_comment_action(u, 1 as libc::c_int != 0, 0 as libc::c_int != 0);
            undidmsg = dcgettext(
                0 as *const libc::c_char,
                b"uncomment\0" as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            );
        }
        _ => {}
    }
    if !undidmsg.is_null()
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
                    ) != 0 as libc::c_int as libc::c_uint) && pletion_line.is_null()
    {
        statusline(
            HUSH,
            dcgettext(
                0 as *const libc::c_char,
                b"Undid %s\0" as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ),
            undidmsg,
        );
    }
    (*openfile).current_undo = (*(*openfile).current_undo).next;
    (*openfile).last_action = OTHER;
    (*openfile).mark = 0 as *mut linestruct;
    (*openfile).placewewant = xplustabs();
    (*openfile).totsize = (*u).wassize;
    if (*u).type_0 as libc::c_uint <= REPLACE as libc::c_int as libc::c_uint {
        check_the_multis((*openfile).current);
    } else if (*u).type_0 as libc::c_uint == INSERT as libc::c_int as libc::c_uint {
        perturbed = 1 as libc::c_int != 0;
    }
    if (*openfile).current_undo == (*openfile).last_saved {
        (*openfile).modified = 0 as libc::c_int != 0;
        titlebar(0 as *const libc::c_char);
    } else {
        set_modified();
    };
}
pub unsafe extern "C" fn do_redo() {
    let mut u: *mut undostruct = (*openfile).undotop;
    let mut suppress_modification: bool = 0 as libc::c_int != 0;
    let mut line: *mut linestruct = 0 as *mut linestruct;
    let mut intruder: *mut linestruct = 0 as *mut linestruct;
    let mut redidmsg: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut data: *mut libc::c_char = 0 as *mut libc::c_char;
    if u.is_null() || u == (*openfile).current_undo {
        statusline(
            AHEM,
            dcgettext(
                0 as *const libc::c_char,
                b"Nothing to redo\0" as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ),
        );
        return;
    }
    while (*u).next != (*openfile).current_undo {
        u = (*u).next;
    }
    if (*u).type_0 as libc::c_uint <= REPLACE as libc::c_int as libc::c_uint {
        line = line_from_number((*u).tail_lineno);
    }
    match (*u).type_0 as libc::c_uint {
        0 => {
            redidmsg = dcgettext(
                0 as *const libc::c_char,
                b"addition\0" as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            );
            if (*u).xflags & (1 as libc::c_int) << 3 as libc::c_int != 0
                && !(flags[(NO_NEWLINES as libc::c_int as libc::c_ulong)
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
            {
                new_magicline();
            }
            data = nmalloc(
                (strlen((*line).data))
                    .wrapping_add(strlen((*u).strdata))
                    .wrapping_add(1 as libc::c_int as libc::c_ulong),
            ) as *mut libc::c_char;
            strncpy(data, (*line).data, (*u).head_x);
            strcpy(&mut *data.offset((*u).head_x as isize), (*u).strdata);
            strcpy(
                &mut *data
                    .offset(
                        ((*u).head_x)
                            .wrapping_add(
                                (strlen
                                    as unsafe extern "C" fn(
                                        *const libc::c_char,
                                    ) -> libc::c_ulong)((*u).strdata),
                            ) as isize,
                    ),
                &mut *((*line).data).offset((*u).head_x as isize),
            );
            rpl_free((*line).data as *mut libc::c_void);
            (*line).data = data;
            goto_line_posx((*u).tail_lineno, (*u).tail_x);
        }
        1 => {
            redidmsg = dcgettext(
                0 as *const libc::c_char,
                b"line break\0" as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            );
            *((*line).data).offset((*u).head_x as isize) = '\0' as i32 as libc::c_char;
            intruder = make_new_node(line);
            (*intruder).data = copy_of((*u).strdata);
            splice_node(line, intruder);
            renumber_from(intruder);
            goto_line_posx(
                (*u).head_lineno + 1 as libc::c_int as libc::c_long,
                (*u).tail_x,
            );
        }
        2 | 3 => {
            redidmsg = dcgettext(
                0 as *const libc::c_char,
                b"deletion\0" as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            );
            memmove(
                ((*line).data).offset((*u).head_x as isize) as *mut libc::c_void,
                ((*line).data)
                    .offset((*u).head_x as isize)
                    .offset(strlen((*u).strdata) as isize) as *const libc::c_void,
                (strlen(((*line).data).offset((*u).head_x as isize)))
                    .wrapping_sub(strlen((*u).strdata))
                    .wrapping_add(1 as libc::c_int as libc::c_ulong),
            );
            goto_line_posx((*u).head_lineno, (*u).head_x);
        }
        4 => {
            redidmsg = dcgettext(
                0 as *const libc::c_char,
                b"line join\0" as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            );
            if (*u).xflags & (1 as libc::c_int) << 1 as libc::c_int != 0
                && !(flags[(NO_NEWLINES as libc::c_int as libc::c_ulong)
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
            {
                goto_line_posx((*u).tail_lineno, (*u).tail_x);
            } else {
                (*line)
                    .data = nrealloc(
                    (*line).data as *mut libc::c_void,
                    (strlen((*line).data))
                        .wrapping_add(strlen((*u).strdata))
                        .wrapping_add(1 as libc::c_int as libc::c_ulong),
                ) as *mut libc::c_char;
                strcat((*line).data, (*u).strdata);
                unlink_node((*line).next);
                renumber_from(line);
                (*openfile).current = line;
                goto_line_posx((*u).tail_lineno, (*u).tail_x);
            }
        }
        5 => {
            redidmsg = dcgettext(
                0 as *const libc::c_char,
                b"replacement\0" as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            );
            if (*u).xflags & (1 as libc::c_int) << 3 as libc::c_int != 0
                && !(flags[(NO_NEWLINES as libc::c_int as libc::c_ulong)
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
            {
                new_magicline();
            }
            data = (*u).strdata;
            (*u).strdata = (*line).data;
            (*line).data = data;
            goto_line_posx((*u).head_lineno, (*u).head_x);
        }
        6 => {
            (*openfile).current_undo = u;
            while (*(*openfile).current_undo).type_0 as libc::c_uint
                != SPLIT_END as libc::c_int as libc::c_uint
            {
                do_redo();
            }
            u = (*openfile).current_undo;
            goto_line_posx((*u).head_lineno, (*u).head_x);
        }
        7 => {
            redidmsg = dcgettext(
                0 as *const libc::c_char,
                b"addition\0" as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            );
        }
        13 => {
            redidmsg = dcgettext(
                0 as *const libc::c_char,
                b"erasure\0" as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            );
            redo_cut(u);
        }
        15 | 14 => {
            redidmsg = dcgettext(
                0 as *const libc::c_char,
                b"cut\0" as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            );
            redo_cut(u);
        }
        17 => {
            redidmsg = dcgettext(
                0 as *const libc::c_char,
                b"paste\0" as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            );
            undo_cut(u);
        }
        18 => {
            redidmsg = dcgettext(
                0 as *const libc::c_char,
                b"insertion\0" as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            );
            goto_line_posx((*u).head_lineno, (*u).head_x);
            if !((*u).cutbuffer).is_null() {
                copy_from_buffer((*u).cutbuffer);
            } else {
                suppress_modification = 1 as libc::c_int != 0;
            }
            free_lines((*u).cutbuffer);
            (*u).cutbuffer = 0 as *mut linestruct;
        }
        19 => {
            (*openfile).current_undo = u;
            do_redo();
            do_redo();
            do_redo();
            return;
        }
        20 => {
            redidmsg = (*u).strdata;
            goto_line_posx((*u).tail_lineno, (*u).tail_x);
            (*openfile).current_y = (*u).head_lineno;
            adjust_viewport(STATIONARY);
        }
        8 => {
            handle_indent_action(u, 0 as libc::c_int != 0, 1 as libc::c_int != 0);
            redidmsg = dcgettext(
                0 as *const libc::c_char,
                b"indent\0" as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            );
        }
        9 => {
            handle_indent_action(u, 0 as libc::c_int != 0, 0 as libc::c_int != 0);
            redidmsg = dcgettext(
                0 as *const libc::c_char,
                b"unindent\0" as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            );
        }
        10 => {
            handle_comment_action(u, 0 as libc::c_int != 0, 1 as libc::c_int != 0);
            redidmsg = dcgettext(
                0 as *const libc::c_char,
                b"comment\0" as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            );
        }
        11 => {
            handle_comment_action(u, 0 as libc::c_int != 0, 0 as libc::c_int != 0);
            redidmsg = dcgettext(
                0 as *const libc::c_char,
                b"uncomment\0" as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            );
        }
        _ => {}
    }
    if !redidmsg.is_null()
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
    {
        statusline(
            HUSH,
            dcgettext(
                0 as *const libc::c_char,
                b"Redid %s\0" as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ),
            redidmsg,
        );
    }
    (*openfile).current_undo = u;
    (*openfile).last_action = OTHER;
    (*openfile).mark = 0 as *mut linestruct;
    (*openfile).placewewant = xplustabs();
    (*openfile).totsize = (*u).newsize;
    if (*u).type_0 as libc::c_uint <= REPLACE as libc::c_int as libc::c_uint {
        check_the_multis((*openfile).current);
    } else if (*u).type_0 as libc::c_uint == INSERT as libc::c_int as libc::c_uint {
        recook = 1 as libc::c_int != 0;
    }
    if (*openfile).current_undo == (*openfile).last_saved {
        (*openfile).modified = 0 as libc::c_int != 0;
        titlebar(0 as *const libc::c_char);
    } else if !suppress_modification {
        set_modified();
    }
}
pub unsafe extern "C" fn do_enter() {
    let mut newnode: *mut linestruct = make_new_node((*openfile).current);
    let mut extra: size_t = 0 as libc::c_int as size_t;
    let mut sampleline: *mut linestruct = (*openfile).current;
    let mut allblanks: bool = 0 as libc::c_int != 0;
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
            && !((*sampleline).next).is_null()
            && inpar((*sampleline).next) as libc::c_int != 0
            && !begpar((*sampleline).next, 0 as libc::c_int)
        {
            sampleline = (*sampleline).next;
        }
        extra = indent_length((*sampleline).data);
        if extra > (*openfile).current_x {
            extra = (*openfile).current_x;
        } else if extra == (*openfile).current_x {
            allblanks = indent_length((*(*openfile).current).data) == extra;
        }
    }
    (*newnode)
        .data = nmalloc(
        (strlen(((*(*openfile).current).data).offset((*openfile).current_x as isize)))
            .wrapping_add(extra)
            .wrapping_add(1 as libc::c_int as libc::c_ulong),
    ) as *mut libc::c_char;
    strcpy(
        &mut *((*newnode).data).offset(extra as isize),
        ((*(*openfile).current).data).offset((*openfile).current_x as isize),
    );
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
        strncpy((*newnode).data, (*sampleline).data, extra);
        if allblanks {
            (*openfile).current_x = 0 as libc::c_int as size_t;
        }
    }
    *((*(*openfile).current).data)
        .offset((*openfile).current_x as isize) = '\0' as i32 as libc::c_char;
    add_undo(ENTER, 0 as *const libc::c_char);
    if (*openfile).mark == (*openfile).current
        && (*openfile).mark_x > (*openfile).current_x
    {
        (*openfile).mark = newnode;
        (*openfile)
            .mark_x = ((*openfile).mark_x as libc::c_ulong)
            .wrapping_add(extra.wrapping_sub((*openfile).current_x)) as size_t as size_t;
    }
    splice_node((*openfile).current, newnode);
    renumber_from(newnode);
    (*openfile).current = newnode;
    (*openfile).current_x = extra;
    (*openfile).placewewant = xplustabs();
    (*openfile).totsize = ((*openfile).totsize).wrapping_add(1);
    (*openfile).totsize;
    set_modified();
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
                ) != 0 as libc::c_int as libc::c_uint && !allblanks
    {
        (*openfile)
            .totsize = ((*openfile).totsize as libc::c_ulong).wrapping_add(extra)
            as size_t as size_t;
    }
    update_undo(ENTER);
    refresh_needed = 1 as libc::c_int != 0;
    focusing = 0 as libc::c_int != 0;
}
pub unsafe extern "C" fn discard_until(mut thisitem: *const undostruct) {
    let mut dropit: *mut undostruct = (*openfile).undotop;
    let mut group: *mut groupstruct = 0 as *mut groupstruct;
    while !dropit.is_null() && dropit != thisitem as *mut undostruct {
        (*openfile).undotop = (*dropit).next;
        rpl_free((*dropit).strdata as *mut libc::c_void);
        free_lines((*dropit).cutbuffer);
        group = (*dropit).grouping;
        while !group.is_null() {
            let mut next: *mut groupstruct = (*group).next;
            free_chararray(
                (*group).indentations,
                ((*group).bottom_line - (*group).top_line
                    + 1 as libc::c_int as libc::c_long) as size_t,
            );
            rpl_free(group as *mut libc::c_void);
            group = next;
        }
        rpl_free(dropit as *mut libc::c_void);
        dropit = (*openfile).undotop;
    }
    (*openfile).current_undo = thisitem as *mut undostruct;
    (*openfile).last_action = OTHER;
}
pub unsafe extern "C" fn add_undo(
    mut action: undo_type,
    mut message: *const libc::c_char,
) {
    let mut u: *mut undostruct = nmalloc(
        ::std::mem::size_of::<undostruct>() as libc::c_ulong,
    ) as *mut undostruct;
    let mut thisline: *mut linestruct = (*openfile).current;
    (*u).type_0 = action;
    (*u).strdata = 0 as *mut libc::c_char;
    (*u).cutbuffer = 0 as *mut linestruct;
    (*u).head_lineno = (*thisline).lineno;
    (*u).head_x = (*openfile).current_x;
    (*u).tail_lineno = (*thisline).lineno;
    (*u).tail_x = (*openfile).current_x;
    (*u).wassize = (*openfile).totsize;
    (*u).newsize = (*openfile).totsize;
    (*u).grouping = 0 as *mut groupstruct;
    (*u).xflags = 0 as libc::c_int;
    discard_until((*openfile).current_undo);
    if (*u).type_0 as libc::c_uint == SPLIT_BEGIN as libc::c_int as libc::c_uint {
        action = (*(*openfile).undotop).type_0;
        (*u).wassize = (*(*openfile).undotop).wassize;
        (*u).next = (*(*openfile).undotop).next;
        (*(*openfile).undotop).next = u;
    } else {
        (*u).next = (*openfile).undotop;
        (*openfile).undotop = u;
        (*openfile).current_undo = u;
    }
    let mut current_block_69: u64;
    match (*u).type_0 as libc::c_uint {
        0 => {
            if thisline == (*openfile).filebot {
                (*u).xflags |= (1 as libc::c_int) << 3 as libc::c_int;
            }
            current_block_69 = 5597585068398118923;
        }
        2 => {
            if (*thisline).next == (*openfile).filebot
                && *((*thisline).data).offset(0 as libc::c_int as isize) as libc::c_int
                    != '\0' as i32
            {
                (*u).xflags |= (1 as libc::c_int) << 1 as libc::c_int;
            }
            current_block_69 = 11948064939145634034;
        }
        3 => {
            current_block_69 = 11948064939145634034;
        }
        5 => {
            (*u).strdata = copy_of((*thisline).data);
            if thisline == (*openfile).filebot
                && *answer.offset(0 as libc::c_int as isize) as libc::c_int
                    != '\0' as i32
            {
                (*u).xflags |= (1 as libc::c_int) << 3 as libc::c_int;
            }
            current_block_69 = 5597585068398118923;
        }
        15 => {
            (*u).xflags
                |= (1 as libc::c_int) << 3 as libc::c_int
                    | (1 as libc::c_int) << 5 as libc::c_int;
            if (*(*openfile).current).has_anchor {
                (*u).xflags |= (1 as libc::c_int) << 6 as libc::c_int;
            }
            current_block_69 = 5597585068398118923;
        }
        13 | 14 => {
            if !((*openfile).mark).is_null() {
                if mark_is_before_cursor() {
                    (*u).head_lineno = (*(*openfile).mark).lineno;
                    (*u).head_x = (*openfile).mark_x;
                    (*u).xflags |= (1 as libc::c_int) << 4 as libc::c_int;
                } else {
                    (*u).tail_lineno = (*(*openfile).mark).lineno;
                    (*u).tail_x = (*openfile).mark_x;
                    (*u).xflags
                        |= (1 as libc::c_int) << 4 as libc::c_int
                            | (1 as libc::c_int) << 5 as libc::c_int;
                }
                if (*u).tail_lineno == (*(*openfile).filebot).lineno {
                    (*u).xflags |= (1 as libc::c_int) << 3 as libc::c_int;
                }
            } else if !(flags[(CUT_FROM_CURSOR as libc::c_int as libc::c_ulong)
                .wrapping_div(
                    (::std::mem::size_of::<libc::c_uint>() as libc::c_ulong)
                        .wrapping_mul(8 as libc::c_int as libc::c_ulong),
                ) as usize]
                & (1 as libc::c_int as libc::c_uint)
                    << (CUT_FROM_CURSOR as libc::c_int as libc::c_ulong)
                        .wrapping_rem(
                            (::std::mem::size_of::<libc::c_uint>() as libc::c_ulong)
                                .wrapping_mul(8 as libc::c_int as libc::c_ulong),
                        ) != 0 as libc::c_int as libc::c_uint)
            {
                (*u).xflags
                    |= (1 as libc::c_int) << 2 as libc::c_int
                        | (1 as libc::c_int) << 5 as libc::c_int;
                (*u).tail_x = 0 as libc::c_int as size_t;
            } else {
                (*u).xflags |= (1 as libc::c_int) << 5 as libc::c_int;
            }
            if !((*openfile).mark).is_null()
                && mark_is_before_cursor() as libc::c_int != 0
                && (*(*openfile).mark).has_anchor as libc::c_int != 0
                || (((*openfile).mark).is_null() || !mark_is_before_cursor())
                    && (*(*openfile).current).has_anchor as libc::c_int != 0
            {
                (*u).xflags |= (1 as libc::c_int) << 6 as libc::c_int;
            }
            current_block_69 = 5597585068398118923;
        }
        17 => {
            (*u).cutbuffer = copy_buffer(cutbuffer);
            current_block_69 = 11482775374129048072;
        }
        18 => {
            current_block_69 = 11482775374129048072;
        }
        19 => {
            (*u).tail_lineno = (*openfile).current_y;
            current_block_69 = 5673697351071730138;
        }
        20 => {
            current_block_69 = 5673697351071730138;
        }
        1 | 6 | 7 | 8 | 9 | 10 | 11 => {
            current_block_69 = 5597585068398118923;
        }
        _ => {
            die(
                b"Bad undo type -- please report a bug\n\0" as *const u8
                    as *const libc::c_char,
            );
            current_block_69 = 5597585068398118923;
        }
    }
    match current_block_69 {
        11948064939145634034 => {
            if *((*thisline).data).offset((*openfile).current_x as isize) as libc::c_int
                != '\0' as i32
            {
                let mut charlen: libc::c_int = char_length(
                    ((*thisline).data).offset((*u).head_x as isize),
                );
                (*u)
                    .strdata = measured_copy(
                    ((*thisline).data).offset((*u).head_x as isize),
                    charlen as size_t,
                );
                if (*u).type_0 as libc::c_uint == BACK as libc::c_int as libc::c_uint {
                    (*u)
                        .tail_x = ((*u).tail_x as libc::c_ulong)
                        .wrapping_add(charlen as libc::c_ulong) as size_t as size_t;
                }
            } else {
                action = JOIN;
                if !((*thisline).next).is_null() {
                    if (*u).type_0 as libc::c_uint == BACK as libc::c_int as libc::c_uint
                    {
                        (*u).head_lineno = (*(*thisline).next).lineno;
                        (*u).head_x = 0 as libc::c_int as size_t;
                    }
                    (*u).strdata = copy_of((*(*thisline).next).data);
                }
                (*u).type_0 = JOIN;
            }
        }
        11482775374129048072 => {
            if thisline == (*openfile).filebot {
                (*u).xflags |= (1 as libc::c_int) << 3 as libc::c_int;
            }
        }
        5673697351071730138 => {
            (*u)
                .strdata = copy_of(
                dcgettext(0 as *const libc::c_char, message, 5 as libc::c_int),
            );
        }
        _ => {}
    }
    (*openfile).last_action = action;
}
pub unsafe extern "C" fn update_multiline_undo(
    mut lineno: ssize_t,
    mut indentation: *mut libc::c_char,
) {
    let mut u: *mut undostruct = (*openfile).current_undo;
    if !((*u).grouping).is_null()
        && (*(*u).grouping).bottom_line + 1 as libc::c_int as libc::c_long == lineno
    {
        let mut number_of_lines: size_t = (lineno - (*(*u).grouping).top_line
            + 1 as libc::c_int as libc::c_long) as size_t;
        (*(*u).grouping).bottom_line = lineno;
        (*(*u).grouping)
            .indentations = nrealloc(
            (*(*u).grouping).indentations as *mut libc::c_void,
            number_of_lines
                .wrapping_mul(
                    ::std::mem::size_of::<*mut libc::c_char>() as libc::c_ulong,
                ),
        ) as *mut *mut libc::c_char;
        let ref mut fresh1 = *((*(*u).grouping).indentations)
            .offset(
                number_of_lines.wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize,
            );
        *fresh1 = copy_of(indentation);
    } else {
        let mut born: *mut groupstruct = nmalloc(
            ::std::mem::size_of::<groupstruct>() as libc::c_ulong,
        ) as *mut groupstruct;
        (*born).top_line = lineno;
        (*born).bottom_line = lineno;
        (*born)
            .indentations = nmalloc(
            ::std::mem::size_of::<*mut libc::c_char>() as libc::c_ulong,
        ) as *mut *mut libc::c_char;
        let ref mut fresh2 = *((*born).indentations).offset(0 as libc::c_int as isize);
        *fresh2 = copy_of(indentation);
        (*born).next = (*u).grouping;
        (*u).grouping = born;
    }
    (*u).newsize = (*openfile).totsize;
}
pub unsafe extern "C" fn update_undo(mut action: undo_type) {
    let mut u: *mut undostruct = (*openfile).undotop;
    let mut datalen: size_t = 0;
    let mut newlen: size_t = 0;
    let mut textposition: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut charlen: libc::c_int = 0;
    if (*u).type_0 as libc::c_uint != action as libc::c_uint {
        die(
            b"Mismatching undo type -- please report a bug\n\0" as *const u8
                as *const libc::c_char,
        );
    }
    (*u).newsize = (*openfile).totsize;
    let mut current_block_46: u64;
    match (*u).type_0 as libc::c_uint {
        0 => {
            newlen = ((*openfile).current_x).wrapping_sub((*u).head_x);
            (*u)
                .strdata = nrealloc(
                (*u).strdata as *mut libc::c_void,
                newlen.wrapping_add(1 as libc::c_int as libc::c_ulong),
            ) as *mut libc::c_char;
            strncpy(
                (*u).strdata,
                ((*(*openfile).current).data).offset((*u).head_x as isize),
                newlen,
            );
            *((*u).strdata).offset(newlen as isize) = '\0' as i32 as libc::c_char;
            (*u).tail_x = (*openfile).current_x;
        }
        1 => {
            (*u).strdata = copy_of((*(*openfile).current).data);
            (*u).tail_x = (*openfile).current_x;
        }
        2 | 3 => {
            textposition = ((*(*openfile).current).data)
                .offset((*openfile).current_x as isize);
            charlen = char_length(textposition);
            datalen = strlen((*u).strdata);
            if (*openfile).current_x == (*u).head_x {
                (*u)
                    .strdata = nrealloc(
                    (*u).strdata as *mut libc::c_void,
                    datalen
                        .wrapping_add(charlen as libc::c_ulong)
                        .wrapping_add(1 as libc::c_int as libc::c_ulong),
                ) as *mut libc::c_char;
                strncpy(
                    ((*u).strdata).offset(datalen as isize),
                    textposition,
                    charlen as libc::c_ulong,
                );
                *((*u).strdata)
                    .offset(
                        datalen.wrapping_add(charlen as libc::c_ulong) as isize,
                    ) = '\0' as i32 as libc::c_char;
                (*u).tail_x = (*openfile).current_x;
            } else if (*openfile).current_x
                == ((*u).head_x).wrapping_sub(charlen as libc::c_ulong)
            {
                (*u)
                    .strdata = nrealloc(
                    (*u).strdata as *mut libc::c_void,
                    datalen
                        .wrapping_add(charlen as libc::c_ulong)
                        .wrapping_add(1 as libc::c_int as libc::c_ulong),
                ) as *mut libc::c_char;
                memmove(
                    ((*u).strdata).offset(charlen as isize) as *mut libc::c_void,
                    (*u).strdata as *const libc::c_void,
                    datalen.wrapping_add(1 as libc::c_int as libc::c_ulong),
                );
                strncpy((*u).strdata, textposition, charlen as libc::c_ulong);
                (*u).head_x = (*openfile).current_x;
            } else {
                add_undo((*u).type_0, 0 as *const libc::c_char);
            }
        }
        13 | 15 | 14 => {
            if (*u).type_0 as libc::c_uint == ZAP as libc::c_int as libc::c_uint {
                (*u).cutbuffer = cutbuffer;
                current_block_46 = 5689316957504528238;
            } else if !cutbuffer.is_null() {
                free_lines((*u).cutbuffer);
                (*u).cutbuffer = copy_buffer(cutbuffer);
                current_block_46 = 5689316957504528238;
            } else {
                current_block_46 = 14775119014532381840;
            }
            match current_block_46 {
                14775119014532381840 => {}
                _ => {
                    if (*u).xflags & (1 as libc::c_int) << 4 as libc::c_int == 0 {
                        let mut bottomline: *mut linestruct = (*u).cutbuffer;
                        let mut count: size_t = 0 as libc::c_int as size_t;
                        while !((*bottomline).next).is_null() {
                            bottomline = (*bottomline).next;
                            count = count.wrapping_add(1);
                            count;
                        }
                        (*u)
                            .tail_lineno = ((*u).head_lineno as libc::c_ulong)
                            .wrapping_add(count) as ssize_t;
                        if flags[(CUT_FROM_CURSOR as libc::c_int as libc::c_ulong)
                            .wrapping_div(
                                (::std::mem::size_of::<libc::c_uint>() as libc::c_ulong)
                                    .wrapping_mul(8 as libc::c_int as libc::c_ulong),
                            ) as usize]
                            & (1 as libc::c_int as libc::c_uint)
                                << (CUT_FROM_CURSOR as libc::c_int as libc::c_ulong)
                                    .wrapping_rem(
                                        (::std::mem::size_of::<libc::c_uint>() as libc::c_ulong)
                                            .wrapping_mul(8 as libc::c_int as libc::c_ulong),
                                    ) != 0 as libc::c_int as libc::c_uint
                            || (*u).type_0 as libc::c_uint
                                == CUT_TO_EOF as libc::c_int as libc::c_uint
                        {
                            (*u).tail_x = strlen((*bottomline).data);
                            if count == 0 as libc::c_int as libc::c_ulong {
                                (*u)
                                    .tail_x = ((*u).tail_x as libc::c_ulong)
                                    .wrapping_add((*u).head_x) as size_t as size_t;
                            }
                        } else if (*openfile).current == (*openfile).filebot
                            && flags[(NO_NEWLINES as libc::c_int as libc::c_ulong)
                                .wrapping_div(
                                    (::std::mem::size_of::<libc::c_uint>() as libc::c_ulong)
                                        .wrapping_mul(8 as libc::c_int as libc::c_ulong),
                                ) as usize]
                                & (1 as libc::c_int as libc::c_uint)
                                    << (NO_NEWLINES as libc::c_int as libc::c_ulong)
                                        .wrapping_rem(
                                            (::std::mem::size_of::<libc::c_uint>() as libc::c_ulong)
                                                .wrapping_mul(8 as libc::c_int as libc::c_ulong),
                                        ) != 0 as libc::c_int as libc::c_uint
                        {
                            (*u).tail_x = strlen((*bottomline).data);
                        }
                    }
                }
            }
        }
        5 | 6 | 7 | 19 => {}
        20 | 17 | 18 => {
            (*u).tail_lineno = (*(*openfile).current).lineno;
            (*u).tail_x = (*openfile).current_x;
        }
        _ => {
            die(
                b"Bad undo type -- please report a bug\n\0" as *const u8
                    as *const libc::c_char,
            );
        }
    };
}
pub unsafe extern "C" fn do_wrap() {
    let mut line: *mut linestruct = (*openfile).current;
    let mut line_len: size_t = strlen((*line).data);
    let mut quot_len: size_t = quote_length((*line).data);
    let mut lead_len: size_t = quot_len
        .wrapping_add(indent_length(((*line).data).offset(quot_len as isize)));
    let mut cursor_x: size_t = (*openfile).current_x;
    let mut wrap_loc: ssize_t = 0;
    let mut remainder: *const libc::c_char = 0 as *const libc::c_char;
    let mut rest_length: size_t = 0;
    wrap_loc = break_line(
        ((*line).data).offset(lead_len as isize),
        wrap_at.wrapping_sub(wideness((*line).data, lead_len)) as ssize_t,
        0 as libc::c_int != 0,
    );
    if wrap_loc < 0 as libc::c_int as libc::c_long
        || lead_len.wrapping_add(wrap_loc as libc::c_ulong) == line_len
    {
        return;
    }
    wrap_loc = lead_len
        .wrapping_add(
            step_right(((*line).data).offset(lead_len as isize), wrap_loc as size_t),
        ) as ssize_t;
    if *((*line).data).offset(wrap_loc as isize) as libc::c_int == '\0' as i32 {
        return;
    }
    add_undo(SPLIT_BEGIN, 0 as *const libc::c_char);
    let mut autowhite: bool = flags[(AUTOINDENT as libc::c_int as libc::c_ulong)
        .wrapping_div(
            (::std::mem::size_of::<libc::c_uint>() as libc::c_ulong)
                .wrapping_mul(8 as libc::c_int as libc::c_ulong),
        ) as usize]
        & (1 as libc::c_int as libc::c_uint)
            << (AUTOINDENT as libc::c_int as libc::c_ulong)
                .wrapping_rem(
                    (::std::mem::size_of::<libc::c_uint>() as libc::c_ulong)
                        .wrapping_mul(8 as libc::c_int as libc::c_ulong),
                ) != 0 as libc::c_int as libc::c_uint;
    if quot_len > 0 as libc::c_int as libc::c_ulong {
        flags[(AUTOINDENT as libc::c_int as libc::c_ulong)
            .wrapping_div(
                (::std::mem::size_of::<libc::c_uint>() as libc::c_ulong)
                    .wrapping_mul(8 as libc::c_int as libc::c_ulong),
            ) as usize]
            &= !((1 as libc::c_int as libc::c_uint)
                << (AUTOINDENT as libc::c_int as libc::c_ulong)
                    .wrapping_rem(
                        (::std::mem::size_of::<libc::c_uint>() as libc::c_ulong)
                            .wrapping_mul(8 as libc::c_int as libc::c_ulong),
                    ));
    }
    remainder = ((*line).data).offset(wrap_loc as isize);
    rest_length = line_len.wrapping_sub(wrap_loc as libc::c_ulong);
    if !((*openfile).spillage_line).is_null()
        && (*openfile).spillage_line == (*line).next
        && rest_length.wrapping_add(breadth((*(*line).next).data)) <= wrap_at
    {
        (*openfile).current_x = line_len;
        if !is_blank_char(remainder.offset(step_left(remainder, rest_length) as isize)) {
            add_undo(ADD, 0 as *const libc::c_char);
            (*line)
                .data = nrealloc(
                (*line).data as *mut libc::c_void,
                line_len.wrapping_add(2 as libc::c_int as libc::c_ulong),
            ) as *mut libc::c_char;
            *((*line).data).offset(line_len as isize) = ' ' as i32 as libc::c_char;
            *((*line).data)
                .offset(
                    line_len.wrapping_add(1 as libc::c_int as libc::c_ulong) as isize,
                ) = '\0' as i32 as libc::c_char;
            rest_length = rest_length.wrapping_add(1);
            rest_length;
            (*openfile).totsize = ((*openfile).totsize).wrapping_add(1);
            (*openfile).totsize;
            (*openfile).current_x = ((*openfile).current_x).wrapping_add(1);
            (*openfile).current_x;
            update_undo(ADD);
        }
        do_delete();
        if strncmp(
            (*line).data,
            ((*line).data).offset((*openfile).current_x as isize),
            lead_len,
        ) == 0 as libc::c_int
        {
            let mut i: size_t = lead_len;
            while i > 0 as libc::c_int as libc::c_ulong {
                do_delete();
                i = i.wrapping_sub(1);
                i;
            }
        }
        while is_blank_char(
            &mut *((*line).data).offset((*openfile).current_x as isize),
        ) {
            do_delete();
        }
    }
    (*openfile).current_x = wrap_loc as size_t;
    if flags[(TRIM_BLANKS as libc::c_int as libc::c_ulong)
        .wrapping_div(
            (::std::mem::size_of::<libc::c_uint>() as libc::c_ulong)
                .wrapping_mul(8 as libc::c_int as libc::c_ulong),
        ) as usize]
        & (1 as libc::c_int as libc::c_uint)
            << (TRIM_BLANKS as libc::c_int as libc::c_ulong)
                .wrapping_rem(
                    (::std::mem::size_of::<libc::c_uint>() as libc::c_ulong)
                        .wrapping_mul(8 as libc::c_int as libc::c_ulong),
                ) != 0 as libc::c_int as libc::c_uint
    {
        let mut rear_x: size_t = step_left((*line).data, wrap_loc as size_t);
        let mut typed_x: size_t = step_left((*line).data, cursor_x);
        while (rear_x != typed_x || cursor_x >= wrap_loc as libc::c_ulong)
            && is_blank_char(((*line).data).offset(rear_x as isize)) as libc::c_int != 0
        {
            (*openfile).current_x = rear_x;
            do_delete();
            rear_x = step_left((*line).data, rear_x);
        }
    }
    do_enter();
    if quot_len > 0 as libc::c_int as libc::c_ulong {
        line = (*line).next;
        line_len = strlen((*line).data);
        (*line)
            .data = nrealloc(
            (*line).data as *mut libc::c_void,
            lead_len
                .wrapping_add(line_len)
                .wrapping_add(1 as libc::c_int as libc::c_ulong),
        ) as *mut libc::c_char;
        memmove(
            ((*line).data).offset(lead_len as isize) as *mut libc::c_void,
            (*line).data as *const libc::c_void,
            line_len.wrapping_add(1 as libc::c_int as libc::c_ulong),
        );
        strncpy((*line).data, (*(*line).prev).data, lead_len);
        (*openfile)
            .current_x = ((*openfile).current_x as libc::c_ulong).wrapping_add(lead_len)
            as size_t as size_t;
        (*openfile)
            .totsize = ((*openfile).totsize as libc::c_ulong).wrapping_add(lead_len)
            as size_t as size_t;
        rpl_free((*(*openfile).undotop).strdata as *mut libc::c_void);
        update_undo(ENTER);
        if autowhite {
            flags[(AUTOINDENT as libc::c_int as libc::c_ulong)
                .wrapping_div(
                    (::std::mem::size_of::<libc::c_uint>() as libc::c_ulong)
                        .wrapping_mul(8 as libc::c_int as libc::c_ulong),
                ) as usize]
                |= (1 as libc::c_int as libc::c_uint)
                    << (AUTOINDENT as libc::c_int as libc::c_ulong)
                        .wrapping_rem(
                            (::std::mem::size_of::<libc::c_uint>() as libc::c_ulong)
                                .wrapping_mul(8 as libc::c_int as libc::c_ulong),
                        );
        }
    }
    (*openfile).spillage_line = (*openfile).current;
    if cursor_x < wrap_loc as libc::c_ulong {
        (*openfile).current = (*(*openfile).current).prev;
        (*openfile).current_x = cursor_x;
    } else {
        (*openfile)
            .current_x = ((*openfile).current_x as libc::c_ulong)
            .wrapping_add(cursor_x.wrapping_sub(wrap_loc as libc::c_ulong)) as size_t
            as size_t;
    }
    (*openfile).placewewant = xplustabs();
    add_undo(SPLIT_END, 0 as *const libc::c_char);
    refresh_needed = 1 as libc::c_int != 0;
}
pub unsafe extern "C" fn break_line(
    mut textstart: *const libc::c_char,
    mut goal: ssize_t,
    mut snap_at_nl: bool,
) -> ssize_t {
    let mut lastblank: *const libc::c_char = 0 as *const libc::c_char;
    let mut pointer: *const libc::c_char = textstart;
    let mut column: size_t = 0 as libc::c_int as size_t;
    while *pointer as libc::c_int != '\0' as i32
        && is_blank_char(pointer) as libc::c_int != 0
    {
        pointer = pointer.offset(advance_over(pointer, &mut column) as isize);
    }
    while *pointer as libc::c_int != '\0' as i32 && column as ssize_t <= goal {
        if is_blank_char(pointer) as libc::c_int != 0
            && (!inhelp || column > 17 as libc::c_int as libc::c_ulong
                || goal < 40 as libc::c_int as libc::c_long)
        {
            lastblank = pointer;
        } else if snap_at_nl as libc::c_int != 0
            && *pointer as libc::c_int == '\n' as i32
        {
            lastblank = pointer;
            break;
        }
        pointer = pointer.offset(advance_over(pointer, &mut column) as isize);
    }
    if column as ssize_t <= goal {
        return pointer.offset_from(textstart) as libc::c_long;
    }
    if snap_at_nl as libc::c_int != 0 && lastblank.is_null() {
        return step_left(
            textstart,
            pointer.offset_from(textstart) as libc::c_long as size_t,
        ) as ssize_t;
    }
    while lastblank.is_null() {
        if *pointer as libc::c_int == '\0' as i32 {
            return -(1 as libc::c_int) as ssize_t;
        }
        if is_blank_char(pointer) {
            lastblank = pointer;
        } else {
            pointer = pointer.offset(char_length(pointer) as isize);
        }
    }
    pointer = lastblank.offset(char_length(lastblank) as isize);
    while *pointer as libc::c_int != '\0' as i32
        && is_blank_char(pointer) as libc::c_int != 0
    {
        lastblank = pointer;
        pointer = pointer.offset(char_length(pointer) as isize);
    }
    return lastblank.offset_from(textstart) as libc::c_long;
}
pub unsafe extern "C" fn indent_length(mut line: *const libc::c_char) -> size_t {
    let mut start: *const libc::c_char = line;
    while *line as libc::c_int != '\0' as i32 && is_blank_char(line) as libc::c_int != 0
    {
        line = line.offset(char_length(line) as isize);
    }
    return line.offset_from(start) as libc::c_long as size_t;
}
pub unsafe extern "C" fn quote_length(mut line: *const libc::c_char) -> size_t {
    let mut matches: regmatch_t = regmatch_t { rm_so: 0, rm_eo: 0 };
    let mut rc: libc::c_int = rpl_regexec(
        &mut quotereg,
        line,
        1 as libc::c_int as size_t,
        &mut matches,
        0 as libc::c_int,
    );
    if rc == _REG_NOMATCH as libc::c_int
        || matches.rm_so == -(1 as libc::c_int) as regoff_t
    {
        return 0 as libc::c_int as size_t;
    }
    return matches.rm_eo as size_t;
}
pub unsafe extern "C" fn begpar(
    line: *const linestruct,
    mut depth: libc::c_int,
) -> bool {
    let mut quot_len: size_t = 0;
    let mut indent_len: size_t = 0;
    let mut prev_dent_len: size_t = 0;
    if ((*line).prev).is_null() {
        return 1 as libc::c_int != 0;
    }
    if depth > 222 as libc::c_int {
        return 0 as libc::c_int != 0;
    }
    quot_len = quote_length((*line).data);
    indent_len = indent_length(((*line).data).offset(quot_len as isize));
    if *((*line).data).offset(quot_len.wrapping_add(indent_len) as isize) as libc::c_int
        == '\0' as i32
    {
        return 0 as libc::c_int != 0;
    }
    if flags[(BOOKSTYLE as libc::c_int as libc::c_ulong)
        .wrapping_div(
            (::std::mem::size_of::<libc::c_uint>() as libc::c_ulong)
                .wrapping_mul(8 as libc::c_int as libc::c_ulong),
        ) as usize]
        & (1 as libc::c_int as libc::c_uint)
            << (BOOKSTYLE as libc::c_int as libc::c_ulong)
                .wrapping_rem(
                    (::std::mem::size_of::<libc::c_uint>() as libc::c_ulong)
                        .wrapping_mul(8 as libc::c_int as libc::c_ulong),
                ) != 0 as libc::c_int as libc::c_uint
        && !(flags[(AUTOINDENT as libc::c_int as libc::c_ulong)
            .wrapping_div(
                (::std::mem::size_of::<libc::c_uint>() as libc::c_ulong)
                    .wrapping_mul(8 as libc::c_int as libc::c_ulong),
            ) as usize]
            & (1 as libc::c_int as libc::c_uint)
                << (AUTOINDENT as libc::c_int as libc::c_ulong)
                    .wrapping_rem(
                        (::std::mem::size_of::<libc::c_uint>() as libc::c_ulong)
                            .wrapping_mul(8 as libc::c_int as libc::c_ulong),
                    ) != 0 as libc::c_int as libc::c_uint)
        && is_blank_char((*line).data) as libc::c_int != 0
    {
        return 1 as libc::c_int != 0;
    }
    if quot_len != quote_length((*(*line).prev).data)
        || strncmp((*line).data, (*(*line).prev).data, quot_len) != 0 as libc::c_int
    {
        return 1 as libc::c_int != 0;
    }
    prev_dent_len = indent_length(((*(*line).prev).data).offset(quot_len as isize));
    if *((*(*line).prev).data).offset(quot_len.wrapping_add(prev_dent_len) as isize)
        as libc::c_int == '\0' as i32
    {
        return 1 as libc::c_int != 0;
    }
    if wideness((*(*line).prev).data, quot_len.wrapping_add(prev_dent_len))
        == wideness((*line).data, quot_len.wrapping_add(indent_len))
    {
        return 0 as libc::c_int != 0;
    }
    return !begpar((*line).prev, depth + 1 as libc::c_int);
}
pub unsafe extern "C" fn inpar(line: *const linestruct) -> bool {
    let mut quot_len: size_t = quote_length((*line).data);
    let mut indent_len: size_t = indent_length(((*line).data).offset(quot_len as isize));
    return *((*line).data).offset(quot_len.wrapping_add(indent_len) as isize)
        as libc::c_int != '\0' as i32;
}
pub unsafe extern "C" fn find_paragraph(
    mut firstline: *mut *mut linestruct,
    linecount: *mut size_t,
) -> bool {
    let mut line: *mut linestruct = *firstline;
    while !inpar(line) && !((*line).next).is_null() {
        line = (*line).next;
    }
    *firstline = line;
    do_para_end(&mut line);
    if !inpar(line) {
        return 0 as libc::c_int != 0;
    }
    *linecount = ((*line).lineno - (**firstline).lineno
        + 1 as libc::c_int as libc::c_long) as size_t;
    return 1 as libc::c_int != 0;
}
pub unsafe extern "C" fn concat_paragraph(mut line: *mut linestruct, mut count: size_t) {
    while count > 1 as libc::c_int as libc::c_ulong {
        let mut next_line: *mut linestruct = (*line).next;
        let mut next_line_len: size_t = strlen((*next_line).data);
        let mut next_quot_len: size_t = quote_length((*next_line).data);
        let mut next_lead_len: size_t = next_quot_len
            .wrapping_add(
                indent_length(((*next_line).data).offset(next_quot_len as isize)),
            );
        let mut line_len: size_t = strlen((*line).data);
        if line_len > 0 as libc::c_int as libc::c_ulong
            && *((*line).data)
                .offset(
                    line_len.wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize,
                ) as libc::c_int != ' ' as i32
        {
            (*line)
                .data = nrealloc(
                (*line).data as *mut libc::c_void,
                line_len.wrapping_add(2 as libc::c_int as libc::c_ulong),
            ) as *mut libc::c_char;
            let fresh3 = line_len;
            line_len = line_len.wrapping_add(1);
            *((*line).data).offset(fresh3 as isize) = ' ' as i32 as libc::c_char;
            *((*line).data).offset(line_len as isize) = '\0' as i32 as libc::c_char;
        }
        (*line)
            .data = nrealloc(
            (*line).data as *mut libc::c_void,
            line_len
                .wrapping_add(next_line_len)
                .wrapping_sub(next_lead_len)
                .wrapping_add(1 as libc::c_int as libc::c_ulong),
        ) as *mut libc::c_char;
        strcat((*line).data, ((*next_line).data).offset(next_lead_len as isize));
        (*line)
            .has_anchor = ((*line).has_anchor as libc::c_int
            | (*next_line).has_anchor as libc::c_int) != 0;
        unlink_node(next_line);
        count = count.wrapping_sub(1);
        count;
    }
}
pub unsafe extern "C" fn copy_character(
    mut from: *mut *mut libc::c_char,
    mut to: *mut *mut libc::c_char,
) {
    let mut charlen: libc::c_int = char_length(*from);
    if *from == *to {
        *from = (*from).offset(charlen as isize);
        *to = (*to).offset(charlen as isize);
    } else {
        loop {
            charlen -= 1;
            if !(charlen >= 0 as libc::c_int) {
                break;
            }
            let fresh4 = *from;
            *from = (*from).offset(1);
            let fresh5 = *to;
            *to = (*to).offset(1);
            *fresh5 = *fresh4;
        }
    };
}
pub unsafe extern "C" fn squeeze(mut line: *mut linestruct, mut skip: size_t) {
    let mut start: *mut libc::c_char = ((*line).data).offset(skip as isize);
    let mut from: *mut libc::c_char = start;
    let mut to: *mut libc::c_char = start;
    while *from as libc::c_int != '\0' as i32 {
        if is_blank_char(from) {
            from = from.offset(char_length(from) as isize);
            let fresh6 = to;
            to = to.offset(1);
            *fresh6 = ' ' as i32 as libc::c_char;
            while *from as libc::c_int != '\0' as i32
                && is_blank_char(from) as libc::c_int != 0
            {
                from = from.offset(char_length(from) as isize);
            }
        } else if !(mbstrchr(punct, from)).is_null() {
            copy_character(&mut from, &mut to);
            if *from as libc::c_int != '\0' as i32
                && !(mbstrchr(brackets, from)).is_null()
            {
                copy_character(&mut from, &mut to);
            }
            if *from as libc::c_int != '\0' as i32
                && is_blank_char(from) as libc::c_int != 0
            {
                from = from.offset(char_length(from) as isize);
                let fresh7 = to;
                to = to.offset(1);
                *fresh7 = ' ' as i32 as libc::c_char;
            }
            if *from as libc::c_int != '\0' as i32
                && is_blank_char(from) as libc::c_int != 0
            {
                from = from.offset(char_length(from) as isize);
                let fresh8 = to;
                to = to.offset(1);
                *fresh8 = ' ' as i32 as libc::c_char;
            }
            while *from as libc::c_int != '\0' as i32
                && is_blank_char(from) as libc::c_int != 0
            {
                from = from.offset(char_length(from) as isize);
            }
        } else {
            copy_character(&mut from, &mut to);
        }
    }
    while to > start
        && *to.offset(-(1 as libc::c_int as isize)) as libc::c_int == ' ' as i32
    {
        to = to.offset(-1);
        to;
    }
    *to = '\0' as i32 as libc::c_char;
}
pub unsafe extern "C" fn rewrap_paragraph(
    mut line: *mut *mut linestruct,
    mut lead_string: *mut libc::c_char,
    mut lead_len: size_t,
) {
    let mut break_pos: ssize_t = 0;
    while breadth((**line).data) > wrap_at {
        let mut line_len: size_t = strlen((**line).data);
        break_pos = break_line(
            ((**line).data).offset(lead_len as isize),
            wrap_at.wrapping_sub(wideness((**line).data, lead_len)) as ssize_t,
            0 as libc::c_int != 0,
        );
        if break_pos < 0 as libc::c_int as libc::c_long
            || lead_len.wrapping_add(break_pos as libc::c_ulong) == line_len
        {
            break;
        }
        break_pos = (break_pos as libc::c_ulong)
            .wrapping_add(lead_len.wrapping_add(1 as libc::c_int as libc::c_ulong))
            as ssize_t as ssize_t;
        splice_node(*line, make_new_node(*line));
        (*(**line).next)
            .data = nmalloc(
            lead_len
                .wrapping_add(line_len)
                .wrapping_sub(break_pos as libc::c_ulong)
                .wrapping_add(1 as libc::c_int as libc::c_ulong),
        ) as *mut libc::c_char;
        strncpy((*(**line).next).data, lead_string, lead_len);
        strcpy(
            ((*(**line).next).data).offset(lead_len as isize),
            ((**line).data).offset(break_pos as isize),
        );
        if flags[(TRIM_BLANKS as libc::c_int as libc::c_ulong)
            .wrapping_div(
                (::std::mem::size_of::<libc::c_uint>() as libc::c_ulong)
                    .wrapping_mul(8 as libc::c_int as libc::c_ulong),
            ) as usize]
            & (1 as libc::c_int as libc::c_uint)
                << (TRIM_BLANKS as libc::c_int as libc::c_ulong)
                    .wrapping_rem(
                        (::std::mem::size_of::<libc::c_uint>() as libc::c_ulong)
                            .wrapping_mul(8 as libc::c_int as libc::c_ulong),
                    ) != 0 as libc::c_int as libc::c_uint
        {
            while break_pos > 0 as libc::c_int as libc::c_long
                && *((**line).data)
                    .offset((break_pos - 1 as libc::c_int as libc::c_long) as isize)
                    as libc::c_int == ' ' as i32
            {
                break_pos -= 1;
                break_pos;
            }
        }
        *((**line).data).offset(break_pos as isize) = '\0' as i32 as libc::c_char;
        *line = (**line).next;
    }
    if !((**line).next).is_null() {
        *line = (**line).next;
    }
}
pub unsafe extern "C" fn justify_paragraph(
    mut line: *mut *mut linestruct,
    mut count: size_t,
) {
    let mut sampleline: *mut linestruct = 0 as *mut linestruct;
    let mut quot_len: size_t = 0;
    let mut lead_len: size_t = 0;
    let mut lead_string: *mut libc::c_char = 0 as *mut libc::c_char;
    sampleline = if count == 1 as libc::c_int as libc::c_ulong {
        *line
    } else {
        (**line).next
    };
    quot_len = quote_length((*sampleline).data);
    lead_len = quot_len
        .wrapping_add(indent_length(((*sampleline).data).offset(quot_len as isize)));
    lead_string = measured_copy((*sampleline).data, lead_len);
    concat_paragraph(*line, count);
    squeeze(
        *line,
        quot_len.wrapping_add(indent_length(((**line).data).offset(quot_len as isize))),
    );
    rewrap_paragraph(line, lead_string, lead_len);
    rpl_free(lead_string as *mut libc::c_void);
}
pub unsafe extern "C" fn justify_text(mut whole_buffer: bool) {
    let mut linecount: size_t = 0;
    let mut startline: *mut linestruct = 0 as *mut linestruct;
    let mut endline: *mut linestruct = 0 as *mut linestruct;
    let mut start_x: size_t = 0;
    let mut end_x: size_t = 0;
    let mut was_cutbuffer: *mut linestruct = cutbuffer;
    let mut jusline: *mut linestruct = 0 as *mut linestruct;
    let mut before_eol: bool = 0 as libc::c_int != 0;
    let mut primary_lead: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut primary_len: size_t = 0 as libc::c_int as size_t;
    let mut secondary_lead: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut secondary_len: size_t = 0 as libc::c_int as size_t;
    let mut was_the_linenumber: ssize_t = (*(*openfile).current).lineno;
    add_undo(COUPLE_BEGIN, b"justification\0" as *const u8 as *const libc::c_char);
    if !((*openfile).mark).is_null() {
        let mut quot_len: size_t = 0;
        let mut fore_len: size_t = 0;
        let mut other_quot_len: size_t = 0;
        let mut other_white_len: size_t = 0;
        let mut sampleline: *mut linestruct = 0 as *mut linestruct;
        get_region(&mut startline, &mut start_x, &mut endline, &mut end_x);
        if startline == endline && start_x == end_x {
            statusline(
                AHEM,
                dcgettext(
                    0 as *const libc::c_char,
                    b"Selection is empty\0" as *const u8 as *const libc::c_char,
                    5 as libc::c_int,
                ),
            );
            discard_until((*(*openfile).undotop).next);
            return;
        }
        quot_len = quote_length((*startline).data);
        fore_len = quot_len
            .wrapping_add(indent_length(((*startline).data).offset(quot_len as isize)));
        if start_x <= fore_len {
            start_x = 0 as libc::c_int as size_t;
        }
        while start_x > 0 as libc::c_int as libc::c_ulong
            && is_blank_char(
                &mut *((*startline).data)
                    .offset(
                        start_x.wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize,
                    ),
            ) as libc::c_int != 0
        {
            start_x = step_left((*startline).data, start_x);
        }
        quot_len = quote_length((*endline).data);
        fore_len = quot_len
            .wrapping_add(indent_length(((*endline).data).offset(quot_len as isize)));
        if (0 as libc::c_int as libc::c_ulong) < end_x && end_x < fore_len {
            end_x = fore_len;
        }
        while end_x > 0 as libc::c_int as libc::c_ulong
            && is_blank_char(&mut *((*endline).data).offset(end_x as isize))
                as libc::c_int != 0
        {
            end_x = step_right((*endline).data, end_x);
        }
        sampleline = startline;
        while !((*sampleline).prev).is_null() && inpar(sampleline) as libc::c_int != 0
            && !begpar(sampleline, 0 as libc::c_int)
        {
            sampleline = (*sampleline).prev;
        }
        while !((*sampleline).next).is_null() && !inpar(sampleline) {
            sampleline = (*sampleline).next;
        }
        quot_len = quote_length((*sampleline).data);
        primary_len = quot_len
            .wrapping_add(indent_length(((*sampleline).data).offset(quot_len as isize)));
        primary_lead = measured_copy((*sampleline).data, primary_len);
        if !((*sampleline).next).is_null() && startline != endline {
            sampleline = (*sampleline).next;
        }
        other_quot_len = quote_length((*sampleline).data);
        other_white_len = indent_length(
            ((*sampleline).data).offset(other_quot_len as isize),
        );
        secondary_len = quot_len.wrapping_add(other_white_len);
        secondary_lead = nmalloc(
            secondary_len.wrapping_add(1 as libc::c_int as libc::c_ulong),
        ) as *mut libc::c_char;
        strncpy(secondary_lead, (*startline).data, quot_len);
        strncpy(
            secondary_lead.offset(quot_len as isize),
            ((*sampleline).data).offset(other_quot_len as isize),
            other_white_len,
        );
        *secondary_lead.offset(secondary_len as isize) = '\0' as i32 as libc::c_char;
        (*openfile).mark = startline;
        (*openfile).mark_x = start_x;
        (*openfile).current = endline;
        (*openfile).current_x = end_x;
        linecount = ((*endline).lineno - (*startline).lineno
            + (if end_x > 0 as libc::c_int as libc::c_ulong {
                1 as libc::c_int
            } else {
                0 as libc::c_int
            }) as libc::c_long) as size_t;
        before_eol = *((*endline).data).offset(end_x as isize) as libc::c_int
            != '\0' as i32;
    } else {
        if whole_buffer {
            (*openfile).current = (*openfile).filetop;
        } else if inpar((*openfile).current) as libc::c_int != 0
            && !begpar((*openfile).current, 0 as libc::c_int)
        {
            do_para_begin(&mut (*openfile).current);
        }
        if !find_paragraph(&mut (*openfile).current, &mut linecount) {
            (*openfile).current_x = strlen((*(*openfile).filebot).data);
            discard_until((*(*openfile).undotop).next);
            refresh_needed = 1 as libc::c_int != 0;
            return;
        }
        startline = (*openfile).current;
        start_x = 0 as libc::c_int as size_t;
        if whole_buffer {
            endline = (*openfile).filebot;
        } else {
            endline = startline;
            let mut count: size_t = linecount;
            while count > 1 as libc::c_int as libc::c_ulong {
                endline = (*endline).next;
                count = count.wrapping_sub(1);
                count;
            }
        }
        if !((*endline).next).is_null() {
            endline = (*endline).next;
            end_x = 0 as libc::c_int as size_t;
        } else {
            end_x = strlen((*endline).data);
        }
    }
    add_undo(CUT, 0 as *const libc::c_char);
    cutbuffer = 0 as *mut linestruct;
    extract_segment(startline, start_x, endline, end_x);
    update_undo(CUT);
    if !((*openfile).mark).is_null() {
        let mut line: *mut linestruct = cutbuffer;
        let mut quot_len_0: size_t = quote_length((*line).data);
        let mut fore_len_0: size_t = quot_len_0
            .wrapping_add(indent_length(((*line).data).offset(quot_len_0 as isize)));
        let mut text_len: size_t = (strlen((*line).data)).wrapping_sub(fore_len_0);
        if fore_len_0 > 0 as libc::c_int as libc::c_ulong {
            memmove(
                (*line).data as *mut libc::c_void,
                ((*line).data).offset(fore_len_0 as isize) as *const libc::c_void,
                text_len.wrapping_add(1 as libc::c_int as libc::c_ulong),
            );
        }
        if primary_len > 0 as libc::c_int as libc::c_ulong {
            (*line)
                .data = nrealloc(
                (*line).data as *mut libc::c_void,
                primary_len
                    .wrapping_add(text_len)
                    .wrapping_add(1 as libc::c_int as libc::c_ulong),
            ) as *mut libc::c_char;
            memmove(
                ((*line).data).offset(primary_len as isize) as *mut libc::c_void,
                (*line).data as *const libc::c_void,
                text_len.wrapping_add(1 as libc::c_int as libc::c_ulong),
            );
            strncpy((*line).data, primary_lead, primary_len);
        }
        concat_paragraph(cutbuffer, linecount);
        squeeze(cutbuffer, primary_len);
        rewrap_paragraph(&mut line, secondary_lead, secondary_len);
        if start_x > 0 as libc::c_int as libc::c_ulong {
            (*cutbuffer).prev = make_new_node(0 as *mut linestruct);
            (*(*cutbuffer).prev)
                .data = copy_of(b"\0" as *const u8 as *const libc::c_char);
            (*(*cutbuffer).prev).next = cutbuffer;
            cutbuffer = (*cutbuffer).prev;
        }
        if end_x > 0 as libc::c_int as libc::c_ulong && before_eol as libc::c_int != 0 {
            (*line).next = make_new_node(line);
            (*(*line).next).data = copy_of(primary_lead);
        }
        rpl_free(secondary_lead as *mut libc::c_void);
        rpl_free(primary_lead as *mut libc::c_void);
    } else {
        jusline = cutbuffer;
        justify_paragraph(&mut jusline, linecount);
        if whole_buffer {
            while find_paragraph(&mut jusline, &mut linecount) {
                justify_paragraph(&mut jusline, linecount);
                if ((*jusline).next).is_null() {
                    break;
                }
            }
        }
    }
    if whole_buffer as libc::c_int != 0 && ((*openfile).mark).is_null()
        && !(*cutbuffer).has_anchor
    {
        (*(*openfile).current).has_anchor = 0 as libc::c_int != 0;
    }
    add_undo(PASTE, 0 as *const libc::c_char);
    ingraft_buffer(cutbuffer);
    update_undo(PASTE);
    if !((*openfile).mark).is_null() && !mark_is_before_cursor() {
        let mut bottom: *mut linestruct = (*openfile).current;
        let mut bottom_x: size_t = (*openfile).current_x;
        (*openfile).current = (*openfile).mark;
        (*openfile).current_x = (*openfile).mark_x;
        (*openfile).mark = bottom;
        (*openfile).mark_x = bottom_x;
    } else if whole_buffer as libc::c_int != 0 && ((*openfile).mark).is_null() {
        goto_line_posx(was_the_linenumber, 0 as libc::c_int as size_t);
    }
    add_undo(COUPLE_END, b"justification\0" as *const u8 as *const libc::c_char);
    if !((*openfile).mark).is_null() {
        statusline(
            REMARK,
            dcgettext(
                0 as *const libc::c_char,
                b"Justified selection\0" as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ),
        );
    } else if whole_buffer {
        statusline(
            REMARK,
            dcgettext(
                0 as *const libc::c_char,
                b"Justified file\0" as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ),
        );
    } else {
        statusbar(
            dcgettext(
                0 as *const libc::c_char,
                b"Justified paragraph\0" as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ),
        );
    }
    cutbuffer = was_cutbuffer;
    (*openfile).placewewant = xplustabs();
    set_modified();
    refresh_needed = 1 as libc::c_int != 0;
    shift_held = 1 as libc::c_int != 0;
}
pub unsafe extern "C" fn do_justify() {
    justify_text(0 as libc::c_int != 0);
}
pub unsafe extern "C" fn do_full_justify() {
    justify_text(1 as libc::c_int != 0);
    ran_a_tool = 1 as libc::c_int != 0;
}
pub unsafe extern "C" fn construct_argument_list(
    mut arguments: *mut *mut *mut libc::c_char,
    mut command: *mut libc::c_char,
    mut filename: *mut libc::c_char,
) {
    let mut copy_of_command: *mut libc::c_char = copy_of(command);
    let mut element: *mut libc::c_char = strtok(
        copy_of_command,
        b" \0" as *const u8 as *const libc::c_char,
    );
    let mut count: libc::c_int = 2 as libc::c_int;
    while !element.is_null() {
        count += 1;
        *arguments = nrealloc(
            *arguments as *mut libc::c_void,
            (count as libc::c_ulong)
                .wrapping_mul(
                    ::std::mem::size_of::<*mut libc::c_char>() as libc::c_ulong,
                ),
        ) as *mut *mut libc::c_char;
        let ref mut fresh9 = *(*arguments).offset((count - 3 as libc::c_int) as isize);
        *fresh9 = element;
        element = strtok(
            0 as *mut libc::c_char,
            b" \0" as *const u8 as *const libc::c_char,
        );
    }
    let ref mut fresh10 = *(*arguments).offset((count - 2 as libc::c_int) as isize);
    *fresh10 = filename;
    let ref mut fresh11 = *(*arguments).offset((count - 1 as libc::c_int) as isize);
    *fresh11 = 0 as *mut libc::c_char;
}
pub unsafe extern "C" fn replace_buffer(
    mut filename: *const libc::c_char,
    mut action: undo_type,
    mut operation: *const libc::c_char,
) -> bool {
    let mut was_cutbuffer: *mut linestruct = cutbuffer;
    let mut descriptor: libc::c_int = 0;
    let mut stream: *mut FILE = 0 as *mut FILE;
    descriptor = open_file(filename, 0 as libc::c_int != 0, &mut stream);
    if descriptor < 0 as libc::c_int {
        return 0 as libc::c_int != 0;
    }
    add_undo(COUPLE_BEGIN, operation);
    if action as libc::c_uint == CUT_TO_EOF as libc::c_int as libc::c_uint {
        (*openfile).current = (*openfile).filetop;
        (*openfile).current_x = 0 as libc::c_int as size_t;
    }
    cutbuffer = 0 as *mut linestruct;
    add_undo(action, 0 as *const libc::c_char);
    do_snip(
        !((*openfile).mark).is_null(),
        ((*openfile).mark).is_null(),
        0 as libc::c_int != 0,
    );
    update_undo(action);
    free_lines(cutbuffer);
    cutbuffer = was_cutbuffer;
    read_file(stream, descriptor, filename, 1 as libc::c_int != 0);
    add_undo(COUPLE_END, operation);
    return 1 as libc::c_int != 0;
}
pub unsafe extern "C" fn treat(
    mut tempfile_name: *mut libc::c_char,
    mut theprogram: *mut libc::c_char,
    mut spelling: bool,
) {
    let mut was_lineno: ssize_t = (*(*openfile).current).lineno;
    let mut was_pww: size_t = (*openfile).placewewant;
    let mut was_x: size_t = (*openfile).current_x;
    let mut was_at_eol: bool = *((*(*openfile).current).data)
        .offset((*openfile).current_x as isize) as libc::c_int == '\0' as i32;
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
    let mut timestamp_sec: libc::c_long = 0 as libc::c_int as libc::c_long;
    let mut timestamp_nsec: libc::c_long = 0 as libc::c_int as libc::c_long;
    static mut arguments: *mut *mut libc::c_char = 0 as *const *mut libc::c_char
        as *mut *mut libc::c_char;
    let mut thepid: pid_t = 0;
    let mut program_status: libc::c_int = 0;
    let mut errornumber: libc::c_int = 0;
    let mut replaced: bool = 0 as libc::c_int != 0;
    if stat(tempfile_name, &mut fileinfo) == 0 as libc::c_int {
        if fileinfo.st_size == 0 as libc::c_int as libc::c_long {
            if spelling as libc::c_int != 0 && !((*openfile).mark).is_null() {
                statusline(
                    AHEM,
                    dcgettext(
                        0 as *const libc::c_char,
                        b"Selection is empty\0" as *const u8 as *const libc::c_char,
                        5 as libc::c_int,
                    ),
                );
            } else {
                statusline(
                    AHEM,
                    dcgettext(
                        0 as *const libc::c_char,
                        b"Buffer is empty\0" as *const u8 as *const libc::c_char,
                        5 as libc::c_int,
                    ),
                );
            }
            return;
        }
        timestamp_sec = fileinfo.st_mtim.tv_sec;
        timestamp_nsec = fileinfo.st_mtim.tv_nsec;
    }
    if spelling {
        endwin();
    } else {
        statusbar(
            dcgettext(
                0 as *const libc::c_char,
                b"Invoking formatter...\0" as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ),
        );
    }
    construct_argument_list(&mut arguments, theprogram, tempfile_name);
    thepid = fork();
    if thepid == 0 as libc::c_int {
        execvp(
            *arguments.offset(0 as libc::c_int as isize),
            arguments as *const *mut libc::c_char,
        );
        exit(9 as libc::c_int);
    } else if thepid > 0 as libc::c_int {
        block_sigwinch(1 as libc::c_int != 0);
        wait(&mut program_status);
        block_sigwinch(0 as libc::c_int != 0);
    }
    errornumber = *__errno_location();
    if spelling {
        terminal_init();
        doupdate();
    } else {
        full_refresh();
    }
    if thepid < 0 as libc::c_int {
        statusline(
            ALERT,
            dcgettext(
                0 as *const libc::c_char,
                b"Could not fork: %s\0" as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ),
            strerror(errornumber),
        );
        rpl_free(*arguments.offset(0 as libc::c_int as isize) as *mut libc::c_void);
        return;
    } else if !(program_status & 0x7f as libc::c_int == 0 as libc::c_int)
        || (program_status & 0xff00 as libc::c_int) >> 8 as libc::c_int
            > 2 as libc::c_int
    {
        statusline(
            ALERT,
            dcgettext(
                0 as *const libc::c_char,
                b"Error invoking '%s'\0" as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ),
            *arguments.offset(0 as libc::c_int as isize),
        );
        rpl_free(*arguments.offset(0 as libc::c_int as isize) as *mut libc::c_void);
        return;
    } else if (program_status & 0xff00 as libc::c_int) >> 8 as libc::c_int
        != 0 as libc::c_int
    {
        statusline(
            ALERT,
            dcgettext(
                0 as *const libc::c_char,
                b"Program '%s' complained\0" as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ),
            *arguments.offset(0 as libc::c_int as isize),
        );
    }
    rpl_free(*arguments.offset(0 as libc::c_int as isize) as *mut libc::c_void);
    if timestamp_sec > 0 as libc::c_int as libc::c_long
        && stat(tempfile_name, &mut fileinfo) == 0 as libc::c_int
        && fileinfo.st_mtim.tv_sec == timestamp_sec
        && fileinfo.st_mtim.tv_nsec == timestamp_nsec
    {
        statusline(
            REMARK,
            dcgettext(
                0 as *const libc::c_char,
                b"Nothing changed\0" as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ),
        );
        return;
    }
    if spelling as libc::c_int != 0 && !((*openfile).mark).is_null() {
        let mut was_mark_lineno: ssize_t = (*(*openfile).mark).lineno;
        let mut upright: bool = mark_is_before_cursor();
        replaced = replace_buffer(
            tempfile_name,
            CUT,
            b"spelling correction\0" as *const u8 as *const libc::c_char,
        );
        if upright {
            was_x = (*openfile).current_x;
        } else {
            (*openfile).mark_x = (*openfile).current_x;
        }
        (*openfile).mark = line_from_number(was_mark_lineno);
    } else {
        replaced = replace_buffer(
            tempfile_name,
            CUT_TO_EOF,
            if spelling as libc::c_int != 0 {
                b"spelling correction\0" as *const u8 as *const libc::c_char
            } else {
                b"formatting\0" as *const u8 as *const libc::c_char
            },
        );
    }
    goto_line_posx(was_lineno, was_x);
    if was_at_eol as libc::c_int != 0
        || (*openfile).current_x > strlen((*(*openfile).current).data)
    {
        (*openfile).current_x = strlen((*(*openfile).current).data);
    }
    if replaced {
        (*(*openfile).filetop).has_anchor = 0 as libc::c_int != 0;
        update_undo(COUPLE_END);
    }
    (*openfile).placewewant = was_pww;
    adjust_viewport(STATIONARY);
    if spelling {
        statusline(
            REMARK,
            dcgettext(
                0 as *const libc::c_char,
                b"Finished checking spelling\0" as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ),
        );
    } else {
        statusline(
            REMARK,
            dcgettext(
                0 as *const libc::c_char,
                b"Buffer has been processed\0" as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ),
        );
    };
}
pub unsafe extern "C" fn fix_spello(mut word: *const libc::c_char) -> bool {
    let mut was_edittop: *mut linestruct = (*openfile).edittop;
    let mut was_current: *mut linestruct = (*openfile).current;
    let mut was_firstcolumn: size_t = (*openfile).firstcolumn;
    let mut was_x: size_t = (*openfile).current_x;
    let mut proceed: bool = 0 as libc::c_int != 0;
    let mut result: libc::c_int = 0;
    let mut right_side_up: bool = !((*openfile).mark).is_null()
        && mark_is_before_cursor() as libc::c_int != 0;
    let mut top: *mut linestruct = 0 as *mut linestruct;
    let mut bot: *mut linestruct = 0 as *mut linestruct;
    let mut top_x: size_t = 0;
    let mut bot_x: size_t = 0;
    if !((*openfile).mark).is_null() {
        get_region(&mut top, &mut top_x, &mut bot, &mut bot_x);
        if right_side_up {
            (*openfile).current = top;
            (*openfile).current_x = top_x;
            (*openfile).mark = bot;
            (*openfile).mark_x = bot_x;
        }
    } else {
        (*openfile).current = (*openfile).filetop;
        (*openfile).current_x = 0 as libc::c_int as size_t;
    }
    result = findnextstr(
        word,
        1 as libc::c_int != 0,
        2 as libc::c_int,
        0 as *mut size_t,
        0 as libc::c_int != 0,
        0 as *const linestruct,
        0 as libc::c_int as size_t,
    );
    if result == 0 as libc::c_int {
        statusline(
            ALERT,
            dcgettext(
                0 as *const libc::c_char,
                b"Unfindable word: %s\0" as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ),
            word,
        );
        lastmessage = VACUUM;
        proceed = 1 as libc::c_int != 0;
        napms(2800 as libc::c_int);
    } else if result == 1 as libc::c_int {
        spotlighted = 1 as libc::c_int != 0;
        light_from_col = xplustabs();
        light_to_col = light_from_col.wrapping_add(breadth(word));
        let mut saved_mark: *mut linestruct = (*openfile).mark;
        (*openfile).mark = 0 as *mut linestruct;
        edit_refresh();
        put_cursor_at_end_of_answer();
        proceed = do_prompt(
            (1 as libc::c_int) << 9 as libc::c_int,
            word,
            0 as *mut *mut linestruct,
            Some(edit_refresh as unsafe extern "C" fn() -> ()),
            dcgettext(
                0 as *const libc::c_char,
                b"Edit a replacement\0" as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ),
        ) != -(1 as libc::c_int);
        spotlighted = 0 as libc::c_int != 0;
        (*openfile).mark = saved_mark;
        if proceed as libc::c_int != 0 && strcmp(word, answer) != 0 as libc::c_int {
            do_replace_loop(word, 1 as libc::c_int != 0, was_current, &mut was_x);
            statusbar(
                dcgettext(
                    0 as *const libc::c_char,
                    b"Next word...\0" as *const u8 as *const libc::c_char,
                    5 as libc::c_int,
                ),
            );
            napms(400 as libc::c_int);
        }
    }
    if !((*openfile).mark).is_null() {
        if right_side_up {
            (*openfile).current = (*openfile).mark;
            (*openfile).current_x = (*openfile).mark_x;
            (*openfile).mark = top;
            (*openfile).mark_x = top_x;
        } else {
            (*openfile).current = top;
            (*openfile).current_x = top_x;
        }
    } else {
        (*openfile).current = was_current;
        (*openfile).current_x = was_x;
    }
    (*openfile).edittop = was_edittop;
    (*openfile).firstcolumn = was_firstcolumn;
    return proceed;
}
pub unsafe extern "C" fn do_int_speller(mut tempfile_name: *const libc::c_char) {
    let mut misspellings: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut pointer: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut oneword: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut pipesize: libc::c_long = 0;
    let mut buffersize: size_t = 0;
    let mut bytesread: size_t = 0;
    let mut totalread: size_t = 0;
    let mut spell_fd: [libc::c_int; 2] = [0; 2];
    let mut sort_fd: [libc::c_int; 2] = [0; 2];
    let mut uniq_fd: [libc::c_int; 2] = [0; 2];
    let mut tempfile_fd: libc::c_int = -(1 as libc::c_int);
    let mut pid_spell: pid_t = 0;
    let mut pid_sort: pid_t = 0;
    let mut pid_uniq: pid_t = 0;
    let mut spell_status: libc::c_int = 0;
    let mut sort_status: libc::c_int = 0;
    let mut uniq_status: libc::c_int = 0;
    let mut stash: [libc::c_uint; 4] = [0; 4];
    if pipe(spell_fd.as_mut_ptr()) == -(1 as libc::c_int)
        || pipe(sort_fd.as_mut_ptr()) == -(1 as libc::c_int)
        || pipe(uniq_fd.as_mut_ptr()) == -(1 as libc::c_int)
    {
        statusline(
            ALERT,
            dcgettext(
                0 as *const libc::c_char,
                b"Could not create pipe: %s\0" as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ),
            strerror(*__errno_location()),
        );
        return;
    }
    statusbar(
        dcgettext(
            0 as *const libc::c_char,
            b"Invoking spell checker...\0" as *const u8 as *const libc::c_char,
            5 as libc::c_int,
        ),
    );
    pid_spell = fork();
    if pid_spell == 0 as libc::c_int {
        tempfile_fd = open(tempfile_name, 0 as libc::c_int);
        if tempfile_fd == -(1 as libc::c_int) {
            exit(6 as libc::c_int);
        }
        if dup2(tempfile_fd, 0 as libc::c_int) < 0 as libc::c_int {
            exit(7 as libc::c_int);
        }
        if dup2(spell_fd[1 as libc::c_int as usize], 1 as libc::c_int) < 0 as libc::c_int
        {
            exit(8 as libc::c_int);
        }
        close(tempfile_fd);
        close(spell_fd[0 as libc::c_int as usize]);
        close(spell_fd[1 as libc::c_int as usize]);
        execlp(
            b"hunspell\0" as *const u8 as *const libc::c_char,
            b"hunspell\0" as *const u8 as *const libc::c_char,
            b"-l\0" as *const u8 as *const libc::c_char,
            0 as *mut libc::c_void,
        );
        execlp(
            b"spell\0" as *const u8 as *const libc::c_char,
            b"spell\0" as *const u8 as *const libc::c_char,
            0 as *mut libc::c_void,
        );
        exit(9 as libc::c_int);
    }
    close(spell_fd[1 as libc::c_int as usize]);
    pid_sort = fork();
    if pid_sort == 0 as libc::c_int {
        if dup2(spell_fd[0 as libc::c_int as usize], 0 as libc::c_int) < 0 as libc::c_int
        {
            exit(7 as libc::c_int);
        }
        if dup2(sort_fd[1 as libc::c_int as usize], 1 as libc::c_int) < 0 as libc::c_int
        {
            exit(8 as libc::c_int);
        }
        close(spell_fd[0 as libc::c_int as usize]);
        close(sort_fd[0 as libc::c_int as usize]);
        close(sort_fd[1 as libc::c_int as usize]);
        execlp(
            b"sort\0" as *const u8 as *const libc::c_char,
            b"sort\0" as *const u8 as *const libc::c_char,
            b"-f\0" as *const u8 as *const libc::c_char,
            0 as *mut libc::c_void,
        );
        exit(9 as libc::c_int);
    }
    close(spell_fd[0 as libc::c_int as usize]);
    close(sort_fd[1 as libc::c_int as usize]);
    pid_uniq = fork();
    if pid_uniq == 0 as libc::c_int {
        if dup2(sort_fd[0 as libc::c_int as usize], 0 as libc::c_int) < 0 as libc::c_int
        {
            exit(7 as libc::c_int);
        }
        if dup2(uniq_fd[1 as libc::c_int as usize], 1 as libc::c_int) < 0 as libc::c_int
        {
            exit(8 as libc::c_int);
        }
        close(sort_fd[0 as libc::c_int as usize]);
        close(uniq_fd[0 as libc::c_int as usize]);
        close(uniq_fd[1 as libc::c_int as usize]);
        execlp(
            b"uniq\0" as *const u8 as *const libc::c_char,
            b"uniq\0" as *const u8 as *const libc::c_char,
            0 as *mut libc::c_void,
        );
        exit(9 as libc::c_int);
    }
    close(sort_fd[0 as libc::c_int as usize]);
    close(uniq_fd[1 as libc::c_int as usize]);
    if pid_spell < 0 as libc::c_int || pid_sort < 0 as libc::c_int
        || pid_uniq < 0 as libc::c_int
    {
        statusline(
            ALERT,
            dcgettext(
                0 as *const libc::c_char,
                b"Could not fork: %s\0" as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ),
            strerror(*__errno_location()),
        );
        close(uniq_fd[0 as libc::c_int as usize]);
        return;
    }
    pipesize = fpathconf(
        uniq_fd[0 as libc::c_int as usize],
        _PC_PIPE_BUF as libc::c_int,
    );
    if pipesize < 1 as libc::c_int as libc::c_long {
        statusline(
            ALERT,
            dcgettext(
                0 as *const libc::c_char,
                b"Could not get size of pipe buffer\0" as *const u8
                    as *const libc::c_char,
                5 as libc::c_int,
            ),
        );
        close(uniq_fd[0 as libc::c_int as usize]);
        return;
    }
    endwin();
    block_sigwinch(1 as libc::c_int != 0);
    totalread = 0 as libc::c_int as size_t;
    buffersize = (pipesize + 1 as libc::c_int as libc::c_long) as size_t;
    misspellings = nmalloc(buffersize) as *mut libc::c_char;
    pointer = misspellings;
    loop {
        bytesread = read(
            uniq_fd[0 as libc::c_int as usize],
            pointer as *mut libc::c_void,
            pipesize as size_t,
        ) as size_t;
        if !(bytesread > 0 as libc::c_int as libc::c_ulong) {
            break;
        }
        totalread = (totalread as libc::c_ulong).wrapping_add(bytesread) as size_t
            as size_t;
        buffersize = (buffersize as libc::c_ulong)
            .wrapping_add(pipesize as libc::c_ulong) as size_t as size_t;
        misspellings = nrealloc(misspellings as *mut libc::c_void, buffersize)
            as *mut libc::c_char;
        pointer = misspellings.offset(totalread as isize);
    }
    *pointer = '\0' as i32 as libc::c_char;
    close(uniq_fd[0 as libc::c_int as usize]);
    block_sigwinch(0 as libc::c_int != 0);
    terminal_init();
    doupdate();
    memcpy(
        stash.as_mut_ptr() as *mut libc::c_void,
        flags.as_mut_ptr() as *const libc::c_void,
        ::std::mem::size_of::<[libc::c_uint; 4]>() as libc::c_ulong,
    );
    flags[(CASE_SENSITIVE as libc::c_int as libc::c_ulong)
        .wrapping_div(
            (::std::mem::size_of::<libc::c_uint>() as libc::c_ulong)
                .wrapping_mul(8 as libc::c_int as libc::c_ulong),
        ) as usize]
        |= (1 as libc::c_int as libc::c_uint)
            << (CASE_SENSITIVE as libc::c_int as libc::c_ulong)
                .wrapping_rem(
                    (::std::mem::size_of::<libc::c_uint>() as libc::c_ulong)
                        .wrapping_mul(8 as libc::c_int as libc::c_ulong),
                );
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
    pointer = misspellings;
    oneword = misspellings;
    while *pointer as libc::c_int != '\0' as i32 {
        if *pointer as libc::c_int == '\r' as i32
            || *pointer as libc::c_int == '\n' as i32
        {
            *pointer = '\0' as i32 as libc::c_char;
            if oneword != pointer {
                if !fix_spello(oneword) {
                    oneword = pointer;
                    break;
                }
            }
            oneword = pointer.offset(1 as libc::c_int as isize);
        }
        pointer = pointer.offset(1);
        pointer;
    }
    if oneword != pointer {
        fix_spello(oneword);
    }
    rpl_free(misspellings as *mut libc::c_void);
    refresh_needed = 1 as libc::c_int != 0;
    memcpy(
        flags.as_mut_ptr() as *mut libc::c_void,
        stash.as_mut_ptr() as *const libc::c_void,
        ::std::mem::size_of::<[libc::c_uint; 4]>() as libc::c_ulong,
    );
    waitpid(pid_spell, &mut spell_status, 0 as libc::c_int);
    waitpid(pid_sort, &mut sort_status, 0 as libc::c_int);
    waitpid(pid_uniq, &mut uniq_status, 0 as libc::c_int);
    if (uniq_status & 0x7f as libc::c_int == 0 as libc::c_int) as libc::c_int
        == 0 as libc::c_int
        || (uniq_status & 0xff00 as libc::c_int) >> 8 as libc::c_int != 0
    {
        statusline(
            ALERT,
            dcgettext(
                0 as *const libc::c_char,
                b"Error invoking \"uniq\"\0" as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ),
        );
    } else if (sort_status & 0x7f as libc::c_int == 0 as libc::c_int) as libc::c_int
        == 0 as libc::c_int
        || (sort_status & 0xff00 as libc::c_int) >> 8 as libc::c_int != 0
    {
        statusline(
            ALERT,
            dcgettext(
                0 as *const libc::c_char,
                b"Error invoking \"sort\"\0" as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ),
        );
    } else if (spell_status & 0x7f as libc::c_int == 0 as libc::c_int) as libc::c_int
        == 0 as libc::c_int
        || (spell_status & 0xff00 as libc::c_int) >> 8 as libc::c_int != 0
    {
        statusline(
            ALERT,
            dcgettext(
                0 as *const libc::c_char,
                b"Error invoking \"spell\"\0" as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ),
        );
    } else {
        statusline(
            REMARK,
            dcgettext(
                0 as *const libc::c_char,
                b"Finished checking spelling\0" as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ),
        );
    };
}
pub unsafe extern "C" fn do_spell() {
    let mut stream: *mut FILE = 0 as *mut FILE;
    let mut temp_name: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut okay: bool = false;
    ran_a_tool = 1 as libc::c_int != 0;
    if in_restricted_mode() {
        return;
    }
    temp_name = safe_tempfile(&mut stream);
    if temp_name.is_null() {
        statusline(
            ALERT,
            dcgettext(
                0 as *const libc::c_char,
                b"Error writing temp file: %s\0" as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ),
            strerror(*__errno_location()),
        );
        return;
    }
    if !((*openfile).mark).is_null() {
        okay = write_region_to_file(temp_name, stream, 0 as libc::c_int != 0, OVERWRITE);
    } else {
        okay = write_file(
            temp_name,
            stream,
            0 as libc::c_int != 0,
            OVERWRITE,
            0 as libc::c_int != 0,
        );
    }
    if !okay {
        statusline(
            ALERT,
            dcgettext(
                0 as *const libc::c_char,
                b"Error writing temp file: %s\0" as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ),
            strerror(*__errno_location()),
        );
        unlink(temp_name);
        rpl_free(temp_name as *mut libc::c_void);
        return;
    }
    blank_bottombars();
    if !alt_speller.is_null() && *alt_speller as libc::c_int != 0 {
        treat(temp_name, alt_speller, 1 as libc::c_int != 0);
    } else {
        do_int_speller(temp_name);
    }
    unlink(temp_name);
    rpl_free(temp_name as *mut libc::c_void);
    currmenu = (1 as libc::c_int) << 0 as libc::c_int
        | (1 as libc::c_int) << 1 as libc::c_int | (1 as libc::c_int) << 2 as libc::c_int
        | (1 as libc::c_int) << 3 as libc::c_int | (1 as libc::c_int) << 4 as libc::c_int
        | (1 as libc::c_int) << 5 as libc::c_int | (1 as libc::c_int) << 6 as libc::c_int
        | (1 as libc::c_int) << 7 as libc::c_int
        | (1 as libc::c_int) << 11 as libc::c_int
        | (1 as libc::c_int) << 12 as libc::c_int
        | (1 as libc::c_int) << 15 as libc::c_int
        | (1 as libc::c_int) << 9 as libc::c_int
        | (1 as libc::c_int) << 14 as libc::c_int;
    shift_held = 1 as libc::c_int != 0;
}
pub unsafe extern "C" fn do_linter() {
    let mut lintings: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut pointer: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut onelint: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut pipesize: libc::c_long = 0;
    let mut buffersize: size_t = 0;
    let mut bytesread: size_t = 0;
    let mut totalread: size_t = 0;
    let mut parsesuccess: bool = 0 as libc::c_int != 0;
    let mut lint_status: libc::c_int = 0;
    let mut lint_fd: [libc::c_int; 2] = [0; 2];
    let mut pid_lint: pid_t = 0;
    let mut helpless: bool = flags[(NO_HELP as libc::c_int as libc::c_ulong)
        .wrapping_div(
            (::std::mem::size_of::<libc::c_uint>() as libc::c_ulong)
                .wrapping_mul(8 as libc::c_int as libc::c_ulong),
        ) as usize]
        & (1 as libc::c_int as libc::c_uint)
            << (NO_HELP as libc::c_int as libc::c_ulong)
                .wrapping_rem(
                    (::std::mem::size_of::<libc::c_uint>() as libc::c_ulong)
                        .wrapping_mul(8 as libc::c_int as libc::c_ulong),
                ) != 0 as libc::c_int as libc::c_uint;
    let mut lints: *mut lintstruct = 0 as *mut lintstruct;
    let mut tmplint: *mut lintstruct = 0 as *mut lintstruct;
    let mut curlint: *mut lintstruct = 0 as *mut lintstruct;
    let mut last_wait: time_t = 0 as libc::c_int as time_t;
    ran_a_tool = 1 as libc::c_int != 0;
    if in_restricted_mode() {
        return;
    }
    if ((*openfile).syntax).is_null() || ((*(*openfile).syntax).linter).is_null() {
        statusline(
            AHEM,
            dcgettext(
                0 as *const libc::c_char,
                b"No linter is defined for this type of file\0" as *const u8
                    as *const libc::c_char,
                5 as libc::c_int,
            ),
        );
        return;
    }
    (*openfile).mark = 0 as *mut linestruct;
    edit_refresh();
    if (*openfile).modified {
        let mut choice: libc::c_int = ask_user(
            0 as libc::c_int != 0,
            dcgettext(
                0 as *const libc::c_char,
                b"Save modified buffer before linting?\0" as *const u8
                    as *const libc::c_char,
                5 as libc::c_int,
            ),
        );
        if choice == -(1 as libc::c_int) {
            statusbar(
                dcgettext(
                    0 as *const libc::c_char,
                    b"Cancelled\0" as *const u8 as *const libc::c_char,
                    5 as libc::c_int,
                ),
            );
            return;
        } else if choice == 1 as libc::c_int
            && write_it_out(0 as libc::c_int != 0, 0 as libc::c_int != 0)
                != 1 as libc::c_int
        {
            return
        }
    }
    if pipe(lint_fd.as_mut_ptr()) == -(1 as libc::c_int) {
        statusline(
            ALERT,
            dcgettext(
                0 as *const libc::c_char,
                b"Could not create pipe: %s\0" as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ),
            strerror(*__errno_location()),
        );
        return;
    }
    blank_bottombars();
    currmenu = (1 as libc::c_int) << 14 as libc::c_int;
    statusbar(
        dcgettext(
            0 as *const libc::c_char,
            b"Invoking linter...\0" as *const u8 as *const libc::c_char,
            5 as libc::c_int,
        ),
    );
    pid_lint = fork();
    if pid_lint == 0 as libc::c_int {
        let mut lintargs: *mut *mut libc::c_char = 0 as *mut *mut libc::c_char;
        if dup2(lint_fd[1 as libc::c_int as usize], 1 as libc::c_int) < 0 as libc::c_int
        {
            exit(7 as libc::c_int);
        }
        if dup2(lint_fd[1 as libc::c_int as usize], 2 as libc::c_int) < 0 as libc::c_int
        {
            exit(8 as libc::c_int);
        }
        close(lint_fd[0 as libc::c_int as usize]);
        close(lint_fd[1 as libc::c_int as usize]);
        construct_argument_list(
            &mut lintargs,
            (*(*openfile).syntax).linter,
            (*openfile).filename,
        );
        execvp(
            *lintargs.offset(0 as libc::c_int as isize),
            lintargs as *const *mut libc::c_char,
        );
        exit(9 as libc::c_int);
    }
    close(lint_fd[1 as libc::c_int as usize]);
    if pid_lint < 0 as libc::c_int {
        statusline(
            ALERT,
            dcgettext(
                0 as *const libc::c_char,
                b"Could not fork: %s\0" as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ),
            strerror(*__errno_location()),
        );
        close(lint_fd[0 as libc::c_int as usize]);
        return;
    }
    pipesize = fpathconf(
        lint_fd[0 as libc::c_int as usize],
        _PC_PIPE_BUF as libc::c_int,
    );
    if pipesize < 1 as libc::c_int as libc::c_long {
        statusline(
            ALERT,
            dcgettext(
                0 as *const libc::c_char,
                b"Could not get size of pipe buffer\0" as *const u8
                    as *const libc::c_char,
                5 as libc::c_int,
            ),
        );
        close(lint_fd[0 as libc::c_int as usize]);
        return;
    }
    block_sigwinch(1 as libc::c_int != 0);
    totalread = 0 as libc::c_int as size_t;
    buffersize = (pipesize + 1 as libc::c_int as libc::c_long) as size_t;
    lintings = nmalloc(buffersize) as *mut libc::c_char;
    pointer = lintings;
    loop {
        bytesread = read(
            lint_fd[0 as libc::c_int as usize],
            pointer as *mut libc::c_void,
            pipesize as size_t,
        ) as size_t;
        if !(bytesread > 0 as libc::c_int as libc::c_ulong) {
            break;
        }
        totalread = (totalread as libc::c_ulong).wrapping_add(bytesread) as size_t
            as size_t;
        buffersize = (buffersize as libc::c_ulong)
            .wrapping_add(pipesize as libc::c_ulong) as size_t as size_t;
        lintings = nrealloc(lintings as *mut libc::c_void, buffersize)
            as *mut libc::c_char;
        pointer = lintings.offset(totalread as isize);
    }
    *pointer = '\0' as i32 as libc::c_char;
    close(lint_fd[0 as libc::c_int as usize]);
    block_sigwinch(0 as libc::c_int != 0);
    pointer = lintings;
    onelint = lintings;
    while *pointer as libc::c_int != '\0' as i32 {
        if *pointer as libc::c_int == '\r' as i32
            || *pointer as libc::c_int == '\n' as i32
        {
            *pointer = '\0' as i32 as libc::c_char;
            if onelint != pointer {
                let mut filename: *mut libc::c_char = 0 as *mut libc::c_char;
                let mut linestring: *mut libc::c_char = 0 as *mut libc::c_char;
                let mut colstring: *mut libc::c_char = 0 as *mut libc::c_char;
                let mut complaint: *mut libc::c_char = copy_of(onelint);
                let mut spacer: *mut libc::c_char = strstr(
                    complaint,
                    b" \0" as *const u8 as *const libc::c_char,
                );
                filename = strtok(onelint, b":\0" as *const u8 as *const libc::c_char);
                if !filename.is_null() && !spacer.is_null() {
                    linestring = strtok(
                        0 as *mut libc::c_char,
                        b":\0" as *const u8 as *const libc::c_char,
                    );
                    if !linestring.is_null() {
                        colstring = strtok(
                            0 as *mut libc::c_char,
                            b" \0" as *const u8 as *const libc::c_char,
                        );
                        if !colstring.is_null() {
                            let mut linenumber: ssize_t = strtol(
                                linestring,
                                0 as *mut *mut libc::c_char,
                                10 as libc::c_int,
                            );
                            let mut colnumber: ssize_t = strtol(
                                colstring,
                                0 as *mut *mut libc::c_char,
                                10 as libc::c_int,
                            );
                            if linenumber <= 0 as libc::c_int as libc::c_long {
                                rpl_free(complaint as *mut libc::c_void);
                                pointer = pointer.offset(1);
                                pointer;
                                continue;
                            } else {
                                if colnumber <= 0 as libc::c_int as libc::c_long {
                                    colnumber = 1 as libc::c_int as ssize_t;
                                    strtok(
                                        linestring,
                                        b",\0" as *const u8 as *const libc::c_char,
                                    );
                                    colstring = strtok(
                                        0 as *mut libc::c_char,
                                        b",\0" as *const u8 as *const libc::c_char,
                                    );
                                    if !colstring.is_null() {
                                        colnumber = strtol(
                                            colstring,
                                            0 as *mut *mut libc::c_char,
                                            10 as libc::c_int,
                                        );
                                    }
                                }
                                parsesuccess = 1 as libc::c_int != 0;
                                tmplint = curlint;
                                curlint = nmalloc(
                                    ::std::mem::size_of::<lintstruct>() as libc::c_ulong,
                                ) as *mut lintstruct;
                                (*curlint).next = 0 as *mut lintstruct;
                                (*curlint).prev = tmplint;
                                if !((*curlint).prev).is_null() {
                                    (*(*curlint).prev).next = curlint;
                                }
                                (*curlint).filename = copy_of(filename);
                                (*curlint).lineno = linenumber;
                                (*curlint).colno = colnumber;
                                (*curlint)
                                    .msg = copy_of(spacer.offset(1 as libc::c_int as isize));
                                if lints.is_null() {
                                    lints = curlint;
                                }
                            }
                        }
                    }
                }
                rpl_free(complaint as *mut libc::c_void);
            }
            onelint = pointer.offset(1 as libc::c_int as isize);
        }
        pointer = pointer.offset(1);
        pointer;
    }
    rpl_free(lintings as *mut libc::c_void);
    waitpid(pid_lint, &mut lint_status, 0 as libc::c_int);
    if !(lint_status & 0x7f as libc::c_int == 0 as libc::c_int)
        || (lint_status & 0xff00 as libc::c_int) >> 8 as libc::c_int > 2 as libc::c_int
    {
        statusline(
            ALERT,
            dcgettext(
                0 as *const libc::c_char,
                b"Error invoking '%s'\0" as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ),
            (*(*openfile).syntax).linter,
        );
        return;
    }
    if !parsesuccess {
        statusline(
            REMARK,
            dcgettext(
                0 as *const libc::c_char,
                b"Got 0 parsable lines from command: %s\0" as *const u8
                    as *const libc::c_char,
                5 as libc::c_int,
            ),
            (*(*openfile).syntax).linter,
        );
        return;
    }
    if helpless as libc::c_int != 0 && LINES > 5 as libc::c_int {
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
        window_init();
    }
    titlebar(0 as *const libc::c_char);
    bottombars((1 as libc::c_int) << 14 as libc::c_int);
    tmplint = 0 as *mut lintstruct;
    curlint = lints;
    loop {
        let mut kbinput: libc::c_int = 0;
        let mut function: functionptrtype = None;
        let mut lintfileinfo: stat = stat {
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
        if stat((*curlint).filename, &mut lintfileinfo) != -(1 as libc::c_int)
            && (((*openfile).statinfo).is_null()
                || (*(*openfile).statinfo).st_ino != lintfileinfo.st_ino)
        {
            let mut started_at: *const openfilestruct = openfile;
            openfile = (*openfile).next;
            while openfile != started_at as *mut openfilestruct
                && (((*openfile).statinfo).is_null()
                    || (*(*openfile).statinfo).st_ino != lintfileinfo.st_ino)
            {
                openfile = (*openfile).next;
            }
            if ((*openfile).statinfo).is_null()
                || (*(*openfile).statinfo).st_ino != lintfileinfo.st_ino
            {
                let mut msg: *mut libc::c_char = nmalloc(
                    (1024 as libc::c_int as libc::c_ulong)
                        .wrapping_add(strlen((*curlint).filename)),
                ) as *mut libc::c_char;
                let mut choice_0: libc::c_int = 0;
                sprintf(
                    msg,
                    dcgettext(
                        0 as *const libc::c_char,
                        b"This message is for unopened file %s, open it in a new buffer?\0"
                            as *const u8 as *const libc::c_char,
                        5 as libc::c_int,
                    ),
                    (*curlint).filename,
                );
                choice_0 = ask_user(0 as libc::c_int != 0, msg);
                currmenu = (1 as libc::c_int) << 14 as libc::c_int;
                rpl_free(msg as *mut libc::c_void);
                if choice_0 == -(1 as libc::c_int) {
                    statusbar(
                        dcgettext(
                            0 as *const libc::c_char,
                            b"Cancelled\0" as *const u8 as *const libc::c_char,
                            5 as libc::c_int,
                        ),
                    );
                    break;
                } else if choice_0 == 1 as libc::c_int {
                    open_buffer((*curlint).filename, 1 as libc::c_int != 0);
                } else {
                    let mut dontwantfile: *mut libc::c_char = copy_of(
                        (*curlint).filename,
                    );
                    let mut restlint: *mut lintstruct = 0 as *mut lintstruct;
                    while !curlint.is_null() {
                        if strcmp((*curlint).filename, dontwantfile) == 0 as libc::c_int
                        {
                            if curlint == lints {
                                lints = (*curlint).next;
                            } else {
                                (*(*curlint).prev).next = (*curlint).next;
                            }
                            if !((*curlint).next).is_null() {
                                (*(*curlint).next).prev = (*curlint).prev;
                            }
                            tmplint = curlint;
                            curlint = (*curlint).next;
                            rpl_free((*tmplint).msg as *mut libc::c_void);
                            rpl_free((*tmplint).filename as *mut libc::c_void);
                            rpl_free(tmplint as *mut libc::c_void);
                        } else {
                            if restlint.is_null() {
                                restlint = curlint;
                            }
                            curlint = (*curlint).next;
                        }
                    }
                    rpl_free(dontwantfile as *mut libc::c_void);
                    if restlint.is_null() {
                        statusline(
                            REMARK,
                            dcgettext(
                                0 as *const libc::c_char,
                                b"No messages for this file\0" as *const u8
                                    as *const libc::c_char,
                                5 as libc::c_int,
                            ),
                        );
                        break;
                    } else {
                        curlint = restlint;
                        continue;
                    }
                }
            }
        }
        if tmplint != curlint {
            goto_line_posx(
                (*curlint).lineno,
                ((*curlint).colno - 1 as libc::c_int as libc::c_long) as size_t,
            );
            (*openfile)
                .current_x = actual_x(
                (*(*openfile).current).data,
                (*openfile).placewewant,
            );
            titlebar(0 as *const libc::c_char);
            adjust_viewport(CENTERING);
            confirm_margin();
            edit_refresh();
            statusline(NOTICE, (*curlint).msg);
            bottombars((1 as libc::c_int) << 14 as libc::c_int);
        }
        place_the_cursor();
        wnoutrefresh(midwin);
        kbinput = get_kbinput(footwin, 1 as libc::c_int != 0);
        if kbinput == -(2 as libc::c_int) {
            continue;
        }
        function = func_from_key(kbinput);
        tmplint = curlint;
        if function == Some(do_cancel as unsafe extern "C" fn() -> ())
            || function == Some(do_enter as unsafe extern "C" fn() -> ())
        {
            wipe_statusbar();
            break;
        } else if function == Some(do_help as unsafe extern "C" fn() -> ()) {
            tmplint = 0 as *mut lintstruct;
            do_help();
        } else if function == Some(do_page_up as unsafe extern "C" fn() -> ())
            || function == Some(to_prev_block as unsafe extern "C" fn() -> ())
        {
            if !((*curlint).prev).is_null() {
                curlint = (*curlint).prev;
            } else if last_wait != time(0 as *mut time_t) {
                statusbar(
                    dcgettext(
                        0 as *const libc::c_char,
                        b"At first message\0" as *const u8 as *const libc::c_char,
                        5 as libc::c_int,
                    ),
                );
                beep();
                napms(600 as libc::c_int);
                last_wait = time(0 as *mut time_t);
                statusline(NOTICE, (*curlint).msg);
            }
        } else if function == Some(do_page_down as unsafe extern "C" fn() -> ())
            || function == Some(to_next_block as unsafe extern "C" fn() -> ())
        {
            if !((*curlint).next).is_null() {
                curlint = (*curlint).next;
            } else if last_wait != time(0 as *mut time_t) {
                statusbar(
                    dcgettext(
                        0 as *const libc::c_char,
                        b"At last message\0" as *const u8 as *const libc::c_char,
                        5 as libc::c_int,
                    ),
                );
                beep();
                napms(600 as libc::c_int);
                last_wait = time(0 as *mut time_t);
                statusline(NOTICE, (*curlint).msg);
            }
        } else {
            beep();
        }
    }
    curlint = lints;
    while !curlint.is_null() {
        tmplint = curlint;
        curlint = (*curlint).next;
        rpl_free((*tmplint).msg as *mut libc::c_void);
        rpl_free((*tmplint).filename as *mut libc::c_void);
        rpl_free(tmplint as *mut libc::c_void);
    }
    if helpless {
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
        refresh_needed = 1 as libc::c_int != 0;
    }
    lastmessage = VACUUM;
    currmenu = (1 as libc::c_int) << 0 as libc::c_int
        | (1 as libc::c_int) << 1 as libc::c_int | (1 as libc::c_int) << 2 as libc::c_int
        | (1 as libc::c_int) << 3 as libc::c_int | (1 as libc::c_int) << 4 as libc::c_int
        | (1 as libc::c_int) << 5 as libc::c_int | (1 as libc::c_int) << 6 as libc::c_int
        | (1 as libc::c_int) << 7 as libc::c_int
        | (1 as libc::c_int) << 11 as libc::c_int
        | (1 as libc::c_int) << 12 as libc::c_int
        | (1 as libc::c_int) << 15 as libc::c_int
        | (1 as libc::c_int) << 9 as libc::c_int
        | (1 as libc::c_int) << 14 as libc::c_int;
    titlebar(0 as *const libc::c_char);
}
pub unsafe extern "C" fn do_formatter() {
    let mut stream: *mut FILE = 0 as *mut FILE;
    let mut temp_name: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut okay: bool = 0 as libc::c_int != 0;
    ran_a_tool = 1 as libc::c_int != 0;
    if in_restricted_mode() {
        return;
    }
    if ((*openfile).syntax).is_null() || ((*(*openfile).syntax).formatter).is_null() {
        statusline(
            AHEM,
            dcgettext(
                0 as *const libc::c_char,
                b"No formatter is defined for this type of file\0" as *const u8
                    as *const libc::c_char,
                5 as libc::c_int,
            ),
        );
        return;
    }
    (*openfile).mark = 0 as *mut linestruct;
    temp_name = safe_tempfile(&mut stream);
    if !temp_name.is_null() {
        okay = write_file(
            temp_name,
            stream,
            0 as libc::c_int != 0,
            OVERWRITE,
            0 as libc::c_int != 0,
        );
    }
    if !okay {
        statusline(
            ALERT,
            dcgettext(
                0 as *const libc::c_char,
                b"Error writing temp file: %s\0" as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ),
            strerror(*__errno_location()),
        );
    } else {
        treat(temp_name, (*(*openfile).syntax).formatter, 0 as libc::c_int != 0);
    }
    unlink(temp_name);
    rpl_free(temp_name as *mut libc::c_void);
}
pub unsafe extern "C" fn count_lines_words_and_characters() {
    let mut was_current: *mut linestruct = (*openfile).current;
    let mut was_x: size_t = (*openfile).current_x;
    let mut topline: *mut linestruct = 0 as *mut linestruct;
    let mut botline: *mut linestruct = 0 as *mut linestruct;
    let mut top_x: size_t = 0;
    let mut bot_x: size_t = 0;
    let mut words: size_t = 0 as libc::c_int as size_t;
    let mut chars: size_t = 0 as libc::c_int as size_t;
    let mut lines: ssize_t = 0 as libc::c_int as ssize_t;
    if !((*openfile).mark).is_null() {
        get_region(&mut topline, &mut top_x, &mut botline, &mut bot_x);
        if topline != botline {
            chars = (number_of_characters_in((*topline).next, botline))
                .wrapping_add(1 as libc::c_int as libc::c_ulong);
        }
        chars = (chars as libc::c_ulong)
            .wrapping_add(
                (mbstrlen(((*topline).data).offset(top_x as isize)))
                    .wrapping_sub(mbstrlen(((*botline).data).offset(bot_x as isize))),
            ) as size_t as size_t;
    } else {
        topline = (*openfile).filetop;
        top_x = 0 as libc::c_int as size_t;
        botline = (*openfile).filebot;
        bot_x = strlen((*botline).data);
        chars = (*openfile).totsize;
    }
    lines = (*botline).lineno - (*topline).lineno;
    lines
        += (if bot_x == 0 as libc::c_int as libc::c_ulong
            || topline == botline && top_x == bot_x
        {
            0 as libc::c_int
        } else {
            1 as libc::c_int
        }) as libc::c_long;
    (*openfile).current = topline;
    (*openfile).current_x = top_x;
    while (*(*openfile).current).lineno < (*botline).lineno
        || (*openfile).current == botline && (*openfile).current_x < bot_x
    {
        if do_next_word(0 as libc::c_int != 0) {
            words = words.wrapping_add(1);
            words;
        }
    }
    (*openfile).current = was_current;
    (*openfile).current_x = was_x;
    statusline(
        INFO,
        dcgettext(
            0 as *const libc::c_char,
            b"%s%zd %s,  %zu %s,  %zu %s\0" as *const u8 as *const libc::c_char,
            5 as libc::c_int,
        ),
        if !((*openfile).mark).is_null() {
            dcgettext(
                0 as *const libc::c_char,
                b"In Selection:  \0" as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ) as *const libc::c_char
        } else {
            b"\0" as *const u8 as *const libc::c_char
        },
        lines,
        dcngettext(
            0 as *const libc::c_char,
            b"line\0" as *const u8 as *const libc::c_char,
            b"lines\0" as *const u8 as *const libc::c_char,
            lines as libc::c_ulong,
            5 as libc::c_int,
        ),
        words,
        dcngettext(
            0 as *const libc::c_char,
            b"word\0" as *const u8 as *const libc::c_char,
            b"words\0" as *const u8 as *const libc::c_char,
            words,
            5 as libc::c_int,
        ),
        chars,
        dcngettext(
            0 as *const libc::c_char,
            b"character\0" as *const u8 as *const libc::c_char,
            b"characters\0" as *const u8 as *const libc::c_char,
            chars,
            5 as libc::c_int,
        ),
    );
}
pub unsafe extern "C" fn do_verbatim_input() {
    let mut count: size_t = 1 as libc::c_int as size_t;
    let mut bytes: *mut libc::c_char = 0 as *mut libc::c_char;
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
        && (*openfile).current_y == (editwinrows - 1 as libc::c_int) as libc::c_long
        && LINES > 1 as libc::c_int
    {
        edit_scroll(1 as libc::c_int != 0);
        edit_refresh();
    }
    statusline(
        INFO,
        dcgettext(
            0 as *const libc::c_char,
            b"Verbatim Input\0" as *const u8 as *const libc::c_char,
            5 as libc::c_int,
        ),
    );
    place_the_cursor();
    bytes = get_verbatim_kbinput(midwin, &mut count);
    if count > 0 as libc::c_int as libc::c_ulong {
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
                        ) != 0 as libc::c_int as libc::c_uint
        {
            lastmessage = VACUUM;
        }
        if count < 999 as libc::c_int as libc::c_ulong {
            inject(bytes, count);
        }
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
            && currmenu == (1 as libc::c_int) << 0 as libc::c_int
        {
            wredrawln(midwin, editwinrows - 1 as libc::c_int, 1 as libc::c_int);
        } else {
            wipe_statusbar();
        }
    } else {
        statusline(
            AHEM,
            dcgettext(
                0 as *const libc::c_char,
                b"Invalid code\0" as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ),
        );
    }
    rpl_free(bytes as *mut libc::c_void);
}
pub unsafe extern "C" fn copy_completion(
    mut text: *mut libc::c_char,
) -> *mut libc::c_char {
    let mut word: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut length: size_t = 0 as libc::c_int as size_t;
    let mut index: size_t = 0 as libc::c_int as size_t;
    while is_word_char(&mut *text.offset(length as isize), 0 as libc::c_int != 0) {
        length = step_right(text, length);
    }
    word = nmalloc(length.wrapping_add(1 as libc::c_int as libc::c_ulong))
        as *mut libc::c_char;
    while index < length {
        let fresh12 = text;
        text = text.offset(1);
        let fresh13 = index;
        index = index.wrapping_add(1);
        *word.offset(fresh13 as isize) = *fresh12;
    }
    *word.offset(index as isize) = '\0' as i32 as libc::c_char;
    return word;
}
pub unsafe extern "C" fn complete_a_word() {
    static mut scouring: *mut openfilestruct = 0 as *const openfilestruct
        as *mut openfilestruct;
    let mut shard: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut completion: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut start_of_shard: size_t = 0;
    let mut shard_length: size_t = 0 as libc::c_int as size_t;
    let mut i: size_t = 0 as libc::c_int as size_t;
    let mut j: size_t = 0 as libc::c_int as size_t;
    let mut some_word: *mut completionstruct = 0 as *mut completionstruct;
    let mut was_set_wrapping: bool = flags[(BREAK_LONG_LINES as libc::c_int
        as libc::c_ulong)
        .wrapping_div(
            (::std::mem::size_of::<libc::c_uint>() as libc::c_ulong)
                .wrapping_mul(8 as libc::c_int as libc::c_ulong),
        ) as usize]
        & (1 as libc::c_int as libc::c_uint)
            << (BREAK_LONG_LINES as libc::c_int as libc::c_ulong)
                .wrapping_rem(
                    (::std::mem::size_of::<libc::c_uint>() as libc::c_ulong)
                        .wrapping_mul(8 as libc::c_int as libc::c_ulong),
                ) != 0 as libc::c_int as libc::c_uint;
    if pletion_line.is_null() {
        while !list_of_completions.is_null() {
            let mut dropit: *mut completionstruct = list_of_completions;
            list_of_completions = (*list_of_completions).next;
            rpl_free((*dropit).word as *mut libc::c_void);
            rpl_free(dropit as *mut libc::c_void);
        }
        (*openfile).last_action = OTHER;
        scouring = openfile;
        pletion_line = (*openfile).filetop;
        pletion_x = 0 as libc::c_int;
        wipe_statusbar();
    } else {
        do_undo();
    }
    start_of_shard = (*openfile).current_x;
    while start_of_shard > 0 as libc::c_int as libc::c_ulong {
        let mut oneleft: size_t = step_left((*(*openfile).current).data, start_of_shard);
        if !is_word_char(
            &mut *((*(*openfile).current).data).offset(oneleft as isize),
            0 as libc::c_int != 0,
        ) {
            break;
        }
        start_of_shard = oneleft;
    }
    if start_of_shard == (*openfile).current_x {
        statusline(
            AHEM,
            dcgettext(
                0 as *const libc::c_char,
                b"No word fragment\0" as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ),
        );
        pletion_line = 0 as *mut linestruct;
        return;
    }
    shard = nmalloc(
        ((*openfile).current_x)
            .wrapping_sub(start_of_shard)
            .wrapping_add(1 as libc::c_int as libc::c_ulong),
    ) as *mut libc::c_char;
    while start_of_shard < (*openfile).current_x {
        let fresh14 = start_of_shard;
        start_of_shard = start_of_shard.wrapping_add(1);
        let fresh15 = shard_length;
        shard_length = shard_length.wrapping_add(1);
        *shard
            .offset(
                fresh15 as isize,
            ) = *((*(*openfile).current).data).offset(fresh14 as isize);
    }
    *shard.offset(shard_length as isize) = '\0' as i32 as libc::c_char;
    while !pletion_line.is_null() {
        let mut threshold: ssize_t = (strlen((*pletion_line).data))
            .wrapping_sub(shard_length)
            .wrapping_sub(1 as libc::c_int as libc::c_ulong) as ssize_t;
        i = pletion_x as size_t;
        while (i as ssize_t) < threshold {
            if !(*((*pletion_line).data).offset(i as isize) as libc::c_int
                != *shard.offset(0 as libc::c_int as isize) as libc::c_int)
            {
                j = 1 as libc::c_int as size_t;
                while j < shard_length {
                    if *((*pletion_line).data).offset(i.wrapping_add(j) as isize)
                        as libc::c_int != *shard.offset(j as isize) as libc::c_int
                    {
                        break;
                    }
                    j = j.wrapping_add(1);
                    j;
                }
                if !(j < shard_length) {
                    if is_word_char(
                        &mut *((*pletion_line).data).offset(i.wrapping_add(j) as isize),
                        0 as libc::c_int != 0,
                    ) {
                        if !(i > 0 as libc::c_int as libc::c_ulong
                            && is_word_char(
                                &mut *((*pletion_line).data)
                                    .offset(
                                        (step_left
                                            as unsafe extern "C" fn(
                                                *const libc::c_char,
                                                size_t,
                                            ) -> size_t)((*pletion_line).data, i) as isize,
                                    ),
                                0 as libc::c_int != 0,
                            ) as libc::c_int != 0)
                        {
                            if !(pletion_line == (*openfile).current
                                && i == ((*openfile).current_x).wrapping_sub(shard_length))
                            {
                                completion = copy_completion(
                                    ((*pletion_line).data).offset(i as isize),
                                );
                                some_word = list_of_completions;
                                while !some_word.is_null()
                                    && strcmp((*some_word).word, completion) != 0 as libc::c_int
                                {
                                    some_word = (*some_word).next;
                                }
                                if !some_word.is_null() {
                                    rpl_free(completion as *mut libc::c_void);
                                } else {
                                    some_word = nmalloc(
                                        ::std::mem::size_of::<completionstruct>() as libc::c_ulong,
                                    ) as *mut completionstruct;
                                    (*some_word).word = completion;
                                    (*some_word).next = list_of_completions;
                                    list_of_completions = some_word;
                                    flags[(BREAK_LONG_LINES as libc::c_int as libc::c_ulong)
                                        .wrapping_div(
                                            (::std::mem::size_of::<libc::c_uint>() as libc::c_ulong)
                                                .wrapping_mul(8 as libc::c_int as libc::c_ulong),
                                        ) as usize]
                                        &= !((1 as libc::c_int as libc::c_uint)
                                            << (BREAK_LONG_LINES as libc::c_int as libc::c_ulong)
                                                .wrapping_rem(
                                                    (::std::mem::size_of::<libc::c_uint>() as libc::c_ulong)
                                                        .wrapping_mul(8 as libc::c_int as libc::c_ulong),
                                                ));
                                    inject(
                                        &mut *completion.offset(shard_length as isize),
                                        (strlen(completion)).wrapping_sub(shard_length),
                                    );
                                    if was_set_wrapping {
                                        flags[(BREAK_LONG_LINES as libc::c_int as libc::c_ulong)
                                            .wrapping_div(
                                                (::std::mem::size_of::<libc::c_uint>() as libc::c_ulong)
                                                    .wrapping_mul(8 as libc::c_int as libc::c_ulong),
                                            ) as usize]
                                            |= (1 as libc::c_int as libc::c_uint)
                                                << (BREAK_LONG_LINES as libc::c_int as libc::c_ulong)
                                                    .wrapping_rem(
                                                        (::std::mem::size_of::<libc::c_uint>() as libc::c_ulong)
                                                            .wrapping_mul(8 as libc::c_int as libc::c_ulong),
                                                    );
                                        do_wrap();
                                    }
                                    i = i.wrapping_add(1);
                                    pletion_x = i as libc::c_int;
                                    rpl_free(shard as *mut libc::c_void);
                                    return;
                                }
                            }
                        }
                    }
                }
            }
            i = i.wrapping_add(1);
            i;
        }
        pletion_line = (*pletion_line).next;
        pletion_x = 0 as libc::c_int;
        if pletion_line.is_null() && (*scouring).next != openfile {
            scouring = (*scouring).next;
            pletion_line = (*scouring).filetop;
        }
    }
    if !list_of_completions.is_null() {
        edit_refresh();
        statusline(
            AHEM,
            dcgettext(
                0 as *const libc::c_char,
                b"No further matches\0" as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ),
        );
    } else {
        statusline(
            AHEM,
            dcgettext(
                0 as *const libc::c_char,
                b"No matches\0" as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ),
        );
    }
    rpl_free(shard as *mut libc::c_void);
}
