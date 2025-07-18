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
    fn delwin(_: *mut WINDOW) -> libc::c_int;
    fn doupdate() -> libc::c_int;
    fn endwin() -> libc::c_int;
    fn has_colors() -> bool;
    fn initscr() -> *mut WINDOW;
    fn keypad(_: *mut WINDOW, _: bool) -> libc::c_int;
    fn newwin(
        _: libc::c_int,
        _: libc::c_int,
        _: libc::c_int,
        _: libc::c_int,
    ) -> *mut WINDOW;
    fn noecho() -> libc::c_int;
    fn nonl() -> libc::c_int;
    fn raw() -> libc::c_int;
    fn start_color() -> libc::c_int;
    fn ungetch(_: libc::c_int) -> libc::c_int;
    fn wnoutrefresh(_: *mut WINDOW) -> libc::c_int;
    fn wredrawln(_: *mut WINDOW, _: libc::c_int, _: libc::c_int) -> libc::c_int;
    fn wrefresh(_: *mut WINDOW) -> libc::c_int;
    fn tigetstr(_: *const libc::c_char) -> *mut libc::c_char;
    fn key_defined(_: *const libc::c_char) -> libc::c_int;
    fn set_escdelay(_: libc::c_int) -> libc::c_int;
    static mut stdscr: *mut WINDOW;
    static mut COLS: libc::c_int;
    static mut LINES: libc::c_int;
    fn chmod(__file: *const libc::c_char, __mode: __mode_t) -> libc::c_int;
    static mut stdout: *mut FILE;
    static mut stderr: *mut FILE;
    fn fflush(__stream: *mut FILE) -> libc::c_int;
    fn fopen(_: *const libc::c_char, _: *const libc::c_char) -> *mut FILE;
    fn fprintf(_: *mut FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
    fn sprintf(_: *mut libc::c_char, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn vfprintf(
        _: *mut FILE,
        _: *const libc::c_char,
        _: ::std::ffi::VaList,
    ) -> libc::c_int;
    fn rpl_regerror(
        __errcode: libc::c_int,
        __preg: *const regex_t,
        __errbuf: *mut libc::c_char,
        __errbuf_size: size_t,
    ) -> size_t;
    fn rpl_regcomp(
        __preg: *mut regex_t,
        __pattern: *const libc::c_char,
        __cflags: libc::c_int,
    ) -> libc::c_int;
    fn sigaction(
        __sig: libc::c_int,
        __act: *const sigaction,
        __oact: *mut sigaction,
    ) -> libc::c_int;
    fn sigprocmask(
        __how: libc::c_int,
        __set: *const sigset_t,
        __oset: *mut sigset_t,
    ) -> libc::c_int;
    fn sigaddset(__set: *mut sigset_t, __signo: libc::c_int) -> libc::c_int;
    fn sigfillset(__set: *mut sigset_t) -> libc::c_int;
    fn sigemptyset(__set: *mut sigset_t) -> libc::c_int;
    fn kill(__pid: __pid_t, __sig: libc::c_int) -> libc::c_int;
    fn rpl_free(ptr: *mut libc::c_void);
    fn putenv(__string: *mut libc::c_char) -> libc::c_int;
    fn getenv(__name: *const libc::c_char) -> *mut libc::c_char;
    fn exit(_: libc::c_int) -> !;
    fn mousemask(_: mmask_t, _: *mut mmask_t) -> mmask_t;
    fn mouseinterval(_: libc::c_int) -> libc::c_int;
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
    fn textdomain(__domainname: *const libc::c_char) -> *mut libc::c_char;
    fn bindtextdomain(
        __domainname: *const libc::c_char,
        __dirname: *const libc::c_char,
    ) -> *mut libc::c_char;
    fn setlocale(
        __category: libc::c_int,
        __locale: *const libc::c_char,
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
    static mut ran_a_tool: bool;
    static mut focusing: bool;
    static mut as_an_at: bool;
    static mut control_C_was_pressed: bool;
    static mut lastmessage: message_type;
    static mut pletion_line: *mut linestruct;
    static mut also_the_last: bool;
    static mut last_search: *mut libc::c_char;
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
    static mut fill: ssize_t;
    static mut wrap_at: size_t;
    static mut topwin: *mut WINDOW;
    static mut midwin: *mut WINDOW;
    static mut footwin: *mut WINDOW;
    static mut editwinrows: libc::c_int;
    static mut editwincols: libc::c_int;
    static mut margin: libc::c_int;
    static mut thebar: libc::c_int;
    static mut bardata: *mut libc::c_int;
    static mut stripe_column: ssize_t;
    static mut cutbuffer: *mut linestruct;
    static mut keep_cutbuffer: bool;
    static mut openfile: *mut openfilestruct;
    static mut matchbrackets: *mut libc::c_char;
    static mut whitespace: *mut libc::c_char;
    static mut whitelen: [libc::c_int; 2];
    static mut punct: *mut libc::c_char;
    static mut brackets: *mut libc::c_char;
    static mut quotestr: *mut libc::c_char;
    static mut quotereg: regex_t;
    static mut word_chars: *mut libc::c_char;
    static mut tabsize: ssize_t;
    static mut backup_dir: *mut libc::c_char;
    static mut operating_dir: *mut libc::c_char;
    static mut alt_speller: *mut libc::c_char;
    static mut syntaxstr: *mut libc::c_char;
    static mut rescind_colors: bool;
    static mut refresh_needed: bool;
    static mut currmenu: libc::c_int;
    static mut hilite_attribute: libc::c_int;
    static mut interface_color_pair: [libc::c_int; 12];
    static mut startup_problem: *mut libc::c_char;
    static mut custom_nanorc: *mut libc::c_char;
    static mut commandname: *mut libc::c_char;
    fn utf8_init();
    fn using_utf8() -> bool;
    fn mbstrlen(pointer: *const libc::c_char) -> size_t;
    fn set_interface_colorpairs();
    fn find_and_prime_applicable_syntax();
    fn check_the_multis(line: *mut linestruct);
    fn precalc_multicolorinfo();
    fn do_delete();
    fn do_backspace();
    fn chop_previous_word();
    fn chop_next_word();
    fn cut_text();
    fn cut_till_eof();
    fn zap_text();
    fn copy_text();
    fn paste_text();
    fn make_new_buffer();
    fn delete_lockfile(lockfilename: *const libc::c_char) -> bool;
    fn open_buffer(filename: *const libc::c_char, new_one: bool) -> bool;
    fn set_modified();
    fn prepare_for_display();
    fn mention_name_and_linecount();
    fn switch_to_next_buffer();
    fn close_buffer();
    fn read_file(
        f: *mut FILE,
        fd: libc::c_int,
        filename: *const libc::c_char,
        undoable: bool,
    );
    fn get_next_filename(
        name: *const libc::c_char,
        suffix: *const libc::c_char,
    ) -> *mut libc::c_char;
    fn do_execute();
    fn init_operating_dir();
    fn init_backup_dir();
    fn write_file(
        name: *const libc::c_char,
        thefile: *mut FILE,
        normal: bool,
        method: kind_of_writing_type,
        annotate: bool,
    ) -> bool;
    fn write_it_out(exiting: bool, withprompt: bool) -> libc::c_int;
    fn do_writeout();
    fn do_savefile();
    fn first_sc_for(
        menu: libc::c_int,
        function: Option::<unsafe extern "C" fn() -> ()>,
    ) -> *const keystruct;
    fn get_shortcut(keycode: libc::c_int) -> *const keystruct;
    fn shortcut_init();
    fn epithet_of_flag(flag: libc::c_int) -> *const libc::c_char;
    fn do_help();
    fn history_init();
    fn have_statedir() -> bool;
    fn load_history();
    fn save_history();
    fn load_poshistory();
    fn update_poshistory();
    fn has_old_position(
        file: *const libc::c_char,
        line: *mut ssize_t,
        column: *mut ssize_t,
    ) -> bool;
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
    fn do_left();
    fn do_right();
    fn ask_user(withall: bool, question: *const libc::c_char) -> libc::c_int;
    fn digits(n: ssize_t) -> libc::c_int;
    fn nrealloc(ptr: *mut libc::c_void, howmuch: size_t) -> *mut libc::c_void;
    fn nmalloc(howmuch: size_t) -> *mut libc::c_void;
    fn do_verbatim_input();
    fn do_replace();
    fn complete_a_word();
    fn do_formatter();
    fn do_spell();
    fn do_comment();
    fn do_full_justify();
    fn do_justify();
    fn do_unindent();
    fn do_indent();
    fn do_tab();
    fn do_enter();
    fn copy_of(string: *const libc::c_char) -> *mut libc::c_char;
    fn xplustabs() -> size_t;
    fn do_wrap();
    fn update_undo(action: undo_type);
    fn new_magicline();
    fn add_undo(action: undo_type, message: *const libc::c_char);
    fn put_cursor_at_end_of_answer();
    fn do_rcfiles();
    fn regexp_init(regexp: *const libc::c_char) -> bool;
    fn tidy_up_after_search();
    fn findnextstr(
        needle: *const libc::c_char,
        whole_word_only: bool,
        modus: libc::c_int,
        match_len: *mut size_t,
        skipone: bool,
        begin: *const linestruct,
        begin_x: size_t,
    ) -> libc::c_int;
    fn not_found_msg(str: *const libc::c_char);
    fn goto_line_and_column(
        line: ssize_t,
        column: ssize_t,
        retain_answer: bool,
        interactive: bool,
    );
    fn do_mark();
    fn tail(path: *const libc::c_char) -> *const libc::c_char;
    fn parse_num(str: *const libc::c_char, result: *mut ssize_t) -> bool;
    fn parse_line_column(
        str: *const libc::c_char,
        line: *mut ssize_t,
        column: *mut ssize_t,
    ) -> bool;
    fn mallocstrcpy(
        dest: *mut libc::c_char,
        src: *const libc::c_char,
    ) -> *mut libc::c_char;
    fn get_page_start(column: size_t) -> size_t;
    fn actual_x(text: *const libc::c_char, column: size_t) -> size_t;
    fn breadth(text: *const libc::c_char) -> size_t;
    fn display_rcfile_errors();
    fn wipe_statusbar();
    fn minibar();
    fn get_mouseinput(
        mouse_y: *mut libc::c_int,
        mouse_x: *mut libc::c_int,
        allow_shortcuts: bool,
    ) -> libc::c_int;
    fn place_the_cursor();
    fn go_back_chunks(
        nrows: libc::c_int,
        line: *mut *mut linestruct,
        leftedge: *mut size_t,
    ) -> libc::c_int;
    fn go_forward_chunks(
        nrows: libc::c_int,
        line: *mut *mut linestruct,
        leftedge: *mut size_t,
    ) -> libc::c_int;
    fn edit_scroll(direction: bool);
    fn leftedge_for(column: size_t, line: *mut linestruct) -> size_t;
    fn actual_last_column(leftedge: size_t, column: size_t) -> size_t;
    fn edit_redraw(old_current: *mut linestruct, manner: update_type);
    fn edit_refresh();
    fn adjust_viewport(manner: update_type);
    fn blank_statusbar();
    fn titlebar(path: *const libc::c_char);
    fn statusbar(msg: *const libc::c_char);
    fn warn_and_briefly_pause(msg: *const libc::c_char);
    fn set_blankdelay_to_one();
    fn update_line(line: *mut linestruct, index: size_t) -> libc::c_int;
    fn chunk_for(column: size_t, line: *mut linestruct) -> size_t;
    fn extra_chunks_in(line: *mut linestruct) -> size_t;
    fn statusline(importance: message_type, msg: *const libc::c_char, _: ...);
    fn bottombars(menu: libc::c_int);
    fn blank_bottombars();
    fn waiting_keycodes() -> size_t;
    fn implant(string: *const libc::c_char);
    fn get_kbinput(win: *mut WINDOW, showcursor: bool) -> libc::c_int;
    fn ensure_firstcolumn_is_aligned();
    fn report_cursor_position();
    fn draw_all_subwindows();
    fn do_toggle();
    fn __ctype_b_loc() -> *mut *const libc::c_ushort;
    fn __ctype_toupper_loc() -> *mut *const __int32_t;
    fn __errno_location() -> *mut libc::c_int;
    fn open(__file: *const libc::c_char, __oflag: libc::c_int, _: ...) -> libc::c_int;
    fn rpl_fcntl(fd: libc::c_int, action: libc::c_int, _: ...) -> libc::c_int;
    fn getopt_long(
        ___argc: libc::c_int,
        ___argv: *const *mut libc::c_char,
        __shortopts: *const libc::c_char,
        __longopts: *const option,
        __longind: *mut libc::c_int,
    ) -> libc::c_int;
    static mut optarg: *mut libc::c_char;
    static mut optind: libc::c_int;
    fn ioctl(__fd: libc::c_int, __request: libc::c_ulong, _: ...) -> libc::c_int;
    fn nl_langinfo(__item: nl_item) -> *mut libc::c_char;
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
    fn strncpy(
        _: *mut libc::c_char,
        _: *const libc::c_char,
        _: libc::c_ulong,
    ) -> *mut libc::c_char;
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    fn strerror(_: libc::c_int) -> *mut libc::c_char;
    fn tcgetattr(__fd: libc::c_int, __termios_p: *mut termios) -> libc::c_int;
    fn tcsetattr(
        __fd: libc::c_int,
        __optional_actions: libc::c_int,
        __termios_p: *const termios,
    ) -> libc::c_int;
    fn close(__fd: libc::c_int) -> libc::c_int;
    fn chown(
        __file: *const libc::c_char,
        __owner: __uid_t,
        __group: __gid_t,
    ) -> libc::c_int;
    fn dup2(__fd: libc::c_int, __fd2: libc::c_int) -> libc::c_int;
    fn getpid() -> __pid_t;
    fn isatty(__fd: libc::c_int) -> libc::c_int;
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
pub type __uint32_t = libc::c_uint;
pub type __dev_t = libc::c_ulong;
pub type __uid_t = libc::c_uint;
pub type __gid_t = libc::c_uint;
pub type __ino_t = libc::c_ulong;
pub type __mode_t = libc::c_uint;
pub type __nlink_t = libc::c_ulong;
pub type __off_t = libc::c_long;
pub type __off64_t = libc::c_long;
pub type __pid_t = libc::c_int;
pub type __clock_t = libc::c_long;
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
pub struct __sigset_t {
    pub __val: [libc::c_ulong; 16],
}
pub type sigset_t = __sigset_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct timespec {
    pub tv_sec: __time_t,
    pub tv_nsec: __syscall_slong_t,
}
pub type sig_atomic_t = __sig_atomic_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub union sigval {
    pub sival_int: libc::c_int,
    pub sival_ptr: *mut libc::c_void,
}
pub type __sigval_t = sigval;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct siginfo_t {
    pub si_signo: libc::c_int,
    pub si_errno: libc::c_int,
    pub si_code: libc::c_int,
    pub __pad0: libc::c_int,
    pub _sifields: C2RustUnnamed,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed {
    pub _pad: [libc::c_int; 28],
    pub _kill: C2RustUnnamed_8,
    pub _timer: C2RustUnnamed_7,
    pub _rt: C2RustUnnamed_6,
    pub _sigchld: C2RustUnnamed_5,
    pub _sigfault: C2RustUnnamed_2,
    pub _sigpoll: C2RustUnnamed_1,
    pub _sigsys: C2RustUnnamed_0,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_0 {
    pub _call_addr: *mut libc::c_void,
    pub _syscall: libc::c_int,
    pub _arch: libc::c_uint,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_1 {
    pub si_band: libc::c_long,
    pub si_fd: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_2 {
    pub si_addr: *mut libc::c_void,
    pub si_addr_lsb: libc::c_short,
    pub _bounds: C2RustUnnamed_3,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_3 {
    pub _addr_bnd: C2RustUnnamed_4,
    pub _pkey: __uint32_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_4 {
    pub _lower: *mut libc::c_void,
    pub _upper: *mut libc::c_void,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_5 {
    pub si_pid: __pid_t,
    pub si_uid: __uid_t,
    pub si_status: libc::c_int,
    pub si_utime: __clock_t,
    pub si_stime: __clock_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_6 {
    pub si_pid: __pid_t,
    pub si_uid: __uid_t,
    pub si_sigval: __sigval_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_7 {
    pub si_tid: libc::c_int,
    pub si_overrun: libc::c_int,
    pub si_sigval: __sigval_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_8 {
    pub si_pid: __pid_t,
    pub si_uid: __uid_t,
}
pub type __sighandler_t = Option::<unsafe extern "C" fn(libc::c_int) -> ()>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sigaction {
    pub __sigaction_handler: C2RustUnnamed_9,
    pub sa_mask: __sigset_t,
    pub sa_flags: libc::c_int,
    pub sa_restorer: Option::<unsafe extern "C" fn() -> ()>,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_9 {
    pub sa_handler: __sighandler_t,
    pub sa_sigaction: Option::<
        unsafe extern "C" fn(libc::c_int, *mut siginfo_t, *mut libc::c_void) -> (),
    >,
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
pub type C2RustUnnamed_10 = libc::c_uint;
pub const NUMBER_OF_ELEMENTS: C2RustUnnamed_10 = 12;
pub const FUNCTION_TAG: C2RustUnnamed_10 = 11;
pub const KEY_COMBO: C2RustUnnamed_10 = 10;
pub const ERROR_MESSAGE: C2RustUnnamed_10 = 9;
pub const STATUS_BAR: C2RustUnnamed_10 = 8;
pub const PROMPT_BAR: C2RustUnnamed_10 = 7;
pub const MINI_INFOBAR: C2RustUnnamed_10 = 6;
pub const SPOTLIGHTED: C2RustUnnamed_10 = 5;
pub const SELECTED_TEXT: C2RustUnnamed_10 = 4;
pub const SCROLL_BAR: C2RustUnnamed_10 = 3;
pub const GUIDE_STRIPE: C2RustUnnamed_10 = 2;
pub const LINE_NUMBER: C2RustUnnamed_10 = 1;
pub const TITLE_BAR: C2RustUnnamed_10 = 0;
pub type C2RustUnnamed_11 = libc::c_uint;
pub const ZERO: C2RustUnnamed_11 = 48;
pub const MINIBAR: C2RustUnnamed_11 = 47;
pub const USE_MAGIC: C2RustUnnamed_11 = 46;
pub const STATEFLAGS: C2RustUnnamed_11 = 45;
pub const BOOKSTYLE: C2RustUnnamed_11 = 44;
pub const INDICATOR: C2RustUnnamed_11 = 43;
pub const EMPTY_LINE: C2RustUnnamed_11 = 42;
pub const JUMPY_SCROLLING: C2RustUnnamed_11 = 41;
pub const BREAK_LONG_LINES: C2RustUnnamed_11 = 40;
pub const LET_THEM_ZAP: C2RustUnnamed_11 = 39;
pub const AFTER_ENDS: C2RustUnnamed_11 = 38;
pub const AT_BLANKS: C2RustUnnamed_11 = 37;
pub const LINE_NUMBERS: C2RustUnnamed_11 = 36;
pub const SHOW_CURSOR: C2RustUnnamed_11 = 35;
pub const TRIM_BLANKS: C2RustUnnamed_11 = 34;
pub const MAKE_IT_UNIX: C2RustUnnamed_11 = 33;
pub const NOREAD_MODE: C2RustUnnamed_11 = 32;
pub const LOCKING: C2RustUnnamed_11 = 31;
pub const POSITIONLOG: C2RustUnnamed_11 = 30;
pub const SOFTWRAP: C2RustUnnamed_11 = 29;
pub const BOLD_TEXT: C2RustUnnamed_11 = 28;
pub const NO_NEWLINES: C2RustUnnamed_11 = 27;
pub const WORD_BOUNDS: C2RustUnnamed_11 = 26;
pub const QUICK_BLANK: C2RustUnnamed_11 = 25;
pub const TABS_TO_SPACES: C2RustUnnamed_11 = 24;
pub const WHITESPACE_DISPLAY: C2RustUnnamed_11 = 23;
pub const SMART_HOME: C2RustUnnamed_11 = 22;
pub const RESTRICTED: C2RustUnnamed_11 = 21;
pub const HISTORYLOG: C2RustUnnamed_11 = 20;
pub const PRESERVE: C2RustUnnamed_11 = 19;
pub const NO_SYNTAX: C2RustUnnamed_11 = 18;
pub const INSECURE_BACKUP: C2RustUnnamed_11 = 17;
pub const MAKE_BACKUP: C2RustUnnamed_11 = 16;
pub const NO_CONVERT: C2RustUnnamed_11 = 15;
pub const RAW_SEQUENCES: C2RustUnnamed_11 = 14;
pub const REBIND_DELETE: C2RustUnnamed_11 = 13;
pub const MULTIBUFFER: C2RustUnnamed_11 = 12;
pub const BACKWARDS_SEARCH: C2RustUnnamed_11 = 11;
pub const CUT_FROM_CURSOR: C2RustUnnamed_11 = 10;
pub const SAVE_ON_EXIT: C2RustUnnamed_11 = 9;
pub const USE_REGEXP: C2RustUnnamed_11 = 8;
pub const USE_MOUSE: C2RustUnnamed_11 = 7;
pub const VIEW_MODE: C2RustUnnamed_11 = 6;
pub const AUTOINDENT: C2RustUnnamed_11 = 5;
pub const NO_WRAP: C2RustUnnamed_11 = 4;
pub const NO_HELP: C2RustUnnamed_11 = 3;
pub const CONSTANT_SHOW: C2RustUnnamed_11 = 2;
pub const CASE_SENSITIVE: C2RustUnnamed_11 = 1;
pub const DONTUSE: C2RustUnnamed_11 = 0;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct termios {
    pub c_iflag: tcflag_t,
    pub c_oflag: tcflag_t,
    pub c_cflag: tcflag_t,
    pub c_lflag: tcflag_t,
    pub c_line: cc_t,
    pub c_cc: [cc_t; 32],
    pub c_ispeed: speed_t,
    pub c_ospeed: speed_t,
}
pub type speed_t = libc::c_uint;
pub type cc_t = libc::c_uchar;
pub type tcflag_t = libc::c_uint;
pub type C2RustUnnamed_12 = libc::c_uint;
pub const _ISalnum: C2RustUnnamed_12 = 8;
pub const _ISpunct: C2RustUnnamed_12 = 4;
pub const _IScntrl: C2RustUnnamed_12 = 2;
pub const _ISblank: C2RustUnnamed_12 = 1;
pub const _ISgraph: C2RustUnnamed_12 = 32768;
pub const _ISprint: C2RustUnnamed_12 = 16384;
pub const _ISspace: C2RustUnnamed_12 = 8192;
pub const _ISxdigit: C2RustUnnamed_12 = 4096;
pub const _ISdigit: C2RustUnnamed_12 = 2048;
pub const _ISalpha: C2RustUnnamed_12 = 1024;
pub const _ISlower: C2RustUnnamed_12 = 512;
pub const _ISupper: C2RustUnnamed_12 = 256;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct option {
    pub name: *const libc::c_char,
    pub has_arg: libc::c_int,
    pub flag: *mut libc::c_int,
    pub val: libc::c_int,
}
pub type nl_item = libc::c_int;
pub type C2RustUnnamed_13 = libc::c_uint;
pub const _NL_NUM: C2RustUnnamed_13 = 786449;
pub const _NL_NUM_LC_IDENTIFICATION: C2RustUnnamed_13 = 786448;
pub const _NL_IDENTIFICATION_CODESET: C2RustUnnamed_13 = 786447;
pub const _NL_IDENTIFICATION_CATEGORY: C2RustUnnamed_13 = 786446;
pub const _NL_IDENTIFICATION_DATE: C2RustUnnamed_13 = 786445;
pub const _NL_IDENTIFICATION_REVISION: C2RustUnnamed_13 = 786444;
pub const _NL_IDENTIFICATION_ABBREVIATION: C2RustUnnamed_13 = 786443;
pub const _NL_IDENTIFICATION_APPLICATION: C2RustUnnamed_13 = 786442;
pub const _NL_IDENTIFICATION_AUDIENCE: C2RustUnnamed_13 = 786441;
pub const _NL_IDENTIFICATION_TERRITORY: C2RustUnnamed_13 = 786440;
pub const _NL_IDENTIFICATION_LANGUAGE: C2RustUnnamed_13 = 786439;
pub const _NL_IDENTIFICATION_FAX: C2RustUnnamed_13 = 786438;
pub const _NL_IDENTIFICATION_TEL: C2RustUnnamed_13 = 786437;
pub const _NL_IDENTIFICATION_EMAIL: C2RustUnnamed_13 = 786436;
pub const _NL_IDENTIFICATION_CONTACT: C2RustUnnamed_13 = 786435;
pub const _NL_IDENTIFICATION_ADDRESS: C2RustUnnamed_13 = 786434;
pub const _NL_IDENTIFICATION_SOURCE: C2RustUnnamed_13 = 786433;
pub const _NL_IDENTIFICATION_TITLE: C2RustUnnamed_13 = 786432;
pub const _NL_NUM_LC_MEASUREMENT: C2RustUnnamed_13 = 720898;
pub const _NL_MEASUREMENT_CODESET: C2RustUnnamed_13 = 720897;
pub const _NL_MEASUREMENT_MEASUREMENT: C2RustUnnamed_13 = 720896;
pub const _NL_NUM_LC_TELEPHONE: C2RustUnnamed_13 = 655365;
pub const _NL_TELEPHONE_CODESET: C2RustUnnamed_13 = 655364;
pub const _NL_TELEPHONE_INT_PREFIX: C2RustUnnamed_13 = 655363;
pub const _NL_TELEPHONE_INT_SELECT: C2RustUnnamed_13 = 655362;
pub const _NL_TELEPHONE_TEL_DOM_FMT: C2RustUnnamed_13 = 655361;
pub const _NL_TELEPHONE_TEL_INT_FMT: C2RustUnnamed_13 = 655360;
pub const _NL_NUM_LC_ADDRESS: C2RustUnnamed_13 = 589837;
pub const _NL_ADDRESS_CODESET: C2RustUnnamed_13 = 589836;
pub const _NL_ADDRESS_LANG_LIB: C2RustUnnamed_13 = 589835;
pub const _NL_ADDRESS_LANG_TERM: C2RustUnnamed_13 = 589834;
pub const _NL_ADDRESS_LANG_AB: C2RustUnnamed_13 = 589833;
pub const _NL_ADDRESS_LANG_NAME: C2RustUnnamed_13 = 589832;
pub const _NL_ADDRESS_COUNTRY_ISBN: C2RustUnnamed_13 = 589831;
pub const _NL_ADDRESS_COUNTRY_NUM: C2RustUnnamed_13 = 589830;
pub const _NL_ADDRESS_COUNTRY_CAR: C2RustUnnamed_13 = 589829;
pub const _NL_ADDRESS_COUNTRY_AB3: C2RustUnnamed_13 = 589828;
pub const _NL_ADDRESS_COUNTRY_AB2: C2RustUnnamed_13 = 589827;
pub const _NL_ADDRESS_COUNTRY_POST: C2RustUnnamed_13 = 589826;
pub const _NL_ADDRESS_COUNTRY_NAME: C2RustUnnamed_13 = 589825;
pub const _NL_ADDRESS_POSTAL_FMT: C2RustUnnamed_13 = 589824;
pub const _NL_NUM_LC_NAME: C2RustUnnamed_13 = 524295;
pub const _NL_NAME_CODESET: C2RustUnnamed_13 = 524294;
pub const _NL_NAME_NAME_MS: C2RustUnnamed_13 = 524293;
pub const _NL_NAME_NAME_MISS: C2RustUnnamed_13 = 524292;
pub const _NL_NAME_NAME_MRS: C2RustUnnamed_13 = 524291;
pub const _NL_NAME_NAME_MR: C2RustUnnamed_13 = 524290;
pub const _NL_NAME_NAME_GEN: C2RustUnnamed_13 = 524289;
pub const _NL_NAME_NAME_FMT: C2RustUnnamed_13 = 524288;
pub const _NL_NUM_LC_PAPER: C2RustUnnamed_13 = 458755;
pub const _NL_PAPER_CODESET: C2RustUnnamed_13 = 458754;
pub const _NL_PAPER_WIDTH: C2RustUnnamed_13 = 458753;
pub const _NL_PAPER_HEIGHT: C2RustUnnamed_13 = 458752;
pub const _NL_NUM_LC_MESSAGES: C2RustUnnamed_13 = 327685;
pub const _NL_MESSAGES_CODESET: C2RustUnnamed_13 = 327684;
pub const __NOSTR: C2RustUnnamed_13 = 327683;
pub const __YESSTR: C2RustUnnamed_13 = 327682;
pub const __NOEXPR: C2RustUnnamed_13 = 327681;
pub const __YESEXPR: C2RustUnnamed_13 = 327680;
pub const _NL_NUM_LC_NUMERIC: C2RustUnnamed_13 = 65542;
pub const _NL_NUMERIC_CODESET: C2RustUnnamed_13 = 65541;
pub const _NL_NUMERIC_THOUSANDS_SEP_WC: C2RustUnnamed_13 = 65540;
pub const _NL_NUMERIC_DECIMAL_POINT_WC: C2RustUnnamed_13 = 65539;
pub const __GROUPING: C2RustUnnamed_13 = 65538;
pub const THOUSEP: C2RustUnnamed_13 = 65537;
pub const __THOUSANDS_SEP: C2RustUnnamed_13 = 65537;
pub const RADIXCHAR: C2RustUnnamed_13 = 65536;
pub const __DECIMAL_POINT: C2RustUnnamed_13 = 65536;
pub const _NL_NUM_LC_MONETARY: C2RustUnnamed_13 = 262190;
pub const _NL_MONETARY_CODESET: C2RustUnnamed_13 = 262189;
pub const _NL_MONETARY_THOUSANDS_SEP_WC: C2RustUnnamed_13 = 262188;
pub const _NL_MONETARY_DECIMAL_POINT_WC: C2RustUnnamed_13 = 262187;
pub const _NL_MONETARY_CONVERSION_RATE: C2RustUnnamed_13 = 262186;
pub const _NL_MONETARY_DUO_VALID_TO: C2RustUnnamed_13 = 262185;
pub const _NL_MONETARY_DUO_VALID_FROM: C2RustUnnamed_13 = 262184;
pub const _NL_MONETARY_UNO_VALID_TO: C2RustUnnamed_13 = 262183;
pub const _NL_MONETARY_UNO_VALID_FROM: C2RustUnnamed_13 = 262182;
pub const _NL_MONETARY_DUO_INT_N_SIGN_POSN: C2RustUnnamed_13 = 262181;
pub const _NL_MONETARY_DUO_INT_P_SIGN_POSN: C2RustUnnamed_13 = 262180;
pub const _NL_MONETARY_DUO_N_SIGN_POSN: C2RustUnnamed_13 = 262179;
pub const _NL_MONETARY_DUO_P_SIGN_POSN: C2RustUnnamed_13 = 262178;
pub const _NL_MONETARY_DUO_INT_N_SEP_BY_SPACE: C2RustUnnamed_13 = 262177;
pub const _NL_MONETARY_DUO_INT_N_CS_PRECEDES: C2RustUnnamed_13 = 262176;
pub const _NL_MONETARY_DUO_INT_P_SEP_BY_SPACE: C2RustUnnamed_13 = 262175;
pub const _NL_MONETARY_DUO_INT_P_CS_PRECEDES: C2RustUnnamed_13 = 262174;
pub const _NL_MONETARY_DUO_N_SEP_BY_SPACE: C2RustUnnamed_13 = 262173;
pub const _NL_MONETARY_DUO_N_CS_PRECEDES: C2RustUnnamed_13 = 262172;
pub const _NL_MONETARY_DUO_P_SEP_BY_SPACE: C2RustUnnamed_13 = 262171;
pub const _NL_MONETARY_DUO_P_CS_PRECEDES: C2RustUnnamed_13 = 262170;
pub const _NL_MONETARY_DUO_FRAC_DIGITS: C2RustUnnamed_13 = 262169;
pub const _NL_MONETARY_DUO_INT_FRAC_DIGITS: C2RustUnnamed_13 = 262168;
pub const _NL_MONETARY_DUO_CURRENCY_SYMBOL: C2RustUnnamed_13 = 262167;
pub const _NL_MONETARY_DUO_INT_CURR_SYMBOL: C2RustUnnamed_13 = 262166;
pub const __INT_N_SIGN_POSN: C2RustUnnamed_13 = 262165;
pub const __INT_P_SIGN_POSN: C2RustUnnamed_13 = 262164;
pub const __INT_N_SEP_BY_SPACE: C2RustUnnamed_13 = 262163;
pub const __INT_N_CS_PRECEDES: C2RustUnnamed_13 = 262162;
pub const __INT_P_SEP_BY_SPACE: C2RustUnnamed_13 = 262161;
pub const __INT_P_CS_PRECEDES: C2RustUnnamed_13 = 262160;
pub const _NL_MONETARY_CRNCYSTR: C2RustUnnamed_13 = 262159;
pub const __N_SIGN_POSN: C2RustUnnamed_13 = 262158;
pub const __P_SIGN_POSN: C2RustUnnamed_13 = 262157;
pub const __N_SEP_BY_SPACE: C2RustUnnamed_13 = 262156;
pub const __N_CS_PRECEDES: C2RustUnnamed_13 = 262155;
pub const __P_SEP_BY_SPACE: C2RustUnnamed_13 = 262154;
pub const __P_CS_PRECEDES: C2RustUnnamed_13 = 262153;
pub const __FRAC_DIGITS: C2RustUnnamed_13 = 262152;
pub const __INT_FRAC_DIGITS: C2RustUnnamed_13 = 262151;
pub const __NEGATIVE_SIGN: C2RustUnnamed_13 = 262150;
pub const __POSITIVE_SIGN: C2RustUnnamed_13 = 262149;
pub const __MON_GROUPING: C2RustUnnamed_13 = 262148;
pub const __MON_THOUSANDS_SEP: C2RustUnnamed_13 = 262147;
pub const __MON_DECIMAL_POINT: C2RustUnnamed_13 = 262146;
pub const __CURRENCY_SYMBOL: C2RustUnnamed_13 = 262145;
pub const __INT_CURR_SYMBOL: C2RustUnnamed_13 = 262144;
pub const _NL_NUM_LC_CTYPE: C2RustUnnamed_13 = 86;
pub const _NL_CTYPE_EXTRA_MAP_14: C2RustUnnamed_13 = 85;
pub const _NL_CTYPE_EXTRA_MAP_13: C2RustUnnamed_13 = 84;
pub const _NL_CTYPE_EXTRA_MAP_12: C2RustUnnamed_13 = 83;
pub const _NL_CTYPE_EXTRA_MAP_11: C2RustUnnamed_13 = 82;
pub const _NL_CTYPE_EXTRA_MAP_10: C2RustUnnamed_13 = 81;
pub const _NL_CTYPE_EXTRA_MAP_9: C2RustUnnamed_13 = 80;
pub const _NL_CTYPE_EXTRA_MAP_8: C2RustUnnamed_13 = 79;
pub const _NL_CTYPE_EXTRA_MAP_7: C2RustUnnamed_13 = 78;
pub const _NL_CTYPE_EXTRA_MAP_6: C2RustUnnamed_13 = 77;
pub const _NL_CTYPE_EXTRA_MAP_5: C2RustUnnamed_13 = 76;
pub const _NL_CTYPE_EXTRA_MAP_4: C2RustUnnamed_13 = 75;
pub const _NL_CTYPE_EXTRA_MAP_3: C2RustUnnamed_13 = 74;
pub const _NL_CTYPE_EXTRA_MAP_2: C2RustUnnamed_13 = 73;
pub const _NL_CTYPE_EXTRA_MAP_1: C2RustUnnamed_13 = 72;
pub const _NL_CTYPE_NONASCII_CASE: C2RustUnnamed_13 = 71;
pub const _NL_CTYPE_MAP_TO_NONASCII: C2RustUnnamed_13 = 70;
pub const _NL_CTYPE_TRANSLIT_IGNORE: C2RustUnnamed_13 = 69;
pub const _NL_CTYPE_TRANSLIT_IGNORE_LEN: C2RustUnnamed_13 = 68;
pub const _NL_CTYPE_TRANSLIT_DEFAULT_MISSING: C2RustUnnamed_13 = 67;
pub const _NL_CTYPE_TRANSLIT_DEFAULT_MISSING_LEN: C2RustUnnamed_13 = 66;
pub const _NL_CTYPE_TRANSLIT_TO_TBL: C2RustUnnamed_13 = 65;
pub const _NL_CTYPE_TRANSLIT_TO_IDX: C2RustUnnamed_13 = 64;
pub const _NL_CTYPE_TRANSLIT_FROM_TBL: C2RustUnnamed_13 = 63;
pub const _NL_CTYPE_TRANSLIT_FROM_IDX: C2RustUnnamed_13 = 62;
pub const _NL_CTYPE_TRANSLIT_TAB_SIZE: C2RustUnnamed_13 = 61;
pub const _NL_CTYPE_OUTDIGIT9_WC: C2RustUnnamed_13 = 60;
pub const _NL_CTYPE_OUTDIGIT8_WC: C2RustUnnamed_13 = 59;
pub const _NL_CTYPE_OUTDIGIT7_WC: C2RustUnnamed_13 = 58;
pub const _NL_CTYPE_OUTDIGIT6_WC: C2RustUnnamed_13 = 57;
pub const _NL_CTYPE_OUTDIGIT5_WC: C2RustUnnamed_13 = 56;
pub const _NL_CTYPE_OUTDIGIT4_WC: C2RustUnnamed_13 = 55;
pub const _NL_CTYPE_OUTDIGIT3_WC: C2RustUnnamed_13 = 54;
pub const _NL_CTYPE_OUTDIGIT2_WC: C2RustUnnamed_13 = 53;
pub const _NL_CTYPE_OUTDIGIT1_WC: C2RustUnnamed_13 = 52;
pub const _NL_CTYPE_OUTDIGIT0_WC: C2RustUnnamed_13 = 51;
pub const _NL_CTYPE_OUTDIGIT9_MB: C2RustUnnamed_13 = 50;
pub const _NL_CTYPE_OUTDIGIT8_MB: C2RustUnnamed_13 = 49;
pub const _NL_CTYPE_OUTDIGIT7_MB: C2RustUnnamed_13 = 48;
pub const _NL_CTYPE_OUTDIGIT6_MB: C2RustUnnamed_13 = 47;
pub const _NL_CTYPE_OUTDIGIT5_MB: C2RustUnnamed_13 = 46;
pub const _NL_CTYPE_OUTDIGIT4_MB: C2RustUnnamed_13 = 45;
pub const _NL_CTYPE_OUTDIGIT3_MB: C2RustUnnamed_13 = 44;
pub const _NL_CTYPE_OUTDIGIT2_MB: C2RustUnnamed_13 = 43;
pub const _NL_CTYPE_OUTDIGIT1_MB: C2RustUnnamed_13 = 42;
pub const _NL_CTYPE_OUTDIGIT0_MB: C2RustUnnamed_13 = 41;
pub const _NL_CTYPE_INDIGITS9_WC: C2RustUnnamed_13 = 40;
pub const _NL_CTYPE_INDIGITS8_WC: C2RustUnnamed_13 = 39;
pub const _NL_CTYPE_INDIGITS7_WC: C2RustUnnamed_13 = 38;
pub const _NL_CTYPE_INDIGITS6_WC: C2RustUnnamed_13 = 37;
pub const _NL_CTYPE_INDIGITS5_WC: C2RustUnnamed_13 = 36;
pub const _NL_CTYPE_INDIGITS4_WC: C2RustUnnamed_13 = 35;
pub const _NL_CTYPE_INDIGITS3_WC: C2RustUnnamed_13 = 34;
pub const _NL_CTYPE_INDIGITS2_WC: C2RustUnnamed_13 = 33;
pub const _NL_CTYPE_INDIGITS1_WC: C2RustUnnamed_13 = 32;
pub const _NL_CTYPE_INDIGITS0_WC: C2RustUnnamed_13 = 31;
pub const _NL_CTYPE_INDIGITS_WC_LEN: C2RustUnnamed_13 = 30;
pub const _NL_CTYPE_INDIGITS9_MB: C2RustUnnamed_13 = 29;
pub const _NL_CTYPE_INDIGITS8_MB: C2RustUnnamed_13 = 28;
pub const _NL_CTYPE_INDIGITS7_MB: C2RustUnnamed_13 = 27;
pub const _NL_CTYPE_INDIGITS6_MB: C2RustUnnamed_13 = 26;
pub const _NL_CTYPE_INDIGITS5_MB: C2RustUnnamed_13 = 25;
pub const _NL_CTYPE_INDIGITS4_MB: C2RustUnnamed_13 = 24;
pub const _NL_CTYPE_INDIGITS3_MB: C2RustUnnamed_13 = 23;
pub const _NL_CTYPE_INDIGITS2_MB: C2RustUnnamed_13 = 22;
pub const _NL_CTYPE_INDIGITS1_MB: C2RustUnnamed_13 = 21;
pub const _NL_CTYPE_INDIGITS0_MB: C2RustUnnamed_13 = 20;
pub const _NL_CTYPE_INDIGITS_MB_LEN: C2RustUnnamed_13 = 19;
pub const _NL_CTYPE_MAP_OFFSET: C2RustUnnamed_13 = 18;
pub const _NL_CTYPE_CLASS_OFFSET: C2RustUnnamed_13 = 17;
pub const _NL_CTYPE_TOLOWER32: C2RustUnnamed_13 = 16;
pub const _NL_CTYPE_TOUPPER32: C2RustUnnamed_13 = 15;
pub const CODESET: C2RustUnnamed_13 = 14;
pub const _NL_CTYPE_CODESET_NAME: C2RustUnnamed_13 = 14;
pub const _NL_CTYPE_MB_CUR_MAX: C2RustUnnamed_13 = 13;
pub const _NL_CTYPE_WIDTH: C2RustUnnamed_13 = 12;
pub const _NL_CTYPE_MAP_NAMES: C2RustUnnamed_13 = 11;
pub const _NL_CTYPE_CLASS_NAMES: C2RustUnnamed_13 = 10;
pub const _NL_CTYPE_GAP6: C2RustUnnamed_13 = 9;
pub const _NL_CTYPE_GAP5: C2RustUnnamed_13 = 8;
pub const _NL_CTYPE_GAP4: C2RustUnnamed_13 = 7;
pub const _NL_CTYPE_GAP3: C2RustUnnamed_13 = 6;
pub const _NL_CTYPE_CLASS32: C2RustUnnamed_13 = 5;
pub const _NL_CTYPE_GAP2: C2RustUnnamed_13 = 4;
pub const _NL_CTYPE_TOLOWER: C2RustUnnamed_13 = 3;
pub const _NL_CTYPE_GAP1: C2RustUnnamed_13 = 2;
pub const _NL_CTYPE_TOUPPER: C2RustUnnamed_13 = 1;
pub const _NL_CTYPE_CLASS: C2RustUnnamed_13 = 0;
pub const _NL_NUM_LC_COLLATE: C2RustUnnamed_13 = 196627;
pub const _NL_COLLATE_CODESET: C2RustUnnamed_13 = 196626;
pub const _NL_COLLATE_COLLSEQWC: C2RustUnnamed_13 = 196625;
pub const _NL_COLLATE_COLLSEQMB: C2RustUnnamed_13 = 196624;
pub const _NL_COLLATE_SYMB_EXTRAMB: C2RustUnnamed_13 = 196623;
pub const _NL_COLLATE_SYMB_TABLEMB: C2RustUnnamed_13 = 196622;
pub const _NL_COLLATE_SYMB_HASH_SIZEMB: C2RustUnnamed_13 = 196621;
pub const _NL_COLLATE_INDIRECTWC: C2RustUnnamed_13 = 196620;
pub const _NL_COLLATE_EXTRAWC: C2RustUnnamed_13 = 196619;
pub const _NL_COLLATE_WEIGHTWC: C2RustUnnamed_13 = 196618;
pub const _NL_COLLATE_TABLEWC: C2RustUnnamed_13 = 196617;
pub const _NL_COLLATE_GAP3: C2RustUnnamed_13 = 196616;
pub const _NL_COLLATE_GAP2: C2RustUnnamed_13 = 196615;
pub const _NL_COLLATE_GAP1: C2RustUnnamed_13 = 196614;
pub const _NL_COLLATE_INDIRECTMB: C2RustUnnamed_13 = 196613;
pub const _NL_COLLATE_EXTRAMB: C2RustUnnamed_13 = 196612;
pub const _NL_COLLATE_WEIGHTMB: C2RustUnnamed_13 = 196611;
pub const _NL_COLLATE_TABLEMB: C2RustUnnamed_13 = 196610;
pub const _NL_COLLATE_RULESETS: C2RustUnnamed_13 = 196609;
pub const _NL_COLLATE_NRULES: C2RustUnnamed_13 = 196608;
pub const _NL_NUM_LC_TIME: C2RustUnnamed_13 = 131231;
pub const _NL_WABALTMON_12: C2RustUnnamed_13 = 131230;
pub const _NL_WABALTMON_11: C2RustUnnamed_13 = 131229;
pub const _NL_WABALTMON_10: C2RustUnnamed_13 = 131228;
pub const _NL_WABALTMON_9: C2RustUnnamed_13 = 131227;
pub const _NL_WABALTMON_8: C2RustUnnamed_13 = 131226;
pub const _NL_WABALTMON_7: C2RustUnnamed_13 = 131225;
pub const _NL_WABALTMON_6: C2RustUnnamed_13 = 131224;
pub const _NL_WABALTMON_5: C2RustUnnamed_13 = 131223;
pub const _NL_WABALTMON_4: C2RustUnnamed_13 = 131222;
pub const _NL_WABALTMON_3: C2RustUnnamed_13 = 131221;
pub const _NL_WABALTMON_2: C2RustUnnamed_13 = 131220;
pub const _NL_WABALTMON_1: C2RustUnnamed_13 = 131219;
pub const _NL_ABALTMON_12: C2RustUnnamed_13 = 131218;
pub const _NL_ABALTMON_11: C2RustUnnamed_13 = 131217;
pub const _NL_ABALTMON_10: C2RustUnnamed_13 = 131216;
pub const _NL_ABALTMON_9: C2RustUnnamed_13 = 131215;
pub const _NL_ABALTMON_8: C2RustUnnamed_13 = 131214;
pub const _NL_ABALTMON_7: C2RustUnnamed_13 = 131213;
pub const _NL_ABALTMON_6: C2RustUnnamed_13 = 131212;
pub const _NL_ABALTMON_5: C2RustUnnamed_13 = 131211;
pub const _NL_ABALTMON_4: C2RustUnnamed_13 = 131210;
pub const _NL_ABALTMON_3: C2RustUnnamed_13 = 131209;
pub const _NL_ABALTMON_2: C2RustUnnamed_13 = 131208;
pub const _NL_ABALTMON_1: C2RustUnnamed_13 = 131207;
pub const _NL_WALTMON_12: C2RustUnnamed_13 = 131206;
pub const _NL_WALTMON_11: C2RustUnnamed_13 = 131205;
pub const _NL_WALTMON_10: C2RustUnnamed_13 = 131204;
pub const _NL_WALTMON_9: C2RustUnnamed_13 = 131203;
pub const _NL_WALTMON_8: C2RustUnnamed_13 = 131202;
pub const _NL_WALTMON_7: C2RustUnnamed_13 = 131201;
pub const _NL_WALTMON_6: C2RustUnnamed_13 = 131200;
pub const _NL_WALTMON_5: C2RustUnnamed_13 = 131199;
pub const _NL_WALTMON_4: C2RustUnnamed_13 = 131198;
pub const _NL_WALTMON_3: C2RustUnnamed_13 = 131197;
pub const _NL_WALTMON_2: C2RustUnnamed_13 = 131196;
pub const _NL_WALTMON_1: C2RustUnnamed_13 = 131195;
pub const __ALTMON_12: C2RustUnnamed_13 = 131194;
pub const __ALTMON_11: C2RustUnnamed_13 = 131193;
pub const __ALTMON_10: C2RustUnnamed_13 = 131192;
pub const __ALTMON_9: C2RustUnnamed_13 = 131191;
pub const __ALTMON_8: C2RustUnnamed_13 = 131190;
pub const __ALTMON_7: C2RustUnnamed_13 = 131189;
pub const __ALTMON_6: C2RustUnnamed_13 = 131188;
pub const __ALTMON_5: C2RustUnnamed_13 = 131187;
pub const __ALTMON_4: C2RustUnnamed_13 = 131186;
pub const __ALTMON_3: C2RustUnnamed_13 = 131185;
pub const __ALTMON_2: C2RustUnnamed_13 = 131184;
pub const __ALTMON_1: C2RustUnnamed_13 = 131183;
pub const _NL_TIME_CODESET: C2RustUnnamed_13 = 131182;
pub const _NL_W_DATE_FMT: C2RustUnnamed_13 = 131181;
pub const _DATE_FMT: C2RustUnnamed_13 = 131180;
pub const _NL_TIME_TIMEZONE: C2RustUnnamed_13 = 131179;
pub const _NL_TIME_CAL_DIRECTION: C2RustUnnamed_13 = 131178;
pub const _NL_TIME_FIRST_WORKDAY: C2RustUnnamed_13 = 131177;
pub const _NL_TIME_FIRST_WEEKDAY: C2RustUnnamed_13 = 131176;
pub const _NL_TIME_WEEK_1STWEEK: C2RustUnnamed_13 = 131175;
pub const _NL_TIME_WEEK_1STDAY: C2RustUnnamed_13 = 131174;
pub const _NL_TIME_WEEK_NDAYS: C2RustUnnamed_13 = 131173;
pub const _NL_WERA_T_FMT: C2RustUnnamed_13 = 131172;
pub const _NL_WERA_D_T_FMT: C2RustUnnamed_13 = 131171;
pub const _NL_WALT_DIGITS: C2RustUnnamed_13 = 131170;
pub const _NL_WERA_D_FMT: C2RustUnnamed_13 = 131169;
pub const _NL_WERA_YEAR: C2RustUnnamed_13 = 131168;
pub const _NL_WT_FMT_AMPM: C2RustUnnamed_13 = 131167;
pub const _NL_WT_FMT: C2RustUnnamed_13 = 131166;
pub const _NL_WD_FMT: C2RustUnnamed_13 = 131165;
pub const _NL_WD_T_FMT: C2RustUnnamed_13 = 131164;
pub const _NL_WPM_STR: C2RustUnnamed_13 = 131163;
pub const _NL_WAM_STR: C2RustUnnamed_13 = 131162;
pub const _NL_WMON_12: C2RustUnnamed_13 = 131161;
pub const _NL_WMON_11: C2RustUnnamed_13 = 131160;
pub const _NL_WMON_10: C2RustUnnamed_13 = 131159;
pub const _NL_WMON_9: C2RustUnnamed_13 = 131158;
pub const _NL_WMON_8: C2RustUnnamed_13 = 131157;
pub const _NL_WMON_7: C2RustUnnamed_13 = 131156;
pub const _NL_WMON_6: C2RustUnnamed_13 = 131155;
pub const _NL_WMON_5: C2RustUnnamed_13 = 131154;
pub const _NL_WMON_4: C2RustUnnamed_13 = 131153;
pub const _NL_WMON_3: C2RustUnnamed_13 = 131152;
pub const _NL_WMON_2: C2RustUnnamed_13 = 131151;
pub const _NL_WMON_1: C2RustUnnamed_13 = 131150;
pub const _NL_WABMON_12: C2RustUnnamed_13 = 131149;
pub const _NL_WABMON_11: C2RustUnnamed_13 = 131148;
pub const _NL_WABMON_10: C2RustUnnamed_13 = 131147;
pub const _NL_WABMON_9: C2RustUnnamed_13 = 131146;
pub const _NL_WABMON_8: C2RustUnnamed_13 = 131145;
pub const _NL_WABMON_7: C2RustUnnamed_13 = 131144;
pub const _NL_WABMON_6: C2RustUnnamed_13 = 131143;
pub const _NL_WABMON_5: C2RustUnnamed_13 = 131142;
pub const _NL_WABMON_4: C2RustUnnamed_13 = 131141;
pub const _NL_WABMON_3: C2RustUnnamed_13 = 131140;
pub const _NL_WABMON_2: C2RustUnnamed_13 = 131139;
pub const _NL_WABMON_1: C2RustUnnamed_13 = 131138;
pub const _NL_WDAY_7: C2RustUnnamed_13 = 131137;
pub const _NL_WDAY_6: C2RustUnnamed_13 = 131136;
pub const _NL_WDAY_5: C2RustUnnamed_13 = 131135;
pub const _NL_WDAY_4: C2RustUnnamed_13 = 131134;
pub const _NL_WDAY_3: C2RustUnnamed_13 = 131133;
pub const _NL_WDAY_2: C2RustUnnamed_13 = 131132;
pub const _NL_WDAY_1: C2RustUnnamed_13 = 131131;
pub const _NL_WABDAY_7: C2RustUnnamed_13 = 131130;
pub const _NL_WABDAY_6: C2RustUnnamed_13 = 131129;
pub const _NL_WABDAY_5: C2RustUnnamed_13 = 131128;
pub const _NL_WABDAY_4: C2RustUnnamed_13 = 131127;
pub const _NL_WABDAY_3: C2RustUnnamed_13 = 131126;
pub const _NL_WABDAY_2: C2RustUnnamed_13 = 131125;
pub const _NL_WABDAY_1: C2RustUnnamed_13 = 131124;
pub const _NL_TIME_ERA_ENTRIES: C2RustUnnamed_13 = 131123;
pub const _NL_TIME_ERA_NUM_ENTRIES: C2RustUnnamed_13 = 131122;
pub const ERA_T_FMT: C2RustUnnamed_13 = 131121;
pub const ERA_D_T_FMT: C2RustUnnamed_13 = 131120;
pub const ALT_DIGITS: C2RustUnnamed_13 = 131119;
pub const ERA_D_FMT: C2RustUnnamed_13 = 131118;
pub const __ERA_YEAR: C2RustUnnamed_13 = 131117;
pub const ERA: C2RustUnnamed_13 = 131116;
pub const T_FMT_AMPM: C2RustUnnamed_13 = 131115;
pub const T_FMT: C2RustUnnamed_13 = 131114;
pub const D_FMT: C2RustUnnamed_13 = 131113;
pub const D_T_FMT: C2RustUnnamed_13 = 131112;
pub const PM_STR: C2RustUnnamed_13 = 131111;
pub const AM_STR: C2RustUnnamed_13 = 131110;
pub const MON_12: C2RustUnnamed_13 = 131109;
pub const MON_11: C2RustUnnamed_13 = 131108;
pub const MON_10: C2RustUnnamed_13 = 131107;
pub const MON_9: C2RustUnnamed_13 = 131106;
pub const MON_8: C2RustUnnamed_13 = 131105;
pub const MON_7: C2RustUnnamed_13 = 131104;
pub const MON_6: C2RustUnnamed_13 = 131103;
pub const MON_5: C2RustUnnamed_13 = 131102;
pub const MON_4: C2RustUnnamed_13 = 131101;
pub const MON_3: C2RustUnnamed_13 = 131100;
pub const MON_2: C2RustUnnamed_13 = 131099;
pub const MON_1: C2RustUnnamed_13 = 131098;
pub const ABMON_12: C2RustUnnamed_13 = 131097;
pub const ABMON_11: C2RustUnnamed_13 = 131096;
pub const ABMON_10: C2RustUnnamed_13 = 131095;
pub const ABMON_9: C2RustUnnamed_13 = 131094;
pub const ABMON_8: C2RustUnnamed_13 = 131093;
pub const ABMON_7: C2RustUnnamed_13 = 131092;
pub const ABMON_6: C2RustUnnamed_13 = 131091;
pub const ABMON_5: C2RustUnnamed_13 = 131090;
pub const ABMON_4: C2RustUnnamed_13 = 131089;
pub const ABMON_3: C2RustUnnamed_13 = 131088;
pub const ABMON_2: C2RustUnnamed_13 = 131087;
pub const ABMON_1: C2RustUnnamed_13 = 131086;
pub const DAY_7: C2RustUnnamed_13 = 131085;
pub const DAY_6: C2RustUnnamed_13 = 131084;
pub const DAY_5: C2RustUnnamed_13 = 131083;
pub const DAY_4: C2RustUnnamed_13 = 131082;
pub const DAY_3: C2RustUnnamed_13 = 131081;
pub const DAY_2: C2RustUnnamed_13 = 131080;
pub const DAY_1: C2RustUnnamed_13 = 131079;
pub const ABDAY_7: C2RustUnnamed_13 = 131078;
pub const ABDAY_6: C2RustUnnamed_13 = 131077;
pub const ABDAY_5: C2RustUnnamed_13 = 131076;
pub const ABDAY_4: C2RustUnnamed_13 = 131075;
pub const ABDAY_3: C2RustUnnamed_13 = 131074;
pub const ABDAY_2: C2RustUnnamed_13 = 131073;
pub const ABDAY_1: C2RustUnnamed_13 = 131072;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct vt_stat {
    pub v_active: libc::c_ushort,
    pub v_signal: libc::c_ushort,
    pub v_state: libc::c_ushort,
}
#[inline]
unsafe extern "C" fn toupper(mut __c: libc::c_int) -> libc::c_int {
    return if __c >= -(128 as libc::c_int) && __c < 256 as libc::c_int {
        *(*__ctype_toupper_loc()).offset(__c as isize)
    } else {
        __c
    };
}
static mut oldinterval: libc::c_int = -(1 as libc::c_int);
static mut original_state: termios = termios {
    c_iflag: 0,
    c_oflag: 0,
    c_cflag: 0,
    c_lflag: 0,
    c_line: 0,
    c_cc: [0; 32],
    c_ispeed: 0,
    c_ospeed: 0,
};
static mut oldaction: sigaction = sigaction {
    __sigaction_handler: C2RustUnnamed_9 {
        sa_handler: None,
    },
    sa_mask: __sigset_t { __val: [0; 16] },
    sa_flags: 0,
    sa_restorer: None,
};
static mut newaction: sigaction = sigaction {
    __sigaction_handler: C2RustUnnamed_9 {
        sa_handler: None,
    },
    sa_mask: __sigset_t { __val: [0; 16] },
    sa_flags: 0,
    sa_restorer: None,
};
pub unsafe extern "C" fn make_new_node(
    mut prevnode: *mut linestruct,
) -> *mut linestruct {
    let mut newnode: *mut linestruct = nmalloc(
        ::std::mem::size_of::<linestruct>() as libc::c_ulong,
    ) as *mut linestruct;
    (*newnode).prev = prevnode;
    (*newnode).next = 0 as *mut linestruct;
    (*newnode).data = 0 as *mut libc::c_char;
    (*newnode).multidata = 0 as *mut libc::c_short;
    (*newnode)
        .lineno = if !prevnode.is_null() {
        (*prevnode).lineno + 1 as libc::c_int as libc::c_long
    } else {
        1 as libc::c_int as libc::c_long
    };
    (*newnode).has_anchor = 0 as libc::c_int != 0;
    return newnode;
}
pub unsafe extern "C" fn splice_node(
    mut afterthis: *mut linestruct,
    mut newnode: *mut linestruct,
) {
    (*newnode).next = (*afterthis).next;
    (*newnode).prev = afterthis;
    if !((*afterthis).next).is_null() {
        (*(*afterthis).next).prev = newnode;
    }
    (*afterthis).next = newnode;
    if !openfile.is_null() && (*openfile).filebot == afterthis {
        (*openfile).filebot = newnode;
    }
}
pub unsafe extern "C" fn delete_node(mut line: *mut linestruct) {
    if line == (*openfile).edittop {
        (*openfile).edittop = (*line).prev;
    }
    if line == (*openfile).spillage_line {
        (*openfile).spillage_line = 0 as *mut linestruct;
    }
    rpl_free((*line).data as *mut libc::c_void);
    rpl_free((*line).multidata as *mut libc::c_void);
    rpl_free(line as *mut libc::c_void);
}
pub unsafe extern "C" fn unlink_node(mut line: *mut linestruct) {
    if !((*line).prev).is_null() {
        (*(*line).prev).next = (*line).next;
    }
    if !((*line).next).is_null() {
        (*(*line).next).prev = (*line).prev;
    }
    if !openfile.is_null() && (*openfile).filebot == line {
        (*openfile).filebot = (*line).prev;
    }
    delete_node(line);
}
pub unsafe extern "C" fn free_lines(mut src: *mut linestruct) {
    if src.is_null() {
        return;
    }
    while !((*src).next).is_null() {
        src = (*src).next;
        delete_node((*src).prev);
    }
    delete_node(src);
}
pub unsafe extern "C" fn copy_node(mut src: *const linestruct) -> *mut linestruct {
    let mut dst: *mut linestruct = nmalloc(
        ::std::mem::size_of::<linestruct>() as libc::c_ulong,
    ) as *mut linestruct;
    (*dst).data = copy_of((*src).data);
    (*dst).multidata = 0 as *mut libc::c_short;
    (*dst).lineno = (*src).lineno;
    (*dst).has_anchor = (*src).has_anchor;
    return dst;
}
pub unsafe extern "C" fn copy_buffer(mut src: *const linestruct) -> *mut linestruct {
    let mut head: *mut linestruct = 0 as *mut linestruct;
    let mut item: *mut linestruct = 0 as *mut linestruct;
    head = copy_node(src);
    (*head).prev = 0 as *mut linestruct;
    item = head;
    src = (*src).next;
    while !src.is_null() {
        (*item).next = copy_node(src);
        (*(*item).next).prev = item;
        item = (*item).next;
        src = (*src).next;
    }
    (*item).next = 0 as *mut linestruct;
    return head;
}
pub unsafe extern "C" fn renumber_from(mut line: *mut linestruct) {
    let mut number: ssize_t = if ((*line).prev).is_null() {
        0 as libc::c_int as libc::c_long
    } else {
        (*(*line).prev).lineno
    };
    while !line.is_null() {
        number += 1;
        (*line).lineno = number;
        line = (*line).next;
    }
}
pub unsafe extern "C" fn print_view_warning() {
    statusline(
        AHEM,
        dcgettext(
            0 as *const libc::c_char,
            b"Key is invalid in view mode\0" as *const u8 as *const libc::c_char,
            5 as libc::c_int,
        ),
    );
}
pub unsafe extern "C" fn in_restricted_mode() -> bool {
    if flags[(RESTRICTED as libc::c_int as libc::c_ulong)
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
        statusline(
            AHEM,
            dcgettext(
                0 as *const libc::c_char,
                b"This function is disabled in restricted mode\0" as *const u8
                    as *const libc::c_char,
                5 as libc::c_int,
            ),
        );
        beep();
        return 1 as libc::c_int != 0;
    } else {
        return 0 as libc::c_int != 0
    };
}
pub unsafe extern "C" fn suggest_ctrlT_ctrlZ() {
    if !(first_sc_for(
        (1 as libc::c_int) << 0 as libc::c_int,
        Some(do_execute as unsafe extern "C" fn() -> ()),
    ))
        .is_null()
        && (*first_sc_for(
            (1 as libc::c_int) << 0 as libc::c_int,
            Some(do_execute as unsafe extern "C" fn() -> ()),
        ))
            .keycode == 0x14 as libc::c_int
        && !(first_sc_for(
            (1 as libc::c_int) << 7 as libc::c_int,
            Some(do_suspend as unsafe extern "C" fn() -> ()),
        ))
            .is_null()
        && (*first_sc_for(
            (1 as libc::c_int) << 7 as libc::c_int,
            Some(do_suspend as unsafe extern "C" fn() -> ()),
        ))
            .keycode == 0x1a as libc::c_int
    {
        statusline(
            AHEM,
            dcgettext(
                0 as *const libc::c_char,
                b"To suspend, type ^T^Z\0" as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ),
        );
    }
}
pub unsafe extern "C" fn restore_terminal() {
    curs_set(1 as libc::c_int);
    endwin();
    printf(b"\x1B[?2004l\0" as *const u8 as *const libc::c_char);
    fflush(stdout);
    tcsetattr(0 as libc::c_int, 0 as libc::c_int, &mut original_state);
}
pub unsafe extern "C" fn finish() {
    blank_statusbar();
    blank_bottombars();
    wrefresh(footwin);
    if !topwin.is_null() {
        delwin(topwin);
    }
    delwin(midwin);
    delwin(footwin);
    restore_terminal();
    display_rcfile_errors();
    exit(0 as libc::c_int);
}
pub unsafe extern "C" fn close_and_go() {
    if !((*openfile).lock_filename).is_null() {
        delete_lockfile((*openfile).lock_filename);
    }
    if flags[(POSITIONLOG as libc::c_int as libc::c_ulong)
        .wrapping_div(
            (::std::mem::size_of::<libc::c_uint>() as libc::c_ulong)
                .wrapping_mul(8 as libc::c_int as libc::c_ulong),
        ) as usize]
        & (1 as libc::c_int as libc::c_uint)
            << (POSITIONLOG as libc::c_int as libc::c_ulong)
                .wrapping_rem(
                    (::std::mem::size_of::<libc::c_uint>() as libc::c_ulong)
                        .wrapping_mul(8 as libc::c_int as libc::c_ulong),
                ) != 0 as libc::c_int as libc::c_uint
    {
        update_poshistory();
    }
    if openfile != (*openfile).next {
        switch_to_next_buffer();
        openfile = (*openfile).prev;
        close_buffer();
        openfile = (*openfile).next;
        titlebar(0 as *const libc::c_char);
    } else {
        if flags[(HISTORYLOG as libc::c_int as libc::c_ulong)
            .wrapping_div(
                (::std::mem::size_of::<libc::c_uint>() as libc::c_ulong)
                    .wrapping_mul(8 as libc::c_int as libc::c_ulong),
            ) as usize]
            & (1 as libc::c_int as libc::c_uint)
                << (HISTORYLOG as libc::c_int as libc::c_ulong)
                    .wrapping_rem(
                        (::std::mem::size_of::<libc::c_uint>() as libc::c_ulong)
                            .wrapping_mul(8 as libc::c_int as libc::c_ulong),
                    ) != 0 as libc::c_int as libc::c_uint
        {
            save_history();
        }
        finish();
    };
}
pub unsafe extern "C" fn do_exit() {
    let mut choice: libc::c_int = 0;
    if !(*openfile).modified {
        choice = 0 as libc::c_int;
    } else if flags[(SAVE_ON_EXIT as libc::c_int as libc::c_ulong)
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
        && *((*openfile).filename).offset(0 as libc::c_int as isize) as libc::c_int
            != '\0' as i32
    {
        choice = 1 as libc::c_int;
    } else {
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
        {
            warn_and_briefly_pause(
                dcgettext(
                    0 as *const libc::c_char,
                    b"No file name\0" as *const u8 as *const libc::c_char,
                    5 as libc::c_int,
                ),
            );
        }
        choice = ask_user(
            0 as libc::c_int != 0,
            dcgettext(
                0 as *const libc::c_char,
                b"Save modified buffer? \0" as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ),
        );
    }
    if choice == 0 as libc::c_int
        || choice == 1 as libc::c_int
            && write_it_out(1 as libc::c_int != 0, 1 as libc::c_int != 0)
                > 0 as libc::c_int
    {
        close_and_go();
    } else if choice != 1 as libc::c_int {
        statusbar(
            dcgettext(
                0 as *const libc::c_char,
                b"Cancelled\0" as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ),
        );
    }
}
pub unsafe extern "C" fn emergency_save(mut filename: *const libc::c_char) {
    let mut plainname: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut targetname: *mut libc::c_char = 0 as *mut libc::c_char;
    if *filename as libc::c_int == '\0' as i32 {
        plainname = nmalloc(28 as libc::c_int as size_t) as *mut libc::c_char;
        sprintf(plainname, b"nano.%u\0" as *const u8 as *const libc::c_char, getpid());
    } else {
        plainname = copy_of(filename);
    }
    targetname = get_next_filename(
        plainname,
        b".save\0" as *const u8 as *const libc::c_char,
    );
    if *targetname as libc::c_int == '\0' as i32 {
        fprintf(
            stderr,
            dcgettext(
                0 as *const libc::c_char,
                b"\nToo many .save files\n\0" as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ),
        );
    } else if write_file(
        targetname,
        0 as *mut FILE,
        0 as libc::c_int != 0,
        OVERWRITE,
        0 as libc::c_int != 0,
    ) {
        fprintf(
            stderr,
            dcgettext(
                0 as *const libc::c_char,
                b"\nBuffer written to %s\n\0" as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ),
            targetname,
        );
        if !((*openfile).statinfo).is_null() {
            chmod(targetname, (*(*openfile).statinfo).st_mode) != 0;
            chown(
                targetname,
                (*(*openfile).statinfo).st_uid,
                (*(*openfile).statinfo).st_gid,
            ) != 0;
        }
    }
    rpl_free(targetname as *mut libc::c_void);
    rpl_free(plainname as *mut libc::c_void);
}
pub unsafe extern "C" fn die(mut msg: *const libc::c_char, mut args: ...) {
    let mut firstone: *mut openfilestruct = openfile;
    static mut stabs: libc::c_int = 0 as libc::c_int;
    let mut ap: ::std::ffi::VaListImpl;
    stabs += 1;
    if stabs > 1 as libc::c_int {
        exit(11 as libc::c_int);
    }
    restore_terminal();
    display_rcfile_errors();
    ap = args.clone();
    vfprintf(stderr, msg, ap.as_va_list());
    while !openfile.is_null() {
        if !((*openfile).lock_filename).is_null() {
            delete_lockfile((*openfile).lock_filename);
        }
        if (*openfile).modified as libc::c_int != 0
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
            emergency_save((*openfile).filename);
        }
        openfile = (*openfile).next;
        if openfile == firstone {
            break;
        }
    }
    exit(1 as libc::c_int);
}
pub unsafe extern "C" fn window_init() {
    if !midwin.is_null() {
        if !topwin.is_null() {
            delwin(topwin);
        }
        delwin(midwin);
        delwin(footwin);
    }
    topwin = 0 as *mut WINDOW;
    if LINES < 3 as libc::c_int {
        editwinrows = if flags[(ZERO as libc::c_int as libc::c_ulong)
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
            LINES
        } else {
            1 as libc::c_int
        };
        midwin = newwin(editwinrows, COLS, 0 as libc::c_int, 0 as libc::c_int);
        footwin = newwin(
            1 as libc::c_int,
            COLS,
            LINES - 1 as libc::c_int,
            0 as libc::c_int,
        );
    } else {
        let mut toprows: libc::c_int = if flags[(EMPTY_LINE as libc::c_int
            as libc::c_ulong)
            .wrapping_div(
                (::std::mem::size_of::<libc::c_uint>() as libc::c_ulong)
                    .wrapping_mul(8 as libc::c_int as libc::c_ulong),
            ) as usize]
            & (1 as libc::c_int as libc::c_uint)
                << (EMPTY_LINE as libc::c_int as libc::c_ulong)
                    .wrapping_rem(
                        (::std::mem::size_of::<libc::c_uint>() as libc::c_ulong)
                            .wrapping_mul(8 as libc::c_int as libc::c_ulong),
                    ) != 0 as libc::c_int as libc::c_uint && LINES > 6 as libc::c_int
        {
            2 as libc::c_int
        } else {
            1 as libc::c_int
        };
        let mut bottomrows: libc::c_int = if flags[(NO_HELP as libc::c_int
            as libc::c_ulong)
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
            1 as libc::c_int
        } else {
            3 as libc::c_int
        };
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
            toprows = 0 as libc::c_int;
        }
        editwinrows = LINES - toprows - bottomrows
            + (if flags[(ZERO as libc::c_int as libc::c_ulong)
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
                1 as libc::c_int
            } else {
                0 as libc::c_int
            });
        if toprows > 0 as libc::c_int {
            topwin = newwin(toprows, COLS, 0 as libc::c_int, 0 as libc::c_int);
        }
        midwin = newwin(editwinrows, COLS, toprows, 0 as libc::c_int);
        footwin = newwin(bottomrows, COLS, LINES - bottomrows, 0 as libc::c_int);
    }
    wnoutrefresh(footwin);
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
    if COLS as libc::c_long + fill < 0 as libc::c_int as libc::c_long {
        wrap_at = 0 as libc::c_int as size_t;
    } else if fill <= 0 as libc::c_int as libc::c_long {
        wrap_at = (COLS as libc::c_long + fill) as size_t;
    } else {
        wrap_at = fill as size_t;
    };
}
pub unsafe extern "C" fn disable_mouse_support() {
    mousemask(0 as libc::c_int as mmask_t, 0 as *mut mmask_t);
    mouseinterval(oldinterval);
}
pub unsafe extern "C" fn enable_mouse_support() {
    mousemask(
        (((0o10 as libc::c_long)
            << (6 as libc::c_int - 1 as libc::c_int) * 5 as libc::c_int)
            - 1 as libc::c_int as libc::c_long) as mmask_t,
        0 as *mut mmask_t,
    );
    oldinterval = mouseinterval(50 as libc::c_int);
}
pub unsafe extern "C" fn mouse_init() {
    if flags[(USE_MOUSE as libc::c_int as libc::c_ulong)
        .wrapping_div(
            (::std::mem::size_of::<libc::c_uint>() as libc::c_ulong)
                .wrapping_mul(8 as libc::c_int as libc::c_ulong),
        ) as usize]
        & (1 as libc::c_int as libc::c_uint)
            << (USE_MOUSE as libc::c_int as libc::c_ulong)
                .wrapping_rem(
                    (::std::mem::size_of::<libc::c_uint>() as libc::c_ulong)
                        .wrapping_mul(8 as libc::c_int as libc::c_ulong),
                ) != 0 as libc::c_int as libc::c_uint
    {
        enable_mouse_support();
    } else {
        disable_mouse_support();
    };
}
pub unsafe extern "C" fn print_opt(
    mut shortflag: *const libc::c_char,
    mut longflag: *const libc::c_char,
    mut description: *const libc::c_char,
) {
    let mut firstwidth: libc::c_int = breadth(shortflag) as libc::c_int;
    let mut secondwidth: libc::c_int = breadth(longflag) as libc::c_int;
    printf(b" %s\0" as *const u8 as *const libc::c_char, shortflag);
    if firstwidth < 14 as libc::c_int {
        printf(
            b"%*s\0" as *const u8 as *const libc::c_char,
            14 as libc::c_int - firstwidth,
            b" \0" as *const u8 as *const libc::c_char,
        );
    }
    printf(b" %s\0" as *const u8 as *const libc::c_char, longflag);
    if secondwidth < 24 as libc::c_int {
        printf(
            b"%*s\0" as *const u8 as *const libc::c_char,
            24 as libc::c_int - secondwidth,
            b" \0" as *const u8 as *const libc::c_char,
        );
    }
    printf(
        b"%s\n\0" as *const u8 as *const libc::c_char,
        dcgettext(0 as *const libc::c_char, description, 5 as libc::c_int),
    );
}
pub unsafe extern "C" fn usage() {
    printf(
        dcgettext(
            0 as *const libc::c_char,
            b"Usage: nano [OPTIONS] [[+LINE[,COLUMN]] FILE]...\n\n\0" as *const u8
                as *const libc::c_char,
            5 as libc::c_int,
        ),
    );
    printf(
        dcgettext(
            0 as *const libc::c_char,
            b"To place the cursor on a specific line of a file, put the line number with\na '+' before the filename.  The column number can be added after a comma.\n\0"
                as *const u8 as *const libc::c_char,
            5 as libc::c_int,
        ),
    );
    printf(
        dcgettext(
            0 as *const libc::c_char,
            b"When a filename is '-', nano reads data from standard input.\n\n\0"
                as *const u8 as *const libc::c_char,
            5 as libc::c_int,
        ),
    );
    print_opt(
        dcgettext(
            0 as *const libc::c_char,
            b"Option\0" as *const u8 as *const libc::c_char,
            5 as libc::c_int,
        ),
        dcgettext(
            0 as *const libc::c_char,
            b"Long option\0" as *const u8 as *const libc::c_char,
            5 as libc::c_int,
        ),
        b"Meaning\0" as *const u8 as *const libc::c_char,
    );
    print_opt(
        b"-A\0" as *const u8 as *const libc::c_char,
        b"--smarthome\0" as *const u8 as *const libc::c_char,
        b"Enable smart home key\0" as *const u8 as *const libc::c_char,
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
        print_opt(
            b"-B\0" as *const u8 as *const libc::c_char,
            b"--backup\0" as *const u8 as *const libc::c_char,
            b"Save backups of existing files\0" as *const u8 as *const libc::c_char,
        );
        print_opt(
            dcgettext(
                0 as *const libc::c_char,
                b"-C <dir>\0" as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ),
            dcgettext(
                0 as *const libc::c_char,
                b"--backupdir=<dir>\0" as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ),
            b"Directory for saving unique backup files\0" as *const u8
                as *const libc::c_char,
        );
    }
    print_opt(
        b"-D\0" as *const u8 as *const libc::c_char,
        b"--boldtext\0" as *const u8 as *const libc::c_char,
        b"Use bold instead of reverse video text\0" as *const u8 as *const libc::c_char,
    );
    print_opt(
        b"-E\0" as *const u8 as *const libc::c_char,
        b"--tabstospaces\0" as *const u8 as *const libc::c_char,
        b"Convert typed tabs to spaces\0" as *const u8 as *const libc::c_char,
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
        print_opt(
            b"-F\0" as *const u8 as *const libc::c_char,
            b"--multibuffer\0" as *const u8 as *const libc::c_char,
            b"Read a file into a new buffer by default\0" as *const u8
                as *const libc::c_char,
        );
    }
    print_opt(
        b"-G\0" as *const u8 as *const libc::c_char,
        b"--locking\0" as *const u8 as *const libc::c_char,
        b"Use (vim-style) lock files\0" as *const u8 as *const libc::c_char,
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
        print_opt(
            b"-H\0" as *const u8 as *const libc::c_char,
            b"--historylog\0" as *const u8 as *const libc::c_char,
            b"Save & reload old search/replace strings\0" as *const u8
                as *const libc::c_char,
        );
    }
    print_opt(
        b"-I\0" as *const u8 as *const libc::c_char,
        b"--ignorercfiles\0" as *const u8 as *const libc::c_char,
        b"Don't look at nanorc files\0" as *const u8 as *const libc::c_char,
    );
    print_opt(
        dcgettext(
            0 as *const libc::c_char,
            b"-J <number>\0" as *const u8 as *const libc::c_char,
            5 as libc::c_int,
        ),
        dcgettext(
            0 as *const libc::c_char,
            b"--guidestripe=<number>\0" as *const u8 as *const libc::c_char,
            5 as libc::c_int,
        ),
        b"Show a guiding bar at this column\0" as *const u8 as *const libc::c_char,
    );
    print_opt(
        b"-K\0" as *const u8 as *const libc::c_char,
        b"--rawsequences\0" as *const u8 as *const libc::c_char,
        b"Fix numeric keypad key confusion problem\0" as *const u8 as *const libc::c_char,
    );
    print_opt(
        b"-L\0" as *const u8 as *const libc::c_char,
        b"--nonewlines\0" as *const u8 as *const libc::c_char,
        b"Don't add an automatic newline\0" as *const u8 as *const libc::c_char,
    );
    print_opt(
        b"-M\0" as *const u8 as *const libc::c_char,
        b"--trimblanks\0" as *const u8 as *const libc::c_char,
        b"Trim tail spaces when hard-wrapping\0" as *const u8 as *const libc::c_char,
    );
    print_opt(
        b"-N\0" as *const u8 as *const libc::c_char,
        b"--noconvert\0" as *const u8 as *const libc::c_char,
        b"Don't convert files from DOS/Mac format\0" as *const u8 as *const libc::c_char,
    );
    print_opt(
        b"-O\0" as *const u8 as *const libc::c_char,
        b"--bookstyle\0" as *const u8 as *const libc::c_char,
        b"Leading whitespace means new paragraph\0" as *const u8 as *const libc::c_char,
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
        print_opt(
            b"-P\0" as *const u8 as *const libc::c_char,
            b"--positionlog\0" as *const u8 as *const libc::c_char,
            b"Save & restore position of the cursor\0" as *const u8
                as *const libc::c_char,
        );
    }
    print_opt(
        dcgettext(
            0 as *const libc::c_char,
            b"-Q <regex>\0" as *const u8 as *const libc::c_char,
            5 as libc::c_int,
        ),
        dcgettext(
            0 as *const libc::c_char,
            b"--quotestr=<regex>\0" as *const u8 as *const libc::c_char,
            5 as libc::c_int,
        ),
        b"Regular expression to match quoting\0" as *const u8 as *const libc::c_char,
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
        print_opt(
            b"-R\0" as *const u8 as *const libc::c_char,
            b"--restricted\0" as *const u8 as *const libc::c_char,
            b"Restrict access to the filesystem\0" as *const u8 as *const libc::c_char,
        );
    }
    print_opt(
        b"-S\0" as *const u8 as *const libc::c_char,
        b"--softwrap\0" as *const u8 as *const libc::c_char,
        b"Display overlong lines on multiple rows\0" as *const u8 as *const libc::c_char,
    );
    print_opt(
        dcgettext(
            0 as *const libc::c_char,
            b"-T <number>\0" as *const u8 as *const libc::c_char,
            5 as libc::c_int,
        ),
        dcgettext(
            0 as *const libc::c_char,
            b"--tabsize=<number>\0" as *const u8 as *const libc::c_char,
            5 as libc::c_int,
        ),
        b"Make a tab this number of columns wide\0" as *const u8 as *const libc::c_char,
    );
    print_opt(
        b"-U\0" as *const u8 as *const libc::c_char,
        b"--quickblank\0" as *const u8 as *const libc::c_char,
        b"Wipe status bar upon next keystroke\0" as *const u8 as *const libc::c_char,
    );
    print_opt(
        b"-V\0" as *const u8 as *const libc::c_char,
        b"--version\0" as *const u8 as *const libc::c_char,
        b"Print version information and exit\0" as *const u8 as *const libc::c_char,
    );
    print_opt(
        b"-W\0" as *const u8 as *const libc::c_char,
        b"--wordbounds\0" as *const u8 as *const libc::c_char,
        b"Detect word boundaries more accurately\0" as *const u8 as *const libc::c_char,
    );
    print_opt(
        dcgettext(
            0 as *const libc::c_char,
            b"-X <string>\0" as *const u8 as *const libc::c_char,
            5 as libc::c_int,
        ),
        dcgettext(
            0 as *const libc::c_char,
            b"--wordchars=<string>\0" as *const u8 as *const libc::c_char,
            5 as libc::c_int,
        ),
        b"Which other characters are word parts\0" as *const u8 as *const libc::c_char,
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
        print_opt(
            dcgettext(
                0 as *const libc::c_char,
                b"-Y <name>\0" as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ),
            dcgettext(
                0 as *const libc::c_char,
                b"--syntax=<name>\0" as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ),
            b"Syntax definition to use for coloring\0" as *const u8
                as *const libc::c_char,
        );
    }
    print_opt(
        b"-Z\0" as *const u8 as *const libc::c_char,
        b"--zap\0" as *const u8 as *const libc::c_char,
        b"Let Bsp and Del erase a marked region\0" as *const u8 as *const libc::c_char,
    );
    print_opt(
        b"-a\0" as *const u8 as *const libc::c_char,
        b"--atblanks\0" as *const u8 as *const libc::c_char,
        b"When soft-wrapping, do it at whitespace\0" as *const u8 as *const libc::c_char,
    );
    print_opt(
        b"-b\0" as *const u8 as *const libc::c_char,
        b"--breaklonglines\0" as *const u8 as *const libc::c_char,
        b"Automatically hard-wrap overlong lines\0" as *const u8 as *const libc::c_char,
    );
    print_opt(
        b"-c\0" as *const u8 as *const libc::c_char,
        b"--constantshow\0" as *const u8 as *const libc::c_char,
        b"Constantly show cursor position\0" as *const u8 as *const libc::c_char,
    );
    print_opt(
        b"-d\0" as *const u8 as *const libc::c_char,
        b"--rebinddelete\0" as *const u8 as *const libc::c_char,
        b"Fix Backspace/Delete confusion problem\0" as *const u8 as *const libc::c_char,
    );
    print_opt(
        b"-e\0" as *const u8 as *const libc::c_char,
        b"--emptyline\0" as *const u8 as *const libc::c_char,
        b"Keep the line below the title bar empty\0" as *const u8 as *const libc::c_char,
    );
    print_opt(
        dcgettext(
            0 as *const libc::c_char,
            b"-f <file>\0" as *const u8 as *const libc::c_char,
            5 as libc::c_int,
        ),
        dcgettext(
            0 as *const libc::c_char,
            b"--rcfile=<file>\0" as *const u8 as *const libc::c_char,
            5 as libc::c_int,
        ),
        b"Use only this file for configuring nano\0" as *const u8 as *const libc::c_char,
    );
    print_opt(
        b"-g\0" as *const u8 as *const libc::c_char,
        b"--showcursor\0" as *const u8 as *const libc::c_char,
        b"Show cursor in file browser & help text\0" as *const u8 as *const libc::c_char,
    );
    print_opt(
        b"-h\0" as *const u8 as *const libc::c_char,
        b"--help\0" as *const u8 as *const libc::c_char,
        b"Show this help text and exit\0" as *const u8 as *const libc::c_char,
    );
    print_opt(
        b"-i\0" as *const u8 as *const libc::c_char,
        b"--autoindent\0" as *const u8 as *const libc::c_char,
        b"Automatically indent new lines\0" as *const u8 as *const libc::c_char,
    );
    print_opt(
        b"-j\0" as *const u8 as *const libc::c_char,
        b"--jumpyscrolling\0" as *const u8 as *const libc::c_char,
        b"Scroll per half-screen, not per line\0" as *const u8 as *const libc::c_char,
    );
    print_opt(
        b"-k\0" as *const u8 as *const libc::c_char,
        b"--cutfromcursor\0" as *const u8 as *const libc::c_char,
        b"Cut from cursor to end of line\0" as *const u8 as *const libc::c_char,
    );
    print_opt(
        b"-l\0" as *const u8 as *const libc::c_char,
        b"--linenumbers\0" as *const u8 as *const libc::c_char,
        b"Show line numbers in front of the text\0" as *const u8 as *const libc::c_char,
    );
    print_opt(
        b"-m\0" as *const u8 as *const libc::c_char,
        b"--mouse\0" as *const u8 as *const libc::c_char,
        b"Enable the use of the mouse\0" as *const u8 as *const libc::c_char,
    );
    print_opt(
        b"-n\0" as *const u8 as *const libc::c_char,
        b"--noread\0" as *const u8 as *const libc::c_char,
        b"Do not read the file (only write it)\0" as *const u8 as *const libc::c_char,
    );
    print_opt(
        dcgettext(
            0 as *const libc::c_char,
            b"-o <dir>\0" as *const u8 as *const libc::c_char,
            5 as libc::c_int,
        ),
        dcgettext(
            0 as *const libc::c_char,
            b"--operatingdir=<dir>\0" as *const u8 as *const libc::c_char,
            5 as libc::c_int,
        ),
        b"Set operating directory\0" as *const u8 as *const libc::c_char,
    );
    print_opt(
        b"-p\0" as *const u8 as *const libc::c_char,
        b"--preserve\0" as *const u8 as *const libc::c_char,
        b"Preserve XON (^Q) and XOFF (^S) keys\0" as *const u8 as *const libc::c_char,
    );
    print_opt(
        b"-q\0" as *const u8 as *const libc::c_char,
        b"--indicator\0" as *const u8 as *const libc::c_char,
        b"Show a position+portion indicator\0" as *const u8 as *const libc::c_char,
    );
    print_opt(
        dcgettext(
            0 as *const libc::c_char,
            b"-r <number>\0" as *const u8 as *const libc::c_char,
            5 as libc::c_int,
        ),
        dcgettext(
            0 as *const libc::c_char,
            b"--fill=<number>\0" as *const u8 as *const libc::c_char,
            5 as libc::c_int,
        ),
        b"Set width for hard-wrap and justify\0" as *const u8 as *const libc::c_char,
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
        print_opt(
            dcgettext(
                0 as *const libc::c_char,
                b"-s <program>\0" as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ),
            dcgettext(
                0 as *const libc::c_char,
                b"--speller=<program>\0" as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ),
            b"Use this alternative spell checker\0" as *const u8 as *const libc::c_char,
        );
    }
    print_opt(
        b"-t\0" as *const u8 as *const libc::c_char,
        b"--saveonexit\0" as *const u8 as *const libc::c_char,
        b"Save changes on exit, don't prompt\0" as *const u8 as *const libc::c_char,
    );
    print_opt(
        b"-u\0" as *const u8 as *const libc::c_char,
        b"--unix\0" as *const u8 as *const libc::c_char,
        b"Save a file by default in Unix format\0" as *const u8 as *const libc::c_char,
    );
    print_opt(
        b"-v\0" as *const u8 as *const libc::c_char,
        b"--view\0" as *const u8 as *const libc::c_char,
        b"View mode (read-only)\0" as *const u8 as *const libc::c_char,
    );
    print_opt(
        b"-w\0" as *const u8 as *const libc::c_char,
        b"--nowrap\0" as *const u8 as *const libc::c_char,
        b"Don't hard-wrap long lines [default]\0" as *const u8 as *const libc::c_char,
    );
    print_opt(
        b"-x\0" as *const u8 as *const libc::c_char,
        b"--nohelp\0" as *const u8 as *const libc::c_char,
        b"Don't show the two help lines\0" as *const u8 as *const libc::c_char,
    );
    print_opt(
        b"-y\0" as *const u8 as *const libc::c_char,
        b"--afterends\0" as *const u8 as *const libc::c_char,
        b"Make Ctrl+Right stop at word ends\0" as *const u8 as *const libc::c_char,
    );
    print_opt(
        b"-%\0" as *const u8 as *const libc::c_char,
        b"--stateflags\0" as *const u8 as *const libc::c_char,
        b"Show some states on the title bar\0" as *const u8 as *const libc::c_char,
    );
    print_opt(
        b"-_\0" as *const u8 as *const libc::c_char,
        b"--minibar\0" as *const u8 as *const libc::c_char,
        b"Show a feedback bar at the bottom\0" as *const u8 as *const libc::c_char,
    );
    print_opt(
        b"-0\0" as *const u8 as *const libc::c_char,
        b"--zero\0" as *const u8 as *const libc::c_char,
        b"Hide all bars, use whole terminal\0" as *const u8 as *const libc::c_char,
    );
}
pub unsafe extern "C" fn version() {
    printf(
        dcgettext(
            0 as *const libc::c_char,
            b" GNU nano, version %s\n\0" as *const u8 as *const libc::c_char,
            5 as libc::c_int,
        ),
        b"7.2\0" as *const u8 as *const libc::c_char,
    );
    printf(
        dcgettext(
            0 as *const libc::c_char,
            b" (C) %s the Free Software Foundation and various contributors\n\0"
                as *const u8 as *const libc::c_char,
            5 as libc::c_int,
        ),
        b"2023\0" as *const u8 as *const libc::c_char,
    );
    printf(
        dcgettext(
            0 as *const libc::c_char,
            b" Compiled options:\0" as *const u8 as *const libc::c_char,
            5 as libc::c_int,
        ),
    );
    printf(b" --disable-libmagic\0" as *const u8 as *const libc::c_char);
    printf(b" --enable-utf8\0" as *const u8 as *const libc::c_char);
    printf(b"\n\0" as *const u8 as *const libc::c_char);
}
pub unsafe extern "C" fn make_a_note(mut signal: libc::c_int) {
    control_C_was_pressed = 1 as libc::c_int != 0;
}
pub unsafe extern "C" fn install_handler_for_Ctrl_C() {
    enable_kb_interrupt();
    newaction
        .__sigaction_handler
        .sa_handler = Some(make_a_note as unsafe extern "C" fn(libc::c_int) -> ());
    newaction.sa_flags = 0 as libc::c_int;
    sigaction(2 as libc::c_int, &mut newaction, &mut oldaction);
}
pub unsafe extern "C" fn restore_handler_for_Ctrl_C() {
    sigaction(2 as libc::c_int, &mut oldaction, 0 as *mut sigaction);
    disable_kb_interrupt();
}
pub unsafe extern "C" fn reconnect_and_store_state() {
    let mut thetty: libc::c_int = open(
        b"/dev/tty\0" as *const u8 as *const libc::c_char,
        0 as libc::c_int,
    );
    if thetty < 0 as libc::c_int || dup2(thetty, 0 as libc::c_int) < 0 as libc::c_int {
        die(
            dcgettext(
                0 as *const libc::c_char,
                b"Could not reconnect stdin to keyboard\n\0" as *const u8
                    as *const libc::c_char,
                5 as libc::c_int,
            ),
        );
    }
    close(thetty);
    if !control_C_was_pressed {
        tcgetattr(0 as libc::c_int, &mut original_state);
    }
}
pub unsafe extern "C" fn scoop_stdin() -> bool {
    let mut stream: *mut FILE = 0 as *mut FILE;
    restore_terminal();
    if isatty(0 as libc::c_int) != 0 {
        fprintf(
            stderr,
            dcgettext(
                0 as *const libc::c_char,
                b"Reading data from keyboard; type ^D or ^D^D to finish.\n\0"
                    as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ),
        );
    }
    stream = fopen(
        b"/dev/stdin\0" as *const u8 as *const libc::c_char,
        b"rb\0" as *const u8 as *const libc::c_char,
    );
    if stream.is_null() {
        let mut errnumber: libc::c_int = *__errno_location();
        terminal_init();
        doupdate();
        statusline(
            ALERT,
            dcgettext(
                0 as *const libc::c_char,
                b"Failed to open stdin: %s\0" as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ),
            strerror(errnumber),
        );
        return 0 as libc::c_int != 0;
    }
    install_handler_for_Ctrl_C();
    make_new_buffer();
    read_file(
        stream,
        0 as libc::c_int,
        b"stdin\0" as *const u8 as *const libc::c_char,
        1 as libc::c_int != 0,
    );
    find_and_prime_applicable_syntax();
    restore_handler_for_Ctrl_C();
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
        && (*openfile).totsize > 0 as libc::c_int as libc::c_ulong
    {
        set_modified();
    }
    return 1 as libc::c_int != 0;
}
pub unsafe extern "C" fn signal_init() {
    let mut deed: sigaction = {
        let mut init = sigaction {
            __sigaction_handler: C2RustUnnamed_9 {
                sa_handler: None,
            },
            sa_mask: __sigset_t { __val: [0; 16] },
            sa_flags: 0,
            sa_restorer: None,
        };
        init
    };
    deed
        .__sigaction_handler
        .sa_handler = ::std::mem::transmute::<
        libc::intptr_t,
        __sighandler_t,
    >(1 as libc::c_int as libc::intptr_t);
    sigaction(2 as libc::c_int, &mut deed, 0 as *mut sigaction);
    sigaction(3 as libc::c_int, &mut deed, 0 as *mut sigaction);
    deed
        .__sigaction_handler
        .sa_handler = Some(handle_hupterm as unsafe extern "C" fn(libc::c_int) -> ());
    sigaction(1 as libc::c_int, &mut deed, 0 as *mut sigaction);
    sigaction(15 as libc::c_int, &mut deed, 0 as *mut sigaction);
    deed
        .__sigaction_handler
        .sa_handler = Some(handle_sigwinch as unsafe extern "C" fn(libc::c_int) -> ());
    sigaction(28 as libc::c_int, &mut deed, 0 as *mut sigaction);
    sigfillset(&mut deed.sa_mask);
    deed
        .__sigaction_handler
        .sa_handler = Some(suspend_nano as unsafe extern "C" fn(libc::c_int) -> ());
    sigaction(20 as libc::c_int, &mut deed, 0 as *mut sigaction);
    sigfillset(&mut deed.sa_mask);
    deed
        .__sigaction_handler
        .sa_handler = Some(continue_nano as unsafe extern "C" fn(libc::c_int) -> ());
    sigaction(18 as libc::c_int, &mut deed, 0 as *mut sigaction);
    if (getenv(b"NANO_NOCATCH\0" as *const u8 as *const libc::c_char)).is_null() {
        deed
            .__sigaction_handler
            .sa_handler = Some(handle_crash as unsafe extern "C" fn(libc::c_int) -> ());
        deed
            .sa_flags = (deed.sa_flags as libc::c_uint | 0x80000000 as libc::c_uint)
            as libc::c_int;
        sigaction(11 as libc::c_int, &mut deed, 0 as *mut sigaction);
        sigaction(6 as libc::c_int, &mut deed, 0 as *mut sigaction);
    }
}
pub unsafe extern "C" fn handle_hupterm(mut signal: libc::c_int) {
    die(
        dcgettext(
            0 as *const libc::c_char,
            b"Received SIGHUP or SIGTERM\n\0" as *const u8 as *const libc::c_char,
            5 as libc::c_int,
        ),
    );
}
pub unsafe extern "C" fn handle_crash(mut signal: libc::c_int) {
    die(
        dcgettext(
            0 as *const libc::c_char,
            b"Sorry! Nano crashed!  Code: %d.  Please report a bug.\n\0" as *const u8
                as *const libc::c_char,
            5 as libc::c_int,
        ),
        signal,
    );
}
pub unsafe extern "C" fn suspend_nano(mut signal: libc::c_int) {
    disable_mouse_support();
    restore_terminal();
    printf(b"\n\n\0" as *const u8 as *const libc::c_char);
    printf(
        dcgettext(
            0 as *const libc::c_char,
            b"Use \"fg\" to return to nano.\n\0" as *const u8 as *const libc::c_char,
            5 as libc::c_int,
        ),
    );
    fflush(stdout);
    lastmessage = HUSH;
    kill(0 as libc::c_int, 19 as libc::c_int);
}
pub unsafe extern "C" fn do_suspend() {
    if in_restricted_mode() {
        return;
    }
    suspend_nano(0 as libc::c_int);
    ran_a_tool = 1 as libc::c_int != 0;
}
pub unsafe extern "C" fn continue_nano(mut signal: libc::c_int) {
    if flags[(USE_MOUSE as libc::c_int as libc::c_ulong)
        .wrapping_div(
            (::std::mem::size_of::<libc::c_uint>() as libc::c_ulong)
                .wrapping_mul(8 as libc::c_int as libc::c_ulong),
        ) as usize]
        & (1 as libc::c_int as libc::c_uint)
            << (USE_MOUSE as libc::c_int as libc::c_ulong)
                .wrapping_rem(
                    (::std::mem::size_of::<libc::c_uint>() as libc::c_ulong)
                        .wrapping_mul(8 as libc::c_int as libc::c_ulong),
                ) != 0 as libc::c_int as libc::c_uint
    {
        enable_mouse_support();
    }
    ::std::ptr::write_volatile(
        &mut the_window_resized as *mut sig_atomic_t,
        1 as libc::c_int,
    );
    ungetch(0x4fe as libc::c_int);
}
pub unsafe extern "C" fn block_sigwinch(mut blockit: bool) {
    let mut winch: sigset_t = __sigset_t { __val: [0; 16] };
    sigemptyset(&mut winch);
    sigaddset(&mut winch, 28 as libc::c_int);
    sigprocmask(
        if blockit as libc::c_int != 0 { 0 as libc::c_int } else { 1 as libc::c_int },
        &mut winch,
        0 as *mut sigset_t,
    );
    if the_window_resized != 0 {
        regenerate_screen();
    }
}
pub unsafe extern "C" fn handle_sigwinch(mut signal: libc::c_int) {
    ::std::ptr::write_volatile(
        &mut the_window_resized as *mut sig_atomic_t,
        1 as libc::c_int,
    );
}
pub unsafe extern "C" fn regenerate_screen() {
    ::std::ptr::write_volatile(
        &mut the_window_resized as *mut sig_atomic_t,
        0 as libc::c_int,
    );
    endwin();
    wrefresh(stdscr);
    thebar = if flags[(INDICATOR as libc::c_int as libc::c_ulong)
        .wrapping_div(
            (::std::mem::size_of::<libc::c_uint>() as libc::c_ulong)
                .wrapping_mul(8 as libc::c_int as libc::c_ulong),
        ) as usize]
        & (1 as libc::c_int as libc::c_uint)
            << (INDICATOR as libc::c_int as libc::c_ulong)
                .wrapping_rem(
                    (::std::mem::size_of::<libc::c_uint>() as libc::c_ulong)
                        .wrapping_mul(8 as libc::c_int as libc::c_ulong),
                ) != 0 as libc::c_int as libc::c_uint && LINES > 5 as libc::c_int
        && COLS > 9 as libc::c_int
    {
        1 as libc::c_int
    } else {
        0 as libc::c_int
    };
    bardata = nrealloc(
        bardata as *mut libc::c_void,
        (LINES as libc::c_ulong)
            .wrapping_mul(::std::mem::size_of::<libc::c_int>() as libc::c_ulong),
    ) as *mut libc::c_int;
    editwincols = COLS - margin - thebar;
    terminal_init();
    window_init();
    if !openfile.is_null() {
        ensure_firstcolumn_is_aligned();
        draw_all_subwindows();
    }
}
pub unsafe extern "C" fn toggle_this(mut flag: libc::c_int) {
    let mut enabled: bool = !(flags[(flag as libc::c_ulong)
        .wrapping_div(
            (::std::mem::size_of::<libc::c_uint>() as libc::c_ulong)
                .wrapping_mul(8 as libc::c_int as libc::c_ulong),
        ) as usize]
        & (1 as libc::c_int as libc::c_uint)
            << (flag as libc::c_ulong)
                .wrapping_rem(
                    (::std::mem::size_of::<libc::c_uint>() as libc::c_ulong)
                        .wrapping_mul(8 as libc::c_int as libc::c_ulong),
                ) != 0 as libc::c_int as libc::c_uint);
    flags[(flag as libc::c_ulong)
        .wrapping_div(
            (::std::mem::size_of::<libc::c_uint>() as libc::c_ulong)
                .wrapping_mul(8 as libc::c_int as libc::c_ulong),
        ) as usize]
        ^= (1 as libc::c_int as libc::c_uint)
            << (flag as libc::c_ulong)
                .wrapping_rem(
                    (::std::mem::size_of::<libc::c_uint>() as libc::c_ulong)
                        .wrapping_mul(8 as libc::c_int as libc::c_ulong),
                );
    focusing = 0 as libc::c_int != 0;
    match flag {
        48 => {
            window_init();
            draw_all_subwindows();
            return;
        }
        3 => {
            if LINES < 6 as libc::c_int {
                statusline(
                    AHEM,
                    dcgettext(
                        0 as *const libc::c_char,
                        b"Too tiny\0" as *const u8 as *const libc::c_char,
                        5 as libc::c_int,
                    ),
                );
                flags[(flag as libc::c_ulong)
                    .wrapping_div(
                        (::std::mem::size_of::<libc::c_uint>() as libc::c_ulong)
                            .wrapping_mul(8 as libc::c_int as libc::c_ulong),
                    ) as usize]
                    ^= (1 as libc::c_int as libc::c_uint)
                        << (flag as libc::c_ulong)
                            .wrapping_rem(
                                (::std::mem::size_of::<libc::c_uint>() as libc::c_ulong)
                                    .wrapping_mul(8 as libc::c_int as libc::c_ulong),
                            );
                return;
            }
            window_init();
            draw_all_subwindows();
        }
        2 => {
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
                || LINES == 1 as libc::c_int
            {
                statusline(
                    AHEM,
                    dcgettext(
                        0 as *const libc::c_char,
                        b"Not possible\0" as *const u8 as *const libc::c_char,
                        5 as libc::c_int,
                    ),
                );
                flags[(flag as libc::c_ulong)
                    .wrapping_div(
                        (::std::mem::size_of::<libc::c_uint>() as libc::c_ulong)
                            .wrapping_mul(8 as libc::c_int as libc::c_ulong),
                    ) as usize]
                    ^= (1 as libc::c_int as libc::c_uint)
                        << (flag as libc::c_ulong)
                            .wrapping_rem(
                                (::std::mem::size_of::<libc::c_uint>() as libc::c_ulong)
                                    .wrapping_mul(8 as libc::c_int as libc::c_ulong),
                            );
            } else if !(flags[(MINIBAR as libc::c_int as libc::c_ulong)
                .wrapping_div(
                    (::std::mem::size_of::<libc::c_uint>() as libc::c_ulong)
                        .wrapping_mul(8 as libc::c_int as libc::c_ulong),
                ) as usize]
                & (1 as libc::c_int as libc::c_uint)
                    << (MINIBAR as libc::c_int as libc::c_ulong)
                        .wrapping_rem(
                            (::std::mem::size_of::<libc::c_uint>() as libc::c_ulong)
                                .wrapping_mul(8 as libc::c_int as libc::c_ulong),
                        ) != 0 as libc::c_int as libc::c_uint)
            {
                wipe_statusbar();
            }
            return;
        }
        29 => {
            if !(flags[(SOFTWRAP as libc::c_int as libc::c_ulong)
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
            {
                (*openfile).firstcolumn = 0 as libc::c_int as size_t;
            }
            refresh_needed = 1 as libc::c_int != 0;
        }
        23 => {
            titlebar(0 as *const libc::c_char);
            refresh_needed = 1 as libc::c_int != 0;
        }
        18 => {
            precalc_multicolorinfo();
            refresh_needed = 1 as libc::c_int != 0;
        }
        24 => {
            if !((*openfile).syntax).is_null() && !((*(*openfile).syntax).tab).is_null()
            {
                statusline(
                    AHEM,
                    dcgettext(
                        0 as *const libc::c_char,
                        b"Current syntax determines Tab\0" as *const u8
                            as *const libc::c_char,
                        5 as libc::c_int,
                    ),
                );
                flags[(flag as libc::c_ulong)
                    .wrapping_div(
                        (::std::mem::size_of::<libc::c_uint>() as libc::c_ulong)
                            .wrapping_mul(8 as libc::c_int as libc::c_ulong),
                    ) as usize]
                    ^= (1 as libc::c_int as libc::c_uint)
                        << (flag as libc::c_ulong)
                            .wrapping_rem(
                                (::std::mem::size_of::<libc::c_uint>() as libc::c_ulong)
                                    .wrapping_mul(8 as libc::c_int as libc::c_ulong),
                            );
                return;
            }
        }
        7 => {
            mouse_init();
        }
        _ => {}
    }
    if flag == AUTOINDENT as libc::c_int || flag == BREAK_LONG_LINES as libc::c_int
        || flag == SOFTWRAP as libc::c_int
    {
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
        {
            return;
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
    if flag == NO_HELP as libc::c_int || flag == LINE_NUMBERS as libc::c_int
        || flag == WHITESPACE_DISPLAY as libc::c_int
    {
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
            return;
        }
    }
    if flag == NO_HELP as libc::c_int || flag == NO_SYNTAX as libc::c_int {
        enabled = !enabled;
    }
    statusline(
        REMARK,
        b"%s %s\0" as *const u8 as *const libc::c_char,
        dcgettext(0 as *const libc::c_char, epithet_of_flag(flag), 5 as libc::c_int),
        if enabled as libc::c_int != 0 {
            dcgettext(
                0 as *const libc::c_char,
                b"enabled\0" as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            )
        } else {
            dcgettext(
                0 as *const libc::c_char,
                b"disabled\0" as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            )
        },
    );
}
pub unsafe extern "C" fn disable_extended_io() {
    let mut settings: termios = {
        let mut init = termios {
            c_iflag: 0 as libc::c_int as tcflag_t,
            c_oflag: 0,
            c_cflag: 0,
            c_lflag: 0,
            c_line: 0,
            c_cc: [0; 32],
            c_ispeed: 0,
            c_ospeed: 0,
        };
        init
    };
    tcgetattr(0 as libc::c_int, &mut settings);
    settings.c_lflag &= !(0o100000 as libc::c_int) as libc::c_uint;
    settings.c_oflag &= !(0o1 as libc::c_int) as libc::c_uint;
    tcsetattr(0 as libc::c_int, 0 as libc::c_int, &mut settings);
}
pub unsafe extern "C" fn disable_kb_interrupt() {
    let mut settings: termios = {
        let mut init = termios {
            c_iflag: 0 as libc::c_int as tcflag_t,
            c_oflag: 0,
            c_cflag: 0,
            c_lflag: 0,
            c_line: 0,
            c_cc: [0; 32],
            c_ispeed: 0,
            c_ospeed: 0,
        };
        init
    };
    tcgetattr(0 as libc::c_int, &mut settings);
    settings.c_lflag &= !(0o1 as libc::c_int) as libc::c_uint;
    tcsetattr(0 as libc::c_int, 0 as libc::c_int, &mut settings);
}
pub unsafe extern "C" fn enable_kb_interrupt() {
    let mut settings: termios = {
        let mut init = termios {
            c_iflag: 0 as libc::c_int as tcflag_t,
            c_oflag: 0,
            c_cflag: 0,
            c_lflag: 0,
            c_line: 0,
            c_cc: [0; 32],
            c_ispeed: 0,
            c_ospeed: 0,
        };
        init
    };
    tcgetattr(0 as libc::c_int, &mut settings);
    settings.c_lflag |= 0o1 as libc::c_int as libc::c_uint;
    tcsetattr(0 as libc::c_int, 0 as libc::c_int, &mut settings);
}
pub unsafe extern "C" fn disable_flow_control() {
    let mut settings: termios = termios {
        c_iflag: 0,
        c_oflag: 0,
        c_cflag: 0,
        c_lflag: 0,
        c_line: 0,
        c_cc: [0; 32],
        c_ispeed: 0,
        c_ospeed: 0,
    };
    tcgetattr(0 as libc::c_int, &mut settings);
    settings.c_iflag &= !(0o2000 as libc::c_int) as libc::c_uint;
    tcsetattr(0 as libc::c_int, 0 as libc::c_int, &mut settings);
}
pub unsafe extern "C" fn enable_flow_control() {
    let mut settings: termios = termios {
        c_iflag: 0,
        c_oflag: 0,
        c_cflag: 0,
        c_lflag: 0,
        c_line: 0,
        c_cc: [0; 32],
        c_ispeed: 0,
        c_ospeed: 0,
    };
    tcgetattr(0 as libc::c_int, &mut settings);
    settings.c_iflag |= 0o2000 as libc::c_int as libc::c_uint;
    tcsetattr(0 as libc::c_int, 0 as libc::c_int, &mut settings);
}
pub unsafe extern "C" fn terminal_init() {
    raw();
    nonl();
    noecho();
    disable_extended_io();
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
    disable_kb_interrupt();
    printf(b"\x1B[?2004h\0" as *const u8 as *const libc::c_char);
    fflush(stdout);
}
pub unsafe extern "C" fn get_keycode(
    mut keyname: *const libc::c_char,
    standard: libc::c_int,
) -> libc::c_int {
    let mut keyvalue: *const libc::c_char = tigetstr(keyname);
    if !keyvalue.is_null()
        && keyvalue != -(1 as libc::c_int) as *mut libc::c_char as *const libc::c_char
        && key_defined(keyvalue) != 0
    {
        return key_defined(keyvalue);
    }
    return standard;
}
pub unsafe extern "C" fn confirm_margin() {
    let mut needed_margin: libc::c_int = digits((*(*openfile).filebot).lineno)
        + 1 as libc::c_int;
    if !(flags[(LINE_NUMBERS as libc::c_int as libc::c_ulong)
        .wrapping_div(
            (::std::mem::size_of::<libc::c_uint>() as libc::c_ulong)
                .wrapping_mul(8 as libc::c_int as libc::c_ulong),
        ) as usize]
        & (1 as libc::c_int as libc::c_uint)
            << (LINE_NUMBERS as libc::c_int as libc::c_ulong)
                .wrapping_rem(
                    (::std::mem::size_of::<libc::c_uint>() as libc::c_ulong)
                        .wrapping_mul(8 as libc::c_int as libc::c_ulong),
                ) != 0 as libc::c_int as libc::c_uint)
        || needed_margin > COLS - 4 as libc::c_int
    {
        needed_margin = 0 as libc::c_int;
    }
    if needed_margin != margin {
        let mut keep_focus: bool = margin > 0 as libc::c_int
            && focusing as libc::c_int != 0;
        margin = needed_margin;
        editwincols = COLS - margin - thebar;
        ensure_firstcolumn_is_aligned();
        focusing = keep_focus;
        refresh_needed = 1 as libc::c_int != 0;
    }
}
pub unsafe extern "C" fn unbound_key(mut code: libc::c_int) {
    if code == 0x4fc as libc::c_int {
        statusline(
            AHEM,
            dcgettext(
                0 as *const libc::c_char,
                b"Unknown sequence\0" as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ),
        );
    } else if code == 0x4ef as libc::c_int {
        statusline(
            AHEM,
            dcgettext(
                0 as *const libc::c_char,
                b"Unknown function: %s\0" as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ),
            commandname,
        );
    } else if code == 0x4eb as libc::c_int {
        statusline(
            AHEM,
            dcgettext(
                0 as *const libc::c_char,
                b"Missing }\0" as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ),
        );
    } else if code > 0o410 as libc::c_int
        && code < 0o410 as libc::c_int + 25 as libc::c_int
    {
        statusline(
            AHEM,
            dcgettext(
                0 as *const libc::c_char,
                b"Unbound key: F%i\0" as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ),
            code - 0o410 as libc::c_int,
        );
    } else if code > 0x7f as libc::c_int {
        statusline(
            AHEM,
            dcgettext(
                0 as *const libc::c_char,
                b"Unbound key\0" as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ),
        );
    } else if meta_key {
        if code < 0x20 as libc::c_int {
            statusline(
                AHEM,
                dcgettext(
                    0 as *const libc::c_char,
                    b"Unbindable key: M-^%c\0" as *const u8 as *const libc::c_char,
                    5 as libc::c_int,
                ),
                code + 0x40 as libc::c_int,
            );
        } else if shifted_metas as libc::c_int != 0 && 'A' as i32 <= code
            && code <= 'Z' as i32
        {
            statusline(
                AHEM,
                dcgettext(
                    0 as *const libc::c_char,
                    b"Unbound key: %s%c\0" as *const u8 as *const libc::c_char,
                    5 as libc::c_int,
                ),
                b"Sh-M-\0" as *const u8 as *const libc::c_char,
                code,
            );
        } else {
            statusline(
                AHEM,
                dcgettext(
                    0 as *const libc::c_char,
                    b"Unbound key: %s%c\0" as *const u8 as *const libc::c_char,
                    5 as libc::c_int,
                ),
                b"M-\0" as *const u8 as *const libc::c_char,
                {
                    let mut __res: libc::c_int = 0;
                    if ::std::mem::size_of::<libc::c_int>() as libc::c_ulong
                        > 1 as libc::c_int as libc::c_ulong
                    {
                        if 0 != 0 {
                            let mut __c: libc::c_int = code;
                            __res = if __c < -(128 as libc::c_int)
                                || __c > 255 as libc::c_int
                            {
                                __c
                            } else {
                                *(*__ctype_toupper_loc()).offset(__c as isize)
                            };
                        } else {
                            __res = toupper(code);
                        }
                    } else {
                        __res = *(*__ctype_toupper_loc()).offset(code as isize);
                    }
                    __res
                },
            );
        }
    } else if code == 0x1b as libc::c_int {
        statusline(
            AHEM,
            dcgettext(
                0 as *const libc::c_char,
                b"Unbindable key: ^[\0" as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ),
        );
    } else if code < 0x20 as libc::c_int {
        statusline(
            AHEM,
            dcgettext(
                0 as *const libc::c_char,
                b"Unbound key: %s%c\0" as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ),
            b"^\0" as *const u8 as *const libc::c_char,
            code + 0x40 as libc::c_int,
        );
    } else {
        statusline(
            AHEM,
            dcgettext(
                0 as *const libc::c_char,
                b"Unbound key: %s%c\0" as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ),
            b"\0" as *const u8 as *const libc::c_char,
            code,
        );
    }
    set_blankdelay_to_one();
}
pub unsafe extern "C" fn do_mouse() -> libc::c_int {
    let mut click_row: libc::c_int = 0;
    let mut click_col: libc::c_int = 0;
    let mut retval: libc::c_int = get_mouseinput(
        &mut click_row,
        &mut click_col,
        1 as libc::c_int != 0,
    );
    if retval != 0 as libc::c_int {
        return retval;
    }
    if wmouse_trafo(midwin, &mut click_row, &mut click_col, 0 as libc::c_int != 0) {
        let mut current_save: *mut linestruct = (*openfile).current;
        let mut row_count: ssize_t = click_row as libc::c_long - (*openfile).current_y;
        let mut leftedge: size_t = 0;
        let mut current_x_save: size_t = (*openfile).current_x;
        let mut sameline: bool = click_row as libc::c_long == (*openfile).current_y;
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
            leftedge = leftedge_for(xplustabs(), (*openfile).current);
        } else {
            leftedge = get_page_start(xplustabs());
        }
        if row_count < 0 as libc::c_int as libc::c_long {
            go_back_chunks(
                -row_count as libc::c_int,
                &mut (*openfile).current,
                &mut leftedge,
            );
        } else {
            go_forward_chunks(
                row_count as libc::c_int,
                &mut (*openfile).current,
                &mut leftedge,
            );
        }
        (*openfile)
            .current_x = actual_x(
            (*(*openfile).current).data,
            actual_last_column(leftedge, click_col as size_t),
        );
        if sameline as libc::c_int != 0 && (*openfile).current_x == current_x_save {
            do_mark();
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
        } else {
            keep_cutbuffer = 0 as libc::c_int != 0;
        }
        edit_redraw(current_save, CENTERING);
    }
    return 2 as libc::c_int;
}
pub unsafe extern "C" fn wanted_to_move(
    mut func: Option::<unsafe extern "C" fn() -> ()>,
) -> bool {
    return func == Some(do_left as unsafe extern "C" fn() -> ())
        || func == Some(do_right as unsafe extern "C" fn() -> ())
        || func == Some(do_up as unsafe extern "C" fn() -> ())
        || func == Some(do_down as unsafe extern "C" fn() -> ())
        || func == Some(do_home as unsafe extern "C" fn() -> ())
        || func == Some(do_end as unsafe extern "C" fn() -> ())
        || func == Some(to_prev_word as unsafe extern "C" fn() -> ())
        || func == Some(to_next_word as unsafe extern "C" fn() -> ())
        || func == Some(to_para_begin as unsafe extern "C" fn() -> ())
        || func == Some(to_para_end as unsafe extern "C" fn() -> ())
        || func == Some(to_prev_block as unsafe extern "C" fn() -> ())
        || func == Some(to_next_block as unsafe extern "C" fn() -> ())
        || func == Some(do_page_up as unsafe extern "C" fn() -> ())
        || func == Some(do_page_down as unsafe extern "C" fn() -> ())
        || func == Some(to_first_line as unsafe extern "C" fn() -> ())
        || func == Some(to_last_line as unsafe extern "C" fn() -> ());
}
pub unsafe extern "C" fn changes_something(mut f: functionptrtype) -> bool {
    return f == Some(do_savefile as unsafe extern "C" fn() -> ())
        || f == Some(do_writeout as unsafe extern "C" fn() -> ())
        || f == Some(do_enter as unsafe extern "C" fn() -> ())
        || f == Some(do_tab as unsafe extern "C" fn() -> ())
        || f == Some(do_delete as unsafe extern "C" fn() -> ())
        || f == Some(do_backspace as unsafe extern "C" fn() -> ())
        || f == Some(cut_text as unsafe extern "C" fn() -> ())
        || f == Some(paste_text as unsafe extern "C" fn() -> ())
        || f == Some(chop_previous_word as unsafe extern "C" fn() -> ())
        || f == Some(chop_next_word as unsafe extern "C" fn() -> ())
        || f == Some(zap_text as unsafe extern "C" fn() -> ())
        || f == Some(cut_till_eof as unsafe extern "C" fn() -> ())
        || f == Some(do_execute as unsafe extern "C" fn() -> ())
        || f == Some(do_indent as unsafe extern "C" fn() -> ())
        || f == Some(do_unindent as unsafe extern "C" fn() -> ())
        || f == Some(do_justify as unsafe extern "C" fn() -> ())
        || f == Some(do_full_justify as unsafe extern "C" fn() -> ())
        || f == Some(do_comment as unsafe extern "C" fn() -> ())
        || f == Some(do_spell as unsafe extern "C" fn() -> ())
        || f == Some(do_formatter as unsafe extern "C" fn() -> ())
        || f == Some(complete_a_word as unsafe extern "C" fn() -> ())
        || f == Some(do_replace as unsafe extern "C" fn() -> ())
        || f == Some(do_verbatim_input as unsafe extern "C" fn() -> ());
}
pub unsafe extern "C" fn suck_up_input_and_paste_it() {
    let mut was_cutbuffer: *mut linestruct = cutbuffer;
    let mut line: *mut linestruct = make_new_node(0 as *mut linestruct);
    let mut index: size_t = 0 as libc::c_int as size_t;
    (*line).data = copy_of(b"\0" as *const u8 as *const libc::c_char);
    cutbuffer = line;
    while bracketed_paste {
        let mut input: libc::c_int = get_kbinput(midwin, 0 as libc::c_int != 0);
        if input == '\r' as i32 || input == '\n' as i32 {
            (*line).next = make_new_node(line);
            line = (*line).next;
            (*line).data = copy_of(b"\0" as *const u8 as *const libc::c_char);
            index = 0 as libc::c_int as size_t;
        } else if 0x20 as libc::c_int <= input && input <= 0xff as libc::c_int
            && input != 0x7f as libc::c_int || input == '\t' as i32
        {
            (*line)
                .data = nrealloc(
                (*line).data as *mut libc::c_void,
                index.wrapping_add(2 as libc::c_int as libc::c_ulong),
            ) as *mut libc::c_char;
            let fresh0 = index;
            index = index.wrapping_add(1);
            *((*line).data).offset(fresh0 as isize) = input as libc::c_char;
            *((*line).data).offset(index as isize) = '\0' as i32 as libc::c_char;
        } else if input != 0x4fb as libc::c_int {
            beep();
        }
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
        print_view_warning();
    } else {
        paste_text();
    }
    free_lines(cutbuffer);
    cutbuffer = was_cutbuffer;
}
pub unsafe extern "C" fn inject(mut burst: *mut libc::c_char, mut count: size_t) {
    let mut thisline: *mut linestruct = (*openfile).current;
    let mut datalen: size_t = strlen((*thisline).data);
    let mut original_row: size_t = 0 as libc::c_int as size_t;
    let mut old_amount: size_t = 0 as libc::c_int as size_t;
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
        if (*openfile).current_y == (editwinrows - 1 as libc::c_int) as libc::c_long {
            original_row = chunk_for(xplustabs(), thisline);
        }
        old_amount = extra_chunks_in(thisline);
    }
    let mut index: size_t = 0 as libc::c_int as size_t;
    while index < count {
        if *burst.offset(index as isize) as libc::c_int == '\0' as i32 {
            *burst.offset(index as isize) = '\n' as i32 as libc::c_char;
        }
        index = index.wrapping_add(1);
        index;
    }
    if (*openfile).last_action as libc::c_uint != ADD as libc::c_int as libc::c_uint
        || (*(*openfile).current_undo).tail_lineno != (*thisline).lineno
        || (*(*openfile).current_undo).tail_x != (*openfile).current_x
    {
        add_undo(ADD, 0 as *const libc::c_char);
    }
    (*thisline)
        .data = nrealloc(
        (*thisline).data as *mut libc::c_void,
        datalen.wrapping_add(count).wrapping_add(1 as libc::c_int as libc::c_ulong),
    ) as *mut libc::c_char;
    memmove(
        ((*thisline).data).offset((*openfile).current_x as isize).offset(count as isize)
            as *mut libc::c_void,
        ((*thisline).data).offset((*openfile).current_x as isize) as *const libc::c_void,
        datalen
            .wrapping_sub((*openfile).current_x)
            .wrapping_add(1 as libc::c_int as libc::c_ulong),
    );
    strncpy(((*thisline).data).offset((*openfile).current_x as isize), burst, count);
    if thisline == (*openfile).mark && (*openfile).current_x < (*openfile).mark_x {
        (*openfile)
            .mark_x = ((*openfile).mark_x as libc::c_ulong).wrapping_add(count) as size_t
            as size_t;
    }
    if thisline == (*openfile).edittop
        && (*openfile).firstcolumn > 0 as libc::c_int as libc::c_ulong
    {
        ensure_firstcolumn_is_aligned();
        refresh_needed = 1 as libc::c_int != 0;
    }
    if thisline == (*openfile).filebot
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
        if margin > 0 as libc::c_int {
            refresh_needed = 1 as libc::c_int != 0;
        }
    }
    (*openfile)
        .current_x = ((*openfile).current_x as libc::c_ulong).wrapping_add(count)
        as size_t as size_t;
    (*openfile)
        .totsize = ((*openfile).totsize as libc::c_ulong).wrapping_add(mbstrlen(burst))
        as size_t as size_t;
    set_modified();
    update_undo(ADD);
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
        do_wrap();
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
        && (extra_chunks_in((*openfile).current) != old_amount
            || (*openfile).current_y == (editwinrows - 1 as libc::c_int) as libc::c_long
                && chunk_for(xplustabs(), (*openfile).current) > original_row)
    {
        refresh_needed = 1 as libc::c_int != 0;
        focusing = 0 as libc::c_int != 0;
    }
    (*openfile).placewewant = xplustabs();
    if !refresh_needed {
        check_the_multis((*openfile).current);
    }
    if !refresh_needed {
        update_line((*openfile).current, (*openfile).current_x);
    }
}
pub unsafe extern "C" fn process_a_keystroke() {
    let mut input: libc::c_int = 0;
    static mut puddle: *mut libc::c_char = 0 as *const libc::c_char as *mut libc::c_char;
    static mut capacity: size_t = 12 as libc::c_int as size_t;
    static mut depth: size_t = 0 as libc::c_int as size_t;
    let mut was_mark: *mut linestruct = (*openfile).mark;
    static mut give_a_hint: bool = 1 as libc::c_int != 0;
    let mut shortcut: *const keystruct = 0 as *const keystruct;
    let mut function: functionptrtype = None;
    input = get_kbinput(midwin, 1 as libc::c_int != 0);
    lastmessage = VACUUM;
    if input == -(2 as libc::c_int) {
        return;
    }
    if input == 0o631 as libc::c_int {
        if do_mouse() == 1 as libc::c_int {
            input = get_kbinput(midwin, 0 as libc::c_int != 0);
        } else {
            return
        }
    }
    shortcut = get_shortcut(input);
    function = if !shortcut.is_null() { (*shortcut).func } else { None };
    if function.is_none() {
        if input < 0x20 as libc::c_int || input > 0xff as libc::c_int
            || meta_key as libc::c_int != 0
        {
            unbound_key(input);
        } else if flags[(VIEW_MODE as libc::c_int as libc::c_ulong)
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
            if !((*openfile).mark).is_null() && (*openfile).softmark as libc::c_int != 0
            {
                (*openfile).mark = 0 as *mut linestruct;
                refresh_needed = 1 as libc::c_int != 0;
            }
            if depth.wrapping_add(1 as libc::c_int as libc::c_ulong) == capacity {
                capacity = (2 as libc::c_int as libc::c_ulong).wrapping_mul(capacity);
                puddle = nrealloc(puddle as *mut libc::c_void, capacity)
                    as *mut libc::c_char;
            } else if puddle.is_null() {
                puddle = nmalloc(capacity) as *mut libc::c_char;
            }
            let fresh1 = depth;
            depth = depth.wrapping_add(1);
            *puddle.offset(fresh1 as isize) = input as libc::c_char;
        }
    }
    if depth > 0 as libc::c_int as libc::c_ulong
        && (function.is_some()
            || waiting_keycodes() == 0 as libc::c_int as libc::c_ulong)
    {
        *puddle.offset(depth as isize) = '\0' as i32 as libc::c_char;
        inject(puddle, depth);
        depth = 0 as libc::c_int as size_t;
    }
    if function.is_none() {
        pletion_line = 0 as *mut linestruct;
        keep_cutbuffer = 0 as libc::c_int != 0;
        return;
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
        && changes_something(function) as libc::c_int != 0
    {
        print_view_warning();
        return;
    }
    if input == '\u{8}' as i32 && give_a_hint as libc::c_int != 0
        && (*openfile).current_x == 0 as libc::c_int as libc::c_ulong
        && (*openfile).current == (*openfile).filetop
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
    {
        statusbar(
            dcgettext(
                0 as *const libc::c_char,
                b"^W = Ctrl+W    M-W = Alt+W\0" as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ),
        );
        give_a_hint = 0 as libc::c_int != 0;
    } else if meta_key {
        give_a_hint = 0 as libc::c_int != 0;
    }
    if function != Some(cut_text as unsafe extern "C" fn() -> ()) {
        if function != Some(copy_text as unsafe extern "C" fn() -> ())
            && function != Some(zap_text as unsafe extern "C" fn() -> ())
        {
            keep_cutbuffer = 0 as libc::c_int != 0;
        }
    }
    if function != Some(complete_a_word as unsafe extern "C" fn() -> ()) {
        pletion_line = 0 as *mut linestruct;
    }
    if function
        == ::std::mem::transmute::<
            Option::<unsafe extern "C" fn(*const libc::c_char) -> ()>,
            functionptrtype,
        >(Some(implant as unsafe extern "C" fn(*const libc::c_char) -> ()))
    {
        implant((*shortcut).expansion);
        return;
    }
    if function == Some(do_toggle as unsafe extern "C" fn() -> ()) {
        toggle_this((*shortcut).toggle);
        if (*shortcut).toggle == CUT_FROM_CURSOR as libc::c_int {
            keep_cutbuffer = 0 as libc::c_int != 0;
        }
        return;
    }
    let mut was_current: *mut linestruct = (*openfile).current;
    let mut was_x: size_t = (*openfile).current_x;
    if shift_held as libc::c_int != 0 && ((*openfile).mark).is_null() {
        (*openfile).mark = (*openfile).current;
        (*openfile).mark_x = (*openfile).current_x;
        (*openfile).softmark = 1 as libc::c_int != 0;
    }
    function.unwrap()();
    if !((*openfile).mark).is_null() && (*openfile).softmark as libc::c_int != 0
        && !shift_held
        && ((*openfile).current != was_current || (*openfile).current_x != was_x
            || wanted_to_move(function) as libc::c_int != 0)
    {
        (*openfile).mark = 0 as *mut linestruct;
        refresh_needed = 1 as libc::c_int != 0;
    } else if (*openfile).current != was_current {
        also_the_last = 0 as libc::c_int != 0;
    }
    if bracketed_paste {
        suck_up_input_and_paste_it();
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
                ) != 0 as libc::c_int as libc::c_uint && (*openfile).mark != was_mark
    {
        titlebar(0 as *const libc::c_char);
    }
}
unsafe fn main_0(
    mut argc: libc::c_int,
    mut argv: *mut *mut libc::c_char,
) -> libc::c_int {
    let mut stdin_flags: libc::c_int = 0;
    let mut optchr: libc::c_int = 0;
    let mut ignore_rcfiles: bool = 0 as libc::c_int != 0;
    let mut fill_used: bool = 0 as libc::c_int != 0;
    let mut hardwrap: libc::c_int = -(2 as libc::c_int);
    let mut quoterc: libc::c_int = 0;
    let long_options: [option; 55] = [
        {
            let mut init = option {
                name: b"boldtext\0" as *const u8 as *const libc::c_char,
                has_arg: 0 as libc::c_int,
                flag: 0 as *mut libc::c_int,
                val: 'D' as i32,
            };
            init
        },
        {
            let mut init = option {
                name: b"multibuffer\0" as *const u8 as *const libc::c_char,
                has_arg: 0 as libc::c_int,
                flag: 0 as *mut libc::c_int,
                val: 'F' as i32,
            };
            init
        },
        {
            let mut init = option {
                name: b"ignorercfiles\0" as *const u8 as *const libc::c_char,
                has_arg: 0 as libc::c_int,
                flag: 0 as *mut libc::c_int,
                val: 'I' as i32,
            };
            init
        },
        {
            let mut init = option {
                name: b"rawsequences\0" as *const u8 as *const libc::c_char,
                has_arg: 0 as libc::c_int,
                flag: 0 as *mut libc::c_int,
                val: 'K' as i32,
            };
            init
        },
        {
            let mut init = option {
                name: b"trimblanks\0" as *const u8 as *const libc::c_char,
                has_arg: 0 as libc::c_int,
                flag: 0 as *mut libc::c_int,
                val: 'M' as i32,
            };
            init
        },
        {
            let mut init = option {
                name: b"quotestr\0" as *const u8 as *const libc::c_char,
                has_arg: 1 as libc::c_int,
                flag: 0 as *mut libc::c_int,
                val: 'Q' as i32,
            };
            init
        },
        {
            let mut init = option {
                name: b"restricted\0" as *const u8 as *const libc::c_char,
                has_arg: 0 as libc::c_int,
                flag: 0 as *mut libc::c_int,
                val: 'R' as i32,
            };
            init
        },
        {
            let mut init = option {
                name: b"quickblank\0" as *const u8 as *const libc::c_char,
                has_arg: 0 as libc::c_int,
                flag: 0 as *mut libc::c_int,
                val: 'U' as i32,
            };
            init
        },
        {
            let mut init = option {
                name: b"version\0" as *const u8 as *const libc::c_char,
                has_arg: 0 as libc::c_int,
                flag: 0 as *mut libc::c_int,
                val: 'V' as i32,
            };
            init
        },
        {
            let mut init = option {
                name: b"syntax\0" as *const u8 as *const libc::c_char,
                has_arg: 1 as libc::c_int,
                flag: 0 as *mut libc::c_int,
                val: 'Y' as i32,
            };
            init
        },
        {
            let mut init = option {
                name: b"breaklonglines\0" as *const u8 as *const libc::c_char,
                has_arg: 0 as libc::c_int,
                flag: 0 as *mut libc::c_int,
                val: 'b' as i32,
            };
            init
        },
        {
            let mut init = option {
                name: b"constantshow\0" as *const u8 as *const libc::c_char,
                has_arg: 0 as libc::c_int,
                flag: 0 as *mut libc::c_int,
                val: 'c' as i32,
            };
            init
        },
        {
            let mut init = option {
                name: b"rebinddelete\0" as *const u8 as *const libc::c_char,
                has_arg: 0 as libc::c_int,
                flag: 0 as *mut libc::c_int,
                val: 'd' as i32,
            };
            init
        },
        {
            let mut init = option {
                name: b"rcfile\0" as *const u8 as *const libc::c_char,
                has_arg: 1 as libc::c_int,
                flag: 0 as *mut libc::c_int,
                val: 'f' as i32,
            };
            init
        },
        {
            let mut init = option {
                name: b"showcursor\0" as *const u8 as *const libc::c_char,
                has_arg: 0 as libc::c_int,
                flag: 0 as *mut libc::c_int,
                val: 'g' as i32,
            };
            init
        },
        {
            let mut init = option {
                name: b"help\0" as *const u8 as *const libc::c_char,
                has_arg: 0 as libc::c_int,
                flag: 0 as *mut libc::c_int,
                val: 'h' as i32,
            };
            init
        },
        {
            let mut init = option {
                name: b"linenumbers\0" as *const u8 as *const libc::c_char,
                has_arg: 0 as libc::c_int,
                flag: 0 as *mut libc::c_int,
                val: 'l' as i32,
            };
            init
        },
        {
            let mut init = option {
                name: b"mouse\0" as *const u8 as *const libc::c_char,
                has_arg: 0 as libc::c_int,
                flag: 0 as *mut libc::c_int,
                val: 'm' as i32,
            };
            init
        },
        {
            let mut init = option {
                name: b"operatingdir\0" as *const u8 as *const libc::c_char,
                has_arg: 1 as libc::c_int,
                flag: 0 as *mut libc::c_int,
                val: 'o' as i32,
            };
            init
        },
        {
            let mut init = option {
                name: b"preserve\0" as *const u8 as *const libc::c_char,
                has_arg: 0 as libc::c_int,
                flag: 0 as *mut libc::c_int,
                val: 'p' as i32,
            };
            init
        },
        {
            let mut init = option {
                name: b"fill\0" as *const u8 as *const libc::c_char,
                has_arg: 1 as libc::c_int,
                flag: 0 as *mut libc::c_int,
                val: 'r' as i32,
            };
            init
        },
        {
            let mut init = option {
                name: b"speller\0" as *const u8 as *const libc::c_char,
                has_arg: 1 as libc::c_int,
                flag: 0 as *mut libc::c_int,
                val: 's' as i32,
            };
            init
        },
        {
            let mut init = option {
                name: b"saveonexit\0" as *const u8 as *const libc::c_char,
                has_arg: 0 as libc::c_int,
                flag: 0 as *mut libc::c_int,
                val: 't' as i32,
            };
            init
        },
        {
            let mut init = option {
                name: b"view\0" as *const u8 as *const libc::c_char,
                has_arg: 0 as libc::c_int,
                flag: 0 as *mut libc::c_int,
                val: 'v' as i32,
            };
            init
        },
        {
            let mut init = option {
                name: b"nowrap\0" as *const u8 as *const libc::c_char,
                has_arg: 0 as libc::c_int,
                flag: 0 as *mut libc::c_int,
                val: 'w' as i32,
            };
            init
        },
        {
            let mut init = option {
                name: b"nohelp\0" as *const u8 as *const libc::c_char,
                has_arg: 0 as libc::c_int,
                flag: 0 as *mut libc::c_int,
                val: 'x' as i32,
            };
            init
        },
        {
            let mut init = option {
                name: b"smarthome\0" as *const u8 as *const libc::c_char,
                has_arg: 0 as libc::c_int,
                flag: 0 as *mut libc::c_int,
                val: 'A' as i32,
            };
            init
        },
        {
            let mut init = option {
                name: b"backup\0" as *const u8 as *const libc::c_char,
                has_arg: 0 as libc::c_int,
                flag: 0 as *mut libc::c_int,
                val: 'B' as i32,
            };
            init
        },
        {
            let mut init = option {
                name: b"backupdir\0" as *const u8 as *const libc::c_char,
                has_arg: 1 as libc::c_int,
                flag: 0 as *mut libc::c_int,
                val: 'C' as i32,
            };
            init
        },
        {
            let mut init = option {
                name: b"tabstospaces\0" as *const u8 as *const libc::c_char,
                has_arg: 0 as libc::c_int,
                flag: 0 as *mut libc::c_int,
                val: 'E' as i32,
            };
            init
        },
        {
            let mut init = option {
                name: b"locking\0" as *const u8 as *const libc::c_char,
                has_arg: 0 as libc::c_int,
                flag: 0 as *mut libc::c_int,
                val: 'G' as i32,
            };
            init
        },
        {
            let mut init = option {
                name: b"historylog\0" as *const u8 as *const libc::c_char,
                has_arg: 0 as libc::c_int,
                flag: 0 as *mut libc::c_int,
                val: 'H' as i32,
            };
            init
        },
        {
            let mut init = option {
                name: b"guidestripe\0" as *const u8 as *const libc::c_char,
                has_arg: 1 as libc::c_int,
                flag: 0 as *mut libc::c_int,
                val: 'J' as i32,
            };
            init
        },
        {
            let mut init = option {
                name: b"nonewlines\0" as *const u8 as *const libc::c_char,
                has_arg: 0 as libc::c_int,
                flag: 0 as *mut libc::c_int,
                val: 'L' as i32,
            };
            init
        },
        {
            let mut init = option {
                name: b"noconvert\0" as *const u8 as *const libc::c_char,
                has_arg: 0 as libc::c_int,
                flag: 0 as *mut libc::c_int,
                val: 'N' as i32,
            };
            init
        },
        {
            let mut init = option {
                name: b"bookstyle\0" as *const u8 as *const libc::c_char,
                has_arg: 0 as libc::c_int,
                flag: 0 as *mut libc::c_int,
                val: 'O' as i32,
            };
            init
        },
        {
            let mut init = option {
                name: b"positionlog\0" as *const u8 as *const libc::c_char,
                has_arg: 0 as libc::c_int,
                flag: 0 as *mut libc::c_int,
                val: 'P' as i32,
            };
            init
        },
        {
            let mut init = option {
                name: b"softwrap\0" as *const u8 as *const libc::c_char,
                has_arg: 0 as libc::c_int,
                flag: 0 as *mut libc::c_int,
                val: 'S' as i32,
            };
            init
        },
        {
            let mut init = option {
                name: b"tabsize\0" as *const u8 as *const libc::c_char,
                has_arg: 1 as libc::c_int,
                flag: 0 as *mut libc::c_int,
                val: 'T' as i32,
            };
            init
        },
        {
            let mut init = option {
                name: b"wordbounds\0" as *const u8 as *const libc::c_char,
                has_arg: 0 as libc::c_int,
                flag: 0 as *mut libc::c_int,
                val: 'W' as i32,
            };
            init
        },
        {
            let mut init = option {
                name: b"wordchars\0" as *const u8 as *const libc::c_char,
                has_arg: 1 as libc::c_int,
                flag: 0 as *mut libc::c_int,
                val: 'X' as i32,
            };
            init
        },
        {
            let mut init = option {
                name: b"zap\0" as *const u8 as *const libc::c_char,
                has_arg: 0 as libc::c_int,
                flag: 0 as *mut libc::c_int,
                val: 'Z' as i32,
            };
            init
        },
        {
            let mut init = option {
                name: b"atblanks\0" as *const u8 as *const libc::c_char,
                has_arg: 0 as libc::c_int,
                flag: 0 as *mut libc::c_int,
                val: 'a' as i32,
            };
            init
        },
        {
            let mut init = option {
                name: b"emptyline\0" as *const u8 as *const libc::c_char,
                has_arg: 0 as libc::c_int,
                flag: 0 as *mut libc::c_int,
                val: 'e' as i32,
            };
            init
        },
        {
            let mut init = option {
                name: b"autoindent\0" as *const u8 as *const libc::c_char,
                has_arg: 0 as libc::c_int,
                flag: 0 as *mut libc::c_int,
                val: 'i' as i32,
            };
            init
        },
        {
            let mut init = option {
                name: b"jumpyscrolling\0" as *const u8 as *const libc::c_char,
                has_arg: 0 as libc::c_int,
                flag: 0 as *mut libc::c_int,
                val: 'j' as i32,
            };
            init
        },
        {
            let mut init = option {
                name: b"cutfromcursor\0" as *const u8 as *const libc::c_char,
                has_arg: 0 as libc::c_int,
                flag: 0 as *mut libc::c_int,
                val: 'k' as i32,
            };
            init
        },
        {
            let mut init = option {
                name: b"noread\0" as *const u8 as *const libc::c_char,
                has_arg: 0 as libc::c_int,
                flag: 0 as *mut libc::c_int,
                val: 'n' as i32,
            };
            init
        },
        {
            let mut init = option {
                name: b"indicator\0" as *const u8 as *const libc::c_char,
                has_arg: 0 as libc::c_int,
                flag: 0 as *mut libc::c_int,
                val: 'q' as i32,
            };
            init
        },
        {
            let mut init = option {
                name: b"unix\0" as *const u8 as *const libc::c_char,
                has_arg: 0 as libc::c_int,
                flag: 0 as *mut libc::c_int,
                val: 'u' as i32,
            };
            init
        },
        {
            let mut init = option {
                name: b"afterends\0" as *const u8 as *const libc::c_char,
                has_arg: 0 as libc::c_int,
                flag: 0 as *mut libc::c_int,
                val: 'y' as i32,
            };
            init
        },
        {
            let mut init = option {
                name: b"stateflags\0" as *const u8 as *const libc::c_char,
                has_arg: 0 as libc::c_int,
                flag: 0 as *mut libc::c_int,
                val: '%' as i32,
            };
            init
        },
        {
            let mut init = option {
                name: b"minibar\0" as *const u8 as *const libc::c_char,
                has_arg: 0 as libc::c_int,
                flag: 0 as *mut libc::c_int,
                val: '_' as i32,
            };
            init
        },
        {
            let mut init = option {
                name: b"zero\0" as *const u8 as *const libc::c_char,
                has_arg: 0 as libc::c_int,
                flag: 0 as *mut libc::c_int,
                val: '0' as i32,
            };
            init
        },
        {
            let mut init = option {
                name: 0 as *const libc::c_char,
                has_arg: 0 as libc::c_int,
                flag: 0 as *mut libc::c_int,
                val: 0 as libc::c_int,
            };
            init
        },
    ];
    let mut dummy: vt_stat = vt_stat {
        v_active: 0,
        v_signal: 0,
        v_state: 0,
    };
    on_a_vt = ioctl(
        1 as libc::c_int,
        0x5603 as libc::c_int as libc::c_ulong,
        &mut dummy as *mut vt_stat,
    ) == 0 as libc::c_int;
    tcgetattr(0 as libc::c_int, &mut original_state);
    stdin_flags = rpl_fcntl(0 as libc::c_int, 3 as libc::c_int, 0 as libc::c_int);
    if stdin_flags != -(1 as libc::c_int) {
        rpl_fcntl(
            0 as libc::c_int,
            4 as libc::c_int,
            stdin_flags & !(0o4000 as libc::c_int),
        );
    }
    if !(setlocale(6 as libc::c_int, b"\0" as *const u8 as *const libc::c_char))
        .is_null()
        && strcmp(
            nl_langinfo(CODESET as libc::c_int),
            b"UTF-8\0" as *const u8 as *const libc::c_char,
        ) == 0 as libc::c_int
    {
        utf8_init();
    }
    bindtextdomain(
        b"nano\0" as *const u8 as *const libc::c_char,
        b"/usr/local/share/locale\0" as *const u8 as *const libc::c_char,
    );
    textdomain(b"nano\0" as *const u8 as *const libc::c_char);
    flags[(NO_WRAP as libc::c_int as libc::c_ulong)
        .wrapping_div(
            (::std::mem::size_of::<libc::c_uint>() as libc::c_ulong)
                .wrapping_mul(8 as libc::c_int as libc::c_ulong),
        ) as usize]
        |= (1 as libc::c_int as libc::c_uint)
            << (NO_WRAP as libc::c_int as libc::c_ulong)
                .wrapping_rem(
                    (::std::mem::size_of::<libc::c_uint>() as libc::c_ulong)
                        .wrapping_mul(8 as libc::c_int as libc::c_ulong),
                );
    if *tail(*argv.offset(0 as libc::c_int as isize)) as libc::c_int == 'r' as i32 {
        flags[(RESTRICTED as libc::c_int as libc::c_ulong)
            .wrapping_div(
                (::std::mem::size_of::<libc::c_uint>() as libc::c_ulong)
                    .wrapping_mul(8 as libc::c_int as libc::c_ulong),
            ) as usize]
            |= (1 as libc::c_int as libc::c_uint)
                << (RESTRICTED as libc::c_int as libc::c_ulong)
                    .wrapping_rem(
                        (::std::mem::size_of::<libc::c_uint>() as libc::c_ulong)
                            .wrapping_mul(8 as libc::c_int as libc::c_ulong),
                    );
    }
    loop {
        optchr = getopt_long(
            argc,
            argv,
            b"ABC:DEFGHIJ:KLMNOPQ:RS$T:UVWX:Y:Zabcdef:ghijklmno:pqr:s:tuvwxy!%_0\0"
                as *const u8 as *const libc::c_char,
            long_options.as_ptr(),
            0 as *mut libc::c_int,
        );
        if !(optchr != -(1 as libc::c_int)) {
            break;
        }
        match optchr {
            65 => {
                flags[(SMART_HOME as libc::c_int as libc::c_ulong)
                    .wrapping_div(
                        (::std::mem::size_of::<libc::c_uint>() as libc::c_ulong)
                            .wrapping_mul(8 as libc::c_int as libc::c_ulong),
                    ) as usize]
                    |= (1 as libc::c_int as libc::c_uint)
                        << (SMART_HOME as libc::c_int as libc::c_ulong)
                            .wrapping_rem(
                                (::std::mem::size_of::<libc::c_uint>() as libc::c_ulong)
                                    .wrapping_mul(8 as libc::c_int as libc::c_ulong),
                            );
            }
            66 => {
                flags[(MAKE_BACKUP as libc::c_int as libc::c_ulong)
                    .wrapping_div(
                        (::std::mem::size_of::<libc::c_uint>() as libc::c_ulong)
                            .wrapping_mul(8 as libc::c_int as libc::c_ulong),
                    ) as usize]
                    |= (1 as libc::c_int as libc::c_uint)
                        << (MAKE_BACKUP as libc::c_int as libc::c_ulong)
                            .wrapping_rem(
                                (::std::mem::size_of::<libc::c_uint>() as libc::c_ulong)
                                    .wrapping_mul(8 as libc::c_int as libc::c_ulong),
                            );
            }
            67 => {
                backup_dir = mallocstrcpy(backup_dir, optarg);
            }
            68 => {
                flags[(BOLD_TEXT as libc::c_int as libc::c_ulong)
                    .wrapping_div(
                        (::std::mem::size_of::<libc::c_uint>() as libc::c_ulong)
                            .wrapping_mul(8 as libc::c_int as libc::c_ulong),
                    ) as usize]
                    |= (1 as libc::c_int as libc::c_uint)
                        << (BOLD_TEXT as libc::c_int as libc::c_ulong)
                            .wrapping_rem(
                                (::std::mem::size_of::<libc::c_uint>() as libc::c_ulong)
                                    .wrapping_mul(8 as libc::c_int as libc::c_ulong),
                            );
            }
            69 => {
                flags[(TABS_TO_SPACES as libc::c_int as libc::c_ulong)
                    .wrapping_div(
                        (::std::mem::size_of::<libc::c_uint>() as libc::c_ulong)
                            .wrapping_mul(8 as libc::c_int as libc::c_ulong),
                    ) as usize]
                    |= (1 as libc::c_int as libc::c_uint)
                        << (TABS_TO_SPACES as libc::c_int as libc::c_ulong)
                            .wrapping_rem(
                                (::std::mem::size_of::<libc::c_uint>() as libc::c_ulong)
                                    .wrapping_mul(8 as libc::c_int as libc::c_ulong),
                            );
            }
            70 => {
                flags[(MULTIBUFFER as libc::c_int as libc::c_ulong)
                    .wrapping_div(
                        (::std::mem::size_of::<libc::c_uint>() as libc::c_ulong)
                            .wrapping_mul(8 as libc::c_int as libc::c_ulong),
                    ) as usize]
                    |= (1 as libc::c_int as libc::c_uint)
                        << (MULTIBUFFER as libc::c_int as libc::c_ulong)
                            .wrapping_rem(
                                (::std::mem::size_of::<libc::c_uint>() as libc::c_ulong)
                                    .wrapping_mul(8 as libc::c_int as libc::c_ulong),
                            );
            }
            71 => {
                flags[(LOCKING as libc::c_int as libc::c_ulong)
                    .wrapping_div(
                        (::std::mem::size_of::<libc::c_uint>() as libc::c_ulong)
                            .wrapping_mul(8 as libc::c_int as libc::c_ulong),
                    ) as usize]
                    |= (1 as libc::c_int as libc::c_uint)
                        << (LOCKING as libc::c_int as libc::c_ulong)
                            .wrapping_rem(
                                (::std::mem::size_of::<libc::c_uint>() as libc::c_ulong)
                                    .wrapping_mul(8 as libc::c_int as libc::c_ulong),
                            );
            }
            72 => {
                flags[(HISTORYLOG as libc::c_int as libc::c_ulong)
                    .wrapping_div(
                        (::std::mem::size_of::<libc::c_uint>() as libc::c_ulong)
                            .wrapping_mul(8 as libc::c_int as libc::c_ulong),
                    ) as usize]
                    |= (1 as libc::c_int as libc::c_uint)
                        << (HISTORYLOG as libc::c_int as libc::c_ulong)
                            .wrapping_rem(
                                (::std::mem::size_of::<libc::c_uint>() as libc::c_ulong)
                                    .wrapping_mul(8 as libc::c_int as libc::c_ulong),
                            );
            }
            73 => {
                ignore_rcfiles = 1 as libc::c_int != 0;
            }
            74 => {
                if !parse_num(optarg, &mut stripe_column)
                    || stripe_column <= 0 as libc::c_int as libc::c_long
                {
                    fprintf(
                        stderr,
                        dcgettext(
                            0 as *const libc::c_char,
                            b"Guide column \"%s\" is invalid\0" as *const u8
                                as *const libc::c_char,
                            5 as libc::c_int,
                        ),
                        optarg,
                    );
                    fprintf(stderr, b"\n\0" as *const u8 as *const libc::c_char);
                    exit(1 as libc::c_int);
                }
            }
            75 => {
                flags[(RAW_SEQUENCES as libc::c_int as libc::c_ulong)
                    .wrapping_div(
                        (::std::mem::size_of::<libc::c_uint>() as libc::c_ulong)
                            .wrapping_mul(8 as libc::c_int as libc::c_ulong),
                    ) as usize]
                    |= (1 as libc::c_int as libc::c_uint)
                        << (RAW_SEQUENCES as libc::c_int as libc::c_ulong)
                            .wrapping_rem(
                                (::std::mem::size_of::<libc::c_uint>() as libc::c_ulong)
                                    .wrapping_mul(8 as libc::c_int as libc::c_ulong),
                            );
            }
            76 => {
                flags[(NO_NEWLINES as libc::c_int as libc::c_ulong)
                    .wrapping_div(
                        (::std::mem::size_of::<libc::c_uint>() as libc::c_ulong)
                            .wrapping_mul(8 as libc::c_int as libc::c_ulong),
                    ) as usize]
                    |= (1 as libc::c_int as libc::c_uint)
                        << (NO_NEWLINES as libc::c_int as libc::c_ulong)
                            .wrapping_rem(
                                (::std::mem::size_of::<libc::c_uint>() as libc::c_ulong)
                                    .wrapping_mul(8 as libc::c_int as libc::c_ulong),
                            );
            }
            77 => {
                flags[(TRIM_BLANKS as libc::c_int as libc::c_ulong)
                    .wrapping_div(
                        (::std::mem::size_of::<libc::c_uint>() as libc::c_ulong)
                            .wrapping_mul(8 as libc::c_int as libc::c_ulong),
                    ) as usize]
                    |= (1 as libc::c_int as libc::c_uint)
                        << (TRIM_BLANKS as libc::c_int as libc::c_ulong)
                            .wrapping_rem(
                                (::std::mem::size_of::<libc::c_uint>() as libc::c_ulong)
                                    .wrapping_mul(8 as libc::c_int as libc::c_ulong),
                            );
            }
            78 => {
                flags[(NO_CONVERT as libc::c_int as libc::c_ulong)
                    .wrapping_div(
                        (::std::mem::size_of::<libc::c_uint>() as libc::c_ulong)
                            .wrapping_mul(8 as libc::c_int as libc::c_ulong),
                    ) as usize]
                    |= (1 as libc::c_int as libc::c_uint)
                        << (NO_CONVERT as libc::c_int as libc::c_ulong)
                            .wrapping_rem(
                                (::std::mem::size_of::<libc::c_uint>() as libc::c_ulong)
                                    .wrapping_mul(8 as libc::c_int as libc::c_ulong),
                            );
            }
            79 => {
                flags[(BOOKSTYLE as libc::c_int as libc::c_ulong)
                    .wrapping_div(
                        (::std::mem::size_of::<libc::c_uint>() as libc::c_ulong)
                            .wrapping_mul(8 as libc::c_int as libc::c_ulong),
                    ) as usize]
                    |= (1 as libc::c_int as libc::c_uint)
                        << (BOOKSTYLE as libc::c_int as libc::c_ulong)
                            .wrapping_rem(
                                (::std::mem::size_of::<libc::c_uint>() as libc::c_ulong)
                                    .wrapping_mul(8 as libc::c_int as libc::c_ulong),
                            );
            }
            80 => {
                flags[(POSITIONLOG as libc::c_int as libc::c_ulong)
                    .wrapping_div(
                        (::std::mem::size_of::<libc::c_uint>() as libc::c_ulong)
                            .wrapping_mul(8 as libc::c_int as libc::c_ulong),
                    ) as usize]
                    |= (1 as libc::c_int as libc::c_uint)
                        << (POSITIONLOG as libc::c_int as libc::c_ulong)
                            .wrapping_rem(
                                (::std::mem::size_of::<libc::c_uint>() as libc::c_ulong)
                                    .wrapping_mul(8 as libc::c_int as libc::c_ulong),
                            );
            }
            81 => {
                quotestr = mallocstrcpy(quotestr, optarg);
            }
            82 => {
                flags[(RESTRICTED as libc::c_int as libc::c_ulong)
                    .wrapping_div(
                        (::std::mem::size_of::<libc::c_uint>() as libc::c_ulong)
                            .wrapping_mul(8 as libc::c_int as libc::c_ulong),
                    ) as usize]
                    |= (1 as libc::c_int as libc::c_uint)
                        << (RESTRICTED as libc::c_int as libc::c_ulong)
                            .wrapping_rem(
                                (::std::mem::size_of::<libc::c_uint>() as libc::c_ulong)
                                    .wrapping_mul(8 as libc::c_int as libc::c_ulong),
                            );
            }
            83 | 36 => {
                flags[(SOFTWRAP as libc::c_int as libc::c_ulong)
                    .wrapping_div(
                        (::std::mem::size_of::<libc::c_uint>() as libc::c_ulong)
                            .wrapping_mul(8 as libc::c_int as libc::c_ulong),
                    ) as usize]
                    |= (1 as libc::c_int as libc::c_uint)
                        << (SOFTWRAP as libc::c_int as libc::c_ulong)
                            .wrapping_rem(
                                (::std::mem::size_of::<libc::c_uint>() as libc::c_ulong)
                                    .wrapping_mul(8 as libc::c_int as libc::c_ulong),
                            );
            }
            84 => {
                if !parse_num(optarg, &mut tabsize)
                    || tabsize <= 0 as libc::c_int as libc::c_long
                {
                    fprintf(
                        stderr,
                        dcgettext(
                            0 as *const libc::c_char,
                            b"Requested tab size \"%s\" is invalid\0" as *const u8
                                as *const libc::c_char,
                            5 as libc::c_int,
                        ),
                        optarg,
                    );
                    fprintf(stderr, b"\n\0" as *const u8 as *const libc::c_char);
                    exit(1 as libc::c_int);
                }
            }
            85 => {
                flags[(QUICK_BLANK as libc::c_int as libc::c_ulong)
                    .wrapping_div(
                        (::std::mem::size_of::<libc::c_uint>() as libc::c_ulong)
                            .wrapping_mul(8 as libc::c_int as libc::c_ulong),
                    ) as usize]
                    |= (1 as libc::c_int as libc::c_uint)
                        << (QUICK_BLANK as libc::c_int as libc::c_ulong)
                            .wrapping_rem(
                                (::std::mem::size_of::<libc::c_uint>() as libc::c_ulong)
                                    .wrapping_mul(8 as libc::c_int as libc::c_ulong),
                            );
            }
            86 => {
                version();
                exit(0 as libc::c_int);
            }
            87 => {
                flags[(WORD_BOUNDS as libc::c_int as libc::c_ulong)
                    .wrapping_div(
                        (::std::mem::size_of::<libc::c_uint>() as libc::c_ulong)
                            .wrapping_mul(8 as libc::c_int as libc::c_ulong),
                    ) as usize]
                    |= (1 as libc::c_int as libc::c_uint)
                        << (WORD_BOUNDS as libc::c_int as libc::c_ulong)
                            .wrapping_rem(
                                (::std::mem::size_of::<libc::c_uint>() as libc::c_ulong)
                                    .wrapping_mul(8 as libc::c_int as libc::c_ulong),
                            );
            }
            88 => {
                word_chars = mallocstrcpy(word_chars, optarg);
            }
            89 => {
                syntaxstr = mallocstrcpy(syntaxstr, optarg);
            }
            90 => {
                flags[(LET_THEM_ZAP as libc::c_int as libc::c_ulong)
                    .wrapping_div(
                        (::std::mem::size_of::<libc::c_uint>() as libc::c_ulong)
                            .wrapping_mul(8 as libc::c_int as libc::c_ulong),
                    ) as usize]
                    |= (1 as libc::c_int as libc::c_uint)
                        << (LET_THEM_ZAP as libc::c_int as libc::c_ulong)
                            .wrapping_rem(
                                (::std::mem::size_of::<libc::c_uint>() as libc::c_ulong)
                                    .wrapping_mul(8 as libc::c_int as libc::c_ulong),
                            );
            }
            97 => {
                flags[(AT_BLANKS as libc::c_int as libc::c_ulong)
                    .wrapping_div(
                        (::std::mem::size_of::<libc::c_uint>() as libc::c_ulong)
                            .wrapping_mul(8 as libc::c_int as libc::c_ulong),
                    ) as usize]
                    |= (1 as libc::c_int as libc::c_uint)
                        << (AT_BLANKS as libc::c_int as libc::c_ulong)
                            .wrapping_rem(
                                (::std::mem::size_of::<libc::c_uint>() as libc::c_ulong)
                                    .wrapping_mul(8 as libc::c_int as libc::c_ulong),
                            );
            }
            98 => {
                hardwrap = 1 as libc::c_int;
            }
            99 => {
                flags[(CONSTANT_SHOW as libc::c_int as libc::c_ulong)
                    .wrapping_div(
                        (::std::mem::size_of::<libc::c_uint>() as libc::c_ulong)
                            .wrapping_mul(8 as libc::c_int as libc::c_ulong),
                    ) as usize]
                    |= (1 as libc::c_int as libc::c_uint)
                        << (CONSTANT_SHOW as libc::c_int as libc::c_ulong)
                            .wrapping_rem(
                                (::std::mem::size_of::<libc::c_uint>() as libc::c_ulong)
                                    .wrapping_mul(8 as libc::c_int as libc::c_ulong),
                            );
            }
            100 => {
                flags[(REBIND_DELETE as libc::c_int as libc::c_ulong)
                    .wrapping_div(
                        (::std::mem::size_of::<libc::c_uint>() as libc::c_ulong)
                            .wrapping_mul(8 as libc::c_int as libc::c_ulong),
                    ) as usize]
                    |= (1 as libc::c_int as libc::c_uint)
                        << (REBIND_DELETE as libc::c_int as libc::c_ulong)
                            .wrapping_rem(
                                (::std::mem::size_of::<libc::c_uint>() as libc::c_ulong)
                                    .wrapping_mul(8 as libc::c_int as libc::c_ulong),
                            );
            }
            101 => {
                flags[(EMPTY_LINE as libc::c_int as libc::c_ulong)
                    .wrapping_div(
                        (::std::mem::size_of::<libc::c_uint>() as libc::c_ulong)
                            .wrapping_mul(8 as libc::c_int as libc::c_ulong),
                    ) as usize]
                    |= (1 as libc::c_int as libc::c_uint)
                        << (EMPTY_LINE as libc::c_int as libc::c_ulong)
                            .wrapping_rem(
                                (::std::mem::size_of::<libc::c_uint>() as libc::c_ulong)
                                    .wrapping_mul(8 as libc::c_int as libc::c_ulong),
                            );
            }
            102 => {
                custom_nanorc = mallocstrcpy(custom_nanorc, optarg);
            }
            103 => {
                flags[(SHOW_CURSOR as libc::c_int as libc::c_ulong)
                    .wrapping_div(
                        (::std::mem::size_of::<libc::c_uint>() as libc::c_ulong)
                            .wrapping_mul(8 as libc::c_int as libc::c_ulong),
                    ) as usize]
                    |= (1 as libc::c_int as libc::c_uint)
                        << (SHOW_CURSOR as libc::c_int as libc::c_ulong)
                            .wrapping_rem(
                                (::std::mem::size_of::<libc::c_uint>() as libc::c_ulong)
                                    .wrapping_mul(8 as libc::c_int as libc::c_ulong),
                            );
            }
            104 => {
                usage();
                exit(0 as libc::c_int);
            }
            105 => {
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
            106 => {
                flags[(JUMPY_SCROLLING as libc::c_int as libc::c_ulong)
                    .wrapping_div(
                        (::std::mem::size_of::<libc::c_uint>() as libc::c_ulong)
                            .wrapping_mul(8 as libc::c_int as libc::c_ulong),
                    ) as usize]
                    |= (1 as libc::c_int as libc::c_uint)
                        << (JUMPY_SCROLLING as libc::c_int as libc::c_ulong)
                            .wrapping_rem(
                                (::std::mem::size_of::<libc::c_uint>() as libc::c_ulong)
                                    .wrapping_mul(8 as libc::c_int as libc::c_ulong),
                            );
            }
            107 => {
                flags[(CUT_FROM_CURSOR as libc::c_int as libc::c_ulong)
                    .wrapping_div(
                        (::std::mem::size_of::<libc::c_uint>() as libc::c_ulong)
                            .wrapping_mul(8 as libc::c_int as libc::c_ulong),
                    ) as usize]
                    |= (1 as libc::c_int as libc::c_uint)
                        << (CUT_FROM_CURSOR as libc::c_int as libc::c_ulong)
                            .wrapping_rem(
                                (::std::mem::size_of::<libc::c_uint>() as libc::c_ulong)
                                    .wrapping_mul(8 as libc::c_int as libc::c_ulong),
                            );
            }
            108 => {
                flags[(LINE_NUMBERS as libc::c_int as libc::c_ulong)
                    .wrapping_div(
                        (::std::mem::size_of::<libc::c_uint>() as libc::c_ulong)
                            .wrapping_mul(8 as libc::c_int as libc::c_ulong),
                    ) as usize]
                    |= (1 as libc::c_int as libc::c_uint)
                        << (LINE_NUMBERS as libc::c_int as libc::c_ulong)
                            .wrapping_rem(
                                (::std::mem::size_of::<libc::c_uint>() as libc::c_ulong)
                                    .wrapping_mul(8 as libc::c_int as libc::c_ulong),
                            );
            }
            109 => {
                flags[(USE_MOUSE as libc::c_int as libc::c_ulong)
                    .wrapping_div(
                        (::std::mem::size_of::<libc::c_uint>() as libc::c_ulong)
                            .wrapping_mul(8 as libc::c_int as libc::c_ulong),
                    ) as usize]
                    |= (1 as libc::c_int as libc::c_uint)
                        << (USE_MOUSE as libc::c_int as libc::c_ulong)
                            .wrapping_rem(
                                (::std::mem::size_of::<libc::c_uint>() as libc::c_ulong)
                                    .wrapping_mul(8 as libc::c_int as libc::c_ulong),
                            );
            }
            110 => {
                flags[(NOREAD_MODE as libc::c_int as libc::c_ulong)
                    .wrapping_div(
                        (::std::mem::size_of::<libc::c_uint>() as libc::c_ulong)
                            .wrapping_mul(8 as libc::c_int as libc::c_ulong),
                    ) as usize]
                    |= (1 as libc::c_int as libc::c_uint)
                        << (NOREAD_MODE as libc::c_int as libc::c_ulong)
                            .wrapping_rem(
                                (::std::mem::size_of::<libc::c_uint>() as libc::c_ulong)
                                    .wrapping_mul(8 as libc::c_int as libc::c_ulong),
                            );
            }
            111 => {
                operating_dir = mallocstrcpy(operating_dir, optarg);
            }
            112 => {
                flags[(PRESERVE as libc::c_int as libc::c_ulong)
                    .wrapping_div(
                        (::std::mem::size_of::<libc::c_uint>() as libc::c_ulong)
                            .wrapping_mul(8 as libc::c_int as libc::c_ulong),
                    ) as usize]
                    |= (1 as libc::c_int as libc::c_uint)
                        << (PRESERVE as libc::c_int as libc::c_ulong)
                            .wrapping_rem(
                                (::std::mem::size_of::<libc::c_uint>() as libc::c_ulong)
                                    .wrapping_mul(8 as libc::c_int as libc::c_ulong),
                            );
            }
            113 => {
                flags[(INDICATOR as libc::c_int as libc::c_ulong)
                    .wrapping_div(
                        (::std::mem::size_of::<libc::c_uint>() as libc::c_ulong)
                            .wrapping_mul(8 as libc::c_int as libc::c_ulong),
                    ) as usize]
                    |= (1 as libc::c_int as libc::c_uint)
                        << (INDICATOR as libc::c_int as libc::c_ulong)
                            .wrapping_rem(
                                (::std::mem::size_of::<libc::c_uint>() as libc::c_ulong)
                                    .wrapping_mul(8 as libc::c_int as libc::c_ulong),
                            );
            }
            114 => {
                if !parse_num(optarg, &mut fill) {
                    fprintf(
                        stderr,
                        dcgettext(
                            0 as *const libc::c_char,
                            b"Requested fill size \"%s\" is invalid\0" as *const u8
                                as *const libc::c_char,
                            5 as libc::c_int,
                        ),
                        optarg,
                    );
                    fprintf(stderr, b"\n\0" as *const u8 as *const libc::c_char);
                    exit(1 as libc::c_int);
                }
                fill_used = 1 as libc::c_int != 0;
            }
            115 => {
                alt_speller = mallocstrcpy(alt_speller, optarg);
            }
            116 => {
                flags[(SAVE_ON_EXIT as libc::c_int as libc::c_ulong)
                    .wrapping_div(
                        (::std::mem::size_of::<libc::c_uint>() as libc::c_ulong)
                            .wrapping_mul(8 as libc::c_int as libc::c_ulong),
                    ) as usize]
                    |= (1 as libc::c_int as libc::c_uint)
                        << (SAVE_ON_EXIT as libc::c_int as libc::c_ulong)
                            .wrapping_rem(
                                (::std::mem::size_of::<libc::c_uint>() as libc::c_ulong)
                                    .wrapping_mul(8 as libc::c_int as libc::c_ulong),
                            );
            }
            117 => {
                flags[(MAKE_IT_UNIX as libc::c_int as libc::c_ulong)
                    .wrapping_div(
                        (::std::mem::size_of::<libc::c_uint>() as libc::c_ulong)
                            .wrapping_mul(8 as libc::c_int as libc::c_ulong),
                    ) as usize]
                    |= (1 as libc::c_int as libc::c_uint)
                        << (MAKE_IT_UNIX as libc::c_int as libc::c_ulong)
                            .wrapping_rem(
                                (::std::mem::size_of::<libc::c_uint>() as libc::c_ulong)
                                    .wrapping_mul(8 as libc::c_int as libc::c_ulong),
                            );
            }
            118 => {
                flags[(VIEW_MODE as libc::c_int as libc::c_ulong)
                    .wrapping_div(
                        (::std::mem::size_of::<libc::c_uint>() as libc::c_ulong)
                            .wrapping_mul(8 as libc::c_int as libc::c_ulong),
                    ) as usize]
                    |= (1 as libc::c_int as libc::c_uint)
                        << (VIEW_MODE as libc::c_int as libc::c_ulong)
                            .wrapping_rem(
                                (::std::mem::size_of::<libc::c_uint>() as libc::c_ulong)
                                    .wrapping_mul(8 as libc::c_int as libc::c_ulong),
                            );
            }
            119 => {
                hardwrap = 0 as libc::c_int;
            }
            120 => {
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
            }
            121 => {
                flags[(AFTER_ENDS as libc::c_int as libc::c_ulong)
                    .wrapping_div(
                        (::std::mem::size_of::<libc::c_uint>() as libc::c_ulong)
                            .wrapping_mul(8 as libc::c_int as libc::c_ulong),
                    ) as usize]
                    |= (1 as libc::c_int as libc::c_uint)
                        << (AFTER_ENDS as libc::c_int as libc::c_ulong)
                            .wrapping_rem(
                                (::std::mem::size_of::<libc::c_uint>() as libc::c_ulong)
                                    .wrapping_mul(8 as libc::c_int as libc::c_ulong),
                            );
            }
            37 => {
                flags[(STATEFLAGS as libc::c_int as libc::c_ulong)
                    .wrapping_div(
                        (::std::mem::size_of::<libc::c_uint>() as libc::c_ulong)
                            .wrapping_mul(8 as libc::c_int as libc::c_ulong),
                    ) as usize]
                    |= (1 as libc::c_int as libc::c_uint)
                        << (STATEFLAGS as libc::c_int as libc::c_ulong)
                            .wrapping_rem(
                                (::std::mem::size_of::<libc::c_uint>() as libc::c_ulong)
                                    .wrapping_mul(8 as libc::c_int as libc::c_ulong),
                            );
            }
            95 => {
                flags[(MINIBAR as libc::c_int as libc::c_ulong)
                    .wrapping_div(
                        (::std::mem::size_of::<libc::c_uint>() as libc::c_ulong)
                            .wrapping_mul(8 as libc::c_int as libc::c_ulong),
                    ) as usize]
                    |= (1 as libc::c_int as libc::c_uint)
                        << (MINIBAR as libc::c_int as libc::c_ulong)
                            .wrapping_rem(
                                (::std::mem::size_of::<libc::c_uint>() as libc::c_ulong)
                                    .wrapping_mul(8 as libc::c_int as libc::c_ulong),
                            );
            }
            48 => {
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
            }
            _ => {
                printf(
                    dcgettext(
                        0 as *const libc::c_char,
                        b"Type '%s -h' for a list of available options.\n\0" as *const u8
                            as *const libc::c_char,
                        5 as libc::c_int,
                    ),
                    *argv.offset(0 as libc::c_int as isize),
                );
                exit(1 as libc::c_int);
            }
        }
    }
    if (getenv(b"TERM\0" as *const u8 as *const libc::c_char)).is_null() {
        putenv(b"TERM=vt220\0" as *const u8 as *const libc::c_char as *mut libc::c_char);
    }
    if (initscr()).is_null() {
        exit(1 as libc::c_int);
    }
    if has_colors() {
        start_color();
    }
    rescind_colors = !(getenv(b"NO_COLOR\0" as *const u8 as *const libc::c_char))
        .is_null();
    shortcut_init();
    if !ignore_rcfiles {
        let mut fill_cmdline: ssize_t = fill;
        let mut backup_dir_cmdline: *mut libc::c_char = backup_dir;
        let mut word_chars_cmdline: *mut libc::c_char = word_chars;
        let mut stripeclm_cmdline: size_t = stripe_column as size_t;
        let mut tabsize_cmdline: ssize_t = tabsize;
        let mut operating_dir_cmdline: *mut libc::c_char = operating_dir;
        let mut quotestr_cmdline: *mut libc::c_char = quotestr;
        let mut alt_speller_cmdline: *mut libc::c_char = alt_speller;
        let mut flags_cmdline: [libc::c_uint; 4] = [0; 4];
        memcpy(
            flags_cmdline.as_mut_ptr() as *mut libc::c_void,
            flags.as_mut_ptr() as *const libc::c_void,
            ::std::mem::size_of::<[libc::c_uint; 4]>() as libc::c_ulong,
        );
        backup_dir = 0 as *mut libc::c_char;
        word_chars = 0 as *mut libc::c_char;
        operating_dir = 0 as *mut libc::c_char;
        quotestr = 0 as *mut libc::c_char;
        alt_speller = 0 as *mut libc::c_char;
        do_rcfiles();
        if fill_used {
            fill = fill_cmdline;
        }
        if !backup_dir_cmdline.is_null() {
            rpl_free(backup_dir as *mut libc::c_void);
            backup_dir = backup_dir_cmdline;
        }
        if !word_chars_cmdline.is_null() {
            rpl_free(word_chars as *mut libc::c_void);
            word_chars = word_chars_cmdline;
        }
        if stripeclm_cmdline > 0 as libc::c_int as libc::c_ulong {
            stripe_column = stripeclm_cmdline as ssize_t;
        }
        if tabsize_cmdline != -(1 as libc::c_int) as libc::c_long {
            tabsize = tabsize_cmdline;
        }
        if !operating_dir_cmdline.is_null()
            || flags[(RESTRICTED as libc::c_int as libc::c_ulong)
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
            rpl_free(operating_dir as *mut libc::c_void);
            operating_dir = operating_dir_cmdline;
        }
        if !quotestr_cmdline.is_null() {
            rpl_free(quotestr as *mut libc::c_void);
            quotestr = quotestr_cmdline;
        }
        if !alt_speller_cmdline.is_null() {
            rpl_free(alt_speller as *mut libc::c_void);
            alt_speller = alt_speller_cmdline;
        }
        while !alt_speller.is_null() && *alt_speller as libc::c_int != 0
            && *(*__ctype_b_loc()).offset(*alt_speller as libc::c_int as isize)
                as libc::c_int & _ISblank as libc::c_int as libc::c_ushort as libc::c_int
                != 0
        {
            memmove(
                alt_speller as *mut libc::c_void,
                alt_speller.offset(1 as libc::c_int as isize) as *const libc::c_void,
                strlen(alt_speller),
            );
        }
        if !(flags[(NO_WRAP as libc::c_int as libc::c_ulong)
            .wrapping_div(
                (::std::mem::size_of::<libc::c_uint>() as libc::c_ulong)
                    .wrapping_mul(8 as libc::c_int as libc::c_ulong),
            ) as usize]
            & (1 as libc::c_int as libc::c_uint)
                << (NO_WRAP as libc::c_int as libc::c_ulong)
                    .wrapping_rem(
                        (::std::mem::size_of::<libc::c_uint>() as libc::c_ulong)
                            .wrapping_mul(8 as libc::c_int as libc::c_ulong),
                    ) != 0 as libc::c_int as libc::c_uint)
        {
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
        }
        let mut i: size_t = 0 as libc::c_int as size_t;
        while i
            < (::std::mem::size_of::<[libc::c_uint; 4]>() as libc::c_ulong)
                .wrapping_div(::std::mem::size_of::<libc::c_uint>() as libc::c_ulong)
        {
            flags[i as usize] |= flags_cmdline[i as usize];
            i = i.wrapping_add(1);
            i;
        }
    }
    if hardwrap == 0 as libc::c_int {
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
    } else if hardwrap == 1 as libc::c_int {
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
    }
    if flags[(BOLD_TEXT as libc::c_int as libc::c_ulong)
        .wrapping_div(
            (::std::mem::size_of::<libc::c_uint>() as libc::c_ulong)
                .wrapping_mul(8 as libc::c_int as libc::c_ulong),
        ) as usize]
        & (1 as libc::c_int as libc::c_uint)
            << (BOLD_TEXT as libc::c_int as libc::c_ulong)
                .wrapping_rem(
                    (::std::mem::size_of::<libc::c_uint>() as libc::c_ulong)
                        .wrapping_mul(8 as libc::c_int as libc::c_ulong),
                ) != 0 as libc::c_int as libc::c_uint
    {
        hilite_attribute = ((1 as libc::c_uint) << 13 as libc::c_int + 8 as libc::c_int)
            as libc::c_int;
    }
    if flags[(RESTRICTED as libc::c_int as libc::c_ulong)
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
        flags[(MAKE_BACKUP as libc::c_int as libc::c_ulong)
            .wrapping_div(
                (::std::mem::size_of::<libc::c_uint>() as libc::c_ulong)
                    .wrapping_mul(8 as libc::c_int as libc::c_ulong),
            ) as usize]
            &= !((1 as libc::c_int as libc::c_uint)
                << (MAKE_BACKUP as libc::c_int as libc::c_ulong)
                    .wrapping_rem(
                        (::std::mem::size_of::<libc::c_uint>() as libc::c_ulong)
                            .wrapping_mul(8 as libc::c_int as libc::c_ulong),
                    ));
        flags[(HISTORYLOG as libc::c_int as libc::c_ulong)
            .wrapping_div(
                (::std::mem::size_of::<libc::c_uint>() as libc::c_ulong)
                    .wrapping_mul(8 as libc::c_int as libc::c_ulong),
            ) as usize]
            &= !((1 as libc::c_int as libc::c_uint)
                << (HISTORYLOG as libc::c_int as libc::c_ulong)
                    .wrapping_rem(
                        (::std::mem::size_of::<libc::c_uint>() as libc::c_ulong)
                            .wrapping_mul(8 as libc::c_int as libc::c_ulong),
                    ));
        flags[(POSITIONLOG as libc::c_int as libc::c_ulong)
            .wrapping_div(
                (::std::mem::size_of::<libc::c_uint>() as libc::c_ulong)
                    .wrapping_mul(8 as libc::c_int as libc::c_ulong),
            ) as usize]
            &= !((1 as libc::c_int as libc::c_uint)
                << (POSITIONLOG as libc::c_int as libc::c_ulong)
                    .wrapping_rem(
                        (::std::mem::size_of::<libc::c_uint>() as libc::c_ulong)
                            .wrapping_mul(8 as libc::c_int as libc::c_ulong),
                    ));
    }
    if flags[(RAW_SEQUENCES as libc::c_int as libc::c_ulong)
        .wrapping_div(
            (::std::mem::size_of::<libc::c_uint>() as libc::c_ulong)
                .wrapping_mul(8 as libc::c_int as libc::c_ulong),
        ) as usize]
        & (1 as libc::c_int as libc::c_uint)
            << (RAW_SEQUENCES as libc::c_int as libc::c_ulong)
                .wrapping_rem(
                    (::std::mem::size_of::<libc::c_uint>() as libc::c_ulong)
                        .wrapping_mul(8 as libc::c_int as libc::c_ulong),
                ) != 0 as libc::c_int as libc::c_uint
    {
        flags[(USE_MOUSE as libc::c_int as libc::c_ulong)
            .wrapping_div(
                (::std::mem::size_of::<libc::c_uint>() as libc::c_ulong)
                    .wrapping_mul(8 as libc::c_int as libc::c_ulong),
            ) as usize]
            &= !((1 as libc::c_int as libc::c_uint)
                << (USE_MOUSE as libc::c_int as libc::c_ulong)
                    .wrapping_rem(
                        (::std::mem::size_of::<libc::c_uint>() as libc::c_ulong)
                            .wrapping_mul(8 as libc::c_int as libc::c_ulong),
                    ));
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
    {
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
    }
    history_init();
    if (flags[(HISTORYLOG as libc::c_int as libc::c_ulong)
        .wrapping_div(
            (::std::mem::size_of::<libc::c_uint>() as libc::c_ulong)
                .wrapping_mul(8 as libc::c_int as libc::c_ulong),
        ) as usize]
        & (1 as libc::c_int as libc::c_uint)
            << (HISTORYLOG as libc::c_int as libc::c_ulong)
                .wrapping_rem(
                    (::std::mem::size_of::<libc::c_uint>() as libc::c_ulong)
                        .wrapping_mul(8 as libc::c_int as libc::c_ulong),
                ) != 0 as libc::c_int as libc::c_uint
        || flags[(POSITIONLOG as libc::c_int as libc::c_ulong)
            .wrapping_div(
                (::std::mem::size_of::<libc::c_uint>() as libc::c_ulong)
                    .wrapping_mul(8 as libc::c_int as libc::c_ulong),
            ) as usize]
            & (1 as libc::c_int as libc::c_uint)
                << (POSITIONLOG as libc::c_int as libc::c_ulong)
                    .wrapping_rem(
                        (::std::mem::size_of::<libc::c_uint>() as libc::c_ulong)
                            .wrapping_mul(8 as libc::c_int as libc::c_ulong),
                    ) != 0 as libc::c_int as libc::c_uint) && !have_statedir()
    {
        flags[(HISTORYLOG as libc::c_int as libc::c_ulong)
            .wrapping_div(
                (::std::mem::size_of::<libc::c_uint>() as libc::c_ulong)
                    .wrapping_mul(8 as libc::c_int as libc::c_ulong),
            ) as usize]
            &= !((1 as libc::c_int as libc::c_uint)
                << (HISTORYLOG as libc::c_int as libc::c_ulong)
                    .wrapping_rem(
                        (::std::mem::size_of::<libc::c_uint>() as libc::c_ulong)
                            .wrapping_mul(8 as libc::c_int as libc::c_ulong),
                    ));
        flags[(POSITIONLOG as libc::c_int as libc::c_ulong)
            .wrapping_div(
                (::std::mem::size_of::<libc::c_uint>() as libc::c_ulong)
                    .wrapping_mul(8 as libc::c_int as libc::c_ulong),
            ) as usize]
            &= !((1 as libc::c_int as libc::c_uint)
                << (POSITIONLOG as libc::c_int as libc::c_ulong)
                    .wrapping_rem(
                        (::std::mem::size_of::<libc::c_uint>() as libc::c_ulong)
                            .wrapping_mul(8 as libc::c_int as libc::c_ulong),
                    ));
    }
    if flags[(HISTORYLOG as libc::c_int as libc::c_ulong)
        .wrapping_div(
            (::std::mem::size_of::<libc::c_uint>() as libc::c_ulong)
                .wrapping_mul(8 as libc::c_int as libc::c_ulong),
        ) as usize]
        & (1 as libc::c_int as libc::c_uint)
            << (HISTORYLOG as libc::c_int as libc::c_ulong)
                .wrapping_rem(
                    (::std::mem::size_of::<libc::c_uint>() as libc::c_ulong)
                        .wrapping_mul(8 as libc::c_int as libc::c_ulong),
                ) != 0 as libc::c_int as libc::c_uint
    {
        load_history();
    }
    if flags[(POSITIONLOG as libc::c_int as libc::c_ulong)
        .wrapping_div(
            (::std::mem::size_of::<libc::c_uint>() as libc::c_ulong)
                .wrapping_mul(8 as libc::c_int as libc::c_ulong),
        ) as usize]
        & (1 as libc::c_int as libc::c_uint)
            << (POSITIONLOG as libc::c_int as libc::c_ulong)
                .wrapping_rem(
                    (::std::mem::size_of::<libc::c_uint>() as libc::c_ulong)
                        .wrapping_mul(8 as libc::c_int as libc::c_ulong),
                ) != 0 as libc::c_int as libc::c_uint
    {
        load_poshistory();
    }
    if !backup_dir.is_null()
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
        init_backup_dir();
    }
    if !operating_dir.is_null() {
        init_operating_dir();
    }
    if punct.is_null() {
        punct = copy_of(b"!.?\0" as *const u8 as *const libc::c_char);
    }
    if brackets.is_null() {
        brackets = copy_of(b"\"')>]}\0" as *const u8 as *const libc::c_char);
    }
    if quotestr.is_null() {
        quotestr = copy_of(
            b"^([ \t]*([!#%:;>|}]|/{2}))+\0" as *const u8 as *const libc::c_char,
        );
    }
    quoterc = rpl_regcomp(&mut quotereg, quotestr, 1 as libc::c_int);
    if quoterc != 0 as libc::c_int {
        let mut size: size_t = rpl_regerror(
            quoterc,
            &mut quotereg,
            0 as *mut libc::c_char,
            0 as libc::c_int as size_t,
        );
        let mut message: *mut libc::c_char = nmalloc(size) as *mut libc::c_char;
        rpl_regerror(quoterc, &mut quotereg, message, size);
        die(
            dcgettext(
                0 as *const libc::c_char,
                b"Bad quoting regex \"%s\": %s\n\0" as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ),
            quotestr,
            message,
        );
    } else {
        rpl_free(quotestr as *mut libc::c_void);
    }
    if alt_speller.is_null()
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
        let mut spellenv: *const libc::c_char = getenv(
            b"SPELL\0" as *const u8 as *const libc::c_char,
        );
        if !spellenv.is_null() {
            alt_speller = copy_of(spellenv);
        }
    }
    if matchbrackets.is_null() {
        matchbrackets = copy_of(b"(<[{)>]}\0" as *const u8 as *const libc::c_char);
    }
    if whitespace.is_null() {
        if using_utf8() {
            whitespace = copy_of(
                b"\xC2\xBB\xC2\xB7\0" as *const u8 as *const libc::c_char,
            );
            whitelen[0 as libc::c_int as usize] = 2 as libc::c_int;
            whitelen[1 as libc::c_int as usize] = 2 as libc::c_int;
        } else {
            whitespace = copy_of(b">.\0" as *const u8 as *const libc::c_char);
            whitelen[0 as libc::c_int as usize] = 1 as libc::c_int;
            whitelen[1 as libc::c_int as usize] = 1 as libc::c_int;
        }
    }
    last_search = copy_of(b"\0" as *const u8 as *const libc::c_char);
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
    if tabsize == -(1 as libc::c_int) as libc::c_long {
        tabsize = 8 as libc::c_int as ssize_t;
    }
    if has_colors() {
        set_interface_colorpairs();
    } else {
        interface_color_pair[TITLE_BAR as libc::c_int as usize] = hilite_attribute;
        interface_color_pair[LINE_NUMBER as libc::c_int as usize] = hilite_attribute;
        interface_color_pair[GUIDE_STRIPE as libc::c_int
            as usize] = ((1 as libc::c_uint) << 10 as libc::c_int + 8 as libc::c_int)
            as libc::c_int;
        interface_color_pair[SCROLL_BAR as libc::c_int
            as usize] = (1 as libc::c_uint).wrapping_sub(1 as libc::c_uint)
            as libc::c_int;
        interface_color_pair[SELECTED_TEXT as libc::c_int as usize] = hilite_attribute;
        interface_color_pair[SPOTLIGHTED as libc::c_int
            as usize] = ((1 as libc::c_uint) << 10 as libc::c_int + 8 as libc::c_int)
            as libc::c_int;
        interface_color_pair[MINI_INFOBAR as libc::c_int as usize] = hilite_attribute;
        interface_color_pair[PROMPT_BAR as libc::c_int as usize] = hilite_attribute;
        interface_color_pair[STATUS_BAR as libc::c_int as usize] = hilite_attribute;
        interface_color_pair[ERROR_MESSAGE as libc::c_int as usize] = hilite_attribute;
        interface_color_pair[KEY_COMBO as libc::c_int as usize] = hilite_attribute;
        interface_color_pair[FUNCTION_TAG as libc::c_int
            as usize] = (1 as libc::c_uint).wrapping_sub(1 as libc::c_uint)
            as libc::c_int;
    }
    terminal_init();
    window_init();
    curs_set(0 as libc::c_int);
    thebar = if flags[(INDICATOR as libc::c_int as libc::c_ulong)
        .wrapping_div(
            (::std::mem::size_of::<libc::c_uint>() as libc::c_ulong)
                .wrapping_mul(8 as libc::c_int as libc::c_ulong),
        ) as usize]
        & (1 as libc::c_int as libc::c_uint)
            << (INDICATOR as libc::c_int as libc::c_ulong)
                .wrapping_rem(
                    (::std::mem::size_of::<libc::c_uint>() as libc::c_ulong)
                        .wrapping_mul(8 as libc::c_int as libc::c_ulong),
                ) != 0 as libc::c_int as libc::c_uint && LINES > 5 as libc::c_int
        && COLS > 9 as libc::c_int
    {
        1 as libc::c_int
    } else {
        0 as libc::c_int
    };
    bardata = nrealloc(
        bardata as *mut libc::c_void,
        (LINES as libc::c_ulong)
            .wrapping_mul(::std::mem::size_of::<libc::c_int>() as libc::c_ulong),
    ) as *mut libc::c_int;
    editwincols = COLS - thebar;
    signal_init();
    mouse_init();
    controlleft = get_keycode(
        b"kLFT5\0" as *const u8 as *const libc::c_char,
        0x401 as libc::c_int,
    );
    controlright = get_keycode(
        b"kRIT5\0" as *const u8 as *const libc::c_char,
        0x402 as libc::c_int,
    );
    controlup = get_keycode(
        b"kUP5\0" as *const u8 as *const libc::c_char,
        0x403 as libc::c_int,
    );
    controldown = get_keycode(
        b"kDN5\0" as *const u8 as *const libc::c_char,
        0x404 as libc::c_int,
    );
    controlhome = get_keycode(
        b"kHOM5\0" as *const u8 as *const libc::c_char,
        0x405 as libc::c_int,
    );
    controlend = get_keycode(
        b"kEND5\0" as *const u8 as *const libc::c_char,
        0x406 as libc::c_int,
    );
    controldelete = get_keycode(
        b"kDC5\0" as *const u8 as *const libc::c_char,
        0x40d as libc::c_int,
    );
    controlshiftdelete = get_keycode(
        b"kDC6\0" as *const u8 as *const libc::c_char,
        0x41d as libc::c_int,
    );
    shiftup = get_keycode(
        b"kUP\0" as *const u8 as *const libc::c_char,
        0x453 as libc::c_int,
    );
    shiftdown = get_keycode(
        b"kDN\0" as *const u8 as *const libc::c_char,
        0x454 as libc::c_int,
    );
    shiftcontrolleft = get_keycode(
        b"kLFT6\0" as *const u8 as *const libc::c_char,
        0x411 as libc::c_int,
    );
    shiftcontrolright = get_keycode(
        b"kRIT6\0" as *const u8 as *const libc::c_char,
        0x412 as libc::c_int,
    );
    shiftcontrolup = get_keycode(
        b"kUP6\0" as *const u8 as *const libc::c_char,
        0x413 as libc::c_int,
    );
    shiftcontroldown = get_keycode(
        b"kDN6\0" as *const u8 as *const libc::c_char,
        0x414 as libc::c_int,
    );
    shiftcontrolhome = get_keycode(
        b"kHOM6\0" as *const u8 as *const libc::c_char,
        0x415 as libc::c_int,
    );
    shiftcontrolend = get_keycode(
        b"kEND6\0" as *const u8 as *const libc::c_char,
        0x416 as libc::c_int,
    );
    altleft = get_keycode(
        b"kLFT3\0" as *const u8 as *const libc::c_char,
        0x421 as libc::c_int,
    );
    altright = get_keycode(
        b"kRIT3\0" as *const u8 as *const libc::c_char,
        0x422 as libc::c_int,
    );
    altup = get_keycode(
        b"kUP3\0" as *const u8 as *const libc::c_char,
        0x423 as libc::c_int,
    );
    altdown = get_keycode(
        b"kDN3\0" as *const u8 as *const libc::c_char,
        0x424 as libc::c_int,
    );
    altpageup = get_keycode(
        b"kPRV3\0" as *const u8 as *const libc::c_char,
        0x427 as libc::c_int,
    );
    altpagedown = get_keycode(
        b"kNXT3\0" as *const u8 as *const libc::c_char,
        0x428 as libc::c_int,
    );
    altinsert = get_keycode(
        b"kIC3\0" as *const u8 as *const libc::c_char,
        0x42c as libc::c_int,
    );
    altdelete = get_keycode(
        b"kDC3\0" as *const u8 as *const libc::c_char,
        0x42d as libc::c_int,
    );
    shiftaltleft = get_keycode(
        b"kLFT4\0" as *const u8 as *const libc::c_char,
        0x431 as libc::c_int,
    );
    shiftaltright = get_keycode(
        b"kRIT4\0" as *const u8 as *const libc::c_char,
        0x432 as libc::c_int,
    );
    shiftaltup = get_keycode(
        b"kUP4\0" as *const u8 as *const libc::c_char,
        0x433 as libc::c_int,
    );
    shiftaltdown = get_keycode(
        b"kDN4\0" as *const u8 as *const libc::c_char,
        0x434 as libc::c_int,
    );
    set_escdelay(50 as libc::c_int);
    while optind < argc && (openfile.is_null() || 1 as libc::c_int != 0) {
        let mut givenline: ssize_t = 0 as libc::c_int as ssize_t;
        let mut givencol: ssize_t = 0 as libc::c_int as ssize_t;
        let mut searchstring: *mut libc::c_char = 0 as *mut libc::c_char;
        if optind < argc - 1 as libc::c_int
            && *(*argv.offset(optind as isize)).offset(0 as libc::c_int as isize)
                as libc::c_int == '+' as i32
        {
            let mut n: libc::c_int = 1 as libc::c_int;
            while *(*__ctype_b_loc())
                .offset(
                    *(*argv.offset(optind as isize)).offset(n as isize) as libc::c_int
                        as isize,
                ) as libc::c_int
                & _ISalpha as libc::c_int as libc::c_ushort as libc::c_int != 0
            {
                let fresh2 = n;
                n = n + 1;
                match *(*argv.offset(optind as isize)).offset(fresh2 as isize)
                    as libc::c_int
                {
                    99 => {
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
                    }
                    67 => {
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
                    }
                    114 => {
                        flags[(USE_REGEXP as libc::c_int as libc::c_ulong)
                            .wrapping_div(
                                (::std::mem::size_of::<libc::c_uint>() as libc::c_ulong)
                                    .wrapping_mul(8 as libc::c_int as libc::c_ulong),
                            ) as usize]
                            |= (1 as libc::c_int as libc::c_uint)
                                << (USE_REGEXP as libc::c_int as libc::c_ulong)
                                    .wrapping_rem(
                                        (::std::mem::size_of::<libc::c_uint>() as libc::c_ulong)
                                            .wrapping_mul(8 as libc::c_int as libc::c_ulong),
                                    );
                    }
                    82 => {
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
                    }
                    _ => {
                        statusline(
                            ALERT,
                            dcgettext(
                                0 as *const libc::c_char,
                                b"Invalid search modifier '%c'\0" as *const u8
                                    as *const libc::c_char,
                                5 as libc::c_int,
                            ),
                            *(*argv.offset(optind as isize))
                                .offset((n - 1 as libc::c_int) as isize) as libc::c_int,
                        );
                    }
                }
            }
            if *(*argv.offset(optind as isize)).offset(n as isize) as libc::c_int
                == '/' as i32
                || *(*argv.offset(optind as isize)).offset(n as isize) as libc::c_int
                    == '?' as i32
            {
                if *(*argv.offset(optind as isize))
                    .offset((n + 1 as libc::c_int) as isize) != 0
                {
                    searchstring = copy_of(
                        &mut *(*argv.offset(optind as isize))
                            .offset((n + 1 as libc::c_int) as isize),
                    );
                    if *(*argv.offset(optind as isize)).offset(n as isize) as libc::c_int
                        == '?' as i32
                    {
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
                    }
                } else {
                    statusline(
                        ALERT,
                        dcgettext(
                            0 as *const libc::c_char,
                            b"Empty search string\0" as *const u8 as *const libc::c_char,
                            5 as libc::c_int,
                        ),
                    );
                }
                optind += 1;
                optind;
            } else {
                let fresh3 = optind;
                optind = optind + 1;
                if *(*argv.offset(fresh3 as isize)).offset(1 as libc::c_int as isize)
                    as libc::c_int == '\0' as i32
                {
                    givenline = -(1 as libc::c_int) as ssize_t;
                } else if !parse_line_column(
                    &mut *(*argv.offset((optind - 1 as libc::c_int) as isize))
                        .offset(1 as libc::c_int as isize),
                    &mut givenline,
                    &mut givencol,
                ) {
                    statusline(
                        ALERT,
                        dcgettext(
                            0 as *const libc::c_char,
                            b"Invalid line or column number\0" as *const u8
                                as *const libc::c_char,
                            5 as libc::c_int,
                        ),
                    );
                }
            }
        }
        if strcmp(
            *argv.offset(optind as isize),
            b"-\0" as *const u8 as *const libc::c_char,
        ) == 0 as libc::c_int
        {
            optind += 1;
            optind;
            if !scoop_stdin() {
                continue;
            }
        } else {
            let fresh4 = optind;
            optind = optind + 1;
            if !open_buffer(*argv.offset(fresh4 as isize), 1 as libc::c_int != 0) {
                continue;
            }
        }
        if givenline != 0 as libc::c_int as libc::c_long
            || givencol != 0 as libc::c_int as libc::c_long
        {
            goto_line_and_column(
                givenline,
                givencol,
                0 as libc::c_int != 0,
                0 as libc::c_int != 0,
            );
        } else if !searchstring.is_null() {
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
                regexp_init(searchstring);
            }
            if findnextstr(
                searchstring,
                0 as libc::c_int != 0,
                0 as libc::c_int,
                0 as *mut size_t,
                flags[(BACKWARDS_SEARCH as libc::c_int as libc::c_ulong)
                    .wrapping_div(
                        (::std::mem::size_of::<libc::c_uint>() as libc::c_ulong)
                            .wrapping_mul(8 as libc::c_int as libc::c_ulong),
                    ) as usize]
                    & (1 as libc::c_int as libc::c_uint)
                        << (BACKWARDS_SEARCH as libc::c_int as libc::c_ulong)
                            .wrapping_rem(
                                (::std::mem::size_of::<libc::c_uint>() as libc::c_ulong)
                                    .wrapping_mul(8 as libc::c_int as libc::c_ulong),
                            ) != 0 as libc::c_int as libc::c_uint,
                (*openfile).filetop,
                0 as libc::c_int as size_t,
            ) == 0
            {
                not_found_msg(searchstring);
            } else if lastmessage as libc::c_uint
                <= REMARK as libc::c_int as libc::c_uint
            {
                wipe_statusbar();
            }
            (*openfile).placewewant = xplustabs();
            adjust_viewport(CENTERING);
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
                tidy_up_after_search();
            }
            rpl_free(last_search as *mut libc::c_void);
            last_search = searchstring;
            searchstring = 0 as *mut libc::c_char;
        } else if flags[(POSITIONLOG as libc::c_int as libc::c_ulong)
            .wrapping_div(
                (::std::mem::size_of::<libc::c_uint>() as libc::c_ulong)
                    .wrapping_mul(8 as libc::c_int as libc::c_ulong),
            ) as usize]
            & (1 as libc::c_int as libc::c_uint)
                << (POSITIONLOG as libc::c_int as libc::c_ulong)
                    .wrapping_rem(
                        (::std::mem::size_of::<libc::c_uint>() as libc::c_ulong)
                            .wrapping_mul(8 as libc::c_int as libc::c_ulong),
                    ) != 0 as libc::c_int as libc::c_uint
            && *((*openfile).filename).offset(0 as libc::c_int as isize) as libc::c_int
                != '\0' as i32
        {
            let mut savedline: ssize_t = 0;
            let mut savedcol: ssize_t = 0;
            if has_old_position(
                *argv.offset((optind - 1 as libc::c_int) as isize),
                &mut savedline,
                &mut savedcol,
            ) {
                goto_line_and_column(
                    savedline,
                    savedcol,
                    0 as libc::c_int != 0,
                    0 as libc::c_int != 0,
                );
            }
        }
    }
    flags[(NOREAD_MODE as libc::c_int as libc::c_ulong)
        .wrapping_div(
            (::std::mem::size_of::<libc::c_uint>() as libc::c_ulong)
                .wrapping_mul(8 as libc::c_int as libc::c_ulong),
        ) as usize]
        &= !((1 as libc::c_int as libc::c_uint)
            << (NOREAD_MODE as libc::c_int as libc::c_ulong)
                .wrapping_rem(
                    (::std::mem::size_of::<libc::c_uint>() as libc::c_ulong)
                        .wrapping_mul(8 as libc::c_int as libc::c_ulong),
                ));
    if isatty(0 as libc::c_int) == 0 {
        die(
            dcgettext(
                0 as *const libc::c_char,
                b"Standard input is not a terminal\n\0" as *const u8
                    as *const libc::c_char,
                5 as libc::c_int,
            ),
        );
    }
    if openfile.is_null() {
        open_buffer(b"\0" as *const u8 as *const libc::c_char, 1 as libc::c_int != 0);
        flags[(VIEW_MODE as libc::c_int as libc::c_ulong)
            .wrapping_div(
                (::std::mem::size_of::<libc::c_uint>() as libc::c_ulong)
                    .wrapping_mul(8 as libc::c_int as libc::c_ulong),
            ) as usize]
            &= !((1 as libc::c_int as libc::c_uint)
                << (VIEW_MODE as libc::c_int as libc::c_ulong)
                    .wrapping_rem(
                        (::std::mem::size_of::<libc::c_uint>() as libc::c_ulong)
                            .wrapping_mul(8 as libc::c_int as libc::c_ulong),
                    ));
    } else {
        openfile = (*openfile).next;
        if more_than_one {
            mention_name_and_linecount();
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
            flags[(MULTIBUFFER as libc::c_int as libc::c_ulong)
                .wrapping_div(
                    (::std::mem::size_of::<libc::c_uint>() as libc::c_ulong)
                        .wrapping_mul(8 as libc::c_int as libc::c_ulong),
                ) as usize]
                |= (1 as libc::c_int as libc::c_uint)
                    << (MULTIBUFFER as libc::c_int as libc::c_ulong)
                        .wrapping_rem(
                            (::std::mem::size_of::<libc::c_uint>() as libc::c_ulong)
                                .wrapping_mul(8 as libc::c_int as libc::c_ulong),
                        );
        }
    }
    prepare_for_display();
    if !startup_problem.is_null() {
        statusline(ALERT, startup_problem);
    }
    if *(*openfile).filename as libc::c_int == '\0' as i32
        && (*openfile).totsize == 0 as libc::c_int as libc::c_ulong
        && (*openfile).next == openfile
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
        && !(first_sc_for(
            (1 as libc::c_int) << 0 as libc::c_int,
            Some(do_help as unsafe extern "C" fn() -> ()),
        ))
            .is_null()
        && (*first_sc_for(
            (1 as libc::c_int) << 0 as libc::c_int,
            Some(do_help as unsafe extern "C" fn() -> ()),
        ))
            .keycode == 0x7 as libc::c_int
    {
        statusbar(
            dcgettext(
                0 as *const libc::c_char,
                b"Welcome to nano.  For basic help, type Ctrl+G.\0" as *const u8
                    as *const libc::c_char,
                5 as libc::c_int,
            ),
        );
    }
    margin = 12345 as libc::c_int;
    we_are_running = 1 as libc::c_int != 0;
    loop {
        confirm_margin();
        if on_a_vt as libc::c_int != 0
            && waiting_keycodes() == 0 as libc::c_int as libc::c_ulong
        {
            mute_modifiers = 0 as libc::c_int != 0;
        }
        if currmenu != (1 as libc::c_int) << 0 as libc::c_int {
            bottombars((1 as libc::c_int) << 0 as libc::c_int);
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
            && (lastmessage as libc::c_uint) < REMARK as libc::c_int as libc::c_uint
        {
            minibar();
        } else if flags[(CONSTANT_SHOW as libc::c_int as libc::c_ulong)
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
            && lastmessage as libc::c_uint == VACUUM as libc::c_int as libc::c_uint
            && LINES > 1 as libc::c_int
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
            && waiting_keycodes() == 0 as libc::c_int as libc::c_ulong
        {
            report_cursor_position();
        }
        as_an_at = 1 as libc::c_int != 0;
        if refresh_needed as libc::c_int != 0 && LINES > 1 as libc::c_int
            || LINES == 1 as libc::c_int
                && lastmessage as libc::c_uint <= HUSH as libc::c_int as libc::c_uint
        {
            edit_refresh();
        } else {
            place_the_cursor();
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
            && lastmessage as libc::c_uint > HUSH as libc::c_int as libc::c_uint
        {
            if (*openfile).current_y == (editwinrows - 1 as libc::c_int) as libc::c_long
                && LINES > 1 as libc::c_int
            {
                edit_scroll(1 as libc::c_int != 0);
                wnoutrefresh(midwin);
            }
            wredrawln(footwin, 0 as libc::c_int, 1 as libc::c_int);
            wnoutrefresh(footwin);
            place_the_cursor();
        } else if flags[(ZERO as libc::c_int as libc::c_ulong)
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
            && lastmessage as libc::c_uint > VACUUM as libc::c_int as libc::c_uint
        {
            wredrawln(midwin, editwinrows - 1 as libc::c_int, 1 as libc::c_int);
        }
        *__errno_location() = 0 as libc::c_int;
        focusing = 1 as libc::c_int != 0;
        put_cursor_at_end_of_answer();
        process_a_keystroke();
    };
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
