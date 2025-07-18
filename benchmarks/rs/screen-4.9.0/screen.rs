use ::libc;
use ::c2rust_bitfields;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    fn select(
        __nfds: libc::c_int,
        __readfds: *mut fd_set,
        __writefds: *mut fd_set,
        __exceptfds: *mut fd_set,
        __timeout: *mut timeval,
    ) -> libc::c_int;
    fn memcpy(
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
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    fn strerror(_: libc::c_int) -> *mut libc::c_char;
    fn bcopy(__src: *const libc::c_void, __dest: *mut libc::c_void, __n: size_t);
    fn bzero(_: *mut libc::c_void, _: libc::c_ulong);
    fn index(_: *const libc::c_char, _: libc::c_int) -> *mut libc::c_char;
    fn access(__name: *const libc::c_char, __type: libc::c_int) -> libc::c_int;
    fn close(__fd: libc::c_int) -> libc::c_int;
    fn read(__fd: libc::c_int, __buf: *mut libc::c_void, __nbytes: size_t) -> ssize_t;
    fn write(__fd: libc::c_int, __buf: *const libc::c_void, __n: size_t) -> ssize_t;
    fn sleep(__seconds: libc::c_uint) -> libc::c_uint;
    fn chown(
        __file: *const libc::c_char,
        __owner: __uid_t,
        __group: __gid_t,
    ) -> libc::c_int;
    fn dup(__fd: libc::c_int) -> libc::c_int;
    static mut environ: *mut *mut libc::c_char;
    fn _exit(_: libc::c_int) -> !;
    fn getpid() -> __pid_t;
    fn getppid() -> __pid_t;
    fn getuid() -> __uid_t;
    fn geteuid() -> __uid_t;
    fn getgid() -> __gid_t;
    fn getegid() -> __gid_t;
    fn setuid(__uid: __uid_t) -> libc::c_int;
    fn setgid(__gid: __gid_t) -> libc::c_int;
    fn fork() -> __pid_t;
    fn unlink(__name: *const libc::c_char) -> libc::c_int;
    fn getlogin() -> *mut libc::c_char;
    fn gethostname(__name: *mut libc::c_char, __len: size_t) -> libc::c_int;
    fn ttyname(__fd: libc::c_int) -> *mut libc::c_char;
    fn readlink(
        __path: *const libc::c_char,
        __buf: *mut libc::c_char,
        __len: size_t,
    ) -> ssize_t;
    fn fcntl(__fd: libc::c_int, __cmd: libc::c_int, _: ...) -> libc::c_int;
    fn chmod(__file: *const libc::c_char, __mode: __mode_t) -> libc::c_int;
    fn umask(__mask: __mode_t) -> __mode_t;
    fn mkdir(__path: *const libc::c_char, __mode: __mode_t) -> libc::c_int;
    fn __xstat(
        __ver: libc::c_int,
        __filename: *const libc::c_char,
        __stat_buf: *mut stat,
    ) -> libc::c_int;
    fn kill(__pid: __pid_t, __sig: libc::c_int) -> libc::c_int;
    fn killpg(__pgrp: __pid_t, __sig: libc::c_int) -> libc::c_int;
    fn setlocale(
        __category: libc::c_int,
        __locale: *const libc::c_char,
    ) -> *mut libc::c_char;
    fn nl_langinfo(__item: nl_item) -> *mut libc::c_char;
    fn strtol(
        _: *const libc::c_char,
        _: *mut *mut libc::c_char,
        _: libc::c_int,
    ) -> libc::c_long;
    fn __errno_location() -> *mut libc::c_int;
    fn vsnprintf(
        _: *mut libc::c_char,
        _: libc::c_ulong,
        _: *const libc::c_char,
        _: ::std::ffi::VaList,
    ) -> libc::c_int;
    fn snprintf(
        _: *mut libc::c_char,
        _: libc::c_ulong,
        _: *const libc::c_char,
        _: ...
    ) -> libc::c_int;
    fn sprintf(_: *mut libc::c_char, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    fn free(__ptr: *mut libc::c_void);
    fn abort() -> !;
    fn exit(_: libc::c_int) -> !;
    fn getenv(__name: *const libc::c_char) -> *mut libc::c_char;
    fn time(__timer: *mut time_t) -> time_t;
    fn strftime(
        __s: *mut libc::c_char,
        __maxsize: size_t,
        __format: *const libc::c_char,
        __tp: *const tm,
    ) -> size_t;
    fn localtime(__timer: *const time_t) -> *mut tm;
    fn freopen(
        __filename: *const libc::c_char,
        __modes: *const libc::c_char,
        __stream: *mut FILE,
    ) -> *mut FILE;
    fn gettimeofday(__tv: *mut timeval, __tz: *mut libc::c_void) -> libc::c_int;
    fn waitpid(
        __pid: __pid_t,
        __stat_loc: *mut libc::c_int,
        __options: libc::c_int,
    ) -> __pid_t;
    static mut DefaultEsc: libc::c_int;
    static mut DefaultMetaEsc: libc::c_int;
    fn SetCanvasWindow(_: *mut canvas, _: *mut win);
    fn MakeDefaultCanvas() -> libc::c_int;
    fn RethinkViewportOffsets(_: *mut canvas);
    fn AutosaveLayout(_: *mut layout);
    static mut stdin: *mut FILE;
    static mut stdout: *mut FILE;
    static mut stderr: *mut FILE;
    fn ctime(__timer: *const time_t) -> *mut libc::c_char;
    fn getpwuid(__uid: __uid_t) -> *mut passwd;
    fn getpwnam(__name: *const libc::c_char) -> *mut passwd;
    fn endspent();
    fn getspnam(__name: *const libc::c_char) -> *mut spwd;
    fn islogfile(name: *mut libc::c_char) -> libc::c_int;
    fn logfclose(_: *mut logfile) -> libc::c_int;
    fn logfwrite(_: *mut logfile, _: *mut libc::c_char, _: libc::c_int) -> libc::c_int;
    fn logfflush(ifany: *mut logfile) -> libc::c_int;
    fn logreopen_register(
        fn_0: Option::<
            unsafe extern "C" fn(
                *mut libc::c_char,
                libc::c_int,
                *mut logfile,
            ) -> libc::c_int,
        >,
    );
    fn lf_move_fd(fd: libc::c_int, wantfd: libc::c_int) -> libc::c_int;
    static mut Term: [libc::c_char; 0];
    static mut screenterm: [libc::c_char; 0];
    static mut displays: *mut display;
    static mut display: *mut display;
    static mut MarkLf: LayFuncs;
    static mut visual_bell: libc::c_int;
    static mut mark_key_tab: [libc::c_uchar; 0];
    static mut version: [libc::c_char; 0];
    static mut DefaultShell: [libc::c_char; 0];
    static mut zmodem_sendcmd: *mut libc::c_char;
    static mut zmodem_recvcmd: *mut libc::c_char;
    static mut layout_last: *mut layout;
    static mut nwin_undef: NewWindow;
    static mut nwin_default: NewWindow;
    static mut nwin_options: NewWindow;
    fn Kill(_: libc::c_int, _: libc::c_int);
    fn ResetWindow(_: *mut win);
    fn WriteString(_: *mut win, _: *mut libc::c_char, _: libc::c_int);
    fn WindowChanged(_: *mut win, _: libc::c_int);
    fn MFindUsedLine(_: *mut win, _: libc::c_int, _: libc::c_int) -> libc::c_int;
    fn StartRc(_: *mut libc::c_char, _: libc::c_int) -> libc::c_int;
    fn FinishRc(_: *mut libc::c_char);
    fn secopen(_: *mut libc::c_char, _: libc::c_int, _: libc::c_int) -> libc::c_int;
    fn readpipe(_: *mut *mut libc::c_char) -> libc::c_int;
    fn GetTTY(_: libc::c_int, _: *mut mode);
    fn SetTTY(_: libc::c_int, _: *mut mode);
    fn SetMode(_: *mut mode, _: *mut mode, _: libc::c_int, _: libc::c_int);
    fn TtyGrabConsole(
        _: libc::c_int,
        _: libc::c_int,
        _: *mut libc::c_char,
    ) -> libc::c_int;
    fn brktty(_: libc::c_int);
    fn CheckTtyname(_: *mut libc::c_char) -> libc::c_int;
    fn exit_with_usage(_: *mut libc::c_char, _: *mut libc::c_char, _: *mut libc::c_char);
    fn display_copyright();
    fn MakeWindow(_: *mut NewWindow) -> libc::c_int;
    fn FreeWindow(_: *mut win);
    fn FreePseudowin(_: *mut win);
    fn nwin_compose(_: *mut NewWindow, _: *mut NewWindow, _: *mut NewWindow);
    fn ReleaseAutoWritelock(_: *mut display, _: *mut win) -> libc::c_int;
    fn CloseDevice(_: *mut win);
    fn InitUtmp();
    fn RemoveLoginSlot();
    fn RestoreLoginSlot();
    fn RemoveUtmp(_: *mut win) -> libc::c_int;
    fn InitLoadav();
    fn AddLoadav(_: *mut libc::c_char);
    fn InitKeytab();
    fn KillWindow(_: *mut win);
    fn SetEscape(_: *mut acluser, _: libc::c_int, _: libc::c_int);
    fn AddWindows(
        _: *mut libc::c_char,
        _: libc::c_int,
        _: libc::c_int,
        _: libc::c_int,
    ) -> *mut libc::c_char;
    fn AddWindowFlags(
        _: *mut libc::c_char,
        _: libc::c_int,
        _: *mut win,
    ) -> *mut libc::c_char;
    fn AddOtherUsers(
        _: *mut libc::c_char,
        _: libc::c_int,
        _: *mut win,
    ) -> *mut libc::c_char;
    fn CompileKeys(
        _: *mut libc::c_char,
        _: libc::c_int,
        _: *mut libc::c_uchar,
    ) -> libc::c_int;
    fn ParseAttrColor(
        _: *mut libc::c_char,
        _: *mut libc::c_char,
        _: libc::c_int,
    ) -> libc::c_int;
    fn ApplyAttrColor(_: libc::c_int, _: *mut mchar);
    fn InitTermcap(_: libc::c_int, _: libc::c_int) -> libc::c_int;
    fn MakeTermcap(_: libc::c_int) -> *mut libc::c_char;
    fn Attach(_: libc::c_int) -> libc::c_int;
    fn Attacher();
    fn AttacherFinit(_: libc::c_int);
    fn SendCmdMessage(
        _: *mut libc::c_char,
        _: *mut libc::c_char,
        _: *mut *mut libc::c_char,
        _: libc::c_int,
    );
    fn MakeDisplay(
        _: *mut libc::c_char,
        _: *mut libc::c_char,
        _: *mut libc::c_char,
        _: libc::c_int,
        _: libc::c_int,
        _: *mut mode,
    ) -> *mut display;
    fn FreeDisplay();
    fn InitTerm(_: libc::c_int);
    fn FinitTerm();
    fn PUTCHARLP(_: libc::c_int);
    fn ClearAll();
    fn SetRendition(_: *mut mchar);
    fn MakeStatus(_: *mut libc::c_char);
    fn RemoveStatus();
    fn AddStr(_: *mut libc::c_char);
    fn Flush(_: libc::c_int);
    fn freetty();
    fn evenq(_: *mut event);
    fn evdeq(_: *mut event);
    fn SetTimeout(_: *mut event, _: libc::c_int);
    fn sched();
    fn FindSocket(
        _: *mut libc::c_int,
        _: *mut libc::c_int,
        _: *mut libc::c_int,
        _: *mut libc::c_char,
        _: *mut bool,
    ) -> libc::c_int;
    fn MakeServerSocket(_: bool) -> libc::c_int;
    fn RecoverSocket() -> libc::c_int;
    fn chsock() -> libc::c_int;
    fn ReceiveMsg();
    fn SendCreateMsg(_: *mut libc::c_char, _: *mut NewWindow);
    fn SendErrorMsg(_: *mut libc::c_char, _: *mut libc::c_char) -> libc::c_int;
    fn SaveStr(_: *const libc::c_char) -> *mut libc::c_char;
    fn stripdev(_: *mut libc::c_char) -> *mut libc::c_char;
    fn closeallfiles(_: libc::c_int);
    fn UserContext() -> libc::c_int;
    fn UserReturn(_: libc::c_int);
    fn UserStatus() -> libc::c_int;
    fn xsignal(
        _: libc::c_int,
        _: Option::<unsafe extern "C" fn(libc::c_int) -> ()>,
    ) -> Option::<unsafe extern "C" fn(libc::c_int) -> ()>;
    fn xseteuid(_: libc::c_int);
    fn xsetegid(_: libc::c_int);
    fn UserAdd(
        _: *mut libc::c_char,
        _: *mut libc::c_char,
        _: *mut *mut acluser,
    ) -> libc::c_int;
    fn LGotoPos(_: *mut layer, _: libc::c_int, _: libc::c_int);
    fn LScrollH(
        _: *mut layer,
        _: libc::c_int,
        _: libc::c_int,
        _: libc::c_int,
        _: libc::c_int,
        _: libc::c_int,
        _: *mut mline,
    );
    fn LScrollV(
        _: *mut layer,
        _: libc::c_int,
        _: libc::c_int,
        _: libc::c_int,
        _: libc::c_int,
    );
    fn DoNLS(_: *const libc::c_char) -> *const libc::c_char;
    fn InitBuiltinTabs();
    fn FindEncoding(_: *mut libc::c_char) -> libc::c_int;
    fn EncodingName(_: libc::c_int) -> *mut libc::c_char;
    fn RecodeBuf(
        _: *mut libc::c_uchar,
        _: libc::c_int,
        _: libc::c_int,
        _: libc::c_int,
        _: *mut libc::c_uchar,
    ) -> libc::c_int;
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
pub type __pid_t = libc::c_int;
pub type __time_t = libc::c_long;
pub type __suseconds_t = libc::c_long;
pub type __blksize_t = libc::c_long;
pub type __blkcnt_t = libc::c_long;
pub type __ssize_t = libc::c_long;
pub type __syscall_slong_t = libc::c_long;
pub type pid_t = __pid_t;
pub type ssize_t = __ssize_t;
pub type time_t = __time_t;
pub type size_t = libc::c_ulong;
pub type int32_t = __int32_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct timeval {
    pub tv_sec: __time_t,
    pub tv_usec: __suseconds_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct timespec {
    pub tv_sec: __time_t,
    pub tv_nsec: __syscall_slong_t,
}
pub type __fd_mask = libc::c_long;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct fd_set {
    pub fds_bits: [__fd_mask; 16],
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
pub type __sighandler_t = Option::<unsafe extern "C" fn(libc::c_int) -> ()>;
pub type nl_item = libc::c_int;
pub type C2RustUnnamed = libc::c_uint;
pub const _NL_NUM: C2RustUnnamed = 786449;
pub const _NL_NUM_LC_IDENTIFICATION: C2RustUnnamed = 786448;
pub const _NL_IDENTIFICATION_CODESET: C2RustUnnamed = 786447;
pub const _NL_IDENTIFICATION_CATEGORY: C2RustUnnamed = 786446;
pub const _NL_IDENTIFICATION_DATE: C2RustUnnamed = 786445;
pub const _NL_IDENTIFICATION_REVISION: C2RustUnnamed = 786444;
pub const _NL_IDENTIFICATION_ABBREVIATION: C2RustUnnamed = 786443;
pub const _NL_IDENTIFICATION_APPLICATION: C2RustUnnamed = 786442;
pub const _NL_IDENTIFICATION_AUDIENCE: C2RustUnnamed = 786441;
pub const _NL_IDENTIFICATION_TERRITORY: C2RustUnnamed = 786440;
pub const _NL_IDENTIFICATION_LANGUAGE: C2RustUnnamed = 786439;
pub const _NL_IDENTIFICATION_FAX: C2RustUnnamed = 786438;
pub const _NL_IDENTIFICATION_TEL: C2RustUnnamed = 786437;
pub const _NL_IDENTIFICATION_EMAIL: C2RustUnnamed = 786436;
pub const _NL_IDENTIFICATION_CONTACT: C2RustUnnamed = 786435;
pub const _NL_IDENTIFICATION_ADDRESS: C2RustUnnamed = 786434;
pub const _NL_IDENTIFICATION_SOURCE: C2RustUnnamed = 786433;
pub const _NL_IDENTIFICATION_TITLE: C2RustUnnamed = 786432;
pub const _NL_NUM_LC_MEASUREMENT: C2RustUnnamed = 720898;
pub const _NL_MEASUREMENT_CODESET: C2RustUnnamed = 720897;
pub const _NL_MEASUREMENT_MEASUREMENT: C2RustUnnamed = 720896;
pub const _NL_NUM_LC_TELEPHONE: C2RustUnnamed = 655365;
pub const _NL_TELEPHONE_CODESET: C2RustUnnamed = 655364;
pub const _NL_TELEPHONE_INT_PREFIX: C2RustUnnamed = 655363;
pub const _NL_TELEPHONE_INT_SELECT: C2RustUnnamed = 655362;
pub const _NL_TELEPHONE_TEL_DOM_FMT: C2RustUnnamed = 655361;
pub const _NL_TELEPHONE_TEL_INT_FMT: C2RustUnnamed = 655360;
pub const _NL_NUM_LC_ADDRESS: C2RustUnnamed = 589837;
pub const _NL_ADDRESS_CODESET: C2RustUnnamed = 589836;
pub const _NL_ADDRESS_LANG_LIB: C2RustUnnamed = 589835;
pub const _NL_ADDRESS_LANG_TERM: C2RustUnnamed = 589834;
pub const _NL_ADDRESS_LANG_AB: C2RustUnnamed = 589833;
pub const _NL_ADDRESS_LANG_NAME: C2RustUnnamed = 589832;
pub const _NL_ADDRESS_COUNTRY_ISBN: C2RustUnnamed = 589831;
pub const _NL_ADDRESS_COUNTRY_NUM: C2RustUnnamed = 589830;
pub const _NL_ADDRESS_COUNTRY_CAR: C2RustUnnamed = 589829;
pub const _NL_ADDRESS_COUNTRY_AB3: C2RustUnnamed = 589828;
pub const _NL_ADDRESS_COUNTRY_AB2: C2RustUnnamed = 589827;
pub const _NL_ADDRESS_COUNTRY_POST: C2RustUnnamed = 589826;
pub const _NL_ADDRESS_COUNTRY_NAME: C2RustUnnamed = 589825;
pub const _NL_ADDRESS_POSTAL_FMT: C2RustUnnamed = 589824;
pub const _NL_NUM_LC_NAME: C2RustUnnamed = 524295;
pub const _NL_NAME_CODESET: C2RustUnnamed = 524294;
pub const _NL_NAME_NAME_MS: C2RustUnnamed = 524293;
pub const _NL_NAME_NAME_MISS: C2RustUnnamed = 524292;
pub const _NL_NAME_NAME_MRS: C2RustUnnamed = 524291;
pub const _NL_NAME_NAME_MR: C2RustUnnamed = 524290;
pub const _NL_NAME_NAME_GEN: C2RustUnnamed = 524289;
pub const _NL_NAME_NAME_FMT: C2RustUnnamed = 524288;
pub const _NL_NUM_LC_PAPER: C2RustUnnamed = 458755;
pub const _NL_PAPER_CODESET: C2RustUnnamed = 458754;
pub const _NL_PAPER_WIDTH: C2RustUnnamed = 458753;
pub const _NL_PAPER_HEIGHT: C2RustUnnamed = 458752;
pub const _NL_NUM_LC_MESSAGES: C2RustUnnamed = 327685;
pub const _NL_MESSAGES_CODESET: C2RustUnnamed = 327684;
pub const __NOSTR: C2RustUnnamed = 327683;
pub const __YESSTR: C2RustUnnamed = 327682;
pub const __NOEXPR: C2RustUnnamed = 327681;
pub const __YESEXPR: C2RustUnnamed = 327680;
pub const _NL_NUM_LC_NUMERIC: C2RustUnnamed = 65542;
pub const _NL_NUMERIC_CODESET: C2RustUnnamed = 65541;
pub const _NL_NUMERIC_THOUSANDS_SEP_WC: C2RustUnnamed = 65540;
pub const _NL_NUMERIC_DECIMAL_POINT_WC: C2RustUnnamed = 65539;
pub const __GROUPING: C2RustUnnamed = 65538;
pub const THOUSEP: C2RustUnnamed = 65537;
pub const __THOUSANDS_SEP: C2RustUnnamed = 65537;
pub const RADIXCHAR: C2RustUnnamed = 65536;
pub const __DECIMAL_POINT: C2RustUnnamed = 65536;
pub const _NL_NUM_LC_MONETARY: C2RustUnnamed = 262190;
pub const _NL_MONETARY_CODESET: C2RustUnnamed = 262189;
pub const _NL_MONETARY_THOUSANDS_SEP_WC: C2RustUnnamed = 262188;
pub const _NL_MONETARY_DECIMAL_POINT_WC: C2RustUnnamed = 262187;
pub const _NL_MONETARY_CONVERSION_RATE: C2RustUnnamed = 262186;
pub const _NL_MONETARY_DUO_VALID_TO: C2RustUnnamed = 262185;
pub const _NL_MONETARY_DUO_VALID_FROM: C2RustUnnamed = 262184;
pub const _NL_MONETARY_UNO_VALID_TO: C2RustUnnamed = 262183;
pub const _NL_MONETARY_UNO_VALID_FROM: C2RustUnnamed = 262182;
pub const _NL_MONETARY_DUO_INT_N_SIGN_POSN: C2RustUnnamed = 262181;
pub const _NL_MONETARY_DUO_INT_P_SIGN_POSN: C2RustUnnamed = 262180;
pub const _NL_MONETARY_DUO_N_SIGN_POSN: C2RustUnnamed = 262179;
pub const _NL_MONETARY_DUO_P_SIGN_POSN: C2RustUnnamed = 262178;
pub const _NL_MONETARY_DUO_INT_N_SEP_BY_SPACE: C2RustUnnamed = 262177;
pub const _NL_MONETARY_DUO_INT_N_CS_PRECEDES: C2RustUnnamed = 262176;
pub const _NL_MONETARY_DUO_INT_P_SEP_BY_SPACE: C2RustUnnamed = 262175;
pub const _NL_MONETARY_DUO_INT_P_CS_PRECEDES: C2RustUnnamed = 262174;
pub const _NL_MONETARY_DUO_N_SEP_BY_SPACE: C2RustUnnamed = 262173;
pub const _NL_MONETARY_DUO_N_CS_PRECEDES: C2RustUnnamed = 262172;
pub const _NL_MONETARY_DUO_P_SEP_BY_SPACE: C2RustUnnamed = 262171;
pub const _NL_MONETARY_DUO_P_CS_PRECEDES: C2RustUnnamed = 262170;
pub const _NL_MONETARY_DUO_FRAC_DIGITS: C2RustUnnamed = 262169;
pub const _NL_MONETARY_DUO_INT_FRAC_DIGITS: C2RustUnnamed = 262168;
pub const _NL_MONETARY_DUO_CURRENCY_SYMBOL: C2RustUnnamed = 262167;
pub const _NL_MONETARY_DUO_INT_CURR_SYMBOL: C2RustUnnamed = 262166;
pub const __INT_N_SIGN_POSN: C2RustUnnamed = 262165;
pub const __INT_P_SIGN_POSN: C2RustUnnamed = 262164;
pub const __INT_N_SEP_BY_SPACE: C2RustUnnamed = 262163;
pub const __INT_N_CS_PRECEDES: C2RustUnnamed = 262162;
pub const __INT_P_SEP_BY_SPACE: C2RustUnnamed = 262161;
pub const __INT_P_CS_PRECEDES: C2RustUnnamed = 262160;
pub const _NL_MONETARY_CRNCYSTR: C2RustUnnamed = 262159;
pub const __N_SIGN_POSN: C2RustUnnamed = 262158;
pub const __P_SIGN_POSN: C2RustUnnamed = 262157;
pub const __N_SEP_BY_SPACE: C2RustUnnamed = 262156;
pub const __N_CS_PRECEDES: C2RustUnnamed = 262155;
pub const __P_SEP_BY_SPACE: C2RustUnnamed = 262154;
pub const __P_CS_PRECEDES: C2RustUnnamed = 262153;
pub const __FRAC_DIGITS: C2RustUnnamed = 262152;
pub const __INT_FRAC_DIGITS: C2RustUnnamed = 262151;
pub const __NEGATIVE_SIGN: C2RustUnnamed = 262150;
pub const __POSITIVE_SIGN: C2RustUnnamed = 262149;
pub const __MON_GROUPING: C2RustUnnamed = 262148;
pub const __MON_THOUSANDS_SEP: C2RustUnnamed = 262147;
pub const __MON_DECIMAL_POINT: C2RustUnnamed = 262146;
pub const __CURRENCY_SYMBOL: C2RustUnnamed = 262145;
pub const __INT_CURR_SYMBOL: C2RustUnnamed = 262144;
pub const _NL_NUM_LC_CTYPE: C2RustUnnamed = 86;
pub const _NL_CTYPE_EXTRA_MAP_14: C2RustUnnamed = 85;
pub const _NL_CTYPE_EXTRA_MAP_13: C2RustUnnamed = 84;
pub const _NL_CTYPE_EXTRA_MAP_12: C2RustUnnamed = 83;
pub const _NL_CTYPE_EXTRA_MAP_11: C2RustUnnamed = 82;
pub const _NL_CTYPE_EXTRA_MAP_10: C2RustUnnamed = 81;
pub const _NL_CTYPE_EXTRA_MAP_9: C2RustUnnamed = 80;
pub const _NL_CTYPE_EXTRA_MAP_8: C2RustUnnamed = 79;
pub const _NL_CTYPE_EXTRA_MAP_7: C2RustUnnamed = 78;
pub const _NL_CTYPE_EXTRA_MAP_6: C2RustUnnamed = 77;
pub const _NL_CTYPE_EXTRA_MAP_5: C2RustUnnamed = 76;
pub const _NL_CTYPE_EXTRA_MAP_4: C2RustUnnamed = 75;
pub const _NL_CTYPE_EXTRA_MAP_3: C2RustUnnamed = 74;
pub const _NL_CTYPE_EXTRA_MAP_2: C2RustUnnamed = 73;
pub const _NL_CTYPE_EXTRA_MAP_1: C2RustUnnamed = 72;
pub const _NL_CTYPE_NONASCII_CASE: C2RustUnnamed = 71;
pub const _NL_CTYPE_MAP_TO_NONASCII: C2RustUnnamed = 70;
pub const _NL_CTYPE_TRANSLIT_IGNORE: C2RustUnnamed = 69;
pub const _NL_CTYPE_TRANSLIT_IGNORE_LEN: C2RustUnnamed = 68;
pub const _NL_CTYPE_TRANSLIT_DEFAULT_MISSING: C2RustUnnamed = 67;
pub const _NL_CTYPE_TRANSLIT_DEFAULT_MISSING_LEN: C2RustUnnamed = 66;
pub const _NL_CTYPE_TRANSLIT_TO_TBL: C2RustUnnamed = 65;
pub const _NL_CTYPE_TRANSLIT_TO_IDX: C2RustUnnamed = 64;
pub const _NL_CTYPE_TRANSLIT_FROM_TBL: C2RustUnnamed = 63;
pub const _NL_CTYPE_TRANSLIT_FROM_IDX: C2RustUnnamed = 62;
pub const _NL_CTYPE_TRANSLIT_TAB_SIZE: C2RustUnnamed = 61;
pub const _NL_CTYPE_OUTDIGIT9_WC: C2RustUnnamed = 60;
pub const _NL_CTYPE_OUTDIGIT8_WC: C2RustUnnamed = 59;
pub const _NL_CTYPE_OUTDIGIT7_WC: C2RustUnnamed = 58;
pub const _NL_CTYPE_OUTDIGIT6_WC: C2RustUnnamed = 57;
pub const _NL_CTYPE_OUTDIGIT5_WC: C2RustUnnamed = 56;
pub const _NL_CTYPE_OUTDIGIT4_WC: C2RustUnnamed = 55;
pub const _NL_CTYPE_OUTDIGIT3_WC: C2RustUnnamed = 54;
pub const _NL_CTYPE_OUTDIGIT2_WC: C2RustUnnamed = 53;
pub const _NL_CTYPE_OUTDIGIT1_WC: C2RustUnnamed = 52;
pub const _NL_CTYPE_OUTDIGIT0_WC: C2RustUnnamed = 51;
pub const _NL_CTYPE_OUTDIGIT9_MB: C2RustUnnamed = 50;
pub const _NL_CTYPE_OUTDIGIT8_MB: C2RustUnnamed = 49;
pub const _NL_CTYPE_OUTDIGIT7_MB: C2RustUnnamed = 48;
pub const _NL_CTYPE_OUTDIGIT6_MB: C2RustUnnamed = 47;
pub const _NL_CTYPE_OUTDIGIT5_MB: C2RustUnnamed = 46;
pub const _NL_CTYPE_OUTDIGIT4_MB: C2RustUnnamed = 45;
pub const _NL_CTYPE_OUTDIGIT3_MB: C2RustUnnamed = 44;
pub const _NL_CTYPE_OUTDIGIT2_MB: C2RustUnnamed = 43;
pub const _NL_CTYPE_OUTDIGIT1_MB: C2RustUnnamed = 42;
pub const _NL_CTYPE_OUTDIGIT0_MB: C2RustUnnamed = 41;
pub const _NL_CTYPE_INDIGITS9_WC: C2RustUnnamed = 40;
pub const _NL_CTYPE_INDIGITS8_WC: C2RustUnnamed = 39;
pub const _NL_CTYPE_INDIGITS7_WC: C2RustUnnamed = 38;
pub const _NL_CTYPE_INDIGITS6_WC: C2RustUnnamed = 37;
pub const _NL_CTYPE_INDIGITS5_WC: C2RustUnnamed = 36;
pub const _NL_CTYPE_INDIGITS4_WC: C2RustUnnamed = 35;
pub const _NL_CTYPE_INDIGITS3_WC: C2RustUnnamed = 34;
pub const _NL_CTYPE_INDIGITS2_WC: C2RustUnnamed = 33;
pub const _NL_CTYPE_INDIGITS1_WC: C2RustUnnamed = 32;
pub const _NL_CTYPE_INDIGITS0_WC: C2RustUnnamed = 31;
pub const _NL_CTYPE_INDIGITS_WC_LEN: C2RustUnnamed = 30;
pub const _NL_CTYPE_INDIGITS9_MB: C2RustUnnamed = 29;
pub const _NL_CTYPE_INDIGITS8_MB: C2RustUnnamed = 28;
pub const _NL_CTYPE_INDIGITS7_MB: C2RustUnnamed = 27;
pub const _NL_CTYPE_INDIGITS6_MB: C2RustUnnamed = 26;
pub const _NL_CTYPE_INDIGITS5_MB: C2RustUnnamed = 25;
pub const _NL_CTYPE_INDIGITS4_MB: C2RustUnnamed = 24;
pub const _NL_CTYPE_INDIGITS3_MB: C2RustUnnamed = 23;
pub const _NL_CTYPE_INDIGITS2_MB: C2RustUnnamed = 22;
pub const _NL_CTYPE_INDIGITS1_MB: C2RustUnnamed = 21;
pub const _NL_CTYPE_INDIGITS0_MB: C2RustUnnamed = 20;
pub const _NL_CTYPE_INDIGITS_MB_LEN: C2RustUnnamed = 19;
pub const _NL_CTYPE_MAP_OFFSET: C2RustUnnamed = 18;
pub const _NL_CTYPE_CLASS_OFFSET: C2RustUnnamed = 17;
pub const _NL_CTYPE_TOLOWER32: C2RustUnnamed = 16;
pub const _NL_CTYPE_TOUPPER32: C2RustUnnamed = 15;
pub const CODESET: C2RustUnnamed = 14;
pub const _NL_CTYPE_CODESET_NAME: C2RustUnnamed = 14;
pub const _NL_CTYPE_MB_CUR_MAX: C2RustUnnamed = 13;
pub const _NL_CTYPE_WIDTH: C2RustUnnamed = 12;
pub const _NL_CTYPE_MAP_NAMES: C2RustUnnamed = 11;
pub const _NL_CTYPE_CLASS_NAMES: C2RustUnnamed = 10;
pub const _NL_CTYPE_GAP6: C2RustUnnamed = 9;
pub const _NL_CTYPE_GAP5: C2RustUnnamed = 8;
pub const _NL_CTYPE_GAP4: C2RustUnnamed = 7;
pub const _NL_CTYPE_GAP3: C2RustUnnamed = 6;
pub const _NL_CTYPE_CLASS32: C2RustUnnamed = 5;
pub const _NL_CTYPE_GAP2: C2RustUnnamed = 4;
pub const _NL_CTYPE_TOLOWER: C2RustUnnamed = 3;
pub const _NL_CTYPE_GAP1: C2RustUnnamed = 2;
pub const _NL_CTYPE_TOUPPER: C2RustUnnamed = 1;
pub const _NL_CTYPE_CLASS: C2RustUnnamed = 0;
pub const _NL_NUM_LC_COLLATE: C2RustUnnamed = 196627;
pub const _NL_COLLATE_CODESET: C2RustUnnamed = 196626;
pub const _NL_COLLATE_COLLSEQWC: C2RustUnnamed = 196625;
pub const _NL_COLLATE_COLLSEQMB: C2RustUnnamed = 196624;
pub const _NL_COLLATE_SYMB_EXTRAMB: C2RustUnnamed = 196623;
pub const _NL_COLLATE_SYMB_TABLEMB: C2RustUnnamed = 196622;
pub const _NL_COLLATE_SYMB_HASH_SIZEMB: C2RustUnnamed = 196621;
pub const _NL_COLLATE_INDIRECTWC: C2RustUnnamed = 196620;
pub const _NL_COLLATE_EXTRAWC: C2RustUnnamed = 196619;
pub const _NL_COLLATE_WEIGHTWC: C2RustUnnamed = 196618;
pub const _NL_COLLATE_TABLEWC: C2RustUnnamed = 196617;
pub const _NL_COLLATE_GAP3: C2RustUnnamed = 196616;
pub const _NL_COLLATE_GAP2: C2RustUnnamed = 196615;
pub const _NL_COLLATE_GAP1: C2RustUnnamed = 196614;
pub const _NL_COLLATE_INDIRECTMB: C2RustUnnamed = 196613;
pub const _NL_COLLATE_EXTRAMB: C2RustUnnamed = 196612;
pub const _NL_COLLATE_WEIGHTMB: C2RustUnnamed = 196611;
pub const _NL_COLLATE_TABLEMB: C2RustUnnamed = 196610;
pub const _NL_COLLATE_RULESETS: C2RustUnnamed = 196609;
pub const _NL_COLLATE_NRULES: C2RustUnnamed = 196608;
pub const _NL_NUM_LC_TIME: C2RustUnnamed = 131231;
pub const _NL_WABALTMON_12: C2RustUnnamed = 131230;
pub const _NL_WABALTMON_11: C2RustUnnamed = 131229;
pub const _NL_WABALTMON_10: C2RustUnnamed = 131228;
pub const _NL_WABALTMON_9: C2RustUnnamed = 131227;
pub const _NL_WABALTMON_8: C2RustUnnamed = 131226;
pub const _NL_WABALTMON_7: C2RustUnnamed = 131225;
pub const _NL_WABALTMON_6: C2RustUnnamed = 131224;
pub const _NL_WABALTMON_5: C2RustUnnamed = 131223;
pub const _NL_WABALTMON_4: C2RustUnnamed = 131222;
pub const _NL_WABALTMON_3: C2RustUnnamed = 131221;
pub const _NL_WABALTMON_2: C2RustUnnamed = 131220;
pub const _NL_WABALTMON_1: C2RustUnnamed = 131219;
pub const _NL_ABALTMON_12: C2RustUnnamed = 131218;
pub const _NL_ABALTMON_11: C2RustUnnamed = 131217;
pub const _NL_ABALTMON_10: C2RustUnnamed = 131216;
pub const _NL_ABALTMON_9: C2RustUnnamed = 131215;
pub const _NL_ABALTMON_8: C2RustUnnamed = 131214;
pub const _NL_ABALTMON_7: C2RustUnnamed = 131213;
pub const _NL_ABALTMON_6: C2RustUnnamed = 131212;
pub const _NL_ABALTMON_5: C2RustUnnamed = 131211;
pub const _NL_ABALTMON_4: C2RustUnnamed = 131210;
pub const _NL_ABALTMON_3: C2RustUnnamed = 131209;
pub const _NL_ABALTMON_2: C2RustUnnamed = 131208;
pub const _NL_ABALTMON_1: C2RustUnnamed = 131207;
pub const _NL_WALTMON_12: C2RustUnnamed = 131206;
pub const _NL_WALTMON_11: C2RustUnnamed = 131205;
pub const _NL_WALTMON_10: C2RustUnnamed = 131204;
pub const _NL_WALTMON_9: C2RustUnnamed = 131203;
pub const _NL_WALTMON_8: C2RustUnnamed = 131202;
pub const _NL_WALTMON_7: C2RustUnnamed = 131201;
pub const _NL_WALTMON_6: C2RustUnnamed = 131200;
pub const _NL_WALTMON_5: C2RustUnnamed = 131199;
pub const _NL_WALTMON_4: C2RustUnnamed = 131198;
pub const _NL_WALTMON_3: C2RustUnnamed = 131197;
pub const _NL_WALTMON_2: C2RustUnnamed = 131196;
pub const _NL_WALTMON_1: C2RustUnnamed = 131195;
pub const __ALTMON_12: C2RustUnnamed = 131194;
pub const __ALTMON_11: C2RustUnnamed = 131193;
pub const __ALTMON_10: C2RustUnnamed = 131192;
pub const __ALTMON_9: C2RustUnnamed = 131191;
pub const __ALTMON_8: C2RustUnnamed = 131190;
pub const __ALTMON_7: C2RustUnnamed = 131189;
pub const __ALTMON_6: C2RustUnnamed = 131188;
pub const __ALTMON_5: C2RustUnnamed = 131187;
pub const __ALTMON_4: C2RustUnnamed = 131186;
pub const __ALTMON_3: C2RustUnnamed = 131185;
pub const __ALTMON_2: C2RustUnnamed = 131184;
pub const __ALTMON_1: C2RustUnnamed = 131183;
pub const _NL_TIME_CODESET: C2RustUnnamed = 131182;
pub const _NL_W_DATE_FMT: C2RustUnnamed = 131181;
pub const _DATE_FMT: C2RustUnnamed = 131180;
pub const _NL_TIME_TIMEZONE: C2RustUnnamed = 131179;
pub const _NL_TIME_CAL_DIRECTION: C2RustUnnamed = 131178;
pub const _NL_TIME_FIRST_WORKDAY: C2RustUnnamed = 131177;
pub const _NL_TIME_FIRST_WEEKDAY: C2RustUnnamed = 131176;
pub const _NL_TIME_WEEK_1STWEEK: C2RustUnnamed = 131175;
pub const _NL_TIME_WEEK_1STDAY: C2RustUnnamed = 131174;
pub const _NL_TIME_WEEK_NDAYS: C2RustUnnamed = 131173;
pub const _NL_WERA_T_FMT: C2RustUnnamed = 131172;
pub const _NL_WERA_D_T_FMT: C2RustUnnamed = 131171;
pub const _NL_WALT_DIGITS: C2RustUnnamed = 131170;
pub const _NL_WERA_D_FMT: C2RustUnnamed = 131169;
pub const _NL_WERA_YEAR: C2RustUnnamed = 131168;
pub const _NL_WT_FMT_AMPM: C2RustUnnamed = 131167;
pub const _NL_WT_FMT: C2RustUnnamed = 131166;
pub const _NL_WD_FMT: C2RustUnnamed = 131165;
pub const _NL_WD_T_FMT: C2RustUnnamed = 131164;
pub const _NL_WPM_STR: C2RustUnnamed = 131163;
pub const _NL_WAM_STR: C2RustUnnamed = 131162;
pub const _NL_WMON_12: C2RustUnnamed = 131161;
pub const _NL_WMON_11: C2RustUnnamed = 131160;
pub const _NL_WMON_10: C2RustUnnamed = 131159;
pub const _NL_WMON_9: C2RustUnnamed = 131158;
pub const _NL_WMON_8: C2RustUnnamed = 131157;
pub const _NL_WMON_7: C2RustUnnamed = 131156;
pub const _NL_WMON_6: C2RustUnnamed = 131155;
pub const _NL_WMON_5: C2RustUnnamed = 131154;
pub const _NL_WMON_4: C2RustUnnamed = 131153;
pub const _NL_WMON_3: C2RustUnnamed = 131152;
pub const _NL_WMON_2: C2RustUnnamed = 131151;
pub const _NL_WMON_1: C2RustUnnamed = 131150;
pub const _NL_WABMON_12: C2RustUnnamed = 131149;
pub const _NL_WABMON_11: C2RustUnnamed = 131148;
pub const _NL_WABMON_10: C2RustUnnamed = 131147;
pub const _NL_WABMON_9: C2RustUnnamed = 131146;
pub const _NL_WABMON_8: C2RustUnnamed = 131145;
pub const _NL_WABMON_7: C2RustUnnamed = 131144;
pub const _NL_WABMON_6: C2RustUnnamed = 131143;
pub const _NL_WABMON_5: C2RustUnnamed = 131142;
pub const _NL_WABMON_4: C2RustUnnamed = 131141;
pub const _NL_WABMON_3: C2RustUnnamed = 131140;
pub const _NL_WABMON_2: C2RustUnnamed = 131139;
pub const _NL_WABMON_1: C2RustUnnamed = 131138;
pub const _NL_WDAY_7: C2RustUnnamed = 131137;
pub const _NL_WDAY_6: C2RustUnnamed = 131136;
pub const _NL_WDAY_5: C2RustUnnamed = 131135;
pub const _NL_WDAY_4: C2RustUnnamed = 131134;
pub const _NL_WDAY_3: C2RustUnnamed = 131133;
pub const _NL_WDAY_2: C2RustUnnamed = 131132;
pub const _NL_WDAY_1: C2RustUnnamed = 131131;
pub const _NL_WABDAY_7: C2RustUnnamed = 131130;
pub const _NL_WABDAY_6: C2RustUnnamed = 131129;
pub const _NL_WABDAY_5: C2RustUnnamed = 131128;
pub const _NL_WABDAY_4: C2RustUnnamed = 131127;
pub const _NL_WABDAY_3: C2RustUnnamed = 131126;
pub const _NL_WABDAY_2: C2RustUnnamed = 131125;
pub const _NL_WABDAY_1: C2RustUnnamed = 131124;
pub const _NL_TIME_ERA_ENTRIES: C2RustUnnamed = 131123;
pub const _NL_TIME_ERA_NUM_ENTRIES: C2RustUnnamed = 131122;
pub const ERA_T_FMT: C2RustUnnamed = 131121;
pub const ERA_D_T_FMT: C2RustUnnamed = 131120;
pub const ALT_DIGITS: C2RustUnnamed = 131119;
pub const ERA_D_FMT: C2RustUnnamed = 131118;
pub const __ERA_YEAR: C2RustUnnamed = 131117;
pub const ERA: C2RustUnnamed = 131116;
pub const T_FMT_AMPM: C2RustUnnamed = 131115;
pub const T_FMT: C2RustUnnamed = 131114;
pub const D_FMT: C2RustUnnamed = 131113;
pub const D_T_FMT: C2RustUnnamed = 131112;
pub const PM_STR: C2RustUnnamed = 131111;
pub const AM_STR: C2RustUnnamed = 131110;
pub const MON_12: C2RustUnnamed = 131109;
pub const MON_11: C2RustUnnamed = 131108;
pub const MON_10: C2RustUnnamed = 131107;
pub const MON_9: C2RustUnnamed = 131106;
pub const MON_8: C2RustUnnamed = 131105;
pub const MON_7: C2RustUnnamed = 131104;
pub const MON_6: C2RustUnnamed = 131103;
pub const MON_5: C2RustUnnamed = 131102;
pub const MON_4: C2RustUnnamed = 131101;
pub const MON_3: C2RustUnnamed = 131100;
pub const MON_2: C2RustUnnamed = 131099;
pub const MON_1: C2RustUnnamed = 131098;
pub const ABMON_12: C2RustUnnamed = 131097;
pub const ABMON_11: C2RustUnnamed = 131096;
pub const ABMON_10: C2RustUnnamed = 131095;
pub const ABMON_9: C2RustUnnamed = 131094;
pub const ABMON_8: C2RustUnnamed = 131093;
pub const ABMON_7: C2RustUnnamed = 131092;
pub const ABMON_6: C2RustUnnamed = 131091;
pub const ABMON_5: C2RustUnnamed = 131090;
pub const ABMON_4: C2RustUnnamed = 131089;
pub const ABMON_3: C2RustUnnamed = 131088;
pub const ABMON_2: C2RustUnnamed = 131087;
pub const ABMON_1: C2RustUnnamed = 131086;
pub const DAY_7: C2RustUnnamed = 131085;
pub const DAY_6: C2RustUnnamed = 131084;
pub const DAY_5: C2RustUnnamed = 131083;
pub const DAY_4: C2RustUnnamed = 131082;
pub const DAY_3: C2RustUnnamed = 131081;
pub const DAY_2: C2RustUnnamed = 131080;
pub const DAY_1: C2RustUnnamed = 131079;
pub const ABDAY_7: C2RustUnnamed = 131078;
pub const ABDAY_6: C2RustUnnamed = 131077;
pub const ABDAY_5: C2RustUnnamed = 131076;
pub const ABDAY_4: C2RustUnnamed = 131075;
pub const ABDAY_3: C2RustUnnamed = 131074;
pub const ABDAY_2: C2RustUnnamed = 131073;
pub const ABDAY_1: C2RustUnnamed = 131072;
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
pub struct tm {
    pub tm_sec: libc::c_int,
    pub tm_min: libc::c_int,
    pub tm_hour: libc::c_int,
    pub tm_mday: libc::c_int,
    pub tm_mon: libc::c_int,
    pub tm_year: libc::c_int,
    pub tm_wday: libc::c_int,
    pub tm_yday: libc::c_int,
    pub tm_isdst: libc::c_int,
    pub tm_gmtoff: libc::c_long,
    pub tm_zone: *const libc::c_char,
}
pub type cc_t = libc::c_uchar;
pub type speed_t = libc::c_uint;
pub type tcflag_t = libc::c_uint;
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
pub type slot_t = *mut libc::c_char;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct exit_status {
    pub e_termination: libc::c_short,
    pub e_exit: libc::c_short,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct utmp {
    pub ut_type: libc::c_short,
    pub ut_pid: pid_t,
    pub ut_line: [libc::c_char; 32],
    pub ut_id: [libc::c_char; 4],
    pub ut_user: [libc::c_char; 32],
    pub ut_host: [libc::c_char; 256],
    pub ut_exit: exit_status,
    pub ut_session: int32_t,
    pub ut_tv: C2RustUnnamed_0,
    pub ut_addr_v6: [int32_t; 4],
    pub __glibc_reserved: [libc::c_char; 20],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_0 {
    pub tv_sec: int32_t,
    pub tv_usec: int32_t,
}
pub type state_t = libc::c_uint;
pub const PRIN4: state_t = 8;
pub const PRINCSI: state_t = 7;
pub const PRINESC: state_t = 6;
pub const PRIN: state_t = 5;
pub const CSI: state_t = 4;
pub const STRESC: state_t = 3;
pub const ASTR: state_t = 2;
pub const ESC: state_t = 1;
pub const LIT: state_t = 0;
pub type string_t = libc::c_uint;
pub const STATUS: string_t = 7;
pub const GM: string_t = 6;
pub const AKA: string_t = 5;
pub const PM: string_t = 4;
pub const APC: string_t = 3;
pub const OSC: string_t = 2;
pub const DCS: string_t = 1;
pub const NONE: string_t = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct event {
    pub next: *mut event,
    pub handler: Option::<unsafe extern "C" fn(*mut event, *mut libc::c_char) -> ()>,
    pub data: *mut libc::c_char,
    pub fd: libc::c_int,
    pub type_0: libc::c_int,
    pub pri: libc::c_int,
    pub timeout: timeval,
    pub queued: libc::c_int,
    pub active: libc::c_int,
    pub condpos: *mut libc::c_int,
    pub condneg: *mut libc::c_int,
}
pub type AclBits = *mut libc::c_uchar;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct aclusergroup {
    pub u: *mut acluser,
    pub next: *mut aclusergroup,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct acluser {
    pub u_next: *mut acluser,
    pub u_name: [libc::c_char; 257],
    pub u_password: *mut libc::c_char,
    pub u_checkpassword: libc::c_int,
    pub u_detachwin: libc::c_int,
    pub u_detachotherwin: libc::c_int,
    pub u_Esc: libc::c_int,
    pub u_MetaEsc: libc::c_int,
    pub u_plop: plop,
    pub u_id: libc::c_int,
    pub u_umask_w_bits: [AclBits; 3],
    pub u_group: *mut aclusergroup,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct plop {
    pub buf: *mut libc::c_char,
    pub len: libc::c_int,
    pub enc: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct action {
    pub nr: libc::c_int,
    pub args: *mut *mut libc::c_char,
    pub argl: *mut libc::c_int,
    pub quiet: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct mchar {
    pub image: libc::c_uchar,
    pub attr: libc::c_uchar,
    pub font: libc::c_uchar,
    pub fontx: libc::c_uchar,
    pub color: libc::c_uchar,
    pub mbcs: libc::c_uchar,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct LayFuncs {
    pub lf_LayProcess: Option::<
        unsafe extern "C" fn(*mut *mut libc::c_char, *mut libc::c_int) -> (),
    >,
    pub lf_LayAbort: Option::<unsafe extern "C" fn() -> ()>,
    pub lf_LayRedisplayLine: Option::<
        unsafe extern "C" fn(libc::c_int, libc::c_int, libc::c_int, libc::c_int) -> (),
    >,
    pub lf_LayClearLine: Option::<
        unsafe extern "C" fn(libc::c_int, libc::c_int, libc::c_int, libc::c_int) -> (),
    >,
    pub lf_LayRewrite: Option::<
        unsafe extern "C" fn(
            libc::c_int,
            libc::c_int,
            libc::c_int,
            *mut mchar,
            libc::c_int,
        ) -> libc::c_int,
    >,
    pub lf_LayResize: Option::<
        unsafe extern "C" fn(libc::c_int, libc::c_int) -> libc::c_int,
    >,
    pub lf_LayRestore: Option::<unsafe extern "C" fn() -> ()>,
    pub lf_LayFree: Option::<unsafe extern "C" fn(*mut libc::c_void) -> ()>,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct layer {
    pub l_cvlist: *mut canvas,
    pub l_width: libc::c_int,
    pub l_height: libc::c_int,
    pub l_x: libc::c_int,
    pub l_y: libc::c_int,
    pub l_encoding: libc::c_int,
    pub l_layfn: *mut LayFuncs,
    pub l_data: *mut libc::c_void,
    pub l_next: *mut layer,
    pub l_bottom: *mut layer,
    pub l_blocking: libc::c_int,
    pub l_mode: libc::c_int,
    pub l_mouseevent: C2RustUnnamed_2,
    pub l_pause: C2RustUnnamed_1,
}
#[derive(Copy, Clone, BitfieldStruct)]
#[repr(C)]
pub struct C2RustUnnamed_1 {
    #[bitfield(name = "d", ty = "libc::c_int", bits = "0..=0")]
    pub d: [u8; 1],
    #[bitfield(padding)]
    pub c2rust_padding: [u8; 7],
    pub left: *mut libc::c_int,
    pub right: *mut libc::c_int,
    pub top: libc::c_int,
    pub bottom: libc::c_int,
    pub lines: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_2 {
    pub buffer: [libc::c_uchar; 3],
    pub len: libc::c_int,
    pub start: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct canvas {
    pub c_next: *mut canvas,
    pub c_display: *mut display,
    pub c_slnext: *mut canvas,
    pub c_slprev: *mut canvas,
    pub c_slperp: *mut canvas,
    pub c_slback: *mut canvas,
    pub c_slorient: libc::c_int,
    pub c_slweight: libc::c_int,
    pub c_vplist: *mut viewport,
    pub c_layer: *mut layer,
    pub c_lnext: *mut canvas,
    pub c_blank: layer,
    pub c_xoff: libc::c_int,
    pub c_yoff: libc::c_int,
    pub c_xs: libc::c_int,
    pub c_xe: libc::c_int,
    pub c_ys: libc::c_int,
    pub c_ye: libc::c_int,
    pub c_captev: event,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct viewport {
    pub v_next: *mut viewport,
    pub v_canvas: *mut canvas,
    pub v_xoff: libc::c_int,
    pub v_yoff: libc::c_int,
    pub v_xs: libc::c_int,
    pub v_xe: libc::c_int,
    pub v_ys: libc::c_int,
    pub v_ye: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct display {
    pub d_next: *mut display,
    pub d_user: *mut acluser,
    pub d_canvas: canvas,
    pub d_cvlist: *mut canvas,
    pub d_forecv: *mut canvas,
    pub d_layout: *mut layout,
    pub d_processinput: Option::<
        unsafe extern "C" fn(*mut libc::c_char, libc::c_int) -> (),
    >,
    pub d_processinputdata: *mut libc::c_char,
    pub d_vpxmin: libc::c_int,
    pub d_vpxmax: libc::c_int,
    pub d_fore: *mut win,
    pub d_other: *mut win,
    pub d_nonblock: libc::c_int,
    pub d_termname: [libc::c_char; 33],
    pub d_tentry: *mut libc::c_char,
    pub d_tcinited: libc::c_char,
    pub d_width: libc::c_int,
    pub d_height: libc::c_int,
    pub d_defwidth: libc::c_int,
    pub d_defheight: libc::c_int,
    pub d_top: libc::c_int,
    pub d_bot: libc::c_int,
    pub d_x: libc::c_int,
    pub d_y: libc::c_int,
    pub d_rend: mchar,
    pub d_col16change: libc::c_int,
    pub d_atyp: libc::c_char,
    pub d_mbcs: libc::c_int,
    pub d_encoding: libc::c_int,
    pub d_decodestate: libc::c_int,
    pub d_realfont: libc::c_int,
    pub d_insert: libc::c_int,
    pub d_keypad: libc::c_int,
    pub d_cursorkeys: libc::c_int,
    pub d_revvid: libc::c_int,
    pub d_curvis: libc::c_int,
    pub d_has_hstatus: libc::c_int,
    pub d_hstatus: libc::c_int,
    pub d_lp_missing: libc::c_int,
    pub d_mouse: libc::c_int,
    pub d_extmouse: libc::c_int,
    pub d_mouse_parse: mouse_parse,
    pub d_mousetrack: libc::c_int,
    pub d_lpchar: mchar,
    pub d_status_time: timeval,
    pub d_status: libc::c_int,
    pub d_status_bell: libc::c_char,
    pub d_status_len: libc::c_int,
    pub d_status_lastmsg: *mut libc::c_char,
    pub d_status_buflen: libc::c_int,
    pub d_status_lastx: libc::c_int,
    pub d_status_lasty: libc::c_int,
    pub d_status_obuflen: libc::c_int,
    pub d_status_obuffree: libc::c_int,
    pub d_status_obufpos: libc::c_int,
    pub d_statusev: event,
    pub d_hstatusev: event,
    pub d_kaablamm: libc::c_int,
    pub d_ESCseen: *mut action,
    pub d_userpid: libc::c_int,
    pub d_usertty: [libc::c_char; 4096],
    pub d_userfd: libc::c_int,
    pub d_readev: event,
    pub d_writeev: event,
    pub d_blockedev: event,
    pub d_OldMode: mode,
    pub d_NewMode: mode,
    pub d_flow: libc::c_int,
    pub d_intrc: libc::c_int,
    pub d_obuf: *mut libc::c_char,
    pub d_obuflen: libc::c_int,
    pub d_obufmax: libc::c_int,
    pub d_obuflenmax: libc::c_int,
    pub d_obufp: *mut libc::c_char,
    pub d_obuffree: libc::c_int,
    pub d_auto_nuke: libc::c_int,
    pub d_nseqs: libc::c_int,
    pub d_aseqs: libc::c_int,
    pub d_kmaps: *mut libc::c_uchar,
    pub d_seqp: *mut libc::c_uchar,
    pub d_seql: libc::c_int,
    pub d_seqh: *mut libc::c_uchar,
    pub d_mapev: event,
    pub d_dontmap: libc::c_int,
    pub d_mapdefault: libc::c_int,
    pub d_tcs: [tcu; 201],
    pub d_attrtab: [*mut libc::c_char; 6],
    pub d_attrtyp: [libc::c_char; 6],
    pub d_hascolor: libc::c_int,
    pub d_dospeed: libc::c_short,
    pub d_c0_tab: [libc::c_char; 256],
    pub d_xtable: *mut *mut *mut libc::c_char,
    pub d_UPcost: libc::c_int,
    pub d_DOcost: libc::c_int,
    pub d_LEcost: libc::c_int,
    pub d_NDcost: libc::c_int,
    pub d_CRcost: libc::c_int,
    pub d_IMcost: libc::c_int,
    pub d_EIcost: libc::c_int,
    pub d_NLcost: libc::c_int,
    pub d_printfd: libc::c_int,
    pub d_loginslot: slot_t,
    pub d_utmp_logintty: utmp,
    pub d_loginttymode: libc::c_int,
    pub d_blocked: libc::c_int,
    pub d_blocked_fuzz: libc::c_int,
    pub d_idleev: event,
    pub d_blankerpid: libc::c_int,
    pub d_blankerev: event,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union tcu {
    pub flg: libc::c_int,
    pub num: libc::c_int,
    pub str_0: *mut libc::c_char,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct mode {
    pub tio: termios,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct mouse_parse {
    pub sgrmode: libc::c_char,
    pub state: libc::c_int,
    pub params: [libc::c_int; 3],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct win {
    pub w_next: *mut win,
    pub w_type: libc::c_int,
    pub w_data: *mut libc::c_void,
    pub w_layer: layer,
    pub w_savelayer: *mut layer,
    pub w_blocked: libc::c_int,
    pub w_pwin: *mut pseudowin,
    pub w_pdisplay: *mut display,
    pub w_lastdisp: *mut display,
    pub w_number: libc::c_int,
    pub w_readev: event,
    pub w_writeev: event,
    pub w_silenceev: event,
    pub w_zombieev: event,
    pub w_poll_zombie_timeout: libc::c_int,
    pub w_ptyfd: libc::c_int,
    pub w_inbuf: [libc::c_char; 4096],
    pub w_inlen: libc::c_int,
    pub w_outbuf: [libc::c_char; 4096],
    pub w_outlen: libc::c_int,
    pub w_aflag: libc::c_int,
    pub w_dynamicaka: libc::c_int,
    pub w_title: *mut libc::c_char,
    pub w_akachange: *mut libc::c_char,
    pub w_akabuf: [libc::c_char; 768],
    pub w_autoaka: libc::c_int,
    pub w_group: *mut win,
    pub w_intermediate: libc::c_int,
    pub w_args: [libc::c_int; 64],
    pub w_NumArgs: libc::c_int,
    pub w_wlock: libc::c_int,
    pub w_wlockuser: *mut acluser,
    pub w_userbits: [AclBits; 3],
    pub w_lio_notify: AclBits,
    pub w_mon_notify: AclBits,
    pub w_state: state_t,
    pub w_StringType: string_t,
    pub w_mlines: *mut mline,
    pub w_rend: mchar,
    pub w_FontL: libc::c_char,
    pub w_FontR: libc::c_char,
    pub w_FontE: libc::c_char,
    pub w_Charset: libc::c_int,
    pub w_CharsetR: libc::c_int,
    pub w_charsets: [libc::c_int; 4],
    pub w_ss: libc::c_int,
    pub w_saved: cursor,
    pub w_top: libc::c_int,
    pub w_bot: libc::c_int,
    pub w_wrap: libc::c_int,
    pub w_origin: libc::c_int,
    pub w_insert: libc::c_int,
    pub w_keypad: libc::c_int,
    pub w_cursorkeys: libc::c_int,
    pub w_revvid: libc::c_int,
    pub w_curinv: libc::c_int,
    pub w_curvvis: libc::c_int,
    pub w_autolf: libc::c_int,
    pub w_hstatus: *mut libc::c_char,
    pub w_gr: libc::c_int,
    pub w_c1: libc::c_int,
    pub w_bce: libc::c_int,
    pub w_decodestate: libc::c_int,
    pub w_mbcs: libc::c_int,
    pub w_string: [libc::c_char; 768],
    pub w_stringp: *mut libc::c_char,
    pub w_tabs: *mut libc::c_char,
    pub w_bell: libc::c_int,
    pub w_flow: libc::c_int,
    pub w_log: *mut logfile,
    pub w_logsilence: libc::c_int,
    pub w_monitor: libc::c_int,
    pub w_silencewait: libc::c_int,
    pub w_silence: libc::c_int,
    pub w_vbwait: libc::c_char,
    pub w_norefresh: libc::c_char,
    pub w_mouse: libc::c_int,
    pub w_extmouse: libc::c_int,
    pub w_slowpaste: libc::c_int,
    pub w_histheight: libc::c_int,
    pub w_histidx: libc::c_int,
    pub w_scrollback_height: libc::c_int,
    pub w_hlines: *mut mline,
    pub w_paster: paster,
    pub w_pid: libc::c_int,
    pub w_deadpid: libc::c_int,
    pub w_cmdargs: [*mut libc::c_char; 64],
    pub w_dir: *mut libc::c_char,
    pub w_term: *mut libc::c_char,
    pub w_lflag: libc::c_int,
    pub w_slot: slot_t,
    pub w_savut: utmp,
    pub w_tty: [libc::c_char; 768],
    pub w_zauto: libc::c_int,
    pub w_zdisplay: *mut display,
    pub w_alt: C2RustUnnamed_3,
    pub w_destroyev: event,
    pub w_exitstatus: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_3 {
    pub on: libc::c_int,
    pub mlines: *mut mline,
    pub width: libc::c_int,
    pub height: libc::c_int,
    pub histheight: libc::c_int,
    pub hlines: *mut mline,
    pub histidx: libc::c_int,
    pub cursor: cursor,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct cursor {
    pub on: libc::c_int,
    pub x: libc::c_int,
    pub y: libc::c_int,
    pub Rend: mchar,
    pub Charset: libc::c_int,
    pub CharsetR: libc::c_int,
    pub Charsets: [libc::c_int; 4],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct mline {
    pub image: *mut libc::c_uchar,
    pub attr: *mut libc::c_uchar,
    pub font: *mut libc::c_uchar,
    pub fontx: *mut libc::c_uchar,
    pub color: *mut libc::c_uchar,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct paster {
    pub pa_pastebuf: *mut libc::c_char,
    pub pa_pasteptr: *mut libc::c_char,
    pub pa_pastelen: libc::c_int,
    pub pa_pastelayer: *mut layer,
    pub pa_slowev: event,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct logfile {
    pub next: *mut logfile,
    pub fp: *mut FILE,
    pub name: *mut libc::c_char,
    pub opencount: libc::c_int,
    pub writecount: libc::c_int,
    pub flushcount: libc::c_int,
    pub st: *mut stat,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct pseudowin {
    pub p_fdpat: libc::c_int,
    pub p_pid: libc::c_int,
    pub p_ptyfd: libc::c_int,
    pub p_readev: event,
    pub p_writeev: event,
    pub p_cmd: [libc::c_char; 768],
    pub p_tty: [libc::c_char; 768],
    pub p_inbuf: [libc::c_char; 4096],
    pub p_inlen: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct layout {
    pub lay_next: *mut layout,
    pub lay_title: *mut libc::c_char,
    pub lay_number: libc::c_int,
    pub lay_canvas: canvas,
    pub lay_forecv: *mut canvas,
    pub lay_cvlist: *mut canvas,
    pub lay_autosave: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct NewWindow {
    pub StartAt: libc::c_int,
    pub aka: *mut libc::c_char,
    pub args: *mut *mut libc::c_char,
    pub dir: *mut libc::c_char,
    pub term: *mut libc::c_char,
    pub aflag: libc::c_int,
    pub dynamicaka: libc::c_int,
    pub flowflag: libc::c_int,
    pub lflag: libc::c_int,
    pub histheight: libc::c_int,
    pub monitor: libc::c_int,
    pub wlock: libc::c_int,
    pub silence: libc::c_int,
    pub wrap: libc::c_int,
    pub Lflag: libc::c_int,
    pub slow: libc::c_int,
    pub gr: libc::c_int,
    pub c1: libc::c_int,
    pub bce: libc::c_int,
    pub encoding: libc::c_int,
    pub hstatus: *mut libc::c_char,
    pub charset: *mut libc::c_char,
    pub poll_zombie_timeout: libc::c_int,
}
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct spwd {
    pub sp_namp: *mut libc::c_char,
    pub sp_pwdp: *mut libc::c_char,
    pub sp_lstchg: libc::c_long,
    pub sp_min: libc::c_long,
    pub sp_max: libc::c_long,
    pub sp_warn: libc::c_long,
    pub sp_inact: libc::c_long,
    pub sp_expire: libc::c_long,
    pub sp_flag: libc::c_ulong,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct backtick {
    pub next: *mut backtick,
    pub num: libc::c_int,
    pub tick: libc::c_int,
    pub lifespan: libc::c_int,
    pub bestbefore: time_t,
    pub result: [libc::c_char; 768],
    pub cmdv: *mut *mut libc::c_char,
    pub ev: event,
    pub buf: *mut libc::c_char,
    pub bufi: libc::c_int,
}
#[inline]
unsafe extern "C" fn stat(
    mut __path: *const libc::c_char,
    mut __statbuf: *mut stat,
) -> libc::c_int {
    return __xstat(1 as libc::c_int, __path, __statbuf);
}
#[inline]
unsafe extern "C" fn atoi(mut __nptr: *const libc::c_char) -> libc::c_int {
    return strtol(
        __nptr,
        0 as *mut libc::c_void as *mut *mut libc::c_char,
        10 as libc::c_int,
    ) as libc::c_int;
}
pub static mut force_vt: libc::c_int = 1 as libc::c_int;
pub static mut VBellWait: libc::c_int = 0;
pub static mut MsgWait: libc::c_int = 0;
pub static mut MsgMinWait: libc::c_int = 0;
pub static mut SilenceWait: libc::c_int = 0;
pub static mut ShellProg: *mut libc::c_char = 0 as *const libc::c_char
    as *mut libc::c_char;
pub static mut ShellArgs: [*mut libc::c_char; 2] = [0 as *const libc::c_char
    as *mut libc::c_char; 2];
pub static mut nversion: libc::c_int = 0;
pub static mut ppp: *mut passwd = 0 as *const passwd as *mut passwd;
pub static mut attach_tty: *mut libc::c_char = 0 as *const libc::c_char
    as *mut libc::c_char;
pub static mut attach_tty_is_in_new_ns: bool = 0 as libc::c_int != 0;
pub static mut attach_tty_name_in_ns: [libc::c_char; 4096] = [0; 4096];
pub static mut attach_fd: libc::c_int = -(1 as libc::c_int);
pub static mut attach_term: *mut libc::c_char = 0 as *const libc::c_char
    as *mut libc::c_char;
pub static mut LoginName: *mut libc::c_char = 0 as *const libc::c_char
    as *mut libc::c_char;
pub static mut attach_Mode: mode = mode {
    tio: termios {
        c_iflag: 0,
        c_oflag: 0,
        c_cflag: 0,
        c_lflag: 0,
        c_line: 0,
        c_cc: [0; 32],
        c_ispeed: 0,
        c_ospeed: 0,
    },
};
pub static mut SockPath: [libc::c_char; 5632] = [0; 5632];
pub static mut SockName: *mut libc::c_char = 0 as *const libc::c_char
    as *mut libc::c_char;
pub static mut SockMatch: *mut libc::c_char = 0 as *const libc::c_char
    as *mut libc::c_char;
pub static mut ServerSocket: libc::c_int = -(1 as libc::c_int);
pub static mut serv_read: event = event {
    next: 0 as *const event as *mut event,
    handler: None,
    data: 0 as *const libc::c_char as *mut libc::c_char,
    fd: 0,
    type_0: 0,
    pri: 0,
    timeout: timeval { tv_sec: 0, tv_usec: 0 },
    queued: 0,
    active: 0,
    condpos: 0 as *const libc::c_int as *mut libc::c_int,
    condneg: 0 as *const libc::c_int as *mut libc::c_int,
};
pub static mut serv_select: event = event {
    next: 0 as *const event as *mut event,
    handler: None,
    data: 0 as *const libc::c_char as *mut libc::c_char,
    fd: 0,
    type_0: 0,
    pri: 0,
    timeout: timeval { tv_sec: 0, tv_usec: 0 },
    queued: 0,
    active: 0,
    condpos: 0 as *const libc::c_int as *mut libc::c_int,
    condneg: 0 as *const libc::c_int as *mut libc::c_int,
};
pub static mut logflushev: event = event {
    next: 0 as *const event as *mut event,
    handler: None,
    data: 0 as *const libc::c_char as *mut libc::c_char,
    fd: 0,
    type_0: 0,
    pri: 0,
    timeout: timeval { tv_sec: 0, tv_usec: 0 },
    queued: 0,
    active: 0,
    condpos: 0 as *const libc::c_int as *mut libc::c_int,
    condneg: 0 as *const libc::c_int as *mut libc::c_int,
};
pub static mut NewEnv: *mut *mut libc::c_char = 0 as *const *mut libc::c_char
    as *mut *mut libc::c_char;
pub static mut RcFileName: *mut libc::c_char = 0 as *const libc::c_char
    as *mut libc::c_char;
pub static mut home: *mut libc::c_char = 0 as *const libc::c_char as *mut libc::c_char;
pub static mut screenlogfile: *mut libc::c_char = 0 as *const libc::c_char
    as *mut libc::c_char;
pub static mut log_flush: libc::c_int = 10 as libc::c_int;
pub static mut logtstamp_on: libc::c_int = 0 as libc::c_int;
pub static mut logtstamp_string: *mut libc::c_char = 0 as *const libc::c_char
    as *mut libc::c_char;
pub static mut logtstamp_after: libc::c_int = 120 as libc::c_int;
pub static mut hardcopydir: *mut libc::c_char = 0 as *const libc::c_char
    as *mut libc::c_char;
pub static mut BellString: *mut libc::c_char = 0 as *const libc::c_char
    as *mut libc::c_char;
pub static mut VisualBellString: *mut libc::c_char = 0 as *const libc::c_char
    as *mut libc::c_char;
pub static mut ActivityString: *mut libc::c_char = 0 as *const libc::c_char
    as *mut libc::c_char;
pub static mut BufferFile: *mut libc::c_char = 0 as *const libc::c_char
    as *mut libc::c_char;
pub static mut PowDetachString: *mut libc::c_char = 0 as *const libc::c_char
    as *mut libc::c_char;
pub static mut hstatusstring: *mut libc::c_char = 0 as *const libc::c_char
    as *mut libc::c_char;
pub static mut captionstring: *mut libc::c_char = 0 as *const libc::c_char
    as *mut libc::c_char;
pub static mut timestring: *mut libc::c_char = 0 as *const libc::c_char
    as *mut libc::c_char;
pub static mut wliststr: *mut libc::c_char = 0 as *const libc::c_char
    as *mut libc::c_char;
pub static mut wlisttit: *mut libc::c_char = 0 as *const libc::c_char
    as *mut libc::c_char;
pub static mut auto_detach: libc::c_int = 1 as libc::c_int;
pub static mut iflag: libc::c_int = 0;
pub static mut rflag: libc::c_int = 0;
pub static mut dflag: libc::c_int = 0;
pub static mut lsflag: libc::c_int = 0;
pub static mut quietflag: libc::c_int = 0;
pub static mut wipeflag: libc::c_int = 0;
pub static mut xflag: libc::c_int = 0;
pub static mut cmdflag: libc::c_int = 0;
pub static mut queryflag: libc::c_int = -(1 as libc::c_int);
pub static mut adaptflag: libc::c_int = 0;
pub static mut multi: *mut libc::c_char = 0 as *const libc::c_char as *mut libc::c_char;
pub static mut multi_home: *mut libc::c_char = 0 as *const libc::c_char
    as *mut libc::c_char;
pub static mut multi_uid: libc::c_int = 0;
pub static mut own_uid: libc::c_int = 0;
pub static mut multiattach: libc::c_int = 0;
pub static mut tty_mode: libc::c_int = 0;
pub static mut tty_oldmode: libc::c_int = -(1 as libc::c_int);
pub static mut HostName: [libc::c_char; 768] = [0; 768];
pub static mut MasterPid: libc::c_int = 0;
pub static mut PanicPid: libc::c_int = 0;
pub static mut real_uid: libc::c_int = 0;
pub static mut real_gid: libc::c_int = 0;
pub static mut eff_uid: libc::c_int = 0;
pub static mut eff_gid: libc::c_int = 0;
pub static mut default_startup: libc::c_int = 0;
pub static mut ZombieKey_destroy: libc::c_int = 0;
pub static mut ZombieKey_resurrect: libc::c_int = 0;
pub static mut ZombieKey_onerror: libc::c_int = 0;
pub static mut preselect: *mut libc::c_char = 0 as *const libc::c_char
    as *mut libc::c_char;
pub static mut screenencodings: *mut libc::c_char = 0 as *const libc::c_char
    as *mut libc::c_char;
pub static mut cjkwidth: libc::c_int = 0;
pub static mut nethackflag: libc::c_int = 0 as libc::c_int;
pub static mut maxwin: libc::c_int = 0;
pub static mut flayer: *mut layer = 0 as *const layer as *mut layer;
pub static mut fore: *mut win = 0 as *const win as *mut win;
pub static mut windows: *mut win = 0 as *const win as *mut win;
pub static mut console_window: *mut win = 0 as *const win as *mut win;
pub static mut strnomem: [libc::c_char; 15] = unsafe {
    *::std::mem::transmute::<&[u8; 15], &mut [libc::c_char; 15]>(b"Out of memory.\0")
};
static mut InterruptPlease: libc::c_int = 0;
static mut GotSigChld: libc::c_int = 0;
unsafe extern "C" fn lf_secreopen(
    mut name: *mut libc::c_char,
    mut wantfd: libc::c_int,
    mut l: *mut logfile,
) -> libc::c_int {
    let mut got_fd: libc::c_int = 0;
    close(wantfd);
    got_fd = secopen(
        name,
        0o1 as libc::c_int | 0o100 as libc::c_int | 0o2000 as libc::c_int,
        0o666 as libc::c_int,
    );
    if got_fd < 0 as libc::c_int || lf_move_fd(got_fd, wantfd) < 0 as libc::c_int {
        logfclose(l);
        return -(1 as libc::c_int);
    }
    (*(*l).st).st_dev = 0 as libc::c_int as __dev_t;
    (*(*l).st).st_ino = (*(*l).st).st_dev;
    return 0 as libc::c_int;
}
unsafe extern "C" fn getpwbyname(
    mut name: *mut libc::c_char,
    mut ppp_0: *mut passwd,
) -> *mut passwd {
    let mut n: libc::c_int = 0;
    let mut sss: *mut spwd = 0 as *mut spwd;
    static mut spw: *mut libc::c_char = 0 as *const libc::c_char as *mut libc::c_char;
    if ppp_0.is_null()
        && {
            ppp_0 = getpwnam(name);
            ppp_0.is_null()
        }
    {
        return 0 as *mut passwd;
    }
    loop {
        n = 0 as libc::c_int;
        if *((*ppp_0).pw_passwd).offset(0 as libc::c_int as isize) as libc::c_int
            == '#' as i32
            && *((*ppp_0).pw_passwd).offset(1 as libc::c_int as isize) as libc::c_int
                == '#' as i32
            && strcmp(
                ((*ppp_0).pw_passwd).offset(2 as libc::c_int as isize),
                (*ppp_0).pw_name,
            ) == 0 as libc::c_int
        {
            n = 13 as libc::c_int;
        }
        while n < 13 as libc::c_int {
            let mut c: libc::c_char = *((*ppp_0).pw_passwd).offset(n as isize);
            if !(c as libc::c_int == '.' as i32 || c as libc::c_int == '/' as i32
                || c as libc::c_int == '$' as i32
                || c as libc::c_int >= '0' as i32 && c as libc::c_int <= '9' as i32
                || c as libc::c_int >= 'a' as i32 && c as libc::c_int <= 'z' as i32
                || c as libc::c_int >= 'A' as i32 && c as libc::c_int <= 'Z' as i32)
            {
                break;
            }
            n += 1;
            n;
        }
        if !(n < 13 as libc::c_int && sss.is_null()) {
            break;
        }
        sss = getspnam((*ppp_0).pw_name);
        if !sss.is_null() {
            if !spw.is_null() {
                free(spw as *mut libc::c_void);
            }
            spw = SaveStr((*sss).sp_pwdp);
            (*ppp_0).pw_passwd = spw;
            endspent();
        } else {
            endspent();
            break;
        }
    }
    if n < 13 as libc::c_int {
        (*ppp_0).pw_passwd = 0 as *mut libc::c_char;
    }
    if !((*ppp_0).pw_passwd).is_null()
        && strlen((*ppp_0).pw_passwd)
            == (13 as libc::c_int + 11 as libc::c_int) as libc::c_ulong
    {
        *((*ppp_0).pw_passwd)
            .offset(13 as libc::c_int as isize) = 0 as libc::c_int as libc::c_char;
    }
    return ppp_0;
}
unsafe extern "C" fn locale_name() -> *mut libc::c_char {
    static mut s: *mut libc::c_char = 0 as *const libc::c_char as *mut libc::c_char;
    if s.is_null() {
        s = getenv(b"LC_ALL\0" as *const u8 as *const libc::c_char);
        if s.is_null() {
            s = getenv(b"LC_CTYPE\0" as *const u8 as *const libc::c_char);
        }
        if s.is_null() {
            s = getenv(b"LANG\0" as *const u8 as *const libc::c_char);
        }
    }
    return s;
}
unsafe fn main_0(mut ac: libc::c_int, mut av: *mut *mut libc::c_char) -> libc::c_int {
    let mut n: libc::c_int = 0;
    let mut ap: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut av0: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut socknamebuf: [libc::c_char; 1536] = [0; 1536];
    let mut mflag: libc::c_int = 0 as libc::c_int;
    let mut myname: *mut libc::c_char = (if ac == 0 as libc::c_int {
        b"screen\0" as *const u8 as *const libc::c_char
    } else {
        *av.offset(0 as libc::c_int as isize) as *const libc::c_char
    }) as *mut libc::c_char;
    let mut SockDir: *mut libc::c_char = 0 as *mut libc::c_char;
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
    let mut oumask: libc::c_int = 0;
    let mut nwin: NewWindow = NewWindow {
        StartAt: 0,
        aka: 0 as *const libc::c_char as *mut libc::c_char,
        args: 0 as *const *mut libc::c_char as *mut *mut libc::c_char,
        dir: 0 as *const libc::c_char as *mut libc::c_char,
        term: 0 as *const libc::c_char as *mut libc::c_char,
        aflag: 0,
        dynamicaka: 0,
        flowflag: 0,
        lflag: 0,
        histheight: 0,
        monitor: 0,
        wlock: 0,
        silence: 0,
        wrap: 0,
        Lflag: 0,
        slow: 0,
        gr: 0,
        c1: 0,
        bce: 0,
        encoding: 0,
        hstatus: 0 as *const libc::c_char as *mut libc::c_char,
        charset: 0 as *const libc::c_char as *mut libc::c_char,
        poll_zombie_timeout: 0,
    };
    let mut detached: libc::c_int = 0 as libc::c_int;
    let mut sockp: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut sty: *mut libc::c_char = 0 as *mut libc::c_char;
    closeallfiles(0 as libc::c_int);
    snprintf(
        version.as_mut_ptr(),
        59 as libc::c_int as libc::c_ulong,
        b"%d.%.2d.%.2d%s (%s%s) %s\0" as *const u8 as *const libc::c_char,
        4 as libc::c_int,
        9 as libc::c_int,
        0 as libc::c_int,
        b"\0" as *const u8 as *const libc::c_char,
        b"GNU\0" as *const u8 as *const libc::c_char,
        b"\0" as *const u8 as *const libc::c_char,
        b"30-Jan-22\0" as *const u8 as *const libc::c_char,
    );
    nversion = 4 as libc::c_int * 10000 as libc::c_int
        + 9 as libc::c_int * 100 as libc::c_int + 0 as libc::c_int;
    BellString = SaveStr(b"Bell in window %n\0" as *const u8 as *const libc::c_char);
    VisualBellString = SaveStr(
        b"   Wuff,  Wuff!!  \0" as *const u8 as *const libc::c_char,
    );
    ActivityString = SaveStr(
        b"Activity in window %n\0" as *const u8 as *const libc::c_char,
    );
    screenlogfile = SaveStr(b"screenlog.%n\0" as *const u8 as *const libc::c_char);
    logtstamp_string = SaveStr(
        b"-- %n:%t -- time-stamp -- %M/%d/%y %c:%s --\n\0" as *const u8
            as *const libc::c_char,
    );
    hstatusstring = SaveStr(b"%h\0" as *const u8 as *const libc::c_char);
    captionstring = SaveStr(b"%4n %t\0" as *const u8 as *const libc::c_char);
    timestring = SaveStr(b"%c:%s %M %d %H%? %l%?\0" as *const u8 as *const libc::c_char);
    wlisttit = SaveStr(b" Num Name%=Flags\0" as *const u8 as *const libc::c_char);
    wliststr = SaveStr(b"%4n %t%=%f\0" as *const u8 as *const libc::c_char);
    BufferFile = SaveStr(b"/tmp/screen-exchange\0" as *const u8 as *const libc::c_char);
    ShellProg = 0 as *mut libc::c_char;
    PowDetachString = 0 as *mut libc::c_char;
    default_startup = if ac > 1 as libc::c_int {
        0 as libc::c_int
    } else {
        1 as libc::c_int
    };
    adaptflag = 0 as libc::c_int;
    VBellWait = 1 as libc::c_int * 1000 as libc::c_int;
    MsgWait = 5 as libc::c_int * 1000 as libc::c_int;
    MsgMinWait = 1 as libc::c_int * 1000 as libc::c_int;
    SilenceWait = 30 as libc::c_int;
    zmodem_sendcmd = SaveStr(b"!!! sz -vv -b \0" as *const u8 as *const libc::c_char);
    zmodem_recvcmd = SaveStr(b"!!! rz -vv -b -E\0" as *const u8 as *const libc::c_char);
    CompileKeys(0 as *mut libc::c_char, 0 as libc::c_int, mark_key_tab.as_mut_ptr());
    InitBuiltinTabs();
    screenencodings = SaveStr(
        b"/usr/share/screen/utf8encodings\0" as *const u8 as *const libc::c_char,
    );
    cjkwidth = 0 as libc::c_int;
    nwin = nwin_undef;
    nwin_options = nwin_undef;
    strncpy(
        screenterm.as_mut_ptr(),
        b"screen\0" as *const u8 as *const libc::c_char,
        32 as libc::c_int as libc::c_ulong,
    );
    *screenterm
        .as_mut_ptr()
        .offset(32 as libc::c_int as isize) = '\0' as i32 as libc::c_char;
    real_uid = getuid() as libc::c_int;
    real_gid = getgid() as libc::c_int;
    eff_uid = geteuid() as libc::c_int;
    eff_gid = getegid() as libc::c_int;
    logreopen_register(
        ::std::mem::transmute::<
            Option::<unsafe extern "C" fn() -> libc::c_int>,
            Option::<
                unsafe extern "C" fn(
                    *mut libc::c_char,
                    libc::c_int,
                    *mut logfile,
                ) -> libc::c_int,
            >,
        >(
            Some(
                ::std::mem::transmute::<
                    unsafe extern "C" fn(
                        *mut libc::c_char,
                        libc::c_int,
                        *mut logfile,
                    ) -> libc::c_int,
                    unsafe extern "C" fn() -> libc::c_int,
                >(lf_secreopen),
            ),
        ),
    );
    av0 = *av;
    if *av0 as libc::c_int == '-' as i32 {
        rflag = 4 as libc::c_int;
        xflag = 1 as libc::c_int;
        ShellProg = SaveStr(DefaultShell.as_mut_ptr());
    }
    while ac > 0 as libc::c_int {
        av = av.offset(1);
        ap = *av;
        ac -= 1;
        if !(ac > 0 as libc::c_int && *ap as libc::c_int == '-' as i32) {
            break;
        }
        if *ap.offset(1 as libc::c_int as isize) as libc::c_int == '-' as i32
            && *ap.offset(2 as libc::c_int as isize) as libc::c_int == 0 as libc::c_int
        {
            av = av.offset(1);
            av;
            ac -= 1;
            ac;
            break;
        } else {
            if *ap.offset(1 as libc::c_int as isize) as libc::c_int == '-' as i32
                && strcmp(ap, b"--version\0" as *const u8 as *const libc::c_char) == 0
            {
                printf(
                    b"Screen version %s\n\0" as *const u8 as *const libc::c_char,
                    version.as_mut_ptr(),
                );
                exit(0 as libc::c_int);
            }
            if *ap.offset(1 as libc::c_int as isize) as libc::c_int == '-' as i32
                && strcmp(ap, b"--help\0" as *const u8 as *const libc::c_char) == 0
            {
                exit_with_usage(myname, 0 as *mut libc::c_char, 0 as *mut libc::c_char);
            }
            while !ap.is_null() && *ap as libc::c_int != 0
                && {
                    ap = ap.offset(1);
                    *ap as libc::c_int != 0
                }
            {
                let mut current_block_207: u64;
                match *ap as libc::c_int {
                    97 => {
                        nwin_options.aflag = 1 as libc::c_int;
                        current_block_207 = 12463749970033092792;
                    }
                    65 => {
                        adaptflag = 1 as libc::c_int;
                        current_block_207 = 12463749970033092792;
                    }
                    112 => {
                        ap = ap.offset(1);
                        if *ap != 0 {
                            preselect = ap;
                        } else {
                            ac -= 1;
                            if ac == 0 {
                                exit_with_usage(
                                    myname,
                                    b"Specify a window to preselect with -p\0" as *const u8
                                        as *const libc::c_char as *mut libc::c_char,
                                    0 as *mut libc::c_char,
                                );
                            }
                            av = av.offset(1);
                            preselect = *av;
                        }
                        ap = 0 as *mut libc::c_char;
                        current_block_207 = 12463749970033092792;
                    }
                    99 => {
                        ap = ap.offset(1);
                        if *ap != 0 {
                            RcFileName = ap;
                        } else {
                            ac -= 1;
                            if ac == 0 as libc::c_int {
                                exit_with_usage(
                                    myname,
                                    b"Specify an alternate rc-filename with -c\0" as *const u8
                                        as *const libc::c_char as *mut libc::c_char,
                                    0 as *mut libc::c_char,
                                );
                            }
                            av = av.offset(1);
                            RcFileName = *av;
                        }
                        ap = 0 as *mut libc::c_char;
                        current_block_207 = 12463749970033092792;
                    }
                    101 => {
                        ap = ap.offset(1);
                        if *ap == 0 {
                            ac -= 1;
                            if ac == 0 as libc::c_int {
                                exit_with_usage(
                                    myname,
                                    b"Specify command characters with -e\0" as *const u8
                                        as *const libc::c_char as *mut libc::c_char,
                                    0 as *mut libc::c_char,
                                );
                            }
                            av = av.offset(1);
                            ap = *av;
                        }
                        if ParseEscape(ap) != 0 {
                            Panic(
                                0 as libc::c_int,
                                b"Two characters are required with -e option, not '%s'.\0"
                                    as *const u8 as *const libc::c_char,
                                ap,
                            );
                        }
                        ap = 0 as *mut libc::c_char;
                        current_block_207 = 12463749970033092792;
                    }
                    102 => {
                        ap = ap.offset(1);
                        ap;
                        let mut current_block_105: u64;
                        let fresh0 = ap;
                        ap = ap.offset(1);
                        match *fresh0 as libc::c_int {
                            110 | 48 => {
                                nwin_options
                                    .flowflag = ((1 as libc::c_int) << 0 as libc::c_int)
                                    * 0 as libc::c_int;
                                current_block_105 = 2362946907345824597;
                            }
                            0 => {
                                ap = ap.offset(-1);
                                ap;
                                current_block_105 = 1228885388998600134;
                            }
                            121 | 49 => {
                                current_block_105 = 1228885388998600134;
                            }
                            97 => {
                                nwin_options
                                    .flowflag = (1 as libc::c_int) << 2 as libc::c_int;
                                current_block_105 = 2362946907345824597;
                            }
                            _ => {
                                ap = ap.offset(-1);
                                exit_with_usage(
                                    myname,
                                    b"Unknown flow option -%s\0" as *const u8
                                        as *const libc::c_char as *mut libc::c_char,
                                    ap,
                                );
                                current_block_105 = 2362946907345824597;
                            }
                        }
                        match current_block_105 {
                            1228885388998600134 => {
                                nwin_options
                                    .flowflag = ((1 as libc::c_int) << 0 as libc::c_int)
                                    * 1 as libc::c_int;
                            }
                            _ => {}
                        }
                        current_block_207 = 12463749970033092792;
                    }
                    104 => {
                        ac -= 1;
                        if ac == 0 as libc::c_int {
                            exit_with_usage(
                                myname,
                                0 as *mut libc::c_char,
                                0 as *mut libc::c_char,
                            );
                        }
                        av = av.offset(1);
                        nwin_options.histheight = atoi(*av);
                        if nwin_options.histheight < 0 as libc::c_int {
                            exit_with_usage(
                                myname,
                                b"-h: %s: negative scrollback size?\0" as *const u8
                                    as *const libc::c_char as *mut libc::c_char,
                                *av,
                            );
                        }
                        current_block_207 = 12463749970033092792;
                    }
                    105 => {
                        iflag = 1 as libc::c_int;
                        current_block_207 = 12463749970033092792;
                    }
                    116 => {
                        ac -= 1;
                        if ac == 0 as libc::c_int {
                            exit_with_usage(
                                myname,
                                b"Specify a new window-name with -t\0" as *const u8
                                    as *const libc::c_char as *mut libc::c_char,
                                0 as *mut libc::c_char,
                            );
                        }
                        av = av.offset(1);
                        nwin_options.aka = *av;
                        current_block_207 = 12463749970033092792;
                    }
                    108 => {
                        ap = ap.offset(1);
                        ap;
                        let mut current_block_127: u64;
                        let fresh1 = ap;
                        ap = ap.offset(1);
                        match *fresh1 as libc::c_int {
                            110 | 48 => {
                                nwin_options.lflag = 0 as libc::c_int;
                                current_block_127 = 9180031981464905198;
                            }
                            0 => {
                                ap = ap.offset(-1);
                                ap;
                                current_block_127 = 9030470892597113464;
                            }
                            121 | 49 => {
                                current_block_127 = 9030470892597113464;
                            }
                            97 => {
                                nwin_options.lflag = 3 as libc::c_int;
                                current_block_127 = 9180031981464905198;
                            }
                            115 | 105 => {
                                lsflag = 1 as libc::c_int;
                                if ac > 1 as libc::c_int && SockMatch.is_null() {
                                    av = av.offset(1);
                                    SockMatch = *av;
                                    ac -= 1;
                                    ac;
                                }
                                ap = 0 as *mut libc::c_char;
                                current_block_127 = 9180031981464905198;
                            }
                            _ => {
                                ap = ap.offset(-1);
                                exit_with_usage(
                                    myname,
                                    b"%s: Unknown suboption to -l\0" as *const u8
                                        as *const libc::c_char as *mut libc::c_char,
                                    ap,
                                );
                                current_block_127 = 9180031981464905198;
                            }
                        }
                        match current_block_127 {
                            9030470892597113464 => {
                                nwin_options.lflag = 1 as libc::c_int;
                            }
                            _ => {}
                        }
                        current_block_207 = 12463749970033092792;
                    }
                    119 => {
                        if strcmp(
                            ap.offset(1 as libc::c_int as isize),
                            b"ipe\0" as *const u8 as *const libc::c_char,
                        ) != 0
                        {
                            ap = ap.offset(-1);
                            exit_with_usage(
                                myname,
                                b"Unknown option %s\0" as *const u8 as *const libc::c_char
                                    as *mut libc::c_char,
                                ap,
                            );
                        }
                        lsflag = 1 as libc::c_int;
                        wipeflag = 1 as libc::c_int;
                        if ac > 1 as libc::c_int && SockMatch.is_null() {
                            av = av.offset(1);
                            SockMatch = *av;
                            ac -= 1;
                            ac;
                        }
                        current_block_207 = 12463749970033092792;
                    }
                    76 => {
                        if strcmp(
                            ap.offset(1 as libc::c_int as isize),
                            b"ogfile\0" as *const u8 as *const libc::c_char,
                        ) == 0
                        {
                            ac -= 1;
                            if ac == 0 as libc::c_int {
                                exit_with_usage(
                                    myname,
                                    b"Specify logfile path with -Logfile\0" as *const u8
                                        as *const libc::c_char as *mut libc::c_char,
                                    0 as *mut libc::c_char,
                                );
                            }
                            av = av.offset(1);
                            if strlen(*av) > 4096 as libc::c_int as libc::c_ulong {
                                Panic(
                                    1 as libc::c_int,
                                    b"-Logfile name too long. (max. %d char)\0" as *const u8
                                        as *const libc::c_char,
                                    4096 as libc::c_int,
                                );
                            }
                            free(screenlogfile as *mut libc::c_void);
                            screenlogfile = SaveStr(*av);
                            ap = 0 as *mut libc::c_char;
                        } else if strcmp(ap, b"L\0" as *const u8 as *const libc::c_char)
                            == 0
                        {
                            nwin_options.Lflag = 1 as libc::c_int;
                        }
                        current_block_207 = 12463749970033092792;
                    }
                    109 => {
                        mflag = 1 as libc::c_int;
                        current_block_207 = 12463749970033092792;
                    }
                    79 => {
                        force_vt = 0 as libc::c_int;
                        current_block_207 = 12463749970033092792;
                    }
                    84 => {
                        ac -= 1;
                        if ac == 0 as libc::c_int {
                            exit_with_usage(
                                myname,
                                b"Specify terminal-type with -T\0" as *const u8
                                    as *const libc::c_char as *mut libc::c_char,
                                0 as *mut libc::c_char,
                            );
                        }
                        av = av.offset(1);
                        if strlen(*av) < 32 as libc::c_int as libc::c_ulong {
                            strncpy(
                                screenterm.as_mut_ptr(),
                                *av,
                                32 as libc::c_int as libc::c_ulong,
                            );
                            *screenterm
                                .as_mut_ptr()
                                .offset(
                                    32 as libc::c_int as isize,
                                ) = '\0' as i32 as libc::c_char;
                        } else {
                            Panic(
                                0 as libc::c_int,
                                b"-T: terminal name too long. (max. %d char)\0" as *const u8
                                    as *const libc::c_char,
                                32 as libc::c_int,
                            );
                        }
                        nwin_options.term = screenterm.as_mut_ptr();
                        current_block_207 = 12463749970033092792;
                    }
                    113 => {
                        quietflag = 1 as libc::c_int;
                        current_block_207 = 12463749970033092792;
                    }
                    81 => {
                        queryflag = 1 as libc::c_int;
                        cmdflag = 1 as libc::c_int;
                        current_block_207 = 12463749970033092792;
                    }
                    114 | 82 | 120 => {
                        if ac > 1 as libc::c_int
                            && **av.offset(1 as libc::c_int as isize) as libc::c_int
                                != '-' as i32 && SockMatch.is_null()
                        {
                            av = av.offset(1);
                            SockMatch = *av;
                            ac -= 1;
                            ac;
                        }
                        if *ap as libc::c_int == 'x' as i32 {
                            xflag = 1 as libc::c_int;
                        }
                        if rflag != 0 {
                            rflag = 2 as libc::c_int;
                        }
                        rflag
                            += if *ap as libc::c_int == 'R' as i32 {
                                2 as libc::c_int
                            } else {
                                1 as libc::c_int
                            };
                        current_block_207 = 12463749970033092792;
                    }
                    100 => {
                        dflag = 1 as libc::c_int;
                        current_block_207 = 18386024953926853052;
                    }
                    68 => {
                        current_block_207 = 18386024953926853052;
                    }
                    115 => {
                        ac -= 1;
                        if ac == 0 as libc::c_int {
                            exit_with_usage(
                                myname,
                                b"Specify shell with -s\0" as *const u8
                                    as *const libc::c_char as *mut libc::c_char,
                                0 as *mut libc::c_char,
                            );
                        }
                        if !ShellProg.is_null() {
                            free(ShellProg as *mut libc::c_void);
                        }
                        av = av.offset(1);
                        ShellProg = SaveStr(*av);
                        current_block_207 = 12463749970033092792;
                    }
                    83 => {
                        if SockMatch.is_null() {
                            ac -= 1;
                            if ac == 0 as libc::c_int {
                                exit_with_usage(
                                    myname,
                                    b"Specify session-name with -S\0" as *const u8
                                        as *const libc::c_char as *mut libc::c_char,
                                    0 as *mut libc::c_char,
                                );
                            }
                            av = av.offset(1);
                            SockMatch = *av;
                            if strlen(SockMatch) > 80 as libc::c_int as libc::c_ulong {
                                exit_with_usage(
                                    myname,
                                    b"Session-name is too long (max length is 80 symbols)\0"
                                        as *const u8 as *const libc::c_char as *mut libc::c_char,
                                    0 as *mut libc::c_char,
                                );
                            }
                        }
                        if *SockMatch == 0 {
                            exit_with_usage(
                                myname,
                                b"Empty session-name?\0" as *const u8 as *const libc::c_char
                                    as *mut libc::c_char,
                                0 as *mut libc::c_char,
                            );
                        }
                        current_block_207 = 12463749970033092792;
                    }
                    88 => {
                        cmdflag = 1 as libc::c_int;
                        current_block_207 = 12463749970033092792;
                    }
                    118 => {
                        printf(
                            b"Screen version %s\n\0" as *const u8 as *const libc::c_char,
                            version.as_mut_ptr(),
                        );
                        exit(0 as libc::c_int);
                    }
                    85 => {
                        nwin_options
                            .encoding = if nwin_options.encoding == -(1 as libc::c_int) {
                            8 as libc::c_int
                        } else {
                            0 as libc::c_int
                        };
                        current_block_207 = 12463749970033092792;
                    }
                    _ => {
                        ap = ap.offset(-1);
                        exit_with_usage(
                            myname,
                            b"Unknown option %s\0" as *const u8 as *const libc::c_char
                                as *mut libc::c_char,
                            ap,
                        );
                        current_block_207 = 12463749970033092792;
                    }
                }
                match current_block_207 {
                    18386024953926853052 => {
                        if dflag == 0 {
                            dflag = 2 as libc::c_int;
                        }
                        if ac == 2 as libc::c_int {
                            if **av.offset(1 as libc::c_int as isize) as libc::c_int
                                != '-' as i32 && SockMatch.is_null()
                            {
                                av = av.offset(1);
                                SockMatch = *av;
                                ac -= 1;
                                ac;
                            }
                        }
                    }
                    _ => {}
                }
            }
        }
    }
    xsignal(7 as libc::c_int, Some(CoreDump as unsafe extern "C" fn(libc::c_int) -> ()));
    xsignal(
        11 as libc::c_int,
        Some(CoreDump as unsafe extern "C" fn(libc::c_int) -> ()),
    );
    setlocale(6 as libc::c_int, b"\0" as *const u8 as *const libc::c_char);
    if nwin_options.encoding == -(1 as libc::c_int) {
        nwin_options.encoding = FindEncoding(nl_langinfo(CODESET as libc::c_int));
    }
    let mut s: *mut libc::c_char = 0 as *mut libc::c_char;
    s = locale_name();
    if !s.is_null() {
        if strncmp(
            s,
            b"zh_\0" as *const u8 as *const libc::c_char,
            3 as libc::c_int as libc::c_ulong,
        ) == 0
            || strncmp(
                s,
                b"ja_\0" as *const u8 as *const libc::c_char,
                3 as libc::c_int as libc::c_ulong,
            ) == 0
            || strncmp(
                s,
                b"ko_\0" as *const u8 as *const libc::c_char,
                3 as libc::c_int as libc::c_ulong,
            ) == 0
        {
            cjkwidth = 1 as libc::c_int;
        }
    }
    if !(nwin_options.aka).is_null() {
        if nwin_options.encoding > 0 as libc::c_int {
            let mut len: size_t = strlen(nwin_options.aka);
            let mut newsz: size_t = 0;
            let mut newbuf: *mut libc::c_char = malloc(
                (3 as libc::c_int as libc::c_ulong).wrapping_mul(len),
            ) as *mut libc::c_char;
            if newbuf.is_null() {
                Panic(
                    0 as libc::c_int,
                    b"%s\0" as *const u8 as *const libc::c_char,
                    strnomem.as_mut_ptr(),
                );
            }
            newsz = RecodeBuf(
                nwin_options.aka as *mut libc::c_uchar,
                len as libc::c_int,
                nwin_options.encoding,
                0 as libc::c_int,
                newbuf as *mut libc::c_uchar,
            ) as size_t;
            *newbuf.offset(newsz as isize) = '\0' as i32 as libc::c_char;
            nwin_options.aka = newbuf;
        } else {
            nwin_options.aka = SaveStr(nwin_options.aka);
        }
    }
    if !SockMatch.is_null() && strlen(SockMatch) >= 768 as libc::c_int as libc::c_ulong {
        Panic(
            0 as libc::c_int,
            b"Ridiculously long socketname - try again.\0" as *const u8
                as *const libc::c_char,
        );
    }
    if cmdflag != 0 && rflag == 0 && dflag == 0 && xflag == 0 {
        xflag = 1 as libc::c_int;
    }
    if cmdflag == 0 && dflag != 0 && mflag != 0 && !(rflag != 0 || xflag != 0) {
        detached = 1 as libc::c_int;
    }
    nwin = nwin_options;
    nwin.encoding = nwin_undef.encoding;
    if ac != 0 {
        nwin.args = av;
    }
    xsignal(
        25 as libc::c_int,
        ::std::mem::transmute::<
            libc::intptr_t,
            __sighandler_t,
        >(1 as libc::c_int as libc::intptr_t),
    );
    xsignal(
        13 as libc::c_int,
        ::std::mem::transmute::<
            libc::intptr_t,
            __sighandler_t,
        >(1 as libc::c_int as libc::intptr_t),
    );
    if ShellProg.is_null() {
        let mut sh: *mut libc::c_char = 0 as *mut libc::c_char;
        sh = getenv(b"SHELL\0" as *const u8 as *const libc::c_char);
        ShellProg = SaveStr(if !sh.is_null() { sh } else { DefaultShell.as_mut_ptr() });
    }
    ShellArgs[0 as libc::c_int as usize] = ShellProg;
    home = getenv(b"HOME\0" as *const u8 as *const libc::c_char);
    if mflag == 0 && SockMatch.is_null() {
        sty = getenv(b"STY\0" as *const u8 as *const libc::c_char);
        if !sty.is_null() && *sty as libc::c_int == 0 as libc::c_int {
            sty = 0 as *mut libc::c_char;
        }
    }
    nethackflag = (getenv(b"NETHACKOPTIONS\0" as *const u8 as *const libc::c_char)
        != 0 as *mut libc::c_void as *mut libc::c_char) as libc::c_int;
    if nethackflag == 0 {
        let mut nethackrc: [libc::c_char; 4096] = [0; 4096];
        if !home.is_null()
            && strlen(home) < (4096 as libc::c_int - 20 as libc::c_int) as libc::c_ulong
        {
            sprintf(
                nethackrc.as_mut_ptr(),
                b"%s/.nethackrc\0" as *const u8 as *const libc::c_char,
                home,
            );
            nethackflag = (access(nethackrc.as_mut_ptr(), 0 as libc::c_int) == 0)
                as libc::c_int;
        }
    }
    multi_uid = real_uid;
    own_uid = multi_uid;
    if !SockMatch.is_null()
        && {
            sockp = index(SockMatch, '/' as i32);
            !sockp.is_null()
        }
    {
        *sockp = 0 as libc::c_int as libc::c_char;
        multi = SockMatch;
        SockMatch = sockp.offset(1 as libc::c_int as isize);
        if *multi != 0 {
            let mut mppp: *mut passwd = 0 as *mut passwd;
            mppp = getpwnam(multi);
            if mppp.is_null() {
                Panic(
                    0 as libc::c_int,
                    b"Cannot identify account '%s'.\0" as *const u8
                        as *const libc::c_char,
                    multi,
                );
            }
            multi_uid = (*mppp).pw_uid as libc::c_int;
            multi_home = SaveStr((*mppp).pw_dir);
            if strlen(multi_home)
                > (4096 as libc::c_int - 10 as libc::c_int) as libc::c_ulong
            {
                Panic(
                    0 as libc::c_int,
                    b"home directory path too long\0" as *const u8 as *const libc::c_char,
                );
            }
            if rflag != 0 || lsflag != 0 {
                xflag = 1 as libc::c_int;
            }
            detached = 0 as libc::c_int;
            multiattach = 1 as libc::c_int;
        }
        if eff_uid != 0 && multi_uid != eff_uid {
            Panic(
                0 as libc::c_int,
                b"Must run suid root for multiuser support.\0" as *const u8
                    as *const libc::c_char,
            );
        }
    }
    if !SockMatch.is_null() && *SockMatch as libc::c_int == 0 as libc::c_int {
        SockMatch = 0 as *mut libc::c_char;
    }
    LoginName = getlogin();
    if !LoginName.is_null()
        && *LoginName.offset(0 as libc::c_int as isize) as libc::c_int != '\0' as i32
    {
        ppp = getpwnam(LoginName);
        if !ppp.is_null() {
            if (*ppp).pw_uid as libc::c_int != real_uid {
                ppp = 0 as *mut passwd;
            }
        }
    }
    if ppp.is_null() {
        ppp = getpwuid(real_uid as __uid_t);
        if ppp.is_null() {
            Panic(
                0 as libc::c_int,
                b"getpwuid() can't identify your account!\0" as *const u8
                    as *const libc::c_char,
            );
        }
        LoginName = (*ppp).pw_name;
    }
    LoginName = SaveStr(LoginName);
    ppp = getpwbyname(LoginName, ppp);
    if home.is_null() || *home as libc::c_int == '\0' as i32 {
        home = (*ppp).pw_dir;
    }
    if strlen(LoginName) > 256 as libc::c_int as libc::c_ulong {
        Panic(
            0 as libc::c_int,
            b"LoginName too long - sorry.\0" as *const u8 as *const libc::c_char,
        );
    }
    if !multi.is_null() && strlen(multi) > 256 as libc::c_int as libc::c_ulong {
        Panic(
            0 as libc::c_int,
            b"Screen owner name too long - sorry.\0" as *const u8 as *const libc::c_char,
        );
    }
    if strlen(home) > (4096 as libc::c_int - 25 as libc::c_int) as libc::c_ulong {
        Panic(
            0 as libc::c_int,
            b"$HOME too long - sorry.\0" as *const u8 as *const libc::c_char,
        );
    }
    attach_tty = b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char;
    if detached == 0 && lsflag == 0 && cmdflag == 0
        && !(dflag != 0 && mflag == 0 && rflag == 0 && xflag == 0)
        && !(!sty.is_null() && SockMatch.is_null() && mflag == 0 && rflag == 0
            && xflag == 0)
    {
        let mut fl: libc::c_int = 0;
        SetTtyname(1 as libc::c_int != 0, &mut st);
        tty_mode = st.st_mode as libc::c_int & 0o777 as libc::c_int;
        fl = fcntl(0 as libc::c_int, 3 as libc::c_int, 0 as libc::c_int);
        if fl != -(1 as libc::c_int)
            && fl & (0o2 as libc::c_int | 0 as libc::c_int | 0o1 as libc::c_int)
                == 0o2 as libc::c_int
        {
            attach_fd = 0 as libc::c_int;
        }
        if attach_fd == -(1 as libc::c_int) {
            n = secopen(
                attach_tty,
                0o2 as libc::c_int | 0o4000 as libc::c_int,
                0 as libc::c_int,
            );
            if n < 0 as libc::c_int {
                Panic(
                    0 as libc::c_int,
                    b"Cannot open your terminal '%s' - please check.\0" as *const u8
                        as *const libc::c_char,
                    attach_tty,
                );
            }
            attach_fd = n;
        }
        attach_term = getenv(b"TERM\0" as *const u8 as *const libc::c_char);
        if attach_term.is_null() || *attach_term as libc::c_int == 0 as libc::c_int {
            Panic(
                0 as libc::c_int,
                b"Please set a terminal type.\0" as *const u8 as *const libc::c_char,
            );
        }
        if strlen(attach_term) > 32 as libc::c_int as libc::c_ulong {
            Panic(
                0 as libc::c_int,
                b"$TERM too long - sorry.\0" as *const u8 as *const libc::c_char,
            );
        }
        GetTTY(0 as libc::c_int, &mut attach_Mode);
    }
    oumask = umask(0 as libc::c_int as __mode_t) as libc::c_int;
    if oumask == -(1 as libc::c_int) {
        Panic(
            *__errno_location(),
            b"Cannot change umask to zero\0" as *const u8 as *const libc::c_char,
        );
    }
    SockDir = getenv(b"SCREENDIR\0" as *const u8 as *const libc::c_char);
    if !SockDir.is_null() {
        if strlen(SockDir) >= (4096 as libc::c_int - 1 as libc::c_int) as libc::c_ulong {
            Panic(
                0 as libc::c_int,
                b"Ridiculously long $SCREENDIR - try again.\0" as *const u8
                    as *const libc::c_char,
            );
        }
        if !multi.is_null() {
            Panic(
                0 as libc::c_int,
                b"No $SCREENDIR with multi screens, please.\0" as *const u8
                    as *const libc::c_char,
            );
        }
    }
    if multiattach != 0 {
        SockDir = (if eff_uid != 0 {
            b"/tmp/uscreens\0" as *const u8 as *const libc::c_char
        } else {
            b"/tmp/screens\0" as *const u8 as *const libc::c_char
        }) as *mut libc::c_char;
        sprintf(
            SockPath.as_mut_ptr(),
            b"%s/S-%s\0" as *const u8 as *const libc::c_char,
            SockDir,
            multi,
        );
    } else if !SockDir.is_null() {
        if access(SockDir, 0 as libc::c_int) != 0 {
            if UserContext() > 0 as libc::c_int {
                if mkdir(SockDir, 0o700 as libc::c_int as __mode_t) != 0 {
                    UserReturn(0 as libc::c_int);
                }
                UserReturn(1 as libc::c_int);
            }
            if UserStatus() <= 0 as libc::c_int {
                Panic(
                    0 as libc::c_int,
                    b"Cannot make directory '%s'.\0" as *const u8 as *const libc::c_char,
                    SockDir,
                );
            }
        }
        if SockDir != SockPath.as_mut_ptr() {
            strcpy(SockPath.as_mut_ptr(), SockDir);
        }
    } else {
        SockDir = (if eff_uid != 0 {
            b"/tmp/uscreens\0" as *const u8 as *const libc::c_char
        } else {
            b"/tmp/screens\0" as *const u8 as *const libc::c_char
        }) as *mut libc::c_char;
        if stat(SockDir, &mut st) != 0 {
            n = if eff_uid == 0 as libc::c_int && (real_uid != 0 || eff_gid == real_gid)
            {
                0o755 as libc::c_int
            } else if eff_gid != real_gid {
                0o775 as libc::c_int
            } else {
                0o777 as libc::c_int | 0o1000 as libc::c_int
            };
            if mkdir(SockDir, n as __mode_t) == -(1 as libc::c_int) {
                Panic(
                    *__errno_location(),
                    b"Cannot make directory '%s'\0" as *const u8 as *const libc::c_char,
                    SockDir,
                );
            }
        } else {
            if !(st.st_mode & 0o170000 as libc::c_int as libc::c_uint
                == 0o40000 as libc::c_int as libc::c_uint)
            {
                Panic(
                    0 as libc::c_int,
                    b"'%s' must be a directory.\0" as *const u8 as *const libc::c_char,
                    SockDir,
                );
            }
            if eff_uid == 0 as libc::c_int && real_uid != 0
                && st.st_uid as libc::c_int != eff_uid
            {
                Panic(
                    0 as libc::c_int,
                    b"Directory '%s' must be owned by root.\0" as *const u8
                        as *const libc::c_char,
                    SockDir,
                );
            }
            n = if eff_uid == 0 as libc::c_int
                && (real_uid != 0
                    || st.st_mode & 0o775 as libc::c_int as libc::c_uint
                        != 0o775 as libc::c_int as libc::c_uint)
            {
                0o755 as libc::c_int
            } else if eff_gid == st.st_gid as libc::c_int && eff_gid != real_gid {
                0o775 as libc::c_int
            } else {
                0o777 as libc::c_int
            };
            if st.st_mode as libc::c_int & 0o777 as libc::c_int != n {
                Panic(
                    0 as libc::c_int,
                    b"Directory '%s' must have mode %03o.\0" as *const u8
                        as *const libc::c_char,
                    SockDir,
                    n,
                );
            }
        }
        sprintf(
            SockPath.as_mut_ptr(),
            b"%s/S-%s\0" as *const u8 as *const libc::c_char,
            SockDir,
            LoginName,
        );
        if access(SockPath.as_mut_ptr(), 0 as libc::c_int) != 0 {
            if mkdir(SockPath.as_mut_ptr(), 0o700 as libc::c_int as __mode_t)
                == -(1 as libc::c_int) && *__errno_location() != 17 as libc::c_int
            {
                Panic(
                    *__errno_location(),
                    b"Cannot make directory '%s'\0" as *const u8 as *const libc::c_char,
                    SockPath.as_mut_ptr(),
                );
            }
            chown(SockPath.as_mut_ptr(), real_uid as __uid_t, real_gid as __gid_t);
        }
    }
    if stat(SockPath.as_mut_ptr(), &mut st) == -(1 as libc::c_int) {
        Panic(
            *__errno_location(),
            b"Cannot access %s\0" as *const u8 as *const libc::c_char,
            SockPath.as_mut_ptr(),
        );
    } else if !(st.st_mode & 0o170000 as libc::c_int as libc::c_uint
        == 0o40000 as libc::c_int as libc::c_uint)
    {
        Panic(
            0 as libc::c_int,
            b"%s is not a directory.\0" as *const u8 as *const libc::c_char,
            SockPath.as_mut_ptr(),
        );
    }
    if !multi.is_null() {
        if st.st_uid as libc::c_int != multi_uid {
            Panic(
                0 as libc::c_int,
                b"%s is not the owner of %s.\0" as *const u8 as *const libc::c_char,
                multi,
                SockPath.as_mut_ptr(),
            );
        }
    } else if st.st_uid as libc::c_int != real_uid {
        Panic(
            0 as libc::c_int,
            b"You are not the owner of %s.\0" as *const u8 as *const libc::c_char,
            SockPath.as_mut_ptr(),
        );
    }
    if st.st_mode & 0o777 as libc::c_int as libc::c_uint
        != 0o700 as libc::c_int as libc::c_uint
    {
        Panic(
            0 as libc::c_int,
            b"Directory %s must have mode 700.\0" as *const u8 as *const libc::c_char,
            SockPath.as_mut_ptr(),
        );
    }
    if !SockMatch.is_null() && !(index(SockMatch, '/' as i32)).is_null() {
        Panic(
            0 as libc::c_int,
            b"Bad session name '%s'\0" as *const u8 as *const libc::c_char,
            SockMatch,
        );
    }
    SockName = SockPath
        .as_mut_ptr()
        .offset(strlen(SockPath.as_mut_ptr()) as isize)
        .offset(1 as libc::c_int as isize);
    *SockName = 0 as libc::c_int as libc::c_char;
    umask(oumask as __mode_t);
    gethostname(HostName.as_mut_ptr(), 768 as libc::c_int as size_t);
    HostName[(768 as libc::c_int - 1 as libc::c_int)
        as usize] = '\0' as i32 as libc::c_char;
    ap = index(HostName.as_mut_ptr(), '.' as i32);
    if !ap.is_null() {
        *ap = '\0' as i32 as libc::c_char;
    }
    if lsflag != 0 {
        let mut i: libc::c_int = 0;
        let mut fo: libc::c_int = 0;
        let mut oth: libc::c_int = 0;
        let mut sock: bool = false;
        if !multi.is_null() {
            real_uid = multi_uid;
        }
        setgid(real_gid as __gid_t);
        setuid(real_uid as __uid_t);
        eff_uid = real_uid;
        eff_gid = real_gid;
        i = FindSocket(
            0 as *mut libc::c_void as *mut libc::c_int,
            &mut fo,
            &mut oth,
            SockMatch,
            &mut sock,
        );
        if quietflag != 0 {
            if rflag != 0 {
                exit(10 as libc::c_int + i);
            } else {
                exit(
                    9 as libc::c_int
                        + (if fo != 0 || oth != 0 {
                            1 as libc::c_int
                        } else {
                            0 as libc::c_int
                        }) + fo,
                );
            }
        }
        if fo == 0 as libc::c_int {
            Panic(
                0 as libc::c_int,
                b"No Sockets found in %s.\n\0" as *const u8 as *const libc::c_char,
                SockPath.as_mut_ptr(),
            );
        }
        Msg(
            0 as libc::c_int,
            b"%d Socket%s in %s.\0" as *const u8 as *const libc::c_char,
            fo,
            if fo > 1 as libc::c_int {
                b"s\0" as *const u8 as *const libc::c_char
            } else {
                b"\0" as *const u8 as *const libc::c_char
            },
            SockPath.as_mut_ptr(),
        );
        eexit(0 as libc::c_int);
    }
    xsignal(
        1 as libc::c_int,
        Some(AttacherFinit as unsafe extern "C" fn(libc::c_int) -> ()),
    );
    if cmdflag != 0 {
        if !multi.is_null() {
            real_uid = multi_uid;
        }
        SetTtyname(0 as libc::c_int != 0, &mut st);
        if (*av).is_null() {
            Panic(
                0 as libc::c_int,
                b"Please specify a command.\0" as *const u8 as *const libc::c_char,
            );
        }
        if strncmp(
            b"sessionname\0" as *const u8 as *const libc::c_char,
            *av,
            11 as libc::c_int as libc::c_ulong,
        ) == 0
        {
            av = av.offset(1);
            if (*av).is_null() {
                Panic(
                    0 as libc::c_int,
                    b"Please specify a parameter.\0" as *const u8 as *const libc::c_char,
                );
            }
            if strlen(*av) > 80 as libc::c_int as libc::c_ulong {
                Panic(
                    0 as libc::c_int,
                    b"Parameter of command 'sessionname' is too long.\0" as *const u8
                        as *const libc::c_char,
                );
            }
            av = av.offset(-1);
            *av;
        }
        setgid(real_gid as __gid_t);
        setuid(real_uid as __uid_t);
        eff_uid = real_uid;
        eff_gid = real_gid;
        SendCmdMessage(
            sty,
            SockMatch,
            av,
            (queryflag >= 0 as libc::c_int) as libc::c_int,
        );
        exit(0 as libc::c_int);
    } else if rflag != 0 || xflag != 0 {
        if Attach(2 as libc::c_int) != 0 {
            Attacher();
        }
        if multiattach != 0 {
            Panic(
                0 as libc::c_int,
                b"Can't create sessions of other users.\0" as *const u8
                    as *const libc::c_char,
            );
        }
    } else if dflag != 0 && mflag == 0 {
        SetTtyname(0 as libc::c_int != 0, &mut st);
        Attach(4 as libc::c_int);
        Msg(
            0 as libc::c_int,
            b"[%s %sdetached.]\n\0" as *const u8 as *const libc::c_char,
            SockName,
            if dflag > 1 as libc::c_int {
                b"power \0" as *const u8 as *const libc::c_char
            } else {
                b"\0" as *const u8 as *const libc::c_char
            },
        );
        eexit(0 as libc::c_int);
    }
    if SockMatch.is_null() && mflag == 0 && !sty.is_null() {
        SetTtyname(0 as libc::c_int != 0, &mut st);
        setgid(real_gid as __gid_t);
        setuid(real_uid as __uid_t);
        eff_uid = real_uid;
        eff_gid = real_gid;
        nwin_options.args = av;
        SendCreateMsg(sty, &mut nwin);
        exit(0 as libc::c_int);
    }
    nwin_compose(&mut nwin_default, &mut nwin_options, &mut nwin_default);
    if detached == 0 || dflag != 2 as libc::c_int {
        MasterPid = fork();
    } else {
        MasterPid = 0 as libc::c_int;
    }
    match MasterPid {
        -1 => {
            Panic(*__errno_location(), b"fork\0" as *const u8 as *const libc::c_char);
        }
        0 => {}
        _ => {
            if detached != 0 {
                exit(0 as libc::c_int);
            }
            if !SockMatch.is_null() {
                sprintf(
                    socknamebuf.as_mut_ptr(),
                    b"%d.%s\0" as *const u8 as *const libc::c_char,
                    MasterPid,
                    SockMatch,
                );
            } else {
                sprintf(
                    socknamebuf.as_mut_ptr(),
                    b"%d.%s.%s\0" as *const u8 as *const libc::c_char,
                    MasterPid,
                    stripdev(attach_tty),
                    HostName.as_mut_ptr(),
                );
            }
            ap = socknamebuf.as_mut_ptr();
            while *ap != 0 {
                if *ap as libc::c_int == '/' as i32 {
                    *ap = '-' as i32 as libc::c_char;
                }
                ap = ap.offset(1);
                ap;
            }
            if strlen(socknamebuf.as_mut_ptr()) > 255 as libc::c_int as libc::c_ulong {
                socknamebuf[255 as libc::c_int
                    as usize] = 0 as libc::c_int as libc::c_char;
            }
            sprintf(
                SockPath.as_mut_ptr().offset(strlen(SockPath.as_mut_ptr()) as isize),
                b"/%s\0" as *const u8 as *const libc::c_char,
                socknamebuf.as_mut_ptr(),
            );
            setgid(real_gid as __gid_t);
            setuid(real_uid as __uid_t);
            eff_uid = real_uid;
            eff_gid = real_gid;
            Attacher();
        }
    }
    if detached == 0 {
        PanicPid = getppid();
    }
    if DefaultEsc == -(1 as libc::c_int) {
        DefaultEsc = 'a' as i32 & 0o37 as libc::c_int;
    }
    if DefaultMetaEsc == -(1 as libc::c_int) {
        DefaultMetaEsc = 'a' as i32;
    }
    ap = av0.offset(strlen(av0) as isize).offset(-(1 as libc::c_int as isize));
    while ap >= av0 {
        if strncmp(
            b"screen\0" as *const u8 as *const libc::c_char,
            ap,
            6 as libc::c_int as libc::c_ulong,
        ) == 0
        {
            memcpy(
                ap as *mut libc::c_void,
                b"SCREEN\0" as *const u8 as *const libc::c_char as *const libc::c_void,
                6 as libc::c_int as libc::c_ulong,
            );
            break;
        } else {
            ap = ap.offset(-1);
            ap;
        }
    }
    if ap < av0 {
        *av0 = 'S' as i32 as libc::c_char;
    }
    if detached == 0 {
        if attach_fd == -(1 as libc::c_int) {
            n = secopen(
                attach_tty,
                0o2 as libc::c_int | 0o4000 as libc::c_int,
                0 as libc::c_int,
            );
            if n < 0 as libc::c_int {
                Panic(
                    0 as libc::c_int,
                    b"Cannot reopen '%s' - please check.\0" as *const u8
                        as *const libc::c_char,
                    attach_tty,
                );
            }
        } else {
            n = dup(attach_fd);
        }
    } else {
        n = -(1 as libc::c_int);
    }
    freopen(
        b"/dev/null\0" as *const u8 as *const libc::c_char,
        b"r\0" as *const u8 as *const libc::c_char,
        stdin,
    );
    freopen(
        b"/dev/null\0" as *const u8 as *const libc::c_char,
        b"w\0" as *const u8 as *const libc::c_char,
        stdout,
    );
    freopen(
        b"/dev/null\0" as *const u8 as *const libc::c_char,
        b"w\0" as *const u8 as *const libc::c_char,
        stderr,
    );
    if UserAdd(LoginName, 0 as *mut libc::c_char, 0 as *mut *mut acluser)
        < 0 as libc::c_int
    {
        Panic(
            0 as libc::c_int,
            b"Could not create user info\0" as *const u8 as *const libc::c_char,
        );
    }
    if detached == 0 {
        if (MakeDisplay(
            LoginName,
            attach_tty,
            attach_term,
            n,
            getppid(),
            &mut attach_Mode,
        ))
            .is_null()
        {
            Panic(
                0 as libc::c_int,
                b"Could not alloc display\0" as *const u8 as *const libc::c_char,
            );
        }
        PanicPid = 0 as libc::c_int;
        (*display)
            .d_encoding = if nwin_options.encoding > 0 as libc::c_int {
            nwin_options.encoding
        } else {
            0 as libc::c_int
        };
    }
    if !SockMatch.is_null() {
        sprintf(
            socknamebuf.as_mut_ptr(),
            b"%d.%s\0" as *const u8 as *const libc::c_char,
            getpid(),
            SockMatch,
        );
    } else {
        sprintf(
            socknamebuf.as_mut_ptr(),
            b"%d.%s.%s\0" as *const u8 as *const libc::c_char,
            getpid(),
            stripdev(attach_tty),
            HostName.as_mut_ptr(),
        );
    }
    ap = socknamebuf.as_mut_ptr();
    while *ap != 0 {
        if *ap as libc::c_int == '/' as i32 {
            *ap = '-' as i32 as libc::c_char;
        }
        ap = ap.offset(1);
        ap;
    }
    if strlen(socknamebuf.as_mut_ptr()) > 255 as libc::c_int as libc::c_ulong {
        socknamebuf[255 as libc::c_int as usize] = 0 as libc::c_int as libc::c_char;
    }
    sprintf(
        SockPath.as_mut_ptr().offset(strlen(SockPath.as_mut_ptr()) as isize),
        b"/%s\0" as *const u8 as *const libc::c_char,
        socknamebuf.as_mut_ptr(),
    );
    ServerSocket = MakeServerSocket(1 as libc::c_int != 0);
    ap = getenv(b"SYSSCREENRC\0" as *const u8 as *const libc::c_char);
    if !ap.is_null() {
        StartRc(ap, 0 as libc::c_int);
    } else {
        StartRc(
            b"/usr/etc/screenrc\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            0 as libc::c_int,
        );
    }
    StartRc(RcFileName, 0 as libc::c_int);
    InitUtmp();
    if !display.is_null() {
        if InitTermcap(0 as libc::c_int, 0 as libc::c_int) != 0 {
            fcntl((*display).d_userfd, 4 as libc::c_int, 0 as libc::c_int);
            freetty();
            if (*display).d_userpid != 0 {
                Kill((*display).d_userpid, 1 as libc::c_int);
            }
            eexit(1 as libc::c_int);
        }
        MakeDefaultCanvas();
        InitTerm(0 as libc::c_int);
        RemoveLoginSlot();
    } else {
        MakeTermcap(1 as libc::c_int);
    }
    InitLoadav();
    InitKeytab();
    MakeNewEnv();
    xsignal(1 as libc::c_int, Some(SigHup as unsafe extern "C" fn(libc::c_int) -> ()));
    xsignal(
        2 as libc::c_int,
        Some(FinitHandler as unsafe extern "C" fn(libc::c_int) -> ()),
    );
    xsignal(
        3 as libc::c_int,
        Some(FinitHandler as unsafe extern "C" fn(libc::c_int) -> ()),
    );
    xsignal(
        15 as libc::c_int,
        Some(FinitHandler as unsafe extern "C" fn(libc::c_int) -> ()),
    );
    xsignal(
        21 as libc::c_int,
        ::std::mem::transmute::<
            libc::intptr_t,
            __sighandler_t,
        >(1 as libc::c_int as libc::intptr_t),
    );
    xsignal(
        22 as libc::c_int,
        ::std::mem::transmute::<
            libc::intptr_t,
            __sighandler_t,
        >(1 as libc::c_int as libc::intptr_t),
    );
    if !display.is_null() {
        brktty((*display).d_userfd);
        SetMode(
            &mut (*display).d_OldMode,
            &mut (*display).d_NewMode,
            (*display).d_flow,
            iflag,
        );
        SetTTY((*display).d_userfd, &mut (*display).d_NewMode);
        if fcntl((*display).d_userfd, 4 as libc::c_int, 0o4000 as libc::c_int) != 0 {
            Msg(
                *__errno_location(),
                b"Warning: NBLOCK fcntl failed\0" as *const u8 as *const libc::c_char,
            );
        }
    } else {
        brktty(-(1 as libc::c_int));
    }
    xsignal(17 as libc::c_int, Some(SigChld as unsafe extern "C" fn(libc::c_int) -> ()));
    ap = getenv(b"SYSSCREENRC\0" as *const u8 as *const libc::c_char);
    if !ap.is_null() {
        FinishRc(ap);
    } else {
        FinishRc(
            b"/usr/etc/screenrc\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        );
    }
    FinishRc(RcFileName);
    if windows.is_null() {
        if MakeWindow(&mut nwin) == -(1 as libc::c_int) {
            let mut rfd: fd_set = fd_set { fds_bits: [0; 16] };
            let mut tv: timeval = {
                let mut init = timeval {
                    tv_sec: (MsgWait / 1000 as libc::c_int) as __time_t,
                    tv_usec: (1000 as libc::c_int * (MsgWait % 1000 as libc::c_int))
                        as __suseconds_t,
                };
                init
            };
            rfd
                .fds_bits[(0 as libc::c_int
                / (8 as libc::c_int
                    * ::std::mem::size_of::<__fd_mask>() as libc::c_ulong
                        as libc::c_int)) as usize]
                |= ((1 as libc::c_ulong)
                    << 0 as libc::c_int
                        % (8 as libc::c_int
                            * ::std::mem::size_of::<__fd_mask>() as libc::c_ulong
                                as libc::c_int)) as __fd_mask;
            Msg(
                0 as libc::c_int,
                b"Sorry, could not find a PTY or TTY.\0" as *const u8
                    as *const libc::c_char,
            );
            select(
                1 as libc::c_int,
                &mut rfd,
                0 as *mut fd_set,
                0 as *mut fd_set,
                &mut tv,
            );
            Finit(0 as libc::c_int);
        }
    } else if ac != 0 {
        MakeWindow(&mut nwin);
    }
    if !display.is_null() && default_startup != 0 {
        display_copyright();
    }
    xsignal(2 as libc::c_int, Some(SigInt as unsafe extern "C" fn(libc::c_int) -> ()));
    if rflag != 0 && rflag & 1 as libc::c_int == 0 as libc::c_int && quietflag == 0 {
        Msg(0 as libc::c_int, b"New screen...\0" as *const u8 as *const libc::c_char);
        rflag = 0 as libc::c_int;
    }
    serv_read.type_0 = 1 as libc::c_int;
    serv_read.fd = ServerSocket;
    serv_read
        .handler = Some(
        serv_read_fn as unsafe extern "C" fn(*mut event, *mut libc::c_char) -> (),
    );
    evenq(&mut serv_read);
    serv_select.pri = -(10 as libc::c_int);
    serv_select.type_0 = 3 as libc::c_int;
    serv_select
        .handler = Some(
        serv_select_fn as unsafe extern "C" fn(*mut event, *mut libc::c_char) -> (),
    );
    evenq(&mut serv_select);
    logflushev.type_0 = 0 as libc::c_int;
    logflushev
        .handler = Some(
        logflush_fn as unsafe extern "C" fn(*mut event, *mut libc::c_char) -> (),
    );
    sched();
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn WindowDied(
    mut p: *mut win,
    mut wstat: libc::c_int,
    mut wstat_valid: libc::c_int,
) {
    let mut killit: libc::c_int = 0 as libc::c_int;
    if (*p).w_destroyev.data == p as *mut libc::c_char {
        wstat = (*p).w_exitstatus;
        wstat_valid = 1 as libc::c_int;
        evdeq(&mut (*p).w_destroyev);
        (*p).w_destroyev.data = 0 as *mut libc::c_char;
    }
    if wstat_valid == 0 && (*p).w_pid > 0 as libc::c_int {
        if waitpid((*p).w_pid, &mut wstat, 1 as libc::c_int | 2 as libc::c_int)
            == (*p).w_pid
        {
            (*p).w_pid = 0 as libc::c_int;
            wstat_valid = 1 as libc::c_int;
        }
    }
    if ZombieKey_destroy != 0 && ZombieKey_onerror != 0 && wstat_valid != 0
        && wstat & 0x7f as libc::c_int == 0 as libc::c_int
        && (wstat & 0xff00 as libc::c_int) >> 8 as libc::c_int == 0 as libc::c_int
    {
        killit = 1 as libc::c_int;
    }
    if ZombieKey_destroy != 0 && killit == 0 {
        let mut buf: [libc::c_char; 100] = [0; 100];
        let mut s: *mut libc::c_char = 0 as *mut libc::c_char;
        let mut reason: [libc::c_char; 100] = [0; 100];
        let mut now: time_t = 0;
        if wstat_valid != 0 {
            if wstat & 0x7f as libc::c_int == 0 as libc::c_int {
                if (wstat & 0xff00 as libc::c_int) >> 8 as libc::c_int != 0 {
                    sprintf(
                        reason.as_mut_ptr(),
                        b"terminated with exit status %d\0" as *const u8
                            as *const libc::c_char,
                        (wstat & 0xff00 as libc::c_int) >> 8 as libc::c_int,
                    );
                } else {
                    sprintf(
                        reason.as_mut_ptr(),
                        b"terminated normally\0" as *const u8 as *const libc::c_char,
                    );
                }
            } else if ((wstat & 0x7f as libc::c_int) + 1 as libc::c_int) as libc::c_schar
                as libc::c_int >> 1 as libc::c_int > 0 as libc::c_int
            {
                sprintf(
                    reason.as_mut_ptr(),
                    b"terminated with signal %d%s\0" as *const u8 as *const libc::c_char,
                    wstat & 0x7f as libc::c_int,
                    if wstat & 0x80 as libc::c_int != 0 {
                        b" (core file generated)\0" as *const u8 as *const libc::c_char
                    } else {
                        b"\0" as *const u8 as *const libc::c_char
                    },
                );
            }
        } else {
            sprintf(
                reason.as_mut_ptr(),
                b"detached from window\0" as *const u8 as *const libc::c_char,
            );
        }
        time(&mut now);
        s = ctime(&mut now);
        if !s.is_null() && *s as libc::c_int != 0 {
            *s
                .offset(
                    (strlen(s)).wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize,
                ) = '\0' as i32 as libc::c_char;
        }
        if !((*p).w_slot).is_null() && (*p).w_slot != -(1 as libc::c_int) as slot_t {
            RemoveUtmp(p);
            (*p).w_slot = 0 as slot_t;
        }
        CloseDevice(p);
        (*p).w_deadpid = (*p).w_pid;
        (*p).w_pid = 0 as libc::c_int;
        ResetWindow(p);
        (*p).w_layer.l_y = MFindUsedLine(p, (*p).w_bot, 1 as libc::c_int);
        sprintf(
            buf.as_mut_ptr(),
            b"\n\r=== Command %s (%s) ===\0" as *const u8 as *const libc::c_char,
            reason.as_mut_ptr(),
            if !s.is_null() {
                s as *const libc::c_char
            } else {
                b"?\0" as *const u8 as *const libc::c_char
            },
        );
        WriteString(p, buf.as_mut_ptr(), strlen(buf.as_mut_ptr()) as libc::c_int);
        if (*p).w_poll_zombie_timeout != 0 {
            SetTimeout(
                &mut (*p).w_zombieev,
                (*p).w_poll_zombie_timeout * 1000 as libc::c_int,
            );
            evenq(&mut (*p).w_zombieev);
        }
        WindowChanged(p, 'f' as i32);
    } else {
        KillWindow(p);
    };
}
unsafe extern "C" fn SigChldHandler() {
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
    while GotSigChld != 0 {
        GotSigChld = 0 as libc::c_int;
        DoWait();
    }
    if stat(SockPath.as_mut_ptr(), &mut st) == -(1 as libc::c_int) {
        if RecoverSocket() == 0 {
            Finit(1 as libc::c_int);
        }
    }
}
unsafe extern "C" fn SigChld(mut sigsig: libc::c_int) {
    GotSigChld = 1 as libc::c_int;
}
pub unsafe extern "C" fn SigHup(mut sigsig: libc::c_int) {
    loop {
        display = displays;
        if display.is_null() {
            break;
        }
        Hangup();
    };
}
unsafe extern "C" fn SigInt(mut sigsig: libc::c_int) {
    xsignal(
        2 as libc::c_int,
        ::std::mem::transmute::<
            Option::<unsafe extern "C" fn() -> ()>,
            Option::<unsafe extern "C" fn(libc::c_int) -> ()>,
        >(
            Some(
                ::std::mem::transmute::<
                    unsafe extern "C" fn(libc::c_int) -> (),
                    unsafe extern "C" fn() -> (),
                >(SigInt),
            ),
        ),
    );
    InterruptPlease = 1 as libc::c_int;
}
unsafe extern "C" fn CoreDump(mut sigsig: libc::c_int) {
    let mut disp: *mut display = 0 as *mut display;
    let mut buf: [libc::c_char; 80] = [0; 80];
    let mut dump_msg: *mut libc::c_char = b" (core dumped)\0" as *const u8
        as *const libc::c_char as *mut libc::c_char;
    let mut running_w_s_bit: libc::c_int = (getuid() != geteuid()) as libc::c_int;
    if running_w_s_bit != 0 {
        dump_msg = b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char;
    }
    setgid(getgid());
    setuid(getuid());
    unlink(b"core\0" as *const u8 as *const libc::c_char);
    sprintf(
        buf.as_mut_ptr(),
        b"\r\n[screen caught signal %d.%s]\r\n\0" as *const u8 as *const libc::c_char,
        sigsig,
        dump_msg,
    );
    disp = displays;
    while !disp.is_null() {
        if !((*disp).d_nonblock < -(1 as libc::c_int)
            || (*disp).d_nonblock > 1000000 as libc::c_int)
        {
            fcntl((*disp).d_userfd, 4 as libc::c_int, 0 as libc::c_int);
            SetTTY((*disp).d_userfd, &mut (*display).d_OldMode);
            write(
                (*disp).d_userfd,
                buf.as_mut_ptr() as *const libc::c_void,
                strlen(buf.as_mut_ptr()),
            );
            Kill((*disp).d_userpid, 1 as libc::c_int);
        }
        disp = (*disp).d_next;
    }
    if running_w_s_bit != 0 {
        Kill(getpid(), 9 as libc::c_int);
        eexit(11 as libc::c_int);
    } else {
        abort();
    };
}
unsafe extern "C" fn DoWait() {
    let mut pid: libc::c_int = 0;
    let mut p: *mut win = 0 as *mut win;
    let mut next: *mut win = 0 as *mut win;
    let mut wstat: libc::c_int = 0;
    loop {
        pid = waitpid(
            -(1 as libc::c_int),
            &mut wstat,
            1 as libc::c_int | 2 as libc::c_int,
        );
        if !(pid > 0 as libc::c_int) {
            break;
        }
        p = windows;
        while !p.is_null() {
            next = (*p).w_next;
            if (*p).w_pid != 0 && pid == (*p).w_pid
                || (*p).w_deadpid != 0 && pid == (*p).w_deadpid
            {
                (*p).w_pid = 0 as libc::c_int;
                if wstat & 0xff as libc::c_int == 0x7f as libc::c_int {
                    if (wstat & 0xff00 as libc::c_int) >> 8 as libc::c_int
                        == 21 as libc::c_int
                    {
                        Msg(
                            0 as libc::c_int,
                            b"Suspended (tty input)\0" as *const u8
                                as *const libc::c_char,
                        );
                    } else if (wstat & 0xff00 as libc::c_int) >> 8 as libc::c_int
                        == 22 as libc::c_int
                    {
                        Msg(
                            0 as libc::c_int,
                            b"Suspended (tty output)\0" as *const u8
                                as *const libc::c_char,
                        );
                    } else {
                        Msg(
                            0 as libc::c_int,
                            b"Child has been stopped, restarting.\0" as *const u8
                                as *const libc::c_char,
                        );
                        if killpg(pid, 18 as libc::c_int) != 0 {
                            kill(pid, 18 as libc::c_int);
                        }
                        break;
                    }
                } else {
                    (*p).w_destroyev.data = p as *mut libc::c_char;
                    (*p).w_exitstatus = wstat;
                    SetTimeout(
                        &mut (*p).w_destroyev,
                        10 as libc::c_int * 1000 as libc::c_int,
                    );
                    evenq(&mut (*p).w_destroyev);
                    break;
                }
            } else if !((*p).w_pwin).is_null() && pid == (*(*p).w_pwin).p_pid {
                FreePseudowin(p);
                break;
            }
            p = next;
        }
        p.is_null();
    };
}
unsafe extern "C" fn FinitHandler(mut sigsig: libc::c_int) {
    Finit(1 as libc::c_int);
}
pub unsafe extern "C" fn Finit(mut i: libc::c_int) {
    xsignal(17 as libc::c_int, None);
    xsignal(
        1 as libc::c_int,
        ::std::mem::transmute::<
            libc::intptr_t,
            __sighandler_t,
        >(1 as libc::c_int as libc::intptr_t),
    );
    while !windows.is_null() {
        let mut p: *mut win = windows;
        windows = (*windows).w_next;
        FreeWindow(p);
    }
    if ServerSocket != -(1 as libc::c_int) {
        xseteuid(real_uid);
        xsetegid(real_gid);
        unlink(SockPath.as_mut_ptr());
        xseteuid(eff_uid);
        xsetegid(eff_gid);
    }
    display = displays;
    while !display.is_null() {
        if (*display).d_status != 0 {
            RemoveStatus();
        }
        FinitTerm();
        RestoreLoginSlot();
        AddStr(
            b"[screen is terminating]\r\n\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        );
        Flush(3 as libc::c_int);
        SetTTY((*display).d_userfd, &mut (*display).d_OldMode);
        fcntl((*display).d_userfd, 4 as libc::c_int, 0 as libc::c_int);
        freetty();
        Kill((*display).d_userpid, 1 as libc::c_int);
        display = (*display).d_next;
    }
    exit(i);
}
pub unsafe extern "C" fn eexit(mut e: libc::c_int) -> ! {
    if ServerSocket != -(1 as libc::c_int) {
        setgid(real_gid as __gid_t);
        setuid(real_uid as __uid_t);
        unlink(SockPath.as_mut_ptr());
    }
    exit(e);
}
pub unsafe extern "C" fn Hangup() {
    if display.is_null() {
        return;
    }
    if (*display).d_userfd >= 0 as libc::c_int {
        close((*display).d_userfd);
        (*display).d_userfd = -(1 as libc::c_int);
    }
    if auto_detach != 0 || !((*displays).d_next).is_null() {
        Detach(6 as libc::c_int);
    } else {
        Finit(0 as libc::c_int);
    };
}
pub unsafe extern "C" fn Detach(mut mode: libc::c_int) {
    let mut sign: libc::c_int = 0 as libc::c_int;
    let mut pid: libc::c_int = 0;
    let mut cv: *mut canvas = 0 as *mut canvas;
    let mut p: *mut win = 0 as *mut win;
    if display.is_null() {
        return;
    }
    xsignal(
        1 as libc::c_int,
        ::std::mem::transmute::<
            libc::intptr_t,
            __sighandler_t,
        >(1 as libc::c_int as libc::intptr_t),
    );
    if (*display).d_status != 0 {
        RemoveStatus();
    }
    FinitTerm();
    if display.is_null() {
        return;
    }
    match mode {
        6 => {
            sign = 1 as libc::c_int;
        }
        0 => {
            if !SockName.is_null() {
                AddStr(
                    b"[detached from \0" as *const u8 as *const libc::c_char
                        as *mut libc::c_char,
                );
                AddStr(SockName);
                AddStr(
                    b"]\r\n\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                );
            } else {
                AddStr(
                    b"[detached]\r\n\0" as *const u8 as *const libc::c_char
                        as *mut libc::c_char,
                );
            }
            sign = 1 as libc::c_int;
        }
        1 => {
            sign = 20 as libc::c_int;
        }
        2 => {
            if !SockName.is_null() {
                AddStr(
                    b"[remote detached from \0" as *const u8 as *const libc::c_char
                        as *mut libc::c_char,
                );
                AddStr(SockName);
                AddStr(
                    b"]\r\n\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                );
            } else {
                AddStr(
                    b"[remote detached]\r\n\0" as *const u8 as *const libc::c_char
                        as *mut libc::c_char,
                );
            }
            sign = 1 as libc::c_int;
        }
        3 => {
            if !SockName.is_null() {
                AddStr(
                    b"[power detached from \0" as *const u8 as *const libc::c_char
                        as *mut libc::c_char,
                );
                AddStr(SockName);
                AddStr(
                    b"]\r\n\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                );
            } else {
                AddStr(
                    b"[power detached]\r\n\0" as *const u8 as *const libc::c_char
                        as *mut libc::c_char,
                );
            }
            if !PowDetachString.is_null() {
                AddStr(PowDetachString);
                AddStr(
                    b"\r\n\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                );
            }
            sign = 10 as libc::c_int;
        }
        4 => {
            if !SockName.is_null() {
                AddStr(
                    b"[remote power detached from \0" as *const u8 as *const libc::c_char
                        as *mut libc::c_char,
                );
                AddStr(SockName);
                AddStr(
                    b"]\r\n\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                );
            } else {
                AddStr(
                    b"[remote power detached]\r\n\0" as *const u8 as *const libc::c_char
                        as *mut libc::c_char,
                );
            }
            if !PowDetachString.is_null() {
                AddStr(PowDetachString);
                AddStr(
                    b"\r\n\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                );
            }
            sign = 10 as libc::c_int;
        }
        5 => {
            ClearAll();
            sign = 12 as libc::c_int;
        }
        _ => {}
    }
    if ((*displays).d_next).is_null() {
        p = windows;
        while !p.is_null() {
            if (*p).w_slot != -(1 as libc::c_int) as slot_t
                && (*p).w_lflag & 2 as libc::c_int == 0
            {
                RemoveUtmp(p);
                (*p).w_slot = 0 as slot_t;
            }
            p = (*p).w_next;
        }
    }
    if mode != 6 as libc::c_int {
        RestoreLoginSlot();
    }
    if ((*displays).d_next).is_null() && !console_window.is_null() {
        if TtyGrabConsole(
            (*console_window).w_ptyfd,
            0 as libc::c_int,
            b"detach\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        ) != 0
        {
            KillWindow(console_window);
            display = displays;
        }
    }
    if !((*display).d_fore).is_null() {
        ReleaseAutoWritelock(display, (*display).d_fore);
        (*(*display).d_user).u_detachwin = (*(*display).d_fore).w_number;
        (*(*display).d_user)
            .u_detachotherwin = if !((*display).d_other).is_null() {
            (*(*display).d_other).w_number
        } else {
            -(1 as libc::c_int)
        };
    }
    AutosaveLayout((*display).d_layout);
    layout_last = (*display).d_layout;
    cv = (*display).d_cvlist;
    while !cv.is_null() {
        p = (*(*(*cv).c_layer).l_bottom).l_data as *mut win;
        SetCanvasWindow(cv, 0 as *mut win);
        if !p.is_null() {
            WindowChanged(p, 'u' as i32);
        }
        cv = (*cv).c_next;
    }
    pid = (*display).d_userpid;
    FreeDisplay();
    if displays.is_null() {
        chsock();
    }
    Kill(pid, sign);
    xsignal(
        1 as libc::c_int,
        ::std::mem::transmute::<
            Option::<unsafe extern "C" fn() -> ()>,
            Option::<unsafe extern "C" fn(libc::c_int) -> ()>,
        >(
            Some(
                ::std::mem::transmute::<
                    unsafe extern "C" fn(libc::c_int) -> (),
                    unsafe extern "C" fn() -> (),
                >(SigHup),
            ),
        ),
    );
}
unsafe extern "C" fn IsSymbol(
    mut e: *mut libc::c_char,
    mut s: *mut libc::c_char,
) -> libc::c_int {
    let mut l: libc::c_int = 0;
    l = strlen(s) as libc::c_int;
    return (strncmp(e, s, l as libc::c_ulong) == 0 as libc::c_int
        && *e.offset(l as isize) as libc::c_int == '=' as i32) as libc::c_int;
}
pub unsafe extern "C" fn MakeNewEnv() {
    let mut op: *mut *mut libc::c_char = 0 as *mut *mut libc::c_char;
    let mut np: *mut *mut libc::c_char = 0 as *mut *mut libc::c_char;
    static mut stybuf: [libc::c_char; 768] = [0; 768];
    op = environ;
    while !(*op).is_null() {
        op = op.offset(1);
        op;
    }
    if !NewEnv.is_null() {
        free(NewEnv as *mut libc::c_char as *mut libc::c_void);
    }
    np = malloc(
        ((op.offset_from(environ) as libc::c_long + 7 as libc::c_int as libc::c_long
            + 1 as libc::c_int as libc::c_long) as libc::c_uint as libc::c_ulong)
            .wrapping_mul(
                ::std::mem::size_of::<*mut *mut libc::c_char>() as libc::c_ulong,
            ),
    ) as *mut *mut libc::c_char;
    NewEnv = np;
    if NewEnv.is_null() {
        Panic(
            0 as libc::c_int,
            b"%s\0" as *const u8 as *const libc::c_char,
            strnomem.as_mut_ptr(),
        );
    }
    sprintf(
        stybuf.as_mut_ptr(),
        b"STY=%s\0" as *const u8 as *const libc::c_char,
        if strlen(SockName) <= (768 as libc::c_int - 5 as libc::c_int) as libc::c_ulong {
            SockName as *const libc::c_char
        } else {
            b"?\0" as *const u8 as *const libc::c_char
        },
    );
    let fresh2 = np;
    np = np.offset(1);
    *fresh2 = stybuf.as_mut_ptr();
    let fresh3 = np;
    np = np.offset(1);
    *fresh3 = Term.as_mut_ptr();
    np = np.offset(1);
    np;
    np = np.offset(2 as libc::c_int as isize);
    op = environ;
    while !(*op).is_null() {
        if IsSymbol(
            *op,
            b"TERM\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        ) == 0
            && IsSymbol(
                *op,
                b"TERMCAP\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            ) == 0
            && IsSymbol(
                *op,
                b"STY\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            ) == 0
            && IsSymbol(
                *op,
                b"WINDOW\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            ) == 0
            && IsSymbol(
                *op,
                b"SCREENCAP\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            ) == 0
            && IsSymbol(
                *op,
                b"SHELL\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            ) == 0
            && IsSymbol(
                *op,
                b"LINES\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            ) == 0
            && IsSymbol(
                *op,
                b"COLUMNS\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            ) == 0
        {
            let fresh4 = np;
            np = np.offset(1);
            *fresh4 = *op;
        }
        op = op.offset(1);
        op;
    }
    *np = 0 as *mut libc::c_char;
}
pub unsafe extern "C" fn Msg(
    mut err: libc::c_int,
    mut fmt: *const libc::c_char,
    mut args: ...
) {
    let mut buf: [libc::c_char; 8192] = [0; 8192];
    let mut p: *mut libc::c_char = buf.as_mut_ptr();
    let mut ap: ::std::ffi::VaListImpl;
    ap = args.clone();
    fmt = DoNLS(fmt);
    vsnprintf(
        p,
        (::std::mem::size_of::<[libc::c_char; 8192]>() as libc::c_ulong)
            .wrapping_sub(100 as libc::c_int as libc::c_ulong),
        fmt,
        ap.as_va_list(),
    );
    if err != 0 {
        p = p.offset(strlen(p) as isize);
        let fresh5 = p;
        p = p.offset(1);
        *fresh5 = ':' as i32 as libc::c_char;
        let fresh6 = p;
        p = p.offset(1);
        *fresh6 = ' ' as i32 as libc::c_char;
        strncpy(
            p,
            strerror(err),
            (buf
                .as_mut_ptr()
                .offset(
                    ::std::mem::size_of::<[libc::c_char; 8192]>() as libc::c_ulong
                        as isize,
                )
                .offset_from(p) as libc::c_long - 1 as libc::c_int as libc::c_long)
                as libc::c_ulong,
        );
        buf[(::std::mem::size_of::<[libc::c_char; 8192]>() as libc::c_ulong)
            .wrapping_sub(1 as libc::c_int as libc::c_ulong)
            as usize] = 0 as libc::c_int as libc::c_char;
    }
    if !display.is_null() && !displays.is_null() {
        MakeStatus(buf.as_mut_ptr());
    } else if !displays.is_null() {
        display = displays;
        while !display.is_null() {
            MakeStatus(buf.as_mut_ptr());
            display = (*display).d_next;
        }
    } else if !display.is_null() {
        let mut tty: *mut libc::c_char = ((*display).d_usertty).as_mut_ptr();
        let mut olddisplay: *mut display = display;
        display = 0 as *mut display;
        SendErrorMsg(tty, buf.as_mut_ptr());
        display = olddisplay;
    } else {
        printf(b"%s\r\n\0" as *const u8 as *const libc::c_char, buf.as_mut_ptr());
    }
    if queryflag >= 0 as libc::c_int {
        write(
            queryflag,
            buf.as_mut_ptr() as *const libc::c_void,
            strlen(buf.as_mut_ptr()),
        );
    }
}
pub unsafe extern "C" fn Panic(
    mut err: libc::c_int,
    mut fmt: *const libc::c_char,
    mut args: ...
) -> ! {
    let mut buf: [libc::c_char; 8192] = [0; 8192];
    let mut p: *mut libc::c_char = buf.as_mut_ptr();
    let mut ap: ::std::ffi::VaListImpl;
    ap = args.clone();
    fmt = DoNLS(fmt);
    vsnprintf(
        p,
        (::std::mem::size_of::<[libc::c_char; 8192]>() as libc::c_ulong)
            .wrapping_sub(100 as libc::c_int as libc::c_ulong),
        fmt,
        ap.as_va_list(),
    );
    if err != 0 {
        p = p.offset(strlen(p) as isize);
        let fresh7 = p;
        p = p.offset(1);
        *fresh7 = ':' as i32 as libc::c_char;
        let fresh8 = p;
        p = p.offset(1);
        *fresh8 = ' ' as i32 as libc::c_char;
        strncpy(
            p,
            strerror(err),
            (buf
                .as_mut_ptr()
                .offset(
                    ::std::mem::size_of::<[libc::c_char; 8192]>() as libc::c_ulong
                        as isize,
                )
                .offset_from(p) as libc::c_long - 1 as libc::c_int as libc::c_long)
                as libc::c_ulong,
        );
        buf[(::std::mem::size_of::<[libc::c_char; 8192]>() as libc::c_ulong)
            .wrapping_sub(1 as libc::c_int as libc::c_ulong)
            as usize] = 0 as libc::c_int as libc::c_char;
    }
    if displays.is_null() && display.is_null() {
        printf(b"%s\r\n\0" as *const u8 as *const libc::c_char, buf.as_mut_ptr());
        if PanicPid != 0 {
            Kill(PanicPid, 1 as libc::c_int);
        }
    } else if displays.is_null() {
        let mut tty: *mut libc::c_char = ((*display).d_usertty).as_mut_ptr();
        display = 0 as *mut display;
        SendErrorMsg(tty, buf.as_mut_ptr());
        sleep(2 as libc::c_int as libc::c_uint);
        _exit(1 as libc::c_int);
    } else {
        display = displays;
        while !display.is_null() {
            if (*display).d_status != 0 {
                RemoveStatus();
            }
            FinitTerm();
            Flush(3 as libc::c_int);
            RestoreLoginSlot();
            SetTTY((*display).d_userfd, &mut (*display).d_OldMode);
            fcntl((*display).d_userfd, 4 as libc::c_int, 0 as libc::c_int);
            write(
                (*display).d_userfd,
                buf.as_mut_ptr() as *const libc::c_void,
                strlen(buf.as_mut_ptr()),
            );
            write(
                (*display).d_userfd,
                b"\n\0" as *const u8 as *const libc::c_char as *const libc::c_void,
                1 as libc::c_int as size_t,
            );
            freetty();
            if (*display).d_userpid != 0 {
                Kill((*display).d_userpid, 1 as libc::c_int);
            }
            display = (*display).d_next;
        }
    }
    if tty_oldmode >= 0 as libc::c_int {
        if setuid(own_uid as __uid_t) != 0 {
            xseteuid(own_uid);
        }
        chmod(attach_tty, tty_oldmode as __mode_t);
    }
    eexit(1 as libc::c_int);
}
pub unsafe extern "C" fn QueryMsg(
    mut err: libc::c_int,
    mut fmt: *const libc::c_char,
    mut args: ...
) {
    let mut buf: [libc::c_char; 8192] = [0; 8192];
    if queryflag < 0 as libc::c_int {
        return;
    }
    let mut p: *mut libc::c_char = buf.as_mut_ptr();
    let mut ap: ::std::ffi::VaListImpl;
    ap = args.clone();
    fmt = DoNLS(fmt);
    vsnprintf(
        p,
        (::std::mem::size_of::<[libc::c_char; 8192]>() as libc::c_ulong)
            .wrapping_sub(100 as libc::c_int as libc::c_ulong),
        fmt,
        ap.as_va_list(),
    );
    if err != 0 {
        p = p.offset(strlen(p) as isize);
        let fresh9 = p;
        p = p.offset(1);
        *fresh9 = ':' as i32 as libc::c_char;
        let fresh10 = p;
        p = p.offset(1);
        *fresh10 = ' ' as i32 as libc::c_char;
        strncpy(
            p,
            strerror(err),
            (buf
                .as_mut_ptr()
                .offset(
                    ::std::mem::size_of::<[libc::c_char; 8192]>() as libc::c_ulong
                        as isize,
                )
                .offset_from(p) as libc::c_long - 1 as libc::c_int as libc::c_long)
                as libc::c_ulong,
        );
        buf[(::std::mem::size_of::<[libc::c_char; 8192]>() as libc::c_ulong)
            .wrapping_sub(1 as libc::c_int as libc::c_ulong)
            as usize] = 0 as libc::c_int as libc::c_char;
    }
    write(queryflag, buf.as_mut_ptr() as *const libc::c_void, strlen(buf.as_mut_ptr()));
}
pub unsafe extern "C" fn Dummy(
    mut err: libc::c_int,
    mut fmt: *const libc::c_char,
    mut args: ...
) {}
static mut winmsg_buf: [libc::c_char; 768] = [0; 768];
static mut winmsg_rend: [libc::c_int; 256] = [0; 256];
static mut winmsg_rendpos: [libc::c_int; 256] = [0; 256];
static mut winmsg_numrend: libc::c_int = 0;
unsafe extern "C" fn pad_expand(
    mut buf: *mut libc::c_char,
    mut p: *mut libc::c_char,
    mut numpad: libc::c_int,
    mut padlen: libc::c_int,
) -> *mut libc::c_char {
    let mut pn: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut pn2: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut i: libc::c_int = 0;
    let mut r: libc::c_int = 0;
    padlen = (padlen as libc::c_long - p.offset_from(buf) as libc::c_long)
        as libc::c_int;
    if padlen < 0 as libc::c_int {
        padlen = 0 as libc::c_int;
    }
    pn = p.offset(padlen as isize);
    pn2 = pn;
    r = winmsg_numrend;
    while p >= buf {
        if r != 0 && *p as libc::c_int != 127 as libc::c_int
            && p.offset_from(buf) as libc::c_long
                == winmsg_rendpos[(r - 1 as libc::c_int) as usize] as libc::c_long
        {
            r -= 1;
            winmsg_rendpos[r
                as usize] = pn.offset_from(buf) as libc::c_long as libc::c_int;
        } else {
            let fresh11 = pn;
            pn = pn.offset(-1);
            *fresh11 = *p;
            let fresh12 = p;
            p = p.offset(-1);
            if *fresh12 as libc::c_int == 127 as libc::c_int {
                *pn.offset(1 as libc::c_int as isize) = ' ' as i32 as libc::c_char;
                i = if numpad > 0 as libc::c_int {
                    (padlen + numpad - 1 as libc::c_int) / numpad
                } else {
                    0 as libc::c_int
                };
                padlen -= i;
                loop {
                    let fresh13 = i;
                    i = i - 1;
                    if !(fresh13 > 0 as libc::c_int) {
                        break;
                    }
                    let fresh14 = pn;
                    pn = pn.offset(-1);
                    *fresh14 = ' ' as i32 as libc::c_char;
                }
                numpad -= 1;
                numpad;
                if r != 0
                    && p.offset_from(buf) as libc::c_long
                        + 1 as libc::c_int as libc::c_long
                        == winmsg_rendpos[(r - 1 as libc::c_int) as usize]
                            as libc::c_long
                {
                    r -= 1;
                    winmsg_rendpos[r
                        as usize] = (pn.offset_from(buf) as libc::c_long
                        + 1 as libc::c_int as libc::c_long) as libc::c_int;
                }
            }
        }
    }
    return pn2;
}
pub static mut backticks: *mut backtick = 0 as *const backtick as *mut backtick;
unsafe extern "C" fn backtick_filter(mut bt: *mut backtick) {
    let mut p: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut q: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut c: libc::c_int = 0;
    q = ((*bt).result).as_mut_ptr();
    p = q;
    loop {
        let fresh15 = p;
        p = p.offset(1);
        c = *fresh15 as libc::c_uchar as libc::c_int;
        if !(c != 0 as libc::c_int) {
            break;
        }
        if c == '\t' as i32 {
            c = ' ' as i32;
        }
        if c >= ' ' as i32 || c == '\u{5}' as i32 {
            let fresh16 = q;
            q = q.offset(1);
            *fresh16 = c as libc::c_char;
        }
    }
    *q = 0 as libc::c_int as libc::c_char;
}
unsafe extern "C" fn backtick_fn(mut ev: *mut event, mut data: *mut libc::c_char) {
    let mut bt: *mut backtick = 0 as *mut backtick;
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut k: libc::c_int = 0;
    let mut l: libc::c_int = 0;
    bt = data as *mut backtick;
    i = (*bt).bufi;
    l = read(
        (*ev).fd,
        ((*bt).buf).offset(i as isize) as *mut libc::c_void,
        (768 as libc::c_int - i) as size_t,
    ) as libc::c_int;
    if l <= 0 as libc::c_int {
        evdeq(ev);
        close((*ev).fd);
        (*ev).fd = -(1 as libc::c_int);
        return;
    }
    i += l;
    j = 0 as libc::c_int;
    while j < l {
        if *((*bt).buf).offset((i - j - 1 as libc::c_int) as isize) as libc::c_int
            == '\n' as i32
        {
            break;
        }
        j += 1;
        j;
    }
    if j < l {
        k = i - j - 2 as libc::c_int;
        while k >= 0 as libc::c_int {
            if *((*bt).buf).offset(k as isize) as libc::c_int == '\n' as i32 {
                break;
            }
            k -= 1;
            k;
        }
        k += 1;
        k;
        bcopy(
            ((*bt).buf).offset(k as isize) as *const libc::c_void,
            ((*bt).result).as_mut_ptr() as *mut libc::c_void,
            (i - j - k) as size_t,
        );
        (*bt)
            .result[(i - j - k - 1 as libc::c_int)
            as usize] = 0 as libc::c_int as libc::c_char;
        backtick_filter(bt);
        WindowChanged(0 as *mut win, '`' as i32);
    }
    if j == l && i == 768 as libc::c_int {
        j = 768 as libc::c_int / 2 as libc::c_int;
        l = j + 1 as libc::c_int;
    }
    if j < l {
        if j != 0 {
            bcopy(
                ((*bt).buf).offset(i as isize).offset(-(j as isize))
                    as *const libc::c_void,
                (*bt).buf as *mut libc::c_void,
                j as size_t,
            );
        }
        i = j;
    }
    (*bt).bufi = i;
}
pub unsafe extern "C" fn setbacktick(
    mut num: libc::c_int,
    mut lifespan: libc::c_int,
    mut tick: libc::c_int,
    mut cmdv: *mut *mut libc::c_char,
) {
    let mut btp: *mut *mut backtick = 0 as *mut *mut backtick;
    let mut bt: *mut backtick = 0 as *mut backtick;
    let mut v: *mut *mut libc::c_char = 0 as *mut *mut libc::c_char;
    btp = &mut backticks;
    loop {
        bt = *btp;
        if bt.is_null() {
            break;
        }
        if (*bt).num == num {
            break;
        }
        btp = &mut (*bt).next;
    }
    if bt.is_null() && cmdv.is_null() {
        return;
    }
    if !bt.is_null() {
        v = (*bt).cmdv;
        while !(*v).is_null() {
            free(*v as *mut libc::c_void);
            v = v.offset(1);
            v;
        }
        free((*bt).cmdv as *mut libc::c_void);
        if !((*bt).buf).is_null() {
            free((*bt).buf as *mut libc::c_void);
        }
        if (*bt).ev.fd >= 0 as libc::c_int {
            close((*bt).ev.fd);
        }
        evdeq(&mut (*bt).ev);
    }
    if !bt.is_null() && cmdv.is_null() {
        *btp = (*bt).next;
        free(bt as *mut libc::c_void);
        return;
    }
    if bt.is_null() {
        bt = malloc(::std::mem::size_of::<backtick>() as libc::c_ulong) as *mut backtick;
        if bt.is_null() {
            Msg(
                0 as libc::c_int,
                b"%s\0" as *const u8 as *const libc::c_char,
                strnomem.as_mut_ptr(),
            );
            return;
        }
        bzero(
            bt as *mut libc::c_void,
            ::std::mem::size_of::<backtick>() as libc::c_ulong,
        );
        (*bt).next = 0 as *mut backtick;
        *btp = bt;
    }
    (*bt).num = num;
    (*bt).tick = tick;
    (*bt).lifespan = lifespan;
    (*bt).bestbefore = 0 as libc::c_int as time_t;
    (*bt).result[0 as libc::c_int as usize] = 0 as libc::c_int as libc::c_char;
    (*bt).buf = 0 as *mut libc::c_char;
    (*bt).bufi = 0 as libc::c_int;
    (*bt).cmdv = cmdv;
    (*bt).ev.fd = -(1 as libc::c_int);
    if (*bt).tick == 0 as libc::c_int && (*bt).lifespan == 0 as libc::c_int {
        (*bt).buf = malloc(768 as libc::c_int as libc::c_ulong) as *mut libc::c_char;
        if ((*bt).buf).is_null() {
            Msg(
                0 as libc::c_int,
                b"%s\0" as *const u8 as *const libc::c_char,
                strnomem.as_mut_ptr(),
            );
            setbacktick(
                num,
                0 as libc::c_int,
                0 as libc::c_int,
                0 as *mut *mut libc::c_char,
            );
            return;
        }
        (*bt).ev.type_0 = 1 as libc::c_int;
        (*bt).ev.fd = readpipe((*bt).cmdv);
        (*bt)
            .ev
            .handler = Some(
            backtick_fn as unsafe extern "C" fn(*mut event, *mut libc::c_char) -> (),
        );
        (*bt).ev.data = bt as *mut libc::c_char;
        if (*bt).ev.fd >= 0 as libc::c_int {
            evenq(&mut (*bt).ev);
        }
    }
}
unsafe extern "C" fn runbacktick(
    mut bt: *mut backtick,
    mut tickp: *mut libc::c_int,
    mut now: time_t,
) -> *mut libc::c_char {
    let mut f: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    let mut l: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut now2: time_t = 0;
    if (*bt).tick != 0 && (*tickp == 0 || (*bt).tick < *tickp) {
        *tickp = (*bt).tick;
    }
    if (*bt).lifespan == 0 as libc::c_int && (*bt).tick == 0 as libc::c_int
        || now < (*bt).bestbefore
    {
        return ((*bt).result).as_mut_ptr();
    }
    f = readpipe((*bt).cmdv);
    if f == -(1 as libc::c_int) {
        return ((*bt).result).as_mut_ptr();
    }
    i = 0 as libc::c_int;
    loop {
        l = read(
            f,
            ((*bt).result).as_mut_ptr().offset(i as isize) as *mut libc::c_void,
            (::std::mem::size_of::<[libc::c_char; 768]>() as libc::c_ulong)
                .wrapping_sub(i as libc::c_ulong),
        ) as libc::c_int;
        if !(l > 0 as libc::c_int) {
            break;
        }
        i += l;
        j = 1 as libc::c_int;
        while j < l {
            if (*bt).result[(i - j - 1 as libc::c_int) as usize] as libc::c_int
                == '\n' as i32
            {
                break;
            }
            j += 1;
            j;
        }
        if j == l
            && i as libc::c_ulong
                == ::std::mem::size_of::<[libc::c_char; 768]>() as libc::c_ulong
        {
            j = (::std::mem::size_of::<[libc::c_char; 768]>() as libc::c_ulong)
                .wrapping_div(2 as libc::c_int as libc::c_ulong) as libc::c_int;
            l = j + 1 as libc::c_int;
        }
        if j < l {
            bcopy(
                ((*bt).result).as_mut_ptr().offset(i as isize).offset(-(j as isize))
                    as *const libc::c_void,
                ((*bt).result).as_mut_ptr() as *mut libc::c_void,
                j as size_t,
            );
            i = j;
        }
    }
    close(f);
    (*bt)
        .result[(::std::mem::size_of::<[libc::c_char; 768]>() as libc::c_ulong)
        .wrapping_sub(1 as libc::c_int as libc::c_ulong)
        as usize] = '\n' as i32 as libc::c_char;
    if i != 0
        && (*bt).result[(i - 1 as libc::c_int) as usize] as libc::c_int == '\n' as i32
    {
        i -= 1;
        i;
    }
    (*bt).result[i as usize] = 0 as libc::c_int as libc::c_char;
    backtick_filter(bt);
    time(&mut now2);
    (*bt).bestbefore = now2 + (*bt).lifespan as libc::c_long;
    return ((*bt).result).as_mut_ptr();
}
pub unsafe extern "C" fn AddWinMsgRend(
    mut str: *const libc::c_char,
    mut r: libc::c_int,
) -> libc::c_int {
    if winmsg_numrend >= 256 as libc::c_int
        || str < winmsg_buf.as_mut_ptr() as *const libc::c_char
        || str
            >= winmsg_buf.as_mut_ptr().offset(768 as libc::c_int as isize)
                as *const libc::c_char
    {
        return -(1 as libc::c_int);
    }
    winmsg_rend[winmsg_numrend as usize] = r;
    winmsg_rendpos[winmsg_numrend
        as usize] = str.offset_from(winmsg_buf.as_mut_ptr()) as libc::c_long
        as libc::c_int;
    winmsg_numrend += 1;
    winmsg_numrend;
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn MakeWinMsgEv(
    mut str: *mut libc::c_char,
    mut win: *mut win,
    mut esc: libc::c_int,
    mut padlen: libc::c_int,
    mut ev: *mut event,
    mut rec: libc::c_int,
) -> *mut libc::c_char {
    static mut tick: libc::c_int = 0;
    let mut s: *mut libc::c_char = str;
    let mut p: *mut libc::c_char = winmsg_buf.as_mut_ptr();
    let mut ctrl: libc::c_int = 0;
    let mut now: timeval = timeval { tv_sec: 0, tv_usec: 0 };
    let mut tm: *mut tm = 0 as *mut tm;
    let mut l: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    let mut r: libc::c_int = 0;
    let mut num: libc::c_int = 0;
    let mut zeroflg: libc::c_int = 0;
    let mut longflg: libc::c_int = 0;
    let mut minusflg: libc::c_int = 0;
    let mut plusflg: libc::c_int = 0;
    let mut qmflag: libc::c_int = 0 as libc::c_int;
    let mut omflag: libc::c_int = 0 as libc::c_int;
    let mut qmnumrend: libc::c_int = 0 as libc::c_int;
    let mut qmpos: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut numpad: libc::c_int = 0 as libc::c_int;
    let mut lastpad: libc::c_int = 0 as libc::c_int;
    let mut truncpos: libc::c_int = -(1 as libc::c_int);
    let mut truncper: libc::c_int = 0 as libc::c_int;
    let mut trunclong: libc::c_int = 0 as libc::c_int;
    let mut bt: *mut backtick = 0 as *mut backtick;
    if winmsg_numrend >= 0 as libc::c_int {
        winmsg_numrend = 0 as libc::c_int;
    } else {
        winmsg_numrend = -winmsg_numrend;
    }
    tick = 0 as libc::c_int;
    tm = 0 as *mut tm;
    ctrl = 0 as libc::c_int;
    gettimeofday(&mut now, 0 as *mut libc::c_void);
    while *s as libc::c_int != 0
        && {
            l = winmsg_buf
                .as_mut_ptr()
                .offset(768 as libc::c_int as isize)
                .offset(-(1 as libc::c_int as isize))
                .offset_from(p) as libc::c_long as libc::c_int;
            l > 0 as libc::c_int
        }
    {
        *p = *s;
        if ctrl != 0 {
            ctrl = 0 as libc::c_int;
            if *s as libc::c_int != '^' as i32 && *s as libc::c_int >= 64 as libc::c_int
            {
                *p = (*p as libc::c_int & 0x1f as libc::c_int) as libc::c_char;
            }
        } else if *s as libc::c_int != esc {
            if esc == '%' as i32 {
                match *s as libc::c_int {
                    94 => {
                        ctrl = 1 as libc::c_int;
                        let fresh17 = p;
                        p = p.offset(-1);
                        *fresh17 = '^' as i32 as libc::c_char;
                    }
                    _ => {}
                }
            }
        } else {
            s = s.offset(1);
            if !(*s as libc::c_int == esc) {
                plusflg = (*s as libc::c_int == '+' as i32) as libc::c_int;
                if plusflg != 0 as libc::c_int {
                    s = s.offset(1);
                    s;
                }
                minusflg = (*s as libc::c_int == '-' as i32) as libc::c_int;
                if minusflg != 0 as libc::c_int {
                    s = s.offset(1);
                    s;
                }
                zeroflg = (*s as libc::c_int == '0' as i32) as libc::c_int;
                if zeroflg != 0 as libc::c_int {
                    s = s.offset(1);
                    s;
                }
                num = 0 as libc::c_int;
                while *s as libc::c_int >= '0' as i32 && *s as libc::c_int <= '9' as i32
                {
                    let fresh18 = s;
                    s = s.offset(1);
                    num = num * 10 as libc::c_int
                        + (*fresh18 as libc::c_int - '0' as i32);
                }
                longflg = (*s as libc::c_int == 'L' as i32) as libc::c_int;
                if longflg != 0 as libc::c_int {
                    s = s.offset(1);
                    s;
                }
                let mut current_block_315: u64;
                match *s as libc::c_int {
                    63 => {
                        p = p.offset(-1);
                        p;
                        if !qmpos.is_null() {
                            if qmflag == 0 && omflag == 0 || omflag == 1 as libc::c_int {
                                p = qmpos;
                                if qmnumrend < winmsg_numrend {
                                    winmsg_numrend = qmnumrend;
                                }
                            }
                            qmpos = 0 as *mut libc::c_char;
                        } else {
                            qmpos = p;
                            qmnumrend = winmsg_numrend;
                            omflag = 0 as libc::c_int;
                            qmflag = omflag;
                        }
                        current_block_315 = 12782750567233516376;
                    }
                    58 => {
                        p = p.offset(-1);
                        p;
                        if qmpos.is_null() {
                            current_block_315 = 12782750567233516376;
                        } else {
                            if qmflag != 0 && omflag != 1 as libc::c_int {
                                omflag = 1 as libc::c_int;
                                qmpos = p;
                                qmnumrend = winmsg_numrend;
                            } else {
                                p = qmpos;
                                if qmnumrend < winmsg_numrend {
                                    winmsg_numrend = qmnumrend;
                                }
                                omflag = -(1 as libc::c_int);
                            }
                            current_block_315 = 12782750567233516376;
                        }
                    }
                    100 | 68 | 109 | 77 | 121 | 89 | 97 | 65 | 115 | 99 | 67 => {
                        if l < 4 as libc::c_int {
                            current_block_315 = 12782750567233516376;
                        } else {
                            if tm.is_null() {
                                let mut nowsec: time_t = now.tv_sec;
                                tm = localtime(&mut nowsec);
                            }
                            qmflag = 1 as libc::c_int;
                            if tick == 0 || tick > 3600 as libc::c_int {
                                tick = 3600 as libc::c_int;
                            }
                            match *s as libc::c_int {
                                100 => {
                                    sprintf(
                                        p,
                                        b"%02d\0" as *const u8 as *const libc::c_char,
                                        (*tm).tm_mday % 100 as libc::c_int,
                                    );
                                }
                                68 => {
                                    strftime(
                                        p,
                                        l as size_t,
                                        if longflg != 0 {
                                            b"%A\0" as *const u8 as *const libc::c_char
                                        } else {
                                            b"%a\0" as *const u8 as *const libc::c_char
                                        },
                                        tm,
                                    );
                                }
                                109 => {
                                    sprintf(
                                        p,
                                        b"%02d\0" as *const u8 as *const libc::c_char,
                                        (*tm).tm_mon + 1 as libc::c_int,
                                    );
                                }
                                77 => {
                                    strftime(
                                        p,
                                        l as size_t,
                                        if longflg != 0 {
                                            b"%B\0" as *const u8 as *const libc::c_char
                                        } else {
                                            b"%b\0" as *const u8 as *const libc::c_char
                                        },
                                        tm,
                                    );
                                }
                                121 => {
                                    sprintf(
                                        p,
                                        b"%02d\0" as *const u8 as *const libc::c_char,
                                        (*tm).tm_year % 100 as libc::c_int,
                                    );
                                }
                                89 => {
                                    sprintf(
                                        p,
                                        b"%04d\0" as *const u8 as *const libc::c_char,
                                        (*tm).tm_year + 1900 as libc::c_int,
                                    );
                                }
                                97 => {
                                    sprintf(
                                        p,
                                        if (*tm).tm_hour >= 12 as libc::c_int {
                                            b"pm\0" as *const u8 as *const libc::c_char
                                        } else {
                                            b"am\0" as *const u8 as *const libc::c_char
                                        },
                                    );
                                }
                                65 => {
                                    sprintf(
                                        p,
                                        if (*tm).tm_hour >= 12 as libc::c_int {
                                            b"PM\0" as *const u8 as *const libc::c_char
                                        } else {
                                            b"AM\0" as *const u8 as *const libc::c_char
                                        },
                                    );
                                }
                                115 => {
                                    sprintf(
                                        p,
                                        b"%02d\0" as *const u8 as *const libc::c_char,
                                        (*tm).tm_sec,
                                    );
                                    tick = 1 as libc::c_int;
                                }
                                99 => {
                                    sprintf(
                                        p,
                                        if zeroflg != 0 {
                                            b"%02d:%02d\0" as *const u8 as *const libc::c_char
                                        } else {
                                            b"%2d:%02d\0" as *const u8 as *const libc::c_char
                                        },
                                        (*tm).tm_hour,
                                        (*tm).tm_min,
                                    );
                                    if tick == 0 || tick > 60 as libc::c_int {
                                        tick = 60 as libc::c_int;
                                    }
                                }
                                67 => {
                                    sprintf(
                                        p,
                                        if zeroflg != 0 {
                                            b"%02d:%02d\0" as *const u8 as *const libc::c_char
                                        } else {
                                            b"%2d:%02d\0" as *const u8 as *const libc::c_char
                                        },
                                        ((*tm).tm_hour + 11 as libc::c_int) % 12 as libc::c_int
                                            + 1 as libc::c_int,
                                        (*tm).tm_min,
                                    );
                                    if tick == 0 || tick > 60 as libc::c_int {
                                        tick = 60 as libc::c_int;
                                    }
                                }
                                _ => {}
                            }
                            p = p
                                .offset(
                                    (strlen(p)).wrapping_sub(1 as libc::c_int as libc::c_ulong)
                                        as isize,
                                );
                            current_block_315 = 12782750567233516376;
                        }
                    }
                    88 | 120 => {
                        *p = 0 as libc::c_int as libc::c_char;
                        i = 0 as libc::c_int;
                        while !win.is_null() && !((*win).w_cmdargs[i as usize]).is_null()
                        {
                            if (l as libc::c_ulong)
                                < (strlen((*win).w_cmdargs[i as usize]))
                                    .wrapping_add(1 as libc::c_int as libc::c_ulong)
                            {
                                break;
                            }
                            sprintf(
                                p,
                                if i != 0 {
                                    b" %s\0" as *const u8 as *const libc::c_char
                                } else {
                                    b"%s\0" as *const u8 as *const libc::c_char
                                },
                                (*win).w_cmdargs[i as usize],
                            );
                            l = (l as libc::c_ulong).wrapping_sub(strlen(p))
                                as libc::c_int as libc::c_int;
                            p = p.offset(strlen(p) as isize);
                            if i == 0 as libc::c_int && *s as libc::c_int == 'X' as i32 {
                                break;
                            }
                            i += 1;
                            i;
                        }
                        p = p.offset(-1);
                        p;
                        current_block_315 = 12782750567233516376;
                    }
                    108 => {
                        *p = 0 as libc::c_int as libc::c_char;
                        if l > 20 as libc::c_int {
                            AddLoadav(p);
                        }
                        if *p != 0 {
                            qmflag = 1 as libc::c_int;
                            p = p
                                .offset(
                                    (strlen(p)).wrapping_sub(1 as libc::c_int as libc::c_ulong)
                                        as isize,
                                );
                        } else {
                            *p = '?' as i32 as libc::c_char;
                        }
                        if tick == 0 || tick > 60 as libc::c_int {
                            tick = 60 as libc::c_int;
                        }
                        p = p
                            .offset(
                                (strlen(p)).wrapping_sub(1 as libc::c_int as libc::c_ulong)
                                    as isize,
                            );
                        current_block_315 = 12782750567233516376;
                    }
                    96 | 104 => {
                        if rec >= 10 as libc::c_int
                            || *s as libc::c_int == 'h' as i32
                                && (win.is_null() || ((*win).w_hstatus).is_null()
                                    || *(*win).w_hstatus as libc::c_int == 0 as libc::c_int)
                        {
                            p = p.offset(-1);
                            p;
                        } else {
                            if *s as libc::c_int == '`' as i32 {
                                bt = backticks;
                                while !bt.is_null() {
                                    if (*bt).num == num {
                                        break;
                                    }
                                    bt = (*bt).next;
                                }
                                if bt.is_null() {
                                    p = p.offset(-1);
                                    p;
                                    current_block_315 = 12782750567233516376;
                                } else {
                                    current_block_315 = 6478348674394853609;
                                }
                            } else {
                                current_block_315 = 6478348674394853609;
                            }
                            match current_block_315 {
                                12782750567233516376 => {}
                                _ => {
                                    let mut savebuf: [libc::c_char; 768] = [0; 768];
                                    let mut oldtick: libc::c_int = tick;
                                    let mut oldnumrend: libc::c_int = winmsg_numrend;
                                    *p = 0 as libc::c_int as libc::c_char;
                                    strcpy(savebuf.as_mut_ptr(), winmsg_buf.as_mut_ptr());
                                    winmsg_numrend = -winmsg_numrend;
                                    MakeWinMsgEv(
                                        if *s as libc::c_int == 'h' as i32 {
                                            (*win).w_hstatus
                                        } else {
                                            runbacktick(bt, &mut oldtick, now.tv_sec)
                                        },
                                        win,
                                        '\u{5}' as i32,
                                        0 as libc::c_int,
                                        0 as *mut event,
                                        rec + 1 as libc::c_int,
                                    );
                                    if tick == 0 || oldtick < tick {
                                        tick = oldtick;
                                    }
                                    if (strlen(winmsg_buf.as_mut_ptr()) as libc::c_int) < l {
                                        strcat(savebuf.as_mut_ptr(), winmsg_buf.as_mut_ptr());
                                    }
                                    strcpy(winmsg_buf.as_mut_ptr(), savebuf.as_mut_ptr());
                                    while oldnumrend < winmsg_numrend {
                                        let fresh19 = oldnumrend;
                                        oldnumrend = oldnumrend + 1;
                                        winmsg_rendpos[fresh19
                                            as usize] = (winmsg_rendpos[fresh19 as usize]
                                            as libc::c_long
                                            + p.offset_from(winmsg_buf.as_mut_ptr()) as libc::c_long)
                                            as libc::c_int;
                                    }
                                    if *p != 0 {
                                        qmflag = 1 as libc::c_int;
                                    }
                                    p = p
                                        .offset(
                                            (strlen(p)).wrapping_sub(1 as libc::c_int as libc::c_ulong)
                                                as isize,
                                        );
                                }
                            }
                        }
                        current_block_315 = 12782750567233516376;
                    }
                    119 | 87 => {
                        let mut oldfore: *mut win = 0 as *mut win;
                        let mut ss: *mut libc::c_char = 0 as *mut libc::c_char;
                        if !display.is_null() {
                            oldfore = (*display).d_fore;
                            (*display).d_fore = win;
                        }
                        ss = AddWindows(
                            p,
                            l - 1 as libc::c_int,
                            (if *s as libc::c_int == 'w' as i32 {
                                0 as libc::c_int
                            } else {
                                1 as libc::c_int
                            })
                                | (if longflg != 0 {
                                    0 as libc::c_int
                                } else {
                                    2 as libc::c_int
                                })
                                | (if plusflg != 0 {
                                    4 as libc::c_int
                                } else {
                                    0 as libc::c_int
                                })
                                | (if minusflg != 0 {
                                    8 as libc::c_int
                                } else {
                                    0 as libc::c_int
                                }),
                            if !win.is_null() {
                                (*win).w_number
                            } else {
                                -(1 as libc::c_int)
                            },
                        );
                        if !display.is_null() {
                            (*display).d_fore = oldfore;
                        }
                        if *p != 0 {
                            qmflag = 1 as libc::c_int;
                        }
                        p = p
                            .offset(
                                (strlen(p)).wrapping_sub(1 as libc::c_int as libc::c_ulong)
                                    as isize,
                            );
                        current_block_315 = 12782750567233516376;
                    }
                    117 => {
                        *p = 0 as libc::c_int as libc::c_char;
                        if !win.is_null() {
                            AddOtherUsers(p, l - 1 as libc::c_int, win);
                        }
                        if *p != 0 {
                            qmflag = 1 as libc::c_int;
                        }
                        p = p
                            .offset(
                                (strlen(p)).wrapping_sub(1 as libc::c_int as libc::c_ulong)
                                    as isize,
                            );
                        current_block_315 = 12782750567233516376;
                    }
                    102 => {
                        *p = 0 as libc::c_int as libc::c_char;
                        if !win.is_null() {
                            AddWindowFlags(p, l - 1 as libc::c_int, win);
                        }
                        if *p != 0 {
                            qmflag = 1 as libc::c_int;
                        }
                        p = p
                            .offset(
                                (strlen(p)).wrapping_sub(1 as libc::c_int as libc::c_ulong)
                                    as isize,
                            );
                        current_block_315 = 12782750567233516376;
                    }
                    116 => {
                        *p = 0 as libc::c_int as libc::c_char;
                        if !win.is_null() && (strlen((*win).w_title) as libc::c_int) < l
                        {
                            strcpy(p, (*win).w_title);
                            if *p != 0 {
                                qmflag = 1 as libc::c_int;
                            }
                        }
                        p = p
                            .offset(
                                (strlen(p)).wrapping_sub(1 as libc::c_int as libc::c_ulong)
                                    as isize,
                            );
                        current_block_315 = 12782750567233516376;
                    }
                    101 => {
                        *p = 0 as libc::c_int as libc::c_char;
                        (*display)
                            .d_encoding = if nwin_options.encoding > 0 as libc::c_int {
                            nwin_options.encoding
                        } else {
                            0 as libc::c_int
                        };
                        if !win.is_null() && (*win).w_layer.l_encoding != 0 {
                            let fresh20 = p;
                            p = p.offset(1);
                            *fresh20 = ' ' as i32 as libc::c_char;
                            strcpy(p, EncodingName((*win).w_layer.l_encoding));
                        }
                        p = p
                            .offset(
                                (strlen(p)).wrapping_sub(1 as libc::c_int as libc::c_ulong)
                                    as isize,
                            );
                        current_block_315 = 12782750567233516376;
                    }
                    123 => {
                        let mut rbuf: [libc::c_char; 128] = [0; 128];
                        s = s.offset(1);
                        s;
                        i = 0 as libc::c_int;
                        while i < 127 as libc::c_int {
                            if !(*s.offset(i as isize) as libc::c_int != 0
                                && *s.offset(i as isize) as libc::c_int != '}' as i32)
                            {
                                break;
                            }
                            rbuf[i as usize] = *s.offset(i as isize);
                            i += 1;
                            i;
                        }
                        if *s.offset(i as isize) as libc::c_int == '}' as i32
                            && winmsg_numrend < 256 as libc::c_int
                        {
                            r = -(1 as libc::c_int);
                            rbuf[i as usize] = 0 as libc::c_int as libc::c_char;
                            if i != 1 as libc::c_int
                                || rbuf[0 as libc::c_int as usize] as libc::c_int
                                    != '-' as i32
                            {
                                r = ParseAttrColor(
                                    rbuf.as_mut_ptr(),
                                    0 as *mut libc::c_char,
                                    0 as libc::c_int,
                                );
                            }
                            if r != -(1 as libc::c_int)
                                || i == 1 as libc::c_int
                                    && rbuf[0 as libc::c_int as usize] as libc::c_int
                                        == '-' as i32
                            {
                                winmsg_rend[winmsg_numrend as usize] = r;
                                winmsg_rendpos[winmsg_numrend
                                    as usize] = p.offset_from(winmsg_buf.as_mut_ptr())
                                    as libc::c_long as libc::c_int;
                                winmsg_numrend += 1;
                                winmsg_numrend;
                            }
                        }
                        s = s.offset(i as isize);
                        p = p.offset(-1);
                        p;
                        current_block_315 = 12782750567233516376;
                    }
                    72 => {
                        *p = 0 as libc::c_int as libc::c_char;
                        if (strlen(HostName.as_mut_ptr()) as libc::c_int) < l {
                            strcpy(p, HostName.as_mut_ptr());
                            if *p != 0 {
                                qmflag = 1 as libc::c_int;
                            }
                        }
                        p = p
                            .offset(
                                (strlen(p)).wrapping_sub(1 as libc::c_int as libc::c_ulong)
                                    as isize,
                            );
                        current_block_315 = 12782750567233516376;
                    }
                    83 => {
                        let mut session_name: *mut libc::c_char = 0 as *mut libc::c_char;
                        *p = 0 as libc::c_int as libc::c_char;
                        session_name = (strchr(SockName, '.' as i32))
                            .offset(1 as libc::c_int as isize);
                        if (strlen(session_name) as libc::c_int) < l {
                            strcpy(p, session_name);
                            if *p != 0 {
                                qmflag = 1 as libc::c_int;
                            }
                        }
                        p = p
                            .offset(
                                (strlen(p)).wrapping_sub(1 as libc::c_int as libc::c_ulong)
                                    as isize,
                            );
                        current_block_315 = 12782750567233516376;
                    }
                    112 => {
                        sprintf(
                            p,
                            b"%d\0" as *const u8 as *const libc::c_char,
                            if plusflg != 0 && !display.is_null() {
                                (*display).d_userpid
                            } else {
                                getpid()
                            },
                        );
                        p = p
                            .offset(
                                (strlen(p)).wrapping_sub(1 as libc::c_int as libc::c_ulong)
                                    as isize,
                            );
                        current_block_315 = 12782750567233516376;
                    }
                    70 => {
                        p = p.offset(-1);
                        p;
                        if !display.is_null()
                            && (!ev.is_null()
                                && ev == &mut (*(*display).d_forecv).c_captev as *mut event
                                || ev.is_null() && !win.is_null()
                                    && win == (*display).d_fore)
                        {
                            minusflg = (minusflg == 0) as libc::c_int;
                        }
                        if minusflg != 0 {
                            qmflag = 1 as libc::c_int;
                        }
                        current_block_315 = 12782750567233516376;
                    }
                    80 => {
                        p = p.offset(-1);
                        p;
                        if !display.is_null() && !ev.is_null()
                            && ev != &mut (*display).d_hstatusev as *mut event
                        {
                            let mut cv: *mut canvas = (*ev).data as *mut canvas;
                            if ev == &mut (*cv).c_captev as *mut event
                                && (*(*cv).c_layer).l_layfn == &mut MarkLf as *mut LayFuncs
                            {
                                qmflag = 1 as libc::c_int;
                            }
                        }
                        current_block_315 = 12782750567233516376;
                    }
                    69 => {
                        p = p.offset(-1);
                        p;
                        if !display.is_null() && !((*display).d_ESCseen).is_null() {
                            qmflag = 1 as libc::c_int;
                        }
                        current_block_315 = 12782750567233516376;
                    }
                    62 => {
                        truncpos = p.offset_from(winmsg_buf.as_mut_ptr()) as libc::c_long
                            as libc::c_int;
                        truncper = if num > 100 as libc::c_int {
                            100 as libc::c_int
                        } else {
                            num
                        };
                        trunclong = longflg;
                        p = p.offset(-1);
                        p;
                        current_block_315 = 12782750567233516376;
                    }
                    61 | 60 => {
                        *p = ' ' as i32 as libc::c_char;
                        if num != 0 || zeroflg != 0 || plusflg != 0 || longflg != 0
                            || *s as libc::c_int != '=' as i32
                        {
                            if minusflg != 0 {
                                num = (if plusflg != 0 { lastpad } else { padlen }) - num;
                                if plusflg == 0 && padlen == 0 as libc::c_int {
                                    num = p.offset_from(winmsg_buf.as_mut_ptr()) as libc::c_long
                                        as libc::c_int;
                                }
                                plusflg = 0 as libc::c_int;
                            } else if zeroflg == 0 {
                                if *s as libc::c_int != '=' as i32
                                    && num == 0 as libc::c_int && plusflg == 0
                                {
                                    num = 100 as libc::c_int;
                                }
                                if num > 100 as libc::c_int {
                                    num = 100 as libc::c_int;
                                }
                                if padlen == 0 as libc::c_int {
                                    num = p.offset_from(winmsg_buf.as_mut_ptr()) as libc::c_long
                                        as libc::c_int;
                                } else {
                                    num = (padlen
                                        - (if plusflg != 0 { lastpad } else { 0 as libc::c_int }))
                                        * num / 100 as libc::c_int;
                                }
                            }
                            if num < 0 as libc::c_int {
                                num = 0 as libc::c_int;
                            }
                            if plusflg != 0 {
                                num += lastpad;
                            }
                            if num > 768 as libc::c_int - 1 as libc::c_int {
                                num = 768 as libc::c_int - 1 as libc::c_int;
                            }
                            if numpad != 0 {
                                p = pad_expand(winmsg_buf.as_mut_ptr(), p, numpad, num);
                            }
                            numpad = 0 as libc::c_int;
                            if p.offset_from(winmsg_buf.as_mut_ptr()) as libc::c_long
                                > num as libc::c_long && longflg == 0
                            {
                                let mut left: libc::c_int = 0;
                                let mut trunc: libc::c_int = 0;
                                if truncpos == -(1 as libc::c_int) {
                                    truncpos = lastpad;
                                    truncper = 0 as libc::c_int;
                                }
                                trunc = lastpad
                                    + truncper * (num - lastpad) / 100 as libc::c_int;
                                if trunc > num {
                                    trunc = num;
                                }
                                if trunc < lastpad {
                                    trunc = lastpad;
                                }
                                left = truncpos - trunc;
                                if left as libc::c_long
                                    > p.offset_from(winmsg_buf.as_mut_ptr()) as libc::c_long
                                        - num as libc::c_long
                                {
                                    left = (p.offset_from(winmsg_buf.as_mut_ptr())
                                        as libc::c_long - num as libc::c_long) as libc::c_int;
                                }
                                if left > 0 as libc::c_int {
                                    if (left + lastpad) as libc::c_long
                                        > p.offset_from(winmsg_buf.as_mut_ptr()) as libc::c_long
                                    {
                                        left = (p.offset_from(winmsg_buf.as_mut_ptr())
                                            as libc::c_long - lastpad as libc::c_long) as libc::c_int;
                                    }
                                    if p.offset_from(winmsg_buf.as_mut_ptr()) as libc::c_long
                                        - lastpad as libc::c_long - left as libc::c_long
                                        > 0 as libc::c_int as libc::c_long
                                    {
                                        bcopy(
                                            winmsg_buf
                                                .as_mut_ptr()
                                                .offset(lastpad as isize)
                                                .offset(left as isize) as *const libc::c_void,
                                            winmsg_buf.as_mut_ptr().offset(lastpad as isize)
                                                as *mut libc::c_void,
                                            (p.offset_from(winmsg_buf.as_mut_ptr()) as libc::c_long
                                                - lastpad as libc::c_long - left as libc::c_long) as size_t,
                                        );
                                    }
                                    p = p.offset(-(left as isize));
                                    r = winmsg_numrend;
                                    while r != 0
                                        && winmsg_rendpos[(r - 1 as libc::c_int) as usize] > lastpad
                                    {
                                        r -= 1;
                                        r;
                                        winmsg_rendpos[r as usize] -= left;
                                        if winmsg_rendpos[r as usize] < lastpad {
                                            winmsg_rendpos[r as usize] = lastpad;
                                        }
                                    }
                                    if trunclong != 0 {
                                        if p.offset_from(winmsg_buf.as_mut_ptr()) as libc::c_long
                                            > lastpad as libc::c_long
                                        {
                                            winmsg_buf[lastpad as usize] = '.' as i32 as libc::c_char;
                                        }
                                        if p.offset_from(winmsg_buf.as_mut_ptr()) as libc::c_long
                                            > (lastpad + 1 as libc::c_int) as libc::c_long
                                        {
                                            winmsg_buf[(lastpad + 1 as libc::c_int)
                                                as usize] = '.' as i32 as libc::c_char;
                                        }
                                        if p.offset_from(winmsg_buf.as_mut_ptr()) as libc::c_long
                                            > (lastpad + 2 as libc::c_int) as libc::c_long
                                        {
                                            winmsg_buf[(lastpad + 2 as libc::c_int)
                                                as usize] = '.' as i32 as libc::c_char;
                                        }
                                    }
                                }
                                if p.offset_from(winmsg_buf.as_mut_ptr()) as libc::c_long
                                    > num as libc::c_long
                                {
                                    p = winmsg_buf.as_mut_ptr().offset(num as isize);
                                    if trunclong != 0 {
                                        if num - 1 as libc::c_int >= lastpad {
                                            *p
                                                .offset(
                                                    -(1 as libc::c_int) as isize,
                                                ) = '.' as i32 as libc::c_char;
                                        }
                                        if num - 2 as libc::c_int >= lastpad {
                                            *p
                                                .offset(
                                                    -(2 as libc::c_int) as isize,
                                                ) = '.' as i32 as libc::c_char;
                                        }
                                        if num - 3 as libc::c_int >= lastpad {
                                            *p
                                                .offset(
                                                    -(3 as libc::c_int) as isize,
                                                ) = '.' as i32 as libc::c_char;
                                        }
                                    }
                                    r = winmsg_numrend;
                                    while r != 0
                                        && winmsg_rendpos[(r - 1 as libc::c_int) as usize] > num
                                    {
                                        r -= 1;
                                        winmsg_rendpos[r as usize] = num;
                                    }
                                }
                                truncpos = -(1 as libc::c_int);
                                trunclong = 0 as libc::c_int;
                                if lastpad as libc::c_long
                                    > p.offset_from(winmsg_buf.as_mut_ptr()) as libc::c_long
                                {
                                    lastpad = p.offset_from(winmsg_buf.as_mut_ptr())
                                        as libc::c_long as libc::c_int;
                                }
                            }
                            if *s as libc::c_int == '=' as i32 {
                                while (p.offset_from(winmsg_buf.as_mut_ptr())
                                    as libc::c_long) < num as libc::c_long
                                {
                                    let fresh21 = p;
                                    p = p.offset(1);
                                    *fresh21 = ' ' as i32 as libc::c_char;
                                }
                                lastpad = p.offset_from(winmsg_buf.as_mut_ptr())
                                    as libc::c_long as libc::c_int;
                                truncpos = -(1 as libc::c_int);
                                trunclong = 0 as libc::c_int;
                            }
                            p = p.offset(-1);
                            p;
                        } else if padlen != 0 {
                            *p = 127 as libc::c_int as libc::c_char;
                            numpad += 1;
                            numpad;
                        }
                        current_block_315 = 12782750567233516376;
                    }
                    110 => {
                        s = s.offset(1);
                        s;
                        current_block_315 = 11641124301550588080;
                    }
                    _ => {
                        current_block_315 = 11641124301550588080;
                    }
                }
                match current_block_315 {
                    11641124301550588080 => {
                        s = s.offset(-1);
                        s;
                        if l > 10 as libc::c_int + num {
                            if num == 0 as libc::c_int {
                                num = 1 as libc::c_int;
                            }
                            if win.is_null() {
                                sprintf(
                                    p,
                                    b"%*s\0" as *const u8 as *const libc::c_char,
                                    num,
                                    if num > 1 as libc::c_int {
                                        b"--\0" as *const u8 as *const libc::c_char
                                    } else {
                                        b"-\0" as *const u8 as *const libc::c_char
                                    },
                                );
                            } else {
                                sprintf(
                                    p,
                                    b"%*d\0" as *const u8 as *const libc::c_char,
                                    num,
                                    (*win).w_number,
                                );
                            }
                            qmflag = 1 as libc::c_int;
                            p = p
                                .offset(
                                    (strlen(p)).wrapping_sub(1 as libc::c_int as libc::c_ulong)
                                        as isize,
                                );
                        }
                    }
                    _ => {}
                }
            }
        }
        s = s.offset(1);
        s;
        p = p.offset(1);
        p;
    }
    if !qmpos.is_null() && qmflag == 0 {
        p = qmpos.offset(1 as libc::c_int as isize);
    }
    *p = '\0' as i32 as libc::c_char;
    if numpad != 0 {
        if padlen > 768 as libc::c_int - 1 as libc::c_int {
            padlen = 768 as libc::c_int - 1 as libc::c_int;
        }
        p = pad_expand(winmsg_buf.as_mut_ptr(), p, numpad, padlen);
    }
    if !ev.is_null() {
        evdeq(ev);
        (*ev).timeout.tv_sec = 0 as libc::c_int as __time_t;
        (*ev).timeout.tv_usec = 0 as libc::c_int as __suseconds_t;
    }
    if !ev.is_null() && tick != 0 {
        now.tv_usec = 100000 as libc::c_int as __suseconds_t;
        if tick == 1 as libc::c_int {
            now.tv_sec += 1;
            now.tv_sec;
        } else {
            now.tv_sec += tick as libc::c_long - now.tv_sec % tick as libc::c_long;
        }
        (*ev).timeout = now;
    }
    return winmsg_buf.as_mut_ptr();
}
pub unsafe extern "C" fn MakeWinMsg(
    mut s: *mut libc::c_char,
    mut win: *mut win,
    mut esc: libc::c_int,
) -> *mut libc::c_char {
    return MakeWinMsgEv(
        s,
        win,
        esc,
        0 as libc::c_int,
        0 as *mut event,
        0 as libc::c_int,
    );
}
pub unsafe extern "C" fn PutWinMsg(
    mut s: *mut libc::c_char,
    mut start: libc::c_int,
    mut max: libc::c_int,
) {
    let mut i: libc::c_int = 0;
    let mut p: libc::c_int = 0;
    let mut l: libc::c_int = 0;
    let mut r: libc::c_int = 0;
    let mut n: libc::c_int = 0;
    let mut rend: mchar = mchar {
        image: 0,
        attr: 0,
        font: 0,
        fontx: 0,
        color: 0,
        mbcs: 0,
    };
    let mut rendstack: [mchar; 256] = [mchar {
        image: 0,
        attr: 0,
        font: 0,
        fontx: 0,
        color: 0,
        mbcs: 0,
    }; 256];
    let mut rendstackn: libc::c_int = 0 as libc::c_int;
    if s != winmsg_buf.as_mut_ptr() {
        l = strlen(s) as libc::c_int;
        if l > max {
            l = max;
        }
        l -= start;
        s = s.offset(start as isize);
        loop {
            let fresh22 = l;
            l = l - 1;
            if !(fresh22 > 0 as libc::c_int) {
                break;
            }
            let fresh23 = s;
            s = s.offset(1);
            PUTCHARLP(*fresh23 as libc::c_int);
        }
        return;
    }
    rend = (*display).d_rend;
    p = 0 as libc::c_int;
    l = strlen(s) as libc::c_int;
    i = 0 as libc::c_int;
    while i < winmsg_numrend && max > 0 as libc::c_int {
        if p > winmsg_rendpos[i as usize] || winmsg_rendpos[i as usize] > l {
            break;
        }
        if p < winmsg_rendpos[i as usize] {
            n = winmsg_rendpos[i as usize] - p;
            if n > max {
                n = max;
            }
            max -= n;
            p += n;
            loop {
                let fresh24 = n;
                n = n - 1;
                if !(fresh24 > 0 as libc::c_int) {
                    break;
                }
                let fresh25 = start;
                start = start - 1;
                if fresh25 > 0 as libc::c_int {
                    s = s.offset(1);
                    s;
                } else {
                    let fresh26 = s;
                    s = s.offset(1);
                    PUTCHARLP(*fresh26 as libc::c_int);
                }
            }
        }
        r = winmsg_rend[i as usize];
        if r == -(1 as libc::c_int) {
            if rendstackn > 0 as libc::c_int {
                rendstackn -= 1;
                rend = rendstack[rendstackn as usize];
            }
        } else {
            let fresh27 = rendstackn;
            rendstackn = rendstackn + 1;
            rendstack[fresh27 as usize] = rend;
            ApplyAttrColor(r, &mut rend);
        }
        SetRendition(&mut rend);
        i += 1;
        i;
    }
    if p < l {
        n = l - p;
        if n > max {
            n = max;
        }
        loop {
            let fresh28 = n;
            n = n - 1;
            if !(fresh28 > 0 as libc::c_int) {
                break;
            }
            let fresh29 = start;
            start = start - 1;
            if fresh29 > 0 as libc::c_int {
                s = s.offset(1);
                s;
            } else {
                let fresh30 = s;
                s = s.offset(1);
                PUTCHARLP(*fresh30 as libc::c_int);
            }
        }
    }
}
unsafe extern "C" fn serv_read_fn(mut ev: *mut event, mut data: *mut libc::c_char) {
    ReceiveMsg();
}
unsafe extern "C" fn serv_select_fn(mut ev: *mut event, mut data: *mut libc::c_char) {
    let mut p: *mut win = 0 as *mut win;
    if GotSigChld != 0 {
        SigChldHandler();
    }
    if InterruptPlease != 0 {
        if !fore.is_null() && !displays.is_null() {
            let mut ibuf: libc::c_char = (*displays)
                .d_OldMode
                .tio
                .c_cc[0 as libc::c_int as usize] as libc::c_char;
            write(
                if !((*fore).w_pwin).is_null()
                    && (*(*fore).w_pwin).p_fdpat & 0x1000 as libc::c_int != 0
                {
                    (*(*fore).w_pwin).p_ptyfd
                } else {
                    (*fore).w_ptyfd
                },
                &mut ibuf as *mut libc::c_char as *const libc::c_void,
                1 as libc::c_int as size_t,
            );
        }
        InterruptPlease = 0 as libc::c_int;
    }
    p = windows;
    while !p.is_null() {
        if (*p).w_bell == 1 as libc::c_int || (*p).w_bell == 3 as libc::c_int {
            let mut cv: *mut canvas = 0 as *mut canvas;
            let mut visual: libc::c_int = ((*p).w_bell == 3 as libc::c_int
                || visual_bell != 0) as libc::c_int;
            (*p).w_bell = 0 as libc::c_int;
            display = displays;
            while !display.is_null() {
                cv = (*display).d_cvlist;
                while !cv.is_null() {
                    if (*(*cv).c_layer).l_bottom == &mut (*p).w_layer as *mut layer {
                        break;
                    }
                    cv = (*cv).c_next;
                }
                if cv.is_null() {
                    (*p).w_bell = 2 as libc::c_int;
                    Msg(
                        0 as libc::c_int,
                        b"%s\0" as *const u8 as *const libc::c_char,
                        MakeWinMsg(BellString, p, '%' as i32),
                    );
                } else if visual != 0
                    && ((*display).d_tcs[43 as libc::c_int as usize].str_0).is_null()
                    && ((*display).d_status == 0 || (*display).d_status_bell == 0)
                {
                    Msg(
                        0 as libc::c_int,
                        b"%s\0" as *const u8 as *const libc::c_char,
                        VisualBellString,
                    );
                    if (*display).d_status != 0 {
                        (*display).d_status_bell = 1 as libc::c_int as libc::c_char;
                        SetTimeout(&mut (*display).d_statusev, VBellWait);
                    }
                }
                display = (*display).d_next;
            }
            if (*p).w_monitor == 2 as libc::c_int {
                (*p).w_monitor = 3 as libc::c_int;
            }
            WindowChanged(p, 'f' as i32);
        }
        if (*p).w_monitor == 2 as libc::c_int {
            let mut cv_0: *mut canvas = 0 as *mut canvas;
            (*p).w_monitor = 1 as libc::c_int;
            display = displays;
            while !display.is_null() {
                cv_0 = (*display).d_cvlist;
                while !cv_0.is_null() {
                    if (*(*cv_0).c_layer).l_bottom == &mut (*p).w_layer as *mut layer {
                        break;
                    }
                    cv_0 = (*cv_0).c_next;
                }
                if cv_0.is_null() {
                    if !(*((*p).w_mon_notify)
                        .offset(((*(*display).d_user).u_id >> 3 as libc::c_int) as isize)
                        as libc::c_int
                        & 0x80 as libc::c_int
                            >> ((*(*display).d_user).u_id & 7 as libc::c_int) == 0)
                    {
                        Msg(
                            0 as libc::c_int,
                            b"%s\0" as *const u8 as *const libc::c_char,
                            MakeWinMsg(ActivityString, p, '%' as i32),
                        );
                        (*p).w_monitor = 3 as libc::c_int;
                    }
                }
                display = (*display).d_next;
            }
            WindowChanged(p, 'f' as i32);
        }
        if (*p).w_silence == 2 as libc::c_int {
            if !((*p).w_layer.l_cvlist).is_null() {
                (*p).w_silence = 1 as libc::c_int;
                WindowChanged(p, 'f' as i32);
            }
        }
        p = (*p).w_next;
    }
    display = displays;
    while !display.is_null() {
        let mut cv_1: *mut canvas = 0 as *mut canvas;
        if !((*display).d_status == 1 as libc::c_int) {
            cv_1 = (*display).d_cvlist;
            while !cv_1.is_null() {
                let mut lx: libc::c_int = 0;
                let mut ly: libc::c_int = 0;
                lx = (*(*cv_1).c_layer).l_x;
                ly = (*(*cv_1).c_layer).l_y;
                if lx == (*(*cv_1).c_layer).l_width {
                    lx -= 1;
                    lx;
                }
                if ly + (*cv_1).c_yoff < (*cv_1).c_ys {
                    let mut i: libc::c_int = 0;
                    let mut n: libc::c_int = (*cv_1).c_ys - (ly + (*cv_1).c_yoff);
                    (*cv_1).c_yoff = (*cv_1).c_ys - ly;
                    RethinkViewportOffsets(cv_1);
                    if n > (*(*cv_1).c_layer).l_height {
                        n = (*(*cv_1).c_layer).l_height;
                    }
                    let mut olddisplay: *mut display = display;
                    let mut oldflayer: *mut layer = flayer;
                    let mut l: *mut layer = (*cv_1).c_layer;
                    let mut cvlist: *mut canvas = (*l).l_cvlist;
                    let mut cvlnext: *mut canvas = (*cv_1).c_lnext;
                    flayer = l;
                    (*l).l_cvlist = cv_1;
                    (*cv_1).c_lnext = 0 as *mut canvas;
                    LScrollV(
                        flayer,
                        -n,
                        0 as libc::c_int,
                        (*flayer).l_height - 1 as libc::c_int,
                        0 as libc::c_int,
                    );
                    (Some(((*(*flayer).l_layfn).lf_LayRedisplayLine).unwrap()))
                        .unwrap()(
                        -(1 as libc::c_int),
                        -(1 as libc::c_int),
                        -(1 as libc::c_int),
                        1 as libc::c_int,
                    );
                    i = 0 as libc::c_int;
                    while i < n {
                        (Some(((*(*flayer).l_layfn).lf_LayRedisplayLine).unwrap()))
                            .unwrap()(
                            i,
                            0 as libc::c_int,
                            (*flayer).l_width - 1 as libc::c_int,
                            1 as libc::c_int,
                        );
                        i += 1;
                        i;
                    }
                    if cv_1 == (*(*cv_1).c_display).d_forecv {
                        LGotoPos(flayer, (*flayer).l_x, (*flayer).l_y);
                    }
                    flayer = oldflayer;
                    (*l).l_cvlist = cvlist;
                    (*cv_1).c_lnext = cvlnext;
                    display = olddisplay;
                } else if ly + (*cv_1).c_yoff > (*cv_1).c_ye {
                    let mut i_0: libc::c_int = 0;
                    let mut n_0: libc::c_int = ly + (*cv_1).c_yoff - (*cv_1).c_ye;
                    (*cv_1).c_yoff = (*cv_1).c_ye - ly;
                    RethinkViewportOffsets(cv_1);
                    if n_0 > (*(*cv_1).c_layer).l_height {
                        n_0 = (*(*cv_1).c_layer).l_height;
                    }
                    let mut olddisplay_0: *mut display = display;
                    let mut oldflayer_0: *mut layer = flayer;
                    let mut l_0: *mut layer = (*cv_1).c_layer;
                    let mut cvlist_0: *mut canvas = (*l_0).l_cvlist;
                    let mut cvlnext_0: *mut canvas = (*cv_1).c_lnext;
                    flayer = l_0;
                    (*l_0).l_cvlist = cv_1;
                    (*cv_1).c_lnext = 0 as *mut canvas;
                    LScrollV(
                        flayer,
                        n_0,
                        0 as libc::c_int,
                        (*(*cv_1).c_layer).l_height - 1 as libc::c_int,
                        0 as libc::c_int,
                    );
                    (Some(((*(*flayer).l_layfn).lf_LayRedisplayLine).unwrap()))
                        .unwrap()(
                        -(1 as libc::c_int),
                        -(1 as libc::c_int),
                        -(1 as libc::c_int),
                        1 as libc::c_int,
                    );
                    i_0 = 0 as libc::c_int;
                    while i_0 < n_0 {
                        (Some(((*(*flayer).l_layfn).lf_LayRedisplayLine).unwrap()))
                            .unwrap()(
                            i_0 + (*flayer).l_height - n_0,
                            0 as libc::c_int,
                            (*flayer).l_width - 1 as libc::c_int,
                            1 as libc::c_int,
                        );
                        i_0 += 1;
                        i_0;
                    }
                    if cv_1 == (*(*cv_1).c_display).d_forecv {
                        LGotoPos(flayer, (*flayer).l_x, (*flayer).l_y);
                    }
                    flayer = oldflayer_0;
                    (*l_0).l_cvlist = cvlist_0;
                    (*cv_1).c_lnext = cvlnext_0;
                    display = olddisplay_0;
                }
                if lx + (*cv_1).c_xoff < (*cv_1).c_xs {
                    let mut i_1: libc::c_int = 0;
                    let mut n_1: libc::c_int = (*cv_1).c_xs - (lx + (*cv_1).c_xoff);
                    if n_1
                        < ((*cv_1).c_xe - (*cv_1).c_xs + 1 as libc::c_int)
                            / 2 as libc::c_int
                    {
                        n_1 = ((*cv_1).c_xe - (*cv_1).c_xs + 1 as libc::c_int)
                            / 2 as libc::c_int;
                    }
                    if (*cv_1).c_xoff + n_1 > (*cv_1).c_xs {
                        n_1 = (*cv_1).c_xs - (*cv_1).c_xoff;
                    }
                    (*cv_1).c_xoff += n_1;
                    RethinkViewportOffsets(cv_1);
                    if n_1 > (*(*cv_1).c_layer).l_width {
                        n_1 = (*(*cv_1).c_layer).l_width;
                    }
                    let mut olddisplay_1: *mut display = display;
                    let mut oldflayer_1: *mut layer = flayer;
                    let mut l_1: *mut layer = (*cv_1).c_layer;
                    let mut cvlist_1: *mut canvas = (*l_1).l_cvlist;
                    let mut cvlnext_1: *mut canvas = (*cv_1).c_lnext;
                    flayer = l_1;
                    (*l_1).l_cvlist = cv_1;
                    (*cv_1).c_lnext = 0 as *mut canvas;
                    (Some(((*(*flayer).l_layfn).lf_LayRedisplayLine).unwrap()))
                        .unwrap()(
                        -(1 as libc::c_int),
                        -(1 as libc::c_int),
                        -(1 as libc::c_int),
                        1 as libc::c_int,
                    );
                    i_1 = 0 as libc::c_int;
                    while i_1 < (*flayer).l_height {
                        LScrollH(
                            flayer,
                            -n_1,
                            i_1,
                            0 as libc::c_int,
                            (*flayer).l_width - 1 as libc::c_int,
                            0 as libc::c_int,
                            0 as *mut mline,
                        );
                        (Some(((*(*flayer).l_layfn).lf_LayRedisplayLine).unwrap()))
                            .unwrap()(
                            i_1,
                            0 as libc::c_int,
                            n_1 - 1 as libc::c_int,
                            1 as libc::c_int,
                        );
                        i_1 += 1;
                        i_1;
                    }
                    if cv_1 == (*(*cv_1).c_display).d_forecv {
                        LGotoPos(flayer, (*flayer).l_x, (*flayer).l_y);
                    }
                    flayer = oldflayer_1;
                    (*l_1).l_cvlist = cvlist_1;
                    (*cv_1).c_lnext = cvlnext_1;
                    display = olddisplay_1;
                } else if lx + (*cv_1).c_xoff > (*cv_1).c_xe {
                    let mut i_2: libc::c_int = 0;
                    let mut n_2: libc::c_int = lx + (*cv_1).c_xoff - (*cv_1).c_xe;
                    if n_2
                        < ((*cv_1).c_xe - (*cv_1).c_xs + 1 as libc::c_int)
                            / 2 as libc::c_int
                    {
                        n_2 = ((*cv_1).c_xe - (*cv_1).c_xs + 1 as libc::c_int)
                            / 2 as libc::c_int;
                    }
                    if ((*cv_1).c_xoff - n_2 + (*(*cv_1).c_layer).l_width
                        - 1 as libc::c_int) < (*cv_1).c_xe
                    {
                        n_2 = (*cv_1).c_xoff + (*(*cv_1).c_layer).l_width
                            - 1 as libc::c_int - (*cv_1).c_xe;
                    }
                    (*cv_1).c_xoff -= n_2;
                    RethinkViewportOffsets(cv_1);
                    if n_2 > (*(*cv_1).c_layer).l_width {
                        n_2 = (*(*cv_1).c_layer).l_width;
                    }
                    let mut olddisplay_2: *mut display = display;
                    let mut oldflayer_2: *mut layer = flayer;
                    let mut l_2: *mut layer = (*cv_1).c_layer;
                    let mut cvlist_2: *mut canvas = (*l_2).l_cvlist;
                    let mut cvlnext_2: *mut canvas = (*cv_1).c_lnext;
                    flayer = l_2;
                    (*l_2).l_cvlist = cv_1;
                    (*cv_1).c_lnext = 0 as *mut canvas;
                    (Some(((*(*flayer).l_layfn).lf_LayRedisplayLine).unwrap()))
                        .unwrap()(
                        -(1 as libc::c_int),
                        -(1 as libc::c_int),
                        -(1 as libc::c_int),
                        1 as libc::c_int,
                    );
                    i_2 = 0 as libc::c_int;
                    while i_2 < (*flayer).l_height {
                        LScrollH(
                            flayer,
                            n_2,
                            i_2,
                            0 as libc::c_int,
                            (*flayer).l_width - 1 as libc::c_int,
                            0 as libc::c_int,
                            0 as *mut mline,
                        );
                        (Some(((*(*flayer).l_layfn).lf_LayRedisplayLine).unwrap()))
                            .unwrap()(
                            i_2,
                            (*flayer).l_width - n_2,
                            (*flayer).l_width - 1 as libc::c_int,
                            1 as libc::c_int,
                        );
                        i_2 += 1;
                        i_2;
                    }
                    if cv_1 == (*(*cv_1).c_display).d_forecv {
                        LGotoPos(flayer, (*flayer).l_x, (*flayer).l_y);
                    }
                    flayer = oldflayer_2;
                    (*l_2).l_cvlist = cvlist_2;
                    (*cv_1).c_lnext = cvlnext_2;
                    display = olddisplay_2;
                }
                cv_1 = (*cv_1).c_next;
            }
        }
        display = (*display).d_next;
    }
    display = displays;
    while !display.is_null() {
        if !((*display).d_status == 1 as libc::c_int || ((*display).d_cvlist).is_null()
            || ((*(*display).d_cvlist).c_next).is_null())
        {
            let mut olddisplay_3: *mut display = display;
            let mut oldflayer_3: *mut layer = flayer;
            let mut l_3: *mut layer = (*(*display).d_forecv).c_layer;
            let mut cvlist_3: *mut canvas = (*l_3).l_cvlist;
            let mut cvlnext_3: *mut canvas = (*(*display).d_forecv).c_lnext;
            flayer = l_3;
            (*l_3).l_cvlist = (*display).d_forecv;
            (*(*display).d_forecv).c_lnext = 0 as *mut canvas;
            (Some(((*(*flayer).l_layfn).lf_LayRestore).unwrap())).unwrap()();
            LGotoPos(flayer, (*flayer).l_x, (*flayer).l_y);
            flayer = oldflayer_3;
            (*l_3).l_cvlist = cvlist_3;
            (*(*display).d_forecv).c_lnext = cvlnext_3;
            display = olddisplay_3;
        }
        display = (*display).d_next;
    }
}
unsafe extern "C" fn logflush_fn(mut ev: *mut event, mut data: *mut libc::c_char) {
    let mut p: *mut win = 0 as *mut win;
    let mut buf: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut n: libc::c_int = 0;
    if islogfile(0 as *mut libc::c_char) == 0 {
        return;
    }
    logfflush(0 as *mut logfile);
    n = if log_flush != 0 {
        log_flush
    } else {
        (logtstamp_after + 4 as libc::c_int) / 5 as libc::c_int
    };
    if n != 0 {
        SetTimeout(ev, n * 1000 as libc::c_int);
        evenq(ev);
    }
    if logtstamp_on == 0 {
        return;
    }
    p = windows;
    while !p.is_null() {
        if !((*p).w_log).is_null() {
            (*p).w_logsilence += n;
            if !((*p).w_logsilence < logtstamp_after) {
                if !((*p).w_logsilence - n >= logtstamp_after) {
                    buf = MakeWinMsg(logtstamp_string, p, '%' as i32);
                    logfwrite((*p).w_log, buf, strlen(buf) as libc::c_int);
                }
            }
        }
        p = (*p).w_next;
    }
}
unsafe extern "C" fn ParseChar(
    mut p: *mut libc::c_char,
    mut cp: *mut libc::c_char,
) -> *mut libc::c_char {
    if *p as libc::c_int == 0 as libc::c_int {
        return 0 as *mut libc::c_char;
    }
    if *p as libc::c_int == '^' as i32
        && *p.offset(1 as libc::c_int as isize) as libc::c_int != 0
    {
        p = p.offset(1);
        if *p as libc::c_int == '?' as i32 {
            *cp = '\u{7f}' as i32 as libc::c_char;
        } else if *p as libc::c_int >= '@' as i32 {
            *cp = (*p as libc::c_int & 0o37 as libc::c_int) as libc::c_char;
        } else {
            return 0 as *mut libc::c_char
        }
        p = p.offset(1);
        p;
    } else if *p as libc::c_int == '\\' as i32
        && {
            p = p.offset(1);
            *p as libc::c_int <= '7' as i32
        } && *p as libc::c_int >= '0' as i32
    {
        *cp = 0 as libc::c_int as libc::c_char;
        loop {
            *cp = (*cp as libc::c_int * 8 as libc::c_int + *p as libc::c_int
                - '0' as i32) as libc::c_char;
            p = p.offset(1);
            if !(*p as libc::c_int <= '7' as i32 && *p as libc::c_int >= '0' as i32) {
                break;
            }
        }
    } else {
        let fresh31 = p;
        p = p.offset(1);
        *cp = *fresh31;
    }
    return p;
}
unsafe extern "C" fn ParseEscape(mut p: *mut libc::c_char) -> libc::c_int {
    let mut buf: [libc::c_uchar; 2] = [0; 2];
    if *p as libc::c_int == 0 as libc::c_int {
        SetEscape(0 as *mut acluser, -(1 as libc::c_int), -(1 as libc::c_int));
    } else {
        p = ParseChar(p, buf.as_mut_ptr() as *mut libc::c_char);
        if p.is_null()
            || {
                p = ParseChar(
                    p,
                    (buf.as_mut_ptr() as *mut libc::c_char)
                        .offset(1 as libc::c_int as isize),
                );
                p.is_null()
            } || *p as libc::c_int != 0
        {
            return -(1 as libc::c_int);
        }
        SetEscape(
            0 as *mut acluser,
            buf[0 as libc::c_int as usize] as libc::c_int,
            buf[1 as libc::c_int as usize] as libc::c_int,
        );
    }
    return 0 as libc::c_int;
}
unsafe extern "C" fn SetTtyname(mut fatal: bool, mut st: *mut stat) {
    let mut ret: libc::c_int = 0;
    let mut saved_errno: libc::c_int = 0 as libc::c_int;
    attach_tty_is_in_new_ns = 0 as libc::c_int != 0;
    memset(
        &mut attach_tty_name_in_ns as *mut [libc::c_char; 4096] as *mut libc::c_void,
        0 as libc::c_int,
        ::std::mem::size_of::<[libc::c_char; 4096]>() as libc::c_ulong,
    );
    *__errno_location() = 0 as libc::c_int;
    attach_tty = ttyname(0 as libc::c_int);
    if attach_tty.is_null() {
        if *__errno_location() == 19 as libc::c_int {
            saved_errno = *__errno_location();
            attach_tty = b"/proc/self/fd/0\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char;
            attach_tty_is_in_new_ns = 1 as libc::c_int != 0;
            ret = readlink(
                attach_tty,
                attach_tty_name_in_ns.as_mut_ptr(),
                ::std::mem::size_of::<[libc::c_char; 4096]>() as libc::c_ulong,
            ) as libc::c_int;
            if ret < 0 as libc::c_int
                || ret as size_t
                    >= ::std::mem::size_of::<[libc::c_char; 4096]>() as libc::c_ulong
            {
                Panic(
                    0 as libc::c_int,
                    b"Bad tty '%s'\0" as *const u8 as *const libc::c_char,
                    attach_tty,
                );
            }
        } else if fatal {
            Panic(
                0 as libc::c_int,
                b"Must be connected to a terminal.\0" as *const u8 as *const libc::c_char,
            );
        } else {
            attach_tty = b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char;
        }
    }
    if !attach_tty.is_null()
        && strcmp(attach_tty, b"\0" as *const u8 as *const libc::c_char) != 0
    {
        if stat(attach_tty, st) != 0 {
            Panic(
                *__errno_location(),
                b"Cannot access '%s'\0" as *const u8 as *const libc::c_char,
                attach_tty,
            );
        }
        if strlen(attach_tty) >= 4096 as libc::c_int as libc::c_ulong {
            Panic(
                0 as libc::c_int,
                b"TtyName too long - sorry.\0" as *const u8 as *const libc::c_char,
            );
        }
        if saved_errno != 19 as libc::c_int && CheckTtyname(attach_tty) != 0 {
            Panic(
                0 as libc::c_int,
                b"Bad tty '%s'\0" as *const u8 as *const libc::c_char,
                attach_tty,
            );
        }
    }
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
