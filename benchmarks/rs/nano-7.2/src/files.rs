use ::libc;
use ::c2rust_bitfields;
extern "C" {
    pub type __dirstream;
    pub type re_dfa_t;
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    pub type ldat;
    fn beep() -> libc::c_int;
    fn curs_set(_: libc::c_int) -> libc::c_int;
    fn doupdate() -> libc::c_int;
    fn isendwin() -> bool;
    fn napms(_: libc::c_int) -> libc::c_int;
    fn waddnstr(_: *mut WINDOW, _: *const libc::c_char, _: libc::c_int) -> libc::c_int;
    fn opendir(__name: *const libc::c_char) -> *mut DIR;
    fn exit(_: libc::c_int) -> !;
    fn getenv(__name: *const libc::c_char) -> *mut libc::c_char;
    fn mkstemp(__template: *mut libc::c_char) -> libc::c_int;
    fn mkstemps(__template: *mut libc::c_char, __suffixlen: libc::c_int) -> libc::c_int;
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
    fn rpl_free(ptr: *mut libc::c_void);
    fn __xstat(
        __ver: libc::c_int,
        __filename: *const libc::c_char,
        __stat_buf: *mut stat,
    ) -> libc::c_int;
    fn fchmod(__fd: libc::c_int, __mode: __mode_t) -> libc::c_int;
    fn futimens(__fd: libc::c_int, __times: *const timespec) -> libc::c_int;
    fn fclose(__stream: *mut FILE) -> libc::c_int;
    fn fflush(__stream: *mut FILE) -> libc::c_int;
    fn fopen(_: *const libc::c_char, _: *const libc::c_char) -> *mut FILE;
    fn fdopen(__fd: libc::c_int, __modes: *const libc::c_char) -> *mut FILE;
    fn sprintf(_: *mut libc::c_char, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn snprintf(
        _: *mut libc::c_char,
        _: libc::c_ulong,
        _: *const libc::c_char,
        _: ...
    ) -> libc::c_int;
    fn __uflow(_: *mut FILE) -> libc::c_int;
    fn putc(__c: libc::c_int, __stream: *mut FILE) -> libc::c_int;
    fn fread(
        _: *mut libc::c_void,
        _: libc::c_ulong,
        _: libc::c_ulong,
        _: *mut FILE,
    ) -> libc::c_ulong;
    fn fwrite(
        _: *const libc::c_void,
        _: libc::c_ulong,
        _: libc::c_ulong,
        _: *mut FILE,
    ) -> libc::c_ulong;
    fn ferror(__stream: *mut FILE) -> libc::c_int;
    fn fileno(__stream: *mut FILE) -> libc::c_int;
    fn flockfile(__stream: *mut FILE);
    fn funlockfile(__stream: *mut FILE);
    fn sigaction(
        __sig: libc::c_int,
        __act: *const sigaction,
        __oact: *mut sigaction,
    ) -> libc::c_int;
    fn kill(__pid: __pid_t, __sig: libc::c_int) -> libc::c_int;
    fn wmove(_: *mut WINDOW, _: libc::c_int, _: libc::c_int) -> libc::c_int;
    fn wnoutrefresh(_: *mut WINDOW) -> libc::c_int;
    static mut COLS: libc::c_int;
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
    static mut we_are_running: bool;
    static mut more_than_one: bool;
    static mut report_size: bool;
    static mut ran_a_tool: bool;
    static mut inhelp: bool;
    static mut focusing: bool;
    static mut as_an_at: bool;
    static mut control_C_was_pressed: bool;
    static mut lastmessage: message_type;
    static mut answer: *mut libc::c_char;
    static mut present_path: *mut libc::c_char;
    static mut flags: [libc::c_uint; 4];
    static mut midwin: *mut WINDOW;
    static mut editwinrows: libc::c_int;
    static mut cutbuffer: *mut linestruct;
    static mut openfile: *mut openfilestruct;
    static mut startfile: *mut openfilestruct;
    static mut exit_tag: *const libc::c_char;
    static mut close_tag: *const libc::c_char;
    static mut backup_dir: *mut libc::c_char;
    static mut operating_dir: *mut libc::c_char;
    static mut have_palette: bool;
    static mut perturbed: bool;
    static mut recook: bool;
    static mut refresh_needed: bool;
    static mut currmenu: libc::c_int;
    static mut exitfunc: *mut funcstruct;
    static mut execute_history: *mut linestruct;
    static mut homedir: *mut libc::c_char;
    fn browse_in(inpath: *const libc::c_char) -> *mut libc::c_char;
    fn collect_char(
        string: *const libc::c_char,
        thechar: *mut libc::c_char,
    ) -> libc::c_int;
    fn mbstrcasecmp(s1: *const libc::c_char, s2: *const libc::c_char) -> libc::c_int;
    fn revstrstr(
        haystack: *const libc::c_char,
        needle: *const libc::c_char,
        pointer: *const libc::c_char,
    ) -> *mut libc::c_char;
    fn find_and_prime_applicable_syntax();
    fn precalc_multicolorinfo();
    fn ingraft_buffer(topline: *mut linestruct);
    fn do_snip(marked: bool, until_eof: bool, append: bool);
    fn copy_marked_region();
    fn copy_of(string: *const libc::c_char) -> *mut libc::c_char;
    fn make_new_node(prevnode: *mut linestruct) -> *mut linestruct;
    fn nmalloc(howmuch: size_t) -> *mut libc::c_void;
    fn statusline(importance: message_type, msg: *const libc::c_char, _: ...);
    fn mallocstrcpy(
        dest: *mut libc::c_char,
        src: *const libc::c_char,
    ) -> *mut libc::c_char;
    fn restore_handler_for_Ctrl_C();
    fn update_undo(action: undo_type);
    fn less_than_a_screenful(was_lineno: size_t, was_leftedge: size_t) -> bool;
    fn xplustabs() -> size_t;
    fn recode_NUL_to_LF(string: *mut libc::c_char, length: size_t);
    fn terminal_init();
    fn reconnect_and_store_state();
    fn block_sigwinch(blockit: bool);
    fn nrealloc(ptr: *mut libc::c_void, howmuch: size_t) -> *mut libc::c_void;
    fn leftedge_for(column: size_t, line: *mut linestruct) -> size_t;
    fn add_undo(action: undo_type, message: *const libc::c_char);
    fn install_handler_for_Ctrl_C();
    fn measured_copy(string: *const libc::c_char, count: size_t) -> *mut libc::c_char;
    fn get_homedir();
    fn statusbar(msg: *const libc::c_char);
    fn wipe_statusbar();
    fn finish();
    fn ask_user(withall: bool, question: *const libc::c_char) -> libc::c_int;
    fn breadth(text: *const libc::c_char) -> size_t;
    fn display_string(
        buf: *const libc::c_char,
        column: size_t,
        span: size_t,
        isdata: bool,
        isprompt: bool,
    ) -> *mut libc::c_char;
    fn blank_bottombars();
    fn discard_until(thisitem: *const undostruct);
    fn free_lines(src: *mut linestruct);
    fn titlebar(path: *const libc::c_char);
    fn tail(path: *const libc::c_char) -> *const libc::c_char;
    fn ensure_firstcolumn_is_aligned();
    fn goto_line_and_column(
        line: ssize_t,
        column: ssize_t,
        retain_answer: bool,
        interactive: bool,
    );
    fn has_old_position(
        file: *const libc::c_char,
        line: *mut ssize_t,
        column: *mut ssize_t,
    ) -> bool;
    fn free_and_assign(
        dest: *mut libc::c_char,
        src: *mut libc::c_char,
    ) -> *mut libc::c_char;
    fn update_history(
        item: *mut *mut linestruct,
        text: *const libc::c_char,
        avoid_duplicates: bool,
    );
    fn do_undo();
    fn goto_line_posx(line: ssize_t, pos_x: size_t);
    fn enable_kb_interrupt();
    fn recode_LF_to_NUL(string: *mut libc::c_char) -> size_t;
    fn to_files();
    fn func_from_key(keycode: libc::c_int) -> functionptrtype;
    fn add_or_remove_pipe_symbol_from_answer();
    fn flip_pipe();
    fn flip_execute();
    fn flip_convert();
    fn flip_newbuffer();
    fn edit_refresh();
    fn do_prompt(
        menu: libc::c_int,
        provided: *const libc::c_char,
        history_list: *mut *mut linestruct,
        refresh_func: Option::<unsafe extern "C" fn() -> ()>,
        msg: *const libc::c_char,
        _: ...
    ) -> libc::c_int;
    fn in_restricted_mode() -> bool;
    fn die(msg: *const libc::c_char, _: ...);
    fn warn_and_briefly_pause(msg: *const libc::c_char);
    fn delete_node(line: *mut linestruct);
    fn get_region(
        top: *mut *mut linestruct,
        top_x: *mut size_t,
        bot: *mut *mut linestruct,
        bot_x: *mut size_t,
    );
    fn do_credits();
    fn do_help();
    fn prepend_it();
    fn append_it();
    fn back_it_up();
    fn mac_format();
    fn dos_format();
    fn discard_buffer();
    fn close_and_go();
    fn free_chararray(array: *mut *mut libc::c_char, len: size_t);
    fn blank_edit();
    fn readdir(__dirp: *mut DIR) -> *mut dirent;
    fn closedir(__dirp: *mut DIR) -> libc::c_int;
    fn __errno_location() -> *mut libc::c_int;
    fn open(__file: *const libc::c_char, __oflag: libc::c_int, _: ...) -> libc::c_int;
    fn dirname(__path: *mut libc::c_char) -> *mut libc::c_char;
    fn __xpg_basename(__path: *mut libc::c_char) -> *mut libc::c_char;
    fn endpwent();
    fn getpwent() -> *mut passwd;
    fn getpwuid(__uid: __uid_t) -> *mut passwd;
    fn strcat(_: *mut libc::c_char, _: *const libc::c_char) -> *mut libc::c_char;
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    fn strrchr(_: *const libc::c_char, _: libc::c_int) -> *mut libc::c_char;
    fn strncpy(
        _: *mut libc::c_char,
        _: *const libc::c_char,
        _: libc::c_ulong,
    ) -> *mut libc::c_char;
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    fn strchr(_: *const libc::c_char, _: libc::c_int) -> *mut libc::c_char;
    fn memset(
        _: *mut libc::c_void,
        _: libc::c_int,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn strerror(_: libc::c_int) -> *mut libc::c_char;
    fn strncmp(
        _: *const libc::c_char,
        _: *const libc::c_char,
        _: libc::c_ulong,
    ) -> libc::c_int;
    fn memmove(
        _: *mut libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn strstr(_: *const libc::c_char, _: *const libc::c_char) -> *mut libc::c_char;
    fn access(__name: *const libc::c_char, __type: libc::c_int) -> libc::c_int;
    fn close(__fd: libc::c_int) -> libc::c_int;
    fn read(__fd: libc::c_int, __buf: *mut libc::c_void, __nbytes: size_t) -> ssize_t;
    fn pipe(__pipedes: *mut libc::c_int) -> libc::c_int;
    fn fchown(__fd: libc::c_int, __owner: __uid_t, __group: __gid_t) -> libc::c_int;
    fn chdir(__path: *const libc::c_char) -> libc::c_int;
    fn dup2(__fd: libc::c_int, __fd2: libc::c_int) -> libc::c_int;
    fn execl(
        __path: *const libc::c_char,
        __arg: *const libc::c_char,
        _: ...
    ) -> libc::c_int;
    fn getpid() -> __pid_t;
    fn geteuid() -> __uid_t;
    fn fork() -> __pid_t;
    fn isatty(__fd: libc::c_int) -> libc::c_int;
    fn unlink(__name: *const libc::c_char) -> libc::c_int;
    fn gethostname(__name: *mut libc::c_char, __len: size_t) -> libc::c_int;
    fn fsync(__fd: libc::c_int) -> libc::c_int;
    fn waitpid(
        __pid: __pid_t,
        __stat_loc: *mut libc::c_int,
        __options: libc::c_int,
    ) -> __pid_t;
}
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
pub type mode_t = __mode_t;
pub type uid_t = __uid_t;
pub type pid_t = __pid_t;
pub type ssize_t = __ssize_t;
pub type size_t = libc::c_ulong;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __sigset_t {
    pub __val: [libc::c_ulong; 16],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct timespec {
    pub tv_sec: __time_t,
    pub tv_nsec: __syscall_slong_t,
}
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
pub const ZERO: C2RustUnnamed_10 = 48;
pub const MINIBAR: C2RustUnnamed_10 = 47;
pub const USE_MAGIC: C2RustUnnamed_10 = 46;
pub const STATEFLAGS: C2RustUnnamed_10 = 45;
pub const BOOKSTYLE: C2RustUnnamed_10 = 44;
pub const INDICATOR: C2RustUnnamed_10 = 43;
pub const EMPTY_LINE: C2RustUnnamed_10 = 42;
pub const JUMPY_SCROLLING: C2RustUnnamed_10 = 41;
pub const BREAK_LONG_LINES: C2RustUnnamed_10 = 40;
pub const LET_THEM_ZAP: C2RustUnnamed_10 = 39;
pub const AFTER_ENDS: C2RustUnnamed_10 = 38;
pub const AT_BLANKS: C2RustUnnamed_10 = 37;
pub const LINE_NUMBERS: C2RustUnnamed_10 = 36;
pub const SHOW_CURSOR: C2RustUnnamed_10 = 35;
pub const TRIM_BLANKS: C2RustUnnamed_10 = 34;
pub const MAKE_IT_UNIX: C2RustUnnamed_10 = 33;
pub const NOREAD_MODE: C2RustUnnamed_10 = 32;
pub const LOCKING: C2RustUnnamed_10 = 31;
pub const POSITIONLOG: C2RustUnnamed_10 = 30;
pub const SOFTWRAP: C2RustUnnamed_10 = 29;
pub const BOLD_TEXT: C2RustUnnamed_10 = 28;
pub const NO_NEWLINES: C2RustUnnamed_10 = 27;
pub const WORD_BOUNDS: C2RustUnnamed_10 = 26;
pub const QUICK_BLANK: C2RustUnnamed_10 = 25;
pub const TABS_TO_SPACES: C2RustUnnamed_10 = 24;
pub const WHITESPACE_DISPLAY: C2RustUnnamed_10 = 23;
pub const SMART_HOME: C2RustUnnamed_10 = 22;
pub const RESTRICTED: C2RustUnnamed_10 = 21;
pub const HISTORYLOG: C2RustUnnamed_10 = 20;
pub const PRESERVE: C2RustUnnamed_10 = 19;
pub const NO_SYNTAX: C2RustUnnamed_10 = 18;
pub const INSECURE_BACKUP: C2RustUnnamed_10 = 17;
pub const MAKE_BACKUP: C2RustUnnamed_10 = 16;
pub const NO_CONVERT: C2RustUnnamed_10 = 15;
pub const RAW_SEQUENCES: C2RustUnnamed_10 = 14;
pub const REBIND_DELETE: C2RustUnnamed_10 = 13;
pub const MULTIBUFFER: C2RustUnnamed_10 = 12;
pub const BACKWARDS_SEARCH: C2RustUnnamed_10 = 11;
pub const CUT_FROM_CURSOR: C2RustUnnamed_10 = 10;
pub const SAVE_ON_EXIT: C2RustUnnamed_10 = 9;
pub const USE_REGEXP: C2RustUnnamed_10 = 8;
pub const USE_MOUSE: C2RustUnnamed_10 = 7;
pub const VIEW_MODE: C2RustUnnamed_10 = 6;
pub const AUTOINDENT: C2RustUnnamed_10 = 5;
pub const NO_WRAP: C2RustUnnamed_10 = 4;
pub const NO_HELP: C2RustUnnamed_10 = 3;
pub const CONSTANT_SHOW: C2RustUnnamed_10 = 2;
pub const CASE_SENSITIVE: C2RustUnnamed_10 = 1;
pub const DONTUSE: C2RustUnnamed_10 = 0;
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
pub struct funcstruct {
    pub func: Option::<unsafe extern "C" fn() -> ()>,
    pub tag: *const libc::c_char,
    pub phrase: *const libc::c_char,
    pub blank_after: bool,
    pub menus: libc::c_int,
    pub next: *mut funcstruct,
}
pub type functionptrtype = Option::<unsafe extern "C" fn() -> ()>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct passwd {
    pub pw_name: *mut libc::c_char,
    pub pw_passwd: *mut libc::c_char,
    pub pw_uid: __uid_t,
    pub pw_gid: __gid_t,
    pub pw_gecos: *mut libc::c_char,
    pub pw_dir: *mut libc::c_char,
    pub pw_shell: *mut libc::c_char,
}
#[inline]
unsafe extern "C" fn stat(
    mut __path: *const libc::c_char,
    mut __statbuf: *mut stat,
) -> libc::c_int {
    return __xstat(1 as libc::c_int, __path, __statbuf);
}
#[inline]
unsafe extern "C" fn getc_unlocked(mut __fp: *mut FILE) -> libc::c_int {
    return if ((*__fp)._IO_read_ptr >= (*__fp)._IO_read_end) as libc::c_int
        as libc::c_long != 0
    {
        __uflow(__fp)
    } else {
        let fresh0 = (*__fp)._IO_read_ptr;
        (*__fp)._IO_read_ptr = ((*__fp)._IO_read_ptr).offset(1);
        *(fresh0 as *mut libc::c_uchar) as libc::c_int
    };
}
pub unsafe extern "C" fn make_new_buffer() {
    let mut newnode: *mut openfilestruct = nmalloc(
        ::std::mem::size_of::<openfilestruct>() as libc::c_ulong,
    ) as *mut openfilestruct;
    if openfile.is_null() {
        (*newnode).prev = newnode;
        (*newnode).next = newnode;
        startfile = newnode;
    } else {
        (*newnode).prev = openfile;
        (*newnode).next = (*openfile).next;
        (*(*openfile).next).prev = newnode;
        (*openfile).next = newnode;
        (*exitfunc).tag = close_tag;
        more_than_one = !inhelp || more_than_one as libc::c_int != 0;
    }
    openfile = newnode;
    (*openfile).filename = copy_of(b"\0" as *const u8 as *const libc::c_char);
    (*openfile).filetop = make_new_node(0 as *mut linestruct);
    (*(*openfile).filetop).data = copy_of(b"\0" as *const u8 as *const libc::c_char);
    (*openfile).filebot = (*openfile).filetop;
    (*openfile).current = (*openfile).filetop;
    (*openfile).current_x = 0 as libc::c_int as size_t;
    (*openfile).placewewant = 0 as libc::c_int as size_t;
    (*openfile).current_y = 0 as libc::c_int as ssize_t;
    (*openfile).edittop = (*openfile).filetop;
    (*openfile).firstcolumn = 0 as libc::c_int as size_t;
    (*openfile).totsize = 0 as libc::c_int as size_t;
    (*openfile).modified = 0 as libc::c_int != 0;
    (*openfile).spillage_line = 0 as *mut linestruct;
    (*openfile).mark = 0 as *mut linestruct;
    (*openfile).softmark = 0 as libc::c_int != 0;
    (*openfile).fmt = UNSPECIFIED;
    (*openfile).undotop = 0 as *mut undostruct;
    (*openfile).current_undo = 0 as *mut undostruct;
    (*openfile).last_saved = 0 as *mut undostruct;
    (*openfile).last_action = OTHER;
    (*openfile).statinfo = 0 as *mut stat;
    (*openfile).lock_filename = 0 as *mut libc::c_char;
    (*openfile).errormessage = 0 as *mut libc::c_char;
    (*openfile).syntax = 0 as *mut syntaxtype;
}
pub unsafe extern "C" fn crop_to_fit(
    mut name: *const libc::c_char,
    mut room: libc::c_int,
) -> *mut libc::c_char {
    let mut clipped: *mut libc::c_char = 0 as *mut libc::c_char;
    if breadth(name) <= room as libc::c_ulong {
        return display_string(
            name,
            0 as libc::c_int as size_t,
            room as size_t,
            0 as libc::c_int != 0,
            0 as libc::c_int != 0,
        );
    }
    if room < 4 as libc::c_int {
        return copy_of(b"_\0" as *const u8 as *const libc::c_char);
    }
    clipped = display_string(
        name,
        (breadth(name))
            .wrapping_sub(room as libc::c_ulong)
            .wrapping_add(3 as libc::c_int as libc::c_ulong),
        room as size_t,
        0 as libc::c_int != 0,
        0 as libc::c_int != 0,
    );
    clipped = nrealloc(
        clipped as *mut libc::c_void,
        (strlen(clipped)).wrapping_add(4 as libc::c_int as libc::c_ulong),
    ) as *mut libc::c_char;
    memmove(
        clipped.offset(3 as libc::c_int as isize) as *mut libc::c_void,
        clipped as *const libc::c_void,
        (strlen(clipped)).wrapping_add(1 as libc::c_int as libc::c_ulong),
    );
    *clipped.offset(0 as libc::c_int as isize) = '.' as i32 as libc::c_char;
    *clipped.offset(1 as libc::c_int as isize) = '.' as i32 as libc::c_char;
    *clipped.offset(2 as libc::c_int as isize) = '.' as i32 as libc::c_char;
    return clipped;
}
pub unsafe extern "C" fn delete_lockfile(mut lockfilename: *const libc::c_char) -> bool {
    if unlink(lockfilename) < 0 as libc::c_int && *__errno_location() != 2 as libc::c_int
    {
        statusline(
            MILD,
            dcgettext(
                0 as *const libc::c_char,
                b"Error deleting lock file %s: %s\0" as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ),
            lockfilename,
            strerror(*__errno_location()),
        );
        return 0 as libc::c_int != 0;
    } else {
        return 1 as libc::c_int != 0
    };
}
pub static mut locking_prefix: *const libc::c_char = b".\0" as *const u8
    as *const libc::c_char;
pub static mut locking_suffix: *const libc::c_char = b".swp\0" as *const u8
    as *const libc::c_char;
pub unsafe extern "C" fn write_lockfile(
    mut lockfilename: *const libc::c_char,
    mut filename: *const libc::c_char,
    mut modified: bool,
) -> bool {
    let mut mypid: pid_t = getpid();
    let mut myuid: uid_t = geteuid();
    let mut mypwuid: *mut passwd = getpwuid(myuid);
    let mut myhostname: [libc::c_char; 32] = [0; 32];
    let mut fd: libc::c_int = 0;
    let mut filestream: *mut FILE = 0 as *mut FILE;
    let mut lockdata: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut wroteamt: size_t = 0;
    if mypwuid.is_null() {
        statusline(
            MILD,
            dcgettext(
                0 as *const libc::c_char,
                b"Couldn't determine my identity for lock file\0" as *const u8
                    as *const libc::c_char,
                5 as libc::c_int,
            ),
        );
        return 0 as libc::c_int != 0;
    }
    if gethostname(myhostname.as_mut_ptr(), 31 as libc::c_int as size_t)
        < 0 as libc::c_int && *__errno_location() != 36 as libc::c_int
    {
        statusline(
            MILD,
            dcgettext(
                0 as *const libc::c_char,
                b"Couldn't determine hostname: %s\0" as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ),
            strerror(*__errno_location()),
        );
        return 0 as libc::c_int != 0;
    } else {
        myhostname[31 as libc::c_int as usize] = '\0' as i32 as libc::c_char;
    }
    if !delete_lockfile(lockfilename) {
        return 0 as libc::c_int != 0;
    }
    fd = open(
        lockfilename,
        0o1 as libc::c_int | 0o100 as libc::c_int | 0o200 as libc::c_int,
        0o400 as libc::c_int | 0o200 as libc::c_int
            | 0o400 as libc::c_int >> 3 as libc::c_int
            | 0o200 as libc::c_int >> 3 as libc::c_int
            | 0o400 as libc::c_int >> 3 as libc::c_int >> 3 as libc::c_int
            | 0o200 as libc::c_int >> 3 as libc::c_int >> 3 as libc::c_int,
    );
    if fd > 0 as libc::c_int {
        filestream = fdopen(fd, b"wb\0" as *const u8 as *const libc::c_char);
    }
    if filestream.is_null() {
        statusline(
            MILD,
            dcgettext(
                0 as *const libc::c_char,
                b"Error writing lock file %s: %s\0" as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ),
            lockfilename,
            strerror(*__errno_location()),
        );
        if fd > 0 as libc::c_int {
            close(fd);
        }
        return 0 as libc::c_int != 0;
    }
    lockdata = nmalloc(1024 as libc::c_int as size_t) as *mut libc::c_char;
    memset(
        lockdata as *mut libc::c_void,
        0 as libc::c_int,
        1024 as libc::c_int as libc::c_ulong,
    );
    *lockdata.offset(0 as libc::c_int as isize) = 0x62 as libc::c_int as libc::c_char;
    *lockdata.offset(1 as libc::c_int as isize) = 0x30 as libc::c_int as libc::c_char;
    snprintf(
        &mut *lockdata.offset(2 as libc::c_int as isize) as *mut libc::c_char,
        11 as libc::c_int as libc::c_ulong,
        b"nano %s\0" as *const u8 as *const libc::c_char,
        b"7.2\0" as *const u8 as *const libc::c_char,
    );
    *lockdata
        .offset(
            24 as libc::c_int as isize,
        ) = (mypid % 256 as libc::c_int) as libc::c_char;
    *lockdata
        .offset(
            25 as libc::c_int as isize,
        ) = (mypid / 256 as libc::c_int % 256 as libc::c_int) as libc::c_char;
    *lockdata
        .offset(
            26 as libc::c_int as isize,
        ) = (mypid / (256 as libc::c_int * 256 as libc::c_int) % 256 as libc::c_int)
        as libc::c_char;
    *lockdata
        .offset(
            27 as libc::c_int as isize,
        ) = (mypid / (256 as libc::c_int * 256 as libc::c_int * 256 as libc::c_int))
        as libc::c_char;
    strncpy(
        &mut *lockdata.offset(28 as libc::c_int as isize),
        (*mypwuid).pw_name,
        16 as libc::c_int as libc::c_ulong,
    );
    strncpy(
        &mut *lockdata.offset(68 as libc::c_int as isize),
        myhostname.as_mut_ptr(),
        32 as libc::c_int as libc::c_ulong,
    );
    strncpy(
        &mut *lockdata.offset(108 as libc::c_int as isize),
        filename,
        768 as libc::c_int as libc::c_ulong,
    );
    *lockdata
        .offset(
            1007 as libc::c_int as isize,
        ) = (if modified as libc::c_int != 0 {
        0x55 as libc::c_int
    } else {
        0 as libc::c_int
    }) as libc::c_char;
    wroteamt = fwrite(
        lockdata as *const libc::c_void,
        1 as libc::c_int as libc::c_ulong,
        1024 as libc::c_int as libc::c_ulong,
        filestream,
    );
    rpl_free(lockdata as *mut libc::c_void);
    if fclose(filestream) == -(1 as libc::c_int)
        || wroteamt < 1024 as libc::c_int as libc::c_ulong
    {
        statusline(
            MILD,
            dcgettext(
                0 as *const libc::c_char,
                b"Error writing lock file %s: %s\0" as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ),
            lockfilename,
            strerror(*__errno_location()),
        );
        return 0 as libc::c_int != 0;
    }
    return 1 as libc::c_int != 0;
}
pub unsafe extern "C" fn do_lockfile(
    mut filename: *const libc::c_char,
    mut ask_the_user: bool,
) -> *mut libc::c_char {
    let mut namecopy: *mut libc::c_char = copy_of(filename);
    let mut secondcopy: *mut libc::c_char = copy_of(filename);
    let mut locknamesize: size_t = (strlen(filename))
        .wrapping_add(strlen(locking_prefix))
        .wrapping_add(strlen(locking_suffix))
        .wrapping_add(3 as libc::c_int as libc::c_ulong);
    let mut lockfilename: *mut libc::c_char = nmalloc(locknamesize) as *mut libc::c_char;
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
    snprintf(
        lockfilename,
        locknamesize,
        b"%s/%s%s%s\0" as *const u8 as *const libc::c_char,
        dirname(namecopy),
        locking_prefix,
        __xpg_basename(secondcopy),
        locking_suffix,
    );
    rpl_free(secondcopy as *mut libc::c_void);
    rpl_free(namecopy as *mut libc::c_void);
    if !ask_the_user && stat(lockfilename, &mut fileinfo) != -(1 as libc::c_int) {
        blank_bottombars();
        statusline(
            ALERT,
            dcgettext(
                0 as *const libc::c_char,
                b"Someone else is also editing this file\0" as *const u8
                    as *const libc::c_char,
                5 as libc::c_int,
            ),
        );
        napms(1200 as libc::c_int);
    } else if stat(lockfilename, &mut fileinfo) != -(1 as libc::c_int) {
        let mut lockbuf: *mut libc::c_char = 0 as *mut libc::c_char;
        let mut question: *mut libc::c_char = 0 as *mut libc::c_char;
        let mut pidstring: *mut libc::c_char = 0 as *mut libc::c_char;
        let mut postedname: *mut libc::c_char = 0 as *mut libc::c_char;
        let mut promptstr: *mut libc::c_char = 0 as *mut libc::c_char;
        static mut lockprog: [libc::c_char; 11] = [0; 11];
        static mut lockuser: [libc::c_char; 17] = [0; 17];
        let mut lockfd: libc::c_int = 0;
        let mut lockpid: libc::c_int = 0;
        let mut choice: libc::c_int = 0;
        let mut readamt: ssize_t = 0;
        lockfd = open(lockfilename, 0 as libc::c_int);
        if lockfd < 0 as libc::c_int {
            statusline(
                ALERT,
                dcgettext(
                    0 as *const libc::c_char,
                    b"Error opening lock file %s: %s\0" as *const u8
                        as *const libc::c_char,
                    5 as libc::c_int,
                ),
                lockfilename,
                strerror(*__errno_location()),
            );
            rpl_free(lockfilename as *mut libc::c_void);
            return 0 as *mut libc::c_char;
        }
        lockbuf = nmalloc(1024 as libc::c_int as size_t) as *mut libc::c_char;
        readamt = read(
            lockfd,
            lockbuf as *mut libc::c_void,
            1024 as libc::c_int as size_t,
        );
        close(lockfd);
        if readamt < 68 as libc::c_int as libc::c_long
            || *lockbuf.offset(0 as libc::c_int as isize) as libc::c_int
                != 0x62 as libc::c_int
            || *lockbuf.offset(1 as libc::c_int as isize) as libc::c_int
                != 0x30 as libc::c_int
        {
            statusline(
                ALERT,
                dcgettext(
                    0 as *const libc::c_char,
                    b"Bad lock file is ignored: %s\0" as *const u8
                        as *const libc::c_char,
                    5 as libc::c_int,
                ),
                lockfilename,
            );
            rpl_free(lockfilename as *mut libc::c_void);
            rpl_free(lockbuf as *mut libc::c_void);
            return 0 as *mut libc::c_char;
        }
        strncpy(
            lockprog.as_mut_ptr(),
            &mut *lockbuf.offset(2 as libc::c_int as isize),
            10 as libc::c_int as libc::c_ulong,
        );
        lockprog[10 as libc::c_int as usize] = '\0' as i32 as libc::c_char;
        lockpid = ((*lockbuf.offset(27 as libc::c_int as isize) as libc::c_uchar
            as libc::c_int * 256 as libc::c_int
            + *lockbuf.offset(26 as libc::c_int as isize) as libc::c_uchar
                as libc::c_int) * 256 as libc::c_int
            + *lockbuf.offset(25 as libc::c_int as isize) as libc::c_uchar
                as libc::c_int) * 256 as libc::c_int
            + *lockbuf.offset(24 as libc::c_int as isize) as libc::c_uchar
                as libc::c_int;
        strncpy(
            lockuser.as_mut_ptr(),
            &mut *lockbuf.offset(28 as libc::c_int as isize),
            16 as libc::c_int as libc::c_ulong,
        );
        lockuser[16 as libc::c_int as usize] = '\0' as i32 as libc::c_char;
        rpl_free(lockbuf as *mut libc::c_void);
        pidstring = nmalloc(11 as libc::c_int as size_t) as *mut libc::c_char;
        sprintf(
            pidstring,
            b"%u\0" as *const u8 as *const libc::c_char,
            lockpid as libc::c_uint,
        );
        as_an_at = 0 as libc::c_int != 0;
        question = dcgettext(
            0 as *const libc::c_char,
            b"File %s is being edited by %s (with %s, PID %s); open anyway?\0"
                as *const u8 as *const libc::c_char,
            5 as libc::c_int,
        );
        postedname = crop_to_fit(
            filename,
            (COLS as libc::c_ulong)
                .wrapping_sub(breadth(question))
                .wrapping_sub(breadth(lockuser.as_mut_ptr()))
                .wrapping_sub(breadth(lockprog.as_mut_ptr()))
                .wrapping_sub(breadth(pidstring))
                .wrapping_add(7 as libc::c_int as libc::c_ulong) as libc::c_int,
        );
        promptstr = nmalloc(
            (strlen(question))
                .wrapping_add(29 as libc::c_int as libc::c_ulong)
                .wrapping_add(strlen(postedname)),
        ) as *mut libc::c_char;
        sprintf(
            promptstr,
            question,
            postedname,
            lockuser.as_mut_ptr(),
            lockprog.as_mut_ptr(),
            pidstring,
        );
        rpl_free(postedname as *mut libc::c_void);
        rpl_free(pidstring as *mut libc::c_void);
        choice = ask_user(0 as libc::c_int != 0, promptstr);
        rpl_free(promptstr as *mut libc::c_void);
        if choice == -(1 as libc::c_int) && !we_are_running {
            finish();
        }
        if choice != 1 as libc::c_int {
            rpl_free(lockfilename as *mut libc::c_void);
            wipe_statusbar();
            return -(1 as libc::c_int) as *mut libc::c_char;
        }
    }
    if write_lockfile(lockfilename, filename, 0 as libc::c_int != 0) {
        return lockfilename;
    }
    rpl_free(lockfilename as *mut libc::c_void);
    return 0 as *mut libc::c_char;
}
pub unsafe extern "C" fn stat_with_alloc(
    mut filename: *const libc::c_char,
    mut pstat: *mut *mut stat,
) {
    if (*pstat).is_null() {
        *pstat = nmalloc(::std::mem::size_of::<stat>() as libc::c_ulong) as *mut stat;
    }
    if stat(filename, *pstat) != 0 as libc::c_int {
        rpl_free(*pstat as *mut libc::c_void);
        *pstat = 0 as *mut stat;
    }
}
pub unsafe extern "C" fn has_valid_path(mut filename: *const libc::c_char) -> bool {
    let mut namecopy: *mut libc::c_char = copy_of(filename);
    let mut parentdir: *mut libc::c_char = dirname(namecopy);
    let mut parentinfo: stat = stat {
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
    let mut validity: bool = 0 as libc::c_int != 0;
    let mut gone: bool = 0 as libc::c_int != 0;
    if strcmp(parentdir, b".\0" as *const u8 as *const libc::c_char) == 0 as libc::c_int
    {
        let mut currentdir: *mut libc::c_char = realpath(
            b".\0" as *const u8 as *const libc::c_char,
            0 as *mut libc::c_char,
        );
        gone = currentdir.is_null() && *__errno_location() == 2 as libc::c_int;
        rpl_free(currentdir as *mut libc::c_void);
    }
    if gone {
        statusline(
            ALERT,
            dcgettext(
                0 as *const libc::c_char,
                b"The working directory has disappeared\0" as *const u8
                    as *const libc::c_char,
                5 as libc::c_int,
            ),
        );
    } else if stat(parentdir, &mut parentinfo) == -(1 as libc::c_int) {
        if *__errno_location() == 2 as libc::c_int {
            statusline(
                ALERT,
                dcgettext(
                    0 as *const libc::c_char,
                    b"Directory '%s' does not exist\0" as *const u8
                        as *const libc::c_char,
                    5 as libc::c_int,
                ),
                parentdir,
            );
        } else {
            statusline(
                ALERT,
                dcgettext(
                    0 as *const libc::c_char,
                    b"Path '%s': %s\0" as *const u8 as *const libc::c_char,
                    5 as libc::c_int,
                ),
                parentdir,
                strerror(*__errno_location()),
            );
        }
    } else if !(parentinfo.st_mode & 0o170000 as libc::c_int as libc::c_uint
        == 0o40000 as libc::c_int as libc::c_uint)
    {
        statusline(
            ALERT,
            dcgettext(
                0 as *const libc::c_char,
                b"Path '%s' is not a directory\0" as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ),
            parentdir,
        );
    } else if access(parentdir, 1 as libc::c_int) == -(1 as libc::c_int) {
        statusline(
            ALERT,
            dcgettext(
                0 as *const libc::c_char,
                b"Path '%s' is not accessible\0" as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ),
            parentdir,
        );
    } else if flags[(LOCKING as libc::c_int as libc::c_ulong)
        .wrapping_div(
            (::std::mem::size_of::<libc::c_uint>() as libc::c_ulong)
                .wrapping_mul(8 as libc::c_int as libc::c_ulong),
        ) as usize]
        & (1 as libc::c_int as libc::c_uint)
            << (LOCKING as libc::c_int as libc::c_ulong)
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
        && access(parentdir, 2 as libc::c_int) < 0 as libc::c_int
    {
        statusline(
            MILD,
            dcgettext(
                0 as *const libc::c_char,
                b"Directory '%s' is not writable\0" as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ),
            parentdir,
        );
    } else {
        validity = 1 as libc::c_int != 0;
    }
    rpl_free(namecopy as *mut libc::c_void);
    return validity;
}
pub unsafe extern "C" fn open_buffer(
    mut filename: *const libc::c_char,
    mut new_one: bool,
) -> bool {
    let mut realname: *mut libc::c_char = 0 as *mut libc::c_char;
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
    let mut descriptor: libc::c_int = 0 as libc::c_int;
    let mut f: *mut FILE = 0 as *mut FILE;
    as_an_at = 0 as libc::c_int != 0;
    if outside_of_confinement(filename, 0 as libc::c_int != 0) {
        statusline(
            ALERT,
            dcgettext(
                0 as *const libc::c_char,
                b"Can't read file from outside of %s\0" as *const u8
                    as *const libc::c_char,
                5 as libc::c_int,
            ),
            operating_dir,
        );
        return 0 as libc::c_int != 0;
    }
    realname = real_dir_from_tilde(filename);
    if *filename as libc::c_int != '\0' as i32
        && stat(realname, &mut fileinfo) == 0 as libc::c_int
    {
        if fileinfo.st_mode & 0o170000 as libc::c_int as libc::c_uint
            == 0o40000 as libc::c_int as libc::c_uint
        {
            statusline(
                ALERT,
                dcgettext(
                    0 as *const libc::c_char,
                    b"\"%s\" is a directory\0" as *const u8 as *const libc::c_char,
                    5 as libc::c_int,
                ),
                realname,
            );
            rpl_free(realname as *mut libc::c_void);
            return 0 as libc::c_int != 0;
        }
        if fileinfo.st_mode & 0o170000 as libc::c_int as libc::c_uint
            == 0o20000 as libc::c_int as libc::c_uint
            || fileinfo.st_mode & 0o170000 as libc::c_int as libc::c_uint
                == 0o60000 as libc::c_int as libc::c_uint
        {
            statusline(
                ALERT,
                dcgettext(
                    0 as *const libc::c_char,
                    b"\"%s\" is a device file\0" as *const u8 as *const libc::c_char,
                    5 as libc::c_int,
                ),
                realname,
            );
            rpl_free(realname as *mut libc::c_void);
            return 0 as libc::c_int != 0;
        }
        if new_one as libc::c_int != 0
            && fileinfo.st_mode
                & (0o200 as libc::c_int | 0o200 as libc::c_int >> 3 as libc::c_int
                    | 0o200 as libc::c_int >> 3 as libc::c_int >> 3 as libc::c_int)
                    as libc::c_uint == 0 && geteuid() == 0 as libc::c_int as libc::c_uint
        {
            statusline(
                ALERT,
                dcgettext(
                    0 as *const libc::c_char,
                    b"%s is meant to be read-only\0" as *const u8 as *const libc::c_char,
                    5 as libc::c_int,
                ),
                realname,
            );
        }
    }
    if new_one {
        make_new_buffer();
        if has_valid_path(realname) {
            if flags[(LOCKING as libc::c_int as libc::c_ulong)
                .wrapping_div(
                    (::std::mem::size_of::<libc::c_uint>() as libc::c_ulong)
                        .wrapping_mul(8 as libc::c_int as libc::c_ulong),
                ) as usize]
                & (1 as libc::c_int as libc::c_uint)
                    << (LOCKING as libc::c_int as libc::c_ulong)
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
                && *filename.offset(0 as libc::c_int as isize) as libc::c_int
                    != '\0' as i32
            {
                let mut thelocksname: *mut libc::c_char = do_lockfile(
                    realname,
                    1 as libc::c_int != 0,
                );
                if thelocksname == -(1 as libc::c_int) as *mut libc::c_char {
                    close_buffer();
                    rpl_free(realname as *mut libc::c_void);
                    return 0 as libc::c_int != 0;
                } else {
                    (*openfile).lock_filename = thelocksname;
                }
            }
        }
    }
    if *filename.offset(0 as libc::c_int as isize) as libc::c_int != '\0' as i32
        && !(flags[(NOREAD_MODE as libc::c_int as libc::c_ulong)
            .wrapping_div(
                (::std::mem::size_of::<libc::c_uint>() as libc::c_ulong)
                    .wrapping_mul(8 as libc::c_int as libc::c_ulong),
            ) as usize]
            & (1 as libc::c_int as libc::c_uint)
                << (NOREAD_MODE as libc::c_int as libc::c_ulong)
                    .wrapping_rem(
                        (::std::mem::size_of::<libc::c_uint>() as libc::c_ulong)
                            .wrapping_mul(8 as libc::c_int as libc::c_ulong),
                    ) != 0 as libc::c_int as libc::c_uint)
    {
        descriptor = open_file(realname, new_one, &mut f);
    }
    if descriptor > 0 as libc::c_int {
        install_handler_for_Ctrl_C();
        read_file(f, descriptor, realname, !new_one);
        restore_handler_for_Ctrl_C();
        if ((*openfile).statinfo).is_null() {
            stat_with_alloc(realname, &mut (*openfile).statinfo);
        }
    }
    if descriptor >= 0 as libc::c_int && new_one as libc::c_int != 0 {
        (*openfile).filename = mallocstrcpy((*openfile).filename, realname);
        (*openfile).current = (*openfile).filetop;
        (*openfile).current_x = 0 as libc::c_int as size_t;
        (*openfile).placewewant = 0 as libc::c_int as size_t;
    }
    if new_one {
        find_and_prime_applicable_syntax();
    }
    rpl_free(realname as *mut libc::c_void);
    return 1 as libc::c_int != 0;
}
pub unsafe extern "C" fn set_modified() {
    if (*openfile).modified {
        return;
    }
    (*openfile).modified = 1 as libc::c_int != 0;
    titlebar(0 as *const libc::c_char);
    if !((*openfile).lock_filename).is_null() {
        write_lockfile(
            (*openfile).lock_filename,
            (*openfile).filename,
            1 as libc::c_int != 0,
        );
    }
}
pub unsafe extern "C" fn prepare_for_display() {
    if !inhelp {
        titlebar(0 as *const libc::c_char);
    }
    if ((*(*openfile).filetop).multidata).is_null() {
        precalc_multicolorinfo();
    }
    have_palette = 0 as libc::c_int != 0;
    refresh_needed = 1 as libc::c_int != 0;
}
pub unsafe extern "C" fn mention_name_and_linecount() {
    let mut count: size_t = ((*(*openfile).filebot).lineno
        - (if *((*(*openfile).filebot).data).offset(0 as libc::c_int as isize)
            as libc::c_int == '\0' as i32
        {
            1 as libc::c_int
        } else {
            0 as libc::c_int
        }) as libc::c_long) as size_t;
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
    {
        report_size = 1 as libc::c_int != 0;
        return;
    }
    if (*openfile).fmt as libc::c_uint > NIX_FILE as libc::c_int as libc::c_uint {
        statusline(
            HUSH,
            dcngettext(
                0 as *const libc::c_char,
                b"%s -- %zu line (%s)\0" as *const u8 as *const libc::c_char,
                b"%s -- %zu lines (%s)\0" as *const u8 as *const libc::c_char,
                count,
                5 as libc::c_int,
            ),
            if *((*openfile).filename).offset(0 as libc::c_int as isize) as libc::c_int
                == '\0' as i32
            {
                dcgettext(
                    0 as *const libc::c_char,
                    b"New Buffer\0" as *const u8 as *const libc::c_char,
                    5 as libc::c_int,
                ) as *const libc::c_char
            } else {
                tail((*openfile).filename)
            },
            count,
            if (*openfile).fmt as libc::c_uint == DOS_FILE as libc::c_int as libc::c_uint
            {
                dcgettext(
                    0 as *const libc::c_char,
                    b"DOS\0" as *const u8 as *const libc::c_char,
                    5 as libc::c_int,
                )
            } else {
                dcgettext(
                    0 as *const libc::c_char,
                    b"Mac\0" as *const u8 as *const libc::c_char,
                    5 as libc::c_int,
                )
            },
        );
    } else {
        statusline(
            HUSH,
            dcngettext(
                0 as *const libc::c_char,
                b"%s -- %zu line\0" as *const u8 as *const libc::c_char,
                b"%s -- %zu lines\0" as *const u8 as *const libc::c_char,
                count,
                5 as libc::c_int,
            ),
            if *((*openfile).filename).offset(0 as libc::c_int as isize) as libc::c_int
                == '\0' as i32
            {
                dcgettext(
                    0 as *const libc::c_char,
                    b"New Buffer\0" as *const u8 as *const libc::c_char,
                    5 as libc::c_int,
                ) as *const libc::c_char
            } else {
                tail((*openfile).filename)
            },
            count,
        );
    };
}
pub unsafe extern "C" fn redecorate_after_switch() {
    if openfile == (*openfile).next {
        statusline(
            AHEM,
            dcgettext(
                0 as *const libc::c_char,
                b"No more open file buffers\0" as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ),
        );
        return;
    }
    ensure_firstcolumn_is_aligned();
    prepare_for_display();
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
    if !((*openfile).errormessage).is_null() {
        statusline(ALERT, (*openfile).errormessage);
        rpl_free((*openfile).errormessage as *mut libc::c_void);
        (*openfile).errormessage = 0 as *mut libc::c_char;
    } else {
        mention_name_and_linecount();
    };
}
pub unsafe extern "C" fn switch_to_prev_buffer() {
    openfile = (*openfile).prev;
    redecorate_after_switch();
}
pub unsafe extern "C" fn switch_to_next_buffer() {
    openfile = (*openfile).next;
    redecorate_after_switch();
}
pub unsafe extern "C" fn close_buffer() {
    let mut orphan: *mut openfilestruct = openfile;
    if orphan == startfile {
        startfile = (*startfile).next;
    }
    (*(*orphan).prev).next = (*orphan).next;
    (*(*orphan).next).prev = (*orphan).prev;
    rpl_free((*orphan).filename as *mut libc::c_void);
    free_lines((*orphan).filetop);
    rpl_free((*orphan).statinfo as *mut libc::c_void);
    rpl_free((*orphan).lock_filename as *mut libc::c_void);
    discard_until(0 as *const undostruct);
    rpl_free((*orphan).errormessage as *mut libc::c_void);
    openfile = (*orphan).prev;
    if openfile == orphan {
        openfile = 0 as *mut openfilestruct;
    }
    rpl_free(orphan as *mut libc::c_void);
    if !openfile.is_null() && openfile == (*openfile).next {
        (*exitfunc).tag = exit_tag;
    }
}
pub unsafe extern "C" fn encode_data(
    mut text: *mut libc::c_char,
    mut length: size_t,
) -> *mut libc::c_char {
    recode_NUL_to_LF(text, length);
    *text.offset(length as isize) = '\0' as i32 as libc::c_char;
    return copy_of(text);
}
pub unsafe extern "C" fn read_file(
    mut f: *mut FILE,
    mut fd: libc::c_int,
    mut filename: *const libc::c_char,
    mut undoable: bool,
) {
    let mut was_lineno: ssize_t = (*(*openfile).current).lineno;
    let mut was_leftedge: size_t = 0 as libc::c_int as size_t;
    let mut num_lines: size_t = 0 as libc::c_int as size_t;
    let mut len: size_t = 0 as libc::c_int as size_t;
    let mut bufsize: size_t = 120 as libc::c_int as size_t;
    let mut buf: *mut libc::c_char = nmalloc(bufsize) as *mut libc::c_char;
    let mut topline: *mut linestruct = 0 as *mut linestruct;
    let mut bottomline: *mut linestruct = 0 as *mut linestruct;
    let mut onevalue: libc::c_int = 0;
    let mut errornumber: libc::c_int = 0;
    let mut writable: bool = 1 as libc::c_int != 0;
    let mut format: format_type = NIX_FILE;
    if undoable {
        add_undo(INSERT, 0 as *const libc::c_char);
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
        was_leftedge = leftedge_for(xplustabs(), (*openfile).current);
    }
    topline = make_new_node(0 as *mut linestruct);
    bottomline = topline;
    block_sigwinch(1 as libc::c_int != 0);
    flockfile(f);
    control_C_was_pressed = 0 as libc::c_int != 0;
    loop {
        onevalue = getc_unlocked(f);
        if !(onevalue != -(1 as libc::c_int)) {
            break;
        }
        let mut input: libc::c_char = onevalue as libc::c_char;
        if control_C_was_pressed {
            break;
        }
        if input as libc::c_int == '\n' as i32 {
            if len > 0 as libc::c_int as libc::c_ulong
                && *buf
                    .offset(len.wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize)
                    as libc::c_int == '\r' as i32
                && !(flags[(NO_CONVERT as libc::c_int as libc::c_ulong)
                    .wrapping_div(
                        (::std::mem::size_of::<libc::c_uint>() as libc::c_ulong)
                            .wrapping_mul(8 as libc::c_int as libc::c_ulong),
                    ) as usize]
                    & (1 as libc::c_int as libc::c_uint)
                        << (NO_CONVERT as libc::c_int as libc::c_ulong)
                            .wrapping_rem(
                                (::std::mem::size_of::<libc::c_uint>() as libc::c_ulong)
                                    .wrapping_mul(8 as libc::c_int as libc::c_ulong),
                            ) != 0 as libc::c_int as libc::c_uint)
            {
                if num_lines == 0 as libc::c_int as libc::c_ulong {
                    format = DOS_FILE;
                }
                len = len.wrapping_sub(1);
                len;
            }
        } else if (num_lines == 0 as libc::c_int as libc::c_ulong
            || format as libc::c_uint == MAC_FILE as libc::c_int as libc::c_uint)
            && len > 0 as libc::c_int as libc::c_ulong
            && *buf.offset(len.wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize)
                as libc::c_int == '\r' as i32
            && !(flags[(NO_CONVERT as libc::c_int as libc::c_ulong)
                .wrapping_div(
                    (::std::mem::size_of::<libc::c_uint>() as libc::c_ulong)
                        .wrapping_mul(8 as libc::c_int as libc::c_ulong),
                ) as usize]
                & (1 as libc::c_int as libc::c_uint)
                    << (NO_CONVERT as libc::c_int as libc::c_ulong)
                        .wrapping_rem(
                            (::std::mem::size_of::<libc::c_uint>() as libc::c_ulong)
                                .wrapping_mul(8 as libc::c_int as libc::c_ulong),
                        ) != 0 as libc::c_int as libc::c_uint)
        {
            format = MAC_FILE;
            len = len.wrapping_sub(1);
            len;
        } else {
            *buf.offset(len as isize) = input;
            len = len.wrapping_add(1);
            len;
            if len == bufsize {
                bufsize = (bufsize as libc::c_ulong)
                    .wrapping_add(120 as libc::c_int as libc::c_ulong) as size_t
                    as size_t;
                buf = nrealloc(buf as *mut libc::c_void, bufsize) as *mut libc::c_char;
            }
            continue;
        }
        (*bottomline).data = encode_data(buf, len);
        (*bottomline).next = make_new_node(bottomline);
        bottomline = (*bottomline).next;
        num_lines = num_lines.wrapping_add(1);
        num_lines;
        len = 0 as libc::c_int as size_t;
        if input as libc::c_int != '\n' as i32 {
            let fresh1 = len;
            len = len.wrapping_add(1);
            *buf.offset(fresh1 as isize) = input;
        }
    }
    errornumber = *__errno_location();
    funlockfile(f);
    block_sigwinch(0 as libc::c_int != 0);
    if isendwin() {
        if isatty(0 as libc::c_int) == 0 {
            reconnect_and_store_state();
        }
        terminal_init();
        doupdate();
    }
    if ferror(f) != 0 && errornumber != 4 as libc::c_int
        && errornumber != 0 as libc::c_int
    {
        statusline(ALERT, strerror(errornumber));
    }
    if control_C_was_pressed {
        statusline(
            ALERT,
            dcgettext(
                0 as *const libc::c_char,
                b"Interrupted\0" as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ),
        );
    }
    fclose(f);
    if fd > 0 as libc::c_int && !undoable
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
        writable = access(filename, 2 as libc::c_int) == 0 as libc::c_int;
    }
    if len == 0 as libc::c_int as libc::c_ulong {
        (*bottomline).data = copy_of(b"\0" as *const u8 as *const libc::c_char);
    } else {
        let mut mac_line_needs_newline: bool = 0 as libc::c_int != 0;
        if *buf.offset(len.wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize)
            as libc::c_int == '\r' as i32
            && !(flags[(NO_CONVERT as libc::c_int as libc::c_ulong)
                .wrapping_div(
                    (::std::mem::size_of::<libc::c_uint>() as libc::c_ulong)
                        .wrapping_mul(8 as libc::c_int as libc::c_ulong),
                ) as usize]
                & (1 as libc::c_int as libc::c_uint)
                    << (NO_CONVERT as libc::c_int as libc::c_ulong)
                        .wrapping_rem(
                            (::std::mem::size_of::<libc::c_uint>() as libc::c_ulong)
                                .wrapping_mul(8 as libc::c_int as libc::c_ulong),
                        ) != 0 as libc::c_int as libc::c_uint)
        {
            if num_lines == 0 as libc::c_int as libc::c_ulong {
                format = MAC_FILE;
            }
            len = len.wrapping_sub(1);
            *buf.offset(len as isize) = '\0' as i32 as libc::c_char;
            mac_line_needs_newline = 1 as libc::c_int != 0;
        }
        (*bottomline).data = encode_data(buf, len);
        num_lines = num_lines.wrapping_add(1);
        num_lines;
        if mac_line_needs_newline {
            (*bottomline).next = make_new_node(bottomline);
            bottomline = (*bottomline).next;
            (*bottomline).data = copy_of(b"\0" as *const u8 as *const libc::c_char);
        }
    }
    rpl_free(buf as *mut libc::c_void);
    ingraft_buffer(topline);
    (*openfile).placewewant = xplustabs();
    if !writable {
        statusline(
            ALERT,
            dcgettext(
                0 as *const libc::c_char,
                b"File '%s' is unwritable\0" as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ),
            filename,
        );
    } else if format as libc::c_uint == MAC_FILE as libc::c_int as libc::c_uint {
        statusline(
            REMARK,
            dcngettext(
                0 as *const libc::c_char,
                b"Read %zu line (Converted from Mac format)\0" as *const u8
                    as *const libc::c_char,
                b"Read %zu lines (Converted from Mac format)\0" as *const u8
                    as *const libc::c_char,
                num_lines,
                5 as libc::c_int,
            ),
            num_lines,
        );
    } else if format as libc::c_uint == DOS_FILE as libc::c_int as libc::c_uint {
        statusline(
            REMARK,
            dcngettext(
                0 as *const libc::c_char,
                b"Read %zu line (Converted from DOS format)\0" as *const u8
                    as *const libc::c_char,
                b"Read %zu lines (Converted from DOS format)\0" as *const u8
                    as *const libc::c_char,
                num_lines,
                5 as libc::c_int,
            ),
            num_lines,
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
        || we_are_running as libc::c_int != 0 && undoable as libc::c_int != 0
    {
        statusline(
            REMARK,
            dcngettext(
                0 as *const libc::c_char,
                b"Read %zu line\0" as *const u8 as *const libc::c_char,
                b"Read %zu lines\0" as *const u8 as *const libc::c_char,
                num_lines,
                5 as libc::c_int,
            ),
            num_lines,
        );
    }
    report_size = 1 as libc::c_int != 0;
    if undoable as libc::c_int != 0
        && less_than_a_screenful(was_lineno as size_t, was_leftedge) as libc::c_int != 0
    {
        focusing = 0 as libc::c_int != 0;
        perturbed = 1 as libc::c_int != 0;
    } else if undoable {
        recook = 1 as libc::c_int != 0;
    }
    if undoable {
        update_undo(INSERT);
    }
    if flags[(MAKE_IT_UNIX as libc::c_int as libc::c_ulong)
        .wrapping_div(
            (::std::mem::size_of::<libc::c_uint>() as libc::c_ulong)
                .wrapping_mul(8 as libc::c_int as libc::c_ulong),
        ) as usize]
        & (1 as libc::c_int as libc::c_uint)
            << (MAKE_IT_UNIX as libc::c_int as libc::c_ulong)
                .wrapping_rem(
                    (::std::mem::size_of::<libc::c_uint>() as libc::c_ulong)
                        .wrapping_mul(8 as libc::c_int as libc::c_ulong),
                ) != 0 as libc::c_int as libc::c_uint
    {
        (*openfile).fmt = NIX_FILE;
    } else if (*openfile).fmt as libc::c_uint
        == UNSPECIFIED as libc::c_int as libc::c_uint
    {
        (*openfile).fmt = format;
    }
}
pub unsafe extern "C" fn open_file(
    mut filename: *const libc::c_char,
    mut new_one: bool,
    mut f: *mut *mut FILE,
) -> libc::c_int {
    let mut full_filename: *mut libc::c_char = get_full_path(filename);
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
    let mut fd: libc::c_int = 0;
    if full_filename.is_null()
        || stat(full_filename, &mut fileinfo) == -(1 as libc::c_int)
    {
        full_filename = mallocstrcpy(full_filename, filename);
    }
    if stat(full_filename, &mut fileinfo) == -(1 as libc::c_int) {
        rpl_free(full_filename as *mut libc::c_void);
        if new_one {
            statusline(
                REMARK,
                dcgettext(
                    0 as *const libc::c_char,
                    b"New File\0" as *const u8 as *const libc::c_char,
                    5 as libc::c_int,
                ),
            );
            return 0 as libc::c_int;
        } else {
            statusline(
                ALERT,
                dcgettext(
                    0 as *const libc::c_char,
                    b"File \"%s\" not found\0" as *const u8 as *const libc::c_char,
                    5 as libc::c_int,
                ),
                filename,
            );
            return -(1 as libc::c_int);
        }
    }
    if fileinfo.st_mode & 0o170000 as libc::c_int as libc::c_uint
        == 0o10000 as libc::c_int as libc::c_uint
    {
        statusbar(
            dcgettext(
                0 as *const libc::c_char,
                b"Reading from FIFO...\0" as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ),
        );
    }
    block_sigwinch(1 as libc::c_int != 0);
    install_handler_for_Ctrl_C();
    fd = open(full_filename, 0 as libc::c_int);
    restore_handler_for_Ctrl_C();
    block_sigwinch(0 as libc::c_int != 0);
    if fd == -(1 as libc::c_int) {
        if *__errno_location() == 4 as libc::c_int
            || *__errno_location() == 0 as libc::c_int
        {
            statusline(
                ALERT,
                dcgettext(
                    0 as *const libc::c_char,
                    b"Interrupted\0" as *const u8 as *const libc::c_char,
                    5 as libc::c_int,
                ),
            );
        } else {
            statusline(
                ALERT,
                dcgettext(
                    0 as *const libc::c_char,
                    b"Error reading %s: %s\0" as *const u8 as *const libc::c_char,
                    5 as libc::c_int,
                ),
                filename,
                strerror(*__errno_location()),
            );
        }
    } else {
        *f = fdopen(fd, b"rb\0" as *const u8 as *const libc::c_char);
        if (*f).is_null() {
            statusline(
                ALERT,
                dcgettext(
                    0 as *const libc::c_char,
                    b"Error reading %s: %s\0" as *const u8 as *const libc::c_char,
                    5 as libc::c_int,
                ),
                filename,
                strerror(*__errno_location()),
            );
            close(fd);
            fd = -(1 as libc::c_int);
        } else if !(flags[(ZERO as libc::c_int as libc::c_ulong)
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
            || we_are_running as libc::c_int != 0
        {
            statusbar(
                dcgettext(
                    0 as *const libc::c_char,
                    b"Reading...\0" as *const u8 as *const libc::c_char,
                    5 as libc::c_int,
                ),
            );
        }
    }
    rpl_free(full_filename as *mut libc::c_void);
    return fd;
}
pub unsafe extern "C" fn get_next_filename(
    mut name: *const libc::c_char,
    mut suffix: *const libc::c_char,
) -> *mut libc::c_char {
    let mut wholenamelen: size_t = (strlen(name)).wrapping_add(strlen(suffix));
    let mut i: libc::c_ulong = 0 as libc::c_int as libc::c_ulong;
    let mut buf: *mut libc::c_char = 0 as *mut libc::c_char;
    buf = nmalloc(wholenamelen.wrapping_add(7 as libc::c_int as libc::c_ulong))
        as *mut libc::c_char;
    sprintf(buf, b"%s%s\0" as *const u8 as *const libc::c_char, name, suffix);
    loop {
        let mut fs: stat = stat {
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
        if stat(buf, &mut fs) == -(1 as libc::c_int) {
            return buf;
        }
        i = i.wrapping_add(1);
        if i == 100000 as libc::c_int as libc::c_ulong {
            break;
        }
        sprintf(
            buf.offset(wholenamelen as isize),
            b".%lu\0" as *const u8 as *const libc::c_char,
            i,
        );
    }
    *buf = '\0' as i32 as libc::c_char;
    return buf;
}
static mut pid_of_command: pid_t = -(1 as libc::c_int);
static mut pid_of_sender: pid_t = -(1 as libc::c_int);
static mut should_pipe: bool = 0 as libc::c_int != 0;
pub unsafe extern "C" fn cancel_the_command(mut signal: libc::c_int) {
    if pid_of_command > 0 as libc::c_int {
        kill(pid_of_command, 9 as libc::c_int);
    }
    if should_pipe as libc::c_int != 0 && pid_of_sender > 0 as libc::c_int {
        kill(pid_of_sender, 9 as libc::c_int);
    }
}
pub unsafe extern "C" fn send_data(mut line: *const linestruct, mut fd: libc::c_int) {
    let mut tube: *mut FILE = fdopen(fd, b"w\0" as *const u8 as *const libc::c_char);
    if tube.is_null() {
        exit(4 as libc::c_int);
    }
    while !line.is_null()
        && (!((*line).next).is_null()
            || *((*line).data).offset(0 as libc::c_int as isize) as libc::c_int
                != '\0' as i32)
    {
        let mut length: size_t = recode_LF_to_NUL((*line).data);
        if fwrite(
            (*line).data as *const libc::c_void,
            1 as libc::c_int as libc::c_ulong,
            length,
            tube,
        ) < length
        {
            exit(5 as libc::c_int);
        }
        if !((*line).next).is_null() && putc('\n' as i32, tube) == -(1 as libc::c_int) {
            exit(6 as libc::c_int);
        }
        line = (*line).next;
    }
    fclose(tube);
}
pub unsafe extern "C" fn execute_command(mut command: *const libc::c_char) {
    let mut from_fd: [libc::c_int; 2] = [0; 2];
    let mut to_fd: [libc::c_int; 2] = [0; 2];
    let mut oldaction: sigaction = sigaction {
        __sigaction_handler: C2RustUnnamed_9 {
            sa_handler: None,
        },
        sa_mask: __sigset_t { __val: [0; 16] },
        sa_flags: 0,
        sa_restorer: None,
    };
    let mut newaction: sigaction = {
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
    let mut was_lineno: ssize_t = if !((*openfile).mark).is_null() {
        0 as libc::c_int as libc::c_long
    } else {
        (*(*openfile).current).lineno
    };
    let mut command_status: libc::c_int = 0;
    let mut sender_status: libc::c_int = 0;
    let mut stream: *mut FILE = 0 as *mut FILE;
    should_pipe = *command.offset(0 as libc::c_int as isize) as libc::c_int
        == '|' as i32;
    if pipe(from_fd.as_mut_ptr()) == -(1 as libc::c_int)
        || should_pipe as libc::c_int != 0
            && pipe(to_fd.as_mut_ptr()) == -(1 as libc::c_int)
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
    pid_of_command = fork();
    if pid_of_command == 0 as libc::c_int {
        let mut theshell: *const libc::c_char = getenv(
            b"SHELL\0" as *const u8 as *const libc::c_char,
        );
        if theshell.is_null() {
            theshell = b"/bin/sh\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char;
        }
        close(from_fd[0 as libc::c_int as usize]);
        if dup2(from_fd[1 as libc::c_int as usize], 1 as libc::c_int) < 0 as libc::c_int
        {
            exit(3 as libc::c_int);
        }
        if dup2(from_fd[1 as libc::c_int as usize], 2 as libc::c_int) < 0 as libc::c_int
        {
            exit(4 as libc::c_int);
        }
        if should_pipe {
            if dup2(to_fd[0 as libc::c_int as usize], 0 as libc::c_int)
                < 0 as libc::c_int
            {
                exit(5 as libc::c_int);
            }
            close(from_fd[1 as libc::c_int as usize]);
            close(to_fd[1 as libc::c_int as usize]);
        }
        execl(
            theshell,
            tail(theshell),
            b"-c\0" as *const u8 as *const libc::c_char,
            if should_pipe as libc::c_int != 0 {
                &*command.offset(1 as libc::c_int as isize) as *const libc::c_char
            } else {
                command
            },
            0 as *mut libc::c_void,
        );
        exit(6 as libc::c_int);
    }
    close(from_fd[1 as libc::c_int as usize]);
    if pid_of_command == -(1 as libc::c_int) {
        statusline(
            ALERT,
            dcgettext(
                0 as *const libc::c_char,
                b"Could not fork: %s\0" as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ),
            strerror(*__errno_location()),
        );
        close(from_fd[0 as libc::c_int as usize]);
        return;
    }
    statusbar(
        dcgettext(
            0 as *const libc::c_char,
            b"Executing...\0" as *const u8 as *const libc::c_char,
            5 as libc::c_int,
        ),
    );
    if should_pipe {
        let mut was_cutbuffer: *mut linestruct = cutbuffer;
        let mut whole_buffer: bool = 0 as libc::c_int != 0;
        cutbuffer = 0 as *mut linestruct;
        if flags[(MULTIBUFFER as libc::c_int as libc::c_ulong)
            .wrapping_div(
                (::std::mem::size_of::<libc::c_uint>() as libc::c_ulong)
                    .wrapping_mul(8 as libc::c_int as libc::c_ulong),
            ) as usize]
            & (1 as libc::c_int as libc::c_uint)
                << (MULTIBUFFER as libc::c_int as libc::c_ulong)
                    .wrapping_rem(
                        (::std::mem::size_of::<libc::c_uint>() as libc::c_ulong)
                            .wrapping_mul(8 as libc::c_int as libc::c_ulong),
                    ) != 0 as libc::c_int as libc::c_uint
        {
            openfile = (*openfile).prev;
            if !((*openfile).mark).is_null() {
                copy_marked_region();
            } else {
                whole_buffer = 1 as libc::c_int != 0;
            }
        } else {
            add_undo(COUPLE_BEGIN, b"filtering\0" as *const u8 as *const libc::c_char);
            if ((*openfile).mark).is_null() {
                (*openfile).current = (*openfile).filetop;
                (*openfile).current_x = 0 as libc::c_int as size_t;
            }
            add_undo(CUT, 0 as *const libc::c_char);
            do_snip(
                !((*openfile).mark).is_null(),
                ((*openfile).mark).is_null(),
                0 as libc::c_int != 0,
            );
            if ((*(*openfile).filetop).next).is_null() {
                (*(*openfile).filetop).has_anchor = 0 as libc::c_int != 0;
            }
            update_undo(CUT);
        }
        pid_of_sender = fork();
        if pid_of_sender == 0 as libc::c_int {
            send_data(
                if whole_buffer as libc::c_int != 0 {
                    (*openfile).filetop
                } else {
                    cutbuffer
                },
                to_fd[1 as libc::c_int as usize],
            );
            exit(0 as libc::c_int);
        }
        if pid_of_sender == -(1 as libc::c_int) {
            statusline(
                ALERT,
                dcgettext(
                    0 as *const libc::c_char,
                    b"Could not fork: %s\0" as *const u8 as *const libc::c_char,
                    5 as libc::c_int,
                ),
                strerror(*__errno_location()),
            );
        }
        close(to_fd[0 as libc::c_int as usize]);
        close(to_fd[1 as libc::c_int as usize]);
        if flags[(MULTIBUFFER as libc::c_int as libc::c_ulong)
            .wrapping_div(
                (::std::mem::size_of::<libc::c_uint>() as libc::c_ulong)
                    .wrapping_mul(8 as libc::c_int as libc::c_ulong),
            ) as usize]
            & (1 as libc::c_int as libc::c_uint)
                << (MULTIBUFFER as libc::c_int as libc::c_ulong)
                    .wrapping_rem(
                        (::std::mem::size_of::<libc::c_uint>() as libc::c_ulong)
                            .wrapping_mul(8 as libc::c_int as libc::c_ulong),
                    ) != 0 as libc::c_int as libc::c_uint
        {
            openfile = (*openfile).next;
        }
        free_lines(cutbuffer);
        cutbuffer = was_cutbuffer;
    }
    enable_kb_interrupt();
    newaction
        .__sigaction_handler
        .sa_handler = Some(
        cancel_the_command as unsafe extern "C" fn(libc::c_int) -> (),
    );
    newaction.sa_flags = 0 as libc::c_int;
    sigaction(2 as libc::c_int, &mut newaction, &mut oldaction);
    stream = fdopen(
        from_fd[0 as libc::c_int as usize],
        b"rb\0" as *const u8 as *const libc::c_char,
    );
    if stream.is_null() {
        statusline(
            ALERT,
            dcgettext(
                0 as *const libc::c_char,
                b"Failed to open pipe: %s\0" as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ),
            strerror(*__errno_location()),
        );
    } else {
        read_file(
            stream,
            0 as libc::c_int,
            b"pipe\0" as *const u8 as *const libc::c_char,
            1 as libc::c_int != 0,
        );
    }
    if should_pipe as libc::c_int != 0
        && !(flags[(MULTIBUFFER as libc::c_int as libc::c_ulong)
            .wrapping_div(
                (::std::mem::size_of::<libc::c_uint>() as libc::c_ulong)
                    .wrapping_mul(8 as libc::c_int as libc::c_ulong),
            ) as usize]
            & (1 as libc::c_int as libc::c_uint)
                << (MULTIBUFFER as libc::c_int as libc::c_ulong)
                    .wrapping_rem(
                        (::std::mem::size_of::<libc::c_uint>() as libc::c_ulong)
                            .wrapping_mul(8 as libc::c_int as libc::c_ulong),
                    ) != 0 as libc::c_int as libc::c_uint)
    {
        if was_lineno != 0 {
            goto_line_posx(was_lineno, 0 as libc::c_int as size_t);
        }
        add_undo(COUPLE_END, b"filtering\0" as *const u8 as *const libc::c_char);
    }
    waitpid(pid_of_command, &mut command_status, 0 as libc::c_int);
    if should_pipe as libc::c_int != 0 && pid_of_sender > 0 as libc::c_int {
        waitpid(pid_of_sender, &mut sender_status, 0 as libc::c_int);
    }
    if (command_status & 0x7f as libc::c_int == 0 as libc::c_int) as libc::c_int
        == 0 as libc::c_int
        || (command_status & 0xff00 as libc::c_int) >> 8 as libc::c_int != 0
    {
        statusline(
            ALERT,
            if ((command_status & 0x7f as libc::c_int) + 1 as libc::c_int)
                as libc::c_schar as libc::c_int >> 1 as libc::c_int > 0 as libc::c_int
            {
                dcgettext(
                    0 as *const libc::c_char,
                    b"Cancelled\0" as *const u8 as *const libc::c_char,
                    5 as libc::c_int,
                )
            } else {
                dcgettext(
                    0 as *const libc::c_char,
                    b"Error: %s\0" as *const u8 as *const libc::c_char,
                    5 as libc::c_int,
                )
            },
            if !((*(*openfile).current).prev).is_null()
                && !(strstr(
                    (*(*(*openfile).current).prev).data,
                    b": \0" as *const u8 as *const libc::c_char,
                ))
                    .is_null()
            {
                (strstr(
                    (*(*(*openfile).current).prev).data,
                    b": \0" as *const u8 as *const libc::c_char,
                ))
                    .offset(2 as libc::c_int as isize) as *const libc::c_char
            } else {
                b"---\0" as *const u8 as *const libc::c_char
            },
        );
    } else if should_pipe as libc::c_int != 0 && pid_of_sender > 0 as libc::c_int
        && ((sender_status & 0x7f as libc::c_int == 0 as libc::c_int) as libc::c_int
            == 0 as libc::c_int
            || (sender_status & 0xff00 as libc::c_int) >> 8 as libc::c_int != 0)
    {
        statusline(
            ALERT,
            dcgettext(
                0 as *const libc::c_char,
                b"Piping failed\0" as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ),
        );
    }
    if lastmessage as libc::c_uint == ALERT as libc::c_int as libc::c_uint {
        do_undo();
        discard_until((*openfile).current_undo);
    }
    sigaction(2 as libc::c_int, &mut oldaction, 0 as *mut sigaction);
    terminal_init();
}
pub unsafe extern "C" fn insert_a_file_or(mut execute: bool) {
    let mut response: libc::c_int = 0;
    let mut msg: *const libc::c_char = 0 as *const libc::c_char;
    let mut given: *mut libc::c_char = copy_of(
        b"\0" as *const u8 as *const libc::c_char,
    );
    let mut was_multibuffer: bool = flags[(MULTIBUFFER as libc::c_int as libc::c_ulong)
        .wrapping_div(
            (::std::mem::size_of::<libc::c_uint>() as libc::c_ulong)
                .wrapping_mul(8 as libc::c_int as libc::c_ulong),
        ) as usize]
        & (1 as libc::c_int as libc::c_uint)
            << (MULTIBUFFER as libc::c_int as libc::c_ulong)
                .wrapping_rem(
                    (::std::mem::size_of::<libc::c_uint>() as libc::c_ulong)
                        .wrapping_mul(8 as libc::c_int as libc::c_ulong),
                ) != 0 as libc::c_int as libc::c_uint;
    as_an_at = 0 as libc::c_int != 0;
    ran_a_tool = 0 as libc::c_int != 0;
    loop {
        if execute {
            if flags[(MULTIBUFFER as libc::c_int as libc::c_ulong)
                .wrapping_div(
                    (::std::mem::size_of::<libc::c_uint>() as libc::c_ulong)
                        .wrapping_mul(8 as libc::c_int as libc::c_ulong),
                ) as usize]
                & (1 as libc::c_int as libc::c_uint)
                    << (MULTIBUFFER as libc::c_int as libc::c_ulong)
                        .wrapping_rem(
                            (::std::mem::size_of::<libc::c_uint>() as libc::c_ulong)
                                .wrapping_mul(8 as libc::c_int as libc::c_ulong),
                        ) != 0 as libc::c_int as libc::c_uint
            {
                msg = dcgettext(
                    0 as *const libc::c_char,
                    b"Command to execute in new buffer\0" as *const u8
                        as *const libc::c_char,
                    5 as libc::c_int,
                );
            } else {
                msg = dcgettext(
                    0 as *const libc::c_char,
                    b"Command to execute\0" as *const u8 as *const libc::c_char,
                    5 as libc::c_int,
                );
            }
        } else if flags[(MULTIBUFFER as libc::c_int as libc::c_ulong)
            .wrapping_div(
                (::std::mem::size_of::<libc::c_uint>() as libc::c_ulong)
                    .wrapping_mul(8 as libc::c_int as libc::c_ulong),
            ) as usize]
            & (1 as libc::c_int as libc::c_uint)
                << (MULTIBUFFER as libc::c_int as libc::c_ulong)
                    .wrapping_rem(
                        (::std::mem::size_of::<libc::c_uint>() as libc::c_ulong)
                            .wrapping_mul(8 as libc::c_int as libc::c_ulong),
                    ) != 0 as libc::c_int as libc::c_uint
        {
            if flags[(NO_CONVERT as libc::c_int as libc::c_ulong)
                .wrapping_div(
                    (::std::mem::size_of::<libc::c_uint>() as libc::c_ulong)
                        .wrapping_mul(8 as libc::c_int as libc::c_ulong),
                ) as usize]
                & (1 as libc::c_int as libc::c_uint)
                    << (NO_CONVERT as libc::c_int as libc::c_ulong)
                        .wrapping_rem(
                            (::std::mem::size_of::<libc::c_uint>() as libc::c_ulong)
                                .wrapping_mul(8 as libc::c_int as libc::c_ulong),
                        ) != 0 as libc::c_int as libc::c_uint
            {
                msg = dcgettext(
                    0 as *const libc::c_char,
                    b"File to read unconverted into new buffer [from %s]\0" as *const u8
                        as *const libc::c_char,
                    5 as libc::c_int,
                );
            } else {
                msg = dcgettext(
                    0 as *const libc::c_char,
                    b"File to read into new buffer [from %s]\0" as *const u8
                        as *const libc::c_char,
                    5 as libc::c_int,
                );
            }
        } else if flags[(NO_CONVERT as libc::c_int as libc::c_ulong)
            .wrapping_div(
                (::std::mem::size_of::<libc::c_uint>() as libc::c_ulong)
                    .wrapping_mul(8 as libc::c_int as libc::c_ulong),
            ) as usize]
            & (1 as libc::c_int as libc::c_uint)
                << (NO_CONVERT as libc::c_int as libc::c_ulong)
                    .wrapping_rem(
                        (::std::mem::size_of::<libc::c_uint>() as libc::c_ulong)
                            .wrapping_mul(8 as libc::c_int as libc::c_ulong),
                    ) != 0 as libc::c_int as libc::c_uint
        {
            msg = dcgettext(
                0 as *const libc::c_char,
                b"File to insert unconverted [from %s]\0" as *const u8
                    as *const libc::c_char,
                5 as libc::c_int,
            );
        } else {
            msg = dcgettext(
                0 as *const libc::c_char,
                b"File to insert [from %s]\0" as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            );
        }
        present_path = mallocstrcpy(
            present_path,
            b"./\0" as *const u8 as *const libc::c_char,
        );
        response = do_prompt(
            if execute as libc::c_int != 0 {
                (1 as libc::c_int) << 7 as libc::c_int
            } else {
                (1 as libc::c_int) << 6 as libc::c_int
            },
            given,
            if execute as libc::c_int != 0 {
                &mut execute_history as *mut *mut linestruct
            } else {
                0 as *mut *mut linestruct
            },
            Some(edit_refresh as unsafe extern "C" fn() -> ()),
            msg,
            if !operating_dir.is_null() {
                operating_dir as *const libc::c_char
            } else {
                b"./\0" as *const u8 as *const libc::c_char
            },
        );
        if response == -(1 as libc::c_int)
            || response == -(2 as libc::c_int)
                && !(flags[(MULTIBUFFER as libc::c_int as libc::c_ulong)
                    .wrapping_div(
                        (::std::mem::size_of::<libc::c_uint>() as libc::c_ulong)
                            .wrapping_mul(8 as libc::c_int as libc::c_ulong),
                    ) as usize]
                    & (1 as libc::c_int as libc::c_uint)
                        << (MULTIBUFFER as libc::c_int as libc::c_ulong)
                            .wrapping_rem(
                                (::std::mem::size_of::<libc::c_uint>() as libc::c_ulong)
                                    .wrapping_mul(8 as libc::c_int as libc::c_ulong),
                            ) != 0 as libc::c_int as libc::c_uint)
        {
            statusbar(
                dcgettext(
                    0 as *const libc::c_char,
                    b"Cancelled\0" as *const u8 as *const libc::c_char,
                    5 as libc::c_int,
                ),
            );
            break;
        } else {
            let mut was_current_lineno: ssize_t = (*(*openfile).current).lineno;
            let mut was_current_x: size_t = (*openfile).current_x;
            let mut function: functionptrtype = func_from_key(response);
            given = mallocstrcpy(given, answer);
            if ran_a_tool {
                break;
            }
            if function == Some(flip_newbuffer as unsafe extern "C" fn() -> ()) {
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
                {
                    flags[(MULTIBUFFER as libc::c_int as libc::c_ulong)
                        .wrapping_div(
                            (::std::mem::size_of::<libc::c_uint>() as libc::c_ulong)
                                .wrapping_mul(8 as libc::c_int as libc::c_ulong),
                        ) as usize]
                        ^= (1 as libc::c_int as libc::c_uint)
                            << (MULTIBUFFER as libc::c_int as libc::c_ulong)
                                .wrapping_rem(
                                    (::std::mem::size_of::<libc::c_uint>() as libc::c_ulong)
                                        .wrapping_mul(8 as libc::c_int as libc::c_ulong),
                                );
                } else {
                    beep();
                }
            } else if function == Some(flip_convert as unsafe extern "C" fn() -> ()) {
                flags[(NO_CONVERT as libc::c_int as libc::c_ulong)
                    .wrapping_div(
                        (::std::mem::size_of::<libc::c_uint>() as libc::c_ulong)
                            .wrapping_mul(8 as libc::c_int as libc::c_ulong),
                    ) as usize]
                    ^= (1 as libc::c_int as libc::c_uint)
                        << (NO_CONVERT as libc::c_int as libc::c_ulong)
                            .wrapping_rem(
                                (::std::mem::size_of::<libc::c_uint>() as libc::c_ulong)
                                    .wrapping_mul(8 as libc::c_int as libc::c_ulong),
                            );
            } else if function == Some(flip_execute as unsafe extern "C" fn() -> ()) {
                execute = !execute;
            } else if function == Some(flip_pipe as unsafe extern "C" fn() -> ()) {
                add_or_remove_pipe_symbol_from_answer();
                given = mallocstrcpy(given, answer);
            } else {
                if function == Some(to_files as unsafe extern "C" fn() -> ()) {
                    let mut chosen: *mut libc::c_char = browse_in(answer);
                    if chosen.is_null() {
                        continue;
                    }
                    rpl_free(answer as *mut libc::c_void);
                    answer = chosen;
                    response = 0 as libc::c_int;
                }
                if response != 0 as libc::c_int
                    && (!(flags[(MULTIBUFFER as libc::c_int as libc::c_ulong)
                        .wrapping_div(
                            (::std::mem::size_of::<libc::c_uint>() as libc::c_ulong)
                                .wrapping_mul(8 as libc::c_int as libc::c_ulong),
                        ) as usize]
                        & (1 as libc::c_int as libc::c_uint)
                            << (MULTIBUFFER as libc::c_int as libc::c_ulong)
                                .wrapping_rem(
                                    (::std::mem::size_of::<libc::c_uint>() as libc::c_ulong)
                                        .wrapping_mul(8 as libc::c_int as libc::c_ulong),
                                ) != 0 as libc::c_int as libc::c_uint)
                        || response != -(2 as libc::c_int))
                {
                    continue;
                }
                if execute {
                    if flags[(MULTIBUFFER as libc::c_int as libc::c_ulong)
                        .wrapping_div(
                            (::std::mem::size_of::<libc::c_uint>() as libc::c_ulong)
                                .wrapping_mul(8 as libc::c_int as libc::c_ulong),
                        ) as usize]
                        & (1 as libc::c_int as libc::c_uint)
                            << (MULTIBUFFER as libc::c_int as libc::c_ulong)
                                .wrapping_rem(
                                    (::std::mem::size_of::<libc::c_uint>() as libc::c_ulong)
                                        .wrapping_mul(8 as libc::c_int as libc::c_ulong),
                                ) != 0 as libc::c_int as libc::c_uint
                    {
                        open_buffer(
                            b"\0" as *const u8 as *const libc::c_char,
                            1 as libc::c_int != 0,
                        );
                    }
                    if *answer as libc::c_int != '\0' as i32 {
                        execute_command(answer);
                        update_history(
                            &mut execute_history,
                            answer,
                            1 as libc::c_int != 0,
                        );
                    }
                    if flags[(MULTIBUFFER as libc::c_int as libc::c_ulong)
                        .wrapping_div(
                            (::std::mem::size_of::<libc::c_uint>() as libc::c_ulong)
                                .wrapping_mul(8 as libc::c_int as libc::c_ulong),
                        ) as usize]
                        & (1 as libc::c_int as libc::c_uint)
                            << (MULTIBUFFER as libc::c_int as libc::c_ulong)
                                .wrapping_rem(
                                    (::std::mem::size_of::<libc::c_uint>() as libc::c_ulong)
                                        .wrapping_mul(8 as libc::c_int as libc::c_ulong),
                                ) != 0 as libc::c_int as libc::c_uint
                    {
                        (*openfile).current = (*openfile).filetop;
                        (*openfile).current_x = 0 as libc::c_int as size_t;
                        (*openfile).placewewant = 0 as libc::c_int as size_t;
                        set_modified();
                    }
                } else {
                    answer = free_and_assign(answer, real_dir_from_tilde(answer));
                    open_buffer(
                        answer,
                        flags[(MULTIBUFFER as libc::c_int as libc::c_ulong)
                            .wrapping_div(
                                (::std::mem::size_of::<libc::c_uint>() as libc::c_ulong)
                                    .wrapping_mul(8 as libc::c_int as libc::c_ulong),
                            ) as usize]
                            & (1 as libc::c_int as libc::c_uint)
                                << (MULTIBUFFER as libc::c_int as libc::c_ulong)
                                    .wrapping_rem(
                                        (::std::mem::size_of::<libc::c_uint>() as libc::c_ulong)
                                            .wrapping_mul(8 as libc::c_int as libc::c_ulong),
                                    ) != 0 as libc::c_int as libc::c_uint,
                    );
                }
                if flags[(MULTIBUFFER as libc::c_int as libc::c_ulong)
                    .wrapping_div(
                        (::std::mem::size_of::<libc::c_uint>() as libc::c_ulong)
                            .wrapping_mul(8 as libc::c_int as libc::c_ulong),
                    ) as usize]
                    & (1 as libc::c_int as libc::c_uint)
                        << (MULTIBUFFER as libc::c_int as libc::c_ulong)
                            .wrapping_rem(
                                (::std::mem::size_of::<libc::c_uint>() as libc::c_ulong)
                                    .wrapping_mul(8 as libc::c_int as libc::c_ulong),
                            ) != 0 as libc::c_int as libc::c_uint
                {
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
                        let mut priorline: ssize_t = 0;
                        let mut priorcol: ssize_t = 0;
                        if !execute {
                            if has_old_position(answer, &mut priorline, &mut priorcol) {
                                goto_line_and_column(
                                    priorline,
                                    priorcol,
                                    0 as libc::c_int != 0,
                                    0 as libc::c_int != 0,
                                );
                            }
                        }
                    }
                    prepare_for_display();
                } else {
                    if (*(*openfile).current).lineno != was_current_lineno
                        || (*openfile).current_x != was_current_x
                    {
                        set_modified();
                    }
                    refresh_needed = 1 as libc::c_int != 0;
                }
                break;
            }
        }
    }
    rpl_free(given as *mut libc::c_void);
    if was_multibuffer {
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
    } else {
        flags[(MULTIBUFFER as libc::c_int as libc::c_ulong)
            .wrapping_div(
                (::std::mem::size_of::<libc::c_uint>() as libc::c_ulong)
                    .wrapping_mul(8 as libc::c_int as libc::c_ulong),
            ) as usize]
            &= !((1 as libc::c_int as libc::c_uint)
                << (MULTIBUFFER as libc::c_int as libc::c_ulong)
                    .wrapping_rem(
                        (::std::mem::size_of::<libc::c_uint>() as libc::c_ulong)
                            .wrapping_mul(8 as libc::c_int as libc::c_ulong),
                    ));
    };
}
pub unsafe extern "C" fn do_insertfile() {
    if !in_restricted_mode() {
        insert_a_file_or(0 as libc::c_int != 0);
    }
}
pub unsafe extern "C" fn do_execute() {
    if !in_restricted_mode() {
        insert_a_file_or(1 as libc::c_int != 0);
    }
}
pub unsafe extern "C" fn get_full_path(
    mut origpath: *const libc::c_char,
) -> *mut libc::c_char {
    let mut untilded: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut target: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut slash: *mut libc::c_char = 0 as *mut libc::c_char;
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
    if origpath.is_null() {
        return 0 as *mut libc::c_char;
    }
    untilded = real_dir_from_tilde(origpath);
    target = realpath(untilded, 0 as *mut libc::c_char);
    slash = strrchr(untilded, '/' as i32);
    if target.is_null() && !slash.is_null()
        && *slash.offset(1 as libc::c_int as isize) as libc::c_int != 0
    {
        *slash = '\0' as i32 as libc::c_char;
        target = realpath(untilded, 0 as *mut libc::c_char);
        if !target.is_null() {
            target = nrealloc(
                target as *mut libc::c_void,
                (strlen(target))
                    .wrapping_add(strlen(slash.offset(1 as libc::c_int as isize)))
                    .wrapping_add(1 as libc::c_int as libc::c_ulong),
            ) as *mut libc::c_char;
            strcat(target, slash.offset(1 as libc::c_int as isize));
        }
    }
    if !target.is_null() && *target.offset(1 as libc::c_int as isize) as libc::c_int != 0
        && stat(target, &mut fileinfo) == 0 as libc::c_int
        && fileinfo.st_mode & 0o170000 as libc::c_int as libc::c_uint
            == 0o40000 as libc::c_int as libc::c_uint
    {
        target = nrealloc(
            target as *mut libc::c_void,
            (strlen(target)).wrapping_add(2 as libc::c_int as libc::c_ulong),
        ) as *mut libc::c_char;
        strcat(target, b"/\0" as *const u8 as *const libc::c_char);
    }
    rpl_free(untilded as *mut libc::c_void);
    return target;
}
pub unsafe extern "C" fn check_writable_directory(
    mut path: *const libc::c_char,
) -> *mut libc::c_char {
    let mut full_path: *mut libc::c_char = get_full_path(path);
    if full_path.is_null() {
        return 0 as *mut libc::c_char;
    }
    if *full_path
        .offset(
            (strlen(full_path)).wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize,
        ) as libc::c_int != '/' as i32
        || access(full_path, 2 as libc::c_int) != 0 as libc::c_int
    {
        rpl_free(full_path as *mut libc::c_void);
        return 0 as *mut libc::c_char;
    }
    return full_path;
}
pub unsafe extern "C" fn safe_tempfile(mut stream: *mut *mut FILE) -> *mut libc::c_char {
    let mut env_dir: *const libc::c_char = getenv(
        b"TMPDIR\0" as *const u8 as *const libc::c_char,
    );
    let mut tempdir: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut tempfile_name: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut extension: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut descriptor: libc::c_int = 0;
    if !env_dir.is_null() {
        tempdir = check_writable_directory(env_dir);
    }
    if tempdir.is_null() {
        tempdir = check_writable_directory(
            b"/tmp\0" as *const u8 as *const libc::c_char,
        );
    }
    if tempdir.is_null() {
        tempdir = copy_of(b"/tmp/\0" as *const u8 as *const libc::c_char);
    }
    extension = strrchr((*openfile).filename, '.' as i32);
    if extension.is_null() || !(strchr(extension, '/' as i32)).is_null() {
        extension = ((*openfile).filename).offset(strlen((*openfile).filename) as isize);
    }
    tempfile_name = nrealloc(
        tempdir as *mut libc::c_void,
        (strlen(tempdir))
            .wrapping_add(12 as libc::c_int as libc::c_ulong)
            .wrapping_add(strlen(extension)),
    ) as *mut libc::c_char;
    strcat(tempfile_name, b"nano.XXXXXX\0" as *const u8 as *const libc::c_char);
    strcat(tempfile_name, extension);
    descriptor = mkstemps(tempfile_name, strlen(extension) as libc::c_int);
    *stream = if descriptor > 0 as libc::c_int {
        fdopen(descriptor, b"r+b\0" as *const u8 as *const libc::c_char)
    } else {
        0 as *mut FILE
    };
    if (*stream).is_null() {
        if descriptor > 0 as libc::c_int {
            close(descriptor);
        }
        rpl_free(tempfile_name as *mut libc::c_void);
        return 0 as *mut libc::c_char;
    }
    return tempfile_name;
}
pub unsafe extern "C" fn init_operating_dir() {
    let mut target: *mut libc::c_char = get_full_path(operating_dir);
    if target.is_null() || chdir(target) == -(1 as libc::c_int) {
        die(
            dcgettext(
                0 as *const libc::c_char,
                b"Invalid operating directory: %s\n\0" as *const u8
                    as *const libc::c_char,
                5 as libc::c_int,
            ),
            operating_dir,
        );
    }
    rpl_free(operating_dir as *mut libc::c_void);
    operating_dir = nrealloc(
        target as *mut libc::c_void,
        (strlen(target)).wrapping_add(1 as libc::c_int as libc::c_ulong),
    ) as *mut libc::c_char;
}
pub unsafe extern "C" fn outside_of_confinement(
    mut somepath: *const libc::c_char,
    mut tabbing: bool,
) -> bool {
    let mut is_inside: bool = false;
    let mut begins_to_be: bool = false;
    let mut fullpath: *mut libc::c_char = 0 as *mut libc::c_char;
    if operating_dir.is_null() {
        return 0 as libc::c_int != 0;
    }
    fullpath = get_full_path(somepath);
    if fullpath.is_null() {
        return tabbing;
    }
    is_inside = strstr(fullpath, operating_dir) == fullpath;
    begins_to_be = tabbing as libc::c_int != 0
        && strstr(operating_dir, fullpath) == operating_dir;
    rpl_free(fullpath as *mut libc::c_void);
    return !is_inside && !begins_to_be;
}
pub unsafe extern "C" fn init_backup_dir() {
    let mut target: *mut libc::c_char = get_full_path(backup_dir);
    if target.is_null()
        || *target
            .offset(
                (strlen(target)).wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize,
            ) as libc::c_int != '/' as i32
    {
        die(
            dcgettext(
                0 as *const libc::c_char,
                b"Invalid backup directory: %s\n\0" as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ),
            backup_dir,
        );
    }
    rpl_free(backup_dir as *mut libc::c_void);
    backup_dir = nrealloc(
        target as *mut libc::c_void,
        (strlen(target)).wrapping_add(1 as libc::c_int as libc::c_ulong),
    ) as *mut libc::c_char;
}
pub unsafe extern "C" fn copy_file(
    mut inn: *mut FILE,
    mut out: *mut FILE,
    mut close_out: bool,
) -> libc::c_int {
    let mut retval: libc::c_int = 0 as libc::c_int;
    let mut buf: [libc::c_char; 8192] = [0; 8192];
    let mut charsread: size_t = 0;
    let mut flush_out_fnc: Option::<unsafe extern "C" fn(*mut FILE) -> libc::c_int> = if close_out
        as libc::c_int != 0
    {
        Some(fclose as unsafe extern "C" fn(*mut FILE) -> libc::c_int)
    } else {
        Some(fflush as unsafe extern "C" fn(*mut FILE) -> libc::c_int)
    };
    loop {
        charsread = fread(
            buf.as_mut_ptr() as *mut libc::c_void,
            1 as libc::c_int as libc::c_ulong,
            8192 as libc::c_int as libc::c_ulong,
            inn,
        );
        if charsread == 0 as libc::c_int as libc::c_ulong && ferror(inn) != 0 {
            retval = -(1 as libc::c_int);
            break;
        } else if fwrite(
            buf.as_mut_ptr() as *const libc::c_void,
            1 as libc::c_int as libc::c_ulong,
            charsread,
            out,
        ) < charsread
        {
            retval = 2 as libc::c_int;
            break;
        } else if !(charsread > 0 as libc::c_int as libc::c_ulong) {
            break;
        }
    }
    if fclose(inn) == -(1 as libc::c_int) {
        retval = -(3 as libc::c_int);
    }
    if flush_out_fnc.unwrap()(out) == -(1 as libc::c_int) {
        retval = 4 as libc::c_int;
    }
    return retval;
}
pub unsafe extern "C" fn make_backup_of(mut realname: *mut libc::c_char) -> bool {
    let mut current_block: u64;
    let mut original: *mut FILE = 0 as *mut FILE;
    let mut backup_file: *mut FILE = 0 as *mut FILE;
    static mut filetime: [timespec; 2] = [timespec { tv_sec: 0, tv_nsec: 0 }; 2];
    let mut creation_flags: libc::c_int = 0;
    let mut descriptor: libc::c_int = 0;
    let mut second_attempt: bool = 0 as libc::c_int != 0;
    let mut backupname: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut verdict: libc::c_int = 0 as libc::c_int;
    filetime[0 as libc::c_int as usize].tv_sec = (*(*openfile).statinfo).st_atim.tv_sec;
    filetime[1 as libc::c_int as usize].tv_sec = (*(*openfile).statinfo).st_mtim.tv_sec;
    statusbar(
        dcgettext(
            0 as *const libc::c_char,
            b"Making backup...\0" as *const u8 as *const libc::c_char,
            5 as libc::c_int,
        ),
    );
    if backup_dir.is_null() {
        backupname = nmalloc(
            (strlen(realname)).wrapping_add(2 as libc::c_int as libc::c_ulong),
        ) as *mut libc::c_char;
        sprintf(backupname, b"%s~\0" as *const u8 as *const libc::c_char, realname);
    } else {
        let mut thename: *mut libc::c_char = get_full_path(realname);
        if !thename.is_null() {
            let mut i: libc::c_int = 0 as libc::c_int;
            while *thename.offset(i as isize) as libc::c_int != '\0' as i32 {
                if *thename.offset(i as isize) as libc::c_int == '/' as i32 {
                    *thename.offset(i as isize) = '!' as i32 as libc::c_char;
                }
                i += 1;
                i;
            }
        } else {
            thename = copy_of(tail(realname));
        }
        backupname = nmalloc(
            (strlen(backup_dir))
                .wrapping_add(strlen(thename))
                .wrapping_add(1 as libc::c_int as libc::c_ulong),
        ) as *mut libc::c_char;
        sprintf(
            backupname,
            b"%s%s\0" as *const u8 as *const libc::c_char,
            backup_dir,
            thename,
        );
        rpl_free(thename as *mut libc::c_void);
        thename = get_next_filename(
            backupname,
            b"~\0" as *const u8 as *const libc::c_char,
        );
        rpl_free(backupname as *mut libc::c_void);
        backupname = thename;
        if *backupname as libc::c_int == '\0' as i32 {
            statusline(
                ALERT,
                dcgettext(
                    0 as *const libc::c_char,
                    b"Too many existing backup files\0" as *const u8
                        as *const libc::c_char,
                    5 as libc::c_int,
                ),
            );
            rpl_free(backupname as *mut libc::c_void);
            return 0 as libc::c_int != 0;
        }
    }
    if unlink(backupname) < 0 as libc::c_int && *__errno_location() != 2 as libc::c_int
        && !(flags[(INSECURE_BACKUP as libc::c_int as libc::c_ulong)
            .wrapping_div(
                (::std::mem::size_of::<libc::c_uint>() as libc::c_ulong)
                    .wrapping_mul(8 as libc::c_int as libc::c_ulong),
            ) as usize]
            & (1 as libc::c_int as libc::c_uint)
                << (INSECURE_BACKUP as libc::c_int as libc::c_ulong)
                    .wrapping_rem(
                        (::std::mem::size_of::<libc::c_uint>() as libc::c_ulong)
                            .wrapping_mul(8 as libc::c_int as libc::c_ulong),
                    ) != 0 as libc::c_int as libc::c_uint)
    {
        current_block = 303729790187482352;
    } else {
        creation_flags = 0o1 as libc::c_int | 0o100 as libc::c_int
            | (if flags[(INSECURE_BACKUP as libc::c_int as libc::c_ulong)
                .wrapping_div(
                    (::std::mem::size_of::<libc::c_uint>() as libc::c_ulong)
                        .wrapping_mul(8 as libc::c_int as libc::c_ulong),
                ) as usize]
                & (1 as libc::c_int as libc::c_uint)
                    << (INSECURE_BACKUP as libc::c_int as libc::c_ulong)
                        .wrapping_rem(
                            (::std::mem::size_of::<libc::c_uint>() as libc::c_ulong)
                                .wrapping_mul(8 as libc::c_int as libc::c_ulong),
                        ) != 0 as libc::c_int as libc::c_uint
            {
                0o1000 as libc::c_int
            } else {
                0o200 as libc::c_int
            });
        descriptor = open(
            backupname,
            creation_flags,
            0o400 as libc::c_int | 0o200 as libc::c_int,
        );
        current_block = 6609595307432477658;
    }
    loop {
        match current_block {
            303729790187482352 => {
                get_homedir();
                if !second_attempt && !homedir.is_null() {
                    unlink(backupname);
                    rpl_free(backupname as *mut libc::c_void);
                    warn_and_briefly_pause(
                        dcgettext(
                            0 as *const libc::c_char,
                            b"Cannot make regular backup\0" as *const u8
                                as *const libc::c_char,
                            5 as libc::c_int,
                        ),
                    );
                    warn_and_briefly_pause(
                        dcgettext(
                            0 as *const libc::c_char,
                            b"Trying again in your home directory\0" as *const u8
                                as *const libc::c_char,
                            5 as libc::c_int,
                        ),
                    );
                    currmenu = (1 as libc::c_int) << 0 as libc::c_int
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
                        | (1 as libc::c_int) << 14 as libc::c_int;
                    backupname = nmalloc(
                        (strlen(homedir))
                            .wrapping_add(strlen(tail(realname)))
                            .wrapping_add(9 as libc::c_int as libc::c_ulong),
                    ) as *mut libc::c_char;
                    sprintf(
                        backupname,
                        b"%s/%s~XXXXXX\0" as *const u8 as *const libc::c_char,
                        homedir,
                        tail(realname),
                    );
                    descriptor = mkstemp(backupname);
                    backup_file = 0 as *mut FILE;
                    second_attempt = 1 as libc::c_int != 0;
                    current_block = 6609595307432477658;
                } else {
                    warn_and_briefly_pause(
                        dcgettext(
                            0 as *const libc::c_char,
                            b"Cannot make backup\0" as *const u8 as *const libc::c_char,
                            5 as libc::c_int,
                        ),
                    );
                    break;
                }
            }
            _ => {
                if descriptor >= 0 as libc::c_int {
                    backup_file = fdopen(
                        descriptor,
                        b"wb\0" as *const u8 as *const libc::c_char,
                    );
                }
                if backup_file.is_null() {
                    current_block = 303729790187482352;
                    continue;
                }
                if fchown(
                    descriptor,
                    (*(*openfile).statinfo).st_uid,
                    (*(*openfile).statinfo).st_gid,
                ) < 0 as libc::c_int && *__errno_location() != 1 as libc::c_int
                {
                    fclose(backup_file);
                    current_block = 303729790187482352;
                } else if fchmod(descriptor, (*(*openfile).statinfo).st_mode)
                    < 0 as libc::c_int && *__errno_location() != 1 as libc::c_int
                {
                    fclose(backup_file);
                    current_block = 303729790187482352;
                } else {
                    original = fopen(
                        realname,
                        b"rb\0" as *const u8 as *const libc::c_char,
                    );
                    if !original.is_null() {
                        verdict = copy_file(
                            original,
                            backup_file,
                            0 as libc::c_int != 0,
                        );
                    }
                    if original.is_null() || verdict < 0 as libc::c_int {
                        warn_and_briefly_pause(
                            dcgettext(
                                0 as *const libc::c_char,
                                b"Cannot read original file\0" as *const u8
                                    as *const libc::c_char,
                                5 as libc::c_int,
                            ),
                        );
                        fclose(backup_file);
                        break;
                    } else if verdict > 0 as libc::c_int {
                        fclose(backup_file);
                        current_block = 303729790187482352;
                    } else if fflush(backup_file) != 0 as libc::c_int
                        || fsync(fileno(backup_file)) != 0 as libc::c_int
                    {
                        fclose(backup_file);
                        current_block = 303729790187482352;
                    } else {
                        futimens(descriptor, filetime.as_mut_ptr() as *const timespec)
                            != 0;
                        if fclose(backup_file) == 0 as libc::c_int {
                            rpl_free(backupname as *mut libc::c_void);
                            return 1 as libc::c_int != 0;
                        }
                        current_block = 303729790187482352;
                    }
                }
            }
        }
    }
    warn_and_briefly_pause(strerror(*__errno_location()));
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
    rpl_free(backupname as *mut libc::c_void);
    if *__errno_location() != 28 as libc::c_int
        && ask_user(
            0 as libc::c_int != 0,
            dcgettext(
                0 as *const libc::c_char,
                b"Cannot make backup; continue and save actual file? \0" as *const u8
                    as *const libc::c_char,
                5 as libc::c_int,
            ),
        ) == 1 as libc::c_int
    {
        return 1 as libc::c_int != 0;
    }
    statusline(
        HUSH,
        dcgettext(
            0 as *const libc::c_char,
            b"Cannot make backup: %s\0" as *const u8 as *const libc::c_char,
            5 as libc::c_int,
        ),
        strerror(*__errno_location()),
    );
    return 0 as libc::c_int != 0;
}
pub unsafe extern "C" fn write_file(
    mut name: *const libc::c_char,
    mut thefile: *mut FILE,
    mut normal: bool,
    mut method: kind_of_writing_type,
    mut annotate: bool,
) -> bool {
    let mut current_block: u64;
    let mut is_existing_file: bool = false;
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
    let mut realname: *mut libc::c_char = real_dir_from_tilde(name);
    let mut tempname: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut line: *mut linestruct = (*openfile).filetop;
    let mut lineswritten: size_t = 0 as libc::c_int as size_t;
    if normal as libc::c_int != 0
        && outside_of_confinement(realname, 0 as libc::c_int != 0) as libc::c_int != 0
    {
        statusline(
            ALERT,
            dcgettext(
                0 as *const libc::c_char,
                b"Can't write outside of %s\0" as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ),
            operating_dir,
        );
    } else {
        is_existing_file = normal as libc::c_int != 0
            && stat(realname, &mut fileinfo) != -(1 as libc::c_int);
        if ((*openfile).statinfo).is_null() && is_existing_file as libc::c_int != 0 {
            stat_with_alloc(realname, &mut (*openfile).statinfo);
        }
        if flags[(MAKE_BACKUP as libc::c_int as libc::c_ulong)
            .wrapping_div(
                (::std::mem::size_of::<libc::c_uint>() as libc::c_ulong)
                    .wrapping_mul(8 as libc::c_int as libc::c_ulong),
            ) as usize]
            & (1 as libc::c_int as libc::c_uint)
                << (MAKE_BACKUP as libc::c_int as libc::c_ulong)
                    .wrapping_rem(
                        (::std::mem::size_of::<libc::c_uint>() as libc::c_ulong)
                            .wrapping_mul(8 as libc::c_int as libc::c_ulong),
                    ) != 0 as libc::c_int as libc::c_uint
            && is_existing_file as libc::c_int != 0
            && !(fileinfo.st_mode & 0o170000 as libc::c_int as libc::c_uint
                == 0o10000 as libc::c_int as libc::c_uint)
            && !((*openfile).statinfo).is_null()
            && ((*(*openfile).statinfo).st_mtim.tv_sec == fileinfo.st_mtim.tv_sec
                || method as libc::c_uint != OVERWRITE as libc::c_int as libc::c_uint
                || !((*openfile).mark).is_null())
        {
            if !make_backup_of(realname) {
                current_block = 16207348652587261309;
            } else {
                current_block = 13513818773234778473;
            }
        } else {
            current_block = 13513818773234778473;
        }
        match current_block {
            16207348652587261309 => {}
            _ => {
                if method as libc::c_uint == PREPEND as libc::c_int as libc::c_uint {
                    let mut source: *mut FILE = 0 as *mut FILE;
                    let mut target: *mut FILE = 0 as *mut FILE;
                    let mut verdict: libc::c_int = 0;
                    if is_existing_file as libc::c_int != 0
                        && fileinfo.st_mode & 0o170000 as libc::c_int as libc::c_uint
                            == 0o10000 as libc::c_int as libc::c_uint
                    {
                        statusline(
                            ALERT,
                            dcgettext(
                                0 as *const libc::c_char,
                                b"Error writing %s: %s\0" as *const u8
                                    as *const libc::c_char,
                                5 as libc::c_int,
                            ),
                            realname,
                            b"FIFO\0" as *const u8 as *const libc::c_char,
                        );
                        current_block = 16207348652587261309;
                    } else {
                        source = fopen(
                            realname,
                            b"rb\0" as *const u8 as *const libc::c_char,
                        );
                        if source.is_null() {
                            statusline(
                                ALERT,
                                dcgettext(
                                    0 as *const libc::c_char,
                                    b"Error reading %s: %s\0" as *const u8
                                        as *const libc::c_char,
                                    5 as libc::c_int,
                                ),
                                realname,
                                strerror(*__errno_location()),
                            );
                            current_block = 16207348652587261309;
                        } else {
                            tempname = safe_tempfile(&mut target);
                            if tempname.is_null() {
                                statusline(
                                    ALERT,
                                    dcgettext(
                                        0 as *const libc::c_char,
                                        b"Error writing temp file: %s\0" as *const u8
                                            as *const libc::c_char,
                                        5 as libc::c_int,
                                    ),
                                    strerror(*__errno_location()),
                                );
                                fclose(source);
                                current_block = 16207348652587261309;
                            } else {
                                verdict = copy_file(source, target, 1 as libc::c_int != 0);
                                if verdict < 0 as libc::c_int {
                                    statusline(
                                        ALERT,
                                        dcgettext(
                                            0 as *const libc::c_char,
                                            b"Error reading %s: %s\0" as *const u8
                                                as *const libc::c_char,
                                            5 as libc::c_int,
                                        ),
                                        realname,
                                        strerror(*__errno_location()),
                                    );
                                    unlink(tempname);
                                    current_block = 16207348652587261309;
                                } else if verdict > 0 as libc::c_int {
                                    statusline(
                                        ALERT,
                                        dcgettext(
                                            0 as *const libc::c_char,
                                            b"Error writing temp file: %s\0" as *const u8
                                                as *const libc::c_char,
                                            5 as libc::c_int,
                                        ),
                                        strerror(*__errno_location()),
                                    );
                                    unlink(tempname);
                                    current_block = 16207348652587261309;
                                } else {
                                    current_block = 9828876828309294594;
                                }
                            }
                        }
                    }
                } else {
                    current_block = 9828876828309294594;
                }
                match current_block {
                    16207348652587261309 => {}
                    _ => {
                        if is_existing_file as libc::c_int != 0
                            && fileinfo.st_mode & 0o170000 as libc::c_int as libc::c_uint
                                == 0o10000 as libc::c_int as libc::c_uint
                        {
                            statusbar(
                                dcgettext(
                                    0 as *const libc::c_char,
                                    b"Writing to FIFO...\0" as *const u8 as *const libc::c_char,
                                    5 as libc::c_int,
                                ),
                            );
                        }
                        if thefile.is_null() {
                            let mut permissions: mode_t = (if normal as libc::c_int != 0
                            {
                                0o400 as libc::c_int | 0o200 as libc::c_int
                                    | 0o400 as libc::c_int >> 3 as libc::c_int
                                    | 0o200 as libc::c_int >> 3 as libc::c_int
                                    | 0o400 as libc::c_int >> 3 as libc::c_int
                                        >> 3 as libc::c_int
                                    | 0o200 as libc::c_int >> 3 as libc::c_int
                                        >> 3 as libc::c_int
                            } else {
                                0o400 as libc::c_int | 0o200 as libc::c_int
                            }) as mode_t;
                            let mut fd: libc::c_int = 0;
                            block_sigwinch(1 as libc::c_int != 0);
                            if normal {
                                install_handler_for_Ctrl_C();
                            }
                            fd = open(
                                realname,
                                0o1 as libc::c_int | 0o100 as libc::c_int
                                    | (if method as libc::c_uint
                                        == APPEND as libc::c_int as libc::c_uint
                                    {
                                        0o2000 as libc::c_int
                                    } else {
                                        if normal as libc::c_int != 0 {
                                            0o1000 as libc::c_int
                                        } else {
                                            0o200 as libc::c_int
                                        }
                                    }),
                                permissions,
                            );
                            if normal {
                                restore_handler_for_Ctrl_C();
                            }
                            block_sigwinch(0 as libc::c_int != 0);
                            if fd == -(1 as libc::c_int) {
                                if *__errno_location() == 4 as libc::c_int
                                    || *__errno_location() == 0 as libc::c_int
                                {
                                    statusline(
                                        ALERT,
                                        dcgettext(
                                            0 as *const libc::c_char,
                                            b"Interrupted\0" as *const u8 as *const libc::c_char,
                                            5 as libc::c_int,
                                        ),
                                    );
                                } else {
                                    statusline(
                                        ALERT,
                                        dcgettext(
                                            0 as *const libc::c_char,
                                            b"Error writing %s: %s\0" as *const u8
                                                as *const libc::c_char,
                                            5 as libc::c_int,
                                        ),
                                        realname,
                                        strerror(*__errno_location()),
                                    );
                                }
                                if !tempname.is_null() {
                                    unlink(tempname);
                                }
                                current_block = 16207348652587261309;
                            } else {
                                thefile = fdopen(
                                    fd,
                                    if method as libc::c_uint
                                        == APPEND as libc::c_int as libc::c_uint
                                    {
                                        b"ab\0" as *const u8 as *const libc::c_char
                                    } else {
                                        b"wb\0" as *const u8 as *const libc::c_char
                                    },
                                );
                                if thefile.is_null() {
                                    statusline(
                                        ALERT,
                                        dcgettext(
                                            0 as *const libc::c_char,
                                            b"Error writing %s: %s\0" as *const u8
                                                as *const libc::c_char,
                                            5 as libc::c_int,
                                        ),
                                        realname,
                                        strerror(*__errno_location()),
                                    );
                                    close(fd);
                                    current_block = 16207348652587261309;
                                } else {
                                    current_block = 15512526488502093901;
                                }
                            }
                        } else {
                            current_block = 15512526488502093901;
                        }
                        match current_block {
                            16207348652587261309 => {}
                            _ => {
                                if normal {
                                    statusbar(
                                        dcgettext(
                                            0 as *const libc::c_char,
                                            b"Writing...\0" as *const u8 as *const libc::c_char,
                                            5 as libc::c_int,
                                        ),
                                    );
                                }
                                loop {
                                    let mut data_len: size_t = 0;
                                    let mut wrote: size_t = 0;
                                    data_len = recode_LF_to_NUL((*line).data);
                                    wrote = fwrite(
                                        (*line).data as *const libc::c_void,
                                        1 as libc::c_int as libc::c_ulong,
                                        data_len,
                                        thefile,
                                    );
                                    recode_NUL_to_LF((*line).data, data_len);
                                    if wrote < data_len {
                                        statusline(
                                            ALERT,
                                            dcgettext(
                                                0 as *const libc::c_char,
                                                b"Error writing %s: %s\0" as *const u8
                                                    as *const libc::c_char,
                                                5 as libc::c_int,
                                            ),
                                            realname,
                                            strerror(*__errno_location()),
                                        );
                                        fclose(thefile);
                                        current_block = 16207348652587261309;
                                        break;
                                    } else if ((*line).next).is_null() {
                                        if *((*line).data).offset(0 as libc::c_int as isize)
                                            as libc::c_int != '\0' as i32
                                        {
                                            lineswritten = lineswritten.wrapping_add(1);
                                            lineswritten;
                                        }
                                        current_block = 3392087639489470149;
                                        break;
                                    } else {
                                        if (*openfile).fmt as libc::c_uint
                                            == DOS_FILE as libc::c_int as libc::c_uint
                                            || (*openfile).fmt as libc::c_uint
                                                == MAC_FILE as libc::c_int as libc::c_uint
                                        {
                                            if putc('\r' as i32, thefile) == -(1 as libc::c_int) {
                                                statusline(
                                                    ALERT,
                                                    dcgettext(
                                                        0 as *const libc::c_char,
                                                        b"Error writing %s: %s\0" as *const u8
                                                            as *const libc::c_char,
                                                        5 as libc::c_int,
                                                    ),
                                                    realname,
                                                    strerror(*__errno_location()),
                                                );
                                                fclose(thefile);
                                                current_block = 16207348652587261309;
                                                break;
                                            }
                                        }
                                        if (*openfile).fmt as libc::c_uint
                                            != MAC_FILE as libc::c_int as libc::c_uint
                                        {
                                            if putc('\n' as i32, thefile) == -(1 as libc::c_int) {
                                                statusline(
                                                    ALERT,
                                                    dcgettext(
                                                        0 as *const libc::c_char,
                                                        b"Error writing %s: %s\0" as *const u8
                                                            as *const libc::c_char,
                                                        5 as libc::c_int,
                                                    ),
                                                    realname,
                                                    strerror(*__errno_location()),
                                                );
                                                fclose(thefile);
                                                current_block = 16207348652587261309;
                                                break;
                                            }
                                        }
                                        line = (*line).next;
                                        lineswritten = lineswritten.wrapping_add(1);
                                        lineswritten;
                                    }
                                }
                                match current_block {
                                    16207348652587261309 => {}
                                    _ => {
                                        if method as libc::c_uint
                                            == PREPEND as libc::c_int as libc::c_uint
                                        {
                                            let mut source_0: *mut FILE = fopen(
                                                tempname,
                                                b"rb\0" as *const u8 as *const libc::c_char,
                                            );
                                            let mut verdict_0: libc::c_int = 0;
                                            if source_0.is_null() {
                                                statusline(
                                                    ALERT,
                                                    dcgettext(
                                                        0 as *const libc::c_char,
                                                        b"Error reading temp file: %s\0" as *const u8
                                                            as *const libc::c_char,
                                                        5 as libc::c_int,
                                                    ),
                                                    strerror(*__errno_location()),
                                                );
                                                fclose(thefile);
                                                current_block = 16207348652587261309;
                                            } else {
                                                verdict_0 = copy_file(
                                                    source_0,
                                                    thefile,
                                                    0 as libc::c_int != 0,
                                                );
                                                if verdict_0 < 0 as libc::c_int {
                                                    statusline(
                                                        ALERT,
                                                        dcgettext(
                                                            0 as *const libc::c_char,
                                                            b"Error reading temp file: %s\0" as *const u8
                                                                as *const libc::c_char,
                                                            5 as libc::c_int,
                                                        ),
                                                        strerror(*__errno_location()),
                                                    );
                                                    fclose(thefile);
                                                    current_block = 16207348652587261309;
                                                } else if verdict_0 > 0 as libc::c_int {
                                                    statusline(
                                                        ALERT,
                                                        dcgettext(
                                                            0 as *const libc::c_char,
                                                            b"Error writing %s: %s\0" as *const u8
                                                                as *const libc::c_char,
                                                            5 as libc::c_int,
                                                        ),
                                                        realname,
                                                        strerror(*__errno_location()),
                                                    );
                                                    fclose(thefile);
                                                    current_block = 16207348652587261309;
                                                } else {
                                                    unlink(tempname);
                                                    current_block = 7385833325316299293;
                                                }
                                            }
                                        } else {
                                            current_block = 7385833325316299293;
                                        }
                                        match current_block {
                                            16207348652587261309 => {}
                                            _ => {
                                                if !is_existing_file
                                                    || !(fileinfo.st_mode
                                                        & 0o170000 as libc::c_int as libc::c_uint
                                                        == 0o10000 as libc::c_int as libc::c_uint)
                                                {
                                                    if fflush(thefile) != 0 as libc::c_int
                                                        || fsync(fileno(thefile)) != 0 as libc::c_int
                                                    {
                                                        statusline(
                                                            ALERT,
                                                            dcgettext(
                                                                0 as *const libc::c_char,
                                                                b"Error writing %s: %s\0" as *const u8
                                                                    as *const libc::c_char,
                                                                5 as libc::c_int,
                                                            ),
                                                            realname,
                                                            strerror(*__errno_location()),
                                                        );
                                                        fclose(thefile);
                                                        current_block = 16207348652587261309;
                                                    } else {
                                                        current_block = 11052029508375673978;
                                                    }
                                                } else {
                                                    current_block = 11052029508375673978;
                                                }
                                                match current_block {
                                                    16207348652587261309 => {}
                                                    _ => {
                                                        if fclose(thefile) != 0 as libc::c_int {
                                                            statusline(
                                                                ALERT,
                                                                dcgettext(
                                                                    0 as *const libc::c_char,
                                                                    b"Error writing %s: %s\0" as *const u8
                                                                        as *const libc::c_char,
                                                                    5 as libc::c_int,
                                                                ),
                                                                realname,
                                                                strerror(*__errno_location()),
                                                            );
                                                        } else {
                                                            if annotate as libc::c_int != 0
                                                                && method as libc::c_uint
                                                                    == OVERWRITE as libc::c_int as libc::c_uint
                                                            {
                                                                if strcmp((*openfile).filename, realname)
                                                                    != 0 as libc::c_int
                                                                {
                                                                    if !((*openfile).lock_filename).is_null() {
                                                                        delete_lockfile((*openfile).lock_filename);
                                                                        rpl_free((*openfile).lock_filename as *mut libc::c_void);
                                                                    }
                                                                    if flags[(LOCKING as libc::c_int as libc::c_ulong)
                                                                        .wrapping_div(
                                                                            (::std::mem::size_of::<libc::c_uint>() as libc::c_ulong)
                                                                                .wrapping_mul(8 as libc::c_int as libc::c_ulong),
                                                                        ) as usize]
                                                                        & (1 as libc::c_int as libc::c_uint)
                                                                            << (LOCKING as libc::c_int as libc::c_ulong)
                                                                                .wrapping_rem(
                                                                                    (::std::mem::size_of::<libc::c_uint>() as libc::c_ulong)
                                                                                        .wrapping_mul(8 as libc::c_int as libc::c_ulong),
                                                                                ) != 0 as libc::c_int as libc::c_uint
                                                                    {
                                                                        (*openfile)
                                                                            .lock_filename = do_lockfile(
                                                                            realname,
                                                                            0 as libc::c_int != 0,
                                                                        );
                                                                    }
                                                                    (*openfile)
                                                                        .filename = mallocstrcpy((*openfile).filename, realname);
                                                                    let mut oldname: *const libc::c_char = 0
                                                                        as *const libc::c_char;
                                                                    let mut newname: *const libc::c_char = 0
                                                                        as *const libc::c_char;
                                                                    oldname = if !((*openfile).syntax).is_null() {
                                                                        (*(*openfile).syntax).name as *const libc::c_char
                                                                    } else {
                                                                        b"\0" as *const u8 as *const libc::c_char
                                                                    };
                                                                    find_and_prime_applicable_syntax();
                                                                    newname = if !((*openfile).syntax).is_null() {
                                                                        (*(*openfile).syntax).name as *const libc::c_char
                                                                    } else {
                                                                        b"\0" as *const u8 as *const libc::c_char
                                                                    };
                                                                    if strcmp(oldname, newname) != 0 as libc::c_int {
                                                                        line = (*openfile).filetop;
                                                                        while !line.is_null() {
                                                                            rpl_free((*line).multidata as *mut libc::c_void);
                                                                            (*line).multidata = 0 as *mut libc::c_short;
                                                                            line = (*line).next;
                                                                        }
                                                                        precalc_multicolorinfo();
                                                                        have_palette = 0 as libc::c_int != 0;
                                                                        refresh_needed = 1 as libc::c_int != 0;
                                                                    }
                                                                }
                                                                stat_with_alloc(realname, &mut (*openfile).statinfo);
                                                                (*openfile).last_saved = (*openfile).current_undo;
                                                                (*openfile).last_action = OTHER;
                                                                (*openfile).modified = 0 as libc::c_int != 0;
                                                                titlebar(0 as *const libc::c_char);
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
                                                                && LINES > 1 as libc::c_int && annotate as libc::c_int != 0
                                                            {
                                                                report_size = 1 as libc::c_int != 0;
                                                            } else if normal {
                                                                statusline(
                                                                    REMARK,
                                                                    dcngettext(
                                                                        0 as *const libc::c_char,
                                                                        b"Wrote %zu line\0" as *const u8 as *const libc::c_char,
                                                                        b"Wrote %zu lines\0" as *const u8 as *const libc::c_char,
                                                                        lineswritten,
                                                                        5 as libc::c_int,
                                                                    ),
                                                                    lineswritten,
                                                                );
                                                            }
                                                            rpl_free(tempname as *mut libc::c_void);
                                                            rpl_free(realname as *mut libc::c_void);
                                                            return 1 as libc::c_int != 0;
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
            }
        }
    }
    if *__errno_location() == 28 as libc::c_int && normal as libc::c_int != 0 {
        napms(3200 as libc::c_int);
        lastmessage = VACUUM;
        statusline(
            ALERT,
            dcgettext(
                0 as *const libc::c_char,
                b"File on disk has been truncated!\0" as *const u8
                    as *const libc::c_char,
                5 as libc::c_int,
            ),
        );
        napms(3200 as libc::c_int);
        lastmessage = VACUUM;
        statusline(
            ALERT,
            dcgettext(
                0 as *const libc::c_char,
                b"Maybe ^T^Z, make room on disk, resume, then ^S^X\0" as *const u8
                    as *const libc::c_char,
                5 as libc::c_int,
            ),
        );
        stat_with_alloc(realname, &mut (*openfile).statinfo);
    }
    rpl_free(tempname as *mut libc::c_void);
    rpl_free(realname as *mut libc::c_void);
    return 0 as libc::c_int != 0;
}
pub unsafe extern "C" fn write_region_to_file(
    mut name: *const libc::c_char,
    mut stream: *mut FILE,
    mut normal: bool,
    mut method: kind_of_writing_type,
) -> bool {
    let mut birthline: *mut linestruct = 0 as *mut linestruct;
    let mut topline: *mut linestruct = 0 as *mut linestruct;
    let mut botline: *mut linestruct = 0 as *mut linestruct;
    let mut stopper: *mut linestruct = 0 as *mut linestruct;
    let mut afterline: *mut linestruct = 0 as *mut linestruct;
    let mut was_datastart: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut saved_byte: libc::c_char = 0;
    let mut top_x: size_t = 0;
    let mut bot_x: size_t = 0;
    let mut retval: bool = false;
    get_region(&mut topline, &mut top_x, &mut botline, &mut bot_x);
    if normal as libc::c_int != 0 && bot_x > 0 as libc::c_int as libc::c_ulong
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
        stopper = make_new_node(botline);
        (*stopper).data = copy_of(b"\0" as *const u8 as *const libc::c_char);
    } else {
        stopper = 0 as *mut linestruct;
    }
    afterline = (*botline).next;
    (*botline).next = stopper;
    saved_byte = *((*botline).data).offset(bot_x as isize);
    *((*botline).data).offset(bot_x as isize) = '\0' as i32 as libc::c_char;
    was_datastart = (*topline).data;
    (*topline).data = ((*topline).data).offset(top_x as isize);
    birthline = (*openfile).filetop;
    (*openfile).filetop = topline;
    retval = write_file(name, stream, normal, method, 0 as libc::c_int != 0);
    (*openfile).filetop = birthline;
    (*topline).data = was_datastart;
    *((*botline).data).offset(bot_x as isize) = saved_byte;
    (*botline).next = afterline;
    if !stopper.is_null() {
        delete_node(stopper);
    }
    return retval;
}
pub unsafe extern "C" fn write_it_out(
    mut exiting: bool,
    mut withprompt: bool,
) -> libc::c_int {
    let mut given: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut maychange: bool = *((*openfile).filename).offset(0 as libc::c_int as isize)
        as libc::c_int == '\0' as i32;
    let mut method: kind_of_writing_type = OVERWRITE;
    static mut did_credits: bool = 0 as libc::c_int != 0;
    as_an_at = 0 as libc::c_int != 0;
    given = copy_of(
        if !((*openfile).mark).is_null() && !exiting {
            b"\0" as *const u8 as *const libc::c_char
        } else {
            (*openfile).filename as *const libc::c_char
        },
    );
    loop {
        let mut function: functionptrtype = None;
        let mut msg: *const libc::c_char = 0 as *const libc::c_char;
        let mut response: libc::c_int = 0 as libc::c_int;
        let mut choice: libc::c_int = 0 as libc::c_int;
        let mut formatstr: *const libc::c_char = if (*openfile).fmt as libc::c_uint
            == DOS_FILE as libc::c_int as libc::c_uint
        {
            dcgettext(
                0 as *const libc::c_char,
                b" [DOS Format]\0" as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ) as *const libc::c_char
        } else if (*openfile).fmt as libc::c_uint
            == MAC_FILE as libc::c_int as libc::c_uint
        {
            dcgettext(
                0 as *const libc::c_char,
                b" [Mac Format]\0" as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ) as *const libc::c_char
        } else {
            b"\0" as *const u8 as *const libc::c_char
        };
        let mut backupstr: *const libc::c_char = if flags[(MAKE_BACKUP as libc::c_int
            as libc::c_ulong)
            .wrapping_div(
                (::std::mem::size_of::<libc::c_uint>() as libc::c_ulong)
                    .wrapping_mul(8 as libc::c_int as libc::c_ulong),
            ) as usize]
            & (1 as libc::c_int as libc::c_uint)
                << (MAKE_BACKUP as libc::c_int as libc::c_ulong)
                    .wrapping_rem(
                        (::std::mem::size_of::<libc::c_uint>() as libc::c_ulong)
                            .wrapping_mul(8 as libc::c_int as libc::c_ulong),
                    ) != 0 as libc::c_int as libc::c_uint
        {
            dcgettext(
                0 as *const libc::c_char,
                b" [Backup]\0" as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ) as *const libc::c_char
        } else {
            b"\0" as *const u8 as *const libc::c_char
        };
        if !((*openfile).mark).is_null() && !exiting
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
            msg = if method as libc::c_uint == PREPEND as libc::c_int as libc::c_uint {
                dcgettext(
                    0 as *const libc::c_char,
                    b"Prepend Selection to File\0" as *const u8 as *const libc::c_char,
                    5 as libc::c_int,
                )
            } else if method as libc::c_uint == APPEND as libc::c_int as libc::c_uint {
                dcgettext(
                    0 as *const libc::c_char,
                    b"Append Selection to File\0" as *const u8 as *const libc::c_char,
                    5 as libc::c_int,
                )
            } else {
                dcgettext(
                    0 as *const libc::c_char,
                    b"Write Selection to File\0" as *const u8 as *const libc::c_char,
                    5 as libc::c_int,
                )
            };
        } else if method as libc::c_uint != OVERWRITE as libc::c_int as libc::c_uint {
            msg = if method as libc::c_uint == PREPEND as libc::c_int as libc::c_uint {
                dcgettext(
                    0 as *const libc::c_char,
                    b"File Name to Prepend to\0" as *const u8 as *const libc::c_char,
                    5 as libc::c_int,
                )
            } else {
                dcgettext(
                    0 as *const libc::c_char,
                    b"File Name to Append to\0" as *const u8 as *const libc::c_char,
                    5 as libc::c_int,
                )
            };
        } else {
            msg = dcgettext(
                0 as *const libc::c_char,
                b"File Name to Write\0" as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            );
        }
        present_path = mallocstrcpy(
            present_path,
            b"./\0" as *const u8 as *const libc::c_char,
        );
        if (!withprompt
            || flags[(SAVE_ON_EXIT as libc::c_int as libc::c_ulong)
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
                && exiting as libc::c_int != 0)
            && *((*openfile).filename).offset(0 as libc::c_int as isize) as libc::c_int
                != '\0' as i32
        {
            answer = mallocstrcpy(answer, (*openfile).filename);
        } else {
            response = do_prompt(
                (1 as libc::c_int) << 5 as libc::c_int,
                given,
                0 as *mut *mut linestruct,
                Some(edit_refresh as unsafe extern "C" fn() -> ()),
                b"%s%s%s\0" as *const u8 as *const libc::c_char,
                msg,
                formatstr,
                backupstr,
            );
        }
        if response < 0 as libc::c_int {
            statusbar(
                dcgettext(
                    0 as *const libc::c_char,
                    b"Cancelled\0" as *const u8 as *const libc::c_char,
                    5 as libc::c_int,
                ),
            );
            rpl_free(given as *mut libc::c_void);
            return 0 as libc::c_int;
        }
        function = func_from_key(response);
        if function == Some(discard_buffer as unsafe extern "C" fn() -> ()) {
            rpl_free(given as *mut libc::c_void);
            return 2 as libc::c_int;
        }
        given = mallocstrcpy(given, answer);
        if function == Some(to_files as unsafe extern "C" fn() -> ()) {
            let mut chosen: *mut libc::c_char = browse_in(answer);
            if chosen.is_null() {
                continue;
            }
            rpl_free(answer as *mut libc::c_void);
            answer = chosen;
        } else if function == Some(dos_format as unsafe extern "C" fn() -> ()) {
            (*openfile)
                .fmt = (if (*openfile).fmt as libc::c_uint
                == DOS_FILE as libc::c_int as libc::c_uint
            {
                NIX_FILE as libc::c_int
            } else {
                DOS_FILE as libc::c_int
            }) as format_type;
            continue;
        } else if function == Some(mac_format as unsafe extern "C" fn() -> ()) {
            (*openfile)
                .fmt = (if (*openfile).fmt as libc::c_uint
                == MAC_FILE as libc::c_int as libc::c_uint
            {
                NIX_FILE as libc::c_int
            } else {
                MAC_FILE as libc::c_int
            }) as format_type;
            continue;
        } else if function == Some(back_it_up as unsafe extern "C" fn() -> ()) {
            flags[(MAKE_BACKUP as libc::c_int as libc::c_ulong)
                .wrapping_div(
                    (::std::mem::size_of::<libc::c_uint>() as libc::c_ulong)
                        .wrapping_mul(8 as libc::c_int as libc::c_ulong),
                ) as usize]
                ^= (1 as libc::c_int as libc::c_uint)
                    << (MAKE_BACKUP as libc::c_int as libc::c_ulong)
                        .wrapping_rem(
                            (::std::mem::size_of::<libc::c_uint>() as libc::c_ulong)
                                .wrapping_mul(8 as libc::c_int as libc::c_ulong),
                        );
            continue;
        } else if function == Some(prepend_it as unsafe extern "C" fn() -> ())
            || function == Some(append_it as unsafe extern "C" fn() -> ())
        {
            if function == Some(prepend_it as unsafe extern "C" fn() -> ()) {
                method = (if method as libc::c_uint
                    == PREPEND as libc::c_int as libc::c_uint
                {
                    OVERWRITE as libc::c_int
                } else {
                    PREPEND as libc::c_int
                }) as kind_of_writing_type;
            } else {
                method = (if method as libc::c_uint
                    == APPEND as libc::c_int as libc::c_uint
                {
                    OVERWRITE as libc::c_int
                } else {
                    APPEND as libc::c_int
                }) as kind_of_writing_type;
            }
            if strcmp(answer, (*openfile).filename) == 0 as libc::c_int {
                *given.offset(0 as libc::c_int as isize) = '\0' as i32 as libc::c_char;
            }
            continue;
        } else if function == Some(do_help as unsafe extern "C" fn() -> ()) {
            continue;
        }
        if exiting as libc::c_int != 0
            && !(flags[(SAVE_ON_EXIT as libc::c_int as libc::c_ulong)
                .wrapping_div(
                    (::std::mem::size_of::<libc::c_uint>() as libc::c_ulong)
                        .wrapping_mul(8 as libc::c_int as libc::c_ulong),
                ) as usize]
                & (1 as libc::c_int as libc::c_uint)
                    << (SAVE_ON_EXIT as libc::c_int as libc::c_ulong)
                        .wrapping_rem(
                            (::std::mem::size_of::<libc::c_uint>() as libc::c_ulong)
                                .wrapping_mul(8 as libc::c_int as libc::c_ulong),
                        ) != 0 as libc::c_int as libc::c_uint)
            && *((*openfile).filename).offset(0 as libc::c_int as isize) as libc::c_int
                == '\0' as i32
            && strcmp(answer, b"zzy\0" as *const u8 as *const libc::c_char)
                == 0 as libc::c_int && !did_credits
        {
            if LINES > 5 as libc::c_int && COLS > 31 as libc::c_int {
                do_credits();
                did_credits = 1 as libc::c_int != 0;
            } else {
                statusline(
                    AHEM,
                    dcgettext(
                        0 as *const libc::c_char,
                        b"Too tiny\0" as *const u8 as *const libc::c_char,
                        5 as libc::c_int,
                    ),
                );
            }
            rpl_free(given as *mut libc::c_void);
            return 0 as libc::c_int;
        }
        if method as libc::c_uint == OVERWRITE as libc::c_int as libc::c_uint {
            let mut name_exists: bool = false;
            let mut do_warning: bool = false;
            let mut full_answer: *mut libc::c_char = 0 as *mut libc::c_char;
            let mut full_filename: *mut libc::c_char = 0 as *mut libc::c_char;
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
            full_answer = get_full_path(answer);
            full_filename = get_full_path((*openfile).filename);
            name_exists = stat(
                if full_answer.is_null() { answer } else { full_answer },
                &mut fileinfo,
            ) != -(1 as libc::c_int);
            if *((*openfile).filename).offset(0 as libc::c_int as isize) as libc::c_int
                == '\0' as i32
            {
                do_warning = name_exists;
            } else {
                do_warning = strcmp(
                    if full_answer.is_null() { answer } else { full_answer },
                    if full_filename.is_null() {
                        (*openfile).filename
                    } else {
                        full_filename
                    },
                ) != 0 as libc::c_int;
            }
            rpl_free(full_filename as *mut libc::c_void);
            rpl_free(full_answer as *mut libc::c_void);
            if do_warning {
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
                    warn_and_briefly_pause(
                        dcgettext(
                            0 as *const libc::c_char,
                            b"File exists -- cannot overwrite\0" as *const u8
                                as *const libc::c_char,
                            5 as libc::c_int,
                        ),
                    );
                    continue;
                } else {
                    if !maychange {
                        if exiting as libc::c_int != 0 || ((*openfile).mark).is_null() {
                            if ask_user(
                                0 as libc::c_int != 0,
                                dcgettext(
                                    0 as *const libc::c_char,
                                    b"Save file under DIFFERENT NAME? \0" as *const u8
                                        as *const libc::c_char,
                                    5 as libc::c_int,
                                ),
                            ) != 1 as libc::c_int
                            {
                                continue;
                            }
                            maychange = 1 as libc::c_int != 0;
                        }
                    }
                    if name_exists {
                        let mut question: *mut libc::c_char = dcgettext(
                            0 as *const libc::c_char,
                            b"File \"%s\" exists; OVERWRITE? \0" as *const u8
                                as *const libc::c_char,
                            5 as libc::c_int,
                        );
                        let mut name: *mut libc::c_char = crop_to_fit(
                            answer,
                            (COLS as libc::c_ulong)
                                .wrapping_sub(breadth(question))
                                .wrapping_add(1 as libc::c_int as libc::c_ulong)
                                as libc::c_int,
                        );
                        let mut message: *mut libc::c_char = nmalloc(
                            (strlen(question))
                                .wrapping_add(strlen(name))
                                .wrapping_add(1 as libc::c_int as libc::c_ulong),
                        ) as *mut libc::c_char;
                        sprintf(message, question, name);
                        choice = ask_user(0 as libc::c_int != 0, message);
                        rpl_free(message as *mut libc::c_void);
                        rpl_free(name as *mut libc::c_void);
                        if choice != 1 as libc::c_int {
                            continue;
                        }
                    }
                }
            } else if name_exists as libc::c_int != 0
                && !((*openfile).statinfo).is_null()
                && ((*(*openfile).statinfo).st_mtim.tv_sec < fileinfo.st_mtim.tv_sec
                    || (*(*openfile).statinfo).st_dev != fileinfo.st_dev
                    || (*(*openfile).statinfo).st_ino != fileinfo.st_ino)
            {
                warn_and_briefly_pause(
                    dcgettext(
                        0 as *const libc::c_char,
                        b"File on disk has changed\0" as *const u8
                            as *const libc::c_char,
                        5 as libc::c_int,
                    ),
                );
                choice = ask_user(
                    0 as libc::c_int != 0,
                    dcgettext(
                        0 as *const libc::c_char,
                        b"File was modified since you opened it; continue saving? \0"
                            as *const u8 as *const libc::c_char,
                        5 as libc::c_int,
                    ),
                );
                wipe_statusbar();
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
                    && withprompt as libc::c_int != 0
                {
                    rpl_free(given as *mut libc::c_void);
                    if choice == 1 as libc::c_int {
                        return write_file(
                            (*openfile).filename,
                            0 as *mut FILE,
                            1 as libc::c_int != 0,
                            OVERWRITE,
                            0 as libc::c_int != 0,
                        ) as libc::c_int
                    } else if choice == 0 as libc::c_int {
                        return 2 as libc::c_int
                    } else {
                        return 0 as libc::c_int
                    }
                } else {
                    if choice == -(1 as libc::c_int) && exiting as libc::c_int != 0 {
                        continue;
                    }
                    if choice != 1 as libc::c_int {
                        rpl_free(given as *mut libc::c_void);
                        return 1 as libc::c_int;
                    }
                }
            }
        }
        rpl_free(given as *mut libc::c_void);
        break;
    }
    if !((*openfile).mark).is_null() && withprompt as libc::c_int != 0 && !exiting
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
        return write_region_to_file(
            answer,
            0 as *mut FILE,
            1 as libc::c_int != 0,
            method,
        ) as libc::c_int
    } else {
        return write_file(
            answer,
            0 as *mut FILE,
            1 as libc::c_int != 0,
            method,
            1 as libc::c_int != 0,
        ) as libc::c_int
    };
}
pub unsafe extern "C" fn do_writeout() {
    if write_it_out(0 as libc::c_int != 0, 1 as libc::c_int != 0) == 2 as libc::c_int {
        close_and_go();
    }
}
pub unsafe extern "C" fn do_savefile() {
    if write_it_out(0 as libc::c_int != 0, 0 as libc::c_int != 0) == 2 as libc::c_int {
        close_and_go();
    }
}
pub unsafe extern "C" fn real_dir_from_tilde(
    mut path: *const libc::c_char,
) -> *mut libc::c_char {
    let mut tilded: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut retval: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut i: size_t = 1 as libc::c_int as size_t;
    if *path as libc::c_int != '~' as i32 {
        return copy_of(path);
    }
    while *path.offset(i as isize) as libc::c_int != '/' as i32
        && *path.offset(i as isize) as libc::c_int != '\0' as i32
    {
        i = i.wrapping_add(1);
        i;
    }
    if i == 1 as libc::c_int as libc::c_ulong {
        get_homedir();
        tilded = copy_of(homedir);
    } else {
        let mut userdata: *const passwd = 0 as *const passwd;
        tilded = measured_copy(path, i);
        loop {
            userdata = getpwent();
            if !(!userdata.is_null()
                && strcmp((*userdata).pw_name, tilded.offset(1 as libc::c_int as isize))
                    != 0 as libc::c_int)
            {
                break;
            }
        }
        endpwent();
        if !userdata.is_null() {
            tilded = mallocstrcpy(tilded, (*userdata).pw_dir);
        }
    }
    retval = nmalloc(
        (strlen(tilded))
            .wrapping_add(strlen(path.offset(i as isize)))
            .wrapping_add(1 as libc::c_int as libc::c_ulong),
    ) as *mut libc::c_char;
    sprintf(
        retval,
        b"%s%s\0" as *const u8 as *const libc::c_char,
        tilded,
        path.offset(i as isize),
    );
    rpl_free(tilded as *mut libc::c_void);
    return retval;
}
pub unsafe extern "C" fn diralphasort(
    mut va: *const libc::c_void,
    mut vb: *const libc::c_void,
) -> libc::c_int {
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
    let mut a: *const libc::c_char = *(va as *const *const libc::c_char);
    let mut b: *const libc::c_char = *(vb as *const *const libc::c_char);
    let mut aisdir: bool = stat(a, &mut fileinfo) != -(1 as libc::c_int)
        && fileinfo.st_mode & 0o170000 as libc::c_int as libc::c_uint
            == 0o40000 as libc::c_int as libc::c_uint;
    let mut bisdir: bool = stat(b, &mut fileinfo) != -(1 as libc::c_int)
        && fileinfo.st_mode & 0o170000 as libc::c_int as libc::c_uint
            == 0o40000 as libc::c_int as libc::c_uint;
    if aisdir as libc::c_int != 0 && !bisdir {
        return -(1 as libc::c_int);
    }
    if !aisdir && bisdir as libc::c_int != 0 {
        return 1 as libc::c_int;
    }
    let mut difference: libc::c_int = mbstrcasecmp(a, b);
    if difference == 0 as libc::c_int { return strcmp(a, b) } else { return difference };
}
pub unsafe extern "C" fn is_dir(mut path: *const libc::c_char) -> bool {
    let mut thepath: *mut libc::c_char = real_dir_from_tilde(path);
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
    let mut retval: bool = false;
    retval = stat(thepath, &mut fileinfo) != -(1 as libc::c_int)
        && fileinfo.st_mode & 0o170000 as libc::c_int as libc::c_uint
            == 0o40000 as libc::c_int as libc::c_uint;
    rpl_free(thepath as *mut libc::c_void);
    return retval;
}
pub unsafe extern "C" fn username_completion(
    mut morsel: *const libc::c_char,
    mut length: size_t,
    mut num_matches: *mut size_t,
) -> *mut *mut libc::c_char {
    let mut matches: *mut *mut libc::c_char = 0 as *mut *mut libc::c_char;
    let mut userdata: *const passwd = 0 as *const passwd;
    loop {
        userdata = getpwent();
        if userdata.is_null() {
            break;
        }
        if !(strncmp(
            (*userdata).pw_name,
            morsel.offset(1 as libc::c_int as isize),
            length.wrapping_sub(1 as libc::c_int as libc::c_ulong),
        ) == 0 as libc::c_int)
        {
            continue;
        }
        if outside_of_confinement((*userdata).pw_dir, 1 as libc::c_int != 0) {
            continue;
        }
        matches = nrealloc(
            matches as *mut libc::c_void,
            (*num_matches)
                .wrapping_add(1 as libc::c_int as libc::c_ulong)
                .wrapping_mul(
                    ::std::mem::size_of::<*mut libc::c_char>() as libc::c_ulong,
                ),
        ) as *mut *mut libc::c_char;
        let ref mut fresh2 = *matches.offset(*num_matches as isize);
        *fresh2 = nmalloc(
            (strlen((*userdata).pw_name)).wrapping_add(2 as libc::c_int as libc::c_ulong),
        ) as *mut libc::c_char;
        sprintf(
            *matches.offset(*num_matches as isize),
            b"~%s\0" as *const u8 as *const libc::c_char,
            (*userdata).pw_name,
        );
        *num_matches = (*num_matches).wrapping_add(1);
        *num_matches;
    }
    endpwent();
    return matches;
}
pub unsafe extern "C" fn filename_completion(
    mut morsel: *const libc::c_char,
    mut num_matches: *mut size_t,
) -> *mut *mut libc::c_char {
    let mut dirname_0: *mut libc::c_char = copy_of(morsel);
    let mut slash: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut filename: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut filenamelen: size_t = 0;
    let mut fullname: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut matches: *mut *mut libc::c_char = 0 as *mut *mut libc::c_char;
    let mut entry: *const dirent = 0 as *const dirent;
    let mut dir: *mut DIR = 0 as *mut DIR;
    slash = strrchr(dirname_0, '/' as i32);
    if !slash.is_null() {
        let mut wasdirname: *mut libc::c_char = dirname_0;
        slash = slash.offset(1);
        filename = copy_of(slash);
        *slash = '\0' as i32 as libc::c_char;
        dirname_0 = real_dir_from_tilde(dirname_0);
        if *dirname_0.offset(0 as libc::c_int as isize) as libc::c_int != '/' as i32 {
            dirname_0 = nrealloc(
                dirname_0 as *mut libc::c_void,
                (strlen(present_path))
                    .wrapping_add(strlen(wasdirname))
                    .wrapping_add(1 as libc::c_int as libc::c_ulong),
            ) as *mut libc::c_char;
            sprintf(
                dirname_0,
                b"%s%s\0" as *const u8 as *const libc::c_char,
                present_path,
                wasdirname,
            );
        }
        rpl_free(wasdirname as *mut libc::c_void);
    } else {
        filename = dirname_0;
        dirname_0 = copy_of(present_path);
    }
    dir = opendir(dirname_0);
    if dir.is_null() {
        beep();
        rpl_free(filename as *mut libc::c_void);
        rpl_free(dirname_0 as *mut libc::c_void);
        return 0 as *mut *mut libc::c_char;
    }
    filenamelen = strlen(filename);
    loop {
        entry = readdir(dir);
        if entry.is_null() {
            break;
        }
        if !(strncmp(((*entry).d_name).as_ptr(), filename, filenamelen)
            == 0 as libc::c_int
            && strcmp(
                ((*entry).d_name).as_ptr(),
                b".\0" as *const u8 as *const libc::c_char,
            ) != 0 as libc::c_int
            && strcmp(
                ((*entry).d_name).as_ptr(),
                b"..\0" as *const u8 as *const libc::c_char,
            ) != 0 as libc::c_int)
        {
            continue;
        }
        fullname = nrealloc(
            fullname as *mut libc::c_void,
            (strlen(dirname_0))
                .wrapping_add(strlen(((*entry).d_name).as_ptr()))
                .wrapping_add(1 as libc::c_int as libc::c_ulong),
        ) as *mut libc::c_char;
        sprintf(
            fullname,
            b"%s%s\0" as *const u8 as *const libc::c_char,
            dirname_0,
            ((*entry).d_name).as_ptr(),
        );
        if outside_of_confinement(fullname, 1 as libc::c_int != 0) {
            continue;
        }
        if currmenu == (1 as libc::c_int) << 12 as libc::c_int && !is_dir(fullname) {
            continue;
        }
        matches = nrealloc(
            matches as *mut libc::c_void,
            (*num_matches)
                .wrapping_add(1 as libc::c_int as libc::c_ulong)
                .wrapping_mul(
                    ::std::mem::size_of::<*mut libc::c_char>() as libc::c_ulong,
                ),
        ) as *mut *mut libc::c_char;
        let ref mut fresh3 = *matches.offset(*num_matches as isize);
        *fresh3 = copy_of(((*entry).d_name).as_ptr());
        *num_matches = (*num_matches).wrapping_add(1);
        *num_matches;
    }
    closedir(dir);
    rpl_free(dirname_0 as *mut libc::c_void);
    rpl_free(filename as *mut libc::c_void);
    rpl_free(fullname as *mut libc::c_void);
    return matches;
}
pub unsafe extern "C" fn input_tab(
    mut morsel: *mut libc::c_char,
    mut place: *mut size_t,
    mut refresh_func: Option::<unsafe extern "C" fn() -> ()>,
    mut listed: *mut bool,
) -> *mut libc::c_char {
    let mut num_matches: size_t = 0 as libc::c_int as size_t;
    let mut matches: *mut *mut libc::c_char = 0 as *mut *mut libc::c_char;
    if *morsel.offset(*place as isize) as libc::c_int != '\0' as i32 {
        beep();
        return morsel;
    }
    if *morsel.offset(0 as libc::c_int as isize) as libc::c_int == '~' as i32
        && (strchr(morsel, '/' as i32)).is_null()
    {
        matches = username_completion(morsel, *place, &mut num_matches);
    }
    if matches.is_null() {
        matches = filename_completion(morsel, &mut num_matches);
    }
    if *listed as libc::c_int != 0 && num_matches < 2 as libc::c_int as libc::c_ulong {
        refresh_func.unwrap()();
        *listed = 0 as libc::c_int != 0;
    }
    if matches.is_null() {
        beep();
        return morsel;
    }
    let mut lastslash: *const libc::c_char = revstrstr(
        morsel,
        b"/\0" as *const u8 as *const libc::c_char,
        morsel.offset(*place as isize),
    );
    let mut length_of_path: size_t = (if lastslash.is_null() {
        0 as libc::c_int as libc::c_long
    } else {
        lastslash.offset_from(morsel) as libc::c_long + 1 as libc::c_int as libc::c_long
    }) as size_t;
    let mut match_0: size_t = 0;
    let mut common_len: size_t = 0 as libc::c_int as size_t;
    let mut shared: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut glued: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut char1: [libc::c_char; 4] = [0; 4];
    let mut char2: [libc::c_char; 4] = [0; 4];
    let mut len1: libc::c_int = 0;
    let mut len2: libc::c_int = 0;
    loop {
        len1 = collect_char(
            (*matches.offset(0 as libc::c_int as isize)).offset(common_len as isize),
            char1.as_mut_ptr(),
        );
        match_0 = 1 as libc::c_int as size_t;
        while match_0 < num_matches {
            len2 = collect_char(
                (*matches.offset(match_0 as isize)).offset(common_len as isize),
                char2.as_mut_ptr(),
            );
            if len1 != len2
                || strncmp(char1.as_mut_ptr(), char2.as_mut_ptr(), len2 as libc::c_ulong)
                    != 0 as libc::c_int
            {
                break;
            }
            match_0 = match_0.wrapping_add(1);
            match_0;
        }
        if match_0 < num_matches
            || *(*matches.offset(0 as libc::c_int as isize)).offset(common_len as isize)
                as libc::c_int == '\0' as i32
        {
            break;
        }
        common_len = (common_len as libc::c_ulong).wrapping_add(len1 as libc::c_ulong)
            as size_t as size_t;
    }
    shared = nmalloc(
        length_of_path
            .wrapping_add(common_len)
            .wrapping_add(1 as libc::c_int as libc::c_ulong),
    ) as *mut libc::c_char;
    strncpy(shared, morsel, length_of_path);
    strncpy(
        shared.offset(length_of_path as isize),
        *matches.offset(0 as libc::c_int as isize),
        common_len,
    );
    common_len = (common_len as libc::c_ulong).wrapping_add(length_of_path) as size_t
        as size_t;
    *shared.offset(common_len as isize) = '\0' as i32 as libc::c_char;
    glued = nmalloc(
        (strlen(present_path))
            .wrapping_add(common_len)
            .wrapping_add(1 as libc::c_int as libc::c_ulong),
    ) as *mut libc::c_char;
    sprintf(glued, b"%s%s\0" as *const u8 as *const libc::c_char, present_path, shared);
    if num_matches == 1 as libc::c_int as libc::c_ulong
        && (is_dir(shared) as libc::c_int != 0 || is_dir(glued) as libc::c_int != 0)
    {
        let fresh4 = common_len;
        common_len = common_len.wrapping_add(1);
        *shared.offset(fresh4 as isize) = '/' as i32 as libc::c_char;
    }
    if common_len != *place {
        morsel = nrealloc(
            morsel as *mut libc::c_void,
            common_len.wrapping_add(1 as libc::c_int as libc::c_ulong),
        ) as *mut libc::c_char;
        strncpy(morsel, shared, common_len);
        *morsel.offset(common_len as isize) = '\0' as i32 as libc::c_char;
        *place = common_len;
    } else if num_matches == 1 as libc::c_int as libc::c_ulong {
        beep();
    }
    if num_matches > 1 as libc::c_int as libc::c_ulong {
        let mut lastrow: libc::c_int = editwinrows - 1 as libc::c_int
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
            });
        let mut longest_name: size_t = 0 as libc::c_int as size_t;
        let mut nrows: size_t = 0;
        let mut ncols: size_t = 0;
        let mut row: libc::c_int = 0;
        if !*listed {
            beep();
        }
        qsort(
            matches as *mut libc::c_void,
            num_matches,
            ::std::mem::size_of::<*mut libc::c_char>() as libc::c_ulong,
            Some(
                diralphasort
                    as unsafe extern "C" fn(
                        *const libc::c_void,
                        *const libc::c_void,
                    ) -> libc::c_int,
            ),
        );
        match_0 = 0 as libc::c_int as size_t;
        while match_0 < num_matches {
            let mut namelen: size_t = breadth(*matches.offset(match_0 as isize));
            if namelen > longest_name {
                longest_name = namelen;
            }
            match_0 = match_0.wrapping_add(1);
            match_0;
        }
        if longest_name > (COLS - 1 as libc::c_int) as libc::c_ulong {
            longest_name = (COLS - 1 as libc::c_int) as size_t;
        }
        ncols = ((COLS + 1 as libc::c_int) as libc::c_ulong)
            .wrapping_div(longest_name.wrapping_add(2 as libc::c_int as libc::c_ulong));
        nrows = num_matches
            .wrapping_add(ncols)
            .wrapping_sub(1 as libc::c_int as libc::c_ulong)
            .wrapping_div(ncols);
        row = (if nrows < lastrow as libc::c_ulong {
            (lastrow as libc::c_ulong).wrapping_sub(nrows)
        } else {
            0 as libc::c_int as libc::c_ulong
        }) as libc::c_int;
        blank_edit();
        curs_set(0 as libc::c_int);
        match_0 = 0 as libc::c_int as size_t;
        while match_0 < num_matches {
            let mut disp: *mut libc::c_char = 0 as *mut libc::c_char;
            wmove(
                midwin,
                row,
                longest_name
                    .wrapping_add(2 as libc::c_int as libc::c_ulong)
                    .wrapping_mul(match_0.wrapping_rem(ncols)) as libc::c_int,
            );
            if row == lastrow
                && match_0
                    .wrapping_add(1 as libc::c_int as libc::c_ulong)
                    .wrapping_rem(ncols) == 0 as libc::c_int as libc::c_ulong
                && match_0.wrapping_add(1 as libc::c_int as libc::c_ulong) < num_matches
            {
                waddnstr(
                    midwin,
                    dcgettext(
                        0 as *const libc::c_char,
                        b"(more)\0" as *const u8 as *const libc::c_char,
                        5 as libc::c_int,
                    ),
                    -(1 as libc::c_int),
                );
                break;
            } else {
                disp = display_string(
                    *matches.offset(match_0 as isize),
                    0 as libc::c_int as size_t,
                    longest_name,
                    0 as libc::c_int != 0,
                    0 as libc::c_int != 0,
                );
                waddnstr(midwin, disp, -(1 as libc::c_int));
                rpl_free(disp as *mut libc::c_void);
                if match_0
                    .wrapping_add(1 as libc::c_int as libc::c_ulong)
                    .wrapping_rem(ncols) == 0 as libc::c_int as libc::c_ulong
                {
                    row += 1;
                    row;
                }
                match_0 = match_0.wrapping_add(1);
                match_0;
            }
        }
        wnoutrefresh(midwin);
        *listed = 1 as libc::c_int != 0;
    }
    free_chararray(matches, num_matches);
    rpl_free(glued as *mut libc::c_void);
    rpl_free(shared as *mut libc::c_void);
    return morsel;
}
